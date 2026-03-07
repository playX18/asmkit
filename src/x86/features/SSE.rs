pub trait X86SSEEmitter: Emitter {
    /// Emits `PREFETCHNTAM` (`PREFETCHNTA`). Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
    /// Reference: [Intel x86 docs for PREFETCHNTA](https://www.felixcloutier.com/x86/PREFETCHh.html)
    fn prefetchntam(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHNTAM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `PREFETCHT0M` (`PREFETCHT0`). Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
    /// Reference: [Intel x86 docs for PREFETCHT0](https://www.felixcloutier.com/x86/PREFETCHh.html)
    fn prefetcht0m(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHT0M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `PREFETCHT1M` (`PREFETCHT1`). Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
    /// Reference: [Intel x86 docs for PREFETCHT1](https://www.felixcloutier.com/x86/PREFETCHh.html)
    fn prefetcht1m(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHT1M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `PREFETCHT2M` (`PREFETCHT2`). Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
    /// Reference: [Intel x86 docs for PREFETCHT2](https://www.felixcloutier.com/x86/PREFETCHh.html)
    fn prefetcht2m(&mut self,op0: impl OperandCast) -> () {
        self.emit(PREFETCHT2M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSHUFWRRI` (`PSHUFW`). Copies words from the source operand (second operand) and inserts them in the destination operand (first operand) at word locations selected with the order operand (third operand). This operation is similar to the operation used by the PSHUFD instruction, which is illustrated in Figure 4-16. For the PSHUFW instruction, each 2-bit field in the order operand selects the contents of one word location in the destination operand. The encodings of the order operand fields select words from the source operand to be copied to the destination operand.
    /// Reference: [Intel x86 docs for PSHUFW](https://www.felixcloutier.com/x86/PSHUFW.html)
    fn mmx_pshufwrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(MMX_PSHUFWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `MMX_PSHUFWRMI` (`PSHUFW`). Copies words from the source operand (second operand) and inserts them in the destination operand (first operand) at word locations selected with the order operand (third operand). This operation is similar to the operation used by the PSHUFD instruction, which is illustrated in Figure 4-16. For the PSHUFW instruction, each 2-bit field in the order operand selects the contents of one word location in the destination operand. The encodings of the order operand fields select words from the source operand to be copied to the destination operand.
    /// Reference: [Intel x86 docs for PSHUFW](https://www.felixcloutier.com/x86/PSHUFW.html)
    fn mmx_pshufwrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(MMX_PSHUFWRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `MMX_PINSRWRRI` (`PINSRW`). Three operand MMX and SSE instructions
    /// Reference: [Intel x86 docs for PINSRW](https://www.felixcloutier.com/x86/PINSRW.html)
    fn mmx_pinsrwrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(MMX_PINSRWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `MMX_PINSRWRMI` (`PINSRW`). Three operand MMX and SSE instructions
    /// Reference: [Intel x86 docs for PINSRW](https://www.felixcloutier.com/x86/PINSRW.html)
    fn mmx_pinsrwrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(MMX_PINSRWRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `MMX_PEXTRWRRI` (`PEXTRW`). Copies the word in the source operand (second operand) specified by the count operand (third operand) to the destination operand (first operand). The source operand can be an MMX technology register or an XMM register. The destination operand can be the low word of a general-purpose register or a 16-bit memory address. The count operand is an 8-bit immediate. When specifying a word location in an MMX technology register, the 2 least-significant bits of the count operand specify the location; for an XMM register, the 3 least-significant bits specify the location. The content of the destination register above bit 16 is cleared (set to all 0s).
    /// Reference: [Intel x86 docs for PEXTRW](https://www.felixcloutier.com/x86/PEXTRW.html)
    fn mmx_pextrwrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(MMX_PEXTRWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVDQ2QRR`.
    fn mmx_movdq2qrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVDQ2QRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVQ2DQRR` (`MOVQ2DQ`). Moves the quadword from the source operand (second operand) to the low quadword of the destination operand (first operand). The source operand is an MMX technology register and the destination operand is an XMM register.
    /// Reference: [Intel x86 docs for MOVQ2DQ](https://www.felixcloutier.com/x86/MOVQ2DQ.html)
    fn mmx_movq2dqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVQ2DQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMOVMSKBRR` (`PMOVMSKB`). Creates a mask made up of the most significant bit of each byte of the source operand (second operand) and stores the result in the low byte or word of the destination operand (first operand).
    /// Reference: [Intel x86 docs for PMOVMSKB](https://www.felixcloutier.com/x86/PMOVMSKB.html)
    fn mmx_pmovmskbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMOVMSKBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMINUBRR` (`PMINUB`). Performs a SIMD compare of the packed unsigned byte or word integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINUB](https://www.felixcloutier.com/x86/PMINUB%3APMINUW.html)
    fn mmx_pminubrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMINUBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMINUBRM` (`PMINUB`). Performs a SIMD compare of the packed unsigned byte or word integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINUB](https://www.felixcloutier.com/x86/PMINUB%3APMINUW.html)
    fn mmx_pminubrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMINUBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMAXUBRR` (`PMAXUB`). Performs a SIMD compare of the packed unsigned byte, word integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXUB](https://www.felixcloutier.com/x86/PMAXUB%3APMAXUW.html)
    fn mmx_pmaxubrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMAXUBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMAXUBRM` (`PMAXUB`). Performs a SIMD compare of the packed unsigned byte, word integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXUB](https://www.felixcloutier.com/x86/PMAXUB%3APMAXUW.html)
    fn mmx_pmaxubrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMAXUBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PAVGBRR` (`PAVGB`). Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
    /// Reference: [Intel x86 docs for PAVGB](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html)
    fn mmx_pavgbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PAVGBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PAVGBRM` (`PAVGB`). Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
    /// Reference: [Intel x86 docs for PAVGB](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html)
    fn mmx_pavgbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PAVGBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PAVGWRR` (`PAVGW`). Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
    /// Reference: [Intel x86 docs for PAVGW](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html)
    fn mmx_pavgwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PAVGWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PAVGWRM` (`PAVGW`). Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
    /// Reference: [Intel x86 docs for PAVGW](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html)
    fn mmx_pavgwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PAVGWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMULHUWRR` (`PMULHUW`). Performs a SIMD unsigned multiply of the packed unsigned word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each 32-bit intermediate results in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULHUW](https://www.felixcloutier.com/x86/PMULHUW.html)
    fn mmx_pmulhuwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMULHUWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMULHUWRM` (`PMULHUW`). Performs a SIMD unsigned multiply of the packed unsigned word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each 32-bit intermediate results in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULHUW](https://www.felixcloutier.com/x86/PMULHUW.html)
    fn mmx_pmulhuwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMULHUWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVNTQMR` (`MOVNTQ`). Moves the quadword in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to minimize cache pollution during the write to memory. The source operand is an MMX technology register, which is assumed to contain packed integer data (packed bytes, words, or doublewords). The destination operand is a 64-bit memory location.
    /// Reference: [Intel x86 docs for MOVNTQ](https://www.felixcloutier.com/x86/MOVNTQ.html)
    fn mmx_movntqmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVNTQMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMINSWRR` (`PMINSW`). Performs a SIMD compare of the packed signed byte, word, or dword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINSW](https://www.felixcloutier.com/x86/PMINSB%3APMINSW.html)
    fn mmx_pminswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMINSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMINSWRM` (`PMINSW`). Performs a SIMD compare of the packed signed byte, word, or dword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINSW](https://www.felixcloutier.com/x86/PMINSB%3APMINSW.html)
    fn mmx_pminswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMINSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMAXSWRR` (`PMAXSW`). Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXSW](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html)
    fn mmx_pmaxswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMAXSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMAXSWRM` (`PMAXSW`). Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXSW](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html)
    fn mmx_pmaxswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMAXSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSADBWRR` (`PSADBW`). Computes the absolute value of the difference of 8 unsigned byte integers from the source operand (second operand) and from the destination operand (first operand). These 8 differences are then summed to produce an unsigned word integer result that is stored in the destination operand. Figure 4-14 shows the operation of the PSADBW instruction when using 64-bit operands.
    /// Reference: [Intel x86 docs for PSADBW](https://www.felixcloutier.com/x86/PSADBW.html)
    fn mmx_psadbwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSADBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSADBWRM` (`PSADBW`). Computes the absolute value of the difference of 8 unsigned byte integers from the source operand (second operand) and from the destination operand (first operand). These 8 differences are then summed to produce an unsigned word integer result that is stored in the destination operand. Figure 4-14 shows the operation of the PSADBW instruction when using 64-bit operands.
    /// Reference: [Intel x86 docs for PSADBW](https://www.felixcloutier.com/x86/PSADBW.html)
    fn mmx_psadbwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSADBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVUPSRR` (`MOVUPS`). Note: VEX.vvvv and EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVUPS](https://www.felixcloutier.com/x86/MOVUPS.html)
    fn sse_movupsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVUPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVUPSRM` (`MOVUPS`). Note: VEX.vvvv and EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVUPS](https://www.felixcloutier.com/x86/MOVUPS.html)
    fn sse_movupsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVUPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVSSRR` (`MOVSS`). Moves a scalar single precision floating-point value from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be XMM registers or 32-bit memory locations. This instruction can be used to move a single precision floating-point value to and from the low doubleword of an XMM register and a 32-bit memory location, or to move a single precision floating-point value between the low doublewords of two XMM registers. The instruction cannot be used to transfer data between memory locations.
    /// Reference: [Intel x86 docs for MOVSS](https://www.felixcloutier.com/x86/MOVSS.html)
    fn sse_movssrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVSSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVSSRM` (`MOVSS`). Moves a scalar single precision floating-point value from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be XMM registers or 32-bit memory locations. This instruction can be used to move a single precision floating-point value to and from the low doubleword of an XMM register and a 32-bit memory location, or to move a single precision floating-point value between the low doublewords of two XMM registers. The instruction cannot be used to transfer data between memory locations.
    /// Reference: [Intel x86 docs for MOVSS](https://www.felixcloutier.com/x86/MOVSS.html)
    fn sse_movssrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVSSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVUPSMR` (`MOVUPS`). Note: VEX.vvvv and EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVUPS](https://www.felixcloutier.com/x86/MOVUPS.html)
    fn sse_movupsmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVUPSMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVSSMR` (`MOVSS`). Moves a scalar single precision floating-point value from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be XMM registers or 32-bit memory locations. This instruction can be used to move a single precision floating-point value to and from the low doubleword of an XMM register and a 32-bit memory location, or to move a single precision floating-point value between the low doublewords of two XMM registers. The instruction cannot be used to transfer data between memory locations.
    /// Reference: [Intel x86 docs for MOVSS](https://www.felixcloutier.com/x86/MOVSS.html)
    fn sse_movssmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVSSMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVLPSRM` (`MOVLPS`). This instruction cannot be used for register to register or memory to memory moves.
    /// Reference: [Intel x86 docs for MOVLPS](https://www.felixcloutier.com/x86/MOVLPS.html)
    fn sse_movlpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVLPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVHLPSRR` (`MOVHLPS`). This instruction cannot be used for memory to register moves.
    /// Reference: [Intel x86 docs for MOVHLPS](https://www.felixcloutier.com/x86/MOVHLPS.html)
    fn sse_movhlpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVHLPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVLPSMR` (`MOVLPS`). This instruction cannot be used for register to register or memory to memory moves.
    /// Reference: [Intel x86 docs for MOVLPS](https://www.felixcloutier.com/x86/MOVLPS.html)
    fn sse_movlpsmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVLPSMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UNPCKLPSRR` (`UNPCKLPS`). Performs an interleaved unpack of the low single precision floating-point values from the first source operand and the second source operand.
    /// Reference: [Intel x86 docs for UNPCKLPS](https://www.felixcloutier.com/x86/UNPCKLPS.html)
    fn sse_unpcklpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UNPCKLPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UNPCKLPSRM` (`UNPCKLPS`). Performs an interleaved unpack of the low single precision floating-point values from the first source operand and the second source operand.
    /// Reference: [Intel x86 docs for UNPCKLPS](https://www.felixcloutier.com/x86/UNPCKLPS.html)
    fn sse_unpcklpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UNPCKLPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UNPCKHPSRR` (`UNPCKHPS`). Performs an interleaved unpack of the high single precision floating-point values from the first source operand and the second source operand.
    /// Reference: [Intel x86 docs for UNPCKHPS](https://www.felixcloutier.com/x86/UNPCKHPS.html)
    fn sse_unpckhpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UNPCKHPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UNPCKHPSRM` (`UNPCKHPS`). Performs an interleaved unpack of the high single precision floating-point values from the first source operand and the second source operand.
    /// Reference: [Intel x86 docs for UNPCKHPS](https://www.felixcloutier.com/x86/UNPCKHPS.html)
    fn sse_unpckhpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UNPCKHPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVHPSRM` (`MOVHPS`). This instruction cannot be used for register to register or memory to memory moves.
    /// Reference: [Intel x86 docs for MOVHPS](https://www.felixcloutier.com/x86/MOVHPS.html)
    fn sse_movhpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVHPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVLHPSRR` (`MOVLHPS`). This instruction cannot be used for memory to register moves.
    /// Reference: [Intel x86 docs for MOVLHPS](https://www.felixcloutier.com/x86/MOVLHPS.html)
    fn sse_movlhpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVLHPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVHPSMR` (`MOVHPS`). This instruction cannot be used for register to register or memory to memory moves.
    /// Reference: [Intel x86 docs for MOVHPS](https://www.felixcloutier.com/x86/MOVHPS.html)
    fn sse_movhpsmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVHPSMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVAPSRR` (`MOVAPS`). Moves 4, 8 or 16 single precision floating-point values from the source operand (second operand) to the destination operand (first operand). This instruction can be used to load an XMM, YMM or ZMM register from an 128-bit, 256-bit or 512-bit memory location, to store the contents of an XMM, YMM or ZMM register into a 128-bit, 256-bit or 512-bit memory location, or to move data between two XMM, two YMM or two ZMM registers.
    /// Reference: [Intel x86 docs for MOVAPS](https://www.felixcloutier.com/x86/MOVAPS.html)
    fn sse_movapsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVAPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVAPSRM` (`MOVAPS`). Moves 4, 8 or 16 single precision floating-point values from the source operand (second operand) to the destination operand (first operand). This instruction can be used to load an XMM, YMM or ZMM register from an 128-bit, 256-bit or 512-bit memory location, to store the contents of an XMM, YMM or ZMM register into a 128-bit, 256-bit or 512-bit memory location, or to move data between two XMM, two YMM or two ZMM registers.
    /// Reference: [Intel x86 docs for MOVAPS](https://www.felixcloutier.com/x86/MOVAPS.html)
    fn sse_movapsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVAPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVAPSMR` (`MOVAPS`). Moves 4, 8 or 16 single precision floating-point values from the source operand (second operand) to the destination operand (first operand). This instruction can be used to load an XMM, YMM or ZMM register from an 128-bit, 256-bit or 512-bit memory location, to store the contents of an XMM, YMM or ZMM register into a 128-bit, 256-bit or 512-bit memory location, or to move data between two XMM, two YMM or two ZMM registers.
    /// Reference: [Intel x86 docs for MOVAPS](https://www.felixcloutier.com/x86/MOVAPS.html)
    fn sse_movapsmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVAPSMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSI2SS32RR` (`CVTSI2SS`). Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a single precision floating-point value in the destination operand (first operand). The “convert-from” source operand can be a general-purpose register or a memory location. The destination operand is an XMM register. The result is stored in the low doubleword of the destination operand, and the upper three doublewords are left unchanged. When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
    /// Reference: [Intel x86 docs for CVTSI2SS](https://www.felixcloutier.com/x86/CVTSI2SS.html)
    fn sse_cvtsi2ss32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSI2SS32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSI2SS32RM` (`CVTSI2SS`). Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a single precision floating-point value in the destination operand (first operand). The “convert-from” source operand can be a general-purpose register or a memory location. The destination operand is an XMM register. The result is stored in the low doubleword of the destination operand, and the upper three doublewords are left unchanged. When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
    /// Reference: [Intel x86 docs for CVTSI2SS](https://www.felixcloutier.com/x86/CVTSI2SS.html)
    fn sse_cvtsi2ss32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSI2SS32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSI2SS64RR` (`CVTSI2SS`). Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a single precision floating-point value in the destination operand (first operand). The “convert-from” source operand can be a general-purpose register or a memory location. The destination operand is an XMM register. The result is stored in the low doubleword of the destination operand, and the upper three doublewords are left unchanged. When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
    /// Reference: [Intel x86 docs for CVTSI2SS](https://www.felixcloutier.com/x86/CVTSI2SS.html)
    fn sse_cvtsi2ss64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSI2SS64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSI2SS64RM` (`CVTSI2SS`). Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a single precision floating-point value in the destination operand (first operand). The “convert-from” source operand can be a general-purpose register or a memory location. The destination operand is an XMM register. The result is stored in the low doubleword of the destination operand, and the upper three doublewords are left unchanged. When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
    /// Reference: [Intel x86 docs for CVTSI2SS](https://www.felixcloutier.com/x86/CVTSI2SS.html)
    fn sse_cvtsi2ss64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSI2SS64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVNTPSMR` (`MOVNTPS`). Moves the packed single precision floating-point values in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to prevent caching of the data during the write to memory. The source operand is an XMM register, YMM register or ZMM register, which is assumed to contain packed single precision, floating-pointing. The destination operand is a 128-bit, 256-bit or 512-bit memory location. The memory operand must be aligned on a 16-byte (128-bit version), 32-byte (VEX.256 encoded version) or 64-byte (EVEX.512 encoded version) boundary otherwise a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for MOVNTPS](https://www.felixcloutier.com/x86/MOVNTPS.html)
    fn sse_movntpsmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTPSMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVNTSSMR`.
    fn sse_movntssmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTSSMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTSS2SI32RR` (`CVTTSS2SI`). Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 32-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
    /// Reference: [Intel x86 docs for CVTTSS2SI](https://www.felixcloutier.com/x86/CVTTSS2SI.html)
    fn sse_cvttss2si32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTSS2SI32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTSS2SI32RM` (`CVTTSS2SI`). Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 32-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
    /// Reference: [Intel x86 docs for CVTTSS2SI](https://www.felixcloutier.com/x86/CVTTSS2SI.html)
    fn sse_cvttss2si32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTSS2SI32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTSS2SI64RR` (`CVTTSS2SI`). Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 32-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
    /// Reference: [Intel x86 docs for CVTTSS2SI](https://www.felixcloutier.com/x86/CVTTSS2SI.html)
    fn sse_cvttss2si64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTSS2SI64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTSS2SI64RM` (`CVTTSS2SI`). Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 32-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
    /// Reference: [Intel x86 docs for CVTTSS2SI](https://www.felixcloutier.com/x86/CVTTSS2SI.html)
    fn sse_cvttss2si64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTSS2SI64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSS2SI32RR` (`CVTSS2SI`). Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
    /// Reference: [Intel x86 docs for CVTSS2SI](https://www.felixcloutier.com/x86/CVTSS2SI.html)
    fn sse_cvtss2si32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSS2SI32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSS2SI32RM` (`CVTSS2SI`). Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
    /// Reference: [Intel x86 docs for CVTSS2SI](https://www.felixcloutier.com/x86/CVTSS2SI.html)
    fn sse_cvtss2si32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSS2SI32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSS2SI64RR` (`CVTSS2SI`). Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
    /// Reference: [Intel x86 docs for CVTSS2SI](https://www.felixcloutier.com/x86/CVTSS2SI.html)
    fn sse_cvtss2si64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSS2SI64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSS2SI64RM` (`CVTSS2SI`). Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
    /// Reference: [Intel x86 docs for CVTSS2SI](https://www.felixcloutier.com/x86/CVTSS2SI.html)
    fn sse_cvtss2si64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSS2SI64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UCOMISSRR` (`UCOMISS`). Compares the single precision floating-point values in the low doublewords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    /// Reference: [Intel x86 docs for UCOMISS](https://www.felixcloutier.com/x86/UCOMISS.html)
    fn sse_ucomissrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UCOMISSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UCOMISSRM` (`UCOMISS`). Compares the single precision floating-point values in the low doublewords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    /// Reference: [Intel x86 docs for UCOMISS](https://www.felixcloutier.com/x86/UCOMISS.html)
    fn sse_ucomissrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UCOMISSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_COMISSRR` (`COMISS`). Compares the single precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    /// Reference: [Intel x86 docs for COMISS](https://www.felixcloutier.com/x86/COMISS.html)
    fn sse_comissrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_COMISSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_COMISSRM` (`COMISS`). Compares the single precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    /// Reference: [Intel x86 docs for COMISS](https://www.felixcloutier.com/x86/COMISS.html)
    fn sse_comissrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_COMISSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVMSKPSRR` (`MOVMSKPS`). Extracts the sign bits from the packed single precision floating-point values in the source operand (second operand), formats them into a 4- or 8-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM or YMM register, and the destination operand is a general-purpose register. The mask is stored in the 4 or 8 low-order bits of the destination operand. The upper bits of the destination operand beyond the mask are filled with zeros.
    /// Reference: [Intel x86 docs for MOVMSKPS](https://www.felixcloutier.com/x86/MOVMSKPS.html)
    fn sse_movmskpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVMSKPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SQRTPSRR` (`SQRTPS`). Performs a SIMD computation of the square roots of the four, eight or sixteen packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand.
    /// Reference: [Intel x86 docs for SQRTPS](https://www.felixcloutier.com/x86/SQRTPS.html)
    fn sse_sqrtpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SQRTPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SQRTPSRM` (`SQRTPS`). Performs a SIMD computation of the square roots of the four, eight or sixteen packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand.
    /// Reference: [Intel x86 docs for SQRTPS](https://www.felixcloutier.com/x86/SQRTPS.html)
    fn sse_sqrtpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SQRTPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SQRTSSRR` (`SQRTSS`). Computes the square root of the low single precision floating-point value in the second source operand and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location. The first source and destination operands is an XMM register.
    /// Reference: [Intel x86 docs for SQRTSS](https://www.felixcloutier.com/x86/SQRTSS.html)
    fn sse_sqrtssrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SQRTSSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SQRTSSRM` (`SQRTSS`). Computes the square root of the low single precision floating-point value in the second source operand and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location. The first source and destination operands is an XMM register.
    /// Reference: [Intel x86 docs for SQRTSS](https://www.felixcloutier.com/x86/SQRTSS.html)
    fn sse_sqrtssrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SQRTSSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_RSQRTPSRR` (`RSQRTPS`). Performs a SIMD computation of the approximate reciprocals of the square roots of the four packed single precision floating-point values in the source operand (second operand) and stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for RSQRTPS](https://www.felixcloutier.com/x86/RSQRTPS.html)
    fn sse_rsqrtpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_RSQRTPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_RSQRTPSRM` (`RSQRTPS`). Performs a SIMD computation of the approximate reciprocals of the square roots of the four packed single precision floating-point values in the source operand (second operand) and stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for RSQRTPS](https://www.felixcloutier.com/x86/RSQRTPS.html)
    fn sse_rsqrtpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_RSQRTPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_RSQRTSSRR` (`RSQRTSS`). Computes an approximate reciprocal of the square root of the low single precision floating-point value in the source operand (second operand) stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    /// Reference: [Intel x86 docs for RSQRTSS](https://www.felixcloutier.com/x86/RSQRTSS.html)
    fn sse_rsqrtssrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_RSQRTSSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_RSQRTSSRM` (`RSQRTSS`). Computes an approximate reciprocal of the square root of the low single precision floating-point value in the source operand (second operand) stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    /// Reference: [Intel x86 docs for RSQRTSS](https://www.felixcloutier.com/x86/RSQRTSS.html)
    fn sse_rsqrtssrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_RSQRTSSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_RCPPSRR` (`RCPPS`). Performs a SIMD computation of the approximate reciprocals of the four packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for RCPPS](https://www.felixcloutier.com/x86/RCPPS.html)
    fn sse_rcppsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_RCPPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_RCPPSRM` (`RCPPS`). Performs a SIMD computation of the approximate reciprocals of the four packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for RCPPS](https://www.felixcloutier.com/x86/RCPPS.html)
    fn sse_rcppsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_RCPPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_RCPSSRR` (`RCPSS`). Computes of an approximate reciprocal of the low single precision floating-point value in the source operand (second operand) and stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    /// Reference: [Intel x86 docs for RCPSS](https://www.felixcloutier.com/x86/RCPSS.html)
    fn sse_rcpssrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_RCPSSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_RCPSSRM` (`RCPSS`). Computes of an approximate reciprocal of the low single precision floating-point value in the source operand (second operand) and stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    /// Reference: [Intel x86 docs for RCPSS](https://www.felixcloutier.com/x86/RCPSS.html)
    fn sse_rcpssrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_RCPSSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ANDPSRR` (`ANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for ANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn sse_andpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ANDPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ANDPSRM` (`ANDPS`). Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for ANDPS](https://www.felixcloutier.com/x86/ANDPS.html)
    fn sse_andpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ANDPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ANDNPSRR` (`ANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for ANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn sse_andnpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ANDNPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ANDNPSRM` (`ANDNPS`). Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for ANDNPS](https://www.felixcloutier.com/x86/ANDNPS.html)
    fn sse_andnpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ANDNPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ORPSRR` (`ORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for ORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn sse_orpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ORPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ORPSRM` (`ORPS`). Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for ORPS](https://www.felixcloutier.com/x86/ORPS.html)
    fn sse_orpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ORPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_XORPSRR` (`XORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for XORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn sse_xorpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_XORPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_XORPSRM` (`XORPS`). Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    /// Reference: [Intel x86 docs for XORPS](https://www.felixcloutier.com/x86/XORPS.html)
    fn sse_xorpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_XORPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDPSRR` (`ADDPS`). Adds four, eight or sixteen packed single precision floating-point values from the first source operand with the second source operand, and stores the packed single precision floating-point result in the destination operand.
    /// Reference: [Intel x86 docs for ADDPS](https://www.felixcloutier.com/x86/ADDPS.html)
    fn sse_addpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDPSRM` (`ADDPS`). Adds four, eight or sixteen packed single precision floating-point values from the first source operand with the second source operand, and stores the packed single precision floating-point result in the destination operand.
    /// Reference: [Intel x86 docs for ADDPS](https://www.felixcloutier.com/x86/ADDPS.html)
    fn sse_addpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDSSRR` (`ADDSS`). Adds the low single precision floating-point values from the second source operand and the first source operand, and stores the double precision floating-point result in the destination operand.
    /// Reference: [Intel x86 docs for ADDSS](https://www.felixcloutier.com/x86/ADDSS.html)
    fn sse_addssrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDSSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDSSRM` (`ADDSS`). Adds the low single precision floating-point values from the second source operand and the first source operand, and stores the double precision floating-point result in the destination operand.
    /// Reference: [Intel x86 docs for ADDSS](https://www.felixcloutier.com/x86/ADDSS.html)
    fn sse_addssrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDSSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MULPSRR` (`MULPS`). Multiply the packed single precision floating-point values from the first source operand with the corresponding values in the second source operand, and stores the packed double precision floating-point results in the destination operand.
    /// Reference: [Intel x86 docs for MULPS](https://www.felixcloutier.com/x86/MULPS.html)
    fn sse_mulpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MULPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MULPSRM` (`MULPS`). Multiply the packed single precision floating-point values from the first source operand with the corresponding values in the second source operand, and stores the packed double precision floating-point results in the destination operand.
    /// Reference: [Intel x86 docs for MULPS](https://www.felixcloutier.com/x86/MULPS.html)
    fn sse_mulpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MULPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MULSSRR` (`MULSS`). Multiplies the low single precision floating-point value from the second source operand by the low single precision floating-point value in the first source operand, and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location. The first source operand and the destination operands are XMM registers.
    /// Reference: [Intel x86 docs for MULSS](https://www.felixcloutier.com/x86/MULSS.html)
    fn sse_mulssrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MULSSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MULSSRM` (`MULSS`). Multiplies the low single precision floating-point value from the second source operand by the low single precision floating-point value in the first source operand, and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location. The first source operand and the destination operands are XMM registers.
    /// Reference: [Intel x86 docs for MULSS](https://www.felixcloutier.com/x86/MULSS.html)
    fn sse_mulssrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MULSSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SUBPSRR` (`SUBPS`). Performs a SIMD subtract of the packed single precision floating-point values in the second Source operand from the First Source operand, and stores the packed single precision floating-point results in the destination operand.
    /// Reference: [Intel x86 docs for SUBPS](https://www.felixcloutier.com/x86/SUBPS.html)
    fn sse_subpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SUBPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SUBPSRM` (`SUBPS`). Performs a SIMD subtract of the packed single precision floating-point values in the second Source operand from the First Source operand, and stores the packed single precision floating-point results in the destination operand.
    /// Reference: [Intel x86 docs for SUBPS](https://www.felixcloutier.com/x86/SUBPS.html)
    fn sse_subpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SUBPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SUBSSRR` (`SUBSS`). Subtract the low single precision floating-point value from the second source operand and the first source operand and store the double precision floating-point result in the low doubleword of the destination operand.
    /// Reference: [Intel x86 docs for SUBSS](https://www.felixcloutier.com/x86/SUBSS.html)
    fn sse_subssrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SUBSSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SUBSSRM` (`SUBSS`). Subtract the low single precision floating-point value from the second source operand and the first source operand and store the double precision floating-point result in the low doubleword of the destination operand.
    /// Reference: [Intel x86 docs for SUBSS](https://www.felixcloutier.com/x86/SUBSS.html)
    fn sse_subssrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SUBSSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MINPSRR` (`MINPS`). Performs a SIMD compare of the packed single precision floating-point values in the first source operand and the second source operand and returns the minimum value for each pair of values to the destination operand.
    /// Reference: [Intel x86 docs for MINPS](https://www.felixcloutier.com/x86/MINPS.html)
    fn sse_minpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MINPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MINPSRM` (`MINPS`). Performs a SIMD compare of the packed single precision floating-point values in the first source operand and the second source operand and returns the minimum value for each pair of values to the destination operand.
    /// Reference: [Intel x86 docs for MINPS](https://www.felixcloutier.com/x86/MINPS.html)
    fn sse_minpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MINPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MINSSRR` (`MINSS`). Compares the low single precision floating-point values in the first source operand and the second source operand and returns the minimum value to the low doubleword of the destination operand.
    /// Reference: [Intel x86 docs for MINSS](https://www.felixcloutier.com/x86/MINSS.html)
    fn sse_minssrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MINSSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MINSSRM` (`MINSS`). Compares the low single precision floating-point values in the first source operand and the second source operand and returns the minimum value to the low doubleword of the destination operand.
    /// Reference: [Intel x86 docs for MINSS](https://www.felixcloutier.com/x86/MINSS.html)
    fn sse_minssrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MINSSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_DIVPSRR` (`DIVPS`). Performs a SIMD divide of the four, eight or sixteen packed single precision floating-point values in the first source operand (the second operand) by the four, eight or sixteen packed single precision floating-point values in the second source operand (the third operand). Results are written to the destination operand (the first operand).
    /// Reference: [Intel x86 docs for DIVPS](https://www.felixcloutier.com/x86/DIVPS.html)
    fn sse_divpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_DIVPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_DIVPSRM` (`DIVPS`). Performs a SIMD divide of the four, eight or sixteen packed single precision floating-point values in the first source operand (the second operand) by the four, eight or sixteen packed single precision floating-point values in the second source operand (the third operand). Results are written to the destination operand (the first operand).
    /// Reference: [Intel x86 docs for DIVPS](https://www.felixcloutier.com/x86/DIVPS.html)
    fn sse_divpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_DIVPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_DIVSSRR` (`DIVSS`). Divides the low single precision floating-point value in the first source operand by the low single precision floating-point value in the second source operand, and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location.
    /// Reference: [Intel x86 docs for DIVSS](https://www.felixcloutier.com/x86/DIVSS.html)
    fn sse_divssrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_DIVSSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_DIVSSRM` (`DIVSS`). Divides the low single precision floating-point value in the first source operand by the low single precision floating-point value in the second source operand, and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location.
    /// Reference: [Intel x86 docs for DIVSS](https://www.felixcloutier.com/x86/DIVSS.html)
    fn sse_divssrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_DIVSSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MAXPSRR` (`MAXPS`). Performs a SIMD compare of the packed single precision floating-point values in the first source operand and the second source operand and returns the maximum value for each pair of values to the destination operand.
    /// Reference: [Intel x86 docs for MAXPS](https://www.felixcloutier.com/x86/MAXPS.html)
    fn sse_maxpsrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MAXPSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MAXPSRM` (`MAXPS`). Performs a SIMD compare of the packed single precision floating-point values in the first source operand and the second source operand and returns the maximum value for each pair of values to the destination operand.
    /// Reference: [Intel x86 docs for MAXPS](https://www.felixcloutier.com/x86/MAXPS.html)
    fn sse_maxpsrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MAXPSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MAXSSRR` (`MAXSS`). Compares the low single precision floating-point values in the first source operand and the second source operand, and returns the maximum value to the low doubleword of the destination operand.
    /// Reference: [Intel x86 docs for MAXSS](https://www.felixcloutier.com/x86/MAXSS.html)
    fn sse_maxssrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MAXSSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MAXSSRM` (`MAXSS`). Compares the low single precision floating-point values in the first source operand and the second source operand, and returns the maximum value to the low doubleword of the destination operand.
    /// Reference: [Intel x86 docs for MAXSS](https://www.felixcloutier.com/x86/MAXSS.html)
    fn sse_maxssrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MAXSSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `LDMXCSRM` (`LDMXCSR`). Loads the source operand into the MXCSR control/status register. The source operand is a 32-bit memory location. See “MXCSR Control and Status Register” in Chapter 10, of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for a description of the MXCSR register and its contents.
    /// Reference: [Intel x86 docs for LDMXCSR](https://www.felixcloutier.com/x86/LDMXCSR.html)
    fn ldmxcsrm(&mut self,op0: impl OperandCast) -> () {
        self.emit(LDMXCSRM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `STMXCSRM` (`STMXCSR`). Stores the contents of the MXCSR control and status register to the destination operand. The destination operand is a 32-bit memory location. The reserved bits in the MXCSR register are stored as 0s.
    /// Reference: [Intel x86 docs for STMXCSR](https://www.felixcloutier.com/x86/STMXCSR.html)
    fn stmxcsrm(&mut self,op0: impl OperandCast) -> () {
        self.emit(STMXCSRM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SFENCE` (`SFENCE`). Orders processor execution relative to all memory stores prior to the SFENCE instruction. The processor ensures that every store prior to SFENCE is globally visible before any store after SFENCE becomes globally visible. The SFENCE instruction is ordered with respect to memory stores, other SFENCE instructions, MFENCE instructions, and any serializing instructions (such as the CPUID instruction). It is not ordered with respect to memory loads or the LFENCE instruction.
    /// Reference: [Intel x86 docs for SFENCE](https://www.felixcloutier.com/x86/SFENCE.html)
    fn sfence(&mut self,) -> () {
        self.emit(SFENCE, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CMPPSRRI` (`CMPPS`). Performs a SIMD compare of the packed single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each of the pairs of packed values.
    /// Reference: [Intel x86 docs for CMPPS](https://www.felixcloutier.com/x86/CMPPS.html)
    fn sse_cmppsrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_CMPPSRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_CMPPSRMI` (`CMPPS`). Performs a SIMD compare of the packed single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each of the pairs of packed values.
    /// Reference: [Intel x86 docs for CMPPS](https://www.felixcloutier.com/x86/CMPPS.html)
    fn sse_cmppsrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_CMPPSRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_CMPSSRRI` (`CMPSS`). Compares the low single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate operand) specifies the type of comparison performed.
    /// Reference: [Intel x86 docs for CMPSS](https://www.felixcloutier.com/x86/CMPSS.html)
    fn sse_cmpssrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_CMPSSRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_CMPSSRMI` (`CMPSS`). Compares the low single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate operand) specifies the type of comparison performed.
    /// Reference: [Intel x86 docs for CMPSS](https://www.felixcloutier.com/x86/CMPSS.html)
    fn sse_cmpssrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_CMPSSRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_SHUFPSRRI` (`SHUFPS`). Selects a single precision floating-point value of an input quadruplet using a two-bit control and move to a designated element of the destination operand. Each 64-bit element-pair of a 128-bit lane of the destination operand is interleaved between the corresponding lane of the first source operand and the second source operand at the granularity 128 bits. Each two bits in the imm8 byte, starting from bit 0, is the select control of the corresponding element of a 128-bit lane of the destination to received the shuffled result of an input quadruplet. The two lower elements of a 128-bit lane in the destination receives shuffle results from the quadruple of the first source operand. The next two elements of the destination receives shuffle results from the quadruple of the second source operand.
    /// Reference: [Intel x86 docs for SHUFPS](https://www.felixcloutier.com/x86/SHUFPS.html)
    fn sse_shufpsrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_SHUFPSRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_SHUFPSRMI` (`SHUFPS`). Selects a single precision floating-point value of an input quadruplet using a two-bit control and move to a designated element of the destination operand. Each 64-bit element-pair of a 128-bit lane of the destination operand is interleaved between the corresponding lane of the first source operand and the second source operand at the granularity 128 bits. Each two bits in the imm8 byte, starting from bit 0, is the select control of the corresponding element of a 128-bit lane of the destination to received the shuffled result of an input quadruplet. The two lower elements of a 128-bit lane in the destination receives shuffle results from the quadruple of the first source operand. The next two elements of the destination receives shuffle results from the quadruple of the second source operand.
    /// Reference: [Intel x86 docs for SHUFPS](https://www.felixcloutier.com/x86/SHUFPS.html)
    fn sse_shufpsrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_SHUFPSRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
