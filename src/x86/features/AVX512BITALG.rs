pub trait X86AVX512BITALGEmitter: Emitter {
    /// Emits `VPOPCNTB128`.
    fn vpopcntb128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTB128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTB128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTB128");
        }
    }
    /// Emits `VPOPCNTB128_MASK`.
    fn vpopcntb128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTB128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTB128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTB128_MASK");
        }
    }
    /// Emits `VPOPCNTB128_MASKZ`.
    fn vpopcntb128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTB128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTB128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTB128_MASKZ");
        }
    }
    /// Emits `VPOPCNTB256`.
    fn vpopcntb256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTB256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTB256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTB256");
        }
    }
    /// Emits `VPOPCNTB256_MASK`.
    fn vpopcntb256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTB256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTB256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTB256_MASK");
        }
    }
    /// Emits `VPOPCNTB256_MASKZ`.
    fn vpopcntb256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTB256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTB256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTB256_MASKZ");
        }
    }
    /// Emits `VPOPCNTB512`.
    fn vpopcntb512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTB512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTB512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTB512");
        }
    }
    /// Emits `VPOPCNTB512_MASK`.
    fn vpopcntb512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTB512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTB512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTB512_MASK");
        }
    }
    /// Emits `VPOPCNTB512_MASKZ`.
    fn vpopcntb512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTB512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTB512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTB512_MASKZ");
        }
    }
    /// Emits `VPOPCNTW128`.
    fn vpopcntw128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTW128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTW128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTW128");
        }
    }
    /// Emits `VPOPCNTW128_MASK`.
    fn vpopcntw128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTW128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTW128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTW128_MASK");
        }
    }
    /// Emits `VPOPCNTW128_MASKZ`.
    fn vpopcntw128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTW128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTW128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTW128_MASKZ");
        }
    }
    /// Emits `VPOPCNTW256`.
    fn vpopcntw256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTW256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTW256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTW256");
        }
    }
    /// Emits `VPOPCNTW256_MASK`.
    fn vpopcntw256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTW256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTW256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTW256_MASK");
        }
    }
    /// Emits `VPOPCNTW256_MASKZ`.
    fn vpopcntw256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTW256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTW256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTW256_MASKZ");
        }
    }
    /// Emits `VPOPCNTW512`.
    fn vpopcntw512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTW512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTW512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTW512");
        }
    }
    /// Emits `VPOPCNTW512_MASK`.
    fn vpopcntw512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTW512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTW512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTW512_MASK");
        }
    }
    /// Emits `VPOPCNTW512_MASKZ`.
    fn vpopcntw512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTW512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTW512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTW512_MASKZ");
        }
    }
    /// Emits `VPSHUFBITQMB128K`.
    fn vpshufbitqmb128k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFBITQMB128KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFBITQMB128KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFBITQMB128K");
        }
    }
    /// Emits `VPSHUFBITQMB128K_MASK`.
    fn vpshufbitqmb128k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFBITQMB128KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFBITQMB128KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFBITQMB128K_MASK");
        }
    }
    /// Emits `VPSHUFBITQMB256K`.
    fn vpshufbitqmb256k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFBITQMB256KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFBITQMB256KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFBITQMB256K");
        }
    }
    /// Emits `VPSHUFBITQMB256K_MASK`.
    fn vpshufbitqmb256k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFBITQMB256KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFBITQMB256KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFBITQMB256K_MASK");
        }
    }
    /// Emits `VPSHUFBITQMB512K`.
    fn vpshufbitqmb512k(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFBITQMB512KRR, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFBITQMB512KRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFBITQMB512K");
        }
    }
    /// Emits `VPSHUFBITQMB512K_MASK`.
    fn vpshufbitqmb512k_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_mask() && op1.is_vec() && op2.is_vec() {
            self.emit(VPSHUFBITQMB512KRR_MASK, op0,op1,op2,&NOREG);
        } else if op0.is_mask() && op1.is_vec() && op2.is_mem() {
            self.emit(VPSHUFBITQMB512KRM_MASK, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPSHUFBITQMB512K_MASK");
        }
    }
}
