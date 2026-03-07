pub trait X863DNOWEmitter: Emitter {
    /// Emits `FEMMS`.
    fn femms(&mut self,) -> Result<(), AsmError> {
        self.emit(FEMMS, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `_3DNOW`.
    fn _3dnow(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) && op2.is_imm() {
            self.emit(_3DNOWRRI, op0,op1,op2,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() && op2.is_imm() {
            self.emit(_3DNOWRMI, op0,op1,op2,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "_3DNOW" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
