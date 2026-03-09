use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `INVPCID`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait InvpcidEmitter<A, B> {
    fn invpcid(&mut self, op0: A, op1: B);
}

impl<'a> InvpcidEmitter<Gpq, Mem> for Assembler<'a> {
    fn invpcid(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            INVPCIDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `INVPCID`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn invpcid<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: InvpcidEmitter<A, B>,
    {
        <Self as InvpcidEmitter<A, B>>::invpcid(self, op0, op1);
    }
}
