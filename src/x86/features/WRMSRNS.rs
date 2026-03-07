pub trait X86WRMSRNSEmitter: Emitter {
    /// Emits `WRMSRNS`.
    fn wrmsrns(&mut self,) -> () {
        self.emit(WRMSRNS, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
