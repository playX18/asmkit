pub trait X86MONITOREmitter: Emitter {
    /// Emits `MONITOR`.
    fn monitor(&mut self,) -> () {
        self.emit(MONITOR, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MWAIT` (`MWAIT`). MWAIT instruction provides hints to allow the processor to enter an implementation-dependent optimized state. There are two principal targeted usages: address-range monitor and advanced power management. Both usages of MWAIT require the use of the MONITOR instruction.
    /// Reference: [Intel x86 docs for MWAIT](https://www.felixcloutier.com/x86/MWAIT.html)
    fn mwait(&mut self,) -> () {
        self.emit(MWAIT, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
