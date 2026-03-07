pub trait X86ENQCMDEmitter: Emitter {
    /// Emits `ENQCMD32RM`.
    fn enqcmd32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENQCMD32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ENQCMD64RM`.
    fn enqcmd64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENQCMD64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ENQCMDS32RM`.
    fn enqcmds32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENQCMDS32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ENQCMDS64RM`.
    fn enqcmds64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENQCMDS64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
