pub trait X86XSAVEEmitter: Emitter {
    /// Emits `XGETBV`.
    fn xgetbv(&mut self,) -> Result<(), AsmError> {
        self.emit(XGETBV, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XRSTOR32M`.
    fn xrstor32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(XRSTOR32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XRSTOR64M`.
    fn xrstor64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(XRSTOR64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XSAVE32M`.
    fn xsave32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(XSAVE32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XSAVE64M`.
    fn xsave64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(XSAVE64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XSETBV`.
    fn xsetbv(&mut self,) -> Result<(), AsmError> {
        self.emit(XSETBV, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
