pub trait X86586Emitter: Emitter {
    /// Emits `CMPXCHG16BM`.
    fn cmpxchg16b(&mut self,op0: impl OperandCast) -> () {
        self.emit(CMPXCHG16BM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CMPXCHG8BM`.
    fn cmpxchg8b(&mut self,op0: impl OperandCast) -> () {
        self.emit(CMPXCHG8BM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CMPXCHGD32M`.
    fn cmpxchgd32(&mut self,op0: impl OperandCast) -> () {
        self.emit(CMPXCHGD32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CMPXCHGD64M`.
    fn cmpxchgd64(&mut self,op0: impl OperandCast) -> () {
        self.emit(CMPXCHGD64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `CPUID`.
    fn cpuid(&mut self,) -> () {
        self.emit(CPUID, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RDMSR`.
    fn rdmsr(&mut self,) -> () {
        self.emit(RDMSR, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RDTSC`.
    fn rdtsc(&mut self,) -> () {
        self.emit(RDTSC, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `RSM`.
    fn rsm(&mut self,) -> () {
        self.emit(RSM, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `WRMSR`.
    fn wrmsr(&mut self,) -> () {
        self.emit(WRMSR, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
