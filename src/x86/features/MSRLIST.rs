pub trait X86MSRLISTEmitter: Emitter {
    /// Emits `RDMSRLIST`.
    fn rdmsrlist(&mut self,) -> () {
        self.emit(RDMSRLIST, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `WRMSRLIST`.
    fn wrmsrlist(&mut self,) -> () {
        self.emit(WRMSRLIST, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
