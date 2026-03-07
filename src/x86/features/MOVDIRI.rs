pub trait X86MOVDIRIEmitter: Emitter {
    /// Emits `MOVDIRI32MR` (`MOVDIRI`). Moves the doubleword integer in the source operand (second operand) to the destination operand (first operand) using a direct-store operation. The source operand is a general purpose register. The destination operand is a 32-bit memory location. In 64-bit mode, the instruction’s default operation size is 32 bits. Use of the REX.R prefix permits access to additional registers (R8-R15). Use of the REX.W prefix promotes operation to 64 bits. See summary chart at the beginning of this section for encoding data and limits.
    /// Reference: [Intel x86 docs for MOVDIRI](https://www.felixcloutier.com/x86/MOVDIRI.html)
    fn movdiri32mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVDIRI32MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MOVDIRI64MR` (`MOVDIRI`). Moves the doubleword integer in the source operand (second operand) to the destination operand (first operand) using a direct-store operation. The source operand is a general purpose register. The destination operand is a 32-bit memory location. In 64-bit mode, the instruction’s default operation size is 32 bits. Use of the REX.R prefix permits access to additional registers (R8-R15). Use of the REX.W prefix promotes operation to 64 bits. See summary chart at the beginning of this section for encoding data and limits.
    /// Reference: [Intel x86 docs for MOVDIRI](https://www.felixcloutier.com/x86/MOVDIRI.html)
    fn movdiri64mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVDIRI64MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
