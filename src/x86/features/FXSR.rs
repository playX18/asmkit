pub trait X86FXSREmitter: Emitter {
    /// Emits `FXRSTOR32M`.
    fn fxrstor32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FXRSTOR32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FXRSTOR64M`.
    fn fxrstor64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FXRSTOR64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FXSAVE32M`.
    fn fxsave32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FXSAVE32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FXSAVE64M`.
    fn fxsave64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FXSAVE64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
