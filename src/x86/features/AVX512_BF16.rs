use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VCVTNE2PS2BF16`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vcvtne2ps2bf16Emitter<A, B, C> {
    fn vcvtne2ps2bf16(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vcvtne2ps2bf16Emitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vcvtne2ps2bf16(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VCVTNE2PS2BF16_128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16Emitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vcvtne2ps2bf16(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VCVTNE2PS2BF16_128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16Emitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vcvtne2ps2bf16(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VCVTNE2PS2BF16_256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16Emitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vcvtne2ps2bf16(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VCVTNE2PS2BF16_256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16Emitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vcvtne2ps2bf16(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VCVTNE2PS2BF16_512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16Emitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vcvtne2ps2bf16(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VCVTNE2PS2BF16_512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VCVTNE2PS2BF16_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vcvtne2ps2bf16MaskEmitter<A, B, C> {
    fn vcvtne2ps2bf16_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vcvtne2ps2bf16MaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vcvtne2ps2bf16_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VCVTNE2PS2BF16_128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16MaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vcvtne2ps2bf16_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VCVTNE2PS2BF16_128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16MaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vcvtne2ps2bf16_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VCVTNE2PS2BF16_256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16MaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vcvtne2ps2bf16_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VCVTNE2PS2BF16_256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16MaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vcvtne2ps2bf16_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VCVTNE2PS2BF16_512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16MaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vcvtne2ps2bf16_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VCVTNE2PS2BF16_512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VCVTNE2PS2BF16_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vcvtne2ps2bf16MaskzEmitter<A, B, C> {
    fn vcvtne2ps2bf16_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vcvtne2ps2bf16MaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vcvtne2ps2bf16_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VCVTNE2PS2BF16_128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16MaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vcvtne2ps2bf16_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VCVTNE2PS2BF16_128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16MaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vcvtne2ps2bf16_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VCVTNE2PS2BF16_256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16MaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vcvtne2ps2bf16_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VCVTNE2PS2BF16_256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16MaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vcvtne2ps2bf16_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VCVTNE2PS2BF16_512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vcvtne2ps2bf16MaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vcvtne2ps2bf16_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VCVTNE2PS2BF16_512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VCVTNEPS2BF16`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Xmm, Ymm |
