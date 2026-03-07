pub trait X86MCOMMITEmitter: Emitter {
    /// Emits `MCOMMIT`.
    fn mcommit(&mut self,) -> () {
        self.emit(MCOMMIT, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
