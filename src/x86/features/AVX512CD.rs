use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VPBROADCASTMB2Q`.
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------+
/// | # | Operands  |
/// +---+-----------+
/// | 1 | Xmm, KReg |
/// | 2 | Ymm, KReg |
/// | 3 | Zmm, KReg |
/// +---+-----------+
/// ```
pub trait Vpbroadcastmb2qEmitter<A, B> {
    fn vpbroadcastmb2q(&mut self, op0: A, op1: B);
}

impl<'a> Vpbroadcastmb2qEmitter<Xmm, KReg> for Assembler<'a> {
    fn vpbroadcastmb2q(&mut self, op0: Xmm, op1: KReg) {
        self.emit(
            VPBROADCASTMB2Q128RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpbroadcastmb2qEmitter<Ymm, KReg> for Assembler<'a> {
    fn vpbroadcastmb2q(&mut self, op0: Ymm, op1: KReg) {
        self.emit(
            VPBROADCASTMB2Q256RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpbroadcastmb2qEmitter<Zmm, KReg> for Assembler<'a> {
    fn vpbroadcastmb2q(&mut self, op0: Zmm, op1: KReg) {
        self.emit(
            VPBROADCASTMB2Q512RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTMW2D`.
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------+
/// | # | Operands  |
/// +---+-----------+
/// | 1 | Xmm, KReg |
/// | 2 | Ymm, KReg |
/// | 3 | Zmm, KReg |
/// +---+-----------+
/// ```
pub trait Vpbroadcastmw2dEmitter<A, B> {
    fn vpbroadcastmw2d(&mut self, op0: A, op1: B);
}

impl<'a> Vpbroadcastmw2dEmitter<Xmm, KReg> for Assembler<'a> {
    fn vpbroadcastmw2d(&mut self, op0: Xmm, op1: KReg) {
        self.emit(
            VPBROADCASTMW2D128RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpbroadcastmw2dEmitter<Ymm, KReg> for Assembler<'a> {
    fn vpbroadcastmw2d(&mut self, op0: Ymm, op1: KReg) {
        self.emit(
            VPBROADCASTMW2D256RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpbroadcastmw2dEmitter<Zmm, KReg> for Assembler<'a> {
    fn vpbroadcastmw2d(&mut self, op0: Zmm, op1: KReg) {
        self.emit(
            VPBROADCASTMW2D512RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCONFLICTD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpconflictdEmitter<A, B> {
    fn vpconflictd(&mut self, op0: A, op1: B);
}

impl<'a> VpconflictdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpconflictd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCONFLICTD128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpconflictd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPCONFLICTD128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpconflictd(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCONFLICTD256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpconflictd(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPCONFLICTD256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpconflictd(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCONFLICTD512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpconflictd(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPCONFLICTD512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCONFLICTD_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpconflictdMaskEmitter<A, B> {
    fn vpconflictd_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpconflictdMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpconflictd_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCONFLICTD128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpconflictd_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPCONFLICTD128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpconflictd_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCONFLICTD256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpconflictd_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPCONFLICTD256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpconflictd_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCONFLICTD512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpconflictd_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPCONFLICTD512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCONFLICTD_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpconflictdMaskzEmitter<A, B> {
    fn vpconflictd_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpconflictdMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpconflictd_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCONFLICTD128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpconflictd_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPCONFLICTD128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpconflictd_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCONFLICTD256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpconflictd_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPCONFLICTD256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpconflictd_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCONFLICTD512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictdMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpconflictd_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPCONFLICTD512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCONFLICTQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpconflictqEmitter<A, B> {
    fn vpconflictq(&mut self, op0: A, op1: B);
}

impl<'a> VpconflictqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpconflictq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCONFLICTQ128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpconflictq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPCONFLICTQ128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpconflictq(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCONFLICTQ256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpconflictq(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPCONFLICTQ256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpconflictq(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCONFLICTQ512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpconflictq(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPCONFLICTQ512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCONFLICTQ_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpconflictqMaskEmitter<A, B> {
    fn vpconflictq_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpconflictqMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpconflictq_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCONFLICTQ128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpconflictq_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPCONFLICTQ128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpconflictq_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCONFLICTQ256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpconflictq_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPCONFLICTQ256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpconflictq_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCONFLICTQ512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpconflictq_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPCONFLICTQ512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCONFLICTQ_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpconflictqMaskzEmitter<A, B> {
    fn vpconflictq_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpconflictqMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpconflictq_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCONFLICTQ128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpconflictq_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPCONFLICTQ128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpconflictq_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCONFLICTQ256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpconflictq_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPCONFLICTQ256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpconflictq_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCONFLICTQ512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpconflictqMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpconflictq_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPCONFLICTQ512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPLZCNTD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VplzcntdEmitter<A, B> {
    fn vplzcntd(&mut self, op0: A, op1: B);
}

impl<'a> VplzcntdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vplzcntd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPLZCNTD128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdEmitter<Xmm, Mem> for Assembler<'a> {
    fn vplzcntd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPLZCNTD128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vplzcntd(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPLZCNTD256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdEmitter<Ymm, Mem> for Assembler<'a> {
    fn vplzcntd(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPLZCNTD256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vplzcntd(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPLZCNTD512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdEmitter<Zmm, Mem> for Assembler<'a> {
    fn vplzcntd(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPLZCNTD512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPLZCNTD_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VplzcntdMaskEmitter<A, B> {
    fn vplzcntd_mask(&mut self, op0: A, op1: B);
}

impl<'a> VplzcntdMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vplzcntd_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPLZCNTD128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vplzcntd_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPLZCNTD128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vplzcntd_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPLZCNTD256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vplzcntd_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPLZCNTD256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vplzcntd_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPLZCNTD512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vplzcntd_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPLZCNTD512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPLZCNTD_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VplzcntdMaskzEmitter<A, B> {
    fn vplzcntd_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VplzcntdMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vplzcntd_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPLZCNTD128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vplzcntd_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPLZCNTD128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vplzcntd_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPLZCNTD256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vplzcntd_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPLZCNTD256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vplzcntd_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPLZCNTD512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntdMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vplzcntd_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPLZCNTD512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPLZCNTQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VplzcntqEmitter<A, B> {
    fn vplzcntq(&mut self, op0: A, op1: B);
}

impl<'a> VplzcntqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vplzcntq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPLZCNTQ128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqEmitter<Xmm, Mem> for Assembler<'a> {
    fn vplzcntq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPLZCNTQ128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vplzcntq(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPLZCNTQ256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqEmitter<Ymm, Mem> for Assembler<'a> {
    fn vplzcntq(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPLZCNTQ256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vplzcntq(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPLZCNTQ512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqEmitter<Zmm, Mem> for Assembler<'a> {
    fn vplzcntq(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPLZCNTQ512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPLZCNTQ_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VplzcntqMaskEmitter<A, B> {
    fn vplzcntq_mask(&mut self, op0: A, op1: B);
}

impl<'a> VplzcntqMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vplzcntq_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPLZCNTQ128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vplzcntq_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPLZCNTQ128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vplzcntq_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPLZCNTQ256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vplzcntq_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPLZCNTQ256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vplzcntq_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPLZCNTQ512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vplzcntq_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPLZCNTQ512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPLZCNTQ_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VplzcntqMaskzEmitter<A, B> {
    fn vplzcntq_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VplzcntqMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vplzcntq_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPLZCNTQ128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vplzcntq_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPLZCNTQ128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vplzcntq_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPLZCNTQ256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vplzcntq_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPLZCNTQ256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vplzcntq_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPLZCNTQ512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VplzcntqMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vplzcntq_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPLZCNTQ512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `VPBROADCASTMB2Q`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------+
    /// | # | Operands  |
    /// +---+-----------+
    /// | 1 | Xmm, KReg |
    /// | 2 | Ymm, KReg |
    /// | 3 | Zmm, KReg |
    /// +---+-----------+
    /// ```
    #[inline]
    pub fn vpbroadcastmb2q<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vpbroadcastmb2qEmitter<A, B>,
    {
        <Self as Vpbroadcastmb2qEmitter<A, B>>::vpbroadcastmb2q(self, op0, op1);
    }
    /// `VPBROADCASTMW2D`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------+
    /// | # | Operands  |
    /// +---+-----------+
    /// | 1 | Xmm, KReg |
    /// | 2 | Ymm, KReg |
    /// | 3 | Zmm, KReg |
    /// +---+-----------+
    /// ```
    #[inline]
    pub fn vpbroadcastmw2d<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vpbroadcastmw2dEmitter<A, B>,
    {
        <Self as Vpbroadcastmw2dEmitter<A, B>>::vpbroadcastmw2d(self, op0, op1);
    }
    /// `VPCONFLICTD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpconflictd<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpconflictdEmitter<A, B>,
    {
        <Self as VpconflictdEmitter<A, B>>::vpconflictd(self, op0, op1);
    }
    /// `VPCONFLICTD_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpconflictd_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpconflictdMaskEmitter<A, B>,
    {
        <Self as VpconflictdMaskEmitter<A, B>>::vpconflictd_mask(self, op0, op1);
    }
    /// `VPCONFLICTD_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpconflictd_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpconflictdMaskzEmitter<A, B>,
    {
        <Self as VpconflictdMaskzEmitter<A, B>>::vpconflictd_maskz(self, op0, op1);
    }
    /// `VPCONFLICTQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpconflictq<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpconflictqEmitter<A, B>,
    {
        <Self as VpconflictqEmitter<A, B>>::vpconflictq(self, op0, op1);
    }
    /// `VPCONFLICTQ_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpconflictq_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpconflictqMaskEmitter<A, B>,
    {
        <Self as VpconflictqMaskEmitter<A, B>>::vpconflictq_mask(self, op0, op1);
    }
    /// `VPCONFLICTQ_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpconflictq_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpconflictqMaskzEmitter<A, B>,
    {
        <Self as VpconflictqMaskzEmitter<A, B>>::vpconflictq_maskz(self, op0, op1);
    }
    /// `VPLZCNTD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vplzcntd<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VplzcntdEmitter<A, B>,
    {
        <Self as VplzcntdEmitter<A, B>>::vplzcntd(self, op0, op1);
    }
    /// `VPLZCNTD_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vplzcntd_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VplzcntdMaskEmitter<A, B>,
    {
        <Self as VplzcntdMaskEmitter<A, B>>::vplzcntd_mask(self, op0, op1);
    }
    /// `VPLZCNTD_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vplzcntd_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VplzcntdMaskzEmitter<A, B>,
    {
        <Self as VplzcntdMaskzEmitter<A, B>>::vplzcntd_maskz(self, op0, op1);
    }
    /// `VPLZCNTQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vplzcntq<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VplzcntqEmitter<A, B>,
    {
        <Self as VplzcntqEmitter<A, B>>::vplzcntq(self, op0, op1);
    }
    /// `VPLZCNTQ_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vplzcntq_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VplzcntqMaskEmitter<A, B>,
    {
        <Self as VplzcntqMaskEmitter<A, B>>::vplzcntq_mask(self, op0, op1);
    }
    /// `VPLZCNTQ_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vplzcntq_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VplzcntqMaskzEmitter<A, B>,
    {
        <Self as VplzcntqMaskzEmitter<A, B>>::vplzcntq_maskz(self, op0, op1);
    }
}
