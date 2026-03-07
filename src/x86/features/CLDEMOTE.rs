pub trait X86CLDEMOTEEmitter: Emitter {
    /// Emits `CLDEMOTEM` (`CLDEMOTE`). Hints to hardware that the cache line that contains the linear address specified with the memory operand should be moved (“demoted”) from the cache(s) closest to the processor core to a level more distant from the processor core. This may accelerate subsequent accesses to the line by other cores in the same coherence domain, especially if the line was written by the core that demotes the line. Moving the line in such a manner is a performance optimization, i.e., it is a hint which does not modify architectural state. Hardware may choose which level in the cache hierarchy to retain the line (e.g., L3 in typical server designs). The source operand is a byte memory location.
    /// Reference: [Intel x86 docs for CLDEMOTE](https://www.felixcloutier.com/x86/CLDEMOTE.html)
    fn cldemotem(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLDEMOTEM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
