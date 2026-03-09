use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `RDSEED`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// | 3 | Gpw      |
/// +---+----------+
/// ```
pub trait RdseedEmitter<A> {
    fn rdseed(&mut self, op0: A);
}

impl<'a> RdseedEmitter<Gpw> for Assembler<'a> {
    fn rdseed(&mut self, op0: Gpw) {
        self.emit(RDSEED16R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> RdseedEmitter<Gpd> for Assembler<'a> {
    fn rdseed(&mut self, op0: Gpd) {
        self.emit(RDSEED32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> RdseedEmitter<Gpq> for Assembler<'a> {
    fn rdseed(&mut self, op0: Gpq) {
        self.emit(RDSEED64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `RDSEED`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// | 3 | Gpw      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn rdseed<A>(&mut self, op0: A)
    where
        Assembler<'a>: RdseedEmitter<A>,
    {
        <Self as RdseedEmitter<A>>::rdseed(self, op0);
    }
}
