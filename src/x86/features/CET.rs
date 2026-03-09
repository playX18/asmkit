use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CLRSSBSY`.
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
pub trait ClrssbsyEmitter<A> {
    fn clrssbsy(&mut self, op0: A);
}

impl<'a> ClrssbsyEmitter<Mem> for Assembler<'a> {
    fn clrssbsy(&mut self, op0: Mem) {
        self.emit(CLRSSBSYM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `ENDBR32`.
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
pub trait Endbr32Emitter {
    fn endbr32(&mut self);
}

impl<'a> Endbr32Emitter for Assembler<'a> {
    fn endbr32(&mut self) {
        self.emit(ENDBR32, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `ENDBR64`.
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
pub trait Endbr64Emitter {
    fn endbr64(&mut self);
}

impl<'a> Endbr64Emitter for Assembler<'a> {
    fn endbr64(&mut self) {
        self.emit(ENDBR64, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `INCSSP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// +---+----------+
/// ```
pub trait IncsspEmitter<A> {
    fn incssp(&mut self, op0: A);
}

impl<'a> IncsspEmitter<Gpd> for Assembler<'a> {
    fn incssp(&mut self, op0: Gpd) {
        self.emit(INCSSP32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> IncsspEmitter<Gpq> for Assembler<'a> {
    fn incssp(&mut self, op0: Gpq) {
        self.emit(INCSSP64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `RDSSP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// +---+----------+
/// ```
pub trait RdsspEmitter<A> {
    fn rdssp(&mut self, op0: A);
}

impl<'a> RdsspEmitter<Gpd> for Assembler<'a> {
    fn rdssp(&mut self, op0: Gpd) {
        self.emit(RDSSP32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> RdsspEmitter<Gpq> for Assembler<'a> {
    fn rdssp(&mut self, op0: Gpq) {
        self.emit(RDSSP64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `RSTORSSP`.
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
pub trait RstorsspEmitter<A> {
    fn rstorssp(&mut self, op0: A);
}

impl<'a> RstorsspEmitter<Mem> for Assembler<'a> {
    fn rstorssp(&mut self, op0: Mem) {
        self.emit(RSTORSSPM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `SAVEPREVSSP`.
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
pub trait SaveprevsspEmitter {
    fn saveprevssp(&mut self);
}

impl<'a> SaveprevsspEmitter for Assembler<'a> {
    fn saveprevssp(&mut self) {
        self.emit(SAVEPREVSSP, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `SETSSBSY`.
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
pub trait SetssbsyEmitter {
    fn setssbsy(&mut self);
}

impl<'a> SetssbsyEmitter for Assembler<'a> {
    fn setssbsy(&mut self) {
        self.emit(SETSSBSY, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `WRSS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Gpd |
/// | 2 | Mem, Gpq |
/// +---+----------+
/// ```
pub trait WrssEmitter<A, B> {
    fn wrss(&mut self, op0: A, op1: B);
}

impl<'a> WrssEmitter<Mem, Gpd> for Assembler<'a> {
    fn wrss(&mut self, op0: Mem, op1: Gpd) {
        self.emit(WRSS32MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> WrssEmitter<Mem, Gpq> for Assembler<'a> {
    fn wrss(&mut self, op0: Mem, op1: Gpq) {
        self.emit(WRSS64MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `WRUSS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Gpd |
/// | 2 | Mem, Gpq |
/// +---+----------+
/// ```
pub trait WrussEmitter<A, B> {
    fn wruss(&mut self, op0: A, op1: B);
}

impl<'a> WrussEmitter<Mem, Gpd> for Assembler<'a> {
    fn wruss(&mut self, op0: Mem, op1: Gpd) {
        self.emit(
            WRUSS32MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> WrussEmitter<Mem, Gpq> for Assembler<'a> {
    fn wruss(&mut self, op0: Mem, op1: Gpq) {
        self.emit(
            WRUSS64MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `CLRSSBSY`.
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
    pub fn clrssbsy<A>(&mut self, op0: A)
    where
        Assembler<'a>: ClrssbsyEmitter<A>,
    {
        <Self as ClrssbsyEmitter<A>>::clrssbsy(self, op0);
    }
    /// `ENDBR32`.
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
    pub fn endbr32(&mut self)
    where
        Assembler<'a>: Endbr32Emitter,
    {
        <Self as Endbr32Emitter>::endbr32(self);
    }
    /// `ENDBR64`.
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
    pub fn endbr64(&mut self)
    where
        Assembler<'a>: Endbr64Emitter,
    {
        <Self as Endbr64Emitter>::endbr64(self);
    }
    /// `INCSSP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn incssp<A>(&mut self, op0: A)
    where
        Assembler<'a>: IncsspEmitter<A>,
    {
        <Self as IncsspEmitter<A>>::incssp(self, op0);
    }
    /// `RDSSP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn rdssp<A>(&mut self, op0: A)
    where
        Assembler<'a>: RdsspEmitter<A>,
    {
        <Self as RdsspEmitter<A>>::rdssp(self, op0);
    }
    /// `RSTORSSP`.
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
    pub fn rstorssp<A>(&mut self, op0: A)
    where
        Assembler<'a>: RstorsspEmitter<A>,
    {
        <Self as RstorsspEmitter<A>>::rstorssp(self, op0);
    }
    /// `SAVEPREVSSP`.
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
    pub fn saveprevssp(&mut self)
    where
        Assembler<'a>: SaveprevsspEmitter,
    {
        <Self as SaveprevsspEmitter>::saveprevssp(self);
    }
    /// `SETSSBSY`.
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
    pub fn setssbsy(&mut self)
    where
        Assembler<'a>: SetssbsyEmitter,
    {
        <Self as SetssbsyEmitter>::setssbsy(self);
    }
    /// `WRSS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Gpd |
    /// | 2 | Mem, Gpq |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn wrss<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: WrssEmitter<A, B>,
    {
        <Self as WrssEmitter<A, B>>::wrss(self, op0, op1);
    }
    /// `WRUSS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Gpd |
    /// | 2 | Mem, Gpq |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn wruss<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: WrussEmitter<A, B>,
    {
        <Self as WrussEmitter<A, B>>::wruss(self, op0, op1);
    }
}
