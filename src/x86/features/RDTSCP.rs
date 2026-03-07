pub trait X86RDTSCPEmitter: Emitter {
    /// Emits `RDTSCP`.
    fn rdtscp(&mut self,) -> Result<(), AsmError> {
        self.emit(RDTSCP, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
