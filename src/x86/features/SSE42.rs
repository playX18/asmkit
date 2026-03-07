pub trait X86SSE42Emitter: Emitter {
    /// Emits `CRC32_8RR` (`CRC32`). Starting with an initial value in the first operand (destination operand), accumulates a CRC32 (polynomial 11EDC6F41H) value for the second operand (source operand) and stores the result in the destination operand. The source operand can be a register or a memory location. The destination operand must be an r32 or r64 register. If the destination is an r64 register, then the 32-bit result is stored in the least significant double word and 00000000H is stored in the most significant double word of the r64 register.
    /// Reference: [Intel x86 docs for CRC32](https://www.felixcloutier.com/x86/CRC32.html)
    fn crc32_8rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CRC32_8RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CRC32_8RM` (`CRC32`). Starting with an initial value in the first operand (destination operand), accumulates a CRC32 (polynomial 11EDC6F41H) value for the second operand (source operand) and stores the result in the destination operand. The source operand can be a register or a memory location. The destination operand must be an r32 or r64 register. If the destination is an r64 register, then the 32-bit result is stored in the least significant double word and 00000000H is stored in the most significant double word of the r64 register.
    /// Reference: [Intel x86 docs for CRC32](https://www.felixcloutier.com/x86/CRC32.html)
    fn crc32_8rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CRC32_8RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CRC32_16RR` (`CRC32`). Starting with an initial value in the first operand (destination operand), accumulates a CRC32 (polynomial 11EDC6F41H) value for the second operand (source operand) and stores the result in the destination operand. The source operand can be a register or a memory location. The destination operand must be an r32 or r64 register. If the destination is an r64 register, then the 32-bit result is stored in the least significant double word and 00000000H is stored in the most significant double word of the r64 register.
    /// Reference: [Intel x86 docs for CRC32](https://www.felixcloutier.com/x86/CRC32.html)
    fn crc32_16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CRC32_16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CRC32_16RM` (`CRC32`). Starting with an initial value in the first operand (destination operand), accumulates a CRC32 (polynomial 11EDC6F41H) value for the second operand (source operand) and stores the result in the destination operand. The source operand can be a register or a memory location. The destination operand must be an r32 or r64 register. If the destination is an r64 register, then the 32-bit result is stored in the least significant double word and 00000000H is stored in the most significant double word of the r64 register.
    /// Reference: [Intel x86 docs for CRC32](https://www.felixcloutier.com/x86/CRC32.html)
    fn crc32_16rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CRC32_16RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CRC32_32RR` (`CRC32`). Starting with an initial value in the first operand (destination operand), accumulates a CRC32 (polynomial 11EDC6F41H) value for the second operand (source operand) and stores the result in the destination operand. The source operand can be a register or a memory location. The destination operand must be an r32 or r64 register. If the destination is an r64 register, then the 32-bit result is stored in the least significant double word and 00000000H is stored in the most significant double word of the r64 register.
    /// Reference: [Intel x86 docs for CRC32](https://www.felixcloutier.com/x86/CRC32.html)
    fn crc32_32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CRC32_32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CRC32_32RM` (`CRC32`). Starting with an initial value in the first operand (destination operand), accumulates a CRC32 (polynomial 11EDC6F41H) value for the second operand (source operand) and stores the result in the destination operand. The source operand can be a register or a memory location. The destination operand must be an r32 or r64 register. If the destination is an r64 register, then the 32-bit result is stored in the least significant double word and 00000000H is stored in the most significant double word of the r64 register.
    /// Reference: [Intel x86 docs for CRC32](https://www.felixcloutier.com/x86/CRC32.html)
    fn crc32_32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CRC32_32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CRC32_64RR` (`CRC32`). Starting with an initial value in the first operand (destination operand), accumulates a CRC32 (polynomial 11EDC6F41H) value for the second operand (source operand) and stores the result in the destination operand. The source operand can be a register or a memory location. The destination operand must be an r32 or r64 register. If the destination is an r64 register, then the 32-bit result is stored in the least significant double word and 00000000H is stored in the most significant double word of the r64 register.
    /// Reference: [Intel x86 docs for CRC32](https://www.felixcloutier.com/x86/CRC32.html)
    fn crc32_64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CRC32_64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CRC32_64RM` (`CRC32`). Starting with an initial value in the first operand (destination operand), accumulates a CRC32 (polynomial 11EDC6F41H) value for the second operand (source operand) and stores the result in the destination operand. The source operand can be a register or a memory location. The destination operand must be an r32 or r64 register. If the destination is an r64 register, then the 32-bit result is stored in the least significant double word and 00000000H is stored in the most significant double word of the r64 register.
    /// Reference: [Intel x86 docs for CRC32](https://www.felixcloutier.com/x86/CRC32.html)
    fn crc32_64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CRC32_64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPESTRMRRI`.
    fn sse_pcmpestrmrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PCMPESTRMRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPESTRMRMI`.
    fn sse_pcmpestrmrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PCMPESTRMRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPESTRIRRI`.
    fn sse_pcmpestrirri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PCMPESTRIRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPESTRIRMI`.
    fn sse_pcmpestrirmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PCMPESTRIRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPISTRMRRI`.
    fn sse_pcmpistrmrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PCMPISTRMRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPISTRMRMI`.
    fn sse_pcmpistrmrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PCMPISTRMRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPISTRIRRI`.
    fn sse_pcmpistrirri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PCMPISTRIRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPISTRIRMI`.
    fn sse_pcmpistrirmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PCMPISTRIRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
}
