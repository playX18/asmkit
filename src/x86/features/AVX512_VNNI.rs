use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VPDPBUSD` (VPDPBUSD). 
/// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpbusdEmitter<A, B, C> {
    fn vpdpbusd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpbusdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpbusd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPBUSD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpbusd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPBUSD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpbusd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPBUSD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpbusd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPBUSD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpbusd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPBUSD512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpbusd(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPBUSD512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPDPBUSDS` (VPDPBUSDS). 
/// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSDS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpbusdsEmitter<A, B, C> {
    fn vpdpbusds(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpbusdsEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpbusds(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPBUSDS128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpbusds(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPBUSDS128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpbusds(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPBUSDS256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpbusds(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPBUSDS256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpbusds(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPBUSDS512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpbusds(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPBUSDS512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPDPBUSDS_MASK` (VPDPBUSDS). 
/// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSDS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpbusdsMaskEmitter<A, B, C> {
    fn vpdpbusds_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpbusdsMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpbusds_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPBUSDS128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpbusds_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPBUSDS128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpbusds_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPBUSDS256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpbusds_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPBUSDS256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpbusds_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPBUSDS512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpbusds_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPBUSDS512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPDPBUSDS_MASKZ` (VPDPBUSDS). 
/// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSDS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpbusdsMaskzEmitter<A, B, C> {
    fn vpdpbusds_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpbusdsMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpbusds_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPBUSDS128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpbusds_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPBUSDS128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpbusds_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPBUSDS256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpbusds_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPBUSDS256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpbusds_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPBUSDS512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdsMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpbusds_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPBUSDS512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPDPBUSD_MASK` (VPDPBUSD). 
/// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpbusdMaskEmitter<A, B, C> {
    fn vpdpbusd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpbusdMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpbusd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPBUSD128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpbusd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPBUSD128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpbusd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPBUSD256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpbusd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPBUSD256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpbusd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPBUSD512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpbusd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPBUSD512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPDPBUSD_MASKZ` (VPDPBUSD). 
/// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpbusdMaskzEmitter<A, B, C> {
    fn vpdpbusd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpbusdMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpbusd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPBUSD128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpbusd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPBUSD128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpbusd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPBUSD256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpbusd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPBUSD256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpbusd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPBUSD512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpbusdMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpbusd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPBUSD512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPDPWSSD` (VPDPWSSD). 
/// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpwssdEmitter<A, B, C> {
    fn vpdpwssd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpwssdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpwssd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPWSSD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpwssd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPWSSD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpwssd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPWSSD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpwssd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPWSSD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpwssd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPWSSD512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpwssd(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPWSSD512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPDPWSSDS` (VPDPWSSDS). 
/// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSDS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpwssdsEmitter<A, B, C> {
    fn vpdpwssds(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpwssdsEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpwssds(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPWSSDS128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpwssds(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPWSSDS128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpwssds(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPWSSDS256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpwssds(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPWSSDS256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpwssds(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPWSSDS512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpwssds(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPWSSDS512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPDPWSSDS_MASK` (VPDPWSSDS). 
/// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSDS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpwssdsMaskEmitter<A, B, C> {
    fn vpdpwssds_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpwssdsMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpwssds_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPWSSDS128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpwssds_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPWSSDS128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpwssds_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPWSSDS256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpwssds_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPWSSDS256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpwssds_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPWSSDS512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpwssds_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPWSSDS512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPDPWSSDS_MASKZ` (VPDPWSSDS). 
/// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSDS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpwssdsMaskzEmitter<A, B, C> {
    fn vpdpwssds_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpwssdsMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpwssds_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPWSSDS128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpwssds_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPWSSDS128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpwssds_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPWSSDS256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpwssds_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPWSSDS256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpwssds_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPWSSDS512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdsMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpwssds_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPWSSDS512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPDPWSSD_MASK` (VPDPWSSD). 
/// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpwssdMaskEmitter<A, B, C> {
    fn vpdpwssd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpwssdMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpwssd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPWSSD128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpwssd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPWSSD128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpwssd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPWSSD256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpwssd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPWSSD256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpwssd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPWSSD512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpwssd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPWSSD512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPDPWSSD_MASKZ` (VPDPWSSD). 
/// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpdpwssdMaskzEmitter<A, B, C> {
    fn vpdpwssd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpdpwssdMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpdpwssd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPDPWSSD128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpdpwssd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPDPWSSD128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpdpwssd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPDPWSSD256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpdpwssd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPDPWSSD256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpdpwssd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPDPWSSD512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpdpwssdMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpdpwssd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPDPWSSD512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `VPDPBUSD` (VPDPBUSD). 
    /// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpbusd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpbusdEmitter<A, B, C> {
        <Self as VpdpbusdEmitter<A, B, C>>::vpdpbusd(self, op0, op1, op2);
    }
    /// `VPDPBUSDS` (VPDPBUSDS). 
    /// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSDS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpbusds<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpbusdsEmitter<A, B, C> {
        <Self as VpdpbusdsEmitter<A, B, C>>::vpdpbusds(self, op0, op1, op2);
    }
    /// `VPDPBUSDS_MASK` (VPDPBUSDS). 
    /// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSDS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpbusds_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpbusdsMaskEmitter<A, B, C> {
        <Self as VpdpbusdsMaskEmitter<A, B, C>>::vpdpbusds_mask(self, op0, op1, op2);
    }
    /// `VPDPBUSDS_MASKZ` (VPDPBUSDS). 
    /// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand. If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSDS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpbusds_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpbusdsMaskzEmitter<A, B, C> {
        <Self as VpdpbusdsMaskzEmitter<A, B, C>>::vpdpbusds_maskz(self, op0, op1, op2);
    }
    /// `VPDPBUSD_MASK` (VPDPBUSD). 
    /// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpbusd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpbusdMaskEmitter<A, B, C> {
        <Self as VpdpbusdMaskEmitter<A, B, C>>::vpdpbusd_mask(self, op0, op1, op2);
    }
    /// `VPDPBUSD_MASKZ` (VPDPBUSD). 
    /// Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results. The word results are then summed and accumulated in the destination dword element size operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPBUSD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpbusd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpbusdMaskzEmitter<A, B, C> {
        <Self as VpdpbusdMaskzEmitter<A, B, C>>::vpdpbusd_maskz(self, op0, op1, op2);
    }
    /// `VPDPWSSD` (VPDPWSSD). 
    /// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpwssd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpwssdEmitter<A, B, C> {
        <Self as VpdpwssdEmitter<A, B, C>>::vpdpwssd(self, op0, op1, op2);
    }
    /// `VPDPWSSDS` (VPDPWSSDS). 
    /// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSDS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpwssds<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpwssdsEmitter<A, B, C> {
        <Self as VpdpwssdsEmitter<A, B, C>>::vpdpwssds(self, op0, op1, op2);
    }
    /// `VPDPWSSDS_MASK` (VPDPWSSDS). 
    /// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSDS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpwssds_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpwssdsMaskEmitter<A, B, C> {
        <Self as VpdpwssdsMaskEmitter<A, B, C>>::vpdpwssds_mask(self, op0, op1, op2);
    }
    /// `VPDPWSSDS_MASKZ` (VPDPWSSDS). 
    /// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand. If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSDS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpwssds_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpwssdsMaskzEmitter<A, B, C> {
        <Self as VpdpwssdsMaskzEmitter<A, B, C>>::vpdpwssds_maskz(self, op0, op1, op2);
    }
    /// `VPDPWSSD_MASK` (VPDPWSSD). 
    /// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpwssd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpwssdMaskEmitter<A, B, C> {
        <Self as VpdpwssdMaskEmitter<A, B, C>>::vpdpwssd_mask(self, op0, op1, op2);
    }
    /// `VPDPWSSD_MASKZ` (VPDPWSSD). 
    /// Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results. The adjacent doubleword results are then summed and accumulated in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPDPWSSD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
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
    pub fn vpdpwssd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpdpwssdMaskzEmitter<A, B, C> {
        <Self as VpdpwssdMaskzEmitter<A, B, C>>::vpdpwssd_maskz(self, op0, op1, op2);
    }
}
