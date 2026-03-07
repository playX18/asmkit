pub trait X86CLWBEmitter: Emitter {
    /// Emits `CLWBM`.
    fn clwb(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLWBM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
