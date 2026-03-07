pub trait X86CLFLUSHOPTEmitter: Emitter {
    /// Emits `CLFLUSHOPTM`.
    fn clflushopt(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(CLFLUSHOPTM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
