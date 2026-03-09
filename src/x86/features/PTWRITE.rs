use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PTWRITE`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// | 3 | Mem      |
/// +---+----------+
/// ```
pub trait PtwriteEmitter<A> {
    fn ptwrite(&mut self, op0: A);
}

impl<'a> PtwriteEmitter<Gpd> for Assembler<'a> {
    fn ptwrite(&mut self, op0: Gpd) {
        self.emit(PTWRITE32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> PtwriteEmitter<Mem> for Assembler<'a> {
    fn ptwrite(&mut self, op0: Mem) {
        self.emit(PTWRITE32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> PtwriteEmitter<Gpq> for Assembler<'a> {
    fn ptwrite(&mut self, op0: Gpq) {
        self.emit(PTWRITE64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `PTWRITE`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// | 3 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn ptwrite<A>(&mut self, op0: A)
    where
        Assembler<'a>: PtwriteEmitter<A>,
    {
        <Self as PtwriteEmitter<A>>::ptwrite(self, op0);
    }
}
