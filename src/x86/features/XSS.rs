pub trait X86XSSEmitter: Emitter {
    /// Emits `XRSTORS32M`.
    fn xrstors32(&mut self,op0: impl OperandCast) -> () {
        self.emit(XRSTORS32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XRSTORS64M`.
    fn xrstors64(&mut self,op0: impl OperandCast) -> () {
        self.emit(XRSTORS64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XSAVES32M`.
    fn xsaves32(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVES32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XSAVES64M`.
    fn xsaves64(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVES64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
