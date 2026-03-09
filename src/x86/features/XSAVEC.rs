use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XSAVEC`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait XsavecEmitter<A> {
    fn xsavec(&mut self, op0: A);
}

impl<'a> XsavecEmitter<Mem> for Assembler<'a> {
    fn xsavec(&mut self, op0: Mem) {
        self.emit(XSAVEC32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `XSAVEC`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn xsavec<A>(&mut self, op0: A)
    where
        Assembler<'a>: XsavecEmitter<A>,
    {
        <Self as XsavecEmitter<A>>::xsavec(self, op0);
    }
}
