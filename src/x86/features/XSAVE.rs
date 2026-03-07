pub trait X86XSAVEEmitter: Emitter {
    /// Emits `XGETBV` (`XGETBV`). Reads the contents of the extended control register (XCR) specified in the ECX register into registers EDX:EAX. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The EDX register is loaded with the high-order 32 bits of the XCR and the EAX register is loaded with the low-order 32 bits. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.) If fewer than 64 bits are implemented in the XCR being read, the values returned to EDX:EAX in unimplemented bit locations are undefined.
    /// Reference: [Intel x86 docs for XGETBV](https://www.felixcloutier.com/x86/XGETBV.html)
    fn xgetbv(&mut self,) -> () {
        self.emit(XGETBV, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XSETBV` (`XSETBV`). Writes the contents of registers EDX:EAX into the 64-bit extended control register (XCR) specified in the ECX register. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The contents of the EDX register are copied to high-order 32 bits of the selected XCR and the contents of the EAX register are copied to low-order 32 bits of the XCR. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are ignored.) Undefined or reserved bits in an XCR should be set to values previously read.
    /// Reference: [Intel x86 docs for XSETBV](https://www.felixcloutier.com/x86/XSETBV.html)
    fn xsetbv(&mut self,) -> () {
        self.emit(XSETBV, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XSAVE32M` (`XSAVE`). Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
    /// Reference: [Intel x86 docs for XSAVE](https://www.felixcloutier.com/x86/XSAVE.html)
    fn xsave32m(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVE32M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XSAVE64M` (`XSAVE`). Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
    /// Reference: [Intel x86 docs for XSAVE](https://www.felixcloutier.com/x86/XSAVE.html)
    fn xsave64m(&mut self,op0: impl OperandCast) -> () {
        self.emit(XSAVE64M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XRSTOR32M` (`XRSTOR`). Performs a full or partial restore of processor state components from the XSAVE area located at the memory address specified by the source operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components restored correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
    /// Reference: [Intel x86 docs for XRSTOR](https://www.felixcloutier.com/x86/XRSTOR.html)
    fn xrstor32m(&mut self,op0: impl OperandCast) -> () {
        self.emit(XRSTOR32M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XRSTOR64M` (`XRSTOR`). Performs a full or partial restore of processor state components from the XSAVE area located at the memory address specified by the source operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components restored correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
    /// Reference: [Intel x86 docs for XRSTOR](https://www.felixcloutier.com/x86/XRSTOR.html)
    fn xrstor64m(&mut self,op0: impl OperandCast) -> () {
        self.emit(XRSTOR64M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
