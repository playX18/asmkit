pub trait X86AVX512CDEmitter: Emitter {
    /// Emits `VPBROADCASTMB2Q128RK`.
    fn vpbroadcastmb2q128k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMB2Q128RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTMB2Q256RK`.
    fn vpbroadcastmb2q256k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMB2Q256RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTMB2Q512RK`.
    fn vpbroadcastmb2q512k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMB2Q512RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTMW2D128RK`.
    fn vpbroadcastmw2d128k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMW2D128RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTMW2D256RK`.
    fn vpbroadcastmw2d256k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMW2D256RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPBROADCASTMW2D512RK`.
    fn vpbroadcastmw2d512k(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPBROADCASTMW2D512RK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTD128`.
    fn vpconflictd128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTD128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTD128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTD128");
        }
    }
    /// Emits `VPCONFLICTD128_MASK`.
    fn vpconflictd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTD128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTD128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTD128_MASK");
        }
    }
    /// Emits `VPCONFLICTD128_MASKZ`.
    fn vpconflictd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTD128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTD128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTD128_MASKZ");
        }
    }
    /// Emits `VPCONFLICTD128RB`.
    fn vpconflictd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTD128RB_MASK`.
    fn vpconflictd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTD128RB_MASKZ`.
    fn vpconflictd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTD256`.
    fn vpconflictd256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTD256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTD256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTD256");
        }
    }
    /// Emits `VPCONFLICTD256_MASK`.
    fn vpconflictd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTD256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTD256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTD256_MASK");
        }
    }
    /// Emits `VPCONFLICTD256_MASKZ`.
    fn vpconflictd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTD256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTD256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTD256_MASKZ");
        }
    }
    /// Emits `VPCONFLICTD256RB`.
    fn vpconflictd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTD256RB_MASK`.
    fn vpconflictd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTD256RB_MASKZ`.
    fn vpconflictd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTD512`.
    fn vpconflictd512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTD512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTD512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTD512");
        }
    }
    /// Emits `VPCONFLICTD512_MASK`.
    fn vpconflictd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTD512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTD512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTD512_MASK");
        }
    }
    /// Emits `VPCONFLICTD512_MASKZ`.
    fn vpconflictd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTD512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTD512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTD512_MASKZ");
        }
    }
    /// Emits `VPCONFLICTD512RB`.
    fn vpconflictd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTD512RB_MASK`.
    fn vpconflictd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTD512RB_MASKZ`.
    fn vpconflictd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTD512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTQ128`.
    fn vpconflictq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTQ128");
        }
    }
    /// Emits `VPCONFLICTQ128_MASK`.
    fn vpconflictq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTQ128_MASK");
        }
    }
    /// Emits `VPCONFLICTQ128_MASKZ`.
    fn vpconflictq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTQ128_MASKZ");
        }
    }
    /// Emits `VPCONFLICTQ128RB`.
    fn vpconflictq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTQ128RB_MASK`.
    fn vpconflictq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTQ128RB_MASKZ`.
    fn vpconflictq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTQ256`.
    fn vpconflictq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTQ256");
        }
    }
    /// Emits `VPCONFLICTQ256_MASK`.
    fn vpconflictq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTQ256_MASK");
        }
    }
    /// Emits `VPCONFLICTQ256_MASKZ`.
    fn vpconflictq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTQ256_MASKZ");
        }
    }
    /// Emits `VPCONFLICTQ256RB`.
    fn vpconflictq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTQ256RB_MASK`.
    fn vpconflictq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTQ256RB_MASKZ`.
    fn vpconflictq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTQ512`.
    fn vpconflictq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTQ512");
        }
    }
    /// Emits `VPCONFLICTQ512_MASK`.
    fn vpconflictq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTQ512_MASK");
        }
    }
    /// Emits `VPCONFLICTQ512_MASKZ`.
    fn vpconflictq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPCONFLICTQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPCONFLICTQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPCONFLICTQ512_MASKZ");
        }
    }
    /// Emits `VPCONFLICTQ512RB`.
    fn vpconflictq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTQ512RB_MASK`.
    fn vpconflictq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPCONFLICTQ512RB_MASKZ`.
    fn vpconflictq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPCONFLICTQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTD128`.
    fn vplzcntd128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTD128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTD128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTD128");
        }
    }
    /// Emits `VPLZCNTD128_MASK`.
    fn vplzcntd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTD128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTD128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTD128_MASK");
        }
    }
    /// Emits `VPLZCNTD128_MASKZ`.
    fn vplzcntd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTD128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTD128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTD128_MASKZ");
        }
    }
    /// Emits `VPLZCNTD128RB`.
    fn vplzcntd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTD128RB_MASK`.
    fn vplzcntd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTD128RB_MASKZ`.
    fn vplzcntd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTD256`.
    fn vplzcntd256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTD256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTD256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTD256");
        }
    }
    /// Emits `VPLZCNTD256_MASK`.
    fn vplzcntd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTD256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTD256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTD256_MASK");
        }
    }
    /// Emits `VPLZCNTD256_MASKZ`.
    fn vplzcntd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTD256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTD256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTD256_MASKZ");
        }
    }
    /// Emits `VPLZCNTD256RB`.
    fn vplzcntd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTD256RB_MASK`.
    fn vplzcntd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTD256RB_MASKZ`.
    fn vplzcntd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTD512`.
    fn vplzcntd512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTD512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTD512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTD512");
        }
    }
    /// Emits `VPLZCNTD512_MASK`.
    fn vplzcntd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTD512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTD512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTD512_MASK");
        }
    }
    /// Emits `VPLZCNTD512_MASKZ`.
    fn vplzcntd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTD512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTD512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTD512_MASKZ");
        }
    }
    /// Emits `VPLZCNTD512RB`.
    fn vplzcntd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTD512RB_MASK`.
    fn vplzcntd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTD512RB_MASKZ`.
    fn vplzcntd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTD512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTQ128`.
    fn vplzcntq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTQ128");
        }
    }
    /// Emits `VPLZCNTQ128_MASK`.
    fn vplzcntq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTQ128_MASK");
        }
    }
    /// Emits `VPLZCNTQ128_MASKZ`.
    fn vplzcntq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTQ128_MASKZ");
        }
    }
    /// Emits `VPLZCNTQ128RB`.
    fn vplzcntq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTQ128RB_MASK`.
    fn vplzcntq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTQ128RB_MASKZ`.
    fn vplzcntq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTQ256`.
    fn vplzcntq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTQ256");
        }
    }
    /// Emits `VPLZCNTQ256_MASK`.
    fn vplzcntq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTQ256_MASK");
        }
    }
    /// Emits `VPLZCNTQ256_MASKZ`.
    fn vplzcntq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTQ256_MASKZ");
        }
    }
    /// Emits `VPLZCNTQ256RB`.
    fn vplzcntq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTQ256RB_MASK`.
    fn vplzcntq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTQ256RB_MASKZ`.
    fn vplzcntq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTQ512`.
    fn vplzcntq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTQ512");
        }
    }
    /// Emits `VPLZCNTQ512_MASK`.
    fn vplzcntq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTQ512_MASK");
        }
    }
    /// Emits `VPLZCNTQ512_MASKZ`.
    fn vplzcntq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPLZCNTQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPLZCNTQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPLZCNTQ512_MASKZ");
        }
    }
    /// Emits `VPLZCNTQ512RB`.
    fn vplzcntq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTQ512RB_MASK`.
    fn vplzcntq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPLZCNTQ512RB_MASKZ`.
    fn vplzcntq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPLZCNTQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
}
