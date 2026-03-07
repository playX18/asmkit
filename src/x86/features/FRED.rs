pub trait X86FREDEmitter: Emitter {
    /// Emits `ERETS`.
    fn erets(&mut self,) -> () {
        self.emit(ERETS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `ERETU`.
    fn eretu(&mut self,) -> () {
        self.emit(ERETU, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `LKGS`.
    fn lkgs(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(LKGSR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(LKGSM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for LKGS");
        }
    }
}
