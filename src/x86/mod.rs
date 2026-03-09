//! X86 instruction encoding and decoding.
//!
//! This module provides functionality for encoding and decoding x86 instructions.
//!
//! Main entrypoint is the [`Assembler`] struct, which provides methods for encoding instructions. For decoding, see the [`decode`] module.
//!
//! ## NOTE
//!
//! Dynamic code generation and text parsing are feature gated, enable `x86-dyn` and `x86-asm` respectively to use those features.
//! But be aware that they increase the size of the crate significantly, so they are not enabled by default.
pub mod arch_traits;
pub mod assembler;
pub mod decode;
pub mod decode_tab;
pub mod features;
pub mod formatter;
pub mod macroassembler;
pub mod opcodes;
pub mod operands;

pub use crate::X86Error;
pub use crate::core::operand::imm;
pub use assembler::*;
pub use features::*;
pub use operands::*;
