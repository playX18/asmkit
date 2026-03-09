use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `MOVDIRI`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Gpd |
/// | 2 | Mem, Gpq |
/// +---+----------+
/// ```
pub trait MovdiriEmitter<A, B> {
    fn movdiri(&mut self, op0: A, op1: B);
}

impl<'a> MovdiriEmitter<Mem, Gpd> for Assembler<'a> {
    fn movdiri(&mut self, op0: Mem, op1: Gpd) {
        self.emit(
            MOVDIRI32MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MovdiriEmitter<Mem, Gpq> for Assembler<'a> {
    fn movdiri(&mut self, op0: Mem, op1: Gpq) {
        self.emit(
            MOVDIRI64MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `MOVDIRI`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Gpd |
    /// | 2 | Mem, Gpq |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn movdiri<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: MovdiriEmitter<A, B>,
    {
        <Self as MovdiriEmitter<A, B>>::movdiri(self, op0, op1);
    }
}
