# asmjit_db

Stdlib-only Python port of asmjit's Node.js codegen database layer, pinned at
`meta/asmjit` (SHA `0bd5787b54b575ed94bf32ac452153b34385c514`, Zlib license):

- `gen_common.py` — port of `tools/generator-commons.js` (`ObjectUtils`,
  `ArrayUtils`, `StringUtils`, `IndexedArray`).
- `exp.py` — port of `db/exp.js` (expression tokenizer / parser / AST /
  evaluator).
- `tablegen.py` — port of `tools/tablegen.js` (`Task`/`TableGen`,
  `InstructionNameData`, `IdEnum`, `NameTable`, `generate_name_data`).
- `cxx_src.py` — vendored-C++ machinery shared by the arch drivers:
  `${Key:Begin/End}` block extraction, table-row/enum/constant-expression
  parsing, C string-literal decoding, Rust emission helpers, token-based
  normalized diff.
- `tablegen_x86.py` / `tablegen_a64.py` — arch drivers that compute the full
  instruction-database tables from the vendored sources and emit Rust
  byte-equal to `src/x86/instdb.rs` / `src/aarch64/instdb.rs`
  (`--check` compares; `--stdout` prints). The a64 driver additionally emits
  the asmkit-specific NZCV `RW_FLAGS_TABLE` (see note 20 below).
- `templates/` — hand-adapted Rust scaffolding (POD struct/enum ports)
  spliced verbatim by the arch drivers.
- `out/` — generated artifacts (`x86_instdb.rs`, `aarch64_instdb.rs`,
  `aarch64_rw_flags.rs`).

Requires Python 3.12+. Tests: `python3 -m meta.asmjit_db.test_all`,
`python3 -m meta.asmjit_db.test_tablegen`, and
`python3 -m meta.asmjit_db.test_instdb` from the repo root (also
discoverable by plain `unittest` discovery).

## Conventions

- Method and function names are snake_case (`add_indexed`, `dec_to_hex`,
  `ext_array`, `minimum_operand_count`, ...).
- Data fields keep their JS camelCase names (`opcodeString`, `aliasOf`,
  `regType`, `immValue`, ...) because they mirror the asmjit database schema
  and the JSON keys consumed by the pipeline.

## Deliberate divergences from the JS

1. **32-bit arithmetic emulation (`exp.evaluate`)** — arithmetic results are
   masked to signed int32; `/` and `%` truncate toward zero like JS `|0`;
   `<<`/`>>` mask the shift count to 5 bits. JS computes `+`, `-`, `*` as
   IEEE-754 doubles, so results can diverge for decimal literals outside the
   int32 range (asmjit data is integer-domain and never does this). Hex
   literals parse to signed int32 exactly as in JS (`0xFFFFFFFF` is `-1`, so
   `0xFFFFFFFF + 1` evaluates to `0` either way). Division/modulo by zero
   raises `ZeroDivisionError` (JS yields `0` via `Infinity|0` / `NaN|0`).
2. **Ternary `?:` rejected** — the JS ternary branch is dead code that
   references an undefined `info` variable (ReferenceError if ever reached),
   and no asmjit JSON data uses ternary. `exp` raises `ExpressionError` on
   `?` instead of replicating the bug.
3. **Sort stability** — Python `sorted()`/`list.sort()` are stable, matching
   JS `Array.prototype.sort` (ES2019+), so sorted output order is equivalent.
4. **Dict insertion order** — Python dicts preserve insertion order exactly
   like JS objects do for string keys. (JS reorders canonical integer-like
   keys numerically first; the database uses string keys, so iteration order
   matches.)
5. **`IndexedArray` dedup key** — JS keys items by `JSON.stringify(item)`
   (for the C++ table generators the item is the final C++ row text). The
   port uses a canonical structural key (dict keys sorted recursively,
   tuples for lists) so the emission format (Rust vs C++) cannot affect
   dedup ORDER. First-seen insertion order is preserved exactly as in JS.
   Only same-content/different-key-order dicts dedup more aggressively than
   JS — never occurs, as items are built programmatically with stable key
   order. Booleans are distinguished from ints (Python `True == 1`; JS
   `true !== 1`). `add_indexed()` returns `(index, is_new)`; JS returns only
   the index.
6. **`Parsing.match_closing_char` `()` fix** — the pinned JS maps `'('`
   (char code 40) to closing code **31** (a control character) instead of
   `')'` (41), so it never matches `)` and `split_operands` glues the rest
   of the string into one operand. The port uses 41, so `(...)` operands
   are skipped correctly.
7. **`find_key` / `has_any`** — JS iterates `keys` with `for..in`, which
   yields array *indices* for array arguments. All JS callers pass plain
   objects (`RegOp`, `MemOp`), so the port iterates the given keys/values,
   which matches every real call site and behaves sanely for lists.
8. **`parse_bit_access` wraps the index in `ImmNode`** — JS stores the raw
   number as the second `$bit` arg, which breaks `evaluate()` and visitors
   on the resulting node.
