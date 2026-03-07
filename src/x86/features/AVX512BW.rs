pub trait X86AVX512BWEmitter: Emitter {
    /// Emits `KADDDKKK`.
    fn kadddkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KADDDKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KADDQKKK`.
    fn kaddqkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KADDQKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KANDDKKK`.
    fn kanddkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KANDDKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KANDNDKKK`.
    fn kandndkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KANDNDKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KANDNQKKK`.
    fn kandnqkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KANDNQKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KANDQKKK`.
    fn kandqkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KANDQKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KMOVDK`.
    fn kmovdk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mask() && op1.is_mem() {
            self.emit(KMOVDKM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_mask() {
            self.emit(KMOVDMK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mask() && op1.is_gp() {
            self.emit(KMOVDKR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mask() {
            self.emit(KMOVDRK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for KMOVDK");
        }
    }
    /// Emits `KMOVDKK`.
    fn kmovdkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KMOVDKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `KMOVQK`.
    fn kmovqk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_mask() && op1.is_mem() {
            self.emit(KMOVQKM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_mask() {
            self.emit(KMOVQMK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mask() && op1.is_gp() {
            self.emit(KMOVQKR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mask() {
            self.emit(KMOVQRK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for KMOVQK");
        }
    }
    /// Emits `KMOVQKK`.
    fn kmovqkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KMOVQKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `KNOTDKK`.
    fn knotdkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KNOTDKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `KNOTQKK`.
    fn knotqkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KNOTQKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `KORDKKK`.
    fn kordkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KORDKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KORQKKK`.
    fn korqkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KORQKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KORTESTDKK`.
    fn kortestdkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KORTESTDKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `KORTESTQKK`.
    fn kortestqkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KORTESTQKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `KSHIFTLDKKI`.
    fn kshiftldkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KSHIFTLDKKI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KSHIFTLQKKI`.
    fn kshiftlqkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KSHIFTLQKKI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KSHIFTRDKKI`.
    fn kshiftrdkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KSHIFTRDKKI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KSHIFTRQKKI`.
    fn kshiftrqkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KSHIFTRQKKI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KTESTDKK`.
    fn ktestdkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KTESTDKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `KTESTQKK`.
    fn ktestqkk(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(KTESTQKK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `KUNPCKDQKKK`.
    fn kunpckdqkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KUNPCKDQKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KUNPCKWDKKK`.
    fn kunpckwdkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KUNPCKWDKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KXNORDKKK`.
    fn kxnordkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KXNORDKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KXNORQKKK`.
    fn kxnorqkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KXNORQKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KXORDKKK`.
    fn kxordkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KXORDKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `KXORQKKK`.
    fn kxorqkkk(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(KXORQKKK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VDBPSADBW128`.
    fn vdbpsadbw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDBPSADBW128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDBPSADBW128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDBPSADBW128");
        }
    }
    /// Emits `VDBPSADBW128_MASK`.
    fn vdbpsadbw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDBPSADBW128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDBPSADBW128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDBPSADBW128_MASK");
        }
    }
    /// Emits `VDBPSADBW128_MASKZ`.
    fn vdbpsadbw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDBPSADBW128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDBPSADBW128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDBPSADBW128_MASKZ");
        }
    }
    /// Emits `VDBPSADBW256`.
    fn vdbpsadbw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDBPSADBW256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDBPSADBW256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDBPSADBW256");
        }
    }
    /// Emits `VDBPSADBW256_MASK`.
    fn vdbpsadbw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDBPSADBW256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDBPSADBW256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDBPSADBW256_MASK");
        }
    }
    /// Emits `VDBPSADBW256_MASKZ`.
    fn vdbpsadbw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDBPSADBW256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDBPSADBW256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDBPSADBW256_MASKZ");
        }
    }
    /// Emits `VDBPSADBW512`.
    fn vdbpsadbw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDBPSADBW512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDBPSADBW512RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDBPSADBW512");
        }
    }
    /// Emits `VDBPSADBW512_MASK`.
    fn vdbpsadbw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDBPSADBW512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDBPSADBW512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDBPSADBW512_MASK");
        }
    }
    /// Emits `VDBPSADBW512_MASKZ`.
    fn vdbpsadbw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VDBPSADBW512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VDBPSADBW512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VDBPSADBW512_MASKZ");
        }
    }
    /// Emits `VMOVDQU16_128`.
    fn vmovdqu16_128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU16_128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU16_128RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU16_128MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU16_128");
        }
    }
    /// Emits `VMOVDQU16_128_MASK`.
    fn vmovdqu16_128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU16_128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU16_128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU16_128MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU16_128_MASK");
        }
    }
    /// Emits `VMOVDQU16_128_MASKZ`.
    fn vmovdqu16_128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU16_128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU16_128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU16_128_MASKZ");
        }
    }
    /// Emits `VMOVDQU16_256`.
    fn vmovdqu16_256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU16_256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU16_256RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU16_256MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU16_256");
        }
    }
    /// Emits `VMOVDQU16_256_MASK`.
    fn vmovdqu16_256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU16_256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU16_256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU16_256MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU16_256_MASK");
        }
    }
    /// Emits `VMOVDQU16_256_MASKZ`.
    fn vmovdqu16_256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU16_256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU16_256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU16_256_MASKZ");
        }
    }
    /// Emits `VMOVDQU16_512`.
    fn vmovdqu16_512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU16_512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU16_512RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU16_512MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU16_512");
        }
    }
    /// Emits `VMOVDQU16_512_MASK`.
    fn vmovdqu16_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU16_512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU16_512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU16_512MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU16_512_MASK");
        }
    }
    /// Emits `VMOVDQU16_512_MASKZ`.
    fn vmovdqu16_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU16_512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU16_512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU16_512_MASKZ");
        }
    }
    /// Emits `VMOVDQU8_128`.
    fn vmovdqu8_128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU8_128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU8_128RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU8_128MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU8_128");
        }
    }
    /// Emits `VMOVDQU8_128_MASK`.
    fn vmovdqu8_128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU8_128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU8_128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU8_128MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU8_128_MASK");
        }
    }
    /// Emits `VMOVDQU8_128_MASKZ`.
    fn vmovdqu8_128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU8_128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU8_128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU8_128_MASKZ");
        }
    }
    /// Emits `VMOVDQU8_256`.
    fn vmovdqu8_256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU8_256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU8_256RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU8_256MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU8_256");
        }
    }
    /// Emits `VMOVDQU8_256_MASK`.
    fn vmovdqu8_256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU8_256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU8_256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU8_256MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU8_256_MASK");
        }
    }
    /// Emits `VMOVDQU8_256_MASKZ`.
    fn vmovdqu8_256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU8_256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU8_256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU8_256_MASKZ");
        }
    }
    /// Emits `VMOVDQU8_512`.
    fn vmovdqu8_512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU8_512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU8_512RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU8_512MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU8_512");
        }
    }
    /// Emits `VMOVDQU8_512_MASK`.
    fn vmovdqu8_512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU8_512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU8_512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VMOVDQU8_512MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU8_512_MASK");
        }
    }
    /// Emits `VMOVDQU8_512_MASKZ`.
    fn vmovdqu8_512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VMOVDQU8_512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VMOVDQU8_512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMOVDQU8_512_MASKZ");
        }
    }
    /// Emits `VPABSB128`.
    fn vpabsb128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSB128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSB128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSB128");
        }
    }
    /// Emits `VPABSB128_MASK`.
    fn vpabsb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSB128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSB128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSB128_MASK");
        }
    }
    /// Emits `VPABSB128_MASKZ`.
    fn vpabsb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSB128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSB128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSB128_MASKZ");
        }
    }
    /// Emits `VPABSB256`.
    fn vpabsb256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSB256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSB256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSB256");
        }
    }
    /// Emits `VPABSB256_MASK`.
    fn vpabsb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSB256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSB256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSB256_MASK");
        }
    }
    /// Emits `VPABSB256_MASKZ`.
    fn vpabsb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSB256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSB256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSB256_MASKZ");
        }
    }
    /// Emits `VPABSB512`.
    fn vpabsb512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSB512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSB512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSB512");
        }
    }
    /// Emits `VPABSB512_MASK`.
    fn vpabsb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSB512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSB512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSB512_MASK");
        }
    }
    /// Emits `VPABSB512_MASKZ`.
    fn vpabsb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSB512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSB512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSB512_MASKZ");
        }
    }
    /// Emits `VPABSW128`.
    fn vpabsw128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSW128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSW128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSW128");
        }
    }
    /// Emits `VPABSW128_MASK`.
    fn vpabsw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSW128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSW128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSW128_MASK");
        }
    }
    /// Emits `VPABSW128_MASKZ`.
    fn vpabsw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSW128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSW128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSW128_MASKZ");
        }
    }
    /// Emits `VPABSW256`.
    fn vpabsw256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSW256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSW256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSW256");
        }
    }
    /// Emits `VPABSW256_MASK`.
    fn vpabsw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSW256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSW256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSW256_MASK");
        }
    }
    /// Emits `VPABSW256_MASKZ`.
    fn vpabsw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSW256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSW256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSW256_MASKZ");
        }
    }
    /// Emits `VPABSW512`.
    fn vpabsw512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSW512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSW512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSW512");
        }
    }
    /// Emits `VPABSW512_MASK`.
    fn vpabsw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSW512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSW512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSW512_MASK");
        }
    }
    /// Emits `VPABSW512_MASKZ`.
    fn vpabsw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPABSW512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPABSW512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPABSW512_MASKZ");
        }
    }
    /// Emits `VPACKSSDW128`.
    fn vpackssdw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSDW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSDW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSDW128");
        }
    }
    /// Emits `VPACKSSDW128_MASK`.
    fn vpackssdw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSDW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSDW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSDW128_MASK");
        }
    }
    /// Emits `VPACKSSDW128_MASKZ`.
    fn vpackssdw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSDW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSDW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSDW128_MASKZ");
        }
    }
    /// Emits `VPACKSSDW128RRB`.
    fn vpackssdw128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKSSDW128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKSSDW128RRB_MASK`.
    fn vpackssdw128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKSSDW128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKSSDW128RRB_MASKZ`.
    fn vpackssdw128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKSSDW128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKSSDW256`.
    fn vpackssdw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSDW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSDW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSDW256");
        }
    }
    /// Emits `VPACKSSDW256_MASK`.
    fn vpackssdw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSDW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSDW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSDW256_MASK");
        }
    }
    /// Emits `VPACKSSDW256_MASKZ`.
    fn vpackssdw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSDW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSDW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSDW256_MASKZ");
        }
    }
    /// Emits `VPACKSSDW256RRB`.
    fn vpackssdw256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKSSDW256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKSSDW256RRB_MASK`.
    fn vpackssdw256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKSSDW256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKSSDW256RRB_MASKZ`.
    fn vpackssdw256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKSSDW256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKSSDW512`.
    fn vpackssdw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSDW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSDW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSDW512");
        }
    }
    /// Emits `VPACKSSDW512_MASK`.
    fn vpackssdw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSDW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSDW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSDW512_MASK");
        }
    }
    /// Emits `VPACKSSDW512_MASKZ`.
    fn vpackssdw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSDW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSDW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSDW512_MASKZ");
        }
    }
    /// Emits `VPACKSSDW512RRB`.
    fn vpackssdw512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKSSDW512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKSSDW512RRB_MASK`.
    fn vpackssdw512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKSSDW512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKSSDW512RRB_MASKZ`.
    fn vpackssdw512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKSSDW512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKSSWB128`.
    fn vpacksswb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSWB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSWB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSWB128");
        }
    }
    /// Emits `VPACKSSWB128_MASK`.
    fn vpacksswb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSWB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSWB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSWB128_MASK");
        }
    }
    /// Emits `VPACKSSWB128_MASKZ`.
    fn vpacksswb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSWB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSWB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSWB128_MASKZ");
        }
    }
    /// Emits `VPACKSSWB256`.
    fn vpacksswb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSWB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSWB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSWB256");
        }
    }
    /// Emits `VPACKSSWB256_MASK`.
    fn vpacksswb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSWB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSWB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSWB256_MASK");
        }
    }
    /// Emits `VPACKSSWB256_MASKZ`.
    fn vpacksswb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSWB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSWB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSWB256_MASKZ");
        }
    }
    /// Emits `VPACKSSWB512`.
    fn vpacksswb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSWB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSWB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSWB512");
        }
    }
    /// Emits `VPACKSSWB512_MASK`.
    fn vpacksswb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSWB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSWB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSWB512_MASK");
        }
    }
    /// Emits `VPACKSSWB512_MASKZ`.
    fn vpacksswb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKSSWB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKSSWB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKSSWB512_MASKZ");
        }
    }
    /// Emits `VPACKUSDW128`.
    fn vpackusdw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSDW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSDW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSDW128");
        }
    }
    /// Emits `VPACKUSDW128_MASK`.
    fn vpackusdw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSDW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSDW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSDW128_MASK");
        }
    }
    /// Emits `VPACKUSDW128_MASKZ`.
    fn vpackusdw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSDW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSDW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSDW128_MASKZ");
        }
    }
    /// Emits `VPACKUSDW128RRB`.
    fn vpackusdw128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKUSDW128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKUSDW128RRB_MASK`.
    fn vpackusdw128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKUSDW128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKUSDW128RRB_MASKZ`.
    fn vpackusdw128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKUSDW128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKUSDW256`.
    fn vpackusdw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSDW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSDW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSDW256");
        }
    }
    /// Emits `VPACKUSDW256_MASK`.
    fn vpackusdw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSDW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSDW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSDW256_MASK");
        }
    }
    /// Emits `VPACKUSDW256_MASKZ`.
    fn vpackusdw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSDW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSDW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSDW256_MASKZ");
        }
    }
    /// Emits `VPACKUSDW256RRB`.
    fn vpackusdw256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKUSDW256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKUSDW256RRB_MASK`.
    fn vpackusdw256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKUSDW256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKUSDW256RRB_MASKZ`.
    fn vpackusdw256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKUSDW256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKUSDW512`.
    fn vpackusdw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSDW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSDW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSDW512");
        }
    }
    /// Emits `VPACKUSDW512_MASK`.
    fn vpackusdw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSDW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSDW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSDW512_MASK");
        }
    }
    /// Emits `VPACKUSDW512_MASKZ`.
    fn vpackusdw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSDW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSDW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSDW512_MASKZ");
        }
    }
    /// Emits `VPACKUSDW512RRB`.
    fn vpackusdw512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKUSDW512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKUSDW512RRB_MASK`.
    fn vpackusdw512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKUSDW512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKUSDW512RRB_MASKZ`.
    fn vpackusdw512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPACKUSDW512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPACKUSWB128`.
    fn vpackuswb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSWB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSWB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSWB128");
        }
    }
    /// Emits `VPACKUSWB128_MASK`.
    fn vpackuswb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSWB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSWB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSWB128_MASK");
        }
    }
    /// Emits `VPACKUSWB128_MASKZ`.
    fn vpackuswb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSWB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSWB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSWB128_MASKZ");
        }
    }
    /// Emits `VPACKUSWB256`.
    fn vpackuswb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSWB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSWB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSWB256");
        }
    }
    /// Emits `VPACKUSWB256_MASK`.
    fn vpackuswb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSWB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSWB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSWB256_MASK");
        }
    }
    /// Emits `VPACKUSWB256_MASKZ`.
    fn vpackuswb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSWB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSWB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSWB256_MASKZ");
        }
    }
    /// Emits `VPACKUSWB512`.
    fn vpackuswb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSWB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSWB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSWB512");
        }
    }
    /// Emits `VPACKUSWB512_MASK`.
    fn vpackuswb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSWB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSWB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSWB512_MASK");
        }
    }
    /// Emits `VPACKUSWB512_MASKZ`.
    fn vpackuswb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPACKUSWB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPACKUSWB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPACKUSWB512_MASKZ");
        }
    }
    /// Emits `VPADDB128`.
    fn vpaddb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDB128");
        }
    }
    /// Emits `VPADDB128_MASK`.
    fn vpaddb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDB128_MASK");
        }
    }
    /// Emits `VPADDB128_MASKZ`.
    fn vpaddb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDB128_MASKZ");
        }
    }
    /// Emits `VPADDB256`.
    fn vpaddb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDB256");
        }
    }
    /// Emits `VPADDB256_MASK`.
    fn vpaddb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDB256_MASK");
        }
    }
    /// Emits `VPADDB256_MASKZ`.
    fn vpaddb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDB256_MASKZ");
        }
    }
    /// Emits `VPADDB512`.
    fn vpaddb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDB512");
        }
    }
    /// Emits `VPADDB512_MASK`.
    fn vpaddb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDB512_MASK");
        }
    }
    /// Emits `VPADDB512_MASKZ`.
    fn vpaddb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDB512_MASKZ");
        }
    }
    /// Emits `VPADDSB128`.
    fn vpaddsb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSB128");
        }
    }
    /// Emits `VPADDSB128_MASK`.
    fn vpaddsb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSB128_MASK");
        }
    }
    /// Emits `VPADDSB128_MASKZ`.
    fn vpaddsb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSB128_MASKZ");
        }
    }
    /// Emits `VPADDSB256`.
    fn vpaddsb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSB256");
        }
    }
    /// Emits `VPADDSB256_MASK`.
    fn vpaddsb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSB256_MASK");
        }
    }
    /// Emits `VPADDSB256_MASKZ`.
    fn vpaddsb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSB256_MASKZ");
        }
    }
    /// Emits `VPADDSB512`.
    fn vpaddsb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSB512");
        }
    }
    /// Emits `VPADDSB512_MASK`.
    fn vpaddsb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSB512_MASK");
        }
    }
    /// Emits `VPADDSB512_MASKZ`.
    fn vpaddsb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSB512_MASKZ");
        }
    }
    /// Emits `VPADDSW128`.
    fn vpaddsw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSW128");
        }
    }
    /// Emits `VPADDSW128_MASK`.
    fn vpaddsw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSW128_MASK");
        }
    }
    /// Emits `VPADDSW128_MASKZ`.
    fn vpaddsw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSW128_MASKZ");
        }
    }
    /// Emits `VPADDSW256`.
    fn vpaddsw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSW256");
        }
    }
    /// Emits `VPADDSW256_MASK`.
    fn vpaddsw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSW256_MASK");
        }
    }
    /// Emits `VPADDSW256_MASKZ`.
    fn vpaddsw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSW256_MASKZ");
        }
    }
    /// Emits `VPADDSW512`.
    fn vpaddsw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSW512");
        }
    }
    /// Emits `VPADDSW512_MASK`.
    fn vpaddsw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSW512_MASK");
        }
    }
    /// Emits `VPADDSW512_MASKZ`.
    fn vpaddsw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDSW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDSW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDSW512_MASKZ");
        }
    }
    /// Emits `VPADDUSB128`.
    fn vpaddusb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSB128");
        }
    }
    /// Emits `VPADDUSB128_MASK`.
    fn vpaddusb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSB128_MASK");
        }
    }
    /// Emits `VPADDUSB128_MASKZ`.
    fn vpaddusb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSB128_MASKZ");
        }
    }
    /// Emits `VPADDUSB256`.
    fn vpaddusb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSB256");
        }
    }
    /// Emits `VPADDUSB256_MASK`.
    fn vpaddusb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSB256_MASK");
        }
    }
    /// Emits `VPADDUSB256_MASKZ`.
    fn vpaddusb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSB256_MASKZ");
        }
    }
    /// Emits `VPADDUSB512`.
    fn vpaddusb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSB512");
        }
    }
    /// Emits `VPADDUSB512_MASK`.
    fn vpaddusb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSB512_MASK");
        }
    }
    /// Emits `VPADDUSB512_MASKZ`.
    fn vpaddusb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSB512_MASKZ");
        }
    }
    /// Emits `VPADDUSW128`.
    fn vpaddusw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSW128");
        }
    }
    /// Emits `VPADDUSW128_MASK`.
    fn vpaddusw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSW128_MASK");
        }
    }
    /// Emits `VPADDUSW128_MASKZ`.
    fn vpaddusw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSW128_MASKZ");
        }
    }
    /// Emits `VPADDUSW256`.
    fn vpaddusw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSW256");
        }
    }
    /// Emits `VPADDUSW256_MASK`.
    fn vpaddusw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSW256_MASK");
        }
    }
    /// Emits `VPADDUSW256_MASKZ`.
    fn vpaddusw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSW256_MASKZ");
        }
    }
    /// Emits `VPADDUSW512`.
    fn vpaddusw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSW512");
        }
    }
    /// Emits `VPADDUSW512_MASK`.
    fn vpaddusw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSW512_MASK");
        }
    }
    /// Emits `VPADDUSW512_MASKZ`.
    fn vpaddusw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDUSW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDUSW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDUSW512_MASKZ");
        }
    }
    /// Emits `VPADDW128`.
    fn vpaddw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDW128");
        }
    }
    /// Emits `VPADDW128_MASK`.
    fn vpaddw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDW128_MASK");
        }
    }
    /// Emits `VPADDW128_MASKZ`.
    fn vpaddw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDW128_MASKZ");
        }
    }
    /// Emits `VPADDW256`.
    fn vpaddw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDW256");
        }
    }
    /// Emits `VPADDW256_MASK`.
    fn vpaddw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDW256_MASK");
        }
    }
    /// Emits `VPADDW256_MASKZ`.
    fn vpaddw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDW256_MASKZ");
        }
    }
    /// Emits `VPADDW512`.
    fn vpaddw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDW512");
        }
    }
    /// Emits `VPADDW512_MASK`.
    fn vpaddw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDW512_MASK");
        }
    }
    /// Emits `VPADDW512_MASKZ`.
    fn vpaddw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPADDW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPADDW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPADDW512_MASKZ");
        }
    }
    /// Emits `VPALIGNR128`.
    fn vpalignr128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPALIGNR128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPALIGNR128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPALIGNR128");
        }
    }
    /// Emits `VPALIGNR128_MASK`.
    fn vpalignr128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPALIGNR128RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPALIGNR128RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPALIGNR128_MASK");
        }
    }
    /// Emits `VPALIGNR128_MASKZ`.
    fn vpalignr128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPALIGNR128RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPALIGNR128RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPALIGNR128_MASKZ");
        }
    }
    /// Emits `VPALIGNR256`.
    fn vpalignr256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPALIGNR256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPALIGNR256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPALIGNR256");
        }
    }
    /// Emits `VPALIGNR256_MASK`.
    fn vpalignr256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPALIGNR256RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPALIGNR256RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPALIGNR256_MASK");
        }
    }
    /// Emits `VPALIGNR256_MASKZ`.
    fn vpalignr256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPALIGNR256RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPALIGNR256RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPALIGNR256_MASKZ");
        }
    }
    /// Emits `VPALIGNR512`.
    fn vpalignr512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPALIGNR512RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPALIGNR512RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPALIGNR512");
        }
    }
    /// Emits `VPALIGNR512_MASK`.
    fn vpalignr512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPALIGNR512RRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPALIGNR512RRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPALIGNR512_MASK");
        }
    }
    /// Emits `VPALIGNR512_MASKZ`.
    fn vpalignr512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPALIGNR512RRRI_MASKZ, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPALIGNR512RRMI_MASKZ, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPALIGNR512_MASKZ");
        }
    }
    /// Emits `VPAVGB128`.
    fn vpavgb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGB128");
        }
    }
    /// Emits `VPAVGB128_MASK`.
    fn vpavgb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGB128_MASK");
        }
    }
    /// Emits `VPAVGB128_MASKZ`.
    fn vpavgb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGB128_MASKZ");
        }
    }
    /// Emits `VPAVGB256`.
    fn vpavgb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGB256");
        }
    }
    /// Emits `VPAVGB256_MASK`.
    fn vpavgb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGB256_MASK");
        }
    }
    /// Emits `VPAVGB256_MASKZ`.
    fn vpavgb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGB256_MASKZ");
        }
    }
    /// Emits `VPAVGB512`.
    fn vpavgb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGB512");
        }
    }
    /// Emits `VPAVGB512_MASK`.
    fn vpavgb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGB512_MASK");
        }
    }
    /// Emits `VPAVGB512_MASKZ`.
    fn vpavgb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGB512_MASKZ");
        }
    }
    /// Emits `VPAVGW128`.
    fn vpavgw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGW128");
        }
    }
    /// Emits `VPAVGW128_MASK`.
    fn vpavgw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGW128_MASK");
        }
    }
    /// Emits `VPAVGW128_MASKZ`.
    fn vpavgw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGW128_MASKZ");
        }
    }
    /// Emits `VPAVGW256`.
    fn vpavgw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGW256");
        }
    }
    /// Emits `VPAVGW256_MASK`.
    fn vpavgw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGW256_MASK");
        }
    }
    /// Emits `VPAVGW256_MASKZ`.
    fn vpavgw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGW256_MASKZ");
        }
    }
    /// Emits `VPAVGW512`.
    fn vpavgw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGW512");
        }
    }
    /// Emits `VPAVGW512_MASK`.
    fn vpavgw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGW512_MASK");
        }
    }
    /// Emits `VPAVGW512_MASKZ`.
    fn vpavgw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPAVGW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPAVGW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPAVGW512_MASKZ");
        }
    }
    /// Emits `VPBLENDMB128`.
    fn vpblendmb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMB128");
        }
    }
    /// Emits `VPBLENDMB128_MASK`.
    fn vpblendmb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMB128_MASK");
        }
    }
    /// Emits `VPBLENDMB128_MASKZ`.
    fn vpblendmb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMB128_MASKZ");
        }
    }
    /// Emits `VPBLENDMB256`.
    fn vpblendmb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMB256");
        }
    }
    /// Emits `VPBLENDMB256_MASK`.
    fn vpblendmb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMB256_MASK");
        }
    }
    /// Emits `VPBLENDMB256_MASKZ`.
    fn vpblendmb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMB256_MASKZ");
        }
    }
    /// Emits `VPBLENDMB512`.
    fn vpblendmb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMB512");
        }
    }
    /// Emits `VPBLENDMB512_MASK`.
    fn vpblendmb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMB512_MASK");
        }
    }
    /// Emits `VPBLENDMB512_MASKZ`.
    fn vpblendmb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMB512_MASKZ");
        }
    }
    /// Emits `VPBLENDMW128`.
    fn vpblendmw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMW128");
        }
    }
    /// Emits `VPBLENDMW128_MASK`.
    fn vpblendmw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMW128_MASK");
        }
    }
    /// Emits `VPBLENDMW128_MASKZ`.
    fn vpblendmw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMW128_MASKZ");
        }
    }
    /// Emits `VPBLENDMW256`.
    fn vpblendmw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMW256");
        }
    }
    /// Emits `VPBLENDMW256_MASK`.
    fn vpblendmw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMW256_MASK");
        }
    }
    /// Emits `VPBLENDMW256_MASKZ`.
    fn vpblendmw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMW256_MASKZ");
        }
    }
    /// Emits `VPBLENDMW512`.
    fn vpblendmw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMW512");
        }
    }
    /// Emits `VPBLENDMW512_MASK`.
    fn vpblendmw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMW512_MASK");
        }
    }
    /// Emits `VPBLENDMW512_MASKZ`.
    fn vpblendmw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPBLENDMW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPBLENDMW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBLENDMW512_MASKZ");
        }
    }
    /// Emits `VPBROADCASTB128`.
    fn vpbroadcastb128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTB128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTB128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTB128");
        }
    }
    /// Emits `VPBROADCASTB128_MASK`.
    fn vpbroadcastb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTB128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTB128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTB128_MASK");
        }
    }
    /// Emits `VPBROADCASTB128_MASKZ`.
    fn vpbroadcastb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTB128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTB128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTB128_MASKZ");
        }
    }
    /// Emits `VPBROADCASTB256`.
    fn vpbroadcastb256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTB256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTB256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTB256");
        }
    }
    /// Emits `VPBROADCASTB256_MASK`.
    fn vpbroadcastb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTB256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTB256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTB256_MASK");
        }
    }
    /// Emits `VPBROADCASTB256_MASKZ`.
    fn vpbroadcastb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTB256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTB256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTB256_MASKZ");
        }
    }
    /// Emits `VPBROADCASTB512`.
    fn vpbroadcastb512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTB512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTB512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTB512");
        }
    }
    /// Emits `VPBROADCASTB512_MASK`.
    fn vpbroadcastb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTB512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTB512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTB512_MASK");
        }
    }
    /// Emits `VPBROADCASTB512_MASKZ`.
    fn vpbroadcastb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTB512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTB512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTB512_MASKZ");
        }
    }
    /// Emits `VPBROADCASTB_GP128RR`.
    fn vpbroadcastb_gp128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTB_GP128RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTB_GP128RR_MASK`.
    fn vpbroadcastb_gp128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTB_GP128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTB_GP128RR_MASKZ`.
    fn vpbroadcastb_gp128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTB_GP128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTB_GP256RR`.
    fn vpbroadcastb_gp256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTB_GP256RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTB_GP256RR_MASK`.
    fn vpbroadcastb_gp256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTB_GP256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTB_GP256RR_MASKZ`.
    fn vpbroadcastb_gp256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTB_GP256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTB_GP512RR`.
    fn vpbroadcastb_gp512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTB_GP512RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTB_GP512RR_MASK`.
    fn vpbroadcastb_gp512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTB_GP512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTB_GP512RR_MASKZ`.
    fn vpbroadcastb_gp512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTB_GP512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTW128`.
    fn vpbroadcastw128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTW128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTW128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTW128");
        }
    }
    /// Emits `VPBROADCASTW128_MASK`.
    fn vpbroadcastw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTW128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTW128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTW128_MASK");
        }
    }
    /// Emits `VPBROADCASTW128_MASKZ`.
    fn vpbroadcastw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTW128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTW128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTW128_MASKZ");
        }
    }
    /// Emits `VPBROADCASTW256`.
    fn vpbroadcastw256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTW256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTW256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTW256");
        }
    }
    /// Emits `VPBROADCASTW256_MASK`.
    fn vpbroadcastw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTW256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTW256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTW256_MASK");
        }
    }
    /// Emits `VPBROADCASTW256_MASKZ`.
    fn vpbroadcastw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTW256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTW256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTW256_MASKZ");
        }
    }
    /// Emits `VPBROADCASTW512`.
    fn vpbroadcastw512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTW512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTW512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTW512");
        }
    }
    /// Emits `VPBROADCASTW512_MASK`.
    fn vpbroadcastw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTW512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTW512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTW512_MASK");
        }
    }
    /// Emits `VPBROADCASTW512_MASKZ`.
    fn vpbroadcastw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPBROADCASTW512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPBROADCASTW512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPBROADCASTW512_MASKZ");
        }
    }
    /// Emits `VPBROADCASTW_GP128RR`.
    fn vpbroadcastw_gp128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTW_GP128RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTW_GP128RR_MASK`.
    fn vpbroadcastw_gp128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTW_GP128RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTW_GP128RR_MASKZ`.
    fn vpbroadcastw_gp128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTW_GP128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTW_GP256RR`.
    fn vpbroadcastw_gp256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTW_GP256RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTW_GP256RR_MASK`.
    fn vpbroadcastw_gp256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTW_GP256RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTW_GP256RR_MASKZ`.
    fn vpbroadcastw_gp256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTW_GP256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTW_GP512RR`.
    fn vpbroadcastw_gp512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTW_GP512RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTW_GP512RR_MASK`.
    fn vpbroadcastw_gp512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTW_GP512RR_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTW_GP512RR_MASKZ`.
    fn vpbroadcastw_gp512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTW_GP512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCMPB128K`.
    fn vpcmpb128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPB128KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPB128KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPB128K");
        }
    }
    /// Emits `VPCMPB128K_MASK`.
    fn vpcmpb128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPB128KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPB128KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPB128K_MASK");
        }
    }
    /// Emits `VPCMPB256K`.
    fn vpcmpb256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPB256KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPB256KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPB256K");
        }
    }
    /// Emits `VPCMPB256K_MASK`.
    fn vpcmpb256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPB256KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPB256KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPB256K_MASK");
        }
    }
    /// Emits `VPCMPB512K`.
    fn vpcmpb512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPB512KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPB512KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPB512K");
        }
    }
    /// Emits `VPCMPB512K_MASK`.
    fn vpcmpb512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPB512KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPB512KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPB512K_MASK");
        }
    }
    /// Emits `VPCMPUB128K`.
    fn vpcmpub128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUB128KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUB128KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUB128K");
        }
    }
    /// Emits `VPCMPUB128K_MASK`.
    fn vpcmpub128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUB128KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUB128KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUB128K_MASK");
        }
    }
    /// Emits `VPCMPUB256K`.
    fn vpcmpub256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUB256KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUB256KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUB256K");
        }
    }
    /// Emits `VPCMPUB256K_MASK`.
    fn vpcmpub256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUB256KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUB256KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUB256K_MASK");
        }
    }
    /// Emits `VPCMPUB512K`.
    fn vpcmpub512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUB512KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUB512KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUB512K");
        }
    }
    /// Emits `VPCMPUB512K_MASK`.
    fn vpcmpub512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUB512KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUB512KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUB512K_MASK");
        }
    }
    /// Emits `VPCMPUW128K`.
    fn vpcmpuw128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUW128KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUW128KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUW128K");
        }
    }
    /// Emits `VPCMPUW128K_MASK`.
    fn vpcmpuw128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUW128KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUW128KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUW128K_MASK");
        }
    }
    /// Emits `VPCMPUW256K`.
    fn vpcmpuw256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUW256KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUW256KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUW256K");
        }
    }
    /// Emits `VPCMPUW256K_MASK`.
    fn vpcmpuw256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUW256KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUW256KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUW256K_MASK");
        }
    }
    /// Emits `VPCMPUW512K`.
    fn vpcmpuw512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUW512KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUW512KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUW512K");
        }
    }
    /// Emits `VPCMPUW512K_MASK`.
    fn vpcmpuw512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPUW512KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPUW512KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPUW512K_MASK");
        }
    }
    /// Emits `VPCMPW128K`.
    fn vpcmpw128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPW128KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPW128KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPW128K");
        }
    }
    /// Emits `VPCMPW128K_MASK`.
    fn vpcmpw128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPW128KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPW128KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPW128K_MASK");
        }
    }
    /// Emits `VPCMPW256K`.
    fn vpcmpw256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPW256KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPW256KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPW256K");
        }
    }
    /// Emits `VPCMPW256K_MASK`.
    fn vpcmpw256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPW256KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPW256KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPW256K_MASK");
        }
    }
    /// Emits `VPCMPW512K`.
    fn vpcmpw512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPW512KRRI, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPW512KRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPW512K");
        }
    }
    /// Emits `VPCMPW512K_MASK`.
    fn vpcmpw512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPCMPW512KRRI_MASK, op0,op1,op2,op3,);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPCMPW512KRMI_MASK, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPCMPW512K_MASK");
        }
    }
    /// Emits `VPERMI2W128`.
    fn vpermi2w128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2W128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2W128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2W128");
        }
    }
    /// Emits `VPERMI2W128_MASK`.
    fn vpermi2w128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2W128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2W128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2W128_MASK");
        }
    }
    /// Emits `VPERMI2W128_MASKZ`.
    fn vpermi2w128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2W128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2W128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2W128_MASKZ");
        }
    }
    /// Emits `VPERMI2W256`.
    fn vpermi2w256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2W256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2W256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2W256");
        }
    }
    /// Emits `VPERMI2W256_MASK`.
    fn vpermi2w256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2W256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2W256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2W256_MASK");
        }
    }
    /// Emits `VPERMI2W256_MASKZ`.
    fn vpermi2w256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2W256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2W256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2W256_MASKZ");
        }
    }
    /// Emits `VPERMI2W512`.
    fn vpermi2w512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2W512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2W512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2W512");
        }
    }
    /// Emits `VPERMI2W512_MASK`.
    fn vpermi2w512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2W512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2W512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2W512_MASK");
        }
    }
    /// Emits `VPERMI2W512_MASKZ`.
    fn vpermi2w512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2W512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2W512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2W512_MASKZ");
        }
    }
    /// Emits `VPERMT2W128`.
    fn vpermt2w128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2W128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2W128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2W128");
        }
    }
    /// Emits `VPERMT2W128_MASK`.
    fn vpermt2w128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2W128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2W128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2W128_MASK");
        }
    }
    /// Emits `VPERMT2W128_MASKZ`.
    fn vpermt2w128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2W128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2W128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2W128_MASKZ");
        }
    }
    /// Emits `VPERMT2W256`.
    fn vpermt2w256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2W256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2W256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2W256");
        }
    }
    /// Emits `VPERMT2W256_MASK`.
    fn vpermt2w256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2W256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2W256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2W256_MASK");
        }
    }
    /// Emits `VPERMT2W256_MASKZ`.
    fn vpermt2w256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2W256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2W256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2W256_MASKZ");
        }
    }
    /// Emits `VPERMT2W512`.
    fn vpermt2w512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2W512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2W512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2W512");
        }
    }
    /// Emits `VPERMT2W512_MASK`.
    fn vpermt2w512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2W512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2W512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2W512_MASK");
        }
    }
    /// Emits `VPERMT2W512_MASKZ`.
    fn vpermt2w512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2W512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2W512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2W512_MASKZ");
        }
    }
    /// Emits `VPERMW128`.
    fn vpermw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMW128");
        }
    }
    /// Emits `VPERMW128_MASK`.
    fn vpermw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMW128_MASK");
        }
    }
    /// Emits `VPERMW128_MASKZ`.
    fn vpermw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMW128_MASKZ");
        }
    }
    /// Emits `VPERMW256`.
    fn vpermw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMW256");
        }
    }
    /// Emits `VPERMW256_MASK`.
    fn vpermw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMW256_MASK");
        }
    }
    /// Emits `VPERMW256_MASKZ`.
    fn vpermw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMW256_MASKZ");
        }
    }
    /// Emits `VPERMW512`.
    fn vpermw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMW512");
        }
    }
    /// Emits `VPERMW512_MASK`.
    fn vpermw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMW512_MASK");
        }
    }
    /// Emits `VPERMW512_MASKZ`.
    fn vpermw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMW512_MASKZ");
        }
    }
    /// Emits `VPEXTRB`.
    fn vpextrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mem() && op1.is_vec() && op2.is_imm() {
            self.emit(VPEXTRBMRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_vec() && op2.is_imm() {
            self.emit(VPEXTRBRRI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXTRB");
        }
    }
    /// Emits `VPEXTRW`.
    fn vpextrw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_vec() && op2.is_imm() {
            self.emit(VPEXTRWRRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_imm() {
            self.emit(VPEXTRWMRI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPEXTRW");
        }
    }
    /// Emits `VPINSRB`.
    fn vpinsrb(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_gp() && op3.is_imm() {
            self.emit(VPINSRBRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPINSRBRRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPINSRB");
        }
    }
    /// Emits `VPINSRW`.
    fn vpinsrw(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_gp() && op3.is_imm() {
            self.emit(VPINSRWRRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPINSRWRRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPINSRW");
        }
    }
    /// Emits `VPMADDUBSW128`.
    fn vpmaddubsw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDUBSW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDUBSW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDUBSW128");
        }
    }
    /// Emits `VPMADDUBSW128_MASK`.
    fn vpmaddubsw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDUBSW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDUBSW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDUBSW128_MASK");
        }
    }
    /// Emits `VPMADDUBSW128_MASKZ`.
    fn vpmaddubsw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDUBSW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDUBSW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDUBSW128_MASKZ");
        }
    }
    /// Emits `VPMADDUBSW256`.
    fn vpmaddubsw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDUBSW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDUBSW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDUBSW256");
        }
    }
    /// Emits `VPMADDUBSW256_MASK`.
    fn vpmaddubsw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDUBSW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDUBSW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDUBSW256_MASK");
        }
    }
    /// Emits `VPMADDUBSW256_MASKZ`.
    fn vpmaddubsw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDUBSW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDUBSW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDUBSW256_MASKZ");
        }
    }
    /// Emits `VPMADDUBSW512`.
    fn vpmaddubsw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDUBSW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDUBSW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDUBSW512");
        }
    }
    /// Emits `VPMADDUBSW512_MASK`.
    fn vpmaddubsw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDUBSW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDUBSW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDUBSW512_MASK");
        }
    }
    /// Emits `VPMADDUBSW512_MASKZ`.
    fn vpmaddubsw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDUBSW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDUBSW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDUBSW512_MASKZ");
        }
    }
    /// Emits `VPMADDWD128`.
    fn vpmaddwd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDWD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDWD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDWD128");
        }
    }
    /// Emits `VPMADDWD128_MASK`.
    fn vpmaddwd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDWD128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDWD128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDWD128_MASK");
        }
    }
    /// Emits `VPMADDWD128_MASKZ`.
    fn vpmaddwd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDWD128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDWD128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDWD128_MASKZ");
        }
    }
    /// Emits `VPMADDWD256`.
    fn vpmaddwd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDWD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDWD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDWD256");
        }
    }
    /// Emits `VPMADDWD256_MASK`.
    fn vpmaddwd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDWD256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDWD256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDWD256_MASK");
        }
    }
    /// Emits `VPMADDWD256_MASKZ`.
    fn vpmaddwd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDWD256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDWD256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDWD256_MASKZ");
        }
    }
    /// Emits `VPMADDWD512`.
    fn vpmaddwd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDWD512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDWD512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDWD512");
        }
    }
    /// Emits `VPMADDWD512_MASK`.
    fn vpmaddwd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDWD512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDWD512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDWD512_MASK");
        }
    }
    /// Emits `VPMADDWD512_MASKZ`.
    fn vpmaddwd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMADDWD512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMADDWD512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMADDWD512_MASKZ");
        }
    }
    /// Emits `VPMAXSB128`.
    fn vpmaxsb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSB128");
        }
    }
    /// Emits `VPMAXSB128_MASK`.
    fn vpmaxsb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSB128_MASK");
        }
    }
    /// Emits `VPMAXSB128_MASKZ`.
    fn vpmaxsb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSB128_MASKZ");
        }
    }
    /// Emits `VPMAXSB256`.
    fn vpmaxsb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSB256");
        }
    }
    /// Emits `VPMAXSB256_MASK`.
    fn vpmaxsb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSB256_MASK");
        }
    }
    /// Emits `VPMAXSB256_MASKZ`.
    fn vpmaxsb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSB256_MASKZ");
        }
    }
    /// Emits `VPMAXSB512`.
    fn vpmaxsb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSB512");
        }
    }
    /// Emits `VPMAXSB512_MASK`.
    fn vpmaxsb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSB512_MASK");
        }
    }
    /// Emits `VPMAXSB512_MASKZ`.
    fn vpmaxsb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSB512_MASKZ");
        }
    }
    /// Emits `VPMAXSW128`.
    fn vpmaxsw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSW128");
        }
    }
    /// Emits `VPMAXSW128_MASK`.
    fn vpmaxsw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSW128_MASK");
        }
    }
    /// Emits `VPMAXSW128_MASKZ`.
    fn vpmaxsw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSW128_MASKZ");
        }
    }
    /// Emits `VPMAXSW256`.
    fn vpmaxsw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSW256");
        }
    }
    /// Emits `VPMAXSW256_MASK`.
    fn vpmaxsw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSW256_MASK");
        }
    }
    /// Emits `VPMAXSW256_MASKZ`.
    fn vpmaxsw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSW256_MASKZ");
        }
    }
    /// Emits `VPMAXSW512`.
    fn vpmaxsw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSW512");
        }
    }
    /// Emits `VPMAXSW512_MASK`.
    fn vpmaxsw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSW512_MASK");
        }
    }
    /// Emits `VPMAXSW512_MASKZ`.
    fn vpmaxsw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXSW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXSW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXSW512_MASKZ");
        }
    }
    /// Emits `VPMAXUB128`.
    fn vpmaxub128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUB128");
        }
    }
    /// Emits `VPMAXUB128_MASK`.
    fn vpmaxub128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUB128_MASK");
        }
    }
    /// Emits `VPMAXUB128_MASKZ`.
    fn vpmaxub128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUB128_MASKZ");
        }
    }
    /// Emits `VPMAXUB256`.
    fn vpmaxub256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUB256");
        }
    }
    /// Emits `VPMAXUB256_MASK`.
    fn vpmaxub256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUB256_MASK");
        }
    }
    /// Emits `VPMAXUB256_MASKZ`.
    fn vpmaxub256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUB256_MASKZ");
        }
    }
    /// Emits `VPMAXUB512`.
    fn vpmaxub512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUB512");
        }
    }
    /// Emits `VPMAXUB512_MASK`.
    fn vpmaxub512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUB512_MASK");
        }
    }
    /// Emits `VPMAXUB512_MASKZ`.
    fn vpmaxub512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUB512_MASKZ");
        }
    }
    /// Emits `VPMAXUW128`.
    fn vpmaxuw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUW128");
        }
    }
    /// Emits `VPMAXUW128_MASK`.
    fn vpmaxuw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUW128_MASK");
        }
    }
    /// Emits `VPMAXUW128_MASKZ`.
    fn vpmaxuw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUW128_MASKZ");
        }
    }
    /// Emits `VPMAXUW256`.
    fn vpmaxuw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUW256");
        }
    }
    /// Emits `VPMAXUW256_MASK`.
    fn vpmaxuw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUW256_MASK");
        }
    }
    /// Emits `VPMAXUW256_MASKZ`.
    fn vpmaxuw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUW256_MASKZ");
        }
    }
    /// Emits `VPMAXUW512`.
    fn vpmaxuw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUW512");
        }
    }
    /// Emits `VPMAXUW512_MASK`.
    fn vpmaxuw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUW512_MASK");
        }
    }
    /// Emits `VPMAXUW512_MASKZ`.
    fn vpmaxuw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMAXUW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMAXUW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMAXUW512_MASKZ");
        }
    }
    /// Emits `VPMINSB128`.
    fn vpminsb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSB128");
        }
    }
    /// Emits `VPMINSB128_MASK`.
    fn vpminsb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSB128_MASK");
        }
    }
    /// Emits `VPMINSB128_MASKZ`.
    fn vpminsb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSB128_MASKZ");
        }
    }
    /// Emits `VPMINSB256`.
    fn vpminsb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSB256");
        }
    }
    /// Emits `VPMINSB256_MASK`.
    fn vpminsb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSB256_MASK");
        }
    }
    /// Emits `VPMINSB256_MASKZ`.
    fn vpminsb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSB256_MASKZ");
        }
    }
    /// Emits `VPMINSB512`.
    fn vpminsb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSB512");
        }
    }
    /// Emits `VPMINSB512_MASK`.
    fn vpminsb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSB512_MASK");
        }
    }
    /// Emits `VPMINSB512_MASKZ`.
    fn vpminsb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSB512_MASKZ");
        }
    }
    /// Emits `VPMINSW128`.
    fn vpminsw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSW128");
        }
    }
    /// Emits `VPMINSW128_MASK`.
    fn vpminsw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSW128_MASK");
        }
    }
    /// Emits `VPMINSW128_MASKZ`.
    fn vpminsw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSW128_MASKZ");
        }
    }
    /// Emits `VPMINSW256`.
    fn vpminsw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSW256");
        }
    }
    /// Emits `VPMINSW256_MASK`.
    fn vpminsw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSW256_MASK");
        }
    }
    /// Emits `VPMINSW256_MASKZ`.
    fn vpminsw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSW256_MASKZ");
        }
    }
    /// Emits `VPMINSW512`.
    fn vpminsw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSW512");
        }
    }
    /// Emits `VPMINSW512_MASK`.
    fn vpminsw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSW512_MASK");
        }
    }
    /// Emits `VPMINSW512_MASKZ`.
    fn vpminsw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINSW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINSW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINSW512_MASKZ");
        }
    }
    /// Emits `VPMINUB128`.
    fn vpminub128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUB128");
        }
    }
    /// Emits `VPMINUB128_MASK`.
    fn vpminub128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUB128_MASK");
        }
    }
    /// Emits `VPMINUB128_MASKZ`.
    fn vpminub128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUB128_MASKZ");
        }
    }
    /// Emits `VPMINUB256`.
    fn vpminub256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUB256");
        }
    }
    /// Emits `VPMINUB256_MASK`.
    fn vpminub256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUB256_MASK");
        }
    }
    /// Emits `VPMINUB256_MASKZ`.
    fn vpminub256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUB256_MASKZ");
        }
    }
    /// Emits `VPMINUB512`.
    fn vpminub512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUB512");
        }
    }
    /// Emits `VPMINUB512_MASK`.
    fn vpminub512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUB512_MASK");
        }
    }
    /// Emits `VPMINUB512_MASKZ`.
    fn vpminub512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUB512_MASKZ");
        }
    }
    /// Emits `VPMINUW128`.
    fn vpminuw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUW128");
        }
    }
    /// Emits `VPMINUW128_MASK`.
    fn vpminuw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUW128_MASK");
        }
    }
    /// Emits `VPMINUW128_MASKZ`.
    fn vpminuw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUW128_MASKZ");
        }
    }
    /// Emits `VPMINUW256`.
    fn vpminuw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUW256");
        }
    }
    /// Emits `VPMINUW256_MASK`.
    fn vpminuw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUW256_MASK");
        }
    }
    /// Emits `VPMINUW256_MASKZ`.
    fn vpminuw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUW256_MASKZ");
        }
    }
    /// Emits `VPMINUW512`.
    fn vpminuw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUW512");
        }
    }
    /// Emits `VPMINUW512_MASK`.
    fn vpminuw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUW512_MASK");
        }
    }
    /// Emits `VPMINUW512_MASKZ`.
    fn vpminuw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMINUW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMINUW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMINUW512_MASKZ");
        }
    }
    /// Emits `VPMOVB2M128KR`.
    fn vpmovb2m128k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVB2M128KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVB2M256KR`.
    fn vpmovb2m256k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVB2M256KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVB2M512KR`.
    fn vpmovb2m512k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVB2M512KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVM2B128RK`.
    fn vpmovm2b128k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2B128RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVM2B256RK`.
    fn vpmovm2b256k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2B256RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVM2B512RK`.
    fn vpmovm2b512k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2B512RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVM2W128RK`.
    fn vpmovm2w128k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2W128RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVM2W256RK`.
    fn vpmovm2w256k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2W256RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVM2W512RK`.
    fn vpmovm2w512k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVM2W512RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVSWB128`.
    fn vpmovswb128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVSWB128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVSWB128MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVSWB128");
        }
    }
    /// Emits `VPMOVSWB128_MASK`.
    fn vpmovswb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVSWB128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVSWB128MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVSWB128_MASK");
        }
    }
    /// Emits `VPMOVSWB128RR_MASKZ`.
    fn vpmovswb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVSWB128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVSWB256`.
    fn vpmovswb256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVSWB256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVSWB256MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVSWB256");
        }
    }
    /// Emits `VPMOVSWB256_MASK`.
    fn vpmovswb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVSWB256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVSWB256MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVSWB256_MASK");
        }
    }
    /// Emits `VPMOVSWB256RR_MASKZ`.
    fn vpmovswb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVSWB256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVSWB512`.
    fn vpmovswb512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVSWB512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVSWB512MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVSWB512");
        }
    }
    /// Emits `VPMOVSWB512_MASK`.
    fn vpmovswb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVSWB512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVSWB512MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVSWB512_MASK");
        }
    }
    /// Emits `VPMOVSWB512RR_MASKZ`.
    fn vpmovswb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVSWB512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVUSWB128`.
    fn vpmovuswb128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVUSWB128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVUSWB128MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVUSWB128");
        }
    }
    /// Emits `VPMOVUSWB128_MASK`.
    fn vpmovuswb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVUSWB128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVUSWB128MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVUSWB128_MASK");
        }
    }
    /// Emits `VPMOVUSWB128RR_MASKZ`.
    fn vpmovuswb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVUSWB128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVUSWB256`.
    fn vpmovuswb256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVUSWB256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVUSWB256MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVUSWB256");
        }
    }
    /// Emits `VPMOVUSWB256_MASK`.
    fn vpmovuswb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVUSWB256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVUSWB256MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVUSWB256_MASK");
        }
    }
    /// Emits `VPMOVUSWB256RR_MASKZ`.
    fn vpmovuswb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVUSWB256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVUSWB512`.
    fn vpmovuswb512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVUSWB512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVUSWB512MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVUSWB512");
        }
    }
    /// Emits `VPMOVUSWB512_MASK`.
    fn vpmovuswb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVUSWB512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVUSWB512MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVUSWB512_MASK");
        }
    }
    /// Emits `VPMOVUSWB512RR_MASKZ`.
    fn vpmovuswb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVUSWB512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVW2M128KR`.
    fn vpmovw2m128k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVW2M128KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVW2M256KR`.
    fn vpmovw2m256k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVW2M256KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVW2M512KR`.
    fn vpmovw2m512k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVW2M512KR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVWB128`.
    fn vpmovwb128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVWB128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVWB128MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVWB128");
        }
    }
    /// Emits `VPMOVWB128_MASK`.
    fn vpmovwb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVWB128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVWB128MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVWB128_MASK");
        }
    }
    /// Emits `VPMOVWB128RR_MASKZ`.
    fn vpmovwb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVWB128RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVWB256`.
    fn vpmovwb256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVWB256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVWB256MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVWB256");
        }
    }
    /// Emits `VPMOVWB256_MASK`.
    fn vpmovwb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVWB256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVWB256MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVWB256_MASK");
        }
    }
    /// Emits `VPMOVWB256RR_MASKZ`.
    fn vpmovwb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVWB256RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMOVWB512`.
    fn vpmovwb512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVWB512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVWB512MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVWB512");
        }
    }
    /// Emits `VPMOVWB512_MASK`.
    fn vpmovwb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPMOVWB512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_vec() {
            self.emit(VPMOVWB512MR_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMOVWB512_MASK");
        }
    }
    /// Emits `VPMOVWB512RR_MASKZ`.
    fn vpmovwb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPMOVWB512RR_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPMULHRSW128`.
    fn vpmulhrsw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHRSW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHRSW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHRSW128");
        }
    }
    /// Emits `VPMULHRSW128_MASK`.
    fn vpmulhrsw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHRSW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHRSW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHRSW128_MASK");
        }
    }
    /// Emits `VPMULHRSW128_MASKZ`.
    fn vpmulhrsw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHRSW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHRSW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHRSW128_MASKZ");
        }
    }
    /// Emits `VPMULHRSW256`.
    fn vpmulhrsw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHRSW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHRSW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHRSW256");
        }
    }
    /// Emits `VPMULHRSW256_MASK`.
    fn vpmulhrsw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHRSW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHRSW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHRSW256_MASK");
        }
    }
    /// Emits `VPMULHRSW256_MASKZ`.
    fn vpmulhrsw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHRSW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHRSW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHRSW256_MASKZ");
        }
    }
    /// Emits `VPMULHRSW512`.
    fn vpmulhrsw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHRSW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHRSW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHRSW512");
        }
    }
    /// Emits `VPMULHRSW512_MASK`.
    fn vpmulhrsw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHRSW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHRSW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHRSW512_MASK");
        }
    }
    /// Emits `VPMULHRSW512_MASKZ`.
    fn vpmulhrsw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHRSW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHRSW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHRSW512_MASKZ");
        }
    }
    /// Emits `VPMULHUW128`.
    fn vpmulhuw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHUW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHUW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHUW128");
        }
    }
    /// Emits `VPMULHUW128_MASK`.
    fn vpmulhuw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHUW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHUW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHUW128_MASK");
        }
    }
    /// Emits `VPMULHUW128_MASKZ`.
    fn vpmulhuw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHUW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHUW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHUW128_MASKZ");
        }
    }
    /// Emits `VPMULHUW256`.
    fn vpmulhuw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHUW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHUW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHUW256");
        }
    }
    /// Emits `VPMULHUW256_MASK`.
    fn vpmulhuw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHUW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHUW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHUW256_MASK");
        }
    }
    /// Emits `VPMULHUW256_MASKZ`.
    fn vpmulhuw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHUW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHUW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHUW256_MASKZ");
        }
    }
    /// Emits `VPMULHUW512`.
    fn vpmulhuw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHUW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHUW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHUW512");
        }
    }
    /// Emits `VPMULHUW512_MASK`.
    fn vpmulhuw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHUW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHUW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHUW512_MASK");
        }
    }
    /// Emits `VPMULHUW512_MASKZ`.
    fn vpmulhuw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHUW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHUW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHUW512_MASKZ");
        }
    }
    /// Emits `VPMULHW128`.
    fn vpmulhw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHW128");
        }
    }
    /// Emits `VPMULHW128_MASK`.
    fn vpmulhw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHW128_MASK");
        }
    }
    /// Emits `VPMULHW128_MASKZ`.
    fn vpmulhw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHW128_MASKZ");
        }
    }
    /// Emits `VPMULHW256`.
    fn vpmulhw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHW256");
        }
    }
    /// Emits `VPMULHW256_MASK`.
    fn vpmulhw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHW256_MASK");
        }
    }
    /// Emits `VPMULHW256_MASKZ`.
    fn vpmulhw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHW256_MASKZ");
        }
    }
    /// Emits `VPMULHW512`.
    fn vpmulhw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHW512");
        }
    }
    /// Emits `VPMULHW512_MASK`.
    fn vpmulhw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHW512_MASK");
        }
    }
    /// Emits `VPMULHW512_MASKZ`.
    fn vpmulhw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULHW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULHW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULHW512_MASKZ");
        }
    }
    /// Emits `VPMULLW128`.
    fn vpmullw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULLW128");
        }
    }
    /// Emits `VPMULLW128_MASK`.
    fn vpmullw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULLW128_MASK");
        }
    }
    /// Emits `VPMULLW128_MASKZ`.
    fn vpmullw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULLW128_MASKZ");
        }
    }
    /// Emits `VPMULLW256`.
    fn vpmullw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULLW256");
        }
    }
    /// Emits `VPMULLW256_MASK`.
    fn vpmullw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULLW256_MASK");
        }
    }
    /// Emits `VPMULLW256_MASKZ`.
    fn vpmullw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULLW256_MASKZ");
        }
    }
    /// Emits `VPMULLW512`.
    fn vpmullw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULLW512");
        }
    }
    /// Emits `VPMULLW512_MASK`.
    fn vpmullw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULLW512_MASK");
        }
    }
    /// Emits `VPMULLW512_MASKZ`.
    fn vpmullw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULLW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULLW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULLW512_MASKZ");
        }
    }
    /// Emits `VPSADBW128`.
    fn vpsadbw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSADBW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSADBW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSADBW128");
        }
    }
    /// Emits `VPSADBW256`.
    fn vpsadbw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSADBW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSADBW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSADBW256");
        }
    }
    /// Emits `VPSADBW512`.
    fn vpsadbw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSADBW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSADBW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSADBW512");
        }
    }
    /// Emits `VPSHUFB128`.
    fn vpshufb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFB128");
        }
    }
    /// Emits `VPSHUFB128_MASK`.
    fn vpshufb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFB128_MASK");
        }
    }
    /// Emits `VPSHUFB128_MASKZ`.
    fn vpshufb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFB128_MASKZ");
        }
    }
    /// Emits `VPSHUFB256`.
    fn vpshufb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFB256");
        }
    }
    /// Emits `VPSHUFB256_MASK`.
    fn vpshufb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFB256_MASK");
        }
    }
    /// Emits `VPSHUFB256_MASKZ`.
    fn vpshufb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFB256_MASKZ");
        }
    }
    /// Emits `VPSHUFB512`.
    fn vpshufb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFB512");
        }
    }
    /// Emits `VPSHUFB512_MASK`.
    fn vpshufb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFB512_MASK");
        }
    }
    /// Emits `VPSHUFB512_MASKZ`.
    fn vpshufb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFB512_MASKZ");
        }
    }
    /// Emits `VPSHUFHW128`.
    fn vpshufhw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFHW128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFHW128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFHW128");
        }
    }
    /// Emits `VPSHUFHW128_MASK`.
    fn vpshufhw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFHW128RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFHW128RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFHW128_MASK");
        }
    }
    /// Emits `VPSHUFHW128_MASKZ`.
    fn vpshufhw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFHW128RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFHW128RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFHW128_MASKZ");
        }
    }
    /// Emits `VPSHUFHW256`.
    fn vpshufhw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFHW256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFHW256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFHW256");
        }
    }
    /// Emits `VPSHUFHW256_MASK`.
    fn vpshufhw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFHW256RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFHW256RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFHW256_MASK");
        }
    }
    /// Emits `VPSHUFHW256_MASKZ`.
    fn vpshufhw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFHW256RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFHW256RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFHW256_MASKZ");
        }
    }
    /// Emits `VPSHUFHW512`.
    fn vpshufhw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFHW512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFHW512RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFHW512");
        }
    }
    /// Emits `VPSHUFHW512_MASK`.
    fn vpshufhw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFHW512RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFHW512RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFHW512_MASK");
        }
    }
    /// Emits `VPSHUFHW512_MASKZ`.
    fn vpshufhw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFHW512RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFHW512RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFHW512_MASKZ");
        }
    }
    /// Emits `VPSHUFLW128`.
    fn vpshuflw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFLW128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFLW128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFLW128");
        }
    }
    /// Emits `VPSHUFLW128_MASK`.
    fn vpshuflw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFLW128RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFLW128RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFLW128_MASK");
        }
    }
    /// Emits `VPSHUFLW128_MASKZ`.
    fn vpshuflw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFLW128RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFLW128RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFLW128_MASKZ");
        }
    }
    /// Emits `VPSHUFLW256`.
    fn vpshuflw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFLW256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFLW256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFLW256");
        }
    }
    /// Emits `VPSHUFLW256_MASK`.
    fn vpshuflw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFLW256RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFLW256RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFLW256_MASK");
        }
    }
    /// Emits `VPSHUFLW256_MASKZ`.
    fn vpshuflw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFLW256RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFLW256RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFLW256_MASKZ");
        }
    }
    /// Emits `VPSHUFLW512`.
    fn vpshuflw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFLW512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFLW512RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFLW512");
        }
    }
    /// Emits `VPSHUFLW512_MASK`.
    fn vpshuflw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFLW512RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFLW512RMI_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFLW512_MASK");
        }
    }
    /// Emits `VPSHUFLW512_MASKZ`.
    fn vpshuflw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSHUFLW512RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSHUFLW512RMI_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFLW512_MASKZ");
        }
    }
    /// Emits `VPSLLDQ128`.
    fn vpslldq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLDQ128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLDQ128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLDQ128");
        }
    }
    /// Emits `VPSLLDQ256`.
    fn vpslldq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLDQ256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLDQ256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLDQ256");
        }
    }
    /// Emits `VPSLLDQ512`.
    fn vpslldq512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLDQ512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLDQ512RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLDQ512");
        }
    }
    /// Emits `VPSLLVW128`.
    fn vpsllvw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLVW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLVW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLVW128");
        }
    }
    /// Emits `VPSLLVW128_MASK`.
    fn vpsllvw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLVW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLVW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLVW128_MASK");
        }
    }
    /// Emits `VPSLLVW128_MASKZ`.
    fn vpsllvw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLVW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLVW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLVW128_MASKZ");
        }
    }
    /// Emits `VPSLLVW256`.
    fn vpsllvw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLVW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLVW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLVW256");
        }
    }
    /// Emits `VPSLLVW256_MASK`.
    fn vpsllvw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLVW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLVW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLVW256_MASK");
        }
    }
    /// Emits `VPSLLVW256_MASKZ`.
    fn vpsllvw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLVW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLVW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLVW256_MASKZ");
        }
    }
    /// Emits `VPSLLVW512`.
    fn vpsllvw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLVW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLVW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLVW512");
        }
    }
    /// Emits `VPSLLVW512_MASK`.
    fn vpsllvw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLVW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLVW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLVW512_MASK");
        }
    }
    /// Emits `VPSLLVW512_MASKZ`.
    fn vpsllvw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLVW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLVW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLVW512_MASKZ");
        }
    }
    /// Emits `VPSLLW128`.
    fn vpsllw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLW128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLW128RRM, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLW128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLW128");
        }
    }
    /// Emits `VPSLLW128_MASK`.
    fn vpsllw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLW128RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLW128RMI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLW128_MASK");
        }
    }
    /// Emits `VPSLLW128_MASKZ`.
    fn vpsllw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLW128RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLW128RMI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLW128_MASKZ");
        }
    }
    /// Emits `VPSLLW256`.
    fn vpsllw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLW256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLW256RRM, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLW256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLW256");
        }
    }
    /// Emits `VPSLLW256_MASK`.
    fn vpsllw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLW256RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLW256RMI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLW256_MASK");
        }
    }
    /// Emits `VPSLLW256_MASKZ`.
    fn vpsllw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLW256RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLW256RMI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLW256_MASKZ");
        }
    }
    /// Emits `VPSLLW512`.
    fn vpsllw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLW512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLW512RMI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLW512");
        }
    }
    /// Emits `VPSLLW512_MASK`.
    fn vpsllw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLW512RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLW512RMI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLW512_MASK");
        }
    }
    /// Emits `VPSLLW512_MASKZ`.
    fn vpsllw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSLLW512RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSLLW512RMI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSLLW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSLLW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSLLW512_MASKZ");
        }
    }
    /// Emits `VPSRAVW128`.
    fn vpsravw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAVW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAVW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAVW128");
        }
    }
    /// Emits `VPSRAVW128_MASK`.
    fn vpsravw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAVW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAVW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAVW128_MASK");
        }
    }
    /// Emits `VPSRAVW128_MASKZ`.
    fn vpsravw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAVW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAVW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAVW128_MASKZ");
        }
    }
    /// Emits `VPSRAVW256`.
    fn vpsravw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAVW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAVW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAVW256");
        }
    }
    /// Emits `VPSRAVW256_MASK`.
    fn vpsravw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAVW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAVW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAVW256_MASK");
        }
    }
    /// Emits `VPSRAVW256_MASKZ`.
    fn vpsravw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAVW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAVW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAVW256_MASKZ");
        }
    }
    /// Emits `VPSRAVW512`.
    fn vpsravw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAVW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAVW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAVW512");
        }
    }
    /// Emits `VPSRAVW512_MASK`.
    fn vpsravw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAVW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAVW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAVW512_MASK");
        }
    }
    /// Emits `VPSRAVW512_MASKZ`.
    fn vpsravw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAVW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAVW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAVW512_MASKZ");
        }
    }
    /// Emits `VPSRAW128`.
    fn vpsraw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRAW128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAW128RRM, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRAW128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAW128");
        }
    }
    /// Emits `VPSRAW128_MASK`.
    fn vpsraw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRAW128RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRAW128RMI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAW128_MASK");
        }
    }
    /// Emits `VPSRAW128_MASKZ`.
    fn vpsraw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRAW128RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRAW128RMI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAW128_MASKZ");
        }
    }
    /// Emits `VPSRAW256`.
    fn vpsraw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRAW256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAW256RRM, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRAW256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAW256");
        }
    }
    /// Emits `VPSRAW256_MASK`.
    fn vpsraw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRAW256RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRAW256RMI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAW256_MASK");
        }
    }
    /// Emits `VPSRAW256_MASKZ`.
    fn vpsraw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRAW256RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRAW256RMI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAW256_MASKZ");
        }
    }
    /// Emits `VPSRAW512`.
    fn vpsraw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRAW512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRAW512RMI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAW512");
        }
    }
    /// Emits `VPSRAW512_MASK`.
    fn vpsraw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRAW512RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRAW512RMI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAW512_MASK");
        }
    }
    /// Emits `VPSRAW512_MASKZ`.
    fn vpsraw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRAW512RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRAW512RMI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRAW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRAW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRAW512_MASKZ");
        }
    }
    /// Emits `VPSRLDQ128`.
    fn vpsrldq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLDQ128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLDQ128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLDQ128");
        }
    }
    /// Emits `VPSRLDQ256`.
    fn vpsrldq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLDQ256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLDQ256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLDQ256");
        }
    }
    /// Emits `VPSRLDQ512`.
    fn vpsrldq512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLDQ512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLDQ512RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLDQ512");
        }
    }
    /// Emits `VPSRLVW128`.
    fn vpsrlvw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLVW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLVW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLVW128");
        }
    }
    /// Emits `VPSRLVW128_MASK`.
    fn vpsrlvw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLVW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLVW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLVW128_MASK");
        }
    }
    /// Emits `VPSRLVW128_MASKZ`.
    fn vpsrlvw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLVW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLVW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLVW128_MASKZ");
        }
    }
    /// Emits `VPSRLVW256`.
    fn vpsrlvw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLVW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLVW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLVW256");
        }
    }
    /// Emits `VPSRLVW256_MASK`.
    fn vpsrlvw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLVW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLVW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLVW256_MASK");
        }
    }
    /// Emits `VPSRLVW256_MASKZ`.
    fn vpsrlvw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLVW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLVW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLVW256_MASKZ");
        }
    }
    /// Emits `VPSRLVW512`.
    fn vpsrlvw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLVW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLVW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLVW512");
        }
    }
    /// Emits `VPSRLVW512_MASK`.
    fn vpsrlvw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLVW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLVW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLVW512_MASK");
        }
    }
    /// Emits `VPSRLVW512_MASKZ`.
    fn vpsrlvw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLVW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLVW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLVW512_MASKZ");
        }
    }
    /// Emits `VPSRLW128`.
    fn vpsrlw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLW128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLW128RRM, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLW128RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLW128");
        }
    }
    /// Emits `VPSRLW128_MASK`.
    fn vpsrlw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLW128RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLW128RMI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLW128_MASK");
        }
    }
    /// Emits `VPSRLW128_MASKZ`.
    fn vpsrlw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLW128RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLW128RMI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLW128_MASKZ");
        }
    }
    /// Emits `VPSRLW256`.
    fn vpsrlw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLW256RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLW256RRM, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLW256RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLW256");
        }
    }
    /// Emits `VPSRLW256_MASK`.
    fn vpsrlw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLW256RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLW256RMI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLW256_MASK");
        }
    }
    /// Emits `VPSRLW256_MASKZ`.
    fn vpsrlw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLW256RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLW256RMI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLW256_MASKZ");
        }
    }
    /// Emits `VPSRLW512`.
    fn vpsrlw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLW512RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLW512RMI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLW512");
        }
    }
    /// Emits `VPSRLW512_MASK`.
    fn vpsrlw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLW512RRI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLW512RMI_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLW512_MASK");
        }
    }
    /// Emits `VPSRLW512_MASKZ`.
    fn vpsrlw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VPSRLW512RRI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(VPSRLW512RMI_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSRLW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSRLW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSRLW512_MASKZ");
        }
    }
    /// Emits `VPSUBB128`.
    fn vpsubb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBB128");
        }
    }
    /// Emits `VPSUBB128_MASK`.
    fn vpsubb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBB128_MASK");
        }
    }
    /// Emits `VPSUBB128_MASKZ`.
    fn vpsubb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBB128_MASKZ");
        }
    }
    /// Emits `VPSUBB256`.
    fn vpsubb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBB256");
        }
    }
    /// Emits `VPSUBB256_MASK`.
    fn vpsubb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBB256_MASK");
        }
    }
    /// Emits `VPSUBB256_MASKZ`.
    fn vpsubb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBB256_MASKZ");
        }
    }
    /// Emits `VPSUBB512`.
    fn vpsubb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBB512");
        }
    }
    /// Emits `VPSUBB512_MASK`.
    fn vpsubb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBB512_MASK");
        }
    }
    /// Emits `VPSUBB512_MASKZ`.
    fn vpsubb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBB512_MASKZ");
        }
    }
    /// Emits `VPSUBSB128`.
    fn vpsubsb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSB128");
        }
    }
    /// Emits `VPSUBSB128_MASK`.
    fn vpsubsb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSB128_MASK");
        }
    }
    /// Emits `VPSUBSB128_MASKZ`.
    fn vpsubsb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSB128_MASKZ");
        }
    }
    /// Emits `VPSUBSB256`.
    fn vpsubsb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSB256");
        }
    }
    /// Emits `VPSUBSB256_MASK`.
    fn vpsubsb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSB256_MASK");
        }
    }
    /// Emits `VPSUBSB256_MASKZ`.
    fn vpsubsb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSB256_MASKZ");
        }
    }
    /// Emits `VPSUBSB512`.
    fn vpsubsb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSB512");
        }
    }
    /// Emits `VPSUBSB512_MASK`.
    fn vpsubsb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSB512_MASK");
        }
    }
    /// Emits `VPSUBSB512_MASKZ`.
    fn vpsubsb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSB512_MASKZ");
        }
    }
    /// Emits `VPSUBSW128`.
    fn vpsubsw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSW128");
        }
    }
    /// Emits `VPSUBSW128_MASK`.
    fn vpsubsw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSW128_MASK");
        }
    }
    /// Emits `VPSUBSW128_MASKZ`.
    fn vpsubsw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSW128_MASKZ");
        }
    }
    /// Emits `VPSUBSW256`.
    fn vpsubsw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSW256");
        }
    }
    /// Emits `VPSUBSW256_MASK`.
    fn vpsubsw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSW256_MASK");
        }
    }
    /// Emits `VPSUBSW256_MASKZ`.
    fn vpsubsw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSW256_MASKZ");
        }
    }
    /// Emits `VPSUBSW512`.
    fn vpsubsw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSW512");
        }
    }
    /// Emits `VPSUBSW512_MASK`.
    fn vpsubsw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSW512_MASK");
        }
    }
    /// Emits `VPSUBSW512_MASKZ`.
    fn vpsubsw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBSW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBSW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBSW512_MASKZ");
        }
    }
    /// Emits `VPSUBUSB128`.
    fn vpsubusb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSB128");
        }
    }
    /// Emits `VPSUBUSB128_MASK`.
    fn vpsubusb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSB128_MASK");
        }
    }
    /// Emits `VPSUBUSB128_MASKZ`.
    fn vpsubusb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSB128_MASKZ");
        }
    }
    /// Emits `VPSUBUSB256`.
    fn vpsubusb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSB256");
        }
    }
    /// Emits `VPSUBUSB256_MASK`.
    fn vpsubusb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSB256_MASK");
        }
    }
    /// Emits `VPSUBUSB256_MASKZ`.
    fn vpsubusb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSB256_MASKZ");
        }
    }
    /// Emits `VPSUBUSB512`.
    fn vpsubusb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSB512");
        }
    }
    /// Emits `VPSUBUSB512_MASK`.
    fn vpsubusb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSB512_MASK");
        }
    }
    /// Emits `VPSUBUSB512_MASKZ`.
    fn vpsubusb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSB512_MASKZ");
        }
    }
    /// Emits `VPSUBUSW128`.
    fn vpsubusw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSW128");
        }
    }
    /// Emits `VPSUBUSW128_MASK`.
    fn vpsubusw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSW128_MASK");
        }
    }
    /// Emits `VPSUBUSW128_MASKZ`.
    fn vpsubusw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSW128_MASKZ");
        }
    }
    /// Emits `VPSUBUSW256`.
    fn vpsubusw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSW256");
        }
    }
    /// Emits `VPSUBUSW256_MASK`.
    fn vpsubusw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSW256_MASK");
        }
    }
    /// Emits `VPSUBUSW256_MASKZ`.
    fn vpsubusw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSW256_MASKZ");
        }
    }
    /// Emits `VPSUBUSW512`.
    fn vpsubusw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSW512");
        }
    }
    /// Emits `VPSUBUSW512_MASK`.
    fn vpsubusw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSW512_MASK");
        }
    }
    /// Emits `VPSUBUSW512_MASKZ`.
    fn vpsubusw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBUSW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBUSW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBUSW512_MASKZ");
        }
    }
    /// Emits `VPSUBW128`.
    fn vpsubw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBW128");
        }
    }
    /// Emits `VPSUBW128_MASK`.
    fn vpsubw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBW128_MASK");
        }
    }
    /// Emits `VPSUBW128_MASKZ`.
    fn vpsubw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBW128_MASKZ");
        }
    }
    /// Emits `VPSUBW256`.
    fn vpsubw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBW256");
        }
    }
    /// Emits `VPSUBW256_MASK`.
    fn vpsubw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBW256_MASK");
        }
    }
    /// Emits `VPSUBW256_MASKZ`.
    fn vpsubw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBW256_MASKZ");
        }
    }
    /// Emits `VPSUBW512`.
    fn vpsubw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBW512");
        }
    }
    /// Emits `VPSUBW512_MASK`.
    fn vpsubw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBW512_MASK");
        }
    }
    /// Emits `VPSUBW512_MASKZ`.
    fn vpsubw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSUBW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSUBW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSUBW512_MASKZ");
        }
    }
    /// Emits `VPTESTMB128K`.
    fn vptestmb128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMB128KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMB128KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMB128K");
        }
    }
    /// Emits `VPTESTMB128K_MASK`.
    fn vptestmb128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMB128KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMB128KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMB128K_MASK");
        }
    }
    /// Emits `VPTESTMB256K`.
    fn vptestmb256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMB256KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMB256KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMB256K");
        }
    }
    /// Emits `VPTESTMB256K_MASK`.
    fn vptestmb256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMB256KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMB256KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMB256K_MASK");
        }
    }
    /// Emits `VPTESTMB512K`.
    fn vptestmb512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMB512KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMB512KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMB512K");
        }
    }
    /// Emits `VPTESTMB512K_MASK`.
    fn vptestmb512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMB512KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMB512KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMB512K_MASK");
        }
    }
    /// Emits `VPTESTMW128K`.
    fn vptestmw128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMW128KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMW128KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMW128K");
        }
    }
    /// Emits `VPTESTMW128K_MASK`.
    fn vptestmw128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMW128KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMW128KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMW128K_MASK");
        }
    }
    /// Emits `VPTESTMW256K`.
    fn vptestmw256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMW256KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMW256KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMW256K");
        }
    }
    /// Emits `VPTESTMW256K_MASK`.
    fn vptestmw256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMW256KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMW256KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMW256K_MASK");
        }
    }
    /// Emits `VPTESTMW512K`.
    fn vptestmw512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMW512KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMW512KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMW512K");
        }
    }
    /// Emits `VPTESTMW512K_MASK`.
    fn vptestmw512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTMW512KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTMW512KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTMW512K_MASK");
        }
    }
    /// Emits `VPTESTNMB128K`.
    fn vptestnmb128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMB128KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMB128KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMB128K");
        }
    }
    /// Emits `VPTESTNMB128K_MASK`.
    fn vptestnmb128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMB128KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMB128KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMB128K_MASK");
        }
    }
    /// Emits `VPTESTNMB256K`.
    fn vptestnmb256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMB256KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMB256KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMB256K");
        }
    }
    /// Emits `VPTESTNMB256K_MASK`.
    fn vptestnmb256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMB256KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMB256KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMB256K_MASK");
        }
    }
    /// Emits `VPTESTNMB512K`.
    fn vptestnmb512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMB512KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMB512KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMB512K");
        }
    }
    /// Emits `VPTESTNMB512K_MASK`.
    fn vptestnmb512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMB512KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMB512KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMB512K_MASK");
        }
    }
    /// Emits `VPTESTNMW128K`.
    fn vptestnmw128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMW128KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMW128KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMW128K");
        }
    }
    /// Emits `VPTESTNMW128K_MASK`.
    fn vptestnmw128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMW128KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMW128KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMW128K_MASK");
        }
    }
    /// Emits `VPTESTNMW256K`.
    fn vptestnmw256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMW256KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMW256KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMW256K");
        }
    }
    /// Emits `VPTESTNMW256K_MASK`.
    fn vptestnmw256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMW256KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMW256KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMW256K_MASK");
        }
    }
    /// Emits `VPTESTNMW512K`.
    fn vptestnmw512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMW512KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMW512KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMW512K");
        }
    }
    /// Emits `VPTESTNMW512K_MASK`.
    fn vptestnmw512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPTESTNMW512KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPTESTNMW512KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPTESTNMW512K_MASK");
        }
    }
    /// Emits `VPUNPCKHBW128`.
    fn vpunpckhbw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHBW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHBW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHBW128");
        }
    }
    /// Emits `VPUNPCKHBW128_MASK`.
    fn vpunpckhbw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHBW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHBW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHBW128_MASK");
        }
    }
    /// Emits `VPUNPCKHBW128_MASKZ`.
    fn vpunpckhbw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHBW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHBW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHBW128_MASKZ");
        }
    }
    /// Emits `VPUNPCKHBW256`.
    fn vpunpckhbw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHBW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHBW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHBW256");
        }
    }
    /// Emits `VPUNPCKHBW256_MASK`.
    fn vpunpckhbw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHBW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHBW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHBW256_MASK");
        }
    }
    /// Emits `VPUNPCKHBW256_MASKZ`.
    fn vpunpckhbw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHBW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHBW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHBW256_MASKZ");
        }
    }
    /// Emits `VPUNPCKHBW512`.
    fn vpunpckhbw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHBW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHBW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHBW512");
        }
    }
    /// Emits `VPUNPCKHBW512_MASK`.
    fn vpunpckhbw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHBW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHBW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHBW512_MASK");
        }
    }
    /// Emits `VPUNPCKHBW512_MASKZ`.
    fn vpunpckhbw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHBW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHBW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHBW512_MASKZ");
        }
    }
    /// Emits `VPUNPCKHWD128`.
    fn vpunpckhwd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHWD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHWD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHWD128");
        }
    }
    /// Emits `VPUNPCKHWD128_MASK`.
    fn vpunpckhwd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHWD128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHWD128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHWD128_MASK");
        }
    }
    /// Emits `VPUNPCKHWD128_MASKZ`.
    fn vpunpckhwd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHWD128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHWD128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHWD128_MASKZ");
        }
    }
    /// Emits `VPUNPCKHWD256`.
    fn vpunpckhwd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHWD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHWD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHWD256");
        }
    }
    /// Emits `VPUNPCKHWD256_MASK`.
    fn vpunpckhwd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHWD256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHWD256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHWD256_MASK");
        }
    }
    /// Emits `VPUNPCKHWD256_MASKZ`.
    fn vpunpckhwd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHWD256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHWD256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHWD256_MASKZ");
        }
    }
    /// Emits `VPUNPCKHWD512`.
    fn vpunpckhwd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHWD512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHWD512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHWD512");
        }
    }
    /// Emits `VPUNPCKHWD512_MASK`.
    fn vpunpckhwd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHWD512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHWD512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHWD512_MASK");
        }
    }
    /// Emits `VPUNPCKHWD512_MASKZ`.
    fn vpunpckhwd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKHWD512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKHWD512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKHWD512_MASKZ");
        }
    }
    /// Emits `VPUNPCKLBW128`.
    fn vpunpcklbw128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLBW128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLBW128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLBW128");
        }
    }
    /// Emits `VPUNPCKLBW128_MASK`.
    fn vpunpcklbw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLBW128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLBW128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLBW128_MASK");
        }
    }
    /// Emits `VPUNPCKLBW128_MASKZ`.
    fn vpunpcklbw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLBW128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLBW128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLBW128_MASKZ");
        }
    }
    /// Emits `VPUNPCKLBW256`.
    fn vpunpcklbw256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLBW256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLBW256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLBW256");
        }
    }
    /// Emits `VPUNPCKLBW256_MASK`.
    fn vpunpcklbw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLBW256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLBW256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLBW256_MASK");
        }
    }
    /// Emits `VPUNPCKLBW256_MASKZ`.
    fn vpunpcklbw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLBW256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLBW256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLBW256_MASKZ");
        }
    }
    /// Emits `VPUNPCKLBW512`.
    fn vpunpcklbw512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLBW512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLBW512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLBW512");
        }
    }
    /// Emits `VPUNPCKLBW512_MASK`.
    fn vpunpcklbw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLBW512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLBW512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLBW512_MASK");
        }
    }
    /// Emits `VPUNPCKLBW512_MASKZ`.
    fn vpunpcklbw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLBW512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLBW512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLBW512_MASKZ");
        }
    }
    /// Emits `VPUNPCKLWD128`.
    fn vpunpcklwd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLWD128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLWD128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLWD128");
        }
    }
    /// Emits `VPUNPCKLWD128_MASK`.
    fn vpunpcklwd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLWD128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLWD128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLWD128_MASK");
        }
    }
    /// Emits `VPUNPCKLWD128_MASKZ`.
    fn vpunpcklwd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLWD128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLWD128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLWD128_MASKZ");
        }
    }
    /// Emits `VPUNPCKLWD256`.
    fn vpunpcklwd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLWD256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLWD256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLWD256");
        }
    }
    /// Emits `VPUNPCKLWD256_MASK`.
    fn vpunpcklwd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLWD256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLWD256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLWD256_MASK");
        }
    }
    /// Emits `VPUNPCKLWD256_MASKZ`.
    fn vpunpcklwd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLWD256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLWD256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLWD256_MASKZ");
        }
    }
    /// Emits `VPUNPCKLWD512`.
    fn vpunpcklwd512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLWD512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLWD512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLWD512");
        }
    }
    /// Emits `VPUNPCKLWD512_MASK`.
    fn vpunpcklwd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLWD512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLWD512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLWD512_MASK");
        }
    }
    /// Emits `VPUNPCKLWD512_MASKZ`.
    fn vpunpcklwd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPUNPCKLWD512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPUNPCKLWD512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPUNPCKLWD512_MASKZ");
        }
    }
}
