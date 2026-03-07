pub trait X86MOVDIR64BEmitter: Emitter {
    /// Emits `MOVDIR64BRM`.
    fn movdir64b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVDIR64BRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
}