/// | 4 | Ymm, Mem |
/// | 5 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtneps2bf16Emitter<A, B> {
    fn vcvtneps2bf16(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtneps2bf16Emitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtneps2bf16(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VCVTNEPS2BF16_128RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16Emitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtneps2bf16(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VCVTNEPS2BF16_128RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16Emitter<Xmm, Ymm> for Assembler<'a> {
    fn vcvtneps2bf16(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VCVTNEPS2BF16_256RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16Emitter<Ymm, Zmm> for Assembler<'a> {
    fn vcvtneps2bf16(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VCVTNEPS2BF16_512RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16Emitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtneps2bf16(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VCVTNEPS2BF16_512RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VCVTNEPS2BF16_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Xmm, Ymm |
/// | 4 | Ymm, Mem |
/// | 5 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtneps2bf16MaskEmitter<A, B> {
    fn vcvtneps2bf16_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtneps2bf16MaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtneps2bf16_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VCVTNEPS2BF16_128RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16MaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtneps2bf16_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VCVTNEPS2BF16_128RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16MaskEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vcvtneps2bf16_mask(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VCVTNEPS2BF16_256RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16MaskEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vcvtneps2bf16_mask(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VCVTNEPS2BF16_512RR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16MaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtneps2bf16_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VCVTNEPS2BF16_512RM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VCVTNEPS2BF16_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Xmm, Ymm |
/// | 4 | Ymm, Mem |
/// | 5 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtneps2bf16MaskzEmitter<A, B> {
    fn vcvtneps2bf16_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtneps2bf16MaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtneps2bf16_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            VCVTNEPS2BF16_128RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16MaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtneps2bf16_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            VCVTNEPS2BF16_128RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16MaskzEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vcvtneps2bf16_maskz(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(
            VCVTNEPS2BF16_256RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16MaskzEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vcvtneps2bf16_maskz(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(
            VCVTNEPS2BF16_512RR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Vcvtneps2bf16MaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtneps2bf16_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(
            VCVTNEPS2BF16_512RM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `VDPBF16PS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vdpbf16psEmitter<A, B, C> {
    fn vdpbf16ps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vdpbf16psEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vdpbf16ps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VDPBF16PS128RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vdpbf16ps(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VDPBF16PS128RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vdpbf16ps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VDPBF16PS256RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vdpbf16ps(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VDPBF16PS256RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vdpbf16ps(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VDPBF16PS512RRR,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vdpbf16ps(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VDPBF16PS512RRM,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VDPBF16PS_MASK`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vdpbf16psMaskEmitter<A, B, C> {
    fn vdpbf16ps_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vdpbf16psMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vdpbf16ps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VDPBF16PS128RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vdpbf16ps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VDPBF16PS128RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vdpbf16ps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VDPBF16PS256RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vdpbf16ps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VDPBF16PS256RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vdpbf16ps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VDPBF16PS512RRR_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vdpbf16ps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VDPBF16PS512RRM_MASK,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `VDPBF16PS_MASKZ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait Vdpbf16psMaskzEmitter<A, B, C> {
    fn vdpbf16ps_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vdpbf16psMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vdpbf16ps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(
            VDPBF16PS128RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vdpbf16ps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(
            VDPBF16PS128RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vdpbf16ps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(
            VDPBF16PS256RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vdpbf16ps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(
            VDPBF16PS256RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vdpbf16ps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(
            VDPBF16PS512RRR_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Vdpbf16psMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vdpbf16ps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(
            VDPBF16PS512RRM_MASKZ,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `VCVTNE2PS2BF16`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vcvtne2ps2bf16<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vcvtne2ps2bf16Emitter<A, B, C>,
    {
        <Self as Vcvtne2ps2bf16Emitter<A, B, C>>::vcvtne2ps2bf16(self, op0, op1, op2);
    }
    /// `VCVTNE2PS2BF16_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vcvtne2ps2bf16_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vcvtne2ps2bf16MaskEmitter<A, B, C>,
    {
        <Self as Vcvtne2ps2bf16MaskEmitter<A, B, C>>::vcvtne2ps2bf16_mask(self, op0, op1, op2);
    }
    /// `VCVTNE2PS2BF16_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vcvtne2ps2bf16_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vcvtne2ps2bf16MaskzEmitter<A, B, C>,
    {
        <Self as Vcvtne2ps2bf16MaskzEmitter<A, B, C>>::vcvtne2ps2bf16_maskz(self, op0, op1, op2);
    }
    /// `VCVTNEPS2BF16`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Xmm, Ymm |
    /// | 4 | Ymm, Mem |
    /// | 5 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtneps2bf16<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vcvtneps2bf16Emitter<A, B>,
    {
        <Self as Vcvtneps2bf16Emitter<A, B>>::vcvtneps2bf16(self, op0, op1);
    }
    /// `VCVTNEPS2BF16_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Xmm, Ymm |
    /// | 4 | Ymm, Mem |
    /// | 5 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtneps2bf16_mask<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vcvtneps2bf16MaskEmitter<A, B>,
    {
        <Self as Vcvtneps2bf16MaskEmitter<A, B>>::vcvtneps2bf16_mask(self, op0, op1);
    }
    /// `VCVTNEPS2BF16_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Xmm, Ymm |
    /// | 4 | Ymm, Mem |
    /// | 5 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtneps2bf16_maskz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: Vcvtneps2bf16MaskzEmitter<A, B>,
    {
        <Self as Vcvtneps2bf16MaskzEmitter<A, B>>::vcvtneps2bf16_maskz(self, op0, op1);
    }
    /// `VDPBF16PS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vdpbf16ps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vdpbf16psEmitter<A, B, C>,
    {
        <Self as Vdpbf16psEmitter<A, B, C>>::vdpbf16ps(self, op0, op1, op2);
    }
    /// `VDPBF16PS_MASK`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vdpbf16ps_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vdpbf16psMaskEmitter<A, B, C>,
    {
        <Self as Vdpbf16psMaskEmitter<A, B, C>>::vdpbf16ps_mask(self, op0, op1, op2);
    }
    /// `VDPBF16PS_MASKZ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vdpbf16ps_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: Vdpbf16psMaskzEmitter<A, B, C>,
    {
        <Self as Vdpbf16psMaskzEmitter<A, B, C>>::vdpbf16ps_maskz(self, op0, op1, op2);
    }
}
