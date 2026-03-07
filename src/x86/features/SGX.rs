pub trait X86SGXEmitter: Emitter {
    /// Emits `ENCLV` (`ENCLV`). The ENCLV instruction invokes the virtualization SGX leaf functions for managing enclaves in a virtualized environment. Software specifies the leaf function by setting the appropriate value in the register EAX as input. The registers RBX, RCX, and RDX have leaf-specific purpose, and may act as input, as output, or may be unused. In non 64-bit mode, the instruction ignores upper 32 bits of the RAX register.
    /// Reference: [Intel x86 docs for ENCLV](https://www.felixcloutier.com/x86/ENCLV.html)
    fn enclv(&mut self,) -> () {
        self.emit(ENCLV, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ENCLS`.
    fn encls(&mut self,) -> () {
        self.emit(ENCLS, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ENCLU`.
    fn enclu(&mut self,) -> () {
        self.emit(ENCLU, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
