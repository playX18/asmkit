use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `SHA1MSG1`.
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
pub trait Sha1msg1Emitter<A, B> {
    fn sha1msg1(&mut self, op0: A, op1: B);
}

impl<'a> Sha1msg1Emitter<Xmm, Xmm> for Assembler<'a> {
    fn sha1msg1(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SHA1MSG1RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Sha1msg1Emitter<Xmm, Mem> for Assembler<'a> {
    fn sha1msg1(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SHA1MSG1RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SHA1MSG2`.
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
pub trait Sha1msg2Emitter<A, B> {
    fn sha1msg2(&mut self, op0: A, op1: B);
}

impl<'a> Sha1msg2Emitter<Xmm, Xmm> for Assembler<'a> {
    fn sha1msg2(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SHA1MSG2RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Sha1msg2Emitter<Xmm, Mem> for Assembler<'a> {
    fn sha1msg2(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SHA1MSG2RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SHA1NEXTE`.
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
pub trait Sha1nexteEmitter<A, B> {
    fn sha1nexte(&mut self, op0: A, op1: B);
}

impl<'a> Sha1nexteEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sha1nexte(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SHA1NEXTERR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Sha1nexteEmitter<Xmm, Mem> for Assembler<'a> {
    fn sha1nexte(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SHA1NEXTERM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SHA1RNDS4`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// +---+---------------+
/// ```
pub trait Sha1rnds4Emitter<A, B, C> {
    fn sha1rnds4(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Sha1rnds4Emitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sha1rnds4(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            SHA1RNDS4RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Sha1rnds4Emitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sha1rnds4(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            SHA1RNDS4RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `SHA256MSG1`.
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
pub trait Sha256msg1Emitter<A, B> {
    fn sha256msg1(&mut self, op0: A, op1: B);
}

impl<'a> Sha256msg1Emitter<Xmm, Xmm> for Assembler<'a> {
    fn sha256msg1(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SHA256MSG1RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Sha256msg1Emitter<Xmm, Mem> for Assembler<'a> {
    fn sha256msg1(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SHA256MSG1RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SHA256MSG2`.
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
pub trait Sha256msg2Emitter<A, B> {
    fn sha256msg2(&mut self, op0: A, op1: B);
}

impl<'a> Sha256msg2Emitter<Xmm, Xmm> for Assembler<'a> {
    fn sha256msg2(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SHA256MSG2RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Sha256msg2Emitter<Xmm, Mem> for Assembler<'a> {
    fn sha256msg2(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SHA256MSG2RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SHA256RNDS2`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Xmm, Xmm, Xmm |
/// +---+---------------+
/// ```
pub trait Sha256rnds2Emitter<A, B, C> {
    fn sha256rnds2(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Sha256rnds2Emitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn sha256rnds2(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            SHA256RNDS2RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Sha256rnds2Emitter<Xmm, Mem, Xmm> for Assembler<'a> {
    fn sha256rnds2(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(
            SHA256RNDS2RMR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `SHA1MSG1`.
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
    pub fn sha1msg1<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Sha1msg1Emitter<A, B>,
    {
        <Self as Sha1msg1Emitter<A, B>>::sha1msg1(self, op0, op1);
    }
    /// `SHA1MSG2`.
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
    pub fn sha1msg2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Sha1msg2Emitter<A, B>,
    {
        <Self as Sha1msg2Emitter<A, B>>::sha1msg2(self, op0, op1);
    }
    /// `SHA1NEXTE`.
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
    pub fn sha1nexte<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Sha1nexteEmitter<A, B>,
    {
        <Self as Sha1nexteEmitter<A, B>>::sha1nexte(self, op0, op1);
    }
    /// `SHA1RNDS4`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn sha1rnds4<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Sha1rnds4Emitter<A, B, C>,
    {
        <Self as Sha1rnds4Emitter<A, B, C>>::sha1rnds4(self, op0, op1, op2);
    }
    /// `SHA256MSG1`.
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
    pub fn sha256msg1<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Sha256msg1Emitter<A, B>,
    {
        <Self as Sha256msg1Emitter<A, B>>::sha256msg1(self, op0, op1);
    }
    /// `SHA256MSG2`.
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
    pub fn sha256msg2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Sha256msg2Emitter<A, B>,
    {
        <Self as Sha256msg2Emitter<A, B>>::sha256msg2(self, op0, op1);
    }
    /// `SHA256RNDS2`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Xmm, Xmm, Xmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn sha256rnds2<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Sha256rnds2Emitter<A, B, C>,
    {
        <Self as Sha256rnds2Emitter<A, B, C>>::sha256rnds2(self, op0, op1, op2);
    }
}
