use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VMGEXIT`.
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
pub trait VmgexitEmitter {
    fn vmgexit(&mut self);
}

impl<'a> VmgexitEmitter for Assembler<'a> {
    fn vmgexit(&mut self) {
        self.emit(VMGEXIT, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `VMGEXIT`.
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
    pub fn vmgexit(&mut self)
    where
        Assembler<'a>: VmgexitEmitter,
    {
        <Self as VmgexitEmitter>::vmgexit(self);
    }
}
