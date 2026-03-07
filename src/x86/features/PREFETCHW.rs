pub trait X86PREFETCHWEmitter: Emitter {
    /// Emits `PREFETCHWM`.
    fn prefetchw(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHWM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
