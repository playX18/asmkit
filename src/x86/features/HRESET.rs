pub trait X86HRESETEmitter: Emitter {
    /// Emits `HRESETI` (`HRESET`). Requests the processor to selectively reset selected components of hardware history maintained by the current logical processor. HRESET operation is controlled by the implicit EAX operand. The value of the explicit imm8 operand is ignored. This instruction can only be executed at privilege level 0.
    /// Reference: [Intel x86 docs for HRESET](https://www.felixcloutier.com/x86/HRESET.html)
    fn hreseti(&mut self,op0: impl OperandCast) -> () {
        self.emit(HRESETI, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
