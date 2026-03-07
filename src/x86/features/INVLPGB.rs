pub trait X86INVLPGBEmitter: Emitter {
    /// Emits `INVLPGB`.
    fn invlpgb(&mut self,) -> () {
        self.emit(INVLPGB, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `TLBSYNC`.
    fn tlbsync(&mut self,) -> () {
        self.emit(TLBSYNC, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
