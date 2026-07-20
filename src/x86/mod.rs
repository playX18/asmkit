//! X86 Assembler.
//!
//! This module provides functionality for encoding x86 instructions.
//!
//! Main entrypoint is the [`Assembler`] struct. Instructions are emitted either by id
//! through [`Assembler::emit_n`] or through the generated per-mnemonic emitter traits
//! (e.g. `MovEmitter::mov`), both backed by the asmjit-style InstInfo
//! pipeline in the internal emitter implementation.
//!
#[doc(hidden)]
pub(crate) mod arch_traits;
#[cfg(feature = "x86")]
pub mod assembler;
#[cfg(feature = "x86")]
pub(crate) mod emit;
#[cfg(feature = "x86")]
#[doc(hidden)]
mod emitter;
pub(crate) mod encoder;
pub(crate) mod encoder_tables;
pub(crate) mod instapi;
#[allow(
    dead_code,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_html_tags,
    rustdoc::private_intra_doc_links
)]
pub(crate) mod instdb;
pub(crate) mod opcode;
pub mod operands;

pub use crate::X86Error;
pub use crate::core::operand::imm;
#[cfg(feature = "x86")]
pub use assembler::*;
#[cfg(feature = "x86")]
pub use emitter::*;
pub use instapi::query_rw_info;
pub use instdb::{CpuFeature, InstId};
pub use operands::*;

/// Unstable validation tooling; this is not a stable instruction-encoding API.
#[cfg(feature = "validation")]
#[doc(hidden)]
pub mod coverage {
    pub use super::instdb::{
        Avx512Flags, INST_COMMON_INFO_TABLE, INST_INFO_TABLE, INST_NAME_INDEX_TABLE,
        INST_NAME_STRING_TABLE, INST_SIGNATURE_TABLE, InstSignature, Mode, OP_SIGNATURE_TABLE,
        OpFlags, OpSignature,
    };
}
