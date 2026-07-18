use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `GF2P8AFFINEINVQB`.
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
pub trait Gf2p8affineinvqbEmitter<A, B, C> {
    fn gf2p8affineinvqb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Gf2p8affineinvqbEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn gf2p8affineinvqb(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            GF2P8AFFINEINVQBRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Gf2p8affineinvqbEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn gf2p8affineinvqb(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            GF2P8AFFINEINVQBRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `GF2P8AFFINEQB`.
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
pub trait Gf2p8affineqbEmitter<A, B, C> {
    fn gf2p8affineqb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Gf2p8affineqbEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn gf2p8affineqb(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            GF2P8AFFINEQBRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Gf2p8affineqbEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn gf2p8affineqb(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            GF2P8AFFINEQBRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `GF2P8MULB`.
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
pub trait Gf2p8mulbEmitter<A, B> {
    fn gf2p8mulb(&mut self, op0: A, op1: B);
}

impl<'a> Gf2p8mulbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn gf2p8mulb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            GF2P8MULBRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Gf2p8mulbEmitter<Xmm, Mem> for Assembler<'a> {
    fn gf2p8mulb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            GF2P8MULBRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `GF2P8AFFINEINVQB`.
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
    pub fn gf2p8affineinvqb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Gf2p8affineinvqbEmitter<A, B, C>,
    {
        <Self as Gf2p8affineinvqbEmitter<A, B, C>>::gf2p8affineinvqb(self, op0, op1, op2);
    }
    /// `GF2P8AFFINEQB`.
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
    pub fn gf2p8affineqb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Gf2p8affineqbEmitter<A, B, C>,
    {
        <Self as Gf2p8affineqbEmitter<A, B, C>>::gf2p8affineqb(self, op0, op1, op2);
    }
    /// `GF2P8MULB`.
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
    pub fn gf2p8mulb<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Gf2p8mulbEmitter<A, B>,
    {
        <Self as Gf2p8mulbEmitter<A, B>>::gf2p8mulb(self, op0, op1);
    }
}
