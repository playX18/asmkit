pub trait X86AVX512BF16Emitter: Emitter {
    /// Emits `VCVTNE2PS2BF16_128`.
    fn vcvtne2ps2bf16_128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTNE2PS2BF16_128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTNE2PS2BF16_128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNE2PS2BF16_128");
        }
    }
    /// Emits `VCVTNE2PS2BF16_128_MASK`.
    fn vcvtne2ps2bf16_128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTNE2PS2BF16_128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTNE2PS2BF16_128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNE2PS2BF16_128_MASK");
        }
    }
    /// Emits `VCVTNE2PS2BF16_128_MASKZ`.
    fn vcvtne2ps2bf16_128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTNE2PS2BF16_128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTNE2PS2BF16_128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNE2PS2BF16_128_MASKZ");
        }
    }
    /// Emits `VCVTNE2PS2BF16_128RRB`.
    fn vcvtne2ps2bf16_128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTNE2PS2BF16_128RRB_MASK`.
    fn vcvtne2ps2bf16_128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTNE2PS2BF16_128RRB_MASKZ`.
    fn vcvtne2ps2bf16_128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTNE2PS2BF16_256`.
    fn vcvtne2ps2bf16_256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTNE2PS2BF16_256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTNE2PS2BF16_256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNE2PS2BF16_256");
        }
    }
    /// Emits `VCVTNE2PS2BF16_256_MASK`.
    fn vcvtne2ps2bf16_256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTNE2PS2BF16_256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTNE2PS2BF16_256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNE2PS2BF16_256_MASK");
        }
    }
    /// Emits `VCVTNE2PS2BF16_256_MASKZ`.
    fn vcvtne2ps2bf16_256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTNE2PS2BF16_256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTNE2PS2BF16_256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNE2PS2BF16_256_MASKZ");
        }
    }
    /// Emits `VCVTNE2PS2BF16_256RRB`.
    fn vcvtne2ps2bf16_256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTNE2PS2BF16_256RRB_MASK`.
    fn vcvtne2ps2bf16_256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTNE2PS2BF16_256RRB_MASKZ`.
    fn vcvtne2ps2bf16_256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTNE2PS2BF16_512`.
    fn vcvtne2ps2bf16_512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTNE2PS2BF16_512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTNE2PS2BF16_512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNE2PS2BF16_512");
        }
    }
    /// Emits `VCVTNE2PS2BF16_512_MASK`.
    fn vcvtne2ps2bf16_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTNE2PS2BF16_512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTNE2PS2BF16_512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNE2PS2BF16_512_MASK");
        }
    }
    /// Emits `VCVTNE2PS2BF16_512_MASKZ`.
    fn vcvtne2ps2bf16_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VCVTNE2PS2BF16_512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VCVTNE2PS2BF16_512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNE2PS2BF16_512_MASKZ");
        }
    }
    /// Emits `VCVTNE2PS2BF16_512RRB`.
    fn vcvtne2ps2bf16_512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTNE2PS2BF16_512RRB_MASK`.
    fn vcvtne2ps2bf16_512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTNE2PS2BF16_512RRB_MASKZ`.
    fn vcvtne2ps2bf16_512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VCVTNE2PS2BF16_512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VCVTNEPS2BF16_128`.
    fn vcvtneps2bf16_128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTNEPS2BF16_128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTNEPS2BF16_128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNEPS2BF16_128");
        }
    }
    /// Emits `VCVTNEPS2BF16_128_MASK`.
    fn vcvtneps2bf16_128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTNEPS2BF16_128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTNEPS2BF16_128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNEPS2BF16_128_MASK");
        }
    }
    /// Emits `VCVTNEPS2BF16_128_MASKZ`.
    fn vcvtneps2bf16_128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTNEPS2BF16_128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTNEPS2BF16_128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNEPS2BF16_128_MASKZ");
        }
    }
    /// Emits `VCVTNEPS2BF16_128RB`.
    fn vcvtneps2bf16_128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEPS2BF16_128RB_MASK`.
    fn vcvtneps2bf16_128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEPS2BF16_128RB_MASKZ`.
    fn vcvtneps2bf16_128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEPS2BF16_256`.
    fn vcvtneps2bf16_256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTNEPS2BF16_256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTNEPS2BF16_256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNEPS2BF16_256");
        }
    }
    /// Emits `VCVTNEPS2BF16_256_MASK`.
    fn vcvtneps2bf16_256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTNEPS2BF16_256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTNEPS2BF16_256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNEPS2BF16_256_MASK");
        }
    }
    /// Emits `VCVTNEPS2BF16_256_MASKZ`.
    fn vcvtneps2bf16_256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTNEPS2BF16_256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTNEPS2BF16_256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNEPS2BF16_256_MASKZ");
        }
    }
    /// Emits `VCVTNEPS2BF16_256RB`.
    fn vcvtneps2bf16_256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEPS2BF16_256RB_MASK`.
    fn vcvtneps2bf16_256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEPS2BF16_256RB_MASKZ`.
    fn vcvtneps2bf16_256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEPS2BF16_512`.
    fn vcvtneps2bf16_512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTNEPS2BF16_512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTNEPS2BF16_512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNEPS2BF16_512");
        }
    }
    /// Emits `VCVTNEPS2BF16_512_MASK`.
    fn vcvtneps2bf16_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTNEPS2BF16_512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTNEPS2BF16_512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNEPS2BF16_512_MASK");
        }
    }
    /// Emits `VCVTNEPS2BF16_512_MASKZ`.
    fn vcvtneps2bf16_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VCVTNEPS2BF16_512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VCVTNEPS2BF16_512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VCVTNEPS2BF16_512_MASKZ");
        }
    }
    /// Emits `VCVTNEPS2BF16_512RB`.
    fn vcvtneps2bf16_512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEPS2BF16_512RB_MASK`.
    fn vcvtneps2bf16_512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VCVTNEPS2BF16_512RB_MASKZ`.
    fn vcvtneps2bf16_512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VCVTNEPS2BF16_512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VDPBF16PS128`.
    fn vdpbf16ps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDPBF16PS128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDPBF16PS128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDPBF16PS128");
        }
    }
    /// Emits `VDPBF16PS128_MASK`.
    fn vdpbf16ps128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDPBF16PS128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDPBF16PS128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDPBF16PS128_MASK");
        }
    }
    /// Emits `VDPBF16PS128_MASKZ`.
    fn vdpbf16ps128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDPBF16PS128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDPBF16PS128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDPBF16PS128_MASKZ");
        }
    }
    /// Emits `VDPBF16PS128RRB`.
    fn vdpbf16ps128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDPBF16PS128RRB_MASK`.
    fn vdpbf16ps128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDPBF16PS128RRB_MASKZ`.
    fn vdpbf16ps128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDPBF16PS256`.
    fn vdpbf16ps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDPBF16PS256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDPBF16PS256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDPBF16PS256");
        }
    }
    /// Emits `VDPBF16PS256_MASK`.
    fn vdpbf16ps256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDPBF16PS256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDPBF16PS256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDPBF16PS256_MASK");
        }
    }
    /// Emits `VDPBF16PS256_MASKZ`.
    fn vdpbf16ps256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDPBF16PS256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDPBF16PS256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDPBF16PS256_MASKZ");
        }
    }
    /// Emits `VDPBF16PS256RRB`.
    fn vdpbf16ps256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDPBF16PS256RRB_MASK`.
    fn vdpbf16ps256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDPBF16PS256RRB_MASKZ`.
    fn vdpbf16ps256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDPBF16PS512`.
    fn vdpbf16ps512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDPBF16PS512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDPBF16PS512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDPBF16PS512");
        }
    }
    /// Emits `VDPBF16PS512_MASK`.
    fn vdpbf16ps512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDPBF16PS512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDPBF16PS512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDPBF16PS512_MASK");
        }
    }
    /// Emits `VDPBF16PS512_MASKZ`.
    fn vdpbf16ps512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VDPBF16PS512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VDPBF16PS512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VDPBF16PS512_MASKZ");
        }
    }
    /// Emits `VDPBF16PS512RRB`.
    fn vdpbf16ps512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDPBF16PS512RRB_MASK`.
    fn vdpbf16ps512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDPBF16PS512RRB_MASKZ`.
    fn vdpbf16ps512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VDPBF16PS512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
}
