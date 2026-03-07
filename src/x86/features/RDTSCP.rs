pub trait X86RDTSCPEmitter: Emitter {
    /// Emits `RDTSCP` (`RDTSCP`). Reads the current value of the processor’s time-stamp counter (a 64-bit MSR) into the EDX:EAX registers and also reads the value of the IA32_TSC_AUX MSR (address C0000103H) into the ECX register. The EDX register is loaded with the high-order 32 bits of the IA32_TSC MSR; the EAX register is loaded with the low-order 32 bits of the IA32_TSC MSR; and the ECX register is loaded with the low-order 32-bits of IA32_TSC_AUX MSR. On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX, RDX, and RCX are cleared.
    /// Reference: [Intel x86 docs for RDTSCP](https://www.felixcloutier.com/x86/RDTSCP.html)
    fn rdtscp(&mut self,) -> () {
        self.emit(RDTSCP, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
