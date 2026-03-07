pub trait X86MSRIMMEmitter: Emitter {
    /// Emits `RDMSRRI`.
    fn rdmsr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(RDMSRRI, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `WRMSRNSIR`.
    fn wrmsrns(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(WRMSRNSIR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
}
