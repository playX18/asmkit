use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `POPCNT` (POPCNT). 
/// This instruction calculates the number of bits set to 1 in the second operand (source) and returns the count in the first operand (a destination register).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/POPCNT.html).
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
pub trait PopcntEmitter<A, B> {
    fn popcnt(&mut self, op0: A, op1: B);
}

impl<'a> PopcntEmitter<Gpw, Gpw> for Assembler<'a> {
    fn popcnt(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(POPCNT16RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> PopcntEmitter<Gpw, Mem> for Assembler<'a> {
    fn popcnt(&mut self, op0: Gpw, op1: Mem) {
        self.emit(POPCNT16RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> PopcntEmitter<Gpd, Gpd> for Assembler<'a> {
    fn popcnt(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(POPCNT32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> PopcntEmitter<Gpd, Mem> for Assembler<'a> {
    fn popcnt(&mut self, op0: Gpd, op1: Mem) {
        self.emit(POPCNT32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> PopcntEmitter<Gpq, Gpq> for Assembler<'a> {
    fn popcnt(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(POPCNT64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> PopcntEmitter<Gpq, Mem> for Assembler<'a> {
    fn popcnt(&mut self, op0: Gpq, op1: Mem) {
        self.emit(POPCNT64RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `POPCNT` (POPCNT). 
    /// This instruction calculates the number of bits set to 1 in the second operand (source) and returns the count in the first operand (a destination register).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/POPCNT.html).
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
    pub fn popcnt<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: PopcntEmitter<A, B> {
        <Self as PopcntEmitter<A, B>>::popcnt(self, op0, op1);
    }
}
