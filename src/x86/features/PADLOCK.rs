use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XSTORE`.
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
pub trait XstoreEmitter {
    fn xstore(&mut self);
}

impl<'a> XstoreEmitter for Assembler<'a> {
    fn xstore(&mut self) {
        self.emit(XSTORE, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `XSTORE`.
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
    pub fn xstore(&mut self)
    where
        Assembler<'a>: XstoreEmitter,
    {
        <Self as XstoreEmitter>::xstore(self);
    }
}
