pub trait X86MONITOREmitter: Emitter {
    /// Emits `MONITOR`.
    fn monitor(&mut self,) -> () {
        self.emit(MONITOR, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `MWAIT`.
    fn mwait(&mut self,) -> () {
        self.emit(MWAIT, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
