use asmkit::{Arch, CodeBuffer, ConstantData, Environment, aarch64::*};
use capstone::arch::BuildsCapstone;

fn main() {
    let mut buf = CodeBuffer::new(Environment::new(Arch::AArch64));
    let big = buf.add_constant(ConstantData::Bytes(vec![0x0; 8192 * 4]));
    let cbig = buf.get_label_for_constant(big);
    let c = buf.add_constant(ConstantData::U64(42.42f64.to_bits().to_le_bytes()));
    let clabel = buf.get_label_for_constant(c);

    let mut asm = Assembler::new(&mut buf);
    let end = asm.get_label();
    asm.adrp(x1, cbig);
    asm.load_constant(x2, clabel);
    asm.bind_label(end);
    let off = buf.label_offset(end);

    let code = buf.finish().expect("assembly failed");

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
