use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `RDMSRLIST`.
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
pub trait RdmsrlistEmitter {
    fn rdmsrlist(&mut self);
}

impl<'a> RdmsrlistEmitter for Assembler<'a> {
    fn rdmsrlist(&mut self) {
        self.emit(RDMSRLIST, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `WRMSRLIST`.
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
pub trait WrmsrlistEmitter {
    fn wrmsrlist(&mut self);
}

impl<'a> WrmsrlistEmitter for Assembler<'a> {
    fn wrmsrlist(&mut self) {
        self.emit(WRMSRLIST, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `RDMSRLIST`.
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
    pub fn rdmsrlist(&mut self)
    where
        Assembler<'a>: RdmsrlistEmitter,
    {
        <Self as RdmsrlistEmitter>::rdmsrlist(self);
    }
    /// `WRMSRLIST`.
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
    pub fn wrmsrlist(&mut self)
    where
        Assembler<'a>: WrmsrlistEmitter,
    {
        <Self as WrmsrlistEmitter>::wrmsrlist(self);
    }
}
