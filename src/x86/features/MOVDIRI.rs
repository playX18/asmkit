pub trait X86MOVDIRIEmitter: Emitter {
    /// Emits `MOVDIRI32MR`.
    fn movdiri32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVDIRI32MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `MOVDIRI64MR`.
    fn movdiri64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVDIRI64MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
}
