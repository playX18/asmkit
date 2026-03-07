pub trait X86387Emitter: Emitter {
    /// Emits `F2XM1`.
    fn f2xm1(&mut self,) -> () {
        self.emit(F2XM1, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FABS`.
    fn fabs(&mut self,) -> () {
        self.emit(FABS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FADDRR`.
    fn fadd(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FADDRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FADDPRR`.
    fn faddp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FADDPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FADDM32`.
    fn faddm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FADDM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FADDM64`.
    fn faddm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FADDM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FBLDM`.
    fn fbld(&mut self,op0: impl OperandCast) -> () {
        self.emit(FBLDM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FBSTPM`.
    fn fbstp(&mut self,op0: impl OperandCast) -> () {
        self.emit(FBSTPM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCHS`.
    fn fchs(&mut self,) -> () {
        self.emit(FCHS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCLEX`.
    fn fclex(&mut self,) -> () {
        self.emit(FCLEX, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCOMRR`.
    fn fcom(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FCOMRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FCOMPRR`.
    fn fcomp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FCOMPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FCOMPP`.
    fn fcompp(&mut self,) -> () {
        self.emit(FCOMPP, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCOMPM32`.
    fn fcompm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCOMPM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCOMPM64`.
    fn fcompm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCOMPM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCOMM32`.
    fn fcomm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCOMM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCOMM64`.
    fn fcomm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FCOMM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FCOS`.
    fn fcos(&mut self,) -> () {
        self.emit(FCOS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FDECSTP`.
    fn fdecstp(&mut self,) -> () {
        self.emit(FDECSTP, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FDIVRR`.
    fn fdiv(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FDIVRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FDIVPRR`.
    fn fdivp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FDIVPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FDIVRRR`.
    fn fdivr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FDIVRRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FDIVRPRR`.
    fn fdivrp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FDIVRPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FDIVRM32`.
    fn fdivrm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FDIVRM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FDIVRM64`.
    fn fdivrm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FDIVRM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FDIVM32`.
    fn fdivm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FDIVM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FDIVM64`.
    fn fdivm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FDIVM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FFREER`.
    fn ffree(&mut self,op0: impl OperandCast) -> () {
        self.emit(FFREER, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FIADDM16`.
    fn fiaddm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIADDM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FIADDM32`.
    fn fiaddm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIADDM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FICOMPM16`.
    fn ficompm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FICOMPM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FICOMPM32`.
    fn ficompm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FICOMPM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FICOMM16`.
    fn ficomm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FICOMM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FICOMM32`.
    fn ficomm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FICOMM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FIDIVRM16`.
    fn fidivrm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIDIVRM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FIDIVRM32`.
    fn fidivrm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIDIVRM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FIDIVM16`.
    fn fidivm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIDIVM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FIDIVM32`.
    fn fidivm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIDIVM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FILDM16`.
    fn fildm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FILDM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FILDM32`.
    fn fildm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FILDM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FILDM64`.
    fn fildm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FILDM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FIMULM16`.
    fn fimulm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIMULM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FIMULM32`.
    fn fimulm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FIMULM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FINCSTP`.
    fn fincstp(&mut self,) -> () {
        self.emit(FINCSTP, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FINIT`.
    fn finit(&mut self,) -> () {
        self.emit(FINIT, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FISTPM16`.
    fn fistpm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTPM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FISTPM32`.
    fn fistpm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTPM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FISTPM64`.
    fn fistpm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTPM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FISTTPM16`.
    fn fisttpm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTTPM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FISTTPM64`.
    fn fisttpm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTTPM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FISTM16`.
    fn fistm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FISTM32`.
    fn fistm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISTM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FISUBRM16`.
    fn fisubrm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISUBRM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FISUBRM32`.
    fn fisubrm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISUBRM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FISUBM16`.
    fn fisubm16(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISUBM16, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FISUBM32`.
    fn fisubm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FISUBM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDR`.
    fn fld(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLD1`.
    fn fld1(&mut self,) -> () {
        self.emit(FLD1, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDCWM`.
    fn fldcw(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDCWM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDENVM`.
    fn fldenv(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDENVM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDL2E`.
    fn fldl2e(&mut self,) -> () {
        self.emit(FLDL2E, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDL2T`.
    fn fldl2t(&mut self,) -> () {
        self.emit(FLDL2T, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDLG2`.
    fn fldlg2(&mut self,) -> () {
        self.emit(FLDLG2, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDLN2`.
    fn fldln2(&mut self,) -> () {
        self.emit(FLDLN2, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDPI`.
    fn fldpi(&mut self,) -> () {
        self.emit(FLDPI, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDZ`.
    fn fldz(&mut self,) -> () {
        self.emit(FLDZ, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDM32`.
    fn fldm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDM64`.
    fn fldm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FLDM80`.
    fn fldm80(&mut self,op0: impl OperandCast) -> () {
        self.emit(FLDM80, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FMULRR`.
    fn fmul(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FMULRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FMULPRR`.
    fn fmulp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FMULPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FMULM32`.
    fn fmulm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FMULM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FMULM64`.
    fn fmulm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FMULM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FNOP`.
    fn fnop(&mut self,) -> () {
        self.emit(FNOP, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FPATAN`.
    fn fpatan(&mut self,) -> () {
        self.emit(FPATAN, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FPREM`.
    fn fprem(&mut self,) -> () {
        self.emit(FPREM, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FPREM1`.
    fn fprem1(&mut self,) -> () {
        self.emit(FPREM1, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FPTAN`.
    fn fptan(&mut self,) -> () {
        self.emit(FPTAN, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FRNDINT`.
    fn frndint(&mut self,) -> () {
        self.emit(FRNDINT, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FRSTORM`.
    fn frstor(&mut self,op0: impl OperandCast) -> () {
        self.emit(FRSTORM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSAVEM`.
    fn fsave(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSAVEM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSCALE`.
    fn fscale(&mut self,) -> () {
        self.emit(FSCALE, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSIN`.
    fn fsin(&mut self,) -> () {
        self.emit(FSIN, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSINCOS`.
    fn fsincos(&mut self,) -> () {
        self.emit(FSINCOS, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSQRT`.
    fn fsqrt(&mut self,) -> () {
        self.emit(FSQRT, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSTR`.
    fn fst(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSTCWM`.
    fn fstcw(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTCWM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSTENVM`.
    fn fstenv(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTENVM, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSTPR`.
    fn fstp(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTPR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSTPM32`.
    fn fstpm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTPM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSTPM64`.
    fn fstpm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTPM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSTPM80`.
    fn fstpm80(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTPM80, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSTSW`.
    fn fstsw(&mut self,op0: impl OperandCast) -> () {
        let op0 = op0.as_operand();
        if op0.is_mem() {
            self.emit(FSTSWM, op0,&NOREG,&NOREG,&NOREG);
        } else if op0.is_gp() {
            self.emit(FSTSWR, op0,&NOREG,&NOREG,&NOREG);
        } else {
            unreachable!("invalid operand types for FSTSW");
        }
    }
    /// Emits `FSTM32`.
    fn fstm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSTM64`.
    fn fstm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSTM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSUBRR`.
    fn fsub(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FSUBRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FSUBPRR`.
    fn fsubp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FSUBPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FSUBRRR`.
    fn fsubr(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FSUBRRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FSUBRPRR`.
    fn fsubrp(&mut self,op0: impl OperandCast,op1: impl OperandCast) -> () {
        self.emit(FSUBRPRR, op0.as_operand(),op1.as_operand(),&NOREG,&NOREG);
    }
    /// Emits `FSUBRM32`.
    fn fsubrm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSUBRM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSUBRM64`.
    fn fsubrm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSUBRM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSUBM32`.
    fn fsubm32(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSUBM32, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FSUBM64`.
    fn fsubm64(&mut self,op0: impl OperandCast) -> () {
        self.emit(FSUBM64, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FTST`.
    fn ftst(&mut self,) -> () {
        self.emit(FTST, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FUCOMR`.
    fn fucom(&mut self,op0: impl OperandCast) -> () {
        self.emit(FUCOMR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FUCOMPR`.
    fn fucomp(&mut self,op0: impl OperandCast) -> () {
        self.emit(FUCOMPR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FUCOMPP`.
    fn fucompp(&mut self,) -> () {
        self.emit(FUCOMPP, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FXAM`.
    fn fxam(&mut self,) -> () {
        self.emit(FXAM, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FXCHR`.
    fn fxch(&mut self,op0: impl OperandCast) -> () {
        self.emit(FXCHR, op0.as_operand(),&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FXTRACT`.
    fn fxtract(&mut self,) -> () {
        self.emit(FXTRACT, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FYL2X`.
    fn fyl2x(&mut self,) -> () {
        self.emit(FYL2X, &NOREG,&NOREG,&NOREG,&NOREG);
    }
    /// Emits `FYL2XP1`.
    fn fyl2xp1(&mut self,) -> () {
        self.emit(FYL2XP1, &NOREG,&NOREG,&NOREG,&NOREG);
    }
}
