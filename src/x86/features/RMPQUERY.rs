pub trait X86RMPQUERYEmitter: Emitter {
    /// Emits `RMPQUERY`.
    fn rmpquery(&mut self,) -> () {
        self.emit(RMPQUERY, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
