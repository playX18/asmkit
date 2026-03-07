pub trait X86WRMSRNSEmitter: Emitter {
    /// Emits `WRMSRNS`.
    fn wrmsrns(&mut self,) -> () {
        self.emit(WRMSRNS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
