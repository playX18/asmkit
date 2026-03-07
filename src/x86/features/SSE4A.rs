pub trait X86SSE4AEmitter: Emitter {
    /// Emits `SSE_EXTRQRI`.
    fn sse_extrqri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_EXTRQRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_INSERTQRRI`.
    fn sse_insertqrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_INSERTQRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_EXTRQRR`.
    fn sse_extrqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_EXTRQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_INSERTQRR`.
    fn sse_insertqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_INSERTQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
