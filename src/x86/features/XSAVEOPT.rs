pub trait X86XSAVEOPTEmitter: Emitter {
    /// Emits `XSAVEOPT32M` (`XSAVEOPT`). Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
    /// Reference: [Intel x86 docs for XSAVEOPT](https://www.felixcloutier.com/x86/XSAVEOPT.html)
    fn xsaveopt32m(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVEOPT32M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XSAVEOPT64M` (`XSAVEOPT`). Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
    /// Reference: [Intel x86 docs for XSAVEOPT](https://www.felixcloutier.com/x86/XSAVEOPT.html)
    fn xsaveopt64m(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVEOPT64M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
