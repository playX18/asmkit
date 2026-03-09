use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VPMADD52HUQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpmadd52huqEmitter<A, B, C> {
    fn vpmadd52huq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpmadd52huqEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmadd52huq(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADD52HUQ128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmadd52huq(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADD52HUQ128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmadd52huq(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADD52HUQ256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmadd52huq(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADD52HUQ256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmadd52huq(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADD52HUQ512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmadd52huq(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADD52HUQ512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMADD52HUQ_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpmadd52huqMaskEmitter<A, B, C> {
    fn vpmadd52huq_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpmadd52huqMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmadd52huq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADD52HUQ128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmadd52huq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADD52HUQ128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmadd52huq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADD52HUQ256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmadd52huq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADD52HUQ256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmadd52huq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADD52HUQ512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmadd52huq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADD52HUQ512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMADD52HUQ_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpmadd52huqMaskzEmitter<A, B, C> {
    fn vpmadd52huq_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpmadd52huqMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmadd52huq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADD52HUQ128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmadd52huq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADD52HUQ128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmadd52huq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADD52HUQ256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmadd52huq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADD52HUQ256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmadd52huq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADD52HUQ512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52huqMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmadd52huq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADD52HUQ512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMADD52LUQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpmadd52luqEmitter<A, B, C> {
    fn vpmadd52luq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpmadd52luqEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmadd52luq(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADD52LUQ128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmadd52luq(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADD52LUQ128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmadd52luq(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADD52LUQ256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmadd52luq(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADD52LUQ256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmadd52luq(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADD52LUQ512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmadd52luq(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADD52LUQ512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMADD52LUQ_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpmadd52luqMaskEmitter<A, B, C> {
    fn vpmadd52luq_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpmadd52luqMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmadd52luq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADD52LUQ128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmadd52luq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADD52LUQ128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmadd52luq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADD52LUQ256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmadd52luq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADD52LUQ256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmadd52luq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADD52LUQ512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmadd52luq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADD52LUQ512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMADD52LUQ_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpmadd52luqMaskzEmitter<A, B, C> {
    fn vpmadd52luq_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpmadd52luqMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmadd52luq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADD52LUQ128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmadd52luq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADD52LUQ128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmadd52luq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADD52LUQ256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmadd52luq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADD52LUQ256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmadd52luq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADD52LUQ512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpmadd52luqMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmadd52luq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADD52LUQ512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `VPMADD52HUQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmadd52huq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpmadd52huqEmitter<A, B, C>,
    {
        <Self as Vpmadd52huqEmitter<A, B, C>>::vpmadd52huq(self, op0, op1, op2);
    }
    /// `VPMADD52HUQ_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmadd52huq_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpmadd52huqMaskEmitter<A, B, C>,
    {
        <Self as Vpmadd52huqMaskEmitter<A, B, C>>::vpmadd52huq_mask(self, op0, op1, op2);
    }
    /// `VPMADD52HUQ_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmadd52huq_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpmadd52huqMaskzEmitter<A, B, C>,
    {
        <Self as Vpmadd52huqMaskzEmitter<A, B, C>>::vpmadd52huq_maskz(self, op0, op1, op2);
    }
    /// `VPMADD52LUQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmadd52luq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpmadd52luqEmitter<A, B, C>,
    {
        <Self as Vpmadd52luqEmitter<A, B, C>>::vpmadd52luq(self, op0, op1, op2);
    }
    /// `VPMADD52LUQ_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmadd52luq_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpmadd52luqMaskEmitter<A, B, C>,
    {
        <Self as Vpmadd52luqMaskEmitter<A, B, C>>::vpmadd52luq_mask(self, op0, op1, op2);
    }
    /// `VPMADD52LUQ_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmadd52luq_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpmadd52luqMaskzEmitter<A, B, C>,
    {
        <Self as Vpmadd52luqMaskzEmitter<A, B, C>>::vpmadd52luq_maskz(self, op0, op1, op2);
    }
}
