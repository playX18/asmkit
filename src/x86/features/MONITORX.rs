pub trait X86MONITORXEmitter: Emitter {
    /// Emits `MONITORX`.
    fn monitorx(&mut self,) -> () {
        self.emit(MONITORX, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MWAITX`.
    fn mwaitx(&mut self,) -> () {
        self.emit(MWAITX, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
