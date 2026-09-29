//! Post-finalization patching: retarget a jump, rewrite an immediate, and
//! fill a reserved region.
//!
//! Run with: `cargo run --example patch --features jit`

use asmkit::{
    Arch, CodeBuffer, Environment, JitAllocator, read_value, repatch_jump, repatch_jump_span,
    repatch_value, repatch_value_span, rewrite_region, rewrite_region_span,
};

fn main() {
    use asmkit::x86::*;

    let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
    let (jump, imm, custom, fast) = {
        let mut asm = Assembler::new(&mut buf);

        let slow = asm.get_label();
        let fast = asm.get_label();

        // Patchable mov-imm: the mark covers the immediate bytes only.
        let imm = asm.patchable_mov(RAX, imm(0));

        // Patchable near jump (rel32).
        let jump = asm.patchable_jmp(slow);

        // Nop region for a later rewrite.
        let custom = asm.reserve_patch_region(4, 1).expect("reserve_patch_region");

        asm.bind_label(slow);
        asm.ret();

        asm.bind_label(fast);
        // After retargeting `jump` here, execution falls into this path.
        asm.add(RAX, 1);
        asm.ret();

        (jump, imm, custom, fast)
    };

    let code = buf.finish().expect("finish");
    // Locate the marks in the finished image.
    let (jump, imm, custom) = (
        code.location_of(jump),
        code.location_of(imm),
        code.location_of(custom),
    );
    let fast = code.label_offset(fast);
    println!("assembled {} bytes", code.data().len());

    // Offline patching (no JIT): mutate a copy of the image.
    let mut offline = code.data().to_vec();
    repatch_jump(&mut offline, jump, fast).unwrap();
    repatch_value(&mut offline, imm, 41).unwrap();
    rewrite_region(&mut offline, custom, &[0x90, 0x90, 0x90, 0x90]).unwrap();
    assert_eq!(read_value(&offline, imm).unwrap(), 41);
    println!("offline retarget/rewrite ok");

    let mut jit = JitAllocator::new(Default::default());
    let mut span = code.allocate(&mut jit).expect("allocate");
    // SAFETY: `span` holds `code`, and nothing is executing it yet.
    unsafe {
        let target = span.rx().add(fast as usize);
        repatch_jump_span(&mut jit, &mut span, jump, target).unwrap();
        repatch_value_span(&mut jit, &mut span, imm, 41).unwrap();
        // One-byte NOP into the region (padded with more nops).
        rewrite_region_span(&mut jit, &mut span, custom, &[0x90]).unwrap();
    }

    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: the image starts with an `extern "C" fn() -> u64`.
        let f: extern "C" fn() -> u64 = unsafe { std::mem::transmute(span.rx()) };
        let result = f();
        println!("executed patched code → {result}");
        assert_eq!(result, 42);
    }
}
