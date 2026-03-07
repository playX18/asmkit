pub trait X86PREFETCHEmitter: Emitter {
    /// Emits `PREFETCHM`.
    fn prefetchm(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
