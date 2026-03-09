use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XGETBV`.
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
pub trait XgetbvEmitter {
    fn xgetbv(&mut self);
}

impl<'a> XgetbvEmitter for Assembler<'a> {
    fn xgetbv(&mut self) {
        self.emit(XGETBV, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `XRSTOR`.
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
pub trait XrstorEmitter<A> {
    fn xrstor(&mut self, op0: A);
}

impl<'a> XrstorEmitter<Mem> for Assembler<'a> {
    fn xrstor(&mut self, op0: Mem) {
        self.emit(XRSTOR32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `XSAVE`.
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
pub trait XsaveEmitter<A> {
    fn xsave(&mut self, op0: A);
}

impl<'a> XsaveEmitter<Mem> for Assembler<'a> {
    fn xsave(&mut self, op0: Mem) {
        self.emit(XSAVE32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `XSETBV`.
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
pub trait XsetbvEmitter {
    fn xsetbv(&mut self);
}

impl<'a> XsetbvEmitter for Assembler<'a> {
    fn xsetbv(&mut self) {
        self.emit(XSETBV, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `XGETBV`.
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
    pub fn xgetbv(&mut self)
    where
        Assembler<'a>: XgetbvEmitter,
    {
        <Self as XgetbvEmitter>::xgetbv(self);
    }
    /// `XRSTOR`.
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
    pub fn xrstor<A>(&mut self, op0: A)
    where
        Assembler<'a>: XrstorEmitter<A>,
    {
        <Self as XrstorEmitter<A>>::xrstor(self, op0);
    }
    /// `XSAVE`.
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
    pub fn xsave<A>(&mut self, op0: A)
    where
        Assembler<'a>: XsaveEmitter<A>,
    {
        <Self as XsaveEmitter<A>>::xsave(self, op0);
    }
    /// `XSETBV`.
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
    pub fn xsetbv(&mut self)
    where
        Assembler<'a>: XsetbvEmitter,
    {
        <Self as XsetbvEmitter>::xsetbv(self);
    }
}
