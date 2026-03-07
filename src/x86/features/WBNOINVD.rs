pub trait X86WBNOINVDEmitter: Emitter {
    /// Emits `WBNOINVD`.
    fn wbnoinvd(&mut self,) -> Result<(), AsmError> {
        self.emit(WBNOINVD, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
