//! Minimal native ABI checks.
//!
//! Cross-target CI builds this test for AArch64 and RV64 and runs its test
//! binary under the matching QEMU user emulator. It intentionally uses the
//! public void mnemonic API and observes the sticky error at finalization.

#[cfg(all(feature = "jit", feature = "x86", target_arch = "x86_64"))]
#[test]
fn x86_64_returns_a_scalar() {
    use asmkit::x86::*;
    use asmkit::{Arch, CodeBuffer, Environment, JitAllocator};

    let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
    {
        let mut assembler = Assembler::new(&mut buffer);
        assembler.mov(RAX, 42i64);
        assembler.ret();
    }
    let code = buffer.finish().expect("x86_64 assembly failed");
    let mut allocator = JitAllocator::new(Default::default());
    let span = code
        .allocate(&mut allocator)
        .expect("x86_64 JIT allocation failed");
    let function: extern "C" fn() -> u64 = unsafe { core::mem::transmute(span.rx()) };
    assert_eq!(function(), 42);
}

#[cfg(all(feature = "jit", feature = "aarch64", target_arch = "aarch64"))]
#[test]
fn aarch64_returns_a_scalar() {
    use asmkit::aarch64::*;
    use asmkit::{Arch, CodeBuffer, Environment, JitAllocator};

    let mut buffer = CodeBuffer::new(Environment::new(Arch::AArch64));
    {
        let mut assembler = Assembler::new(&mut buffer);
        assembler.mov(regs::x(0), 42i64);
        assembler.ret(regs::x(30));
    }
    let code = buffer.finish().expect("AArch64 assembly failed");
    let mut allocator = JitAllocator::new(Default::default());
    let span = code
        .allocate(&mut allocator)
        .expect("AArch64 JIT allocation failed");
    let function: extern "C" fn() -> u64 = unsafe { core::mem::transmute(span.rx()) };
    assert_eq!(function(), 42);
}

#[cfg(all(feature = "jit", feature = "riscv", target_arch = "riscv64"))]
#[test]
fn riscv64_returns_a_scalar() {
    use asmkit::riscv::*;
    use asmkit::{Arch, CodeBuffer, Environment, JitAllocator};

    let mut buffer = CodeBuffer::new(Environment::new(Arch::RISCV64));
    {
        let mut assembler = Assembler::new(&mut buffer);
        assembler.addi(A0, ZERO, 42i64);
        assembler.ret();
    }
    let code = buffer.finish().expect("RV64 assembly failed");
    let mut allocator = JitAllocator::new(Default::default());
    let span = code
        .allocate(&mut allocator)
        .expect("RV64 JIT allocation failed");
    let function: extern "C" fn() -> u64 = unsafe { core::mem::transmute(span.rx()) };
    assert_eq!(function(), 42);
}
