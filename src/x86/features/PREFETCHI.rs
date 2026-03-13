use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PREFETCHIT0`.
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
pub trait Prefetchit0Emitter<A> {
    fn prefetchit0(&mut self, op0: A);
}

impl<'a> Prefetchit0Emitter<Mem> for Assembler<'a> {
    fn prefetchit0(&mut self, op0: Mem) {
        self.emit(PREFETCHIT0M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `PREFETCHIT1`.
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
pub trait Prefetchit1Emitter<A> {
    fn prefetchit1(&mut self, op0: A);
}

impl<'a> Prefetchit1Emitter<Mem> for Assembler<'a> {
    fn prefetchit1(&mut self, op0: Mem) {
        self.emit(PREFETCHIT1M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `PREFETCHIT0`.
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
    pub fn prefetchit0<A>(&mut self, op0: A)
    where Assembler<'a>: Prefetchit0Emitter<A> {
        <Self as Prefetchit0Emitter<A>>::prefetchit0(self, op0);
    }
    /// `PREFETCHIT1`.
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
    pub fn prefetchit1<A>(&mut self, op0: A)
    where Assembler<'a>: Prefetchit1Emitter<A> {
        <Self as Prefetchit1Emitter<A>>::prefetchit1(self, op0);
    }
}
