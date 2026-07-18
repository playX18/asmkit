# asmkit

`#![no_std]` assembler library, built around an [AsmJit](https://github.com/asmjit/asmjit)-style
instruction-database model.

# Features

- X64, RISC-V, and AArch64 assemblers (64-bit x86 only; PPC is WIP)
- Uniform per-architecture model, shared by every backend:
    - dense `InstId` enum → static instdb tables (`src/{x86,aarch64,riscv}/instdb.rs`)
    - one core emit path: `Assembler::emit_n(impl Into<u32>, &[&Operand])`
    - generated per-mnemonic emitter traits with typed sized registers
      (`asm.mov(RAX, 42)` — no dereferencing, integer literals convert to immediates)
    - prefix setters on the x86 `Assembler` (`rep()`, `lock()`, `seg()`, `k()`, `z()`,
      SAE/rounding) consumed by the next emit
- Read/write effects per instruction: `query_rw_info(&Inst) -> InstRwInfo`
  (`src/{x86,aarch64,riscv}/instapi.rs`), intended as the input for a future regalloc
- `Builder` (src/core/builder.rs): record instructions as generic `Inst` nodes, inspect or
  mutate them, then replay into an `InstSink` (every architecture's `Assembler` implements it)
- Small and portable:
    - `libc`, `intrusive-collections`, `errno`: for JIT support
    - `paste`, `bitflags`, `cfgenius`, `num-traits`: make declaring arch-specific stuff simpler
    - `smallvec`: avoid frequent heap allocation during code generation
- Relocations are provided by the `CodeBuffer` interface and the assembler will use them if
  you use symbols in the API.

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

```rust
use asmkit::core::buffer::CodeBuffer;
use asmkit::core::jit_allocator::JitAllocator;
use asmkit::x86::*;

let mut buf = CodeBuffer::new();
let mut asm = Assembler::new(&mut buf);

// Typed sized registers (RAX: Gpq) and plain integer immediates.
asm.mov(RAX, 5);
asm.add(RAX, 37);
asm.ret();

let result = buf.finish();
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

- `x86`, `riscv`, `aarch64` — per-architecture backends
- `jit` — JIT allocator (`libc`, `errno`)
- default: `x86`, `riscv`, `aarch64`, `jit`

# Testing

```sh
cargo test                                    # unit tests + x86 differential corpus (sampled) + iced-x86 decode roundtrip
ASMKIT_X86_CORPUS_FULL=1 cargo test           # full ~136k-record corpus (sampled ~23k by default)
cargo check --all-targets
```

`tests/x86_differential.rs` replays a pre-rewrite baseline corpus through the new encoder and
round-trips ~52k encodings through iced-x86. The `meta/difftest/` harness materializes the
baseline.

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

- [X] ~~Add support for ARM64~~ Works! Not everything is tested, so if errors occur please open an issue.
- [X] ~~Emit instructions as `Inst` structures for inspection/modification~~ `Builder` + `InstSink`.
- [X] ~~RW info and implicit operand info for all opcodes~~ `query_rw_info` per arch.
- [ ] Register allocation pass over `Builder` output
- [ ] Add support for PPC64
- [ ] Add support for OpenPOWER (POWER9/POWER10)
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
- [GDB](https://sourceware.org/gdb/) (GPL): `ppc-opc.c` as an opcode table for PowerPC (WIP).

asmkit itself is `MIT OR Apache-2.0`. See [meta/README.md](meta/README.md) for the exact pins
and licensing obligations of generated files.
