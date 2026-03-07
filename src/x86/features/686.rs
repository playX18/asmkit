pub trait X86686Emitter: Emitter {
    /// Emits `RDPMC` (`RDPMC`). Reads the contents of the performance monitoring counter (PMC) specified in ECX register into registers EDX:EAX. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The EDX register is loaded with the high-order 32 bits of the PMC and the EAX register is loaded with the low-order 32 bits. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.) If fewer than 64 bits are implemented in the PMC being read, unimplemented bits returned to EDX:EAX will have value zero.
    /// Reference: [Intel x86 docs for RDPMC](https://www.felixcloutier.com/x86/RDPMC.html)
    fn rdpmc(&mut self,) -> () {
        self.emit(RDPMC, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SYSENTER` (`SYSENTER`). Executes a fast call to a level 0 system procedure or routine. SYSENTER is a companion instruction to SYSEXIT. The instruction is optimized to provide the maximum performance for system calls from user code running at privilege level 3 to operating system or executive procedures running at privilege level 0.
    /// Reference: [Intel x86 docs for SYSENTER](https://www.felixcloutier.com/x86/SYSENTER.html)
    fn sysenter(&mut self,) -> () {
        self.emit(SYSENTER, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SYSEXIT` (`SYSEXIT`). Executes a fast return to privilege level 3 user code. SYSEXIT is a companion instruction to the SYSENTER instruction. The instruction is optimized to provide the maximum performance for returns from system procedures executing at protections levels 0 to user procedures executing at protection level 3. It must be executed from code executing at privilege level 0.
    /// Reference: [Intel x86 docs for SYSEXIT](https://www.felixcloutier.com/x86/SYSEXIT.html)
    fn sysexit(&mut self,) -> () {
        self.emit(SYSEXIT, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCMOVBR` (`FCMOVB`). Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    /// Reference: [Intel x86 docs for FCMOVB](https://www.felixcloutier.com/x86/FCMOVcc.html)
    fn fcmovbr(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVBR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCMOVER` (`FCMOVE`). Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    /// Reference: [Intel x86 docs for FCMOVE](https://www.felixcloutier.com/x86/FCMOVcc.html)
    fn fcmover(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVER, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCMOVBER` (`FCMOVBE`). Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    /// Reference: [Intel x86 docs for FCMOVBE](https://www.felixcloutier.com/x86/FCMOVcc.html)
    fn fcmovber(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVBER, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCMOVUR` (`FCMOVU`). Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    /// Reference: [Intel x86 docs for FCMOVU](https://www.felixcloutier.com/x86/FCMOVcc.html)
    fn fcmovur(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVUR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCMOVNBR` (`FCMOVNB`). Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    /// Reference: [Intel x86 docs for FCMOVNB](https://www.felixcloutier.com/x86/FCMOVcc.html)
    fn fcmovnbr(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVNBR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCMOVNER` (`FCMOVNE`). Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    /// Reference: [Intel x86 docs for FCMOVNE](https://www.felixcloutier.com/x86/FCMOVcc.html)
    fn fcmovner(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVNER, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCMOVNBER` (`FCMOVNBE`). Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    /// Reference: [Intel x86 docs for FCMOVNBE](https://www.felixcloutier.com/x86/FCMOVcc.html)
    fn fcmovnber(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVNBER, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCMOVNUR` (`FCMOVNU`). Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    /// Reference: [Intel x86 docs for FCMOVNU](https://www.felixcloutier.com/x86/FCMOVcc.html)
    fn fcmovnur(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCMOVNUR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FUCOMIR` (`FUCOMI`). Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FUCOMI](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html)
    fn fucomir(&mut self,op0: impl OperandCast) -> () {
        self.emit(FUCOMIR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCOMIR` (`FCOMI`). Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FCOMI](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html)
    fn fcomir(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCOMIR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FUCOMIPRR` (`FUCOMIP`). Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FUCOMIP](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html)
    fn fucomiprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FUCOMIPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCOMIPRR` (`FCOMIP`). Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FCOMIP](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html)
    fn fcomiprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FCOMIPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
