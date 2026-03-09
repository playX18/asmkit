use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `WBNOINVD`.
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
pub trait WbnoinvdEmitter {
    fn wbnoinvd(&mut self);
}

impl<'a> WbnoinvdEmitter for Assembler<'a> {
    fn wbnoinvd(&mut self) {
        self.emit(WBNOINVD, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `WBNOINVD`.
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
    pub fn wbnoinvd(&mut self)
    where
        Assembler<'a>: WbnoinvdEmitter,
    {
        <Self as WbnoinvdEmitter>::wbnoinvd(self);
    }
}
