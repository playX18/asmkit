use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `HRESET` (HRESET). 
/// Requests the processor to selectively reset selected components of hardware history maintained by the current logical processor. HRESET operation is controlled by the implicit EAX operand. The value of the explicit imm8 operand is ignored. This instruction can only be executed at privilege level 0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HRESET.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Imm      |
/// +---+----------+
/// ```
pub trait HresetEmitter<A> {
    fn hreset(&mut self, op0: A);
}

impl<'a> HresetEmitter<Imm> for Assembler<'a> {
    fn hreset(&mut self, op0: Imm) {
        self.emit(HRESETI, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `HRESET` (HRESET). 
    /// Requests the processor to selectively reset selected components of hardware history maintained by the current logical processor. HRESET operation is controlled by the implicit EAX operand. The value of the explicit imm8 operand is ignored. This instruction can only be executed at privilege level 0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HRESET.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Imm      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn hreset<A>(&mut self, op0: A)
    where Assembler<'a>: HresetEmitter<A> {
        <Self as HresetEmitter<A>>::hreset(self, op0);
    }
}
