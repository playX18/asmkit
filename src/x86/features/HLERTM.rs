pub trait X86HLERTMEmitter: Emitter {
    /// Emits `XABORTI` (`XABORT`). XABORT forces an RTM abort. Following an RTM abort, the logical processor resumes execution at the fallback address computed through the outermost XBEGIN instruction. The EAX register is updated to reflect an XABORT instruction caused the abort, and the imm8 argument will be provided in bits 31:24 of EAX.
    /// Reference: [Intel x86 docs for XABORT](https://www.felixcloutier.com/x86/XABORT.html)
    fn xaborti(&mut self,op0: impl OperandCast) -> () {
        self.emit(XABORTI, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XBEGIN` (`XBEGIN`). The XBEGIN instruction specifies the start of an RTM code region. If the logical processor was not already in transactional execution, then the XBEGIN instruction causes the logical processor to transition into transactional execution. The XBEGIN instruction that transitions the logical processor into transactional execution is referred to as the outermost XBEGIN instruction. The instruction also specifies a relative offset to compute the address of the fallback code path following a transactional abort. (Use of the 16-bit operand size does not cause this address to be truncated to 16 bits, unlike a near jump to a relative offset.)
    /// Reference: [Intel x86 docs for XBEGIN](https://www.felixcloutier.com/x86/XBEGIN.html)
    fn xbegin(&mut self,op0: impl OperandCast) -> () {
        self.emit(XBEGIN, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XEND` (`XEND`). The instruction marks the end of an RTM code region. If this corresponds to the outermost scope (that is, including this XEND instruction, the number of XBEGIN instructions is the same as number of XEND instructions), the logical processor will attempt to commit the logical processor state atomically. If the commit fails, the logical processor will rollback all architectural register and memory updates performed during the RTM execution. The logical processor will resume execution at the fallback address computed from the outermost XBEGIN instruction. The EAX register is updated to reflect RTM abort information.
    /// Reference: [Intel x86 docs for XEND](https://www.felixcloutier.com/x86/XEND.html)
    fn xend(&mut self,) -> () {
        self.emit(XEND, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XTEST` (`XTEST`). The XTEST instruction queries the transactional execution status. If the instruction executes inside a transactionally executing RTM region or a transactionally executing HLE region, then the ZF flag is cleared, else it is set.
    /// Reference: [Intel x86 docs for XTEST](https://www.felixcloutier.com/x86/XTEST.html)
    fn xtest(&mut self,) -> () {
        self.emit(XTEST, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
