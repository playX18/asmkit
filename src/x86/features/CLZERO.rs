pub trait X86CLZEROEmitter: Emitter {
    /// Emits `CLZERO16R`.
    fn clzero16r(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLZERO16R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CLZERO32R`.
    fn clzero32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLZERO32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CLZERO64R`.
    fn clzero64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLZERO64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
