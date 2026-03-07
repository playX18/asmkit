pub trait X86AVX512BF16Emitter: Emitter {
    /// Emits `VCVTNEPS2BF16_128RR` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_128rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_128RM` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_128rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_256RR` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_256rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_256RM` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_256rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_128RRR` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_128RRR_MASK` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_128RRR_MASKZ` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_128RRM` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_128RRM_MASK` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_128RRM_MASKZ` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_128RRB` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_128RRB_MASK` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_128RRB_MASKZ` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_256RRR` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_256RRR_MASK` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_256RRR_MASKZ` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_256RRM` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_256RRM_MASK` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_256RRM_MASKZ` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_256RRB` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_256RRB_MASK` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_256RRB_MASKZ` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_512RRR` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_512RRR_MASK` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_512RRR_MASKZ` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_512RRM` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_512RRM_MASK` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_512RRM_MASKZ` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_512RRB` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_512RRB_MASK` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNE2PS2BF16_512RRB_MASKZ` (`VCVTNE2PS2BF16`). Converts two SIMD registers of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNE2PS2BF16](https://www.felixcloutier.com/x86/VCVTNE2PS2BF16.html)
    fn vcvtne2ps2bf16_512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_128RR_MASK` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_128rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_128RR_MASKZ` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_128rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_128RM_MASK` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_128rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_128RM_MASKZ` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_128rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_128RB` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_128rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_128RB_MASK` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_128rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_128RB_MASKZ` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_128rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_256RR_MASK` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_256rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_256RR_MASKZ` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_256rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_256RM_MASK` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_256rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_256RM_MASKZ` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_256rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_256RB` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_256rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_256RB_MASK` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_256rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_256RB_MASKZ` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_256rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_512RR` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_512rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_512RR_MASK` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_512rr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_512RR_MASKZ` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_512rr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_512RM` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_512rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_512RM_MASK` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_512rm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RM_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_512RM_MASKZ` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_512rm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RM_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_512RB` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_512rb(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RB, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_512RB_MASK` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_512rb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VCVTNEPS2BF16_512RB_MASKZ` (`VCVTNEPS2BF16`). Converts one SIMD register of packed single data into a single register of packed BF16 data.
    /// Reference: [Intel x86 docs for VCVTNEPS2BF16](https://www.felixcloutier.com/x86/VCVTNEPS2BF16.html)
    fn vcvtneps2bf16_512rb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS128RRR` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps128rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS128RRR_MASK` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps128rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS128RRR_MASKZ` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps128rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS128RRM` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps128rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS128RRM_MASK` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps128rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS128RRM_MASKZ` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps128rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS128RRB` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps128rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS128RRB_MASK` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps128rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS128RRB_MASKZ` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps128rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS256RRR` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps256rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS256RRR_MASK` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps256rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS256RRR_MASKZ` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps256rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS256RRM` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps256rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS256RRM_MASK` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps256rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS256RRM_MASKZ` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps256rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS256RRB` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps256rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS256RRB_MASK` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps256rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS256RRB_MASKZ` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps256rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS512RRR` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps512rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS512RRR_MASK` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps512rrr_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRR_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS512RRR_MASKZ` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps512rrr_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRR_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS512RRM` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps512rrm(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRM, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS512RRM_MASK` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps512rrm_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRM_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS512RRM_MASKZ` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps512rrm_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRM_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS512RRB` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps512rrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS512RRB_MASK` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps512rrb_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `VDPBF16PS512RRB_MASKZ` (`VDPBF16PS`). This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register.
    /// Reference: [Intel x86 docs for VDPBF16PS](https://www.felixcloutier.com/x86/VDPBF16PS.html)
    fn vdpbf16ps512rrb_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
