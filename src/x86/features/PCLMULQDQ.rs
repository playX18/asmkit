use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `SSE_PCLMULQDQ` (PCLMULQDQ). 
/// Performs a carry-less multiplication of two quadwords, selected from the first source and second source operand according to the value of the immediate byte. Bits 4 and 0 are used to select which 64-bit half of each operand to use according to Table 4-13, other bits of the immediate byte are ignored.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCLMULQDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// +---+---------------+
/// ```
pub trait SsePclmulqdqEmitter<A, B, C> {
    fn sse_pclmulqdq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePclmulqdqEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_pclmulqdq(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_PCLMULQDQRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePclmulqdqEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pclmulqdq(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PCLMULQDQRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `SSE_PCLMULQDQ` (PCLMULQDQ). 
    /// Performs a carry-less multiplication of two quadwords, selected from the first source and second source operand according to the value of the immediate byte. Bits 4 and 0 are used to select which 64-bit half of each operand to use according to Table 4-13, other bits of the immediate byte are ignored.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCLMULQDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn sse_pclmulqdq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePclmulqdqEmitter<A, B, C> {
        <Self as SsePclmulqdqEmitter<A, B, C>>::sse_pclmulqdq(self, op0, op1, op2);
    }
}
