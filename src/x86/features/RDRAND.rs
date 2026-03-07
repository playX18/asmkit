pub trait X86RDRANDEmitter: Emitter {
    /// Emits `RDRAND16R`.
    fn rdrand16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(RDRAND16R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RDRAND32R`.
    fn rdrand32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(RDRAND32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RDRAND64R`.
    fn rdrand64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(RDRAND64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
