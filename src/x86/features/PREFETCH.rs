pub trait X86PREFETCHEmitter: Emitter {
    /// Emits `PREFETCHM`.
    fn prefetch(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
