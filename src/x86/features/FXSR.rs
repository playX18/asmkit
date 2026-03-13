use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `FXRSTOR` (FXRSTOR). 
/// Reloads the x87 FPU, MMX technology, XMM, and MXCSR registers from the 512-byte memory image specified in the source operand. This data should have been written to memory previously using the FXSAVE instruction, and in the same format as required by the operating modes. The first byte of the data should be located on a 16-byte boundary. There are three distinct layouts of the FXSAVE state map: one for legacy and compatibility mode, a second format for 64-bit mode FXSAVE/FXRSTOR with REX.W=0, and the third format is for 64-bit mode with FXSAVE64/FXRSTOR64. Table 3-43 shows the layout of the legacy/compatibility mode state information in memory and describes the fields in the memory image for the FXRSTOR and FXSAVE instructions. Table 3-46 shows the layout of the 64-bit mode state information when REX.W is set (FXSAVE64/FXRSTOR64). Table 3-47 shows the layout of the 64-bit mode state information when REX.W is clear (FXSAVE/FXRSTOR).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FXRSTOR.html).
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
pub trait FxrstorEmitter<A> {
    fn fxrstor(&mut self, op0: A);
}

impl<'a> FxrstorEmitter<Mem> for Assembler<'a> {
    fn fxrstor(&mut self, op0: Mem) {
        self.emit(FXRSTOR32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FXSAVE` (FXSAVE). 
/// Saves the current state of the x87 FPU, MMX technology, XMM, and MXCSR registers to a 512-byte memory location specified in the destination operand. The content layout of the 512 byte region depends on whether the processor is operating in non-64-bit operating modes or 64-bit sub-mode of IA-32e mode.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FXSAVE.html).
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
pub trait FxsaveEmitter<A> {
    fn fxsave(&mut self, op0: A);
}

impl<'a> FxsaveEmitter<Mem> for Assembler<'a> {
    fn fxsave(&mut self, op0: Mem) {
        self.emit(FXSAVE32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `FXRSTOR` (FXRSTOR). 
    /// Reloads the x87 FPU, MMX technology, XMM, and MXCSR registers from the 512-byte memory image specified in the source operand. This data should have been written to memory previously using the FXSAVE instruction, and in the same format as required by the operating modes. The first byte of the data should be located on a 16-byte boundary. There are three distinct layouts of the FXSAVE state map: one for legacy and compatibility mode, a second format for 64-bit mode FXSAVE/FXRSTOR with REX.W=0, and the third format is for 64-bit mode with FXSAVE64/FXRSTOR64. Table 3-43 shows the layout of the legacy/compatibility mode state information in memory and describes the fields in the memory image for the FXRSTOR and FXSAVE instructions. Table 3-46 shows the layout of the 64-bit mode state information when REX.W is set (FXSAVE64/FXRSTOR64). Table 3-47 shows the layout of the 64-bit mode state information when REX.W is clear (FXSAVE/FXRSTOR).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FXRSTOR.html).
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
    pub fn fxrstor<A>(&mut self, op0: A)
    where Assembler<'a>: FxrstorEmitter<A> {
        <Self as FxrstorEmitter<A>>::fxrstor(self, op0);
    }
    /// `FXSAVE` (FXSAVE). 
    /// Saves the current state of the x87 FPU, MMX technology, XMM, and MXCSR registers to a 512-byte memory location specified in the destination operand. The content layout of the 512 byte region depends on whether the processor is operating in non-64-bit operating modes or 64-bit sub-mode of IA-32e mode.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FXSAVE.html).
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
    pub fn fxsave<A>(&mut self, op0: A)
    where Assembler<'a>: FxsaveEmitter<A> {
        <Self as FxsaveEmitter<A>>::fxsave(self, op0);
    }
}
