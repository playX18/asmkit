use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CLDEMOTE`.
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
pub trait CldemoteEmitter<A> {
    fn cldemote(&mut self, op0: A);
}

impl<'a> CldemoteEmitter<Mem> for Assembler<'a> {
    fn cldemote(&mut self, op0: Mem) {
        self.emit(CLDEMOTEM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `CLDEMOTE`.
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
    pub fn cldemote<A>(&mut self, op0: A)
    where
        Assembler<'a>: CldemoteEmitter<A>,
    {
        <Self as CldemoteEmitter<A>>::cldemote(self, op0);
    }
}
