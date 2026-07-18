//! X86 Assembler.
//!
//! This module provides functionality for encoding x86 instructions.
//!
//! Main entrypoint is the [`Assembler`] struct, which provides methods for encoding instructions.
//!
pub mod arch_traits;
#[cfg(feature = "x86")]
pub mod assembler;
pub mod encoder;
pub mod encoder_tables;
#[cfg(feature = "x86")]
pub mod features;
pub mod instapi;
pub mod instdb;
pub mod opcode;
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
