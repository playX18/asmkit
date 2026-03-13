use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `3DNOW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------+
/// | # | Operands     |
/// +---+--------------+
/// | 1 | Mm, Mem, Imm |
/// | 2 | Mm, Mm, Imm  |
/// +---+--------------+
/// ```
pub trait _3dnowEmitter<A, B, C> {
    fn _3dnow(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> _3dnowEmitter<Mm, Mm, Imm> for Assembler<'a> {
    fn _3dnow(&mut self, op0: Mm, op1: Mm, op2: Imm) {
        self.emit(_3DNOWRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> _3dnowEmitter<Mm, Mem, Imm> for Assembler<'a> {
    fn _3dnow(&mut self, op0: Mm, op1: Mem, op2: Imm) {
        self.emit(_3DNOWRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `FEMMS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FemmsEmitter {
    fn femms(&mut self);
}

impl<'a> FemmsEmitter for Assembler<'a> {
    fn femms(&mut self) {
        self.emit(FEMMS, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `3DNOW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------+
    /// | # | Operands     |
    /// +---+--------------+
    /// | 1 | Mm, Mem, Imm |
    /// | 2 | Mm, Mm, Imm  |
    /// +---+--------------+
    /// ```
    #[inline]
    pub fn _3dnow<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: _3dnowEmitter<A, B, C> {
        <Self as _3dnowEmitter<A, B, C>>::_3dnow(self, op0, op1, op2);
    }
    /// `FEMMS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn femms(&mut self)
    where Assembler<'a>: FemmsEmitter {
        <Self as FemmsEmitter>::femms(self);
    }
}
