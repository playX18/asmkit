use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XRSTORS`.
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
pub trait XrstorsEmitter<A> {
    fn xrstors(&mut self, op0: A);
}

impl<'a> XrstorsEmitter<Mem> for Assembler<'a> {
    fn xrstors(&mut self, op0: Mem) {
        self.emit(XRSTORS32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `XSAVES`.
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
pub trait XsavesEmitter<A> {
    fn xsaves(&mut self, op0: A);
}

impl<'a> XsavesEmitter<Mem> for Assembler<'a> {
    fn xsaves(&mut self, op0: Mem) {
        self.emit(XSAVES32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `XRSTORS`.
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
    pub fn xrstors<A>(&mut self, op0: A)
    where
        Assembler<'a>: XrstorsEmitter<A>,
    {
        <Self as XrstorsEmitter<A>>::xrstors(self, op0);
    }
    /// `XSAVES`.
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
    pub fn xsaves<A>(&mut self, op0: A)
    where
        Assembler<'a>: XsavesEmitter<A>,
    {
        <Self as XsavesEmitter<A>>::xsaves(self, op0);
    }
}
