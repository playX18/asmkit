use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `BZHI`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Gpd, Gpd |
/// | 2 | Gpd, Mem, Gpd |
/// | 3 | Gpq, Gpq, Gpq |
/// | 4 | Gpq, Mem, Gpq |
/// +---+---------------+
/// ```
pub trait BzhiEmitter<A, B, C> {
    fn bzhi(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> BzhiEmitter<Gpd, Gpd, Gpd> for Assembler<'a> {
    fn bzhi(&mut self, op0: Gpd, op1: Gpd, op2: Gpd) {
        self.emit(
            BZHI32RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> BzhiEmitter<Gpd, Mem, Gpd> for Assembler<'a> {
    fn bzhi(&mut self, op0: Gpd, op1: Mem, op2: Gpd) {
        self.emit(
            BZHI32RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> BzhiEmitter<Gpq, Gpq, Gpq> for Assembler<'a> {
    fn bzhi(&mut self, op0: Gpq, op1: Gpq, op2: Gpq) {
        self.emit(
            BZHI64RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> BzhiEmitter<Gpq, Mem, Gpq> for Assembler<'a> {
    fn bzhi(&mut self, op0: Gpq, op1: Mem, op2: Gpq) {
        self.emit(
            BZHI64RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `MULX`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Gpd, Gpd |
/// | 2 | Gpd, Gpd, Mem |
/// | 3 | Gpq, Gpq, Gpq |
/// | 4 | Gpq, Gpq, Mem |
/// +---+---------------+
/// ```
pub trait MulxEmitter<A, B, C> {
    fn mulx(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> MulxEmitter<Gpd, Gpd, Gpd> for Assembler<'a> {
    fn mulx(&mut self, op0: Gpd, op1: Gpd, op2: Gpd) {
        self.emit(
            MULX32RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> MulxEmitter<Gpd, Gpd, Mem> for Assembler<'a> {
    fn mulx(&mut self, op0: Gpd, op1: Gpd, op2: Mem) {
        self.emit(
            MULX32RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> MulxEmitter<Gpq, Gpq, Gpq> for Assembler<'a> {
    fn mulx(&mut self, op0: Gpq, op1: Gpq, op2: Gpq) {
        self.emit(
            MULX64RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> MulxEmitter<Gpq, Gpq, Mem> for Assembler<'a> {
    fn mulx(&mut self, op0: Gpq, op1: Gpq, op2: Mem) {
        self.emit(
            MULX64RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `PDEP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Gpd, Gpd |
/// | 2 | Gpd, Gpd, Mem |
/// | 3 | Gpq, Gpq, Gpq |
/// | 4 | Gpq, Gpq, Mem |
/// +---+---------------+
/// ```
pub trait PdepEmitter<A, B, C> {
    fn pdep(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> PdepEmitter<Gpd, Gpd, Gpd> for Assembler<'a> {
    fn pdep(&mut self, op0: Gpd, op1: Gpd, op2: Gpd) {
        self.emit(
            PDEP32RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> PdepEmitter<Gpd, Gpd, Mem> for Assembler<'a> {
    fn pdep(&mut self, op0: Gpd, op1: Gpd, op2: Mem) {
        self.emit(
            PDEP32RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> PdepEmitter<Gpq, Gpq, Gpq> for Assembler<'a> {
    fn pdep(&mut self, op0: Gpq, op1: Gpq, op2: Gpq) {
        self.emit(
            PDEP64RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> PdepEmitter<Gpq, Gpq, Mem> for Assembler<'a> {
    fn pdep(&mut self, op0: Gpq, op1: Gpq, op2: Mem) {
        self.emit(
            PDEP64RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `PEXT`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Gpd, Gpd |
/// | 2 | Gpd, Gpd, Mem |
/// | 3 | Gpq, Gpq, Gpq |
/// | 4 | Gpq, Gpq, Mem |
/// +---+---------------+
/// ```
pub trait PextEmitter<A, B, C> {
    fn pext(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> PextEmitter<Gpd, Gpd, Gpd> for Assembler<'a> {
    fn pext(&mut self, op0: Gpd, op1: Gpd, op2: Gpd) {
        self.emit(
            PEXT32RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> PextEmitter<Gpd, Gpd, Mem> for Assembler<'a> {
    fn pext(&mut self, op0: Gpd, op1: Gpd, op2: Mem) {
        self.emit(
            PEXT32RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> PextEmitter<Gpq, Gpq, Gpq> for Assembler<'a> {
    fn pext(&mut self, op0: Gpq, op1: Gpq, op2: Gpq) {
        self.emit(
            PEXT64RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> PextEmitter<Gpq, Gpq, Mem> for Assembler<'a> {
    fn pext(&mut self, op0: Gpq, op1: Gpq, op2: Mem) {
        self.emit(
            PEXT64RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `RORX`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Gpd, Imm |
/// | 2 | Gpd, Mem, Imm |
/// | 3 | Gpq, Gpq, Imm |
/// | 4 | Gpq, Mem, Imm |
/// +---+---------------+
/// ```
pub trait RorxEmitter<A, B, C> {
    fn rorx(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> RorxEmitter<Gpd, Gpd, Imm> for Assembler<'a> {
    fn rorx(&mut self, op0: Gpd, op1: Gpd, op2: Imm) {
        self.emit(
            RORX32RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> RorxEmitter<Gpd, Mem, Imm> for Assembler<'a> {
    fn rorx(&mut self, op0: Gpd, op1: Mem, op2: Imm) {
        self.emit(
            RORX32RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> RorxEmitter<Gpq, Gpq, Imm> for Assembler<'a> {
    fn rorx(&mut self, op0: Gpq, op1: Gpq, op2: Imm) {
        self.emit(
            RORX64RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> RorxEmitter<Gpq, Mem, Imm> for Assembler<'a> {
    fn rorx(&mut self, op0: Gpq, op1: Mem, op2: Imm) {
        self.emit(
            RORX64RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `SARX`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Gpd, Gpd |
/// | 2 | Gpd, Mem, Gpd |
/// | 3 | Gpq, Gpq, Gpq |
/// | 4 | Gpq, Mem, Gpq |
/// +---+---------------+
/// ```
pub trait SarxEmitter<A, B, C> {
    fn sarx(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SarxEmitter<Gpd, Gpd, Gpd> for Assembler<'a> {
    fn sarx(&mut self, op0: Gpd, op1: Gpd, op2: Gpd) {
        self.emit(
            SARX32RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> SarxEmitter<Gpd, Mem, Gpd> for Assembler<'a> {
    fn sarx(&mut self, op0: Gpd, op1: Mem, op2: Gpd) {
        self.emit(
            SARX32RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> SarxEmitter<Gpq, Gpq, Gpq> for Assembler<'a> {
    fn sarx(&mut self, op0: Gpq, op1: Gpq, op2: Gpq) {
        self.emit(
            SARX64RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> SarxEmitter<Gpq, Mem, Gpq> for Assembler<'a> {
    fn sarx(&mut self, op0: Gpq, op1: Mem, op2: Gpq) {
        self.emit(
            SARX64RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `SHLX`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Gpd, Gpd |
/// | 2 | Gpd, Mem, Gpd |
/// | 3 | Gpq, Gpq, Gpq |
/// | 4 | Gpq, Mem, Gpq |
/// +---+---------------+
/// ```
pub trait ShlxEmitter<A, B, C> {
    fn shlx(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> ShlxEmitter<Gpd, Gpd, Gpd> for Assembler<'a> {
    fn shlx(&mut self, op0: Gpd, op1: Gpd, op2: Gpd) {
        self.emit(
            SHLX32RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> ShlxEmitter<Gpd, Mem, Gpd> for Assembler<'a> {
    fn shlx(&mut self, op0: Gpd, op1: Mem, op2: Gpd) {
        self.emit(
            SHLX32RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> ShlxEmitter<Gpq, Gpq, Gpq> for Assembler<'a> {
    fn shlx(&mut self, op0: Gpq, op1: Gpq, op2: Gpq) {
        self.emit(
            SHLX64RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> ShlxEmitter<Gpq, Mem, Gpq> for Assembler<'a> {
    fn shlx(&mut self, op0: Gpq, op1: Mem, op2: Gpq) {
        self.emit(
            SHLX64RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `SHRX`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Gpd, Gpd |
/// | 2 | Gpd, Mem, Gpd |
/// | 3 | Gpq, Gpq, Gpq |
/// | 4 | Gpq, Mem, Gpq |
/// +---+---------------+
/// ```
pub trait ShrxEmitter<A, B, C> {
    fn shrx(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> ShrxEmitter<Gpd, Gpd, Gpd> for Assembler<'a> {
    fn shrx(&mut self, op0: Gpd, op1: Gpd, op2: Gpd) {
        self.emit(
            SHRX32RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> ShrxEmitter<Gpd, Mem, Gpd> for Assembler<'a> {
    fn shrx(&mut self, op0: Gpd, op1: Mem, op2: Gpd) {
        self.emit(
            SHRX32RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> ShrxEmitter<Gpq, Gpq, Gpq> for Assembler<'a> {
    fn shrx(&mut self, op0: Gpq, op1: Gpq, op2: Gpq) {
        self.emit(
            SHRX64RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> ShrxEmitter<Gpq, Mem, Gpq> for Assembler<'a> {
    fn shrx(&mut self, op0: Gpq, op1: Mem, op2: Gpq) {
        self.emit(
            SHRX64RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `BZHI`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Gpd, Gpd |
    /// | 2 | Gpd, Mem, Gpd |
    /// | 3 | Gpq, Gpq, Gpq |
    /// | 4 | Gpq, Mem, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn bzhi<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: BzhiEmitter<A, B, C>,
    {
        <Self as BzhiEmitter<A, B, C>>::bzhi(self, op0, op1, op2);
    }
    /// `MULX`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Gpd, Gpd |
    /// | 2 | Gpd, Gpd, Mem |
    /// | 3 | Gpq, Gpq, Gpq |
    /// | 4 | Gpq, Gpq, Mem |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn mulx<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: MulxEmitter<A, B, C>,
    {
        <Self as MulxEmitter<A, B, C>>::mulx(self, op0, op1, op2);
    }
    /// `PDEP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Gpd, Gpd |
    /// | 2 | Gpd, Gpd, Mem |
    /// | 3 | Gpq, Gpq, Gpq |
    /// | 4 | Gpq, Gpq, Mem |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn pdep<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: PdepEmitter<A, B, C>,
    {
        <Self as PdepEmitter<A, B, C>>::pdep(self, op0, op1, op2);
    }
    /// `PEXT`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Gpd, Gpd |
    /// | 2 | Gpd, Gpd, Mem |
    /// | 3 | Gpq, Gpq, Gpq |
    /// | 4 | Gpq, Gpq, Mem |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn pext<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: PextEmitter<A, B, C>,
    {
        <Self as PextEmitter<A, B, C>>::pext(self, op0, op1, op2);
    }
    /// `RORX`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Gpd, Imm |
    /// | 2 | Gpd, Mem, Imm |
    /// | 3 | Gpq, Gpq, Imm |
    /// | 4 | Gpq, Mem, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn rorx<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: RorxEmitter<A, B, C>,
    {
        <Self as RorxEmitter<A, B, C>>::rorx(self, op0, op1, op2);
    }
    /// `SARX`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Gpd, Gpd |
    /// | 2 | Gpd, Mem, Gpd |
    /// | 3 | Gpq, Gpq, Gpq |
    /// | 4 | Gpq, Mem, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn sarx<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: SarxEmitter<A, B, C>,
    {
        <Self as SarxEmitter<A, B, C>>::sarx(self, op0, op1, op2);
    }
    /// `SHLX`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Gpd, Gpd |
    /// | 2 | Gpd, Mem, Gpd |
    /// | 3 | Gpq, Gpq, Gpq |
    /// | 4 | Gpq, Mem, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn shlx<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: ShlxEmitter<A, B, C>,
    {
        <Self as ShlxEmitter<A, B, C>>::shlx(self, op0, op1, op2);
    }
    /// `SHRX`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Gpd, Gpd |
    /// | 2 | Gpd, Mem, Gpd |
    /// | 3 | Gpq, Gpq, Gpq |
    /// | 4 | Gpq, Mem, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn shrx<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: ShrxEmitter<A, B, C>,
    {
        <Self as ShrxEmitter<A, B, C>>::shrx(self, op0, op1, op2);
    }
}
