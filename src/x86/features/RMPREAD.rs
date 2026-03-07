pub trait X86RMPREADEmitter: Emitter {
    /// Emits `RMPREAD`.
    fn rmpread(&mut self,) -> Result<(), AsmError> {
        self.emit(RMPREAD, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
