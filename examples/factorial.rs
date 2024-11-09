use capstone::prelude::*;
use uasm::core::jit_allocator::{JitAllocator, JitAllocatorOptions};

fn main() {
    {
        use uasm::core::buffer::CodeBuffer;
        use uasm::x86::*;
        let mut buf = CodeBuffer::new();
        let mut asm = Assembler::new(&mut buf);

        let label = asm.get_label();
        let fac = asm.get_label();

        asm.bind_label(fac);
        asm.mov64ri(RAX, imm(1));
        asm.test64rr(RDI, RDI);
        asm.jnz(label);
        asm.ret();

        {
            asm.bind_label(label);
            asm.pushr(RBX);
            asm.mov64rr(RBX, RDI);
            asm.lea64rm(RDI, ptr64(RDI, -1));
            asm.call(fac);
            asm.mov64rr(RDX, RAX);
            asm.mov64rr(RAX, RBX);
            asm.imul64rr(RAX, RDX);
            asm.popr(RBX);
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
                .syntax(arch::x86::ArchSyntax::Intel)
                .detail(true)
                .build()
                .unwrap();

            for i in cs.disasm_all(result.data(), span.rx() as _).unwrap().iter() {
                println!("{}", i);
            }
            #[cfg(target_arch = "x86_64")]
            {
                let f: extern "C" fn(u64) -> u64 = std::mem::transmute(span.rx());

                println!("X86 factorial(5) = {:?}", f(5));
            }
        }
    }

    {
        use uasm::core::buffer::CodeBuffer;
        use uasm::riscv::*;
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
                .detail(true)
                .build()
                .unwrap();

            for i in cs.disasm_all(result.data(), span.rx() as _).unwrap().iter() {
                println!("{}", i);
            }
            #[cfg(target_arch = "riscv64")]
            {
                let f: extern "C" fn(u64) -> u64 = std::mem::transmute(span.rx());

                println!("RV64 factorial(5) = {:?}", f(5));
            }
        }
    }
}