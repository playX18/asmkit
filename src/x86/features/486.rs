pub trait X86486Emitter: Emitter {
    /// Emits `INVLPG8M`.
    fn invlpg8m(&mut self,op0: impl OperandCast) -> () {
        self.emit(INVLPG8M, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `INVD` (`INVD`). Invalidates (flushes) the processor’s internal caches and issues a special-function bus cycle that directs external caches to also flush themselves. Data held in internal caches is not written back to main memory.
    /// Reference: [Intel x86 docs for INVD](https://www.felixcloutier.com/x86/INVD.html)
    fn invd(&mut self,) -> () {
        self.emit(INVD, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WBINVD` (`WBINVD`). Writes back all modified cache lines in the processor’s internal cache to main memory and invalidates (flushes) the internal caches. The instruction then issues a special-function bus cycle that directs external caches to also write back modified data and another bus cycle to indicate that the external caches should be invalidated.
    /// Reference: [Intel x86 docs for WBINVD](https://www.felixcloutier.com/x86/WBINVD.html)
    fn wbinvd(&mut self,) -> () {
        self.emit(WBINVD, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHG8RR` (`CMPXCHG`). Compares the value in the AL, AX, EAX, or RAX register with the first operand (destination operand). If the two values are equal, the second operand (source operand) is loaded into the destination operand. Otherwise, the destination operand is loaded into the AL, AX, EAX or RAX register. RAX register is available only in 64-bit mode.
    /// Reference: [Intel x86 docs for CMPXCHG](https://www.felixcloutier.com/x86/CMPXCHG.html)
    fn cmpxchg8rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMPXCHG8RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHG8MR` (`CMPXCHG`). Compares the value in the AL, AX, EAX, or RAX register with the first operand (destination operand). If the two values are equal, the second operand (source operand) is loaded into the destination operand. Otherwise, the destination operand is loaded into the AL, AX, EAX or RAX register. RAX register is available only in 64-bit mode.
    /// Reference: [Intel x86 docs for CMPXCHG](https://www.felixcloutier.com/x86/CMPXCHG.html)
    fn cmpxchg8mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMPXCHG8MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHG16RR` (`CMPXCHG`). Compares the value in the AL, AX, EAX, or RAX register with the first operand (destination operand). If the two values are equal, the second operand (source operand) is loaded into the destination operand. Otherwise, the destination operand is loaded into the AL, AX, EAX or RAX register. RAX register is available only in 64-bit mode.
    /// Reference: [Intel x86 docs for CMPXCHG](https://www.felixcloutier.com/x86/CMPXCHG.html)
    fn cmpxchg16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMPXCHG16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHG16MR` (`CMPXCHG`). Compares the value in the AL, AX, EAX, or RAX register with the first operand (destination operand). If the two values are equal, the second operand (source operand) is loaded into the destination operand. Otherwise, the destination operand is loaded into the AL, AX, EAX or RAX register. RAX register is available only in 64-bit mode.
    /// Reference: [Intel x86 docs for CMPXCHG](https://www.felixcloutier.com/x86/CMPXCHG.html)
    fn cmpxchg16mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMPXCHG16MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHG32RR` (`CMPXCHG`). Compares the value in the AL, AX, EAX, or RAX register with the first operand (destination operand). If the two values are equal, the second operand (source operand) is loaded into the destination operand. Otherwise, the destination operand is loaded into the AL, AX, EAX or RAX register. RAX register is available only in 64-bit mode.
    /// Reference: [Intel x86 docs for CMPXCHG](https://www.felixcloutier.com/x86/CMPXCHG.html)
    fn cmpxchg32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMPXCHG32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHG32MR` (`CMPXCHG`). Compares the value in the AL, AX, EAX, or RAX register with the first operand (destination operand). If the two values are equal, the second operand (source operand) is loaded into the destination operand. Otherwise, the destination operand is loaded into the AL, AX, EAX or RAX register. RAX register is available only in 64-bit mode.
    /// Reference: [Intel x86 docs for CMPXCHG](https://www.felixcloutier.com/x86/CMPXCHG.html)
    fn cmpxchg32mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMPXCHG32MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHG64RR` (`CMPXCHG`). Compares the value in the AL, AX, EAX, or RAX register with the first operand (destination operand). If the two values are equal, the second operand (source operand) is loaded into the destination operand. Otherwise, the destination operand is loaded into the AL, AX, EAX or RAX register. RAX register is available only in 64-bit mode.
    /// Reference: [Intel x86 docs for CMPXCHG](https://www.felixcloutier.com/x86/CMPXCHG.html)
    fn cmpxchg64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMPXCHG64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CMPXCHG64MR` (`CMPXCHG`). Compares the value in the AL, AX, EAX, or RAX register with the first operand (destination operand). If the two values are equal, the second operand (source operand) is loaded into the destination operand. Otherwise, the destination operand is loaded into the AL, AX, EAX or RAX register. RAX register is available only in 64-bit mode.
    /// Reference: [Intel x86 docs for CMPXCHG](https://www.felixcloutier.com/x86/CMPXCHG.html)
    fn cmpxchg64mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(CMPXCHG64MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XADD8RR` (`XADD`). Exchanges the first operand (destination operand) with the second operand (source operand), then loads the sum of the two values into the destination operand. The destination operand can be a register or a memory location; the source operand is a register.
    /// Reference: [Intel x86 docs for XADD](https://www.felixcloutier.com/x86/XADD.html)
    fn xadd8rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(XADD8RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XADD8MR` (`XADD`). Exchanges the first operand (destination operand) with the second operand (source operand), then loads the sum of the two values into the destination operand. The destination operand can be a register or a memory location; the source operand is a register.
    /// Reference: [Intel x86 docs for XADD](https://www.felixcloutier.com/x86/XADD.html)
    fn xadd8mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(XADD8MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XADD16RR` (`XADD`). Exchanges the first operand (destination operand) with the second operand (source operand), then loads the sum of the two values into the destination operand. The destination operand can be a register or a memory location; the source operand is a register.
    /// Reference: [Intel x86 docs for XADD](https://www.felixcloutier.com/x86/XADD.html)
    fn xadd16rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(XADD16RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XADD16MR` (`XADD`). Exchanges the first operand (destination operand) with the second operand (source operand), then loads the sum of the two values into the destination operand. The destination operand can be a register or a memory location; the source operand is a register.
    /// Reference: [Intel x86 docs for XADD](https://www.felixcloutier.com/x86/XADD.html)
    fn xadd16mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(XADD16MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XADD32RR` (`XADD`). Exchanges the first operand (destination operand) with the second operand (source operand), then loads the sum of the two values into the destination operand. The destination operand can be a register or a memory location; the source operand is a register.
    /// Reference: [Intel x86 docs for XADD](https://www.felixcloutier.com/x86/XADD.html)
    fn xadd32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(XADD32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XADD32MR` (`XADD`). Exchanges the first operand (destination operand) with the second operand (source operand), then loads the sum of the two values into the destination operand. The destination operand can be a register or a memory location; the source operand is a register.
    /// Reference: [Intel x86 docs for XADD](https://www.felixcloutier.com/x86/XADD.html)
    fn xadd32mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(XADD32MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XADD64RR` (`XADD`). Exchanges the first operand (destination operand) with the second operand (source operand), then loads the sum of the two values into the destination operand. The destination operand can be a register or a memory location; the source operand is a register.
    /// Reference: [Intel x86 docs for XADD](https://www.felixcloutier.com/x86/XADD.html)
    fn xadd64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(XADD64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `XADD64MR` (`XADD`). Exchanges the first operand (destination operand) with the second operand (source operand), then loads the sum of the two values into the destination operand. The destination operand can be a register or a memory location; the source operand is a register.
    /// Reference: [Intel x86 docs for XADD](https://www.felixcloutier.com/x86/XADD.html)
    fn xadd64mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(XADD64MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BSWAP16R` (`BSWAP`). Reverses the byte order of a 32-bit or 64-bit (destination) register. This instruction is provided for converting little-endian values to big-endian format and vice versa. To swap bytes in a word value (16-bit register), use the XCHG instruction. When the BSWAP instruction references a 16-bit register, the result is undefined.
    /// Reference: [Intel x86 docs for BSWAP](https://www.felixcloutier.com/x86/BSWAP.html)
    fn bswap16r(&mut self,op0: impl OperandCast) -> () {
        self.emit(BSWAP16R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BSWAP32R` (`BSWAP`). Reverses the byte order of a 32-bit or 64-bit (destination) register. This instruction is provided for converting little-endian values to big-endian format and vice versa. To swap bytes in a word value (16-bit register), use the XCHG instruction. When the BSWAP instruction references a 16-bit register, the result is undefined.
    /// Reference: [Intel x86 docs for BSWAP](https://www.felixcloutier.com/x86/BSWAP.html)
    fn bswap32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(BSWAP32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `BSWAP64R` (`BSWAP`). Reverses the byte order of a 32-bit or 64-bit (destination) register. This instruction is provided for converting little-endian values to big-endian format and vice versa. To swap bytes in a word value (16-bit register), use the XCHG instruction. When the BSWAP instruction references a 16-bit register, the result is undefined.
    /// Reference: [Intel x86 docs for BSWAP](https://www.felixcloutier.com/x86/BSWAP.html)
    fn bswap64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(BSWAP64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
