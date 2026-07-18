use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `RMPREAD`.
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
pub trait RmpreadEmitter {
    fn rmpread(&mut self);
}

impl<'a> RmpreadEmitter for Assembler<'a> {
    fn rmpread(&mut self) {
        self.emit(RMPREAD, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `RMPREAD`.
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
    pub fn rmpread(&mut self)
    where
        Assembler<'a>: RmpreadEmitter,
    {
        <Self as RmpreadEmitter>::rmpread(self);
    }
}
