use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `AESDEC128KL`.
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
pub trait Aesdec128klEmitter<A, B> {
    fn aesdec128kl(&mut self, op0: A, op1: B);
}

impl<'a> Aesdec128klEmitter<Xmm, Mem> for Assembler<'a> {
    fn aesdec128kl(&mut self, op0: Xmm, op1: Mem) {
        self.emit(AESDEC128KLRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `AESDEC256KL`.
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
pub trait Aesdec256klEmitter<A, B> {
    fn aesdec256kl(&mut self, op0: A, op1: B);
}

impl<'a> Aesdec256klEmitter<Xmm, Mem> for Assembler<'a> {
    fn aesdec256kl(&mut self, op0: Xmm, op1: Mem) {
        self.emit(AESDEC256KLRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `AESDECWIDE128KL`.
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
pub trait Aesdecwide128klEmitter<A> {
    fn aesdecwide128kl(&mut self, op0: A);
}

impl<'a> Aesdecwide128klEmitter<Mem> for Assembler<'a> {
    fn aesdecwide128kl(&mut self, op0: Mem) {
        self.emit(AESDECWIDE128KLM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `AESDECWIDE256KL`.
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
pub trait Aesdecwide256klEmitter<A> {
    fn aesdecwide256kl(&mut self, op0: A);
}

impl<'a> Aesdecwide256klEmitter<Mem> for Assembler<'a> {
    fn aesdecwide256kl(&mut self, op0: Mem) {
        self.emit(AESDECWIDE256KLM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `AESENC128KL`.
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
pub trait Aesenc128klEmitter<A, B> {
    fn aesenc128kl(&mut self, op0: A, op1: B);
}

impl<'a> Aesenc128klEmitter<Xmm, Mem> for Assembler<'a> {
    fn aesenc128kl(&mut self, op0: Xmm, op1: Mem) {
        self.emit(AESENC128KLRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `AESENC256KL`.
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
pub trait Aesenc256klEmitter<A, B> {
    fn aesenc256kl(&mut self, op0: A, op1: B);
}

impl<'a> Aesenc256klEmitter<Xmm, Mem> for Assembler<'a> {
    fn aesenc256kl(&mut self, op0: Xmm, op1: Mem) {
        self.emit(AESENC256KLRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `AESENCWIDE128KL`.
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
pub trait Aesencwide128klEmitter<A> {
    fn aesencwide128kl(&mut self, op0: A);
}

impl<'a> Aesencwide128klEmitter<Mem> for Assembler<'a> {
    fn aesencwide128kl(&mut self, op0: Mem) {
        self.emit(AESENCWIDE128KLM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `AESENCWIDE256KL`.
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
pub trait Aesencwide256klEmitter<A> {
    fn aesencwide256kl(&mut self, op0: A);
}

impl<'a> Aesencwide256klEmitter<Mem> for Assembler<'a> {
    fn aesencwide256kl(&mut self, op0: Mem) {
        self.emit(AESENCWIDE256KLM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `ENCODEKEY128`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Gpd |
/// +---+----------+
/// ```
pub trait Encodekey128Emitter<A, B> {
    fn encodekey128(&mut self, op0: A, op1: B);
}

impl<'a> Encodekey128Emitter<Gpd, Gpd> for Assembler<'a> {
    fn encodekey128(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(ENCODEKEY128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `ENCODEKEY256`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Gpd |
/// +---+----------+
/// ```
pub trait Encodekey256Emitter<A, B> {
    fn encodekey256(&mut self, op0: A, op1: B);
}

impl<'a> Encodekey256Emitter<Gpd, Gpd> for Assembler<'a> {
    fn encodekey256(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(ENCODEKEY256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `LOADIWKEY`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait LoadiwkeyEmitter<A, B> {
    fn loadiwkey(&mut self, op0: A, op1: B);
}

impl<'a> LoadiwkeyEmitter<Xmm, Xmm> for Assembler<'a> {
    fn loadiwkey(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(LOADIWKEYRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `AESDEC128KL`.
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
    pub fn aesdec128kl<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Aesdec128klEmitter<A, B> {
        <Self as Aesdec128klEmitter<A, B>>::aesdec128kl(self, op0, op1);
    }
    /// `AESDEC256KL`.
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
    pub fn aesdec256kl<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Aesdec256klEmitter<A, B> {
        <Self as Aesdec256klEmitter<A, B>>::aesdec256kl(self, op0, op1);
    }
    /// `AESDECWIDE128KL`.
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
    pub fn aesdecwide128kl<A>(&mut self, op0: A)
    where Assembler<'a>: Aesdecwide128klEmitter<A> {
        <Self as Aesdecwide128klEmitter<A>>::aesdecwide128kl(self, op0);
    }
    /// `AESDECWIDE256KL`.
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
    pub fn aesdecwide256kl<A>(&mut self, op0: A)
    where Assembler<'a>: Aesdecwide256klEmitter<A> {
        <Self as Aesdecwide256klEmitter<A>>::aesdecwide256kl(self, op0);
    }
    /// `AESENC128KL`.
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
    pub fn aesenc128kl<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Aesenc128klEmitter<A, B> {
        <Self as Aesenc128klEmitter<A, B>>::aesenc128kl(self, op0, op1);
    }
    /// `AESENC256KL`.
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
    pub fn aesenc256kl<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Aesenc256klEmitter<A, B> {
        <Self as Aesenc256klEmitter<A, B>>::aesenc256kl(self, op0, op1);
    }
    /// `AESENCWIDE128KL`.
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
    pub fn aesencwide128kl<A>(&mut self, op0: A)
    where Assembler<'a>: Aesencwide128klEmitter<A> {
        <Self as Aesencwide128klEmitter<A>>::aesencwide128kl(self, op0);
    }
    /// `AESENCWIDE256KL`.
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
    pub fn aesencwide256kl<A>(&mut self, op0: A)
    where Assembler<'a>: Aesencwide256klEmitter<A> {
        <Self as Aesencwide256klEmitter<A>>::aesencwide256kl(self, op0);
    }
    /// `ENCODEKEY128`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Gpd |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn encodekey128<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Encodekey128Emitter<A, B> {
        <Self as Encodekey128Emitter<A, B>>::encodekey128(self, op0, op1);
    }
    /// `ENCODEKEY256`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Gpd |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn encodekey256<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Encodekey256Emitter<A, B> {
        <Self as Encodekey256Emitter<A, B>>::encodekey256(self, op0, op1);
    }
    /// `LOADIWKEY`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn loadiwkey<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: LoadiwkeyEmitter<A, B> {
        <Self as LoadiwkeyEmitter<A, B>>::loadiwkey(self, op0, op1);
    }
}
