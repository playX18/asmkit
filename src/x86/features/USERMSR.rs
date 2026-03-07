pub trait X86USERMSREmitter: Emitter {
    /// Emits `URDMSR`.
    fn urdmsr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(URDMSRRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_imm() {
            self.emit(URDMSRRI, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for URDMSR");
        }
    }
    /// Emits `UWRMSR`.
    fn uwrmsr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(UWRMSRRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_imm() && op1.is_gp() {
            self.emit(UWRMSRIR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for UWRMSR");
        }
    }
}
