pub trait X86PREFETCHIEmitter: Emitter {
    /// Emits `PREFETCHIT1M`.
    fn prefetchit1m(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHIT1M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `PREFETCHIT0M`.
    fn prefetchit0m(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHIT0M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
