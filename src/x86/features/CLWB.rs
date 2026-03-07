pub trait X86CLWBEmitter: Emitter {
    /// Emits `CLWBM`.
    fn clwb(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(CLWBM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
