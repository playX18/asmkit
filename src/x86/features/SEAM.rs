pub trait X86SEAMEmitter: Emitter {
    /// Emits `SEAMCALL`.
    fn seamcall(&mut self,) -> Result<(), AsmError> {
        self.emit(SEAMCALL, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `SEAMOPS`.
    fn seamops(&mut self,) -> Result<(), AsmError> {
        self.emit(SEAMOPS, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `SEAMRET`.
    fn seamret(&mut self,) -> Result<(), AsmError> {
        self.emit(SEAMRET, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `TDCALL`.
    fn tdcall(&mut self,) -> Result<(), AsmError> {
        self.emit(TDCALL, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
