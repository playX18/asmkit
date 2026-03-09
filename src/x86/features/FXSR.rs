use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `FXRSTOR`.
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
pub trait FxrstorEmitter<A> {
    fn fxrstor(&mut self, op0: A);
}

impl<'a> FxrstorEmitter<Mem> for Assembler<'a> {
    fn fxrstor(&mut self, op0: Mem) {
        self.emit(FXRSTOR32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FXSAVE`.
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
pub trait FxsaveEmitter<A> {
    fn fxsave(&mut self, op0: A);
}

impl<'a> FxsaveEmitter<Mem> for Assembler<'a> {
    fn fxsave(&mut self, op0: Mem) {
        self.emit(FXSAVE32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `FXRSTOR`.
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
    pub fn fxrstor<A>(&mut self, op0: A)
    where
        Assembler<'a>: FxrstorEmitter<A>,
    {
        <Self as FxrstorEmitter<A>>::fxrstor(self, op0);
    }
    /// `FXSAVE`.
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
    pub fn fxsave<A>(&mut self, op0: A)
    where
        Assembler<'a>: FxsaveEmitter<A>,
    {
        <Self as FxsaveEmitter<A>>::fxsave(self, op0);
    }
}
