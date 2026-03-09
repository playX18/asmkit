use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VBROADCASTI128`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcasti128Emitter<A, B> {
    fn vbroadcasti128(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti128Emitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcasti128(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VBROADCASTI128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VEXTRACTI128`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Ymm, Imm |
/// | 2 | Xmm, Ymm, Imm |
/// +---+---------------+
/// ```
pub trait Vextracti128Emitter<A, B, C> {
    fn vextracti128(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vextracti128Emitter<Xmm, Ymm, Imm> for Assembler<'a> {
    fn vextracti128(&mut self, op0: Xmm, op1: Ymm, op2: Imm) {
        self.emit(
            VEXTRACTI128RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vextracti128Emitter<Mem, Ymm, Imm> for Assembler<'a> {
    fn vextracti128(&mut self, op0: Mem, op1: Ymm, op2: Imm) {
        self.emit(
            VEXTRACTI128MRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VGATHERDPD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VgatherdpdEmitter_3<A, B, C> {
    fn vgatherdpd_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VgatherdpdEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vgatherdpd_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(
            VGATHERDPD128RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VgatherdpdEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vgatherdpd_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(
            VGATHERDPD256RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VGATHERDPS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VgatherdpsEmitter_3<A, B, C> {
    fn vgatherdps_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VgatherdpsEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vgatherdps_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(
            VGATHERDPS128RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VgatherdpsEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vgatherdps_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(
            VGATHERDPS256RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VGATHERQPD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VgatherqpdEmitter_3<A, B, C> {
    fn vgatherqpd_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VgatherqpdEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vgatherqpd_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(
            VGATHERQPD128RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VgatherqpdEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vgatherqpd_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(
            VGATHERQPD256RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VGATHERQPS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// +---+---------------+
/// ```
pub trait VgatherqpsEmitter_3<A, B, C> {
    fn vgatherqps_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VgatherqpsEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vgatherqps_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(
            VGATHERQPS128RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VINSERTI128`.
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Ymm, Ymm, Mem, Imm |
/// | 2 | Ymm, Ymm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinserti128Emitter<A, B, C, D> {
    fn vinserti128(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinserti128Emitter<Ymm, Ymm, Xmm, Imm> for Assembler<'a> {
    fn vinserti128(&mut self, op0: Ymm, op1: Ymm, op2: Xmm, op3: Imm) {
        self.emit(
            VINSERTI128RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> Vinserti128Emitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vinserti128(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VINSERTI128RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPBLENDD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// | 3 | Ymm, Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Ymm, Imm |
/// +---+--------------------+
/// ```
pub trait VpblenddEmitter<A, B, C, D> {
    fn vpblendd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpblenddEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpblendd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPBLENDD128RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpblenddEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpblendd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPBLENDD128RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpblenddEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpblendd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPBLENDD256RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpblenddEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpblendd(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPBLENDD256RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPERM2I128`.
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Ymm, Ymm, Mem, Imm |
/// | 2 | Ymm, Ymm, Ymm, Imm |
/// +---+--------------------+
/// ```
pub trait Vperm2i128Emitter<A, B, C, D> {
    fn vperm2i128(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vperm2i128Emitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vperm2i128(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPERM2I128_256RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> Vperm2i128Emitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vperm2i128(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPERM2I128_256RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPGATHERDD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VpgatherddEmitter_3<A, B, C> {
    fn vpgatherdd_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpgatherddEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vpgatherdd_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(
            VPGATHERDD128RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpgatherddEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vpgatherdd_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(
            VPGATHERDD256RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPGATHERDQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VpgatherdqEmitter_3<A, B, C> {
    fn vpgatherdq_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpgatherdqEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vpgatherdq_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(
            VPGATHERDQ128RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpgatherdqEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vpgatherdq_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(
            VPGATHERDQ256RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPGATHERQD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// +---+---------------+
/// ```
pub trait VpgatherqdEmitter_3<A, B, C> {
    fn vpgatherqd_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpgatherqdEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vpgatherqd_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(
            VPGATHERQD128RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPGATHERQQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VpgatherqqEmitter_3<A, B, C> {
    fn vpgatherqq_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpgatherqqEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vpgatherqq_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(
            VPGATHERQQ128RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpgatherqqEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vpgatherqq_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(
            VPGATHERQQ256RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMASKMOVD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Xmm, Xmm |
/// | 2 | Mem, Ymm, Ymm |
/// | 3 | Xmm, Xmm, Mem |
/// | 4 | Ymm, Ymm, Mem |
/// +---+---------------+
/// ```
pub trait VpmaskmovdEmitter<A, B, C> {
    fn vpmaskmovd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaskmovdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaskmovd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMASKMOVD128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaskmovdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaskmovd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMASKMOVD256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaskmovdEmitter<Mem, Xmm, Xmm> for Assembler<'a> {
    fn vpmaskmovd(&mut self, op0: Mem, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMASKMOVD128MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaskmovdEmitter<Mem, Ymm, Ymm> for Assembler<'a> {
    fn vpmaskmovd(&mut self, op0: Mem, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMASKMOVD256MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMASKMOVQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Xmm, Xmm |
/// | 2 | Mem, Ymm, Ymm |
/// | 3 | Xmm, Xmm, Mem |
/// | 4 | Ymm, Ymm, Mem |
/// +---+---------------+
/// ```
pub trait VpmaskmovqEmitter<A, B, C> {
    fn vpmaskmovq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaskmovqEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaskmovq(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMASKMOVQ128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaskmovqEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaskmovq(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMASKMOVQ256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaskmovqEmitter<Mem, Xmm, Xmm> for Assembler<'a> {
    fn vpmaskmovq(&mut self, op0: Mem, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMASKMOVQ128MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaskmovqEmitter<Mem, Ymm, Ymm> for Assembler<'a> {
    fn vpmaskmovq(&mut self, op0: Mem, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMASKMOVQ256MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `VBROADCASTI128`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti128<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vbroadcasti128Emitter<A, B>,
    {
        <Self as Vbroadcasti128Emitter<A, B>>::vbroadcasti128(self, op0, op1);
    }
    /// `VEXTRACTI128`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Ymm, Imm |
    /// | 2 | Xmm, Ymm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vextracti128<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vextracti128Emitter<A, B, C>,
    {
        <Self as Vextracti128Emitter<A, B, C>>::vextracti128(self, op0, op1, op2);
    }
    /// `VGATHERDPD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vgatherdpd_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VgatherdpdEmitter_3<A, B, C>,
    {
        <Self as VgatherdpdEmitter_3<A, B, C>>::vgatherdpd_3(self, op0, op1, op2);
    }
    /// `VGATHERDPS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vgatherdps_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VgatherdpsEmitter_3<A, B, C>,
    {
        <Self as VgatherdpsEmitter_3<A, B, C>>::vgatherdps_3(self, op0, op1, op2);
    }
    /// `VGATHERQPD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vgatherqpd_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VgatherqpdEmitter_3<A, B, C>,
    {
        <Self as VgatherqpdEmitter_3<A, B, C>>::vgatherqpd_3(self, op0, op1, op2);
    }
    /// `VGATHERQPS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vgatherqps_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VgatherqpsEmitter_3<A, B, C>,
    {
        <Self as VgatherqpsEmitter_3<A, B, C>>::vgatherqps_3(self, op0, op1, op2);
    }
    /// `VINSERTI128`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Ymm, Ymm, Mem, Imm |
    /// | 2 | Ymm, Ymm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinserti128<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: Vinserti128Emitter<A, B, C, D>,
    {
        <Self as Vinserti128Emitter<A, B, C, D>>::vinserti128(self, op0, op1, op2, op3);
    }
    /// `VPBLENDD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// | 3 | Ymm, Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Ymm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vpblendd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpblenddEmitter<A, B, C, D>,
    {
        <Self as VpblenddEmitter<A, B, C, D>>::vpblendd(self, op0, op1, op2, op3);
    }
    /// `VPERM2I128`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Ymm, Ymm, Mem, Imm |
    /// | 2 | Ymm, Ymm, Ymm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vperm2i128<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: Vperm2i128Emitter<A, B, C, D>,
    {
        <Self as Vperm2i128Emitter<A, B, C, D>>::vperm2i128(self, op0, op1, op2, op3);
    }
    /// `VPGATHERDD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpgatherdd_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpgatherddEmitter_3<A, B, C>,
    {
        <Self as VpgatherddEmitter_3<A, B, C>>::vpgatherdd_3(self, op0, op1, op2);
    }
    /// `VPGATHERDQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpgatherdq_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpgatherdqEmitter_3<A, B, C>,
    {
        <Self as VpgatherdqEmitter_3<A, B, C>>::vpgatherdq_3(self, op0, op1, op2);
    }
    /// `VPGATHERQD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpgatherqd_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpgatherqdEmitter_3<A, B, C>,
    {
        <Self as VpgatherqdEmitter_3<A, B, C>>::vpgatherqd_3(self, op0, op1, op2);
    }
    /// `VPGATHERQQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpgatherqq_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpgatherqqEmitter_3<A, B, C>,
    {
        <Self as VpgatherqqEmitter_3<A, B, C>>::vpgatherqq_3(self, op0, op1, op2);
    }
    /// `VPMASKMOVD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Xmm, Xmm |
    /// | 2 | Mem, Ymm, Ymm |
    /// | 3 | Xmm, Xmm, Mem |
    /// | 4 | Ymm, Ymm, Mem |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmaskmovd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaskmovdEmitter<A, B, C>,
    {
        <Self as VpmaskmovdEmitter<A, B, C>>::vpmaskmovd(self, op0, op1, op2);
    }
    /// `VPMASKMOVQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Xmm, Xmm |
    /// | 2 | Mem, Ymm, Ymm |
    /// | 3 | Xmm, Xmm, Mem |
    /// | 4 | Ymm, Ymm, Mem |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmaskmovq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaskmovqEmitter<A, B, C>,
    {
        <Self as VpmaskmovqEmitter<A, B, C>>::vpmaskmovq(self, op0, op1, op2);
    }
}
