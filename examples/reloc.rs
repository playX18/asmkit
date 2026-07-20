use asmkit::Linker;
use asmkit::{Arch, Environment, JitAllocator};
use asmkit::{CodeBuffer, RelocDistance};

use capstone::prelude::*;
unsafe extern "C" {
    fn puts(_: *const i8);
}

/// Resolves external symbols by name at load time.
fn resolve(name: &str) -> *const u8 {
    match name {
        "puts" => puts as *const u8,
        _ => std::ptr::null(),
    }
}

fn main() {
    #[cfg(unix)]
    {
        use asmkit::x86::*;

        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let puts_sym = buf.extern_sym("puts", RelocDistance::Far);

        let (entry, code_len) = {
            let mut asm = Assembler::new(&mut buf);
            let str_constant = asm.add_constant("Hello, World!\0");
            let entry = asm.get_label();
            asm.bind_label(entry);
            asm.lea(RDI, ptr64_label(str_constant, 0));
            asm.call(ptr64_sym(puts_sym, 0));
            asm.ret();
            let end = asm.get_label();
            asm.bind_label(end);
            (entry, asm.label_offset(end))
        };
        // Export the entry point so it can be looked up by name after linking.
        buf.bind_symbol("main", entry);

        // Link the module (only one here; linking is what resolves `main`).
        let mut linker = Linker::new();
        linker.add_buffer(buf.finish().unwrap());
        let image = linker.link().unwrap();
        let entry_offset = image.defined_symbol_offset("main").unwrap();

        let mut jit = JitAllocator::new(Default::default());
        let loaded = image.allocate_resolved(&mut jit, resolve).unwrap();
        let span = loaded.span();

        unsafe {
            let cs = Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .build()
                .unwrap();

            let insns = cs
                .disasm_all(
                    std::slice::from_raw_parts(span.rx(), code_len as usize),
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
                let f: extern "C" fn() = std::mem::transmute(span.rx().add(entry_offset as usize));

                f();
            }
        }
    }
    #[cfg(windows)]
    {
        use asmkit::x86::*;

        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let puts_sym = buf.extern_sym("puts", RelocDistance::Far);

        let (entry, code_len) = {
            let mut asm = Assembler::new(&mut buf);
            let str_constant = asm.add_constant("Hello, World!\0");
            let entry = asm.get_label();
            asm.bind_label(entry);
            // Win64: first arg in rcx, 32-byte shadow space before the call.
            asm.sub(RSP, imm(40));
            asm.lea(RCX, ptr64_label(str_constant, 0));
            asm.call(ptr64_sym(puts_sym, 0));
            asm.add(RSP, imm(40));
            asm.ret();
            let end = asm.get_label();
            asm.bind_label(end);
            (entry, asm.label_offset(end))
        };
        buf.bind_symbol("main", entry);

        let mut linker = Linker::new();
        linker.add_buffer(buf.finish().unwrap());
        let image = linker.link().unwrap();
        let entry_offset = image.defined_symbol_offset("main").unwrap();

        let mut jit = JitAllocator::new(Default::default());
        let loaded = image.allocate_resolved(&mut jit, resolve).unwrap();
        let span = loaded.span();

        unsafe {
            let cs = Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .build()
                .unwrap();

            let insns = cs
                .disasm_all(
                    std::slice::from_raw_parts(span.rx(), code_len as usize),
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
                let f: extern "C" fn() = std::mem::transmute(span.rx().add(entry_offset as usize));

                f();
            }
        }
    }
    #[cfg(target_arch = "riscv64")]
    {
        use asmkit::riscv::*;
        use formatter::pretty_disassembler;

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let puts_sym = buf.extern_sym("puts", RelocDistance::Far);

        let end = {
            let mut asm = Assembler::new(&mut buf);
            let str_constant = asm.add_constant("Hello, World!\0");
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
            end
        };
        let off = buf.label_offset(end);

        let result = buf.finish().unwrap();

        let mut jit = JitAllocator::new(Default::default());
        let loaded = result.allocate_resolved(&mut jit, resolve).unwrap();
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

            let f: extern "C" fn() = std::mem::transmute(span.rx());

            f();
        }
    }

    {
        use asmkit::aarch64::*;

        let mut buf = CodeBuffer::new(Environment::new(Arch::AArch64));
        let str_constant = buf.add_constant("Hello, World!\0");
        let puts_sym = buf.extern_sym("puts", RelocDistance::Far);

        let end = {
            let mut asm = Assembler::new(&mut buf);
            asm.load_constant(x0, str_constant);
            asm.load_constant(x1, puts_sym);
            asm.blr(x1);
            asm.ret(lr);

            let end = asm.get_label();
            asm.bind_label(end);
            end
        };
        let off = buf.label_offset(end);

        let result = buf.finish().unwrap();

        println!("puts at {:p}", puts as *const u8);

        let mut jit = JitAllocator::new(Default::default());

        let loaded = result.allocate_resolved(&mut jit, resolve).unwrap();
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
