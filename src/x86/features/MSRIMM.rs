pub trait X86MSRIMMEmitter: Emitter {
    /// Emits `RDMSRRI`.
    fn rdmsrri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(RDMSRRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WRMSRNSIR`.
    fn wrmsrnsir(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(WRMSRNSIR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
