use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XSAVEOPT`.
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
pub trait XsaveoptEmitter<A> {
    fn xsaveopt(&mut self, op0: A);
}

impl<'a> XsaveoptEmitter<Mem> for Assembler<'a> {
    fn xsaveopt(&mut self, op0: Mem) {
        self.emit(XSAVEOPT32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `XSAVEOPT`.
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
    pub fn xsaveopt<A>(&mut self, op0: A)
    where
        Assembler<'a>: XsaveoptEmitter<A>,
    {
        <Self as XsaveoptEmitter<A>>::xsaveopt(self, op0);
    }
}
