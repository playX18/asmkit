use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `MOVBE` (MOVBE). 
/// Performs a byte swap operation on the data copied from the second operand (source operand) and store the result in the first operand (destination operand). The source operand can be a general-purpose register, or memory location; the destination register can be a general-purpose register, or a memory location; however, both operands can not be registers, and only one operand can be a memory location. Both operands must be the same size, which can be a word, a doubleword or quadword.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVBE.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Mem |
/// | 2 | Gpq, Mem |
/// | 3 | Gpw, Mem |
/// | 4 | Mem, Gpd |
/// | 5 | Mem, Gpq |
/// | 6 | Mem, Gpw |
/// +---+----------+
/// ```
pub trait MovbeEmitter<A, B> {
    fn movbe(&mut self, op0: A, op1: B);
}

impl<'a> MovbeEmitter<Gpw, Mem> for Assembler<'a> {
    fn movbe(&mut self, op0: Gpw, op1: Mem) {
        self.emit(MOVBE16RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MovbeEmitter<Gpd, Mem> for Assembler<'a> {
    fn movbe(&mut self, op0: Gpd, op1: Mem) {
        self.emit(MOVBE32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MovbeEmitter<Gpq, Mem> for Assembler<'a> {
    fn movbe(&mut self, op0: Gpq, op1: Mem) {
        self.emit(MOVBE64RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MovbeEmitter<Mem, Gpw> for Assembler<'a> {
    fn movbe(&mut self, op0: Mem, op1: Gpw) {
        self.emit(MOVBE16MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MovbeEmitter<Mem, Gpd> for Assembler<'a> {
    fn movbe(&mut self, op0: Mem, op1: Gpd) {
        self.emit(MOVBE32MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MovbeEmitter<Mem, Gpq> for Assembler<'a> {
    fn movbe(&mut self, op0: Mem, op1: Gpq) {
        self.emit(MOVBE64MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `MOVBE` (MOVBE). 
    /// Performs a byte swap operation on the data copied from the second operand (source operand) and store the result in the first operand (destination operand). The source operand can be a general-purpose register, or memory location; the destination register can be a general-purpose register, or a memory location; however, both operands can not be registers, and only one operand can be a memory location. Both operands must be the same size, which can be a word, a doubleword or quadword.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVBE.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Mem |
    /// | 2 | Gpq, Mem |
    /// | 3 | Gpw, Mem |
    /// | 4 | Mem, Gpd |
    /// | 5 | Mem, Gpq |
    /// | 6 | Mem, Gpw |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn movbe<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MovbeEmitter<A, B> {
        <Self as MovbeEmitter<A, B>>::movbe(self, op0, op1);
    }
}
