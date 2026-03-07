pub trait X86FSGSBASEEmitter: Emitter {
    /// Emits `RDFSBASE32R`.
    fn rdfsbase32(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDFSBASE32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RDFSBASE64R`.
    fn rdfsbase64(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDFSBASE64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RDGSBASE32R`.
    fn rdgsbase32(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDGSBASE32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RDGSBASE64R`.
    fn rdgsbase64(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDGSBASE64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `WRFSBASE32R`.
    fn wrfsbase32(&mut self,op0: impl OperandCast) -> () {
        self.emit(WRFSBASE32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `WRFSBASE64R`.
    fn wrfsbase64(&mut self,op0: impl OperandCast) -> () {
        self.emit(WRFSBASE64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `WRGSBASE32R`.
    fn wrgsbase32(&mut self,op0: impl OperandCast) -> () {
        self.emit(WRGSBASE32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `WRGSBASE64R`.
    fn wrgsbase64(&mut self,op0: impl OperandCast) -> () {
        self.emit(WRGSBASE64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
