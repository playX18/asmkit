use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `FISTTP`.
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
pub trait FisttpEmitter<A> {
    fn fisttp(&mut self, op0: A);
}

impl<'a> FisttpEmitter<Mem> for Assembler<'a> {
    fn fisttp(&mut self, op0: Mem) {
        self.emit(FISTTPM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `SSE_ADDSUBPD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseAddsubpdEmitter<A, B> {
    fn sse_addsubpd(&mut self, op0: A, op1: B);
}

impl<'a> SseAddsubpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_addsubpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SSE_ADDSUBPDRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SseAddsubpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_addsubpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_ADDSUBPDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_ADDSUBPS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseAddsubpsEmitter<A, B> {
    fn sse_addsubps(&mut self, op0: A, op1: B);
}

impl<'a> SseAddsubpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_addsubps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SSE_ADDSUBPSRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SseAddsubpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_addsubps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_ADDSUBPSRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_HADDPD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseHaddpdEmitter<A, B> {
    fn sse_haddpd(&mut self, op0: A, op1: B);
}

impl<'a> SseHaddpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_haddpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SSE_HADDPDRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SseHaddpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_haddpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_HADDPDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_HADDPS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseHaddpsEmitter<A, B> {
    fn sse_haddps(&mut self, op0: A, op1: B);
}

impl<'a> SseHaddpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_haddps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SSE_HADDPSRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SseHaddpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_haddps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_HADDPSRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_HSUBPD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseHsubpdEmitter<A, B> {
    fn sse_hsubpd(&mut self, op0: A, op1: B);
}

impl<'a> SseHsubpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_hsubpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SSE_HSUBPDRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SseHsubpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_hsubpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_HSUBPDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_HSUBPS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseHsubpsEmitter<A, B> {
    fn sse_hsubps(&mut self, op0: A, op1: B);
}

impl<'a> SseHsubpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_hsubps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SSE_HSUBPSRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SseHsubpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_hsubps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_HSUBPSRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_LDDQU`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// +---+----------+
/// ```
pub trait SseLddquEmitter<A, B> {
    fn sse_lddqu(&mut self, op0: A, op1: B);
}

impl<'a> SseLddquEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_lddqu(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_LDDQURM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_MOVDDUP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovddupEmitter<A, B> {
    fn sse_movddup(&mut self, op0: A, op1: B);
}

impl<'a> SseMovddupEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movddup(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SSE_MOVDDUPRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SseMovddupEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movddup(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_MOVDDUPRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_MOVSHDUP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovshdupEmitter<A, B> {
    fn sse_movshdup(&mut self, op0: A, op1: B);
}

impl<'a> SseMovshdupEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movshdup(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SSE_MOVSHDUPRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SseMovshdupEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movshdup(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_MOVSHDUPRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_MOVSLDUP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovsldupEmitter<A, B> {
    fn sse_movsldup(&mut self, op0: A, op1: B);
}

impl<'a> SseMovsldupEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movsldup(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SSE_MOVSLDUPRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SseMovsldupEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movsldup(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_MOVSLDUPRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `FISTTP`.
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
    pub fn fisttp<A>(&mut self, op0: A)
    where
        Assembler<'a>: FisttpEmitter<A>,
    {
        <Self as FisttpEmitter<A>>::fisttp(self, op0);
    }
    /// `SSE_ADDSUBPD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_addsubpd<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseAddsubpdEmitter<A, B>,
    {
        <Self as SseAddsubpdEmitter<A, B>>::sse_addsubpd(self, op0, op1);
    }
    /// `SSE_ADDSUBPS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_addsubps<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseAddsubpsEmitter<A, B>,
    {
        <Self as SseAddsubpsEmitter<A, B>>::sse_addsubps(self, op0, op1);
    }
    /// `SSE_HADDPD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_haddpd<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseHaddpdEmitter<A, B>,
    {
        <Self as SseHaddpdEmitter<A, B>>::sse_haddpd(self, op0, op1);
    }
    /// `SSE_HADDPS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_haddps<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseHaddpsEmitter<A, B>,
    {
        <Self as SseHaddpsEmitter<A, B>>::sse_haddps(self, op0, op1);
    }
    /// `SSE_HSUBPD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_hsubpd<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseHsubpdEmitter<A, B>,
    {
        <Self as SseHsubpdEmitter<A, B>>::sse_hsubpd(self, op0, op1);
    }
    /// `SSE_HSUBPS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_hsubps<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseHsubpsEmitter<A, B>,
    {
        <Self as SseHsubpsEmitter<A, B>>::sse_hsubps(self, op0, op1);
    }
    /// `SSE_LDDQU`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_lddqu<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseLddquEmitter<A, B>,
    {
        <Self as SseLddquEmitter<A, B>>::sse_lddqu(self, op0, op1);
    }
    /// `SSE_MOVDDUP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movddup<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseMovddupEmitter<A, B>,
    {
        <Self as SseMovddupEmitter<A, B>>::sse_movddup(self, op0, op1);
    }
    /// `SSE_MOVSHDUP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movshdup<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseMovshdupEmitter<A, B>,
    {
        <Self as SseMovshdupEmitter<A, B>>::sse_movshdup(self, op0, op1);
    }
    /// `SSE_MOVSLDUP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movsldup<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseMovsldupEmitter<A, B>,
    {
        <Self as SseMovsldupEmitter<A, B>>::sse_movsldup(self, op0, op1);
    }
}
