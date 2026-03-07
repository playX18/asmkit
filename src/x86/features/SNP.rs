pub trait X86SNPEmitter: Emitter {
    /// Emits `PSMASH`.
    fn psmash(&mut self,) -> Result<(), AsmError> {
        self.emit(PSMASH, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `PVALIDATE`.
    fn pvalidate(&mut self,) -> Result<(), AsmError> {
        self.emit(PVALIDATE, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RMPADJUST`.
    fn rmpadjust(&mut self,) -> Result<(), AsmError> {
        self.emit(RMPADJUST, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RMPUPDATE`.
    fn rmpupdate(&mut self,) -> Result<(), AsmError> {
        self.emit(RMPUPDATE, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
