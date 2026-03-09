use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PREFETCHWT1`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait Prefetchwt1Emitter<A> {
    fn prefetchwt1(&mut self, op0: A);
}

impl<'a> Prefetchwt1Emitter<Mem> for Assembler<'a> {
    fn prefetchwt1(&mut self, op0: Mem) {
        self.emit(PREFETCHWT1M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `PREFETCHWT1`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn prefetchwt1<A>(&mut self, op0: A)
    where
        Assembler<'a>: Prefetchwt1Emitter<A>,
    {
        <Self as Prefetchwt1Emitter<A>>::prefetchwt1(self, op0);
    }
}
