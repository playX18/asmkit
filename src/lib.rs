//#![no_std]
extern crate alloc;

//#[cfg(feature = "assembler")]
//pub mod assembler;

pub mod x86;

//pub mod encdec;
pub mod core;
pub mod util;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
}
