use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PCONFIG` (PCONFIG). 
/// The PCONFIG instruction allows software to configure certain platform features. It supports these features with multiple leaf functions, selecting a leaf function using the value in EAX.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCONFIG.html).
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
pub trait PconfigEmitter {
    fn pconfig(&mut self);
}

impl<'a> PconfigEmitter for Assembler<'a> {
    fn pconfig(&mut self) {
        self.emit(PCONFIG, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `PCONFIG` (PCONFIG). 
    /// The PCONFIG instruction allows software to configure certain platform features. It supports these features with multiple leaf functions, selecting a leaf function using the value in EAX.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCONFIG.html).
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
    pub fn pconfig(&mut self)
    where Assembler<'a>: PconfigEmitter {
        <Self as PconfigEmitter>::pconfig(self);
    }
}
