use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VP2INTERSECTD`.
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
pub trait Vp2intersectdEmitter<A, B, C> {
    fn vp2intersectd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vp2intersectdEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vp2intersectd(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VP2INTERSECTD128KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vp2intersectdEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vp2intersectd(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VP2INTERSECTD128KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vp2intersectdEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vp2intersectd(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VP2INTERSECTD256KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vp2intersectdEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vp2intersectd(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VP2INTERSECTD256KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vp2intersectdEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vp2intersectd(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VP2INTERSECTD512KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vp2intersectdEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vp2intersectd(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VP2INTERSECTD512KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VP2INTERSECTQ`.
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
pub trait Vp2intersectqEmitter<A, B, C> {
    fn vp2intersectq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vp2intersectqEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vp2intersectq(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(
            VP2INTERSECTQ128KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vp2intersectqEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vp2intersectq(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(
            VP2INTERSECTQ128KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vp2intersectqEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vp2intersectq(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(
            VP2INTERSECTQ256KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vp2intersectqEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vp2intersectq(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(
            VP2INTERSECTQ256KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vp2intersectqEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vp2intersectq(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(
            VP2INTERSECTQ512KRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vp2intersectqEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vp2intersectq(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(
            VP2INTERSECTQ512KRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `VP2INTERSECTD`.
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
    pub fn vp2intersectd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vp2intersectdEmitter<A, B, C>,
    {
        <Self as Vp2intersectdEmitter<A, B, C>>::vp2intersectd(self, op0, op1, op2);
    }
    /// `VP2INTERSECTQ`.
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
    pub fn vp2intersectq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vp2intersectqEmitter<A, B, C>,
    {
        <Self as Vp2intersectqEmitter<A, B, C>>::vp2intersectq(self, op0, op1, op2);
    }
}
