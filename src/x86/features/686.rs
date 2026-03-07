pub trait X86686Emitter: Emitter {
    /// Emits `FCMOVBR`.
    fn fcmovb(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVBR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCMOVBER`.
    fn fcmovbe(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVBER, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCMOVER`.
    fn fcmove(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVER, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCMOVNBR`.
    fn fcmovnb(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVNBR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCMOVNBER`.
    fn fcmovnbe(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVNBER, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCMOVNER`.
    fn fcmovne(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVNER, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCMOVNUR`.
    fn fcmovnu(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVNUR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCMOVUR`.
    fn fcmovu(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVUR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCOMIR`.
    fn fcomi(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCOMIR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCOMIPRR`.
    fn fcomip(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FCOMIPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FUCOMIR`.
    fn fucomi(&mut self,op0: impl OperandCast) -> () {
        self.emit(FUCOMIR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FUCOMIPRR`.
    fn fucomip(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FUCOMIPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `RDPMC`.
    fn rdpmc(&mut self,) -> () {
        self.emit(RDPMC, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SYSENTER`.
    fn sysenter(&mut self,) -> () {
        self.emit(SYSENTER, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SYSEXIT`.
    fn sysexit(&mut self,) -> () {
        self.emit(SYSEXIT, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
