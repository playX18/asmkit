use asmkit::core::buffer::CodeBuffer;
use asmkit::core::jit_allocator::JitAllocator;
use asmkit::x86::*;

fn main() {
    let mut buf = CodeBuffer::new();
    let mut asm = Assembler::new(&mut buf);

    let dst = RDI;
    let arg0 = RSI;
    let arg1 = RDX;

    asm.sse_movdqurm(XMM0, ptr64(arg0, 0)); // load 4 ints from [arg0] to XMM0
    asm.sse_movdqurm(XMM1, ptr64(arg1, 0)); // load 4 ints from [arg1] to XMM1
    asm.sse_paddwrr(XMM0, XMM1); // add 4 ints
    asm.sse_movdqumr(ptr64(dst, 0), XMM0); // store result in [dst]
    asm.ret(); // return from function

    let result = buf.finish();
    let mut jit = JitAllocator::new(Default::default());
    // you can also use jit.alloc + jit.write manually.
    let span = result
        .allocate(&mut jit)
        .expect("failed to allocate JIT-code");

    // JIT Allocator uses dual-mapping: it allocates two pages which map to same physical space
    // and you write to executable code through `span.rw()` pointer while you can execute `span.rx()`.
    let f: extern "C" fn(*mut i32, *const i32, *const i32) = unsafe { std::mem::transmute(span.rx()) };

    let mut res = [0; 4];
    f(res.as_mut_ptr(), [4, 3, 2, 1].as_ptr(), [1, 5, 2, 8].as_ptr());

    println!("{:?}", res);
}
