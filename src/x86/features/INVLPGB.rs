pub trait X86INVLPGBEmitter: Emitter {
    /// Emits `INVLPGB`.
    fn invlpgb(&mut self,) -> Result<(), AsmError> {
        self.emit(INVLPGB, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `TLBSYNC`.
    fn tlbsync(&mut self,) -> Result<(), AsmError> {
        self.emit(TLBSYNC, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
