use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `RDMSR`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Imm |
/// +---+----------+
/// ```
pub trait RdmsrEmitter_2<A, B> {
    fn rdmsr_2(&mut self, op0: A, op1: B);
}

impl<'a> RdmsrEmitter_2<Gpd, Imm> for Assembler<'a> {
    fn rdmsr_2(&mut self, op0: Gpd, op1: Imm) {
        self.emit(RDMSRRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `WRMSRNS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Imm, Gpd |
/// +---+----------+
/// ```
pub trait WrmsrnsEmitter_2<A, B> {
    fn wrmsrns_2(&mut self, op0: A, op1: B);
}

impl<'a> WrmsrnsEmitter_2<Imm, Gpd> for Assembler<'a> {
    fn wrmsrns_2(&mut self, op0: Imm, op1: Gpd) {
        self.emit(
            WRMSRNSIR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `RDMSR`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Imm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn rdmsr_2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: RdmsrEmitter_2<A, B>,
    {
        <Self as RdmsrEmitter_2<A, B>>::rdmsr_2(self, op0, op1);
    }
    /// `WRMSRNS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Imm, Gpd |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn wrmsrns_2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: WrmsrnsEmitter_2<A, B>,
    {
        <Self as WrmsrnsEmitter_2<A, B>>::wrmsrns_2(self, op0, op1);
    }
}
