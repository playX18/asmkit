pub trait X86FXSREmitter: Emitter {
    /// Emits `FXRSTOR32M`.
    fn fxrstor32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FXRSTOR32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FXRSTOR64M`.
    fn fxrstor64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FXRSTOR64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FXSAVE32M`.
    fn fxsave32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FXSAVE32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FXSAVE64M`.
    fn fxsave64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FXSAVE64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
