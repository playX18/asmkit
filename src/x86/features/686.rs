pub trait X86686Emitter: Emitter {
    /// Emits `FCMOVBR`.
    fn fcmovb(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCMOVBR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCMOVBER`.
    fn fcmovbe(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCMOVBER, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCMOVER`.
    fn fcmove(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCMOVER, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCMOVNBR`.
    fn fcmovnb(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCMOVNBR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCMOVNBER`.
    fn fcmovnbe(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCMOVNBER, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCMOVNER`.
    fn fcmovne(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCMOVNER, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCMOVNUR`.
    fn fcmovnu(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCMOVNUR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCMOVUR`.
    fn fcmovu(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCMOVUR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCOMIR`.
    fn fcomi(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCOMIR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCOMIPRR`.
    fn fcomip(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCOMIPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FUCOMIR`.
    fn fucomi(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FUCOMIR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FUCOMIPRR`.
    fn fucomip(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FUCOMIPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RDPMC`.
    fn rdpmc(&mut self,) -> Result<(), AsmError> {
        self.emit(RDPMC, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `SYSENTER`.
    fn sysenter(&mut self,) -> Result<(), AsmError> {
        self.emit(SYSENTER, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `SYSEXIT`.
    fn sysexit(&mut self,) -> Result<(), AsmError> {
        self.emit(SYSEXIT, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
