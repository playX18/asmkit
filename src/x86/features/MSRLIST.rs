pub trait X86MSRLISTEmitter: Emitter {
    /// Emits `RDMSRLIST`.
    fn rdmsrlist(&mut self,) -> Result<(), AsmError> {
        self.emit(RDMSRLIST, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `WRMSRLIST`.
    fn wrmsrlist(&mut self,) -> Result<(), AsmError> {
        self.emit(WRMSRLIST, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
