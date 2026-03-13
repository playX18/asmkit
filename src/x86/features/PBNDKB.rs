use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PBNDKB`.
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
pub trait PbndkbEmitter {
    fn pbndkb(&mut self);
}

impl<'a> PbndkbEmitter for Assembler<'a> {
    fn pbndkb(&mut self) {
        self.emit(PBNDKB, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `PBNDKB`.
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
    pub fn pbndkb(&mut self)
    where Assembler<'a>: PbndkbEmitter {
        <Self as PbndkbEmitter>::pbndkb(self);
    }
}
