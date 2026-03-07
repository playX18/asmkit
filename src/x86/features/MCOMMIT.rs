pub trait X86MCOMMITEmitter: Emitter {
    /// Emits `MCOMMIT`.
    fn mcommit(&mut self,) -> Result<(), AsmError> {
        self.emit(MCOMMIT, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
