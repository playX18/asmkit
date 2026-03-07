pub trait X86UINTREmitter: Emitter {
    /// Emits `CLUI`.
    fn clui(&mut self,) -> () {
        self.emit(CLUI, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `SENDUIPIR`.
    fn senduipi(&mut self,op0: impl OperandCast) -> () {
        self.emit(SENDUIPIR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `STUI`.
    fn stui(&mut self,) -> () {
        self.emit(STUI, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `TESTUI`.
    fn testui(&mut self,) -> () {
        self.emit(TESTUI, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `UIRET`.
    fn uiret(&mut self,) -> () {
        self.emit(UIRET, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
