pub trait X86PADLOCKEmitter: Emitter {
    /// Emits `XSTORE`.
    fn xstore(&mut self,) -> Result<(), AsmError> {
        self.emit(XSTORE, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
