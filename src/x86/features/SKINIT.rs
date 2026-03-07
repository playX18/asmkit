pub trait X86SKINITEmitter: Emitter {
    /// Emits `CLGI`.
    fn clgi(&mut self,) -> Result<(), AsmError> {
        self.emit(CLGI, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `SKINIT`.
    fn skinit(&mut self,) -> Result<(), AsmError> {
        self.emit(SKINIT, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `STGI`.
    fn stgi(&mut self,) -> Result<(), AsmError> {
        self.emit(STGI, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
