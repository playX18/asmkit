pub trait X86PCLMULQDQEmitter: Emitter {
    /// Emits `SSE_PCLMULQDQ`.
    fn sse_pclmulqdq(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SSE_PCLMULQDQRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SSE_PCLMULQDQRMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "SSE_PCLMULQDQ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
