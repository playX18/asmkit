use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `KADDD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KadddEmitter<A, B, C> {
    fn kaddd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KadddEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kaddd(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KADDDKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KADDQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KaddqEmitter<A, B, C> {
    fn kaddq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KaddqEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kaddq(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KADDQKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KANDD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KanddEmitter<A, B, C> {
    fn kandd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KanddEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kandd(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KANDDKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KANDND`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KandndEmitter<A, B, C> {
    fn kandnd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KandndEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kandnd(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KANDNDKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KANDNQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KandnqEmitter<A, B, C> {
    fn kandnq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KandnqEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kandnq(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KANDNQKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KANDQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KandqEmitter<A, B, C> {
    fn kandq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KandqEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kandq(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KANDQKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KMOVD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | Gpd, KReg  |
/// | 2 | KReg, Gpd  |
/// | 3 | KReg, KReg |
/// | 4 | KReg, Mem  |
/// | 5 | Mem, KReg  |
/// +---+------------+
/// ```
pub trait KmovdEmitter<A, B> {
    fn kmovd(&mut self, op0: A, op1: B);
}

impl<'a> KmovdEmitter<KReg, KReg> for Assembler<'a> {
    fn kmovd(&mut self, op0: KReg, op1: KReg) {
        self.emit(KMOVDKK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovdEmitter<KReg, Mem> for Assembler<'a> {
    fn kmovd(&mut self, op0: KReg, op1: Mem) {
        self.emit(KMOVDKM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovdEmitter<Mem, KReg> for Assembler<'a> {
    fn kmovd(&mut self, op0: Mem, op1: KReg) {
        self.emit(KMOVDMK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovdEmitter<KReg, Gpd> for Assembler<'a> {
    fn kmovd(&mut self, op0: KReg, op1: Gpd) {
        self.emit(KMOVDKR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovdEmitter<Gpd, KReg> for Assembler<'a> {
    fn kmovd(&mut self, op0: Gpd, op1: KReg) {
        self.emit(KMOVDRK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `KMOVQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | Gpd, KReg  |
/// | 2 | KReg, Gpd  |
/// | 3 | KReg, KReg |
/// | 4 | KReg, Mem  |
/// | 5 | Mem, KReg  |
/// +---+------------+
/// ```
pub trait KmovqEmitter<A, B> {
    fn kmovq(&mut self, op0: A, op1: B);
}

impl<'a> KmovqEmitter<KReg, KReg> for Assembler<'a> {
    fn kmovq(&mut self, op0: KReg, op1: KReg) {
        self.emit(KMOVQKK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovqEmitter<KReg, Mem> for Assembler<'a> {
    fn kmovq(&mut self, op0: KReg, op1: Mem) {
        self.emit(KMOVQKM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovqEmitter<Mem, KReg> for Assembler<'a> {
    fn kmovq(&mut self, op0: Mem, op1: KReg) {
        self.emit(KMOVQMK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovqEmitter<KReg, Gpd> for Assembler<'a> {
    fn kmovq(&mut self, op0: KReg, op1: Gpd) {
        self.emit(KMOVQKR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovqEmitter<Gpd, KReg> for Assembler<'a> {
    fn kmovq(&mut self, op0: Gpd, op1: KReg) {
        self.emit(KMOVQRK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `KNOTD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | KReg, KReg |
/// +---+------------+
/// ```
pub trait KnotdEmitter<A, B> {
    fn knotd(&mut self, op0: A, op1: B);
}

impl<'a> KnotdEmitter<KReg, KReg> for Assembler<'a> {
    fn knotd(&mut self, op0: KReg, op1: KReg) {
        self.emit(KNOTDKK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `KNOTQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | KReg, KReg |
/// +---+------------+
/// ```
pub trait KnotqEmitter<A, B> {
    fn knotq(&mut self, op0: A, op1: B);
}

impl<'a> KnotqEmitter<KReg, KReg> for Assembler<'a> {
    fn knotq(&mut self, op0: KReg, op1: KReg) {
        self.emit(KNOTQKK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `KORD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KordEmitter<A, B, C> {
    fn kord(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KordEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kord(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KORDKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KORQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KorqEmitter<A, B, C> {
    fn korq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KorqEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn korq(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KORQKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KORTESTD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | KReg, KReg |
/// +---+------------+
/// ```
pub trait KortestdEmitter<A, B> {
    fn kortestd(&mut self, op0: A, op1: B);
}

impl<'a> KortestdEmitter<KReg, KReg> for Assembler<'a> {
    fn kortestd(&mut self, op0: KReg, op1: KReg) {
        self.emit(
            KORTESTDKK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `KORTESTQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | KReg, KReg |
/// +---+------------+
/// ```
pub trait KortestqEmitter<A, B> {
    fn kortestq(&mut self, op0: A, op1: B);
}

impl<'a> KortestqEmitter<KReg, KReg> for Assembler<'a> {
    fn kortestq(&mut self, op0: KReg, op1: KReg) {
        self.emit(
            KORTESTQKK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `KSHIFTLD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------------+
/// | # | Operands        |
/// +---+-----------------+
/// | 1 | KReg, KReg, Imm |
/// +---+-----------------+
/// ```
pub trait KshiftldEmitter<A, B, C> {
    fn kshiftld(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KshiftldEmitter<KReg, KReg, Imm> for Assembler<'a> {
    fn kshiftld(&mut self, op0: KReg, op1: KReg, op2: Imm) {
        self.emit(
            KSHIFTLDKKI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KSHIFTLQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------------+
/// | # | Operands        |
/// +---+-----------------+
/// | 1 | KReg, KReg, Imm |
/// +---+-----------------+
/// ```
pub trait KshiftlqEmitter<A, B, C> {
    fn kshiftlq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KshiftlqEmitter<KReg, KReg, Imm> for Assembler<'a> {
    fn kshiftlq(&mut self, op0: KReg, op1: KReg, op2: Imm) {
        self.emit(
            KSHIFTLQKKI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KSHIFTRD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------------+
/// | # | Operands        |
/// +---+-----------------+
/// | 1 | KReg, KReg, Imm |
/// +---+-----------------+
/// ```
pub trait KshiftrdEmitter<A, B, C> {
    fn kshiftrd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KshiftrdEmitter<KReg, KReg, Imm> for Assembler<'a> {
    fn kshiftrd(&mut self, op0: KReg, op1: KReg, op2: Imm) {
        self.emit(
            KSHIFTRDKKI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KSHIFTRQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------------+
/// | # | Operands        |
/// +---+-----------------+
/// | 1 | KReg, KReg, Imm |
/// +---+-----------------+
/// ```
pub trait KshiftrqEmitter<A, B, C> {
    fn kshiftrq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KshiftrqEmitter<KReg, KReg, Imm> for Assembler<'a> {
    fn kshiftrq(&mut self, op0: KReg, op1: KReg, op2: Imm) {
        self.emit(
            KSHIFTRQKKI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KTESTD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | KReg, KReg |
/// +---+------------+
/// ```
pub trait KtestdEmitter<A, B> {
    fn ktestd(&mut self, op0: A, op1: B);
}

impl<'a> KtestdEmitter<KReg, KReg> for Assembler<'a> {
    fn ktestd(&mut self, op0: KReg, op1: KReg) {
        self.emit(KTESTDKK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `KTESTQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | KReg, KReg |
/// +---+------------+
/// ```
pub trait KtestqEmitter<A, B> {
    fn ktestq(&mut self, op0: A, op1: B);
}

impl<'a> KtestqEmitter<KReg, KReg> for Assembler<'a> {
    fn ktestq(&mut self, op0: KReg, op1: KReg) {
        self.emit(KTESTQKK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `KUNPCKDQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KunpckdqEmitter<A, B, C> {
    fn kunpckdq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KunpckdqEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kunpckdq(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KUNPCKDQKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KUNPCKWD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KunpckwdEmitter<A, B, C> {
    fn kunpckwd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KunpckwdEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kunpckwd(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KUNPCKWDKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KXNORD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KxnordEmitter<A, B, C> {
    fn kxnord(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KxnordEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kxnord(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KXNORDKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KXNORQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KxnorqEmitter<A, B, C> {
    fn kxnorq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KxnorqEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kxnorq(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KXNORQKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KXORD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KxordEmitter<A, B, C> {
    fn kxord(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KxordEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kxord(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KXORDKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `KXORQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KxorqEmitter<A, B, C> {
    fn kxorq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KxorqEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kxorq(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(
            KXORQKKK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VDBPSADBW`.
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
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VdbpsadbwEmitter<A, B, C, D> {
    fn vdbpsadbw(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VdbpsadbwEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vdbpsadbw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VDBPSADBW128RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vdbpsadbw(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VDBPSADBW128RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vdbpsadbw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VDBPSADBW256RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vdbpsadbw(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VDBPSADBW256RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vdbpsadbw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VDBPSADBW512RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vdbpsadbw(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VDBPSADBW512RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VDBPSADBW_MASK`.
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
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VdbpsadbwMaskEmitter<A, B, C, D> {
    fn vdbpsadbw_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VdbpsadbwMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vdbpsadbw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VDBPSADBW128RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vdbpsadbw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VDBPSADBW128RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwMaskEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vdbpsadbw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VDBPSADBW256RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwMaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vdbpsadbw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VDBPSADBW256RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwMaskEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vdbpsadbw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VDBPSADBW512RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwMaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vdbpsadbw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VDBPSADBW512RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VDBPSADBW_MASKZ`.
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
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VdbpsadbwMaskzEmitter<A, B, C, D> {
    fn vdbpsadbw_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VdbpsadbwMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vdbpsadbw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VDBPSADBW128RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vdbpsadbw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VDBPSADBW128RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwMaskzEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vdbpsadbw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VDBPSADBW256RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwMaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vdbpsadbw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VDBPSADBW256RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwMaskzEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vdbpsadbw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VDBPSADBW512RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VdbpsadbwMaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vdbpsadbw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VDBPSADBW512RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VMOVDQU16`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Mem, Zmm |
/// | 4 | Xmm, Mem |
/// | 5 | Xmm, Xmm |
/// | 6 | Ymm, Mem |
/// | 7 | Ymm, Ymm |
/// | 8 | Zmm, Mem |
/// | 9 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vmovdqu16Emitter<A, B> {
    fn vmovdqu16(&mut self, op0: A, op1: B);
}

impl<'a> Vmovdqu16Emitter<Xmm, Xmm> for Assembler<'a> {
    fn vmovdqu16(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VMOVDQU16_128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16Emitter<Xmm, Mem> for Assembler<'a> {
    fn vmovdqu16(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VMOVDQU16_128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16Emitter<Ymm, Ymm> for Assembler<'a> {
    fn vmovdqu16(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VMOVDQU16_256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16Emitter<Ymm, Mem> for Assembler<'a> {
    fn vmovdqu16(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VMOVDQU16_256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16Emitter<Zmm, Zmm> for Assembler<'a> {
    fn vmovdqu16(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VMOVDQU16_512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16Emitter<Zmm, Mem> for Assembler<'a> {
    fn vmovdqu16(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VMOVDQU16_512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16Emitter<Mem, Xmm> for Assembler<'a> {
    fn vmovdqu16(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VMOVDQU16_128MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16Emitter<Mem, Ymm> for Assembler<'a> {
    fn vmovdqu16(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VMOVDQU16_256MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16Emitter<Mem, Zmm> for Assembler<'a> {
    fn vmovdqu16(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VMOVDQU16_512MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VMOVDQU16_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Mem, Zmm |
/// | 4 | Xmm, Mem |
/// | 5 | Xmm, Xmm |
/// | 6 | Ymm, Mem |
/// | 7 | Ymm, Ymm |
/// | 8 | Zmm, Mem |
/// | 9 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vmovdqu16MaskEmitter<A, B> {
    fn vmovdqu16_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vmovdqu16MaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vmovdqu16_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VMOVDQU16_128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vmovdqu16_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VMOVDQU16_128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vmovdqu16_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VMOVDQU16_256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vmovdqu16_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VMOVDQU16_256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vmovdqu16_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VMOVDQU16_512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vmovdqu16_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VMOVDQU16_512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskEmitter<Mem, Xmm> for Assembler<'a> {
    fn vmovdqu16_mask(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VMOVDQU16_128MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskEmitter<Mem, Ymm> for Assembler<'a> {
    fn vmovdqu16_mask(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VMOVDQU16_256MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskEmitter<Mem, Zmm> for Assembler<'a> {
    fn vmovdqu16_mask(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VMOVDQU16_512MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VMOVDQU16_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vmovdqu16MaskzEmitter<A, B> {
    fn vmovdqu16_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vmovdqu16MaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vmovdqu16_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VMOVDQU16_128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vmovdqu16_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VMOVDQU16_128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vmovdqu16_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VMOVDQU16_256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vmovdqu16_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VMOVDQU16_256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vmovdqu16_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VMOVDQU16_512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu16MaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vmovdqu16_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VMOVDQU16_512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VMOVDQU8`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Mem, Zmm |
/// | 4 | Xmm, Mem |
/// | 5 | Xmm, Xmm |
/// | 6 | Ymm, Mem |
/// | 7 | Ymm, Ymm |
/// | 8 | Zmm, Mem |
/// | 9 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vmovdqu8Emitter<A, B> {
    fn vmovdqu8(&mut self, op0: A, op1: B);
}

impl<'a> Vmovdqu8Emitter<Xmm, Xmm> for Assembler<'a> {
    fn vmovdqu8(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VMOVDQU8_128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8Emitter<Xmm, Mem> for Assembler<'a> {
    fn vmovdqu8(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VMOVDQU8_128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8Emitter<Ymm, Ymm> for Assembler<'a> {
    fn vmovdqu8(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VMOVDQU8_256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8Emitter<Ymm, Mem> for Assembler<'a> {
    fn vmovdqu8(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VMOVDQU8_256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8Emitter<Zmm, Zmm> for Assembler<'a> {
    fn vmovdqu8(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VMOVDQU8_512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8Emitter<Zmm, Mem> for Assembler<'a> {
    fn vmovdqu8(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VMOVDQU8_512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8Emitter<Mem, Xmm> for Assembler<'a> {
    fn vmovdqu8(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VMOVDQU8_128MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8Emitter<Mem, Ymm> for Assembler<'a> {
    fn vmovdqu8(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VMOVDQU8_256MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8Emitter<Mem, Zmm> for Assembler<'a> {
    fn vmovdqu8(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VMOVDQU8_512MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VMOVDQU8_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Mem, Zmm |
/// | 4 | Xmm, Mem |
/// | 5 | Xmm, Xmm |
/// | 6 | Ymm, Mem |
/// | 7 | Ymm, Ymm |
/// | 8 | Zmm, Mem |
/// | 9 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vmovdqu8MaskEmitter<A, B> {
    fn vmovdqu8_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vmovdqu8MaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vmovdqu8_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VMOVDQU8_128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vmovdqu8_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VMOVDQU8_128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vmovdqu8_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VMOVDQU8_256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vmovdqu8_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VMOVDQU8_256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vmovdqu8_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VMOVDQU8_512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vmovdqu8_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VMOVDQU8_512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskEmitter<Mem, Xmm> for Assembler<'a> {
    fn vmovdqu8_mask(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VMOVDQU8_128MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskEmitter<Mem, Ymm> for Assembler<'a> {
    fn vmovdqu8_mask(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VMOVDQU8_256MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskEmitter<Mem, Zmm> for Assembler<'a> {
    fn vmovdqu8_mask(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VMOVDQU8_512MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VMOVDQU8_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vmovdqu8MaskzEmitter<A, B> {
    fn vmovdqu8_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vmovdqu8MaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vmovdqu8_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VMOVDQU8_128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vmovdqu8_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VMOVDQU8_128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vmovdqu8_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VMOVDQU8_256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vmovdqu8_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VMOVDQU8_256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vmovdqu8_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VMOVDQU8_512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vmovdqu8MaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vmovdqu8_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VMOVDQU8_512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPABSB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpabsbEmitter<A, B> {
    fn vpabsb(&mut self, op0: A, op1: B);
}

impl<'a> VpabsbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpabsb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPABSB128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpabsb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPABSB128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpabsb(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPABSB256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpabsb(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPABSB256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpabsb(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPABSB512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpabsb(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPABSB512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPABSB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpabsbMaskEmitter<A, B> {
    fn vpabsb_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpabsbMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpabsb_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPABSB128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpabsb_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPABSB128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpabsb_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPABSB256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpabsb_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPABSB256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpabsb_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPABSB512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpabsb_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPABSB512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPABSB_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpabsbMaskzEmitter<A, B> {
    fn vpabsb_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpabsbMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpabsb_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPABSB128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpabsb_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPABSB128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpabsb_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPABSB256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpabsb_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPABSB256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpabsb_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPABSB512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabsbMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpabsb_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPABSB512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPABSW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpabswEmitter<A, B> {
    fn vpabsw(&mut self, op0: A, op1: B);
}

impl<'a> VpabswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpabsw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPABSW128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpabsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPABSW128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpabsw(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPABSW256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpabsw(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPABSW256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpabsw(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPABSW512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpabsw(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPABSW512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPABSW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpabswMaskEmitter<A, B> {
    fn vpabsw_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpabswMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpabsw_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPABSW128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpabsw_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPABSW128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpabsw_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPABSW256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpabsw_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPABSW256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpabsw_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPABSW512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpabsw_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPABSW512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPABSW_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpabswMaskzEmitter<A, B> {
    fn vpabsw_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpabswMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpabsw_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPABSW128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpabsw_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPABSW128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpabsw_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPABSW256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpabsw_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPABSW256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpabsw_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPABSW512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpabswMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpabsw_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPABSW512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPACKSSDW`.
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
pub trait VpackssdwEmitter<A, B, C> {
    fn vpackssdw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpackssdwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpackssdw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKSSDW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpackssdw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKSSDW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpackssdw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKSSDW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpackssdw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKSSDW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpackssdw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKSSDW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpackssdw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKSSDW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPACKSSDW_MASK`.
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
pub trait VpackssdwMaskEmitter<A, B, C> {
    fn vpackssdw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpackssdwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpackssdw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKSSDW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpackssdw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKSSDW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpackssdw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKSSDW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpackssdw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKSSDW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpackssdw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKSSDW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpackssdw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKSSDW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPACKSSDW_MASKZ`.
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
pub trait VpackssdwMaskzEmitter<A, B, C> {
    fn vpackssdw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpackssdwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpackssdw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKSSDW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpackssdw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKSSDW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpackssdw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKSSDW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpackssdw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKSSDW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpackssdw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKSSDW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackssdwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpackssdw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKSSDW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPACKSSWB`.
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
pub trait VpacksswbEmitter<A, B, C> {
    fn vpacksswb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpacksswbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpacksswb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKSSWB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpacksswb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKSSWB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpacksswb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKSSWB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpacksswb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKSSWB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpacksswb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKSSWB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpacksswb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKSSWB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPACKSSWB_MASK`.
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
pub trait VpacksswbMaskEmitter<A, B, C> {
    fn vpacksswb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpacksswbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpacksswb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKSSWB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpacksswb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKSSWB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpacksswb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKSSWB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpacksswb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKSSWB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpacksswb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKSSWB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpacksswb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKSSWB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPACKSSWB_MASKZ`.
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
pub trait VpacksswbMaskzEmitter<A, B, C> {
    fn vpacksswb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpacksswbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpacksswb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKSSWB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpacksswb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKSSWB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpacksswb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKSSWB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpacksswb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKSSWB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpacksswb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKSSWB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpacksswbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpacksswb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKSSWB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPACKUSDW`.
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
pub trait VpackusdwEmitter<A, B, C> {
    fn vpackusdw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpackusdwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpackusdw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKUSDW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpackusdw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKUSDW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpackusdw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKUSDW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpackusdw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKUSDW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpackusdw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKUSDW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpackusdw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKUSDW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPACKUSDW_MASK`.
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
pub trait VpackusdwMaskEmitter<A, B, C> {
    fn vpackusdw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpackusdwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpackusdw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKUSDW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpackusdw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKUSDW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpackusdw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKUSDW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpackusdw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKUSDW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpackusdw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKUSDW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpackusdw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKUSDW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPACKUSDW_MASKZ`.
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
pub trait VpackusdwMaskzEmitter<A, B, C> {
    fn vpackusdw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpackusdwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpackusdw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKUSDW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpackusdw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKUSDW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpackusdw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKUSDW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpackusdw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKUSDW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpackusdw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKUSDW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackusdwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpackusdw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKUSDW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPACKUSWB`.
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
pub trait VpackuswbEmitter<A, B, C> {
    fn vpackuswb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpackuswbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpackuswb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKUSWB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpackuswb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKUSWB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpackuswb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKUSWB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpackuswb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKUSWB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpackuswb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKUSWB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpackuswb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKUSWB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPACKUSWB_MASK`.
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
pub trait VpackuswbMaskEmitter<A, B, C> {
    fn vpackuswb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpackuswbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpackuswb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKUSWB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpackuswb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKUSWB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpackuswb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKUSWB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpackuswb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKUSWB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpackuswb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKUSWB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpackuswb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKUSWB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPACKUSWB_MASKZ`.
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
pub trait VpackuswbMaskzEmitter<A, B, C> {
    fn vpackuswb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpackuswbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpackuswb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPACKUSWB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpackuswb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPACKUSWB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpackuswb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPACKUSWB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpackuswb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPACKUSWB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpackuswb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPACKUSWB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpackuswbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpackuswb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPACKUSWB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDB`.
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
pub trait VpaddbEmitter<A, B, C> {
    fn vpaddb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDB_MASK`.
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
pub trait VpaddbMaskEmitter<A, B, C> {
    fn vpaddb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDB_MASKZ`.
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
pub trait VpaddbMaskzEmitter<A, B, C> {
    fn vpaddb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDSB`.
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
pub trait VpaddsbEmitter<A, B, C> {
    fn vpaddsb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddsbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddsb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDSB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddsb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDSB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddsb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDSB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddsb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDSB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddsb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDSB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddsb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDSB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDSB_MASK`.
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
pub trait VpaddsbMaskEmitter<A, B, C> {
    fn vpaddsb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddsbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddsb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDSB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddsb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDSB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddsb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDSB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddsb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDSB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddsb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDSB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddsb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDSB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDSB_MASKZ`.
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
pub trait VpaddsbMaskzEmitter<A, B, C> {
    fn vpaddsb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddsbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddsb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDSB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddsb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDSB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddsb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDSB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddsb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDSB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddsb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDSB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddsbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddsb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDSB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDSW`.
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
pub trait VpaddswEmitter<A, B, C> {
    fn vpaddsw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddswEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddsw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDSW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddsw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDSW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddsw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDSW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddsw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDSW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddsw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDSW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddsw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDSW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDSW_MASK`.
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
pub trait VpaddswMaskEmitter<A, B, C> {
    fn vpaddsw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddswMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDSW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDSW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDSW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDSW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDSW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDSW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDSW_MASKZ`.
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
pub trait VpaddswMaskzEmitter<A, B, C> {
    fn vpaddsw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddswMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDSW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDSW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDSW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDSW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDSW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddswMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDSW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDUSB`.
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
pub trait VpaddusbEmitter<A, B, C> {
    fn vpaddusb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddusbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddusb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDUSB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddusb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDUSB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddusb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDUSB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddusb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDUSB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddusb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDUSB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddusb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDUSB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDUSB_MASK`.
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
pub trait VpaddusbMaskEmitter<A, B, C> {
    fn vpaddusb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddusbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddusb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDUSB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddusb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDUSB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddusb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDUSB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddusb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDUSB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddusb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDUSB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddusb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDUSB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDUSB_MASKZ`.
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
pub trait VpaddusbMaskzEmitter<A, B, C> {
    fn vpaddusb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddusbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddusb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDUSB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddusb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDUSB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddusb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDUSB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddusb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDUSB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddusb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDUSB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddusbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddusb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDUSB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDUSW`.
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
pub trait VpadduswEmitter<A, B, C> {
    fn vpaddusw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpadduswEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddusw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDUSW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddusw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDUSW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddusw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDUSW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddusw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDUSW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddusw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDUSW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddusw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDUSW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDUSW_MASK`.
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
pub trait VpadduswMaskEmitter<A, B, C> {
    fn vpaddusw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpadduswMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddusw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDUSW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddusw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDUSW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddusw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDUSW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddusw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDUSW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddusw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDUSW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddusw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDUSW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDUSW_MASKZ`.
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
pub trait VpadduswMaskzEmitter<A, B, C> {
    fn vpaddusw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpadduswMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddusw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDUSW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddusw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDUSW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddusw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDUSW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddusw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDUSW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddusw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDUSW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpadduswMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddusw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDUSW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDW`.
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
pub trait VpaddwEmitter<A, B, C> {
    fn vpaddw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDW_MASK`.
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
pub trait VpaddwMaskEmitter<A, B, C> {
    fn vpaddw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPADDW_MASKZ`.
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
pub trait VpaddwMaskzEmitter<A, B, C> {
    fn vpaddw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpaddwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpaddw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPADDW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpaddw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPADDW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpaddw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPADDW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpaddw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPADDW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpaddw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPADDW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpaddwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpaddw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPADDW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPALIGNR`.
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
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VpalignrEmitter<A, B, C, D> {
    fn vpalignr(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpalignrEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpalignr(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPALIGNR128RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpalignr(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPALIGNR128RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpalignr(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPALIGNR256RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpalignr(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPALIGNR256RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpalignr(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPALIGNR512RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpalignr(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPALIGNR512RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPALIGNR_MASK`.
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
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VpalignrMaskEmitter<A, B, C, D> {
    fn vpalignr_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpalignrMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpalignr_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPALIGNR128RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpalignr_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPALIGNR128RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrMaskEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpalignr_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPALIGNR256RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrMaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpalignr_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPALIGNR256RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrMaskEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpalignr_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPALIGNR512RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrMaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpalignr_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPALIGNR512RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPALIGNR_MASKZ`.
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
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VpalignrMaskzEmitter<A, B, C, D> {
    fn vpalignr_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpalignrMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpalignr_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPALIGNR128RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpalignr_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPALIGNR128RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrMaskzEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpalignr_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPALIGNR256RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrMaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpalignr_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPALIGNR256RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrMaskzEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpalignr_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPALIGNR512RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpalignrMaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpalignr_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPALIGNR512RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPAVGB`.
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
pub trait VpavgbEmitter<A, B, C> {
    fn vpavgb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpavgbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpavgb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPAVGB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpavgb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPAVGB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpavgb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPAVGB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpavgb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPAVGB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpavgb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPAVGB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpavgb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPAVGB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPAVGB_MASK`.
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
pub trait VpavgbMaskEmitter<A, B, C> {
    fn vpavgb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpavgbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpavgb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPAVGB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpavgb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPAVGB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpavgb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPAVGB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpavgb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPAVGB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpavgb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPAVGB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpavgb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPAVGB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPAVGB_MASKZ`.
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
pub trait VpavgbMaskzEmitter<A, B, C> {
    fn vpavgb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpavgbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpavgb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPAVGB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpavgb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPAVGB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpavgb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPAVGB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpavgb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPAVGB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpavgb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPAVGB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpavgb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPAVGB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPAVGW`.
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
pub trait VpavgwEmitter<A, B, C> {
    fn vpavgw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpavgwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpavgw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPAVGW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpavgw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPAVGW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpavgw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPAVGW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpavgw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPAVGW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpavgw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPAVGW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpavgw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPAVGW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPAVGW_MASK`.
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
pub trait VpavgwMaskEmitter<A, B, C> {
    fn vpavgw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpavgwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpavgw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPAVGW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpavgw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPAVGW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpavgw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPAVGW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpavgw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPAVGW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpavgw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPAVGW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpavgw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPAVGW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPAVGW_MASKZ`.
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
pub trait VpavgwMaskzEmitter<A, B, C> {
    fn vpavgw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpavgwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpavgw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPAVGW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpavgw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPAVGW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpavgw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPAVGW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpavgw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPAVGW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpavgw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPAVGW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpavgwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpavgw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPAVGW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPBLENDMB`.
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
pub trait VpblendmbEmitter<A, B, C> {
    fn vpblendmb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpblendmbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpblendmb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPBLENDMB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpblendmb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPBLENDMB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpblendmb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPBLENDMB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpblendmb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPBLENDMB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpblendmb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPBLENDMB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpblendmb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPBLENDMB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPBLENDMB_MASK`.
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
pub trait VpblendmbMaskEmitter<A, B, C> {
    fn vpblendmb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpblendmbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpblendmb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPBLENDMB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpblendmb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPBLENDMB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpblendmb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPBLENDMB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpblendmb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPBLENDMB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpblendmb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPBLENDMB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpblendmb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPBLENDMB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPBLENDMB_MASKZ`.
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
pub trait VpblendmbMaskzEmitter<A, B, C> {
    fn vpblendmb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpblendmbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpblendmb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPBLENDMB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpblendmb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPBLENDMB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpblendmb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPBLENDMB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpblendmb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPBLENDMB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpblendmb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPBLENDMB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpblendmb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPBLENDMB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPBLENDMW`.
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
pub trait VpblendmwEmitter<A, B, C> {
    fn vpblendmw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpblendmwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpblendmw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPBLENDMW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpblendmw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPBLENDMW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpblendmw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPBLENDMW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpblendmw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPBLENDMW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpblendmw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPBLENDMW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpblendmw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPBLENDMW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPBLENDMW_MASK`.
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
pub trait VpblendmwMaskEmitter<A, B, C> {
    fn vpblendmw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpblendmwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpblendmw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPBLENDMW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpblendmw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPBLENDMW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpblendmw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPBLENDMW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpblendmw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPBLENDMW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpblendmw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPBLENDMW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpblendmw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPBLENDMW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPBLENDMW_MASKZ`.
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
pub trait VpblendmwMaskzEmitter<A, B, C> {
    fn vpblendmw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpblendmwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpblendmw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPBLENDMW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpblendmw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPBLENDMW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpblendmw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPBLENDMW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpblendmw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPBLENDMW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpblendmw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPBLENDMW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpblendmwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpblendmw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPBLENDMW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPBROADCASTB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait VpbroadcastbEmitter<A, B> {
    fn vpbroadcastb(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpbroadcastb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPBROADCASTB128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpbroadcastb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPBROADCASTB128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vpbroadcastb(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(
            VPBROADCASTB256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpbroadcastb(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPBROADCASTB256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbEmitter<Zmm, Xmm> for Assembler<'a> {
    fn vpbroadcastb(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(
            VPBROADCASTB512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpbroadcastb(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPBROADCASTB512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTB_GP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Gpd |
/// | 2 | Ymm, Gpd |
/// | 3 | Zmm, Gpd |
/// +---+----------+
/// ```
pub trait VpbroadcastbGpEmitter<A, B> {
    fn vpbroadcastb_gp(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastbGpEmitter<Xmm, Gpd> for Assembler<'a> {
    fn vpbroadcastb_gp(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(
            VPBROADCASTB_GP128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbGpEmitter<Ymm, Gpd> for Assembler<'a> {
    fn vpbroadcastb_gp(&mut self, op0: Ymm, op1: Gpd) {
        self.emit(
            VPBROADCASTB_GP256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbGpEmitter<Zmm, Gpd> for Assembler<'a> {
    fn vpbroadcastb_gp(&mut self, op0: Zmm, op1: Gpd) {
        self.emit(
            VPBROADCASTB_GP512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTB_GP_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Gpd |
/// | 2 | Ymm, Gpd |
/// | 3 | Zmm, Gpd |
/// +---+----------+
/// ```
pub trait VpbroadcastbGpMaskEmitter<A, B> {
    fn vpbroadcastb_gp_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastbGpMaskEmitter<Xmm, Gpd> for Assembler<'a> {
    fn vpbroadcastb_gp_mask(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(
            VPBROADCASTB_GP128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbGpMaskEmitter<Ymm, Gpd> for Assembler<'a> {
    fn vpbroadcastb_gp_mask(&mut self, op0: Ymm, op1: Gpd) {
        self.emit(
            VPBROADCASTB_GP256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbGpMaskEmitter<Zmm, Gpd> for Assembler<'a> {
    fn vpbroadcastb_gp_mask(&mut self, op0: Zmm, op1: Gpd) {
        self.emit(
            VPBROADCASTB_GP512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTB_GP_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Gpd |
/// | 2 | Ymm, Gpd |
/// | 3 | Zmm, Gpd |
/// +---+----------+
/// ```
pub trait VpbroadcastbGpMaskzEmitter<A, B> {
    fn vpbroadcastb_gp_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastbGpMaskzEmitter<Xmm, Gpd> for Assembler<'a> {
    fn vpbroadcastb_gp_maskz(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(
            VPBROADCASTB_GP128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbGpMaskzEmitter<Ymm, Gpd> for Assembler<'a> {
    fn vpbroadcastb_gp_maskz(&mut self, op0: Ymm, op1: Gpd) {
        self.emit(
            VPBROADCASTB_GP256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbGpMaskzEmitter<Zmm, Gpd> for Assembler<'a> {
    fn vpbroadcastb_gp_maskz(&mut self, op0: Zmm, op1: Gpd) {
        self.emit(
            VPBROADCASTB_GP512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait VpbroadcastbMaskEmitter<A, B> {
    fn vpbroadcastb_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastbMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpbroadcastb_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPBROADCASTB128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpbroadcastb_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPBROADCASTB128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbMaskEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vpbroadcastb_mask(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(
            VPBROADCASTB256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpbroadcastb_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPBROADCASTB256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbMaskEmitter<Zmm, Xmm> for Assembler<'a> {
    fn vpbroadcastb_mask(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(
            VPBROADCASTB512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpbroadcastb_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPBROADCASTB512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTB_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait VpbroadcastbMaskzEmitter<A, B> {
    fn vpbroadcastb_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastbMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpbroadcastb_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPBROADCASTB128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpbroadcastb_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPBROADCASTB128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbMaskzEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vpbroadcastb_maskz(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(
            VPBROADCASTB256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpbroadcastb_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPBROADCASTB256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbMaskzEmitter<Zmm, Xmm> for Assembler<'a> {
    fn vpbroadcastb_maskz(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(
            VPBROADCASTB512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastbMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpbroadcastb_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPBROADCASTB512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait VpbroadcastwEmitter<A, B> {
    fn vpbroadcastw(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpbroadcastw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPBROADCASTW128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpbroadcastw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPBROADCASTW128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vpbroadcastw(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(
            VPBROADCASTW256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpbroadcastw(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPBROADCASTW256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwEmitter<Zmm, Xmm> for Assembler<'a> {
    fn vpbroadcastw(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(
            VPBROADCASTW512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpbroadcastw(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPBROADCASTW512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTW_GP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Gpd |
/// | 2 | Ymm, Gpd |
/// | 3 | Zmm, Gpd |
/// +---+----------+
/// ```
pub trait VpbroadcastwGpEmitter<A, B> {
    fn vpbroadcastw_gp(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastwGpEmitter<Xmm, Gpd> for Assembler<'a> {
    fn vpbroadcastw_gp(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(
            VPBROADCASTW_GP128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwGpEmitter<Ymm, Gpd> for Assembler<'a> {
    fn vpbroadcastw_gp(&mut self, op0: Ymm, op1: Gpd) {
        self.emit(
            VPBROADCASTW_GP256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwGpEmitter<Zmm, Gpd> for Assembler<'a> {
    fn vpbroadcastw_gp(&mut self, op0: Zmm, op1: Gpd) {
        self.emit(
            VPBROADCASTW_GP512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTW_GP_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Gpd |
/// | 2 | Ymm, Gpd |
/// | 3 | Zmm, Gpd |
/// +---+----------+
/// ```
pub trait VpbroadcastwGpMaskEmitter<A, B> {
    fn vpbroadcastw_gp_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastwGpMaskEmitter<Xmm, Gpd> for Assembler<'a> {
    fn vpbroadcastw_gp_mask(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(
            VPBROADCASTW_GP128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwGpMaskEmitter<Ymm, Gpd> for Assembler<'a> {
    fn vpbroadcastw_gp_mask(&mut self, op0: Ymm, op1: Gpd) {
        self.emit(
            VPBROADCASTW_GP256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwGpMaskEmitter<Zmm, Gpd> for Assembler<'a> {
    fn vpbroadcastw_gp_mask(&mut self, op0: Zmm, op1: Gpd) {
        self.emit(
            VPBROADCASTW_GP512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTW_GP_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Gpd |
/// | 2 | Ymm, Gpd |
/// | 3 | Zmm, Gpd |
/// +---+----------+
/// ```
pub trait VpbroadcastwGpMaskzEmitter<A, B> {
    fn vpbroadcastw_gp_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastwGpMaskzEmitter<Xmm, Gpd> for Assembler<'a> {
    fn vpbroadcastw_gp_maskz(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(
            VPBROADCASTW_GP128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwGpMaskzEmitter<Ymm, Gpd> for Assembler<'a> {
    fn vpbroadcastw_gp_maskz(&mut self, op0: Ymm, op1: Gpd) {
        self.emit(
            VPBROADCASTW_GP256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwGpMaskzEmitter<Zmm, Gpd> for Assembler<'a> {
    fn vpbroadcastw_gp_maskz(&mut self, op0: Zmm, op1: Gpd) {
        self.emit(
            VPBROADCASTW_GP512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait VpbroadcastwMaskEmitter<A, B> {
    fn vpbroadcastw_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastwMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpbroadcastw_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPBROADCASTW128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpbroadcastw_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPBROADCASTW128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwMaskEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vpbroadcastw_mask(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(
            VPBROADCASTW256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpbroadcastw_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPBROADCASTW256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwMaskEmitter<Zmm, Xmm> for Assembler<'a> {
    fn vpbroadcastw_mask(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(
            VPBROADCASTW512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpbroadcastw_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPBROADCASTW512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPBROADCASTW_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait VpbroadcastwMaskzEmitter<A, B> {
    fn vpbroadcastw_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpbroadcastwMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpbroadcastw_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPBROADCASTW128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpbroadcastw_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPBROADCASTW128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwMaskzEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vpbroadcastw_maskz(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(
            VPBROADCASTW256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpbroadcastw_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPBROADCASTW256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwMaskzEmitter<Zmm, Xmm> for Assembler<'a> {
    fn vpbroadcastw_maskz(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(
            VPBROADCASTW512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpbroadcastwMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpbroadcastw_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPBROADCASTW512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCMPB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------------+
/// | # | Operands            |
/// +---+---------------------+
/// | 1 | KReg, Xmm, Mem, Imm |
/// | 2 | KReg, Xmm, Xmm, Imm |
/// | 3 | KReg, Ymm, Mem, Imm |
/// | 4 | KReg, Ymm, Ymm, Imm |
/// | 5 | KReg, Zmm, Mem, Imm |
/// | 6 | KReg, Zmm, Zmm, Imm |
/// +---+---------------------+
/// ```
pub trait VpcmpbEmitter<A, B, C, D> {
    fn vpcmpb(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpcmpbEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpb(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPCMPB128KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpbEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpb(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPB128KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpbEmitter<KReg, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpcmpb(&mut self, op0: KReg, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPCMPB256KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpbEmitter<KReg, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpcmpb(&mut self, op0: KReg, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPB256KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpbEmitter<KReg, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpcmpb(&mut self, op0: KReg, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPCMPB512KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpbEmitter<KReg, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpb(&mut self, op0: KReg, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPB512KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPCMPB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------------+
/// | # | Operands            |
/// +---+---------------------+
/// | 1 | KReg, Xmm, Mem, Imm |
/// | 2 | KReg, Xmm, Xmm, Imm |
/// | 3 | KReg, Ymm, Mem, Imm |
/// | 4 | KReg, Ymm, Ymm, Imm |
/// | 5 | KReg, Zmm, Mem, Imm |
/// | 6 | KReg, Zmm, Zmm, Imm |
/// +---+---------------------+
/// ```
pub trait VpcmpbMaskEmitter<A, B, C, D> {
    fn vpcmpb_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpcmpbMaskEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpb_mask(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPCMPB128KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpbMaskEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpb_mask(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPB128KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpbMaskEmitter<KReg, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpcmpb_mask(&mut self, op0: KReg, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPCMPB256KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpbMaskEmitter<KReg, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpcmpb_mask(&mut self, op0: KReg, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPB256KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpbMaskEmitter<KReg, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpcmpb_mask(&mut self, op0: KReg, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPCMPB512KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpbMaskEmitter<KReg, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpb_mask(&mut self, op0: KReg, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPB512KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPCMPUB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------------+
/// | # | Operands            |
/// +---+---------------------+
/// | 1 | KReg, Xmm, Mem, Imm |
/// | 2 | KReg, Xmm, Xmm, Imm |
/// | 3 | KReg, Ymm, Mem, Imm |
/// | 4 | KReg, Ymm, Ymm, Imm |
/// | 5 | KReg, Zmm, Mem, Imm |
/// | 6 | KReg, Zmm, Zmm, Imm |
/// +---+---------------------+
/// ```
pub trait VpcmpubEmitter<A, B, C, D> {
    fn vpcmpub(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpcmpubEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpub(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPCMPUB128KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpubEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpub(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUB128KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpubEmitter<KReg, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpcmpub(&mut self, op0: KReg, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPCMPUB256KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpubEmitter<KReg, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpcmpub(&mut self, op0: KReg, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUB256KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpubEmitter<KReg, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpcmpub(&mut self, op0: KReg, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPCMPUB512KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpubEmitter<KReg, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpub(&mut self, op0: KReg, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUB512KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPCMPUB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------------+
/// | # | Operands            |
/// +---+---------------------+
/// | 1 | KReg, Xmm, Mem, Imm |
/// | 2 | KReg, Xmm, Xmm, Imm |
/// | 3 | KReg, Ymm, Mem, Imm |
/// | 4 | KReg, Ymm, Ymm, Imm |
/// | 5 | KReg, Zmm, Mem, Imm |
/// | 6 | KReg, Zmm, Zmm, Imm |
/// +---+---------------------+
/// ```
pub trait VpcmpubMaskEmitter<A, B, C, D> {
    fn vpcmpub_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpcmpubMaskEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpub_mask(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPCMPUB128KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpubMaskEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpub_mask(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUB128KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpubMaskEmitter<KReg, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpcmpub_mask(&mut self, op0: KReg, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPCMPUB256KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpubMaskEmitter<KReg, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpcmpub_mask(&mut self, op0: KReg, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUB256KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpubMaskEmitter<KReg, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpcmpub_mask(&mut self, op0: KReg, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPCMPUB512KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpubMaskEmitter<KReg, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpub_mask(&mut self, op0: KReg, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUB512KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPCMPUW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------------+
/// | # | Operands            |
/// +---+---------------------+
/// | 1 | KReg, Xmm, Mem, Imm |
/// | 2 | KReg, Xmm, Xmm, Imm |
/// | 3 | KReg, Ymm, Mem, Imm |
/// | 4 | KReg, Ymm, Ymm, Imm |
/// | 5 | KReg, Zmm, Mem, Imm |
/// | 6 | KReg, Zmm, Zmm, Imm |
/// +---+---------------------+
/// ```
pub trait VpcmpuwEmitter<A, B, C, D> {
    fn vpcmpuw(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpcmpuwEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpuw(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPCMPUW128KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpuwEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpuw(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUW128KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpuwEmitter<KReg, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpcmpuw(&mut self, op0: KReg, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPCMPUW256KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpuwEmitter<KReg, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpcmpuw(&mut self, op0: KReg, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUW256KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpuwEmitter<KReg, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpcmpuw(&mut self, op0: KReg, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPCMPUW512KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpuwEmitter<KReg, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpuw(&mut self, op0: KReg, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUW512KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPCMPUW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------------+
/// | # | Operands            |
/// +---+---------------------+
/// | 1 | KReg, Xmm, Mem, Imm |
/// | 2 | KReg, Xmm, Xmm, Imm |
/// | 3 | KReg, Ymm, Mem, Imm |
/// | 4 | KReg, Ymm, Ymm, Imm |
/// | 5 | KReg, Zmm, Mem, Imm |
/// | 6 | KReg, Zmm, Zmm, Imm |
/// +---+---------------------+
/// ```
pub trait VpcmpuwMaskEmitter<A, B, C, D> {
    fn vpcmpuw_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpcmpuwMaskEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpuw_mask(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPCMPUW128KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpuwMaskEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpuw_mask(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUW128KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpuwMaskEmitter<KReg, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpcmpuw_mask(&mut self, op0: KReg, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPCMPUW256KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpuwMaskEmitter<KReg, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpcmpuw_mask(&mut self, op0: KReg, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUW256KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpuwMaskEmitter<KReg, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpcmpuw_mask(&mut self, op0: KReg, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPCMPUW512KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpuwMaskEmitter<KReg, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpuw_mask(&mut self, op0: KReg, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPUW512KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPCMPW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------------+
/// | # | Operands            |
/// +---+---------------------+
/// | 1 | KReg, Xmm, Mem, Imm |
/// | 2 | KReg, Xmm, Xmm, Imm |
/// | 3 | KReg, Ymm, Mem, Imm |
/// | 4 | KReg, Ymm, Ymm, Imm |
/// | 5 | KReg, Zmm, Mem, Imm |
/// | 6 | KReg, Zmm, Zmm, Imm |
/// +---+---------------------+
/// ```
pub trait VpcmpwEmitter<A, B, C, D> {
    fn vpcmpw(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpcmpwEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpw(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPCMPW128KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpwEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpw(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPW128KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpwEmitter<KReg, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpcmpw(&mut self, op0: KReg, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPCMPW256KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpwEmitter<KReg, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpcmpw(&mut self, op0: KReg, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPW256KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpwEmitter<KReg, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpcmpw(&mut self, op0: KReg, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPCMPW512KRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpwEmitter<KReg, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpw(&mut self, op0: KReg, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPW512KRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPCMPW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------------+
/// | # | Operands            |
/// +---+---------------------+
/// | 1 | KReg, Xmm, Mem, Imm |
/// | 2 | KReg, Xmm, Xmm, Imm |
/// | 3 | KReg, Ymm, Mem, Imm |
/// | 4 | KReg, Ymm, Ymm, Imm |
/// | 5 | KReg, Zmm, Mem, Imm |
/// | 6 | KReg, Zmm, Zmm, Imm |
/// +---+---------------------+
/// ```
pub trait VpcmpwMaskEmitter<A, B, C, D> {
    fn vpcmpw_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpcmpwMaskEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpw_mask(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPCMPW128KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpwMaskEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpw_mask(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPW128KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpwMaskEmitter<KReg, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpcmpw_mask(&mut self, op0: KReg, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPCMPW256KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpwMaskEmitter<KReg, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpcmpw_mask(&mut self, op0: KReg, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPW256KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpwMaskEmitter<KReg, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpcmpw_mask(&mut self, op0: KReg, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPCMPW512KRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpcmpwMaskEmitter<KReg, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpw_mask(&mut self, op0: KReg, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPCMPW512KRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPERMI2W`.
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
pub trait Vpermi2wEmitter<A, B, C> {
    fn vpermi2w(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermi2wEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermi2w(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMI2W128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermi2w(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMI2W128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermi2w(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMI2W256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermi2w(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMI2W256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermi2w(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMI2W512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermi2w(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMI2W512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMI2W_MASK`.
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
pub trait Vpermi2wMaskEmitter<A, B, C> {
    fn vpermi2w_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermi2wMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermi2w_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMI2W128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermi2w_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMI2W128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermi2w_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMI2W256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermi2w_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMI2W256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermi2w_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMI2W512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermi2w_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMI2W512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMI2W_MASKZ`.
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
pub trait Vpermi2wMaskzEmitter<A, B, C> {
    fn vpermi2w_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermi2wMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermi2w_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMI2W128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermi2w_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMI2W128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermi2w_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMI2W256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermi2w_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMI2W256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermi2w_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMI2W512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2wMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermi2w_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMI2W512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMT2W`.
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
pub trait Vpermt2wEmitter<A, B, C> {
    fn vpermt2w(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermt2wEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermt2w(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMT2W128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermt2w(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMT2W128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermt2w(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMT2W256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermt2w(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMT2W256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermt2w(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMT2W512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermt2w(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMT2W512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMT2W_MASK`.
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
pub trait Vpermt2wMaskEmitter<A, B, C> {
    fn vpermt2w_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermt2wMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermt2w_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMT2W128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermt2w_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMT2W128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermt2w_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMT2W256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermt2w_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMT2W256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermt2w_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMT2W512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermt2w_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMT2W512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMT2W_MASKZ`.
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
pub trait Vpermt2wMaskzEmitter<A, B, C> {
    fn vpermt2w_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermt2wMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermt2w_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMT2W128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermt2w_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMT2W128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermt2w_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMT2W256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermt2w_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMT2W256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermt2w_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMT2W512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2wMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermt2w_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMT2W512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMW`.
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
pub trait VpermwEmitter<A, B, C> {
    fn vpermw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpermwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMW_MASK`.
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
pub trait VpermwMaskEmitter<A, B, C> {
    fn vpermw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpermwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMW_MASKZ`.
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
pub trait VpermwMaskzEmitter<A, B, C> {
    fn vpermw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpermwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPEXTRB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Xmm, Imm |
/// | 2 | Mem, Xmm, Imm |
/// +---+---------------+
/// ```
pub trait VpextrbEmitter<A, B, C> {
    fn vpextrb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpextrbEmitter<Mem, Xmm, Imm> for Assembler<'a> {
    fn vpextrb(&mut self, op0: Mem, op1: Xmm, op2: Imm) {
        self.emit(
            VPEXTRBMRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpextrbEmitter<Gpd, Xmm, Imm> for Assembler<'a> {
    fn vpextrb(&mut self, op0: Gpd, op1: Xmm, op2: Imm) {
        self.emit(
            VPEXTRBRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPEXTRW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Xmm, Imm |
/// | 2 | Mem, Xmm, Imm |
/// +---+---------------+
/// ```
pub trait VpextrwEmitter<A, B, C> {
    fn vpextrw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpextrwEmitter<Gpd, Xmm, Imm> for Assembler<'a> {
    fn vpextrw(&mut self, op0: Gpd, op1: Xmm, op2: Imm) {
        self.emit(
            VPEXTRWRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpextrwEmitter<Mem, Xmm, Imm> for Assembler<'a> {
    fn vpextrw(&mut self, op0: Mem, op1: Xmm, op2: Imm) {
        self.emit(
            VPEXTRWMRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPINSRB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Gpd, Imm |
/// | 2 | Xmm, Xmm, Mem, Imm |
/// +---+--------------------+
/// ```
pub trait VpinsrbEmitter<A, B, C, D> {
    fn vpinsrb(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpinsrbEmitter<Xmm, Xmm, Gpd, Imm> for Assembler<'a> {
    fn vpinsrb(&mut self, op0: Xmm, op1: Xmm, op2: Gpd, op3: Imm) {
        self.emit(
            VPINSRBRRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpinsrbEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpinsrb(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPINSRBRRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPINSRW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Gpd, Imm |
/// | 2 | Xmm, Xmm, Mem, Imm |
/// +---+--------------------+
/// ```
pub trait VpinsrwEmitter<A, B, C, D> {
    fn vpinsrw(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpinsrwEmitter<Xmm, Xmm, Gpd, Imm> for Assembler<'a> {
    fn vpinsrw(&mut self, op0: Xmm, op1: Xmm, op2: Gpd, op3: Imm) {
        self.emit(
            VPINSRWRRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpinsrwEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpinsrw(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPINSRWRRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPMADDUBSW`.
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
pub trait VpmaddubswEmitter<A, B, C> {
    fn vpmaddubsw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaddubswEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaddubsw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADDUBSW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaddubsw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADDUBSW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaddubsw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADDUBSW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaddubsw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADDUBSW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaddubsw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADDUBSW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaddubsw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADDUBSW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMADDUBSW_MASK`.
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
pub trait VpmaddubswMaskEmitter<A, B, C> {
    fn vpmaddubsw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaddubswMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaddubsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADDUBSW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaddubsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADDUBSW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaddubsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADDUBSW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaddubsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADDUBSW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaddubsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADDUBSW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaddubsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADDUBSW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMADDUBSW_MASKZ`.
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
pub trait VpmaddubswMaskzEmitter<A, B, C> {
    fn vpmaddubsw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaddubswMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaddubsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADDUBSW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaddubsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADDUBSW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaddubsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADDUBSW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaddubsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADDUBSW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaddubsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADDUBSW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddubswMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaddubsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADDUBSW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMADDWD`.
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
pub trait VpmaddwdEmitter<A, B, C> {
    fn vpmaddwd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaddwdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaddwd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADDWD128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaddwd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADDWD128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaddwd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADDWD256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaddwd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADDWD256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaddwd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADDWD512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaddwd(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADDWD512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMADDWD_MASK`.
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
pub trait VpmaddwdMaskEmitter<A, B, C> {
    fn vpmaddwd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaddwdMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaddwd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADDWD128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaddwd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADDWD128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaddwd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADDWD256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaddwd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADDWD256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaddwd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADDWD512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaddwd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADDWD512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMADDWD_MASKZ`.
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
pub trait VpmaddwdMaskzEmitter<A, B, C> {
    fn vpmaddwd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaddwdMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaddwd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMADDWD128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaddwd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMADDWD128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaddwd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMADDWD256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaddwd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMADDWD256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaddwd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMADDWD512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaddwdMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaddwd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMADDWD512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXSB`.
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
pub trait VpmaxsbEmitter<A, B, C> {
    fn vpmaxsb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxsbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxsb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXSB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxsb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXSB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxsb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXSB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxsb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXSB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxsb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXSB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxsb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXSB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXSB_MASK`.
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
pub trait VpmaxsbMaskEmitter<A, B, C> {
    fn vpmaxsb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxsbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxsb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXSB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxsb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXSB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxsb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXSB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxsb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXSB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxsb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXSB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxsb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXSB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXSB_MASKZ`.
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
pub trait VpmaxsbMaskzEmitter<A, B, C> {
    fn vpmaxsb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxsbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxsb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXSB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxsb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXSB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxsb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXSB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxsb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXSB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxsb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXSB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxsbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxsb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXSB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXSW`.
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
pub trait VpmaxswEmitter<A, B, C> {
    fn vpmaxsw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxswEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxsw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXSW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxsw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXSW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxsw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXSW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxsw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXSW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxsw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXSW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxsw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXSW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXSW_MASK`.
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
pub trait VpmaxswMaskEmitter<A, B, C> {
    fn vpmaxsw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxswMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXSW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXSW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXSW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXSW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXSW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXSW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXSW_MASKZ`.
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
pub trait VpmaxswMaskzEmitter<A, B, C> {
    fn vpmaxsw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxswMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXSW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXSW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXSW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXSW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXSW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxswMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXSW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXUB`.
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
pub trait VpmaxubEmitter<A, B, C> {
    fn vpmaxub(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxubEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxub(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXUB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxub(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXUB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxub(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXUB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxub(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXUB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxub(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXUB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxub(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXUB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXUB_MASK`.
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
pub trait VpmaxubMaskEmitter<A, B, C> {
    fn vpmaxub_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxubMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxub_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXUB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxub_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXUB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxub_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXUB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxub_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXUB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxub_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXUB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxub_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXUB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXUB_MASKZ`.
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
pub trait VpmaxubMaskzEmitter<A, B, C> {
    fn vpmaxub_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxubMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxub_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXUB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxub_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXUB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxub_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXUB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxub_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXUB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxub_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXUB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxubMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxub_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXUB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXUW`.
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
pub trait VpmaxuwEmitter<A, B, C> {
    fn vpmaxuw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxuwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxuw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXUW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxuw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXUW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxuw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXUW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxuw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXUW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxuw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXUW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxuw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXUW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXUW_MASK`.
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
pub trait VpmaxuwMaskEmitter<A, B, C> {
    fn vpmaxuw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxuwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxuw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXUW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxuw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXUW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxuw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXUW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxuw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXUW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxuw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXUW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxuw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXUW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMAXUW_MASKZ`.
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
pub trait VpmaxuwMaskzEmitter<A, B, C> {
    fn vpmaxuw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaxuwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmaxuw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMAXUW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaxuw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMAXUW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmaxuw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMAXUW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaxuw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMAXUW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmaxuw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMAXUW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmaxuwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmaxuw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMAXUW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINSB`.
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
pub trait VpminsbEmitter<A, B, C> {
    fn vpminsb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminsbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminsb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINSB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminsb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINSB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminsb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINSB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminsb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINSB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminsb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINSB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminsb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINSB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINSB_MASK`.
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
pub trait VpminsbMaskEmitter<A, B, C> {
    fn vpminsb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminsbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminsb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINSB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminsb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINSB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminsb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINSB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminsb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINSB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminsb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINSB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminsb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINSB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINSB_MASKZ`.
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
pub trait VpminsbMaskzEmitter<A, B, C> {
    fn vpminsb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminsbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminsb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINSB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminsb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINSB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminsb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINSB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminsb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINSB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminsb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINSB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminsbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminsb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINSB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINSW`.
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
pub trait VpminswEmitter<A, B, C> {
    fn vpminsw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminswEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminsw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINSW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminsw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINSW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminsw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINSW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminsw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINSW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminsw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINSW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminsw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINSW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINSW_MASK`.
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
pub trait VpminswMaskEmitter<A, B, C> {
    fn vpminsw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminswMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINSW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINSW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINSW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINSW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINSW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINSW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINSW_MASKZ`.
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
pub trait VpminswMaskzEmitter<A, B, C> {
    fn vpminsw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminswMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINSW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINSW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINSW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINSW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINSW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminswMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINSW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINUB`.
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
pub trait VpminubEmitter<A, B, C> {
    fn vpminub(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminubEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminub(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINUB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminub(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINUB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminub(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINUB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminub(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINUB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminub(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINUB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminub(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINUB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINUB_MASK`.
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
pub trait VpminubMaskEmitter<A, B, C> {
    fn vpminub_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminubMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminub_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINUB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminub_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINUB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminub_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINUB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminub_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINUB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminub_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINUB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminub_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINUB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINUB_MASKZ`.
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
pub trait VpminubMaskzEmitter<A, B, C> {
    fn vpminub_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminubMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminub_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINUB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminub_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINUB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminub_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINUB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminub_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINUB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminub_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINUB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminubMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminub_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINUB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINUW`.
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
pub trait VpminuwEmitter<A, B, C> {
    fn vpminuw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminuwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminuw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINUW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminuw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINUW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminuw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINUW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminuw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINUW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminuw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINUW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminuw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINUW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINUW_MASK`.
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
pub trait VpminuwMaskEmitter<A, B, C> {
    fn vpminuw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminuwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminuw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINUW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminuw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINUW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminuw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINUW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminuw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINUW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminuw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINUW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminuw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINUW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMINUW_MASKZ`.
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
pub trait VpminuwMaskzEmitter<A, B, C> {
    fn vpminuw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpminuwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpminuw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMINUW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpminuw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMINUW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpminuw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMINUW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpminuw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMINUW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpminuw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMINUW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpminuwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpminuw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMINUW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMOVB2M`.
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------+
/// | # | Operands  |
/// +---+-----------+
/// | 1 | KReg, Xmm |
/// | 2 | KReg, Ymm |
/// | 3 | KReg, Zmm |
/// +---+-----------+
/// ```
pub trait Vpmovb2mEmitter<A, B> {
    fn vpmovb2m(&mut self, op0: A, op1: B);
}

impl<'a> Vpmovb2mEmitter<KReg, Xmm> for Assembler<'a> {
    fn vpmovb2m(&mut self, op0: KReg, op1: Xmm) {
        self.emit(
            VPMOVB2M128KR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpmovb2mEmitter<KReg, Ymm> for Assembler<'a> {
    fn vpmovb2m(&mut self, op0: KReg, op1: Ymm) {
        self.emit(
            VPMOVB2M256KR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpmovb2mEmitter<KReg, Zmm> for Assembler<'a> {
    fn vpmovb2m(&mut self, op0: KReg, op1: Zmm) {
        self.emit(
            VPMOVB2M512KR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVM2B`.
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------+
/// | # | Operands  |
/// +---+-----------+
/// | 1 | Xmm, KReg |
/// | 2 | Ymm, KReg |
/// | 3 | Zmm, KReg |
/// +---+-----------+
/// ```
pub trait Vpmovm2bEmitter<A, B> {
    fn vpmovm2b(&mut self, op0: A, op1: B);
}

impl<'a> Vpmovm2bEmitter<Xmm, KReg> for Assembler<'a> {
    fn vpmovm2b(&mut self, op0: Xmm, op1: KReg) {
        self.emit(
            VPMOVM2B128RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpmovm2bEmitter<Ymm, KReg> for Assembler<'a> {
    fn vpmovm2b(&mut self, op0: Ymm, op1: KReg) {
        self.emit(
            VPMOVM2B256RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpmovm2bEmitter<Zmm, KReg> for Assembler<'a> {
    fn vpmovm2b(&mut self, op0: Zmm, op1: KReg) {
        self.emit(
            VPMOVM2B512RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVM2W`.
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------+
/// | # | Operands  |
/// +---+-----------+
/// | 1 | Xmm, KReg |
/// | 2 | Ymm, KReg |
/// | 3 | Zmm, KReg |
/// +---+-----------+
/// ```
pub trait Vpmovm2wEmitter<A, B> {
    fn vpmovm2w(&mut self, op0: A, op1: B);
}

impl<'a> Vpmovm2wEmitter<Xmm, KReg> for Assembler<'a> {
    fn vpmovm2w(&mut self, op0: Xmm, op1: KReg) {
        self.emit(
            VPMOVM2W128RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpmovm2wEmitter<Ymm, KReg> for Assembler<'a> {
    fn vpmovm2w(&mut self, op0: Ymm, op1: KReg) {
        self.emit(
            VPMOVM2W256RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpmovm2wEmitter<Zmm, KReg> for Assembler<'a> {
    fn vpmovm2w(&mut self, op0: Zmm, op1: KReg) {
        self.emit(
            VPMOVM2W512RK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVSWB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Mem, Zmm |
/// | 4 | Xmm, Xmm |
/// | 5 | Xmm, Ymm |
/// | 6 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait VpmovswbEmitter<A, B> {
    fn vpmovswb(&mut self, op0: A, op1: B);
}

impl<'a> VpmovswbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpmovswb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPMOVSWB128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbEmitter<Mem, Xmm> for Assembler<'a> {
    fn vpmovswb(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VPMOVSWB128MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vpmovswb(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VPMOVSWB256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbEmitter<Mem, Ymm> for Assembler<'a> {
    fn vpmovswb(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VPMOVSWB256MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vpmovswb(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VPMOVSWB512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbEmitter<Mem, Zmm> for Assembler<'a> {
    fn vpmovswb(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VPMOVSWB512MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVSWB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Mem, Zmm |
/// | 4 | Xmm, Xmm |
/// | 5 | Xmm, Ymm |
/// | 6 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait VpmovswbMaskEmitter<A, B> {
    fn vpmovswb_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpmovswbMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpmovswb_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPMOVSWB128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbMaskEmitter<Mem, Xmm> for Assembler<'a> {
    fn vpmovswb_mask(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VPMOVSWB128MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbMaskEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vpmovswb_mask(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VPMOVSWB256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbMaskEmitter<Mem, Ymm> for Assembler<'a> {
    fn vpmovswb_mask(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VPMOVSWB256MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbMaskEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vpmovswb_mask(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VPMOVSWB512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbMaskEmitter<Mem, Zmm> for Assembler<'a> {
    fn vpmovswb_mask(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VPMOVSWB512MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVSWB_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Xmm |
/// | 2 | Xmm, Ymm |
/// | 3 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait VpmovswbMaskzEmitter<A, B> {
    fn vpmovswb_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpmovswbMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpmovswb_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPMOVSWB128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbMaskzEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vpmovswb_maskz(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VPMOVSWB256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovswbMaskzEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vpmovswb_maskz(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VPMOVSWB512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVUSWB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Mem, Zmm |
/// | 4 | Xmm, Xmm |
/// | 5 | Xmm, Ymm |
/// | 6 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait VpmovuswbEmitter<A, B> {
    fn vpmovuswb(&mut self, op0: A, op1: B);
}

impl<'a> VpmovuswbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpmovuswb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPMOVUSWB128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbEmitter<Mem, Xmm> for Assembler<'a> {
    fn vpmovuswb(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VPMOVUSWB128MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vpmovuswb(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VPMOVUSWB256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbEmitter<Mem, Ymm> for Assembler<'a> {
    fn vpmovuswb(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VPMOVUSWB256MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vpmovuswb(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VPMOVUSWB512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbEmitter<Mem, Zmm> for Assembler<'a> {
    fn vpmovuswb(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VPMOVUSWB512MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVUSWB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Mem, Zmm |
/// | 4 | Xmm, Xmm |
/// | 5 | Xmm, Ymm |
/// | 6 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait VpmovuswbMaskEmitter<A, B> {
    fn vpmovuswb_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpmovuswbMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpmovuswb_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPMOVUSWB128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbMaskEmitter<Mem, Xmm> for Assembler<'a> {
    fn vpmovuswb_mask(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VPMOVUSWB128MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbMaskEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vpmovuswb_mask(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VPMOVUSWB256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbMaskEmitter<Mem, Ymm> for Assembler<'a> {
    fn vpmovuswb_mask(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VPMOVUSWB256MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbMaskEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vpmovuswb_mask(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VPMOVUSWB512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbMaskEmitter<Mem, Zmm> for Assembler<'a> {
    fn vpmovuswb_mask(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VPMOVUSWB512MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVUSWB_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Xmm |
/// | 2 | Xmm, Ymm |
/// | 3 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait VpmovuswbMaskzEmitter<A, B> {
    fn vpmovuswb_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpmovuswbMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpmovuswb_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPMOVUSWB128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbMaskzEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vpmovuswb_maskz(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VPMOVUSWB256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovuswbMaskzEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vpmovuswb_maskz(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VPMOVUSWB512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVW2M`.
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------+
/// | # | Operands  |
/// +---+-----------+
/// | 1 | KReg, Xmm |
/// | 2 | KReg, Ymm |
/// | 3 | KReg, Zmm |
/// +---+-----------+
/// ```
pub trait Vpmovw2mEmitter<A, B> {
    fn vpmovw2m(&mut self, op0: A, op1: B);
}

impl<'a> Vpmovw2mEmitter<KReg, Xmm> for Assembler<'a> {
    fn vpmovw2m(&mut self, op0: KReg, op1: Xmm) {
        self.emit(
            VPMOVW2M128KR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpmovw2mEmitter<KReg, Ymm> for Assembler<'a> {
    fn vpmovw2m(&mut self, op0: KReg, op1: Ymm) {
        self.emit(
            VPMOVW2M256KR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vpmovw2mEmitter<KReg, Zmm> for Assembler<'a> {
    fn vpmovw2m(&mut self, op0: KReg, op1: Zmm) {
        self.emit(
            VPMOVW2M512KR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVWB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Mem, Zmm |
/// | 4 | Xmm, Xmm |
/// | 5 | Xmm, Ymm |
/// | 6 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait VpmovwbEmitter<A, B> {
    fn vpmovwb(&mut self, op0: A, op1: B);
}

impl<'a> VpmovwbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpmovwb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPMOVWB128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbEmitter<Mem, Xmm> for Assembler<'a> {
    fn vpmovwb(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VPMOVWB128MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vpmovwb(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VPMOVWB256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbEmitter<Mem, Ymm> for Assembler<'a> {
    fn vpmovwb(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VPMOVWB256MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vpmovwb(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VPMOVWB512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbEmitter<Mem, Zmm> for Assembler<'a> {
    fn vpmovwb(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VPMOVWB512MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVWB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Mem, Zmm |
/// | 4 | Xmm, Xmm |
/// | 5 | Xmm, Ymm |
/// | 6 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait VpmovwbMaskEmitter<A, B> {
    fn vpmovwb_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpmovwbMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpmovwb_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPMOVWB128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbMaskEmitter<Mem, Xmm> for Assembler<'a> {
    fn vpmovwb_mask(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VPMOVWB128MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbMaskEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vpmovwb_mask(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VPMOVWB256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbMaskEmitter<Mem, Ymm> for Assembler<'a> {
    fn vpmovwb_mask(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VPMOVWB256MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbMaskEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vpmovwb_mask(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VPMOVWB512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbMaskEmitter<Mem, Zmm> for Assembler<'a> {
    fn vpmovwb_mask(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VPMOVWB512MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMOVWB_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Xmm |
/// | 2 | Xmm, Ymm |
/// | 3 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait VpmovwbMaskzEmitter<A, B> {
    fn vpmovwb_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpmovwbMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpmovwb_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPMOVWB128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbMaskzEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vpmovwb_maskz(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VPMOVWB256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpmovwbMaskzEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vpmovwb_maskz(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VPMOVWB512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPMULHRSW`.
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
pub trait VpmulhrswEmitter<A, B, C> {
    fn vpmulhrsw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulhrswEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulhrsw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULHRSW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulhrsw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULHRSW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulhrsw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULHRSW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulhrsw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULHRSW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulhrsw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULHRSW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulhrsw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULHRSW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULHRSW_MASK`.
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
pub trait VpmulhrswMaskEmitter<A, B, C> {
    fn vpmulhrsw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulhrswMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulhrsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULHRSW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulhrsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULHRSW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulhrsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULHRSW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulhrsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULHRSW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulhrsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULHRSW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulhrsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULHRSW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULHRSW_MASKZ`.
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
pub trait VpmulhrswMaskzEmitter<A, B, C> {
    fn vpmulhrsw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulhrswMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulhrsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULHRSW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulhrsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULHRSW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulhrsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULHRSW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulhrsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULHRSW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulhrsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULHRSW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhrswMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulhrsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULHRSW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULHUW`.
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
pub trait VpmulhuwEmitter<A, B, C> {
    fn vpmulhuw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulhuwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulhuw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULHUW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulhuw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULHUW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulhuw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULHUW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulhuw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULHUW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulhuw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULHUW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulhuw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULHUW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULHUW_MASK`.
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
pub trait VpmulhuwMaskEmitter<A, B, C> {
    fn vpmulhuw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulhuwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulhuw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULHUW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulhuw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULHUW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulhuw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULHUW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulhuw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULHUW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulhuw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULHUW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulhuw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULHUW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULHUW_MASKZ`.
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
pub trait VpmulhuwMaskzEmitter<A, B, C> {
    fn vpmulhuw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulhuwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulhuw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULHUW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulhuw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULHUW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulhuw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULHUW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulhuw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULHUW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulhuw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULHUW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhuwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulhuw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULHUW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULHW`.
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
pub trait VpmulhwEmitter<A, B, C> {
    fn vpmulhw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulhwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulhw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULHW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulhw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULHW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulhw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULHW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulhw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULHW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulhw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULHW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulhw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULHW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULHW_MASK`.
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
pub trait VpmulhwMaskEmitter<A, B, C> {
    fn vpmulhw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulhwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulhw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULHW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulhw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULHW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulhw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULHW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulhw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULHW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulhw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULHW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulhw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULHW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULHW_MASKZ`.
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
pub trait VpmulhwMaskzEmitter<A, B, C> {
    fn vpmulhw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulhwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulhw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULHW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulhw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULHW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulhw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULHW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulhw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULHW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulhw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULHW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmulhwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulhw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULHW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULLW`.
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
pub trait VpmullwEmitter<A, B, C> {
    fn vpmullw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmullwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmullw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULLW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmullw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULLW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmullw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULLW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmullw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULLW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmullw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULLW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmullw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULLW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULLW_MASK`.
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
pub trait VpmullwMaskEmitter<A, B, C> {
    fn vpmullw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmullwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmullw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULLW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmullw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULLW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmullw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULLW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmullw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULLW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmullw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULLW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmullw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULLW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULLW_MASKZ`.
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
pub trait VpmullwMaskzEmitter<A, B, C> {
    fn vpmullw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmullwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmullw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULLW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmullw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULLW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmullw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULLW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmullw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULLW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmullw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULLW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmullwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmullw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULLW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSADBW`.
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
pub trait VpsadbwEmitter<A, B, C> {
    fn vpsadbw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsadbwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsadbw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSADBW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsadbwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsadbw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSADBW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsadbwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsadbw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSADBW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsadbwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsadbw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSADBW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsadbwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsadbw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSADBW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsadbwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsadbw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSADBW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHUFB`.
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
pub trait VpshufbEmitter<A, B, C> {
    fn vpshufb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshufbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshufb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHUFB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshufb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHUFB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshufb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHUFB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshufb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHUFB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshufb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHUFB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshufb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHUFB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHUFB_MASK`.
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
pub trait VpshufbMaskEmitter<A, B, C> {
    fn vpshufb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshufbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshufb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHUFB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshufb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHUFB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshufb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHUFB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshufb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHUFB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshufb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHUFB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshufb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHUFB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHUFB_MASKZ`.
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
pub trait VpshufbMaskzEmitter<A, B, C> {
    fn vpshufb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshufbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshufb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHUFB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshufb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHUFB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshufb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHUFB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshufb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHUFB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshufb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHUFB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshufb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHUFB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHUFHW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VpshufhwEmitter<A, B, C> {
    fn vpshufhw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshufhwEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshufhw(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSHUFHW128RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshufhw(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFHW128RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshufhw(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSHUFHW256RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshufhw(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFHW256RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshufhw(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSHUFHW512RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshufhw(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFHW512RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHUFHW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VpshufhwMaskEmitter<A, B, C> {
    fn vpshufhw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshufhwMaskEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshufhw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSHUFHW128RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwMaskEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshufhw_mask(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFHW128RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwMaskEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshufhw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSHUFHW256RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwMaskEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshufhw_mask(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFHW256RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwMaskEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshufhw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSHUFHW512RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwMaskEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshufhw_mask(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFHW512RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHUFHW_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VpshufhwMaskzEmitter<A, B, C> {
    fn vpshufhw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshufhwMaskzEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshufhw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSHUFHW128RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwMaskzEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshufhw_maskz(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFHW128RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwMaskzEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshufhw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSHUFHW256RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwMaskzEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshufhw_maskz(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFHW256RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwMaskzEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshufhw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSHUFHW512RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshufhwMaskzEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshufhw_maskz(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFHW512RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHUFLW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VpshuflwEmitter<A, B, C> {
    fn vpshuflw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshuflwEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshuflw(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSHUFLW128RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshuflw(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFLW128RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshuflw(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSHUFLW256RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshuflw(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFLW256RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshuflw(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSHUFLW512RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshuflw(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFLW512RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHUFLW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VpshuflwMaskEmitter<A, B, C> {
    fn vpshuflw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshuflwMaskEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshuflw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSHUFLW128RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwMaskEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshuflw_mask(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFLW128RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwMaskEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshuflw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSHUFLW256RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwMaskEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshuflw_mask(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFLW256RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwMaskEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshuflw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSHUFLW512RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwMaskEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshuflw_mask(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFLW512RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHUFLW_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VpshuflwMaskzEmitter<A, B, C> {
    fn vpshuflw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshuflwMaskzEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshuflw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSHUFLW128RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwMaskzEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshuflw_maskz(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFLW128RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwMaskzEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshuflw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSHUFLW256RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwMaskzEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshuflw_maskz(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFLW256RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwMaskzEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshuflw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSHUFLW512RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshuflwMaskzEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshuflw_maskz(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSHUFLW512RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSLLDQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VpslldqEmitter<A, B, C> {
    fn vpslldq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpslldqEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpslldq(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSLLDQ128RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpslldqEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpslldq(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSLLDQ256RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpslldqEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpslldq(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLDQ128RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpslldqEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpslldq(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLDQ256RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpslldqEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpslldq(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSLLDQ512RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpslldqEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpslldq(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLDQ512RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSLLVW`.
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
pub trait VpsllvwEmitter<A, B, C> {
    fn vpsllvw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsllvwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsllvw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSLLVW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsllvw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSLLVW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsllvw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSLLVW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsllvw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSLLVW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsllvw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSLLVW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsllvw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSLLVW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSLLVW_MASK`.
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
pub trait VpsllvwMaskEmitter<A, B, C> {
    fn vpsllvw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsllvwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsllvw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSLLVW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsllvw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSLLVW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsllvw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSLLVW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsllvw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSLLVW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsllvw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSLLVW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsllvw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSLLVW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSLLVW_MASKZ`.
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
pub trait VpsllvwMaskzEmitter<A, B, C> {
    fn vpsllvw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsllvwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsllvw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSLLVW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsllvw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSLLVW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsllvw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSLLVW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsllvw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSLLVW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsllvw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSLLVW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllvwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsllvw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSLLVW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSLLW`.
///
/// Supported operand variants:
///
/// ```text
/// +----+---------------+
/// | #  | Operands      |
/// +----+---------------+
/// | 1  | Xmm, Mem, Imm |
/// | 2  | Xmm, Xmm, Imm |
/// | 3  | Xmm, Xmm, Mem |
/// | 4  | Xmm, Xmm, Xmm |
/// | 5  | Ymm, Mem, Imm |
/// | 6  | Ymm, Ymm, Imm |
/// | 7  | Ymm, Ymm, Mem |
/// | 8  | Ymm, Ymm, Xmm |
/// | 9  | Zmm, Mem, Imm |
/// | 10 | Zmm, Zmm, Imm |
/// | 11 | Zmm, Zmm, Mem |
/// | 12 | Zmm, Zmm, Xmm |
/// +----+---------------+
/// ```
pub trait VpsllwEmitter<A, B, C> {
    fn vpsllw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsllwEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSLLW128RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSLLW256RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSLLW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSLLW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwEmitter<Ymm, Ymm, Xmm> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Ymm, op1: Ymm, op2: Xmm) {
        self.emit(
            VPSLLW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSLLW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLW128RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLW256RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSLLW512RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLW512RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwEmitter<Zmm, Zmm, Xmm> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Zmm, op1: Zmm, op2: Xmm) {
        self.emit(
            VPSLLW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsllw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSLLW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSLLW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +----+---------------+
/// | #  | Operands      |
/// +----+---------------+
/// | 1  | Xmm, Mem, Imm |
/// | 2  | Xmm, Xmm, Imm |
/// | 3  | Xmm, Xmm, Mem |
/// | 4  | Xmm, Xmm, Xmm |
/// | 5  | Ymm, Mem, Imm |
/// | 6  | Ymm, Ymm, Imm |
/// | 7  | Ymm, Ymm, Mem |
/// | 8  | Ymm, Ymm, Xmm |
/// | 9  | Zmm, Mem, Imm |
/// | 10 | Zmm, Zmm, Imm |
/// | 11 | Zmm, Zmm, Mem |
/// | 12 | Zmm, Zmm, Xmm |
/// +----+---------------+
/// ```
pub trait VpsllwMaskEmitter<A, B, C> {
    fn vpsllw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsllwMaskEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSLLW128RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLW128RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSLLW256RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLW256RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSLLW512RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLW512RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSLLW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSLLW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskEmitter<Ymm, Ymm, Xmm> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Xmm) {
        self.emit(
            VPSLLW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSLLW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskEmitter<Zmm, Zmm, Xmm> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Xmm) {
        self.emit(
            VPSLLW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsllw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSLLW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSLLW_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +----+---------------+
/// | #  | Operands      |
/// +----+---------------+
/// | 1  | Xmm, Mem, Imm |
/// | 2  | Xmm, Xmm, Imm |
/// | 3  | Xmm, Xmm, Mem |
/// | 4  | Xmm, Xmm, Xmm |
/// | 5  | Ymm, Mem, Imm |
/// | 6  | Ymm, Ymm, Imm |
/// | 7  | Ymm, Ymm, Mem |
/// | 8  | Ymm, Ymm, Xmm |
/// | 9  | Zmm, Mem, Imm |
/// | 10 | Zmm, Zmm, Imm |
/// | 11 | Zmm, Zmm, Mem |
/// | 12 | Zmm, Zmm, Xmm |
/// +----+---------------+
/// ```
pub trait VpsllwMaskzEmitter<A, B, C> {
    fn vpsllw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsllwMaskzEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSLLW128RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskzEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLW128RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskzEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSLLW256RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskzEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLW256RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskzEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSLLW512RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskzEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSLLW512RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSLLW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSLLW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskzEmitter<Ymm, Ymm, Xmm> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Xmm) {
        self.emit(
            VPSLLW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSLLW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskzEmitter<Zmm, Zmm, Xmm> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Xmm) {
        self.emit(
            VPSLLW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsllwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsllw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSLLW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRAVW`.
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
pub trait VpsravwEmitter<A, B, C> {
    fn vpsravw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsravwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsravw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRAVW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsravw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRAVW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsravw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSRAVW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsravw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRAVW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsravw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSRAVW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsravw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRAVW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRAVW_MASK`.
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
pub trait VpsravwMaskEmitter<A, B, C> {
    fn vpsravw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsravwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsravw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRAVW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsravw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRAVW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsravw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSRAVW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsravw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRAVW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsravw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSRAVW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsravw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRAVW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRAVW_MASKZ`.
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
pub trait VpsravwMaskzEmitter<A, B, C> {
    fn vpsravw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsravwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsravw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRAVW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsravw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRAVW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsravw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSRAVW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsravw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRAVW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsravw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSRAVW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsravwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsravw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRAVW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRAW`.
///
/// Supported operand variants:
///
/// ```text
/// +----+---------------+
/// | #  | Operands      |
/// +----+---------------+
/// | 1  | Xmm, Mem, Imm |
/// | 2  | Xmm, Xmm, Imm |
/// | 3  | Xmm, Xmm, Mem |
/// | 4  | Xmm, Xmm, Xmm |
/// | 5  | Ymm, Mem, Imm |
/// | 6  | Ymm, Ymm, Imm |
/// | 7  | Ymm, Ymm, Mem |
/// | 8  | Ymm, Ymm, Xmm |
/// | 9  | Zmm, Mem, Imm |
/// | 10 | Zmm, Zmm, Imm |
/// | 11 | Zmm, Zmm, Mem |
/// | 12 | Zmm, Zmm, Xmm |
/// +----+---------------+
/// ```
pub trait VpsrawEmitter<A, B, C> {
    fn vpsraw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsrawEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSRAW128RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSRAW256RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRAW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRAW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawEmitter<Ymm, Ymm, Xmm> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Ymm, op1: Ymm, op2: Xmm) {
        self.emit(
            VPSRAW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRAW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRAW128RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRAW256RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSRAW512RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRAW512RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawEmitter<Zmm, Zmm, Xmm> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Zmm, op1: Zmm, op2: Xmm) {
        self.emit(
            VPSRAW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsraw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRAW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRAW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +----+---------------+
/// | #  | Operands      |
/// +----+---------------+
/// | 1  | Xmm, Mem, Imm |
/// | 2  | Xmm, Xmm, Imm |
/// | 3  | Xmm, Xmm, Mem |
/// | 4  | Xmm, Xmm, Xmm |
/// | 5  | Ymm, Mem, Imm |
/// | 6  | Ymm, Ymm, Imm |
/// | 7  | Ymm, Ymm, Mem |
/// | 8  | Ymm, Ymm, Xmm |
/// | 9  | Zmm, Mem, Imm |
/// | 10 | Zmm, Zmm, Imm |
/// | 11 | Zmm, Zmm, Mem |
/// | 12 | Zmm, Zmm, Xmm |
/// +----+---------------+
/// ```
pub trait VpsrawMaskEmitter<A, B, C> {
    fn vpsraw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsrawMaskEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSRAW128RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRAW128RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSRAW256RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRAW256RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSRAW512RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRAW512RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRAW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRAW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskEmitter<Ymm, Ymm, Xmm> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Xmm) {
        self.emit(
            VPSRAW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRAW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskEmitter<Zmm, Zmm, Xmm> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Xmm) {
        self.emit(
            VPSRAW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsraw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRAW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRAW_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +----+---------------+
/// | #  | Operands      |
/// +----+---------------+
/// | 1  | Xmm, Mem, Imm |
/// | 2  | Xmm, Xmm, Imm |
/// | 3  | Xmm, Xmm, Mem |
/// | 4  | Xmm, Xmm, Xmm |
/// | 5  | Ymm, Mem, Imm |
/// | 6  | Ymm, Ymm, Imm |
/// | 7  | Ymm, Ymm, Mem |
/// | 8  | Ymm, Ymm, Xmm |
/// | 9  | Zmm, Mem, Imm |
/// | 10 | Zmm, Zmm, Imm |
/// | 11 | Zmm, Zmm, Mem |
/// | 12 | Zmm, Zmm, Xmm |
/// +----+---------------+
/// ```
pub trait VpsrawMaskzEmitter<A, B, C> {
    fn vpsraw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsrawMaskzEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSRAW128RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskzEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRAW128RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskzEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSRAW256RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskzEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRAW256RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskzEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSRAW512RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskzEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRAW512RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRAW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRAW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskzEmitter<Ymm, Ymm, Xmm> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Xmm) {
        self.emit(
            VPSRAW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRAW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskzEmitter<Zmm, Zmm, Xmm> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Xmm) {
        self.emit(
            VPSRAW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrawMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsraw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRAW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRLDQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VpsrldqEmitter<A, B, C> {
    fn vpsrldq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsrldqEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpsrldq(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSRLDQ128RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrldqEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpsrldq(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSRLDQ256RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrldqEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpsrldq(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLDQ128RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrldqEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpsrldq(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLDQ256RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrldqEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpsrldq(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSRLDQ512RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrldqEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpsrldq(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLDQ512RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRLVW`.
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
pub trait VpsrlvwEmitter<A, B, C> {
    fn vpsrlvw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsrlvwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsrlvw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRLVW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsrlvw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRLVW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsrlvw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSRLVW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsrlvw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRLVW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsrlvw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSRLVW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsrlvw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRLVW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRLVW_MASK`.
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
pub trait VpsrlvwMaskEmitter<A, B, C> {
    fn vpsrlvw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsrlvwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsrlvw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRLVW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsrlvw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRLVW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsrlvw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSRLVW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsrlvw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRLVW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsrlvw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSRLVW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsrlvw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRLVW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRLVW_MASKZ`.
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
pub trait VpsrlvwMaskzEmitter<A, B, C> {
    fn vpsrlvw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsrlvwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsrlvw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRLVW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsrlvw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRLVW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsrlvw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSRLVW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsrlvw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRLVW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsrlvw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSRLVW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlvwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsrlvw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRLVW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRLW`.
///
/// Supported operand variants:
///
/// ```text
/// +----+---------------+
/// | #  | Operands      |
/// +----+---------------+
/// | 1  | Xmm, Mem, Imm |
/// | 2  | Xmm, Xmm, Imm |
/// | 3  | Xmm, Xmm, Mem |
/// | 4  | Xmm, Xmm, Xmm |
/// | 5  | Ymm, Mem, Imm |
/// | 6  | Ymm, Ymm, Imm |
/// | 7  | Ymm, Ymm, Mem |
/// | 8  | Ymm, Ymm, Xmm |
/// | 9  | Zmm, Mem, Imm |
/// | 10 | Zmm, Zmm, Imm |
/// | 11 | Zmm, Zmm, Mem |
/// | 12 | Zmm, Zmm, Xmm |
/// +----+---------------+
/// ```
pub trait VpsrlwEmitter<A, B, C> {
    fn vpsrlw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsrlwEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSRLW128RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSRLW256RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRLW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRLW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwEmitter<Ymm, Ymm, Xmm> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Ymm, op1: Ymm, op2: Xmm) {
        self.emit(
            VPSRLW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRLW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLW128RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLW256RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSRLW512RRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLW512RMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwEmitter<Zmm, Zmm, Xmm> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Zmm, op1: Zmm, op2: Xmm) {
        self.emit(
            VPSRLW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsrlw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRLW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRLW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +----+---------------+
/// | #  | Operands      |
/// +----+---------------+
/// | 1  | Xmm, Mem, Imm |
/// | 2  | Xmm, Xmm, Imm |
/// | 3  | Xmm, Xmm, Mem |
/// | 4  | Xmm, Xmm, Xmm |
/// | 5  | Ymm, Mem, Imm |
/// | 6  | Ymm, Ymm, Imm |
/// | 7  | Ymm, Ymm, Mem |
/// | 8  | Ymm, Ymm, Xmm |
/// | 9  | Zmm, Mem, Imm |
/// | 10 | Zmm, Zmm, Imm |
/// | 11 | Zmm, Zmm, Mem |
/// | 12 | Zmm, Zmm, Xmm |
/// +----+---------------+
/// ```
pub trait VpsrlwMaskEmitter<A, B, C> {
    fn vpsrlw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsrlwMaskEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSRLW128RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLW128RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSRLW256RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLW256RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSRLW512RRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLW512RMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRLW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRLW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskEmitter<Ymm, Ymm, Xmm> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Xmm) {
        self.emit(
            VPSRLW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRLW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskEmitter<Zmm, Zmm, Xmm> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Xmm) {
        self.emit(
            VPSRLW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsrlw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRLW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSRLW_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +----+---------------+
/// | #  | Operands      |
/// +----+---------------+
/// | 1  | Xmm, Mem, Imm |
/// | 2  | Xmm, Xmm, Imm |
/// | 3  | Xmm, Xmm, Mem |
/// | 4  | Xmm, Xmm, Xmm |
/// | 5  | Ymm, Mem, Imm |
/// | 6  | Ymm, Ymm, Imm |
/// | 7  | Ymm, Ymm, Mem |
/// | 8  | Ymm, Ymm, Xmm |
/// | 9  | Zmm, Mem, Imm |
/// | 10 | Zmm, Zmm, Imm |
/// | 11 | Zmm, Zmm, Mem |
/// | 12 | Zmm, Zmm, Xmm |
/// +----+---------------+
/// ```
pub trait VpsrlwMaskzEmitter<A, B, C> {
    fn vpsrlw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsrlwMaskzEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            VPSRLW128RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskzEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLW128RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskzEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(
            VPSRLW256RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskzEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLW256RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskzEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(
            VPSRLW512RRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskzEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(
            VPSRLW512RMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSRLW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSRLW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskzEmitter<Ymm, Ymm, Xmm> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Xmm) {
        self.emit(
            VPSRLW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSRLW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskzEmitter<Zmm, Zmm, Xmm> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Xmm) {
        self.emit(
            VPSRLW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsrlwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsrlw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSRLW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBB`.
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
pub trait VpsubbEmitter<A, B, C> {
    fn vpsubb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBB_MASK`.
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
pub trait VpsubbMaskEmitter<A, B, C> {
    fn vpsubb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBB_MASKZ`.
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
pub trait VpsubbMaskzEmitter<A, B, C> {
    fn vpsubb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBSB`.
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
pub trait VpsubsbEmitter<A, B, C> {
    fn vpsubsb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubsbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubsb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBSB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubsb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBSB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubsb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBSB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubsb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBSB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubsb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBSB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubsb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBSB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBSB_MASK`.
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
pub trait VpsubsbMaskEmitter<A, B, C> {
    fn vpsubsb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubsbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubsb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBSB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubsb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBSB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubsb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBSB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubsb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBSB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubsb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBSB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubsb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBSB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBSB_MASKZ`.
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
pub trait VpsubsbMaskzEmitter<A, B, C> {
    fn vpsubsb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubsbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubsb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBSB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubsb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBSB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubsb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBSB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubsb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBSB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubsb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBSB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubsbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubsb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBSB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBSW`.
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
pub trait VpsubswEmitter<A, B, C> {
    fn vpsubsw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubswEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubsw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBSW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubsw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBSW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubsw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBSW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubsw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBSW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubsw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBSW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubsw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBSW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBSW_MASK`.
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
pub trait VpsubswMaskEmitter<A, B, C> {
    fn vpsubsw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubswMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBSW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubsw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBSW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBSW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubsw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBSW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBSW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubsw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBSW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBSW_MASKZ`.
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
pub trait VpsubswMaskzEmitter<A, B, C> {
    fn vpsubsw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubswMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBSW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubsw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBSW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBSW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubsw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBSW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBSW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubswMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubsw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBSW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBUSB`.
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
pub trait VpsubusbEmitter<A, B, C> {
    fn vpsubusb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubusbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubusb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBUSB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubusb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBUSB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubusb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBUSB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubusb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBUSB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubusb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBUSB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubusb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBUSB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBUSB_MASK`.
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
pub trait VpsubusbMaskEmitter<A, B, C> {
    fn vpsubusb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubusbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubusb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBUSB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubusb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBUSB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubusb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBUSB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubusb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBUSB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubusb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBUSB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubusb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBUSB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBUSB_MASKZ`.
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
pub trait VpsubusbMaskzEmitter<A, B, C> {
    fn vpsubusb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubusbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubusb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBUSB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubusb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBUSB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubusb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBUSB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubusb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBUSB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubusb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBUSB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubusbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubusb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBUSB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBUSW`.
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
pub trait VpsubuswEmitter<A, B, C> {
    fn vpsubusw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubuswEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubusw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBUSW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubusw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBUSW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubusw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBUSW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubusw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBUSW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubusw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBUSW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubusw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBUSW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBUSW_MASK`.
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
pub trait VpsubuswMaskEmitter<A, B, C> {
    fn vpsubusw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubuswMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubusw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBUSW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubusw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBUSW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubusw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBUSW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubusw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBUSW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubusw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBUSW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubusw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBUSW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBUSW_MASKZ`.
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
pub trait VpsubuswMaskzEmitter<A, B, C> {
    fn vpsubusw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubuswMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubusw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBUSW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubusw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBUSW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubusw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBUSW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubusw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBUSW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubusw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBUSW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubuswMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubusw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBUSW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBW`.
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
pub trait VpsubwEmitter<A, B, C> {
    fn vpsubw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBW_MASK`.
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
pub trait VpsubwMaskEmitter<A, B, C> {
    fn vpsubw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSUBW_MASKZ`.
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
pub trait VpsubwMaskzEmitter<A, B, C> {
    fn vpsubw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsubwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsubw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSUBW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsubw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSUBW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsubw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSUBW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsubw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSUBW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpsubw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSUBW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpsubwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpsubw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSUBW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPTESTMB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Xmm, Mem |
/// | 2 | KReg, Xmm, Xmm |
/// | 3 | KReg, Ymm, Mem |
/// | 4 | KReg, Ymm, Ymm |
/// | 5 | KReg, Zmm, Mem |
/// | 6 | KReg, Zmm, Zmm |
/// +---+----------------+
/// ```
pub trait VptestmbEmitter<A, B, C> {
    fn vptestmb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VptestmbEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vptestmb(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VPTESTMB128KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmbEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vptestmb(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VPTESTMB128KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmbEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vptestmb(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VPTESTMB256KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmbEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vptestmb(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VPTESTMB256KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmbEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vptestmb(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VPTESTMB512KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmbEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vptestmb(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VPTESTMB512KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPTESTMB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Xmm, Mem |
/// | 2 | KReg, Xmm, Xmm |
/// | 3 | KReg, Ymm, Mem |
/// | 4 | KReg, Ymm, Ymm |
/// | 5 | KReg, Zmm, Mem |
/// | 6 | KReg, Zmm, Zmm |
/// +---+----------------+
/// ```
pub trait VptestmbMaskEmitter<A, B, C> {
    fn vptestmb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VptestmbMaskEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vptestmb_mask(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VPTESTMB128KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmbMaskEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vptestmb_mask(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VPTESTMB128KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmbMaskEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vptestmb_mask(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VPTESTMB256KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmbMaskEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vptestmb_mask(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VPTESTMB256KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmbMaskEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vptestmb_mask(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VPTESTMB512KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmbMaskEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vptestmb_mask(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VPTESTMB512KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPTESTMW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Xmm, Mem |
/// | 2 | KReg, Xmm, Xmm |
/// | 3 | KReg, Ymm, Mem |
/// | 4 | KReg, Ymm, Ymm |
/// | 5 | KReg, Zmm, Mem |
/// | 6 | KReg, Zmm, Zmm |
/// +---+----------------+
/// ```
pub trait VptestmwEmitter<A, B, C> {
    fn vptestmw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VptestmwEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vptestmw(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VPTESTMW128KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmwEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vptestmw(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VPTESTMW128KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmwEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vptestmw(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VPTESTMW256KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmwEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vptestmw(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VPTESTMW256KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmwEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vptestmw(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VPTESTMW512KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmwEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vptestmw(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VPTESTMW512KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPTESTMW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Xmm, Mem |
/// | 2 | KReg, Xmm, Xmm |
/// | 3 | KReg, Ymm, Mem |
/// | 4 | KReg, Ymm, Ymm |
/// | 5 | KReg, Zmm, Mem |
/// | 6 | KReg, Zmm, Zmm |
/// +---+----------------+
/// ```
pub trait VptestmwMaskEmitter<A, B, C> {
    fn vptestmw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VptestmwMaskEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vptestmw_mask(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VPTESTMW128KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmwMaskEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vptestmw_mask(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VPTESTMW128KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmwMaskEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vptestmw_mask(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VPTESTMW256KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmwMaskEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vptestmw_mask(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VPTESTMW256KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmwMaskEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vptestmw_mask(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VPTESTMW512KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestmwMaskEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vptestmw_mask(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VPTESTMW512KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPTESTNMB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Xmm, Mem |
/// | 2 | KReg, Xmm, Xmm |
/// | 3 | KReg, Ymm, Mem |
/// | 4 | KReg, Ymm, Ymm |
/// | 5 | KReg, Zmm, Mem |
/// | 6 | KReg, Zmm, Zmm |
/// +---+----------------+
/// ```
pub trait VptestnmbEmitter<A, B, C> {
    fn vptestnmb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VptestnmbEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vptestnmb(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VPTESTNMB128KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmbEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vptestnmb(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VPTESTNMB128KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmbEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vptestnmb(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VPTESTNMB256KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmbEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vptestnmb(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VPTESTNMB256KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmbEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vptestnmb(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VPTESTNMB512KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmbEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vptestnmb(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VPTESTNMB512KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPTESTNMB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Xmm, Mem |
/// | 2 | KReg, Xmm, Xmm |
/// | 3 | KReg, Ymm, Mem |
/// | 4 | KReg, Ymm, Ymm |
/// | 5 | KReg, Zmm, Mem |
/// | 6 | KReg, Zmm, Zmm |
/// +---+----------------+
/// ```
pub trait VptestnmbMaskEmitter<A, B, C> {
    fn vptestnmb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VptestnmbMaskEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vptestnmb_mask(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VPTESTNMB128KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmbMaskEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vptestnmb_mask(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VPTESTNMB128KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmbMaskEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vptestnmb_mask(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VPTESTNMB256KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmbMaskEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vptestnmb_mask(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VPTESTNMB256KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmbMaskEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vptestnmb_mask(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VPTESTNMB512KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmbMaskEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vptestnmb_mask(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VPTESTNMB512KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPTESTNMW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Xmm, Mem |
/// | 2 | KReg, Xmm, Xmm |
/// | 3 | KReg, Ymm, Mem |
/// | 4 | KReg, Ymm, Ymm |
/// | 5 | KReg, Zmm, Mem |
/// | 6 | KReg, Zmm, Zmm |
/// +---+----------------+
/// ```
pub trait VptestnmwEmitter<A, B, C> {
    fn vptestnmw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VptestnmwEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vptestnmw(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VPTESTNMW128KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmwEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vptestnmw(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VPTESTNMW128KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmwEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vptestnmw(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VPTESTNMW256KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmwEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vptestnmw(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VPTESTNMW256KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmwEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vptestnmw(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VPTESTNMW512KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmwEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vptestnmw(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VPTESTNMW512KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPTESTNMW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Xmm, Mem |
/// | 2 | KReg, Xmm, Xmm |
/// | 3 | KReg, Ymm, Mem |
/// | 4 | KReg, Ymm, Ymm |
/// | 5 | KReg, Zmm, Mem |
/// | 6 | KReg, Zmm, Zmm |
/// +---+----------------+
/// ```
pub trait VptestnmwMaskEmitter<A, B, C> {
    fn vptestnmw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VptestnmwMaskEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vptestnmw_mask(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VPTESTNMW128KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmwMaskEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vptestnmw_mask(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VPTESTNMW128KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmwMaskEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vptestnmw_mask(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VPTESTNMW256KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmwMaskEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vptestnmw_mask(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VPTESTNMW256KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmwMaskEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vptestnmw_mask(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VPTESTNMW512KRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VptestnmwMaskEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vptestnmw_mask(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VPTESTNMW512KRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKHBW`.
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
pub trait VpunpckhbwEmitter<A, B, C> {
    fn vpunpckhbw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpckhbwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpckhbw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKHBW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpckhbw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKHBW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpckhbw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKHBW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpckhbw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKHBW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpckhbw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKHBW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpckhbw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKHBW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKHBW_MASK`.
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
pub trait VpunpckhbwMaskEmitter<A, B, C> {
    fn vpunpckhbw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpckhbwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpckhbw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKHBW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpckhbw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKHBW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpckhbw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKHBW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpckhbw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKHBW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpckhbw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKHBW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpckhbw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKHBW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKHBW_MASKZ`.
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
pub trait VpunpckhbwMaskzEmitter<A, B, C> {
    fn vpunpckhbw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpckhbwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpckhbw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKHBW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpckhbw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKHBW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpckhbw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKHBW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpckhbw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKHBW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpckhbw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKHBW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhbwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpckhbw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKHBW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKHWD`.
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
pub trait VpunpckhwdEmitter<A, B, C> {
    fn vpunpckhwd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpckhwdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpckhwd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKHWD128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpckhwd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKHWD128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpckhwd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKHWD256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpckhwd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKHWD256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpckhwd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKHWD512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpckhwd(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKHWD512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKHWD_MASK`.
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
pub trait VpunpckhwdMaskEmitter<A, B, C> {
    fn vpunpckhwd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpckhwdMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpckhwd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKHWD128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpckhwd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKHWD128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpckhwd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKHWD256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpckhwd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKHWD256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpckhwd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKHWD512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpckhwd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKHWD512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKHWD_MASKZ`.
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
pub trait VpunpckhwdMaskzEmitter<A, B, C> {
    fn vpunpckhwd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpckhwdMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpckhwd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKHWD128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpckhwd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKHWD128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpckhwd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKHWD256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpckhwd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKHWD256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpckhwd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKHWD512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpckhwdMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpckhwd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKHWD512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKLBW`.
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
pub trait VpunpcklbwEmitter<A, B, C> {
    fn vpunpcklbw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpcklbwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpcklbw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKLBW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpcklbw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKLBW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpcklbw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKLBW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpcklbw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKLBW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpcklbw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKLBW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpcklbw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKLBW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKLBW_MASK`.
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
pub trait VpunpcklbwMaskEmitter<A, B, C> {
    fn vpunpcklbw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpcklbwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpcklbw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKLBW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpcklbw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKLBW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpcklbw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKLBW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpcklbw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKLBW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpcklbw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKLBW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpcklbw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKLBW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKLBW_MASKZ`.
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
pub trait VpunpcklbwMaskzEmitter<A, B, C> {
    fn vpunpcklbw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpcklbwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpcklbw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKLBW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpcklbw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKLBW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpcklbw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKLBW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpcklbw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKLBW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpcklbw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKLBW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklbwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpcklbw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKLBW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKLWD`.
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
pub trait VpunpcklwdEmitter<A, B, C> {
    fn vpunpcklwd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpcklwdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpcklwd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKLWD128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpcklwd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKLWD128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpcklwd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKLWD256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpcklwd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKLWD256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpcklwd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKLWD512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpcklwd(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKLWD512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKLWD_MASK`.
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
pub trait VpunpcklwdMaskEmitter<A, B, C> {
    fn vpunpcklwd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpcklwdMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpcklwd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKLWD128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpcklwd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKLWD128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpcklwd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKLWD256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpcklwd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKLWD256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpcklwd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKLWD512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpcklwd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKLWD512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPUNPCKLWD_MASKZ`.
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
pub trait VpunpcklwdMaskzEmitter<A, B, C> {
    fn vpunpcklwd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpunpcklwdMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpunpcklwd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPUNPCKLWD128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpunpcklwd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPUNPCKLWD128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpunpcklwd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPUNPCKLWD256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpunpcklwd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPUNPCKLWD256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpunpcklwd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPUNPCKLWD512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpunpcklwdMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpunpcklwd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPUNPCKLWD512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `KADDD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kaddd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KadddEmitter<A, B, C>,
    {
        <Self as KadddEmitter<A, B, C>>::kaddd(self, op0, op1, op2);
    }
    /// `KADDQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kaddq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KaddqEmitter<A, B, C>,
    {
        <Self as KaddqEmitter<A, B, C>>::kaddq(self, op0, op1, op2);
    }
    /// `KANDD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kandd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KanddEmitter<A, B, C>,
    {
        <Self as KanddEmitter<A, B, C>>::kandd(self, op0, op1, op2);
    }
    /// `KANDND`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kandnd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KandndEmitter<A, B, C>,
    {
        <Self as KandndEmitter<A, B, C>>::kandnd(self, op0, op1, op2);
    }
    /// `KANDNQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kandnq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KandnqEmitter<A, B, C>,
    {
        <Self as KandnqEmitter<A, B, C>>::kandnq(self, op0, op1, op2);
    }
    /// `KANDQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kandq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KandqEmitter<A, B, C>,
    {
        <Self as KandqEmitter<A, B, C>>::kandq(self, op0, op1, op2);
    }
    /// `KMOVD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | Gpd, KReg  |
    /// | 2 | KReg, Gpd  |
    /// | 3 | KReg, KReg |
    /// | 4 | KReg, Mem  |
    /// | 5 | Mem, KReg  |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn kmovd<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: KmovdEmitter<A, B>,
    {
        <Self as KmovdEmitter<A, B>>::kmovd(self, op0, op1);
    }
    /// `KMOVQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | Gpd, KReg  |
    /// | 2 | KReg, Gpd  |
    /// | 3 | KReg, KReg |
    /// | 4 | KReg, Mem  |
    /// | 5 | Mem, KReg  |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn kmovq<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: KmovqEmitter<A, B>,
    {
        <Self as KmovqEmitter<A, B>>::kmovq(self, op0, op1);
    }
    /// `KNOTD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | KReg, KReg |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn knotd<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: KnotdEmitter<A, B>,
    {
        <Self as KnotdEmitter<A, B>>::knotd(self, op0, op1);
    }
    /// `KNOTQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | KReg, KReg |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn knotq<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: KnotqEmitter<A, B>,
    {
        <Self as KnotqEmitter<A, B>>::knotq(self, op0, op1);
    }
    /// `KORD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kord<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KordEmitter<A, B, C>,
    {
        <Self as KordEmitter<A, B, C>>::kord(self, op0, op1, op2);
    }
    /// `KORQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn korq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KorqEmitter<A, B, C>,
    {
        <Self as KorqEmitter<A, B, C>>::korq(self, op0, op1, op2);
    }
    /// `KORTESTD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | KReg, KReg |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn kortestd<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: KortestdEmitter<A, B>,
    {
        <Self as KortestdEmitter<A, B>>::kortestd(self, op0, op1);
    }
    /// `KORTESTQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | KReg, KReg |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn kortestq<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: KortestqEmitter<A, B>,
    {
        <Self as KortestqEmitter<A, B>>::kortestq(self, op0, op1);
    }
    /// `KSHIFTLD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------------+
    /// | # | Operands        |
    /// +---+-----------------+
    /// | 1 | KReg, KReg, Imm |
    /// +---+-----------------+
    /// ```
    #[inline]
    pub fn kshiftld<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KshiftldEmitter<A, B, C>,
    {
        <Self as KshiftldEmitter<A, B, C>>::kshiftld(self, op0, op1, op2);
    }
    /// `KSHIFTLQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------------+
    /// | # | Operands        |
    /// +---+-----------------+
    /// | 1 | KReg, KReg, Imm |
    /// +---+-----------------+
    /// ```
    #[inline]
    pub fn kshiftlq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KshiftlqEmitter<A, B, C>,
    {
        <Self as KshiftlqEmitter<A, B, C>>::kshiftlq(self, op0, op1, op2);
    }
    /// `KSHIFTRD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------------+
    /// | # | Operands        |
    /// +---+-----------------+
    /// | 1 | KReg, KReg, Imm |
    /// +---+-----------------+
    /// ```
    #[inline]
    pub fn kshiftrd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KshiftrdEmitter<A, B, C>,
    {
        <Self as KshiftrdEmitter<A, B, C>>::kshiftrd(self, op0, op1, op2);
    }
    /// `KSHIFTRQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------------+
    /// | # | Operands        |
    /// +---+-----------------+
    /// | 1 | KReg, KReg, Imm |
    /// +---+-----------------+
    /// ```
    #[inline]
    pub fn kshiftrq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KshiftrqEmitter<A, B, C>,
    {
        <Self as KshiftrqEmitter<A, B, C>>::kshiftrq(self, op0, op1, op2);
    }
    /// `KTESTD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | KReg, KReg |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn ktestd<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: KtestdEmitter<A, B>,
    {
        <Self as KtestdEmitter<A, B>>::ktestd(self, op0, op1);
    }
    /// `KTESTQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | KReg, KReg |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn ktestq<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: KtestqEmitter<A, B>,
    {
        <Self as KtestqEmitter<A, B>>::ktestq(self, op0, op1);
    }
    /// `KUNPCKDQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kunpckdq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KunpckdqEmitter<A, B, C>,
    {
        <Self as KunpckdqEmitter<A, B, C>>::kunpckdq(self, op0, op1, op2);
    }
    /// `KUNPCKWD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kunpckwd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KunpckwdEmitter<A, B, C>,
    {
        <Self as KunpckwdEmitter<A, B, C>>::kunpckwd(self, op0, op1, op2);
    }
    /// `KXNORD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kxnord<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KxnordEmitter<A, B, C>,
    {
        <Self as KxnordEmitter<A, B, C>>::kxnord(self, op0, op1, op2);
    }
    /// `KXNORQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kxnorq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KxnorqEmitter<A, B, C>,
    {
        <Self as KxnorqEmitter<A, B, C>>::kxnorq(self, op0, op1, op2);
    }
    /// `KXORD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kxord<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KxordEmitter<A, B, C>,
    {
        <Self as KxordEmitter<A, B, C>>::kxord(self, op0, op1, op2);
    }
    /// `KXORQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kxorq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: KxorqEmitter<A, B, C>,
    {
        <Self as KxorqEmitter<A, B, C>>::kxorq(self, op0, op1, op2);
    }
    /// `VDBPSADBW`.
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
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vdbpsadbw<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VdbpsadbwEmitter<A, B, C, D>,
    {
        <Self as VdbpsadbwEmitter<A, B, C, D>>::vdbpsadbw(self, op0, op1, op2, op3);
    }
    /// `VDBPSADBW_MASK`.
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
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vdbpsadbw_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VdbpsadbwMaskEmitter<A, B, C, D>,
    {
        <Self as VdbpsadbwMaskEmitter<A, B, C, D>>::vdbpsadbw_mask(self, op0, op1, op2, op3);
    }
    /// `VDBPSADBW_MASKZ`.
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
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vdbpsadbw_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VdbpsadbwMaskzEmitter<A, B, C, D>,
    {
        <Self as VdbpsadbwMaskzEmitter<A, B, C, D>>::vdbpsadbw_maskz(self, op0, op1, op2, op3);
    }
    /// `VMOVDQU16`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Mem, Zmm |
    /// | 4 | Xmm, Mem |
    /// | 5 | Xmm, Xmm |
    /// | 6 | Ymm, Mem |
    /// | 7 | Ymm, Ymm |
    /// | 8 | Zmm, Mem |
    /// | 9 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovdqu16<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vmovdqu16Emitter<A, B>,
    {
        <Self as Vmovdqu16Emitter<A, B>>::vmovdqu16(self, op0, op1);
    }
    /// `VMOVDQU16_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Mem, Zmm |
    /// | 4 | Xmm, Mem |
    /// | 5 | Xmm, Xmm |
    /// | 6 | Ymm, Mem |
    /// | 7 | Ymm, Ymm |
    /// | 8 | Zmm, Mem |
    /// | 9 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovdqu16_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vmovdqu16MaskEmitter<A, B>,
    {
        <Self as Vmovdqu16MaskEmitter<A, B>>::vmovdqu16_mask(self, op0, op1);
    }
    /// `VMOVDQU16_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovdqu16_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vmovdqu16MaskzEmitter<A, B>,
    {
        <Self as Vmovdqu16MaskzEmitter<A, B>>::vmovdqu16_maskz(self, op0, op1);
    }
    /// `VMOVDQU8`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Mem, Zmm |
    /// | 4 | Xmm, Mem |
    /// | 5 | Xmm, Xmm |
    /// | 6 | Ymm, Mem |
    /// | 7 | Ymm, Ymm |
    /// | 8 | Zmm, Mem |
    /// | 9 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovdqu8<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vmovdqu8Emitter<A, B>,
    {
        <Self as Vmovdqu8Emitter<A, B>>::vmovdqu8(self, op0, op1);
    }
    /// `VMOVDQU8_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Mem, Zmm |
    /// | 4 | Xmm, Mem |
    /// | 5 | Xmm, Xmm |
    /// | 6 | Ymm, Mem |
    /// | 7 | Ymm, Ymm |
    /// | 8 | Zmm, Mem |
    /// | 9 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovdqu8_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vmovdqu8MaskEmitter<A, B>,
    {
        <Self as Vmovdqu8MaskEmitter<A, B>>::vmovdqu8_mask(self, op0, op1);
    }
    /// `VMOVDQU8_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovdqu8_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vmovdqu8MaskzEmitter<A, B>,
    {
        <Self as Vmovdqu8MaskzEmitter<A, B>>::vmovdqu8_maskz(self, op0, op1);
    }
    /// `VPABSB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpabsb<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpabsbEmitter<A, B>,
    {
        <Self as VpabsbEmitter<A, B>>::vpabsb(self, op0, op1);
    }
    /// `VPABSB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpabsb_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpabsbMaskEmitter<A, B>,
    {
        <Self as VpabsbMaskEmitter<A, B>>::vpabsb_mask(self, op0, op1);
    }
    /// `VPABSB_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpabsb_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpabsbMaskzEmitter<A, B>,
    {
        <Self as VpabsbMaskzEmitter<A, B>>::vpabsb_maskz(self, op0, op1);
    }
    /// `VPABSW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpabsw<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpabswEmitter<A, B>,
    {
        <Self as VpabswEmitter<A, B>>::vpabsw(self, op0, op1);
    }
    /// `VPABSW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpabsw_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpabswMaskEmitter<A, B>,
    {
        <Self as VpabswMaskEmitter<A, B>>::vpabsw_mask(self, op0, op1);
    }
    /// `VPABSW_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpabsw_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpabswMaskzEmitter<A, B>,
    {
        <Self as VpabswMaskzEmitter<A, B>>::vpabsw_maskz(self, op0, op1);
    }
    /// `VPACKSSDW`.
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
    pub fn vpackssdw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpackssdwEmitter<A, B, C>,
    {
        <Self as VpackssdwEmitter<A, B, C>>::vpackssdw(self, op0, op1, op2);
    }
    /// `VPACKSSDW_MASK`.
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
    pub fn vpackssdw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpackssdwMaskEmitter<A, B, C>,
    {
        <Self as VpackssdwMaskEmitter<A, B, C>>::vpackssdw_mask(self, op0, op1, op2);
    }
    /// `VPACKSSDW_MASKZ`.
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
    pub fn vpackssdw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpackssdwMaskzEmitter<A, B, C>,
    {
        <Self as VpackssdwMaskzEmitter<A, B, C>>::vpackssdw_maskz(self, op0, op1, op2);
    }
    /// `VPACKSSWB`.
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
    pub fn vpacksswb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpacksswbEmitter<A, B, C>,
    {
        <Self as VpacksswbEmitter<A, B, C>>::vpacksswb(self, op0, op1, op2);
    }
    /// `VPACKSSWB_MASK`.
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
    pub fn vpacksswb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpacksswbMaskEmitter<A, B, C>,
    {
        <Self as VpacksswbMaskEmitter<A, B, C>>::vpacksswb_mask(self, op0, op1, op2);
    }
    /// `VPACKSSWB_MASKZ`.
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
    pub fn vpacksswb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpacksswbMaskzEmitter<A, B, C>,
    {
        <Self as VpacksswbMaskzEmitter<A, B, C>>::vpacksswb_maskz(self, op0, op1, op2);
    }
    /// `VPACKUSDW`.
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
    pub fn vpackusdw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpackusdwEmitter<A, B, C>,
    {
        <Self as VpackusdwEmitter<A, B, C>>::vpackusdw(self, op0, op1, op2);
    }
    /// `VPACKUSDW_MASK`.
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
    pub fn vpackusdw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpackusdwMaskEmitter<A, B, C>,
    {
        <Self as VpackusdwMaskEmitter<A, B, C>>::vpackusdw_mask(self, op0, op1, op2);
    }
    /// `VPACKUSDW_MASKZ`.
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
    pub fn vpackusdw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpackusdwMaskzEmitter<A, B, C>,
    {
        <Self as VpackusdwMaskzEmitter<A, B, C>>::vpackusdw_maskz(self, op0, op1, op2);
    }
    /// `VPACKUSWB`.
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
    pub fn vpackuswb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpackuswbEmitter<A, B, C>,
    {
        <Self as VpackuswbEmitter<A, B, C>>::vpackuswb(self, op0, op1, op2);
    }
    /// `VPACKUSWB_MASK`.
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
    pub fn vpackuswb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpackuswbMaskEmitter<A, B, C>,
    {
        <Self as VpackuswbMaskEmitter<A, B, C>>::vpackuswb_mask(self, op0, op1, op2);
    }
    /// `VPACKUSWB_MASKZ`.
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
    pub fn vpackuswb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpackuswbMaskzEmitter<A, B, C>,
    {
        <Self as VpackuswbMaskzEmitter<A, B, C>>::vpackuswb_maskz(self, op0, op1, op2);
    }
    /// `VPADDB`.
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
    pub fn vpaddb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddbEmitter<A, B, C>,
    {
        <Self as VpaddbEmitter<A, B, C>>::vpaddb(self, op0, op1, op2);
    }
    /// `VPADDB_MASK`.
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
    pub fn vpaddb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddbMaskEmitter<A, B, C>,
    {
        <Self as VpaddbMaskEmitter<A, B, C>>::vpaddb_mask(self, op0, op1, op2);
    }
    /// `VPADDB_MASKZ`.
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
    pub fn vpaddb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddbMaskzEmitter<A, B, C>,
    {
        <Self as VpaddbMaskzEmitter<A, B, C>>::vpaddb_maskz(self, op0, op1, op2);
    }
    /// `VPADDSB`.
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
    pub fn vpaddsb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddsbEmitter<A, B, C>,
    {
        <Self as VpaddsbEmitter<A, B, C>>::vpaddsb(self, op0, op1, op2);
    }
    /// `VPADDSB_MASK`.
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
    pub fn vpaddsb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddsbMaskEmitter<A, B, C>,
    {
        <Self as VpaddsbMaskEmitter<A, B, C>>::vpaddsb_mask(self, op0, op1, op2);
    }
    /// `VPADDSB_MASKZ`.
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
    pub fn vpaddsb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddsbMaskzEmitter<A, B, C>,
    {
        <Self as VpaddsbMaskzEmitter<A, B, C>>::vpaddsb_maskz(self, op0, op1, op2);
    }
    /// `VPADDSW`.
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
    pub fn vpaddsw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddswEmitter<A, B, C>,
    {
        <Self as VpaddswEmitter<A, B, C>>::vpaddsw(self, op0, op1, op2);
    }
    /// `VPADDSW_MASK`.
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
    pub fn vpaddsw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddswMaskEmitter<A, B, C>,
    {
        <Self as VpaddswMaskEmitter<A, B, C>>::vpaddsw_mask(self, op0, op1, op2);
    }
    /// `VPADDSW_MASKZ`.
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
    pub fn vpaddsw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddswMaskzEmitter<A, B, C>,
    {
        <Self as VpaddswMaskzEmitter<A, B, C>>::vpaddsw_maskz(self, op0, op1, op2);
    }
    /// `VPADDUSB`.
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
    pub fn vpaddusb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddusbEmitter<A, B, C>,
    {
        <Self as VpaddusbEmitter<A, B, C>>::vpaddusb(self, op0, op1, op2);
    }
    /// `VPADDUSB_MASK`.
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
    pub fn vpaddusb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddusbMaskEmitter<A, B, C>,
    {
        <Self as VpaddusbMaskEmitter<A, B, C>>::vpaddusb_mask(self, op0, op1, op2);
    }
    /// `VPADDUSB_MASKZ`.
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
    pub fn vpaddusb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddusbMaskzEmitter<A, B, C>,
    {
        <Self as VpaddusbMaskzEmitter<A, B, C>>::vpaddusb_maskz(self, op0, op1, op2);
    }
    /// `VPADDUSW`.
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
    pub fn vpaddusw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpadduswEmitter<A, B, C>,
    {
        <Self as VpadduswEmitter<A, B, C>>::vpaddusw(self, op0, op1, op2);
    }
    /// `VPADDUSW_MASK`.
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
    pub fn vpaddusw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpadduswMaskEmitter<A, B, C>,
    {
        <Self as VpadduswMaskEmitter<A, B, C>>::vpaddusw_mask(self, op0, op1, op2);
    }
    /// `VPADDUSW_MASKZ`.
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
    pub fn vpaddusw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpadduswMaskzEmitter<A, B, C>,
    {
        <Self as VpadduswMaskzEmitter<A, B, C>>::vpaddusw_maskz(self, op0, op1, op2);
    }
    /// `VPADDW`.
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
    pub fn vpaddw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddwEmitter<A, B, C>,
    {
        <Self as VpaddwEmitter<A, B, C>>::vpaddw(self, op0, op1, op2);
    }
    /// `VPADDW_MASK`.
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
    pub fn vpaddw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddwMaskEmitter<A, B, C>,
    {
        <Self as VpaddwMaskEmitter<A, B, C>>::vpaddw_mask(self, op0, op1, op2);
    }
    /// `VPADDW_MASKZ`.
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
    pub fn vpaddw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpaddwMaskzEmitter<A, B, C>,
    {
        <Self as VpaddwMaskzEmitter<A, B, C>>::vpaddw_maskz(self, op0, op1, op2);
    }
    /// `VPALIGNR`.
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
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vpalignr<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpalignrEmitter<A, B, C, D>,
    {
        <Self as VpalignrEmitter<A, B, C, D>>::vpalignr(self, op0, op1, op2, op3);
    }
    /// `VPALIGNR_MASK`.
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
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vpalignr_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpalignrMaskEmitter<A, B, C, D>,
    {
        <Self as VpalignrMaskEmitter<A, B, C, D>>::vpalignr_mask(self, op0, op1, op2, op3);
    }
    /// `VPALIGNR_MASKZ`.
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
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vpalignr_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpalignrMaskzEmitter<A, B, C, D>,
    {
        <Self as VpalignrMaskzEmitter<A, B, C, D>>::vpalignr_maskz(self, op0, op1, op2, op3);
    }
    /// `VPAVGB`.
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
    pub fn vpavgb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpavgbEmitter<A, B, C>,
    {
        <Self as VpavgbEmitter<A, B, C>>::vpavgb(self, op0, op1, op2);
    }
    /// `VPAVGB_MASK`.
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
    pub fn vpavgb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpavgbMaskEmitter<A, B, C>,
    {
        <Self as VpavgbMaskEmitter<A, B, C>>::vpavgb_mask(self, op0, op1, op2);
    }
    /// `VPAVGB_MASKZ`.
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
    pub fn vpavgb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpavgbMaskzEmitter<A, B, C>,
    {
        <Self as VpavgbMaskzEmitter<A, B, C>>::vpavgb_maskz(self, op0, op1, op2);
    }
    /// `VPAVGW`.
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
    pub fn vpavgw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpavgwEmitter<A, B, C>,
    {
        <Self as VpavgwEmitter<A, B, C>>::vpavgw(self, op0, op1, op2);
    }
    /// `VPAVGW_MASK`.
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
    pub fn vpavgw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpavgwMaskEmitter<A, B, C>,
    {
        <Self as VpavgwMaskEmitter<A, B, C>>::vpavgw_mask(self, op0, op1, op2);
    }
    /// `VPAVGW_MASKZ`.
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
    pub fn vpavgw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpavgwMaskzEmitter<A, B, C>,
    {
        <Self as VpavgwMaskzEmitter<A, B, C>>::vpavgw_maskz(self, op0, op1, op2);
    }
    /// `VPBLENDMB`.
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
    pub fn vpblendmb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpblendmbEmitter<A, B, C>,
    {
        <Self as VpblendmbEmitter<A, B, C>>::vpblendmb(self, op0, op1, op2);
    }
    /// `VPBLENDMB_MASK`.
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
    pub fn vpblendmb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpblendmbMaskEmitter<A, B, C>,
    {
        <Self as VpblendmbMaskEmitter<A, B, C>>::vpblendmb_mask(self, op0, op1, op2);
    }
    /// `VPBLENDMB_MASKZ`.
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
    pub fn vpblendmb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpblendmbMaskzEmitter<A, B, C>,
    {
        <Self as VpblendmbMaskzEmitter<A, B, C>>::vpblendmb_maskz(self, op0, op1, op2);
    }
    /// `VPBLENDMW`.
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
    pub fn vpblendmw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpblendmwEmitter<A, B, C>,
    {
        <Self as VpblendmwEmitter<A, B, C>>::vpblendmw(self, op0, op1, op2);
    }
    /// `VPBLENDMW_MASK`.
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
    pub fn vpblendmw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpblendmwMaskEmitter<A, B, C>,
    {
        <Self as VpblendmwMaskEmitter<A, B, C>>::vpblendmw_mask(self, op0, op1, op2);
    }
    /// `VPBLENDMW_MASKZ`.
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
    pub fn vpblendmw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpblendmwMaskzEmitter<A, B, C>,
    {
        <Self as VpblendmwMaskzEmitter<A, B, C>>::vpblendmw_maskz(self, op0, op1, op2);
    }
    /// `VPBROADCASTB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastb<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastbEmitter<A, B>,
    {
        <Self as VpbroadcastbEmitter<A, B>>::vpbroadcastb(self, op0, op1);
    }
    /// `VPBROADCASTB_GP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Gpd |
    /// | 2 | Ymm, Gpd |
    /// | 3 | Zmm, Gpd |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastb_gp<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastbGpEmitter<A, B>,
    {
        <Self as VpbroadcastbGpEmitter<A, B>>::vpbroadcastb_gp(self, op0, op1);
    }
    /// `VPBROADCASTB_GP_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Gpd |
    /// | 2 | Ymm, Gpd |
    /// | 3 | Zmm, Gpd |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastb_gp_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastbGpMaskEmitter<A, B>,
    {
        <Self as VpbroadcastbGpMaskEmitter<A, B>>::vpbroadcastb_gp_mask(self, op0, op1);
    }
    /// `VPBROADCASTB_GP_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Gpd |
    /// | 2 | Ymm, Gpd |
    /// | 3 | Zmm, Gpd |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastb_gp_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastbGpMaskzEmitter<A, B>,
    {
        <Self as VpbroadcastbGpMaskzEmitter<A, B>>::vpbroadcastb_gp_maskz(self, op0, op1);
    }
    /// `VPBROADCASTB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastb_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastbMaskEmitter<A, B>,
    {
        <Self as VpbroadcastbMaskEmitter<A, B>>::vpbroadcastb_mask(self, op0, op1);
    }
    /// `VPBROADCASTB_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastb_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastbMaskzEmitter<A, B>,
    {
        <Self as VpbroadcastbMaskzEmitter<A, B>>::vpbroadcastb_maskz(self, op0, op1);
    }
    /// `VPBROADCASTW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastw<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastwEmitter<A, B>,
    {
        <Self as VpbroadcastwEmitter<A, B>>::vpbroadcastw(self, op0, op1);
    }
    /// `VPBROADCASTW_GP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Gpd |
    /// | 2 | Ymm, Gpd |
    /// | 3 | Zmm, Gpd |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastw_gp<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastwGpEmitter<A, B>,
    {
        <Self as VpbroadcastwGpEmitter<A, B>>::vpbroadcastw_gp(self, op0, op1);
    }
    /// `VPBROADCASTW_GP_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Gpd |
    /// | 2 | Ymm, Gpd |
    /// | 3 | Zmm, Gpd |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastw_gp_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastwGpMaskEmitter<A, B>,
    {
        <Self as VpbroadcastwGpMaskEmitter<A, B>>::vpbroadcastw_gp_mask(self, op0, op1);
    }
    /// `VPBROADCASTW_GP_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Gpd |
    /// | 2 | Ymm, Gpd |
    /// | 3 | Zmm, Gpd |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastw_gp_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastwGpMaskzEmitter<A, B>,
    {
        <Self as VpbroadcastwGpMaskzEmitter<A, B>>::vpbroadcastw_gp_maskz(self, op0, op1);
    }
    /// `VPBROADCASTW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastw_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastwMaskEmitter<A, B>,
    {
        <Self as VpbroadcastwMaskEmitter<A, B>>::vpbroadcastw_mask(self, op0, op1);
    }
    /// `VPBROADCASTW_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpbroadcastw_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpbroadcastwMaskzEmitter<A, B>,
    {
        <Self as VpbroadcastwMaskzEmitter<A, B>>::vpbroadcastw_maskz(self, op0, op1);
    }
    /// `VPCMPB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------------+
    /// | # | Operands            |
    /// +---+---------------------+
    /// | 1 | KReg, Xmm, Mem, Imm |
    /// | 2 | KReg, Xmm, Xmm, Imm |
    /// | 3 | KReg, Ymm, Mem, Imm |
    /// | 4 | KReg, Ymm, Ymm, Imm |
    /// | 5 | KReg, Zmm, Mem, Imm |
    /// | 6 | KReg, Zmm, Zmm, Imm |
    /// +---+---------------------+
    /// ```
    #[inline]
    pub fn vpcmpb<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpcmpbEmitter<A, B, C, D>,
    {
        <Self as VpcmpbEmitter<A, B, C, D>>::vpcmpb(self, op0, op1, op2, op3);
    }
    /// `VPCMPB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------------+
    /// | # | Operands            |
    /// +---+---------------------+
    /// | 1 | KReg, Xmm, Mem, Imm |
    /// | 2 | KReg, Xmm, Xmm, Imm |
    /// | 3 | KReg, Ymm, Mem, Imm |
    /// | 4 | KReg, Ymm, Ymm, Imm |
    /// | 5 | KReg, Zmm, Mem, Imm |
    /// | 6 | KReg, Zmm, Zmm, Imm |
    /// +---+---------------------+
    /// ```
    #[inline]
    pub fn vpcmpb_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpcmpbMaskEmitter<A, B, C, D>,
    {
        <Self as VpcmpbMaskEmitter<A, B, C, D>>::vpcmpb_mask(self, op0, op1, op2, op3);
    }
    /// `VPCMPUB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------------+
    /// | # | Operands            |
    /// +---+---------------------+
    /// | 1 | KReg, Xmm, Mem, Imm |
    /// | 2 | KReg, Xmm, Xmm, Imm |
    /// | 3 | KReg, Ymm, Mem, Imm |
    /// | 4 | KReg, Ymm, Ymm, Imm |
    /// | 5 | KReg, Zmm, Mem, Imm |
    /// | 6 | KReg, Zmm, Zmm, Imm |
    /// +---+---------------------+
    /// ```
    #[inline]
    pub fn vpcmpub<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpcmpubEmitter<A, B, C, D>,
    {
        <Self as VpcmpubEmitter<A, B, C, D>>::vpcmpub(self, op0, op1, op2, op3);
    }
    /// `VPCMPUB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------------+
    /// | # | Operands            |
    /// +---+---------------------+
    /// | 1 | KReg, Xmm, Mem, Imm |
    /// | 2 | KReg, Xmm, Xmm, Imm |
    /// | 3 | KReg, Ymm, Mem, Imm |
    /// | 4 | KReg, Ymm, Ymm, Imm |
    /// | 5 | KReg, Zmm, Mem, Imm |
    /// | 6 | KReg, Zmm, Zmm, Imm |
    /// +---+---------------------+
    /// ```
    #[inline]
    pub fn vpcmpub_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpcmpubMaskEmitter<A, B, C, D>,
    {
        <Self as VpcmpubMaskEmitter<A, B, C, D>>::vpcmpub_mask(self, op0, op1, op2, op3);
    }
    /// `VPCMPUW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------------+
    /// | # | Operands            |
    /// +---+---------------------+
    /// | 1 | KReg, Xmm, Mem, Imm |
    /// | 2 | KReg, Xmm, Xmm, Imm |
    /// | 3 | KReg, Ymm, Mem, Imm |
    /// | 4 | KReg, Ymm, Ymm, Imm |
    /// | 5 | KReg, Zmm, Mem, Imm |
    /// | 6 | KReg, Zmm, Zmm, Imm |
    /// +---+---------------------+
    /// ```
    #[inline]
    pub fn vpcmpuw<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpcmpuwEmitter<A, B, C, D>,
    {
        <Self as VpcmpuwEmitter<A, B, C, D>>::vpcmpuw(self, op0, op1, op2, op3);
    }
    /// `VPCMPUW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------------+
    /// | # | Operands            |
    /// +---+---------------------+
    /// | 1 | KReg, Xmm, Mem, Imm |
    /// | 2 | KReg, Xmm, Xmm, Imm |
    /// | 3 | KReg, Ymm, Mem, Imm |
    /// | 4 | KReg, Ymm, Ymm, Imm |
    /// | 5 | KReg, Zmm, Mem, Imm |
    /// | 6 | KReg, Zmm, Zmm, Imm |
    /// +---+---------------------+
    /// ```
    #[inline]
    pub fn vpcmpuw_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpcmpuwMaskEmitter<A, B, C, D>,
    {
        <Self as VpcmpuwMaskEmitter<A, B, C, D>>::vpcmpuw_mask(self, op0, op1, op2, op3);
    }
    /// `VPCMPW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------------+
    /// | # | Operands            |
    /// +---+---------------------+
    /// | 1 | KReg, Xmm, Mem, Imm |
    /// | 2 | KReg, Xmm, Xmm, Imm |
    /// | 3 | KReg, Ymm, Mem, Imm |
    /// | 4 | KReg, Ymm, Ymm, Imm |
    /// | 5 | KReg, Zmm, Mem, Imm |
    /// | 6 | KReg, Zmm, Zmm, Imm |
    /// +---+---------------------+
    /// ```
    #[inline]
    pub fn vpcmpw<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpcmpwEmitter<A, B, C, D>,
    {
        <Self as VpcmpwEmitter<A, B, C, D>>::vpcmpw(self, op0, op1, op2, op3);
    }
    /// `VPCMPW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------------+
    /// | # | Operands            |
    /// +---+---------------------+
    /// | 1 | KReg, Xmm, Mem, Imm |
    /// | 2 | KReg, Xmm, Xmm, Imm |
    /// | 3 | KReg, Ymm, Mem, Imm |
    /// | 4 | KReg, Ymm, Ymm, Imm |
    /// | 5 | KReg, Zmm, Mem, Imm |
    /// | 6 | KReg, Zmm, Zmm, Imm |
    /// +---+---------------------+
    /// ```
    #[inline]
    pub fn vpcmpw_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpcmpwMaskEmitter<A, B, C, D>,
    {
        <Self as VpcmpwMaskEmitter<A, B, C, D>>::vpcmpw_mask(self, op0, op1, op2, op3);
    }
    /// `VPERMI2W`.
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
    pub fn vpermi2w<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermi2wEmitter<A, B, C>,
    {
        <Self as Vpermi2wEmitter<A, B, C>>::vpermi2w(self, op0, op1, op2);
    }
    /// `VPERMI2W_MASK`.
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
    pub fn vpermi2w_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermi2wMaskEmitter<A, B, C>,
    {
        <Self as Vpermi2wMaskEmitter<A, B, C>>::vpermi2w_mask(self, op0, op1, op2);
    }
    /// `VPERMI2W_MASKZ`.
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
    pub fn vpermi2w_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermi2wMaskzEmitter<A, B, C>,
    {
        <Self as Vpermi2wMaskzEmitter<A, B, C>>::vpermi2w_maskz(self, op0, op1, op2);
    }
    /// `VPERMT2W`.
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
    pub fn vpermt2w<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermt2wEmitter<A, B, C>,
    {
        <Self as Vpermt2wEmitter<A, B, C>>::vpermt2w(self, op0, op1, op2);
    }
    /// `VPERMT2W_MASK`.
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
    pub fn vpermt2w_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermt2wMaskEmitter<A, B, C>,
    {
        <Self as Vpermt2wMaskEmitter<A, B, C>>::vpermt2w_mask(self, op0, op1, op2);
    }
    /// `VPERMT2W_MASKZ`.
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
    pub fn vpermt2w_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermt2wMaskzEmitter<A, B, C>,
    {
        <Self as Vpermt2wMaskzEmitter<A, B, C>>::vpermt2w_maskz(self, op0, op1, op2);
    }
    /// `VPERMW`.
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
    pub fn vpermw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpermwEmitter<A, B, C>,
    {
        <Self as VpermwEmitter<A, B, C>>::vpermw(self, op0, op1, op2);
    }
    /// `VPERMW_MASK`.
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
    pub fn vpermw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpermwMaskEmitter<A, B, C>,
    {
        <Self as VpermwMaskEmitter<A, B, C>>::vpermw_mask(self, op0, op1, op2);
    }
    /// `VPERMW_MASKZ`.
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
    pub fn vpermw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpermwMaskzEmitter<A, B, C>,
    {
        <Self as VpermwMaskzEmitter<A, B, C>>::vpermw_maskz(self, op0, op1, op2);
    }
    /// `VPEXTRB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Xmm, Imm |
    /// | 2 | Mem, Xmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpextrb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpextrbEmitter<A, B, C>,
    {
        <Self as VpextrbEmitter<A, B, C>>::vpextrb(self, op0, op1, op2);
    }
    /// `VPEXTRW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Xmm, Imm |
    /// | 2 | Mem, Xmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpextrw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpextrwEmitter<A, B, C>,
    {
        <Self as VpextrwEmitter<A, B, C>>::vpextrw(self, op0, op1, op2);
    }
    /// `VPINSRB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Gpd, Imm |
    /// | 2 | Xmm, Xmm, Mem, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vpinsrb<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpinsrbEmitter<A, B, C, D>,
    {
        <Self as VpinsrbEmitter<A, B, C, D>>::vpinsrb(self, op0, op1, op2, op3);
    }
    /// `VPINSRW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Gpd, Imm |
    /// | 2 | Xmm, Xmm, Mem, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vpinsrw<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpinsrwEmitter<A, B, C, D>,
    {
        <Self as VpinsrwEmitter<A, B, C, D>>::vpinsrw(self, op0, op1, op2, op3);
    }
    /// `VPMADDUBSW`.
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
    pub fn vpmaddubsw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaddubswEmitter<A, B, C>,
    {
        <Self as VpmaddubswEmitter<A, B, C>>::vpmaddubsw(self, op0, op1, op2);
    }
    /// `VPMADDUBSW_MASK`.
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
    pub fn vpmaddubsw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaddubswMaskEmitter<A, B, C>,
    {
        <Self as VpmaddubswMaskEmitter<A, B, C>>::vpmaddubsw_mask(self, op0, op1, op2);
    }
    /// `VPMADDUBSW_MASKZ`.
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
    pub fn vpmaddubsw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaddubswMaskzEmitter<A, B, C>,
    {
        <Self as VpmaddubswMaskzEmitter<A, B, C>>::vpmaddubsw_maskz(self, op0, op1, op2);
    }
    /// `VPMADDWD`.
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
    pub fn vpmaddwd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaddwdEmitter<A, B, C>,
    {
        <Self as VpmaddwdEmitter<A, B, C>>::vpmaddwd(self, op0, op1, op2);
    }
    /// `VPMADDWD_MASK`.
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
    pub fn vpmaddwd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaddwdMaskEmitter<A, B, C>,
    {
        <Self as VpmaddwdMaskEmitter<A, B, C>>::vpmaddwd_mask(self, op0, op1, op2);
    }
    /// `VPMADDWD_MASKZ`.
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
    pub fn vpmaddwd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaddwdMaskzEmitter<A, B, C>,
    {
        <Self as VpmaddwdMaskzEmitter<A, B, C>>::vpmaddwd_maskz(self, op0, op1, op2);
    }
    /// `VPMAXSB`.
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
    pub fn vpmaxsb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxsbEmitter<A, B, C>,
    {
        <Self as VpmaxsbEmitter<A, B, C>>::vpmaxsb(self, op0, op1, op2);
    }
    /// `VPMAXSB_MASK`.
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
    pub fn vpmaxsb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxsbMaskEmitter<A, B, C>,
    {
        <Self as VpmaxsbMaskEmitter<A, B, C>>::vpmaxsb_mask(self, op0, op1, op2);
    }
    /// `VPMAXSB_MASKZ`.
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
    pub fn vpmaxsb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxsbMaskzEmitter<A, B, C>,
    {
        <Self as VpmaxsbMaskzEmitter<A, B, C>>::vpmaxsb_maskz(self, op0, op1, op2);
    }
    /// `VPMAXSW`.
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
    pub fn vpmaxsw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxswEmitter<A, B, C>,
    {
        <Self as VpmaxswEmitter<A, B, C>>::vpmaxsw(self, op0, op1, op2);
    }
    /// `VPMAXSW_MASK`.
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
    pub fn vpmaxsw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxswMaskEmitter<A, B, C>,
    {
        <Self as VpmaxswMaskEmitter<A, B, C>>::vpmaxsw_mask(self, op0, op1, op2);
    }
    /// `VPMAXSW_MASKZ`.
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
    pub fn vpmaxsw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxswMaskzEmitter<A, B, C>,
    {
        <Self as VpmaxswMaskzEmitter<A, B, C>>::vpmaxsw_maskz(self, op0, op1, op2);
    }
    /// `VPMAXUB`.
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
    pub fn vpmaxub<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxubEmitter<A, B, C>,
    {
        <Self as VpmaxubEmitter<A, B, C>>::vpmaxub(self, op0, op1, op2);
    }
    /// `VPMAXUB_MASK`.
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
    pub fn vpmaxub_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxubMaskEmitter<A, B, C>,
    {
        <Self as VpmaxubMaskEmitter<A, B, C>>::vpmaxub_mask(self, op0, op1, op2);
    }
    /// `VPMAXUB_MASKZ`.
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
    pub fn vpmaxub_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxubMaskzEmitter<A, B, C>,
    {
        <Self as VpmaxubMaskzEmitter<A, B, C>>::vpmaxub_maskz(self, op0, op1, op2);
    }
    /// `VPMAXUW`.
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
    pub fn vpmaxuw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxuwEmitter<A, B, C>,
    {
        <Self as VpmaxuwEmitter<A, B, C>>::vpmaxuw(self, op0, op1, op2);
    }
    /// `VPMAXUW_MASK`.
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
    pub fn vpmaxuw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxuwMaskEmitter<A, B, C>,
    {
        <Self as VpmaxuwMaskEmitter<A, B, C>>::vpmaxuw_mask(self, op0, op1, op2);
    }
    /// `VPMAXUW_MASKZ`.
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
    pub fn vpmaxuw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmaxuwMaskzEmitter<A, B, C>,
    {
        <Self as VpmaxuwMaskzEmitter<A, B, C>>::vpmaxuw_maskz(self, op0, op1, op2);
    }
    /// `VPMINSB`.
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
    pub fn vpminsb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminsbEmitter<A, B, C>,
    {
        <Self as VpminsbEmitter<A, B, C>>::vpminsb(self, op0, op1, op2);
    }
    /// `VPMINSB_MASK`.
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
    pub fn vpminsb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminsbMaskEmitter<A, B, C>,
    {
        <Self as VpminsbMaskEmitter<A, B, C>>::vpminsb_mask(self, op0, op1, op2);
    }
    /// `VPMINSB_MASKZ`.
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
    pub fn vpminsb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminsbMaskzEmitter<A, B, C>,
    {
        <Self as VpminsbMaskzEmitter<A, B, C>>::vpminsb_maskz(self, op0, op1, op2);
    }
    /// `VPMINSW`.
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
    pub fn vpminsw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminswEmitter<A, B, C>,
    {
        <Self as VpminswEmitter<A, B, C>>::vpminsw(self, op0, op1, op2);
    }
    /// `VPMINSW_MASK`.
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
    pub fn vpminsw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminswMaskEmitter<A, B, C>,
    {
        <Self as VpminswMaskEmitter<A, B, C>>::vpminsw_mask(self, op0, op1, op2);
    }
    /// `VPMINSW_MASKZ`.
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
    pub fn vpminsw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminswMaskzEmitter<A, B, C>,
    {
        <Self as VpminswMaskzEmitter<A, B, C>>::vpminsw_maskz(self, op0, op1, op2);
    }
    /// `VPMINUB`.
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
    pub fn vpminub<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminubEmitter<A, B, C>,
    {
        <Self as VpminubEmitter<A, B, C>>::vpminub(self, op0, op1, op2);
    }
    /// `VPMINUB_MASK`.
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
    pub fn vpminub_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminubMaskEmitter<A, B, C>,
    {
        <Self as VpminubMaskEmitter<A, B, C>>::vpminub_mask(self, op0, op1, op2);
    }
    /// `VPMINUB_MASKZ`.
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
    pub fn vpminub_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminubMaskzEmitter<A, B, C>,
    {
        <Self as VpminubMaskzEmitter<A, B, C>>::vpminub_maskz(self, op0, op1, op2);
    }
    /// `VPMINUW`.
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
    pub fn vpminuw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminuwEmitter<A, B, C>,
    {
        <Self as VpminuwEmitter<A, B, C>>::vpminuw(self, op0, op1, op2);
    }
    /// `VPMINUW_MASK`.
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
    pub fn vpminuw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminuwMaskEmitter<A, B, C>,
    {
        <Self as VpminuwMaskEmitter<A, B, C>>::vpminuw_mask(self, op0, op1, op2);
    }
    /// `VPMINUW_MASKZ`.
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
    pub fn vpminuw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpminuwMaskzEmitter<A, B, C>,
    {
        <Self as VpminuwMaskzEmitter<A, B, C>>::vpminuw_maskz(self, op0, op1, op2);
    }
    /// `VPMOVB2M`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------+
    /// | # | Operands  |
    /// +---+-----------+
    /// | 1 | KReg, Xmm |
    /// | 2 | KReg, Ymm |
    /// | 3 | KReg, Zmm |
    /// +---+-----------+
    /// ```
    #[inline]
    pub fn vpmovb2m<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vpmovb2mEmitter<A, B>,
    {
        <Self as Vpmovb2mEmitter<A, B>>::vpmovb2m(self, op0, op1);
    }
    /// `VPMOVM2B`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------+
    /// | # | Operands  |
    /// +---+-----------+
    /// | 1 | Xmm, KReg |
    /// | 2 | Ymm, KReg |
    /// | 3 | Zmm, KReg |
    /// +---+-----------+
    /// ```
    #[inline]
    pub fn vpmovm2b<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vpmovm2bEmitter<A, B>,
    {
        <Self as Vpmovm2bEmitter<A, B>>::vpmovm2b(self, op0, op1);
    }
    /// `VPMOVM2W`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------+
    /// | # | Operands  |
    /// +---+-----------+
    /// | 1 | Xmm, KReg |
    /// | 2 | Ymm, KReg |
    /// | 3 | Zmm, KReg |
    /// +---+-----------+
    /// ```
    #[inline]
    pub fn vpmovm2w<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vpmovm2wEmitter<A, B>,
    {
        <Self as Vpmovm2wEmitter<A, B>>::vpmovm2w(self, op0, op1);
    }
    /// `VPMOVSWB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Mem, Zmm |
    /// | 4 | Xmm, Xmm |
    /// | 5 | Xmm, Ymm |
    /// | 6 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpmovswb<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpmovswbEmitter<A, B>,
    {
        <Self as VpmovswbEmitter<A, B>>::vpmovswb(self, op0, op1);
    }
    /// `VPMOVSWB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Mem, Zmm |
    /// | 4 | Xmm, Xmm |
    /// | 5 | Xmm, Ymm |
    /// | 6 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpmovswb_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpmovswbMaskEmitter<A, B>,
    {
        <Self as VpmovswbMaskEmitter<A, B>>::vpmovswb_mask(self, op0, op1);
    }
    /// `VPMOVSWB_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Xmm |
    /// | 2 | Xmm, Ymm |
    /// | 3 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpmovswb_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpmovswbMaskzEmitter<A, B>,
    {
        <Self as VpmovswbMaskzEmitter<A, B>>::vpmovswb_maskz(self, op0, op1);
    }
    /// `VPMOVUSWB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Mem, Zmm |
    /// | 4 | Xmm, Xmm |
    /// | 5 | Xmm, Ymm |
    /// | 6 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpmovuswb<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpmovuswbEmitter<A, B>,
    {
        <Self as VpmovuswbEmitter<A, B>>::vpmovuswb(self, op0, op1);
    }
    /// `VPMOVUSWB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Mem, Zmm |
    /// | 4 | Xmm, Xmm |
    /// | 5 | Xmm, Ymm |
    /// | 6 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpmovuswb_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpmovuswbMaskEmitter<A, B>,
    {
        <Self as VpmovuswbMaskEmitter<A, B>>::vpmovuswb_mask(self, op0, op1);
    }
    /// `VPMOVUSWB_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Xmm |
    /// | 2 | Xmm, Ymm |
    /// | 3 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpmovuswb_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpmovuswbMaskzEmitter<A, B>,
    {
        <Self as VpmovuswbMaskzEmitter<A, B>>::vpmovuswb_maskz(self, op0, op1);
    }
    /// `VPMOVW2M`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------+
    /// | # | Operands  |
    /// +---+-----------+
    /// | 1 | KReg, Xmm |
    /// | 2 | KReg, Ymm |
    /// | 3 | KReg, Zmm |
    /// +---+-----------+
    /// ```
    #[inline]
    pub fn vpmovw2m<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vpmovw2mEmitter<A, B>,
    {
        <Self as Vpmovw2mEmitter<A, B>>::vpmovw2m(self, op0, op1);
    }
    /// `VPMOVWB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Mem, Zmm |
    /// | 4 | Xmm, Xmm |
    /// | 5 | Xmm, Ymm |
    /// | 6 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpmovwb<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpmovwbEmitter<A, B>,
    {
        <Self as VpmovwbEmitter<A, B>>::vpmovwb(self, op0, op1);
    }
    /// `VPMOVWB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Mem, Zmm |
    /// | 4 | Xmm, Xmm |
    /// | 5 | Xmm, Ymm |
    /// | 6 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpmovwb_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpmovwbMaskEmitter<A, B>,
    {
        <Self as VpmovwbMaskEmitter<A, B>>::vpmovwb_mask(self, op0, op1);
    }
    /// `VPMOVWB_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Xmm |
    /// | 2 | Xmm, Ymm |
    /// | 3 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpmovwb_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpmovwbMaskzEmitter<A, B>,
    {
        <Self as VpmovwbMaskzEmitter<A, B>>::vpmovwb_maskz(self, op0, op1);
    }
    /// `VPMULHRSW`.
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
    pub fn vpmulhrsw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmulhrswEmitter<A, B, C>,
    {
        <Self as VpmulhrswEmitter<A, B, C>>::vpmulhrsw(self, op0, op1, op2);
    }
    /// `VPMULHRSW_MASK`.
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
    pub fn vpmulhrsw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmulhrswMaskEmitter<A, B, C>,
    {
        <Self as VpmulhrswMaskEmitter<A, B, C>>::vpmulhrsw_mask(self, op0, op1, op2);
    }
    /// `VPMULHRSW_MASKZ`.
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
    pub fn vpmulhrsw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmulhrswMaskzEmitter<A, B, C>,
    {
        <Self as VpmulhrswMaskzEmitter<A, B, C>>::vpmulhrsw_maskz(self, op0, op1, op2);
    }
    /// `VPMULHUW`.
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
    pub fn vpmulhuw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmulhuwEmitter<A, B, C>,
    {
        <Self as VpmulhuwEmitter<A, B, C>>::vpmulhuw(self, op0, op1, op2);
    }
    /// `VPMULHUW_MASK`.
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
    pub fn vpmulhuw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmulhuwMaskEmitter<A, B, C>,
    {
        <Self as VpmulhuwMaskEmitter<A, B, C>>::vpmulhuw_mask(self, op0, op1, op2);
    }
    /// `VPMULHUW_MASKZ`.
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
    pub fn vpmulhuw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmulhuwMaskzEmitter<A, B, C>,
    {
        <Self as VpmulhuwMaskzEmitter<A, B, C>>::vpmulhuw_maskz(self, op0, op1, op2);
    }
    /// `VPMULHW`.
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
    pub fn vpmulhw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmulhwEmitter<A, B, C>,
    {
        <Self as VpmulhwEmitter<A, B, C>>::vpmulhw(self, op0, op1, op2);
    }
    /// `VPMULHW_MASK`.
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
    pub fn vpmulhw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmulhwMaskEmitter<A, B, C>,
    {
        <Self as VpmulhwMaskEmitter<A, B, C>>::vpmulhw_mask(self, op0, op1, op2);
    }
    /// `VPMULHW_MASKZ`.
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
    pub fn vpmulhw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmulhwMaskzEmitter<A, B, C>,
    {
        <Self as VpmulhwMaskzEmitter<A, B, C>>::vpmulhw_maskz(self, op0, op1, op2);
    }
    /// `VPMULLW`.
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
    pub fn vpmullw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmullwEmitter<A, B, C>,
    {
        <Self as VpmullwEmitter<A, B, C>>::vpmullw(self, op0, op1, op2);
    }
    /// `VPMULLW_MASK`.
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
    pub fn vpmullw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmullwMaskEmitter<A, B, C>,
    {
        <Self as VpmullwMaskEmitter<A, B, C>>::vpmullw_mask(self, op0, op1, op2);
    }
    /// `VPMULLW_MASKZ`.
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
    pub fn vpmullw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmullwMaskzEmitter<A, B, C>,
    {
        <Self as VpmullwMaskzEmitter<A, B, C>>::vpmullw_maskz(self, op0, op1, op2);
    }
    /// `VPSADBW`.
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
    pub fn vpsadbw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsadbwEmitter<A, B, C>,
    {
        <Self as VpsadbwEmitter<A, B, C>>::vpsadbw(self, op0, op1, op2);
    }
    /// `VPSHUFB`.
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
    pub fn vpshufb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshufbEmitter<A, B, C>,
    {
        <Self as VpshufbEmitter<A, B, C>>::vpshufb(self, op0, op1, op2);
    }
    /// `VPSHUFB_MASK`.
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
    pub fn vpshufb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshufbMaskEmitter<A, B, C>,
    {
        <Self as VpshufbMaskEmitter<A, B, C>>::vpshufb_mask(self, op0, op1, op2);
    }
    /// `VPSHUFB_MASKZ`.
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
    pub fn vpshufb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshufbMaskzEmitter<A, B, C>,
    {
        <Self as VpshufbMaskzEmitter<A, B, C>>::vpshufb_maskz(self, op0, op1, op2);
    }
    /// `VPSHUFHW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpshufhw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshufhwEmitter<A, B, C>,
    {
        <Self as VpshufhwEmitter<A, B, C>>::vpshufhw(self, op0, op1, op2);
    }
    /// `VPSHUFHW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpshufhw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshufhwMaskEmitter<A, B, C>,
    {
        <Self as VpshufhwMaskEmitter<A, B, C>>::vpshufhw_mask(self, op0, op1, op2);
    }
    /// `VPSHUFHW_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpshufhw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshufhwMaskzEmitter<A, B, C>,
    {
        <Self as VpshufhwMaskzEmitter<A, B, C>>::vpshufhw_maskz(self, op0, op1, op2);
    }
    /// `VPSHUFLW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpshuflw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshuflwEmitter<A, B, C>,
    {
        <Self as VpshuflwEmitter<A, B, C>>::vpshuflw(self, op0, op1, op2);
    }
    /// `VPSHUFLW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpshuflw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshuflwMaskEmitter<A, B, C>,
    {
        <Self as VpshuflwMaskEmitter<A, B, C>>::vpshuflw_mask(self, op0, op1, op2);
    }
    /// `VPSHUFLW_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpshuflw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshuflwMaskzEmitter<A, B, C>,
    {
        <Self as VpshuflwMaskzEmitter<A, B, C>>::vpshuflw_maskz(self, op0, op1, op2);
    }
    /// `VPSLLDQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpslldq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpslldqEmitter<A, B, C>,
    {
        <Self as VpslldqEmitter<A, B, C>>::vpslldq(self, op0, op1, op2);
    }
    /// `VPSLLVW`.
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
    pub fn vpsllvw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsllvwEmitter<A, B, C>,
    {
        <Self as VpsllvwEmitter<A, B, C>>::vpsllvw(self, op0, op1, op2);
    }
    /// `VPSLLVW_MASK`.
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
    pub fn vpsllvw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsllvwMaskEmitter<A, B, C>,
    {
        <Self as VpsllvwMaskEmitter<A, B, C>>::vpsllvw_mask(self, op0, op1, op2);
    }
    /// `VPSLLVW_MASKZ`.
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
    pub fn vpsllvw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsllvwMaskzEmitter<A, B, C>,
    {
        <Self as VpsllvwMaskzEmitter<A, B, C>>::vpsllvw_maskz(self, op0, op1, op2);
    }
    /// `VPSLLW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+---------------+
    /// | #  | Operands      |
    /// +----+---------------+
    /// | 1  | Xmm, Mem, Imm |
    /// | 2  | Xmm, Xmm, Imm |
    /// | 3  | Xmm, Xmm, Mem |
    /// | 4  | Xmm, Xmm, Xmm |
    /// | 5  | Ymm, Mem, Imm |
    /// | 6  | Ymm, Ymm, Imm |
    /// | 7  | Ymm, Ymm, Mem |
    /// | 8  | Ymm, Ymm, Xmm |
    /// | 9  | Zmm, Mem, Imm |
    /// | 10 | Zmm, Zmm, Imm |
    /// | 11 | Zmm, Zmm, Mem |
    /// | 12 | Zmm, Zmm, Xmm |
    /// +----+---------------+
    /// ```
    #[inline]
    pub fn vpsllw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsllwEmitter<A, B, C>,
    {
        <Self as VpsllwEmitter<A, B, C>>::vpsllw(self, op0, op1, op2);
    }
    /// `VPSLLW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+---------------+
    /// | #  | Operands      |
    /// +----+---------------+
    /// | 1  | Xmm, Mem, Imm |
    /// | 2  | Xmm, Xmm, Imm |
    /// | 3  | Xmm, Xmm, Mem |
    /// | 4  | Xmm, Xmm, Xmm |
    /// | 5  | Ymm, Mem, Imm |
    /// | 6  | Ymm, Ymm, Imm |
    /// | 7  | Ymm, Ymm, Mem |
    /// | 8  | Ymm, Ymm, Xmm |
    /// | 9  | Zmm, Mem, Imm |
    /// | 10 | Zmm, Zmm, Imm |
    /// | 11 | Zmm, Zmm, Mem |
    /// | 12 | Zmm, Zmm, Xmm |
    /// +----+---------------+
    /// ```
    #[inline]
    pub fn vpsllw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsllwMaskEmitter<A, B, C>,
    {
        <Self as VpsllwMaskEmitter<A, B, C>>::vpsllw_mask(self, op0, op1, op2);
    }
    /// `VPSLLW_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+---------------+
    /// | #  | Operands      |
    /// +----+---------------+
    /// | 1  | Xmm, Mem, Imm |
    /// | 2  | Xmm, Xmm, Imm |
    /// | 3  | Xmm, Xmm, Mem |
    /// | 4  | Xmm, Xmm, Xmm |
    /// | 5  | Ymm, Mem, Imm |
    /// | 6  | Ymm, Ymm, Imm |
    /// | 7  | Ymm, Ymm, Mem |
    /// | 8  | Ymm, Ymm, Xmm |
    /// | 9  | Zmm, Mem, Imm |
    /// | 10 | Zmm, Zmm, Imm |
    /// | 11 | Zmm, Zmm, Mem |
    /// | 12 | Zmm, Zmm, Xmm |
    /// +----+---------------+
    /// ```
    #[inline]
    pub fn vpsllw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsllwMaskzEmitter<A, B, C>,
    {
        <Self as VpsllwMaskzEmitter<A, B, C>>::vpsllw_maskz(self, op0, op1, op2);
    }
    /// `VPSRAVW`.
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
    pub fn vpsravw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsravwEmitter<A, B, C>,
    {
        <Self as VpsravwEmitter<A, B, C>>::vpsravw(self, op0, op1, op2);
    }
    /// `VPSRAVW_MASK`.
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
    pub fn vpsravw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsravwMaskEmitter<A, B, C>,
    {
        <Self as VpsravwMaskEmitter<A, B, C>>::vpsravw_mask(self, op0, op1, op2);
    }
    /// `VPSRAVW_MASKZ`.
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
    pub fn vpsravw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsravwMaskzEmitter<A, B, C>,
    {
        <Self as VpsravwMaskzEmitter<A, B, C>>::vpsravw_maskz(self, op0, op1, op2);
    }
    /// `VPSRAW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+---------------+
    /// | #  | Operands      |
    /// +----+---------------+
    /// | 1  | Xmm, Mem, Imm |
    /// | 2  | Xmm, Xmm, Imm |
    /// | 3  | Xmm, Xmm, Mem |
    /// | 4  | Xmm, Xmm, Xmm |
    /// | 5  | Ymm, Mem, Imm |
    /// | 6  | Ymm, Ymm, Imm |
    /// | 7  | Ymm, Ymm, Mem |
    /// | 8  | Ymm, Ymm, Xmm |
    /// | 9  | Zmm, Mem, Imm |
    /// | 10 | Zmm, Zmm, Imm |
    /// | 11 | Zmm, Zmm, Mem |
    /// | 12 | Zmm, Zmm, Xmm |
    /// +----+---------------+
    /// ```
    #[inline]
    pub fn vpsraw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsrawEmitter<A, B, C>,
    {
        <Self as VpsrawEmitter<A, B, C>>::vpsraw(self, op0, op1, op2);
    }
    /// `VPSRAW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+---------------+
    /// | #  | Operands      |
    /// +----+---------------+
    /// | 1  | Xmm, Mem, Imm |
    /// | 2  | Xmm, Xmm, Imm |
    /// | 3  | Xmm, Xmm, Mem |
    /// | 4  | Xmm, Xmm, Xmm |
    /// | 5  | Ymm, Mem, Imm |
    /// | 6  | Ymm, Ymm, Imm |
    /// | 7  | Ymm, Ymm, Mem |
    /// | 8  | Ymm, Ymm, Xmm |
    /// | 9  | Zmm, Mem, Imm |
    /// | 10 | Zmm, Zmm, Imm |
    /// | 11 | Zmm, Zmm, Mem |
    /// | 12 | Zmm, Zmm, Xmm |
    /// +----+---------------+
    /// ```
    #[inline]
    pub fn vpsraw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsrawMaskEmitter<A, B, C>,
    {
        <Self as VpsrawMaskEmitter<A, B, C>>::vpsraw_mask(self, op0, op1, op2);
    }
    /// `VPSRAW_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+---------------+
    /// | #  | Operands      |
    /// +----+---------------+
    /// | 1  | Xmm, Mem, Imm |
    /// | 2  | Xmm, Xmm, Imm |
    /// | 3  | Xmm, Xmm, Mem |
    /// | 4  | Xmm, Xmm, Xmm |
    /// | 5  | Ymm, Mem, Imm |
    /// | 6  | Ymm, Ymm, Imm |
    /// | 7  | Ymm, Ymm, Mem |
    /// | 8  | Ymm, Ymm, Xmm |
    /// | 9  | Zmm, Mem, Imm |
    /// | 10 | Zmm, Zmm, Imm |
    /// | 11 | Zmm, Zmm, Mem |
    /// | 12 | Zmm, Zmm, Xmm |
    /// +----+---------------+
    /// ```
    #[inline]
    pub fn vpsraw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsrawMaskzEmitter<A, B, C>,
    {
        <Self as VpsrawMaskzEmitter<A, B, C>>::vpsraw_maskz(self, op0, op1, op2);
    }
    /// `VPSRLDQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpsrldq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsrldqEmitter<A, B, C>,
    {
        <Self as VpsrldqEmitter<A, B, C>>::vpsrldq(self, op0, op1, op2);
    }
    /// `VPSRLVW`.
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
    pub fn vpsrlvw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsrlvwEmitter<A, B, C>,
    {
        <Self as VpsrlvwEmitter<A, B, C>>::vpsrlvw(self, op0, op1, op2);
    }
    /// `VPSRLVW_MASK`.
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
    pub fn vpsrlvw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsrlvwMaskEmitter<A, B, C>,
    {
        <Self as VpsrlvwMaskEmitter<A, B, C>>::vpsrlvw_mask(self, op0, op1, op2);
    }
    /// `VPSRLVW_MASKZ`.
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
    pub fn vpsrlvw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsrlvwMaskzEmitter<A, B, C>,
    {
        <Self as VpsrlvwMaskzEmitter<A, B, C>>::vpsrlvw_maskz(self, op0, op1, op2);
    }
    /// `VPSRLW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+---------------+
    /// | #  | Operands      |
    /// +----+---------------+
    /// | 1  | Xmm, Mem, Imm |
    /// | 2  | Xmm, Xmm, Imm |
    /// | 3  | Xmm, Xmm, Mem |
    /// | 4  | Xmm, Xmm, Xmm |
    /// | 5  | Ymm, Mem, Imm |
    /// | 6  | Ymm, Ymm, Imm |
    /// | 7  | Ymm, Ymm, Mem |
    /// | 8  | Ymm, Ymm, Xmm |
    /// | 9  | Zmm, Mem, Imm |
    /// | 10 | Zmm, Zmm, Imm |
    /// | 11 | Zmm, Zmm, Mem |
    /// | 12 | Zmm, Zmm, Xmm |
    /// +----+---------------+
    /// ```
    #[inline]
    pub fn vpsrlw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsrlwEmitter<A, B, C>,
    {
        <Self as VpsrlwEmitter<A, B, C>>::vpsrlw(self, op0, op1, op2);
    }
    /// `VPSRLW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+---------------+
    /// | #  | Operands      |
    /// +----+---------------+
    /// | 1  | Xmm, Mem, Imm |
    /// | 2  | Xmm, Xmm, Imm |
    /// | 3  | Xmm, Xmm, Mem |
    /// | 4  | Xmm, Xmm, Xmm |
    /// | 5  | Ymm, Mem, Imm |
    /// | 6  | Ymm, Ymm, Imm |
    /// | 7  | Ymm, Ymm, Mem |
    /// | 8  | Ymm, Ymm, Xmm |
    /// | 9  | Zmm, Mem, Imm |
    /// | 10 | Zmm, Zmm, Imm |
    /// | 11 | Zmm, Zmm, Mem |
    /// | 12 | Zmm, Zmm, Xmm |
    /// +----+---------------+
    /// ```
    #[inline]
    pub fn vpsrlw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsrlwMaskEmitter<A, B, C>,
    {
        <Self as VpsrlwMaskEmitter<A, B, C>>::vpsrlw_mask(self, op0, op1, op2);
    }
    /// `VPSRLW_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+---------------+
    /// | #  | Operands      |
    /// +----+---------------+
    /// | 1  | Xmm, Mem, Imm |
    /// | 2  | Xmm, Xmm, Imm |
    /// | 3  | Xmm, Xmm, Mem |
    /// | 4  | Xmm, Xmm, Xmm |
    /// | 5  | Ymm, Mem, Imm |
    /// | 6  | Ymm, Ymm, Imm |
    /// | 7  | Ymm, Ymm, Mem |
    /// | 8  | Ymm, Ymm, Xmm |
    /// | 9  | Zmm, Mem, Imm |
    /// | 10 | Zmm, Zmm, Imm |
    /// | 11 | Zmm, Zmm, Mem |
    /// | 12 | Zmm, Zmm, Xmm |
    /// +----+---------------+
    /// ```
    #[inline]
    pub fn vpsrlw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsrlwMaskzEmitter<A, B, C>,
    {
        <Self as VpsrlwMaskzEmitter<A, B, C>>::vpsrlw_maskz(self, op0, op1, op2);
    }
    /// `VPSUBB`.
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
    pub fn vpsubb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubbEmitter<A, B, C>,
    {
        <Self as VpsubbEmitter<A, B, C>>::vpsubb(self, op0, op1, op2);
    }
    /// `VPSUBB_MASK`.
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
    pub fn vpsubb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubbMaskEmitter<A, B, C>,
    {
        <Self as VpsubbMaskEmitter<A, B, C>>::vpsubb_mask(self, op0, op1, op2);
    }
    /// `VPSUBB_MASKZ`.
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
    pub fn vpsubb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubbMaskzEmitter<A, B, C>,
    {
        <Self as VpsubbMaskzEmitter<A, B, C>>::vpsubb_maskz(self, op0, op1, op2);
    }
    /// `VPSUBSB`.
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
    pub fn vpsubsb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubsbEmitter<A, B, C>,
    {
        <Self as VpsubsbEmitter<A, B, C>>::vpsubsb(self, op0, op1, op2);
    }
    /// `VPSUBSB_MASK`.
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
    pub fn vpsubsb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubsbMaskEmitter<A, B, C>,
    {
        <Self as VpsubsbMaskEmitter<A, B, C>>::vpsubsb_mask(self, op0, op1, op2);
    }
    /// `VPSUBSB_MASKZ`.
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
    pub fn vpsubsb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubsbMaskzEmitter<A, B, C>,
    {
        <Self as VpsubsbMaskzEmitter<A, B, C>>::vpsubsb_maskz(self, op0, op1, op2);
    }
    /// `VPSUBSW`.
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
    pub fn vpsubsw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubswEmitter<A, B, C>,
    {
        <Self as VpsubswEmitter<A, B, C>>::vpsubsw(self, op0, op1, op2);
    }
    /// `VPSUBSW_MASK`.
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
    pub fn vpsubsw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubswMaskEmitter<A, B, C>,
    {
        <Self as VpsubswMaskEmitter<A, B, C>>::vpsubsw_mask(self, op0, op1, op2);
    }
    /// `VPSUBSW_MASKZ`.
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
    pub fn vpsubsw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubswMaskzEmitter<A, B, C>,
    {
        <Self as VpsubswMaskzEmitter<A, B, C>>::vpsubsw_maskz(self, op0, op1, op2);
    }
    /// `VPSUBUSB`.
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
    pub fn vpsubusb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubusbEmitter<A, B, C>,
    {
        <Self as VpsubusbEmitter<A, B, C>>::vpsubusb(self, op0, op1, op2);
    }
    /// `VPSUBUSB_MASK`.
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
    pub fn vpsubusb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubusbMaskEmitter<A, B, C>,
    {
        <Self as VpsubusbMaskEmitter<A, B, C>>::vpsubusb_mask(self, op0, op1, op2);
    }
    /// `VPSUBUSB_MASKZ`.
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
    pub fn vpsubusb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubusbMaskzEmitter<A, B, C>,
    {
        <Self as VpsubusbMaskzEmitter<A, B, C>>::vpsubusb_maskz(self, op0, op1, op2);
    }
    /// `VPSUBUSW`.
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
    pub fn vpsubusw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubuswEmitter<A, B, C>,
    {
        <Self as VpsubuswEmitter<A, B, C>>::vpsubusw(self, op0, op1, op2);
    }
    /// `VPSUBUSW_MASK`.
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
    pub fn vpsubusw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubuswMaskEmitter<A, B, C>,
    {
        <Self as VpsubuswMaskEmitter<A, B, C>>::vpsubusw_mask(self, op0, op1, op2);
    }
    /// `VPSUBUSW_MASKZ`.
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
    pub fn vpsubusw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubuswMaskzEmitter<A, B, C>,
    {
        <Self as VpsubuswMaskzEmitter<A, B, C>>::vpsubusw_maskz(self, op0, op1, op2);
    }
    /// `VPSUBW`.
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
    pub fn vpsubw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubwEmitter<A, B, C>,
    {
        <Self as VpsubwEmitter<A, B, C>>::vpsubw(self, op0, op1, op2);
    }
    /// `VPSUBW_MASK`.
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
    pub fn vpsubw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubwMaskEmitter<A, B, C>,
    {
        <Self as VpsubwMaskEmitter<A, B, C>>::vpsubw_mask(self, op0, op1, op2);
    }
    /// `VPSUBW_MASKZ`.
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
    pub fn vpsubw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpsubwMaskzEmitter<A, B, C>,
    {
        <Self as VpsubwMaskzEmitter<A, B, C>>::vpsubw_maskz(self, op0, op1, op2);
    }
    /// `VPTESTMB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Xmm, Mem |
    /// | 2 | KReg, Xmm, Xmm |
    /// | 3 | KReg, Ymm, Mem |
    /// | 4 | KReg, Ymm, Ymm |
    /// | 5 | KReg, Zmm, Mem |
    /// | 6 | KReg, Zmm, Zmm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vptestmb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VptestmbEmitter<A, B, C>,
    {
        <Self as VptestmbEmitter<A, B, C>>::vptestmb(self, op0, op1, op2);
    }
    /// `VPTESTMB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Xmm, Mem |
    /// | 2 | KReg, Xmm, Xmm |
    /// | 3 | KReg, Ymm, Mem |
    /// | 4 | KReg, Ymm, Ymm |
    /// | 5 | KReg, Zmm, Mem |
    /// | 6 | KReg, Zmm, Zmm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vptestmb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VptestmbMaskEmitter<A, B, C>,
    {
        <Self as VptestmbMaskEmitter<A, B, C>>::vptestmb_mask(self, op0, op1, op2);
    }
    /// `VPTESTMW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Xmm, Mem |
    /// | 2 | KReg, Xmm, Xmm |
    /// | 3 | KReg, Ymm, Mem |
    /// | 4 | KReg, Ymm, Ymm |
    /// | 5 | KReg, Zmm, Mem |
    /// | 6 | KReg, Zmm, Zmm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vptestmw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VptestmwEmitter<A, B, C>,
    {
        <Self as VptestmwEmitter<A, B, C>>::vptestmw(self, op0, op1, op2);
    }
    /// `VPTESTMW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Xmm, Mem |
    /// | 2 | KReg, Xmm, Xmm |
    /// | 3 | KReg, Ymm, Mem |
    /// | 4 | KReg, Ymm, Ymm |
    /// | 5 | KReg, Zmm, Mem |
    /// | 6 | KReg, Zmm, Zmm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vptestmw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VptestmwMaskEmitter<A, B, C>,
    {
        <Self as VptestmwMaskEmitter<A, B, C>>::vptestmw_mask(self, op0, op1, op2);
    }
    /// `VPTESTNMB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Xmm, Mem |
    /// | 2 | KReg, Xmm, Xmm |
    /// | 3 | KReg, Ymm, Mem |
    /// | 4 | KReg, Ymm, Ymm |
    /// | 5 | KReg, Zmm, Mem |
    /// | 6 | KReg, Zmm, Zmm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vptestnmb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VptestnmbEmitter<A, B, C>,
    {
        <Self as VptestnmbEmitter<A, B, C>>::vptestnmb(self, op0, op1, op2);
    }
    /// `VPTESTNMB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Xmm, Mem |
    /// | 2 | KReg, Xmm, Xmm |
    /// | 3 | KReg, Ymm, Mem |
    /// | 4 | KReg, Ymm, Ymm |
    /// | 5 | KReg, Zmm, Mem |
    /// | 6 | KReg, Zmm, Zmm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vptestnmb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VptestnmbMaskEmitter<A, B, C>,
    {
        <Self as VptestnmbMaskEmitter<A, B, C>>::vptestnmb_mask(self, op0, op1, op2);
    }
    /// `VPTESTNMW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Xmm, Mem |
    /// | 2 | KReg, Xmm, Xmm |
    /// | 3 | KReg, Ymm, Mem |
    /// | 4 | KReg, Ymm, Ymm |
    /// | 5 | KReg, Zmm, Mem |
    /// | 6 | KReg, Zmm, Zmm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vptestnmw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VptestnmwEmitter<A, B, C>,
    {
        <Self as VptestnmwEmitter<A, B, C>>::vptestnmw(self, op0, op1, op2);
    }
    /// `VPTESTNMW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Xmm, Mem |
    /// | 2 | KReg, Xmm, Xmm |
    /// | 3 | KReg, Ymm, Mem |
    /// | 4 | KReg, Ymm, Ymm |
    /// | 5 | KReg, Zmm, Mem |
    /// | 6 | KReg, Zmm, Zmm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vptestnmw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VptestnmwMaskEmitter<A, B, C>,
    {
        <Self as VptestnmwMaskEmitter<A, B, C>>::vptestnmw_mask(self, op0, op1, op2);
    }
    /// `VPUNPCKHBW`.
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
    pub fn vpunpckhbw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpckhbwEmitter<A, B, C>,
    {
        <Self as VpunpckhbwEmitter<A, B, C>>::vpunpckhbw(self, op0, op1, op2);
    }
    /// `VPUNPCKHBW_MASK`.
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
    pub fn vpunpckhbw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpckhbwMaskEmitter<A, B, C>,
    {
        <Self as VpunpckhbwMaskEmitter<A, B, C>>::vpunpckhbw_mask(self, op0, op1, op2);
    }
    /// `VPUNPCKHBW_MASKZ`.
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
    pub fn vpunpckhbw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpckhbwMaskzEmitter<A, B, C>,
    {
        <Self as VpunpckhbwMaskzEmitter<A, B, C>>::vpunpckhbw_maskz(self, op0, op1, op2);
    }
    /// `VPUNPCKHWD`.
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
    pub fn vpunpckhwd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpckhwdEmitter<A, B, C>,
    {
        <Self as VpunpckhwdEmitter<A, B, C>>::vpunpckhwd(self, op0, op1, op2);
    }
    /// `VPUNPCKHWD_MASK`.
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
    pub fn vpunpckhwd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpckhwdMaskEmitter<A, B, C>,
    {
        <Self as VpunpckhwdMaskEmitter<A, B, C>>::vpunpckhwd_mask(self, op0, op1, op2);
    }
    /// `VPUNPCKHWD_MASKZ`.
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
    pub fn vpunpckhwd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpckhwdMaskzEmitter<A, B, C>,
    {
        <Self as VpunpckhwdMaskzEmitter<A, B, C>>::vpunpckhwd_maskz(self, op0, op1, op2);
    }
    /// `VPUNPCKLBW`.
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
    pub fn vpunpcklbw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpcklbwEmitter<A, B, C>,
    {
        <Self as VpunpcklbwEmitter<A, B, C>>::vpunpcklbw(self, op0, op1, op2);
    }
    /// `VPUNPCKLBW_MASK`.
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
    pub fn vpunpcklbw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpcklbwMaskEmitter<A, B, C>,
    {
        <Self as VpunpcklbwMaskEmitter<A, B, C>>::vpunpcklbw_mask(self, op0, op1, op2);
    }
    /// `VPUNPCKLBW_MASKZ`.
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
    pub fn vpunpcklbw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpcklbwMaskzEmitter<A, B, C>,
    {
        <Self as VpunpcklbwMaskzEmitter<A, B, C>>::vpunpcklbw_maskz(self, op0, op1, op2);
    }
    /// `VPUNPCKLWD`.
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
    pub fn vpunpcklwd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpcklwdEmitter<A, B, C>,
    {
        <Self as VpunpcklwdEmitter<A, B, C>>::vpunpcklwd(self, op0, op1, op2);
    }
    /// `VPUNPCKLWD_MASK`.
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
    pub fn vpunpcklwd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpcklwdMaskEmitter<A, B, C>,
    {
        <Self as VpunpcklwdMaskEmitter<A, B, C>>::vpunpcklwd_mask(self, op0, op1, op2);
    }
    /// `VPUNPCKLWD_MASKZ`.
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
    pub fn vpunpcklwd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpunpcklwdMaskzEmitter<A, B, C>,
    {
        <Self as VpunpcklwdMaskzEmitter<A, B, C>>::vpunpcklwd_maskz(self, op0, op1, op2);
    }
}
