pub trait X86PADLOCKEmitter: Emitter {
    /// Emits `XSTORE`.
    fn xstore(&mut self,) -> () {
        self.emit(XSTORE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
