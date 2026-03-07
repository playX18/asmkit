pub trait X86MMXEmitter: Emitter {
    /// Emits `MMX_PUNPCKLBWRR` (`PUNPCKLBW`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLBW](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn mmx_punpcklbwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKLBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PUNPCKLBWRM` (`PUNPCKLBW`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLBW](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn mmx_punpcklbwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKLBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PUNPCKLWDRR` (`PUNPCKLWD`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLWD](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn mmx_punpcklwdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKLWDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PUNPCKLWDRM` (`PUNPCKLWD`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLWD](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn mmx_punpcklwdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKLWDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PUNPCKLDQRR` (`PUNPCKLDQ`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLDQ](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn mmx_punpckldqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKLDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PUNPCKLDQRM` (`PUNPCKLDQ`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLDQ](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn mmx_punpckldqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKLDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PACKSSWBRR` (`PACKSSWB`). Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
    /// Reference: [Intel x86 docs for PACKSSWB](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html)
    fn mmx_packsswbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PACKSSWBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PACKSSWBRM` (`PACKSSWB`). Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
    /// Reference: [Intel x86 docs for PACKSSWB](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html)
    fn mmx_packsswbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PACKSSWBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPGTBRR` (`PCMPGTB`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTB](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn mmx_pcmpgtbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPGTBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPGTBRM` (`PCMPGTB`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTB](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn mmx_pcmpgtbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPGTBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPGTWRR` (`PCMPGTW`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTW](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn mmx_pcmpgtwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPGTWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPGTWRM` (`PCMPGTW`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTW](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn mmx_pcmpgtwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPGTWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPGTDRR` (`PCMPGTD`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTD](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn mmx_pcmpgtdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPGTDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPGTDRM` (`PCMPGTD`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTD](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn mmx_pcmpgtdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPGTDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PACKUSWBRR` (`PACKUSWB`). Converts 4, 8, 16, or 32 signed word integers from the destination operand (first operand) and 4, 8, 16, or 32 signed word integers from the source operand (second operand) into 8, 16, 32 or 64 unsigned byte integers and stores the result in the destination operand. (See Figure 4-6 for an example of the packing operation.) If a signed word integer value is beyond the range of an unsigned byte integer (that is, greater than FFH or less than 00H), the saturated unsigned byte integer value of FFH or 00H, respectively, is stored in the destination.
    /// Reference: [Intel x86 docs for PACKUSWB](https://www.felixcloutier.com/x86/PACKUSWB.html)
    fn mmx_packuswbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PACKUSWBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PACKUSWBRM` (`PACKUSWB`). Converts 4, 8, 16, or 32 signed word integers from the destination operand (first operand) and 4, 8, 16, or 32 signed word integers from the source operand (second operand) into 8, 16, 32 or 64 unsigned byte integers and stores the result in the destination operand. (See Figure 4-6 for an example of the packing operation.) If a signed word integer value is beyond the range of an unsigned byte integer (that is, greater than FFH or less than 00H), the saturated unsigned byte integer value of FFH or 00H, respectively, is stored in the destination.
    /// Reference: [Intel x86 docs for PACKUSWB](https://www.felixcloutier.com/x86/PACKUSWB.html)
    fn mmx_packuswbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PACKUSWBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PUNPCKHBWRR` (`PUNPCKHBW`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHBW](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn mmx_punpckhbwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKHBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PUNPCKHBWRM` (`PUNPCKHBW`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHBW](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn mmx_punpckhbwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKHBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PUNPCKHWDRR` (`PUNPCKHWD`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHWD](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn mmx_punpckhwdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKHWDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PUNPCKHWDRM` (`PUNPCKHWD`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHWD](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn mmx_punpckhwdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKHWDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PUNPCKHDQRR` (`PUNPCKHDQ`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHDQ](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn mmx_punpckhdqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKHDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PUNPCKHDQRM` (`PUNPCKHDQ`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHDQ](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn mmx_punpckhdqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PUNPCKHDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PACKSSDWRR` (`PACKSSDW`). Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
    /// Reference: [Intel x86 docs for PACKSSDW](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html)
    fn mmx_packssdwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PACKSSDWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PACKSSDWRM` (`PACKSSDW`). Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
    /// Reference: [Intel x86 docs for PACKSSDW](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html)
    fn mmx_packssdwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PACKSSDWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVD_G2MRR`.
    fn mmx_movd_g2mrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVD_G2MRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVD_G2MRM`.
    fn mmx_movd_g2mrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVD_G2MRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVQ_G2MRR`.
    fn mmx_movq_g2mrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVQ_G2MRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVQ_G2MRM`.
    fn mmx_movq_g2mrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVQ_G2MRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVQRR` (`MOVQ`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVQ](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn mmx_movqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVQRM` (`MOVQ`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVQ](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn mmx_movqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRLWRI` (`PSRLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLW](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn mmx_psrlwri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRLWRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRAWRI` (`PSRAW`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAW](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn mmx_psrawri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRAWRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSLLWRI` (`PSLLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLW](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn mmx_psllwri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSLLWRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRLDRI` (`PSRLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLD](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn mmx_psrldri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRLDRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRADRI` (`PSRAD`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAD](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn mmx_psradri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRADRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSLLDRI` (`PSLLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLD](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn mmx_pslldri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSLLDRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRLQRI` (`PSRLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLQ](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn mmx_psrlqri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRLQRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSLLQRI` (`PSLLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLQ](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn mmx_psllqri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSLLQRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPEQBRR` (`PCMPEQB`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQB](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn mmx_pcmpeqbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPEQBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPEQBRM` (`PCMPEQB`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQB](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn mmx_pcmpeqbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPEQBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPEQWRR` (`PCMPEQW`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQW](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn mmx_pcmpeqwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPEQWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPEQWRM` (`PCMPEQW`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQW](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn mmx_pcmpeqwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPEQWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPEQDRR` (`PCMPEQD`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQD](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn mmx_pcmpeqdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPEQDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PCMPEQDRM` (`PCMPEQD`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQD](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn mmx_pcmpeqdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PCMPEQDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_EMMS` (`EMMS`). Sets the values of all the tags in the x87 FPU tag word to empty (all 1s). This operation marks the x87 FPU data registers (which are aliased to the MMX technology registers) as available for use by x87 FPU floating-point instructions. (See Figure 8-7 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for the format of the x87 FPU tag word.) All other MMX instructions (other than the EMMS instruction) set all the tags in x87 FPU tag word to valid (all 0s).
    /// Reference: [Intel x86 docs for EMMS](https://www.felixcloutier.com/x86/EMMS.html)
    fn mmx_emms(&mut self,) -> () {
        self.emit(MMX_EMMS, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVD_M2GRR`.
    fn mmx_movd_m2grr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVD_M2GRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVD_M2GMR`.
    fn mmx_movd_m2gmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVD_M2GMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVQ_M2GRR`.
    fn mmx_movq_m2grr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVQ_M2GRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVQ_M2GMR`.
    fn mmx_movq_m2gmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVQ_M2GMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_MOVQMR` (`MOVQ`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVQ](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn mmx_movqmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_MOVQMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRLWRR` (`PSRLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLW](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn mmx_psrlwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRLWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRLWRM` (`PSRLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLW](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn mmx_psrlwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRLWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRLDRR` (`PSRLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLD](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn mmx_psrldrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRLDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRLDRM` (`PSRLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLD](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn mmx_psrldrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRLDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRLQRR` (`PSRLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLQ](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn mmx_psrlqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRLQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRLQRM` (`PSRLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLQ](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn mmx_psrlqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRLQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDQRR` (`PADDQ`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDQ](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn mmx_paddqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDQRM` (`PADDQ`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDQ](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn mmx_paddqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMULLWRR` (`PMULLW`). Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the low 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULLW](https://www.felixcloutier.com/x86/PMULLW.html)
    fn mmx_pmullwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMULLWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMULLWRM` (`PMULLW`). Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the low 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULLW](https://www.felixcloutier.com/x86/PMULLW.html)
    fn mmx_pmullwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMULLWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBUSBRR` (`PSUBUSB`). Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBUSB](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html)
    fn mmx_psubusbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBUSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBUSBRM` (`PSUBUSB`). Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBUSB](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html)
    fn mmx_psubusbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBUSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBUSWRR` (`PSUBUSW`). Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBUSW](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html)
    fn mmx_psubuswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBUSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBUSWRM` (`PSUBUSW`). Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBUSW](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html)
    fn mmx_psubuswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBUSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PANDRR` (`PAND`). Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for PAND](https://www.felixcloutier.com/x86/PAND.html)
    fn mmx_pandrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PANDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PANDRM` (`PAND`). Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for PAND](https://www.felixcloutier.com/x86/PAND.html)
    fn mmx_pandrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PANDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDUSBRR` (`PADDUSB`). Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDUSB](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html)
    fn mmx_paddusbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDUSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDUSBRM` (`PADDUSB`). Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDUSB](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html)
    fn mmx_paddusbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDUSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDUSWRR` (`PADDUSW`). Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDUSW](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html)
    fn mmx_padduswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDUSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDUSWRM` (`PADDUSW`). Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDUSW](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html)
    fn mmx_padduswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDUSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PANDNRR` (`PANDN`). Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for PANDN](https://www.felixcloutier.com/x86/PANDN.html)
    fn mmx_pandnrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PANDNRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PANDNRM` (`PANDN`). Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for PANDN](https://www.felixcloutier.com/x86/PANDN.html)
    fn mmx_pandnrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PANDNRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRAWRR` (`PSRAW`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAW](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn mmx_psrawrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRAWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRAWRM` (`PSRAW`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAW](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn mmx_psrawrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRAWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRADRR` (`PSRAD`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAD](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn mmx_psradrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRADRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSRADRM` (`PSRAD`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAD](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn mmx_psradrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSRADRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMULHWRR` (`PMULHW`). Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULHW](https://www.felixcloutier.com/x86/PMULHW.html)
    fn mmx_pmulhwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMULHWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMULHWRM` (`PMULHW`). Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULHW](https://www.felixcloutier.com/x86/PMULHW.html)
    fn mmx_pmulhwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMULHWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBSBRR` (`PSUBSB`). Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBSB](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html)
    fn mmx_psubsbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBSBRM` (`PSUBSB`). Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBSB](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html)
    fn mmx_psubsbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBSWRR` (`PSUBSW`). Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBSW](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html)
    fn mmx_psubswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBSWRM` (`PSUBSW`). Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBSW](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html)
    fn mmx_psubswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PORRR` (`POR`). Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
    /// Reference: [Intel x86 docs for POR](https://www.felixcloutier.com/x86/POR.html)
    fn mmx_porrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PORRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PORRM` (`POR`). Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
    /// Reference: [Intel x86 docs for POR](https://www.felixcloutier.com/x86/POR.html)
    fn mmx_porrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PORRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDSBRR` (`PADDSB`). Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDSB](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html)
    fn mmx_paddsbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDSBRM` (`PADDSB`). Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDSB](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html)
    fn mmx_paddsbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDSWRR` (`PADDSW`). Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDSW](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html)
    fn mmx_paddswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDSWRM` (`PADDSW`). Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDSW](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html)
    fn mmx_paddswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PXORRR` (`PXOR`). Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
    /// Reference: [Intel x86 docs for PXOR](https://www.felixcloutier.com/x86/PXOR.html)
    fn mmx_pxorrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PXORRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PXORRM` (`PXOR`). Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
    /// Reference: [Intel x86 docs for PXOR](https://www.felixcloutier.com/x86/PXOR.html)
    fn mmx_pxorrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PXORRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSLLWRR` (`PSLLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLW](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn mmx_psllwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSLLWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSLLWRM` (`PSLLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLW](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn mmx_psllwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSLLWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSLLDRR` (`PSLLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLD](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn mmx_pslldrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSLLDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSLLDRM` (`PSLLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLD](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn mmx_pslldrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSLLDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSLLQRR` (`PSLLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLQ](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn mmx_psllqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSLLQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSLLQRM` (`PSLLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLQ](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn mmx_psllqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSLLQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMULUDQRR` (`PMULUDQ`). Multiplies the first operand (destination operand) by the second operand (source operand) and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for PMULUDQ](https://www.felixcloutier.com/x86/PMULUDQ.html)
    fn mmx_pmuludqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMULUDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMULUDQRM` (`PMULUDQ`). Multiplies the first operand (destination operand) by the second operand (source operand) and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for PMULUDQ](https://www.felixcloutier.com/x86/PMULUDQ.html)
    fn mmx_pmuludqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMULUDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMADDWDRR` (`PMADDWD`). Multiplies the individual signed words of the destination operand (first operand) by the corresponding signed words of the source operand (second operand), producing temporary signed, doubleword results. The adjacent double-word results are then summed and stored in the destination operand. For example, the corresponding low-order words (15-0) and (31-16) in the source and destination operands are multiplied by one another and the double-word results are added together and stored in the low doubleword of the destination register (31-0). The same operation is performed on the other pairs of adjacent words. (Figure 4-11 shows this operation when using 64-bit operands).
    /// Reference: [Intel x86 docs for PMADDWD](https://www.felixcloutier.com/x86/PMADDWD.html)
    fn mmx_pmaddwdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMADDWDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMADDWDRM` (`PMADDWD`). Multiplies the individual signed words of the destination operand (first operand) by the corresponding signed words of the source operand (second operand), producing temporary signed, doubleword results. The adjacent double-word results are then summed and stored in the destination operand. For example, the corresponding low-order words (15-0) and (31-16) in the source and destination operands are multiplied by one another and the double-word results are added together and stored in the low doubleword of the destination register (31-0). The same operation is performed on the other pairs of adjacent words. (Figure 4-11 shows this operation when using 64-bit operands).
    /// Reference: [Intel x86 docs for PMADDWD](https://www.felixcloutier.com/x86/PMADDWD.html)
    fn mmx_pmaddwdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMADDWDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBBRR` (`PSUBB`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBB](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn mmx_psubbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBBRM` (`PSUBB`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBB](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn mmx_psubbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBWRR` (`PSUBW`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBW](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn mmx_psubwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBWRM` (`PSUBW`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBW](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn mmx_psubwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBDRR` (`PSUBD`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBD](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn mmx_psubdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBDRM` (`PSUBD`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBD](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn mmx_psubdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBQRR` (`PSUBQ`). Subtracts the second operand (source operand) from the first operand (destination operand) and stores the result in the destination operand. When packed quadword operands are used, a SIMD subtract is performed. When a quadword result is too large to be represented in 64 bits (overflow), the result is wrapped around and the low 64 bits are written to the destination element (that is, the carry is ignored).
    /// Reference: [Intel x86 docs for PSUBQ](https://www.felixcloutier.com/x86/PSUBQ.html)
    fn mmx_psubqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSUBQRM` (`PSUBQ`). Subtracts the second operand (source operand) from the first operand (destination operand) and stores the result in the destination operand. When packed quadword operands are used, a SIMD subtract is performed. When a quadword result is too large to be represented in 64 bits (overflow), the result is wrapped around and the low 64 bits are written to the destination element (that is, the carry is ignored).
    /// Reference: [Intel x86 docs for PSUBQ](https://www.felixcloutier.com/x86/PSUBQ.html)
    fn mmx_psubqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSUBQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDBRR` (`PADDB`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDB](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn mmx_paddbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDBRM` (`PADDB`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDB](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn mmx_paddbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDWRR` (`PADDW`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDW](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn mmx_paddwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDWRM` (`PADDW`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDW](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn mmx_paddwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDDRR` (`PADDD`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDD](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn mmx_padddrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PADDDRM` (`PADDD`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDD](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn mmx_padddrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PADDDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
