use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `BSWAP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// | 3 | Gpw      |
/// +---+----------+
/// ```
pub trait BswapEmitter<A> {
    fn bswap(&mut self, op0: A);
}

impl<'a> BswapEmitter<Gpw> for Assembler<'a> {
    fn bswap(&mut self, op0: Gpw) {
        self.emit(BSWAP16R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> BswapEmitter<Gpd> for Assembler<'a> {
    fn bswap(&mut self, op0: Gpd) {
        self.emit(BSWAP32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> BswapEmitter<Gpq> for Assembler<'a> {
    fn bswap(&mut self, op0: Gpq) {
        self.emit(BSWAP64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `CMPXCHG`.
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------+
/// | # | Operands     |
/// +---+--------------+
/// | 1 | GpbLo, GpbLo |
/// | 2 | Gpd, Gpd     |
/// | 3 | Gpq, Gpq     |
/// | 4 | Gpw, Gpw     |
/// | 5 | Mem, GpbLo   |
/// | 6 | Mem, Gpd     |
/// | 7 | Mem, Gpq     |
/// | 8 | Mem, Gpw     |
/// +---+--------------+
/// ```
pub trait CmpxchgEmitter<A, B> {
    fn cmpxchg(&mut self, op0: A, op1: B);
}

impl<'a> CmpxchgEmitter<GpbLo, GpbLo> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: GpbLo, op1: GpbLo) {
        self.emit(
            CMPXCHG8RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmpxchgEmitter<Mem, GpbLo> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Mem, op1: GpbLo) {
        self.emit(
            CMPXCHG8MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmpxchgEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMPXCHG16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmpxchgEmitter<Mem, Gpw> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Mem, op1: Gpw) {
        self.emit(
            CMPXCHG16MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmpxchgEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMPXCHG32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmpxchgEmitter<Mem, Gpd> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Mem, op1: Gpd) {
        self.emit(
            CMPXCHG32MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmpxchgEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMPXCHG64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmpxchgEmitter<Mem, Gpq> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Mem, op1: Gpq) {
        self.emit(
            CMPXCHG64MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `INVD`.
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
pub trait InvdEmitter {
    fn invd(&mut self);
}

impl<'a> InvdEmitter for Assembler<'a> {
    fn invd(&mut self) {
        self.emit(INVD, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `INVLPG`.
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
pub trait InvlpgEmitter<A> {
    fn invlpg(&mut self, op0: A);
}

impl<'a> InvlpgEmitter<Mem> for Assembler<'a> {
    fn invlpg(&mut self, op0: Mem) {
        self.emit(INVLPG8M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `WBINVD`.
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
pub trait WbinvdEmitter {
    fn wbinvd(&mut self);
}

impl<'a> WbinvdEmitter for Assembler<'a> {
    fn wbinvd(&mut self) {
        self.emit(WBINVD, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `XADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------+
/// | # | Operands     |
/// +---+--------------+
/// | 1 | GpbLo, GpbLo |
/// | 2 | Gpd, Gpd     |
/// | 3 | Gpq, Gpq     |
/// | 4 | Gpw, Gpw     |
/// | 5 | Mem, GpbLo   |
/// | 6 | Mem, Gpd     |
/// | 7 | Mem, Gpq     |
/// | 8 | Mem, Gpw     |
/// +---+--------------+
/// ```
pub trait XaddEmitter<A, B> {
    fn xadd(&mut self, op0: A, op1: B);
}

impl<'a> XaddEmitter<GpbLo, GpbLo> for Assembler<'a> {
    fn xadd(&mut self, op0: GpbLo, op1: GpbLo) {
        self.emit(XADD8RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Mem, GpbLo> for Assembler<'a> {
    fn xadd(&mut self, op0: Mem, op1: GpbLo) {
        self.emit(XADD8MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Gpw, Gpw> for Assembler<'a> {
    fn xadd(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(XADD16RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Mem, Gpw> for Assembler<'a> {
    fn xadd(&mut self, op0: Mem, op1: Gpw) {
        self.emit(XADD16MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Gpd, Gpd> for Assembler<'a> {
    fn xadd(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(XADD32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Mem, Gpd> for Assembler<'a> {
    fn xadd(&mut self, op0: Mem, op1: Gpd) {
        self.emit(XADD32MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Gpq, Gpq> for Assembler<'a> {
    fn xadd(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(XADD64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Mem, Gpq> for Assembler<'a> {
    fn xadd(&mut self, op0: Mem, op1: Gpq) {
        self.emit(XADD64MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `BSWAP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// | 3 | Gpw      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn bswap<A>(&mut self, op0: A)
    where
        Assembler<'a>: BswapEmitter<A>,
    {
        <Self as BswapEmitter<A>>::bswap(self, op0);
    }
    /// `CMPXCHG`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------+
    /// | # | Operands     |
    /// +---+--------------+
    /// | 1 | GpbLo, GpbLo |
    /// | 2 | Gpd, Gpd     |
    /// | 3 | Gpq, Gpq     |
    /// | 4 | Gpw, Gpw     |
    /// | 5 | Mem, GpbLo   |
    /// | 6 | Mem, Gpd     |
    /// | 7 | Mem, Gpq     |
    /// | 8 | Mem, Gpw     |
    /// +---+--------------+
    /// ```
    #[inline]
    pub fn cmpxchg<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmpxchgEmitter<A, B>,
    {
        <Self as CmpxchgEmitter<A, B>>::cmpxchg(self, op0, op1);
    }
    /// `INVD`.
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
    pub fn invd(&mut self)
    where
        Assembler<'a>: InvdEmitter,
    {
        <Self as InvdEmitter>::invd(self);
    }
    /// `INVLPG`.
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
    pub fn invlpg<A>(&mut self, op0: A)
    where
        Assembler<'a>: InvlpgEmitter<A>,
    {
        <Self as InvlpgEmitter<A>>::invlpg(self, op0);
    }
    /// `WBINVD`.
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
    pub fn wbinvd(&mut self)
    where
        Assembler<'a>: WbinvdEmitter,
    {
        <Self as WbinvdEmitter>::wbinvd(self);
    }
    /// `XADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------+
    /// | # | Operands     |
    /// +---+--------------+
    /// | 1 | GpbLo, GpbLo |
    /// | 2 | Gpd, Gpd     |
    /// | 3 | Gpq, Gpq     |
    /// | 4 | Gpw, Gpw     |
    /// | 5 | Mem, GpbLo   |
    /// | 6 | Mem, Gpd     |
    /// | 7 | Mem, Gpq     |
    /// | 8 | Mem, Gpw     |
    /// +---+--------------+
    /// ```
    #[inline]
    pub fn xadd<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: XaddEmitter<A, B>,
    {
        <Self as XaddEmitter<A, B>>::xadd(self, op0, op1);
    }
}
