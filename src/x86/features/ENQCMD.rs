pub trait X86ENQCMDEmitter: Emitter {
    /// Emits `ENQCMD32RM`.
    fn enqcmd32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENQCMD32RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `ENQCMD64RM`.
    fn enqcmd64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENQCMD64RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `ENQCMDS32RM`.
    fn enqcmds32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENQCMDS32RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `ENQCMDS64RM`.
    fn enqcmds64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENQCMDS64RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
}
