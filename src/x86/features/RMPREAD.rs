pub trait X86RMPREADEmitter: Emitter {
    /// Emits `RMPREAD`.
    fn rmpread(&mut self,) -> () {
        self.emit(RMPREAD, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
