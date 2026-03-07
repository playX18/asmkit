pub trait X86WAITPKGEmitter: Emitter {
    /// Emits `TPAUSER` (`TPAUSE`). TPAUSE instructs the processor to enter an implementation-dependent optimized state. There are two such optimized states to choose from: light-weight power/performance optimized state, and improved power/performance optimized state. The selection between the two is governed by the explicit input register bit[0] source operand.
    /// Reference: [Intel x86 docs for TPAUSE](https://www.felixcloutier.com/x86/TPAUSE.html)
    fn tpauser(&mut self,op0: impl OperandCast) -> () {
        self.emit(TPAUSER, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `UMONITOR32R` (`UMONITOR`). The UMONITOR instruction arms address monitoring hardware using an address specified in the source register (the address range that the monitoring hardware checks for store operations can be determined by using the CPUID monitor leaf function, EAX=05H). A store to an address within the specified address range triggers the monitoring hardware. The state of monitor hardware is used by UMWAIT.
    /// Reference: [Intel x86 docs for UMONITOR](https://www.felixcloutier.com/x86/UMONITOR.html)
    fn umonitor32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(UMONITOR32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `UMONITOR64R` (`UMONITOR`). The UMONITOR instruction arms address monitoring hardware using an address specified in the source register (the address range that the monitoring hardware checks for store operations can be determined by using the CPUID monitor leaf function, EAX=05H). A store to an address within the specified address range triggers the monitoring hardware. The state of monitor hardware is used by UMWAIT.
    /// Reference: [Intel x86 docs for UMONITOR](https://www.felixcloutier.com/x86/UMONITOR.html)
    fn umonitor64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(UMONITOR64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `UMWAITR` (`UMWAIT`). UMWAIT instructs the processor to enter an implementation-dependent optimized state while monitoring a range of addresses. The optimized state may be either a light-weight power/performance optimized state or an improved power/performance optimized state. The selection between the two states is governed by the explicit input register bit[0] source operand.
    /// Reference: [Intel x86 docs for UMWAIT](https://www.felixcloutier.com/x86/UMWAIT.html)
    fn umwaitr(&mut self,op0: impl OperandCast) -> () {
        self.emit(UMWAITR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
