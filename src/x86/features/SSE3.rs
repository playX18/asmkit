pub trait X86SSE3Emitter: Emitter {
    /// Emits `SSE_MOVSLDUPRR` (`MOVSLDUP`). Duplicates even-indexed single precision floating-point values from the source operand (the second operand). See Figure 4-4. The source operand is an XMM, YMM or ZMM register or 128, 256 or 512-bit memory location and the destination operand is an XMM, YMM or ZMM register.
    /// Reference: [Intel x86 docs for MOVSLDUP](https://www.felixcloutier.com/x86/MOVSLDUP.html)
    fn sse_movslduprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVSLDUPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVSLDUPRM` (`MOVSLDUP`). Duplicates even-indexed single precision floating-point values from the source operand (the second operand). See Figure 4-4. The source operand is an XMM, YMM or ZMM register or 128, 256 or 512-bit memory location and the destination operand is an XMM, YMM or ZMM register.
    /// Reference: [Intel x86 docs for MOVSLDUP](https://www.felixcloutier.com/x86/MOVSLDUP.html)
    fn sse_movslduprm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVSLDUPRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVDDUPRR` (`MOVDDUP`). For 256-bit or higher versions: Duplicates even-indexed double precision floating-point values from the source operand (the second operand) and into adjacent pair and store to the destination operand (the first operand).
    /// Reference: [Intel x86 docs for MOVDDUP](https://www.felixcloutier.com/x86/MOVDDUP.html)
    fn sse_movdduprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVDDUPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVDDUPRM` (`MOVDDUP`). For 256-bit or higher versions: Duplicates even-indexed double precision floating-point values from the source operand (the second operand) and into adjacent pair and store to the destination operand (the first operand).
    /// Reference: [Intel x86 docs for MOVDDUP](https://www.felixcloutier.com/x86/MOVDDUP.html)
    fn sse_movdduprm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVDDUPRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVSHDUPRR` (`MOVSHDUP`). Duplicates odd-indexed single precision floating-point values from the source operand (the second operand) to adjacent element pair in the destination operand (the first operand). See Figure 4-3. The source operand is an XMM, YMM or ZMM register or 128, 256 or 512-bit memory location and the destination operand is an XMM, YMM or ZMM register.
    /// Reference: [Intel x86 docs for MOVSHDUP](https://www.felixcloutier.com/x86/MOVSHDUP.html)
    fn sse_movshduprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVSHDUPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVSHDUPRM` (`MOVSHDUP`). Duplicates odd-indexed single precision floating-point values from the source operand (the second operand) to adjacent element pair in the destination operand (the first operand). See Figure 4-3. The source operand is an XMM, YMM or ZMM register or 128, 256 or 512-bit memory location and the destination operand is an XMM, YMM or ZMM register.
    /// Reference: [Intel x86 docs for MOVSHDUP](https://www.felixcloutier.com/x86/MOVSHDUP.html)
    fn sse_movshduprm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVSHDUPRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_HADDPDRR` (`HADDPD`). Adds the double precision floating-point values in the high and low quadwords of the destination operand and stores the result in the low quadword of the destination operand.
    /// Reference: [Intel x86 docs for HADDPD](https://www.felixcloutier.com/x86/HADDPD.html)
    fn sse_haddpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_HADDPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_HADDPDRM` (`HADDPD`). Adds the double precision floating-point values in the high and low quadwords of the destination operand and stores the result in the low quadword of the destination operand.
    /// Reference: [Intel x86 docs for HADDPD](https://www.felixcloutier.com/x86/HADDPD.html)
    fn sse_haddpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_HADDPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_HADDPSRR` (`HADDPS`). Adds the single precision floating-point values in the first and second dwords of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for HADDPS](https://www.felixcloutier.com/x86/HADDPS.html)
    fn sse_haddpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_HADDPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_HADDPSRM` (`HADDPS`). Adds the single precision floating-point values in the first and second dwords of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for HADDPS](https://www.felixcloutier.com/x86/HADDPS.html)
    fn sse_haddpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_HADDPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_HSUBPDRR` (`HSUBPD`). The HSUBPD instruction subtracts horizontally the packed double precision floating-point numbers of both operands.
    /// Reference: [Intel x86 docs for HSUBPD](https://www.felixcloutier.com/x86/HSUBPD.html)
    fn sse_hsubpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_HSUBPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_HSUBPDRM` (`HSUBPD`). The HSUBPD instruction subtracts horizontally the packed double precision floating-point numbers of both operands.
    /// Reference: [Intel x86 docs for HSUBPD](https://www.felixcloutier.com/x86/HSUBPD.html)
    fn sse_hsubpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_HSUBPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_HSUBPSRR` (`HSUBPS`). Subtracts the single precision floating-point value in the second dword of the destination operand from the first dword of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for HSUBPS](https://www.felixcloutier.com/x86/HSUBPS.html)
    fn sse_hsubpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_HSUBPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_HSUBPSRM` (`HSUBPS`). Subtracts the single precision floating-point value in the second dword of the destination operand from the first dword of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for HSUBPS](https://www.felixcloutier.com/x86/HSUBPS.html)
    fn sse_hsubpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_HSUBPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDSUBPDRR` (`ADDSUBPD`). Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the corresponding double precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered double precision floating-point values from the second source operand from the corresponding double precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for ADDSUBPD](https://www.felixcloutier.com/x86/ADDSUBPD.html)
    fn sse_addsubpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDSUBPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDSUBPDRM` (`ADDSUBPD`). Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the corresponding double precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered double precision floating-point values from the second source operand from the corresponding double precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for ADDSUBPD](https://www.felixcloutier.com/x86/ADDSUBPD.html)
    fn sse_addsubpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDSUBPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDSUBPSRR` (`ADDSUBPS`). Adds odd-numbered single precision floating-point values of the first source operand (second operand) with the corresponding single precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered single precision floating-point values from the second source operand from the corresponding single precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for ADDSUBPS](https://www.felixcloutier.com/x86/ADDSUBPS.html)
    fn sse_addsubpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDSUBPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDSUBPSRM` (`ADDSUBPS`). Adds odd-numbered single precision floating-point values of the first source operand (second operand) with the corresponding single precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered single precision floating-point values from the second source operand from the corresponding single precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for ADDSUBPS](https://www.felixcloutier.com/x86/ADDSUBPS.html)
    fn sse_addsubpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDSUBPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_LDDQURM` (`LDDQU`). The instruction is functionally similar to (V)MOVDQU ymm/xmm, m256/m128 for loading from memory. That is: 32/16 bytes of data starting at an address specified by the source memory operand (second operand) are fetched from memory and placed in a destination register (first operand). The source operand need not be aligned on a 32/16-byte boundary. Up to 64/32 bytes may be loaded from memory; this is implementation dependent.
    /// Reference: [Intel x86 docs for LDDQU](https://www.felixcloutier.com/x86/LDDQU.html)
    fn sse_lddqurm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_LDDQURM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISTTPM32` (`FISTTP`). FISTTP converts the value in ST into a signed integer using truncation (chop) as rounding mode, transfers the result to the destination, and pop ST. FISTTP accepts word, short integer, and long integer destinations.
    /// Reference: [Intel x86 docs for FISTTP](https://www.felixcloutier.com/x86/FISTTP.html)
    fn fisttpm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTTPM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
