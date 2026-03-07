pub trait X86USERMSREmitter: Emitter {
    /// Emits `URDMSRRR`.
    fn urdmsrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(URDMSRRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `UWRMSRRR`.
    fn uwrmsrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(UWRMSRRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `URDMSRRI`.
    fn urdmsrri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(URDMSRRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `UWRMSRIR`.
    fn uwrmsrir(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(UWRMSRIR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
