pub trait X86PREFETCHWT1Emitter: Emitter {
    /// Emits `PREFETCHWT1M` (`PREFETCHWT1`). Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by an intent to write hint (so that data is brought into ‘Exclusive’ state via a request for ownership) and a locality hint
    /// Reference: [Intel x86 docs for PREFETCHWT1](https://www.felixcloutier.com/x86/PREFETCHWT1.html)
    fn prefetchwt1m(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHWT1M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
