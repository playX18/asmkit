pub trait X86SSSE3Emitter: Emitter {
    /// Emits `MMX_PSHUFBRR` (`PSHUFB`). PSHUFB performs in-place shuffles of bytes in the destination operand (the first operand) according to the shuffle control mask in the source operand (the second operand). The instruction permutes the data in the destination operand, leaving the shuffle mask unaffected. If the most significant bit (bit[7]) of each byte of the shuffle control mask is set, then constant zero is written in the result byte. Each byte in the shuffle control mask forms an index to permute the corresponding byte in the destination operand. The value of each index is the least significant 4 bits (128-bit operation) or 3 bits (64-bit operation) of the shuffle control byte. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PSHUFB](https://www.felixcloutier.com/x86/PSHUFB.html)
    fn mmx_pshufbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSHUFBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSHUFBRM` (`PSHUFB`). PSHUFB performs in-place shuffles of bytes in the destination operand (the first operand) according to the shuffle control mask in the source operand (the second operand). The instruction permutes the data in the destination operand, leaving the shuffle mask unaffected. If the most significant bit (bit[7]) of each byte of the shuffle control mask is set, then constant zero is written in the result byte. Each byte in the shuffle control mask forms an index to permute the corresponding byte in the destination operand. The value of each index is the least significant 4 bits (128-bit operation) or 3 bits (64-bit operation) of the shuffle control byte. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PSHUFB](https://www.felixcloutier.com/x86/PSHUFB.html)
    fn mmx_pshufbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSHUFBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHADDWRR` (`PHADDW`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDW](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn mmx_phaddwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHADDWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHADDWRM` (`PHADDW`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDW](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn mmx_phaddwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHADDWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHADDDRR` (`PHADDD`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDD](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn mmx_phadddrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHADDDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHADDDRM` (`PHADDD`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDD](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn mmx_phadddrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHADDDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHADDSWRR` (`PHADDSW`). (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDSW](https://www.felixcloutier.com/x86/PHADDSW.html)
    fn mmx_phaddswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHADDSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHADDSWRM` (`PHADDSW`). (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDSW](https://www.felixcloutier.com/x86/PHADDSW.html)
    fn mmx_phaddswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHADDSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMADDUBSWRR` (`PMADDUBSW`). (V)PMADDUBSW multiplies vertically each unsigned byte of the destination operand (first operand) with the corresponding signed byte of the source operand (second operand), producing intermediate signed 16-bit integers. Each adjacent pair of signed words is added and the saturated result is packed to the destination operand. For example, the lowest-order bytes (bits 7-0) in the source and destination operands are multiplied and the intermediate signed word result is added with the corresponding intermediate result from the 2nd lowest-order bytes (bits 15-8) of the operands; the sign-saturated result is stored in the lowest word of the destination register (15-0). The same operation is performed on the other pairs of adjacent bytes. Both operands can be MMX register or XMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PMADDUBSW](https://www.felixcloutier.com/x86/PMADDUBSW.html)
    fn mmx_pmaddubswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMADDUBSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMADDUBSWRM` (`PMADDUBSW`). (V)PMADDUBSW multiplies vertically each unsigned byte of the destination operand (first operand) with the corresponding signed byte of the source operand (second operand), producing intermediate signed 16-bit integers. Each adjacent pair of signed words is added and the saturated result is packed to the destination operand. For example, the lowest-order bytes (bits 7-0) in the source and destination operands are multiplied and the intermediate signed word result is added with the corresponding intermediate result from the 2nd lowest-order bytes (bits 15-8) of the operands; the sign-saturated result is stored in the lowest word of the destination register (15-0). The same operation is performed on the other pairs of adjacent bytes. Both operands can be MMX register or XMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PMADDUBSW](https://www.felixcloutier.com/x86/PMADDUBSW.html)
    fn mmx_pmaddubswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMADDUBSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHSUBWRR` (`PHSUBW`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBW](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn mmx_phsubwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHSUBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHSUBWRM` (`PHSUBW`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBW](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn mmx_phsubwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHSUBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHSUBDRR` (`PHSUBD`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBD](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn mmx_phsubdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHSUBDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHSUBDRM` (`PHSUBD`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBD](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn mmx_phsubdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHSUBDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHSUBSWRR` (`PHSUBSW`). (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBSW](https://www.felixcloutier.com/x86/PHSUBSW.html)
    fn mmx_phsubswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHSUBSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PHSUBSWRM` (`PHSUBSW`). (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBSW](https://www.felixcloutier.com/x86/PHSUBSW.html)
    fn mmx_phsubswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PHSUBSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSIGNBRR` (`PSIGNB`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGNB](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn mmx_psignbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSIGNBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSIGNBRM` (`PSIGNB`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGNB](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn mmx_psignbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSIGNBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSIGNWRR` (`PSIGNW`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGNW](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn mmx_psignwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSIGNWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSIGNWRM` (`PSIGNW`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGNW](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn mmx_psignwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSIGNWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSIGNDRR` (`PSIGND`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGND](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn mmx_psigndrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSIGNDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PSIGNDRM` (`PSIGND`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGND](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn mmx_psigndrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PSIGNDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMULHRSWRR` (`PMULHRSW`). PMULHRSW multiplies vertically each signed 16-bit integer from the destination operand (first operand) with the corresponding signed 16-bit integer of the source operand (second operand), producing intermediate, signed 32-bit integers. Each intermediate 32-bit integer is truncated to the 18 most significant bits. Rounding is always performed by adding 1 to the least significant bit of the 18-bit intermediate result. The final result is obtained by selecting the 16 bits immediately to the right of the most significant bit of each 18-bit intermediate result and packed to the destination operand.
    /// Reference: [Intel x86 docs for PMULHRSW](https://www.felixcloutier.com/x86/PMULHRSW.html)
    fn mmx_pmulhrswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMULHRSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PMULHRSWRM` (`PMULHRSW`). PMULHRSW multiplies vertically each signed 16-bit integer from the destination operand (first operand) with the corresponding signed 16-bit integer of the source operand (second operand), producing intermediate, signed 32-bit integers. Each intermediate 32-bit integer is truncated to the 18 most significant bits. Rounding is always performed by adding 1 to the least significant bit of the 18-bit intermediate result. The final result is obtained by selecting the 16 bits immediately to the right of the most significant bit of each 18-bit intermediate result and packed to the destination operand.
    /// Reference: [Intel x86 docs for PMULHRSW](https://www.felixcloutier.com/x86/PMULHRSW.html)
    fn mmx_pmulhrswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PMULHRSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PABSBRR` (`PABSB`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSB](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn mmx_pabsbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PABSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PABSBRM` (`PABSB`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSB](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn mmx_pabsbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PABSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PABSWRR` (`PABSW`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSW](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn mmx_pabswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PABSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PABSWRM` (`PABSW`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSW](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn mmx_pabswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PABSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PABSDRR` (`PABSD`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSD](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn mmx_pabsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PABSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PABSDRM` (`PABSD`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSD](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn mmx_pabsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_PABSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_PALIGNRRRI` (`PALIGNR`). (V)PALIGNR concatenates the destination operand (the first operand) and the source operand (the second operand) into an intermediate composite, shifts the composite at byte granularity to the right by a constant immediate, and extracts the right-aligned result into the destination. The first and the second operands can be an MMX, XMM or a YMM register. The immediate value is considered unsigned. Immediate shift counts larger than the 2L (i.e., 32 for 128-bit operands, or 16 for 64-bit operands) produce a zero result. Both operands can be MMX registers, XMM registers or YMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PALIGNR](https://www.felixcloutier.com/x86/PALIGNR.html)
    fn mmx_palignrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(MMX_PALIGNRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `MMX_PALIGNRRMI` (`PALIGNR`). (V)PALIGNR concatenates the destination operand (the first operand) and the source operand (the second operand) into an intermediate composite, shifts the composite at byte granularity to the right by a constant immediate, and extracts the right-aligned result into the destination. The first and the second operands can be an MMX, XMM or a YMM register. The immediate value is considered unsigned. Immediate shift counts larger than the 2L (i.e., 32 for 128-bit operands, or 16 for 64-bit operands) produce a zero result. Both operands can be MMX registers, XMM registers or YMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PALIGNR](https://www.felixcloutier.com/x86/PALIGNR.html)
    fn mmx_palignrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(MMX_PALIGNRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PSHUFBRR` (`PSHUFB`). PSHUFB performs in-place shuffles of bytes in the destination operand (the first operand) according to the shuffle control mask in the source operand (the second operand). The instruction permutes the data in the destination operand, leaving the shuffle mask unaffected. If the most significant bit (bit[7]) of each byte of the shuffle control mask is set, then constant zero is written in the result byte. Each byte in the shuffle control mask forms an index to permute the corresponding byte in the destination operand. The value of each index is the least significant 4 bits (128-bit operation) or 3 bits (64-bit operation) of the shuffle control byte. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PSHUFB](https://www.felixcloutier.com/x86/PSHUFB.html)
    fn sse_pshufbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSHUFBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSHUFBRM` (`PSHUFB`). PSHUFB performs in-place shuffles of bytes in the destination operand (the first operand) according to the shuffle control mask in the source operand (the second operand). The instruction permutes the data in the destination operand, leaving the shuffle mask unaffected. If the most significant bit (bit[7]) of each byte of the shuffle control mask is set, then constant zero is written in the result byte. Each byte in the shuffle control mask forms an index to permute the corresponding byte in the destination operand. The value of each index is the least significant 4 bits (128-bit operation) or 3 bits (64-bit operation) of the shuffle control byte. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PSHUFB](https://www.felixcloutier.com/x86/PSHUFB.html)
    fn sse_pshufbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSHUFBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHADDWRR` (`PHADDW`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDW](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn sse_phaddwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHADDWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHADDWRM` (`PHADDW`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDW](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn sse_phaddwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHADDWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHADDDRR` (`PHADDD`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDD](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn sse_phadddrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHADDDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHADDDRM` (`PHADDD`). (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDD](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html)
    fn sse_phadddrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHADDDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHADDSWRR` (`PHADDSW`). (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDSW](https://www.felixcloutier.com/x86/PHADDSW.html)
    fn sse_phaddswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHADDSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHADDSWRM` (`PHADDSW`). (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHADDSW](https://www.felixcloutier.com/x86/PHADDSW.html)
    fn sse_phaddswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHADDSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMADDUBSWRR` (`PMADDUBSW`). (V)PMADDUBSW multiplies vertically each unsigned byte of the destination operand (first operand) with the corresponding signed byte of the source operand (second operand), producing intermediate signed 16-bit integers. Each adjacent pair of signed words is added and the saturated result is packed to the destination operand. For example, the lowest-order bytes (bits 7-0) in the source and destination operands are multiplied and the intermediate signed word result is added with the corresponding intermediate result from the 2nd lowest-order bytes (bits 15-8) of the operands; the sign-saturated result is stored in the lowest word of the destination register (15-0). The same operation is performed on the other pairs of adjacent bytes. Both operands can be MMX register or XMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PMADDUBSW](https://www.felixcloutier.com/x86/PMADDUBSW.html)
    fn sse_pmaddubswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMADDUBSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMADDUBSWRM` (`PMADDUBSW`). (V)PMADDUBSW multiplies vertically each unsigned byte of the destination operand (first operand) with the corresponding signed byte of the source operand (second operand), producing intermediate signed 16-bit integers. Each adjacent pair of signed words is added and the saturated result is packed to the destination operand. For example, the lowest-order bytes (bits 7-0) in the source and destination operands are multiplied and the intermediate signed word result is added with the corresponding intermediate result from the 2nd lowest-order bytes (bits 15-8) of the operands; the sign-saturated result is stored in the lowest word of the destination register (15-0). The same operation is performed on the other pairs of adjacent bytes. Both operands can be MMX register or XMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PMADDUBSW](https://www.felixcloutier.com/x86/PMADDUBSW.html)
    fn sse_pmaddubswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMADDUBSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHSUBWRR` (`PHSUBW`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBW](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn sse_phsubwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHSUBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHSUBWRM` (`PHSUBW`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBW](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn sse_phsubwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHSUBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHSUBDRR` (`PHSUBD`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBD](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn sse_phsubdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHSUBDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHSUBDRM` (`PHSUBD`). (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBD](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html)
    fn sse_phsubdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHSUBDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHSUBSWRR` (`PHSUBSW`). (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBSW](https://www.felixcloutier.com/x86/PHSUBSW.html)
    fn sse_phsubswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHSUBSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PHSUBSWRM` (`PHSUBSW`). (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PHSUBSW](https://www.felixcloutier.com/x86/PHSUBSW.html)
    fn sse_phsubswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PHSUBSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSIGNBRR` (`PSIGNB`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGNB](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn sse_psignbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSIGNBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSIGNBRM` (`PSIGNB`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGNB](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn sse_psignbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSIGNBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSIGNWRR` (`PSIGNW`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGNW](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn sse_psignwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSIGNWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSIGNWRM` (`PSIGNW`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGNW](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn sse_psignwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSIGNWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSIGNDRR` (`PSIGND`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGND](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn sse_psigndrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSIGNDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSIGNDRM` (`PSIGND`). (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    /// Reference: [Intel x86 docs for PSIGND](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html)
    fn sse_psigndrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSIGNDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULHRSWRR` (`PMULHRSW`). PMULHRSW multiplies vertically each signed 16-bit integer from the destination operand (first operand) with the corresponding signed 16-bit integer of the source operand (second operand), producing intermediate, signed 32-bit integers. Each intermediate 32-bit integer is truncated to the 18 most significant bits. Rounding is always performed by adding 1 to the least significant bit of the 18-bit intermediate result. The final result is obtained by selecting the 16 bits immediately to the right of the most significant bit of each 18-bit intermediate result and packed to the destination operand.
    /// Reference: [Intel x86 docs for PMULHRSW](https://www.felixcloutier.com/x86/PMULHRSW.html)
    fn sse_pmulhrswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULHRSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULHRSWRM` (`PMULHRSW`). PMULHRSW multiplies vertically each signed 16-bit integer from the destination operand (first operand) with the corresponding signed 16-bit integer of the source operand (second operand), producing intermediate, signed 32-bit integers. Each intermediate 32-bit integer is truncated to the 18 most significant bits. Rounding is always performed by adding 1 to the least significant bit of the 18-bit intermediate result. The final result is obtained by selecting the 16 bits immediately to the right of the most significant bit of each 18-bit intermediate result and packed to the destination operand.
    /// Reference: [Intel x86 docs for PMULHRSW](https://www.felixcloutier.com/x86/PMULHRSW.html)
    fn sse_pmulhrswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULHRSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PABSBRR` (`PABSB`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSB](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn sse_pabsbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PABSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PABSBRM` (`PABSB`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSB](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn sse_pabsbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PABSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PABSWRR` (`PABSW`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSW](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn sse_pabswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PABSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PABSWRM` (`PABSW`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSW](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn sse_pabswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PABSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PABSDRR` (`PABSD`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSD](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn sse_pabsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PABSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PABSDRM` (`PABSD`). PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    /// Reference: [Intel x86 docs for PABSD](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html)
    fn sse_pabsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PABSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PALIGNRRRI` (`PALIGNR`). (V)PALIGNR concatenates the destination operand (the first operand) and the source operand (the second operand) into an intermediate composite, shifts the composite at byte granularity to the right by a constant immediate, and extracts the right-aligned result into the destination. The first and the second operands can be an MMX, XMM or a YMM register. The immediate value is considered unsigned. Immediate shift counts larger than the 2L (i.e., 32 for 128-bit operands, or 16 for 64-bit operands) produce a zero result. Both operands can be MMX registers, XMM registers or YMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PALIGNR](https://www.felixcloutier.com/x86/PALIGNR.html)
    fn sse_palignrrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PALIGNRRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PALIGNRRMI` (`PALIGNR`). (V)PALIGNR concatenates the destination operand (the first operand) and the source operand (the second operand) into an intermediate composite, shifts the composite at byte granularity to the right by a constant immediate, and extracts the right-aligned result into the destination. The first and the second operands can be an MMX, XMM or a YMM register. The immediate value is considered unsigned. Immediate shift counts larger than the 2L (i.e., 32 for 128-bit operands, or 16 for 64-bit operands) produce a zero result. Both operands can be MMX registers, XMM registers or YMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for PALIGNR](https://www.felixcloutier.com/x86/PALIGNR.html)
    fn sse_palignrrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PALIGNRRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
