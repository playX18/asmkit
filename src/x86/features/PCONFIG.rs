pub trait X86PCONFIGEmitter: Emitter {
    /// Emits `PCONFIG`.
    fn pconfig(&mut self,) -> Result<(), AsmError> {
        self.emit(PCONFIG, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
