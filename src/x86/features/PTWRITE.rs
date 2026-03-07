pub trait X86PTWRITEEmitter: Emitter {
    /// Emits `PTWRITE32R` (`PTWRITE`). This instruction reads data in the source operand and sends it to the Intel Processor Trace hardware to be encoded in a PTW packet if TriggerEn, ContextEn, FilterEn, and PTWEn are all set to 1. For more details on these values, see Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 3C, Section 33.2.2, “Software Trace Instrumentation with PTWRITE.” The size of data is 64-bit if using REX.W in 64-bit mode, otherwise 32-bits of data are copied from the source operand.
    /// Reference: [Intel x86 docs for PTWRITE](https://www.felixcloutier.com/x86/PTWRITE.html)
    fn ptwrite32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(PTWRITE32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `PTWRITE32M` (`PTWRITE`). This instruction reads data in the source operand and sends it to the Intel Processor Trace hardware to be encoded in a PTW packet if TriggerEn, ContextEn, FilterEn, and PTWEn are all set to 1. For more details on these values, see Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 3C, Section 33.2.2, “Software Trace Instrumentation with PTWRITE.” The size of data is 64-bit if using REX.W in 64-bit mode, otherwise 32-bits of data are copied from the source operand.
    /// Reference: [Intel x86 docs for PTWRITE](https://www.felixcloutier.com/x86/PTWRITE.html)
    fn ptwrite32m(&mut self,op0: impl OperandCast) -> () {
        self.emit(PTWRITE32M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `PTWRITE64R` (`PTWRITE`). This instruction reads data in the source operand and sends it to the Intel Processor Trace hardware to be encoded in a PTW packet if TriggerEn, ContextEn, FilterEn, and PTWEn are all set to 1. For more details on these values, see Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 3C, Section 33.2.2, “Software Trace Instrumentation with PTWRITE.” The size of data is 64-bit if using REX.W in 64-bit mode, otherwise 32-bits of data are copied from the source operand.
    /// Reference: [Intel x86 docs for PTWRITE](https://www.felixcloutier.com/x86/PTWRITE.html)
    fn ptwrite64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(PTWRITE64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `PTWRITE64M` (`PTWRITE`). This instruction reads data in the source operand and sends it to the Intel Processor Trace hardware to be encoded in a PTW packet if TriggerEn, ContextEn, FilterEn, and PTWEn are all set to 1. For more details on these values, see Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 3C, Section 33.2.2, “Software Trace Instrumentation with PTWRITE.” The size of data is 64-bit if using REX.W in 64-bit mode, otherwise 32-bits of data are copied from the source operand.
    /// Reference: [Intel x86 docs for PTWRITE](https://www.felixcloutier.com/x86/PTWRITE.html)
    fn ptwrite64m(&mut self,op0: impl OperandCast) -> () {
        self.emit(PTWRITE64M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
