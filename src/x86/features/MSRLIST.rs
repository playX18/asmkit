pub trait X86MSRLISTEmitter: Emitter {
    /// Emits `RDMSRLIST`.
    fn rdmsrlist(&mut self,) -> () {
        self.emit(RDMSRLIST, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WRMSRLIST`.
    fn wrmsrlist(&mut self,) -> () {
        self.emit(WRMSRLIST, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
