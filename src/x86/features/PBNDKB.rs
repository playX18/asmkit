pub trait X86PBNDKBEmitter: Emitter {
    /// Emits `PBNDKB`.
    fn pbndkb(&mut self,) -> () {
        self.emit(PBNDKB, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
