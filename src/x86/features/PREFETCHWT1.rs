pub trait X86PREFETCHWT1Emitter: Emitter {
    /// Emits `PREFETCHWT1M`.
    fn prefetchwt1(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHWT1M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
