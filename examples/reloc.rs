use asmkit::Linker;
use asmkit::{Arch, Environment, ExternalName, JitAllocator};
use asmkit::{CodeBuffer, RelocDistance};

use capstone::prelude::*;
unsafe extern "C" {
    fn puts(_: *const i8);
}

/// Resolves external symbols at load time by [`ExternalName`].
fn resolve(name: &ExternalName) -> *const u8 {
    match name {
        ExternalName::Symbol(s) if s == "puts" => puts as *const u8,
        _ => std::ptr::null(),
    }
}

fn disassemble(cs: &Capstone, bytes: &[u8], address: u64, code_end: usize, unit: usize) {
    let mut offset = 0;
    while offset < code_end {
        let insns = cs.disasm_count(&bytes[offset..code_end], address + offset as u64, 1);
        if let Some(insn) = insns.as_ref().ok().and_then(|insns| insns.iter().next()) {
            println!(
                "0x{:x}:\t{}\t{}",
                insn.address(),
                insn.mnemonic().unwrap_or(""),
                insn.op_str().unwrap_or("")
            );
            offset += insn.len();
        } else {
            let chunk = &bytes[offset..code_end.min(offset + unit)];
            print_data(address + offset as u64, chunk);
            offset += chunk.len();
        }
    }

    for chunk in bytes[code_end..].chunks(16) {
        print_data(address + offset as u64, chunk);
        offset += chunk.len();
    }
}

fn print_data(address: u64, chunk: &[u8]) {
    let hex: Vec<String> = chunk.iter().map(|b| format!("0x{b:02x}")).collect();
    let ascii: String = chunk
        .iter()
        .map(|&b| {
            if b.is_ascii_graphic() || b == b' ' {
                b as char
            } else {
                '.'
            }
        })
        .collect();
    println!("0x{address:x}:\t.byte\t{}\t; {ascii}", hex.join(", "));
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
        let mut jit = JitAllocator::new(Default::default());
        let code = image.load(&mut jit, resolve).unwrap();

        unsafe {
            let cs = Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .build()
                .unwrap();

            disassemble(
                &cs,
                std::slice::from_raw_parts(code.rx(), code.code_size()),
                code.rx() as u64,
                code_len as usize,
                1,
            );

            #[cfg(target_arch = "x86_64")]
            {
                let f: extern "C" fn() = std::mem::transmute(code.symbol("main").unwrap());

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
        let mut jit = JitAllocator::new(Default::default());
        let code = image.load(&mut jit, resolve).unwrap();

        unsafe {
            let cs = Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .build()
                .unwrap();

            disassemble(
                &cs,
                std::slice::from_raw_parts(code.rx(), code.code_size()),
                code.rx() as u64,
                code_len as usize,
                1,
            );

            #[cfg(target_arch = "x86_64")]
            {
                let f: extern "C" fn() = std::mem::transmute(code.symbol("main").unwrap());

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
        let code = result.load(&mut jit, resolve).unwrap();

        unsafe {
            let mut out = String::new();
            pretty_disassembler(
                &mut out,
                64,
                std::slice::from_raw_parts(code.rx(), off as usize),
                code.rx() as _,
            )
            .unwrap();

            println!("{}", out);

            let f: extern "C" fn() = std::mem::transmute(code.rx());

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
            // `blr` overwrites lr, so save the caller's frame record first.
            asm.stp(x29, x30, ptr(sp, 0).pre_offset(-16));
            asm.mov(x29, sp);
            asm.load_constant(x0, str_constant);
            asm.load_constant(x1, puts_sym);
            asm.blr(x1);
            asm.ldp(x29, x30, ptr(sp, 0).post_offset(16));
            asm.ret(lr);

            let end = asm.get_label();
            asm.bind_label(end);
            end
        };
        let off = buf.label_offset(end);

        let result = buf.finish().unwrap();

        println!("puts at {:p}", puts as *const u8);

        let mut jit = JitAllocator::new(Default::default());

        let code = result.load(&mut jit, resolve).unwrap();

        unsafe {
            let cs = Capstone::new()
                .arm64()
                .mode(arch::arm64::ArchMode::Arm)
                .build()
                .unwrap();

            disassemble(
                &cs,
                std::slice::from_raw_parts(code.rx(), code.code_size()),
                code.rx() as u64,
                off as usize,
                4,
            );
            #[cfg(target_arch = "aarch64")]
            {
                let f: extern "C" fn() = std::mem::transmute(code.rx());

                f();
            }
        }
    }
}
