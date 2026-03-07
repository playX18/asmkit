pub trait X86INVPCIDEmitter: Emitter {
    /// Emits `INVPCIDRM`.
    fn invpcid(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(INVPCIDRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
}
