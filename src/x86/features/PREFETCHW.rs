pub trait X86PREFETCHWEmitter: Emitter {
    /// Emits `PREFETCHWM`.
    fn prefetchw(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(PREFETCHWM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
