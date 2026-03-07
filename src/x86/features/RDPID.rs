pub trait X86RDPIDEmitter: Emitter {
    /// Emits `RDPIDR`.
    fn rdpid(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDPIDR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
}
