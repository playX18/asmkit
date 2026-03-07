pub trait X86SNPEmitter: Emitter {
    /// Emits `RMPADJUST`.
    fn rmpadjust(&mut self,) -> () {
        self.emit(RMPADJUST, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RMPUPDATE`.
    fn rmpupdate(&mut self,) -> () {
        self.emit(RMPUPDATE, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `PSMASH`.
    fn psmash(&mut self,) -> () {
        self.emit(PSMASH, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `PVALIDATE`.
    fn pvalidate(&mut self,) -> () {
        self.emit(PVALIDATE, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
