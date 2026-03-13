//! X86 Assembler.
//!
//! This module provides functionality for encoding x86 instructions.
//!
//! Main entrypoint is the [`Assembler`] struct, which provides methods for encoding instructions.
//! ## NOTE
//!
//! Dynamic code generation and text parsing are feature gated, enable `x86-dyn` and `x86-asm` respectively to use those features.
//! But be aware that they increase the size of the crate significantly, so they are not enabled by default.
//!
//!
pub mod arch_traits;
#[cfg(feature = "x86")]
pub mod assembler;
#[cfg(feature = "x86")]
pub mod features;
#[cfg(feature = "x86")]
pub mod opcodes;
pub mod operands;

pub use crate::X86Error;
pub use crate::core::operand::imm;
#[cfg(feature = "x86")]
pub use assembler::*;
#[cfg(feature = "x86")]
pub use features::*;
pub use operands::*;
