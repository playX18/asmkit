//! JSC-style post-emit patching: retarget a jump, rewrite a constant, and fill a custom block.
//!
//! Run with: `cargo run --example patch --features jit`

use asmkit::{Arch, CodeBuffer, Environment, JitAllocator};

fn main() {
    use asmkit::x86::*;

    let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
    let (jump, imm, custom, slow_off, fast_off) = {
        let mut asm = Assembler::new(&mut buf);

        let slow = asm.get_label();
        let fast = asm.get_label();

        // Patchable mov-imm (block covers the immediate bytes only).
        let imm = asm.patchable_mov(RAX, imm(0));

        // Patchable near jump (rel32 site).
        let jump = asm.patchable_jmp(slow);

        // Custom nop island for later rewrite.
        let custom = asm.reserve_patch_block(4, 1).expect("reserve_patch_block");

        asm.bind_label(slow);
        asm.ret();
        let slow_off = asm.label_offset(slow);

        asm.bind_label(fast);
        // After retargeting `jump` here, execution falls into this path.
        asm.add(RAX, 1);
        asm.ret();
        let fast_off = asm.label_offset(fast);

        (jump, imm, custom, slow_off, fast_off)
    };

    let _ = slow_off;
    let code = buf.finish_patched().expect("finish_patched");
    println!(
        "assembled {} bytes; {} patch site(s), {} patch block(s)",
        code.data().len(),
        code.patch_catalog().sites().len(),
        code.patch_catalog().blocks().len()
    );

    // Offline patching (no JIT): mutate a copy of the image.
    let mut offline = code.data().to_vec();
    unsafe {
        jump.retarget(&mut offline, fast_off).unwrap();
        imm.repatch_u64(&mut offline, 41).unwrap();
        custom
            .rewrite(&mut offline, &[0x90, 0x90, 0x90, 0x90])
            .unwrap();
    }
    println!("offline retarget/rewrite ok");

    #[cfg(feature = "jit")]
    {
        let mut jit = JitAllocator::new(Default::default());
        let mut span = code.allocate(&mut jit).expect("allocate");
        unsafe {
            jump.retarget_span(&mut jit, &mut span, fast_off).unwrap();
            imm.repatch_u64_span(&mut jit, &mut span, 41).unwrap();
            // One-byte NOP into the custom island (padded with more nops).
            custom.rewrite_span(&mut jit, &mut span, &[0x90]).unwrap();
        }

        #[cfg(target_arch = "x86_64")]
        {
            let f: extern "C" fn() -> u64 = unsafe { std::mem::transmute(span.rx()) };
            let result = f();
            println!("executed patched code → {result}");
            assert_eq!(result, 42);
        }
    }
}
