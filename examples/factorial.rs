use asmkit::JitAllocatorOptions;
use asmkit::{Arch, Environment, JitAllocator};
use capstone::prelude::*;

// Recursive factorial written directly with each backend's Assembler. On a
// matching host the code is also executed through the JIT.
fn main() {
    println!("X86:");
    #[cfg(unix)]
    {
        use asmkit::CodeBuffer;
        use asmkit::CondCode;
        use asmkit::x86::*;

        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let mut asm = Assembler::new(&mut buf);

        let fac = asm.get_label();
        let rec = asm.get_label();

        // fn fac(n: u64) -> u64 (System V: n in rdi, result in rax).
        asm.bind_label(fac);
        asm.cmp(RDI, imm(1));
        asm.j(CondCode::GT, rec);
        asm.mov(RAX, imm(1));
        asm.ret();
        {
            asm.bind_label(rec);
            // Save rbx (n); the single push also aligns the stack for the call.
            asm.push(RBX);
            asm.mov(RBX, RDI);
            asm.sub(RDI, imm(1));
            asm.call(fac);
            asm.imul(RAX, RBX);
            asm.pop(RBX);
            asm.ret();
        }
        let result = buf.finish().unwrap();

        let mut jit = JitAllocator::new(JitAllocatorOptions::default());

        let mut span = jit
            .alloc(result.data().len())
            .expect("failed to allocate code");
        unsafe {
            jit.write(&mut span, |span| {
                span.rw()
                    .copy_from_nonoverlapping(result.data().as_ptr(), result.data().len());
            })
            .unwrap();

            let cs = Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .build()
                .unwrap();

            let insns = cs
                .disasm_all(
                    std::slice::from_raw_parts(span.rx(), result.data().len()),
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
                let f: extern "C" fn(u64) -> u64 = std::mem::transmute(span.rx());

                println!("X86 factorial(5) = {:}", f(5));
            }
        }
    }
    #[cfg(windows)]
    {
        use asmkit::CodeBuffer;
        use asmkit::CondCode;
        use asmkit::x86::*;

        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let mut asm = Assembler::new(&mut buf);

        let fac = asm.get_label();
        let rec = asm.get_label();

        // fn fac(n: u64) -> u64 (Win64: n in rcx, result in rax).
        asm.bind_label(fac);
        asm.cmp(RCX, imm(1));
        asm.j(CondCode::GT, rec);
        asm.mov(RAX, imm(1));
        asm.ret();
        {
            asm.bind_label(rec);
            // Save rbx (n), then allocate the 32-byte Win64 shadow space.
            asm.push(RBX);
            asm.sub(RSP, imm(32));
            asm.mov(RBX, RCX);
            asm.sub(RCX, imm(1));
            asm.call(fac);
            asm.imul(RAX, RBX);
            asm.add(RSP, imm(32));
            asm.pop(RBX);
            asm.ret();
        }
        let result = buf.finish().unwrap();

        let mut jit = JitAllocator::new(JitAllocatorOptions::default());

        let mut span = jit
            .alloc(result.data().len())
            .expect("failed to allocate code");
        unsafe {
            jit.write(&mut span, |span| {
                span.rw()
                    .copy_from_nonoverlapping(result.data().as_ptr(), result.data().len());
            })
            .unwrap();

            let cs = Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .build()
                .unwrap();

            let insns = cs
                .disasm_all(
                    std::slice::from_raw_parts(span.rx(), result.data().len()),
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
                let f: extern "C" fn(u64) -> u64 = std::mem::transmute(span.rx());

                println!("X86 factorial(5) = {:}", f(5));
            }
        }
    }

    println!("RISC-V:");
    {
        use asmkit::CodeBuffer;
        use asmkit::riscv::*;

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);

        let fac = asm.get_label();
        let rec = asm.get_label();

        // fn fac(n: u64) -> u64 (psABI: n in a0, result in a0).
        asm.bind_label(fac);
        asm.addi(T0, ZERO, imm(1));
        asm.blt(T0, A0, rec);
        asm.addi(A0, ZERO, imm(1));
        asm.ret();
        {
            asm.bind_label(rec);
            // Frame: save ra and s1 (n).
            asm.addi(SP, SP, imm(-16));
            asm.sd(SP, RA, imm(8));
            asm.sd(SP, S1, imm(0));
            asm.mv(S1, A0);
            asm.addi(A0, A0, imm(-1));
            asm.call(fac);
            asm.mul(A0, S1, A0);
            asm.ld(RA, SP, imm(8));
            asm.ld(S1, SP, imm(0));
            asm.addi(SP, SP, imm(16));
            asm.ret();
        }
        let result = buf.finish().unwrap();

        let mut jit = JitAllocator::new(JitAllocatorOptions::default());

        let mut span = jit
            .alloc(result.data().len())
            .expect("failed to allocate code");
        unsafe {
            jit.write(&mut span, |span| {
                span.rw()
                    .copy_from_nonoverlapping(result.data().as_ptr(), result.data().len());
            })
            .unwrap();

            let cs = Capstone::new()
                .riscv()
                .mode(arch::riscv::ArchMode::RiscV64)
                .build()
                .unwrap();

            let insns = cs
                .disasm_all(
                    std::slice::from_raw_parts(span.rx(), result.data().len()),
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

            #[cfg(target_arch = "riscv64")]
            {
                let f: extern "C" fn(u64) -> u64 = std::mem::transmute(span.rx());

                println!("RV64 factorial(5) = {:}", f(5));
            }
        }
    }

    println!("Aarch64:");
    {
        use asmkit::CodeBuffer;
        use asmkit::CondCode;
        use asmkit::OperandCast;
        use asmkit::aarch64::*;

        let mut buf = CodeBuffer::new(Environment::new(Arch::AArch64));
        let mut asm = Assembler::new(&mut buf);

        let fac = asm.get_label();
        let rec = asm.get_label();

        // fn fac(n: u32) -> u32 (AAPCS64: n in w0, result in w0).
        asm.bind_label(fac);
        asm.cmp(w0, imm(1));
        asm.emit_n(InstId::B.with_cc(CondCode::GT), &[rec.as_operand()]);
        asm.mov(w0, imm(1));
        asm.ret(lr);
        {
            asm.bind_label(rec);
            // Frame: x29/x30 pair, plus a slot for x19 (n).
            asm.stp(x29, x30, ptr(sp, 0).pre_offset(-16));
            asm.mov(x29, sp);
            asm.sub(sp, sp, imm(16));
            asm.str(x19, ptr(sp, 8));
            asm.mov(w19, w0);
            asm.sub(w0, w0, imm(1));
            asm.bl(fac);
            asm.mul(w0, w0, w19);
            asm.ldr(x19, ptr(sp, 8));
            asm.add(sp, sp, imm(16));
            asm.ldp(x29, x30, ptr(sp, 0).post_offset(16));
            asm.ret(lr);
        }
        let result = buf.finish().unwrap();
        let mut jit = JitAllocator::new(JitAllocatorOptions::default());
        let mut span = jit
            .alloc(result.data().len())
            .expect("failed to allocate code");
        unsafe {
            jit.write(&mut span, |span| {
                span.rw()
                    .copy_from_nonoverlapping(result.data().as_ptr(), result.data().len());
            })
            .unwrap();

            let cs = Capstone::new()
                .arm64()
                .mode(arch::arm64::ArchMode::Arm)
                .build()
                .unwrap();

            let insns = cs
                .disasm_all(
                    std::slice::from_raw_parts(span.rx(), result.data().len()),
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
                let f: extern "C" fn(u64) -> u64 = std::mem::transmute(span.rx());

                println!("AArch64 factorial(5) = {:}", f(5));
            }
        }
    }
}
