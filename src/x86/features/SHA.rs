pub trait X86SHAEmitter: Emitter {
    /// Emits `SHA1MSG1`.
    fn sha1msg1(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SHA1MSG1RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SHA1MSG1RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHA1MSG1");
        }
    }
    /// Emits `SHA1MSG2`.
    fn sha1msg2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SHA1MSG2RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SHA1MSG2RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHA1MSG2");
        }
    }
    /// Emits `SHA1NEXTE`.
    fn sha1nexte(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SHA1NEXTERR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SHA1NEXTERM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHA1NEXTE");
        }
    }
    /// Emits `SHA1RNDS4`.
    fn sha1rnds4(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_imm() {
            self.emit(SHA1RNDS4RRI, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_imm() {
            self.emit(SHA1RNDS4RMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHA1RNDS4");
        }
    }
    /// Emits `SHA256MSG1`.
    fn sha256msg1(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SHA256MSG1RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SHA256MSG1RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHA256MSG1");
        }
    }
    /// Emits `SHA256MSG2`.
    fn sha256msg2(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_vec() && op1.is_vec() {
            self.emit(SHA256MSG2RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_vec() && op1.is_mem() {
            self.emit(SHA256MSG2RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for SHA256MSG2");
        }
    }
    /// Emits `SHA256RNDS2`.
    fn sha256rnds2(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_vec() && op1.is_vec() && op2.is_vec() {
            self.emit(SHA256RNDS2RRR, op0,op1,op2,&NOREG);
        } else if op0.is_vec() && op1.is_mem() && op2.is_vec() {
            self.emit(SHA256RNDS2RMR, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for SHA256RNDS2");
        }
    }
}
