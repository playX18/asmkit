//! X86 Assembler.
//!
//! This module provides functionality for encoding x86 instructions.
//!
//! Main entrypoint is the [`Assembler`] struct. Instructions are emitted either by id
//! through [`Assembler::emit_n`] or through the generated per-mnemonic emitter traits
//! ([`emitter`], e.g. `MovEmitter::mov`), both backed by the asmjit-style InstInfo
//! pipeline in [`emit`].
//!
pub mod arch_traits;
#[cfg(feature = "x86")]
pub mod assembler;
#[cfg(feature = "x86")]
pub mod emit;
#[cfg(feature = "x86")]
pub mod emitter;
pub mod encoder;
pub mod encoder_tables;
pub mod instapi;
pub mod instdb;
pub mod opcode;
pub mod operands;

pub use crate::X86Error;
pub use crate::core::operand::imm;
#[cfg(feature = "x86")]
pub use assembler::*;
#[cfg(feature = "x86")]
pub use emitter::*;
pub use operands::*;
