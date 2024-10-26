extern crate alloc;

#[cfg(feature = "assembler")]
pub mod assembler;

pub mod encdec;
pub mod util;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AsmError {
    InvalidOperand,
    InvalidInstruction,
    OutOfMemory,
    InvalidState,
    TooManyHandles,
    InvalidArgument,
    FailedToOpenAnonymousMemory
}
