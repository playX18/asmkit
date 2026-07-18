use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PSMASH`.
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
pub trait PsmashEmitter {
    fn psmash(&mut self);
}

impl<'a> PsmashEmitter for Assembler<'a> {
    fn psmash(&mut self) {
        self.emit(PSMASH, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `PVALIDATE`.
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
pub trait PvalidateEmitter {
    fn pvalidate(&mut self);
}

impl<'a> PvalidateEmitter for Assembler<'a> {
    fn pvalidate(&mut self) {
        self.emit(PVALIDATE, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `RMPADJUST`.
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
pub trait RmpadjustEmitter {
    fn rmpadjust(&mut self);
}

impl<'a> RmpadjustEmitter for Assembler<'a> {
    fn rmpadjust(&mut self) {
        self.emit(RMPADJUST, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `RMPUPDATE`.
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
pub trait RmpupdateEmitter {
    fn rmpupdate(&mut self);
}

impl<'a> RmpupdateEmitter for Assembler<'a> {
    fn rmpupdate(&mut self) {
        self.emit(RMPUPDATE, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `PSMASH`.
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
    pub fn psmash(&mut self)
    where
        Assembler<'a>: PsmashEmitter,
    {
        <Self as PsmashEmitter>::psmash(self);
    }
    /// `PVALIDATE`.
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
    pub fn pvalidate(&mut self)
    where
        Assembler<'a>: PvalidateEmitter,
    {
        <Self as PvalidateEmitter>::pvalidate(self);
    }
    /// `RMPADJUST`.
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
    pub fn rmpadjust(&mut self)
    where
        Assembler<'a>: RmpadjustEmitter,
    {
        <Self as RmpadjustEmitter>::rmpadjust(self);
    }
    /// `RMPUPDATE`.
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
    pub fn rmpupdate(&mut self)
    where
        Assembler<'a>: RmpupdateEmitter,
    {
        <Self as RmpupdateEmitter>::rmpupdate(self);
    }
}
