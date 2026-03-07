pub trait X86PREFETCHIEmitter: Emitter {
    /// Emits `PREFETCHIT0M`.
    fn prefetchit0(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHIT0M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `PREFETCHIT1M`.
    fn prefetchit1(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHIT1M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
