pub trait X86MSRIMMEmitter: Emitter {
    /// Emits `RDMSRRI`.
    fn rdmsr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(RDMSRRI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `WRMSRNSIR`.
    fn wrmsrns(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(WRMSRNSIR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
