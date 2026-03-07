pub trait X86SMAPEmitter: Emitter {
    /// Emits `CLAC`.
    fn clac(&mut self,) -> Result<(), AsmError> {
        self.emit(CLAC, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `STAC`.
    fn stac(&mut self,) -> Result<(), AsmError> {
        self.emit(STAC, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
