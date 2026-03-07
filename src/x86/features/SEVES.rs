pub trait X86SEVESEmitter: Emitter {
    /// Emits `VMGEXIT`.
    fn vmgexit(&mut self,) -> () {
        self.emit(VMGEXIT, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
