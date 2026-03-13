use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `MMX_PABSB` (PABSB). 
/// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
pub trait MmxPabsbEmitter<A, B> {
    fn mmx_pabsb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPabsbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pabsb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PABSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPabsbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pabsb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PABSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PABSD` (PABSD). 
/// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
pub trait MmxPabsdEmitter<A, B> {
    fn mmx_pabsd(&mut self, op0: A, op1: B);
}

impl<'a> MmxPabsdEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pabsd(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PABSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPabsdEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pabsd(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PABSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PABSW` (PABSW). 
/// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
pub trait MmxPabswEmitter<A, B> {
    fn mmx_pabsw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPabswEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pabsw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PABSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPabswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pabsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PABSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PALIGNR` (PALIGNR). 
/// (V)PALIGNR concatenates the destination operand (the first operand) and the source operand (the second operand) into an intermediate composite, shifts the composite at byte granularity to the right by a constant immediate, and extracts the right-aligned result into the destination. The first and the second operands can be an MMX, XMM or a YMM register. The immediate value is considered unsigned. Immediate shift counts larger than the 2L (i.e., 32 for 128-bit operands, or 16 for 64-bit operands) produce a zero result. Both operands can be MMX registers, XMM registers or YMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PALIGNR.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------+
/// | # | Operands     |
/// +---+--------------+
/// | 1 | Mm, Mem, Imm |
/// | 2 | Mm, Mm, Imm  |
/// +---+--------------+
/// ```
pub trait MmxPalignrEmitter<A, B, C> {
    fn mmx_palignr(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> MmxPalignrEmitter<Mm, Mm, Imm> for Assembler<'a> {
    fn mmx_palignr(&mut self, op0: Mm, op1: Mm, op2: Imm) {
        self.emit(MMX_PALIGNRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> MmxPalignrEmitter<Mm, Mem, Imm> for Assembler<'a> {
    fn mmx_palignr(&mut self, op0: Mm, op1: Mem, op2: Imm) {
        self.emit(MMX_PALIGNRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `MMX_PHADDD` (PHADDD). 
/// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
pub trait MmxPhadddEmitter<A, B> {
    fn mmx_phaddd(&mut self, op0: A, op1: B);
}

impl<'a> MmxPhadddEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_phaddd(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PHADDDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPhadddEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phaddd(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PHADDDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PHADDSW` (PHADDSW). 
/// (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDSW.html).
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
pub trait MmxPhaddswEmitter<A, B> {
    fn mmx_phaddsw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPhaddswEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_phaddsw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PHADDSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPhaddswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phaddsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PHADDSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PHADDW` (PHADDW). 
/// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
pub trait MmxPhaddwEmitter<A, B> {
    fn mmx_phaddw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPhaddwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_phaddw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PHADDWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPhaddwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phaddw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PHADDWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PHSUBD` (PHSUBD). 
/// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
pub trait MmxPhsubdEmitter<A, B> {
    fn mmx_phsubd(&mut self, op0: A, op1: B);
}

impl<'a> MmxPhsubdEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_phsubd(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PHSUBDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPhsubdEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phsubd(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PHSUBDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PHSUBSW` (PHSUBSW). 
/// (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBSW.html).
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
pub trait MmxPhsubswEmitter<A, B> {
    fn mmx_phsubsw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPhsubswEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_phsubsw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PHSUBSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPhsubswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phsubsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PHSUBSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PHSUBW` (PHSUBW). 
/// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
pub trait MmxPhsubwEmitter<A, B> {
    fn mmx_phsubw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPhsubwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_phsubw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PHSUBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPhsubwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phsubw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PHSUBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PMADDUBSW` (PMADDUBSW). 
/// (V)PMADDUBSW multiplies vertically each unsigned byte of the destination operand (first operand) with the corresponding signed byte of the source operand (second operand), producing intermediate signed 16-bit integers. Each adjacent pair of signed words is added and the saturated result is packed to the destination operand. For example, the lowest-order bytes (bits 7-0) in the source and destination operands are multiplied and the intermediate signed word result is added with the corresponding intermediate result from the 2nd lowest-order bytes (bits 15-8) of the operands; the sign-saturated result is stored in the lowest word of the destination register (15-0). The same operation is performed on the other pairs of adjacent bytes. Both operands can be MMX register or XMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMADDUBSW.html).
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
pub trait MmxPmaddubswEmitter<A, B> {
    fn mmx_pmaddubsw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPmaddubswEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pmaddubsw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PMADDUBSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPmaddubswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pmaddubsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PMADDUBSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PMULHRSW` (PMULHRSW). 
/// PMULHRSW multiplies vertically each signed 16-bit integer from the destination operand (first operand) with the corresponding signed 16-bit integer of the source operand (second operand), producing intermediate, signed 32-bit integers. Each intermediate 32-bit integer is truncated to the 18 most significant bits. Rounding is always performed by adding 1 to the least significant bit of the 18-bit intermediate result. The final result is obtained by selecting the 16 bits immediately to the right of the most significant bit of each 18-bit intermediate result and packed to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULHRSW.html).
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
pub trait MmxPmulhrswEmitter<A, B> {
    fn mmx_pmulhrsw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPmulhrswEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pmulhrsw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PMULHRSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPmulhrswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pmulhrsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PMULHRSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSHUFB` (PSHUFB). 
/// PSHUFB performs in-place shuffles of bytes in the destination operand (the first operand) according to the shuffle control mask in the source operand (the second operand). The instruction permutes the data in the destination operand, leaving the shuffle mask unaffected. If the most significant bit (bit[7]) of each byte of the shuffle control mask is set, then constant zero is written in the result byte. Each byte in the shuffle control mask forms an index to permute the corresponding byte in the destination operand. The value of each index is the least significant 4 bits (128-bit operation) or 3 bits (64-bit operation) of the shuffle control byte. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFB.html).
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
pub trait MmxPshufbEmitter<A, B> {
    fn mmx_pshufb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPshufbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pshufb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSHUFBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPshufbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pshufb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSHUFBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSIGNB` (PSIGNB). 
/// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
pub trait MmxPsignbEmitter<A, B> {
    fn mmx_psignb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsignbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psignb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSIGNBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsignbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psignb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSIGNBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSIGND` (PSIGND). 
/// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
pub trait MmxPsigndEmitter<A, B> {
    fn mmx_psignd(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsigndEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psignd(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSIGNDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsigndEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psignd(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSIGNDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSIGNW` (PSIGNW). 
/// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
pub trait MmxPsignwEmitter<A, B> {
    fn mmx_psignw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsignwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psignw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSIGNWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsignwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psignw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSIGNWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PABSB` (PABSB). 
/// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
pub trait SsePabsbEmitter<A, B> {
    fn sse_pabsb(&mut self, op0: A, op1: B);
}

impl<'a> SsePabsbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pabsb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PABSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePabsbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pabsb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PABSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PABSD` (PABSD). 
/// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
pub trait SsePabsdEmitter<A, B> {
    fn sse_pabsd(&mut self, op0: A, op1: B);
}

impl<'a> SsePabsdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pabsd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PABSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePabsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pabsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PABSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PABSW` (PABSW). 
/// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
pub trait SsePabswEmitter<A, B> {
    fn sse_pabsw(&mut self, op0: A, op1: B);
}

impl<'a> SsePabswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pabsw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PABSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePabswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pabsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PABSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PALIGNR` (PALIGNR). 
/// (V)PALIGNR concatenates the destination operand (the first operand) and the source operand (the second operand) into an intermediate composite, shifts the composite at byte granularity to the right by a constant immediate, and extracts the right-aligned result into the destination. The first and the second operands can be an MMX, XMM or a YMM register. The immediate value is considered unsigned. Immediate shift counts larger than the 2L (i.e., 32 for 128-bit operands, or 16 for 64-bit operands) produce a zero result. Both operands can be MMX registers, XMM registers or YMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PALIGNR.html).
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
pub trait SsePalignrEmitter<A, B, C> {
    fn sse_palignr(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePalignrEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_palignr(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_PALIGNRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePalignrEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_palignr(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PALIGNRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PHADDD` (PHADDD). 
/// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
pub trait SsePhadddEmitter<A, B> {
    fn sse_phaddd(&mut self, op0: A, op1: B);
}

impl<'a> SsePhadddEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_phaddd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PHADDDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePhadddEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phaddd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PHADDDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PHADDSW` (PHADDSW). 
/// (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDSW.html).
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
pub trait SsePhaddswEmitter<A, B> {
    fn sse_phaddsw(&mut self, op0: A, op1: B);
}

impl<'a> SsePhaddswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_phaddsw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PHADDSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePhaddswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phaddsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PHADDSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PHADDW` (PHADDW). 
/// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
pub trait SsePhaddwEmitter<A, B> {
    fn sse_phaddw(&mut self, op0: A, op1: B);
}

impl<'a> SsePhaddwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_phaddw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PHADDWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePhaddwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phaddw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PHADDWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PHSUBD` (PHSUBD). 
/// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
pub trait SsePhsubdEmitter<A, B> {
    fn sse_phsubd(&mut self, op0: A, op1: B);
}

impl<'a> SsePhsubdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_phsubd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PHSUBDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePhsubdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phsubd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PHSUBDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PHSUBSW` (PHSUBSW). 
/// (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBSW.html).
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
pub trait SsePhsubswEmitter<A, B> {
    fn sse_phsubsw(&mut self, op0: A, op1: B);
}

impl<'a> SsePhsubswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_phsubsw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PHSUBSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePhsubswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phsubsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PHSUBSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PHSUBW` (PHSUBW). 
/// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
pub trait SsePhsubwEmitter<A, B> {
    fn sse_phsubw(&mut self, op0: A, op1: B);
}

impl<'a> SsePhsubwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_phsubw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PHSUBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePhsubwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phsubw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PHSUBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMADDUBSW` (PMADDUBSW). 
/// (V)PMADDUBSW multiplies vertically each unsigned byte of the destination operand (first operand) with the corresponding signed byte of the source operand (second operand), producing intermediate signed 16-bit integers. Each adjacent pair of signed words is added and the saturated result is packed to the destination operand. For example, the lowest-order bytes (bits 7-0) in the source and destination operands are multiplied and the intermediate signed word result is added with the corresponding intermediate result from the 2nd lowest-order bytes (bits 15-8) of the operands; the sign-saturated result is stored in the lowest word of the destination register (15-0). The same operation is performed on the other pairs of adjacent bytes. Both operands can be MMX register or XMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMADDUBSW.html).
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
pub trait SsePmaddubswEmitter<A, B> {
    fn sse_pmaddubsw(&mut self, op0: A, op1: B);
}

impl<'a> SsePmaddubswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmaddubsw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMADDUBSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmaddubswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmaddubsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMADDUBSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMULHRSW` (PMULHRSW). 
/// PMULHRSW multiplies vertically each signed 16-bit integer from the destination operand (first operand) with the corresponding signed 16-bit integer of the source operand (second operand), producing intermediate, signed 32-bit integers. Each intermediate 32-bit integer is truncated to the 18 most significant bits. Rounding is always performed by adding 1 to the least significant bit of the 18-bit intermediate result. The final result is obtained by selecting the 16 bits immediately to the right of the most significant bit of each 18-bit intermediate result and packed to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULHRSW.html).
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
pub trait SsePmulhrswEmitter<A, B> {
    fn sse_pmulhrsw(&mut self, op0: A, op1: B);
}

impl<'a> SsePmulhrswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmulhrsw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMULHRSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmulhrswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmulhrsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMULHRSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSHUFB` (PSHUFB). 
/// PSHUFB performs in-place shuffles of bytes in the destination operand (the first operand) according to the shuffle control mask in the source operand (the second operand). The instruction permutes the data in the destination operand, leaving the shuffle mask unaffected. If the most significant bit (bit[7]) of each byte of the shuffle control mask is set, then constant zero is written in the result byte. Each byte in the shuffle control mask forms an index to permute the corresponding byte in the destination operand. The value of each index is the least significant 4 bits (128-bit operation) or 3 bits (64-bit operation) of the shuffle control byte. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFB.html).
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
pub trait SsePshufbEmitter<A, B> {
    fn sse_pshufb(&mut self, op0: A, op1: B);
}

impl<'a> SsePshufbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pshufb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSHUFBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePshufbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pshufb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSHUFBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSIGNB` (PSIGNB). 
/// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
pub trait SsePsignbEmitter<A, B> {
    fn sse_psignb(&mut self, op0: A, op1: B);
}

impl<'a> SsePsignbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psignb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSIGNBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsignbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psignb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSIGNBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSIGND` (PSIGND). 
/// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
pub trait SsePsigndEmitter<A, B> {
    fn sse_psignd(&mut self, op0: A, op1: B);
}

impl<'a> SsePsigndEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psignd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSIGNDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsigndEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psignd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSIGNDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSIGNW` (PSIGNW). 
/// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
pub trait SsePsignwEmitter<A, B> {
    fn sse_psignw(&mut self, op0: A, op1: B);
}

impl<'a> SsePsignwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psignw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSIGNWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsignwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psignw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSIGNWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `MMX_PABSB` (PABSB). 
    /// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
    pub fn mmx_pabsb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPabsbEmitter<A, B> {
        <Self as MmxPabsbEmitter<A, B>>::mmx_pabsb(self, op0, op1);
    }
    /// `MMX_PABSD` (PABSD). 
    /// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
    pub fn mmx_pabsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPabsdEmitter<A, B> {
        <Self as MmxPabsdEmitter<A, B>>::mmx_pabsd(self, op0, op1);
    }
    /// `MMX_PABSW` (PABSW). 
    /// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
    pub fn mmx_pabsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPabswEmitter<A, B> {
        <Self as MmxPabswEmitter<A, B>>::mmx_pabsw(self, op0, op1);
    }
    /// `MMX_PALIGNR` (PALIGNR). 
    /// (V)PALIGNR concatenates the destination operand (the first operand) and the source operand (the second operand) into an intermediate composite, shifts the composite at byte granularity to the right by a constant immediate, and extracts the right-aligned result into the destination. The first and the second operands can be an MMX, XMM or a YMM register. The immediate value is considered unsigned. Immediate shift counts larger than the 2L (i.e., 32 for 128-bit operands, or 16 for 64-bit operands) produce a zero result. Both operands can be MMX registers, XMM registers or YMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PALIGNR.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------+
    /// | # | Operands     |
    /// +---+--------------+
    /// | 1 | Mm, Mem, Imm |
    /// | 2 | Mm, Mm, Imm  |
    /// +---+--------------+
    /// ```
    #[inline]
    pub fn mmx_palignr<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: MmxPalignrEmitter<A, B, C> {
        <Self as MmxPalignrEmitter<A, B, C>>::mmx_palignr(self, op0, op1, op2);
    }
    /// `MMX_PHADDD` (PHADDD). 
    /// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
    pub fn mmx_phaddd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPhadddEmitter<A, B> {
        <Self as MmxPhadddEmitter<A, B>>::mmx_phaddd(self, op0, op1);
    }
    /// `MMX_PHADDSW` (PHADDSW). 
    /// (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDSW.html).
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
    pub fn mmx_phaddsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPhaddswEmitter<A, B> {
        <Self as MmxPhaddswEmitter<A, B>>::mmx_phaddsw(self, op0, op1);
    }
    /// `MMX_PHADDW` (PHADDW). 
    /// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
    pub fn mmx_phaddw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPhaddwEmitter<A, B> {
        <Self as MmxPhaddwEmitter<A, B>>::mmx_phaddw(self, op0, op1);
    }
    /// `MMX_PHSUBD` (PHSUBD). 
    /// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
    pub fn mmx_phsubd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPhsubdEmitter<A, B> {
        <Self as MmxPhsubdEmitter<A, B>>::mmx_phsubd(self, op0, op1);
    }
    /// `MMX_PHSUBSW` (PHSUBSW). 
    /// (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBSW.html).
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
    pub fn mmx_phsubsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPhsubswEmitter<A, B> {
        <Self as MmxPhsubswEmitter<A, B>>::mmx_phsubsw(self, op0, op1);
    }
    /// `MMX_PHSUBW` (PHSUBW). 
    /// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
    pub fn mmx_phsubw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPhsubwEmitter<A, B> {
        <Self as MmxPhsubwEmitter<A, B>>::mmx_phsubw(self, op0, op1);
    }
    /// `MMX_PMADDUBSW` (PMADDUBSW). 
    /// (V)PMADDUBSW multiplies vertically each unsigned byte of the destination operand (first operand) with the corresponding signed byte of the source operand (second operand), producing intermediate signed 16-bit integers. Each adjacent pair of signed words is added and the saturated result is packed to the destination operand. For example, the lowest-order bytes (bits 7-0) in the source and destination operands are multiplied and the intermediate signed word result is added with the corresponding intermediate result from the 2nd lowest-order bytes (bits 15-8) of the operands; the sign-saturated result is stored in the lowest word of the destination register (15-0). The same operation is performed on the other pairs of adjacent bytes. Both operands can be MMX register or XMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMADDUBSW.html).
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
    pub fn mmx_pmaddubsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPmaddubswEmitter<A, B> {
        <Self as MmxPmaddubswEmitter<A, B>>::mmx_pmaddubsw(self, op0, op1);
    }
    /// `MMX_PMULHRSW` (PMULHRSW). 
    /// PMULHRSW multiplies vertically each signed 16-bit integer from the destination operand (first operand) with the corresponding signed 16-bit integer of the source operand (second operand), producing intermediate, signed 32-bit integers. Each intermediate 32-bit integer is truncated to the 18 most significant bits. Rounding is always performed by adding 1 to the least significant bit of the 18-bit intermediate result. The final result is obtained by selecting the 16 bits immediately to the right of the most significant bit of each 18-bit intermediate result and packed to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULHRSW.html).
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
    pub fn mmx_pmulhrsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPmulhrswEmitter<A, B> {
        <Self as MmxPmulhrswEmitter<A, B>>::mmx_pmulhrsw(self, op0, op1);
    }
    /// `MMX_PSHUFB` (PSHUFB). 
    /// PSHUFB performs in-place shuffles of bytes in the destination operand (the first operand) according to the shuffle control mask in the source operand (the second operand). The instruction permutes the data in the destination operand, leaving the shuffle mask unaffected. If the most significant bit (bit[7]) of each byte of the shuffle control mask is set, then constant zero is written in the result byte. Each byte in the shuffle control mask forms an index to permute the corresponding byte in the destination operand. The value of each index is the least significant 4 bits (128-bit operation) or 3 bits (64-bit operation) of the shuffle control byte. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFB.html).
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
    pub fn mmx_pshufb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPshufbEmitter<A, B> {
        <Self as MmxPshufbEmitter<A, B>>::mmx_pshufb(self, op0, op1);
    }
    /// `MMX_PSIGNB` (PSIGNB). 
    /// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
    pub fn mmx_psignb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsignbEmitter<A, B> {
        <Self as MmxPsignbEmitter<A, B>>::mmx_psignb(self, op0, op1);
    }
    /// `MMX_PSIGND` (PSIGND). 
    /// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
    pub fn mmx_psignd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsigndEmitter<A, B> {
        <Self as MmxPsigndEmitter<A, B>>::mmx_psignd(self, op0, op1);
    }
    /// `MMX_PSIGNW` (PSIGNW). 
    /// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
    pub fn mmx_psignw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsignwEmitter<A, B> {
        <Self as MmxPsignwEmitter<A, B>>::mmx_psignw(self, op0, op1);
    }
    /// `SSE_PABSB` (PABSB). 
    /// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
    pub fn sse_pabsb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePabsbEmitter<A, B> {
        <Self as SsePabsbEmitter<A, B>>::sse_pabsb(self, op0, op1);
    }
    /// `SSE_PABSD` (PABSD). 
    /// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
    pub fn sse_pabsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePabsdEmitter<A, B> {
        <Self as SsePabsdEmitter<A, B>>::sse_pabsd(self, op0, op1);
    }
    /// `SSE_PABSW` (PABSW). 
    /// PABSB/W/D computes the absolute value of each data element of the source operand (the second operand) and stores the UNSIGNED results in the destination operand (the first operand). PABSB operates on signed bytes, PABSW operates on signed 16-bit words, and PABSD operates on signed 32-bit integers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PABSB%3APABSW%3APABSD%3APABSQ.html).
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
    pub fn sse_pabsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePabswEmitter<A, B> {
        <Self as SsePabswEmitter<A, B>>::sse_pabsw(self, op0, op1);
    }
    /// `SSE_PALIGNR` (PALIGNR). 
    /// (V)PALIGNR concatenates the destination operand (the first operand) and the source operand (the second operand) into an intermediate composite, shifts the composite at byte granularity to the right by a constant immediate, and extracts the right-aligned result into the destination. The first and the second operands can be an MMX, XMM or a YMM register. The immediate value is considered unsigned. Immediate shift counts larger than the 2L (i.e., 32 for 128-bit operands, or 16 for 64-bit operands) produce a zero result. Both operands can be MMX registers, XMM registers or YMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PALIGNR.html).
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
    pub fn sse_palignr<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePalignrEmitter<A, B, C> {
        <Self as SsePalignrEmitter<A, B, C>>::sse_palignr(self, op0, op1, op2);
    }
    /// `SSE_PHADDD` (PHADDD). 
    /// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
    pub fn sse_phaddd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePhadddEmitter<A, B> {
        <Self as SsePhadddEmitter<A, B>>::sse_phaddd(self, op0, op1);
    }
    /// `SSE_PHADDSW` (PHADDSW). 
    /// (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDSW.html).
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
    pub fn sse_phaddsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePhaddswEmitter<A, B> {
        <Self as SsePhaddswEmitter<A, B>>::sse_phaddsw(self, op0, op1);
    }
    /// `SSE_PHADDW` (PHADDW). 
    /// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
    pub fn sse_phaddw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePhaddwEmitter<A, B> {
        <Self as SsePhaddwEmitter<A, B>>::sse_phaddw(self, op0, op1);
    }
    /// `SSE_PHSUBD` (PHSUBD). 
    /// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
    pub fn sse_phsubd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePhsubdEmitter<A, B> {
        <Self as SsePhsubdEmitter<A, B>>::sse_phsubd(self, op0, op1);
    }
    /// `SSE_PHSUBSW` (PHSUBSW). 
    /// (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBSW.html).
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
    pub fn sse_phsubsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePhsubswEmitter<A, B> {
        <Self as SsePhsubswEmitter<A, B>>::sse_phsubsw(self, op0, op1);
    }
    /// `SSE_PHSUBW` (PHSUBW). 
    /// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
    pub fn sse_phsubw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePhsubwEmitter<A, B> {
        <Self as SsePhsubwEmitter<A, B>>::sse_phsubw(self, op0, op1);
    }
    /// `SSE_PMADDUBSW` (PMADDUBSW). 
    /// (V)PMADDUBSW multiplies vertically each unsigned byte of the destination operand (first operand) with the corresponding signed byte of the source operand (second operand), producing intermediate signed 16-bit integers. Each adjacent pair of signed words is added and the saturated result is packed to the destination operand. For example, the lowest-order bytes (bits 7-0) in the source and destination operands are multiplied and the intermediate signed word result is added with the corresponding intermediate result from the 2nd lowest-order bytes (bits 15-8) of the operands; the sign-saturated result is stored in the lowest word of the destination register (15-0). The same operation is performed on the other pairs of adjacent bytes. Both operands can be MMX register or XMM registers. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMADDUBSW.html).
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
    pub fn sse_pmaddubsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmaddubswEmitter<A, B> {
        <Self as SsePmaddubswEmitter<A, B>>::sse_pmaddubsw(self, op0, op1);
    }
    /// `SSE_PMULHRSW` (PMULHRSW). 
    /// PMULHRSW multiplies vertically each signed 16-bit integer from the destination operand (first operand) with the corresponding signed 16-bit integer of the source operand (second operand), producing intermediate, signed 32-bit integers. Each intermediate 32-bit integer is truncated to the 18 most significant bits. Rounding is always performed by adding 1 to the least significant bit of the 18-bit intermediate result. The final result is obtained by selecting the 16 bits immediately to the right of the most significant bit of each 18-bit intermediate result and packed to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULHRSW.html).
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
    pub fn sse_pmulhrsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmulhrswEmitter<A, B> {
        <Self as SsePmulhrswEmitter<A, B>>::sse_pmulhrsw(self, op0, op1);
    }
    /// `SSE_PSHUFB` (PSHUFB). 
    /// PSHUFB performs in-place shuffles of bytes in the destination operand (the first operand) according to the shuffle control mask in the source operand (the second operand). The instruction permutes the data in the destination operand, leaving the shuffle mask unaffected. If the most significant bit (bit[7]) of each byte of the shuffle control mask is set, then constant zero is written in the result byte. Each byte in the shuffle control mask forms an index to permute the corresponding byte in the destination operand. The value of each index is the least significant 4 bits (128-bit operation) or 3 bits (64-bit operation) of the shuffle control byte. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFB.html).
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
    pub fn sse_pshufb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePshufbEmitter<A, B> {
        <Self as SsePshufbEmitter<A, B>>::sse_pshufb(self, op0, op1);
    }
    /// `SSE_PSIGNB` (PSIGNB). 
    /// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
    pub fn sse_psignb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsignbEmitter<A, B> {
        <Self as SsePsignbEmitter<A, B>>::sse_psignb(self, op0, op1);
    }
    /// `SSE_PSIGND` (PSIGND). 
    /// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
    pub fn sse_psignd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsigndEmitter<A, B> {
        <Self as SsePsigndEmitter<A, B>>::sse_psignd(self, op0, op1);
    }
    /// `SSE_PSIGNW` (PSIGNW). 
    /// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
    pub fn sse_psignw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsignwEmitter<A, B> {
        <Self as SsePsignwEmitter<A, B>>::sse_psignw(self, op0, op1);
    }
}
