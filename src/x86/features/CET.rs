pub trait X86CETEmitter: Emitter {
    /// Emits `RSTORSSPM`.
    fn rstorsspm(&mut self,op0: impl OperandCast) -> () {
        self.emit(RSTORSSPM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SETSSBSY` (`SETSSBSY`). The SETSSBSY instruction verifies the presence of a non-busy supervisor shadow stack token at the address in the IA32_PL0_SSP MSR and marks it busy. Following successful execution of the instruction, the SSP is set to the value of the IA32_PL0_SSP MSR.
    /// Reference: [Intel x86 docs for SETSSBSY](https://www.felixcloutier.com/x86/SETSSBSY.html)
    fn setssbsy(&mut self,) -> () {
        self.emit(SETSSBSY, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `SAVEPREVSSP`.
    fn saveprevssp(&mut self,) -> () {
        self.emit(SAVEPREVSSP, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RDSSP32R`.
    fn rdssp32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDSSP32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `RDSSP64R`.
    fn rdssp64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(RDSSP64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ENDBR64` (`ENDBR64`). Terminate an indirect branch in 64 bit mode.
    /// Reference: [Intel x86 docs for ENDBR64](https://www.felixcloutier.com/x86/ENDBR64.html)
    fn endbr64(&mut self,) -> () {
        self.emit(ENDBR64, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `ENDBR32` (`ENDBR32`). Terminate an indirect branch in 32 bit and compatibility mode.
    /// Reference: [Intel x86 docs for ENDBR32](https://www.felixcloutier.com/x86/ENDBR32.html)
    fn endbr32(&mut self,) -> () {
        self.emit(ENDBR32, &NOREG /* op0 */,&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WRUSS32MR`.
    fn wruss32mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(WRUSS32MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WRUSS64MR`.
    fn wruss64mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(WRUSS64MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WRSS32MR`.
    fn wrss32mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(WRSS32MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `WRSS64MR`.
    fn wrss64mr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(WRSS64MR, op0.as_operand(),op1.as_operand(),&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `CLRSSBSYM` (`CLRSSBSY`). Clear busy flag in supervisor shadow stack token reference by m64. Subsequent to marking the shadow stack as not busy the SSP is loaded with value 0.
    /// Reference: [Intel x86 docs for CLRSSBSY](https://www.felixcloutier.com/x86/CLRSSBSY.html)
    fn clrssbsym(&mut self,op0: impl OperandCast) -> () {
        self.emit(CLRSSBSYM, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `INCSSP32R`.
    fn incssp32r(&mut self,op0: impl OperandCast) -> () {
        self.emit(INCSSP32R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
    /// Emits `INCSSP64R`.
    fn incssp64r(&mut self,op0: impl OperandCast) -> () {
        self.emit(INCSSP64R, op0.as_operand(),&NOREG /* op1 */,&NOREG /* op2 */,&NOREG /* op3 */)
    }
}
