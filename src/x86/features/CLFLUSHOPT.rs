use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CLFLUSHOPT`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait ClflushoptEmitter<A> {
    fn clflushopt(&mut self, op0: A);
}

impl<'a> ClflushoptEmitter<Mem> for Assembler<'a> {
    fn clflushopt(&mut self, op0: Mem) {
        self.emit(CLFLUSHOPTM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `CLFLUSHOPT`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn clflushopt<A>(&mut self, op0: A)
    where Assembler<'a>: ClflushoptEmitter<A> {
        <Self as ClflushoptEmitter<A>>::clflushopt(self, op0);
    }
}
