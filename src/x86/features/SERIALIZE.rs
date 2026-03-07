pub trait X86SERIALIZEEmitter: Emitter {
    /// Emits `SERIALIZE`.
    fn serialize(&mut self,) -> Result<(), AsmError> {
        self.emit(SERIALIZE, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
