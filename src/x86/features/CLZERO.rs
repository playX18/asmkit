use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CLZERO`.
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
pub trait ClzeroEmitter<A> {
    fn clzero(&mut self, op0: A);
}

impl<'a> ClzeroEmitter<Gpw> for Assembler<'a> {
    fn clzero(&mut self, op0: Gpw) {
        self.emit(CLZERO16R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> ClzeroEmitter<Gpd> for Assembler<'a> {
    fn clzero(&mut self, op0: Gpd) {
        self.emit(CLZERO32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> ClzeroEmitter<Gpq> for Assembler<'a> {
    fn clzero(&mut self, op0: Gpq) {
        self.emit(CLZERO64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `CLZERO`.
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
    pub fn clzero<A>(&mut self, op0: A)
    where Assembler<'a>: ClzeroEmitter<A> {
        <Self as ClzeroEmitter<A>>::clzero(self, op0);
    }
}
