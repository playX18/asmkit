pub trait X86SEVESEmitter: Emitter {
    /// Emits `VMGEXIT`.
    fn vmgexit(&mut self,) -> () {
        self.emit(VMGEXIT, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
