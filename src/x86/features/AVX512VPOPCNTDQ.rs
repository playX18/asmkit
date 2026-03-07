pub trait X86AVX512VPOPCNTDQEmitter: Emitter {
    /// Emits `VPOPCNTD128`.
    fn vpopcntd128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTD128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTD128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTD128");
        }
    }
    /// Emits `VPOPCNTD128_MASK`.
    fn vpopcntd128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTD128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTD128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTD128_MASK");
        }
    }
    /// Emits `VPOPCNTD128_MASKZ`.
    fn vpopcntd128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTD128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTD128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTD128_MASKZ");
        }
    }
    /// Emits `VPOPCNTD128RB`.
    fn vpopcntd128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTD128RB_MASK`.
    fn vpopcntd128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTD128RB_MASKZ`.
    fn vpopcntd128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTD256`.
    fn vpopcntd256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTD256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTD256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTD256");
        }
    }
    /// Emits `VPOPCNTD256_MASK`.
    fn vpopcntd256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTD256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTD256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTD256_MASK");
        }
    }
    /// Emits `VPOPCNTD256_MASKZ`.
    fn vpopcntd256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTD256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTD256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTD256_MASKZ");
        }
    }
    /// Emits `VPOPCNTD256RB`.
    fn vpopcntd256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTD256RB_MASK`.
    fn vpopcntd256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTD256RB_MASKZ`.
    fn vpopcntd256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTD512`.
    fn vpopcntd512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTD512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTD512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTD512");
        }
    }
    /// Emits `VPOPCNTD512_MASK`.
    fn vpopcntd512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTD512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTD512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTD512_MASK");
        }
    }
    /// Emits `VPOPCNTD512_MASKZ`.
    fn vpopcntd512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTD512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTD512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTD512_MASKZ");
        }
    }
    /// Emits `VPOPCNTD512RB`.
    fn vpopcntd512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTD512RB_MASK`.
    fn vpopcntd512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTD512RB_MASKZ`.
    fn vpopcntd512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTD512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTQ128`.
    fn vpopcntq128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTQ128RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTQ128RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTQ128");
        }
    }
    /// Emits `VPOPCNTQ128_MASK`.
    fn vpopcntq128_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTQ128RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTQ128RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTQ128_MASK");
        }
    }
    /// Emits `VPOPCNTQ128_MASKZ`.
    fn vpopcntq128_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTQ128RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTQ128RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTQ128_MASKZ");
        }
    }
    /// Emits `VPOPCNTQ128RB`.
    fn vpopcntq128b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTQ128RB_MASK`.
    fn vpopcntq128b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTQ128RB_MASKZ`.
    fn vpopcntq128b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ128RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTQ256`.
    fn vpopcntq256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTQ256RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTQ256RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTQ256");
        }
    }
    /// Emits `VPOPCNTQ256_MASK`.
    fn vpopcntq256_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTQ256RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTQ256RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTQ256_MASK");
        }
    }
    /// Emits `VPOPCNTQ256_MASKZ`.
    fn vpopcntq256_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTQ256RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTQ256RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTQ256_MASKZ");
        }
    }
    /// Emits `VPOPCNTQ256RB`.
    fn vpopcntq256b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTQ256RB_MASK`.
    fn vpopcntq256b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTQ256RB_MASKZ`.
    fn vpopcntq256b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ256RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTQ512`.
    fn vpopcntq512(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTQ512RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTQ512RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTQ512");
        }
    }
    /// Emits `VPOPCNTQ512_MASK`.
    fn vpopcntq512_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTQ512RR_MASK, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTQ512RM_MASK, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTQ512_MASK");
        }
    }
    /// Emits `VPOPCNTQ512_MASKZ`.
    fn vpopcntq512_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(VPOPCNTQ512RR_MASKZ, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(VPOPCNTQ512RM_MASKZ, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VPOPCNTQ512_MASKZ");
        }
    }
    /// Emits `VPOPCNTQ512RB`.
    fn vpopcntq512b(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RB, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTQ512RB_MASK`.
    fn vpopcntq512b_mask(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RB_MASK, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VPOPCNTQ512RB_MASKZ`.
    fn vpopcntq512b_maskz(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VPOPCNTQ512RB_MASKZ, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
}
