pub trait X86RDRANDEmitter: Emitter {
    /// Emits `RDRAND16R`.
    fn rdrand16r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDRAND16R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RDRAND32R`.
    fn rdrand32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDRAND32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RDRAND64R`.
    fn rdrand64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDRAND64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
