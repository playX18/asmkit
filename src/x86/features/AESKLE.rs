pub trait X86AESKLEEmitter: Emitter {
    /// Emits `AESDEC128KLRM`.
    fn aesdec128kl(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESDEC128KLRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `AESDEC256KLRM`.
    fn aesdec256kl(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESDEC256KLRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `AESDECWIDE128KLM`.
    fn aesdecwide128kl(&mut self,op0: impl OperandCast) -> () {
        self.emit(AESDECWIDE128KLM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `AESDECWIDE256KLM`.
    fn aesdecwide256kl(&mut self,op0: impl OperandCast) -> () {
        self.emit(AESDECWIDE256KLM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `AESENC128KLRM`.
    fn aesenc128kl(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESENC128KLRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `AESENC256KLRM`.
    fn aesenc256kl(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(AESENC256KLRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `AESENCWIDE128KLM`.
    fn aesencwide128kl(&mut self,op0: impl OperandCast) -> () {
        self.emit(AESENCWIDE128KLM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `AESENCWIDE256KLM`.
    fn aesencwide256kl(&mut self,op0: impl OperandCast) -> () {
        self.emit(AESENCWIDE256KLM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `ENCODEKEY128RR`.
    fn encodekey128(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENCODEKEY128RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `ENCODEKEY256RR`.
    fn encodekey256(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(ENCODEKEY256RR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `LOADIWKEYRR`.
    fn loadiwkey(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LOADIWKEYRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
}
