use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PREFETCHW`.
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
pub trait PrefetchwEmitter<A> {
    fn prefetchw(&mut self, op0: A);
}

impl<'a> PrefetchwEmitter<Mem> for Assembler<'a> {
    fn prefetchw(&mut self, op0: Mem) {
        self.emit(PREFETCHWM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `PREFETCHW`.
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
    pub fn prefetchw<A>(&mut self, op0: A)
    where
        Assembler<'a>: PrefetchwEmitter<A>,
    {
        <Self as PrefetchwEmitter<A>>::prefetchw(self, op0);
    }
}
