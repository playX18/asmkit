#[doc(hidden)]
pub(crate) mod arch_traits;
pub mod assembler;
#[doc(hidden)]
// Generated instruction APIs retain spec prose and operand-expression shape.
#[allow(clippy::doc_lazy_continuation)]
mod emitter;
pub(crate) mod instapi;
#[allow(dead_code)]
pub(crate) mod instdb;
#[allow(
    clippy::doc_lazy_continuation,
    clippy::identity_op,
    clippy::needless_borrow
)]
#[allow(dead_code)]
pub(crate) mod opcodes;
pub mod operands;

pub use crate::core::operand::imm;
pub use assembler::*;
pub use emitter::*;
pub use instapi::query_rw_info;
pub use instdb::{ALL_CPU_FEATURES, CPU_FEATURE_COUNT, CPU_FEATURE_NAMES, CpuFeature};
pub use opcodes::Opcode;
pub use operands::*;
pub use regs::*;

/// Unstable validation tooling; this is not a stable instruction-encoding API.
#[cfg(feature = "validation")]
#[doc(hidden)]
pub mod coverage {
    pub use super::instdb::{ANY, FP, GP, IMM, INST_INFO_TABLE, SIGNATURE_TABLE, VEC};
    pub use super::opcodes::{ALL_OPCODES, Encoding, OPCODE_STR, OPCODE_XLEN};
}

/// Return the length (in bytes) of an instruction given the low 16 bits of it.
///
/// The current spec reserves a bit pattern for instructions of length >= 192 bits, but for
/// simplicity this function just returns 24 in that case. The largest instructions currently
/// defined are 4 bytes so it will likely be a long time until this difference matters.
pub fn instruction_length(i: u16) -> usize {
    if i & 0b11 != 0b11 {
        2
    } else if i & 0b11100 != 0b11100 {
        4
    } else if i & 0b111111 == 0b011111 {
        6
    } else if i & 0b1111111 == 0b011111 {
        8
    } else {
        10 + 2 * ((i >> 12) & 0b111) as usize
    }
}
