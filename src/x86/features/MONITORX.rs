use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `MONITORX`.
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
pub trait MonitorxEmitter {
    fn monitorx(&mut self);
}

impl<'a> MonitorxEmitter for Assembler<'a> {
    fn monitorx(&mut self) {
        self.emit(MONITORX, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `MWAITX`.
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
pub trait MwaitxEmitter {
    fn mwaitx(&mut self);
}

impl<'a> MwaitxEmitter for Assembler<'a> {
    fn mwaitx(&mut self) {
        self.emit(MWAITX, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `MONITORX`.
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
    pub fn monitorx(&mut self)
    where Assembler<'a>: MonitorxEmitter {
        <Self as MonitorxEmitter>::monitorx(self);
    }
    /// `MWAITX`.
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
    pub fn mwaitx(&mut self)
    where Assembler<'a>: MwaitxEmitter {
        <Self as MwaitxEmitter>::mwaitx(self);
    }
}
