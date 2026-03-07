pub trait X86AVX512IFMAEmitter: Emitter {
    /// Emits `VPMADD52LUQ128RRR` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ128RRM` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ256RRR` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ256RRM` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ128RRR` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ128RRM` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ256RRR` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ256RRM` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ128RRR_MASK` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ128RRR_MASKZ` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ128RRM_MASK` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ128RRM_MASKZ` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ128RRB` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ128RRB_MASK` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ128RRB_MASKZ` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ256RRR_MASK` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ256RRR_MASKZ` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ256RRM_MASK` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ256RRM_MASKZ` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ256RRB` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ256RRB_MASK` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ256RRB_MASKZ` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ512RRR` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ512RRR_MASK` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ512RRR_MASKZ` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ512RRM` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ512RRM_MASK` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ512RRM_MASKZ` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ512RRB` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ512RRB_MASK` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52LUQ512RRB_MASKZ` (`VPMADD52LUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The low 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52LUQ](https://www.felixcloutier.com/x86/VPMADD52LUQ.html)
    fn vpmadd52luq512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52LUQ512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ128RRR_MASK` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ128RRR_MASKZ` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ128RRM_MASK` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ128RRM_MASKZ` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ128RRB` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ128RRB_MASK` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ128RRB_MASKZ` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ256RRR_MASK` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ256RRR_MASKZ` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ256RRM_MASK` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ256RRM_MASKZ` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ256RRB` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ256RRB_MASK` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ256RRB_MASKZ` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ512RRR` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ512RRR_MASK` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ512RRR_MASKZ` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ512RRM` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ512RRM_MASK` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ512RRM_MASKZ` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ512RRB` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ512RRB_MASK` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VPMADD52HUQ512RRB_MASKZ` (`VPMADD52HUQ`). Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second operand) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results. The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.
    /// Reference: [Intel x86 docs for VPMADD52HUQ](https://www.felixcloutier.com/x86/VPMADD52HUQ.html)
    fn vpmadd52huq512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMADD52HUQ512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
