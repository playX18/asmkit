use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `SERIALIZE`.
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
pub trait SerializeEmitter {
    fn serialize(&mut self);
}

impl<'a> SerializeEmitter for Assembler<'a> {
    fn serialize(&mut self) {
        self.emit(SERIALIZE, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `SERIALIZE`.
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
    pub fn serialize(&mut self)
    where
        Assembler<'a>: SerializeEmitter,
    {
        <Self as SerializeEmitter>::serialize(self);
    }
}
