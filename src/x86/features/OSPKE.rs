use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `RDPKRU`.
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
pub trait RdpkruEmitter {
    fn rdpkru(&mut self);
}

impl<'a> RdpkruEmitter for Assembler<'a> {
    fn rdpkru(&mut self) {
        self.emit(RDPKRU, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `WRPKRU`.
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
pub trait WrpkruEmitter {
    fn wrpkru(&mut self);
}

impl<'a> WrpkruEmitter for Assembler<'a> {
    fn wrpkru(&mut self) {
        self.emit(WRPKRU, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `RDPKRU`.
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
    pub fn rdpkru(&mut self)
    where
        Assembler<'a>: RdpkruEmitter,
    {
        <Self as RdpkruEmitter>::rdpkru(self);
    }
    /// `WRPKRU`.
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
    pub fn wrpkru(&mut self)
    where
        Assembler<'a>: WrpkruEmitter,
    {
        <Self as WrpkruEmitter>::wrpkru(self);
    }
}
