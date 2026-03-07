pub trait X86SSE41Emitter: Emitter {
    /// Emits `SSE_PBLENDVBRR` (`PBLENDVB`). Conditionally copies byte elements from the source operand (second operand) to the destination operand (first operand) depending on mask bits defined in the implicit third register argument, XMM0. The mask bits are the most significant bit in each byte element of the XMM0 register.
    /// Reference: [Intel x86 docs for PBLENDVB](https://www.felixcloutier.com/x86/PBLENDVB.html)
    fn sse_pblendvbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PBLENDVBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PBLENDVBRM` (`PBLENDVB`). Conditionally copies byte elements from the source operand (second operand) to the destination operand (first operand) depending on mask bits defined in the implicit third register argument, XMM0. The mask bits are the most significant bit in each byte element of the XMM0 register.
    /// Reference: [Intel x86 docs for PBLENDVB](https://www.felixcloutier.com/x86/PBLENDVB.html)
    fn sse_pblendvbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PBLENDVBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_BLENDVPSRRR` (`BLENDVPS`). Conditionally copy each dword data element of single precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each dword element of the mask register.
    /// Reference: [Intel x86 docs for BLENDVPS](https://www.felixcloutier.com/x86/BLENDVPS.html)
    fn sse_blendvpsrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_BLENDVPSRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_BLENDVPSRMR` (`BLENDVPS`). Conditionally copy each dword data element of single precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each dword element of the mask register.
    /// Reference: [Intel x86 docs for BLENDVPS](https://www.felixcloutier.com/x86/BLENDVPS.html)
    fn sse_blendvpsrmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_BLENDVPSRMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_BLENDVPDRRR` (`BLENDVPD`). Conditionally copy each quadword data element of double precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each quadword element of the mask register.
    /// Reference: [Intel x86 docs for BLENDVPD](https://www.felixcloutier.com/x86/BLENDVPD.html)
    fn sse_blendvpdrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_BLENDVPDRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_BLENDVPDRMR` (`BLENDVPD`). Conditionally copy each quadword data element of double precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each quadword element of the mask register.
    /// Reference: [Intel x86 docs for BLENDVPD](https://www.felixcloutier.com/x86/BLENDVPD.html)
    fn sse_blendvpdrmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_BLENDVPDRMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PTESTRR` (`PTEST`). PTEST and VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND of the first source operand (first operand) and the second source operand (second operand). VPTEST sets the CF flag if all bits in the result are 0 of the bitwise AND of the second source operand (second operand) and the logical NOT of the destination operand.
    /// Reference: [Intel x86 docs for PTEST](https://www.felixcloutier.com/x86/PTEST.html)
    fn sse_ptestrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PTESTRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PTESTRM` (`PTEST`). PTEST and VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND of the first source operand (first operand) and the second source operand (second operand). VPTEST sets the CF flag if all bits in the result are 0 of the bitwise AND of the second source operand (second operand) and the logical NOT of the destination operand.
    /// Reference: [Intel x86 docs for PTEST](https://www.felixcloutier.com/x86/PTEST.html)
    fn sse_ptestrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PTESTRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXBWRR` (`PMOVSXBW`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXBW](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxbwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXBWRM` (`PMOVSXBW`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXBW](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxbwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXBDRR` (`PMOVSXBD`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXBD](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxbdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXBDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXBDRM` (`PMOVSXBD`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXBD](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxbdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXBDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXBQRR` (`PMOVSXBQ`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXBQ](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxbqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXBQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXBQRM` (`PMOVSXBQ`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXBQ](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxbqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXBQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXWDRR` (`PMOVSXWD`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXWD](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxwdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXWDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXWDRM` (`PMOVSXWD`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXWD](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxwdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXWDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXWQRR` (`PMOVSXWQ`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXWQ](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxwqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXWQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXWQRM` (`PMOVSXWQ`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXWQ](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxwqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXWQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXDQRR` (`PMOVSXDQ`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXDQ](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxdqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVSXDQRM` (`PMOVSXDQ`). Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVSXDQ](https://www.felixcloutier.com/x86/PMOVSX.html)
    fn sse_pmovsxdqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVSXDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULDQRR` (`PMULDQ`). Multiplies packed signed doubleword integers in the even-numbered (zero-based reference) elements of the first source operand with the packed signed doubleword integers in the corresponding elements of the second source operand and stores packed signed quadword results in the destination operand.
    /// Reference: [Intel x86 docs for PMULDQ](https://www.felixcloutier.com/x86/PMULDQ.html)
    fn sse_pmuldqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULDQRM` (`PMULDQ`). Multiplies packed signed doubleword integers in the even-numbered (zero-based reference) elements of the first source operand with the packed signed doubleword integers in the corresponding elements of the second source operand and stores packed signed quadword results in the destination operand.
    /// Reference: [Intel x86 docs for PMULDQ](https://www.felixcloutier.com/x86/PMULDQ.html)
    fn sse_pmuldqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPEQQRR` (`PCMPEQQ`). Performs an SIMD compare for equality of the packed quadwords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for PCMPEQQ](https://www.felixcloutier.com/x86/PCMPEQQ.html)
    fn sse_pcmpeqqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPEQQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPEQQRM` (`PCMPEQQ`). Performs an SIMD compare for equality of the packed quadwords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for PCMPEQQ](https://www.felixcloutier.com/x86/PCMPEQQ.html)
    fn sse_pcmpeqqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPEQQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVNTDQARM` (`MOVNTDQA`). MOVNTDQA loads a double quadword from the source operand (second operand) to the destination operand (first operand) using a non-temporal hint if the memory source is WC (write combining) memory type. For WC memory type, the nontemporal hint may be implemented by loading a temporary internal buffer with the equivalent of an aligned cache line without filling this data to the cache. Any memory-type aliased lines in the cache will be snooped and flushed. Subsequent MOVNTDQA reads to unread portions of the WC cache line will receive data from the temporary internal buffer if data is available. The temporary internal buffer may be flushed by the processor at any time for any reason, for example
    /// Reference: [Intel x86 docs for MOVNTDQA](https://www.felixcloutier.com/x86/MOVNTDQA.html)
    fn sse_movntdqarm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTDQARM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PACKUSDWRR` (`PACKUSDW`). Converts packed signed doubleword integers in the first and second source operands into packed unsigned word integers using unsigned saturation to handle overflow conditions. If the signed doubleword value is beyond the range of an unsigned word (that is, greater than FFFFH or less than 0000H), the saturated unsigned word integer value of FFFFH or 0000H, respectively, is stored in the destination.
    /// Reference: [Intel x86 docs for PACKUSDW](https://www.felixcloutier.com/x86/PACKUSDW.html)
    fn sse_packusdwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PACKUSDWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PACKUSDWRM` (`PACKUSDW`). Converts packed signed doubleword integers in the first and second source operands into packed unsigned word integers using unsigned saturation to handle overflow conditions. If the signed doubleword value is beyond the range of an unsigned word (that is, greater than FFFFH or less than 0000H), the saturated unsigned word integer value of FFFFH or 0000H, respectively, is stored in the destination.
    /// Reference: [Intel x86 docs for PACKUSDW](https://www.felixcloutier.com/x86/PACKUSDW.html)
    fn sse_packusdwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PACKUSDWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXBWRR` (`PMOVZXBW`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXBW](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxbwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXBWRM` (`PMOVZXBW`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXBW](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxbwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXBDRR` (`PMOVZXBD`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXBD](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxbdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXBDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXBDRM` (`PMOVZXBD`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXBD](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxbdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXBDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXBQRR` (`PMOVZXBQ`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXBQ](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxbqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXBQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXBQRM` (`PMOVZXBQ`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXBQ](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxbqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXBQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXWDRR` (`PMOVZXWD`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXWD](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxwdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXWDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXWDRM` (`PMOVZXWD`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXWD](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxwdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXWDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXWQRR` (`PMOVZXWQ`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXWQ](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxwqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXWQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXWQRM` (`PMOVZXWQ`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXWQ](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxwqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXWQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXDQRR` (`PMOVZXDQ`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXDQ](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxdqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVZXDQRM` (`PMOVZXDQ`). Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    /// Reference: [Intel x86 docs for PMOVZXDQ](https://www.felixcloutier.com/x86/PMOVZX.html)
    fn sse_pmovzxdqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVZXDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPGTQRR` (`PCMPGTQ`). Performs an SIMD signed compare for the packed quadwords in the destination operand (first operand) and the source operand (second operand). If the data element in the first (destination) operand is greater than the corresponding element in the second (source) operand, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for PCMPGTQ](https://www.felixcloutier.com/x86/PCMPGTQ.html)
    fn sse_pcmpgtqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPGTQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPGTQRM` (`PCMPGTQ`). Performs an SIMD signed compare for the packed quadwords in the destination operand (first operand) and the source operand (second operand). If the data element in the first (destination) operand is greater than the corresponding element in the second (source) operand, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for PCMPGTQ](https://www.felixcloutier.com/x86/PCMPGTQ.html)
    fn sse_pcmpgtqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPGTQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINSBRR` (`PMINSB`). Performs a SIMD compare of the packed signed byte, word, or dword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINSB](https://www.felixcloutier.com/x86/PMINSB%3APMINSW.html)
    fn sse_pminsbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINSBRM` (`PMINSB`). Performs a SIMD compare of the packed signed byte, word, or dword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINSB](https://www.felixcloutier.com/x86/PMINSB%3APMINSW.html)
    fn sse_pminsbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINSDRR` (`PMINSD`). Performs a SIMD compare of the packed signed dword or qword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINSD](https://www.felixcloutier.com/x86/PMINSD%3APMINSQ.html)
    fn sse_pminsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINSDRM` (`PMINSD`). Performs a SIMD compare of the packed signed dword or qword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINSD](https://www.felixcloutier.com/x86/PMINSD%3APMINSQ.html)
    fn sse_pminsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINUWRR` (`PMINUW`). Performs a SIMD compare of the packed unsigned byte or word integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINUW](https://www.felixcloutier.com/x86/PMINUB%3APMINUW.html)
    fn sse_pminuwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINUWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINUWRM` (`PMINUW`). Performs a SIMD compare of the packed unsigned byte or word integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINUW](https://www.felixcloutier.com/x86/PMINUB%3APMINUW.html)
    fn sse_pminuwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINUWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINUDRR` (`PMINUD`). Performs a SIMD compare of the packed unsigned dword/qword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINUD](https://www.felixcloutier.com/x86/PMINUD%3APMINUQ.html)
    fn sse_pminudrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINUDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINUDRM` (`PMINUD`). Performs a SIMD compare of the packed unsigned dword/qword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINUD](https://www.felixcloutier.com/x86/PMINUD%3APMINUQ.html)
    fn sse_pminudrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINUDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXSBRR` (`PMAXSB`). Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXSB](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html)
    fn sse_pmaxsbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXSBRM` (`PMAXSB`). Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXSB](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html)
    fn sse_pmaxsbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXSDRR` (`PMAXSD`). Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXSD](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html)
    fn sse_pmaxsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXSDRM` (`PMAXSD`). Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXSD](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html)
    fn sse_pmaxsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXUWRR` (`PMAXUW`). Performs a SIMD compare of the packed unsigned byte, word integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXUW](https://www.felixcloutier.com/x86/PMAXUB%3APMAXUW.html)
    fn sse_pmaxuwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXUWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXUWRM` (`PMAXUW`). Performs a SIMD compare of the packed unsigned byte, word integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXUW](https://www.felixcloutier.com/x86/PMAXUB%3APMAXUW.html)
    fn sse_pmaxuwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXUWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXUDRR` (`PMAXUD`). Performs a SIMD compare of the packed unsigned dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXUD](https://www.felixcloutier.com/x86/PMAXUD%3APMAXUQ.html)
    fn sse_pmaxudrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXUDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXUDRM` (`PMAXUD`). Performs a SIMD compare of the packed unsigned dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXUD](https://www.felixcloutier.com/x86/PMAXUD%3APMAXUQ.html)
    fn sse_pmaxudrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXUDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULLDRR` (`PMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for PMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn sse_pmulldrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULLDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULLDRM` (`PMULLD`). Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    /// Reference: [Intel x86 docs for PMULLD](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html)
    fn sse_pmulldrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULLDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHMINPOSUWRR` (`PHMINPOSUW`). Determine the minimum unsigned word value in the source operand (second operand) and place the unsigned word in the low word (bits 0-15) of the destination operand (first operand). The word index of the minimum value is stored in bits 16-18 of the destination operand. The remaining upper bits of the destination are set to zero.
    /// Reference: [Intel x86 docs for PHMINPOSUW](https://www.felixcloutier.com/x86/PHMINPOSUW.html)
    fn sse_phminposuwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHMINPOSUWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHMINPOSUWRM` (`PHMINPOSUW`). Determine the minimum unsigned word value in the source operand (second operand) and place the unsigned word in the low word (bits 0-15) of the destination operand (first operand). The word index of the minimum value is stored in bits 16-18 of the destination operand. The remaining upper bits of the destination are set to zero.
    /// Reference: [Intel x86 docs for PHMINPOSUW](https://www.felixcloutier.com/x86/PHMINPOSUW.html)
    fn sse_phminposuwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHMINPOSUWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ROUNDPSRRI` (`ROUNDPS`). Round the 4 single precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a single precision floating-point value.
    /// Reference: [Intel x86 docs for ROUNDPS](https://www.felixcloutier.com/x86/ROUNDPS.html)
    fn sse_roundpsrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_ROUNDPSRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_ROUNDPSRMI` (`ROUNDPS`). Round the 4 single precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a single precision floating-point value.
    /// Reference: [Intel x86 docs for ROUNDPS](https://www.felixcloutier.com/x86/ROUNDPS.html)
    fn sse_roundpsrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_ROUNDPSRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_ROUNDPDRRI` (`ROUNDPD`). Round the 2 double precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a double precision floating-point value.
    /// Reference: [Intel x86 docs for ROUNDPD](https://www.felixcloutier.com/x86/ROUNDPD.html)
    fn sse_roundpdrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_ROUNDPDRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_ROUNDPDRMI` (`ROUNDPD`). Round the 2 double precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a double precision floating-point value.
    /// Reference: [Intel x86 docs for ROUNDPD](https://www.felixcloutier.com/x86/ROUNDPD.html)
    fn sse_roundpdrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_ROUNDPDRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_ROUNDSSRRI` (`ROUNDSS`). Round the single precision floating-point value in the lowest dword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a single precision floating-point input to an integer value and returns the result as a single precision floating-point value in the lowest position. The upper three single precision floating-point values in the destination are retained.
    /// Reference: [Intel x86 docs for ROUNDSS](https://www.felixcloutier.com/x86/ROUNDSS.html)
    fn sse_roundssrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_ROUNDSSRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_ROUNDSSRMI` (`ROUNDSS`). Round the single precision floating-point value in the lowest dword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a single precision floating-point input to an integer value and returns the result as a single precision floating-point value in the lowest position. The upper three single precision floating-point values in the destination are retained.
    /// Reference: [Intel x86 docs for ROUNDSS](https://www.felixcloutier.com/x86/ROUNDSS.html)
    fn sse_roundssrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_ROUNDSSRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_ROUNDSDRRI` (`ROUNDSD`). Round the double precision floating-point value in the lower qword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a double precision floating-point input to an integer value and returns the integer result as a double precision floating-point value in the lowest position. The upper double precision floating-point value in the destination is retained.
    /// Reference: [Intel x86 docs for ROUNDSD](https://www.felixcloutier.com/x86/ROUNDSD.html)
    fn sse_roundsdrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_ROUNDSDRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_ROUNDSDRMI` (`ROUNDSD`). Round the double precision floating-point value in the lower qword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a double precision floating-point input to an integer value and returns the integer result as a double precision floating-point value in the lowest position. The upper double precision floating-point value in the destination is retained.
    /// Reference: [Intel x86 docs for ROUNDSD](https://www.felixcloutier.com/x86/ROUNDSD.html)
    fn sse_roundsdrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_ROUNDSDRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_BLENDPSRRI` (`BLENDPS`). Packed single precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [7:0] determine whether the corresponding single precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is “1”, then the single precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for BLENDPS](https://www.felixcloutier.com/x86/BLENDPS.html)
    fn sse_blendpsrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_BLENDPSRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_BLENDPSRMI` (`BLENDPS`). Packed single precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [7:0] determine whether the corresponding single precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is “1”, then the single precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for BLENDPS](https://www.felixcloutier.com/x86/BLENDPS.html)
    fn sse_blendpsrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_BLENDPSRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_BLENDPDRRI` (`BLENDPD`). Double-precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [3:0] determine whether the corresponding double precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is ”1”, then the double precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for BLENDPD](https://www.felixcloutier.com/x86/BLENDPD.html)
    fn sse_blendpdrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_BLENDPDRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_BLENDPDRMI` (`BLENDPD`). Double-precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [3:0] determine whether the corresponding double precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is ”1”, then the double precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for BLENDPD](https://www.felixcloutier.com/x86/BLENDPD.html)
    fn sse_blendpdrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_BLENDPDRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PBLENDWRRI` (`PBLENDW`). Words from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source. If a bit in the mask, corresponding to a word, is “1", then the word is copied, else the word element in the destination operand is unchanged.
    /// Reference: [Intel x86 docs for PBLENDW](https://www.felixcloutier.com/x86/PBLENDW.html)
    fn sse_pblendwrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PBLENDWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PBLENDWRMI` (`PBLENDW`). Words from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source. If a bit in the mask, corresponding to a word, is “1", then the word is copied, else the word element in the destination operand is unchanged.
    /// Reference: [Intel x86 docs for PBLENDW](https://www.felixcloutier.com/x86/PBLENDW.html)
    fn sse_pblendwrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PBLENDWRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PEXTRBMRI` (`PEXTRB`). Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
    /// Reference: [Intel x86 docs for PEXTRB](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html)
    fn sse_pextrbmri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PEXTRBMRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PEXTRBRRI` (`PEXTRB`). Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
    /// Reference: [Intel x86 docs for PEXTRB](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html)
    fn sse_pextrbrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PEXTRBRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PEXTRWMRI` (`PEXTRW`). Copies the word in the source operand (second operand) specified by the count operand (third operand) to the destination operand (first operand). The source operand can be an MMX technology register or an XMM register. The destination operand can be the low word of a general-purpose register or a 16-bit memory address. The count operand is an 8-bit immediate. When specifying a word location in an MMX technology register, the 2 least-significant bits of the count operand specify the location; for an XMM register, the 3 least-significant bits specify the location. The content of the destination register above bit 16 is cleared (set to all 0s).
    /// Reference: [Intel x86 docs for PEXTRW](https://www.felixcloutier.com/x86/PEXTRW.html)
    fn sse_pextrwmri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PEXTRWMRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PEXTRDRRI` (`PEXTRD`). Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
    /// Reference: [Intel x86 docs for PEXTRD](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html)
    fn sse_pextrdrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PEXTRDRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PEXTRDMRI` (`PEXTRD`). Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
    /// Reference: [Intel x86 docs for PEXTRD](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html)
    fn sse_pextrdmri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PEXTRDMRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PEXTRQRRI` (`PEXTRQ`). Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
    /// Reference: [Intel x86 docs for PEXTRQ](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html)
    fn sse_pextrqrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PEXTRQRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PEXTRQMRI` (`PEXTRQ`). Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
    /// Reference: [Intel x86 docs for PEXTRQ](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html)
    fn sse_pextrqmri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PEXTRQMRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_EXTRACTPSRRI` (`EXTRACTPS`). Extracts a single precision floating-point value from the source operand (second operand) at the 32-bit offset specified from imm8. Immediate bits higher than the most significant offset for the vector length are ignored.
    /// Reference: [Intel x86 docs for EXTRACTPS](https://www.felixcloutier.com/x86/EXTRACTPS.html)
    fn sse_extractpsrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_EXTRACTPSRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_EXTRACTPSMRI` (`EXTRACTPS`). Extracts a single precision floating-point value from the source operand (second operand) at the 32-bit offset specified from imm8. Immediate bits higher than the most significant offset for the vector length are ignored.
    /// Reference: [Intel x86 docs for EXTRACTPS](https://www.felixcloutier.com/x86/EXTRACTPS.html)
    fn sse_extractpsmri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_EXTRACTPSMRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PINSRBRRI` (`PINSRB`). Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
    /// Reference: [Intel x86 docs for PINSRB](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html)
    fn sse_pinsrbrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PINSRBRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PINSRBRMI` (`PINSRB`). Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
    /// Reference: [Intel x86 docs for PINSRB](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html)
    fn sse_pinsrbrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PINSRBRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_INSERTPSRRI` (`INSERTPS`). (register source form)
    /// Reference: [Intel x86 docs for INSERTPS](https://www.felixcloutier.com/x86/INSERTPS.html)
    fn sse_insertpsrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_INSERTPSRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_INSERTPSRMI` (`INSERTPS`). (register source form)
    /// Reference: [Intel x86 docs for INSERTPS](https://www.felixcloutier.com/x86/INSERTPS.html)
    fn sse_insertpsrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_INSERTPSRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PINSRDRRI` (`PINSRD`). Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
    /// Reference: [Intel x86 docs for PINSRD](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html)
    fn sse_pinsrdrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PINSRDRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PINSRDRMI` (`PINSRD`). Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
    /// Reference: [Intel x86 docs for PINSRD](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html)
    fn sse_pinsrdrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PINSRDRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PINSRQRRI` (`PINSRQ`). Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
    /// Reference: [Intel x86 docs for PINSRQ](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html)
    fn sse_pinsrqrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PINSRQRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PINSRQRMI` (`PINSRQ`). Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
    /// Reference: [Intel x86 docs for PINSRQ](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html)
    fn sse_pinsrqrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PINSRQRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_DPPSRRI` (`DPPS`). Conditionally multiplies the packed single precision floating-point values in the destination operand (first operand) with the packed single precision floats in the source (second operand) depending on a mask extracted from the high 4 bits of the immediate byte (third operand). If a condition mask bit in imm8[7:4] is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    /// Reference: [Intel x86 docs for DPPS](https://www.felixcloutier.com/x86/DPPS.html)
    fn sse_dppsrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_DPPSRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_DPPSRMI` (`DPPS`). Conditionally multiplies the packed single precision floating-point values in the destination operand (first operand) with the packed single precision floats in the source (second operand) depending on a mask extracted from the high 4 bits of the immediate byte (third operand). If a condition mask bit in imm8[7:4] is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    /// Reference: [Intel x86 docs for DPPS](https://www.felixcloutier.com/x86/DPPS.html)
    fn sse_dppsrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_DPPSRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_DPPDRRI` (`DPPD`). Conditionally multiplies the packed double precision floating-point values in the destination operand (first operand) with the packed double precision floating-point values in the source (second operand) depending on a mask extracted from bits [5:4] of the immediate operand (third operand). If a condition mask bit is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    /// Reference: [Intel x86 docs for DPPD](https://www.felixcloutier.com/x86/DPPD.html)
    fn sse_dppdrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_DPPDRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_DPPDRMI` (`DPPD`). Conditionally multiplies the packed double precision floating-point values in the destination operand (first operand) with the packed double precision floating-point values in the source (second operand) depending on a mask extracted from bits [5:4] of the immediate operand (third operand). If a condition mask bit is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    /// Reference: [Intel x86 docs for DPPD](https://www.felixcloutier.com/x86/DPPD.html)
    fn sse_dppdrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_DPPDRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_MPSADBWRRI` (`MPSADBW`). (V)MPSADBW calculates packed word results of sum-absolute-difference (SAD) of unsigned bytes from two blocks of 32-bit dword elements, using two select fields in the immediate byte to select the offsets of the two blocks within the first source operand and the second operand. Packed SAD word results are calculated within each 128-bit lane. Each SAD word result is calculated between a stationary block_2 (whose offset within the second source operand is selected by a two bit select control, multiplied by 32 bits) and a sliding block_1 at consecutive byte-granular position within the first source operand. The offset of the first 32-bit block of block_1 is selectable using a one bit select control, multiplied by 32 bits.
    /// Reference: [Intel x86 docs for MPSADBW](https://www.felixcloutier.com/x86/MPSADBW.html)
    fn sse_mpsadbwrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_MPSADBWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_MPSADBWRMI` (`MPSADBW`). (V)MPSADBW calculates packed word results of sum-absolute-difference (SAD) of unsigned bytes from two blocks of 32-bit dword elements, using two select fields in the immediate byte to select the offsets of the two blocks within the first source operand and the second operand. Packed SAD word results are calculated within each 128-bit lane. Each SAD word result is calculated between a stationary block_2 (whose offset within the second source operand is selected by a two bit select control, multiplied by 32 bits) and a sliding block_1 at consecutive byte-granular position within the first source operand. The offset of the first 32-bit block of block_1 is selectable using a one bit select control, multiplied by 32 bits.
    /// Reference: [Intel x86 docs for MPSADBW](https://www.felixcloutier.com/x86/MPSADBW.html)
    fn sse_mpsadbwrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_MPSADBWRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
