# asmkit

`#![no_std]` assembler library, built around an [AsmJit](https://github.com/asmjit/asmjit)-style
instruction-database model. We use AsmJit DB for x86 and aarch64 directly, plus APIs are direct ports of AsmJit ones. All credit goes to AsmJit authors. 

The minimum supported Rust version (MSRV) is Rust 1.85 (edition 2024). 

# Features

- x86/x64, RISC-V (rv32 and rv64), and AArch64 assemblers
- The RISC-V backend targets both XLENs: construct the buffer with
  `Environment::new(Arch::RISCV32)` or `Environment::new(Arch::RISCV64)`;
  and instructions with no encoding on the target XLEN (e.g. `ld` on rv32) are rejected at
  emit time
- Uniform per-architecture model, shared by every backend:
    - dense `InstId` enum → static instdb tables (`src/{x86,aarch64,riscv}/instdb.rs`)
    - one core emit path: `Assembler::emit_n(impl Into<u32>, &[&Operand])`
    - generated per-mnemonic emitter traits with typed sized registers
      (`asm.mov(RAX, 42)` — no dereferencing, integer literals convert to immediates)
    - prefix setters on the x86 `Assembler` (`rep()`, `lock()`, `seg()`, `k()`, `z()`,
      SAE/rounding) consumed by the next emit
- Read/write effects per instruction: `query_rw_info(&Inst) -> InstRwInfo`
- `Builder` (src/core/builder.rs): record instructions as generic `Inst` nodes, inspect or
  mutate them, then replay into an `InstSink` (every architecture's `Assembler` implements it)
- Small and portable:
    - `libc`, `intrusive-collections`, `errno`: for JIT support
    - `paste`, `bitflags`, `cfgenius`, `num-traits`: make declaring arch-specific stuff simpler
    - `smallvec`: avoid frequent heap allocation during code generation
- Relocations are provided by the `CodeBuffer` interface and the assembler will use them if
  you use symbols in the API.
- Multi-module linking: `Section` + `Linker` (src/core/{section,linker}.rs) lay out named
  sections, resolve cross-module symbol references, and leave undefined symbols external for
  load-time resolution (`CodeBufferFinalized::allocate_resolved`); entry points are looked up
  with `CodeBufferFinalized::defined_symbol_offset`.

# Architecture in one paragraph

Every backend follows the same layout: a generated `instdb.rs` (dense `InstId` + static
tables translated from AsmJit's instdb), an `assembler.rs` whose `emit_n` is the single emit
entry point, a generated `emitter.rs` of per-mnemonic traits forwarding to `emit_n`, and an
`instapi.rs` implementing the shared `query_rw_info` contract over `src/core/inst.rs`'s
generic `Inst`. The x86 backend additionally ports AsmJit's `_emit` signature validation
(`src/x86/emit.rs`) and the `x86assembler.cpp` encoding handlers (`src/x86/encoder.rs`).
See [PORTING.md](PORTING.md) for the backend structure and
[meta/README.md](meta/README.md) for the code-generation pipeline.

# Quick start

The executable-memory portion of this example requires `--features jit`.

```rust
use asmkit::{Arch, CodeBuffer, Environment, JitAllocator};
use asmkit::x86::*;

let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
{
let mut asm = Assembler::new(&mut buf);

// Typed sized registers (RAX: Gpq) and plain integer immediates.
asm.mov(RAX, 5);
asm.add(RAX, 37);
asm.ret();
}

let result = buf.finish().expect("assembly failed");
let mut jit = JitAllocator::new(Default::default());
// Dual-mapped pages: write through `span.rw()`, execute `span.rx()`.
let span = result.allocate(&mut jit).expect("failed to allocate JIT code");
let f: extern "C" fn() -> u64 = unsafe { std::mem::transmute(span.rx()) };
#[cfg(all(unix, target_arch = "x86_64"))]
assert_eq!(f(), 42);
```

Prefixes apply to the next emitted instruction:

```rust,ignore
asm.lock().add(ptr64(RCX, 0), EAX); // lock add [rcx], eax
```

More complete examples live in `examples/`: `factorial.rs` (x86/RISC-V/AArch64 + JIT +
disassembly), `reloc.rs` (relocations), `a64.rs` (AArch64).

# Cargo features

- `x86`, `riscv`, `aarch64` — per-architecture backends (enabled by default)
- `jit` — JIT allocator (`libc`/`errno` on Unix and `windows-sys` on Windows)
- default: `x86`, `riscv`, `aarch64`; add `jit` explicitly for executable-memory allocation

The default x86/x64 target enables AVX, AVX2, and all AVX-512 families represented by the
instruction database. This permits unrestricted assembly; it does not detect the host CPU.
Use `Environment::baseline(Arch::X64)` for a conservative target, then opt into individual
extensions with `set_x86_feature` or the AVX/AVX2/AVX-512 convenience setters.

# Testing

```sh
cargo test                                    # standard unit and integration tests
cargo test --features validation              # exhaustive validation suites (sampled x86 corpus)
ASMKIT_X86_CORPUS_FULL=1 \
  cargo test --features validation # full ~136k-record corpus (sampled ~23k by default)
ASMKIT_X86_CORPUS_FULL=1 ASMKIT_X86_CORPUS_SHARD=0/4 \
  cargo test --features validation --test x86_differential corpus_full
cargo test --all-features --test independent_oracles # LLVM MC checks when llvm-mc is installed
cargo test --all-features --test abi_execution       # native ABI-safe JIT program on the host target
cargo +nightly miri test --no-default-features --lib builder_tests
RUSTFLAGS='-Zsanitizer=leak' cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --all-features --lib core::jit_allocator::tests
RUSTFLAGS='-Zsanitizer=thread' cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --all-features --lib info_initialization_is_thread_safe
(cd fuzz && cargo +nightly fuzz run raw_emission -- -max_total_time=30)
(cd fuzz && cargo +nightly fuzz run buffer_lifecycle -- -max_total_time=30)
cargo check --all-targets
```

The `validation` feature enables the exhaustive suites below; `--all-features` includes it.
`tests/x86_differential.rs` replays a pre-rewrite baseline corpus through the new encoder and
round-trips ~52k encodings through iced-x86. The `meta/difftest/` harness materializes the
baseline. `tests/x86_coverage.rs` sweeps every x86 `InstId` through all of its declared instdb
signatures in both 64-bit and 32-bit modes and asserts everything except the documented
mode-gated allowlists emits successfully. `tests/aarch64_vectors.rs` pins AArch64 encodings
against AsmJit/LLVM-verified vectors, and `tests/aarch64_selfcheck.rs` hashes a broad
instruction sweep to catch accidental behavior changes in the AArch64 encoder.

The full x86 corpus is split by stable record index in CI. `iced-x86` byte comparisons only use
forms with one canonical encoding; its broader decode round-trip remains the secondary check.
`independent_oracles` invokes LLVM MC when available and currently covers representative immediate,
addressing, alias, system, feature-gated, compressed, vector, atomic, CSR, and label-pair forms.
It is deliberately not a textual assembler for every generated signature: LLVM syntax and feature
spelling are external-tool constraints. Known byte discrepancies live in the small ratcheting table
in `tests/independent_oracles.rs`; any other oracle difference fails the test.
The Miri gate is limited to the core builder/ownership subset; JIT allocation and emitted-code
execution are platform-memory tests and are excluded from Miri. LeakSanitizer exercises allocation,
release, reset, shrink, stale-handle, and panic-restoration paths; ThreadSanitizer exercises the
concurrent virtual-memory information initializer.

The two cargo-fuzz targets cover raw operands/emission and labels, fixups, finalization, linking,
and patch metadata. CI runs short fuzz smoke jobs on push and pull requests and longer jobs weekly.
The ABI test executes a no-argument scalar-return program natively on x86_64 and under QEMU for
AArch64 and RV64 in CI.

# Regenerating the tables

```sh
bash meta/regen.sh --check   # CI mode: fails if regeneration would change any tracked file
bash meta/regen.sh           # actually regenerate
```

Requires the pinned external inputs described in [meta/README.md](meta/README.md)
(AsmJit clone, riscv-opcodes, riscv-unified-db, optional docs).

# Goals

- Auto-generated assemblers for as many platforms as possible.
- Portability: the library should build & run on any platform (even one without an assembler
  for it), and the assemblers must not depend on the platform `asmkit` was built on.

# TODO

- [X] ~~Add support for ARM64~~ 
- [X] ~~Emit instructions as `Inst` structures for inspection/modification~~ `Builder` + `InstSink`.
- [X] ~~RW info and implicit operand info for all opcodes~~ `query_rw_info` per arch.
- [ ] Register allocation pass over `Builder` output
- [ ] Cross-platform helpers to perform calls
- [ ] JSC/SpiderMonkey-like `MacroAssembler` to help generate assembly without worrying about target architecture

# Related projects & licenses

- [AsmJit](https://github.com/asmjit/asmjit) (Zlib): the instruction-database model, the x86
  and AArch64 instdb tables, the x86 emitter signatures, and the `_emit`/encoder logic are
  ported from AsmJit. Derived files carry the Zlib notice as required.
- [riscv-opcodes](https://github.com/riscv/riscv-opcodes) (BSD-3-Clause): opcode tables used
  to generate the RISC-V assembler.
- [riscv-unified-db](https://github.com/riscv-software-src/riscv-unified-db) (BSD-3-Clause-Clear):
  RISC-V instruction descriptions for generated docs.

asmkit itself is `MIT OR Apache-2.0`. See [meta/README.md](meta/README.md) for the exact pins
and licensing obligations of generated files.
