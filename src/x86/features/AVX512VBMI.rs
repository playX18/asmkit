pub trait X86AVX512VBMIEmitter: Emitter {
    /// Emits `VPERMB128`.
    fn vpermb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMB128");
        }
    }
    /// Emits `VPERMB128_MASK`.
    fn vpermb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMB128_MASK");
        }
    }
    /// Emits `VPERMB128_MASKZ`.
    fn vpermb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMB128_MASKZ");
        }
    }
    /// Emits `VPERMB256`.
    fn vpermb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMB256");
        }
    }
    /// Emits `VPERMB256_MASK`.
    fn vpermb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMB256_MASK");
        }
    }
    /// Emits `VPERMB256_MASKZ`.
    fn vpermb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMB256_MASKZ");
        }
    }
    /// Emits `VPERMB512`.
    fn vpermb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMB512");
        }
    }
    /// Emits `VPERMB512_MASK`.
    fn vpermb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMB512_MASK");
        }
    }
    /// Emits `VPERMB512_MASKZ`.
    fn vpermb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMB512_MASKZ");
        }
    }
    /// Emits `VPERMI2B128`.
    fn vpermi2b128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2B128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2B128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2B128");
        }
    }
    /// Emits `VPERMI2B128_MASK`.
    fn vpermi2b128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2B128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2B128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2B128_MASK");
        }
    }
    /// Emits `VPERMI2B128_MASKZ`.
    fn vpermi2b128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2B128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2B128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2B128_MASKZ");
        }
    }
    /// Emits `VPERMI2B256`.
    fn vpermi2b256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2B256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2B256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2B256");
        }
    }
    /// Emits `VPERMI2B256_MASK`.
    fn vpermi2b256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2B256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2B256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2B256_MASK");
        }
    }
    /// Emits `VPERMI2B256_MASKZ`.
    fn vpermi2b256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2B256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2B256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2B256_MASKZ");
        }
    }
    /// Emits `VPERMI2B512`.
    fn vpermi2b512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2B512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2B512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2B512");
        }
    }
    /// Emits `VPERMI2B512_MASK`.
    fn vpermi2b512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2B512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2B512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2B512_MASK");
        }
    }
    /// Emits `VPERMI2B512_MASKZ`.
    fn vpermi2b512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMI2B512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMI2B512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMI2B512_MASKZ");
        }
    }
    /// Emits `VPERMT2B128`.
    fn vpermt2b128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2B128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2B128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2B128");
        }
    }
    /// Emits `VPERMT2B128_MASK`.
    fn vpermt2b128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2B128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2B128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2B128_MASK");
        }
    }
    /// Emits `VPERMT2B128_MASKZ`.
    fn vpermt2b128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2B128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2B128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2B128_MASKZ");
        }
    }
    /// Emits `VPERMT2B256`.
    fn vpermt2b256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2B256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2B256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2B256");
        }
    }
    /// Emits `VPERMT2B256_MASK`.
    fn vpermt2b256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2B256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2B256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2B256_MASK");
        }
    }
    /// Emits `VPERMT2B256_MASKZ`.
    fn vpermt2b256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2B256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2B256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2B256_MASKZ");
        }
    }
    /// Emits `VPERMT2B512`.
    fn vpermt2b512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2B512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2B512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2B512");
        }
    }
    /// Emits `VPERMT2B512_MASK`.
    fn vpermt2b512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2B512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2B512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2B512_MASK");
        }
    }
    /// Emits `VPERMT2B512_MASKZ`.
    fn vpermt2b512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPERMT2B512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPERMT2B512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPERMT2B512_MASKZ");
        }
    }
    /// Emits `VPMULTISHIFTQB128`.
    fn vpmultishiftqb128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULTISHIFTQB128RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULTISHIFTQB128RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULTISHIFTQB128");
        }
    }
    /// Emits `VPMULTISHIFTQB128_MASK`.
    fn vpmultishiftqb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULTISHIFTQB128RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULTISHIFTQB128RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULTISHIFTQB128_MASK");
        }
    }
    /// Emits `VPMULTISHIFTQB128_MASKZ`.
    fn vpmultishiftqb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULTISHIFTQB128RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULTISHIFTQB128RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULTISHIFTQB128_MASKZ");
        }
    }
    /// Emits `VPMULTISHIFTQB128RRB`.
    fn vpmultishiftqb128b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPMULTISHIFTQB128RRB_MASK`.
    fn vpmultishiftqb128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPMULTISHIFTQB128RRB_MASKZ`.
    fn vpmultishiftqb128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB128RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPMULTISHIFTQB256`.
    fn vpmultishiftqb256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULTISHIFTQB256RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULTISHIFTQB256RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULTISHIFTQB256");
        }
    }
    /// Emits `VPMULTISHIFTQB256_MASK`.
    fn vpmultishiftqb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULTISHIFTQB256RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULTISHIFTQB256RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULTISHIFTQB256_MASK");
        }
    }
    /// Emits `VPMULTISHIFTQB256_MASKZ`.
    fn vpmultishiftqb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULTISHIFTQB256RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULTISHIFTQB256RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULTISHIFTQB256_MASKZ");
        }
    }
    /// Emits `VPMULTISHIFTQB256RRB`.
    fn vpmultishiftqb256b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPMULTISHIFTQB256RRB_MASK`.
    fn vpmultishiftqb256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPMULTISHIFTQB256RRB_MASKZ`.
    fn vpmultishiftqb256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB256RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPMULTISHIFTQB512`.
    fn vpmultishiftqb512(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULTISHIFTQB512RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULTISHIFTQB512RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULTISHIFTQB512");
        }
    }
    /// Emits `VPMULTISHIFTQB512_MASK`.
    fn vpmultishiftqb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULTISHIFTQB512RRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULTISHIFTQB512RRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULTISHIFTQB512_MASK");
        }
    }
    /// Emits `VPMULTISHIFTQB512_MASKZ`.
    fn vpmultishiftqb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMULTISHIFTQB512RRR_MASKZ, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMULTISHIFTQB512RRM_MASKZ, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMULTISHIFTQB512_MASKZ");
        }
    }
    /// Emits `VPMULTISHIFTQB512RRB`.
    fn vpmultishiftqb512b(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRB, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPMULTISHIFTQB512RRB_MASK`.
    fn vpmultishiftqb512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRB_MASK, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPMULTISHIFTQB512RRB_MASKZ`.
    fn vpmultishiftqb512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPMULTISHIFTQB512RRB_MASKZ, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
}
