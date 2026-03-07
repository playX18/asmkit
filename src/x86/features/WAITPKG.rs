pub trait X86WAITPKGEmitter: Emitter {
    /// Emits `TPAUSER`.
    fn tpause(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(TPAUSER, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `UMONITOR32R`.
    fn umonitor32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(UMONITOR32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `UMONITOR64R`.
    fn umonitor64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(UMONITOR64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `UMWAITR`.
    fn umwait(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(UMWAITR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
