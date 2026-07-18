use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `URDMSR`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Gpd |
/// | 2 | Gpd, Imm |
/// +---+----------+
/// ```
pub trait UrdmsrEmitter<A, B> {
    fn urdmsr(&mut self, op0: A, op1: B);
}

impl<'a> UrdmsrEmitter<Gpd, Gpd> for Assembler<'a> {
    fn urdmsr(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(URDMSRRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> UrdmsrEmitter<Gpd, Imm> for Assembler<'a> {
    fn urdmsr(&mut self, op0: Gpd, op1: Imm) {
        self.emit(URDMSRRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `UWRMSR`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Gpd |
/// | 2 | Imm, Gpd |
/// +---+----------+
/// ```
pub trait UwrmsrEmitter<A, B> {
    fn uwrmsr(&mut self, op0: A, op1: B);
}

impl<'a> UwrmsrEmitter<Gpd, Gpd> for Assembler<'a> {
    fn uwrmsr(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(UWRMSRRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> UwrmsrEmitter<Imm, Gpd> for Assembler<'a> {
    fn uwrmsr(&mut self, op0: Imm, op1: Gpd) {
        self.emit(UWRMSRIR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `URDMSR`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Gpd |
    /// | 2 | Gpd, Imm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn urdmsr<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: UrdmsrEmitter<A, B>,
    {
        <Self as UrdmsrEmitter<A, B>>::urdmsr(self, op0, op1);
    }
    /// `UWRMSR`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Gpd |
    /// | 2 | Imm, Gpd |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn uwrmsr<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: UwrmsrEmitter<A, B>,
    {
        <Self as UwrmsrEmitter<A, B>>::uwrmsr(self, op0, op1);
    }
}
