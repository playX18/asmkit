use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `INVLPGB`.
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
pub trait InvlpgbEmitter {
    fn invlpgb(&mut self);
}

impl<'a> InvlpgbEmitter for Assembler<'a> {
    fn invlpgb(&mut self) {
        self.emit(INVLPGB, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `TLBSYNC`.
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
pub trait TlbsyncEmitter {
    fn tlbsync(&mut self);
}

impl<'a> TlbsyncEmitter for Assembler<'a> {
    fn tlbsync(&mut self) {
        self.emit(TLBSYNC, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `INVLPGB`.
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
    pub fn invlpgb(&mut self)
    where
        Assembler<'a>: InvlpgbEmitter,
    {
        <Self as InvlpgbEmitter>::invlpgb(self);
    }
    /// `TLBSYNC`.
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
    pub fn tlbsync(&mut self)
    where
        Assembler<'a>: TlbsyncEmitter,
    {
        <Self as TlbsyncEmitter>::tlbsync(self);
    }
}
