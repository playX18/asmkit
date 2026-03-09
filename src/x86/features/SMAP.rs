use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CLAC`.
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
pub trait ClacEmitter {
    fn clac(&mut self);
}

impl<'a> ClacEmitter for Assembler<'a> {
    fn clac(&mut self) {
        self.emit(CLAC, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `STAC`.
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
pub trait StacEmitter {
    fn stac(&mut self);
}

impl<'a> StacEmitter for Assembler<'a> {
    fn stac(&mut self) {
        self.emit(STAC, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `CLAC`.
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
    pub fn clac(&mut self)
    where
        Assembler<'a>: ClacEmitter,
    {
        <Self as ClacEmitter>::clac(self);
    }
    /// `STAC`.
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
    pub fn stac(&mut self)
    where
        Assembler<'a>: StacEmitter,
    {
        <Self as StacEmitter>::stac(self);
    }
}
