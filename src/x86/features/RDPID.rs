pub trait X86RDPIDEmitter: Emitter {
    /// Emits `RDPIDR`.
    fn rdpid(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(RDPIDR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
