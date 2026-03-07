pub trait X86RMPQUERYEmitter: Emitter {
    /// Emits `RMPQUERY`.
    fn rmpquery(&mut self,) -> Result<(), AsmError> {
        self.emit(RMPQUERY, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
