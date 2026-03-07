pub trait X86INVPCIDEmitter: Emitter {
    /// Emits `INVPCIDRM` (`INVPCID`). Invalidates mappings in the translation lookaside buffers (TLBs) and paging-structure caches based on process-context identifier (PCID). (See Section 4.10, “Caching Translation Information,” in the Intel 64 and IA-32 Architecture Software Developer’s Manual, Volume 3A.) Invalidation is based on the INVPCID type specified in the register operand and the INVPCID descriptor specified in the memory operand.
    /// Reference: [Intel x86 docs for INVPCID](https://www.felixcloutier.com/x86/INVPCID.html)
    fn invpcidrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(INVPCIDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
