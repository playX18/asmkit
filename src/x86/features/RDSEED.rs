pub trait X86RDSEEDEmitter: Emitter {
    /// Emits `RDSEED16R`.
    fn rdseed16r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDSEED16R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RDSEED32R`.
    fn rdseed32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDSEED32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RDSEED64R`.
    fn rdseed64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDSEED64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
