pub trait X86AVX2Emitter: Emitter {
    /// Emits `VBROADCASTI128RM`.
    fn vbroadcasti128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(VBROADCASTI128RM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VEXTRACTI128`.
    fn vextracti128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(VEXTRACTI128RRI, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_imm() {
            self.emit(VEXTRACTI128MRI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VEXTRACTI128");
        }
    }
    /// Emits `VGATHERDPD128RMR`.
    fn vgatherdpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERDPD128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGATHERDPD256RMR`.
    fn vgatherdpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERDPD256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGATHERDPS128RMR`.
    fn vgatherdps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERDPS128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGATHERDPS256RMR`.
    fn vgatherdps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERDPS256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGATHERQPD128RMR`.
    fn vgatherqpd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERQPD128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGATHERQPD256RMR`.
    fn vgatherqpd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERQPD256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGATHERQPS128RMR`.
    fn vgatherqps128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERQPS128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VGATHERQPS256RMR`.
    fn vgatherqps256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VGATHERQPS256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VINSERTI128`.
    fn vinserti128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VINSERTI128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VINSERTI128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VINSERTI128");
        }
    }
    /// Emits `VPBLENDD128`.
    fn vpblendd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPBLENDD128RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPBLENDD128RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPBLENDD128");
        }
    }
    /// Emits `VPBLENDD256`.
    fn vpblendd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPBLENDD256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPBLENDD256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPBLENDD256");
        }
    }
    /// Emits `VPERM2I128_256`.
    fn vperm2i128_256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast,op3: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        let op3 = op3.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() && op3.is_imm() {
            self.emit(VPERM2I128_256RRRI, op0,op1,op2,op3,);
        } else if op0.is_vec() && op1.is_vec() && op2.is_mem() && op3.is_imm() {
            self.emit(VPERM2I128_256RRMI, op0,op1,op2,op3,);
        } else {
            unreachable!("invalid operand types for VPERM2I128_256");
        }
    }
    /// Emits `VPGATHERDD128RMR`.
    fn vpgatherdd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERDD128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPGATHERDD256RMR`.
    fn vpgatherdd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERDD256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPGATHERDQ128RMR`.
    fn vpgatherdq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERDQ128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPGATHERDQ256RMR`.
    fn vpgatherdq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERDQ256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPGATHERQD128RMR`.
    fn vpgatherqd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERQD128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPGATHERQD256RMR`.
    fn vpgatherqd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERQD256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPGATHERQQ128RMR`.
    fn vpgatherqq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERQQ128RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPGATHERQQ256RMR`.
    fn vpgatherqq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(VPGATHERQQ256RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG);
    }
    /// Emits `VPMASKMOVD128`.
    fn vpmaskmovd128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMASKMOVD128RRM, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMASKMOVD128MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMASKMOVD128");
        }
    }
    /// Emits `VPMASKMOVD256`.
    fn vpmaskmovd256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMASKMOVD256RRM, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMASKMOVD256MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMASKMOVD256");
        }
    }
    /// Emits `VPMASKMOVQ128`.
    fn vpmaskmovq128(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMASKMOVQ128RRM, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMASKMOVQ128MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMASKMOVQ128");
        }
    }
    /// Emits `VPMASKMOVQ256`.
    fn vpmaskmovq256(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_mem() {
            self.emit(VPMASKMOVQ256RRM, op0,op1,op2,&NOREG);
        } else if op0.is_mem() && op1.is_vec() && op2.is_vec() {
            self.emit(VPMASKMOVQ256MRR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for VPMASKMOVQ256");
        }
    }
}
