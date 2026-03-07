pub trait X86XSAVEEmitter: Emitter {
    /// Emits `XGETBV`.
    fn xgetbv(&mut self,) -> () {
        self.emit(XGETBV, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XRSTOR32M`.
    fn xrstor32(&mut self,op0: impl OperandCast) -> () {
        self.emit(XRSTOR32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XRSTOR64M`.
    fn xrstor64(&mut self,op0: impl OperandCast) -> () {
        self.emit(XRSTOR64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XSAVE32M`.
    fn xsave32(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVE32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XSAVE64M`.
    fn xsave64(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVE64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XSETBV`.
    fn xsetbv(&mut self,) -> () {
        self.emit(XSETBV, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
