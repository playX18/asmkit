pub trait X86PREFETCHWT1Emitter: Emitter {
    /// Emits `PREFETCHWT1M`.
    fn prefetchwt1(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(PREFETCHWT1M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
