pub trait X86PREFETCHEmitter: Emitter {
    /// Emits `PREFETCHM`.
    fn prefetch(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(PREFETCHM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
