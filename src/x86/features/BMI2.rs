pub trait X86BMI2Emitter: Emitter {
    /// Emits `BZHI32`.
    fn bzhi32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(BZHI32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_gp() {
            self.emit(BZHI32RMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for BZHI32");
        }
    }
    /// Emits `BZHI64`.
    fn bzhi64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(BZHI64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_gp() {
            self.emit(BZHI64RMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for BZHI64");
        }
    }
    /// Emits `MULX32`.
    fn mulx32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(MULX32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_mem() {
            self.emit(MULX32RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for MULX32");
        }
    }
    /// Emits `MULX64`.
    fn mulx64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(MULX64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_mem() {
            self.emit(MULX64RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for MULX64");
        }
    }
    /// Emits `PDEP32`.
    fn pdep32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(PDEP32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_mem() {
            self.emit(PDEP32RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for PDEP32");
        }
    }
    /// Emits `PDEP64`.
    fn pdep64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(PDEP64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_mem() {
            self.emit(PDEP64RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for PDEP64");
        }
    }
    /// Emits `PEXT32`.
    fn pext32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(PEXT32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_mem() {
            self.emit(PEXT32RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for PEXT32");
        }
    }
    /// Emits `PEXT64`.
    fn pext64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(PEXT64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_gp() && op2.is_mem() {
            self.emit(PEXT64RRM, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for PEXT64");
        }
    }
    /// Emits `RORX32`.
    fn rorx32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_imm() {
            self.emit(RORX32RRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_imm() {
            self.emit(RORX32RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for RORX32");
        }
    }
    /// Emits `RORX64`.
    fn rorx64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_imm() {
            self.emit(RORX64RRI, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_imm() {
            self.emit(RORX64RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for RORX64");
        }
    }
    /// Emits `SARX32`.
    fn sarx32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SARX32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_gp() {
            self.emit(SARX32RMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SARX32");
        }
    }
    /// Emits `SARX64`.
    fn sarx64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SARX64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_gp() {
            self.emit(SARX64RMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SARX64");
        }
    }
    /// Emits `SHLX32`.
    fn shlx32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SHLX32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_gp() {
            self.emit(SHLX32RMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHLX32");
        }
    }
    /// Emits `SHLX64`.
    fn shlx64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SHLX64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_gp() {
            self.emit(SHLX64RMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHLX64");
        }
    }
    /// Emits `SHRX32`.
    fn shrx32(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SHRX32RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_gp() {
            self.emit(SHRX32RMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHRX32");
        }
    }
    /// Emits `SHRX64`.
    fn shrx64(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_gp() && op1.is_gp() && op2.is_gp() {
            self.emit(SHRX64RRR, op0,op1,op2,&NOREG);
        } else if op0.is_gp() && op1.is_mem() && op2.is_gp() {
            self.emit(SHRX64RMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHRX64");
        }
    }
}
