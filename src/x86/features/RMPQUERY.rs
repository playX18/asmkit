pub trait X86RMPQUERYEmitter: Emitter {
    /// Emits `RMPQUERY`.
    fn rmpquery(&mut self,) -> () {
        self.emit(RMPQUERY, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
