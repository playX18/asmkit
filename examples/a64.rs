use asmkit::{
    aarch64::{assembler::*, operands::*},
    core::buffer::{CodeBuffer, ConstantData},
};
use capstone::arch::BuildsCapstone;

fn main() {
    let mut buf = CodeBuffer::new();
    let mut asm = Assembler::new(&mut buf);
    let big = asm
        .buffer
        .add_constant(ConstantData::Bytes(vec![0x0; 8192 * 4]));
    let cbig = asm.buffer.get_label_for_constant(big);
    let c = asm
        .buffer
        .add_constant(ConstantData::U64(42.42f64.to_bits().to_le_bytes()));

    let clabel = asm.buffer.get_label_for_constant(c);
    let end = asm.buffer.get_label();
    asm.adrp(x1, cbig);
    asm.load_constant(x2, clabel);
    asm.buffer.bind_label(end);
    let off = asm.buffer.label_offset(end);
    if let Some(err) = asm.last_error() {
        println!("{err:?}");
    }

    let code = buf.finish();

    let cs = capstone::Capstone::new()
        .arm64()
        .mode(capstone::arch::arm64::ArchMode::Arm)
        .build()
        .unwrap();

    let insns = cs.disasm_all(&code.data()[..off as usize], 0x1000).unwrap();

    for i in insns.iter() {
        println!(
            "0x{:x}:\t{}\t{}",
            i.address(),
            i.mnemonic().unwrap(),
            i.op_str().unwrap()
        );
    }
}
