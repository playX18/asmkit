# uasm

`#![no_std]` assembler library. 

# Features
- X86, RISC-V, and ARM (WIP) assemblers
- Small and portable library.
- Tiny amount of dependencies:
    - `libc`, `intrusive-collections`: for JIT support
    - `paste`, `derive-more`: makes our life simpler when declaring arch-specific stuff over and over
    - `smallvec`: for code generation to not heap allocate often
- Relocations are provided by CodeBuffer interface and assembler will use them if you use symbols in API.


# TODO
- [ ] Add support for ARM64
- [ ] Add support for OpenPOWER (POWER9/POWER10)
- [ ] Add support for RW info and implicit operand info for all opcodes
- [ ] Cross-platform helpers to perform calls
- [ ] SpiderMonkey-like `MacroAssembler` to help generate assembly without worrying about target architecture
- [ ] Compiler/Builder interface: emit instructions as `Inst` structure and allow modifying them before emitting,
and also possibly to have regalloc pass over them.