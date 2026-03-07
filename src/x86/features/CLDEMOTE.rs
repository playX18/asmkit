pub trait X86CLDEMOTEEmitter: Emitter {
    /// Emits `CLDEMOTEM`.
    fn cldemote(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLDEMOTEM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
