pub trait X86AVX512VBMI2Emitter: Emitter {
    /// Emits `VPCOMPRESSB128`.
    fn vpcompressb128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSB128MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSB128RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSB128");
        }
    }
    /// Emits `VPCOMPRESSB128_MASK`.
    fn vpcompressb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSB128MR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSB128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSB128_MASK");
        }
    }
    /// Emits `VPCOMPRESSB128RR_MASKZ`.
    fn vpcompressb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCOMPRESSB256`.
    fn vpcompressb256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSB256MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSB256RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSB256");
        }
    }
    /// Emits `VPCOMPRESSB256_MASK`.
    fn vpcompressb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSB256MR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSB256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSB256_MASK");
        }
    }
    /// Emits `VPCOMPRESSB256RR_MASKZ`.
    fn vpcompressb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCOMPRESSB512`.
    fn vpcompressb512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSB512MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSB512RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSB512");
        }
    }
    /// Emits `VPCOMPRESSB512_MASK`.
    fn vpcompressb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSB512MR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSB512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSB512_MASK");
        }
    }
    /// Emits `VPCOMPRESSB512RR_MASKZ`.
    fn vpcompressb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSB512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCOMPRESSW128`.
    fn vpcompressw128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSW128MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSW128RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSW128");
        }
    }
    /// Emits `VPCOMPRESSW128_MASK`.
    fn vpcompressw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSW128MR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSW128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSW128_MASK");
        }
    }
    /// Emits `VPCOMPRESSW128RR_MASKZ`.
    fn vpcompressw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCOMPRESSW256`.
    fn vpcompressw256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSW256MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSW256RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSW256");
        }
    }
    /// Emits `VPCOMPRESSW256_MASK`.
    fn vpcompressw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSW256MR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSW256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSW256_MASK");
        }
    }
    /// Emits `VPCOMPRESSW256RR_MASKZ`.
    fn vpcompressw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCOMPRESSW512`.
    fn vpcompressw512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSW512MR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSW512RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSW512");
        }
    }
    /// Emits `VPCOMPRESSW512_MASK`.
    fn vpcompressw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mem() && op1.is_vec() {
            self.emit(VPCOMPRESSW512MR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPCOMPRESSW512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCOMPRESSW512_MASK");
        }
    }
    /// Emits `VPCOMPRESSW512RR_MASKZ`.
    fn vpcompressw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCOMPRESSW512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPEXPANDB128`.
    fn vpexpandb128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDB128RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDB128RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDB128");
        }
    }
    /// Emits `VPEXPANDB128_MASK`.
    fn vpexpandb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDB128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDB128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDB128_MASK");
        }
    }
    /// Emits `VPEXPANDB128_MASKZ`.
    fn vpexpandb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDB128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDB128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDB128_MASKZ");
        }
    }
    /// Emits `VPEXPANDB256`.
    fn vpexpandb256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDB256RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDB256RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDB256");
        }
    }
    /// Emits `VPEXPANDB256_MASK`.
    fn vpexpandb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDB256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDB256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDB256_MASK");
        }
    }
    /// Emits `VPEXPANDB256_MASKZ`.
    fn vpexpandb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDB256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDB256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDB256_MASKZ");
        }
    }
    /// Emits `VPEXPANDB512`.
    fn vpexpandb512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDB512RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDB512RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDB512");
        }
    }
    /// Emits `VPEXPANDB512_MASK`.
    fn vpexpandb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDB512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDB512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDB512_MASK");
        }
    }
    /// Emits `VPEXPANDB512_MASKZ`.
    fn vpexpandb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDB512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDB512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDB512_MASKZ");
        }
    }
    /// Emits `VPEXPANDW128`.
    fn vpexpandw128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDW128RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDW128RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDW128");
        }
    }
    /// Emits `VPEXPANDW128_MASK`.
    fn vpexpandw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDW128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDW128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDW128_MASK");
        }
    }
    /// Emits `VPEXPANDW128_MASKZ`.
    fn vpexpandw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDW128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDW128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDW128_MASKZ");
        }
    }
    /// Emits `VPEXPANDW256`.
    fn vpexpandw256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDW256RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDW256RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDW256");
        }
    }
    /// Emits `VPEXPANDW256_MASK`.
    fn vpexpandw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDW256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDW256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDW256_MASK");
        }
    }
    /// Emits `VPEXPANDW256_MASKZ`.
    fn vpexpandw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDW256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDW256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDW256_MASKZ");
        }
    }
    /// Emits `VPEXPANDW512`.
    fn vpexpandw512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDW512RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDW512RR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDW512");
        }
    }
    /// Emits `VPEXPANDW512_MASK`.
    fn vpexpandw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDW512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDW512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDW512_MASK");
        }
    }
    /// Emits `VPEXPANDW512_MASKZ`.
    fn vpexpandw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_mem() {
            self.emit(VPEXPANDW512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_vec() {
            self.emit(VPEXPANDW512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXPANDW512_MASKZ");
        }
    }
    /// Emits `VPSHLDD128`.
    fn vpshldd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDD128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDD128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDD128");
        }
    }
    /// Emits `VPSHLDD128_MASK`.
    fn vpshldd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDD128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDD128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDD128_MASK");
        }
    }
    /// Emits `VPSHLDD128_MASKZ`.
    fn vpshldd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDD128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDD128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDD128_MASKZ");
        }
    }
    /// Emits `VPSHLDD128RRBI`.
    fn vpshldd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDD128RRBI_MASK`.
    fn vpshldd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDD128RRBI_MASKZ`.
    fn vpshldd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDD256`.
    fn vpshldd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDD256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDD256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDD256");
        }
    }
    /// Emits `VPSHLDD256_MASK`.
    fn vpshldd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDD256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDD256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDD256_MASK");
        }
    }
    /// Emits `VPSHLDD256_MASKZ`.
    fn vpshldd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDD256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDD256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDD256_MASKZ");
        }
    }
    /// Emits `VPSHLDD256RRBI`.
    fn vpshldd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDD256RRBI_MASK`.
    fn vpshldd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDD256RRBI_MASKZ`.
    fn vpshldd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDD512`.
    fn vpshldd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDD512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDD512RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDD512");
        }
    }
    /// Emits `VPSHLDD512_MASK`.
    fn vpshldd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDD512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDD512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDD512_MASK");
        }
    }
    /// Emits `VPSHLDD512_MASKZ`.
    fn vpshldd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDD512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDD512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDD512_MASKZ");
        }
    }
    /// Emits `VPSHLDD512RRBI`.
    fn vpshldd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDD512RRBI_MASK`.
    fn vpshldd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDD512RRBI_MASKZ`.
    fn vpshldd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDD512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDQ128`.
    fn vpshldq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDQ128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDQ128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDQ128");
        }
    }
    /// Emits `VPSHLDQ128_MASK`.
    fn vpshldq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDQ128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDQ128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDQ128_MASK");
        }
    }
    /// Emits `VPSHLDQ128_MASKZ`.
    fn vpshldq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDQ128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDQ128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDQ128_MASKZ");
        }
    }
    /// Emits `VPSHLDQ128RRBI`.
    fn vpshldq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDQ128RRBI_MASK`.
    fn vpshldq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDQ128RRBI_MASKZ`.
    fn vpshldq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDQ256`.
    fn vpshldq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDQ256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDQ256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDQ256");
        }
    }
    /// Emits `VPSHLDQ256_MASK`.
    fn vpshldq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDQ256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDQ256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDQ256_MASK");
        }
    }
    /// Emits `VPSHLDQ256_MASKZ`.
    fn vpshldq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDQ256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDQ256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDQ256_MASKZ");
        }
    }
    /// Emits `VPSHLDQ256RRBI`.
    fn vpshldq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDQ256RRBI_MASK`.
    fn vpshldq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDQ256RRBI_MASKZ`.
    fn vpshldq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDQ512`.
    fn vpshldq512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDQ512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDQ512RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDQ512");
        }
    }
    /// Emits `VPSHLDQ512_MASK`.
    fn vpshldq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDQ512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDQ512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDQ512_MASK");
        }
    }
    /// Emits `VPSHLDQ512_MASKZ`.
    fn vpshldq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDQ512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDQ512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDQ512_MASKZ");
        }
    }
    /// Emits `VPSHLDQ512RRBI`.
    fn vpshldq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDQ512RRBI_MASK`.
    fn vpshldq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDQ512RRBI_MASKZ`.
    fn vpshldq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHLDQ512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHLDVD128`.
    fn vpshldvd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVD128");
        }
    }
    /// Emits `VPSHLDVD128_MASK`.
    fn vpshldvd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVD128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVD128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVD128_MASK");
        }
    }
    /// Emits `VPSHLDVD128_MASKZ`.
    fn vpshldvd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVD128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVD128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVD128_MASKZ");
        }
    }
    /// Emits `VPSHLDVD128RRB`.
    fn vpshldvd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVD128RRB_MASK`.
    fn vpshldvd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVD128RRB_MASKZ`.
    fn vpshldvd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVD256`.
    fn vpshldvd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVD256");
        }
    }
    /// Emits `VPSHLDVD256_MASK`.
    fn vpshldvd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVD256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVD256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVD256_MASK");
        }
    }
    /// Emits `VPSHLDVD256_MASKZ`.
    fn vpshldvd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVD256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVD256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVD256_MASKZ");
        }
    }
    /// Emits `VPSHLDVD256RRB`.
    fn vpshldvd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVD256RRB_MASK`.
    fn vpshldvd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVD256RRB_MASKZ`.
    fn vpshldvd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVD512`.
    fn vpshldvd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVD512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVD512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVD512");
        }
    }
    /// Emits `VPSHLDVD512_MASK`.
    fn vpshldvd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVD512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVD512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVD512_MASK");
        }
    }
    /// Emits `VPSHLDVD512_MASKZ`.
    fn vpshldvd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVD512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVD512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVD512_MASKZ");
        }
    }
    /// Emits `VPSHLDVD512RRB`.
    fn vpshldvd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVD512RRB_MASK`.
    fn vpshldvd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVD512RRB_MASKZ`.
    fn vpshldvd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVQ128`.
    fn vpshldvq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVQ128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVQ128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVQ128");
        }
    }
    /// Emits `VPSHLDVQ128_MASK`.
    fn vpshldvq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVQ128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVQ128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVQ128_MASK");
        }
    }
    /// Emits `VPSHLDVQ128_MASKZ`.
    fn vpshldvq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVQ128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVQ128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVQ128_MASKZ");
        }
    }
    /// Emits `VPSHLDVQ128RRB`.
    fn vpshldvq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVQ128RRB_MASK`.
    fn vpshldvq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVQ128RRB_MASKZ`.
    fn vpshldvq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVQ256`.
    fn vpshldvq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVQ256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVQ256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVQ256");
        }
    }
    /// Emits `VPSHLDVQ256_MASK`.
    fn vpshldvq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVQ256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVQ256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVQ256_MASK");
        }
    }
    /// Emits `VPSHLDVQ256_MASKZ`.
    fn vpshldvq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVQ256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVQ256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVQ256_MASKZ");
        }
    }
    /// Emits `VPSHLDVQ256RRB`.
    fn vpshldvq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVQ256RRB_MASK`.
    fn vpshldvq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVQ256RRB_MASKZ`.
    fn vpshldvq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVQ512`.
    fn vpshldvq512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVQ512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVQ512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVQ512");
        }
    }
    /// Emits `VPSHLDVQ512_MASK`.
    fn vpshldvq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVQ512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVQ512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVQ512_MASK");
        }
    }
    /// Emits `VPSHLDVQ512_MASKZ`.
    fn vpshldvq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVQ512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVQ512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVQ512_MASKZ");
        }
    }
    /// Emits `VPSHLDVQ512RRB`.
    fn vpshldvq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVQ512RRB_MASK`.
    fn vpshldvq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVQ512RRB_MASKZ`.
    fn vpshldvq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHLDVQ512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHLDVW128`.
    fn vpshldvw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVW128");
        }
    }
    /// Emits `VPSHLDVW128_MASK`.
    fn vpshldvw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVW128_MASK");
        }
    }
    /// Emits `VPSHLDVW128_MASKZ`.
    fn vpshldvw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVW128_MASKZ");
        }
    }
    /// Emits `VPSHLDVW256`.
    fn vpshldvw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVW256");
        }
    }
    /// Emits `VPSHLDVW256_MASK`.
    fn vpshldvw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVW256_MASK");
        }
    }
    /// Emits `VPSHLDVW256_MASKZ`.
    fn vpshldvw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVW256_MASKZ");
        }
    }
    /// Emits `VPSHLDVW512`.
    fn vpshldvw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVW512");
        }
    }
    /// Emits `VPSHLDVW512_MASK`.
    fn vpshldvw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVW512_MASK");
        }
    }
    /// Emits `VPSHLDVW512_MASKZ`.
    fn vpshldvw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHLDVW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHLDVW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHLDVW512_MASKZ");
        }
    }
    /// Emits `VPSHLDW128`.
    fn vpshldw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDW128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDW128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDW128");
        }
    }
    /// Emits `VPSHLDW128_MASK`.
    fn vpshldw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDW128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDW128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDW128_MASK");
        }
    }
    /// Emits `VPSHLDW128_MASKZ`.
    fn vpshldw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDW128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDW128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDW128_MASKZ");
        }
    }
    /// Emits `VPSHLDW256`.
    fn vpshldw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDW256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDW256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDW256");
        }
    }
    /// Emits `VPSHLDW256_MASK`.
    fn vpshldw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDW256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDW256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDW256_MASK");
        }
    }
    /// Emits `VPSHLDW256_MASKZ`.
    fn vpshldw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDW256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDW256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDW256_MASKZ");
        }
    }
    /// Emits `VPSHLDW512`.
    fn vpshldw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDW512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDW512RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDW512");
        }
    }
    /// Emits `VPSHLDW512_MASK`.
    fn vpshldw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDW512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDW512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDW512_MASK");
        }
    }
    /// Emits `VPSHLDW512_MASKZ`.
    fn vpshldw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHLDW512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHLDW512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHLDW512_MASKZ");
        }
    }
    /// Emits `VPSHRDD128`.
    fn vpshrdd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDD128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDD128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDD128");
        }
    }
    /// Emits `VPSHRDD128_MASK`.
    fn vpshrdd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDD128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDD128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDD128_MASK");
        }
    }
    /// Emits `VPSHRDD128_MASKZ`.
    fn vpshrdd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDD128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDD128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDD128_MASKZ");
        }
    }
    /// Emits `VPSHRDD128RRBI`.
    fn vpshrdd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDD128RRBI_MASK`.
    fn vpshrdd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDD128RRBI_MASKZ`.
    fn vpshrdd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDD256`.
    fn vpshrdd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDD256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDD256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDD256");
        }
    }
    /// Emits `VPSHRDD256_MASK`.
    fn vpshrdd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDD256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDD256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDD256_MASK");
        }
    }
    /// Emits `VPSHRDD256_MASKZ`.
    fn vpshrdd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDD256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDD256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDD256_MASKZ");
        }
    }
    /// Emits `VPSHRDD256RRBI`.
    fn vpshrdd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDD256RRBI_MASK`.
    fn vpshrdd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDD256RRBI_MASKZ`.
    fn vpshrdd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDD512`.
    fn vpshrdd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDD512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDD512RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDD512");
        }
    }
    /// Emits `VPSHRDD512_MASK`.
    fn vpshrdd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDD512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDD512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDD512_MASK");
        }
    }
    /// Emits `VPSHRDD512_MASKZ`.
    fn vpshrdd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDD512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDD512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDD512_MASKZ");
        }
    }
    /// Emits `VPSHRDD512RRBI`.
    fn vpshrdd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDD512RRBI_MASK`.
    fn vpshrdd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDD512RRBI_MASKZ`.
    fn vpshrdd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDD512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDQ128`.
    fn vpshrdq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDQ128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDQ128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDQ128");
        }
    }
    /// Emits `VPSHRDQ128_MASK`.
    fn vpshrdq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDQ128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDQ128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDQ128_MASK");
        }
    }
    /// Emits `VPSHRDQ128_MASKZ`.
    fn vpshrdq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDQ128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDQ128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDQ128_MASKZ");
        }
    }
    /// Emits `VPSHRDQ128RRBI`.
    fn vpshrdq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDQ128RRBI_MASK`.
    fn vpshrdq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDQ128RRBI_MASKZ`.
    fn vpshrdq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ128RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDQ256`.
    fn vpshrdq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDQ256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDQ256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDQ256");
        }
    }
    /// Emits `VPSHRDQ256_MASK`.
    fn vpshrdq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDQ256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDQ256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDQ256_MASK");
        }
    }
    /// Emits `VPSHRDQ256_MASKZ`.
    fn vpshrdq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDQ256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDQ256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDQ256_MASKZ");
        }
    }
    /// Emits `VPSHRDQ256RRBI`.
    fn vpshrdq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDQ256RRBI_MASK`.
    fn vpshrdq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDQ256RRBI_MASKZ`.
    fn vpshrdq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ256RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDQ512`.
    fn vpshrdq512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDQ512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDQ512RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDQ512");
        }
    }
    /// Emits `VPSHRDQ512_MASK`.
    fn vpshrdq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDQ512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDQ512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDQ512_MASK");
        }
    }
    /// Emits `VPSHRDQ512_MASKZ`.
    fn vpshrdq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDQ512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDQ512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDQ512_MASKZ");
        }
    }
    /// Emits `VPSHRDQ512RRBI`.
    fn vpshrdq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRBI, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDQ512RRBI_MASK`.
    fn vpshrdq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRBI_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDQ512RRBI_MASKZ`.
    fn vpshrdq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        self.emit(VPSHRDQ512RRBI_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),op3.as_operand(),);
    }
    /// Emits `VPSHRDVD128`.
    fn vpshrdvd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVD128");
        }
    }
    /// Emits `VPSHRDVD128_MASK`.
    fn vpshrdvd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVD128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVD128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVD128_MASK");
        }
    }
    /// Emits `VPSHRDVD128_MASKZ`.
    fn vpshrdvd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVD128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVD128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVD128_MASKZ");
        }
    }
    /// Emits `VPSHRDVD128RRB`.
    fn vpshrdvd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVD128RRB_MASK`.
    fn vpshrdvd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVD128RRB_MASKZ`.
    fn vpshrdvd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVD256`.
    fn vpshrdvd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVD256");
        }
    }
    /// Emits `VPSHRDVD256_MASK`.
    fn vpshrdvd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVD256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVD256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVD256_MASK");
        }
    }
    /// Emits `VPSHRDVD256_MASKZ`.
    fn vpshrdvd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVD256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVD256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVD256_MASKZ");
        }
    }
    /// Emits `VPSHRDVD256RRB`.
    fn vpshrdvd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVD256RRB_MASK`.
    fn vpshrdvd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVD256RRB_MASKZ`.
    fn vpshrdvd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVD512`.
    fn vpshrdvd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVD512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVD512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVD512");
        }
    }
    /// Emits `VPSHRDVD512_MASK`.
    fn vpshrdvd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVD512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVD512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVD512_MASK");
        }
    }
    /// Emits `VPSHRDVD512_MASKZ`.
    fn vpshrdvd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVD512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVD512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVD512_MASKZ");
        }
    }
    /// Emits `VPSHRDVD512RRB`.
    fn vpshrdvd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVD512RRB_MASK`.
    fn vpshrdvd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVD512RRB_MASKZ`.
    fn vpshrdvd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVD512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVQ128`.
    fn vpshrdvq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVQ128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVQ128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVQ128");
        }
    }
    /// Emits `VPSHRDVQ128_MASK`.
    fn vpshrdvq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVQ128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVQ128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVQ128_MASK");
        }
    }
    /// Emits `VPSHRDVQ128_MASKZ`.
    fn vpshrdvq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVQ128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVQ128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVQ128_MASKZ");
        }
    }
    /// Emits `VPSHRDVQ128RRB`.
    fn vpshrdvq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVQ128RRB_MASK`.
    fn vpshrdvq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVQ128RRB_MASKZ`.
    fn vpshrdvq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVQ256`.
    fn vpshrdvq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVQ256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVQ256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVQ256");
        }
    }
    /// Emits `VPSHRDVQ256_MASK`.
    fn vpshrdvq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVQ256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVQ256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVQ256_MASK");
        }
    }
    /// Emits `VPSHRDVQ256_MASKZ`.
    fn vpshrdvq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVQ256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVQ256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVQ256_MASKZ");
        }
    }
    /// Emits `VPSHRDVQ256RRB`.
    fn vpshrdvq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVQ256RRB_MASK`.
    fn vpshrdvq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVQ256RRB_MASKZ`.
    fn vpshrdvq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVQ512`.
    fn vpshrdvq512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVQ512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVQ512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVQ512");
        }
    }
    /// Emits `VPSHRDVQ512_MASK`.
    fn vpshrdvq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVQ512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVQ512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVQ512_MASK");
        }
    }
    /// Emits `VPSHRDVQ512_MASKZ`.
    fn vpshrdvq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVQ512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVQ512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVQ512_MASKZ");
        }
    }
    /// Emits `VPSHRDVQ512RRB`.
    fn vpshrdvq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVQ512RRB_MASK`.
    fn vpshrdvq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVQ512RRB_MASKZ`.
    fn vpshrdvq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPSHRDVQ512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPSHRDVW128`.
    fn vpshrdvw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVW128");
        }
    }
    /// Emits `VPSHRDVW128_MASK`.
    fn vpshrdvw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVW128_MASK");
        }
    }
    /// Emits `VPSHRDVW128_MASKZ`.
    fn vpshrdvw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVW128_MASKZ");
        }
    }
    /// Emits `VPSHRDVW256`.
    fn vpshrdvw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVW256");
        }
    }
    /// Emits `VPSHRDVW256_MASK`.
    fn vpshrdvw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVW256_MASK");
        }
    }
    /// Emits `VPSHRDVW256_MASKZ`.
    fn vpshrdvw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVW256_MASKZ");
        }
    }
    /// Emits `VPSHRDVW512`.
    fn vpshrdvw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVW512");
        }
    }
    /// Emits `VPSHRDVW512_MASK`.
    fn vpshrdvw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVW512_MASK");
        }
    }
    /// Emits `VPSHRDVW512_MASKZ`.
    fn vpshrdvw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHRDVW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHRDVW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHRDVW512_MASKZ");
        }
    }
    /// Emits `VPSHRDW128`.
    fn vpshrdw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDW128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDW128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDW128");
        }
    }
    /// Emits `VPSHRDW128_MASK`.
    fn vpshrdw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDW128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDW128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDW128_MASK");
        }
    }
    /// Emits `VPSHRDW128_MASKZ`.
    fn vpshrdw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDW128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDW128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDW128_MASKZ");
        }
    }
    /// Emits `VPSHRDW256`.
    fn vpshrdw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDW256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDW256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDW256");
        }
    }
    /// Emits `VPSHRDW256_MASK`.
    fn vpshrdw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDW256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDW256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDW256_MASK");
        }
    }
    /// Emits `VPSHRDW256_MASKZ`.
    fn vpshrdw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDW256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDW256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDW256_MASKZ");
        }
    }
    /// Emits `VPSHRDW512`.
    fn vpshrdw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDW512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDW512RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDW512");
        }
    }
    /// Emits `VPSHRDW512_MASK`.
    fn vpshrdw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDW512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDW512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDW512_MASK");
        }
    }
    /// Emits `VPSHRDW512_MASKZ`.
    fn vpshrdw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPSHRDW512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPSHRDW512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPSHRDW512_MASKZ");
        }
    }
}
