pub trait X86PCONFIGEmitter: Emitter {
    /// Emits `PCONFIG` (`PCONFIG`). The PCONFIG instruction allows software to configure certain platform features. It supports these features with multiple leaf functions, selecting a leaf function using the value in EAX.
    /// Reference: [Intel x86 docs for PCONFIG](https://www.felixcloutier.com/x86/PCONFIG.html)
    fn pconfig(&mut self,) -> () {
        self.emit(PCONFIG, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
