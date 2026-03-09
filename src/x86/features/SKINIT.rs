use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CLGI`.
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
pub trait ClgiEmitter {
    fn clgi(&mut self);
}

impl<'a> ClgiEmitter for Assembler<'a> {
    fn clgi(&mut self) {
        self.emit(CLGI, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `SKINIT`.
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
pub trait SkinitEmitter {
    fn skinit(&mut self);
}

impl<'a> SkinitEmitter for Assembler<'a> {
    fn skinit(&mut self) {
        self.emit(SKINIT, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `STGI`.
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
pub trait StgiEmitter {
    fn stgi(&mut self);
}

impl<'a> StgiEmitter for Assembler<'a> {
    fn stgi(&mut self) {
        self.emit(STGI, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `CLGI`.
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
    pub fn clgi(&mut self)
    where
        Assembler<'a>: ClgiEmitter,
    {
        <Self as ClgiEmitter>::clgi(self);
    }
    /// `SKINIT`.
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
    pub fn skinit(&mut self)
    where
        Assembler<'a>: SkinitEmitter,
    {
        <Self as SkinitEmitter>::skinit(self);
    }
    /// `STGI`.
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
    pub fn stgi(&mut self)
    where
        Assembler<'a>: StgiEmitter,
    {
        <Self as StgiEmitter>::stgi(self);
    }
}
