use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `WRMSRNS`.
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
pub trait WrmsrnsEmitter {
    fn wrmsrns(&mut self);
}

impl<'a> WrmsrnsEmitter for Assembler<'a> {
    fn wrmsrns(&mut self) {
        self.emit(WRMSRNS, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `WRMSRNS`.
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
    pub fn wrmsrns(&mut self)
    where Assembler<'a>: WrmsrnsEmitter {
        <Self as WrmsrnsEmitter>::wrmsrns(self);
    }
}
