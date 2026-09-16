# Backend structure

Each architecture under `src/{x86,aarch64,riscv}/` follows the same shape:

- `instdb.rs`: generated, dense `InstId` enum + static tables (encodings, operand
  signatures, RW effects, features)
- `operands.rs`: architecture-specific operand types (registers, memory, immediates)
- `assembler.rs`: hand-written, the `Assembler`, with `emit_n(InstId, &[&Operand])` as the
  single emit entry point
- `emitter.rs`: generated, per-mnemonic typed traits (`asm.mov(RAX, 42)`) forwarding to
  `emit_n`
- `instapi.rs`: hand-written, implements `query_rw_info` and friends over `src/core/inst.rs`'s
  generic `Inst`, using the tables in `instdb.rs`
- `arch_traits.rs`: glue implementing the shared `src/core` traits for this architecture

x86 additionally has:

- `emit.rs`: validates operand signatures before encoding
- `encoder.rs` / `encoder_tables.rs`: does the actual bit packing
- `opcode.rs`: opcode/prefix constants

AArch64 additionally has `rwflags.rs` (per-instruction PSTATE read/write, asmkit-only,
generated). RISC-V additionally has `opcodes.rs` (opcode constants, partly hand-maintained
above a generation marker, see `meta/riscv.py`).

`src/core/` holds everything shared across backends: the generic `Inst`/`Operand` model
(`inst.rs`, `operand.rs`), `CodeBuffer`/`Section`/`Linker` for buffer management and
multi-module linking, `Builder` for recording/replaying instructions, `patch.rs` for
JSC-style patching, and `jit_allocator.rs` for executable memory.

# Code generation (`meta/`)

Everything above marked "generated" is produced from external instruction databases
(AsmJit for x86/AArch64, riscv-opcodes/riscv-unified-db for RISC-V) by Python scripts in
`meta/`, driven by `meta/regen.sh`. See [meta/README.md](meta/README.md) for the pinned
inputs, the generator pipeline, and licensing obligations for the generated files.

# Adding a new backend

This walks through wiring up a new architecture (`newarch` below) end to end. RISC-V is
the best reference for a backend that isn't driven by AsmJit's database: its `instdb.rs`
and `opcodes.rs` are generated from a different external source, and its `emitter.rs`/
`assembler.rs` are structured the same as x86/AArch64.

## 1. Feature flag and module wiring

- Add a feature to `Cargo.toml`: `newarch = []`, and add it to `default` if the backend
  should ship by default.
- Add `pub mod newarch;` (feature-gated) to `src/lib.rs`, alongside the existing
  `x86`/`aarch64`/`riscv` modules.
- Create `src/newarch/mod.rs` re-exporting `assembler`, `emitter`, `instapi::query_rw_info`,
  `instdb`'s CPU feature types, `operands`, and any opcode/regs modules; mirror
  `src/riscv/mod.rs`.

## 2. `Arch` enum and `arch_traits.rs`

- Add a variant to `Arch` in `src/core/arch_traits.rs` (pick an unused discriminant; the
  even/odd-width convention documented next to the enum is cosmetic, not load-bearing,
  keep it if convenient).
- Add `NEWARCH_ARCH_TRAITS: ArchTraits` (feature-gated, with a `NO_ARCH_TRAITS` fallback
  when the feature is off, copy the `X86`/`RISCV` `#[cfg]` pairs) and wire it into
  whatever dispatches on `Arch` (`ArchTraits::for_arch` or equivalent).
- Implement `src/newarch/arch_traits.rs`: the `ArchTraits` table (stack/frame/link/IP
  register ids, stack alignment, and the `RegType`/`TypeId`/signature tables for every
  register class the architecture has).

## 3. Operands

Write `src/newarch/operands.rs`: register types and constants (implementing
`src/core/operand.rs`'s `RegTraits`/`OperandCast`), memory operand types, and any
architecture-specific immediate wrappers. This is hand-written; it's the public API
surface users interact with directly (`asm.mov(RAX, 42)`).

## 4. Instruction database (`instdb.rs`)

This is normally generated, not hand-written, because instruction sets are large and
error-prone to transcribe by hand. Decide where the source data comes from, for example
follow the RISC-V pattern (`meta/riscv.py`): pull from whatever authoritative
machine-readable source exists for the ISA (riscv-opcodes/riscv-unified-db in that case),
and generate the same shape of tables: an `InstId` enum, per-instruction operand
signatures, and encoding data, so the rest of `src/core` (which is architecture-agnostic
over these tables) doesn't need to special-case the backend.

Whichever source you use, the generator should also be able to emit `emitter.rs` (the typed
per-mnemonic traits) from the same signature data, and, if the architecture has flag/CPU-state
read/write effects worth modeling, an `RwInfo`-shaped table for `instapi.rs` to consume.

Add the new generator invocation and its output files to `GENERATED_FILES` and the step list
in `meta/regen.sh`, and document any new pinned external input (repo, commit, license) in
`meta/README.md`'s input table.

## 5. `assembler.rs` and encoding

Write the actual bit-packing/encoding logic and the `Assembler` type implementing
`src/core/builder.rs`'s `InstSink` trait (`arch()`, `emit_inst()`, `bind_label()`) plus
whatever direct `emit_n(InstId, &[&Operand])` entry point the generated `emitter.rs` calls
into. For a fixed-width instruction set this can be a single file; x86's variable-length
encoding needed the extra `emit.rs` (operand-signature validation) / `encoder.rs` /
`encoder_tables.rs` split, don't replicate that split unless the architecture actually
needs it.

## 6. `instapi.rs`

Hand-written code implementing `query_rw_info` (and any other per-instruction query
functions) over the generic `Inst` type from `src/core/inst.rs`, reading whatever RW/flags
tables `instdb.rs` (or a sibling generated file, like AArch64's `rwflags.rs`) provides.
