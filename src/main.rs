use asmkit::core::jit_allocator::JitAllocator;
use asmkit::x86::formatter::pretty_disassembler;
use asmkit::x86::*;
use asmkit::AsmError;

fn main() -> Result<(), AsmError> {
    let mut cbuf = asmkit::core::buffer::CodeBuffer::new();
    let mut asm = asmkit::x86::Assembler::new(&mut cbuf);

    let dst = RDI;
    let arg0 = RSI;
    let arg1 = RDX;

    asm.sse_movdqu(XMM0, ptr64(arg0, 0))?; // load 4 ints from [arg0] to XMM0
    asm.sse_movdqu(XMM1, ptr64(arg1, 0))?; // load 4 ints from [arg1] to XMM1
    asm.sse_paddw(XMM0, XMM1)?; // add the two vectors (4 16-bit integers each) and store the result in XMM0
    asm.sse_movdqu(ptr64(dst, 0), XMM0)?; // store the result back to [dst]
    asm.ret()?;
    let buf = cbuf.finish();

    let mut out = String::new();
    pretty_disassembler(&mut out, 64, buf.data(), 0x1000).unwrap();
    println!("{}", out);
    let mut jit = JitAllocator::new(Default::default());
    let span = buf
        .allocate(&mut jit)
        .expect("Failed to allocate JIT memory");
    let f: extern "C" fn(*mut i16, *const i16, *const i16) =
        unsafe { std::mem::transmute(span.rx()) };
    let mut res = [0; 4];
    f(
        res.as_mut_ptr(),
        [128, 128, 128, 128].as_ptr(),
        [4, 3, 2, 1].as_ptr(),
    );
    println!("Result: {:?}", res);

    Ok(())
}
