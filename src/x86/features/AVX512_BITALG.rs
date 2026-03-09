use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VPOPCNTB`.
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
pub trait VpopcntbEmitter<A, B> {
    fn vpopcntb(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPOPCNTB128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPOPCNTB128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntb(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPOPCNTB256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntb(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPOPCNTB256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntb(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPOPCNTB512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntb(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPOPCNTB512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPOPCNTB_MASK`.
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
pub trait VpopcntbMaskEmitter<A, B> {
    fn vpopcntb_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntbMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntb_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPOPCNTB128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntb_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPOPCNTB128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntb_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPOPCNTB256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntb_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPOPCNTB256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntb_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPOPCNTB512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntb_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPOPCNTB512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPOPCNTB_MASKZ`.
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
pub trait VpopcntbMaskzEmitter<A, B> {
    fn vpopcntb_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntbMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntb_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPOPCNTB128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntb_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPOPCNTB128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntb_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPOPCNTB256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntb_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPOPCNTB256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntb_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPOPCNTB512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntbMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntb_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPOPCNTB512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPOPCNTW`.
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
pub trait VpopcntwEmitter<A, B> {
    fn vpopcntw(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPOPCNTW128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPOPCNTW128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntw(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPOPCNTW256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntw(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPOPCNTW256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntw(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPOPCNTW512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntw(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPOPCNTW512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPOPCNTW_MASK`.
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
pub trait VpopcntwMaskEmitter<A, B> {
    fn vpopcntw_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntwMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntw_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPOPCNTW128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntw_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPOPCNTW128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntw_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPOPCNTW256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntw_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPOPCNTW256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntw_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPOPCNTW512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntw_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPOPCNTW512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPOPCNTW_MASKZ`.
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
pub trait VpopcntwMaskzEmitter<A, B> {
    fn vpopcntw_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntwMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntw_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPOPCNTW128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntw_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPOPCNTW128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntw_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPOPCNTW256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntw_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPOPCNTW256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntw_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPOPCNTW512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpopcntwMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntw_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPOPCNTW512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPSHUFBITQMB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Xmm, Mem |
/// | 2 | KReg, Xmm, Xmm |
/// | 3 | KReg, Ymm, Mem |
/// | 4 | KReg, Ymm, Ymm |
/// | 5 | KReg, Zmm, Mem |
/// | 6 | KReg, Zmm, Zmm |
/// +---+----------------+
/// ```
pub trait VpshufbitqmbEmitter<A, B, C> {
    fn vpshufbitqmb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshufbitqmbEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vpshufbitqmb(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHUFBITQMB128KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbitqmbEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vpshufbitqmb(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHUFBITQMB128KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbitqmbEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vpshufbitqmb(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHUFBITQMB256KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbitqmbEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vpshufbitqmb(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHUFBITQMB256KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbitqmbEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vpshufbitqmb(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHUFBITQMB512KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbitqmbEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vpshufbitqmb(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHUFBITQMB512KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHUFBITQMB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Xmm, Mem |
/// | 2 | KReg, Xmm, Xmm |
/// | 3 | KReg, Ymm, Mem |
/// | 4 | KReg, Ymm, Ymm |
/// | 5 | KReg, Zmm, Mem |
/// | 6 | KReg, Zmm, Zmm |
/// +---+----------------+
/// ```
pub trait VpshufbitqmbMaskEmitter<A, B, C> {
    fn vpshufbitqmb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshufbitqmbMaskEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vpshufbitqmb_mask(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHUFBITQMB128KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbitqmbMaskEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vpshufbitqmb_mask(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHUFBITQMB128KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbitqmbMaskEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vpshufbitqmb_mask(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHUFBITQMB256KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbitqmbMaskEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vpshufbitqmb_mask(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHUFBITQMB256KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbitqmbMaskEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vpshufbitqmb_mask(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHUFBITQMB512KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbitqmbMaskEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vpshufbitqmb_mask(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHUFBITQMB512KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `VPOPCNTB`.
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
    pub fn vpopcntb<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpopcntbEmitter<A, B>,
    {
        <Self as VpopcntbEmitter<A, B>>::vpopcntb(self, op0, op1);
    }
    /// `VPOPCNTB_MASK`.
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
    pub fn vpopcntb_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpopcntbMaskEmitter<A, B>,
    {
        <Self as VpopcntbMaskEmitter<A, B>>::vpopcntb_mask(self, op0, op1);
    }
    /// `VPOPCNTB_MASKZ`.
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
    pub fn vpopcntb_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpopcntbMaskzEmitter<A, B>,
    {
        <Self as VpopcntbMaskzEmitter<A, B>>::vpopcntb_maskz(self, op0, op1);
    }
    /// `VPOPCNTW`.
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
    pub fn vpopcntw<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpopcntwEmitter<A, B>,
    {
        <Self as VpopcntwEmitter<A, B>>::vpopcntw(self, op0, op1);
    }
    /// `VPOPCNTW_MASK`.
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
    pub fn vpopcntw_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpopcntwMaskEmitter<A, B>,
    {
        <Self as VpopcntwMaskEmitter<A, B>>::vpopcntw_mask(self, op0, op1);
    }
    /// `VPOPCNTW_MASKZ`.
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
    pub fn vpopcntw_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpopcntwMaskzEmitter<A, B>,
    {
        <Self as VpopcntwMaskzEmitter<A, B>>::vpopcntw_maskz(self, op0, op1);
    }
    /// `VPSHUFBITQMB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Xmm, Mem |
    /// | 2 | KReg, Xmm, Xmm |
    /// | 3 | KReg, Ymm, Mem |
    /// | 4 | KReg, Ymm, Ymm |
    /// | 5 | KReg, Zmm, Mem |
    /// | 6 | KReg, Zmm, Zmm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vpshufbitqmb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshufbitqmbEmitter<A, B, C>,
    {
        <Self as VpshufbitqmbEmitter<A, B, C>>::vpshufbitqmb(self, op0, op1, op2);
    }
    /// `VPSHUFBITQMB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Xmm, Mem |
    /// | 2 | KReg, Xmm, Xmm |
    /// | 3 | KReg, Ymm, Mem |
    /// | 4 | KReg, Ymm, Ymm |
    /// | 5 | KReg, Zmm, Mem |
    /// | 6 | KReg, Zmm, Zmm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vpshufbitqmb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshufbitqmbMaskEmitter<A, B, C>,
    {
        <Self as VpshufbitqmbMaskEmitter<A, B, C>>::vpshufbitqmb_mask(self, op0, op1, op2);
    }
}
