use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XRSTORS` (XRSTORS). 
/// Performs a full or partial restore of processor state components from the XSAVE area located at the memory address specified by the source operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components restored correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and the logical-OR of XCR0 with the IA32_XSS MSR. XRSTORS may be executed only if CPL = 0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XRSTORS.html).
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
pub trait XrstorsEmitter<A> {
    fn xrstors(&mut self, op0: A);
}

impl<'a> XrstorsEmitter<Mem> for Assembler<'a> {
    fn xrstors(&mut self, op0: Mem) {
        self.emit(XRSTORS32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `XSAVES` (XSAVES). 
/// Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), the logicalAND of EDX:EAX and the logical-OR of XCR0 with the IA32_XSS MSR. XSAVES may be executed only if CPL = 0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XSAVES.html).
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
pub trait XsavesEmitter<A> {
    fn xsaves(&mut self, op0: A);
}

impl<'a> XsavesEmitter<Mem> for Assembler<'a> {
    fn xsaves(&mut self, op0: Mem) {
        self.emit(XSAVES32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `XRSTORS` (XRSTORS). 
    /// Performs a full or partial restore of processor state components from the XSAVE area located at the memory address specified by the source operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components restored correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and the logical-OR of XCR0 with the IA32_XSS MSR. XRSTORS may be executed only if CPL = 0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XRSTORS.html).
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
    pub fn xrstors<A>(&mut self, op0: A)
    where Assembler<'a>: XrstorsEmitter<A> {
        <Self as XrstorsEmitter<A>>::xrstors(self, op0);
    }
    /// `XSAVES` (XSAVES). 
    /// Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), the logicalAND of EDX:EAX and the logical-OR of XCR0 with the IA32_XSS MSR. XSAVES may be executed only if CPL = 0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XSAVES.html).
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
    pub fn xsaves<A>(&mut self, op0: A)
    where Assembler<'a>: XsavesEmitter<A> {
        <Self as XsavesEmitter<A>>::xsaves(self, op0);
    }
}