9. **`Instruction._assign_attribute` actually validates keys** — JS intends
   to `FAIL` on unknown keys but `typeof this[key] === undefined` is always
   false (a JS bug), so JS silently adds arbitrary fields. The port raises
   `AttributeError` for unknown attributes.
10. **Tokenizer quirks kept for fidelity** — the hex parser rejects
    lowercase `g`-`z` (the JS uppercase clause `'g' <= c <= 'Z'` is dead code
    and kept dead); the JS number regex's `[E|e]` / `[+|-]` character-class
    quirk (a literal `|` is accepted in exponents) is replicated together
    with a `parseFloat` emulation.
11. **Smaller fixes** — `UnaryNode.clone()` clones `child` (JS clones the
    nonexistent `this.left`, producing `null`); `union_cpu_features()` drops
    the unused JS `name` parameter; `ObjectUtils.clone()` is a shallow copy
    (as JS `Object.assign`), while `merge()` recurses into dict/dict pairs
    (JS recurses into any object pair, including arrays — never used that
    way); evaluating a variable/call without a context raises
    `ExpressionError` (JS throws a `TypeError`).

### tablegen.py

12. **`Injector` replaced by `outputs: dict[str, str]`** — the JS `Injector`
    patches `// ${Key:Begin/End}` regions inside asmjit's own C++ sources.
    This pipeline emits whole files, so tasks collect generated sections into
    `TableGen.outputs` (sizes in `TableGen.table_sizes`) via `Task.emit()`.
13. **Small-encoding regex is the JS `/^[a-z0-4]{0,6}$/`** — the empty name
    of `kIdNone` IS small-encoded (first index-table entry `0x80000000`).
    A `{1,6}` reading would route it through the large-name path and produce
    a completely different table.
14. **`index()` keeps JS floating-point semantics** for
    `min_prefix_size = name.length / 2 + 1` (e.g. `5.5` for length 9) in the
    `longest_prefix >= min_prefix_size` comparison; floor division
    (`len//2 + 1`) would split names that JS keeps whole.
15. **Candidate sort is a pure stable sort by descending name length** — the
    JS tiebreak compares two objects with `<`/`>` (always `0`), i.e. dead
    code; equal lengths keep insertion order either way.
16. **Dead JS artifacts not ported** — the `size === -1` guards (`size` is
    never `-1`), `getIndex()` (reads a never-assigned `map`), the
    `aliasMem`/`aliasMap` typo (single `alias_map` here), and a
    `console.log` debug leftover for `"jz"` in `add()`.
17. **Empty display name (`kIdNone`) is skipped** in the 26 per-letter span
    computation — JS computes a `NaN` alpha index there, which silently
    bypasses the range check and lands in a slot the output loop never reads;
    skipping is observably identical.

### Arch drivers (tablegen_x86.py / tablegen_a64.py)

18. **Name tables are computed, the rest is parsed from the pinned generated
    C++** — `InstructionNameData` (from `InstId` enum order + db alias
    formats) regenerates the name string/index/alias tables, and each driver
    cross-checks them against the pinned `NameData` section at load time
    (they are byte-identical, as the JS pipeline is deterministic). The
    remaining tables (opcodes, signatures, RW info, …) are the deeply
    arch-specific precomputed output of the JS pipeline; the drivers parse
    the pinned generated C++ for those and re-emit them as Rust, with
    row-count/ordinal self-checks at every step.
19. **Templates are spliced verbatim** — the hand-adapted Rust scaffolding in
    `templates/` (POD struct ports, macro blocks) was extracted once from the
    translator-generated `src/` files, the same methodology the translator
    uses for its embedded literals; `--check` guards any later drift.
20. **AArch64 `RW_FLAGS_TABLE` is an asmkit extension** — NZCV (PSTATE)
    effects are not part of asmjit's generated tables (see the `TODO(P3)` in
    `src/aarch64/instapi.rs`); the a64 driver computes them from
    `db/isa_aarch64.json` `io` attributes using base.py's JS-faithful io
    parser. Exactly 8 distinct io strings exist. Mapping: `R`/`W`/`X` =
    read/write/read-write of `A64_N|Z|C|V`; `QC|=SAT` (which the JS parser
    splits into key `QC` value `SAT` plus an empty key from the trailing
    pipe — the empty key is ignored) is a conditional write of the
    saturation flag `A64_Q`. Name mapping: enum `_v` names use the db name
    without the suffix, `b.<cond>` maps to `b`, `asr|asrv`-style pipe names
    are split (they carry no `io` on current data). Flags are the union
    across all db forms of a name, emitted per `InstId` in enum order as
    `(read_flags, write_flags)` tuples of `CpuRwFlags::*.bits()` expressions.
21. **`--check` semantics** — the emitted instdb is compared against the
    translator-generated `src/` file (read-only): byte-exact first, with a
    token-based normalized diff as fallback/report; exit status is non-zero
    on any token mismatch. Currently both files match **byte-exactly**.
