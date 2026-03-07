pub trait X86RDPRUEmitter: Emitter {
    /// Emits `RDPRU`.
    fn rdpru(&mut self,) -> Result<(), AsmError> {
        self.emit(RDPRU, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
