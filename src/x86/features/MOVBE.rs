pub trait X86MOVBEEmitter: Emitter {
    /// Emits `MOVBE16`.
    fn movbe16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_mem() {
            self.emit(MOVBE16RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(MOVBE16MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOVBE16");
        }
    }
    /// Emits `MOVBE32`.
    fn movbe32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_mem() {
            self.emit(MOVBE32RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(MOVBE32MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOVBE32");
        }
    }
    /// Emits `MOVBE64`.
    fn movbe64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_mem() {
            self.emit(MOVBE64RM, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(MOVBE64MR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for MOVBE64");
        }
    }
}
