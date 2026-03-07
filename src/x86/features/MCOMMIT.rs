pub trait X86MCOMMITEmitter: Emitter {
    /// Emits `MCOMMIT`.
    fn mcommit(&mut self,) -> () {
        self.emit(MCOMMIT, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
