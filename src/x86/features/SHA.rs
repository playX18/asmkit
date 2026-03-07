pub trait X86SHAEmitter: Emitter {
    /// Emits `SHA1NEXTERR` (`SHA1NEXTE`). The SHA1NEXTE calculates the SHA1 state variable E after four rounds of operation from the current SHA1 state variable A in the destination operand. The calculated value of the SHA1 state variable E is added to the source operand, which contains the scheduled dwords.
    /// Reference: [Intel x86 docs for SHA1NEXTE](https://www.felixcloutier.com/x86/SHA1NEXTE.html)
    fn sha1nexterr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SHA1NEXTERR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SHA1NEXTERM` (`SHA1NEXTE`). The SHA1NEXTE calculates the SHA1 state variable E after four rounds of operation from the current SHA1 state variable A in the destination operand. The calculated value of the SHA1 state variable E is added to the source operand, which contains the scheduled dwords.
    /// Reference: [Intel x86 docs for SHA1NEXTE](https://www.felixcloutier.com/x86/SHA1NEXTE.html)
    fn sha1nexterm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SHA1NEXTERM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SHA1MSG1RR` (`SHA1MSG1`). The SHA1MSG1 instruction is one of two SHA1 message scheduling instructions. The instruction performs an intermediate calculation for the next four SHA1 message dwords.
    /// Reference: [Intel x86 docs for SHA1MSG1](https://www.felixcloutier.com/x86/SHA1MSG1.html)
    fn sha1msg1rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SHA1MSG1RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SHA1MSG1RM` (`SHA1MSG1`). The SHA1MSG1 instruction is one of two SHA1 message scheduling instructions. The instruction performs an intermediate calculation for the next four SHA1 message dwords.
    /// Reference: [Intel x86 docs for SHA1MSG1](https://www.felixcloutier.com/x86/SHA1MSG1.html)
    fn sha1msg1rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SHA1MSG1RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SHA1MSG2RR` (`SHA1MSG2`). The SHA1MSG2 instruction is one of two SHA1 message scheduling instructions. The instruction performs the final calculation to derive the next four SHA1 message dwords.
    /// Reference: [Intel x86 docs for SHA1MSG2](https://www.felixcloutier.com/x86/SHA1MSG2.html)
    fn sha1msg2rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SHA1MSG2RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SHA1MSG2RM` (`SHA1MSG2`). The SHA1MSG2 instruction is one of two SHA1 message scheduling instructions. The instruction performs the final calculation to derive the next four SHA1 message dwords.
    /// Reference: [Intel x86 docs for SHA1MSG2](https://www.felixcloutier.com/x86/SHA1MSG2.html)
    fn sha1msg2rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SHA1MSG2RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SHA256RNDS2RRR` (`SHA256RNDS2`). The SHA256RNDS2 instruction performs 2 rounds of SHA256 operation using an initial SHA256 state (C,D,G,H) from the first operand, an initial SHA256 state (A,B,E,F) from the second operand, and a pre-computed sum of the next 2 round message dwords and the corresponding round constants from the implicit operand xmm0. Note that only the two lower dwords of XMM0 are used by the instruction.
    /// Reference: [Intel x86 docs for SHA256RNDS2](https://www.felixcloutier.com/x86/SHA256RNDS2.html)
    fn sha256rnds2rrr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SHA256RNDS2RRR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SHA256RNDS2RMR` (`SHA256RNDS2`). The SHA256RNDS2 instruction performs 2 rounds of SHA256 operation using an initial SHA256 state (C,D,G,H) from the first operand, an initial SHA256 state (A,B,E,F) from the second operand, and a pre-computed sum of the next 2 round message dwords and the corresponding round constants from the implicit operand xmm0. Note that only the two lower dwords of XMM0 are used by the instruction.
    /// Reference: [Intel x86 docs for SHA256RNDS2](https://www.felixcloutier.com/x86/SHA256RNDS2.html)
    fn sha256rnds2rmr(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SHA256RNDS2RMR, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SHA256MSG1RR` (`SHA256MSG1`). The SHA256MSG1 instruction is one of two SHA256 message scheduling instructions. The instruction performs an intermediate calculation for the next four SHA256 message dwords.
    /// Reference: [Intel x86 docs for SHA256MSG1](https://www.felixcloutier.com/x86/SHA256MSG1.html)
    fn sha256msg1rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SHA256MSG1RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SHA256MSG1RM` (`SHA256MSG1`). The SHA256MSG1 instruction is one of two SHA256 message scheduling instructions. The instruction performs an intermediate calculation for the next four SHA256 message dwords.
    /// Reference: [Intel x86 docs for SHA256MSG1](https://www.felixcloutier.com/x86/SHA256MSG1.html)
    fn sha256msg1rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SHA256MSG1RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SHA256MSG2RR` (`SHA256MSG2`). The SHA256MSG2 instruction is one of two SHA2 message scheduling instructions. The instruction performs the final calculation for the next four SHA256 message dwords.
    /// Reference: [Intel x86 docs for SHA256MSG2](https://www.felixcloutier.com/x86/SHA256MSG2.html)
    fn sha256msg2rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SHA256MSG2RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SHA256MSG2RM` (`SHA256MSG2`). The SHA256MSG2 instruction is one of two SHA2 message scheduling instructions. The instruction performs the final calculation for the next four SHA256 message dwords.
    /// Reference: [Intel x86 docs for SHA256MSG2](https://www.felixcloutier.com/x86/SHA256MSG2.html)
    fn sha256msg2rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SHA256MSG2RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SHA1RNDS4RRI` (`SHA1RNDS4`). The SHA1RNDS4 instruction performs four rounds of SHA1 operation using an initial SHA1 state (A,B,C,D) from the first operand (which is a source operand and the destination operand) and some pre-computed sum of the next 4 round message dwords, and state variable E from the second operand (a source operand). The updated SHA1 state (A,B,C,D) after four rounds of processing is stored in the destination operand.
    /// Reference: [Intel x86 docs for SHA1RNDS4](https://www.felixcloutier.com/x86/SHA1RNDS4.html)
    fn sha1rnds4rri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SHA1RNDS4RRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SHA1RNDS4RMI` (`SHA1RNDS4`). The SHA1RNDS4 instruction performs four rounds of SHA1 operation using an initial SHA1 state (A,B,C,D) from the first operand (which is a source operand and the destination operand) and some pre-computed sum of the next 4 round message dwords, and state variable E from the second operand (a source operand). The updated SHA1 state (A,B,C,D) after four rounds of processing is stored in the destination operand.
    /// Reference: [Intel x86 docs for SHA1RNDS4](https://www.felixcloutier.com/x86/SHA1RNDS4.html)
    fn sha1rnds4rmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SHA1RNDS4RMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
