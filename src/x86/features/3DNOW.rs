pub trait X863DNOWEmitter: Emitter {
    /// Emits `FEMMS`.
    fn femms(&mut self,) -> () {
        self.emit(FEMMS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `_3DNOW`.
    fn _3dnow(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        let op2 = op2.as_operand();
        if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_reg_group_of(RegGroup::X86MM) && op2.is_imm() {
            self.emit(_3DNOWRRI, op0,op1,op2,&NOREG);
        } else if op0.is_reg_group_of(RegGroup::X86MM) && op1.is_mem() && op2.is_imm() {
            self.emit(_3DNOWRMI, op0,op1,op2,&NOREG);
        } else {
            unreachable!("invalid operand types for _3DNOW");
        }
    }
}
