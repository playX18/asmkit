use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `RDPID` (RDPID). 
/// Reads the value of the IA32_TSC_AUX MSR (address C0000103H) into the destination register. The value of CS.D and operand-size prefixes (66H and REX.W) do not affect the behavior of the RDPID instruction.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RDPID.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpq      |
/// +---+----------+
/// ```
pub trait RdpidEmitter<A> {
    fn rdpid(&mut self, op0: A);
}

impl<'a> RdpidEmitter<Gpq> for Assembler<'a> {
    fn rdpid(&mut self, op0: Gpq) {
        self.emit(RDPIDR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `RDPID` (RDPID). 
    /// Reads the value of the IA32_TSC_AUX MSR (address C0000103H) into the destination register. The value of CS.D and operand-size prefixes (66H and REX.W) do not affect the behavior of the RDPID instruction.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RDPID.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpq      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn rdpid<A>(&mut self, op0: A)
    where Assembler<'a>: RdpidEmitter<A> {
        <Self as RdpidEmitter<A>>::rdpid(self, op0);
    }
}
