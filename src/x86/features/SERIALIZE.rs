pub trait X86SERIALIZEEmitter: Emitter {
    /// Emits `SERIALIZE` (`SERIALIZE`). Serializes instruction execution. Before the next instruction is fetched and executed, the SERIALIZE instruction ensures that all modifications to flags, registers, and memory by previous instructions are completed, draining all buffered writes to memory. This instruction is also a serializing instruction as defined in the section “Serializing Instructions” in Chapter 9 of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 3A.
    /// Reference: [Intel x86 docs for SERIALIZE](https://www.felixcloutier.com/x86/SERIALIZE.html)
    fn serialize(&mut self,) -> () {
        self.emit(SERIALIZE, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
