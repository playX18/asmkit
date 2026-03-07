pub trait X86PCLMULQDQEmitter: Emitter {
    /// Emits `SSE_PCLMULQDQRRI` (`PCLMULQDQ`). Performs a carry-less multiplication of two quadwords, selected from the first source and second source operand according to the value of the immediate byte. Bits 4 and 0 are used to select which 64-bit half of each operand to use according to Table 4-13, other bits of the immediate byte are ignored.
    /// Reference: [Intel x86 docs for PCLMULQDQ](https://www.felixcloutier.com/x86/PCLMULQDQ.html)
    fn sse_pclmulqdqrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PCLMULQDQRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PCLMULQDQRMI` (`PCLMULQDQ`). Performs a carry-less multiplication of two quadwords, selected from the first source and second source operand according to the value of the immediate byte. Bits 4 and 0 are used to select which 64-bit half of each operand to use according to Table 4-13, other bits of the immediate byte are ignored.
    /// Reference: [Intel x86 docs for PCLMULQDQ](https://www.felixcloutier.com/x86/PCLMULQDQ.html)
    fn sse_pclmulqdqrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PCLMULQDQRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
