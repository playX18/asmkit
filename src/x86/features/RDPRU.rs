pub trait X86RDPRUEmitter: Emitter {
    /// Emits `RDPRU`.
    fn rdpru(&mut self,) -> () {
        self.emit(RDPRU, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
