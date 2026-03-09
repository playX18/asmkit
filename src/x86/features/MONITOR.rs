use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `MONITOR`.
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
pub trait MonitorEmitter {
    fn monitor(&mut self);
}

impl<'a> MonitorEmitter for Assembler<'a> {
    fn monitor(&mut self) {
        self.emit(MONITOR, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `MWAIT`.
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
pub trait MwaitEmitter {
    fn mwait(&mut self);
}

impl<'a> MwaitEmitter for Assembler<'a> {
    fn mwait(&mut self) {
        self.emit(MWAIT, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `MONITOR`.
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
    pub fn monitor(&mut self)
    where
        Assembler<'a>: MonitorEmitter,
    {
        <Self as MonitorEmitter>::monitor(self);
    }
    /// `MWAIT`.
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
    pub fn mwait(&mut self)
    where
        Assembler<'a>: MwaitEmitter,
    {
        <Self as MwaitEmitter>::mwait(self);
    }
}
