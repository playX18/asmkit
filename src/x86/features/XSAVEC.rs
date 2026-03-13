use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XSAVEC` (XSAVEC). 
/// Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XSAVEC.html).
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
pub trait XsavecEmitter<A> {
    fn xsavec(&mut self, op0: A);
}

impl<'a> XsavecEmitter<Mem> for Assembler<'a> {
    fn xsavec(&mut self, op0: Mem) {
        self.emit(XSAVEC32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `XSAVEC` (XSAVEC). 
    /// Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XSAVEC.html).
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
    pub fn xsavec<A>(&mut self, op0: A)
    where Assembler<'a>: XsavecEmitter<A> {
        <Self as XsavecEmitter<A>>::xsavec(self, op0);
    }
}
