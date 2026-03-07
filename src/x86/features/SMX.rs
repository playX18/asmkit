pub trait X86SMXEmitter: Emitter {
    /// Emits `GETSEC`.
    fn getsec(&mut self,) -> Result<(), AsmError> {
        self.emit(GETSEC, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
