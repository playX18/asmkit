pub trait X86CETEmitter: Emitter {
    /// Emits `CLRSSBSYM`.
    fn clrssbsy(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(CLRSSBSYM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `ENDBR32`.
    fn endbr32(&mut self,) -> Result<(), AsmError> {
        self.emit(ENDBR32, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `ENDBR64`.
    fn endbr64(&mut self,) -> Result<(), AsmError> {
        self.emit(ENDBR64, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `INCSSP32R`.
    fn incssp32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(INCSSP32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `INCSSP64R`.
    fn incssp64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(INCSSP64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RDSSP32R`.
    fn rdssp32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(RDSSP32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RDSSP64R`.
    fn rdssp64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(RDSSP64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RSTORSSPM`.
    fn rstorssp(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(RSTORSSPM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `SAVEPREVSSP`.
    fn saveprevssp(&mut self,) -> Result<(), AsmError> {
        self.emit(SAVEPREVSSP, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `SETSSBSY`.
    fn setssbsy(&mut self,) -> Result<(), AsmError> {
        self.emit(SETSSBSY, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `WRSS32MR`.
    fn wrss32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(WRSS32MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `WRSS64MR`.
    fn wrss64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(WRSS64MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `WRUSS32MR`.
    fn wruss32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(WRUSS32MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `WRUSS64MR`.
    fn wruss64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(WRUSS64MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
