pub trait X86PTWRITEEmitter: Emitter {
    /// Emits `PTWRITE32`.
    fn ptwrite32(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(PTWRITE32R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(PTWRITE32M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for PTWRITE32");
        }
    }
    /// Emits `PTWRITE64`.
    fn ptwrite64(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(PTWRITE64R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(PTWRITE64M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for PTWRITE64");
        }
    }
}
