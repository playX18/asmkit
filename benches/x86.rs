use std::hint::black_box;
use asmkit::core::buffer::CodeBuffer;
use criterion::{criterion_group, criterion_main, Criterion};


fn emit_factorial_benchmark(c: &mut Criterion) {
   
    c.bench_function("asmkit", |b| b.iter_with_large_drop(|| {
        use asmkit::x86::*;
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

        result
    }));


}

criterion_group!(benches, emit_factorial_benchmark);
criterion_main!(benches);