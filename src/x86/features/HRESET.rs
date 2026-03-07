pub trait X86HRESETEmitter: Emitter {
    /// Emits `HRESETI`.
    fn hreset(&mut self,op0: impl OperandCast) -> () {
        self.emit(HRESETI, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
