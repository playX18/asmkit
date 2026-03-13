use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `SEAMCALL`.
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
pub trait SeamcallEmitter {
    fn seamcall(&mut self);
}

impl<'a> SeamcallEmitter for Assembler<'a> {
    fn seamcall(&mut self) {
        self.emit(SEAMCALL, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `SEAMOPS`.
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
pub trait SeamopsEmitter {
    fn seamops(&mut self);
}

impl<'a> SeamopsEmitter for Assembler<'a> {
    fn seamops(&mut self) {
        self.emit(SEAMOPS, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `SEAMRET`.
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
pub trait SeamretEmitter {
    fn seamret(&mut self);
}

impl<'a> SeamretEmitter for Assembler<'a> {
    fn seamret(&mut self) {
        self.emit(SEAMRET, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `TDCALL`.
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
pub trait TdcallEmitter {
    fn tdcall(&mut self);
}

impl<'a> TdcallEmitter for Assembler<'a> {
    fn tdcall(&mut self) {
        self.emit(TDCALL, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `SEAMCALL`.
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
    pub fn seamcall(&mut self)
    where Assembler<'a>: SeamcallEmitter {
        <Self as SeamcallEmitter>::seamcall(self);
    }
    /// `SEAMOPS`.
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
    pub fn seamops(&mut self)
    where Assembler<'a>: SeamopsEmitter {
        <Self as SeamopsEmitter>::seamops(self);
    }
    /// `SEAMRET`.
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
    pub fn seamret(&mut self)
    where Assembler<'a>: SeamretEmitter {
        <Self as SeamretEmitter>::seamret(self);
    }
    /// `TDCALL`.
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
    pub fn tdcall(&mut self)
    where Assembler<'a>: TdcallEmitter {
        <Self as TdcallEmitter>::tdcall(self);
    }
}
