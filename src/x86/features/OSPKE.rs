pub trait X86OSPKEEmitter: Emitter {
    /// Emits `RDPKRU`.
    fn rdpkru(&mut self,) -> () {
        self.emit(RDPKRU, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `WRPKRU`.
    fn wrpkru(&mut self,) -> () {
        self.emit(WRPKRU, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
