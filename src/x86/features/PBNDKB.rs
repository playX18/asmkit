pub trait X86PBNDKBEmitter: Emitter {
    /// Emits `PBNDKB`.
    fn pbndkb(&mut self,) -> () {
        self.emit(PBNDKB, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
