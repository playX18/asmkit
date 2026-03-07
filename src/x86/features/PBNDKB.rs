pub trait X86PBNDKBEmitter: Emitter {
    /// Emits `PBNDKB`.
    fn pbndkb(&mut self,) -> Result<(), AsmError> {
        self.emit(PBNDKB, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
