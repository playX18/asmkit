pub trait X86RDTSCPEmitter: Emitter {
    /// Emits `RDTSCP`.
    fn rdtscp(&mut self,) -> () {
        self.emit(RDTSCP, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
