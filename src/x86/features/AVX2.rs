use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VBROADCASTI128` (VBROADCASTI128). 
/// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcasti128Emitter<A, B> {
    fn vbroadcasti128(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti128Emitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcasti128(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTI128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VEXTRACTI128` (VEXTRACTI128). 
/// VEXTRACTI128/VEXTRACTI32x4 and VEXTRACTI64x2 extract 128-bits of doubleword integer values from the source operand (the second operand) and store to the low 128-bit of the destination operand (the first operand). The 128-bit data extraction occurs at an 128-bit granular offset specified by imm8[0] (256-bit) or imm8[1:0] as the multiply factor. The destination may be either a vector register or an 128-bit memory location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VEXTRACTI128%3AVEXTRACTI32x4%3AVEXTRACTI64x2%3AVEXTRACTI32x8%3AVEXTRACTI64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Ymm, Imm |
/// | 2 | Xmm, Ymm, Imm |
/// +---+---------------+
/// ```
pub trait Vextracti128Emitter<A, B, C> {
    fn vextracti128(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vextracti128Emitter<Xmm, Ymm, Imm> for Assembler<'a> {
    fn vextracti128(&mut self, op0: Xmm, op1: Ymm, op2: Imm) {
        self.emit(VEXTRACTI128RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> Vextracti128Emitter<Mem, Ymm, Imm> for Assembler<'a> {
    fn vextracti128(&mut self, op0: Mem, op1: Ymm, op2: Imm) {
        self.emit(VEXTRACTI128MRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VGATHERDPD` (VGATHERDPD). 
/// The instruction conditionally loads up to 2 or 4 double precision floating-point values from memory addresses specified by the memory operand (the second operand) and using qword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VGATHERDPD%3AVGATHERQPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VgatherdpdEmitter_3<A, B, C> {
    fn vgatherdpd_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VgatherdpdEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vgatherdpd_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(VGATHERDPD128RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VgatherdpdEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vgatherdpd_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(VGATHERDPD256RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VGATHERDPS` (VGATHERDPS). 
/// The instruction conditionally loads up to 4 or 8 single-precision floating-point values from memory addresses specified by the memory operand (the second operand) and using dword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VGATHERDPS%3AVGATHERQPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VgatherdpsEmitter_3<A, B, C> {
    fn vgatherdps_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VgatherdpsEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vgatherdps_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(VGATHERDPS128RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VgatherdpsEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vgatherdps_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(VGATHERDPS256RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VGATHERQPD` (VGATHERQPD). 
/// The instruction conditionally loads up to 2 or 4 double precision floating-point values from memory addresses specified by the memory operand (the second operand) and using qword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VGATHERDPD%3AVGATHERQPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VgatherqpdEmitter_3<A, B, C> {
    fn vgatherqpd_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VgatherqpdEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vgatherqpd_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(VGATHERQPD128RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VgatherqpdEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vgatherqpd_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(VGATHERQPD256RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VGATHERQPS` (VGATHERQPS). 
/// The instruction conditionally loads up to 4 or 8 single-precision floating-point values from memory addresses specified by the memory operand (the second operand) and using dword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VGATHERDPS%3AVGATHERQPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// +---+---------------+
/// ```
pub trait VgatherqpsEmitter_3<A, B, C> {
    fn vgatherqps_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VgatherqpsEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vgatherqps_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(VGATHERQPS128RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VINSERTI128` (VINSERTI128). 
/// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Ymm, Ymm, Mem, Imm |
/// | 2 | Ymm, Ymm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinserti128Emitter<A, B, C, D> {
    fn vinserti128(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinserti128Emitter<Ymm, Ymm, Xmm, Imm> for Assembler<'a> {
    fn vinserti128(&mut self, op0: Ymm, op1: Ymm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTI128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti128Emitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vinserti128(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VINSERTI128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VPBLENDD` (VPBLENDD). 
/// Dword elements from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding dword in the destination is copied from the source. If a bit in the mask, corresponding to a dword, is “1", then the dword is copied, else the dword is unchanged.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBLENDD.html).
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
/// +---+--------------------+
/// ```
pub trait VpblenddEmitter<A, B, C, D> {
    fn vpblendd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpblenddEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpblendd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VPBLENDD128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpblenddEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpblendd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VPBLENDD128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpblenddEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpblendd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VPBLENDD256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpblenddEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpblendd(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VPBLENDD256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VPERM2I128` (VPERM2I128). 
/// Permute 128 bit integer data from the first source operand (second operand) and second source operand (third operand) using bits in the 8-bit immediate and store results in the destination operand (first operand). The first source operand is a YMM register, the second source operand is a YMM register or a 256-bit memory location, and the destination operand is a YMM register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPERM2I128.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Ymm, Ymm, Mem, Imm |
/// | 2 | Ymm, Ymm, Ymm, Imm |
/// +---+--------------------+
/// ```
pub trait Vperm2i128Emitter<A, B, C, D> {
    fn vperm2i128(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vperm2i128Emitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vperm2i128(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VPERM2I128_256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vperm2i128Emitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vperm2i128(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VPERM2I128_256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VPGATHERDD` (VPGATHERDD). 
/// A set of 16 or 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into vector zmm1. The elements are specified via the VSIB (i.e., the index register is a zmm, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register (zmm1) is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPGATHERDD%3AVPGATHERDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VpgatherddEmitter_3<A, B, C> {
    fn vpgatherdd_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpgatherddEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vpgatherdd_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(VPGATHERDD128RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpgatherddEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vpgatherdd_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(VPGATHERDD256RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPGATHERDQ` (VPGATHERDQ). 
/// A set of 16 or 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into vector zmm1. The elements are specified via the VSIB (i.e., the index register is a zmm, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register (zmm1) is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPGATHERDD%3AVPGATHERDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VpgatherdqEmitter_3<A, B, C> {
    fn vpgatherdq_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpgatherdqEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vpgatherdq_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(VPGATHERDQ128RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpgatherdqEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vpgatherdq_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(VPGATHERDQ256RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPGATHERQD` (VPGATHERQD). 
/// A set of 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into a vector register. The elements are specified via the VSIB (i.e., the index register is a vector register, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPGATHERQD%3AVPGATHERQQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// +---+---------------+
/// ```
pub trait VpgatherqdEmitter_3<A, B, C> {
    fn vpgatherqd_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpgatherqdEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vpgatherqd_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(VPGATHERQD128RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPGATHERQQ` (VPGATHERQQ). 
/// A set of 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into a vector register. The elements are specified via the VSIB (i.e., the index register is a vector register, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPGATHERQD%3AVPGATHERQQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Ymm, Mem, Ymm |
/// +---+---------------+
/// ```
pub trait VpgatherqqEmitter_3<A, B, C> {
    fn vpgatherqq_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpgatherqqEmitter_3<Xmm, Mem, Xmm> for Assembler<'a> {
    fn vpgatherqq_3(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(VPGATHERQQ128RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpgatherqqEmitter_3<Ymm, Mem, Ymm> for Assembler<'a> {
    fn vpgatherqq_3(&mut self, op0: Ymm, op1: Mem, op2: Ymm) {
        self.emit(VPGATHERQQ256RMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPMASKMOVD` (VPMASKMOVD). 
/// Conditionally moves packed data elements from the second source operand into the corresponding data element of the destination operand, depending on the mask bits associated with each data element. The mask bits are specified in the first source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMASKMOV.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Xmm, Xmm |
/// | 2 | Mem, Ymm, Ymm |
/// | 3 | Xmm, Xmm, Mem |
/// | 4 | Ymm, Ymm, Mem |
/// +---+---------------+
/// ```
pub trait VpmaskmovdEmitter<A, B, C> {
    fn vpmaskmovd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaskmovdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaskmovd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPMASKMOVD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmaskmovdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaskmovd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPMASKMOVD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmaskmovdEmitter<Mem, Xmm, Xmm> for Assembler<'a> {
    fn vpmaskmovd(&mut self, op0: Mem, op1: Xmm, op2: Xmm) {
        self.emit(VPMASKMOVD128MRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmaskmovdEmitter<Mem, Ymm, Ymm> for Assembler<'a> {
    fn vpmaskmovd(&mut self, op0: Mem, op1: Ymm, op2: Ymm) {
        self.emit(VPMASKMOVD256MRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPMASKMOVQ` (VPMASKMOVQ). 
/// Conditionally moves packed data elements from the second source operand into the corresponding data element of the destination operand, depending on the mask bits associated with each data element. The mask bits are specified in the first source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMASKMOV.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Xmm, Xmm |
/// | 2 | Mem, Ymm, Ymm |
/// | 3 | Xmm, Xmm, Mem |
/// | 4 | Ymm, Ymm, Mem |
/// +---+---------------+
/// ```
pub trait VpmaskmovqEmitter<A, B, C> {
    fn vpmaskmovq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmaskmovqEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmaskmovq(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPMASKMOVQ128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmaskmovqEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmaskmovq(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPMASKMOVQ256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmaskmovqEmitter<Mem, Xmm, Xmm> for Assembler<'a> {
    fn vpmaskmovq(&mut self, op0: Mem, op1: Xmm, op2: Xmm) {
        self.emit(VPMASKMOVQ128MRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmaskmovqEmitter<Mem, Ymm, Ymm> for Assembler<'a> {
    fn vpmaskmovq(&mut self, op0: Mem, op1: Ymm, op2: Ymm) {
        self.emit(VPMASKMOVQ256MRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `VBROADCASTI128` (VBROADCASTI128). 
    /// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti128<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti128Emitter<A, B> {
        <Self as Vbroadcasti128Emitter<A, B>>::vbroadcasti128(self, op0, op1);
    }
    /// `VEXTRACTI128` (VEXTRACTI128). 
    /// VEXTRACTI128/VEXTRACTI32x4 and VEXTRACTI64x2 extract 128-bits of doubleword integer values from the source operand (the second operand) and store to the low 128-bit of the destination operand (the first operand). The 128-bit data extraction occurs at an 128-bit granular offset specified by imm8[0] (256-bit) or imm8[1:0] as the multiply factor. The destination may be either a vector register or an 128-bit memory location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VEXTRACTI128%3AVEXTRACTI32x4%3AVEXTRACTI64x2%3AVEXTRACTI32x8%3AVEXTRACTI64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Ymm, Imm |
    /// | 2 | Xmm, Ymm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vextracti128<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: Vextracti128Emitter<A, B, C> {
        <Self as Vextracti128Emitter<A, B, C>>::vextracti128(self, op0, op1, op2);
    }
    /// `VGATHERDPD` (VGATHERDPD). 
    /// The instruction conditionally loads up to 2 or 4 double precision floating-point values from memory addresses specified by the memory operand (the second operand) and using qword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VGATHERDPD%3AVGATHERQPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vgatherdpd_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VgatherdpdEmitter_3<A, B, C> {
        <Self as VgatherdpdEmitter_3<A, B, C>>::vgatherdpd_3(self, op0, op1, op2);
    }
    /// `VGATHERDPS` (VGATHERDPS). 
    /// The instruction conditionally loads up to 4 or 8 single-precision floating-point values from memory addresses specified by the memory operand (the second operand) and using dword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VGATHERDPS%3AVGATHERQPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vgatherdps_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VgatherdpsEmitter_3<A, B, C> {
        <Self as VgatherdpsEmitter_3<A, B, C>>::vgatherdps_3(self, op0, op1, op2);
    }
    /// `VGATHERQPD` (VGATHERQPD). 
    /// The instruction conditionally loads up to 2 or 4 double precision floating-point values from memory addresses specified by the memory operand (the second operand) and using qword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VGATHERDPD%3AVGATHERQPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vgatherqpd_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VgatherqpdEmitter_3<A, B, C> {
        <Self as VgatherqpdEmitter_3<A, B, C>>::vgatherqpd_3(self, op0, op1, op2);
    }
    /// `VGATHERQPS` (VGATHERQPS). 
    /// The instruction conditionally loads up to 4 or 8 single-precision floating-point values from memory addresses specified by the memory operand (the second operand) and using dword indices. The memory operand uses the VSIB form of the SIB byte to specify a general purpose register operand as the common base, a vector register for an array of indices relative to the base and a constant scale factor.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VGATHERDPS%3AVGATHERQPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vgatherqps_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VgatherqpsEmitter_3<A, B, C> {
        <Self as VgatherqpsEmitter_3<A, B, C>>::vgatherqps_3(self, op0, op1, op2);
    }
    /// `VINSERTI128` (VINSERTI128). 
    /// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Ymm, Ymm, Mem, Imm |
    /// | 2 | Ymm, Ymm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinserti128<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinserti128Emitter<A, B, C, D> {
        <Self as Vinserti128Emitter<A, B, C, D>>::vinserti128(self, op0, op1, op2, op3);
    }
    /// `VPBLENDD` (VPBLENDD). 
    /// Dword elements from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding dword in the destination is copied from the source. If a bit in the mask, corresponding to a dword, is “1", then the dword is copied, else the dword is unchanged.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBLENDD.html).
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
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vpblendd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VpblenddEmitter<A, B, C, D> {
        <Self as VpblenddEmitter<A, B, C, D>>::vpblendd(self, op0, op1, op2, op3);
    }
    /// `VPERM2I128` (VPERM2I128). 
    /// Permute 128 bit integer data from the first source operand (second operand) and second source operand (third operand) using bits in the 8-bit immediate and store results in the destination operand (first operand). The first source operand is a YMM register, the second source operand is a YMM register or a 256-bit memory location, and the destination operand is a YMM register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPERM2I128.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Ymm, Ymm, Mem, Imm |
    /// | 2 | Ymm, Ymm, Ymm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vperm2i128<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vperm2i128Emitter<A, B, C, D> {
        <Self as Vperm2i128Emitter<A, B, C, D>>::vperm2i128(self, op0, op1, op2, op3);
    }
    /// `VPGATHERDD` (VPGATHERDD). 
    /// A set of 16 or 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into vector zmm1. The elements are specified via the VSIB (i.e., the index register is a zmm, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register (zmm1) is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPGATHERDD%3AVPGATHERDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpgatherdd_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpgatherddEmitter_3<A, B, C> {
        <Self as VpgatherddEmitter_3<A, B, C>>::vpgatherdd_3(self, op0, op1, op2);
    }
    /// `VPGATHERDQ` (VPGATHERDQ). 
    /// A set of 16 or 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into vector zmm1. The elements are specified via the VSIB (i.e., the index register is a zmm, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register (zmm1) is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPGATHERDD%3AVPGATHERDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpgatherdq_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpgatherdqEmitter_3<A, B, C> {
        <Self as VpgatherdqEmitter_3<A, B, C>>::vpgatherdq_3(self, op0, op1, op2);
    }
    /// `VPGATHERQD` (VPGATHERQD). 
    /// A set of 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into a vector register. The elements are specified via the VSIB (i.e., the index register is a vector register, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPGATHERQD%3AVPGATHERQQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpgatherqd_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpgatherqdEmitter_3<A, B, C> {
        <Self as VpgatherqdEmitter_3<A, B, C>>::vpgatherqd_3(self, op0, op1, op2);
    }
    /// `VPGATHERQQ` (VPGATHERQQ). 
    /// A set of 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered. The result is written into a vector register. The elements are specified via the VSIB (i.e., the index register is a vector register, holding packed indices). Elements will only be loaded if their corresponding mask bit is one. If an element’s mask bit is not set, the corresponding element of the destination register is left unchanged. The entire mask register will be set to zero by this instruction unless it triggers an exception.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPGATHERQD%3AVPGATHERQQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Ymm, Mem, Ymm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpgatherqq_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpgatherqqEmitter_3<A, B, C> {
        <Self as VpgatherqqEmitter_3<A, B, C>>::vpgatherqq_3(self, op0, op1, op2);
    }
    /// `VPMASKMOVD` (VPMASKMOVD). 
    /// Conditionally moves packed data elements from the second source operand into the corresponding data element of the destination operand, depending on the mask bits associated with each data element. The mask bits are specified in the first source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMASKMOV.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Xmm, Xmm |
    /// | 2 | Mem, Ymm, Ymm |
    /// | 3 | Xmm, Xmm, Mem |
    /// | 4 | Ymm, Ymm, Mem |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmaskmovd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpmaskmovdEmitter<A, B, C> {
        <Self as VpmaskmovdEmitter<A, B, C>>::vpmaskmovd(self, op0, op1, op2);
    }
    /// `VPMASKMOVQ` (VPMASKMOVQ). 
    /// Conditionally moves packed data elements from the second source operand into the corresponding data element of the destination operand, depending on the mask bits associated with each data element. The mask bits are specified in the first source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMASKMOV.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Xmm, Xmm |
    /// | 2 | Mem, Ymm, Ymm |
    /// | 3 | Xmm, Xmm, Mem |
    /// | 4 | Ymm, Ymm, Mem |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmaskmovq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpmaskmovqEmitter<A, B, C> {
        <Self as VpmaskmovqEmitter<A, B, C>>::vpmaskmovq(self, op0, op1, op2);
    }
}
