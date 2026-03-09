use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PREFETCH`.
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
pub trait PrefetchEmitter<A> {
    fn prefetch(&mut self, op0: A);
}

impl<'a> PrefetchEmitter<Mem> for Assembler<'a> {
    fn prefetch(&mut self, op0: Mem) {
        self.emit(PREFETCHM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `PREFETCH`.
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
    pub fn prefetch<A>(&mut self, op0: A)
    where
        Assembler<'a>: PrefetchEmitter<A>,
    {
        <Self as PrefetchEmitter<A>>::prefetch(self, op0);
    }
}
