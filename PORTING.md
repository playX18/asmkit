# Porting 

Porting asmkit to other architectures requires providing opcode table and a code generator which would generate
encoder and decoder from such opcode-table. You can look at example tables in `meta/` directory. For RISC-V we use `riscv-opcodes`
repo directly and it is cloned if we want to rebuild encodings. 

Code-generator can be written in any language you like. Python is used because of simplicity by default.

## Code generator

The job of codegen is to generate two parts of asmkit port: encoder and decoder. Encoder is used to generate assembler code
while decoder is used to disassemble code. Decoder is optional part while encoder is a must have. To make life simpler you should
consider emitting `<ARCH>EmitterExplicit` directly which then ivnokes some internal functionality to that backend to actually do
encoding. 

