pub trait X86586Emitter: Emitter {
    /// Emits `WRMSR` (`WRMSR`). Writes the contents of registers EDX:EAX into the 64-bit model specific register (MSR) specified in the ECX register. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The contents of the EDX register are copied to high-order 32 bits of the selected MSR and the contents of the EAX register are copied to low-order 32 bits of the MSR. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are ignored.) Undefined or reserved bits in an MSR should be set to values previously read.
    /// Reference: [Intel x86 docs for WRMSR](https://www.felixcloutier.com/x86/WRMSR.html)
    fn wrmsr(&mut self,) -> () {
        self.emit(WRMSR, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RDTSC` (`RDTSC`). Reads the current value of the processor’s time-stamp counter (a 64-bit MSR) into the EDX:EAX registers. The EDX register is loaded with the high-order 32 bits of the MSR and the EAX register is loaded with the low-order 32 bits. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.)
    /// Reference: [Intel x86 docs for RDTSC](https://www.felixcloutier.com/x86/RDTSC.html)
    fn rdtsc(&mut self,) -> () {
        self.emit(RDTSC, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RDMSR`.
    fn rdmsr(&mut self,) -> () {
        self.emit(RDMSR, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CPUID` (`CPUID`). The ID flag (bit 21) in the EFLAGS register indicates support for the CPUID instruction. If a software procedure can set and clear this flag, the processor executing the procedure supports the CPUID instruction. This instruction operates the same in non-64-bit modes and 64-bit mode.
    /// Reference: [Intel x86 docs for CPUID](https://www.felixcloutier.com/x86/CPUID.html)
    fn cpuid(&mut self,) -> () {
        self.emit(CPUID, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RSM` (`RSM`). Returns program control from system management mode (SMM) to the application program or operating-system procedure that was interrupted when the processor received an SMM interrupt. The processor’s state is restored from the dump created upon entering SMM. If the processor detects invalid state information during state restoration, it enters the shutdown state. The following invalid information can cause a shutdown
    /// Reference: [Intel x86 docs for RSM](https://www.felixcloutier.com/x86/RSM.html)
    fn rsm(&mut self,) -> () {
        self.emit(RSM, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHGD32M`.
    fn cmpxchgd32m(&mut self,op0: impl OperandCast) -> () {
        self.emit(CMPXCHGD32M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHG8BM`.
    fn cmpxchg8bm(&mut self,op0: impl OperandCast) -> () {
        self.emit(CMPXCHG8BM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHGD64M`.
    fn cmpxchgd64m(&mut self,op0: impl OperandCast) -> () {
        self.emit(CMPXCHGD64M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHG16BM`.
    fn cmpxchg16bm(&mut self,op0: impl OperandCast) -> () {
        self.emit(CMPXCHG16BM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
