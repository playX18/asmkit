pub trait X86OSPKEEmitter: Emitter {
    /// Emits `RDPKRU`.
    fn rdpkru(&mut self,) -> Result<(), AsmError> {
        self.emit(RDPKRU, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `WRPKRU`.
    fn wrpkru(&mut self,) -> Result<(), AsmError> {
        self.emit(WRPKRU, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
