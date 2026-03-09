use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VPCOMPRESSB`.
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
/// | 5 | Ymm, Ymm |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpcompressbEmitter<A, B> {
    fn vpcompressb(&mut self, op0: A, op1: B);
}

impl<'a> VpcompressbEmitter<Mem, Xmm> for Assembler<'a> {
    fn vpcompressb(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VPCOMPRESSB128MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbEmitter<Mem, Ymm> for Assembler<'a> {
    fn vpcompressb(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VPCOMPRESSB256MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbEmitter<Mem, Zmm> for Assembler<'a> {
    fn vpcompressb(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VPCOMPRESSB512MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpcompressb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCOMPRESSB128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpcompressb(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCOMPRESSB256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpcompressb(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCOMPRESSB512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCOMPRESSB_MASK`.
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
/// | 5 | Ymm, Ymm |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpcompressbMaskEmitter<A, B> {
    fn vpcompressb_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpcompressbMaskEmitter<Mem, Xmm> for Assembler<'a> {
    fn vpcompressb_mask(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VPCOMPRESSB128MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbMaskEmitter<Mem, Ymm> for Assembler<'a> {
    fn vpcompressb_mask(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VPCOMPRESSB256MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbMaskEmitter<Mem, Zmm> for Assembler<'a> {
    fn vpcompressb_mask(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VPCOMPRESSB512MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpcompressb_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCOMPRESSB128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpcompressb_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCOMPRESSB256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpcompressb_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCOMPRESSB512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCOMPRESSB_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Xmm |
/// | 2 | Ymm, Ymm |
/// | 3 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpcompressbMaskzEmitter<A, B> {
    fn vpcompressb_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpcompressbMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpcompressb_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCOMPRESSB128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpcompressb_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCOMPRESSB256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompressbMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpcompressb_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCOMPRESSB512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCOMPRESSW`.
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
/// | 5 | Ymm, Ymm |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpcompresswEmitter<A, B> {
    fn vpcompressw(&mut self, op0: A, op1: B);
}

impl<'a> VpcompresswEmitter<Mem, Xmm> for Assembler<'a> {
    fn vpcompressw(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VPCOMPRESSW128MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswEmitter<Mem, Ymm> for Assembler<'a> {
    fn vpcompressw(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VPCOMPRESSW256MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswEmitter<Mem, Zmm> for Assembler<'a> {
    fn vpcompressw(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VPCOMPRESSW512MR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpcompressw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCOMPRESSW128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpcompressw(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCOMPRESSW256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpcompressw(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCOMPRESSW512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCOMPRESSW_MASK`.
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
/// | 5 | Ymm, Ymm |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpcompresswMaskEmitter<A, B> {
    fn vpcompressw_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpcompresswMaskEmitter<Mem, Xmm> for Assembler<'a> {
    fn vpcompressw_mask(&mut self, op0: Mem, op1: Xmm) {
        self.emit(
            VPCOMPRESSW128MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswMaskEmitter<Mem, Ymm> for Assembler<'a> {
    fn vpcompressw_mask(&mut self, op0: Mem, op1: Ymm) {
        self.emit(
            VPCOMPRESSW256MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswMaskEmitter<Mem, Zmm> for Assembler<'a> {
    fn vpcompressw_mask(&mut self, op0: Mem, op1: Zmm) {
        self.emit(
            VPCOMPRESSW512MR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpcompressw_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCOMPRESSW128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpcompressw_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCOMPRESSW256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpcompressw_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCOMPRESSW512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPCOMPRESSW_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Xmm |
/// | 2 | Ymm, Ymm |
/// | 3 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpcompresswMaskzEmitter<A, B> {
    fn vpcompressw_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpcompresswMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpcompressw_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPCOMPRESSW128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpcompressw_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPCOMPRESSW256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpcompresswMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpcompressw_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPCOMPRESSW512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPEXPANDB`.
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
pub trait VpexpandbEmitter<A, B> {
    fn vpexpandb(&mut self, op0: A, op1: B);
}

impl<'a> VpexpandbEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpexpandb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPEXPANDB128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpexpandb(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPEXPANDB256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpexpandb(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPEXPANDB512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpexpandb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPEXPANDB128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpexpandb(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPEXPANDB256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpexpandb(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPEXPANDB512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPEXPANDB_MASK`.
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
pub trait VpexpandbMaskEmitter<A, B> {
    fn vpexpandb_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpexpandbMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpexpandb_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPEXPANDB128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpexpandb_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPEXPANDB256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpexpandb_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPEXPANDB512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpexpandb_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPEXPANDB128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpexpandb_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPEXPANDB256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpexpandb_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPEXPANDB512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPEXPANDB_MASKZ`.
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
pub trait VpexpandbMaskzEmitter<A, B> {
    fn vpexpandb_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpexpandbMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpexpandb_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPEXPANDB128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpexpandb_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPEXPANDB256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpexpandb_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPEXPANDB512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpexpandb_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPEXPANDB128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpexpandb_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPEXPANDB256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandbMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpexpandb_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPEXPANDB512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPEXPANDW`.
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
pub trait VpexpandwEmitter<A, B> {
    fn vpexpandw(&mut self, op0: A, op1: B);
}

impl<'a> VpexpandwEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpexpandw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPEXPANDW128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpexpandw(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPEXPANDW256RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpexpandw(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPEXPANDW512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpexpandw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPEXPANDW128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpexpandw(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPEXPANDW256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpexpandw(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPEXPANDW512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPEXPANDW_MASK`.
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
pub trait VpexpandwMaskEmitter<A, B> {
    fn vpexpandw_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpexpandwMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpexpandw_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPEXPANDW128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpexpandw_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPEXPANDW256RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpexpandw_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPEXPANDW512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpexpandw_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPEXPANDW128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpexpandw_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPEXPANDW256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpexpandw_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPEXPANDW512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPEXPANDW_MASKZ`.
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
pub trait VpexpandwMaskzEmitter<A, B> {
    fn vpexpandw_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpexpandwMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpexpandw_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VPEXPANDW128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpexpandw_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VPEXPANDW256RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpexpandw_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(
            VPEXPANDW512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpexpandw_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VPEXPANDW128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpexpandw_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(
            VPEXPANDW256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> VpexpandwMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpexpandw_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(
            VPEXPANDW512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VPSHLDD`.
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
pub trait VpshlddEmitter<A, B, C, D> {
    fn vpshldd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshlddEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshldd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHLDD128RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshldd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDD128RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshldd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHLDD256RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshldd(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDD256RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshldd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHLDD512RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshldd(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDD512RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHLDD_MASK`.
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
pub trait VpshlddMaskEmitter<A, B, C, D> {
    fn vpshldd_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshlddMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshldd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHLDD128RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshldd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDD128RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddMaskEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshldd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHLDD256RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddMaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshldd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDD256RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddMaskEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshldd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHLDD512RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddMaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshldd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDD512RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHLDD_MASKZ`.
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
pub trait VpshlddMaskzEmitter<A, B, C, D> {
    fn vpshldd_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshlddMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshldd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHLDD128RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshldd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDD128RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddMaskzEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshldd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHLDD256RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddMaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshldd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDD256RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddMaskzEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshldd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHLDD512RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshlddMaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshldd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDD512RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHLDQ`.
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
pub trait VpshldqEmitter<A, B, C, D> {
    fn vpshldq(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshldqEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshldq(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHLDQ128RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshldq(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDQ128RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshldq(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHLDQ256RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshldq(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDQ256RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshldq(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHLDQ512RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshldq(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDQ512RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHLDQ_MASK`.
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
pub trait VpshldqMaskEmitter<A, B, C, D> {
    fn vpshldq_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshldqMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshldq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHLDQ128RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshldq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDQ128RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqMaskEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshldq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHLDQ256RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqMaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshldq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDQ256RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqMaskEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshldq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHLDQ512RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqMaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshldq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDQ512RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHLDQ_MASKZ`.
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
pub trait VpshldqMaskzEmitter<A, B, C, D> {
    fn vpshldq_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshldqMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshldq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHLDQ128RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshldq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDQ128RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqMaskzEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshldq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHLDQ256RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqMaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshldq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDQ256RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqMaskzEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshldq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHLDQ512RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldqMaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshldq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDQ512RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHLDVD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshldvdEmitter<A, B, C> {
    fn vpshldvd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshldvdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshldvd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHLDVD128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshldvd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHLDVD128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshldvd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHLDVD256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshldvd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHLDVD256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshldvd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHLDVD512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshldvd(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHLDVD512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHLDVD_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshldvdMaskEmitter<A, B, C> {
    fn vpshldvd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshldvdMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshldvd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHLDVD128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshldvd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHLDVD128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshldvd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHLDVD256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshldvd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHLDVD256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshldvd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHLDVD512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshldvd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHLDVD512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHLDVD_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshldvdMaskzEmitter<A, B, C> {
    fn vpshldvd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshldvdMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshldvd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHLDVD128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshldvd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHLDVD128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshldvd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHLDVD256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshldvd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHLDVD256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshldvd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHLDVD512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvdMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshldvd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHLDVD512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHLDVQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshldvqEmitter<A, B, C> {
    fn vpshldvq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshldvqEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshldvq(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHLDVQ128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshldvq(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHLDVQ128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshldvq(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHLDVQ256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshldvq(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHLDVQ256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshldvq(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHLDVQ512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshldvq(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHLDVQ512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHLDVQ_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshldvqMaskEmitter<A, B, C> {
    fn vpshldvq_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshldvqMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshldvq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHLDVQ128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshldvq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHLDVQ128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshldvq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHLDVQ256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshldvq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHLDVQ256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshldvq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHLDVQ512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshldvq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHLDVQ512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHLDVQ_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshldvqMaskzEmitter<A, B, C> {
    fn vpshldvq_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshldvqMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshldvq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHLDVQ128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshldvq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHLDVQ128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshldvq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHLDVQ256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshldvq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHLDVQ256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshldvq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHLDVQ512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvqMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshldvq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHLDVQ512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHLDVW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshldvwEmitter<A, B, C> {
    fn vpshldvw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshldvwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshldvw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHLDVW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshldvw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHLDVW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshldvw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHLDVW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshldvw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHLDVW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshldvw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHLDVW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshldvw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHLDVW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHLDVW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshldvwMaskEmitter<A, B, C> {
    fn vpshldvw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshldvwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshldvw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHLDVW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshldvw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHLDVW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshldvw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHLDVW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshldvw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHLDVW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshldvw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHLDVW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshldvw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHLDVW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHLDVW_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshldvwMaskzEmitter<A, B, C> {
    fn vpshldvw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshldvwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshldvw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHLDVW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshldvw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHLDVW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshldvw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHLDVW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshldvw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHLDVW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshldvw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHLDVW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshldvwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshldvw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHLDVW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHLDW`.
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
pub trait VpshldwEmitter<A, B, C, D> {
    fn vpshldw(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshldwEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshldw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHLDW128RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshldw(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDW128RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshldw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHLDW256RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshldw(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDW256RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshldw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHLDW512RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshldw(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDW512RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHLDW_MASK`.
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
pub trait VpshldwMaskEmitter<A, B, C, D> {
    fn vpshldw_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshldwMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshldw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHLDW128RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshldw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDW128RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwMaskEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshldw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHLDW256RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwMaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshldw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDW256RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwMaskEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshldw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHLDW512RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwMaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshldw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDW512RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHLDW_MASKZ`.
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
pub trait VpshldwMaskzEmitter<A, B, C, D> {
    fn vpshldw_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshldwMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshldw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHLDW128RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshldw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDW128RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwMaskzEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshldw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHLDW256RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwMaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshldw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDW256RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwMaskzEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshldw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHLDW512RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshldwMaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshldw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHLDW512RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHRDD`.
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
pub trait VpshrddEmitter<A, B, C, D> {
    fn vpshrdd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshrddEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshrdd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHRDD128RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDD128RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshrdd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHRDD256RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshrdd(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDD256RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshrdd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHRDD512RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdd(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDD512RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHRDD_MASK`.
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
pub trait VpshrddMaskEmitter<A, B, C, D> {
    fn vpshrdd_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshrddMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshrdd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHRDD128RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDD128RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddMaskEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshrdd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHRDD256RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddMaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshrdd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDD256RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddMaskEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshrdd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHRDD512RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddMaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDD512RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHRDD_MASKZ`.
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
pub trait VpshrddMaskzEmitter<A, B, C, D> {
    fn vpshrdd_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshrddMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshrdd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHRDD128RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDD128RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddMaskzEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshrdd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHRDD256RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddMaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshrdd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDD256RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddMaskzEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshrdd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHRDD512RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrddMaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDD512RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHRDQ`.
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
pub trait VpshrdqEmitter<A, B, C, D> {
    fn vpshrdq(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshrdqEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshrdq(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHRDQ128RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdq(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDQ128RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshrdq(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHRDQ256RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshrdq(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDQ256RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshrdq(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHRDQ512RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdq(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDQ512RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHRDQ_MASK`.
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
pub trait VpshrdqMaskEmitter<A, B, C, D> {
    fn vpshrdq_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshrdqMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshrdq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHRDQ128RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDQ128RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqMaskEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshrdq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHRDQ256RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqMaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshrdq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDQ256RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqMaskEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshrdq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHRDQ512RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqMaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDQ512RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHRDQ_MASKZ`.
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
pub trait VpshrdqMaskzEmitter<A, B, C, D> {
    fn vpshrdq_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshrdqMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshrdq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHRDQ128RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDQ128RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqMaskzEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshrdq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHRDQ256RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqMaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshrdq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDQ256RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqMaskzEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshrdq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHRDQ512RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdqMaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDQ512RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHRDVD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshrdvdEmitter<A, B, C> {
    fn vpshrdvd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshrdvdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshrdvd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHRDVD128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshrdvd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHRDVD128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshrdvd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHRDVD256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshrdvd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHRDVD256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshrdvd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHRDVD512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshrdvd(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHRDVD512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHRDVD_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshrdvdMaskEmitter<A, B, C> {
    fn vpshrdvd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshrdvdMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshrdvd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHRDVD128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshrdvd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHRDVD128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshrdvd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHRDVD256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshrdvd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHRDVD256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshrdvd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHRDVD512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshrdvd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHRDVD512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHRDVD_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshrdvdMaskzEmitter<A, B, C> {
    fn vpshrdvd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshrdvdMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshrdvd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHRDVD128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshrdvd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHRDVD128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshrdvd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHRDVD256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshrdvd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHRDVD256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshrdvd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHRDVD512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvdMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshrdvd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHRDVD512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHRDVQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshrdvqEmitter<A, B, C> {
    fn vpshrdvq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshrdvqEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshrdvq(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHRDVQ128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshrdvq(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHRDVQ128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshrdvq(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHRDVQ256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshrdvq(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHRDVQ256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshrdvq(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHRDVQ512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshrdvq(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHRDVQ512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHRDVQ_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshrdvqMaskEmitter<A, B, C> {
    fn vpshrdvq_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshrdvqMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshrdvq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHRDVQ128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshrdvq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHRDVQ128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshrdvq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHRDVQ256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshrdvq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHRDVQ256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshrdvq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHRDVQ512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshrdvq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHRDVQ512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHRDVQ_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshrdvqMaskzEmitter<A, B, C> {
    fn vpshrdvq_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshrdvqMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshrdvq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHRDVQ128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshrdvq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHRDVQ128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshrdvq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHRDVQ256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshrdvq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHRDVQ256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshrdvq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHRDVQ512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvqMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshrdvq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHRDVQ512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHRDVW`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshrdvwEmitter<A, B, C> {
    fn vpshrdvw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshrdvwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshrdvw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHRDVW128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshrdvw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHRDVW128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshrdvw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHRDVW256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshrdvw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHRDVW256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshrdvw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHRDVW512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshrdvw(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHRDVW512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHRDVW_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshrdvwMaskEmitter<A, B, C> {
    fn vpshrdvw_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshrdvwMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshrdvw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHRDVW128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshrdvw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHRDVW128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshrdvw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHRDVW256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshrdvw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHRDVW256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshrdvw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHRDVW512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshrdvw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHRDVW512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHRDVW_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpshrdvwMaskzEmitter<A, B, C> {
    fn vpshrdvw_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpshrdvwMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpshrdvw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VPSHRDVW128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpshrdvw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VPSHRDVW128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpshrdvw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VPSHRDVW256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpshrdvw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VPSHRDVW256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpshrdvw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VPSHRDVW512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> VpshrdvwMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpshrdvw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VPSHRDVW512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VPSHRDW`.
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
pub trait VpshrdwEmitter<A, B, C, D> {
    fn vpshrdw(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshrdwEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshrdw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHRDW128RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdw(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDW128RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshrdw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHRDW256RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshrdw(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDW256RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshrdw(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHRDW512RRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdw(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDW512RRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHRDW_MASK`.
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
pub trait VpshrdwMaskEmitter<A, B, C, D> {
    fn vpshrdw_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshrdwMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshrdw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHRDW128RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdw_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDW128RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwMaskEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshrdw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHRDW256RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwMaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshrdw_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDW256RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwMaskEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshrdw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHRDW512RRRI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwMaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdw_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDW512RRMI_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

/// `VPSHRDW_MASKZ`.
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
pub trait VpshrdwMaskzEmitter<A, B, C, D> {
    fn vpshrdw_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpshrdwMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpshrdw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(
            VPSHRDW128RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdw_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDW128RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwMaskzEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpshrdw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(
            VPSHRDW256RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwMaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpshrdw_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDW256RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwMaskzEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vpshrdw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(
            VPSHRDW512RRRI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> VpshrdwMaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vpshrdw_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(
            VPSHRDW512RRMI_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            op3.as_operand(),
        );
    }
}

impl<'a> Assembler<'a> {
    /// `VPCOMPRESSB`.
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
    /// | 5 | Ymm, Ymm |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpcompressb<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpcompressbEmitter<A, B>,
    {
        <Self as VpcompressbEmitter<A, B>>::vpcompressb(self, op0, op1);
    }
    /// `VPCOMPRESSB_MASK`.
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
    /// | 5 | Ymm, Ymm |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpcompressb_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpcompressbMaskEmitter<A, B>,
    {
        <Self as VpcompressbMaskEmitter<A, B>>::vpcompressb_mask(self, op0, op1);
    }
    /// `VPCOMPRESSB_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Xmm |
    /// | 2 | Ymm, Ymm |
    /// | 3 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpcompressb_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpcompressbMaskzEmitter<A, B>,
    {
        <Self as VpcompressbMaskzEmitter<A, B>>::vpcompressb_maskz(self, op0, op1);
    }
    /// `VPCOMPRESSW`.
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
    /// | 5 | Ymm, Ymm |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpcompressw<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpcompresswEmitter<A, B>,
    {
        <Self as VpcompresswEmitter<A, B>>::vpcompressw(self, op0, op1);
    }
    /// `VPCOMPRESSW_MASK`.
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
    /// | 5 | Ymm, Ymm |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpcompressw_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpcompresswMaskEmitter<A, B>,
    {
        <Self as VpcompresswMaskEmitter<A, B>>::vpcompressw_mask(self, op0, op1);
    }
    /// `VPCOMPRESSW_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Xmm |
    /// | 2 | Ymm, Ymm |
    /// | 3 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpcompressw_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpcompresswMaskzEmitter<A, B>,
    {
        <Self as VpcompresswMaskzEmitter<A, B>>::vpcompressw_maskz(self, op0, op1);
    }
    /// `VPEXPANDB`.
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
    pub fn vpexpandb<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpexpandbEmitter<A, B>,
    {
        <Self as VpexpandbEmitter<A, B>>::vpexpandb(self, op0, op1);
    }
    /// `VPEXPANDB_MASK`.
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
    pub fn vpexpandb_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpexpandbMaskEmitter<A, B>,
    {
        <Self as VpexpandbMaskEmitter<A, B>>::vpexpandb_mask(self, op0, op1);
    }
    /// `VPEXPANDB_MASKZ`.
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
    pub fn vpexpandb_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpexpandbMaskzEmitter<A, B>,
    {
        <Self as VpexpandbMaskzEmitter<A, B>>::vpexpandb_maskz(self, op0, op1);
    }
    /// `VPEXPANDW`.
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
    pub fn vpexpandw<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpexpandwEmitter<A, B>,
    {
        <Self as VpexpandwEmitter<A, B>>::vpexpandw(self, op0, op1);
    }
    /// `VPEXPANDW_MASK`.
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
    pub fn vpexpandw_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpexpandwMaskEmitter<A, B>,
    {
        <Self as VpexpandwMaskEmitter<A, B>>::vpexpandw_mask(self, op0, op1);
    }
    /// `VPEXPANDW_MASKZ`.
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
    pub fn vpexpandw_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: VpexpandwMaskzEmitter<A, B>,
    {
        <Self as VpexpandwMaskzEmitter<A, B>>::vpexpandw_maskz(self, op0, op1);
    }
    /// `VPSHLDD`.
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
    pub fn vpshldd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshlddEmitter<A, B, C, D>,
    {
        <Self as VpshlddEmitter<A, B, C, D>>::vpshldd(self, op0, op1, op2, op3);
    }
    /// `VPSHLDD_MASK`.
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
    pub fn vpshldd_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshlddMaskEmitter<A, B, C, D>,
    {
        <Self as VpshlddMaskEmitter<A, B, C, D>>::vpshldd_mask(self, op0, op1, op2, op3);
    }
    /// `VPSHLDD_MASKZ`.
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
    pub fn vpshldd_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshlddMaskzEmitter<A, B, C, D>,
    {
        <Self as VpshlddMaskzEmitter<A, B, C, D>>::vpshldd_maskz(self, op0, op1, op2, op3);
    }
    /// `VPSHLDQ`.
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
    pub fn vpshldq<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshldqEmitter<A, B, C, D>,
    {
        <Self as VpshldqEmitter<A, B, C, D>>::vpshldq(self, op0, op1, op2, op3);
    }
    /// `VPSHLDQ_MASK`.
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
    pub fn vpshldq_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshldqMaskEmitter<A, B, C, D>,
    {
        <Self as VpshldqMaskEmitter<A, B, C, D>>::vpshldq_mask(self, op0, op1, op2, op3);
    }
    /// `VPSHLDQ_MASKZ`.
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
    pub fn vpshldq_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshldqMaskzEmitter<A, B, C, D>,
    {
        <Self as VpshldqMaskzEmitter<A, B, C, D>>::vpshldq_maskz(self, op0, op1, op2, op3);
    }
    /// `VPSHLDVD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshldvd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshldvdEmitter<A, B, C>,
    {
        <Self as VpshldvdEmitter<A, B, C>>::vpshldvd(self, op0, op1, op2);
    }
    /// `VPSHLDVD_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshldvd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshldvdMaskEmitter<A, B, C>,
    {
        <Self as VpshldvdMaskEmitter<A, B, C>>::vpshldvd_mask(self, op0, op1, op2);
    }
    /// `VPSHLDVD_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshldvd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshldvdMaskzEmitter<A, B, C>,
    {
        <Self as VpshldvdMaskzEmitter<A, B, C>>::vpshldvd_maskz(self, op0, op1, op2);
    }
    /// `VPSHLDVQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshldvq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshldvqEmitter<A, B, C>,
    {
        <Self as VpshldvqEmitter<A, B, C>>::vpshldvq(self, op0, op1, op2);
    }
    /// `VPSHLDVQ_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshldvq_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshldvqMaskEmitter<A, B, C>,
    {
        <Self as VpshldvqMaskEmitter<A, B, C>>::vpshldvq_mask(self, op0, op1, op2);
    }
    /// `VPSHLDVQ_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshldvq_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshldvqMaskzEmitter<A, B, C>,
    {
        <Self as VpshldvqMaskzEmitter<A, B, C>>::vpshldvq_maskz(self, op0, op1, op2);
    }
    /// `VPSHLDVW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshldvw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshldvwEmitter<A, B, C>,
    {
        <Self as VpshldvwEmitter<A, B, C>>::vpshldvw(self, op0, op1, op2);
    }
    /// `VPSHLDVW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshldvw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshldvwMaskEmitter<A, B, C>,
    {
        <Self as VpshldvwMaskEmitter<A, B, C>>::vpshldvw_mask(self, op0, op1, op2);
    }
    /// `VPSHLDVW_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshldvw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshldvwMaskzEmitter<A, B, C>,
    {
        <Self as VpshldvwMaskzEmitter<A, B, C>>::vpshldvw_maskz(self, op0, op1, op2);
    }
    /// `VPSHLDW`.
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
    pub fn vpshldw<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshldwEmitter<A, B, C, D>,
    {
        <Self as VpshldwEmitter<A, B, C, D>>::vpshldw(self, op0, op1, op2, op3);
    }
    /// `VPSHLDW_MASK`.
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
    pub fn vpshldw_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshldwMaskEmitter<A, B, C, D>,
    {
        <Self as VpshldwMaskEmitter<A, B, C, D>>::vpshldw_mask(self, op0, op1, op2, op3);
    }
    /// `VPSHLDW_MASKZ`.
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
    pub fn vpshldw_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshldwMaskzEmitter<A, B, C, D>,
    {
        <Self as VpshldwMaskzEmitter<A, B, C, D>>::vpshldw_maskz(self, op0, op1, op2, op3);
    }
    /// `VPSHRDD`.
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
    pub fn vpshrdd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshrddEmitter<A, B, C, D>,
    {
        <Self as VpshrddEmitter<A, B, C, D>>::vpshrdd(self, op0, op1, op2, op3);
    }
    /// `VPSHRDD_MASK`.
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
    pub fn vpshrdd_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshrddMaskEmitter<A, B, C, D>,
    {
        <Self as VpshrddMaskEmitter<A, B, C, D>>::vpshrdd_mask(self, op0, op1, op2, op3);
    }
    /// `VPSHRDD_MASKZ`.
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
    pub fn vpshrdd_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshrddMaskzEmitter<A, B, C, D>,
    {
        <Self as VpshrddMaskzEmitter<A, B, C, D>>::vpshrdd_maskz(self, op0, op1, op2, op3);
    }
    /// `VPSHRDQ`.
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
    pub fn vpshrdq<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshrdqEmitter<A, B, C, D>,
    {
        <Self as VpshrdqEmitter<A, B, C, D>>::vpshrdq(self, op0, op1, op2, op3);
    }
    /// `VPSHRDQ_MASK`.
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
    pub fn vpshrdq_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshrdqMaskEmitter<A, B, C, D>,
    {
        <Self as VpshrdqMaskEmitter<A, B, C, D>>::vpshrdq_mask(self, op0, op1, op2, op3);
    }
    /// `VPSHRDQ_MASKZ`.
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
    pub fn vpshrdq_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshrdqMaskzEmitter<A, B, C, D>,
    {
        <Self as VpshrdqMaskzEmitter<A, B, C, D>>::vpshrdq_maskz(self, op0, op1, op2, op3);
    }
    /// `VPSHRDVD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshrdvd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshrdvdEmitter<A, B, C>,
    {
        <Self as VpshrdvdEmitter<A, B, C>>::vpshrdvd(self, op0, op1, op2);
    }
    /// `VPSHRDVD_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshrdvd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshrdvdMaskEmitter<A, B, C>,
    {
        <Self as VpshrdvdMaskEmitter<A, B, C>>::vpshrdvd_mask(self, op0, op1, op2);
    }
    /// `VPSHRDVD_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshrdvd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshrdvdMaskzEmitter<A, B, C>,
    {
        <Self as VpshrdvdMaskzEmitter<A, B, C>>::vpshrdvd_maskz(self, op0, op1, op2);
    }
    /// `VPSHRDVQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshrdvq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshrdvqEmitter<A, B, C>,
    {
        <Self as VpshrdvqEmitter<A, B, C>>::vpshrdvq(self, op0, op1, op2);
    }
    /// `VPSHRDVQ_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshrdvq_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshrdvqMaskEmitter<A, B, C>,
    {
        <Self as VpshrdvqMaskEmitter<A, B, C>>::vpshrdvq_mask(self, op0, op1, op2);
    }
    /// `VPSHRDVQ_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshrdvq_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshrdvqMaskzEmitter<A, B, C>,
    {
        <Self as VpshrdvqMaskzEmitter<A, B, C>>::vpshrdvq_maskz(self, op0, op1, op2);
    }
    /// `VPSHRDVW`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshrdvw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshrdvwEmitter<A, B, C>,
    {
        <Self as VpshrdvwEmitter<A, B, C>>::vpshrdvw(self, op0, op1, op2);
    }
    /// `VPSHRDVW_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshrdvw_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshrdvwMaskEmitter<A, B, C>,
    {
        <Self as VpshrdvwMaskEmitter<A, B, C>>::vpshrdvw_mask(self, op0, op1, op2);
    }
    /// `VPSHRDVW_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpshrdvw_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: VpshrdvwMaskzEmitter<A, B, C>,
    {
        <Self as VpshrdvwMaskzEmitter<A, B, C>>::vpshrdvw_maskz(self, op0, op1, op2);
    }
    /// `VPSHRDW`.
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
    pub fn vpshrdw<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshrdwEmitter<A, B, C, D>,
    {
        <Self as VpshrdwEmitter<A, B, C, D>>::vpshrdw(self, op0, op1, op2, op3);
    }
    /// `VPSHRDW_MASK`.
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
    pub fn vpshrdw_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshrdwMaskEmitter<A, B, C, D>,
    {
        <Self as VpshrdwMaskEmitter<A, B, C, D>>::vpshrdw_mask(self, op0, op1, op2, op3);
    }
    /// `VPSHRDW_MASKZ`.
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
    pub fn vpshrdw_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where
        Assembler<'a>: VpshrdwMaskzEmitter<A, B, C, D>,
    {
        <Self as VpshrdwMaskzEmitter<A, B, C, D>>::vpshrdw_maskz(self, op0, op1, op2, op3);
    }
}
