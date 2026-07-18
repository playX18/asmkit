# asmkit code generation (`meta/`)

This directory contains the generators that produce the checked-in Rust sources under
`src/{x86,aarch64,riscv}/`, plus the data tables they consume. Generators are Python 3
(>= 3.12 required, `meta/rustbuilder.py` uses PEP 695 syntax); third-party Python deps:
`beautifulsoup4` (docenizers only).

## External inputs (pinned, not committed)

All third-party inputs are gitignored. Record exact versions here when updating.

| Input | Where it lives | Pinned version | License | Used for |
|---|---|---|---|---|
| AsmJit | `meta/asmjit` (git clone) | `0bd5787b54b575ed94bf32ac452153b34385c514` | Zlib | instdb C++ tables + `db/isa_*.json` (x86 + aarch64 instruction DB: effects, signatures, encodings, features) |
| felixcloutier.com x86 docs | `asm-docs/` (download) | rolling | (Intel manual derived) | x86 doc comments |
| ARM A64 ISA XML | `asm-docs-arm/` (download) | `ISA_A64_xml_A_profile-2023-03` (CE pin) | ARM proprietary notice — derive facts only, never redistribute | aarch64 doc comments |
| riscv-opcodes | `riscv-opcodes/` (git clone), env `RISCV_OPCODES` | `c6edca7d8c3f92694963a0a0baeb511930fb2af4` | BSD-3-Clause | riscv encodings, CSRs, causes |
| riscv-unified-db | `riscv-unified-db/` (git clone) | `v0.1.0` (`92d67fdaad0cf314701179b879c8b1fb796ade6c`) | BSD-3-Clause-Clear | riscv instruction descriptions (docs) |
| GDB `ppc-opc.c` | `src/ppc/opc.c` (committed, data only — never compiled) | snapshot | GPL — generated tables derived from it are the intended path | ppc (WIP) |

Fetching AsmJit:

```sh
git clone https://github.com/asmjit/asmjit meta/asmjit
git -C meta/asmjit checkout 0bd5787b54b575ed94bf32ac452153b34385c514
```

## The asmjit-model pipeline (new)

asmkit is built around AsmJit's instruction-database model: dense per-arch `InstId`s,
deduplicated static tables (encoding, signatures, RW/effects, features, names), and a
uniform `emit_n(InstId, &[Operand])` sink. Tools maintaining the generated tables:

1. **`meta/asmjit2rust.py`** — one-shot/re-runnable translator: AsmJit's generated C++
   instdb (`asmjit/{x86,arm}/*instdb*.{cpp,h}`, `*globals.h`) → Rust statics/enums
   (`src/x86/instdb.rs`, `src/aarch64/instdb.rs`). Also dumps `meta/{x86,a64}_rows.json`
   (the hand-maintained `INST(...)` rows) which are asmkit's owned input for the table
   generator. Self-checks: row counts, `#N` ordinals, and a diff of the regenerated
   aarch64 instdb against the previous revision.
2. **`meta/asmjit_db/`** — Python rewrite of AsmJit's JS pipeline (`db/*.js` readers +
   `tools/tablegen*.js` generators), emitting Rust directly: db JSON + rows JSON →
   `src/{x86,aarch64}/instdb.rs`, byte-exact against the translator output (both arches;
   132 tests). Also emits the asmkit-only NZCV extension `out/aarch64_rw_flags.rs`
   (per-instruction PSTATE R/W from db `io`), installed as `src/aarch64/rwflags.rs`.
   See `meta/asmjit_db/README.md` for the JS→Python divergence notes.
3. **`meta/x86_emitter.txt`** + **`meta/x86_emitter_gen.py`** — the x86 emitter API
   declarations extracted from AsmJit's `x86emitter.h` (3,275 decls) and the generator
   turning them into `src/x86/emitter.rs` typed traits (same role as `arm64.py` for
   aarch64). Besides the abstract-kind impls it emits impls for the sized register
   wrappers (`Gpq`/`Gpd`/`Gpw`/`GpbLo`/`GpbHi`, `Xmm`/`Ymm`/`Zmm`, ...) with
   `U: Into<Imm>` immediates, using the operand signatures from `meta/asmjit_db`
   (AsmJit's x86instdb.cpp) to keep only width-valid combinations. Re-extract with
   the regex in git history or from a fresh asmjit checkout.
4. **`meta/difftest/`** — differential-test harness: materializes the pre-rewrite
   baseline (`git archive 35caa377d68c4ba3b4577f691f3673f91d4338aa`) and generates an
   instruction corpus with expected bytes; `tests/x86_differential.rs` replays it through
   the new encoder.
5. **`meta/riscv.py`** (+ `meta/docenizer_riscv.py`) — riscv-opcodes →
   `src/riscv/{opcodes,emitter,instdb}.rs`, with derived RW effects and unified-db docs
   (`RISCV_OPCODES`/`RISCV_UNIFIED_DB` env vars default to the repo-root clones;
   `opcodes.rs` preserves the hand-maintained immediate section above the generation
   marker byte-identical). Extensions whose encodings `src/riscv/assembler.rs` cannot
   emit yet (`rv_zicfilp`, `rv_zicfiss`, `rv32_zilsd`, `rv32_zclsd`) are excluded in the
   generator — re-enable there once emit support lands.

## Legacy generators (being replaced)

- `arm64.py` — signature list → `src/aarch64/emitter.rs` (writes `./emitter.rs` at repo
  root; move + add header + rustfmt). Docs: `ASMKIT_ARM64_DOCS=asm-docs-arm python3 arm64.py`
- `ppc.py` — WIP, consumes `src/ppc/opc.c` as data.

## Licensing obligations

- AsmJit-derived files (translated tables, ported logic, db-JSON-derived tables): keep the
  Zlib license notice and mark the files as altered/derived (AsmJit Zlib §2).
- riscv-opcodes / riscv-unified-db derived files: keep BSD notices.
- ARM XML: extract facts (encodings, instruction metadata) and doc text for generated doc
  comments; do not commit the XML itself or large verbatim excerpts.
- Do not commit fetched inputs (`meta/asmjit`, `riscv-opcodes`, `asm-docs*`); they are
  gitignored. Pins belong in this file.
