pub trait X86PTWRITEEmitter: Emitter {
    /// Emits `PTWRITE32`.
    fn ptwrite32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(PTWRITE32R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(PTWRITE32M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "PTWRITE32" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `PTWRITE64`.
    fn ptwrite64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        if op0.is_gp() {
            self.emit(PTWRITE64R, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_mem() {
            self.emit(PTWRITE64M, op0,&NOREG,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "PTWRITE64" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
