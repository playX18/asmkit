pub trait X86SEAMEmitter: Emitter {
    /// Emits `TDCALL`.
    fn tdcall(&mut self,) -> () {
        self.emit(TDCALL, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SEAMRET`.
    fn seamret(&mut self,) -> () {
        self.emit(SEAMRET, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SEAMOPS`.
    fn seamops(&mut self,) -> () {
        self.emit(SEAMOPS, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SEAMCALL`.
    fn seamcall(&mut self,) -> () {
        self.emit(SEAMCALL, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
