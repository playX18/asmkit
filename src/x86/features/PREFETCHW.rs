pub trait X86PREFETCHWEmitter: Emitter {
    /// Emits `PREFETCHWM` (`PREFETCHW`). Fetches the cache line of data from memory that contains the byte specified with the source operand to a location in the 1st or 2nd level cache and invalidates other cached instances of the line.
    /// Reference: [Intel x86 docs for PREFETCHW](https://www.felixcloutier.com/x86/PREFETCHW.html)
    fn prefetchwm(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHWM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
