use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `RDTSCP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait RdtscpEmitter {
    fn rdtscp(&mut self);
}

impl<'a> RdtscpEmitter for Assembler<'a> {
    fn rdtscp(&mut self) {
        self.emit(RDTSCP, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `RDTSCP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn rdtscp(&mut self)
    where
        Assembler<'a>: RdtscpEmitter,
    {
        <Self as RdtscpEmitter>::rdtscp(self);
    }
}
