use asmkit::core::jit_allocator::{JitAllocator, JitAllocatorOptions};
use capstone::prelude::*;
fn main() {
    {
        use asmkit::core::buffer::CodeBuffer;
        use asmkit::x86::*;

        let mut buf = CodeBuffer::new();
        let mut asm = Assembler::new(&mut buf);

        let fac = asm.get_label();

        asm.bind_label(fac);
        asm.mov(RAX, imm(1));
        asm.test(RDI, RDI);
        let label = asm.get_label();
        asm.jnz(label);

        asm.ret();

        {
            asm.bind_label(label);
            asm.push(RBX);
            asm.mov(RBX, RDI);
            asm.sub(RDI, imm(1));
            asm.call(fac);
            asm.mov(RDX, RAX);
            asm.mov(RAX, RBX);
            asm.imul_2(RAX, RDX);
            asm.pop(RBX);
            asm.ret();
        }

        let result = buf.finish();

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

    {
        use asmkit::core::buffer::CodeBuffer;
        use asmkit::riscv::*;

        let mut buf = CodeBuffer::new();
        let mut asm = Assembler::new(&mut buf);

        let label = asm.get_label();
        let fac = asm.get_label();
        asm.bind_label(fac);
        asm.bnez(A0, label);
        asm.addi(A0, ZERO, imm(1));
        asm.ret();
        {
            asm.bind_label(label);
            asm.addi(SP, SP, imm(-16));
            asm.sd(SP, RA, imm(8));
            asm.sd(SP, S0, imm(0));
            asm.mv(S0, A0);
            asm.addi(A0, A0, imm(-1));

            asm.call(fac);
            asm.mul(A0, S0, A0);
            asm.ld(RA, SP, imm(8));
            asm.ld(S0, SP, imm(0));
            asm.addi(SP, SP, imm(16));
            asm.ret();
        }

        let result = buf.finish();

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
        use asmkit::aarch64::*;
        use asmkit::core::buffer::CodeBuffer;

        let mut buf = CodeBuffer::new();
        let mut asm = Assembler::new(&mut buf);

        let label = asm.get_label();
        let fac = asm.get_label();
        asm.bind_label(fac);
        asm.cmp(w0, imm(1));
        asm.b_le(label);
        asm.stp(x29, x30, ptr(sp, 0).pre_offset(-32));
        asm.mov(x29, sp);
        asm.str(x19, ptr(sp, 16));
        asm.mov(w19, w0);
        asm.sub(w0, w0, imm(1));
        asm.bl(fac);
        asm.mul(w0, w0, w19);
        asm.ldr(x19, ptr(sp, 16));
        asm.ldp(x29, x30, ptr(sp, 0).post_offset(32));
        asm.ret(lr);
        asm.bind_label(label);
        asm.mov(w0, imm(1));
        asm.ret(lr);

        let cs = Capstone::new()
            .arm64()
            .mode(arch::arm64::ArchMode::Arm)
            .build()
            .unwrap();
        let insns = cs.disasm_all(buf.data(), 0).unwrap();
        println!("total {:x}", buf.data().len());
        for i in insns.iter() {
            println!(
                "0x{:x}:\t{}\t{}",
                i.address(),
                i.mnemonic().unwrap(),
                i.op_str().unwrap()
            );
        }

        let result = buf.finish();
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
