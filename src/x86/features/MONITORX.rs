pub trait X86MONITORXEmitter: Emitter {
    /// Emits `MONITORX`.
    fn monitorx(&mut self,) -> () {
        self.emit(MONITORX, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `MWAITX`.
    fn mwaitx(&mut self,) -> () {
        self.emit(MWAITX, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
