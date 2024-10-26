use uasm::encdec::riscv::formatter::Formatter;
use uasm::assembler::codeholder::CodeBuffer;
use uasm::encdec::riscv::decode::*;
use uasm::assembler::riscv::*;
fn main() {
    let mut buf = CodeBuffer::new();

    let mut asm = Assembler::new(&mut buf);

    asm.addi(A0, A1, 42).unwrap();
    asm.add(A0, A3, A2).unwrap();
    asm.fcvt_d_w(FA0, A0, X0).unwrap();
    asm.ret().unwrap();  

    let result = buf.finish();

    let mut decoder = Decoder::new(64, result.data() , result.data().as_ptr() as _);
    let mut inst = Instruction::default();
    let formatter = Formatter::new();
    while decoder.can_decode() {
        decoder.decode_out(&mut inst);

        let mut out = String::new();

        formatter.format(&mut out, &inst);

        println!("{:x} {:x}\t {}",inst.address, inst.value.value, out);
    }
}
