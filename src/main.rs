use asmkit::x86::*;
use capstone::prelude::*;
fn main() {
    let mut cbuf = asmkit::core::buffer::CodeBuffer::new();
    let mut asm = Assembler::new(&mut cbuf);

    asm.ret();

    let code = cbuf.finish();

    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .build()
        .expect("Failed to create Capstone object");
    let insns = cs
        .disasm_all(&code.data(), 0x1000)
        .expect("Failed to disassemble code");

    for i in insns.iter() {
        println!(
            "0x{:x}: {}\t{}",
            i.address(),
            i.mnemonic().unwrap_or(""),
            i.op_str().unwrap_or("")
        );
    }
}
