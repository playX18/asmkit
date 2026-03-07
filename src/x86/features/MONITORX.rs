pub trait X86MONITORXEmitter: Emitter {
    /// Emits `MONITORX`.
    fn monitorx(&mut self,) -> Result<(), AsmError> {
        self.emit(MONITORX, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `MWAITX`.
    fn mwaitx(&mut self,) -> Result<(), AsmError> {
        self.emit(MWAITX, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
