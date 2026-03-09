use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `RMPQUERY`.
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
pub trait RmpqueryEmitter {
    fn rmpquery(&mut self);
}

impl<'a> RmpqueryEmitter for Assembler<'a> {
    fn rmpquery(&mut self) {
        self.emit(RMPQUERY, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `RMPQUERY`.
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
    pub fn rmpquery(&mut self)
    where
        Assembler<'a>: RmpqueryEmitter,
    {
        <Self as RmpqueryEmitter>::rmpquery(self);
    }
}
