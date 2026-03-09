use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XRESLDTRK`.
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
pub trait XresldtrkEmitter {
    fn xresldtrk(&mut self);
}

impl<'a> XresldtrkEmitter for Assembler<'a> {
    fn xresldtrk(&mut self) {
        self.emit(XRESLDTRK, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `XSUSLDTRK`.
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
pub trait XsusldtrkEmitter {
    fn xsusldtrk(&mut self);
}

impl<'a> XsusldtrkEmitter for Assembler<'a> {
    fn xsusldtrk(&mut self) {
        self.emit(XSUSLDTRK, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `XRESLDTRK`.
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
    pub fn xresldtrk(&mut self)
    where
        Assembler<'a>: XresldtrkEmitter,
    {
        <Self as XresldtrkEmitter>::xresldtrk(self);
    }
    /// `XSUSLDTRK`.
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
    pub fn xsusldtrk(&mut self)
    where
        Assembler<'a>: XsusldtrkEmitter,
    {
        <Self as XsusldtrkEmitter>::xsusldtrk(self);
    }
}
