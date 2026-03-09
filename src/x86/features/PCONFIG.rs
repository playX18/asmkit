use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PCONFIG`.
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
pub trait PconfigEmitter {
    fn pconfig(&mut self);
}

impl<'a> PconfigEmitter for Assembler<'a> {
    fn pconfig(&mut self) {
        self.emit(PCONFIG, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `PCONFIG`.
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
    pub fn pconfig(&mut self)
    where
        Assembler<'a>: PconfigEmitter,
    {
        <Self as PconfigEmitter>::pconfig(self);
    }
}
