use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XABORT`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Imm      |
/// +---+----------+
/// ```
pub trait XabortEmitter<A> {
    fn xabort(&mut self, op0: A);
}

impl<'a> XabortEmitter<Imm> for Assembler<'a> {
    fn xabort(&mut self, op0: Imm) {
        self.emit(XABORTI, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `XBEGIN`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Imm      |
/// | 2 | Label    |
/// | 3 | Sym      |
/// +---+----------+
/// ```
pub trait XbeginEmitter<A> {
    fn xbegin(&mut self, op0: A);
}

impl<'a> XbeginEmitter<Imm> for Assembler<'a> {
    fn xbegin(&mut self, op0: Imm) {
        self.emit(XBEGIN, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> XbeginEmitter<Sym> for Assembler<'a> {
    fn xbegin(&mut self, op0: Sym) {
        self.emit(XBEGIN, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> XbeginEmitter<Label> for Assembler<'a> {
    fn xbegin(&mut self, op0: Label) {
        self.emit(XBEGIN, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `XEND`.
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
pub trait XendEmitter {
    fn xend(&mut self);
}

impl<'a> XendEmitter for Assembler<'a> {
    fn xend(&mut self) {
        self.emit(XEND, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `XTEST`.
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
pub trait XtestEmitter {
    fn xtest(&mut self);
}

impl<'a> XtestEmitter for Assembler<'a> {
    fn xtest(&mut self) {
        self.emit(XTEST, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `XABORT`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Imm      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn xabort<A>(&mut self, op0: A)
    where
        Assembler<'a>: XabortEmitter<A>,
    {
        <Self as XabortEmitter<A>>::xabort(self, op0);
    }
    /// `XBEGIN`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Imm      |
    /// | 2 | Label    |
    /// | 3 | Sym      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn xbegin<A>(&mut self, op0: A)
    where
        Assembler<'a>: XbeginEmitter<A>,
    {
        <Self as XbeginEmitter<A>>::xbegin(self, op0);
    }
    /// `XEND`.
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
    pub fn xend(&mut self)
    where
        Assembler<'a>: XendEmitter,
    {
        <Self as XendEmitter>::xend(self);
    }
    /// `XTEST`.
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
    pub fn xtest(&mut self)
    where
        Assembler<'a>: XtestEmitter,
    {
        <Self as XtestEmitter>::xtest(self);
    }
}
