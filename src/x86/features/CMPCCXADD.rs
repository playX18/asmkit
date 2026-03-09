use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CMPBEXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpbexaddEmitter<A, B, C> {
    fn cmpbexadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpbexaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpbexadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPBEXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpbexaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpbexadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPBEXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPBXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpbxaddEmitter<A, B, C> {
    fn cmpbxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpbxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpbxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPBXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpbxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpbxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPBXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPLEXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmplexaddEmitter<A, B, C> {
    fn cmplexadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmplexaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmplexadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPLEXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmplexaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmplexadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPLEXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPLXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmplxaddEmitter<A, B, C> {
    fn cmplxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmplxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmplxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPLXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmplxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmplxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPLXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPNBEXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpnbexaddEmitter<A, B, C> {
    fn cmpnbexadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpnbexaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpnbexadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPNBEXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpnbexaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpnbexadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPNBEXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPNBXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpnbxaddEmitter<A, B, C> {
    fn cmpnbxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpnbxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpnbxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPNBXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpnbxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpnbxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPNBXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPNLEXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpnlexaddEmitter<A, B, C> {
    fn cmpnlexadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpnlexaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpnlexadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPNLEXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpnlexaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpnlexadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPNLEXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPNLXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpnlxaddEmitter<A, B, C> {
    fn cmpnlxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpnlxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpnlxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPNLXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpnlxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpnlxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPNLXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPNOXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpnoxaddEmitter<A, B, C> {
    fn cmpnoxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpnoxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpnoxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPNOXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpnoxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpnoxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPNOXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPNPXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpnpxaddEmitter<A, B, C> {
    fn cmpnpxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpnpxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpnpxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPNPXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpnpxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpnpxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPNPXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPNSXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpnsxaddEmitter<A, B, C> {
    fn cmpnsxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpnsxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpnsxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPNSXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpnsxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpnsxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPNSXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPNZXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpnzxaddEmitter<A, B, C> {
    fn cmpnzxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpnzxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpnzxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPNZXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpnzxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpnzxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPNZXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPOXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpoxaddEmitter<A, B, C> {
    fn cmpoxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpoxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpoxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPOXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpoxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpoxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPOXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPPXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmppxaddEmitter<A, B, C> {
    fn cmppxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmppxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmppxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPPXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmppxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmppxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPPXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPSXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpsxaddEmitter<A, B, C> {
    fn cmpsxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpsxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpsxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPSXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpsxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpsxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPSXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPZXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpzxaddEmitter<A, B, C> {
    fn cmpzxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpzxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpzxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPZXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpzxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpzxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPZXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `CMPCCXADD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Gpd, Gpd |
/// | 2 | Mem, Gpq, Gpq |
/// +---+---------------+
/// ```
pub trait CmpccxaddEmitter<A, B, C> {
    fn cmpccxadd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> CmpccxaddEmitter<Mem, Gpd, Gpd> for Assembler<'a> {
    fn cmpccxadd(&mut self, op0: Mem, op1: Gpd, op2: Gpd) {
        self.emit(
            CMPCCXADD32MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> CmpccxaddEmitter<Mem, Gpq, Gpq> for Assembler<'a> {
    fn cmpccxadd(&mut self, op0: Mem, op1: Gpq, op2: Gpq) {
        self.emit(
            CMPCCXADD64MRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `CMPBEXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpbexadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpbexaddEmitter<A, B, C>,
    {
        <Self as CmpbexaddEmitter<A, B, C>>::cmpbexadd(self, op0, op1, op2);
    }
    /// `CMPBXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpbxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpbxaddEmitter<A, B, C>,
    {
        <Self as CmpbxaddEmitter<A, B, C>>::cmpbxadd(self, op0, op1, op2);
    }
    /// `CMPLEXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmplexadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmplexaddEmitter<A, B, C>,
    {
        <Self as CmplexaddEmitter<A, B, C>>::cmplexadd(self, op0, op1, op2);
    }
    /// `CMPLXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmplxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmplxaddEmitter<A, B, C>,
    {
        <Self as CmplxaddEmitter<A, B, C>>::cmplxadd(self, op0, op1, op2);
    }
    /// `CMPNBEXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpnbexadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpnbexaddEmitter<A, B, C>,
    {
        <Self as CmpnbexaddEmitter<A, B, C>>::cmpnbexadd(self, op0, op1, op2);
    }
    /// `CMPNBXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpnbxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpnbxaddEmitter<A, B, C>,
    {
        <Self as CmpnbxaddEmitter<A, B, C>>::cmpnbxadd(self, op0, op1, op2);
    }
    /// `CMPNLEXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpnlexadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpnlexaddEmitter<A, B, C>,
    {
        <Self as CmpnlexaddEmitter<A, B, C>>::cmpnlexadd(self, op0, op1, op2);
    }
    /// `CMPNLXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpnlxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpnlxaddEmitter<A, B, C>,
    {
        <Self as CmpnlxaddEmitter<A, B, C>>::cmpnlxadd(self, op0, op1, op2);
    }
    /// `CMPNOXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpnoxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpnoxaddEmitter<A, B, C>,
    {
        <Self as CmpnoxaddEmitter<A, B, C>>::cmpnoxadd(self, op0, op1, op2);
    }
    /// `CMPNPXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpnpxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpnpxaddEmitter<A, B, C>,
    {
        <Self as CmpnpxaddEmitter<A, B, C>>::cmpnpxadd(self, op0, op1, op2);
    }
    /// `CMPNSXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpnsxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpnsxaddEmitter<A, B, C>,
    {
        <Self as CmpnsxaddEmitter<A, B, C>>::cmpnsxadd(self, op0, op1, op2);
    }
    /// `CMPNZXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpnzxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpnzxaddEmitter<A, B, C>,
    {
        <Self as CmpnzxaddEmitter<A, B, C>>::cmpnzxadd(self, op0, op1, op2);
    }
    /// `CMPOXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpoxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpoxaddEmitter<A, B, C>,
    {
        <Self as CmpoxaddEmitter<A, B, C>>::cmpoxadd(self, op0, op1, op2);
    }
    /// `CMPPXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmppxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmppxaddEmitter<A, B, C>,
    {
        <Self as CmppxaddEmitter<A, B, C>>::cmppxadd(self, op0, op1, op2);
    }
    /// `CMPSXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpsxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpsxaddEmitter<A, B, C>,
    {
        <Self as CmpsxaddEmitter<A, B, C>>::cmpsxadd(self, op0, op1, op2);
    }
    /// `CMPZXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpzxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpzxaddEmitter<A, B, C>,
    {
        <Self as CmpzxaddEmitter<A, B, C>>::cmpzxadd(self, op0, op1, op2);
    }
    /// `CMPCCXADD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Gpd, Gpd |
    /// | 2 | Mem, Gpq, Gpq |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn cmpccxadd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: CmpccxaddEmitter<A, B, C>,
    {
        <Self as CmpccxaddEmitter<A, B, C>>::cmpccxadd(self, op0, op1, op2);
    }
}
