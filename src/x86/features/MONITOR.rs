pub trait X86MONITOREmitter: Emitter {
    /// Emits `MONITOR`.
    fn monitor(&mut self,) -> Result<(), AsmError> {
        self.emit(MONITOR, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `MWAIT`.
    fn mwait(&mut self,) -> Result<(), AsmError> {
        self.emit(MWAIT, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
