pub trait X86AVX512VBMIEmitter: Emitter {
    /// Emits `VPERMB128RRR` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB128RRR_MASK` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB128RRR_MASKZ` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB128RRM` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB128RRM_MASK` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB128RRM_MASKZ` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB256RRR` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB256RRR_MASK` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB256RRR_MASKZ` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB256RRM` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB256RRM_MASK` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB256RRM_MASKZ` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB512RRR` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB512RRR_MASK` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB512RRR_MASKZ` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB512RRM` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB512RRM_MASK` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMB512RRM_MASKZ` (`VPERMB`). Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand). Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
    /// Reference: [Intel x86 docs for VPERMB](https://www.felixcloutier.com/x86/VPERMB.html)
    fn vpermb512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMB512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B128RRR` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B128RRR_MASK` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B128RRR_MASKZ` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B128RRM` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B128RRM_MASK` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B128RRM_MASKZ` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B256RRR` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B256RRR_MASK` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B256RRR_MASKZ` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B256RRM` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B256RRM_MASK` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B256RRM_MASKZ` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B512RRR` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B512RRR_MASK` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B512RRR_MASKZ` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B512RRM` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B512RRM_MASK` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMI2B512RRM_MASKZ` (`VPERMI2B`). Permutes byte values in the second operand (the first source operand) and the third operand (the second source operand) using the byte indices in the first operand (the destination operand) to select byte elements from the second or third source operands. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMI2B](https://www.felixcloutier.com/x86/VPERMI2B.html)
    fn vpermi2b512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMI2B512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B128RRR` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B128RRR_MASK` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B128RRR_MASKZ` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B128RRM` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B128RRM_MASK` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B128RRM_MASKZ` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B256RRR` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B256RRR_MASK` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B256RRR_MASKZ` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B256RRM` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B256RRM_MASK` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B256RRM_MASKZ` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B512RRR` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B512RRR_MASK` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B512RRR_MASKZ` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B512RRM` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B512RRM_MASK` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPERMT2B512RRM_MASKZ` (`VPERMT2B`). Permutes byte values from two tables, comprising of the first operand (also the destination operand) and the third operand (the second source operand). The second operand (the first source operand) provides byte indices to select byte results from the two tables. The selected byte elements are written to the destination at byte granularity under the writemask k1.
    /// Reference: [Intel x86 docs for VPERMT2B](https://www.felixcloutier.com/x86/VPERMT2B.html)
    fn vpermt2b512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPERMT2B512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB128RRR` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB128RRR_MASK` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB128RRR_MASKZ` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB128RRM` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB128RRM_MASK` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB128RRM_MASKZ` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB128RRB` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB128RRB_MASK` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB128RRB_MASKZ` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB256RRR` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB256RRR_MASK` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB256RRR_MASKZ` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB256RRM` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB256RRM_MASK` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB256RRM_MASKZ` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB256RRB` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB256RRB_MASK` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB256RRB_MASKZ` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB512RRR` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB512RRR_MASK` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB512RRR_MASKZ` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB512RRM` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB512RRM_MASK` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB512RRM_MASKZ` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB512RRB` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB512RRB_MASK` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMULTISHIFTQB512RRB_MASKZ` (`VPMULTISHIFTQB`). This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand). Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand). Each byte result in the destination operand is updated under the writemask k1.
    /// Reference: [Intel x86 docs for VPMULTISHIFTQB](https://www.felixcloutier.com/x86/VPMULTISHIFTQB.html)
    fn vpmultishiftqb512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
