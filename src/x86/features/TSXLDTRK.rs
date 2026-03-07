pub trait X86TSXLDTRKEmitter: Emitter {
    /// Emits `XRESLDTRK`.
    fn xresldtrk(&mut self,) -> Result<(), AsmError> {
        self.emit(XRESLDTRK, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `XSUSLDTRK`.
    fn xsusldtrk(&mut self,) -> Result<(), AsmError> {
        self.emit(XSUSLDTRK, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
