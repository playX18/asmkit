pub trait X86LZCNTEmitter: Emitter {
    /// Emits `LZCNT16`.
    fn lzcnt16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(LZCNT16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(LZCNT16RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LZCNT16");
        }
    }
    /// Emits `LZCNT32`.
    fn lzcnt32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(LZCNT32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(LZCNT32RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LZCNT32");
        }
    }
    /// Emits `LZCNT64`.
    fn lzcnt64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(LZCNT64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(LZCNT64RM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LZCNT64");
        }
    }
}
