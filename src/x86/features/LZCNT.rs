use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `LZCNT` (LZCNT). 
/// Counts the number of leading most significant zero bits in a source operand (second operand) returning the result into a destination (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/LZCNT.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Gpd |
/// | 2 | Gpd, Mem |
/// | 3 | Gpq, Gpq |
/// | 4 | Gpq, Mem |
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait LzcntEmitter<A, B> {
    fn lzcnt(&mut self, op0: A, op1: B);
}

impl<'a> LzcntEmitter<Gpw, Gpw> for Assembler<'a> {
    fn lzcnt(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(LZCNT16RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> LzcntEmitter<Gpw, Mem> for Assembler<'a> {
    fn lzcnt(&mut self, op0: Gpw, op1: Mem) {
        self.emit(LZCNT16RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> LzcntEmitter<Gpd, Gpd> for Assembler<'a> {
    fn lzcnt(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(LZCNT32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> LzcntEmitter<Gpd, Mem> for Assembler<'a> {
    fn lzcnt(&mut self, op0: Gpd, op1: Mem) {
        self.emit(LZCNT32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> LzcntEmitter<Gpq, Gpq> for Assembler<'a> {
    fn lzcnt(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(LZCNT64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> LzcntEmitter<Gpq, Mem> for Assembler<'a> {
    fn lzcnt(&mut self, op0: Gpq, op1: Mem) {
        self.emit(LZCNT64RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `LZCNT` (LZCNT). 
    /// Counts the number of leading most significant zero bits in a source operand (second operand) returning the result into a destination (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/LZCNT.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Gpd |
    /// | 2 | Gpd, Mem |
    /// | 3 | Gpq, Gpq |
    /// | 4 | Gpq, Mem |
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn lzcnt<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: LzcntEmitter<A, B> {
        <Self as LzcntEmitter<A, B>>::lzcnt(self, op0, op1);
    }
}
