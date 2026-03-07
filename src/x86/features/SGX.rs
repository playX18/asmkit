pub trait X86SGXEmitter: Emitter {
    /// Emits `ENCLS`.
    fn encls(&mut self,) -> () {
        self.emit(ENCLS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `ENCLU`.
    fn enclu(&mut self,) -> () {
        self.emit(ENCLU, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `ENCLV`.
    fn enclv(&mut self,) -> () {
        self.emit(ENCLV, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
