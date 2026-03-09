use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `MOVDIR64B`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait Movdir64bEmitter<A, B> {
    fn movdir64b(&mut self, op0: A, op1: B);
}

impl<'a> Movdir64bEmitter<Gpq, Mem> for Assembler<'a> {
    fn movdir64b(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            MOVDIR64BRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `MOVDIR64B`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn movdir64b<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Movdir64bEmitter<A, B>,
    {
        <Self as Movdir64bEmitter<A, B>>::movdir64b(self, op0, op1);
    }
}
