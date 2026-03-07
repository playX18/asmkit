pub trait X86XSAVECEmitter: Emitter {
    /// Emits `XSAVEC32M`.
    fn xsavec32(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVEC32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XSAVEC64M`.
    fn xsavec64(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVEC64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
