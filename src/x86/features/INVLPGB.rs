pub trait X86INVLPGBEmitter: Emitter {
    /// Emits `INVLPGB`.
    fn invlpgb(&mut self,) -> () {
        self.emit(INVLPGB, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `TLBSYNC`.
    fn tlbsync(&mut self,) -> () {
        self.emit(TLBSYNC, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
