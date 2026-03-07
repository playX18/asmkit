pub trait X86VMXEmitter: Emitter {
    /// Emits `INVEPTRM`.
    fn invept(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(INVEPTRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `INVVPIDRM`.
    fn invvpid(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(INVVPIDRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `VMCALL`.
    fn vmcall(&mut self,) -> () {
        self.emit(VMCALL, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMCLEARM`.
    fn vmclear(&mut self,op0: impl OperandCast) -> () {
        self.emit(VMCLEARM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMFUNC`.
    fn vmfunc(&mut self,) -> () {
        self.emit(VMFUNC, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMLAUNCH`.
    fn vmlaunch(&mut self,) -> () {
        self.emit(VMLAUNCH, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMPTRLDM`.
    fn vmptrld(&mut self,op0: impl OperandCast) -> () {
        self.emit(VMPTRLDM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMPTRSTM`.
    fn vmptrst(&mut self,op0: impl OperandCast) -> () {
        self.emit(VMPTRSTM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMREAD`.
    fn vmread(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(VMREADRR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_mem() && op1.is_gp() {
            self.emit(VMREADMR, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMREAD");
        }
    }
    /// Emits `VMRESUME`.
    fn vmresume(&mut self,) -> () {
        self.emit(VMRESUME, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMWRITE`.
    fn vmwrite(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        let op1 = op1.as_operand();
        if op0.is_gp() && op1.is_gp() {
            self.emit(VMWRITERR, op0,op1,&NOREG,&NOREG);
        } else if op0.is_gp() && op1.is_mem() {
            self.emit(VMWRITERM, op0,op1,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for VMWRITE");
        }
    }
    /// Emits `VMXOFF`.
    fn vmxoff(&mut self,) -> () {
        self.emit(VMXOFF, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `VMXONM`.
    fn vmxon(&mut self,op0: impl OperandCast) -> () {
        self.emit(VMXONM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
