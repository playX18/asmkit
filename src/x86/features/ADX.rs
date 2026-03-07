pub trait X86ADXEmitter: Emitter {
    /// Emits `ADCX32`.
    fn adcx32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADCX32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADCX32RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "ADCX32" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `ADCX64`.
    fn adcx64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADCX64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADCX64RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "ADCX64" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `ADOX32`.
    fn adox32(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADOX32RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADOX32RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "ADOX32" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `ADOX64`.
    fn adox64(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(ADOX64RR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(ADOX64RM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "ADOX64" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
