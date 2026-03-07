pub trait X86SKINITEmitter: Emitter {
    /// Emits `STGI`.
    fn stgi(&mut self,) -> () {
        self.emit(STGI, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CLGI`.
    fn clgi(&mut self,) -> () {
        self.emit(CLGI, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SKINIT`.
    fn skinit(&mut self,) -> () {
        self.emit(SKINIT, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
