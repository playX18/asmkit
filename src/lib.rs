//#![no_std]
extern crate alloc;

pub mod core;

pub mod aarch64;
pub mod ppc;
pub mod riscv;
pub mod util;
pub mod x86;

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
