# asmkit

`#![no_std]` assembler library. 

# Features
- X64, RISC-V, PPC, and ARM (WIP) assemblers
- Small and portable library.
- Tiny amount of dependencies:
    - `libc`, `intrusive-collections`: for JIT support
    - `paste`, `derive-more`: makes our life simpler when declaring arch-specific stuff over and over
    - `smallvec`: for code generation to not heap allocate often
- Relocations are provided by CodeBuffer interface and assembler will use them if you use symbols in API.

# Goals
- Auto-generated assemblers for as many as possible platform.
- Portability: library should built & run on any platform (even if it does not provide assembler for one), and assemblers on its own
must not be dependent on platform we built `asmkit` on.



# TODO
- [ ] Add support for ARM64
- [ ] Add support for PPC64
- [ ] Add support for OpenPOWER (POWER9/POWER10)
- [ ] Add support for RW info and implicit operand info for all opcodes
- [ ] Cross-platform helpers to perform calls
- [ ] JSC/SpiderMonkey-like `MacroAssembler` to help generate assembly without worrying about target architecture
- [ ] Compiler/Builder interface: emit instructions as `Inst` structure and allow modifying them before emitting,
and also possibly to have regalloc pass over them.

# Related projects

- [AsmJit](https://github.com/asmjit/asmjit): Core API, JIT API, and operands are ported from AsmJit,
the overall idea of making portable assembler in Rust comes from AsmJit
- [fadec](https://github.com/aengelke/fadec): x86/64 encoding/decoding library. We use opcode tables provided by fadec 
for x86/64 support and `encode.c` is partially used for emitting code. 
- [disarm](https://github.com/aengelke/disarm): AARch64 encoding/decoding library. We use opcode tables from disarm 
to generate AArch64 encodigns
- [riscv-opcodes](https://github.com/riscv/riscv-opcodes): Opcode table for RISC-V, used to generate RISC-V assembler/disassembler.
- [GDB](https://sourceware.org/gdb/): GDB is a debugger but we use its `ppc-opc.c` as an opcode table for PowerPC support. 