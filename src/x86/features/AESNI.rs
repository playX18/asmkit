pub trait X86AESNIEmitter: Emitter {
    /// Emits `AESDEC`.
    fn aesdec(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(AESDECRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(AESDECRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for AESDEC");
        }
    }
    /// Emits `AESDECLAST`.
    fn aesdeclast(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(AESDECLASTRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(AESDECLASTRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for AESDECLAST");
        }
    }
    /// Emits `AESENC`.
    fn aesenc(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(AESENCRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(AESENCRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for AESENC");
        }
    }
    /// Emits `AESENCLAST`.
    fn aesenclast(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(AESENCLASTRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(AESENCLASTRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for AESENCLAST");
        }
    }
    /// Emits `AESIMC`.
    fn aesimc(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(AESIMCRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(AESIMCRM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for AESIMC");
        }
    }
    /// Emits `AESKEYGENASSIST`.
    fn aeskeygenassist(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(AESKEYGENASSISTRRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(AESKEYGENASSISTRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for AESKEYGENASSIST");
        }
    }
}
