pub trait X86SEVESEmitter: Emitter {
    /// Emits `VMGEXIT`.
    fn vmgexit(&mut self,) -> Result<(), AsmError> {
        self.emit(VMGEXIT, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
