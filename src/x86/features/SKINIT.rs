pub trait X86SKINITEmitter: Emitter {
    /// Emits `CLGI`.
    fn clgi(&mut self,) -> () {
        self.emit(CLGI, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SKINIT`.
    fn skinit(&mut self,) -> () {
        self.emit(SKINIT, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `STGI`.
    fn stgi(&mut self,) -> () {
        self.emit(STGI, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
