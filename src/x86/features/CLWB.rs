pub trait X86CLWBEmitter: Emitter {
    /// Emits `CLWBM` (`CLWB`). Writes back to memory the cache line (if modified) that contains the linear address specified with the memory operand from any level of the cache hierarchy in the cache coherence domain. The line may be retained in the cache hierarchy in non-modified state. Retaining the line in the cache hierarchy is a performance optimization (treated as a hint by hardware) to reduce the possibility of cache miss on a subsequent access. Hardware may choose to retain the line at any of the levels in the cache hierarchy, and in some cases, may invalidate the line from the cache hierarchy. The source operand is a byte memory location.
    /// Reference: [Intel x86 docs for CLWB](https://www.felixcloutier.com/x86/CLWB.html)
    fn clwbm(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLWBM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
