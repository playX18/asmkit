use asmkit::core::buffer::{CodeBuffer, ExternalName, RelocDistance};
use asmkit::core::jit_allocator::JitAllocator;

use capstone::prelude::*;
unsafe extern "C" {
    fn puts(_: *const i8);
}

fn main() {
    {
        use asmkit::x86::*;

        let mut buf = CodeBuffer::new();
        let mut asm = Assembler::new(&mut buf);

        let str_constant = asm.add_constant("Hello, World!\0");
        let puts_sym = asm
            .buffer
            .add_symbol(ExternalName::Symbol("puts".into()), RelocDistance::Far);

        asm.lea(RDI, ptr64_label(str_constant, 0));
        asm.call(ptr64_sym(puts_sym, 0));
        asm.ret();
        let end = asm.get_label();
        asm.bind_label(end);
        let off = asm.buffer.label_offset(end);

        let result = buf.finish();

        for reloc in result.relocs() {
            println!("{:?}", reloc);
        }

        let mut jit = JitAllocator::new(Default::default());
        let loaded = result
            .allocate_relocated(&mut jit, |_| puts as *const u8, |_| puts as *const u8)
            .unwrap();
        let span = loaded.span();

        unsafe {
            let cs = Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .build()
                .unwrap();

            let insns = cs
                .disasm_all(
                    std::slice::from_raw_parts(span.rx(), off as usize),
                    span.rx() as u64,
                )
                .unwrap();

            for i in insns.iter() {
                println!(
                    "0x{:x}:\t{}\t{}",
                    i.address(),
                    i.mnemonic().unwrap(),
                    i.op_str().unwrap()
                );
            }

            #[cfg(target_arch = "x86_64")]
            {
                let f: extern "C" fn() = std::mem::transmute(span.rx());

                f();
            }
        }
    }
    #[cfg(target_arch = "riscv64")]
    {
        use asmkit::riscv::*;
        use formatter::pretty_disassembler;

        let mut buf = CodeBuffer::new();
        buf.env_mut().set_pic(false);
        let mut asm = Assembler::new(&mut buf);

        let str_constant = asm.add_constant("Hello, World!\0");
        let puts_sym = asm
            .buffer
            .add_symbol(ExternalName::Symbol("puts".into()), RelocDistance::Far);
        asm.addi(SP, SP, imm(-16));
        asm.sd(SP, RA, imm(8));
        asm.sd(SP, S0, imm(0));
        asm.la(A0, str_constant);
        asm.la(A1, puts_sym);
        asm.call(A1);
        asm.ld(RA, SP, imm(8));
        asm.ld(S0, SP, imm(0));
        asm.addi(SP, SP, imm(16));
        asm.ret();
        let end = asm.get_label();
        asm.bind_label(end);
        let off = asm.buffer.label_offset(end);

        let result = buf.finish();

        for reloc in result.relocs() {
            println!("{:?}", reloc);
        }

        let mut jit = JitAllocator::new(Default::default());
        let loaded = result
            .allocate_relocated(&mut jit, |_| puts as *const u8, |_| puts as *const u8)
            .unwrap();
        let span = loaded.span();

        unsafe {
            let mut out = String::new();
            pretty_disassembler(
                &mut out,
                64,
                std::slice::from_raw_parts(span.rx(), off as usize),
                span.rx() as _,
            )
            .unwrap();

            println!("{}", out);

            #[cfg(target_arch = "riscv64")]
            {
                let f: extern "C" fn() = std::mem::transmute(span.rx());

                f();
            }
        }
    }

    {
        use asmkit::aarch64::*;
        use capstone::prelude::*;

        let mut buf = CodeBuffer::new();
        let mut asm = Assembler::new(&mut buf);

        let str_constant = asm.buffer.add_constant("Hello, World!\0");
        let puts_sym = asm
            .buffer
            .add_symbol(ExternalName::Symbol("puts".into()), RelocDistance::Far);
        asm.load_constant(x0, str_constant);
        asm.load_constant(x1, puts_sym);
        asm.blr(x1);
        asm.ret(lr);

        let end = asm.buffer.get_label();
        asm.buffer.bind_label(end);
        let off = asm.buffer.label_offset(end);

        let result = buf.finish();

        for reloc in result.relocs() {
            println!("{:?}", reloc);
        }

        println!("puts at {:p}", puts as *const u8);

        let mut jit = JitAllocator::new(Default::default());

        let loaded = result
            .allocate_relocated(&mut jit, |_| puts as *const u8, |_| puts as *const u8)
            .unwrap();
        let span = loaded.span();

        unsafe {
            let cs = Capstone::new()
                .arm64()
                .mode(arch::arm64::ArchMode::Arm)
                .build()
                .unwrap();

            let insns = cs
                .disasm_all(
                    std::slice::from_raw_parts(span.rx(), off as usize),
                    span.rx() as u64,
                )
                .unwrap();

            for i in insns.iter() {
                println!(
                    "0x{:x}:\t{}\t{}",
                    i.address(),
                    i.mnemonic().unwrap(),
                    i.op_str().unwrap()
                );
            }
            #[cfg(target_arch = "aarch64")]
            {
                let f: extern "C" fn() = std::mem::transmute(span.rx());

                f();
            }
        }
    }
}
