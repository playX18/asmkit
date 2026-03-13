use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PTWRITE` (PTWRITE). 
/// This instruction reads data in the source operand and sends it to the Intel Processor Trace hardware to be encoded in a PTW packet if TriggerEn, ContextEn, FilterEn, and PTWEn are all set to 1. For more details on these values, see Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 3C, Section 33.2.2, “Software Trace Instrumentation with PTWRITE.” The size of data is 64-bit if using REX.W in 64-bit mode, otherwise 32-bits of data are copied from the source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PTWRITE.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// | 3 | Mem      |
/// +---+----------+
/// ```
pub trait PtwriteEmitter<A> {
    fn ptwrite(&mut self, op0: A);
}

impl<'a> PtwriteEmitter<Gpd> for Assembler<'a> {
    fn ptwrite(&mut self, op0: Gpd) {
        self.emit(PTWRITE32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> PtwriteEmitter<Mem> for Assembler<'a> {
    fn ptwrite(&mut self, op0: Mem) {
        self.emit(PTWRITE32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> PtwriteEmitter<Gpq> for Assembler<'a> {
    fn ptwrite(&mut self, op0: Gpq) {
        self.emit(PTWRITE64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `PTWRITE` (PTWRITE). 
    /// This instruction reads data in the source operand and sends it to the Intel Processor Trace hardware to be encoded in a PTW packet if TriggerEn, ContextEn, FilterEn, and PTWEn are all set to 1. For more details on these values, see Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 3C, Section 33.2.2, “Software Trace Instrumentation with PTWRITE.” The size of data is 64-bit if using REX.W in 64-bit mode, otherwise 32-bits of data are copied from the source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PTWRITE.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// | 3 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn ptwrite<A>(&mut self, op0: A)
    where Assembler<'a>: PtwriteEmitter<A> {
        <Self as PtwriteEmitter<A>>::ptwrite(self, op0);
    }
}
