use uasm::{core::buffer::CodeBuffer, riscv::*};
/// Return the length (in bytes) of an instruction given the low 16 bits of it.
///
/// The current spec reserves a bit pattern for instructions of length >= 192 bits, but for
/// simplicity this function just returns 24 in that case. The largest instructions currently
/// defined are 4 bytes so it will likely be a long time until this diffence matters.
fn instruction_length(i: u16) -> usize {
    if i & 0b11 != 0b11 {
        2
    } else if i & 0b11100 != 0b11100 {
        4
    } else if i & 0b111111 == 0b011111 {
        6
    } else if i & 0b1111111 == 0b011111 {
        8
    } else {
        10 + 2 * ((i >> 12) & 0b111) as usize
    }
}
fn main() {
    let mut buf = CodeBuffer::new();

    let mut asm = Assembler::new(&mut buf);

    let l = asm.buffer.get_label();

    asm.c_j(l);

    asm.addi(X5, X0, imm(42));
    asm.buffer.bind_label(l);
    asm.ret();

    let result = buf.finish();

    let mut i = 0;
    while i < result.data().len() {
        let short = u16::from_le_bytes([result.data()[i], result.data()[i + 1]]);

        let len = instruction_length(short);

        if len == 2 {
            println!("{:04x}", short);
        } else {
            let long = u32::from_le_bytes(result.data()[i..i + 4].try_into().unwrap());
            println!("{:08x}", long);
        }
        i += len;
    }
}
