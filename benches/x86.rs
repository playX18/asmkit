use asmkit::core::buffer::CodeBuffer;
use criterion::{Criterion, criterion_group, criterion_main};

fn emit_factorial_benchmark(c: &mut Criterion) {
    c.bench_function("asmkit::x86", |b| {
        let mut buf = CodeBuffer::new();
        b.iter_with_large_drop(|| {
            use asmkit::x86::*;

            let mut asm = Assembler::new(&mut buf);

            let label = asm.get_label();
            let fac = asm.get_label();

            asm.bind_label(fac);
            asm.mov(RAX, imm(1));
            asm.test(RDI, RDI);
            asm.jnz(label);
            asm.ret();

            {
                asm.bind_label(label);
                asm.push(RBX);
                asm.mov(RBX, RDI);
                asm.lea(RDI, ptr64(RDI, -1));
                asm.call(fac);
                asm.mov(RDX, RAX);
                asm.mov(RAX, RBX);
                asm.imul_2(RAX, RDX);
                asm.pop(RBX);
                asm.ret();
            }

            let result = buf.finish();
            buf.clear();
            result
        })
    });

    c.bench_function("iced-x86", |b| {
        b.iter_with_large_drop(|| {
            use iced_x86::code_asm::*;
            let mut asm = CodeAssembler::new(64).unwrap();

            let mut label = asm.create_label();
            let mut fac = asm.create_label();

            asm.set_label(&mut fac).unwrap();
            asm.mov(rax, 1i64).unwrap();
            asm.test(rdi, rdi).unwrap();
            asm.jnz(label).unwrap();
            asm.ret().unwrap();

            {
                asm.set_label(&mut label).unwrap();
                asm.push(rbx).unwrap();
                asm.mov(rbx, rdi).unwrap();
                asm.lea(rdi, ptr(rdi) - 1).unwrap();
                asm.call(fac).unwrap();
                asm.mov(rdx, rax).unwrap();
                asm.mov(rax, rbx).unwrap();
                asm.imul_2(rax, rdx).unwrap();
                asm.pop(rbx).unwrap();
                asm.ret().unwrap();
            }

            let result = asm.assemble(0x1000).unwrap();

            result
        })
    });
}

criterion_group!(benches, emit_factorial_benchmark);
criterion_main!(benches);
