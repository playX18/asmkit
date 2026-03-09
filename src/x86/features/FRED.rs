use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `ERETS`.
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
pub trait EretsEmitter {
    fn erets(&mut self);
}

impl<'a> EretsEmitter for Assembler<'a> {
    fn erets(&mut self) {
        self.emit(ERETS, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `ERETU`.
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
pub trait EretuEmitter {
    fn eretu(&mut self);
}

impl<'a> EretuEmitter for Assembler<'a> {
    fn eretu(&mut self) {
        self.emit(ERETU, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `LKGS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Mem      |
/// +---+----------+
/// ```
pub trait LkgsEmitter<A> {
    fn lkgs(&mut self, op0: A);
}

impl<'a> LkgsEmitter<Gpd> for Assembler<'a> {
    fn lkgs(&mut self, op0: Gpd) {
        self.emit(LKGSR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> LkgsEmitter<Mem> for Assembler<'a> {
    fn lkgs(&mut self, op0: Mem) {
        self.emit(LKGSM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `ERETS`.
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
    pub fn erets(&mut self)
    where
        Assembler<'a>: EretsEmitter,
    {
        <Self as EretsEmitter>::erets(self);
    }
    /// `ERETU`.
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
    pub fn eretu(&mut self)
    where
        Assembler<'a>: EretuEmitter,
    {
        <Self as EretuEmitter>::eretu(self);
    }
    /// `LKGS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn lkgs<A>(&mut self, op0: A)
    where
        Assembler<'a>: LkgsEmitter<A>,
    {
        <Self as LkgsEmitter<A>>::lkgs(self, op0);
    }
}
