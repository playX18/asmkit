pub trait X86BMI1Emitter: Emitter {
    /// Emits `TZCNT16RR` (`TZCNT`). TZCNT counts the number of trailing least significant zero bits in source operand (second operand) and returns the result in destination operand (first operand). TZCNT is an extension of the BSF instruction. The key difference between TZCNT and BSF instruction is that TZCNT provides operand size as output when source operand is zero while in the case of BSF instruction, if source operand is zero, the content of destination operand are undefined. On processors that do not support TZCNT, the instruction byte encoding is executed as BSF.
    /// Reference: [Intel x86 docs for TZCNT](https://www.felixcloutier.com/x86/TZCNT.html)
    fn tzcnt16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(TZCNT16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `TZCNT16RM` (`TZCNT`). TZCNT counts the number of trailing least significant zero bits in source operand (second operand) and returns the result in destination operand (first operand). TZCNT is an extension of the BSF instruction. The key difference between TZCNT and BSF instruction is that TZCNT provides operand size as output when source operand is zero while in the case of BSF instruction, if source operand is zero, the content of destination operand are undefined. On processors that do not support TZCNT, the instruction byte encoding is executed as BSF.
    /// Reference: [Intel x86 docs for TZCNT](https://www.felixcloutier.com/x86/TZCNT.html)
    fn tzcnt16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(TZCNT16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `TZCNT32RR` (`TZCNT`). TZCNT counts the number of trailing least significant zero bits in source operand (second operand) and returns the result in destination operand (first operand). TZCNT is an extension of the BSF instruction. The key difference between TZCNT and BSF instruction is that TZCNT provides operand size as output when source operand is zero while in the case of BSF instruction, if source operand is zero, the content of destination operand are undefined. On processors that do not support TZCNT, the instruction byte encoding is executed as BSF.
    /// Reference: [Intel x86 docs for TZCNT](https://www.felixcloutier.com/x86/TZCNT.html)
    fn tzcnt32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(TZCNT32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `TZCNT32RM` (`TZCNT`). TZCNT counts the number of trailing least significant zero bits in source operand (second operand) and returns the result in destination operand (first operand). TZCNT is an extension of the BSF instruction. The key difference between TZCNT and BSF instruction is that TZCNT provides operand size as output when source operand is zero while in the case of BSF instruction, if source operand is zero, the content of destination operand are undefined. On processors that do not support TZCNT, the instruction byte encoding is executed as BSF.
    /// Reference: [Intel x86 docs for TZCNT](https://www.felixcloutier.com/x86/TZCNT.html)
    fn tzcnt32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(TZCNT32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `TZCNT64RR` (`TZCNT`). TZCNT counts the number of trailing least significant zero bits in source operand (second operand) and returns the result in destination operand (first operand). TZCNT is an extension of the BSF instruction. The key difference between TZCNT and BSF instruction is that TZCNT provides operand size as output when source operand is zero while in the case of BSF instruction, if source operand is zero, the content of destination operand are undefined. On processors that do not support TZCNT, the instruction byte encoding is executed as BSF.
    /// Reference: [Intel x86 docs for TZCNT](https://www.felixcloutier.com/x86/TZCNT.html)
    fn tzcnt64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(TZCNT64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `TZCNT64RM` (`TZCNT`). TZCNT counts the number of trailing least significant zero bits in source operand (second operand) and returns the result in destination operand (first operand). TZCNT is an extension of the BSF instruction. The key difference between TZCNT and BSF instruction is that TZCNT provides operand size as output when source operand is zero while in the case of BSF instruction, if source operand is zero, the content of destination operand are undefined. On processors that do not support TZCNT, the instruction byte encoding is executed as BSF.
    /// Reference: [Intel x86 docs for TZCNT](https://www.felixcloutier.com/x86/TZCNT.html)
    fn tzcnt64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(TZCNT64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ANDN32RRR` (`ANDN`). Performs a bitwise logical AND of inverted second operand (the first source operand) with the third operand (the
    /// Reference: [Intel x86 docs for ANDN](https://www.felixcloutier.com/x86/ANDN.html)
    fn andn32rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(ANDN32RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `ANDN32RRM` (`ANDN`). Performs a bitwise logical AND of inverted second operand (the first source operand) with the third operand (the
    /// Reference: [Intel x86 docs for ANDN](https://www.felixcloutier.com/x86/ANDN.html)
    fn andn32rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(ANDN32RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `ANDN64RRR` (`ANDN`). Performs a bitwise logical AND of inverted second operand (the first source operand) with the third operand (the
    /// Reference: [Intel x86 docs for ANDN](https://www.felixcloutier.com/x86/ANDN.html)
    fn andn64rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(ANDN64RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `ANDN64RRM` (`ANDN`). Performs a bitwise logical AND of inverted second operand (the first source operand) with the third operand (the
    /// Reference: [Intel x86 docs for ANDN](https://www.felixcloutier.com/x86/ANDN.html)
    fn andn64rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(ANDN64RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `BLSR32RR` (`BLSR`). Copies all bits from the source operand to the destination operand and resets (=0) the bit position in the destination operand that corresponds to the lowest set bit of the source operand. If the source operand is zero BLSR sets CF.
    /// Reference: [Intel x86 docs for BLSR](https://www.felixcloutier.com/x86/BLSR.html)
    fn blsr32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSR32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BLSR32RM` (`BLSR`). Copies all bits from the source operand to the destination operand and resets (=0) the bit position in the destination operand that corresponds to the lowest set bit of the source operand. If the source operand is zero BLSR sets CF.
    /// Reference: [Intel x86 docs for BLSR](https://www.felixcloutier.com/x86/BLSR.html)
    fn blsr32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSR32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BLSR64RR` (`BLSR`). Copies all bits from the source operand to the destination operand and resets (=0) the bit position in the destination operand that corresponds to the lowest set bit of the source operand. If the source operand is zero BLSR sets CF.
    /// Reference: [Intel x86 docs for BLSR](https://www.felixcloutier.com/x86/BLSR.html)
    fn blsr64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSR64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BLSR64RM` (`BLSR`). Copies all bits from the source operand to the destination operand and resets (=0) the bit position in the destination operand that corresponds to the lowest set bit of the source operand. If the source operand is zero BLSR sets CF.
    /// Reference: [Intel x86 docs for BLSR](https://www.felixcloutier.com/x86/BLSR.html)
    fn blsr64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSR64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BLSMSK32RR` (`BLSMSK`). Sets all the lower bits of the destination operand to “1” up to and including lowest set bit (=1) in the source operand. If source operand is zero, BLSMSK sets all bits of the destination operand to 1 and also sets CF to 1.
    /// Reference: [Intel x86 docs for BLSMSK](https://www.felixcloutier.com/x86/BLSMSK.html)
    fn blsmsk32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSMSK32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BLSMSK32RM` (`BLSMSK`). Sets all the lower bits of the destination operand to “1” up to and including lowest set bit (=1) in the source operand. If source operand is zero, BLSMSK sets all bits of the destination operand to 1 and also sets CF to 1.
    /// Reference: [Intel x86 docs for BLSMSK](https://www.felixcloutier.com/x86/BLSMSK.html)
    fn blsmsk32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSMSK32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BLSMSK64RR` (`BLSMSK`). Sets all the lower bits of the destination operand to “1” up to and including lowest set bit (=1) in the source operand. If source operand is zero, BLSMSK sets all bits of the destination operand to 1 and also sets CF to 1.
    /// Reference: [Intel x86 docs for BLSMSK](https://www.felixcloutier.com/x86/BLSMSK.html)
    fn blsmsk64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSMSK64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BLSMSK64RM` (`BLSMSK`). Sets all the lower bits of the destination operand to “1” up to and including lowest set bit (=1) in the source operand. If source operand is zero, BLSMSK sets all bits of the destination operand to 1 and also sets CF to 1.
    /// Reference: [Intel x86 docs for BLSMSK](https://www.felixcloutier.com/x86/BLSMSK.html)
    fn blsmsk64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSMSK64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BLSI32RR` (`BLSI`). Extracts the lowest set bit from the source operand and set the corresponding bit in the destination register. All other bits in the destination operand are zeroed. If no bits are set in the source operand, BLSI sets all the bits in the destination to 0 and sets ZF and CF.
    /// Reference: [Intel x86 docs for BLSI](https://www.felixcloutier.com/x86/BLSI.html)
    fn blsi32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSI32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BLSI32RM` (`BLSI`). Extracts the lowest set bit from the source operand and set the corresponding bit in the destination register. All other bits in the destination operand are zeroed. If no bits are set in the source operand, BLSI sets all the bits in the destination to 0 and sets ZF and CF.
    /// Reference: [Intel x86 docs for BLSI](https://www.felixcloutier.com/x86/BLSI.html)
    fn blsi32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSI32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BLSI64RR` (`BLSI`). Extracts the lowest set bit from the source operand and set the corresponding bit in the destination register. All other bits in the destination operand are zeroed. If no bits are set in the source operand, BLSI sets all the bits in the destination to 0 and sets ZF and CF.
    /// Reference: [Intel x86 docs for BLSI](https://www.felixcloutier.com/x86/BLSI.html)
    fn blsi64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSI64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BLSI64RM` (`BLSI`). Extracts the lowest set bit from the source operand and set the corresponding bit in the destination register. All other bits in the destination operand are zeroed. If no bits are set in the source operand, BLSI sets all the bits in the destination to 0 and sets ZF and CF.
    /// Reference: [Intel x86 docs for BLSI](https://www.felixcloutier.com/x86/BLSI.html)
    fn blsi64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(BLSI64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BEXTR32RRR` (`BEXTR`). Extracts contiguous bits from the first source operand (the second operand) using an index value and length value specified in the second source operand (the third operand). Bit 7:0 of the second source operand specifies the starting bit position of bit extraction. A START value exceeding the operand size will not extract any bits from the second source operand. Bit 15:8 of the second source operand specifies the maximum number of bits (LENGTH) beginning at the START position to extract. Only bit positions up to (OperandSize -1) of the first source operand are extracted. The extracted bits are written to the destination register, starting from the least significant bit. All higher order bits in the destination operand (starting at bit position LENGTH) are zeroed. The destination register is cleared if no bits are extracted.
    /// Reference: [Intel x86 docs for BEXTR](https://www.felixcloutier.com/x86/BEXTR.html)
    fn bextr32rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(BEXTR32RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `BEXTR32RMR` (`BEXTR`). Extracts contiguous bits from the first source operand (the second operand) using an index value and length value specified in the second source operand (the third operand). Bit 7:0 of the second source operand specifies the starting bit position of bit extraction. A START value exceeding the operand size will not extract any bits from the second source operand. Bit 15:8 of the second source operand specifies the maximum number of bits (LENGTH) beginning at the START position to extract. Only bit positions up to (OperandSize -1) of the first source operand are extracted. The extracted bits are written to the destination register, starting from the least significant bit. All higher order bits in the destination operand (starting at bit position LENGTH) are zeroed. The destination register is cleared if no bits are extracted.
    /// Reference: [Intel x86 docs for BEXTR](https://www.felixcloutier.com/x86/BEXTR.html)
    fn bextr32rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(BEXTR32RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `BEXTR64RRR` (`BEXTR`). Extracts contiguous bits from the first source operand (the second operand) using an index value and length value specified in the second source operand (the third operand). Bit 7:0 of the second source operand specifies the starting bit position of bit extraction. A START value exceeding the operand size will not extract any bits from the second source operand. Bit 15:8 of the second source operand specifies the maximum number of bits (LENGTH) beginning at the START position to extract. Only bit positions up to (OperandSize -1) of the first source operand are extracted. The extracted bits are written to the destination register, starting from the least significant bit. All higher order bits in the destination operand (starting at bit position LENGTH) are zeroed. The destination register is cleared if no bits are extracted.
    /// Reference: [Intel x86 docs for BEXTR](https://www.felixcloutier.com/x86/BEXTR.html)
    fn bextr64rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(BEXTR64RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `BEXTR64RMR` (`BEXTR`). Extracts contiguous bits from the first source operand (the second operand) using an index value and length value specified in the second source operand (the third operand). Bit 7:0 of the second source operand specifies the starting bit position of bit extraction. A START value exceeding the operand size will not extract any bits from the second source operand. Bit 15:8 of the second source operand specifies the maximum number of bits (LENGTH) beginning at the START position to extract. Only bit positions up to (OperandSize -1) of the first source operand are extracted. The extracted bits are written to the destination register, starting from the least significant bit. All higher order bits in the destination operand (starting at bit position LENGTH) are zeroed. The destination register is cleared if no bits are extracted.
    /// Reference: [Intel x86 docs for BEXTR](https://www.felixcloutier.com/x86/BEXTR.html)
    fn bextr64rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(BEXTR64RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
