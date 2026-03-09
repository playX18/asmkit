use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `ANDN`.
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
pub trait AndnEmitter<A, B, C> {
    fn andn(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> AndnEmitter<Gpd, Gpd, Gpd> for Assembler<'a> {
    fn andn(&mut self, op0: Gpd, op1: Gpd, op2: Gpd) {
        self.emit(
            ANDN32RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> AndnEmitter<Gpd, Gpd, Mem> for Assembler<'a> {
    fn andn(&mut self, op0: Gpd, op1: Gpd, op2: Mem) {
        self.emit(
            ANDN32RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> AndnEmitter<Gpq, Gpq, Gpq> for Assembler<'a> {
    fn andn(&mut self, op0: Gpq, op1: Gpq, op2: Gpq) {
        self.emit(
            ANDN64RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> AndnEmitter<Gpq, Gpq, Mem> for Assembler<'a> {
    fn andn(&mut self, op0: Gpq, op1: Gpq, op2: Mem) {
        self.emit(
            ANDN64RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `BEXTR`.
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
pub trait BextrEmitter<A, B, C> {
    fn bextr(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> BextrEmitter<Gpd, Gpd, Gpd> for Assembler<'a> {
    fn bextr(&mut self, op0: Gpd, op1: Gpd, op2: Gpd) {
        self.emit(
            BEXTR32RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> BextrEmitter<Gpd, Mem, Gpd> for Assembler<'a> {
    fn bextr(&mut self, op0: Gpd, op1: Mem, op2: Gpd) {
        self.emit(
            BEXTR32RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> BextrEmitter<Gpq, Gpq, Gpq> for Assembler<'a> {
    fn bextr(&mut self, op0: Gpq, op1: Gpq, op2: Gpq) {
        self.emit(
            BEXTR64RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> BextrEmitter<Gpq, Mem, Gpq> for Assembler<'a> {
    fn bextr(&mut self, op0: Gpq, op1: Mem, op2: Gpq) {
        self.emit(
            BEXTR64RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `BLSI`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Gpd |
/// | 2 | Gpd, Mem |
/// | 3 | Gpq, Gpq |
/// | 4 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait BlsiEmitter<A, B> {
    fn blsi(&mut self, op0: A, op1: B);
}

impl<'a> BlsiEmitter<Gpd, Gpd> for Assembler<'a> {
    fn blsi(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(BLSI32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> BlsiEmitter<Gpd, Mem> for Assembler<'a> {
    fn blsi(&mut self, op0: Gpd, op1: Mem) {
        self.emit(BLSI32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> BlsiEmitter<Gpq, Gpq> for Assembler<'a> {
    fn blsi(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(BLSI64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> BlsiEmitter<Gpq, Mem> for Assembler<'a> {
    fn blsi(&mut self, op0: Gpq, op1: Mem) {
        self.emit(BLSI64RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `BLSMSK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Gpd |
/// | 2 | Gpd, Mem |
/// | 3 | Gpq, Gpq |
/// | 4 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait BlsmskEmitter<A, B> {
    fn blsmsk(&mut self, op0: A, op1: B);
}

impl<'a> BlsmskEmitter<Gpd, Gpd> for Assembler<'a> {
    fn blsmsk(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            BLSMSK32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> BlsmskEmitter<Gpd, Mem> for Assembler<'a> {
    fn blsmsk(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            BLSMSK32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> BlsmskEmitter<Gpq, Gpq> for Assembler<'a> {
    fn blsmsk(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            BLSMSK64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> BlsmskEmitter<Gpq, Mem> for Assembler<'a> {
    fn blsmsk(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            BLSMSK64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `BLSR`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Gpd |
/// | 2 | Gpd, Mem |
/// | 3 | Gpq, Gpq |
/// | 4 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait BlsrEmitter<A, B> {
    fn blsr(&mut self, op0: A, op1: B);
}

impl<'a> BlsrEmitter<Gpd, Gpd> for Assembler<'a> {
    fn blsr(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(BLSR32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> BlsrEmitter<Gpd, Mem> for Assembler<'a> {
    fn blsr(&mut self, op0: Gpd, op1: Mem) {
        self.emit(BLSR32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> BlsrEmitter<Gpq, Gpq> for Assembler<'a> {
    fn blsr(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(BLSR64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> BlsrEmitter<Gpq, Mem> for Assembler<'a> {
    fn blsr(&mut self, op0: Gpq, op1: Mem) {
        self.emit(BLSR64RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `TZCNT`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Gpd |
/// | 2 | Gpd, Mem |
/// | 3 | Gpq, Gpq |
/// | 4 | Gpq, Mem |
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait TzcntEmitter<A, B> {
    fn tzcnt(&mut self, op0: A, op1: B);
}

impl<'a> TzcntEmitter<Gpw, Gpw> for Assembler<'a> {
    fn tzcnt(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            TZCNT16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> TzcntEmitter<Gpw, Mem> for Assembler<'a> {
    fn tzcnt(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            TZCNT16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> TzcntEmitter<Gpd, Gpd> for Assembler<'a> {
    fn tzcnt(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            TZCNT32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> TzcntEmitter<Gpd, Mem> for Assembler<'a> {
    fn tzcnt(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            TZCNT32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> TzcntEmitter<Gpq, Gpq> for Assembler<'a> {
    fn tzcnt(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            TZCNT64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> TzcntEmitter<Gpq, Mem> for Assembler<'a> {
    fn tzcnt(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            TZCNT64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `ANDN`.
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
    pub fn andn<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: AndnEmitter<A, B, C>,
    {
        <Self as AndnEmitter<A, B, C>>::andn(self, op0, op1, op2);
    }
    /// `BEXTR`.
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
    pub fn bextr<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: BextrEmitter<A, B, C>,
    {
        <Self as BextrEmitter<A, B, C>>::bextr(self, op0, op1, op2);
    }
    /// `BLSI`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Gpd |
    /// | 2 | Gpd, Mem |
    /// | 3 | Gpq, Gpq |
    /// | 4 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn blsi<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: BlsiEmitter<A, B>,
    {
        <Self as BlsiEmitter<A, B>>::blsi(self, op0, op1);
    }
    /// `BLSMSK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Gpd |
    /// | 2 | Gpd, Mem |
    /// | 3 | Gpq, Gpq |
    /// | 4 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn blsmsk<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: BlsmskEmitter<A, B>,
    {
        <Self as BlsmskEmitter<A, B>>::blsmsk(self, op0, op1);
    }
    /// `BLSR`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Gpd |
    /// | 2 | Gpd, Mem |
    /// | 3 | Gpq, Gpq |
    /// | 4 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn blsr<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: BlsrEmitter<A, B>,
    {
        <Self as BlsrEmitter<A, B>>::blsr(self, op0, op1);
    }
    /// `TZCNT`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Gpd |
    /// | 2 | Gpd, Mem |
    /// | 3 | Gpq, Gpq |
    /// | 4 | Gpq, Mem |
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn tzcnt<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: TzcntEmitter<A, B>,
    {
        <Self as TzcntEmitter<A, B>>::tzcnt(self, op0, op1);
    }
}
