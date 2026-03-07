pub trait X86CETEmitter: Emitter {
    /// Emits `CLRSSBSYM`.
    fn clrssbsy(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLRSSBSYM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `ENDBR32`.
    fn endbr32(&mut self,) -> () {
        self.emit(ENDBR32, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `ENDBR64`.
    fn endbr64(&mut self,) -> () {
        self.emit(ENDBR64, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `INCSSP32R`.
    fn incssp32(&mut self,op0: impl OperandCast) -> () {
        self.emit(INCSSP32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `INCSSP64R`.
    fn incssp64(&mut self,op0: impl OperandCast) -> () {
        self.emit(INCSSP64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RDSSP32R`.
    fn rdssp32(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDSSP32R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RDSSP64R`.
    fn rdssp64(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDSSP64R, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RSTORSSPM`.
    fn rstorssp(&mut self,op0: impl OperandCast) -> () {
        self.emit(RSTORSSPM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SAVEPREVSSP`.
    fn saveprevssp(&mut self,) -> () {
        self.emit(SAVEPREVSSP, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SETSSBSY`.
    fn setssbsy(&mut self,) -> () {
        self.emit(SETSSBSY, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `WRSS32MR`.
    fn wrss32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(WRSS32MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `WRSS64MR`.
    fn wrss64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(WRSS64MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `WRUSS32MR`.
    fn wruss32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(WRUSS32MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `WRUSS64MR`.
    fn wruss64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(WRUSS64MR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
}
