pub trait X86LZCNTEmitter: Emitter {
    /// Emits `LZCNT16RR` (`LZCNT`). Counts the number of leading most significant zero bits in a source operand (second operand) returning the result into a destination (first operand).
    /// Reference: [Intel x86 docs for LZCNT](https://www.felixcloutier.com/x86/LZCNT.html)
    fn lzcnt16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LZCNT16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `LZCNT16RM` (`LZCNT`). Counts the number of leading most significant zero bits in a source operand (second operand) returning the result into a destination (first operand).
    /// Reference: [Intel x86 docs for LZCNT](https://www.felixcloutier.com/x86/LZCNT.html)
    fn lzcnt16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LZCNT16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `LZCNT32RR` (`LZCNT`). Counts the number of leading most significant zero bits in a source operand (second operand) returning the result into a destination (first operand).
    /// Reference: [Intel x86 docs for LZCNT](https://www.felixcloutier.com/x86/LZCNT.html)
    fn lzcnt32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LZCNT32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `LZCNT32RM` (`LZCNT`). Counts the number of leading most significant zero bits in a source operand (second operand) returning the result into a destination (first operand).
    /// Reference: [Intel x86 docs for LZCNT](https://www.felixcloutier.com/x86/LZCNT.html)
    fn lzcnt32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LZCNT32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `LZCNT64RR` (`LZCNT`). Counts the number of leading most significant zero bits in a source operand (second operand) returning the result into a destination (first operand).
    /// Reference: [Intel x86 docs for LZCNT](https://www.felixcloutier.com/x86/LZCNT.html)
    fn lzcnt64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LZCNT64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `LZCNT64RM` (`LZCNT`). Counts the number of leading most significant zero bits in a source operand (second operand) returning the result into a destination (first operand).
    /// Reference: [Intel x86 docs for LZCNT](https://www.felixcloutier.com/x86/LZCNT.html)
    fn lzcnt64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(LZCNT64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
