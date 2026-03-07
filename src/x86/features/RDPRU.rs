pub trait X86RDPRUEmitter: Emitter {
    /// Emits `RDPRU`.
    fn rdpru(&mut self,) -> () {
        self.emit(RDPRU, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
