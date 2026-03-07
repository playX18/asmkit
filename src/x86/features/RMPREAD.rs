pub trait X86RMPREADEmitter: Emitter {
    /// Emits `RMPREAD`.
    fn rmpread(&mut self,) -> () {
        self.emit(RMPREAD, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
