pub trait X86FREDEmitter: Emitter {
    /// Emits `LKGSR`.
    fn lkgsr(&mut self,op0: impl OperandCast) -> () {
        self.emit(LKGSR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `LKGSM`.
    fn lkgsm(&mut self,op0: impl OperandCast) -> () {
        self.emit(LKGSM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ERETU`.
    fn eretu(&mut self,) -> () {
        self.emit(ERETU, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ERETS`.
    fn erets(&mut self,) -> () {
        self.emit(ERETS, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
