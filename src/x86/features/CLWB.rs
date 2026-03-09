use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CLWB`.
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
pub trait ClwbEmitter<A> {
    fn clwb(&mut self, op0: A);
}

impl<'a> ClwbEmitter<Mem> for Assembler<'a> {
    fn clwb(&mut self, op0: Mem) {
        self.emit(CLWBM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `CLWB`.
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
    pub fn clwb<A>(&mut self, op0: A)
    where
        Assembler<'a>: ClwbEmitter<A>,
    {
        <Self as ClwbEmitter<A>>::clwb(self, op0);
    }
}
