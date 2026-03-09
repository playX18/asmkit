use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `HRESET`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Imm      |
/// +---+----------+
/// ```
pub trait HresetEmitter<A> {
    fn hreset(&mut self, op0: A);
}

impl<'a> HresetEmitter<Imm> for Assembler<'a> {
    fn hreset(&mut self, op0: Imm) {
        self.emit(HRESETI, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `HRESET`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Imm      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn hreset<A>(&mut self, op0: A)
    where
        Assembler<'a>: HresetEmitter<A>,
    {
        <Self as HresetEmitter<A>>::hreset(self, op0);
    }
}
