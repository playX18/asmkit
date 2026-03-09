use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `F2XM1`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait F2xm1Emitter {
    fn f2xm1(&mut self);
}

impl<'a> F2xm1Emitter for Assembler<'a> {
    fn f2xm1(&mut self) {
        self.emit(F2XM1, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FABS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FabsEmitter {
    fn fabs(&mut self);
}

impl<'a> FabsEmitter for Assembler<'a> {
    fn fabs(&mut self) {
        self.emit(FABS, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FaddEmitter_1<A> {
    fn fadd_1(&mut self, op0: A);
}

impl<'a> FaddEmitter_1<Mem> for Assembler<'a> {
    fn fadd_1(&mut self, op0: Mem) {
        self.emit(FADDM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FaddEmitter_2<A, B> {
    fn fadd_2(&mut self, op0: A, op1: B);
}

impl<'a> FaddEmitter_2<St, St> for Assembler<'a> {
    fn fadd_2(&mut self, op0: St, op1: St) {
        self.emit(FADDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FADDP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FaddpEmitter<A, B> {
    fn faddp(&mut self, op0: A, op1: B);
}

impl<'a> FaddpEmitter<St, St> for Assembler<'a> {
    fn faddp(&mut self, op0: St, op1: St) {
        self.emit(FADDPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FBLD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FbldEmitter<A> {
    fn fbld(&mut self, op0: A);
}

impl<'a> FbldEmitter<Mem> for Assembler<'a> {
    fn fbld(&mut self, op0: Mem) {
        self.emit(FBLDM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FBSTP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FbstpEmitter<A> {
    fn fbstp(&mut self, op0: A);
}

impl<'a> FbstpEmitter<Mem> for Assembler<'a> {
    fn fbstp(&mut self, op0: Mem) {
        self.emit(FBSTPM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCHS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FchsEmitter {
    fn fchs(&mut self);
}

impl<'a> FchsEmitter for Assembler<'a> {
    fn fchs(&mut self) {
        self.emit(FCHS, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FCLEX`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FclexEmitter {
    fn fclex(&mut self);
}

impl<'a> FclexEmitter for Assembler<'a> {
    fn fclex(&mut self) {
        self.emit(FCLEX, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FCOM`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FcomEmitter_1<A> {
    fn fcom_1(&mut self, op0: A);
}

impl<'a> FcomEmitter_1<Mem> for Assembler<'a> {
    fn fcom_1(&mut self, op0: Mem) {
        self.emit(FCOMM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCOM`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FcomEmitter_2<A, B> {
    fn fcom_2(&mut self, op0: A, op1: B);
}

impl<'a> FcomEmitter_2<St, St> for Assembler<'a> {
    fn fcom_2(&mut self, op0: St, op1: St) {
        self.emit(FCOMRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FCOMP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FcompEmitter_1<A> {
    fn fcomp_1(&mut self, op0: A);
}

impl<'a> FcompEmitter_1<Mem> for Assembler<'a> {
    fn fcomp_1(&mut self, op0: Mem) {
        self.emit(FCOMPM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCOMP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FcompEmitter_2<A, B> {
    fn fcomp_2(&mut self, op0: A, op1: B);
}

impl<'a> FcompEmitter_2<St, St> for Assembler<'a> {
    fn fcomp_2(&mut self, op0: St, op1: St) {
        self.emit(FCOMPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FCOMPP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FcomppEmitter {
    fn fcompp(&mut self);
}

impl<'a> FcomppEmitter for Assembler<'a> {
    fn fcompp(&mut self) {
        self.emit(FCOMPP, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FCOS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FcosEmitter {
    fn fcos(&mut self);
}

impl<'a> FcosEmitter for Assembler<'a> {
    fn fcos(&mut self) {
        self.emit(FCOS, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FDECSTP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FdecstpEmitter {
    fn fdecstp(&mut self);
}

impl<'a> FdecstpEmitter for Assembler<'a> {
    fn fdecstp(&mut self) {
        self.emit(FDECSTP, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FDIV`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FdivEmitter_1<A> {
    fn fdiv_1(&mut self, op0: A);
}

impl<'a> FdivEmitter_1<Mem> for Assembler<'a> {
    fn fdiv_1(&mut self, op0: Mem) {
        self.emit(FDIVM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FDIV`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FdivEmitter_2<A, B> {
    fn fdiv_2(&mut self, op0: A, op1: B);
}

impl<'a> FdivEmitter_2<St, St> for Assembler<'a> {
    fn fdiv_2(&mut self, op0: St, op1: St) {
        self.emit(FDIVRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FDIVP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FdivpEmitter<A, B> {
    fn fdivp(&mut self, op0: A, op1: B);
}

impl<'a> FdivpEmitter<St, St> for Assembler<'a> {
    fn fdivp(&mut self, op0: St, op1: St) {
        self.emit(FDIVPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FDIVR`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FdivrEmitter_1<A> {
    fn fdivr_1(&mut self, op0: A);
}

impl<'a> FdivrEmitter_1<Mem> for Assembler<'a> {
    fn fdivr_1(&mut self, op0: Mem) {
        self.emit(FDIVRM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FDIVR`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FdivrEmitter_2<A, B> {
    fn fdivr_2(&mut self, op0: A, op1: B);
}

impl<'a> FdivrEmitter_2<St, St> for Assembler<'a> {
    fn fdivr_2(&mut self, op0: St, op1: St) {
        self.emit(FDIVRRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FDIVRP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FdivrpEmitter<A, B> {
    fn fdivrp(&mut self, op0: A, op1: B);
}

impl<'a> FdivrpEmitter<St, St> for Assembler<'a> {
    fn fdivrp(&mut self, op0: St, op1: St) {
        self.emit(FDIVRPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FFREE`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FfreeEmitter<A> {
    fn ffree(&mut self, op0: A);
}

impl<'a> FfreeEmitter<St> for Assembler<'a> {
    fn ffree(&mut self, op0: St) {
        self.emit(FFREER, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FIADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FiaddEmitter<A> {
    fn fiadd(&mut self, op0: A);
}

impl<'a> FiaddEmitter<Mem> for Assembler<'a> {
    fn fiadd(&mut self, op0: Mem) {
        self.emit(FIADDM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FICOM`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FicomEmitter<A> {
    fn ficom(&mut self, op0: A);
}

impl<'a> FicomEmitter<Mem> for Assembler<'a> {
    fn ficom(&mut self, op0: Mem) {
        self.emit(FICOMM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FICOMP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FicompEmitter<A> {
    fn ficomp(&mut self, op0: A);
}

impl<'a> FicompEmitter<Mem> for Assembler<'a> {
    fn ficomp(&mut self, op0: Mem) {
        self.emit(FICOMPM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FIDIV`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FidivEmitter<A> {
    fn fidiv(&mut self, op0: A);
}

impl<'a> FidivEmitter<Mem> for Assembler<'a> {
    fn fidiv(&mut self, op0: Mem) {
        self.emit(FIDIVM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FIDIVR`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FidivrEmitter<A> {
    fn fidivr(&mut self, op0: A);
}

impl<'a> FidivrEmitter<Mem> for Assembler<'a> {
    fn fidivr(&mut self, op0: Mem) {
        self.emit(FIDIVRM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FILD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FildEmitter<A> {
    fn fild(&mut self, op0: A);
}

impl<'a> FildEmitter<Mem> for Assembler<'a> {
    fn fild(&mut self, op0: Mem) {
        self.emit(FILDM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FIMUL`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FimulEmitter<A> {
    fn fimul(&mut self, op0: A);
}

impl<'a> FimulEmitter<Mem> for Assembler<'a> {
    fn fimul(&mut self, op0: Mem) {
        self.emit(FIMULM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FINCSTP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FincstpEmitter {
    fn fincstp(&mut self);
}

impl<'a> FincstpEmitter for Assembler<'a> {
    fn fincstp(&mut self) {
        self.emit(FINCSTP, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FINIT`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FinitEmitter {
    fn finit(&mut self);
}

impl<'a> FinitEmitter for Assembler<'a> {
    fn finit(&mut self) {
        self.emit(FINIT, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FIST`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FistEmitter<A> {
    fn fist(&mut self, op0: A);
}

impl<'a> FistEmitter<Mem> for Assembler<'a> {
    fn fist(&mut self, op0: Mem) {
        self.emit(FISTM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FISTP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FistpEmitter<A> {
    fn fistp(&mut self, op0: A);
}

impl<'a> FistpEmitter<Mem> for Assembler<'a> {
    fn fistp(&mut self, op0: Mem) {
        self.emit(FISTPM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FISUB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FisubEmitter<A> {
    fn fisub(&mut self, op0: A);
}

impl<'a> FisubEmitter<Mem> for Assembler<'a> {
    fn fisub(&mut self, op0: Mem) {
        self.emit(FISUBM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FISUBR`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FisubrEmitter<A> {
    fn fisubr(&mut self, op0: A);
}

impl<'a> FisubrEmitter<Mem> for Assembler<'a> {
    fn fisubr(&mut self, op0: Mem) {
        self.emit(FISUBRM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FLD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// | 2 | St       |
/// +---+----------+
/// ```
pub trait FldEmitter<A> {
    fn fld(&mut self, op0: A);
}

impl<'a> FldEmitter<Mem> for Assembler<'a> {
    fn fld(&mut self, op0: Mem) {
        self.emit(FLDM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> FldEmitter<St> for Assembler<'a> {
    fn fld(&mut self, op0: St) {
        self.emit(FLDR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FLD1`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait Fld1Emitter {
    fn fld1(&mut self);
}

impl<'a> Fld1Emitter for Assembler<'a> {
    fn fld1(&mut self) {
        self.emit(FLD1, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FLDCW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FldcwEmitter<A> {
    fn fldcw(&mut self, op0: A);
}

impl<'a> FldcwEmitter<Mem> for Assembler<'a> {
    fn fldcw(&mut self, op0: Mem) {
        self.emit(FLDCWM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FLDENV`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FldenvEmitter<A> {
    fn fldenv(&mut self, op0: A);
}

impl<'a> FldenvEmitter<Mem> for Assembler<'a> {
    fn fldenv(&mut self, op0: Mem) {
        self.emit(FLDENVM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FLDL2E`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait Fldl2eEmitter {
    fn fldl2e(&mut self);
}

impl<'a> Fldl2eEmitter for Assembler<'a> {
    fn fldl2e(&mut self) {
        self.emit(FLDL2E, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FLDL2T`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait Fldl2tEmitter {
    fn fldl2t(&mut self);
}

impl<'a> Fldl2tEmitter for Assembler<'a> {
    fn fldl2t(&mut self) {
        self.emit(FLDL2T, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FLDLG2`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait Fldlg2Emitter {
    fn fldlg2(&mut self);
}

impl<'a> Fldlg2Emitter for Assembler<'a> {
    fn fldlg2(&mut self) {
        self.emit(FLDLG2, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FLDLN2`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait Fldln2Emitter {
    fn fldln2(&mut self);
}

impl<'a> Fldln2Emitter for Assembler<'a> {
    fn fldln2(&mut self) {
        self.emit(FLDLN2, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FLDPI`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FldpiEmitter {
    fn fldpi(&mut self);
}

impl<'a> FldpiEmitter for Assembler<'a> {
    fn fldpi(&mut self) {
        self.emit(FLDPI, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FLDZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FldzEmitter {
    fn fldz(&mut self);
}

impl<'a> FldzEmitter for Assembler<'a> {
    fn fldz(&mut self) {
        self.emit(FLDZ, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FMUL`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FmulEmitter_1<A> {
    fn fmul_1(&mut self, op0: A);
}

impl<'a> FmulEmitter_1<Mem> for Assembler<'a> {
    fn fmul_1(&mut self, op0: Mem) {
        self.emit(FMULM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FMUL`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FmulEmitter_2<A, B> {
    fn fmul_2(&mut self, op0: A, op1: B);
}

impl<'a> FmulEmitter_2<St, St> for Assembler<'a> {
    fn fmul_2(&mut self, op0: St, op1: St) {
        self.emit(FMULRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FMULP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FmulpEmitter<A, B> {
    fn fmulp(&mut self, op0: A, op1: B);
}

impl<'a> FmulpEmitter<St, St> for Assembler<'a> {
    fn fmulp(&mut self, op0: St, op1: St) {
        self.emit(FMULPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FNOP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FnopEmitter {
    fn fnop(&mut self);
}

impl<'a> FnopEmitter for Assembler<'a> {
    fn fnop(&mut self) {
        self.emit(FNOP, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FPATAN`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FpatanEmitter {
    fn fpatan(&mut self);
}

impl<'a> FpatanEmitter for Assembler<'a> {
    fn fpatan(&mut self) {
        self.emit(FPATAN, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FPREM`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FpremEmitter {
    fn fprem(&mut self);
}

impl<'a> FpremEmitter for Assembler<'a> {
    fn fprem(&mut self) {
        self.emit(FPREM, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FPREM1`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait Fprem1Emitter {
    fn fprem1(&mut self);
}

impl<'a> Fprem1Emitter for Assembler<'a> {
    fn fprem1(&mut self) {
        self.emit(FPREM1, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FPTAN`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FptanEmitter {
    fn fptan(&mut self);
}

impl<'a> FptanEmitter for Assembler<'a> {
    fn fptan(&mut self) {
        self.emit(FPTAN, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FRNDINT`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FrndintEmitter {
    fn frndint(&mut self);
}

impl<'a> FrndintEmitter for Assembler<'a> {
    fn frndint(&mut self) {
        self.emit(FRNDINT, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FRSTOR`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FrstorEmitter<A> {
    fn frstor(&mut self, op0: A);
}

impl<'a> FrstorEmitter<Mem> for Assembler<'a> {
    fn frstor(&mut self, op0: Mem) {
        self.emit(FRSTORM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FSAVE`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FsaveEmitter<A> {
    fn fsave(&mut self, op0: A);
}

impl<'a> FsaveEmitter<Mem> for Assembler<'a> {
    fn fsave(&mut self, op0: Mem) {
        self.emit(FSAVEM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FSCALE`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FscaleEmitter {
    fn fscale(&mut self);
}

impl<'a> FscaleEmitter for Assembler<'a> {
    fn fscale(&mut self) {
        self.emit(FSCALE, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FSIN`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FsinEmitter {
    fn fsin(&mut self);
}

impl<'a> FsinEmitter for Assembler<'a> {
    fn fsin(&mut self) {
        self.emit(FSIN, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FSINCOS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FsincosEmitter {
    fn fsincos(&mut self);
}

impl<'a> FsincosEmitter for Assembler<'a> {
    fn fsincos(&mut self) {
        self.emit(FSINCOS, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FSQRT`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FsqrtEmitter {
    fn fsqrt(&mut self);
}

impl<'a> FsqrtEmitter for Assembler<'a> {
    fn fsqrt(&mut self) {
        self.emit(FSQRT, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FST`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// | 2 | St       |
/// +---+----------+
/// ```
pub trait FstEmitter<A> {
    fn fst(&mut self, op0: A);
}

impl<'a> FstEmitter<Mem> for Assembler<'a> {
    fn fst(&mut self, op0: Mem) {
        self.emit(FSTM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> FstEmitter<St> for Assembler<'a> {
    fn fst(&mut self, op0: St) {
        self.emit(FSTR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FSTCW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FstcwEmitter<A> {
    fn fstcw(&mut self, op0: A);
}

impl<'a> FstcwEmitter<Mem> for Assembler<'a> {
    fn fstcw(&mut self, op0: Mem) {
        self.emit(FSTCWM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FSTENV`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FstenvEmitter<A> {
    fn fstenv(&mut self, op0: A);
}

impl<'a> FstenvEmitter<Mem> for Assembler<'a> {
    fn fstenv(&mut self, op0: Mem) {
        self.emit(FSTENVM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FSTP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// | 2 | St       |
/// +---+----------+
/// ```
pub trait FstpEmitter<A> {
    fn fstp(&mut self, op0: A);
}

impl<'a> FstpEmitter<Mem> for Assembler<'a> {
    fn fstp(&mut self, op0: Mem) {
        self.emit(FSTPM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> FstpEmitter<St> for Assembler<'a> {
    fn fstp(&mut self, op0: St) {
        self.emit(FSTPR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FSTSW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Mem      |
/// +---+----------+
/// ```
pub trait FstswEmitter<A> {
    fn fstsw(&mut self, op0: A);
}

impl<'a> FstswEmitter<Mem> for Assembler<'a> {
    fn fstsw(&mut self, op0: Mem) {
        self.emit(FSTSWM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> FstswEmitter<Gpd> for Assembler<'a> {
    fn fstsw(&mut self, op0: Gpd) {
        self.emit(FSTSWR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FSUB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FsubEmitter_1<A> {
    fn fsub_1(&mut self, op0: A);
}

impl<'a> FsubEmitter_1<Mem> for Assembler<'a> {
    fn fsub_1(&mut self, op0: Mem) {
        self.emit(FSUBM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FSUB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FsubEmitter_2<A, B> {
    fn fsub_2(&mut self, op0: A, op1: B);
}

impl<'a> FsubEmitter_2<St, St> for Assembler<'a> {
    fn fsub_2(&mut self, op0: St, op1: St) {
        self.emit(FSUBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FSUBP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FsubpEmitter<A, B> {
    fn fsubp(&mut self, op0: A, op1: B);
}

impl<'a> FsubpEmitter<St, St> for Assembler<'a> {
    fn fsubp(&mut self, op0: St, op1: St) {
        self.emit(FSUBPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FSUBR`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait FsubrEmitter_1<A> {
    fn fsubr_1(&mut self, op0: A);
}

impl<'a> FsubrEmitter_1<Mem> for Assembler<'a> {
    fn fsubr_1(&mut self, op0: Mem) {
        self.emit(FSUBRM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FSUBR`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FsubrEmitter_2<A, B> {
    fn fsubr_2(&mut self, op0: A, op1: B);
}

impl<'a> FsubrEmitter_2<St, St> for Assembler<'a> {
    fn fsubr_2(&mut self, op0: St, op1: St) {
        self.emit(FSUBRRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FSUBRP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FsubrpEmitter<A, B> {
    fn fsubrp(&mut self, op0: A, op1: B);
}

impl<'a> FsubrpEmitter<St, St> for Assembler<'a> {
    fn fsubrp(&mut self, op0: St, op1: St) {
        self.emit(FSUBRPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FTST`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FtstEmitter {
    fn ftst(&mut self);
}

impl<'a> FtstEmitter for Assembler<'a> {
    fn ftst(&mut self) {
        self.emit(FTST, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FUCOM`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FucomEmitter<A> {
    fn fucom(&mut self, op0: A);
}

impl<'a> FucomEmitter<St> for Assembler<'a> {
    fn fucom(&mut self, op0: St) {
        self.emit(FUCOMR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FUCOMP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FucompEmitter<A> {
    fn fucomp(&mut self, op0: A);
}

impl<'a> FucompEmitter<St> for Assembler<'a> {
    fn fucomp(&mut self, op0: St) {
        self.emit(FUCOMPR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FUCOMPP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FucomppEmitter {
    fn fucompp(&mut self);
}

impl<'a> FucomppEmitter for Assembler<'a> {
    fn fucompp(&mut self) {
        self.emit(FUCOMPP, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FXAM`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FxamEmitter {
    fn fxam(&mut self);
}

impl<'a> FxamEmitter for Assembler<'a> {
    fn fxam(&mut self) {
        self.emit(FXAM, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FXCH`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FxchEmitter<A> {
    fn fxch(&mut self, op0: A);
}

impl<'a> FxchEmitter<St> for Assembler<'a> {
    fn fxch(&mut self, op0: St) {
        self.emit(FXCHR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FXTRACT`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait FxtractEmitter {
    fn fxtract(&mut self);
}

impl<'a> FxtractEmitter for Assembler<'a> {
    fn fxtract(&mut self) {
        self.emit(FXTRACT, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FYL2X`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait Fyl2xEmitter {
    fn fyl2x(&mut self);
}

impl<'a> Fyl2xEmitter for Assembler<'a> {
    fn fyl2x(&mut self) {
        self.emit(FYL2X, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `FYL2XP1`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait Fyl2xp1Emitter {
    fn fyl2xp1(&mut self);
}

impl<'a> Fyl2xp1Emitter for Assembler<'a> {
    fn fyl2xp1(&mut self) {
        self.emit(FYL2XP1, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `F2XM1`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn f2xm1(&mut self)
    where
        Assembler<'a>: F2xm1Emitter,
    {
        <Self as F2xm1Emitter>::f2xm1(self);
    }
    /// `FABS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fabs(&mut self)
    where
        Assembler<'a>: FabsEmitter,
    {
        <Self as FabsEmitter>::fabs(self);
    }
    /// `FADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fadd_1<A>(&mut self, op0: A)
    where
        Assembler<'a>: FaddEmitter_1<A>,
    {
        <Self as FaddEmitter_1<A>>::fadd_1(self, op0);
    }
    /// `FADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fadd_2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FaddEmitter_2<A, B>,
    {
        <Self as FaddEmitter_2<A, B>>::fadd_2(self, op0, op1);
    }
    /// `FADDP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn faddp<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FaddpEmitter<A, B>,
    {
        <Self as FaddpEmitter<A, B>>::faddp(self, op0, op1);
    }
    /// `FBLD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fbld<A>(&mut self, op0: A)
    where
        Assembler<'a>: FbldEmitter<A>,
    {
        <Self as FbldEmitter<A>>::fbld(self, op0);
    }
    /// `FBSTP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fbstp<A>(&mut self, op0: A)
    where
        Assembler<'a>: FbstpEmitter<A>,
    {
        <Self as FbstpEmitter<A>>::fbstp(self, op0);
    }
    /// `FCHS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fchs(&mut self)
    where
        Assembler<'a>: FchsEmitter,
    {
        <Self as FchsEmitter>::fchs(self);
    }
    /// `FCLEX`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fclex(&mut self)
    where
        Assembler<'a>: FclexEmitter,
    {
        <Self as FclexEmitter>::fclex(self);
    }
    /// `FCOM`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcom_1<A>(&mut self, op0: A)
    where
        Assembler<'a>: FcomEmitter_1<A>,
    {
        <Self as FcomEmitter_1<A>>::fcom_1(self, op0);
    }
    /// `FCOM`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcom_2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FcomEmitter_2<A, B>,
    {
        <Self as FcomEmitter_2<A, B>>::fcom_2(self, op0, op1);
    }
    /// `FCOMP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcomp_1<A>(&mut self, op0: A)
    where
        Assembler<'a>: FcompEmitter_1<A>,
    {
        <Self as FcompEmitter_1<A>>::fcomp_1(self, op0);
    }
    /// `FCOMP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcomp_2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FcompEmitter_2<A, B>,
    {
        <Self as FcompEmitter_2<A, B>>::fcomp_2(self, op0, op1);
    }
    /// `FCOMPP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcompp(&mut self)
    where
        Assembler<'a>: FcomppEmitter,
    {
        <Self as FcomppEmitter>::fcompp(self);
    }
    /// `FCOS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcos(&mut self)
    where
        Assembler<'a>: FcosEmitter,
    {
        <Self as FcosEmitter>::fcos(self);
    }
    /// `FDECSTP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fdecstp(&mut self)
    where
        Assembler<'a>: FdecstpEmitter,
    {
        <Self as FdecstpEmitter>::fdecstp(self);
    }
    /// `FDIV`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fdiv_1<A>(&mut self, op0: A)
    where
        Assembler<'a>: FdivEmitter_1<A>,
    {
        <Self as FdivEmitter_1<A>>::fdiv_1(self, op0);
    }
    /// `FDIV`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fdiv_2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FdivEmitter_2<A, B>,
    {
        <Self as FdivEmitter_2<A, B>>::fdiv_2(self, op0, op1);
    }
    /// `FDIVP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fdivp<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FdivpEmitter<A, B>,
    {
        <Self as FdivpEmitter<A, B>>::fdivp(self, op0, op1);
    }
    /// `FDIVR`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fdivr_1<A>(&mut self, op0: A)
    where
        Assembler<'a>: FdivrEmitter_1<A>,
    {
        <Self as FdivrEmitter_1<A>>::fdivr_1(self, op0);
    }
    /// `FDIVR`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fdivr_2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FdivrEmitter_2<A, B>,
    {
        <Self as FdivrEmitter_2<A, B>>::fdivr_2(self, op0, op1);
    }
    /// `FDIVRP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fdivrp<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FdivrpEmitter<A, B>,
    {
        <Self as FdivrpEmitter<A, B>>::fdivrp(self, op0, op1);
    }
    /// `FFREE`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn ffree<A>(&mut self, op0: A)
    where
        Assembler<'a>: FfreeEmitter<A>,
    {
        <Self as FfreeEmitter<A>>::ffree(self, op0);
    }
    /// `FIADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fiadd<A>(&mut self, op0: A)
    where
        Assembler<'a>: FiaddEmitter<A>,
    {
        <Self as FiaddEmitter<A>>::fiadd(self, op0);
    }
    /// `FICOM`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn ficom<A>(&mut self, op0: A)
    where
        Assembler<'a>: FicomEmitter<A>,
    {
        <Self as FicomEmitter<A>>::ficom(self, op0);
    }
    /// `FICOMP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn ficomp<A>(&mut self, op0: A)
    where
        Assembler<'a>: FicompEmitter<A>,
    {
        <Self as FicompEmitter<A>>::ficomp(self, op0);
    }
    /// `FIDIV`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fidiv<A>(&mut self, op0: A)
    where
        Assembler<'a>: FidivEmitter<A>,
    {
        <Self as FidivEmitter<A>>::fidiv(self, op0);
    }
    /// `FIDIVR`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fidivr<A>(&mut self, op0: A)
    where
        Assembler<'a>: FidivrEmitter<A>,
    {
        <Self as FidivrEmitter<A>>::fidivr(self, op0);
    }
    /// `FILD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fild<A>(&mut self, op0: A)
    where
        Assembler<'a>: FildEmitter<A>,
    {
        <Self as FildEmitter<A>>::fild(self, op0);
    }
    /// `FIMUL`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fimul<A>(&mut self, op0: A)
    where
        Assembler<'a>: FimulEmitter<A>,
    {
        <Self as FimulEmitter<A>>::fimul(self, op0);
    }
    /// `FINCSTP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fincstp(&mut self)
    where
        Assembler<'a>: FincstpEmitter,
    {
        <Self as FincstpEmitter>::fincstp(self);
    }
    /// `FINIT`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn finit(&mut self)
    where
        Assembler<'a>: FinitEmitter,
    {
        <Self as FinitEmitter>::finit(self);
    }
    /// `FIST`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fist<A>(&mut self, op0: A)
    where
        Assembler<'a>: FistEmitter<A>,
    {
        <Self as FistEmitter<A>>::fist(self, op0);
    }
    /// `FISTP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fistp<A>(&mut self, op0: A)
    where
        Assembler<'a>: FistpEmitter<A>,
    {
        <Self as FistpEmitter<A>>::fistp(self, op0);
    }
    /// `FISUB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fisub<A>(&mut self, op0: A)
    where
        Assembler<'a>: FisubEmitter<A>,
    {
        <Self as FisubEmitter<A>>::fisub(self, op0);
    }
    /// `FISUBR`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fisubr<A>(&mut self, op0: A)
    where
        Assembler<'a>: FisubrEmitter<A>,
    {
        <Self as FisubrEmitter<A>>::fisubr(self, op0);
    }
    /// `FLD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// | 2 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fld<A>(&mut self, op0: A)
    where
        Assembler<'a>: FldEmitter<A>,
    {
        <Self as FldEmitter<A>>::fld(self, op0);
    }
    /// `FLD1`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fld1(&mut self)
    where
        Assembler<'a>: Fld1Emitter,
    {
        <Self as Fld1Emitter>::fld1(self);
    }
    /// `FLDCW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fldcw<A>(&mut self, op0: A)
    where
        Assembler<'a>: FldcwEmitter<A>,
    {
        <Self as FldcwEmitter<A>>::fldcw(self, op0);
    }
    /// `FLDENV`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fldenv<A>(&mut self, op0: A)
    where
        Assembler<'a>: FldenvEmitter<A>,
    {
        <Self as FldenvEmitter<A>>::fldenv(self, op0);
    }
    /// `FLDL2E`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fldl2e(&mut self)
    where
        Assembler<'a>: Fldl2eEmitter,
    {
        <Self as Fldl2eEmitter>::fldl2e(self);
    }
    /// `FLDL2T`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fldl2t(&mut self)
    where
        Assembler<'a>: Fldl2tEmitter,
    {
        <Self as Fldl2tEmitter>::fldl2t(self);
    }
    /// `FLDLG2`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fldlg2(&mut self)
    where
        Assembler<'a>: Fldlg2Emitter,
    {
        <Self as Fldlg2Emitter>::fldlg2(self);
    }
    /// `FLDLN2`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fldln2(&mut self)
    where
        Assembler<'a>: Fldln2Emitter,
    {
        <Self as Fldln2Emitter>::fldln2(self);
    }
    /// `FLDPI`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fldpi(&mut self)
    where
        Assembler<'a>: FldpiEmitter,
    {
        <Self as FldpiEmitter>::fldpi(self);
    }
    /// `FLDZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fldz(&mut self)
    where
        Assembler<'a>: FldzEmitter,
    {
        <Self as FldzEmitter>::fldz(self);
    }
    /// `FMUL`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fmul_1<A>(&mut self, op0: A)
    where
        Assembler<'a>: FmulEmitter_1<A>,
    {
        <Self as FmulEmitter_1<A>>::fmul_1(self, op0);
    }
    /// `FMUL`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fmul_2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FmulEmitter_2<A, B>,
    {
        <Self as FmulEmitter_2<A, B>>::fmul_2(self, op0, op1);
    }
    /// `FMULP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fmulp<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FmulpEmitter<A, B>,
    {
        <Self as FmulpEmitter<A, B>>::fmulp(self, op0, op1);
    }
    /// `FNOP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fnop(&mut self)
    where
        Assembler<'a>: FnopEmitter,
    {
        <Self as FnopEmitter>::fnop(self);
    }
    /// `FPATAN`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fpatan(&mut self)
    where
        Assembler<'a>: FpatanEmitter,
    {
        <Self as FpatanEmitter>::fpatan(self);
    }
    /// `FPREM`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fprem(&mut self)
    where
        Assembler<'a>: FpremEmitter,
    {
        <Self as FpremEmitter>::fprem(self);
    }
    /// `FPREM1`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fprem1(&mut self)
    where
        Assembler<'a>: Fprem1Emitter,
    {
        <Self as Fprem1Emitter>::fprem1(self);
    }
    /// `FPTAN`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fptan(&mut self)
    where
        Assembler<'a>: FptanEmitter,
    {
        <Self as FptanEmitter>::fptan(self);
    }
    /// `FRNDINT`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn frndint(&mut self)
    where
        Assembler<'a>: FrndintEmitter,
    {
        <Self as FrndintEmitter>::frndint(self);
    }
    /// `FRSTOR`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn frstor<A>(&mut self, op0: A)
    where
        Assembler<'a>: FrstorEmitter<A>,
    {
        <Self as FrstorEmitter<A>>::frstor(self, op0);
    }
    /// `FSAVE`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fsave<A>(&mut self, op0: A)
    where
        Assembler<'a>: FsaveEmitter<A>,
    {
        <Self as FsaveEmitter<A>>::fsave(self, op0);
    }
    /// `FSCALE`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fscale(&mut self)
    where
        Assembler<'a>: FscaleEmitter,
    {
        <Self as FscaleEmitter>::fscale(self);
    }
    /// `FSIN`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fsin(&mut self)
    where
        Assembler<'a>: FsinEmitter,
    {
        <Self as FsinEmitter>::fsin(self);
    }
    /// `FSINCOS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fsincos(&mut self)
    where
        Assembler<'a>: FsincosEmitter,
    {
        <Self as FsincosEmitter>::fsincos(self);
    }
    /// `FSQRT`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fsqrt(&mut self)
    where
        Assembler<'a>: FsqrtEmitter,
    {
        <Self as FsqrtEmitter>::fsqrt(self);
    }
    /// `FST`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// | 2 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fst<A>(&mut self, op0: A)
    where
        Assembler<'a>: FstEmitter<A>,
    {
        <Self as FstEmitter<A>>::fst(self, op0);
    }
    /// `FSTCW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fstcw<A>(&mut self, op0: A)
    where
        Assembler<'a>: FstcwEmitter<A>,
    {
        <Self as FstcwEmitter<A>>::fstcw(self, op0);
    }
    /// `FSTENV`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fstenv<A>(&mut self, op0: A)
    where
        Assembler<'a>: FstenvEmitter<A>,
    {
        <Self as FstenvEmitter<A>>::fstenv(self, op0);
    }
    /// `FSTP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// | 2 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fstp<A>(&mut self, op0: A)
    where
        Assembler<'a>: FstpEmitter<A>,
    {
        <Self as FstpEmitter<A>>::fstp(self, op0);
    }
    /// `FSTSW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fstsw<A>(&mut self, op0: A)
    where
        Assembler<'a>: FstswEmitter<A>,
    {
        <Self as FstswEmitter<A>>::fstsw(self, op0);
    }
    /// `FSUB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fsub_1<A>(&mut self, op0: A)
    where
        Assembler<'a>: FsubEmitter_1<A>,
    {
        <Self as FsubEmitter_1<A>>::fsub_1(self, op0);
    }
    /// `FSUB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fsub_2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FsubEmitter_2<A, B>,
    {
        <Self as FsubEmitter_2<A, B>>::fsub_2(self, op0, op1);
    }
    /// `FSUBP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fsubp<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FsubpEmitter<A, B>,
    {
        <Self as FsubpEmitter<A, B>>::fsubp(self, op0, op1);
    }
    /// `FSUBR`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fsubr_1<A>(&mut self, op0: A)
    where
        Assembler<'a>: FsubrEmitter_1<A>,
    {
        <Self as FsubrEmitter_1<A>>::fsubr_1(self, op0);
    }
    /// `FSUBR`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fsubr_2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FsubrEmitter_2<A, B>,
    {
        <Self as FsubrEmitter_2<A, B>>::fsubr_2(self, op0, op1);
    }
    /// `FSUBRP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fsubrp<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FsubrpEmitter<A, B>,
    {
        <Self as FsubrpEmitter<A, B>>::fsubrp(self, op0, op1);
    }
    /// `FTST`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn ftst(&mut self)
    where
        Assembler<'a>: FtstEmitter,
    {
        <Self as FtstEmitter>::ftst(self);
    }
    /// `FUCOM`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fucom<A>(&mut self, op0: A)
    where
        Assembler<'a>: FucomEmitter<A>,
    {
        <Self as FucomEmitter<A>>::fucom(self, op0);
    }
    /// `FUCOMP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fucomp<A>(&mut self, op0: A)
    where
        Assembler<'a>: FucompEmitter<A>,
    {
        <Self as FucompEmitter<A>>::fucomp(self, op0);
    }
    /// `FUCOMPP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fucompp(&mut self)
    where
        Assembler<'a>: FucomppEmitter,
    {
        <Self as FucomppEmitter>::fucompp(self);
    }
    /// `FXAM`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fxam(&mut self)
    where
        Assembler<'a>: FxamEmitter,
    {
        <Self as FxamEmitter>::fxam(self);
    }
    /// `FXCH`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fxch<A>(&mut self, op0: A)
    where
        Assembler<'a>: FxchEmitter<A>,
    {
        <Self as FxchEmitter<A>>::fxch(self, op0);
    }
    /// `FXTRACT`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fxtract(&mut self)
    where
        Assembler<'a>: FxtractEmitter,
    {
        <Self as FxtractEmitter>::fxtract(self);
    }
    /// `FYL2X`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fyl2x(&mut self)
    where
        Assembler<'a>: Fyl2xEmitter,
    {
        <Self as Fyl2xEmitter>::fyl2x(self);
    }
    /// `FYL2XP1`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fyl2xp1(&mut self)
    where
        Assembler<'a>: Fyl2xp1Emitter,
    {
        <Self as Fyl2xp1Emitter>::fyl2xp1(self);
    }
}
