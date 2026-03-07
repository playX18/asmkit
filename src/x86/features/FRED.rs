pub trait X86FREDEmitter: Emitter {
    /// Emits `ERETS`.
    fn erets(&mut self,) -> Result<(), AsmError> {
        self.emit(ERETS, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `ERETU`.
    fn eretu(&mut self,) -> Result<(), AsmError> {
        self.emit(ERETU, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `LKGS`.
    fn lkgs(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(LKGSR, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(LKGSM, op0,&NOREG,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "LKGS" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
