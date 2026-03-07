pub trait X86WRMSRNSEmitter: Emitter {
    /// Emits `WRMSRNS`.
    fn wrmsrns(&mut self,) -> Result<(), AsmError> {
        self.emit(WRMSRNS, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
