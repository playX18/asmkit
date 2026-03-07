pub trait X86CLFLUSHOPTEmitter: Emitter {
    /// Emits `CLFLUSHOPTM`.
    fn clflushoptm(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLFLUSHOPTM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
