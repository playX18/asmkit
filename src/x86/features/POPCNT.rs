pub trait X86POPCNTEmitter: Emitter {
    /// Emits `POPCNT16RR` (`POPCNT`). This instruction calculates the number of bits set to 1 in the second operand (source) and returns the count in the first operand (a destination register).
    /// Reference: [Intel x86 docs for POPCNT](https://www.felixcloutier.com/x86/POPCNT.html)
    fn popcnt16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(POPCNT16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `POPCNT16RM` (`POPCNT`). This instruction calculates the number of bits set to 1 in the second operand (source) and returns the count in the first operand (a destination register).
    /// Reference: [Intel x86 docs for POPCNT](https://www.felixcloutier.com/x86/POPCNT.html)
    fn popcnt16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(POPCNT16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `POPCNT32RR` (`POPCNT`). This instruction calculates the number of bits set to 1 in the second operand (source) and returns the count in the first operand (a destination register).
    /// Reference: [Intel x86 docs for POPCNT](https://www.felixcloutier.com/x86/POPCNT.html)
    fn popcnt32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(POPCNT32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `POPCNT32RM` (`POPCNT`). This instruction calculates the number of bits set to 1 in the second operand (source) and returns the count in the first operand (a destination register).
    /// Reference: [Intel x86 docs for POPCNT](https://www.felixcloutier.com/x86/POPCNT.html)
    fn popcnt32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(POPCNT32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `POPCNT64RR` (`POPCNT`). This instruction calculates the number of bits set to 1 in the second operand (source) and returns the count in the first operand (a destination register).
    /// Reference: [Intel x86 docs for POPCNT](https://www.felixcloutier.com/x86/POPCNT.html)
    fn popcnt64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(POPCNT64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `POPCNT64RM` (`POPCNT`). This instruction calculates the number of bits set to 1 in the second operand (source) and returns the count in the first operand (a destination register).
    /// Reference: [Intel x86 docs for POPCNT](https://www.felixcloutier.com/x86/POPCNT.html)
    fn popcnt64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(POPCNT64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
