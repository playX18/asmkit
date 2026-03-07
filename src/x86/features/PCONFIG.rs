pub trait X86PCONFIGEmitter: Emitter {
    /// Emits `PCONFIG`.
    fn pconfig(&mut self,) -> () {
        self.emit(PCONFIG, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
