pub trait X86SSE2Emitter: Emitter {
    /// Emits `MOVNTI32MR` (`MOVNTI`). Moves the doubleword integer in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to minimize cache pollution during the write to memory. The source operand is a general-purpose register. The destination operand is a 32-bit memory location.
    /// Reference: [Intel x86 docs for MOVNTI](https://www.felixcloutier.com/x86/MOVNTI.html)
    fn movnti32mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVNTI32MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MOVNTI64MR` (`MOVNTI`). Moves the doubleword integer in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to minimize cache pollution during the write to memory. The source operand is a general-purpose register. The destination operand is a 32-bit memory location.
    /// Reference: [Intel x86 docs for MOVNTI](https://www.felixcloutier.com/x86/MOVNTI.html)
    fn movnti64mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MOVNTI64MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTPI2PSRR` (`CVTPI2PS`). Converts two packed signed doubleword integers in the source operand (second operand) to two packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPI2PS](https://www.felixcloutier.com/x86/CVTPI2PS.html)
    fn mmx_cvtpi2psrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTPI2PSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTPI2PSRM` (`CVTPI2PS`). Converts two packed signed doubleword integers in the source operand (second operand) to two packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPI2PS](https://www.felixcloutier.com/x86/CVTPI2PS.html)
    fn mmx_cvtpi2psrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTPI2PSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTPI2PDRR` (`CVTPI2PD`). Converts two packed signed doubleword integers in the source operand (second operand) to two packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPI2PD](https://www.felixcloutier.com/x86/CVTPI2PD.html)
    fn mmx_cvtpi2pdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTPI2PDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTPI2PDRM` (`CVTPI2PD`). Converts two packed signed doubleword integers in the source operand (second operand) to two packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPI2PD](https://www.felixcloutier.com/x86/CVTPI2PD.html)
    fn mmx_cvtpi2pdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTPI2PDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTTPS2PIRR` (`CVTTPS2PI`). Converts two packed single precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is an MMX technology register. When the source operand is an XMM register, the two single precision floating-point values are contained in the low quadword of the register.
    /// Reference: [Intel x86 docs for CVTTPS2PI](https://www.felixcloutier.com/x86/CVTTPS2PI.html)
    fn mmx_cvttps2pirr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTTPS2PIRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTTPS2PIRM` (`CVTTPS2PI`). Converts two packed single precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is an MMX technology register. When the source operand is an XMM register, the two single precision floating-point values are contained in the low quadword of the register.
    /// Reference: [Intel x86 docs for CVTTPS2PI](https://www.felixcloutier.com/x86/CVTTPS2PI.html)
    fn mmx_cvttps2pirm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTTPS2PIRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTTPD2PIRR` (`CVTTPD2PI`). Converts two packed double precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand). The source operand can be an XMM register or a 128-bit memory location. The destination operand is an MMX technology register.
    /// Reference: [Intel x86 docs for CVTTPD2PI](https://www.felixcloutier.com/x86/CVTTPD2PI.html)
    fn mmx_cvttpd2pirr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTTPD2PIRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTTPD2PIRM` (`CVTTPD2PI`). Converts two packed double precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand). The source operand can be an XMM register or a 128-bit memory location. The destination operand is an MMX technology register.
    /// Reference: [Intel x86 docs for CVTTPD2PI](https://www.felixcloutier.com/x86/CVTTPD2PI.html)
    fn mmx_cvttpd2pirm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTTPD2PIRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTPS2PIRR` (`CVTPS2PI`). Converts two packed single precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPS2PI](https://www.felixcloutier.com/x86/CVTPS2PI.html)
    fn mmx_cvtps2pirr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTPS2PIRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTPS2PIRM` (`CVTPS2PI`). Converts two packed single precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPS2PI](https://www.felixcloutier.com/x86/CVTPS2PI.html)
    fn mmx_cvtps2pirm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTPS2PIRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTPD2PIRR` (`CVTPD2PI`). Converts two packed double precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPD2PI](https://www.felixcloutier.com/x86/CVTPD2PI.html)
    fn mmx_cvtpd2pirr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTPD2PIRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MMX_CVTPD2PIRM` (`CVTPD2PI`). Converts two packed double precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPD2PI](https://www.felixcloutier.com/x86/CVTPD2PI.html)
    fn mmx_cvtpd2pirm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(MMX_CVTPD2PIRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVUPDRR` (`MOVUPD`). Note: VEX.vvvv and EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVUPD](https://www.felixcloutier.com/x86/MOVUPD.html)
    fn sse_movupdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVUPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVUPDRM` (`MOVUPD`). Note: VEX.vvvv and EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVUPD](https://www.felixcloutier.com/x86/MOVUPD.html)
    fn sse_movupdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVUPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVSDRR` (`MOVSD`). Moves the byte, word, or doubleword specified with the second operand (source operand) to the location specified with the first operand (destination operand). Both the source and destination operands are located in memory. The address of the source operand is read from the DS:ESI or the DS:SI registers (depending on the address-size attribute of the instruction, 32 or 16, respectively). The address of the destination operand is read from the ES:EDI or the ES:DI registers (again depending on the address-size attribute of the instruction). The DS segment may be overridden with a segment override prefix, but the ES segment cannot be overridden.
    /// Reference: [Intel x86 docs for MOVSD](https://www.felixcloutier.com/x86/MOVS%3AMOVSB%3AMOVSW%3AMOVSD%3AMOVSQ.html)
    fn sse_movsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVSDRM` (`MOVSD`). Moves the byte, word, or doubleword specified with the second operand (source operand) to the location specified with the first operand (destination operand). Both the source and destination operands are located in memory. The address of the source operand is read from the DS:ESI or the DS:SI registers (depending on the address-size attribute of the instruction, 32 or 16, respectively). The address of the destination operand is read from the ES:EDI or the ES:DI registers (again depending on the address-size attribute of the instruction). The DS segment may be overridden with a segment override prefix, but the ES segment cannot be overridden.
    /// Reference: [Intel x86 docs for MOVSD](https://www.felixcloutier.com/x86/MOVS%3AMOVSB%3AMOVSW%3AMOVSD%3AMOVSQ.html)
    fn sse_movsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVUPDMR` (`MOVUPD`). Note: VEX.vvvv and EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVUPD](https://www.felixcloutier.com/x86/MOVUPD.html)
    fn sse_movupdmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVUPDMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVSDMR` (`MOVSD`). Moves the byte, word, or doubleword specified with the second operand (source operand) to the location specified with the first operand (destination operand). Both the source and destination operands are located in memory. The address of the source operand is read from the DS:ESI or the DS:SI registers (depending on the address-size attribute of the instruction, 32 or 16, respectively). The address of the destination operand is read from the ES:EDI or the ES:DI registers (again depending on the address-size attribute of the instruction). The DS segment may be overridden with a segment override prefix, but the ES segment cannot be overridden.
    /// Reference: [Intel x86 docs for MOVSD](https://www.felixcloutier.com/x86/MOVS%3AMOVSB%3AMOVSW%3AMOVSD%3AMOVSQ.html)
    fn sse_movsdmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVSDMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVLPDRM` (`MOVLPD`). This instruction cannot be used for register to register or memory to memory moves.
    /// Reference: [Intel x86 docs for MOVLPD](https://www.felixcloutier.com/x86/MOVLPD.html)
    fn sse_movlpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVLPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVLPDMR` (`MOVLPD`). This instruction cannot be used for register to register or memory to memory moves.
    /// Reference: [Intel x86 docs for MOVLPD](https://www.felixcloutier.com/x86/MOVLPD.html)
    fn sse_movlpdmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVLPDMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UNPCKLPDRR` (`UNPCKLPD`). Performs an interleaved unpack of the low double precision floating-point values from the first source operand and the second source operand.
    /// Reference: [Intel x86 docs for UNPCKLPD](https://www.felixcloutier.com/x86/UNPCKLPD.html)
    fn sse_unpcklpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UNPCKLPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UNPCKLPDRM` (`UNPCKLPD`). Performs an interleaved unpack of the low double precision floating-point values from the first source operand and the second source operand.
    /// Reference: [Intel x86 docs for UNPCKLPD](https://www.felixcloutier.com/x86/UNPCKLPD.html)
    fn sse_unpcklpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UNPCKLPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UNPCKHPDRR` (`UNPCKHPD`). Performs an interleaved unpack of the high double precision floating-point values from the first source operand and the second source operand. See Figure 4-15 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 2B.
    /// Reference: [Intel x86 docs for UNPCKHPD](https://www.felixcloutier.com/x86/UNPCKHPD.html)
    fn sse_unpckhpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UNPCKHPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UNPCKHPDRM` (`UNPCKHPD`). Performs an interleaved unpack of the high double precision floating-point values from the first source operand and the second source operand. See Figure 4-15 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 2B.
    /// Reference: [Intel x86 docs for UNPCKHPD](https://www.felixcloutier.com/x86/UNPCKHPD.html)
    fn sse_unpckhpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UNPCKHPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVHPDRM` (`MOVHPD`). This instruction cannot be used for register to register or memory to memory moves.
    /// Reference: [Intel x86 docs for MOVHPD](https://www.felixcloutier.com/x86/MOVHPD.html)
    fn sse_movhpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVHPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVHPDMR` (`MOVHPD`). This instruction cannot be used for register to register or memory to memory moves.
    /// Reference: [Intel x86 docs for MOVHPD](https://www.felixcloutier.com/x86/MOVHPD.html)
    fn sse_movhpdmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVHPDMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVAPDRR` (`MOVAPD`). Moves 2, 4 or 8 double precision floating-point values from the source operand (second operand) to the destination operand (first operand). This instruction can be used to load an XMM, YMM or ZMM register from an 128-bit, 256-bit or 512-bit memory location, to store the contents of an XMM, YMM or ZMM register into a 128-bit, 256-bit or 512-bit memory location, or to move data between two XMM, two YMM or two ZMM registers.
    /// Reference: [Intel x86 docs for MOVAPD](https://www.felixcloutier.com/x86/MOVAPD.html)
    fn sse_movapdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVAPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVAPDRM` (`MOVAPD`). Moves 2, 4 or 8 double precision floating-point values from the source operand (second operand) to the destination operand (first operand). This instruction can be used to load an XMM, YMM or ZMM register from an 128-bit, 256-bit or 512-bit memory location, to store the contents of an XMM, YMM or ZMM register into a 128-bit, 256-bit or 512-bit memory location, or to move data between two XMM, two YMM or two ZMM registers.
    /// Reference: [Intel x86 docs for MOVAPD](https://www.felixcloutier.com/x86/MOVAPD.html)
    fn sse_movapdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVAPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVAPDMR` (`MOVAPD`). Moves 2, 4 or 8 double precision floating-point values from the source operand (second operand) to the destination operand (first operand). This instruction can be used to load an XMM, YMM or ZMM register from an 128-bit, 256-bit or 512-bit memory location, to store the contents of an XMM, YMM or ZMM register into a 128-bit, 256-bit or 512-bit memory location, or to move data between two XMM, two YMM or two ZMM registers.
    /// Reference: [Intel x86 docs for MOVAPD](https://www.felixcloutier.com/x86/MOVAPD.html)
    fn sse_movapdmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVAPDMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSI2SD32RR` (`CVTSI2SD`). Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a double precision floating-point value in the destination operand. The result is stored in the low quadword of the destination operand, and the high quadword left unchanged. When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.
    /// Reference: [Intel x86 docs for CVTSI2SD](https://www.felixcloutier.com/x86/CVTSI2SD.html)
    fn sse_cvtsi2sd32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSI2SD32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSI2SD32RM` (`CVTSI2SD`). Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a double precision floating-point value in the destination operand. The result is stored in the low quadword of the destination operand, and the high quadword left unchanged. When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.
    /// Reference: [Intel x86 docs for CVTSI2SD](https://www.felixcloutier.com/x86/CVTSI2SD.html)
    fn sse_cvtsi2sd32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSI2SD32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSI2SD64RR` (`CVTSI2SD`). Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a double precision floating-point value in the destination operand. The result is stored in the low quadword of the destination operand, and the high quadword left unchanged. When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.
    /// Reference: [Intel x86 docs for CVTSI2SD](https://www.felixcloutier.com/x86/CVTSI2SD.html)
    fn sse_cvtsi2sd64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSI2SD64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSI2SD64RM` (`CVTSI2SD`). Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a double precision floating-point value in the destination operand. The result is stored in the low quadword of the destination operand, and the high quadword left unchanged. When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.
    /// Reference: [Intel x86 docs for CVTSI2SD](https://www.felixcloutier.com/x86/CVTSI2SD.html)
    fn sse_cvtsi2sd64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSI2SD64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVNTPDMR` (`MOVNTPD`). Moves the packed double precision floating-point values in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to prevent caching of the data during the write to memory. The source operand is an XMM register, YMM register or ZMM register, which is assumed to contain packed double precision, floating-pointing data. The destination operand is a 128-bit, 256-bit or 512-bit memory location. The memory operand must be aligned on a 16-byte (128-bit version), 32-byte (VEX.256 encoded version) or 64-byte (EVEX.512 encoded version) boundary otherwise a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for MOVNTPD](https://www.felixcloutier.com/x86/MOVNTPD.html)
    fn sse_movntpdmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTPDMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVNTSDMR`.
    fn sse_movntsdmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTSDMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTSD2SI32RR` (`CVTTSD2SI`). Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
    /// Reference: [Intel x86 docs for CVTTSD2SI](https://www.felixcloutier.com/x86/CVTTSD2SI.html)
    fn sse_cvttsd2si32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTSD2SI32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTSD2SI32RM` (`CVTTSD2SI`). Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
    /// Reference: [Intel x86 docs for CVTTSD2SI](https://www.felixcloutier.com/x86/CVTTSD2SI.html)
    fn sse_cvttsd2si32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTSD2SI32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTSD2SI64RR` (`CVTTSD2SI`). Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
    /// Reference: [Intel x86 docs for CVTTSD2SI](https://www.felixcloutier.com/x86/CVTTSD2SI.html)
    fn sse_cvttsd2si64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTSD2SI64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTSD2SI64RM` (`CVTTSD2SI`). Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
    /// Reference: [Intel x86 docs for CVTTSD2SI](https://www.felixcloutier.com/x86/CVTTSD2SI.html)
    fn sse_cvttsd2si64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTSD2SI64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSD2SI32RR` (`CVTSD2SI`). Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer in the destination operand (first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
    /// Reference: [Intel x86 docs for CVTSD2SI](https://www.felixcloutier.com/x86/CVTSD2SI.html)
    fn sse_cvtsd2si32rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSD2SI32RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSD2SI32RM` (`CVTSD2SI`). Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer in the destination operand (first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
    /// Reference: [Intel x86 docs for CVTSD2SI](https://www.felixcloutier.com/x86/CVTSD2SI.html)
    fn sse_cvtsd2si32rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSD2SI32RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSD2SI64RR` (`CVTSD2SI`). Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer in the destination operand (first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
    /// Reference: [Intel x86 docs for CVTSD2SI](https://www.felixcloutier.com/x86/CVTSD2SI.html)
    fn sse_cvtsd2si64rr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSD2SI64RR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSD2SI64RM` (`CVTSD2SI`). Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer in the destination operand (first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
    /// Reference: [Intel x86 docs for CVTSD2SI](https://www.felixcloutier.com/x86/CVTSD2SI.html)
    fn sse_cvtsd2si64rm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSD2SI64RM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UCOMISDRR` (`UCOMISD`). Performs an unordered compare of the double precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    /// Reference: [Intel x86 docs for UCOMISD](https://www.felixcloutier.com/x86/UCOMISD.html)
    fn sse_ucomisdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UCOMISDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_UCOMISDRM` (`UCOMISD`). Performs an unordered compare of the double precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    /// Reference: [Intel x86 docs for UCOMISD](https://www.felixcloutier.com/x86/UCOMISD.html)
    fn sse_ucomisdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_UCOMISDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_COMISDRR` (`COMISD`). Compares the double precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    /// Reference: [Intel x86 docs for COMISD](https://www.felixcloutier.com/x86/COMISD.html)
    fn sse_comisdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_COMISDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_COMISDRM` (`COMISD`). Compares the double precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    /// Reference: [Intel x86 docs for COMISD](https://www.felixcloutier.com/x86/COMISD.html)
    fn sse_comisdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_COMISDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVMSKPDRR` (`MOVMSKPD`). Extracts the sign bits from the packed double precision floating-point values in the source operand (second operand), formats them into a 2-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM register, and the destination operand is a general-purpose register. The mask is stored in the 2 low-order bits of the destination operand. Zero-extend the upper bits of the destination.
    /// Reference: [Intel x86 docs for MOVMSKPD](https://www.felixcloutier.com/x86/MOVMSKPD.html)
    fn sse_movmskpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVMSKPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SQRTPDRR` (`SQRTPD`). Performs a SIMD computation of the square roots of the two, four or eight packed double precision floating-point values in the source operand (the second operand) stores the packed double precision floating-point results in the destination operand (the first operand).
    /// Reference: [Intel x86 docs for SQRTPD](https://www.felixcloutier.com/x86/SQRTPD.html)
    fn sse_sqrtpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SQRTPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SQRTPDRM` (`SQRTPD`). Performs a SIMD computation of the square roots of the two, four or eight packed double precision floating-point values in the source operand (the second operand) stores the packed double precision floating-point results in the destination operand (the first operand).
    /// Reference: [Intel x86 docs for SQRTPD](https://www.felixcloutier.com/x86/SQRTPD.html)
    fn sse_sqrtpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SQRTPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SQRTSDRR` (`SQRTSD`). Computes the square root of the low double precision floating-point value in the second source operand and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination operands are XMM registers.
    /// Reference: [Intel x86 docs for SQRTSD](https://www.felixcloutier.com/x86/SQRTSD.html)
    fn sse_sqrtsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SQRTSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SQRTSDRM` (`SQRTSD`). Computes the square root of the low double precision floating-point value in the second source operand and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination operands are XMM registers.
    /// Reference: [Intel x86 docs for SQRTSD](https://www.felixcloutier.com/x86/SQRTSD.html)
    fn sse_sqrtsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SQRTSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ANDPDRR` (`ANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for ANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn sse_andpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ANDPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ANDPDRM` (`ANDPD`). Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for ANDPD](https://www.felixcloutier.com/x86/ANDPD.html)
    fn sse_andpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ANDPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ANDNPDRR` (`ANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for ANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn sse_andnpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ANDNPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ANDNPDRM` (`ANDNPD`). Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for ANDNPD](https://www.felixcloutier.com/x86/ANDNPD.html)
    fn sse_andnpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ANDNPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ORPDRR` (`ORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for ORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn sse_orpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ORPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ORPDRM` (`ORPD`). Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for ORPD](https://www.felixcloutier.com/x86/ORPD.html)
    fn sse_orpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ORPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_XORPDRR` (`XORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for XORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn sse_xorpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_XORPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_XORPDRM` (`XORPD`). Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for XORPD](https://www.felixcloutier.com/x86/XORPD.html)
    fn sse_xorpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_XORPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDPDRR` (`ADDPD`). Adds two, four or eight packed double precision floating-point values from the first source operand to the second source operand, and stores the packed double precision floating-point result in the destination operand.
    /// Reference: [Intel x86 docs for ADDPD](https://www.felixcloutier.com/x86/ADDPD.html)
    fn sse_addpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDPDRM` (`ADDPD`). Adds two, four or eight packed double precision floating-point values from the first source operand to the second source operand, and stores the packed double precision floating-point result in the destination operand.
    /// Reference: [Intel x86 docs for ADDPD](https://www.felixcloutier.com/x86/ADDPD.html)
    fn sse_addpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDSDRR` (`ADDSD`). Adds the low double precision floating-point values from the second source operand and the first source operand and stores the double precision floating-point result in the destination operand.
    /// Reference: [Intel x86 docs for ADDSD](https://www.felixcloutier.com/x86/ADDSD.html)
    fn sse_addsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_ADDSDRM` (`ADDSD`). Adds the low double precision floating-point values from the second source operand and the first source operand and stores the double precision floating-point result in the destination operand.
    /// Reference: [Intel x86 docs for ADDSD](https://www.felixcloutier.com/x86/ADDSD.html)
    fn sse_addsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_ADDSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MULPDRR` (`MULPD`). Multiply packed double precision floating-point values from the first source operand with corresponding values in the second source operand, and stores the packed double precision floating-point results in the destination operand.
    /// Reference: [Intel x86 docs for MULPD](https://www.felixcloutier.com/x86/MULPD.html)
    fn sse_mulpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MULPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MULPDRM` (`MULPD`). Multiply packed double precision floating-point values from the first source operand with corresponding values in the second source operand, and stores the packed double precision floating-point results in the destination operand.
    /// Reference: [Intel x86 docs for MULPD](https://www.felixcloutier.com/x86/MULPD.html)
    fn sse_mulpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MULPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MULSDRR` (`MULSD`). Multiplies the low double precision floating-point value in the second source operand by the low double precision floating-point value in the first source operand, and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source operand and the destination operands are XMM registers.
    /// Reference: [Intel x86 docs for MULSD](https://www.felixcloutier.com/x86/MULSD.html)
    fn sse_mulsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MULSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MULSDRM` (`MULSD`). Multiplies the low double precision floating-point value in the second source operand by the low double precision floating-point value in the first source operand, and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source operand and the destination operands are XMM registers.
    /// Reference: [Intel x86 docs for MULSD](https://www.felixcloutier.com/x86/MULSD.html)
    fn sse_mulsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MULSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTPS2PDRR` (`CVTPS2PD`). Converts two, four or eight packed single precision floating-point values in the source operand (second operand) to two, four or eight packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPS2PD](https://www.felixcloutier.com/x86/CVTPS2PD.html)
    fn sse_cvtps2pdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTPS2PDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTPS2PDRM` (`CVTPS2PD`). Converts two, four or eight packed single precision floating-point values in the source operand (second operand) to two, four or eight packed double precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPS2PD](https://www.felixcloutier.com/x86/CVTPS2PD.html)
    fn sse_cvtps2pdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTPS2PDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTPD2PSRR` (`CVTPD2PS`). Converts two, four or eight packed double precision floating-point values in the source operand (second operand) to two, four or eight packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPD2PS](https://www.felixcloutier.com/x86/CVTPD2PS.html)
    fn sse_cvtpd2psrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTPD2PSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTPD2PSRM` (`CVTPD2PS`). Converts two, four or eight packed double precision floating-point values in the source operand (second operand) to two, four or eight packed single precision floating-point values in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPD2PS](https://www.felixcloutier.com/x86/CVTPD2PS.html)
    fn sse_cvtpd2psrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTPD2PSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSS2SDRR` (`CVTSS2SD`). Converts a single precision floating-point value in the “convert-from” source operand to a double precision floating-point value in the destination operand. When the “convert-from” source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register. The result is stored in the low quadword of the destination operand.
    /// Reference: [Intel x86 docs for CVTSS2SD](https://www.felixcloutier.com/x86/CVTSS2SD.html)
    fn sse_cvtss2sdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSS2SDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSS2SDRM` (`CVTSS2SD`). Converts a single precision floating-point value in the “convert-from” source operand to a double precision floating-point value in the destination operand. When the “convert-from” source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register. The result is stored in the low quadword of the destination operand.
    /// Reference: [Intel x86 docs for CVTSS2SD](https://www.felixcloutier.com/x86/CVTSS2SD.html)
    fn sse_cvtss2sdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSS2SDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSD2SSRR` (`CVTSD2SS`). Converts a double precision floating-point value in the “convert-from” source operand (the second operand in SSE2 version, otherwise the third operand) to a single precision floating-point value in the destination operand.
    /// Reference: [Intel x86 docs for CVTSD2SS](https://www.felixcloutier.com/x86/CVTSD2SS.html)
    fn sse_cvtsd2ssrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSD2SSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTSD2SSRM` (`CVTSD2SS`). Converts a double precision floating-point value in the “convert-from” source operand (the second operand in SSE2 version, otherwise the third operand) to a single precision floating-point value in the destination operand.
    /// Reference: [Intel x86 docs for CVTSD2SS](https://www.felixcloutier.com/x86/CVTSD2SS.html)
    fn sse_cvtsd2ssrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTSD2SSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTDQ2PSRR` (`CVTDQ2PS`). Converts four, eight or sixteen packed signed doubleword integers in the source operand to four, eight or sixteen packed single precision floating-point values in the destination operand.
    /// Reference: [Intel x86 docs for CVTDQ2PS](https://www.felixcloutier.com/x86/CVTDQ2PS.html)
    fn sse_cvtdq2psrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTDQ2PSRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTDQ2PSRM` (`CVTDQ2PS`). Converts four, eight or sixteen packed signed doubleword integers in the source operand to four, eight or sixteen packed single precision floating-point values in the destination operand.
    /// Reference: [Intel x86 docs for CVTDQ2PS](https://www.felixcloutier.com/x86/CVTDQ2PS.html)
    fn sse_cvtdq2psrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTDQ2PSRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTPS2DQRR` (`CVTPS2DQ`). Converts four, eight or sixteen packed single precision floating-point values in the source operand to four, eight or sixteen signed doubleword integers in the destination operand.
    /// Reference: [Intel x86 docs for CVTPS2DQ](https://www.felixcloutier.com/x86/CVTPS2DQ.html)
    fn sse_cvtps2dqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTPS2DQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTPS2DQRM` (`CVTPS2DQ`). Converts four, eight or sixteen packed single precision floating-point values in the source operand to four, eight or sixteen signed doubleword integers in the destination operand.
    /// Reference: [Intel x86 docs for CVTPS2DQ](https://www.felixcloutier.com/x86/CVTPS2DQ.html)
    fn sse_cvtps2dqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTPS2DQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTPS2DQRR` (`CVTTPS2DQ`). Converts four, eight or sixteen packed single precision floating-point values in the source operand to four, eight or sixteen signed doubleword integers in the destination operand.
    /// Reference: [Intel x86 docs for CVTTPS2DQ](https://www.felixcloutier.com/x86/CVTTPS2DQ.html)
    fn sse_cvttps2dqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTPS2DQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTPS2DQRM` (`CVTTPS2DQ`). Converts four, eight or sixteen packed single precision floating-point values in the source operand to four, eight or sixteen signed doubleword integers in the destination operand.
    /// Reference: [Intel x86 docs for CVTTPS2DQ](https://www.felixcloutier.com/x86/CVTTPS2DQ.html)
    fn sse_cvttps2dqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTPS2DQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SUBPDRR` (`SUBPD`). Performs a SIMD subtract of the two, four or eight packed double precision floating-point values of the second Source operand from the first Source operand, and stores the packed double precision floating-point results in the destination operand.
    /// Reference: [Intel x86 docs for SUBPD](https://www.felixcloutier.com/x86/SUBPD.html)
    fn sse_subpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SUBPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SUBPDRM` (`SUBPD`). Performs a SIMD subtract of the two, four or eight packed double precision floating-point values of the second Source operand from the first Source operand, and stores the packed double precision floating-point results in the destination operand.
    /// Reference: [Intel x86 docs for SUBPD](https://www.felixcloutier.com/x86/SUBPD.html)
    fn sse_subpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SUBPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SUBSDRR` (`SUBSD`). Subtract the low double precision floating-point value in the second source operand from the first source operand and stores the double precision floating-point result in the low quadword of the destination operand.
    /// Reference: [Intel x86 docs for SUBSD](https://www.felixcloutier.com/x86/SUBSD.html)
    fn sse_subsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SUBSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_SUBSDRM` (`SUBSD`). Subtract the low double precision floating-point value in the second source operand from the first source operand and stores the double precision floating-point result in the low quadword of the destination operand.
    /// Reference: [Intel x86 docs for SUBSD](https://www.felixcloutier.com/x86/SUBSD.html)
    fn sse_subsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_SUBSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MINPDRR` (`MINPD`). Performs a SIMD compare of the packed double precision floating-point values in the first source operand and the second source operand and returns the minimum value for each pair of values to the destination operand.
    /// Reference: [Intel x86 docs for MINPD](https://www.felixcloutier.com/x86/MINPD.html)
    fn sse_minpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MINPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MINPDRM` (`MINPD`). Performs a SIMD compare of the packed double precision floating-point values in the first source operand and the second source operand and returns the minimum value for each pair of values to the destination operand.
    /// Reference: [Intel x86 docs for MINPD](https://www.felixcloutier.com/x86/MINPD.html)
    fn sse_minpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MINPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MINSDRR` (`MINSD`). Compares the low double precision floating-point values in the first source operand and the second source operand, and returns the minimum value to the low quadword of the destination operand. When the source operand is a memory operand, only the 64 bits are accessed.
    /// Reference: [Intel x86 docs for MINSD](https://www.felixcloutier.com/x86/MINSD.html)
    fn sse_minsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MINSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MINSDRM` (`MINSD`). Compares the low double precision floating-point values in the first source operand and the second source operand, and returns the minimum value to the low quadword of the destination operand. When the source operand is a memory operand, only the 64 bits are accessed.
    /// Reference: [Intel x86 docs for MINSD](https://www.felixcloutier.com/x86/MINSD.html)
    fn sse_minsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MINSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_DIVPDRR` (`DIVPD`). Performs a SIMD divide of the double precision floating-point values in the first source operand by the floating-point values in the second source operand (the third operand). Results are written to the destination operand (the first operand).
    /// Reference: [Intel x86 docs for DIVPD](https://www.felixcloutier.com/x86/DIVPD.html)
    fn sse_divpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_DIVPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_DIVPDRM` (`DIVPD`). Performs a SIMD divide of the double precision floating-point values in the first source operand by the floating-point values in the second source operand (the third operand). Results are written to the destination operand (the first operand).
    /// Reference: [Intel x86 docs for DIVPD](https://www.felixcloutier.com/x86/DIVPD.html)
    fn sse_divpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_DIVPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_DIVSDRR` (`DIVSD`). Divides the low double precision floating-point value in the first source operand by the low double precision floating-point value in the second source operand, and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination are XMM registers.
    /// Reference: [Intel x86 docs for DIVSD](https://www.felixcloutier.com/x86/DIVSD.html)
    fn sse_divsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_DIVSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_DIVSDRM` (`DIVSD`). Divides the low double precision floating-point value in the first source operand by the low double precision floating-point value in the second source operand, and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination are XMM registers.
    /// Reference: [Intel x86 docs for DIVSD](https://www.felixcloutier.com/x86/DIVSD.html)
    fn sse_divsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_DIVSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MAXPDRR` (`MAXPD`). Performs a SIMD compare of the packed double precision floating-point values in the first source operand and the second source operand and returns the maximum value for each pair of values to the destination operand.
    /// Reference: [Intel x86 docs for MAXPD](https://www.felixcloutier.com/x86/MAXPD.html)
    fn sse_maxpdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MAXPDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MAXPDRM` (`MAXPD`). Performs a SIMD compare of the packed double precision floating-point values in the first source operand and the second source operand and returns the maximum value for each pair of values to the destination operand.
    /// Reference: [Intel x86 docs for MAXPD](https://www.felixcloutier.com/x86/MAXPD.html)
    fn sse_maxpdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MAXPDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MAXSDRR` (`MAXSD`). Compares the low double precision floating-point values in the first source operand and the second source operand, and returns the maximum value to the low quadword of the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination operands are XMM registers. When the second source operand is a memory operand, only 64 bits are accessed.
    /// Reference: [Intel x86 docs for MAXSD](https://www.felixcloutier.com/x86/MAXSD.html)
    fn sse_maxsdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MAXSDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MAXSDRM` (`MAXSD`). Compares the low double precision floating-point values in the first source operand and the second source operand, and returns the maximum value to the low quadword of the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination operands are XMM registers. When the second source operand is a memory operand, only 64 bits are accessed.
    /// Reference: [Intel x86 docs for MAXSD](https://www.felixcloutier.com/x86/MAXSD.html)
    fn sse_maxsdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MAXSDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKLBWRR` (`PUNPCKLBW`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLBW](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn sse_punpcklbwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKLBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKLBWRM` (`PUNPCKLBW`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLBW](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn sse_punpcklbwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKLBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKLWDRR` (`PUNPCKLWD`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLWD](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn sse_punpcklwdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKLWDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKLWDRM` (`PUNPCKLWD`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLWD](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn sse_punpcklwdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKLWDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKLDQRR` (`PUNPCKLDQ`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLDQ](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn sse_punpckldqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKLDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKLDQRM` (`PUNPCKLDQ`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLDQ](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn sse_punpckldqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKLDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PACKSSWBRR` (`PACKSSWB`). Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
    /// Reference: [Intel x86 docs for PACKSSWB](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html)
    fn sse_packsswbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PACKSSWBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PACKSSWBRM` (`PACKSSWB`). Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
    /// Reference: [Intel x86 docs for PACKSSWB](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html)
    fn sse_packsswbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PACKSSWBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPGTBRR` (`PCMPGTB`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTB](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn sse_pcmpgtbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPGTBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPGTBRM` (`PCMPGTB`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTB](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn sse_pcmpgtbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPGTBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPGTWRR` (`PCMPGTW`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTW](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn sse_pcmpgtwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPGTWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPGTWRM` (`PCMPGTW`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTW](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn sse_pcmpgtwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPGTWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPGTDRR` (`PCMPGTD`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTD](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn sse_pcmpgtdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPGTDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPGTDRM` (`PCMPGTD`). Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPGTD](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html)
    fn sse_pcmpgtdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPGTDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PACKUSWBRR` (`PACKUSWB`). Converts 4, 8, 16, or 32 signed word integers from the destination operand (first operand) and 4, 8, 16, or 32 signed word integers from the source operand (second operand) into 8, 16, 32 or 64 unsigned byte integers and stores the result in the destination operand. (See Figure 4-6 for an example of the packing operation.) If a signed word integer value is beyond the range of an unsigned byte integer (that is, greater than FFH or less than 00H), the saturated unsigned byte integer value of FFH or 00H, respectively, is stored in the destination.
    /// Reference: [Intel x86 docs for PACKUSWB](https://www.felixcloutier.com/x86/PACKUSWB.html)
    fn sse_packuswbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PACKUSWBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PACKUSWBRM` (`PACKUSWB`). Converts 4, 8, 16, or 32 signed word integers from the destination operand (first operand) and 4, 8, 16, or 32 signed word integers from the source operand (second operand) into 8, 16, 32 or 64 unsigned byte integers and stores the result in the destination operand. (See Figure 4-6 for an example of the packing operation.) If a signed word integer value is beyond the range of an unsigned byte integer (that is, greater than FFH or less than 00H), the saturated unsigned byte integer value of FFH or 00H, respectively, is stored in the destination.
    /// Reference: [Intel x86 docs for PACKUSWB](https://www.felixcloutier.com/x86/PACKUSWB.html)
    fn sse_packuswbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PACKUSWBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKHBWRR` (`PUNPCKHBW`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHBW](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn sse_punpckhbwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKHBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKHBWRM` (`PUNPCKHBW`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHBW](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn sse_punpckhbwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKHBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKHWDRR` (`PUNPCKHWD`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHWD](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn sse_punpckhwdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKHWDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKHWDRM` (`PUNPCKHWD`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHWD](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn sse_punpckhwdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKHWDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKHDQRR` (`PUNPCKHDQ`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHDQ](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn sse_punpckhdqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKHDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKHDQRM` (`PUNPCKHDQ`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHDQ](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn sse_punpckhdqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKHDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PACKSSDWRR` (`PACKSSDW`). Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
    /// Reference: [Intel x86 docs for PACKSSDW](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html)
    fn sse_packssdwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PACKSSDWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PACKSSDWRM` (`PACKSSDW`). Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
    /// Reference: [Intel x86 docs for PACKSSDW](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html)
    fn sse_packssdwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PACKSSDWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKLQDQRR` (`PUNPCKLQDQ`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLQDQ](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn sse_punpcklqdqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKLQDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKLQDQRM` (`PUNPCKLQDQ`). Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKLQDQ](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html)
    fn sse_punpcklqdqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKLQDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKHQDQRR` (`PUNPCKHQDQ`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHQDQ](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn sse_punpckhqdqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKHQDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PUNPCKHQDQRM` (`PUNPCKHQDQ`). Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    /// Reference: [Intel x86 docs for PUNPCKHQDQ](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html)
    fn sse_punpckhqdqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PUNPCKHQDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVD_G2XRR` (`MOVD`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVD](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn sse_movd_g2xrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVD_G2XRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVD_G2XRM` (`MOVD`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVD](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn sse_movd_g2xrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVD_G2XRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVQ_G2XRR` (`MOVQ`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVQ](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn sse_movq_g2xrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVQ_G2XRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVQ_G2XRM` (`MOVQ`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVQ](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn sse_movq_g2xrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVQ_G2XRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVDQARR` (`MOVDQA`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVDQA](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html)
    fn sse_movdqarr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVDQARR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVDQARM` (`MOVDQA`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVDQA](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html)
    fn sse_movdqarm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVDQARM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVDQURR` (`MOVDQU`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVDQU](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html)
    fn sse_movdqurr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVDQURR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVDQURM` (`MOVDQU`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVDQU](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html)
    fn sse_movdqurm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVDQURM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSHUFDRRI` (`PSHUFD`). Copies doublewords from source operand (second operand) and inserts them in the destination operand (first operand) at the locations selected with the order operand (third operand). Figure 4-16 shows the operation of the 256-bit VPSHUFD instruction and the encoding of the order operand. Each 2-bit field in the order operand selects the contents of one doubleword location within a 128-bit lane and copy to the target element in the destination operand. For example, bits 0 and 1 of the order operand targets the first doubleword element in the low and high 128-bit lane of the destination operand for 256-bit VPSHUFD. The encoded value of bits 1:0 of the order operand (see the field encoding in Figure 4-16) determines which doubleword element (from the respective 128-bit lane) of the source operand will be copied to doubleword 0 of the destination operand.
    /// Reference: [Intel x86 docs for PSHUFD](https://www.felixcloutier.com/x86/PSHUFD.html)
    fn sse_pshufdrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PSHUFDRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PSHUFDRMI` (`PSHUFD`). Copies doublewords from source operand (second operand) and inserts them in the destination operand (first operand) at the locations selected with the order operand (third operand). Figure 4-16 shows the operation of the 256-bit VPSHUFD instruction and the encoding of the order operand. Each 2-bit field in the order operand selects the contents of one doubleword location within a 128-bit lane and copy to the target element in the destination operand. For example, bits 0 and 1 of the order operand targets the first doubleword element in the low and high 128-bit lane of the destination operand for 256-bit VPSHUFD. The encoded value of bits 1:0 of the order operand (see the field encoding in Figure 4-16) determines which doubleword element (from the respective 128-bit lane) of the source operand will be copied to doubleword 0 of the destination operand.
    /// Reference: [Intel x86 docs for PSHUFD](https://www.felixcloutier.com/x86/PSHUFD.html)
    fn sse_pshufdrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PSHUFDRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PSHUFHWRRI` (`PSHUFHW`). Copies words from the high quadword of a 128-bit lane of the source operand and inserts them in the high quadword of the destination operand at word locations (of the respective lane) selected with the immediate operand. This 256-bit operation is similar to the in-lane operation used by the 256-bit VPSHUFD instruction, which is illustrated in Figure 4-16. For 128-bit operation, only the low 128-bit lane is operative. Each 2-bit field in the immediate operand selects the contents of one word location in the high quadword of the destination operand. The binary encodings of the immediate operand fields select words (0, 1, 2 or 3, 4) from the high quadword of the source operand to be copied to the destination operand. The low quadword of the source operand is copied to the low quadword of the destination operand, for each 128-bit lane.
    /// Reference: [Intel x86 docs for PSHUFHW](https://www.felixcloutier.com/x86/PSHUFHW.html)
    fn sse_pshufhwrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PSHUFHWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PSHUFHWRMI` (`PSHUFHW`). Copies words from the high quadword of a 128-bit lane of the source operand and inserts them in the high quadword of the destination operand at word locations (of the respective lane) selected with the immediate operand. This 256-bit operation is similar to the in-lane operation used by the 256-bit VPSHUFD instruction, which is illustrated in Figure 4-16. For 128-bit operation, only the low 128-bit lane is operative. Each 2-bit field in the immediate operand selects the contents of one word location in the high quadword of the destination operand. The binary encodings of the immediate operand fields select words (0, 1, 2 or 3, 4) from the high quadword of the source operand to be copied to the destination operand. The low quadword of the source operand is copied to the low quadword of the destination operand, for each 128-bit lane.
    /// Reference: [Intel x86 docs for PSHUFHW](https://www.felixcloutier.com/x86/PSHUFHW.html)
    fn sse_pshufhwrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PSHUFHWRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PSHUFLWRRI` (`PSHUFLW`). Copies words from the low quadword of a 128-bit lane of the source operand and inserts them in the low quadword of the destination operand at word locations (of the respective lane) selected with the immediate operand. The 256-bit operation is similar to the in-lane operation used by the 256-bit VPSHUFD instruction, which is illustrated in Figure 4-16. For 128-bit operation, only the low 128-bit lane is operative. Each 2-bit field in the immediate operand selects the contents of one word location in the low quadword of the destination operand. The binary encodings of the immediate operand fields select words (0, 1, 2 or 3) from the low quadword of the source operand to be copied to the destination operand. The high quadword of the source operand is copied to the high quadword of the destination operand, for each 128-bit lane.
    /// Reference: [Intel x86 docs for PSHUFLW](https://www.felixcloutier.com/x86/PSHUFLW.html)
    fn sse_pshuflwrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PSHUFLWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PSHUFLWRMI` (`PSHUFLW`). Copies words from the low quadword of a 128-bit lane of the source operand and inserts them in the low quadword of the destination operand at word locations (of the respective lane) selected with the immediate operand. The 256-bit operation is similar to the in-lane operation used by the 256-bit VPSHUFD instruction, which is illustrated in Figure 4-16. For 128-bit operation, only the low 128-bit lane is operative. Each 2-bit field in the immediate operand selects the contents of one word location in the low quadword of the destination operand. The binary encodings of the immediate operand fields select words (0, 1, 2 or 3) from the low quadword of the source operand to be copied to the destination operand. The high quadword of the source operand is copied to the high quadword of the destination operand, for each 128-bit lane.
    /// Reference: [Intel x86 docs for PSHUFLW](https://www.felixcloutier.com/x86/PSHUFLW.html)
    fn sse_pshuflwrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PSHUFLWRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRLWRI` (`PSRLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLW](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn sse_psrlwri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRLWRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRAWRI` (`PSRAW`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAW](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn sse_psrawri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRAWRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSLLWRI` (`PSLLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLW](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn sse_psllwri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSLLWRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRLDRI` (`PSRLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLD](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn sse_psrldri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRLDRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRADRI` (`PSRAD`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAD](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn sse_psradri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRADRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSLLDRI` (`PSLLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLD](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn sse_pslldri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSLLDRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRLQRI` (`PSRLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLQ](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn sse_psrlqri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRLQRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRLDQRI` (`PSRLDQ`). Shifts the destination operand (first operand) to the right by the number of bytes specified in the count operand (second operand). The empty high-order bytes are cleared (set to all 0s). If the value specified by the count operand is greater than 15, the destination operand is set to all 0s. The count operand is an 8-bit immediate.
    /// Reference: [Intel x86 docs for PSRLDQ](https://www.felixcloutier.com/x86/PSRLDQ.html)
    fn sse_psrldqri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRLDQRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSLLQRI` (`PSLLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLQ](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn sse_psllqri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSLLQRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSLLDQRI` (`PSLLDQ`). Shifts the destination operand (first operand) to the left by the number of bytes specified in the count operand (second operand). The empty low-order bytes are cleared (set to all 0s). If the value specified by the count operand is greater than 15, the destination operand is set to all 0s. The count operand is an 8-bit immediate.
    /// Reference: [Intel x86 docs for PSLLDQ](https://www.felixcloutier.com/x86/PSLLDQ.html)
    fn sse_pslldqri(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSLLDQRI, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPEQBRR` (`PCMPEQB`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQB](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn sse_pcmpeqbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPEQBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPEQBRM` (`PCMPEQB`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQB](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn sse_pcmpeqbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPEQBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPEQWRR` (`PCMPEQW`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQW](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn sse_pcmpeqwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPEQWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPEQWRM` (`PCMPEQW`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQW](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn sse_pcmpeqwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPEQWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPEQDRR` (`PCMPEQD`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQD](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn sse_pcmpeqdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPEQDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PCMPEQDRM` (`PCMPEQD`). Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    /// Reference: [Intel x86 docs for PCMPEQD](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html)
    fn sse_pcmpeqdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PCMPEQDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVD_X2GRR` (`MOVD`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVD](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn sse_movd_x2grr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVD_X2GRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVD_X2GMR` (`MOVD`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVD](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn sse_movd_x2gmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVD_X2GMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVQ_X2GRR` (`MOVQ`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVQ](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn sse_movq_x2grr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVQ_X2GRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVQ_X2GMR` (`MOVQ`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVQ](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn sse_movq_x2gmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVQ_X2GMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVQRR` (`MOVQ`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVQ](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn sse_movqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVQRM` (`MOVQ`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVQ](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn sse_movqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVDQAMR` (`MOVDQA`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVDQA](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html)
    fn sse_movdqamr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVDQAMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVDQUMR` (`MOVDQU`). Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    /// Reference: [Intel x86 docs for MOVDQU](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html)
    fn sse_movdqumr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVDQUMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `LFENCE` (`LFENCE`). Performs a serializing operation on all load-from-memory instructions that were issued prior the LFENCE instruction. Specifically, LFENCE does not execute until all prior instructions have completed locally, and no later instruction begins execution until LFENCE completes. In particular, an instruction that loads from memory and that precedes an LFENCE receives data from memory prior to completion of the LFENCE. (An LFENCE that follows an instruction that stores to memory might complete before the data being stored have become globally visible.) Instructions following an LFENCE may be fetched from memory before the LFENCE, but they will not execute (even speculatively) until the LFENCE completes.
    /// Reference: [Intel x86 docs for LFENCE](https://www.felixcloutier.com/x86/LFENCE.html)
    fn lfence(&mut self,) -> () {
        self.emit(LFENCE, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `MFENCE`.
    fn mfence(&mut self,) -> () {
        self.emit(MFENCE, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CMPPDRRI` (`CMPPD`). Performs a SIMD compare of the packed double precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each pair of packed values in the two source operands.
    /// Reference: [Intel x86 docs for CMPPD](https://www.felixcloutier.com/x86/CMPPD.html)
    fn sse_cmppdrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_CMPPDRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_CMPPDRMI` (`CMPPD`). Performs a SIMD compare of the packed double precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each pair of packed values in the two source operands.
    /// Reference: [Intel x86 docs for CMPPD](https://www.felixcloutier.com/x86/CMPPD.html)
    fn sse_cmppdrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_CMPPDRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_CMPSDRRI` (`CMPSD`). Compares the byte, word, doubleword, or quadword specified with the first source operand with the byte, word, doubleword, or quadword specified with the second source operand and sets the status flags in the EFLAGS register according to the results.
    /// Reference: [Intel x86 docs for CMPSD](https://www.felixcloutier.com/x86/CMPS%3ACMPSB%3ACMPSW%3ACMPSD%3ACMPSQ.html)
    fn sse_cmpsdrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_CMPSDRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_CMPSDRMI` (`CMPSD`). Compares the byte, word, doubleword, or quadword specified with the first source operand with the byte, word, doubleword, or quadword specified with the second source operand and sets the status flags in the EFLAGS register according to the results.
    /// Reference: [Intel x86 docs for CMPSD](https://www.felixcloutier.com/x86/CMPS%3ACMPSB%3ACMPSW%3ACMPSD%3ACMPSQ.html)
    fn sse_cmpsdrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_CMPSDRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PINSRWRRI` (`PINSRW`). Three operand MMX and SSE instructions
    /// Reference: [Intel x86 docs for PINSRW](https://www.felixcloutier.com/x86/PINSRW.html)
    fn sse_pinsrwrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PINSRWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PINSRWRMI` (`PINSRW`). Three operand MMX and SSE instructions
    /// Reference: [Intel x86 docs for PINSRW](https://www.felixcloutier.com/x86/PINSRW.html)
    fn sse_pinsrwrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PINSRWRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PEXTRWRRI` (`PEXTRW`). Copies the word in the source operand (second operand) specified by the count operand (third operand) to the destination operand (first operand). The source operand can be an MMX technology register or an XMM register. The destination operand can be the low word of a general-purpose register or a 16-bit memory address. The count operand is an 8-bit immediate. When specifying a word location in an MMX technology register, the 2 least-significant bits of the count operand specify the location; for an XMM register, the 3 least-significant bits specify the location. The content of the destination register above bit 16 is cleared (set to all 0s).
    /// Reference: [Intel x86 docs for PEXTRW](https://www.felixcloutier.com/x86/PEXTRW.html)
    fn sse_pextrwrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_PEXTRWRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_SHUFPDRRI` (`SHUFPD`). Selects a double precision floating-point value of an input pair using a bit control and move to a designated element of the destination operand. The low-to-high order of double precision element of the destination operand is interleaved between the first source operand and the second source operand at the granularity of input pair of 128 bits. Each bit in the imm8 byte, starting from bit 0, is the select control of the corresponding element of the destination to received the shuffled result of an input pair.
    /// Reference: [Intel x86 docs for SHUFPD](https://www.felixcloutier.com/x86/SHUFPD.html)
    fn sse_shufpdrri(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_SHUFPDRRI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_SHUFPDRMI` (`SHUFPD`). Selects a double precision floating-point value of an input pair using a bit control and move to a designated element of the destination operand. The low-to-high order of double precision element of the destination operand is interleaved between the first source operand and the second source operand at the granularity of input pair of 128 bits. Each bit in the imm8 byte, starting from bit 0, is the select control of the corresponding element of the destination to received the shuffled result of an input pair.
    /// Reference: [Intel x86 docs for SHUFPD](https://www.felixcloutier.com/x86/SHUFPD.html)
    fn sse_shufpdrmi(&mut self,op0: impl OperandCast,op1: impl OperandCast,op2: impl OperandCast) -> () {
        self.emit(SSE_SHUFPDRMI, op0.as_operand(),op1.as_operand(),op2.as_operand(),&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRLWRR` (`PSRLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLW](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn sse_psrlwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRLWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRLWRM` (`PSRLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLW](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn sse_psrlwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRLWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRLDRR` (`PSRLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLD](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn sse_psrldrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRLDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRLDRM` (`PSRLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLD](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn sse_psrldrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRLDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRLQRR` (`PSRLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLQ](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn sse_psrlqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRLQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRLQRM` (`PSRLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSRLQ](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html)
    fn sse_psrlqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRLQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDQRR` (`PADDQ`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDQ](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn sse_paddqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDQRM` (`PADDQ`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDQ](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn sse_paddqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULLWRR` (`PMULLW`). Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the low 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULLW](https://www.felixcloutier.com/x86/PMULLW.html)
    fn sse_pmullwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULLWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULLWRM` (`PMULLW`). Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the low 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULLW](https://www.felixcloutier.com/x86/PMULLW.html)
    fn sse_pmullwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULLWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVQMR` (`MOVQ`). Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    /// Reference: [Intel x86 docs for MOVQ](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html)
    fn sse_movqmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVQMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMOVMSKBRR` (`PMOVMSKB`). Creates a mask made up of the most significant bit of each byte of the source operand (second operand) and stores the result in the low byte or word of the destination operand (first operand).
    /// Reference: [Intel x86 docs for PMOVMSKB](https://www.felixcloutier.com/x86/PMOVMSKB.html)
    fn sse_pmovmskbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMOVMSKBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBUSBRR` (`PSUBUSB`). Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBUSB](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html)
    fn sse_psubusbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBUSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBUSBRM` (`PSUBUSB`). Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBUSB](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html)
    fn sse_psubusbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBUSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBUSWRR` (`PSUBUSW`). Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBUSW](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html)
    fn sse_psubuswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBUSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBUSWRM` (`PSUBUSW`). Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBUSW](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html)
    fn sse_psubuswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBUSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINUBRR` (`PMINUB`). Performs a SIMD compare of the packed unsigned byte or word integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINUB](https://www.felixcloutier.com/x86/PMINUB%3APMINUW.html)
    fn sse_pminubrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINUBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINUBRM` (`PMINUB`). Performs a SIMD compare of the packed unsigned byte or word integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINUB](https://www.felixcloutier.com/x86/PMINUB%3APMINUW.html)
    fn sse_pminubrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINUBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PANDRR` (`PAND`). Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for PAND](https://www.felixcloutier.com/x86/PAND.html)
    fn sse_pandrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PANDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PANDRM` (`PAND`). Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for PAND](https://www.felixcloutier.com/x86/PAND.html)
    fn sse_pandrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PANDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDUSBRR` (`PADDUSB`). Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDUSB](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html)
    fn sse_paddusbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDUSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDUSBRM` (`PADDUSB`). Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDUSB](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html)
    fn sse_paddusbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDUSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDUSWRR` (`PADDUSW`). Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDUSW](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html)
    fn sse_padduswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDUSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDUSWRM` (`PADDUSW`). Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDUSW](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html)
    fn sse_padduswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDUSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXUBRR` (`PMAXUB`). Performs a SIMD compare of the packed unsigned byte, word integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXUB](https://www.felixcloutier.com/x86/PMAXUB%3APMAXUW.html)
    fn sse_pmaxubrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXUBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXUBRM` (`PMAXUB`). Performs a SIMD compare of the packed unsigned byte, word integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXUB](https://www.felixcloutier.com/x86/PMAXUB%3APMAXUW.html)
    fn sse_pmaxubrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXUBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PANDNRR` (`PANDN`). Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for PANDN](https://www.felixcloutier.com/x86/PANDN.html)
    fn sse_pandnrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PANDNRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PANDNRM` (`PANDN`). Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
    /// Reference: [Intel x86 docs for PANDN](https://www.felixcloutier.com/x86/PANDN.html)
    fn sse_pandnrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PANDNRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PAVGBRR` (`PAVGB`). Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
    /// Reference: [Intel x86 docs for PAVGB](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html)
    fn sse_pavgbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PAVGBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PAVGBRM` (`PAVGB`). Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
    /// Reference: [Intel x86 docs for PAVGB](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html)
    fn sse_pavgbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PAVGBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRAWRR` (`PSRAW`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAW](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn sse_psrawrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRAWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRAWRM` (`PSRAW`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAW](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn sse_psrawrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRAWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRADRR` (`PSRAD`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAD](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn sse_psradrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRADRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSRADRM` (`PSRAD`). Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    /// Reference: [Intel x86 docs for PSRAD](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html)
    fn sse_psradrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSRADRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PAVGWRR` (`PAVGW`). Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
    /// Reference: [Intel x86 docs for PAVGW](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html)
    fn sse_pavgwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PAVGWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PAVGWRM` (`PAVGW`). Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
    /// Reference: [Intel x86 docs for PAVGW](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html)
    fn sse_pavgwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PAVGWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULHUWRR` (`PMULHUW`). Performs a SIMD unsigned multiply of the packed unsigned word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each 32-bit intermediate results in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULHUW](https://www.felixcloutier.com/x86/PMULHUW.html)
    fn sse_pmulhuwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULHUWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULHUWRM` (`PMULHUW`). Performs a SIMD unsigned multiply of the packed unsigned word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each 32-bit intermediate results in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULHUW](https://www.felixcloutier.com/x86/PMULHUW.html)
    fn sse_pmulhuwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULHUWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULHWRR` (`PMULHW`). Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULHW](https://www.felixcloutier.com/x86/PMULHW.html)
    fn sse_pmulhwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULHWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULHWRM` (`PMULHW`). Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    /// Reference: [Intel x86 docs for PMULHW](https://www.felixcloutier.com/x86/PMULHW.html)
    fn sse_pmulhwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULHWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTPD2DQRR` (`CVTTPD2DQ`). Converts two, four or eight packed double precision floating-point values in the source operand (second operand) to two, four or eight packed signed doubleword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTTPD2DQ](https://www.felixcloutier.com/x86/CVTTPD2DQ.html)
    fn sse_cvttpd2dqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTPD2DQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTTPD2DQRM` (`CVTTPD2DQ`). Converts two, four or eight packed double precision floating-point values in the source operand (second operand) to two, four or eight packed signed doubleword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTTPD2DQ](https://www.felixcloutier.com/x86/CVTTPD2DQ.html)
    fn sse_cvttpd2dqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTTPD2DQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTDQ2PDRR` (`CVTDQ2PD`). Converts two, four or eight packed signed doubleword integers in the source operand (the second operand) to two, four or eight packed double precision floating-point values in the destination operand (the first operand).
    /// Reference: [Intel x86 docs for CVTDQ2PD](https://www.felixcloutier.com/x86/CVTDQ2PD.html)
    fn sse_cvtdq2pdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTDQ2PDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTDQ2PDRM` (`CVTDQ2PD`). Converts two, four or eight packed signed doubleword integers in the source operand (the second operand) to two, four or eight packed double precision floating-point values in the destination operand (the first operand).
    /// Reference: [Intel x86 docs for CVTDQ2PD](https://www.felixcloutier.com/x86/CVTDQ2PD.html)
    fn sse_cvtdq2pdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTDQ2PDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTPD2DQRR` (`CVTPD2DQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed signed doubleword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPD2DQ](https://www.felixcloutier.com/x86/CVTPD2DQ.html)
    fn sse_cvtpd2dqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTPD2DQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_CVTPD2DQRM` (`CVTPD2DQ`). Converts packed double precision floating-point values in the source operand (second operand) to packed signed doubleword integers in the destination operand (first operand).
    /// Reference: [Intel x86 docs for CVTPD2DQ](https://www.felixcloutier.com/x86/CVTPD2DQ.html)
    fn sse_cvtpd2dqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_CVTPD2DQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_MOVNTDQMR` (`MOVNTDQ`). Moves the packed integers in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to prevent caching of the data during the write to memory. The source operand is an XMM register, YMM register or ZMM register, which is assumed to contain integer data (packed bytes, words, double-words, or quadwords). The destination operand is a 128-bit, 256-bit or 512-bit memory location. The memory operand must be aligned on a 16-byte (128-bit version), 32-byte (VEX.256 encoded version) or 64-byte (512-bit version) boundary otherwise a general-protection exception (#GP) will be generated.
    /// Reference: [Intel x86 docs for MOVNTDQ](https://www.felixcloutier.com/x86/MOVNTDQ.html)
    fn sse_movntdqmr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_MOVNTDQMR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBSBRR` (`PSUBSB`). Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBSB](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html)
    fn sse_psubsbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBSBRM` (`PSUBSB`). Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBSB](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html)
    fn sse_psubsbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBSWRR` (`PSUBSW`). Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBSW](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html)
    fn sse_psubswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBSWRM` (`PSUBSW`). Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBSW](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html)
    fn sse_psubswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PORRR` (`POR`). Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
    /// Reference: [Intel x86 docs for POR](https://www.felixcloutier.com/x86/POR.html)
    fn sse_porrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PORRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PORRM` (`POR`). Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
    /// Reference: [Intel x86 docs for POR](https://www.felixcloutier.com/x86/POR.html)
    fn sse_porrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PORRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDSBRR` (`PADDSB`). Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDSB](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html)
    fn sse_paddsbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDSBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDSBRM` (`PADDSB`). Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDSB](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html)
    fn sse_paddsbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDSBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINSWRR` (`PMINSW`). Performs a SIMD compare of the packed signed byte, word, or dword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINSW](https://www.felixcloutier.com/x86/PMINSB%3APMINSW.html)
    fn sse_pminswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMINSWRM` (`PMINSW`). Performs a SIMD compare of the packed signed byte, word, or dword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMINSW](https://www.felixcloutier.com/x86/PMINSB%3APMINSW.html)
    fn sse_pminswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMINSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXSWRR` (`PMAXSW`). Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXSW](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html)
    fn sse_pmaxswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMAXSWRM` (`PMAXSW`). Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    /// Reference: [Intel x86 docs for PMAXSW](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html)
    fn sse_pmaxswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMAXSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDSWRR` (`PADDSW`). Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDSW](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html)
    fn sse_paddswrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDSWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDSWRM` (`PADDSW`). Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDSW](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html)
    fn sse_paddswrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDSWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PXORRR` (`PXOR`). Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
    /// Reference: [Intel x86 docs for PXOR](https://www.felixcloutier.com/x86/PXOR.html)
    fn sse_pxorrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PXORRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PXORRM` (`PXOR`). Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
    /// Reference: [Intel x86 docs for PXOR](https://www.felixcloutier.com/x86/PXOR.html)
    fn sse_pxorrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PXORRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSLLWRR` (`PSLLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLW](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn sse_psllwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSLLWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSLLWRM` (`PSLLW`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLW](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn sse_psllwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSLLWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSLLDRR` (`PSLLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLD](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn sse_pslldrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSLLDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSLLDRM` (`PSLLD`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLD](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn sse_pslldrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSLLDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSLLQRR` (`PSLLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLQ](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn sse_psllqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSLLQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSLLQRM` (`PSLLQ`). Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    /// Reference: [Intel x86 docs for PSLLQ](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html)
    fn sse_psllqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSLLQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULUDQRR` (`PMULUDQ`). Multiplies the first operand (destination operand) by the second operand (source operand) and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for PMULUDQ](https://www.felixcloutier.com/x86/PMULUDQ.html)
    fn sse_pmuludqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULUDQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMULUDQRM` (`PMULUDQ`). Multiplies the first operand (destination operand) by the second operand (source operand) and stores the result in the destination operand.
    /// Reference: [Intel x86 docs for PMULUDQ](https://www.felixcloutier.com/x86/PMULUDQ.html)
    fn sse_pmuludqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMULUDQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMADDWDRR` (`PMADDWD`). Multiplies the individual signed words of the destination operand (first operand) by the corresponding signed words of the source operand (second operand), producing temporary signed, doubleword results. The adjacent double-word results are then summed and stored in the destination operand. For example, the corresponding low-order words (15-0) and (31-16) in the source and destination operands are multiplied by one another and the double-word results are added together and stored in the low doubleword of the destination register (31-0). The same operation is performed on the other pairs of adjacent words. (Figure 4-11 shows this operation when using 64-bit operands).
    /// Reference: [Intel x86 docs for PMADDWD](https://www.felixcloutier.com/x86/PMADDWD.html)
    fn sse_pmaddwdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMADDWDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PMADDWDRM` (`PMADDWD`). Multiplies the individual signed words of the destination operand (first operand) by the corresponding signed words of the source operand (second operand), producing temporary signed, doubleword results. The adjacent double-word results are then summed and stored in the destination operand. For example, the corresponding low-order words (15-0) and (31-16) in the source and destination operands are multiplied by one another and the double-word results are added together and stored in the low doubleword of the destination register (31-0). The same operation is performed on the other pairs of adjacent words. (Figure 4-11 shows this operation when using 64-bit operands).
    /// Reference: [Intel x86 docs for PMADDWD](https://www.felixcloutier.com/x86/PMADDWD.html)
    fn sse_pmaddwdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PMADDWDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSADBWRR` (`PSADBW`). Computes the absolute value of the difference of 8 unsigned byte integers from the source operand (second operand) and from the destination operand (first operand). These 8 differences are then summed to produce an unsigned word integer result that is stored in the destination operand. Figure 4-14 shows the operation of the PSADBW instruction when using 64-bit operands.
    /// Reference: [Intel x86 docs for PSADBW](https://www.felixcloutier.com/x86/PSADBW.html)
    fn sse_psadbwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSADBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSADBWRM` (`PSADBW`). Computes the absolute value of the difference of 8 unsigned byte integers from the source operand (second operand) and from the destination operand (first operand). These 8 differences are then summed to produce an unsigned word integer result that is stored in the destination operand. Figure 4-14 shows the operation of the PSADBW instruction when using 64-bit operands.
    /// Reference: [Intel x86 docs for PSADBW](https://www.felixcloutier.com/x86/PSADBW.html)
    fn sse_psadbwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSADBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBBRR` (`PSUBB`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBB](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn sse_psubbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBBRM` (`PSUBB`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBB](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn sse_psubbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBWRR` (`PSUBW`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBW](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn sse_psubwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBWRM` (`PSUBW`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBW](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn sse_psubwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBDRR` (`PSUBD`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBD](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn sse_psubdrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBDRM` (`PSUBD`). Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PSUBD](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html)
    fn sse_psubdrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBQRR` (`PSUBQ`). Subtracts the second operand (source operand) from the first operand (destination operand) and stores the result in the destination operand. When packed quadword operands are used, a SIMD subtract is performed. When a quadword result is too large to be represented in 64 bits (overflow), the result is wrapped around and the low 64 bits are written to the destination element (that is, the carry is ignored).
    /// Reference: [Intel x86 docs for PSUBQ](https://www.felixcloutier.com/x86/PSUBQ.html)
    fn sse_psubqrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBQRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PSUBQRM` (`PSUBQ`). Subtracts the second operand (source operand) from the first operand (destination operand) and stores the result in the destination operand. When packed quadword operands are used, a SIMD subtract is performed. When a quadword result is too large to be represented in 64 bits (overflow), the result is wrapped around and the low 64 bits are written to the destination element (that is, the carry is ignored).
    /// Reference: [Intel x86 docs for PSUBQ](https://www.felixcloutier.com/x86/PSUBQ.html)
    fn sse_psubqrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PSUBQRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDBRR` (`PADDB`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDB](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn sse_paddbrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDBRM` (`PADDB`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDB](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn sse_paddbrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDBRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDWRR` (`PADDW`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDW](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn sse_paddwrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDWRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDWRM` (`PADDW`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDW](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn sse_paddwrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDWRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDDRR` (`PADDD`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDD](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn sse_padddrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SSE_PADDDRM` (`PADDD`). Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    /// Reference: [Intel x86 docs for PADDD](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html)
    fn sse_padddrm(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(SSE_PADDDRM, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
