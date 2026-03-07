pub trait X86HRESETEmitter: Emitter {
    /// Emits `HRESETI`.
    fn hreset(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(HRESETI, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
