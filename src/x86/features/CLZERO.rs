pub trait X86CLZEROEmitter: Emitter {
    /// Emits `CLZERO16R`.
    fn clzero16(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLZERO16R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CLZERO32R`.
    fn clzero32(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLZERO32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CLZERO64R`.
    fn clzero64(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLZERO64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
