use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `RDPRU`.
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
pub trait RdpruEmitter {
    fn rdpru(&mut self);
}

impl<'a> RdpruEmitter for Assembler<'a> {
    fn rdpru(&mut self) {
        self.emit(RDPRU, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `RDPRU`.
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
    pub fn rdpru(&mut self)
    where
        Assembler<'a>: RdpruEmitter,
    {
        <Self as RdpruEmitter>::rdpru(self);
    }
}
