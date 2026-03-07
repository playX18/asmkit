pub trait X86387Emitter: Emitter {
    /// Emits `FADDM32` (`FADD`). Adds the destination and source operands and stores the sum in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FADD](https://www.felixcloutier.com/x86/FADD%3AFADDP%3AFIADD.html)
    fn faddm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FADDM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FMULM32` (`FMUL`). Multiplies the destination and source operands and stores the product in the destination location. The destination operand is always an FPU data register; the source operand can be an FPU data register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FMUL](https://www.felixcloutier.com/x86/FMUL%3AFMULP%3AFIMUL.html)
    fn fmulm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FMULM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCOMM32` (`FCOM`). Compares the contents of register ST(0) and source value and sets condition code flags C0, C2, and C3 in the FPU status word according to the results (see the table below). The source operand can be a data register or a memory location. If no source operand is given, the value in ST(0) is compared with the value in ST(1). The sign of zero is ignored, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FCOM](https://www.felixcloutier.com/x86/FCOM%3AFCOMP%3AFCOMPP.html)
    fn fcomm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCOMM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCOMPM32` (`FCOMP`). Compares the contents of register ST(0) and source value and sets condition code flags C0, C2, and C3 in the FPU status word according to the results (see the table below). The source operand can be a data register or a memory location. If no source operand is given, the value in ST(0) is compared with the value in ST(1). The sign of zero is ignored, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FCOMP](https://www.felixcloutier.com/x86/FCOM%3AFCOMP%3AFCOMPP.html)
    fn fcompm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCOMPM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSUBM32` (`FSUB`). Subtracts the source operand from the destination operand and stores the difference in the destination location. The destination operand is always an FPU data register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FSUB](https://www.felixcloutier.com/x86/FSUB%3AFSUBP%3AFISUB.html)
    fn fsubm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSUBM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSUBRM32` (`FSUBR`). Subtracts the destination operand from the source operand and stores the difference in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FSUBR](https://www.felixcloutier.com/x86/FSUBR%3AFSUBRP%3AFISUBR.html)
    fn fsubrm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSUBRM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FDIVM32` (`FDIV`). Divides the destination operand by the source operand and stores the result in the destination location. The destination operand (dividend) is always in an FPU register; the source operand (divisor) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FDIV](https://www.felixcloutier.com/x86/FDIV%3AFDIVP%3AFIDIV.html)
    fn fdivm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FDIVM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FDIVRM32` (`FDIVR`). Divides the source operand by the destination operand and stores the result in the destination location. The destination operand (divisor) is always in an FPU register; the source operand (dividend) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FDIVR](https://www.felixcloutier.com/x86/FDIVR%3AFDIVRP%3AFIDIVR.html)
    fn fdivrm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FDIVRM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FADDRR` (`FADD`). Adds the destination and source operands and stores the sum in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FADD](https://www.felixcloutier.com/x86/FADD%3AFADDP%3AFIADD.html)
    fn faddrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FADDRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FMULRR` (`FMUL`). Multiplies the destination and source operands and stores the product in the destination location. The destination operand is always an FPU data register; the source operand can be an FPU data register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FMUL](https://www.felixcloutier.com/x86/FMUL%3AFMULP%3AFIMUL.html)
    fn fmulrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FMULRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCOMRR` (`FCOM`). Compares the contents of register ST(0) and source value and sets condition code flags C0, C2, and C3 in the FPU status word according to the results (see the table below). The source operand can be a data register or a memory location. If no source operand is given, the value in ST(0) is compared with the value in ST(1). The sign of zero is ignored, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FCOM](https://www.felixcloutier.com/x86/FCOM%3AFCOMP%3AFCOMPP.html)
    fn fcomrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FCOMRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCOMPRR` (`FCOMP`). Compares the contents of register ST(0) and source value and sets condition code flags C0, C2, and C3 in the FPU status word according to the results (see the table below). The source operand can be a data register or a memory location. If no source operand is given, the value in ST(0) is compared with the value in ST(1). The sign of zero is ignored, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FCOMP](https://www.felixcloutier.com/x86/FCOM%3AFCOMP%3AFCOMPP.html)
    fn fcomprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FCOMPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSUBRR` (`FSUB`). Subtracts the source operand from the destination operand and stores the difference in the destination location. The destination operand is always an FPU data register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FSUB](https://www.felixcloutier.com/x86/FSUB%3AFSUBP%3AFISUB.html)
    fn fsubrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FSUBRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSUBRRR` (`FSUBR`). Subtracts the destination operand from the source operand and stores the difference in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FSUBR](https://www.felixcloutier.com/x86/FSUBR%3AFSUBRP%3AFISUBR.html)
    fn fsubrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FSUBRRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FDIVRR` (`FDIV`). Divides the destination operand by the source operand and stores the result in the destination location. The destination operand (dividend) is always in an FPU register; the source operand (divisor) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FDIV](https://www.felixcloutier.com/x86/FDIV%3AFDIVP%3AFIDIV.html)
    fn fdivrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FDIVRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FDIVRRR` (`FDIVR`). Divides the source operand by the destination operand and stores the result in the destination location. The destination operand (divisor) is always in an FPU register; the source operand (dividend) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FDIVR](https://www.felixcloutier.com/x86/FDIVR%3AFDIVRP%3AFIDIVR.html)
    fn fdivrrr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FDIVRRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDM32` (`FLD`). Pushes the source operand onto the FPU register stack. The source operand can be in single precision, double precision, or double extended-precision floating-point format. If the source operand is in single precision or double precision floating-point format, it is automatically converted to the double extended-precision floating-point format before being pushed on the stack.
    /// Reference: [Intel x86 docs for FLD](https://www.felixcloutier.com/x86/FLD.html)
    fn fldm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSTM32` (`FST`). The FST instruction copies the value in the ST(0) register to the destination operand, which can be a memory location or another register in the FPU register stack. When storing the value in memory, the value is converted to single precision or double precision floating-point format.
    /// Reference: [Intel x86 docs for FST](https://www.felixcloutier.com/x86/FST%3AFSTP.html)
    fn fstm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSTPM32` (`FSTP`). The FST instruction copies the value in the ST(0) register to the destination operand, which can be a memory location or another register in the FPU register stack. When storing the value in memory, the value is converted to single precision or double precision floating-point format.
    /// Reference: [Intel x86 docs for FSTP](https://www.felixcloutier.com/x86/FST%3AFSTP.html)
    fn fstpm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTPM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDENVM`.
    fn fldenvm(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDENVM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDCWM`.
    fn fldcwm(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDCWM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSTENVM` (`FSTENV`). Saves the current FPU operating environment at the memory location specified with the destination operand, and then masks all floating-point exceptions. The FPU operating environment consists of the FPU control word, status word, tag word, instruction pointer, data pointer, and last opcode. Figures 8-9 through 8-12 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, show the layout in memory of the stored environment, depending on the operating mode of the processor (protected or real) and the current operand-size attribute (16-bit or 32-bit). In virtual-8086 mode, the real mode layouts are used.
    /// Reference: [Intel x86 docs for FSTENV](https://www.felixcloutier.com/x86/FSTENV%3AFNSTENV.html)
    fn fstenvm(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTENVM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSTCWM` (`FSTCW`). Stores the current value of the FPU control word at the specified destination in memory. The FSTCW instruction checks for and handles pending unmasked floating-point exceptions before storing the control word; the FNSTCW instruction does not.
    /// Reference: [Intel x86 docs for FSTCW](https://www.felixcloutier.com/x86/FSTCW%3AFNSTCW.html)
    fn fstcwm(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTCWM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDR` (`FLD`). Pushes the source operand onto the FPU register stack. The source operand can be in single precision, double precision, or double extended-precision floating-point format. If the source operand is in single precision or double precision floating-point format, it is automatically converted to the double extended-precision floating-point format before being pushed on the stack.
    /// Reference: [Intel x86 docs for FLD](https://www.felixcloutier.com/x86/FLD.html)
    fn fldr(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FXCHR` (`FXCH`). Exchanges the contents of registers ST(0) and ST(i). If no source operand is specified, the contents of ST(0) and ST(1) are exchanged.
    /// Reference: [Intel x86 docs for FXCH](https://www.felixcloutier.com/x86/FXCH.html)
    fn fxchr(&mut self,op0: impl OperandCast) -> () {
        self.emit(FXCHR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FNOP`.
    fn fnop(&mut self,) -> () {
        self.emit(FNOP, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCHS`.
    fn fchs(&mut self,) -> () {
        self.emit(FCHS, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FABS`.
    fn fabs(&mut self,) -> () {
        self.emit(FABS, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FTST`.
    fn ftst(&mut self,) -> () {
        self.emit(FTST, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FXAM`.
    fn fxam(&mut self,) -> () {
        self.emit(FXAM, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLD1` (`FLD1`). Push one of seven commonly used constants (in double extended-precision floating-point format) onto the FPU register stack. The constants that can be loaded with these instructions include +1.0, +0.0, log210, log2e, π, log102, and loge2. For each constant, an internal 66-bit constant is rounded (as specified by the RC field in the FPU control word) to double extended-precision floating-point format. The inexact-result exception (#P) is not generated as a result of the rounding, nor is the C1 flag set in the x87 FPU status word if the value is rounded up.
    /// Reference: [Intel x86 docs for FLD1](https://www.felixcloutier.com/x86/FLD1%3AFLDL2T%3AFLDL2E%3AFLDPI%3AFLDLG2%3AFLDLN2%3AFLDZ.html)
    fn fld1(&mut self,) -> () {
        self.emit(FLD1, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDL2T` (`FLDL2T`). Push one of seven commonly used constants (in double extended-precision floating-point format) onto the FPU register stack. The constants that can be loaded with these instructions include +1.0, +0.0, log210, log2e, π, log102, and loge2. For each constant, an internal 66-bit constant is rounded (as specified by the RC field in the FPU control word) to double extended-precision floating-point format. The inexact-result exception (#P) is not generated as a result of the rounding, nor is the C1 flag set in the x87 FPU status word if the value is rounded up.
    /// Reference: [Intel x86 docs for FLDL2T](https://www.felixcloutier.com/x86/FLD1%3AFLDL2T%3AFLDL2E%3AFLDPI%3AFLDLG2%3AFLDLN2%3AFLDZ.html)
    fn fldl2t(&mut self,) -> () {
        self.emit(FLDL2T, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDL2E` (`FLDL2E`). Push one of seven commonly used constants (in double extended-precision floating-point format) onto the FPU register stack. The constants that can be loaded with these instructions include +1.0, +0.0, log210, log2e, π, log102, and loge2. For each constant, an internal 66-bit constant is rounded (as specified by the RC field in the FPU control word) to double extended-precision floating-point format. The inexact-result exception (#P) is not generated as a result of the rounding, nor is the C1 flag set in the x87 FPU status word if the value is rounded up.
    /// Reference: [Intel x86 docs for FLDL2E](https://www.felixcloutier.com/x86/FLD1%3AFLDL2T%3AFLDL2E%3AFLDPI%3AFLDLG2%3AFLDLN2%3AFLDZ.html)
    fn fldl2e(&mut self,) -> () {
        self.emit(FLDL2E, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDPI` (`FLDPI`). Push one of seven commonly used constants (in double extended-precision floating-point format) onto the FPU register stack. The constants that can be loaded with these instructions include +1.0, +0.0, log210, log2e, π, log102, and loge2. For each constant, an internal 66-bit constant is rounded (as specified by the RC field in the FPU control word) to double extended-precision floating-point format. The inexact-result exception (#P) is not generated as a result of the rounding, nor is the C1 flag set in the x87 FPU status word if the value is rounded up.
    /// Reference: [Intel x86 docs for FLDPI](https://www.felixcloutier.com/x86/FLD1%3AFLDL2T%3AFLDL2E%3AFLDPI%3AFLDLG2%3AFLDLN2%3AFLDZ.html)
    fn fldpi(&mut self,) -> () {
        self.emit(FLDPI, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDLG2` (`FLDLG2`). Push one of seven commonly used constants (in double extended-precision floating-point format) onto the FPU register stack. The constants that can be loaded with these instructions include +1.0, +0.0, log210, log2e, π, log102, and loge2. For each constant, an internal 66-bit constant is rounded (as specified by the RC field in the FPU control word) to double extended-precision floating-point format. The inexact-result exception (#P) is not generated as a result of the rounding, nor is the C1 flag set in the x87 FPU status word if the value is rounded up.
    /// Reference: [Intel x86 docs for FLDLG2](https://www.felixcloutier.com/x86/FLD1%3AFLDL2T%3AFLDL2E%3AFLDPI%3AFLDLG2%3AFLDLN2%3AFLDZ.html)
    fn fldlg2(&mut self,) -> () {
        self.emit(FLDLG2, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDLN2` (`FLDLN2`). Push one of seven commonly used constants (in double extended-precision floating-point format) onto the FPU register stack. The constants that can be loaded with these instructions include +1.0, +0.0, log210, log2e, π, log102, and loge2. For each constant, an internal 66-bit constant is rounded (as specified by the RC field in the FPU control word) to double extended-precision floating-point format. The inexact-result exception (#P) is not generated as a result of the rounding, nor is the C1 flag set in the x87 FPU status word if the value is rounded up.
    /// Reference: [Intel x86 docs for FLDLN2](https://www.felixcloutier.com/x86/FLD1%3AFLDL2T%3AFLDL2E%3AFLDPI%3AFLDLG2%3AFLDLN2%3AFLDZ.html)
    fn fldln2(&mut self,) -> () {
        self.emit(FLDLN2, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDZ` (`FLDZ`). Push one of seven commonly used constants (in double extended-precision floating-point format) onto the FPU register stack. The constants that can be loaded with these instructions include +1.0, +0.0, log210, log2e, π, log102, and loge2. For each constant, an internal 66-bit constant is rounded (as specified by the RC field in the FPU control word) to double extended-precision floating-point format. The inexact-result exception (#P) is not generated as a result of the rounding, nor is the C1 flag set in the x87 FPU status word if the value is rounded up.
    /// Reference: [Intel x86 docs for FLDZ](https://www.felixcloutier.com/x86/FLD1%3AFLDL2T%3AFLDL2E%3AFLDPI%3AFLDLG2%3AFLDLN2%3AFLDZ.html)
    fn fldz(&mut self,) -> () {
        self.emit(FLDZ, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `F2XM1`.
    fn f2xm1(&mut self,) -> () {
        self.emit(F2XM1, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FYL2X` (`FYL2X`). Computes (ST(1) ∗ log2 (ST(0))), stores the result in register ST(1), and pops the FPU register stack. The source operand in ST(0) must be a non-zero positive number.
    /// Reference: [Intel x86 docs for FYL2X](https://www.felixcloutier.com/x86/FYL2X.html)
    fn fyl2x(&mut self,) -> () {
        self.emit(FYL2X, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FPTAN` (`FPTAN`). Computes the approximate tangent of the source operand in register ST(0), stores the result in ST(0), and pushes a 1.0 onto the FPU register stack. The source operand must be given in radians and must be less than ±263. The following table shows the unmasked results obtained when computing the partial tangent of various classes of numbers, assuming that underflow does not occur.
    /// Reference: [Intel x86 docs for FPTAN](https://www.felixcloutier.com/x86/FPTAN.html)
    fn fptan(&mut self,) -> () {
        self.emit(FPTAN, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FPATAN`.
    fn fpatan(&mut self,) -> () {
        self.emit(FPATAN, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FXTRACT` (`FXTRACT`). Separates the source value in the ST(0) register into its exponent and significand, stores the exponent in ST(0), and pushes the significand onto the register stack. Following this operation, the new top-of-stack register ST(0) contains the value of the original significand expressed as a floating-point value. The sign and significand of this value are the same as those found in the source operand, and the exponent is 3FFFH (biased value for a true exponent of zero). The ST(1) register contains the value of the original operand’s true (unbiased) exponent expressed as a floating-point value. (The operation performed by this instruction is a superset of the IEEE-recommended logb(x) function.)
    /// Reference: [Intel x86 docs for FXTRACT](https://www.felixcloutier.com/x86/FXTRACT.html)
    fn fxtract(&mut self,) -> () {
        self.emit(FXTRACT, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FPREM1` (`FPREM1`). Computes the IEEE remainder obtained from dividing the value in the ST(0) register (the dividend) by the value in the ST(1) register (the divisor or modulus), and stores the result in ST(0). The remainder represents the following value
    /// Reference: [Intel x86 docs for FPREM1](https://www.felixcloutier.com/x86/FPREM1.html)
    fn fprem1(&mut self,) -> () {
        self.emit(FPREM1, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FDECSTP`.
    fn fdecstp(&mut self,) -> () {
        self.emit(FDECSTP, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FINCSTP`.
    fn fincstp(&mut self,) -> () {
        self.emit(FINCSTP, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FPREM` (`FPREM`). Computes the remainder obtained from dividing the value in the ST(0) register (the dividend) by the value in the ST(1) register (the divisor or modulus), and stores the result in ST(0). The remainder represents the following value
    /// Reference: [Intel x86 docs for FPREM](https://www.felixcloutier.com/x86/FPREM.html)
    fn fprem(&mut self,) -> () {
        self.emit(FPREM, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FYL2XP1` (`FYL2XP1`). Computes (ST(1) ∗ log2(ST(0) + 1.0)), stores the result in register ST(1), and pops the FPU register stack. The source operand in ST(0) must be in the range
    /// Reference: [Intel x86 docs for FYL2XP1](https://www.felixcloutier.com/x86/FYL2XP1.html)
    fn fyl2xp1(&mut self,) -> () {
        self.emit(FYL2XP1, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSQRT`.
    fn fsqrt(&mut self,) -> () {
        self.emit(FSQRT, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSINCOS` (`FSINCOS`). Computes both the approximate sine and the cosine of the source operand in register ST(0), stores the sine in ST(0), and pushes the cosine onto the top of the FPU register stack. (This instruction is faster than executing the FSIN and FCOS instructions in succession.)
    /// Reference: [Intel x86 docs for FSINCOS](https://www.felixcloutier.com/x86/FSINCOS.html)
    fn fsincos(&mut self,) -> () {
        self.emit(FSINCOS, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FRNDINT`.
    fn frndint(&mut self,) -> () {
        self.emit(FRNDINT, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSCALE`.
    fn fscale(&mut self,) -> () {
        self.emit(FSCALE, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSIN`.
    fn fsin(&mut self,) -> () {
        self.emit(FSIN, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCOS`.
    fn fcos(&mut self,) -> () {
        self.emit(FCOS, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FIADDM32` (`FIADD`). Adds the destination and source operands and stores the sum in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FIADD](https://www.felixcloutier.com/x86/FADD%3AFADDP%3AFIADD.html)
    fn fiaddm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIADDM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FIMULM32` (`FIMUL`). Multiplies the destination and source operands and stores the product in the destination location. The destination operand is always an FPU data register; the source operand can be an FPU data register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FIMUL](https://www.felixcloutier.com/x86/FMUL%3AFMULP%3AFIMUL.html)
    fn fimulm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIMULM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FICOMM32` (`FICOM`). Compares the value in ST(0) with an integer source operand and sets the condition code flags C0, C2, and C3 in the FPU status word according to the results (see table below). The integer value is converted to double extended-precision floating-point format before the comparison is made.
    /// Reference: [Intel x86 docs for FICOM](https://www.felixcloutier.com/x86/FICOM%3AFICOMP.html)
    fn ficomm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FICOMM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FICOMPM32` (`FICOMP`). Compares the value in ST(0) with an integer source operand and sets the condition code flags C0, C2, and C3 in the FPU status word according to the results (see table below). The integer value is converted to double extended-precision floating-point format before the comparison is made.
    /// Reference: [Intel x86 docs for FICOMP](https://www.felixcloutier.com/x86/FICOM%3AFICOMP.html)
    fn ficompm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FICOMPM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISUBM32` (`FISUB`). Subtracts the source operand from the destination operand and stores the difference in the destination location. The destination operand is always an FPU data register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FISUB](https://www.felixcloutier.com/x86/FSUB%3AFSUBP%3AFISUB.html)
    fn fisubm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISUBM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISUBRM32` (`FISUBR`). Subtracts the destination operand from the source operand and stores the difference in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FISUBR](https://www.felixcloutier.com/x86/FSUBR%3AFSUBRP%3AFISUBR.html)
    fn fisubrm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISUBRM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FIDIVM32` (`FIDIV`). Divides the destination operand by the source operand and stores the result in the destination location. The destination operand (dividend) is always in an FPU register; the source operand (divisor) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FIDIV](https://www.felixcloutier.com/x86/FDIV%3AFDIVP%3AFIDIV.html)
    fn fidivm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIDIVM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FIDIVRM32` (`FIDIVR`). Divides the source operand by the destination operand and stores the result in the destination location. The destination operand (divisor) is always in an FPU register; the source operand (dividend) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FIDIVR](https://www.felixcloutier.com/x86/FDIVR%3AFDIVRP%3AFIDIVR.html)
    fn fidivrm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIDIVRM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FUCOMPP` (`FUCOMPP`). Performs an unordered comparison of the contents of register ST(0) and ST(i) and sets condition code flags C0, C2, and C3 in the FPU status word according to the results (see the table below). If no operand is specified, the contents of registers ST(0) and ST(1) are compared. The sign of zero is ignored, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FUCOMPP](https://www.felixcloutier.com/x86/FUCOM%3AFUCOMP%3AFUCOMPP.html)
    fn fucompp(&mut self,) -> () {
        self.emit(FUCOMPP, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FILDM32` (`FILD`). Converts the signed-integer source operand into double extended-precision floating-point format and pushes the value onto the FPU register stack. The source operand can be a word, doubleword, or quadword integer. It is loaded without rounding errors. The sign of the source operand is preserved.
    /// Reference: [Intel x86 docs for FILD](https://www.felixcloutier.com/x86/FILD.html)
    fn fildm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FILDM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISTM32` (`FIST`). The FIST instruction converts the value in the ST(0) register to a signed integer and stores the result in the destination operand. Values can be stored in word or doubleword integer format. The destination operand specifies the address where the first byte of the destination value is to be stored.
    /// Reference: [Intel x86 docs for FIST](https://www.felixcloutier.com/x86/FIST%3AFISTP.html)
    fn fistm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISTPM32` (`FISTP`). The FIST instruction converts the value in the ST(0) register to a signed integer and stores the result in the destination operand. Values can be stored in word or doubleword integer format. The destination operand specifies the address where the first byte of the destination value is to be stored.
    /// Reference: [Intel x86 docs for FISTP](https://www.felixcloutier.com/x86/FIST%3AFISTP.html)
    fn fistpm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTPM32, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDM80` (`FLD`). Pushes the source operand onto the FPU register stack. The source operand can be in single precision, double precision, or double extended-precision floating-point format. If the source operand is in single precision or double precision floating-point format, it is automatically converted to the double extended-precision floating-point format before being pushed on the stack.
    /// Reference: [Intel x86 docs for FLD](https://www.felixcloutier.com/x86/FLD.html)
    fn fldm80(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDM80, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSTPM80` (`FSTP`). The FST instruction copies the value in the ST(0) register to the destination operand, which can be a memory location or another register in the FPU register stack. When storing the value in memory, the value is converted to single precision or double precision floating-point format.
    /// Reference: [Intel x86 docs for FSTP](https://www.felixcloutier.com/x86/FST%3AFSTP.html)
    fn fstpm80(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTPM80, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCLEX` (`FCLEX`). Clears the floating-point exception flags (PE, UE, OE, ZE, DE, and IE), the exception summary status flag (ES), the stack fault flag (SF), and the busy flag (B) in the FPU status word. The FCLEX instruction checks for and handles any pending unmasked floating-point exceptions before clearing the exception flags; the FNCLEX instruction does not.
    /// Reference: [Intel x86 docs for FCLEX](https://www.felixcloutier.com/x86/FCLEX%3AFNCLEX.html)
    fn fclex(&mut self,) -> () {
        self.emit(FCLEX, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FINIT` (`FINIT`). Sets the FPU control, status, tag, instruction pointer, and data pointer registers to their default states. The FPU control word is set to 037FH (round to nearest, all exceptions masked, 64-bit precision). The status word is cleared (no exception flags set, TOP is set to 0). The data registers in the register stack are left unchanged, but they are all tagged as empty (11B). Both the instruction and data pointers are cleared.
    /// Reference: [Intel x86 docs for FINIT](https://www.felixcloutier.com/x86/FINIT%3AFNINIT.html)
    fn finit(&mut self,) -> () {
        self.emit(FINIT, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FADDM64` (`FADD`). Adds the destination and source operands and stores the sum in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FADD](https://www.felixcloutier.com/x86/FADD%3AFADDP%3AFIADD.html)
    fn faddm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FADDM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FMULM64` (`FMUL`). Multiplies the destination and source operands and stores the product in the destination location. The destination operand is always an FPU data register; the source operand can be an FPU data register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FMUL](https://www.felixcloutier.com/x86/FMUL%3AFMULP%3AFIMUL.html)
    fn fmulm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FMULM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCOMM64` (`FCOM`). Compares the contents of register ST(0) and source value and sets condition code flags C0, C2, and C3 in the FPU status word according to the results (see the table below). The source operand can be a data register or a memory location. If no source operand is given, the value in ST(0) is compared with the value in ST(1). The sign of zero is ignored, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FCOM](https://www.felixcloutier.com/x86/FCOM%3AFCOMP%3AFCOMPP.html)
    fn fcomm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCOMM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCOMPM64` (`FCOMP`). Compares the contents of register ST(0) and source value and sets condition code flags C0, C2, and C3 in the FPU status word according to the results (see the table below). The source operand can be a data register or a memory location. If no source operand is given, the value in ST(0) is compared with the value in ST(1). The sign of zero is ignored, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FCOMP](https://www.felixcloutier.com/x86/FCOM%3AFCOMP%3AFCOMPP.html)
    fn fcompm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCOMPM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSUBM64` (`FSUB`). Subtracts the source operand from the destination operand and stores the difference in the destination location. The destination operand is always an FPU data register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FSUB](https://www.felixcloutier.com/x86/FSUB%3AFSUBP%3AFISUB.html)
    fn fsubm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSUBM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSUBRM64` (`FSUBR`). Subtracts the destination operand from the source operand and stores the difference in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FSUBR](https://www.felixcloutier.com/x86/FSUBR%3AFSUBRP%3AFISUBR.html)
    fn fsubrm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSUBRM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FDIVM64` (`FDIV`). Divides the destination operand by the source operand and stores the result in the destination location. The destination operand (dividend) is always in an FPU register; the source operand (divisor) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FDIV](https://www.felixcloutier.com/x86/FDIV%3AFDIVP%3AFIDIV.html)
    fn fdivm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FDIVM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FDIVRM64` (`FDIVR`). Divides the source operand by the destination operand and stores the result in the destination location. The destination operand (divisor) is always in an FPU register; the source operand (dividend) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FDIVR](https://www.felixcloutier.com/x86/FDIVR%3AFDIVRP%3AFIDIVR.html)
    fn fdivrm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FDIVRM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FLDM64` (`FLD`). Pushes the source operand onto the FPU register stack. The source operand can be in single precision, double precision, or double extended-precision floating-point format. If the source operand is in single precision or double precision floating-point format, it is automatically converted to the double extended-precision floating-point format before being pushed on the stack.
    /// Reference: [Intel x86 docs for FLD](https://www.felixcloutier.com/x86/FLD.html)
    fn fldm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISTTPM64` (`FISTTP`). FISTTP converts the value in ST into a signed integer using truncation (chop) as rounding mode, transfers the result to the destination, and pop ST. FISTTP accepts word, short integer, and long integer destinations.
    /// Reference: [Intel x86 docs for FISTTP](https://www.felixcloutier.com/x86/FISTTP.html)
    fn fisttpm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTTPM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSTM64` (`FST`). The FST instruction copies the value in the ST(0) register to the destination operand, which can be a memory location or another register in the FPU register stack. When storing the value in memory, the value is converted to single precision or double precision floating-point format.
    /// Reference: [Intel x86 docs for FST](https://www.felixcloutier.com/x86/FST%3AFSTP.html)
    fn fstm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSTPM64` (`FSTP`). The FST instruction copies the value in the ST(0) register to the destination operand, which can be a memory location or another register in the FPU register stack. When storing the value in memory, the value is converted to single precision or double precision floating-point format.
    /// Reference: [Intel x86 docs for FSTP](https://www.felixcloutier.com/x86/FST%3AFSTP.html)
    fn fstpm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTPM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FRSTORM`.
    fn frstorm(&mut self,op0: impl OperandCast) -> () {
        self.emit(FRSTORM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSAVEM` (`FSAVE`). Stores the current FPU state (operating environment and register stack) at the specified destination in memory, and then re-initializes the FPU. The FSAVE instruction checks for and handles pending unmasked floating-point exceptions before storing the FPU state; the FNSAVE instruction does not.
    /// Reference: [Intel x86 docs for FSAVE](https://www.felixcloutier.com/x86/FSAVE%3AFNSAVE.html)
    fn fsavem(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSAVEM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSTSWM` (`FSTSW`). Stores the current value of the x87 FPU status word in the destination location. The destination operand can be either a two-byte memory location or the AX register. The FSTSW instruction checks for and handles pending unmasked floating-point exceptions before storing the status word; the FNSTSW instruction does not.
    /// Reference: [Intel x86 docs for FSTSW](https://www.felixcloutier.com/x86/FSTSW%3AFNSTSW.html)
    fn fstswm(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTSWM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FFREER`.
    fn ffreer(&mut self,op0: impl OperandCast) -> () {
        self.emit(FFREER, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSTR` (`FST`). The FST instruction copies the value in the ST(0) register to the destination operand, which can be a memory location or another register in the FPU register stack. When storing the value in memory, the value is converted to single precision or double precision floating-point format.
    /// Reference: [Intel x86 docs for FST](https://www.felixcloutier.com/x86/FST%3AFSTP.html)
    fn fstr(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSTPR` (`FSTP`). The FST instruction copies the value in the ST(0) register to the destination operand, which can be a memory location or another register in the FPU register stack. When storing the value in memory, the value is converted to single precision or double precision floating-point format.
    /// Reference: [Intel x86 docs for FSTP](https://www.felixcloutier.com/x86/FST%3AFSTP.html)
    fn fstpr(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTPR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FUCOMR` (`FUCOM`). Performs an unordered comparison of the contents of register ST(0) and ST(i) and sets condition code flags C0, C2, and C3 in the FPU status word according to the results (see the table below). If no operand is specified, the contents of registers ST(0) and ST(1) are compared. The sign of zero is ignored, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FUCOM](https://www.felixcloutier.com/x86/FUCOM%3AFUCOMP%3AFUCOMPP.html)
    fn fucomr(&mut self,op0: impl OperandCast) -> () {
        self.emit(FUCOMR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FUCOMPR` (`FUCOMP`). Performs an unordered comparison of the contents of register ST(0) and ST(i) and sets condition code flags C0, C2, and C3 in the FPU status word according to the results (see the table below). If no operand is specified, the contents of registers ST(0) and ST(1) are compared. The sign of zero is ignored, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FUCOMP](https://www.felixcloutier.com/x86/FUCOM%3AFUCOMP%3AFUCOMPP.html)
    fn fucompr(&mut self,op0: impl OperandCast) -> () {
        self.emit(FUCOMPR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FIADDM16` (`FIADD`). Adds the destination and source operands and stores the sum in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FIADD](https://www.felixcloutier.com/x86/FADD%3AFADDP%3AFIADD.html)
    fn fiaddm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIADDM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FIMULM16` (`FIMUL`). Multiplies the destination and source operands and stores the product in the destination location. The destination operand is always an FPU data register; the source operand can be an FPU data register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FIMUL](https://www.felixcloutier.com/x86/FMUL%3AFMULP%3AFIMUL.html)
    fn fimulm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIMULM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FICOMM16` (`FICOM`). Compares the value in ST(0) with an integer source operand and sets the condition code flags C0, C2, and C3 in the FPU status word according to the results (see table below). The integer value is converted to double extended-precision floating-point format before the comparison is made.
    /// Reference: [Intel x86 docs for FICOM](https://www.felixcloutier.com/x86/FICOM%3AFICOMP.html)
    fn ficomm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FICOMM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FICOMPM16` (`FICOMP`). Compares the value in ST(0) with an integer source operand and sets the condition code flags C0, C2, and C3 in the FPU status word according to the results (see table below). The integer value is converted to double extended-precision floating-point format before the comparison is made.
    /// Reference: [Intel x86 docs for FICOMP](https://www.felixcloutier.com/x86/FICOM%3AFICOMP.html)
    fn ficompm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FICOMPM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISUBM16` (`FISUB`). Subtracts the source operand from the destination operand and stores the difference in the destination location. The destination operand is always an FPU data register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FISUB](https://www.felixcloutier.com/x86/FSUB%3AFSUBP%3AFISUB.html)
    fn fisubm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISUBM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISUBRM16` (`FISUBR`). Subtracts the destination operand from the source operand and stores the difference in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FISUBR](https://www.felixcloutier.com/x86/FSUBR%3AFSUBRP%3AFISUBR.html)
    fn fisubrm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISUBRM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FIDIVM16` (`FIDIV`). Divides the destination operand by the source operand and stores the result in the destination location. The destination operand (dividend) is always in an FPU register; the source operand (divisor) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FIDIV](https://www.felixcloutier.com/x86/FDIV%3AFDIVP%3AFIDIV.html)
    fn fidivm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIDIVM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FIDIVRM16` (`FIDIVR`). Divides the source operand by the destination operand and stores the result in the destination location. The destination operand (divisor) is always in an FPU register; the source operand (dividend) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FIDIVR](https://www.felixcloutier.com/x86/FDIVR%3AFDIVRP%3AFIDIVR.html)
    fn fidivrm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIDIVRM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FADDPRR` (`FADDP`). Adds the destination and source operands and stores the sum in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FADDP](https://www.felixcloutier.com/x86/FADD%3AFADDP%3AFIADD.html)
    fn faddprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FADDPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FMULPRR` (`FMULP`). Multiplies the destination and source operands and stores the product in the destination location. The destination operand is always an FPU data register; the source operand can be an FPU data register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FMULP](https://www.felixcloutier.com/x86/FMUL%3AFMULP%3AFIMUL.html)
    fn fmulprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FMULPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FCOMPP` (`FCOMPP`). Compares the contents of register ST(0) and source value and sets condition code flags C0, C2, and C3 in the FPU status word according to the results (see the table below). The source operand can be a data register or a memory location. If no source operand is given, the value in ST(0) is compared with the value in ST(1). The sign of zero is ignored, so that –0.0 is equal to +0.0.
    /// Reference: [Intel x86 docs for FCOMPP](https://www.felixcloutier.com/x86/FCOM%3AFCOMP%3AFCOMPP.html)
    fn fcompp(&mut self,) -> () {
        self.emit(FCOMPP, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSUBRPRR` (`FSUBRP`). Subtracts the destination operand from the source operand and stores the difference in the destination location. The destination operand is always an FPU register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FSUBRP](https://www.felixcloutier.com/x86/FSUBR%3AFSUBRP%3AFISUBR.html)
    fn fsubrprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FSUBRPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSUBPRR` (`FSUBP`). Subtracts the source operand from the destination operand and stores the difference in the destination location. The destination operand is always an FPU data register; the source operand can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.
    /// Reference: [Intel x86 docs for FSUBP](https://www.felixcloutier.com/x86/FSUB%3AFSUBP%3AFISUB.html)
    fn fsubprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FSUBPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FDIVRPRR` (`FDIVRP`). Divides the source operand by the destination operand and stores the result in the destination location. The destination operand (divisor) is always in an FPU register; the source operand (dividend) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FDIVRP](https://www.felixcloutier.com/x86/FDIVR%3AFDIVRP%3AFIDIVR.html)
    fn fdivrprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FDIVRPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FDIVPRR` (`FDIVP`). Divides the destination operand by the source operand and stores the result in the destination location. The destination operand (dividend) is always in an FPU register; the source operand (divisor) can be a register or a memory location. Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.
    /// Reference: [Intel x86 docs for FDIVP](https://www.felixcloutier.com/x86/FDIV%3AFDIVP%3AFIDIV.html)
    fn fdivprr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FDIVPRR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FILDM16` (`FILD`). Converts the signed-integer source operand into double extended-precision floating-point format and pushes the value onto the FPU register stack. The source operand can be a word, doubleword, or quadword integer. It is loaded without rounding errors. The sign of the source operand is preserved.
    /// Reference: [Intel x86 docs for FILD](https://www.felixcloutier.com/x86/FILD.html)
    fn fildm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FILDM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISTTPM16` (`FISTTP`). FISTTP converts the value in ST into a signed integer using truncation (chop) as rounding mode, transfers the result to the destination, and pop ST. FISTTP accepts word, short integer, and long integer destinations.
    /// Reference: [Intel x86 docs for FISTTP](https://www.felixcloutier.com/x86/FISTTP.html)
    fn fisttpm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTTPM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISTM16` (`FIST`). The FIST instruction converts the value in the ST(0) register to a signed integer and stores the result in the destination operand. Values can be stored in word or doubleword integer format. The destination operand specifies the address where the first byte of the destination value is to be stored.
    /// Reference: [Intel x86 docs for FIST](https://www.felixcloutier.com/x86/FIST%3AFISTP.html)
    fn fistm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISTPM16` (`FISTP`). The FIST instruction converts the value in the ST(0) register to a signed integer and stores the result in the destination operand. Values can be stored in word or doubleword integer format. The destination operand specifies the address where the first byte of the destination value is to be stored.
    /// Reference: [Intel x86 docs for FISTP](https://www.felixcloutier.com/x86/FIST%3AFISTP.html)
    fn fistpm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTPM16, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FBLDM` (`FBLD`). Converts the BCD source operand into double extended-precision floating-point format and pushes the value onto the FPU stack. The source operand is loaded without rounding errors. The sign of the source operand is preserved, including that of −0.
    /// Reference: [Intel x86 docs for FBLD](https://www.felixcloutier.com/x86/FBLD.html)
    fn fbldm(&mut self,op0: impl OperandCast) -> () {
        self.emit(FBLDM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FILDM64` (`FILD`). Converts the signed-integer source operand into double extended-precision floating-point format and pushes the value onto the FPU register stack. The source operand can be a word, doubleword, or quadword integer. It is loaded without rounding errors. The sign of the source operand is preserved.
    /// Reference: [Intel x86 docs for FILD](https://www.felixcloutier.com/x86/FILD.html)
    fn fildm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FILDM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FBSTPM`.
    fn fbstpm(&mut self,op0: impl OperandCast) -> () {
        self.emit(FBSTPM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FISTPM64` (`FISTP`). The FIST instruction converts the value in the ST(0) register to a signed integer and stores the result in the destination operand. Values can be stored in word or doubleword integer format. The destination operand specifies the address where the first byte of the destination value is to be stored.
    /// Reference: [Intel x86 docs for FISTP](https://www.felixcloutier.com/x86/FIST%3AFISTP.html)
    fn fistpm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTPM64, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `FSTSWR` (`FSTSW`). Stores the current value of the x87 FPU status word in the destination location. The destination operand can be either a two-byte memory location or the AX register. The FSTSW instruction checks for and handles pending unmasked floating-point exceptions before storing the status word; the FNSTSW instruction does not.
    /// Reference: [Intel x86 docs for FSTSW](https://www.felixcloutier.com/x86/FSTSW%3AFNSTSW.html)
    fn fstswr(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTSWR, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
