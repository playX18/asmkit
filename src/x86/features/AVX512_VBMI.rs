use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VPERMB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpermbEmitter<A, B, C> {
    fn vpermb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpermbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpermbMaskEmitter<A, B, C> {
    fn vpermb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpermbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMB_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpermbMaskzEmitter<A, B, C> {
    fn vpermb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpermbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpermbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMI2B`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpermi2bEmitter<A, B, C> {
    fn vpermi2b(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermi2bEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermi2b(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMI2B128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermi2b(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMI2B128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermi2b(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMI2B256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermi2b(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMI2B256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermi2b(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMI2B512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermi2b(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMI2B512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMI2B_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpermi2bMaskEmitter<A, B, C> {
    fn vpermi2b_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermi2bMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermi2b_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMI2B128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermi2b_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMI2B128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermi2b_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMI2B256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermi2b_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMI2B256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermi2b_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMI2B512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermi2b_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMI2B512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMI2B_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpermi2bMaskzEmitter<A, B, C> {
    fn vpermi2b_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermi2bMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermi2b_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMI2B128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermi2b_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMI2B128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermi2b_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMI2B256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermi2b_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMI2B256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermi2b_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMI2B512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermi2bMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermi2b_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMI2B512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMT2B`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpermt2bEmitter<A, B, C> {
    fn vpermt2b(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermt2bEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermt2b(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMT2B128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermt2b(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMT2B128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermt2b(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMT2B256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermt2b(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMT2B256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermt2b(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMT2B512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermt2b(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMT2B512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMT2B_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpermt2bMaskEmitter<A, B, C> {
    fn vpermt2b_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermt2bMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermt2b_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMT2B128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermt2b_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMT2B128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermt2b_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMT2B256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermt2b_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMT2B256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermt2b_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMT2B512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermt2b_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMT2B512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPERMT2B_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vpermt2bMaskzEmitter<A, B, C> {
    fn vpermt2b_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vpermt2bMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpermt2b_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPERMT2B128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpermt2b_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPERMT2B128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpermt2b_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPERMT2B256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpermt2b_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPERMT2B256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpermt2b_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPERMT2B512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vpermt2bMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpermt2b_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPERMT2B512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULTISHIFTQB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpmultishiftqbEmitter<A, B, C> {
    fn vpmultishiftqb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmultishiftqbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmultishiftqb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULTISHIFTQB128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmultishiftqb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULTISHIFTQB128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmultishiftqb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULTISHIFTQB256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmultishiftqb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULTISHIFTQB256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmultishiftqb(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULTISHIFTQB512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmultishiftqb(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULTISHIFTQB512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULTISHIFTQB_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpmultishiftqbMaskEmitter<A, B, C> {
    fn vpmultishiftqb_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmultishiftqbMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmultishiftqb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULTISHIFTQB128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmultishiftqb_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULTISHIFTQB128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmultishiftqb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULTISHIFTQB256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmultishiftqb_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULTISHIFTQB256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmultishiftqb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULTISHIFTQB512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmultishiftqb_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULTISHIFTQB512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPMULTISHIFTQB_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpmultishiftqbMaskzEmitter<A, B, C> {
    fn vpmultishiftqb_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmultishiftqbMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmultishiftqb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPMULTISHIFTQB128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmultishiftqb_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPMULTISHIFTQB128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmultishiftqb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPMULTISHIFTQB256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmultishiftqb_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPMULTISHIFTQB256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmultishiftqb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPMULTISHIFTQB512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpmultishiftqbMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmultishiftqb_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPMULTISHIFTQB512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `VPERMB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpermb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpermbEmitter<A, B, C>,
    {
        <Self as VpermbEmitter<A, B, C>>::vpermb(self, op0, op1, op2);
    }
    /// `VPERMB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpermb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpermbMaskEmitter<A, B, C>,
    {
        <Self as VpermbMaskEmitter<A, B, C>>::vpermb_mask(self, op0, op1, op2);
    }
    /// `VPERMB_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpermb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpermbMaskzEmitter<A, B, C>,
    {
        <Self as VpermbMaskzEmitter<A, B, C>>::vpermb_maskz(self, op0, op1, op2);
    }
    /// `VPERMI2B`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpermi2b<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermi2bEmitter<A, B, C>,
    {
        <Self as Vpermi2bEmitter<A, B, C>>::vpermi2b(self, op0, op1, op2);
    }
    /// `VPERMI2B_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpermi2b_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermi2bMaskEmitter<A, B, C>,
    {
        <Self as Vpermi2bMaskEmitter<A, B, C>>::vpermi2b_mask(self, op0, op1, op2);
    }
    /// `VPERMI2B_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpermi2b_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermi2bMaskzEmitter<A, B, C>,
    {
        <Self as Vpermi2bMaskzEmitter<A, B, C>>::vpermi2b_maskz(self, op0, op1, op2);
    }
    /// `VPERMT2B`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpermt2b<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermt2bEmitter<A, B, C>,
    {
        <Self as Vpermt2bEmitter<A, B, C>>::vpermt2b(self, op0, op1, op2);
    }
    /// `VPERMT2B_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpermt2b_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermt2bMaskEmitter<A, B, C>,
    {
        <Self as Vpermt2bMaskEmitter<A, B, C>>::vpermt2b_mask(self, op0, op1, op2);
    }
    /// `VPERMT2B_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpermt2b_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vpermt2bMaskzEmitter<A, B, C>,
    {
        <Self as Vpermt2bMaskzEmitter<A, B, C>>::vpermt2b_maskz(self, op0, op1, op2);
    }
    /// `VPMULTISHIFTQB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpmultishiftqb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmultishiftqbEmitter<A, B, C>,
    {
        <Self as VpmultishiftqbEmitter<A, B, C>>::vpmultishiftqb(self, op0, op1, op2);
    }
    /// `VPMULTISHIFTQB_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpmultishiftqb_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmultishiftqbMaskEmitter<A, B, C>,
    {
        <Self as VpmultishiftqbMaskEmitter<A, B, C>>::vpmultishiftqb_mask(self, op0, op1, op2);
    }
    /// `VPMULTISHIFTQB_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpmultishiftqb_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpmultishiftqbMaskzEmitter<A, B, C>,
    {
        <Self as VpmultishiftqbMaskzEmitter<A, B, C>>::vpmultishiftqb_maskz(self, op0, op1, op2);
    }
}
