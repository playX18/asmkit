pub trait X86UINTREmitter: Emitter {
    /// Emits `CLUI`.
    fn clui(&mut self,) -> Result<(), AsmError> {
        self.emit(CLUI, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `SENDUIPIR`.
    fn senduipi(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(SENDUIPIR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `STUI`.
    fn stui(&mut self,) -> Result<(), AsmError> {
        self.emit(STUI, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `TESTUI`.
    fn testui(&mut self,) -> Result<(), AsmError> {
        self.emit(TESTUI, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `UIRET`.
    fn uiret(&mut self,) -> Result<(), AsmError> {
        self.emit(UIRET, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
