pub trait X86PREFETCHIEmitter: Emitter {
    /// Emits `PREFETCHIT0M`.
    fn prefetchit0(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(PREFETCHIT0M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `PREFETCHIT1M`.
    fn prefetchit1(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(PREFETCHIT1M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
