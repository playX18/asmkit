pub trait X86RDSEEDEmitter: Emitter {
    /// Emits `RDSEED16R`.
    fn rdseed16(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDSEED16R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RDSEED32R`.
    fn rdseed32(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDSEED32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RDSEED64R`.
    fn rdseed64(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDSEED64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
