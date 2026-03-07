pub trait X86XSSEmitter: Emitter {
    /// Emits `XRSTORS32M`.
    fn xrstors32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(XRSTORS32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XRSTORS64M`.
    fn xrstors64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(XRSTORS64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XSAVES32M`.
    fn xsaves32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(XSAVES32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XSAVES64M`.
    fn xsaves64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(XSAVES64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
