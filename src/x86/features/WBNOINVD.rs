pub trait X86WBNOINVDEmitter: Emitter {
    /// Emits `WBNOINVD` (`WBNOINVD`). The WBNOINVD instruction writes back all modified cache lines in the processor’s internal cache to main memory but does not invalidate (flush) the internal caches.
    /// Reference: [Intel x86 docs for WBNOINVD](https://www.felixcloutier.com/x86/WBNOINVD.html)
    fn wbnoinvd(&mut self,) -> () {
        self.emit(WBNOINVD, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
