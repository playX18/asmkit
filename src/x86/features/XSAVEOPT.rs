pub trait X86XSAVEOPTEmitter: Emitter {
    /// Emits `XSAVEOPT32M`.
    fn xsaveopt32(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVEOPT32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XSAVEOPT64M`.
    fn xsaveopt64(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVEOPT64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
