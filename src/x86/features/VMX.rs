pub trait X86VMXEmitter: Emitter {
    /// Emits `INVEPTRM`.
    fn invept(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(INVEPTRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `INVVPIDRM`.
    fn invvpid(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(INVVPIDRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMCALL`.
    fn vmcall(&mut self,) -> Result<(), AsmError> {
        self.emit(VMCALL, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMCLEARM`.
    fn vmclear(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VMCLEARM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMFUNC`.
    fn vmfunc(&mut self,) -> Result<(), AsmError> {
        self.emit(VMFUNC, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMLAUNCH`.
    fn vmlaunch(&mut self,) -> Result<(), AsmError> {
        self.emit(VMLAUNCH, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMPTRLDM`.
    fn vmptrld(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VMPTRLDM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMPTRSTM`.
    fn vmptrst(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VMPTRSTM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMREAD`.
    fn vmread(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(VMREADRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(VMREADMR, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VMREAD" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMRESUME`.
    fn vmresume(&mut self,) -> Result<(), AsmError> {
        self.emit(VMRESUME, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMWRITE`.
    fn vmwrite(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(VMWRITERR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(VMWRITERM, op0,op1,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "VMWRITE" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMXOFF`.
    fn vmxoff(&mut self,) -> Result<(), AsmError> {
        self.emit(VMXOFF, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `VMXONM`.
    fn vmxon(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(VMXONM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
