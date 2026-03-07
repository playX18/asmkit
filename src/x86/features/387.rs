pub trait X86387Emitter: Emitter {
    /// Emits `F2XM1`.
    fn f2xm1(&mut self,) -> Result<(), AsmError> {
        self.emit(F2XM1, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FABS`.
    fn fabs(&mut self,) -> Result<(), AsmError> {
        self.emit(FABS, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FADDRR`.
    fn fadd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FADDRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FADDPRR`.
    fn faddp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FADDPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FADDM32`.
    fn faddm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FADDM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FADDM64`.
    fn faddm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FADDM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FBLDM`.
    fn fbld(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FBLDM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FBSTPM`.
    fn fbstp(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FBSTPM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCHS`.
    fn fchs(&mut self,) -> Result<(), AsmError> {
        self.emit(FCHS, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCLEX`.
    fn fclex(&mut self,) -> Result<(), AsmError> {
        self.emit(FCLEX, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCOMRR`.
    fn fcom(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCOMRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCOMPRR`.
    fn fcomp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCOMPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCOMPP`.
    fn fcompp(&mut self,) -> Result<(), AsmError> {
        self.emit(FCOMPP, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCOMPM32`.
    fn fcompm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCOMPM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCOMPM64`.
    fn fcompm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCOMPM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCOMM32`.
    fn fcomm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCOMM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCOMM64`.
    fn fcomm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FCOMM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FCOS`.
    fn fcos(&mut self,) -> Result<(), AsmError> {
        self.emit(FCOS, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FDECSTP`.
    fn fdecstp(&mut self,) -> Result<(), AsmError> {
        self.emit(FDECSTP, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FDIVRR`.
    fn fdiv(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FDIVRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FDIVPRR`.
    fn fdivp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FDIVPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FDIVRRR`.
    fn fdivr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FDIVRRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FDIVRPRR`.
    fn fdivrp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FDIVRPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FDIVRM32`.
    fn fdivrm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FDIVRM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FDIVRM64`.
    fn fdivrm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FDIVRM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FDIVM32`.
    fn fdivm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FDIVM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FDIVM64`.
    fn fdivm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FDIVM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FFREER`.
    fn ffree(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FFREER, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FIADDM16`.
    fn fiaddm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FIADDM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FIADDM32`.
    fn fiaddm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FIADDM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FICOMPM16`.
    fn ficompm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FICOMPM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FICOMPM32`.
    fn ficompm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FICOMPM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FICOMM16`.
    fn ficomm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FICOMM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FICOMM32`.
    fn ficomm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FICOMM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FIDIVRM16`.
    fn fidivrm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FIDIVRM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FIDIVRM32`.
    fn fidivrm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FIDIVRM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FIDIVM16`.
    fn fidivm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FIDIVM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FIDIVM32`.
    fn fidivm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FIDIVM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FILDM16`.
    fn fildm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FILDM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FILDM32`.
    fn fildm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FILDM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FILDM64`.
    fn fildm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FILDM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FIMULM16`.
    fn fimulm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FIMULM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FIMULM32`.
    fn fimulm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FIMULM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FINCSTP`.
    fn fincstp(&mut self,) -> Result<(), AsmError> {
        self.emit(FINCSTP, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FINIT`.
    fn finit(&mut self,) -> Result<(), AsmError> {
        self.emit(FINIT, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FISTPM16`.
    fn fistpm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FISTPM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FISTPM32`.
    fn fistpm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FISTPM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FISTPM64`.
    fn fistpm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FISTPM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FISTTPM16`.
    fn fisttpm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FISTTPM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FISTTPM64`.
    fn fisttpm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FISTTPM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FISTM16`.
    fn fistm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FISTM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FISTM32`.
    fn fistm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FISTM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FISUBRM16`.
    fn fisubrm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FISUBRM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FISUBRM32`.
    fn fisubrm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FISUBRM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FISUBM16`.
    fn fisubm16(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FISUBM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FISUBM32`.
    fn fisubm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FISUBM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDR`.
    fn fld(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FLDR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLD1`.
    fn fld1(&mut self,) -> Result<(), AsmError> {
        self.emit(FLD1, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDCWM`.
    fn fldcw(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FLDCWM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDENVM`.
    fn fldenv(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FLDENVM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDL2E`.
    fn fldl2e(&mut self,) -> Result<(), AsmError> {
        self.emit(FLDL2E, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDL2T`.
    fn fldl2t(&mut self,) -> Result<(), AsmError> {
        self.emit(FLDL2T, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDLG2`.
    fn fldlg2(&mut self,) -> Result<(), AsmError> {
        self.emit(FLDLG2, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDLN2`.
    fn fldln2(&mut self,) -> Result<(), AsmError> {
        self.emit(FLDLN2, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDPI`.
    fn fldpi(&mut self,) -> Result<(), AsmError> {
        self.emit(FLDPI, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDZ`.
    fn fldz(&mut self,) -> Result<(), AsmError> {
        self.emit(FLDZ, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDM32`.
    fn fldm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FLDM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDM64`.
    fn fldm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FLDM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FLDM80`.
    fn fldm80(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FLDM80, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FMULRR`.
    fn fmul(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FMULRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FMULPRR`.
    fn fmulp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FMULPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FMULM32`.
    fn fmulm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FMULM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FMULM64`.
    fn fmulm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FMULM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FNOP`.
    fn fnop(&mut self,) -> Result<(), AsmError> {
        self.emit(FNOP, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FPATAN`.
    fn fpatan(&mut self,) -> Result<(), AsmError> {
        self.emit(FPATAN, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FPREM`.
    fn fprem(&mut self,) -> Result<(), AsmError> {
        self.emit(FPREM, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FPREM1`.
    fn fprem1(&mut self,) -> Result<(), AsmError> {
        self.emit(FPREM1, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FPTAN`.
    fn fptan(&mut self,) -> Result<(), AsmError> {
        self.emit(FPTAN, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FRNDINT`.
    fn frndint(&mut self,) -> Result<(), AsmError> {
        self.emit(FRNDINT, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FRSTORM`.
    fn frstor(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FRSTORM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSAVEM`.
    fn fsave(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSAVEM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSCALE`.
    fn fscale(&mut self,) -> Result<(), AsmError> {
        self.emit(FSCALE, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSIN`.
    fn fsin(&mut self,) -> Result<(), AsmError> {
        self.emit(FSIN, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSINCOS`.
    fn fsincos(&mut self,) -> Result<(), AsmError> {
        self.emit(FSINCOS, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSQRT`.
    fn fsqrt(&mut self,) -> Result<(), AsmError> {
        self.emit(FSQRT, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSTR`.
    fn fst(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSTR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSTCWM`.
    fn fstcw(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSTCWM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSTENVM`.
    fn fstenv(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSTENVM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSTPR`.
    fn fstp(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSTPR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSTPM32`.
    fn fstpm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSTPM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSTPM64`.
    fn fstpm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSTPM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSTPM80`.
    fn fstpm80(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSTPM80, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSTSW`.
    fn fstsw(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        let op0 = op0.as_operand();
        if op0.is_mem() {
            self.emit(FSTSWM, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_gp() {
            self.emit(FSTSWR, op0,&NOREG,&NOREG,&NOREG);
        } else {
            return Err(AsmError::X86(X86Error::InvalidOperandCombination { mnemonic: "FSTSW" }));
        }
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSTM32`.
    fn fstm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSTM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSTM64`.
    fn fstm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSTM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSUBRR`.
    fn fsub(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSUBRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSUBPRR`.
    fn fsubp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSUBPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSUBRRR`.
    fn fsubr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSUBRRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSUBRPRR`.
    fn fsubrp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSUBRPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSUBRM32`.
    fn fsubrm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSUBRM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSUBRM64`.
    fn fsubrm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSUBRM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSUBM32`.
    fn fsubm32(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSUBM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FSUBM64`.
    fn fsubm64(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FSUBM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FTST`.
    fn ftst(&mut self,) -> Result<(), AsmError> {
        self.emit(FTST, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FUCOMR`.
    fn fucom(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FUCOMR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FUCOMPR`.
    fn fucomp(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FUCOMPR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FUCOMPP`.
    fn fucompp(&mut self,) -> Result<(), AsmError> {
        self.emit(FUCOMPP, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FXAM`.
    fn fxam(&mut self,) -> Result<(), AsmError> {
        self.emit(FXAM, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FXCHR`.
    fn fxch(&mut self,op0: impl OperandCast) -> Result<(), AsmError> {
        self.emit(FXCHR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FXTRACT`.
    fn fxtract(&mut self,) -> Result<(), AsmError> {
        self.emit(FXTRACT, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FYL2X`.
    fn fyl2x(&mut self,) -> Result<(), AsmError> {
        self.emit(FYL2X, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
    /// Emits `FYL2XP1`.
    fn fyl2xp1(&mut self,) -> Result<(), AsmError> {
        self.emit(FYL2XP1, &NOREG,&NOREG,&NOREG,&NOREG);
        if let Some(err) = self.last_error() { Err(err) } else { Ok(()) }
    }
}
