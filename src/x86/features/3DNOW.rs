pub trait X863DNOWEmitter: Emitter {
    /// Emits `FEMMS`.
    fn femms(&mut self,) -> () {
        self.emit(FEMMS, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `_3DNOWRRI`.
    fn _3dnowrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(_3DNOWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `_3DNOWRMI`.
    fn _3dnowrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(_3DNOWRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
