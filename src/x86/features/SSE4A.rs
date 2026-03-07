pub trait X86SSE4AEmitter: Emitter {
    /// Emits `SSE_EXTRQ`.
    fn sse_extrq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_imm() {
            self.emit(SSE_EXTRQRI, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(SSE_EXTRQRR, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "SSE_EXTRQ" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `SSE_INSERTQRR`.
    fn sse_insertq(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(SSE_INSERTQRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `SSE_INSERTQRRI`.
    fn sse_insertq_3(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        self.emit(SSE_INSERTQRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
