pub trait X86POPCNTEmitter: Emitter {
    /// Emits `POPCNT16`.
    fn popcnt16(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(POPCNT16RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(POPCNT16RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "POPCNT16" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `POPCNT32`.
    fn popcnt32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(POPCNT32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(POPCNT32RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "POPCNT32" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `POPCNT64`.
    fn popcnt64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(POPCNT64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(POPCNT64RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "POPCNT64" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
