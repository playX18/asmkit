pub trait X86WBNOINVDEmitter: Emitter {
    /// Emits `WBNOINVD`.
    fn wbnoinvd(&mut self,) -> () {
        self.emit(WBNOINVD, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
