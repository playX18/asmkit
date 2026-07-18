//! # asmkit
//!
//! ### Overview
//! asmkit is a portable assembler toolkit built around an AsmJit-style instruction-database
//! model. It is a small, efficient, `no_std` library for encoding machine code without being
//! tied to a specific platform. Key features include:
//!
//! - **Multi-Architecture Support**: X64 (64-bit only), RISC-V, and AArch64, with PPC as
//!   work-in-progress. Each backend follows the same uniform model: a dense `InstId` enum
//!   backed by generated instdb tables, and a single emit entry point
//!   `Assembler::emit_n(impl Into<u32>, &[&Operand])`.
//! - **Generated emitter traits**: per-mnemonic traits (e.g. `MovEmitter`) with impls for
//!   the sized register wrappers, so register constants and integer immediates are passed
//!   directly (`asm.mov(RAX, 42)` — no dereferencing).
//! - **Read/write effects**: `query_rw_info(&Inst) -> InstRwInfo` per architecture, over the
//!   arch-neutral [`core::inst::Inst`], as input for future register allocation.
//! - **Deferred emission**: [`core::builder::Builder`] records instructions and label binds
//!   and replays them into any [`core::builder::InstSink`] (implemented by every
//!   architecture's `Assembler`).
//! - **Minimal Dependencies**:
//! - - `libc`, `intrusive-collections`, `errno` - For JIT support.
//! - - `paste`, `bitflags`, `cfgenius`, `num-traits` - Utility crates that simplify repetitive
//!     arch-specific declarations.
//! - - `smallvec` - Avoids frequent heap allocation during code generation.
//! - **Code Relocations**: Provides a CodeBuffer interface to handle relocations, allowing
//!   the insertion of symbols into the API seamlessly.
//! - **Portability**: Built to run on any platform, with the architecture-specific parts of
//!   the library being independent of the platform on which asmkit is built.
//!
//! ### Usage
//!
//! To use the library simply import the module for the architecture you want to emit code
//! for, e.g. `use asmkit::x86::*;`; this includes all the code required to generate code for
//! that platform.
//!
//! Example:
//!
//! ```rust
//! use asmkit::core::buffer::CodeBuffer;
//! use asmkit::core::jit_allocator::JitAllocator;
//! use asmkit::x86::*;
//!
//! let mut buf = CodeBuffer::new();
//! let mut asm = Assembler::new(&mut buf);
//!
//! // Typed sized registers and plain integer immediates.
//! asm.mov(RAX, 5);
//! asm.add(RAX, 37);
//! asm.ret();
//!
//! let result = buf.finish();
//! let mut jit = JitAllocator::new(Default::default());
//! // you can also use jit.alloc + jit.write manually.
//! let span = result
//!     .allocate(&mut jit)
//!     .expect("failed to allocate JIT-code");
//!
//! // JIT Allocator uses dual-mapping: it allocates two pages which map to same physical space
//! // and you write to executable code through `span.rw()` pointer while you can execute `span.rx()`.
//! let f: extern "C" fn() -> u64 = unsafe { std::mem::transmute(span.rx()) };
//! #[cfg(all(unix, target_arch = "x86_64"))] // can run only on x64 and on SystemV platforms.
//! assert_eq!(f(), 42);
//! ```

#![cfg_attr(not(test), no_std)]

extern crate alloc;

#[cfg(feature = "aarch64")]
pub mod aarch64;
pub mod core;
pub mod ppc;
#[cfg(feature = "riscv")]
pub mod riscv;
pub mod util;
#[cfg(feature = "x86")]
pub mod x86;

use ::core::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AsmError {
    InvalidPrefix,
    InvalidOperand,
    InvalidImmediate,
    InvalidInstruction,
    OutOfMemory,
    InvalidState,
    TooManyHandles,
    InvalidArgument,
    FailedToOpenAnonymousMemory,
    TooLarge,
    X86(X86Error),
    UnsupportedInstruction { reason: &'static str },
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
            AsmError::InvalidImmediate => write!(f, "invalid immediate"),
            AsmError::FailedToOpenAnonymousMemory => {
                write!(f, "failed to open anonymous memory")
            }
            AsmError::TooLarge => write!(f, "too large"),
            AsmError::X86(e) => write!(f, "x86 error: {}", e),
            AsmError::UnsupportedInstruction { reason } => {
                write!(f, "unsupported instruction: {}", reason)
            }
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
    InvalidOperandCombination {
        mnemonic: &'static str,
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
            X86Error::InvalidOperandCombination { mnemonic } => {
                write!(
                    f,
                    "invalid operand combination for instruction `{}`",
                    mnemonic
                )
            }
        }
    }
}
