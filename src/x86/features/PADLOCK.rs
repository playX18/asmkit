pub trait X86PADLOCKEmitter: Emitter {
    /// Emits `XSTORE`.
    fn xstore(&mut self,) -> () {
        self.emit(XSTORE, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
