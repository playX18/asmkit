pub trait X86BMI1Emitter: Emitter {
    /// Emits `ANDN32`.
    fn andn32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(ANDN32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_mem() {
            self.emit(ANDN32RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for ANDN32");
        }
    }
    /// Emits `ANDN64`.
    fn andn64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(ANDN64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_mem() {
            self.emit(ANDN64RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for ANDN64");
        }
    }
    /// Emits `BEXTR32`.
    fn bextr32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(BEXTR32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_gp() {
            self.emit(BEXTR32RMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for BEXTR32");
        }
    }
    /// Emits `BEXTR64`.
    fn bextr64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(BEXTR64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_gp() {
            self.emit(BEXTR64RMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for BEXTR64");
        }
    }
    /// Emits `BLSI32`.
    fn blsi32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BLSI32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BLSI32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BLSI32");
        }
    }
    /// Emits `BLSI64`.
    fn blsi64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BLSI64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BLSI64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BLSI64");
        }
    }
    /// Emits `BLSMSK32`.
    fn blsmsk32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BLSMSK32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BLSMSK32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BLSMSK32");
        }
    }
    /// Emits `BLSMSK64`.
    fn blsmsk64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BLSMSK64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BLSMSK64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BLSMSK64");
        }
    }
    /// Emits `BLSR32`.
    fn blsr32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BLSR32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BLSR32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BLSR32");
        }
    }
    /// Emits `BLSR64`.
    fn blsr64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(BLSR64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(BLSR64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for BLSR64");
        }
    }
    /// Emits `TZCNT16`.
    fn tzcnt16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(TZCNT16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(TZCNT16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for TZCNT16");
        }
    }
    /// Emits `TZCNT32`.
    fn tzcnt32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(TZCNT32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(TZCNT32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for TZCNT32");
        }
    }
    /// Emits `TZCNT64`.
    fn tzcnt64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(TZCNT64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(TZCNT64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for TZCNT64");
        }
    }
}
