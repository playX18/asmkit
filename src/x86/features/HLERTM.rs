pub trait X86HLERTMEmitter: Emitter {
    /// Emits `XABORTI`.
    fn xabort(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(XABORTI, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XBEGIN`.
    fn xbegin(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(XBEGIN, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XEND`.
    fn xend(&mut self,) -> Result<(), AsmError> {
        self.emit(XEND, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XTEST`.
    fn xtest(&mut self,) -> Result<(), AsmError> {
        self.emit(XTEST, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
