pub trait X86SGXEmitter: Emitter {
    /// Emits `ENCLS`.
    fn encls(&mut self,) -> Result<(), AsmError> {
        self.emit(ENCLS, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `ENCLU`.
    fn enclu(&mut self,) -> Result<(), AsmError> {
        self.emit(ENCLU, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `ENCLV`.
    fn enclv(&mut self,) -> Result<(), AsmError> {
        self.emit(ENCLV, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
