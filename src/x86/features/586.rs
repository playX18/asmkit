pub trait X86586Emitter: Emitter {
    /// Emits `CMPXCHG16BM`.
    fn cmpxchg16b(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(CMPXCHG16BM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `CMPXCHG8BM`.
    fn cmpxchg8b(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(CMPXCHG8BM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `CMPXCHGD32M`.
    fn cmpxchgd32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(CMPXCHGD32M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `CMPXCHGD64M`.
    fn cmpxchgd64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(CMPXCHGD64M, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `CPUID`.
    fn cpuid(&mut self,) -> Result<(), AsmError> {
        self.emit(CPUID, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RDMSR`.
    fn rdmsr(&mut self,) -> Result<(), AsmError> {
        self.emit(RDMSR, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RDTSC`.
    fn rdtsc(&mut self,) -> Result<(), AsmError> {
        self.emit(RDTSC, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `RSM`.
    fn rsm(&mut self,) -> Result<(), AsmError> {
        self.emit(RSM, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `WRMSR`.
    fn wrmsr(&mut self,) -> Result<(), AsmError> {
        self.emit(WRMSR, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
