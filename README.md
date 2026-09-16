# asmkit

`#![no_std]` assembler library for x86/x64, RISC-V (rv32/rv64), and AArch64, built around an
[AsmJit](https://github.com/asmjit/asmjit)-style instruction-database model. The x86 and
AArch64 instruction databases and APIs are direct ports of AsmJit's; credit for that design
goes to the AsmJit authors.

MSRV: Rust 1.85 (edition 2024).

## Quick start

```rust
use asmkit::{Arch, CodeBuffer, Environment, JitAllocator};
use asmkit::x86::*;

let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
{
    let mut asm = Assembler::new(&mut buf);
    asm.mov(RAX, 5);
    asm.add(RAX, 37);
    asm.ret();
}

let result = buf.finish().expect("assembly failed");
let mut jit = JitAllocator::new(Default::default());
let span = result.allocate(&mut jit).expect("failed to allocate JIT code");
let f: extern "C" fn() -> u64 = unsafe { std::mem::transmute(span.rx()) };
assert_eq!(f(), 42);
```

More examples live in `examples/`: `factorial.rs` (x86/RISC-V/AArch64 + JIT + disassembly),
`reloc.rs` (relocations), `a64.rs` (AArch64), `patch.rs` (JSC-style patching).

## Features

- Typed emitter API per architecture (`asm.mov(RAX, 42)`), generated from the instruction
  database, with a shared `emit_n` entry point underneath
- Read/write effects per instruction via `query_rw_info`
- `Builder` + `InstSink` for recording, inspecting, and replaying instructions
- Relocations and multi-module linking (`Section` + `Linker`)
- Patchable jumps/calls/immediates for JSC-style code patching after emission

See [PORTING.md](PORTING.md) for backend structure and [meta/README.md](meta/README.md) for
the code-generation pipeline.

## Cargo features

- `x86`, `riscv`, `aarch64`: per-architecture backends (enabled by default)
- `jit`: JIT allocator, for allocating and executing generated code

## License

asmkit itself is `MIT OR Apache-2.0`.

- [AsmJit](https://github.com/asmjit/asmjit) (Zlib): the instruction-database model, the x86
  and AArch64 instdb tables, the x86 emitter signatures, and the `_emit`/encoder logic are
  ported from AsmJit. Derived files carry the Zlib notice as required.
- [riscv-opcodes](https://github.com/riscv/riscv-opcodes) (BSD-3-Clause): opcode tables used
  to generate the RISC-V assembler.
- [riscv-unified-db](https://github.com/riscv-software-src/riscv-unified-db) (BSD-3-Clause-Clear):
  RISC-V instruction descriptions for generated docs.

See [meta/README.md](meta/README.md) for the exact pins and licensing obligations of generated
files.
