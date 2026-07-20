#![allow(non_upper_case_globals)]
#[doc(hidden)]
pub(crate) mod arch_traits;
pub mod assembler;
// These modules mechanically preserve AsmJit's encoding arithmetic and tables.
#[allow(
    clippy::collapsible_if,
    clippy::get_first,
    clippy::manual_range_contains,
    clippy::needless_late_init,
    clippy::unnecessary_cast
)]
pub(crate) mod emit;
#[doc(hidden)]
mod emitter;
#[allow(
    clippy::identity_op,
    clippy::needless_return,
    clippy::too_many_arguments,
    clippy::unnecessary_cast
)]
pub(crate) mod encoder;
pub(crate) mod encoder_tables;
pub(crate) mod instapi;
#[allow(
    dead_code,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::unnecessary_cast,
    clippy::upper_case_acronyms
)]
pub(crate) mod instdb;
pub mod operands;
pub(crate) mod rwflags;

pub use crate::core::operand::imm;
pub use assembler::*;
#[cfg(test)]
pub(crate) use emit::CPU_FEATURE_REPRESENTATIVE;
#[cfg(test)]
pub(crate) use emit::INST_FEATURE_MASKS;
pub use emit::{ALL_CPU_FEATURES, CPU_FEATURE_COUNT, CPU_FEATURE_NAMES, CpuFeature};
pub use emitter::*;
pub use instapi::query_rw_info;
pub use instdb::InstId;
pub use operands::*;

/// Unstable validation tooling; this is not a stable instruction-encoding API.
#[cfg(feature = "validation")]
#[doc(hidden)]
pub mod coverage {
    pub use super::instdb::{Encoding, INST_INFO_TABLE};
}
