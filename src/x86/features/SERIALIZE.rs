pub trait X86SERIALIZEEmitter: Emitter {
    /// Emits `SERIALIZE`.
    fn serialize(&mut self,) -> () {
        self.emit(SERIALIZE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
