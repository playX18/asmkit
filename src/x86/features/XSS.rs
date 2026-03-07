pub trait X86XSSEmitter: Emitter {
    /// Emits `XRSTORS32M` (`XRSTORS`). Performs a full or partial restore of processor state components from the XSAVE area located at the memory address specified by the source operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components restored correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and the logical-OR of XCR0 with the IA32_XSS MSR. XRSTORS may be executed only if CPL = 0.
    /// Reference: [Intel x86 docs for XRSTORS](https://www.felixcloutier.com/x86/XRSTORS.html)
    fn xrstors32m(&mut self,op0: impl OperandCast) -> () {
        self.emit(XRSTORS32M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XRSTORS64M` (`XRSTORS`). Performs a full or partial restore of processor state components from the XSAVE area located at the memory address specified by the source operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components restored correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and the logical-OR of XCR0 with the IA32_XSS MSR. XRSTORS may be executed only if CPL = 0.
    /// Reference: [Intel x86 docs for XRSTORS](https://www.felixcloutier.com/x86/XRSTORS.html)
    fn xrstors64m(&mut self,op0: impl OperandCast) -> () {
        self.emit(XRSTORS64M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XSAVES32M` (`XSAVES`). Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), the logicalAND of EDX:EAX and the logical-OR of XCR0 with the IA32_XSS MSR. XSAVES may be executed only if CPL = 0.
    /// Reference: [Intel x86 docs for XSAVES](https://www.felixcloutier.com/x86/XSAVES.html)
    fn xsaves32m(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVES32M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XSAVES64M` (`XSAVES`). Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), the logicalAND of EDX:EAX and the logical-OR of XCR0 with the IA32_XSS MSR. XSAVES may be executed only if CPL = 0.
    /// Reference: [Intel x86 docs for XSAVES](https://www.felixcloutier.com/x86/XSAVES.html)
    fn xsaves64m(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVES64M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
