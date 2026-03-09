use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `RDRAND`.
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
pub trait RdrandEmitter<A> {
    fn rdrand(&mut self, op0: A);
}

impl<'a> RdrandEmitter<Gpw> for Assembler<'a> {
    fn rdrand(&mut self, op0: Gpw) {
        self.emit(RDRAND16R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> RdrandEmitter<Gpd> for Assembler<'a> {
    fn rdrand(&mut self, op0: Gpd) {
        self.emit(RDRAND32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> RdrandEmitter<Gpq> for Assembler<'a> {
    fn rdrand(&mut self, op0: Gpq) {
        self.emit(RDRAND64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `RDRAND`.
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
    pub fn rdrand<A>(&mut self, op0: A)
    where
        Assembler<'a>: RdrandEmitter<A>,
    {
        <Self as RdrandEmitter<A>>::rdrand(self, op0);
    }
}
