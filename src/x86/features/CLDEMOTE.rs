pub trait X86CLDEMOTEEmitter: Emitter {
    /// Emits `CLDEMOTEM`.
    fn cldemote(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(CLDEMOTEM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
