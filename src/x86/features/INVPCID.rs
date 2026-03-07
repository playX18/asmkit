pub trait X86INVPCIDEmitter: Emitter {
    /// Emits `INVPCIDRM`.
    fn invpcid(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(INVPCIDRM, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
