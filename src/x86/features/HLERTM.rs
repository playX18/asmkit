pub trait X86HLERTMEmitter: Emitter {
    /// Emits `XABORTI`.
    fn xabort(&mut self,op0: impl OperandCast) -> () {
        self.emit(XABORTI, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XBEGIN`.
    fn xbegin(&mut self,op0: impl OperandCast) -> () {
        self.emit(XBEGIN, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XEND`.
    fn xend(&mut self,) -> () {
        self.emit(XEND, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `XTEST`.
    fn xtest(&mut self,) -> () {
        self.emit(XTEST, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
