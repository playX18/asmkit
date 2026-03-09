use asmkit::core::jit_allocator::{JitAllocator, JitAllocatorOptions};

use asmkit::masm::x86::MacroAssemblerX86;

use asmkit::x86::formatter::pretty_disassembler;
use asmkit::x86::*;

fn main() {
    let mut cbuf = asmkit::core::buffer::CodeBuffer::new();
    let asm = Assembler::new(&mut cbuf);
    let mut asm = MacroAssemblerX86::new(asm);

    asm.convert_uint64_to_double(XMM0, RDI, RDI);
    asm.ret();

    let code = cbuf.finish();
    let mut jit_allocator = JitAllocator::new(JitAllocatorOptions {
        use_multiple_pools: false,
        granularity: 16,
        ..Default::default()
    });
    let span = code.allocate(&mut jit_allocator).unwrap();

    let func: extern "C" fn(u64) -> f64 = unsafe { std::mem::transmute(span.rx()) };

    let mut out = String::new();
    pretty_disassembler(
        &mut out,
        64,
        unsafe { std::slice::from_raw_parts(span.rx(), span.size()) },
        span.rx() as u64,
    )
    .unwrap();

    println!("Disassembly:\n{out}");

    let result = func(1 << 56);
    println!("Result of adding 1 and 2: {result}");
}
