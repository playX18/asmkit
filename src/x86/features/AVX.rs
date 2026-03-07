pub trait X86AVXEmitter: Emitter {
    /// Emits `VMOVMSKPS128RR` (`VMOVMSKPS`). Extracts the sign bits from the packed single precision floating-point values in the source operand (second operand), formats them into a 4- or 8-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM or YMM register, and the destination operand is a general-purpose register. The mask is stored in the 4 or 8 low-order bits of the destination operand. The upper bits of the destination operand beyond the mask are filled with zeros.
    /// Reference: [Intel x86 docs for VMOVMSKPS](https://www.felixcloutier.com/x86/MOVMSKPS.html)
    fn vmovmskps128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVMSKPS128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVMSKPS256RR` (`VMOVMSKPS`). Extracts the sign bits from the packed single precision floating-point values in the source operand (second operand), formats them into a 4- or 8-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM or YMM register, and the destination operand is a general-purpose register. The mask is stored in the 4 or 8 low-order bits of the destination operand. The upper bits of the destination operand beyond the mask are filled with zeros.
    /// Reference: [Intel x86 docs for VMOVMSKPS](https://www.felixcloutier.com/x86/MOVMSKPS.html)
    fn vmovmskps256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVMSKPS256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVMSKPD128RR` (`VMOVMSKPD`). Extracts the sign bits from the packed double precision floating-point values in the source operand (second operand), formats them into a 2-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM register, and the destination operand is a general-purpose register. The mask is stored in the 2 low-order bits of the destination operand. Zero-extend the upper bits of the destination.
    /// Reference: [Intel x86 docs for VMOVMSKPD](https://www.felixcloutier.com/x86/MOVMSKPD.html)
    fn vmovmskpd128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVMSKPD128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVMSKPD256RR` (`VMOVMSKPD`). Extracts the sign bits from the packed double precision floating-point values in the source operand (second operand), formats them into a 2-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM register, and the destination operand is a general-purpose register. The mask is stored in the 2 low-order bits of the destination operand. Zero-extend the upper bits of the destination.
    /// Reference: [Intel x86 docs for VMOVMSKPD](https://www.felixcloutier.com/x86/MOVMSKPD.html)
    fn vmovmskpd256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVMSKPD256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VRSQRTPS128RR` (`VRSQRTPS`). Performs a SIMD computation of the approximate reciprocals of the square roots of the four packed single precision floating-point values in the source operand (second operand) and stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRSQRTPS](https://www.felixcloutier.com/x86/RSQRTPS.html)
    fn vrsqrtps128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPS128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VRSQRTPS128RM` (`VRSQRTPS`). Performs a SIMD computation of the approximate reciprocals of the square roots of the four packed single precision floating-point values in the source operand (second operand) and stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRSQRTPS](https://www.felixcloutier.com/x86/RSQRTPS.html)
    fn vrsqrtps128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPS128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VRSQRTPS256RR` (`VRSQRTPS`). Performs a SIMD computation of the approximate reciprocals of the square roots of the four packed single precision floating-point values in the source operand (second operand) and stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRSQRTPS](https://www.felixcloutier.com/x86/RSQRTPS.html)
    fn vrsqrtps256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPS256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VRSQRTPS256RM` (`VRSQRTPS`). Performs a SIMD computation of the approximate reciprocals of the square roots of the four packed single precision floating-point values in the source operand (second operand) and stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRSQRTPS](https://www.felixcloutier.com/x86/RSQRTPS.html)
    fn vrsqrtps256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRSQRTPS256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VRSQRTSSRRR` (`VRSQRTSS`). Computes an approximate reciprocal of the square root of the low single precision floating-point value in the source operand (second operand) stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRSQRTSS](https://www.felixcloutier.com/x86/RSQRTSS.html)
    fn vrsqrtssrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRSQRTSSRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VRSQRTSSRRM` (`VRSQRTSS`). Computes an approximate reciprocal of the square root of the low single precision floating-point value in the source operand (second operand) stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRSQRTSS](https://www.felixcloutier.com/x86/RSQRTSS.html)
    fn vrsqrtssrrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRSQRTSSRRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VRCPPS128RR` (`VRCPPS`). Performs a SIMD computation of the approximate reciprocals of the four packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRCPPS](https://www.felixcloutier.com/x86/RCPPS.html)
    fn vrcpps128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPS128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VRCPPS128RM` (`VRCPPS`). Performs a SIMD computation of the approximate reciprocals of the four packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRCPPS](https://www.felixcloutier.com/x86/RCPPS.html)
    fn vrcpps128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPS128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VRCPPS256RR` (`VRCPPS`). Performs a SIMD computation of the approximate reciprocals of the four packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRCPPS](https://www.felixcloutier.com/x86/RCPPS.html)
    fn vrcpps256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPS256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VRCPPS256RM` (`VRCPPS`). Performs a SIMD computation of the approximate reciprocals of the four packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRCPPS](https://www.felixcloutier.com/x86/RCPPS.html)
    fn vrcpps256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VRCPPS256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VRCPSSRRR` (`VRCPSS`). Computes of an approximate reciprocal of the low single precision floating-point value in the source operand (second operand) and stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRCPSS](https://www.felixcloutier.com/x86/RCPSS.html)
    fn vrcpssrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRCPSSRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VRCPSSRRM` (`VRCPSS`). Computes of an approximate reciprocal of the low single precision floating-point value in the source operand (second operand) and stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    /// Reference: [Intel x86 docs for VRCPSS](https://www.felixcloutier.com/x86/RCPSS.html)
    fn vrcpssrrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VRCPSSRRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTB128RRR` (`VPCMPGTB`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTB](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtb128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTB128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTB128RRM` (`VPCMPGTB`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTB](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtb128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTB128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTB256RRR` (`VPCMPGTB`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTB](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtb256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTB256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTB256RRM` (`VPCMPGTB`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTB](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtb256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTB256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTW128RRR` (`VPCMPGTW`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTW](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtw128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTW128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTW128RRM` (`VPCMPGTW`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTW](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtw128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTW128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTW256RRR` (`VPCMPGTW`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTW](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtw256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTW256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTW256RRM` (`VPCMPGTW`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTW](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtw256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTW256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTD128RRR` (`VPCMPGTD`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTD](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTD128RRM` (`VPCMPGTD`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTD](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTD256RRR` (`VPCMPGTD`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTD](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTD256RRM` (`VPCMPGTD`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPGTD](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn vpcmpgtd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VMOVDQA128RR` (`VMOVDQA`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQA](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html)
    fn vmovdqa128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQA128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVDQA128RM` (`VMOVDQA`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQA](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html)
    fn vmovdqa128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQA128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVDQA256RR` (`VMOVDQA`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQA](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html)
    fn vmovdqa256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQA256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVDQA256RM` (`VMOVDQA`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQA](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html)
    fn vmovdqa256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQA256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVDQU128RR` (`VMOVDQU`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQU](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html)
    fn vmovdqu128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQU128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVDQU128RM` (`VMOVDQU`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQU](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html)
    fn vmovdqu128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQU128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVDQU256RR` (`VMOVDQU`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQU](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html)
    fn vmovdqu256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQU256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVDQU256RM` (`VMOVDQU`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQU](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html)
    fn vmovdqu256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQU256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQB128RRR` (`VPCMPEQB`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQB](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqb128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQB128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQB128RRM` (`VPCMPEQB`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQB](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqb128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQB128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQB256RRR` (`VPCMPEQB`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQB](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqb256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQB256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQB256RRM` (`VPCMPEQB`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQB](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqb256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQB256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQW128RRR` (`VPCMPEQW`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQW](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqw128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQW128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQW128RRM` (`VPCMPEQW`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQW](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqw128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQW128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQW256RRR` (`VPCMPEQW`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQW](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqw256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQW256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQW256RRM` (`VPCMPEQW`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQW](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqw256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQW256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQD128RRR` (`VPCMPEQD`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQD](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQD128RRM` (`VPCMPEQD`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQD](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQD256RRR` (`VPCMPEQD`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQD](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQD256RRM` (`VPCMPEQD`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for VPCMPEQD](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn vpcmpeqd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VZEROUPPER` (`VZEROUPPER`). In 64-bit mode, the instruction zeroes the bits in positions 128 and higher in YMM0-YMM15 and ZMM0-ZMM15. Outside 64-bit mode, it zeroes those bits only in YMM0-YMM7 and ZMM0-ZMM7. VZEROUPPER does not modify the lower 128 bits of these registers and it does not modify ZMM16-ZMM31.
    /// Reference: [Intel x86 docs for VZEROUPPER](https://www.felixcloutier.com/x86/VZEROUPPER.html)
    fn vzeroupper(&mut self,) -> () {
        self.emit(VZEROUPPER, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VZEROALL` (`VZEROALL`). In 64-bit mode, the instruction zeroes XMM0-XMM15, YMM0-YMM15, and ZMM0-ZMM15. Outside 64-bit mode, it zeroes only XMM0-XMM7, YMM0-YMM7, and ZMM0-ZMM7. VZEROALL does not modify ZMM16-ZMM31.
    /// Reference: [Intel x86 docs for VZEROALL](https://www.felixcloutier.com/x86/VZEROALL.html)
    fn vzeroall(&mut self,) -> () {
        self.emit(VZEROALL, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VHADDPD128RRR` (`VHADDPD`). Adds the double precision floating-point values in the high and low quadwords of the destination operand and stores the result in the low quadword of the destination operand.
    /// Reference: [Intel x86 docs for VHADDPD](https://www.felixcloutier.com/x86/HADDPD.html)
    fn vhaddpd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHADDPD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHADDPD128RRM` (`VHADDPD`). Adds the double precision floating-point values in the high and low quadwords of the destination operand and stores the result in the low quadword of the destination operand.
    /// Reference: [Intel x86 docs for VHADDPD](https://www.felixcloutier.com/x86/HADDPD.html)
    fn vhaddpd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHADDPD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHADDPD256RRR` (`VHADDPD`). Adds the double precision floating-point values in the high and low quadwords of the destination operand and stores the result in the low quadword of the destination operand.
    /// Reference: [Intel x86 docs for VHADDPD](https://www.felixcloutier.com/x86/HADDPD.html)
    fn vhaddpd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHADDPD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHADDPD256RRM` (`VHADDPD`). Adds the double precision floating-point values in the high and low quadwords of the destination operand and stores the result in the low quadword of the destination operand.
    /// Reference: [Intel x86 docs for VHADDPD](https://www.felixcloutier.com/x86/HADDPD.html)
    fn vhaddpd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHADDPD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHADDPS128RRR` (`VHADDPS`). Adds the single precision floating-point values in the first and second dwords of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for VHADDPS](https://www.felixcloutier.com/x86/HADDPS.html)
    fn vhaddps128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHADDPS128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHADDPS128RRM` (`VHADDPS`). Adds the single precision floating-point values in the first and second dwords of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for VHADDPS](https://www.felixcloutier.com/x86/HADDPS.html)
    fn vhaddps128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHADDPS128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHADDPS256RRR` (`VHADDPS`). Adds the single precision floating-point values in the first and second dwords of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for VHADDPS](https://www.felixcloutier.com/x86/HADDPS.html)
    fn vhaddps256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHADDPS256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHADDPS256RRM` (`VHADDPS`). Adds the single precision floating-point values in the first and second dwords of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for VHADDPS](https://www.felixcloutier.com/x86/HADDPS.html)
    fn vhaddps256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHADDPS256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHSUBPD128RRR` (`VHSUBPD`). The HSUBPD instruction subtracts horizontally the packed double precision floating-point numbers of both operands.
    /// Reference: [Intel x86 docs for VHSUBPD](https://www.felixcloutier.com/x86/HSUBPD.html)
    fn vhsubpd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHSUBPD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHSUBPD128RRM` (`VHSUBPD`). The HSUBPD instruction subtracts horizontally the packed double precision floating-point numbers of both operands.
    /// Reference: [Intel x86 docs for VHSUBPD](https://www.felixcloutier.com/x86/HSUBPD.html)
    fn vhsubpd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHSUBPD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHSUBPD256RRR` (`VHSUBPD`). The HSUBPD instruction subtracts horizontally the packed double precision floating-point numbers of both operands.
    /// Reference: [Intel x86 docs for VHSUBPD](https://www.felixcloutier.com/x86/HSUBPD.html)
    fn vhsubpd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHSUBPD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHSUBPD256RRM` (`VHSUBPD`). The HSUBPD instruction subtracts horizontally the packed double precision floating-point numbers of both operands.
    /// Reference: [Intel x86 docs for VHSUBPD](https://www.felixcloutier.com/x86/HSUBPD.html)
    fn vhsubpd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHSUBPD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHSUBPS128RRR` (`VHSUBPS`). Subtracts the single precision floating-point value in the second dword of the destination operand from the first dword of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for VHSUBPS](https://www.felixcloutier.com/x86/HSUBPS.html)
    fn vhsubps128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHSUBPS128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHSUBPS128RRM` (`VHSUBPS`). Subtracts the single precision floating-point value in the second dword of the destination operand from the first dword of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for VHSUBPS](https://www.felixcloutier.com/x86/HSUBPS.html)
    fn vhsubps128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHSUBPS128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHSUBPS256RRR` (`VHSUBPS`). Subtracts the single precision floating-point value in the second dword of the destination operand from the first dword of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for VHSUBPS](https://www.felixcloutier.com/x86/HSUBPS.html)
    fn vhsubps256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHSUBPS256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VHSUBPS256RRM` (`VHSUBPS`). Subtracts the single precision floating-point value in the second dword of the destination operand from the first dword of the destination operand and stores the result in the first dword of the destination operand.
    /// Reference: [Intel x86 docs for VHSUBPS](https://www.felixcloutier.com/x86/HSUBPS.html)
    fn vhsubps256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VHSUBPS256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VMOVDQA128MR` (`VMOVDQA`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQA](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html)
    fn vmovdqa128mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQA128MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVDQA256MR` (`VMOVDQA`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQA](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html)
    fn vmovdqa256mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQA256MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVDQU128MR` (`VMOVDQU`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQU](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html)
    fn vmovdqu128mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQU128MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VMOVDQU256MR` (`VMOVDQU`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for VMOVDQU](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html)
    fn vmovdqu256mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VMOVDQU256MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VLDMXCSRM` (`VLDMXCSR`). Loads the source operand into the MXCSR control/status register. The source operand is a 32-bit memory location. See “MXCSR Control and Status Register” in Chapter 10, of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for a description of the MXCSR register and its contents.
    /// Reference: [Intel x86 docs for VLDMXCSR](https://www.felixcloutier.com/x86/LDMXCSR.html)
    fn vldmxcsrm(&mut self,op0: impl OperandCast) -> () {
        self.emit(VLDMXCSRM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VSTMXCSRM` (`VSTMXCSR`). Stores the contents of the MXCSR control and status register to the destination operand. The destination operand is a 32-bit memory location. The reserved bits in the MXCSR register are stored as 0s.
    /// Reference: [Intel x86 docs for VSTMXCSR](https://www.felixcloutier.com/x86/STMXCSR.html)
    fn vstmxcsrm(&mut self,op0: impl OperandCast) -> () {
        self.emit(VSTMXCSRM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCMPPS128RRRI` (`VCMPPS`). Performs a SIMD compare of the packed single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each of the pairs of packed values.
    /// Reference: [Intel x86 docs for VCMPPS](https://www.felixcloutier.com/x86/CMPPS.html)
    fn vcmpps128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPS128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VCMPPS128RRMI` (`VCMPPS`). Performs a SIMD compare of the packed single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each of the pairs of packed values.
    /// Reference: [Intel x86 docs for VCMPPS](https://www.felixcloutier.com/x86/CMPPS.html)
    fn vcmpps128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPS128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VCMPPS256RRRI` (`VCMPPS`). Performs a SIMD compare of the packed single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each of the pairs of packed values.
    /// Reference: [Intel x86 docs for VCMPPS](https://www.felixcloutier.com/x86/CMPPS.html)
    fn vcmpps256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPS256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VCMPPS256RRMI` (`VCMPPS`). Performs a SIMD compare of the packed single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each of the pairs of packed values.
    /// Reference: [Intel x86 docs for VCMPPS](https://www.felixcloutier.com/x86/CMPPS.html)
    fn vcmpps256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPS256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VCMPPD128RRRI` (`VCMPPD`). Performs a SIMD compare of the packed double precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each pair of packed values in the two source operands.
    /// Reference: [Intel x86 docs for VCMPPD](https://www.felixcloutier.com/x86/CMPPD.html)
    fn vcmppd128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPD128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VCMPPD128RRMI` (`VCMPPD`). Performs a SIMD compare of the packed double precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each pair of packed values in the two source operands.
    /// Reference: [Intel x86 docs for VCMPPD](https://www.felixcloutier.com/x86/CMPPD.html)
    fn vcmppd128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPD128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VCMPPD256RRRI` (`VCMPPD`). Performs a SIMD compare of the packed double precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each pair of packed values in the two source operands.
    /// Reference: [Intel x86 docs for VCMPPD](https://www.felixcloutier.com/x86/CMPPD.html)
    fn vcmppd256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPD256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VCMPPD256RRMI` (`VCMPPD`). Performs a SIMD compare of the packed double precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each pair of packed values in the two source operands.
    /// Reference: [Intel x86 docs for VCMPPD](https://www.felixcloutier.com/x86/CMPPD.html)
    fn vcmppd256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPPD256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VCMPSSRRRI` (`VCMPSS`). Compares the low single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate operand) specifies the type of comparison performed.
    /// Reference: [Intel x86 docs for VCMPSS](https://www.felixcloutier.com/x86/CMPSS.html)
    fn vcmpssrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPSSRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VCMPSSRRMI` (`VCMPSS`). Compares the low single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate operand) specifies the type of comparison performed.
    /// Reference: [Intel x86 docs for VCMPSS](https://www.felixcloutier.com/x86/CMPSS.html)
    fn vcmpssrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPSSRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VCMPSDRRRI`.
    fn vcmpsdrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPSDRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VCMPSDRRMI`.
    fn vcmpsdrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VCMPSDRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VADDSUBPD128RRR` (`VADDSUBPD`). Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the corresponding double precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered double precision floating-point values from the second source operand from the corresponding double precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for VADDSUBPD](https://www.felixcloutier.com/x86/ADDSUBPD.html)
    fn vaddsubpd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDSUBPD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VADDSUBPD128RRM` (`VADDSUBPD`). Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the corresponding double precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered double precision floating-point values from the second source operand from the corresponding double precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for VADDSUBPD](https://www.felixcloutier.com/x86/ADDSUBPD.html)
    fn vaddsubpd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDSUBPD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VADDSUBPD256RRR` (`VADDSUBPD`). Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the corresponding double precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered double precision floating-point values from the second source operand from the corresponding double precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for VADDSUBPD](https://www.felixcloutier.com/x86/ADDSUBPD.html)
    fn vaddsubpd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDSUBPD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VADDSUBPD256RRM` (`VADDSUBPD`). Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the corresponding double precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered double precision floating-point values from the second source operand from the corresponding double precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for VADDSUBPD](https://www.felixcloutier.com/x86/ADDSUBPD.html)
    fn vaddsubpd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDSUBPD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VADDSUBPS128RRR` (`VADDSUBPS`). Adds odd-numbered single precision floating-point values of the first source operand (second operand) with the corresponding single precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered single precision floating-point values from the second source operand from the corresponding single precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for VADDSUBPS](https://www.felixcloutier.com/x86/ADDSUBPS.html)
    fn vaddsubps128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDSUBPS128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VADDSUBPS128RRM` (`VADDSUBPS`). Adds odd-numbered single precision floating-point values of the first source operand (second operand) with the corresponding single precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered single precision floating-point values from the second source operand from the corresponding single precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for VADDSUBPS](https://www.felixcloutier.com/x86/ADDSUBPS.html)
    fn vaddsubps128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDSUBPS128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VADDSUBPS256RRR` (`VADDSUBPS`). Adds odd-numbered single precision floating-point values of the first source operand (second operand) with the corresponding single precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered single precision floating-point values from the second source operand from the corresponding single precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for VADDSUBPS](https://www.felixcloutier.com/x86/ADDSUBPS.html)
    fn vaddsubps256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDSUBPS256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VADDSUBPS256RRM` (`VADDSUBPS`). Adds odd-numbered single precision floating-point values of the first source operand (second operand) with the corresponding single precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered single precision floating-point values from the second source operand from the corresponding single precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    /// Reference: [Intel x86 docs for VADDSUBPS](https://www.felixcloutier.com/x86/ADDSUBPS.html)
    fn vaddsubps256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VADDSUBPS256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMOVMSKB128RR` (`VPMOVMSKB`). Creates a mask made up of the most significant bit of each byte of the source operand (second operand) and stores the result in the low byte or word of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VPMOVMSKB](https://www.felixcloutier.com/x86/PMOVMSKB.html)
    fn vpmovmskb128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVMSKB128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPMOVMSKB256RR` (`VPMOVMSKB`). Creates a mask made up of the most significant bit of each byte of the source operand (second operand) and stores the result in the low byte or word of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VPMOVMSKB](https://www.felixcloutier.com/x86/PMOVMSKB.html)
    fn vpmovmskb256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVMSKB256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPAND128RRR` (`VPAND`). Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for VPAND](https://www.felixcloutier.com/x86/PAND.html)
    fn vpand128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPAND128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPAND128RRM` (`VPAND`). Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for VPAND](https://www.felixcloutier.com/x86/PAND.html)
    fn vpand128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPAND128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPAND256RRR` (`VPAND`). Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for VPAND](https://www.felixcloutier.com/x86/PAND.html)
    fn vpand256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPAND256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPAND256RRM` (`VPAND`). Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for VPAND](https://www.felixcloutier.com/x86/PAND.html)
    fn vpand256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPAND256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPANDN128RRR` (`VPANDN`). Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for VPANDN](https://www.felixcloutier.com/x86/PANDN.html)
    fn vpandn128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPANDN128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPANDN128RRM` (`VPANDN`). Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for VPANDN](https://www.felixcloutier.com/x86/PANDN.html)
    fn vpandn128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPANDN128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPANDN256RRR` (`VPANDN`). Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for VPANDN](https://www.felixcloutier.com/x86/PANDN.html)
    fn vpandn256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPANDN256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPANDN256RRM` (`VPANDN`). Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for VPANDN](https://www.felixcloutier.com/x86/PANDN.html)
    fn vpandn256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPANDN256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPOR128RRR` (`VPOR`). Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
    /// Reference: [Intel x86 docs for VPOR](https://www.felixcloutier.com/x86/POR.html)
    fn vpor128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPOR128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPOR128RRM` (`VPOR`). Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
    /// Reference: [Intel x86 docs for VPOR](https://www.felixcloutier.com/x86/POR.html)
    fn vpor128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPOR128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPOR256RRR` (`VPOR`). Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
    /// Reference: [Intel x86 docs for VPOR](https://www.felixcloutier.com/x86/POR.html)
    fn vpor256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPOR256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPOR256RRM` (`VPOR`). Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
    /// Reference: [Intel x86 docs for VPOR](https://www.felixcloutier.com/x86/POR.html)
    fn vpor256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPOR256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPXOR128RRR` (`VPXOR`). Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
    /// Reference: [Intel x86 docs for VPXOR](https://www.felixcloutier.com/x86/PXOR.html)
    fn vpxor128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPXOR128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPXOR128RRM` (`VPXOR`). Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
    /// Reference: [Intel x86 docs for VPXOR](https://www.felixcloutier.com/x86/PXOR.html)
    fn vpxor128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPXOR128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPXOR256RRR` (`VPXOR`). Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
    /// Reference: [Intel x86 docs for VPXOR](https://www.felixcloutier.com/x86/PXOR.html)
    fn vpxor256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPXOR256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPXOR256RRM` (`VPXOR`). Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
    /// Reference: [Intel x86 docs for VPXOR](https://www.felixcloutier.com/x86/PXOR.html)
    fn vpxor256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPXOR256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VLDDQU128RM` (`VLDDQU`). The instruction is functionally similar to (V)MOVDQU ymm/xmm, m256/m128 for loading from memory. That is: 32/16 bytes of data starting at an address specified by the source memory operand (second operand) are fetched from memory and placed in a destination register (first operand). The source operand need not be aligned on a 32/16-byte boundary. Up to 64/32 bytes may be loaded from memory; this is implementation dependent.
    /// Reference: [Intel x86 docs for VLDDQU](https://www.felixcloutier.com/x86/LDDQU.html)
    fn vlddqu128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VLDDQU128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VLDDQU256RM` (`VLDDQU`). The instruction is functionally similar to (V)MOVDQU ymm/xmm, m256/m128 for loading from memory. That is: 32/16 bytes of data starting at an address specified by the source memory operand (second operand) are fetched from memory and placed in a destination register (first operand). The source operand need not be aligned on a 32/16-byte boundary. Up to 64/32 bytes may be loaded from memory; this is implementation dependent.
    /// Reference: [Intel x86 docs for VLDDQU](https://www.felixcloutier.com/x86/LDDQU.html)
    fn vlddqu256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VLDDQU256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPHADDW128RRR` (`VPHADDW`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDW](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn vphaddw128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDW128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHADDW128RRM` (`VPHADDW`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDW](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn vphaddw128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDW128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHADDW256RRR` (`VPHADDW`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDW](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn vphaddw256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDW256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHADDW256RRM` (`VPHADDW`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDW](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn vphaddw256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDW256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHADDD128RRR` (`VPHADDD`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDD](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn vphaddd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHADDD128RRM` (`VPHADDD`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDD](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn vphaddd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHADDD256RRR` (`VPHADDD`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDD](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn vphaddd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHADDD256RRM` (`VPHADDD`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDD](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn vphaddd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHADDSW128RRR` (`VPHADDSW`). (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDSW](https://www.felixcloutier.com/x86/PHADDSW.html)
    fn vphaddsw128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDSW128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHADDSW128RRM` (`VPHADDSW`). (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDSW](https://www.felixcloutier.com/x86/PHADDSW.html)
    fn vphaddsw128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDSW128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHADDSW256RRR` (`VPHADDSW`). (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDSW](https://www.felixcloutier.com/x86/PHADDSW.html)
    fn vphaddsw256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDSW256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHADDSW256RRM` (`VPHADDSW`). (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHADDSW](https://www.felixcloutier.com/x86/PHADDSW.html)
    fn vphaddsw256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHADDSW256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBW128RRR` (`VPHSUBW`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBW](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn vphsubw128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBW128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBW128RRM` (`VPHSUBW`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBW](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn vphsubw128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBW128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBW256RRR` (`VPHSUBW`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBW](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn vphsubw256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBW256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBW256RRM` (`VPHSUBW`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBW](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn vphsubw256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBW256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBD128RRR` (`VPHSUBD`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBD](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn vphsubd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBD128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBD128RRM` (`VPHSUBD`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBD](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn vphsubd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBD128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBD256RRR` (`VPHSUBD`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBD](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn vphsubd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBD256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBD256RRM` (`VPHSUBD`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBD](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn vphsubd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBD256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBSW128RRR` (`VPHSUBSW`). (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBSW](https://www.felixcloutier.com/x86/PHSUBSW.html)
    fn vphsubsw128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBSW128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBSW128RRM` (`VPHSUBSW`). (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBSW](https://www.felixcloutier.com/x86/PHSUBSW.html)
    fn vphsubsw128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBSW128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBSW256RRR` (`VPHSUBSW`). (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBSW](https://www.felixcloutier.com/x86/PHSUBSW.html)
    fn vphsubsw256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBSW256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHSUBSW256RRM` (`VPHSUBSW`). (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for VPHSUBSW](https://www.felixcloutier.com/x86/PHSUBSW.html)
    fn vphsubsw256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPHSUBSW256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGNB128RRR` (`VPSIGNB`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGNB](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignb128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGNB128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGNB128RRM` (`VPSIGNB`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGNB](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignb128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGNB128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGNB256RRR` (`VPSIGNB`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGNB](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignb256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGNB256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGNB256RRM` (`VPSIGNB`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGNB](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignb256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGNB256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGNW128RRR` (`VPSIGNW`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGNW](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignw128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGNW128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGNW128RRM` (`VPSIGNW`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGNW](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignw128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGNW128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGNW256RRR` (`VPSIGNW`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGNW](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignw256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGNW256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGNW256RRM` (`VPSIGNW`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGNW](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignw256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGNW256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGND128RRR` (`VPSIGND`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGND](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignd128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGND128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGND128RRM` (`VPSIGND`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGND](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignd128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGND128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGND256RRR` (`VPSIGND`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGND](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignd256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGND256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPSIGND256RRM` (`VPSIGND`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for VPSIGND](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn vpsignd256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSIGND256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VTESTPS128RR` (`VTESTPS`). VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
    /// Reference: [Intel x86 docs for VTESTPS](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html)
    fn vtestps128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VTESTPS128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VTESTPS128RM` (`VTESTPS`). VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
    /// Reference: [Intel x86 docs for VTESTPS](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html)
    fn vtestps128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VTESTPS128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VTESTPS256RR` (`VTESTPS`). VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
    /// Reference: [Intel x86 docs for VTESTPS](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html)
    fn vtestps256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VTESTPS256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VTESTPS256RM` (`VTESTPS`). VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
    /// Reference: [Intel x86 docs for VTESTPS](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html)
    fn vtestps256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VTESTPS256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VTESTPD128RR` (`VTESTPD`). VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
    /// Reference: [Intel x86 docs for VTESTPD](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html)
    fn vtestpd128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VTESTPD128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VTESTPD128RM` (`VTESTPD`). VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
    /// Reference: [Intel x86 docs for VTESTPD](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html)
    fn vtestpd128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VTESTPD128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VTESTPD256RR` (`VTESTPD`). VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
    /// Reference: [Intel x86 docs for VTESTPD](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html)
    fn vtestpd256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VTESTPD256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VTESTPD256RM` (`VTESTPD`). VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
    /// Reference: [Intel x86 docs for VTESTPD](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html)
    fn vtestpd256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VTESTPD256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPTEST128RR` (`VPTEST`). PTEST and VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND of the first source operand (first operand) and the second source operand (second operand). VPTEST sets the CF flag if all bits in the result are 0 of the bitwise AND of the second source operand (second operand) and the logical NOT of the destination operand.
    /// Reference: [Intel x86 docs for VPTEST](https://www.felixcloutier.com/x86/PTEST.html)
    fn vptest128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPTEST128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPTEST128RM` (`VPTEST`). PTEST and VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND of the first source operand (first operand) and the second source operand (second operand). VPTEST sets the CF flag if all bits in the result are 0 of the bitwise AND of the second source operand (second operand) and the logical NOT of the destination operand.
    /// Reference: [Intel x86 docs for VPTEST](https://www.felixcloutier.com/x86/PTEST.html)
    fn vptest128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPTEST128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPTEST256RR` (`VPTEST`). PTEST and VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND of the first source operand (first operand) and the second source operand (second operand). VPTEST sets the CF flag if all bits in the result are 0 of the bitwise AND of the second source operand (second operand) and the logical NOT of the destination operand.
    /// Reference: [Intel x86 docs for VPTEST](https://www.felixcloutier.com/x86/PTEST.html)
    fn vptest256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPTEST256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPTEST256RM` (`VPTEST`). PTEST and VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND of the first source operand (first operand) and the second source operand (second operand). VPTEST sets the CF flag if all bits in the result are 0 of the bitwise AND of the second source operand (second operand) and the logical NOT of the destination operand.
    /// Reference: [Intel x86 docs for VPTEST](https://www.felixcloutier.com/x86/PTEST.html)
    fn vptest256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPTEST256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF128_256RR` (`VBROADCASTF128`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF128](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf128_256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF128_256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VBROADCASTF128_256RM` (`VBROADCASTF128`). VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    /// Reference: [Intel x86 docs for VBROADCASTF128](https://www.felixcloutier.com/x86/VBROADCAST.html)
    fn vbroadcastf128_256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTF128_256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQQ128RRR` (`VPCMPEQQ`). Performs an SIMD compare for equality of the packed quadwords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for VPCMPEQQ](https://www.felixcloutier.com/x86/PCMPEQQ.html)
    fn vpcmpeqq128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQQ128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQQ128RRM` (`VPCMPEQQ`). Performs an SIMD compare for equality of the packed quadwords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for VPCMPEQQ](https://www.felixcloutier.com/x86/PCMPEQQ.html)
    fn vpcmpeqq128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQQ128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQQ256RRR` (`VPCMPEQQ`). Performs an SIMD compare for equality of the packed quadwords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for VPCMPEQQ](https://www.felixcloutier.com/x86/PCMPEQQ.html)
    fn vpcmpeqq256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQQ256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPEQQ256RRM` (`VPCMPEQQ`). Performs an SIMD compare for equality of the packed quadwords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for VPCMPEQQ](https://www.felixcloutier.com/x86/PCMPEQQ.html)
    fn vpcmpeqq256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPEQQ256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTQ128RRR` (`VPCMPGTQ`). Performs an SIMD signed compare for the packed quadwords in the destination operand (first operand) and the source operand (second operand). If the data element in the first (destination) operand is greater than the corresponding element in the second (source) operand, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for VPCMPGTQ](https://www.felixcloutier.com/x86/PCMPGTQ.html)
    fn vpcmpgtq128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTQ128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTQ128RRM` (`VPCMPGTQ`). Performs an SIMD signed compare for the packed quadwords in the destination operand (first operand) and the source operand (second operand). If the data element in the first (destination) operand is greater than the corresponding element in the second (source) operand, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for VPCMPGTQ](https://www.felixcloutier.com/x86/PCMPGTQ.html)
    fn vpcmpgtq128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTQ128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTQ256RRR` (`VPCMPGTQ`). Performs an SIMD signed compare for the packed quadwords in the destination operand (first operand) and the source operand (second operand). If the data element in the first (destination) operand is greater than the corresponding element in the second (source) operand, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for VPCMPGTQ](https://www.felixcloutier.com/x86/PCMPGTQ.html)
    fn vpcmpgtq256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTQ256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPGTQ256RRM` (`VPCMPGTQ`). Performs an SIMD signed compare for the packed quadwords in the destination operand (first operand) and the source operand (second operand). If the data element in the first (destination) operand is greater than the corresponding element in the second (source) operand, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    /// Reference: [Intel x86 docs for VPCMPGTQ](https://www.felixcloutier.com/x86/PCMPGTQ.html)
    fn vpcmpgtq256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPGTQ256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPHMINPOSUW128RR` (`VPHMINPOSUW`). Determine the minimum unsigned word value in the source operand (second operand) and place the unsigned word in the low word (bits 0-15) of the destination operand (first operand). The word index of the minimum value is stored in bits 16-18 of the destination operand. The remaining upper bits of the destination are set to zero.
    /// Reference: [Intel x86 docs for VPHMINPOSUW](https://www.felixcloutier.com/x86/PHMINPOSUW.html)
    fn vphminposuw128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPHMINPOSUW128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPHMINPOSUW128RM` (`VPHMINPOSUW`). Determine the minimum unsigned word value in the source operand (second operand) and place the unsigned word in the low word (bits 0-15) of the destination operand (first operand). The word index of the minimum value is stored in bits 16-18 of the destination operand. The remaining upper bits of the destination are set to zero.
    /// Reference: [Intel x86 docs for VPHMINPOSUW](https://www.felixcloutier.com/x86/PHMINPOSUW.html)
    fn vphminposuw128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPHMINPOSUW128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VPERM2F128_256RRRI` (`VPERM2F128`). Permute 128 bit floating-point-containing fields from the first source operand (second operand) and second source operand (third operand) using bits in the 8-bit immediate and store results in the destination operand (first operand). The first source operand is a YMM register, the second source operand is a YMM register or a 256-bit memory location, and the destination operand is a YMM register.
    /// Reference: [Intel x86 docs for VPERM2F128](https://www.felixcloutier.com/x86/VPERM2F128.html)
    fn vperm2f128_256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPERM2F128_256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPERM2F128_256RRMI` (`VPERM2F128`). Permute 128 bit floating-point-containing fields from the first source operand (second operand) and second source operand (third operand) using bits in the 8-bit immediate and store results in the destination operand (first operand). The first source operand is a YMM register, the second source operand is a YMM register or a 256-bit memory location, and the destination operand is a YMM register.
    /// Reference: [Intel x86 docs for VPERM2F128](https://www.felixcloutier.com/x86/VPERM2F128.html)
    fn vperm2f128_256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPERM2F128_256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VROUNDPS128RRI` (`VROUNDPS`). Round the 4 single precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a single precision floating-point value.
    /// Reference: [Intel x86 docs for VROUNDPS](https://www.felixcloutier.com/x86/ROUNDPS.html)
    fn vroundps128rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VROUNDPS128RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VROUNDPS128RMI` (`VROUNDPS`). Round the 4 single precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a single precision floating-point value.
    /// Reference: [Intel x86 docs for VROUNDPS](https://www.felixcloutier.com/x86/ROUNDPS.html)
    fn vroundps128rmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VROUNDPS128RMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VROUNDPS256RRI` (`VROUNDPS`). Round the 4 single precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a single precision floating-point value.
    /// Reference: [Intel x86 docs for VROUNDPS](https://www.felixcloutier.com/x86/ROUNDPS.html)
    fn vroundps256rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VROUNDPS256RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VROUNDPS256RMI` (`VROUNDPS`). Round the 4 single precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a single precision floating-point value.
    /// Reference: [Intel x86 docs for VROUNDPS](https://www.felixcloutier.com/x86/ROUNDPS.html)
    fn vroundps256rmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VROUNDPS256RMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VROUNDPD128RRI` (`VROUNDPD`). Round the 2 double precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a double precision floating-point value.
    /// Reference: [Intel x86 docs for VROUNDPD](https://www.felixcloutier.com/x86/ROUNDPD.html)
    fn vroundpd128rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VROUNDPD128RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VROUNDPD128RMI` (`VROUNDPD`). Round the 2 double precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a double precision floating-point value.
    /// Reference: [Intel x86 docs for VROUNDPD](https://www.felixcloutier.com/x86/ROUNDPD.html)
    fn vroundpd128rmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VROUNDPD128RMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VROUNDPD256RRI` (`VROUNDPD`). Round the 2 double precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a double precision floating-point value.
    /// Reference: [Intel x86 docs for VROUNDPD](https://www.felixcloutier.com/x86/ROUNDPD.html)
    fn vroundpd256rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VROUNDPD256RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VROUNDPD256RMI` (`VROUNDPD`). Round the 2 double precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a double precision floating-point value.
    /// Reference: [Intel x86 docs for VROUNDPD](https://www.felixcloutier.com/x86/ROUNDPD.html)
    fn vroundpd256rmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VROUNDPD256RMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VROUNDSSRRRI` (`VROUNDSS`). Round the single precision floating-point value in the lowest dword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a single precision floating-point input to an integer value and returns the result as a single precision floating-point value in the lowest position. The upper three single precision floating-point values in the destination are retained.
    /// Reference: [Intel x86 docs for VROUNDSS](https://www.felixcloutier.com/x86/ROUNDSS.html)
    fn vroundssrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VROUNDSSRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VROUNDSSRRMI` (`VROUNDSS`). Round the single precision floating-point value in the lowest dword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a single precision floating-point input to an integer value and returns the result as a single precision floating-point value in the lowest position. The upper three single precision floating-point values in the destination are retained.
    /// Reference: [Intel x86 docs for VROUNDSS](https://www.felixcloutier.com/x86/ROUNDSS.html)
    fn vroundssrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VROUNDSSRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VROUNDSDRRRI` (`VROUNDSD`). Round the double precision floating-point value in the lower qword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a double precision floating-point input to an integer value and returns the integer result as a double precision floating-point value in the lowest position. The upper double precision floating-point value in the destination is retained.
    /// Reference: [Intel x86 docs for VROUNDSD](https://www.felixcloutier.com/x86/ROUNDSD.html)
    fn vroundsdrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VROUNDSDRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VROUNDSDRRMI` (`VROUNDSD`). Round the double precision floating-point value in the lower qword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a double precision floating-point input to an integer value and returns the integer result as a double precision floating-point value in the lowest position. The upper double precision floating-point value in the destination is retained.
    /// Reference: [Intel x86 docs for VROUNDSD](https://www.felixcloutier.com/x86/ROUNDSD.html)
    fn vroundsdrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VROUNDSDRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDPS128RRRI` (`VBLENDPS`). Packed single precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [7:0] determine whether the corresponding single precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is “1”, then the single precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for VBLENDPS](https://www.felixcloutier.com/x86/BLENDPS.html)
    fn vblendps128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDPS128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDPS128RRMI` (`VBLENDPS`). Packed single precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [7:0] determine whether the corresponding single precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is “1”, then the single precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for VBLENDPS](https://www.felixcloutier.com/x86/BLENDPS.html)
    fn vblendps128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDPS128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDPS256RRRI` (`VBLENDPS`). Packed single precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [7:0] determine whether the corresponding single precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is “1”, then the single precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for VBLENDPS](https://www.felixcloutier.com/x86/BLENDPS.html)
    fn vblendps256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDPS256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDPS256RRMI` (`VBLENDPS`). Packed single precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [7:0] determine whether the corresponding single precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is “1”, then the single precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for VBLENDPS](https://www.felixcloutier.com/x86/BLENDPS.html)
    fn vblendps256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDPS256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDPD128RRRI` (`VBLENDPD`). Double-precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [3:0] determine whether the corresponding double precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is ”1”, then the double precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for VBLENDPD](https://www.felixcloutier.com/x86/BLENDPD.html)
    fn vblendpd128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDPD128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDPD128RRMI` (`VBLENDPD`). Double-precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [3:0] determine whether the corresponding double precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is ”1”, then the double precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for VBLENDPD](https://www.felixcloutier.com/x86/BLENDPD.html)
    fn vblendpd128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDPD128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDPD256RRRI` (`VBLENDPD`). Double-precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [3:0] determine whether the corresponding double precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is ”1”, then the double precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for VBLENDPD](https://www.felixcloutier.com/x86/BLENDPD.html)
    fn vblendpd256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDPD256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDPD256RRMI` (`VBLENDPD`). Double-precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [3:0] determine whether the corresponding double precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is ”1”, then the double precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    /// Reference: [Intel x86 docs for VBLENDPD](https://www.felixcloutier.com/x86/BLENDPD.html)
    fn vblendpd256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDPD256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPBLENDW128RRRI` (`VPBLENDW`). Words from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source. If a bit in the mask, corresponding to a word, is “1", then the word is copied, else the word element in the destination operand is unchanged.
    /// Reference: [Intel x86 docs for VPBLENDW](https://www.felixcloutier.com/x86/PBLENDW.html)
    fn vpblendw128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDW128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPBLENDW128RRMI` (`VPBLENDW`). Words from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source. If a bit in the mask, corresponding to a word, is “1", then the word is copied, else the word element in the destination operand is unchanged.
    /// Reference: [Intel x86 docs for VPBLENDW](https://www.felixcloutier.com/x86/PBLENDW.html)
    fn vpblendw128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDW128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPBLENDW256RRRI` (`VPBLENDW`). Words from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source. If a bit in the mask, corresponding to a word, is “1", then the word is copied, else the word element in the destination operand is unchanged.
    /// Reference: [Intel x86 docs for VPBLENDW](https://www.felixcloutier.com/x86/PBLENDW.html)
    fn vpblendw256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDW256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPBLENDW256RRMI` (`VPBLENDW`). Words from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source. If a bit in the mask, corresponding to a word, is “1", then the word is copied, else the word element in the destination operand is unchanged.
    /// Reference: [Intel x86 docs for VPBLENDW](https://www.felixcloutier.com/x86/PBLENDW.html)
    fn vpblendw256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDW256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF128RRRI` (`VINSERTF128`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF128](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VINSERTF128RRMI` (`VINSERTF128`). VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    /// Reference: [Intel x86 docs for VINSERTF128](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html)
    fn vinsertf128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VINSERTF128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VEXTRACTF128RRI` (`VEXTRACTF128`). VEXTRACTF128/VEXTRACTF32x4 and VEXTRACTF64x2 extract 128-bits of single precision floating-point values from the source operand (the second operand) and store to the low 128-bit of the destination operand (the first operand). The 128-bit data extraction occurs at an 128-bit granular offset specified by imm8[0] (256-bit) or imm8[1:0] as the multiply factor. The destination may be either a vector register or an 128-bit memory location.
    /// Reference: [Intel x86 docs for VEXTRACTF128](https://www.felixcloutier.com/x86/VEXTRACTF128%3AVEXTRACTF32x4%3AVEXTRACTF64x2%3AVEXTRACTF32x8%3AVEXTRACTF64x4.html)
    fn vextractf128rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VEXTRACTF128RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VEXTRACTF128MRI` (`VEXTRACTF128`). VEXTRACTF128/VEXTRACTF32x4 and VEXTRACTF64x2 extract 128-bits of single precision floating-point values from the source operand (the second operand) and store to the low 128-bit of the destination operand (the first operand). The 128-bit data extraction occurs at an 128-bit granular offset specified by imm8[0] (256-bit) or imm8[1:0] as the multiply factor. The destination may be either a vector register or an 128-bit memory location.
    /// Reference: [Intel x86 docs for VEXTRACTF128](https://www.felixcloutier.com/x86/VEXTRACTF128%3AVEXTRACTF32x4%3AVEXTRACTF64x2%3AVEXTRACTF32x8%3AVEXTRACTF64x4.html)
    fn vextractf128mri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VEXTRACTF128MRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPPS128RRRI` (`VDPPS`). Conditionally multiplies the packed single precision floating-point values in the destination operand (first operand) with the packed single precision floats in the source (second operand) depending on a mask extracted from the high 4 bits of the immediate byte (third operand). If a condition mask bit in imm8[7:4] is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    /// Reference: [Intel x86 docs for VDPPS](https://www.felixcloutier.com/x86/DPPS.html)
    fn vdpps128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VDPPS128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VDPPS128RRMI` (`VDPPS`). Conditionally multiplies the packed single precision floating-point values in the destination operand (first operand) with the packed single precision floats in the source (second operand) depending on a mask extracted from the high 4 bits of the immediate byte (third operand). If a condition mask bit in imm8[7:4] is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    /// Reference: [Intel x86 docs for VDPPS](https://www.felixcloutier.com/x86/DPPS.html)
    fn vdpps128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VDPPS128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VDPPS256RRRI` (`VDPPS`). Conditionally multiplies the packed single precision floating-point values in the destination operand (first operand) with the packed single precision floats in the source (second operand) depending on a mask extracted from the high 4 bits of the immediate byte (third operand). If a condition mask bit in imm8[7:4] is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    /// Reference: [Intel x86 docs for VDPPS](https://www.felixcloutier.com/x86/DPPS.html)
    fn vdpps256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VDPPS256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VDPPS256RRMI` (`VDPPS`). Conditionally multiplies the packed single precision floating-point values in the destination operand (first operand) with the packed single precision floats in the source (second operand) depending on a mask extracted from the high 4 bits of the immediate byte (third operand). If a condition mask bit in imm8[7:4] is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    /// Reference: [Intel x86 docs for VDPPS](https://www.felixcloutier.com/x86/DPPS.html)
    fn vdpps256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VDPPS256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VDPPD128RRRI` (`VDPPD`). Conditionally multiplies the packed double precision floating-point values in the destination operand (first operand) with the packed double precision floating-point values in the source (second operand) depending on a mask extracted from bits [5:4] of the immediate operand (third operand). If a condition mask bit is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    /// Reference: [Intel x86 docs for VDPPD](https://www.felixcloutier.com/x86/DPPD.html)
    fn vdppd128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VDPPD128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VDPPD128RRMI` (`VDPPD`). Conditionally multiplies the packed double precision floating-point values in the destination operand (first operand) with the packed double precision floating-point values in the source (second operand) depending on a mask extracted from bits [5:4] of the immediate operand (third operand). If a condition mask bit is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    /// Reference: [Intel x86 docs for VDPPD](https://www.felixcloutier.com/x86/DPPD.html)
    fn vdppd128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VDPPD128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VMPSADBW128RRRI` (`VMPSADBW`). (V)MPSADBW calculates packed word results of sum-absolute-difference (SAD) of unsigned bytes from two blocks of 32-bit dword elements, using two select fields in the immediate byte to select the offsets of the two blocks within the first source operand and the second operand. Packed SAD word results are calculated within each 128-bit lane. Each SAD word result is calculated between a stationary block_2 (whose offset within the second source operand is selected by a two bit select control, multiplied by 32 bits) and a sliding block_1 at consecutive byte-granular position within the first source operand. The offset of the first 32-bit block of block_1 is selectable using a one bit select control, multiplied by 32 bits.
    /// Reference: [Intel x86 docs for VMPSADBW](https://www.felixcloutier.com/x86/MPSADBW.html)
    fn vmpsadbw128rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VMPSADBW128RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VMPSADBW128RRMI` (`VMPSADBW`). (V)MPSADBW calculates packed word results of sum-absolute-difference (SAD) of unsigned bytes from two blocks of 32-bit dword elements, using two select fields in the immediate byte to select the offsets of the two blocks within the first source operand and the second operand. Packed SAD word results are calculated within each 128-bit lane. Each SAD word result is calculated between a stationary block_2 (whose offset within the second source operand is selected by a two bit select control, multiplied by 32 bits) and a sliding block_1 at consecutive byte-granular position within the first source operand. The offset of the first 32-bit block of block_1 is selectable using a one bit select control, multiplied by 32 bits.
    /// Reference: [Intel x86 docs for VMPSADBW](https://www.felixcloutier.com/x86/MPSADBW.html)
    fn vmpsadbw128rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VMPSADBW128RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VMPSADBW256RRRI` (`VMPSADBW`). (V)MPSADBW calculates packed word results of sum-absolute-difference (SAD) of unsigned bytes from two blocks of 32-bit dword elements, using two select fields in the immediate byte to select the offsets of the two blocks within the first source operand and the second operand. Packed SAD word results are calculated within each 128-bit lane. Each SAD word result is calculated between a stationary block_2 (whose offset within the second source operand is selected by a two bit select control, multiplied by 32 bits) and a sliding block_1 at consecutive byte-granular position within the first source operand. The offset of the first 32-bit block of block_1 is selectable using a one bit select control, multiplied by 32 bits.
    /// Reference: [Intel x86 docs for VMPSADBW](https://www.felixcloutier.com/x86/MPSADBW.html)
    fn vmpsadbw256rrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VMPSADBW256RRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VMPSADBW256RRMI` (`VMPSADBW`). (V)MPSADBW calculates packed word results of sum-absolute-difference (SAD) of unsigned bytes from two blocks of 32-bit dword elements, using two select fields in the immediate byte to select the offsets of the two blocks within the first source operand and the second operand. Packed SAD word results are calculated within each 128-bit lane. Each SAD word result is calculated between a stationary block_2 (whose offset within the second source operand is selected by a two bit select control, multiplied by 32 bits) and a sliding block_1 at consecutive byte-granular position within the first source operand. The offset of the first 32-bit block of block_1 is selectable using a one bit select control, multiplied by 32 bits.
    /// Reference: [Intel x86 docs for VMPSADBW](https://www.felixcloutier.com/x86/MPSADBW.html)
    fn vmpsadbw256rrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VMPSADBW256RRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDVPS128RRRR` (`VBLENDVPS`). Conditionally copy each dword data element of single precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each dword element of the mask register.
    /// Reference: [Intel x86 docs for VBLENDVPS](https://www.felixcloutier.com/x86/BLENDVPS.html)
    fn vblendvps128rrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDVPS128RRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDVPS128RRMR` (`VBLENDVPS`). Conditionally copy each dword data element of single precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each dword element of the mask register.
    /// Reference: [Intel x86 docs for VBLENDVPS](https://www.felixcloutier.com/x86/BLENDVPS.html)
    fn vblendvps128rrmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDVPS128RRMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDVPS256RRRR` (`VBLENDVPS`). Conditionally copy each dword data element of single precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each dword element of the mask register.
    /// Reference: [Intel x86 docs for VBLENDVPS](https://www.felixcloutier.com/x86/BLENDVPS.html)
    fn vblendvps256rrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDVPS256RRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDVPS256RRMR` (`VBLENDVPS`). Conditionally copy each dword data element of single precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each dword element of the mask register.
    /// Reference: [Intel x86 docs for VBLENDVPS](https://www.felixcloutier.com/x86/BLENDVPS.html)
    fn vblendvps256rrmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDVPS256RRMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDVPD128RRRR` (`VBLENDVPD`). Conditionally copy each quadword data element of double precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each quadword element of the mask register.
    /// Reference: [Intel x86 docs for VBLENDVPD](https://www.felixcloutier.com/x86/BLENDVPD.html)
    fn vblendvpd128rrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDVPD128RRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDVPD128RRMR` (`VBLENDVPD`). Conditionally copy each quadword data element of double precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each quadword element of the mask register.
    /// Reference: [Intel x86 docs for VBLENDVPD](https://www.felixcloutier.com/x86/BLENDVPD.html)
    fn vblendvpd128rrmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDVPD128RRMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDVPD256RRRR` (`VBLENDVPD`). Conditionally copy each quadword data element of double precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each quadword element of the mask register.
    /// Reference: [Intel x86 docs for VBLENDVPD](https://www.felixcloutier.com/x86/BLENDVPD.html)
    fn vblendvpd256rrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDVPD256RRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VBLENDVPD256RRMR` (`VBLENDVPD`). Conditionally copy each quadword data element of double precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each quadword element of the mask register.
    /// Reference: [Intel x86 docs for VBLENDVPD](https://www.felixcloutier.com/x86/BLENDVPD.html)
    fn vblendvpd256rrmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VBLENDVPD256RRMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPBLENDVB128RRRR` (`VPBLENDVB`). Conditionally copies byte elements from the source operand (second operand) to the destination operand (first operand) depending on mask bits defined in the implicit third register argument, XMM0. The mask bits are the most significant bit in each byte element of the XMM0 register.
    /// Reference: [Intel x86 docs for VPBLENDVB](https://www.felixcloutier.com/x86/PBLENDVB.html)
    fn vpblendvb128rrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDVB128RRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPBLENDVB128RRMR` (`VPBLENDVB`). Conditionally copies byte elements from the source operand (second operand) to the destination operand (first operand) depending on mask bits defined in the implicit third register argument, XMM0. The mask bits are the most significant bit in each byte element of the XMM0 register.
    /// Reference: [Intel x86 docs for VPBLENDVB](https://www.felixcloutier.com/x86/PBLENDVB.html)
    fn vpblendvb128rrmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDVB128RRMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPBLENDVB256RRRR` (`VPBLENDVB`). Conditionally copies byte elements from the source operand (second operand) to the destination operand (first operand) depending on mask bits defined in the implicit third register argument, XMM0. The mask bits are the most significant bit in each byte element of the XMM0 register.
    /// Reference: [Intel x86 docs for VPBLENDVB](https://www.felixcloutier.com/x86/PBLENDVB.html)
    fn vpblendvb256rrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDVB256RRRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPBLENDVB256RRMR` (`VPBLENDVB`). Conditionally copies byte elements from the source operand (second operand) to the destination operand (first operand) depending on mask bits defined in the implicit third register argument, XMM0. The mask bits are the most significant bit in each byte element of the XMM0 register.
    /// Reference: [Intel x86 docs for VPBLENDVB](https://www.felixcloutier.com/x86/PBLENDVB.html)
    fn vpblendvb256rrmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPBLENDVB256RRMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),)
    }
    /// Emits `VPCMPESTRMRRI` (`VPCMPESTRM`). The instruction compares data from two string fragments based on the encoded value in the imm8 contol byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates a mask stored to XMM0.
    /// Reference: [Intel x86 docs for VPCMPESTRM](https://www.felixcloutier.com/x86/PCMPESTRM.html)
    fn vpcmpestrmrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPESTRMRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPESTRMRMI` (`VPCMPESTRM`). The instruction compares data from two string fragments based on the encoded value in the imm8 contol byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates a mask stored to XMM0.
    /// Reference: [Intel x86 docs for VPCMPESTRM](https://www.felixcloutier.com/x86/PCMPESTRM.html)
    fn vpcmpestrmrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPESTRMRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPESTRIRRI` (`VPCMPESTRI`). The instruction compares and processes data from two string fragments based on the encoded value in the imm8 control byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates an index stored to the count register (ECX).
    /// Reference: [Intel x86 docs for VPCMPESTRI](https://www.felixcloutier.com/x86/PCMPESTRI.html)
    fn vpcmpestrirri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPESTRIRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPESTRIRMI` (`VPCMPESTRI`). The instruction compares and processes data from two string fragments based on the encoded value in the imm8 control byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates an index stored to the count register (ECX).
    /// Reference: [Intel x86 docs for VPCMPESTRI](https://www.felixcloutier.com/x86/PCMPESTRI.html)
    fn vpcmpestrirmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPESTRIRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPISTRMRRI` (`VPCMPISTRM`). The instruction compares data from two strings based on the encoded value in the imm8 byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”) generating a mask stored to XMM0.
    /// Reference: [Intel x86 docs for VPCMPISTRM](https://www.felixcloutier.com/x86/PCMPISTRM.html)
    fn vpcmpistrmrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPISTRMRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPISTRMRMI` (`VPCMPISTRM`). The instruction compares data from two strings based on the encoded value in the imm8 byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”) generating a mask stored to XMM0.
    /// Reference: [Intel x86 docs for VPCMPISTRM](https://www.felixcloutier.com/x86/PCMPISTRM.html)
    fn vpcmpistrmrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPISTRMRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPISTRIRRI` (`VPCMPISTRI`). The instruction compares data from two strings based on the encoded value in the imm8 control byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates an index stored to ECX.
    /// Reference: [Intel x86 docs for VPCMPISTRI](https://www.felixcloutier.com/x86/PCMPISTRI.html)
    fn vpcmpistrirri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPISTRIRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPCMPISTRIRMI` (`VPCMPISTRI`). The instruction compares data from two strings based on the encoded value in the imm8 control byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates an index stored to ECX.
    /// Reference: [Intel x86 docs for VPCMPISTRI](https://www.felixcloutier.com/x86/PCMPISTRI.html)
    fn vpcmpistrirmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPCMPISTRIRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
