pub trait X86RDSEEDEmitter: Emitter {
    /// Emits `RDSEED16R`.
    fn rdseed16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(RDSEED16R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RDSEED32R`.
    fn rdseed32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(RDSEED32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RDSEED64R`.
    fn rdseed64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(RDSEED64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
