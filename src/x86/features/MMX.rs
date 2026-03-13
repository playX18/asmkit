use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `MMX_EMMS` (EMMS). 
/// Sets the values of all the tags in the x87 FPU tag word to empty (all 1s). This operation marks the x87 FPU data registers (which are aliased to the MMX technology registers) as available for use by x87 FPU floating-point instructions. (See Figure 8-7 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for the format of the x87 FPU tag word.) All other MMX instructions (other than the EMMS instruction) set all the tags in x87 FPU tag word to valid (all 0s).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/EMMS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | (none)   |
/// +---+----------+
/// ```
pub trait MmxEmmsEmitter {
    fn mmx_emms(&mut self);
}

impl<'a> MmxEmmsEmitter for Assembler<'a> {
    fn mmx_emms(&mut self) {
        self.emit(MMX_EMMS, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `MMX_MOVD_G2M`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Gpd  |
/// | 2 | Mm, Mem  |
/// +---+----------+
/// ```
pub trait MmxMovdG2mEmitter<A, B> {
    fn mmx_movd_g2m(&mut self, op0: A, op1: B);
}

impl<'a> MmxMovdG2mEmitter<Mm, Gpd> for Assembler<'a> {
    fn mmx_movd_g2m(&mut self, op0: Mm, op1: Gpd) {
        self.emit(MMX_MOVD_G2MRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxMovdG2mEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_movd_g2m(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_MOVD_G2MRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_MOVD_M2G`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Mm  |
/// | 2 | Mem, Mm  |
/// +---+----------+
/// ```
pub trait MmxMovdM2gEmitter<A, B> {
    fn mmx_movd_m2g(&mut self, op0: A, op1: B);
}

impl<'a> MmxMovdM2gEmitter<Gpd, Mm> for Assembler<'a> {
    fn mmx_movd_m2g(&mut self, op0: Gpd, op1: Mm) {
        self.emit(MMX_MOVD_M2GRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxMovdM2gEmitter<Mem, Mm> for Assembler<'a> {
    fn mmx_movd_m2g(&mut self, op0: Mem, op1: Mm) {
        self.emit(MMX_MOVD_M2GMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_MOVQ` (MOVQ). 
/// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Mm  |
/// | 2 | Mm, Mem  |
/// | 3 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxMovqEmitter<A, B> {
    fn mmx_movq(&mut self, op0: A, op1: B);
}

impl<'a> MmxMovqEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_movq(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_MOVQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxMovqEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_movq(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_MOVQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxMovqEmitter<Mem, Mm> for Assembler<'a> {
    fn mmx_movq(&mut self, op0: Mem, op1: Mm) {
        self.emit(MMX_MOVQMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_MOVQ_G2M`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Gpd  |
/// | 2 | Mm, Mem  |
/// +---+----------+
/// ```
pub trait MmxMovqG2mEmitter<A, B> {
    fn mmx_movq_g2m(&mut self, op0: A, op1: B);
}

impl<'a> MmxMovqG2mEmitter<Mm, Gpd> for Assembler<'a> {
    fn mmx_movq_g2m(&mut self, op0: Mm, op1: Gpd) {
        self.emit(MMX_MOVQ_G2MRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxMovqG2mEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_movq_g2m(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_MOVQ_G2MRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_MOVQ_M2G`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Mm  |
/// | 2 | Mem, Mm  |
/// +---+----------+
/// ```
pub trait MmxMovqM2gEmitter<A, B> {
    fn mmx_movq_m2g(&mut self, op0: A, op1: B);
}

impl<'a> MmxMovqM2gEmitter<Gpd, Mm> for Assembler<'a> {
    fn mmx_movq_m2g(&mut self, op0: Gpd, op1: Mm) {
        self.emit(MMX_MOVQ_M2GRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxMovqM2gEmitter<Mem, Mm> for Assembler<'a> {
    fn mmx_movq_m2g(&mut self, op0: Mem, op1: Mm) {
        self.emit(MMX_MOVQ_M2GMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PACKSSDW` (PACKSSDW). 
/// Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPackssdwEmitter<A, B> {
    fn mmx_packssdw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPackssdwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_packssdw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PACKSSDWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPackssdwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_packssdw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PACKSSDWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PACKSSWB` (PACKSSWB). 
/// Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPacksswbEmitter<A, B> {
    fn mmx_packsswb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPacksswbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_packsswb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PACKSSWBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPacksswbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_packsswb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PACKSSWBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PACKUSWB` (PACKUSWB). 
/// Converts 4, 8, 16, or 32 signed word integers from the destination operand (first operand) and 4, 8, 16, or 32 signed word integers from the source operand (second operand) into 8, 16, 32 or 64 unsigned byte integers and stores the result in the destination operand. (See Figure 4-6 for an example of the packing operation.) If a signed word integer value is beyond the range of an unsigned byte integer (that is, greater than FFH or less than 00H), the saturated unsigned byte integer value of FFH or 00H, respectively, is stored in the destination.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PACKUSWB.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPackuswbEmitter<A, B> {
    fn mmx_packuswb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPackuswbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_packuswb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PACKUSWBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPackuswbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_packuswb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PACKUSWBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PADDB` (PADDB). 
/// Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPaddbEmitter<A, B> {
    fn mmx_paddb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPaddbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_paddb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PADDBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPaddbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_paddb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PADDBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PADDD` (PADDD). 
/// Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPadddEmitter<A, B> {
    fn mmx_paddd(&mut self, op0: A, op1: B);
}

impl<'a> MmxPadddEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_paddd(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PADDDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPadddEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_paddd(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PADDDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PADDQ` (PADDQ). 
/// Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPaddqEmitter<A, B> {
    fn mmx_paddq(&mut self, op0: A, op1: B);
}

impl<'a> MmxPaddqEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_paddq(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PADDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPaddqEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_paddq(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PADDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PADDSB` (PADDSB). 
/// Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPaddsbEmitter<A, B> {
    fn mmx_paddsb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPaddsbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_paddsb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PADDSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPaddsbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_paddsb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PADDSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PADDSW` (PADDSW). 
/// Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPaddswEmitter<A, B> {
    fn mmx_paddsw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPaddswEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_paddsw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PADDSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPaddswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_paddsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PADDSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PADDUSB` (PADDUSB). 
/// Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPaddusbEmitter<A, B> {
    fn mmx_paddusb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPaddusbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_paddusb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PADDUSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPaddusbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_paddusb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PADDUSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PADDUSW` (PADDUSW). 
/// Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPadduswEmitter<A, B> {
    fn mmx_paddusw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPadduswEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_paddusw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PADDUSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPadduswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_paddusw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PADDUSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PADDW` (PADDW). 
/// Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPaddwEmitter<A, B> {
    fn mmx_paddw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPaddwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_paddw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PADDWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPaddwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_paddw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PADDWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PAND` (PAND). 
/// Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PAND.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPandEmitter<A, B> {
    fn mmx_pand(&mut self, op0: A, op1: B);
}

impl<'a> MmxPandEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pand(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PANDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPandEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pand(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PANDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PANDN` (PANDN). 
/// Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PANDN.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPandnEmitter<A, B> {
    fn mmx_pandn(&mut self, op0: A, op1: B);
}

impl<'a> MmxPandnEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pandn(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PANDNRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPandnEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pandn(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PANDNRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PCMPEQB` (PCMPEQB). 
/// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPcmpeqbEmitter<A, B> {
    fn mmx_pcmpeqb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPcmpeqbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pcmpeqb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PCMPEQBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPcmpeqbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pcmpeqb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PCMPEQBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PCMPEQD` (PCMPEQD). 
/// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPcmpeqdEmitter<A, B> {
    fn mmx_pcmpeqd(&mut self, op0: A, op1: B);
}

impl<'a> MmxPcmpeqdEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pcmpeqd(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PCMPEQDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPcmpeqdEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pcmpeqd(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PCMPEQDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PCMPEQW` (PCMPEQW). 
/// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPcmpeqwEmitter<A, B> {
    fn mmx_pcmpeqw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPcmpeqwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pcmpeqw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PCMPEQWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPcmpeqwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pcmpeqw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PCMPEQWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PCMPGTB` (PCMPGTB). 
/// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPcmpgtbEmitter<A, B> {
    fn mmx_pcmpgtb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPcmpgtbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pcmpgtb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PCMPGTBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPcmpgtbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pcmpgtb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PCMPGTBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PCMPGTD` (PCMPGTD). 
/// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPcmpgtdEmitter<A, B> {
    fn mmx_pcmpgtd(&mut self, op0: A, op1: B);
}

impl<'a> MmxPcmpgtdEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pcmpgtd(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PCMPGTDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPcmpgtdEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pcmpgtd(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PCMPGTDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PCMPGTW` (PCMPGTW). 
/// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPcmpgtwEmitter<A, B> {
    fn mmx_pcmpgtw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPcmpgtwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pcmpgtw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PCMPGTWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPcmpgtwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pcmpgtw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PCMPGTWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PMADDWD` (PMADDWD). 
/// Multiplies the individual signed words of the destination operand (first operand) by the corresponding signed words of the source operand (second operand), producing temporary signed, doubleword results. The adjacent double-word results are then summed and stored in the destination operand. For example, the corresponding low-order words (15-0) and (31-16) in the source and destination operands are multiplied by one another and the double-word results are added together and stored in the low doubleword of the destination register (31-0). The same operation is performed on the other pairs of adjacent words. (Figure 4-11 shows this operation when using 64-bit operands).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMADDWD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPmaddwdEmitter<A, B> {
    fn mmx_pmaddwd(&mut self, op0: A, op1: B);
}

impl<'a> MmxPmaddwdEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pmaddwd(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PMADDWDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPmaddwdEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pmaddwd(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PMADDWDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PMULHW` (PMULHW). 
/// Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULHW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPmulhwEmitter<A, B> {
    fn mmx_pmulhw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPmulhwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pmulhw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PMULHWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPmulhwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pmulhw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PMULHWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PMULLW` (PMULLW). 
/// Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the low 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPmullwEmitter<A, B> {
    fn mmx_pmullw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPmullwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pmullw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PMULLWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPmullwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pmullw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PMULLWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PMULUDQ` (PMULUDQ). 
/// Multiplies the first operand (destination operand) by the second operand (source operand) and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULUDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPmuludqEmitter<A, B> {
    fn mmx_pmuludq(&mut self, op0: A, op1: B);
}

impl<'a> MmxPmuludqEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pmuludq(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PMULUDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPmuludqEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pmuludq(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PMULUDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_POR` (POR). 
/// Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/POR.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPorEmitter<A, B> {
    fn mmx_por(&mut self, op0: A, op1: B);
}

impl<'a> MmxPorEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_por(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PORRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPorEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_por(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PORRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSLLD` (PSLLD). 
/// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Imm  |
/// | 2 | Mm, Mem  |
/// | 3 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPslldEmitter<A, B> {
    fn mmx_pslld(&mut self, op0: A, op1: B);
}

impl<'a> MmxPslldEmitter<Mm, Imm> for Assembler<'a> {
    fn mmx_pslld(&mut self, op0: Mm, op1: Imm) {
        self.emit(MMX_PSLLDRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPslldEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pslld(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSLLDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPslldEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pslld(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSLLDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSLLQ` (PSLLQ). 
/// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Imm  |
/// | 2 | Mm, Mem  |
/// | 3 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsllqEmitter<A, B> {
    fn mmx_psllq(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsllqEmitter<Mm, Imm> for Assembler<'a> {
    fn mmx_psllq(&mut self, op0: Mm, op1: Imm) {
        self.emit(MMX_PSLLQRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsllqEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psllq(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSLLQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsllqEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psllq(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSLLQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSLLW` (PSLLW). 
/// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Imm  |
/// | 2 | Mm, Mem  |
/// | 3 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsllwEmitter<A, B> {
    fn mmx_psllw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsllwEmitter<Mm, Imm> for Assembler<'a> {
    fn mmx_psllw(&mut self, op0: Mm, op1: Imm) {
        self.emit(MMX_PSLLWRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsllwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psllw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSLLWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsllwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psllw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSLLWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSRAD` (PSRAD). 
/// Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Imm  |
/// | 2 | Mm, Mem  |
/// | 3 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsradEmitter<A, B> {
    fn mmx_psrad(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsradEmitter<Mm, Imm> for Assembler<'a> {
    fn mmx_psrad(&mut self, op0: Mm, op1: Imm) {
        self.emit(MMX_PSRADRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsradEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psrad(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSRADRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsradEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psrad(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSRADRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSRAW` (PSRAW). 
/// Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Imm  |
/// | 2 | Mm, Mem  |
/// | 3 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsrawEmitter<A, B> {
    fn mmx_psraw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsrawEmitter<Mm, Imm> for Assembler<'a> {
    fn mmx_psraw(&mut self, op0: Mm, op1: Imm) {
        self.emit(MMX_PSRAWRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsrawEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psraw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSRAWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsrawEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psraw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSRAWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSRLD` (PSRLD). 
/// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Imm  |
/// | 2 | Mm, Mem  |
/// | 3 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsrldEmitter<A, B> {
    fn mmx_psrld(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsrldEmitter<Mm, Imm> for Assembler<'a> {
    fn mmx_psrld(&mut self, op0: Mm, op1: Imm) {
        self.emit(MMX_PSRLDRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsrldEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psrld(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSRLDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsrldEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psrld(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSRLDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSRLQ` (PSRLQ). 
/// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Imm  |
/// | 2 | Mm, Mem  |
/// | 3 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsrlqEmitter<A, B> {
    fn mmx_psrlq(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsrlqEmitter<Mm, Imm> for Assembler<'a> {
    fn mmx_psrlq(&mut self, op0: Mm, op1: Imm) {
        self.emit(MMX_PSRLQRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsrlqEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psrlq(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSRLQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsrlqEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psrlq(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSRLQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSRLW` (PSRLW). 
/// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Imm  |
/// | 2 | Mm, Mem  |
/// | 3 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsrlwEmitter<A, B> {
    fn mmx_psrlw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsrlwEmitter<Mm, Imm> for Assembler<'a> {
    fn mmx_psrlw(&mut self, op0: Mm, op1: Imm) {
        self.emit(MMX_PSRLWRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsrlwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psrlw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSRLWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsrlwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psrlw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSRLWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSUBB` (PSUBB). 
/// Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsubbEmitter<A, B> {
    fn mmx_psubb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsubbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psubb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSUBBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsubbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psubb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSUBBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSUBD` (PSUBD). 
/// Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsubdEmitter<A, B> {
    fn mmx_psubd(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsubdEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psubd(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSUBDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsubdEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psubd(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSUBDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSUBQ` (PSUBQ). 
/// Subtracts the second operand (source operand) from the first operand (destination operand) and stores the result in the destination operand. When packed quadword operands are used, a SIMD subtract is performed. When a quadword result is too large to be represented in 64 bits (overflow), the result is wrapped around and the low 64 bits are written to the destination element (that is, the carry is ignored).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsubqEmitter<A, B> {
    fn mmx_psubq(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsubqEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psubq(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSUBQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsubqEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psubq(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSUBQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSUBSB` (PSUBSB). 
/// Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsubsbEmitter<A, B> {
    fn mmx_psubsb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsubsbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psubsb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSUBSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsubsbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psubsb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSUBSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSUBSW` (PSUBSW). 
/// Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsubswEmitter<A, B> {
    fn mmx_psubsw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsubswEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psubsw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSUBSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsubswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psubsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSUBSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSUBUSB` (PSUBUSB). 
/// Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsubusbEmitter<A, B> {
    fn mmx_psubusb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsubusbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psubusb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSUBUSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsubusbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psubusb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSUBUSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSUBUSW` (PSUBUSW). 
/// Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsubuswEmitter<A, B> {
    fn mmx_psubusw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsubuswEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psubusw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSUBUSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsubuswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psubusw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSUBUSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSUBW` (PSUBW). 
/// Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPsubwEmitter<A, B> {
    fn mmx_psubw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsubwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psubw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSUBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsubwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psubw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSUBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PUNPCKHBW` (PUNPCKHBW). 
/// Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPunpckhbwEmitter<A, B> {
    fn mmx_punpckhbw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPunpckhbwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_punpckhbw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PUNPCKHBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPunpckhbwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_punpckhbw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PUNPCKHBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PUNPCKHDQ` (PUNPCKHDQ). 
/// Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPunpckhdqEmitter<A, B> {
    fn mmx_punpckhdq(&mut self, op0: A, op1: B);
}

impl<'a> MmxPunpckhdqEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_punpckhdq(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PUNPCKHDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPunpckhdqEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_punpckhdq(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PUNPCKHDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PUNPCKHWD` (PUNPCKHWD). 
/// Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPunpckhwdEmitter<A, B> {
    fn mmx_punpckhwd(&mut self, op0: A, op1: B);
}

impl<'a> MmxPunpckhwdEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_punpckhwd(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PUNPCKHWDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPunpckhwdEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_punpckhwd(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PUNPCKHWDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PUNPCKLBW` (PUNPCKLBW). 
/// Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPunpcklbwEmitter<A, B> {
    fn mmx_punpcklbw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPunpcklbwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_punpcklbw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PUNPCKLBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPunpcklbwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_punpcklbw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PUNPCKLBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PUNPCKLDQ` (PUNPCKLDQ). 
/// Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPunpckldqEmitter<A, B> {
    fn mmx_punpckldq(&mut self, op0: A, op1: B);
}

impl<'a> MmxPunpckldqEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_punpckldq(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PUNPCKLDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPunpckldqEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_punpckldq(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PUNPCKLDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PUNPCKLWD` (PUNPCKLWD). 
/// Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPunpcklwdEmitter<A, B> {
    fn mmx_punpcklwd(&mut self, op0: A, op1: B);
}

impl<'a> MmxPunpcklwdEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_punpcklwd(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PUNPCKLWDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPunpcklwdEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_punpcklwd(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PUNPCKLWDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PXOR` (PXOR). 
/// Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PXOR.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPxorEmitter<A, B> {
    fn mmx_pxor(&mut self, op0: A, op1: B);
}

impl<'a> MmxPxorEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pxor(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PXORRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPxorEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pxor(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PXORRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `MMX_EMMS` (EMMS). 
    /// Sets the values of all the tags in the x87 FPU tag word to empty (all 1s). This operation marks the x87 FPU data registers (which are aliased to the MMX technology registers) as available for use by x87 FPU floating-point instructions. (See Figure 8-7 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for the format of the x87 FPU tag word.) All other MMX instructions (other than the EMMS instruction) set all the tags in x87 FPU tag word to valid (all 0s).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/EMMS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | (none)   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_emms(&mut self)
    where Assembler<'a>: MmxEmmsEmitter {
        <Self as MmxEmmsEmitter>::mmx_emms(self);
    }
    /// `MMX_MOVD_G2M`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Gpd  |
    /// | 2 | Mm, Mem  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_movd_g2m<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxMovdG2mEmitter<A, B> {
        <Self as MmxMovdG2mEmitter<A, B>>::mmx_movd_g2m(self, op0, op1);
    }
    /// `MMX_MOVD_M2G`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Mm  |
    /// | 2 | Mem, Mm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_movd_m2g<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxMovdM2gEmitter<A, B> {
        <Self as MmxMovdM2gEmitter<A, B>>::mmx_movd_m2g(self, op0, op1);
    }
    /// `MMX_MOVQ` (MOVQ). 
    /// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Mm  |
    /// | 2 | Mm, Mem  |
    /// | 3 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_movq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxMovqEmitter<A, B> {
        <Self as MmxMovqEmitter<A, B>>::mmx_movq(self, op0, op1);
    }
    /// `MMX_MOVQ_G2M`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Gpd  |
    /// | 2 | Mm, Mem  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_movq_g2m<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxMovqG2mEmitter<A, B> {
        <Self as MmxMovqG2mEmitter<A, B>>::mmx_movq_g2m(self, op0, op1);
    }
    /// `MMX_MOVQ_M2G`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Mm  |
    /// | 2 | Mem, Mm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_movq_m2g<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxMovqM2gEmitter<A, B> {
        <Self as MmxMovqM2gEmitter<A, B>>::mmx_movq_m2g(self, op0, op1);
    }
    /// `MMX_PACKSSDW` (PACKSSDW). 
    /// Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_packssdw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPackssdwEmitter<A, B> {
        <Self as MmxPackssdwEmitter<A, B>>::mmx_packssdw(self, op0, op1);
    }
    /// `MMX_PACKSSWB` (PACKSSWB). 
    /// Converts packed signed word integers into packed signed byte integers (PACKSSWB) or converts packed signed doubleword integers into packed signed word integers (PACKSSDW), using saturation to handle overflow conditions. See Figure 4-6 for an example of the packing operation.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PACKSSWB%3APACKSSDW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_packsswb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPacksswbEmitter<A, B> {
        <Self as MmxPacksswbEmitter<A, B>>::mmx_packsswb(self, op0, op1);
    }
    /// `MMX_PACKUSWB` (PACKUSWB). 
    /// Converts 4, 8, 16, or 32 signed word integers from the destination operand (first operand) and 4, 8, 16, or 32 signed word integers from the source operand (second operand) into 8, 16, 32 or 64 unsigned byte integers and stores the result in the destination operand. (See Figure 4-6 for an example of the packing operation.) If a signed word integer value is beyond the range of an unsigned byte integer (that is, greater than FFH or less than 00H), the saturated unsigned byte integer value of FFH or 00H, respectively, is stored in the destination.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PACKUSWB.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_packuswb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPackuswbEmitter<A, B> {
        <Self as MmxPackuswbEmitter<A, B>>::mmx_packuswb(self, op0, op1);
    }
    /// `MMX_PADDB` (PADDB). 
    /// Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_paddb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPaddbEmitter<A, B> {
        <Self as MmxPaddbEmitter<A, B>>::mmx_paddb(self, op0, op1);
    }
    /// `MMX_PADDD` (PADDD). 
    /// Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_paddd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPadddEmitter<A, B> {
        <Self as MmxPadddEmitter<A, B>>::mmx_paddd(self, op0, op1);
    }
    /// `MMX_PADDQ` (PADDQ). 
    /// Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_paddq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPaddqEmitter<A, B> {
        <Self as MmxPaddqEmitter<A, B>>::mmx_paddq(self, op0, op1);
    }
    /// `MMX_PADDSB` (PADDSB). 
    /// Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_paddsb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPaddsbEmitter<A, B> {
        <Self as MmxPaddsbEmitter<A, B>>::mmx_paddsb(self, op0, op1);
    }
    /// `MMX_PADDSW` (PADDSW). 
    /// Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDSB%3APADDSW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_paddsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPaddswEmitter<A, B> {
        <Self as MmxPaddswEmitter<A, B>>::mmx_paddsw(self, op0, op1);
    }
    /// `MMX_PADDUSB` (PADDUSB). 
    /// Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_paddusb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPaddusbEmitter<A, B> {
        <Self as MmxPaddusbEmitter<A, B>>::mmx_paddusb(self, op0, op1);
    }
    /// `MMX_PADDUSW` (PADDUSW). 
    /// Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDUSB%3APADDUSW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_paddusw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPadduswEmitter<A, B> {
        <Self as MmxPadduswEmitter<A, B>>::mmx_paddusw(self, op0, op1);
    }
    /// `MMX_PADDW` (PADDW). 
    /// Performs a SIMD add of the packed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PADDB%3APADDW%3APADDD%3APADDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_paddw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPaddwEmitter<A, B> {
        <Self as MmxPaddwEmitter<A, B>>::mmx_paddw(self, op0, op1);
    }
    /// `MMX_PAND` (PAND). 
    /// Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PAND.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pand<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPandEmitter<A, B> {
        <Self as MmxPandEmitter<A, B>>::mmx_pand(self, op0, op1);
    }
    /// `MMX_PANDN` (PANDN). 
    /// Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PANDN.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pandn<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPandnEmitter<A, B> {
        <Self as MmxPandnEmitter<A, B>>::mmx_pandn(self, op0, op1);
    }
    /// `MMX_PCMPEQB` (PCMPEQB). 
    /// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pcmpeqb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPcmpeqbEmitter<A, B> {
        <Self as MmxPcmpeqbEmitter<A, B>>::mmx_pcmpeqb(self, op0, op1);
    }
    /// `MMX_PCMPEQD` (PCMPEQD). 
    /// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pcmpeqd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPcmpeqdEmitter<A, B> {
        <Self as MmxPcmpeqdEmitter<A, B>>::mmx_pcmpeqd(self, op0, op1);
    }
    /// `MMX_PCMPEQW` (PCMPEQW). 
    /// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pcmpeqw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPcmpeqwEmitter<A, B> {
        <Self as MmxPcmpeqwEmitter<A, B>>::mmx_pcmpeqw(self, op0, op1);
    }
    /// `MMX_PCMPGTB` (PCMPGTB). 
    /// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pcmpgtb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPcmpgtbEmitter<A, B> {
        <Self as MmxPcmpgtbEmitter<A, B>>::mmx_pcmpgtb(self, op0, op1);
    }
    /// `MMX_PCMPGTD` (PCMPGTD). 
    /// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pcmpgtd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPcmpgtdEmitter<A, B> {
        <Self as MmxPcmpgtdEmitter<A, B>>::mmx_pcmpgtd(self, op0, op1);
    }
    /// `MMX_PCMPGTW` (PCMPGTW). 
    /// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pcmpgtw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPcmpgtwEmitter<A, B> {
        <Self as MmxPcmpgtwEmitter<A, B>>::mmx_pcmpgtw(self, op0, op1);
    }
    /// `MMX_PMADDWD` (PMADDWD). 
    /// Multiplies the individual signed words of the destination operand (first operand) by the corresponding signed words of the source operand (second operand), producing temporary signed, doubleword results. The adjacent double-word results are then summed and stored in the destination operand. For example, the corresponding low-order words (15-0) and (31-16) in the source and destination operands are multiplied by one another and the double-word results are added together and stored in the low doubleword of the destination register (31-0). The same operation is performed on the other pairs of adjacent words. (Figure 4-11 shows this operation when using 64-bit operands).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMADDWD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pmaddwd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPmaddwdEmitter<A, B> {
        <Self as MmxPmaddwdEmitter<A, B>>::mmx_pmaddwd(self, op0, op1);
    }
    /// `MMX_PMULHW` (PMULHW). 
    /// Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULHW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pmulhw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPmulhwEmitter<A, B> {
        <Self as MmxPmulhwEmitter<A, B>>::mmx_pmulhw(self, op0, op1);
    }
    /// `MMX_PMULLW` (PMULLW). 
    /// Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the low 16 bits of each intermediate 32-bit result in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pmullw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPmullwEmitter<A, B> {
        <Self as MmxPmullwEmitter<A, B>>::mmx_pmullw(self, op0, op1);
    }
    /// `MMX_PMULUDQ` (PMULUDQ). 
    /// Multiplies the first operand (destination operand) by the second operand (source operand) and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULUDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pmuludq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPmuludqEmitter<A, B> {
        <Self as MmxPmuludqEmitter<A, B>>::mmx_pmuludq(self, op0, op1);
    }
    /// `MMX_POR` (POR). 
    /// Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/POR.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_por<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPorEmitter<A, B> {
        <Self as MmxPorEmitter<A, B>>::mmx_por(self, op0, op1);
    }
    /// `MMX_PSLLD` (PSLLD). 
    /// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Imm  |
    /// | 2 | Mm, Mem  |
    /// | 3 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pslld<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPslldEmitter<A, B> {
        <Self as MmxPslldEmitter<A, B>>::mmx_pslld(self, op0, op1);
    }
    /// `MMX_PSLLQ` (PSLLQ). 
    /// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Imm  |
    /// | 2 | Mm, Mem  |
    /// | 3 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psllq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsllqEmitter<A, B> {
        <Self as MmxPsllqEmitter<A, B>>::mmx_psllq(self, op0, op1);
    }
    /// `MMX_PSLLW` (PSLLW). 
    /// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the left by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted left, the empty low-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-17 gives an example of shifting words in a 64-bit operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSLLW%3APSLLD%3APSLLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Imm  |
    /// | 2 | Mm, Mem  |
    /// | 3 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psllw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsllwEmitter<A, B> {
        <Self as MmxPsllwEmitter<A, B>>::mmx_psllw(self, op0, op1);
    }
    /// `MMX_PSRAD` (PSRAD). 
    /// Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Imm  |
    /// | 2 | Mm, Mem  |
    /// | 3 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psrad<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsradEmitter<A, B> {
        <Self as MmxPsradEmitter<A, B>>::mmx_psrad(self, op0, op1);
    }
    /// `MMX_PSRAW` (PSRAW). 
    /// Shifts the bits in the individual data elements (words, doublewords or quadwords) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are filled with the initial value of the sign bit of the data element. If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for quadwords), each destination data element is filled with the initial value of the sign bit of the element. (Figure 4-18 gives an example of shifting words in a 64-bit operand.)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRAW%3APSRAD%3APSRAQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Imm  |
    /// | 2 | Mm, Mem  |
    /// | 3 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psraw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsrawEmitter<A, B> {
        <Self as MmxPsrawEmitter<A, B>>::mmx_psraw(self, op0, op1);
    }
    /// `MMX_PSRLD` (PSRLD). 
    /// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Imm  |
    /// | 2 | Mm, Mem  |
    /// | 3 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psrld<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsrldEmitter<A, B> {
        <Self as MmxPsrldEmitter<A, B>>::mmx_psrld(self, op0, op1);
    }
    /// `MMX_PSRLQ` (PSRLQ). 
    /// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Imm  |
    /// | 2 | Mm, Mem  |
    /// | 3 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psrlq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsrlqEmitter<A, B> {
        <Self as MmxPsrlqEmitter<A, B>>::mmx_psrlq(self, op0, op1);
    }
    /// `MMX_PSRLW` (PSRLW). 
    /// Shifts the bits in the individual data elements (words, doublewords, or quadword) in the destination operand (first operand) to the right by the number of bits specified in the count operand (second operand). As the bits in the data elements are shifted right, the empty high-order bits are cleared (set to 0). If the value specified by the count operand is greater than 15 (for words), 31 (for doublewords), or 63 (for a quadword), then the destination operand is set to all 0s. Figure 4-19 gives an example of shifting words in a 64-bit operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRLW%3APSRLD%3APSRLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Imm  |
    /// | 2 | Mm, Mem  |
    /// | 3 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psrlw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsrlwEmitter<A, B> {
        <Self as MmxPsrlwEmitter<A, B>>::mmx_psrlw(self, op0, op1);
    }
    /// `MMX_PSUBB` (PSUBB). 
    /// Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psubb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsubbEmitter<A, B> {
        <Self as MmxPsubbEmitter<A, B>>::mmx_psubb(self, op0, op1);
    }
    /// `MMX_PSUBD` (PSUBD). 
    /// Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psubd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsubdEmitter<A, B> {
        <Self as MmxPsubdEmitter<A, B>>::mmx_psubd(self, op0, op1);
    }
    /// `MMX_PSUBQ` (PSUBQ). 
    /// Subtracts the second operand (source operand) from the first operand (destination operand) and stores the result in the destination operand. When packed quadword operands are used, a SIMD subtract is performed. When a quadword result is too large to be represented in 64 bits (overflow), the result is wrapped around and the low 64 bits are written to the destination element (that is, the carry is ignored).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psubq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsubqEmitter<A, B> {
        <Self as MmxPsubqEmitter<A, B>>::mmx_psubq(self, op0, op1);
    }
    /// `MMX_PSUBSB` (PSUBSB). 
    /// Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psubsb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsubsbEmitter<A, B> {
        <Self as MmxPsubsbEmitter<A, B>>::mmx_psubsb(self, op0, op1);
    }
    /// `MMX_PSUBSW` (PSUBSW). 
    /// Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with signed saturation, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBSB%3APSUBSW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psubsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsubswEmitter<A, B> {
        <Self as MmxPsubswEmitter<A, B>>::mmx_psubsw(self, op0, op1);
    }
    /// `MMX_PSUBUSB` (PSUBUSB). 
    /// Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psubusb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsubusbEmitter<A, B> {
        <Self as MmxPsubusbEmitter<A, B>>::mmx_psubusb(self, op0, op1);
    }
    /// `MMX_PSUBUSW` (PSUBUSW). 
    /// Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with unsigned saturation, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBUSB%3APSUBUSW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psubusw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsubuswEmitter<A, B> {
        <Self as MmxPsubuswEmitter<A, B>>::mmx_psubusw(self, op0, op1);
    }
    /// `MMX_PSUBW` (PSUBW). 
    /// Performs a SIMD subtract of the packed integers of the source operand (second operand) from the packed integers of the destination operand (first operand), and stores the packed integer results in the destination operand. See Figure 9-4 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD operation. Overflow is handled with wraparound, as described in the following paragraphs.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSUBB%3APSUBW%3APSUBD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_psubw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsubwEmitter<A, B> {
        <Self as MmxPsubwEmitter<A, B>>::mmx_psubw(self, op0, op1);
    }
    /// `MMX_PUNPCKHBW` (PUNPCKHBW). 
    /// Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_punpckhbw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPunpckhbwEmitter<A, B> {
        <Self as MmxPunpckhbwEmitter<A, B>>::mmx_punpckhbw(self, op0, op1);
    }
    /// `MMX_PUNPCKHDQ` (PUNPCKHDQ). 
    /// Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_punpckhdq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPunpckhdqEmitter<A, B> {
        <Self as MmxPunpckhdqEmitter<A, B>>::mmx_punpckhdq(self, op0, op1);
    }
    /// `MMX_PUNPCKHWD` (PUNPCKHWD). 
    /// Unpacks and interleaves the high-order data elements (bytes, words, doublewords, or quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. Figure 4-20 shows the unpack operation for bytes in 64-bit operands. The low-order data elements are ignored.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKHBW%3APUNPCKHWD%3APUNPCKHDQ%3APUNPCKHQDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_punpckhwd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPunpckhwdEmitter<A, B> {
        <Self as MmxPunpckhwdEmitter<A, B>>::mmx_punpckhwd(self, op0, op1);
    }
    /// `MMX_PUNPCKLBW` (PUNPCKLBW). 
    /// Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_punpcklbw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPunpcklbwEmitter<A, B> {
        <Self as MmxPunpcklbwEmitter<A, B>>::mmx_punpcklbw(self, op0, op1);
    }
    /// `MMX_PUNPCKLDQ` (PUNPCKLDQ). 
    /// Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_punpckldq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPunpckldqEmitter<A, B> {
        <Self as MmxPunpckldqEmitter<A, B>>::mmx_punpckldq(self, op0, op1);
    }
    /// `MMX_PUNPCKLWD` (PUNPCKLWD). 
    /// Unpacks and interleaves the low-order data elements (bytes, words, doublewords, and quadwords) of the destination operand (first operand) and source operand (second operand) into the destination operand. (Figure 4-22 shows the unpack operation for bytes in 64-bit operands.). The high-order data elements are ignored.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PUNPCKLBW%3APUNPCKLWD%3APUNPCKLDQ%3APUNPCKLQDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_punpcklwd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPunpcklwdEmitter<A, B> {
        <Self as MmxPunpcklwdEmitter<A, B>>::mmx_punpcklwd(self, op0, op1);
    }
    /// `MMX_PXOR` (PXOR). 
    /// Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PXOR.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pxor<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPxorEmitter<A, B> {
        <Self as MmxPxorEmitter<A, B>>::mmx_pxor(self, op0, op1);
    }
}
