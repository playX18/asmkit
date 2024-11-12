//! # asmkit
//! 
//! ### Overview
//! asmkit is a portable assembler toolkit designed for decoding and encoding various assembly architectures. It aims to provide a small, efficient, and cross-platform library that can be used to build and manipulate assembly code without being tied to a specific platform or architecture. The library is written in Rust and supports several architectures including X64, RISC-V, and ARM (work-in-progress). Key features include:
//! - **Multi-Architecture Support**: Supports multiple architectures such as X64, RISC-V, ARM (WIP), PPC (WIP), and plans to support PPC64 and OpenPOWER in the future.
//! - **Minimal Dependencies**: Relies on a minimal set of dependencies to ensure portability and efficiency:
//! - - `libc` and `intrusive-collections`` - For JIT support.
//! - - `paste` and `derive-more` - Utility crates that simplify repetitive code.
//! - - `smallvec` - A crate used to manage collections that avoid too frequent heap allocations during code generation.
//! 
//! - **Code Relocations**: Provides a CodeBuffer interface to handle relocations, allowing the insertion of symbols into the API seamlessly.
//! - **Auto-Generated Assemblers**: The goal is to support a wide range of platforms and provide auto-generated assemblers for as many architectures as possible.
//! - **Portability**: Built to run on any platform, with the architecture-specific parts of the library being independent of the platform on which asmkit is built.
//! #![no_std]
//! 
//! 
//! ### Usage
//! 
//! To use the library simply import a module for architecture you want to emit code for e.g `use asmkit::x86::*;`; This would
//! include all the required code to generate code for platform. 
//! 
//! Example:
//! 
//! ```rust
//! use asmkit::core::buffer::CodeBuffer;
//! use asmkit::core::jit_allocator::JitAllocator;
//! use asmkit::x86::*;
//!
//! fn main() {
//!     let mut buf = CodeBuffer::new();
//!     let mut asm = Assembler::new(&mut buf);
//!
//!     let dst = RDI;
//!     let arg0 = RSI;
//!     let arg1 = RDX;
//!
//!     asm.sse_movdqurm(XMM0, ptr64(arg0, 0)); // load 4 ints from [arg0] to XMM0
//!     asm.sse_movdqurm(XMM1, ptr64(arg1, 0)); // load 4 ints from [arg1] to XMM1
//!     asm.sse_paddwrr(XMM0, XMM1); // add 4 ints
//!     asm.sse_movdqumr(ptr64(dst, 0), XMM0); // store result in [dst]
//!     asm.ret(); // return from function
//!
//!     let result = buf.finish();
//!     let mut jit = JitAllocator::new(Default::default());
//!     // you can also use jit.alloc + jit.write manually.
//!     let span = result
//!         .allocate(&mut jit)
//!         .expect("failed to allocate JIT-code");
//!
//!     // JIT Allocator uses dual-mapping: it allocates two pages which map to same physical space
//!     // and you write to executable code through `span.rw()` pointer while you can execute `span.rx()`.
//!     let f: extern "C" fn(*mut i32, *const i32, *const i32) = unsafe { std::mem::transmute(span.rx()) };
//!     #[cfg(all(unix, target_arch="x86_64"))] // can run only on x64 and on SystemV platforms.
//!     {
//!         let mut res = [0; 4];
//!         f(res.as_mut_ptr(), [4, 3, 2, 1].as_ptr(), [1, 5, 2, 8].as_ptr());
//!
//!         println!("{:?}", res);
//!     }
//! }

//!```

extern crate alloc;

pub mod core;
pub mod aarch64;
pub mod ppc;
pub mod riscv;
pub mod util;
pub mod x86;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AsmError {
    InvalidPrefix,
    InvalidOperand,
    InvalidInstruction,
    OutOfMemory,
    InvalidState,
    TooManyHandles,
    InvalidArgument,
    FailedToOpenAnonymousMemory,
    TooLarge,
}
