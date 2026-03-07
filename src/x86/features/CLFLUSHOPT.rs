pub trait X86CLFLUSHOPTEmitter: Emitter {
    /// Emits `CLFLUSHOPTM`.
    fn clflushopt(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLFLUSHOPTM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
