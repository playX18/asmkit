use asmkit::{
    aarch64::*,
    core::{
        buffer::CodeBuffer,
        operand::*,
        operand::{BaseReg, OperandSignature, OperandType},
    },
};

use capstone::prelude::*;

fn main() {
    let mut buf = CodeBuffer::new();

    let r = |id: u32| {
        BaseReg::from_signature_and_id(OperandSignature::from_op_type(OperandType::Reg), id)
    };

    let mut asm = Assembler::new(&mut buf);

    asm.addw(r(0), r(1), r(2));
    asm.faddd(r(0), r(1), r(4));
    let cs = Capstone::new()
        .arm64()
        .mode(arch::arm64::ArchMode::Arm)
        .endian(capstone::Endian::Little)
        .build().unwrap();

    let insns = cs.disasm_all(&buf.data(), 0x0).unwrap();

    for i in insns.iter() {
        println!("{}", i);
    }
}
