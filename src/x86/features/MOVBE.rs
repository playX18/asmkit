pub trait X86MOVBEEmitter: Emitter {
    /// Emits `MOVBE16RM` (`MOVBE`). Performs a byte swap operation on the data copied from the second operand (source operand) and store the result in the first operand (destination operand). The source operand can be a general-purpose register, or memory location; the destination register can be a general-purpose register, or a memory location; however, both operands can not be registers, and only one operand can be a memory location. Both operands must be the same size, which can be a word, a doubleword or quadword.
    /// Reference: [Intel x86 docs for MOVBE](https://www.felixcloutier.com/x86/MOVBE.html)
    fn movbe16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVBE16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MOVBE32RM` (`MOVBE`). Performs a byte swap operation on the data copied from the second operand (source operand) and store the result in the first operand (destination operand). The source operand can be a general-purpose register, or memory location; the destination register can be a general-purpose register, or a memory location; however, both operands can not be registers, and only one operand can be a memory location. Both operands must be the same size, which can be a word, a doubleword or quadword.
    /// Reference: [Intel x86 docs for MOVBE](https://www.felixcloutier.com/x86/MOVBE.html)
    fn movbe32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVBE32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MOVBE64RM` (`MOVBE`). Performs a byte swap operation on the data copied from the second operand (source operand) and store the result in the first operand (destination operand). The source operand can be a general-purpose register, or memory location; the destination register can be a general-purpose register, or a memory location; however, both operands can not be registers, and only one operand can be a memory location. Both operands must be the same size, which can be a word, a doubleword or quadword.
    /// Reference: [Intel x86 docs for MOVBE](https://www.felixcloutier.com/x86/MOVBE.html)
    fn movbe64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVBE64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MOVBE16MR` (`MOVBE`). Performs a byte swap operation on the data copied from the second operand (source operand) and store the result in the first operand (destination operand). The source operand can be a general-purpose register, or memory location; the destination register can be a general-purpose register, or a memory location; however, both operands can not be registers, and only one operand can be a memory location. Both operands must be the same size, which can be a word, a doubleword or quadword.
    /// Reference: [Intel x86 docs for MOVBE](https://www.felixcloutier.com/x86/MOVBE.html)
    fn movbe16mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVBE16MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MOVBE32MR` (`MOVBE`). Performs a byte swap operation on the data copied from the second operand (source operand) and store the result in the first operand (destination operand). The source operand can be a general-purpose register, or memory location; the destination register can be a general-purpose register, or a memory location; however, both operands can not be registers, and only one operand can be a memory location. Both operands must be the same size, which can be a word, a doubleword or quadword.
    /// Reference: [Intel x86 docs for MOVBE](https://www.felixcloutier.com/x86/MOVBE.html)
    fn movbe32mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVBE32MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MOVBE64MR` (`MOVBE`). Performs a byte swap operation on the data copied from the second operand (source operand) and store the result in the first operand (destination operand). The source operand can be a general-purpose register, or memory location; the destination register can be a general-purpose register, or a memory location; however, both operands can not be registers, and only one operand can be a memory location. Both operands must be the same size, which can be a word, a doubleword or quadword.
    /// Reference: [Intel x86 docs for MOVBE](https://www.felixcloutier.com/x86/MOVBE.html)
    fn movbe64mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVBE64MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
