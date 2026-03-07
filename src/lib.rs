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

#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod aarch64;
pub mod core;
pub mod ppc;
pub mod riscv;
pub mod util;
pub mod x86;
pub mod masm;

use ::core::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
    X86(X86Error),
}

impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsmError::InvalidPrefix => write!(f, "invalid prefix"),
            AsmError::InvalidOperand => write!(f, "invalid operand"),
            AsmError::InvalidInstruction => write!(f, "invalid instruction"),
            AsmError::OutOfMemory => write!(f, "out of memory"),
            AsmError::InvalidState => write!(f, "invalid state"),
            AsmError::TooManyHandles => write!(f, "too many handles"),
            AsmError::InvalidArgument => write!(f, "invalid argument"),
            AsmError::FailedToOpenAnonymousMemory => {
                write!(f, "failed to open anonymous memory")
            }
            AsmError::TooLarge => write!(f, "too large"),
            AsmError::X86(e) => write!(f, "x86 error: {}", e),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum X86Error {
    InvalidPrefix {
        prefix: u64,
        reason: &'static str,
    },
    InvalidOperand {
        operand_index: usize,
        reason: &'static str,
    },
    InvalidInstruction {
        opcode: u64,
        reason: &'static str,
    },
    InvalidEncoding {
        encoding: u8,
        reason: &'static str,
    },
    InvalidModRM {
        modrm: u8,
        reason: &'static str,
    },
    InvalidSIB {
        sib: u8,
        reason: &'static str,
    },
    InvalidDisplacement {
        value: i64,
        size: usize,
        reason: &'static str,
    },
    InvalidImmediate {
        value: i64,
        size: usize,
        reason: &'static str,
    },
    InvalidRegister {
        reg_id: u32,
        reg_type: &'static str,
        reason: &'static str,
    },
    InvalidMemoryOperand {
        base: Option<u32>,
        index: Option<u32>,
        scale: u8,
        offset: i64,
        reason: &'static str,
    },
    InvalidVSIB {
        index_reg: u32,
        reason: &'static str,
    },
    InvalidMasking {
        mask_reg: u32,
        reason: &'static str,
    },
    InvalidBroadcast {
        reason: &'static str,
    },
    InvalidRoundingControl {
        rc: u64,
        reason: &'static str,
    },
    InvalidEVEX {
        field: &'static str,
        reason: &'static str,
    },
    InvalidVEX {
        field: &'static str,
        reason: &'static str,
    },
    TooLongInstruction {
        length: usize,
        max_length: usize,
    },
    SegmentOverrideNotAllowed {
        segment: u8,
        reason: &'static str,
    },
    AddressSizeMismatch {
        expected: usize,
        actual: usize,
    },
    OperandSizeMismatch {
        expected: usize,
        actual: usize,
    },
    InvalidRIPRelative {
        offset: i64,
        reason: &'static str,
    },
    InvalidLabel {
        label_id: u32,
        reason: &'static str,
    },
    InvalidSymbol {
        symbol_id: u32,
        reason: &'static str,
    },
    InvalidRelocation {
        reloc_type: &'static str,
        reason: &'static str,
    },
}

impl fmt::Display for X86Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X86Error::InvalidPrefix { prefix, reason } => {
                write!(f, "invalid prefix 0x{:x}: {}", prefix, reason)
            }
            X86Error::InvalidOperand {
                operand_index,
                reason,
            } => write!(f, "invalid operand {}: {}", operand_index, reason),
            X86Error::InvalidInstruction { opcode, reason } => {
                write!(f, "invalid instruction 0x{:x}: {}", opcode, reason)
            }
            X86Error::InvalidEncoding { encoding, reason } => {
                write!(f, "invalid encoding {}: {}", encoding, reason)
            }
            X86Error::InvalidModRM { modrm, reason } => {
                write!(f, "invalid ModRM byte 0x{:02x}: {}", modrm, reason)
            }
            X86Error::InvalidSIB { sib, reason } => {
                write!(f, "invalid SIB byte 0x{:02x}: {}", sib, reason)
            }
            X86Error::InvalidDisplacement {
                value,
                size,
                reason,
            } => write!(
                f,
                "invalid displacement 0x{:x} (size {}): {}",
                value, size, reason
            ),
            X86Error::InvalidImmediate {
                value,
                size,
                reason,
            } => {
                write!(
                    f,
                    "invalid immediate 0x{:x} (size {}): {}",
                    value, size, reason
                )
            }
            X86Error::InvalidRegister {
                reg_id,
                reg_type,
                reason,
            } => write!(
                f,
                "invalid register {} (type {}): {}",
                reg_id, reg_type, reason
            ),
            X86Error::InvalidMemoryOperand {
                base,
                index,
                scale,
                offset,
                reason,
            } => write!(
                f,
                "invalid memory operand [base={:?}, index={:?}, scale={}, offset={}]: {}",
                base, index, scale, offset, reason
            ),
            X86Error::InvalidVSIB { index_reg, reason } => {
                write!(f, "invalid VSIB index register {}: {}", index_reg, reason)
            }
            X86Error::InvalidMasking { mask_reg, reason } => {
                write!(f, "invalid mask register {}: {}", mask_reg, reason)
            }
            X86Error::InvalidBroadcast { reason } => {
                write!(f, "invalid broadcast: {}", reason)
            }
            X86Error::InvalidRoundingControl { rc, reason } => {
                write!(f, "invalid rounding control 0x{:x}: {}", rc, reason)
            }
            X86Error::InvalidEVEX { field, reason } => {
                write!(f, "invalid EVEX field '{}': {}", field, reason)
            }
            X86Error::InvalidVEX { field, reason } => {
                write!(f, "invalid VEX field '{}': {}", field, reason)
            }
            X86Error::TooLongInstruction { length, max_length } => write!(
                f,
                "instruction too long: {} bytes (max {})",
                length, max_length
            ),
            X86Error::SegmentOverrideNotAllowed { segment, reason } => {
                write!(f, "segment override {} not allowed: {}", segment, reason)
            }
            X86Error::AddressSizeMismatch { expected, actual } => write!(
                f,
                "address size mismatch: expected {} bytes, got {}",
                expected, actual
            ),
            X86Error::OperandSizeMismatch { expected, actual } => write!(
                f,
                "operand size mismatch: expected {} bytes, got {}",
                expected, actual
            ),
            X86Error::InvalidRIPRelative { offset, reason } => {
                write!(f, "invalid RIP-relative offset {}: {}", offset, reason)
            }
            X86Error::InvalidLabel { label_id, reason } => {
                write!(f, "invalid label {}: {}", label_id, reason)
            }
            X86Error::InvalidSymbol { symbol_id, reason } => {
                write!(f, "invalid symbol {}: {}", symbol_id, reason)
            }
            X86Error::InvalidRelocation { reloc_type, reason } => {
                write!(f, "invalid relocation {}: {}", reloc_type, reason)
            }
        }
    }
}
