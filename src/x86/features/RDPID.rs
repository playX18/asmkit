pub trait X86RDPIDEmitter: Emitter {
    /// Emits `RDPIDR` (`RDPID`). Reads the value of the IA32_TSC_AUX MSR (address C0000103H) into the destination register. The value of CS.D and operand-size prefixes (66H and REX.W) do not affect the behavior of the RDPID instruction.
    /// Reference: [Intel x86 docs for RDPID](https://www.felixcloutier.com/x86/RDPID.html)
    fn rdpidr(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDPIDR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
