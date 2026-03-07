pub trait X86SMXEmitter: Emitter {
    /// Emits `GETSEC`.
    fn getsec(&mut self,) -> () {
        self.emit(GETSEC, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
