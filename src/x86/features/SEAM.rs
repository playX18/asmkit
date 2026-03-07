pub trait X86SEAMEmitter: Emitter {
    /// Emits `SEAMCALL`.
    fn seamcall(&mut self,) -> () {
        self.emit(SEAMCALL, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SEAMOPS`.
    fn seamops(&mut self,) -> () {
        self.emit(SEAMOPS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SEAMRET`.
    fn seamret(&mut self,) -> () {
        self.emit(SEAMRET, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `TDCALL`.
    fn tdcall(&mut self,) -> () {
        self.emit(TDCALL, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
