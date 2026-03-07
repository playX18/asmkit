pub trait X86SMAPEmitter: Emitter {
    /// Emits `CLAC`.
    fn clac(&mut self,) -> () {
        self.emit(CLAC, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `STAC`.
    fn stac(&mut self,) -> () {
        self.emit(STAC, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
