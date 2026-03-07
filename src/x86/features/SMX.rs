pub trait X86SMXEmitter: Emitter {
    /// Emits `GETSEC`.
    fn getsec(&mut self,) -> () {
        self.emit(GETSEC, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
