use capstone::{arch::riscv::ArchMode, prelude::*};
use uasm::{assembler::{codeholder::CodeBuffer, riscv::*},  encdec::riscv::{decode::*, formatter::pretty_disassembler}};

fn main() {
    let mut buf = CodeBuffer::new();

    let mut asm = Assembler::new(&mut buf);

    asm.c_ld(X8, X8, 16).unwrap();
    
    let result = buf.finish();
    

    let mut out = String::new();

    pretty_disassembler(&mut out, 64, result.data(), 0x0);

    println!("{}", out);
}
