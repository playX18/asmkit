use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `SSE_BLENDPD` (BLENDPD). 
/// Double-precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [3:0] determine whether the corresponding double precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is ”1”, then the double precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDPD.html).
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
pub trait SseBlendpdEmitter<A, B, C> {
    fn sse_blendpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseBlendpdEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_blendpd(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_BLENDPDRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseBlendpdEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_blendpd(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_BLENDPDRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_BLENDPS` (BLENDPS). 
/// Packed single precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [7:0] determine whether the corresponding single precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is “1”, then the single precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDPS.html).
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
pub trait SseBlendpsEmitter<A, B, C> {
    fn sse_blendps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseBlendpsEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_blendps(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_BLENDPSRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseBlendpsEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_blendps(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_BLENDPSRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_BLENDVPD` (BLENDVPD). 
/// Conditionally copy each quadword data element of double precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each quadword element of the mask register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDVPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Xmm, Xmm, Xmm |
/// +---+---------------+
/// ```
pub trait SseBlendvpdEmitter<A, B, C> {
    fn sse_blendvpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseBlendvpdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn sse_blendvpd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(SSE_BLENDVPDRRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseBlendvpdEmitter<Xmm, Mem, Xmm> for Assembler<'a> {
    fn sse_blendvpd(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(SSE_BLENDVPDRMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_BLENDVPS` (BLENDVPS). 
/// Conditionally copy each dword data element of single precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each dword element of the mask register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDVPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Xmm |
/// | 2 | Xmm, Xmm, Xmm |
/// +---+---------------+
/// ```
pub trait SseBlendvpsEmitter<A, B, C> {
    fn sse_blendvps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseBlendvpsEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn sse_blendvps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(SSE_BLENDVPSRRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseBlendvpsEmitter<Xmm, Mem, Xmm> for Assembler<'a> {
    fn sse_blendvps(&mut self, op0: Xmm, op1: Mem, op2: Xmm) {
        self.emit(SSE_BLENDVPSRMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_DPPD` (DPPD). 
/// Conditionally multiplies the packed double precision floating-point values in the destination operand (first operand) with the packed double precision floating-point values in the source (second operand) depending on a mask extracted from bits [5:4] of the immediate operand (third operand). If a condition mask bit is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DPPD.html).
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
pub trait SseDppdEmitter<A, B, C> {
    fn sse_dppd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseDppdEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_dppd(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_DPPDRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseDppdEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_dppd(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_DPPDRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_DPPS` (DPPS). 
/// Conditionally multiplies the packed single precision floating-point values in the destination operand (first operand) with the packed single precision floats in the source (second operand) depending on a mask extracted from the high 4 bits of the immediate byte (third operand). If a condition mask bit in imm8[7:4] is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DPPS.html).
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
pub trait SseDppsEmitter<A, B, C> {
    fn sse_dpps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseDppsEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_dpps(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_DPPSRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseDppsEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_dpps(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_DPPSRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_EXTRACTPS` (EXTRACTPS). 
/// Extracts a single precision floating-point value from the source operand (second operand) at the 32-bit offset specified from imm8. Immediate bits higher than the most significant offset for the vector length are ignored.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/EXTRACTPS.html).
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
pub trait SseExtractpsEmitter<A, B, C> {
    fn sse_extractps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseExtractpsEmitter<Gpd, Xmm, Imm> for Assembler<'a> {
    fn sse_extractps(&mut self, op0: Gpd, op1: Xmm, op2: Imm) {
        self.emit(SSE_EXTRACTPSRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseExtractpsEmitter<Mem, Xmm, Imm> for Assembler<'a> {
    fn sse_extractps(&mut self, op0: Mem, op1: Xmm, op2: Imm) {
        self.emit(SSE_EXTRACTPSMRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_INSERTPS` (INSERTPS). 
/// (register source form)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/INSERTPS.html).
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
pub trait SseInsertpsEmitter<A, B, C> {
    fn sse_insertps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseInsertpsEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_insertps(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_INSERTPSRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseInsertpsEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_insertps(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_INSERTPSRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_MOVNTDQA` (MOVNTDQA). 
/// MOVNTDQA loads a double quadword from the source operand (second operand) to the destination operand (first operand) using a non-temporal hint if the memory source is WC (write combining) memory type. For WC memory type, the nontemporal hint may be implemented by loading a temporary internal buffer with the equivalent of an aligned cache line without filling this data to the cache. Any memory-type aliased lines in the cache will be snooped and flushed. Subsequent MOVNTDQA reads to unread portions of the WC cache line will receive data from the temporary internal buffer if data is available. The temporary internal buffer may be flushed by the processor at any time for any reason, for example
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTDQA.html).
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
pub trait SseMovntdqaEmitter<A, B> {
    fn sse_movntdqa(&mut self, op0: A, op1: B);
}

impl<'a> SseMovntdqaEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movntdqa(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVNTDQARM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MPSADBW` (MPSADBW). 
/// (V)MPSADBW calculates packed word results of sum-absolute-difference (SAD) of unsigned bytes from two blocks of 32-bit dword elements, using two select fields in the immediate byte to select the offsets of the two blocks within the first source operand and the second operand. Packed SAD word results are calculated within each 128-bit lane. Each SAD word result is calculated between a stationary block_2 (whose offset within the second source operand is selected by a two bit select control, multiplied by 32 bits) and a sliding block_1 at consecutive byte-granular position within the first source operand. The offset of the first 32-bit block of block_1 is selectable using a one bit select control, multiplied by 32 bits.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MPSADBW.html).
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
pub trait SseMpsadbwEmitter<A, B, C> {
    fn sse_mpsadbw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseMpsadbwEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_mpsadbw(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_MPSADBWRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseMpsadbwEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_mpsadbw(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_MPSADBWRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PACKUSDW` (PACKUSDW). 
/// Converts packed signed doubleword integers in the first and second source operands into packed unsigned word integers using unsigned saturation to handle overflow conditions. If the signed doubleword value is beyond the range of an unsigned word (that is, greater than FFFFH or less than 0000H), the saturated unsigned word integer value of FFFFH or 0000H, respectively, is stored in the destination.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PACKUSDW.html).
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
pub trait SsePackusdwEmitter<A, B> {
    fn sse_packusdw(&mut self, op0: A, op1: B);
}

impl<'a> SsePackusdwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_packusdw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PACKUSDWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePackusdwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_packusdw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PACKUSDWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PBLENDVB` (PBLENDVB). 
/// Conditionally copies byte elements from the source operand (second operand) to the destination operand (first operand) depending on mask bits defined in the implicit third register argument, XMM0. The mask bits are the most significant bit in each byte element of the XMM0 register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PBLENDVB.html).
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
pub trait SsePblendvbEmitter<A, B> {
    fn sse_pblendvb(&mut self, op0: A, op1: B);
}

impl<'a> SsePblendvbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pblendvb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PBLENDVBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePblendvbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pblendvb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PBLENDVBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PBLENDW` (PBLENDW). 
/// Words from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source. If a bit in the mask, corresponding to a word, is “1", then the word is copied, else the word element in the destination operand is unchanged.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PBLENDW.html).
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
pub trait SsePblendwEmitter<A, B, C> {
    fn sse_pblendw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePblendwEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_pblendw(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_PBLENDWRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePblendwEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pblendw(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PBLENDWRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PCMPEQQ` (PCMPEQQ). 
/// Performs an SIMD compare for equality of the packed quadwords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQQ.html).
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
pub trait SsePcmpeqqEmitter<A, B> {
    fn sse_pcmpeqq(&mut self, op0: A, op1: B);
}

impl<'a> SsePcmpeqqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pcmpeqq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PCMPEQQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePcmpeqqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pcmpeqq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PCMPEQQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PCMPGTQ` (PCMPGTQ). 
/// Performs an SIMD signed compare for the packed quadwords in the destination operand (first operand) and the source operand (second operand). If the data element in the first (destination) operand is greater than the corresponding element in the second (source) operand, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTQ.html).
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
pub trait SsePcmpgtqEmitter<A, B> {
    fn sse_pcmpgtq(&mut self, op0: A, op1: B);
}

impl<'a> SsePcmpgtqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pcmpgtq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PCMPGTQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePcmpgtqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pcmpgtq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PCMPGTQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PEXTRB` (PEXTRB). 
/// Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html).
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
pub trait SsePextrbEmitter<A, B, C> {
    fn sse_pextrb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePextrbEmitter<Mem, Xmm, Imm> for Assembler<'a> {
    fn sse_pextrb(&mut self, op0: Mem, op1: Xmm, op2: Imm) {
        self.emit(SSE_PEXTRBMRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePextrbEmitter<Gpd, Xmm, Imm> for Assembler<'a> {
    fn sse_pextrb(&mut self, op0: Gpd, op1: Xmm, op2: Imm) {
        self.emit(SSE_PEXTRBRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PEXTRD` (PEXTRD). 
/// Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html).
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
pub trait SsePextrdEmitter<A, B, C> {
    fn sse_pextrd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePextrdEmitter<Gpd, Xmm, Imm> for Assembler<'a> {
    fn sse_pextrd(&mut self, op0: Gpd, op1: Xmm, op2: Imm) {
        self.emit(SSE_PEXTRDRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePextrdEmitter<Mem, Xmm, Imm> for Assembler<'a> {
    fn sse_pextrd(&mut self, op0: Mem, op1: Xmm, op2: Imm) {
        self.emit(SSE_PEXTRDMRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PEXTRQ` (PEXTRQ). 
/// Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html).
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
pub trait SsePextrqEmitter<A, B, C> {
    fn sse_pextrq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePextrqEmitter<Gpd, Xmm, Imm> for Assembler<'a> {
    fn sse_pextrq(&mut self, op0: Gpd, op1: Xmm, op2: Imm) {
        self.emit(SSE_PEXTRQRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePextrqEmitter<Mem, Xmm, Imm> for Assembler<'a> {
    fn sse_pextrq(&mut self, op0: Mem, op1: Xmm, op2: Imm) {
        self.emit(SSE_PEXTRQMRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PHMINPOSUW` (PHMINPOSUW). 
/// Determine the minimum unsigned word value in the source operand (second operand) and place the unsigned word in the low word (bits 0-15) of the destination operand (first operand). The word index of the minimum value is stored in bits 16-18 of the destination operand. The remaining upper bits of the destination are set to zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHMINPOSUW.html).
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
pub trait SsePhminposuwEmitter<A, B> {
    fn sse_phminposuw(&mut self, op0: A, op1: B);
}

impl<'a> SsePhminposuwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_phminposuw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PHMINPOSUWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePhminposuwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phminposuw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PHMINPOSUWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PINSRB` (PINSRB). 
/// Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Gpd, Imm |
/// | 2 | Xmm, Mem, Imm |
/// +---+---------------+
/// ```
pub trait SsePinsrbEmitter<A, B, C> {
    fn sse_pinsrb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePinsrbEmitter<Xmm, Gpd, Imm> for Assembler<'a> {
    fn sse_pinsrb(&mut self, op0: Xmm, op1: Gpd, op2: Imm) {
        self.emit(SSE_PINSRBRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePinsrbEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pinsrb(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PINSRBRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PINSRD` (PINSRD). 
/// Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Gpd, Imm |
/// | 2 | Xmm, Mem, Imm |
/// +---+---------------+
/// ```
pub trait SsePinsrdEmitter<A, B, C> {
    fn sse_pinsrd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePinsrdEmitter<Xmm, Gpd, Imm> for Assembler<'a> {
    fn sse_pinsrd(&mut self, op0: Xmm, op1: Gpd, op2: Imm) {
        self.emit(SSE_PINSRDRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePinsrdEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pinsrd(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PINSRDRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PINSRQ` (PINSRQ). 
/// Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Gpd, Imm |
/// | 2 | Xmm, Mem, Imm |
/// +---+---------------+
/// ```
pub trait SsePinsrqEmitter<A, B, C> {
    fn sse_pinsrq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePinsrqEmitter<Xmm, Gpd, Imm> for Assembler<'a> {
    fn sse_pinsrq(&mut self, op0: Xmm, op1: Gpd, op2: Imm) {
        self.emit(SSE_PINSRQRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePinsrqEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pinsrq(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PINSRQRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PMAXSB` (PMAXSB). 
/// Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html).
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
pub trait SsePmaxsbEmitter<A, B> {
    fn sse_pmaxsb(&mut self, op0: A, op1: B);
}

impl<'a> SsePmaxsbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmaxsb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMAXSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmaxsbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmaxsb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMAXSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMAXSD` (PMAXSD). 
/// Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html).
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
pub trait SsePmaxsdEmitter<A, B> {
    fn sse_pmaxsd(&mut self, op0: A, op1: B);
}

impl<'a> SsePmaxsdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmaxsd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMAXSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmaxsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmaxsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMAXSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMAXUD` (PMAXUD). 
/// Performs a SIMD compare of the packed unsigned dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMAXUD%3APMAXUQ.html).
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
pub trait SsePmaxudEmitter<A, B> {
    fn sse_pmaxud(&mut self, op0: A, op1: B);
}

impl<'a> SsePmaxudEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmaxud(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMAXUDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmaxudEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmaxud(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMAXUDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMAXUW` (PMAXUW). 
/// Performs a SIMD compare of the packed unsigned byte, word integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMAXUB%3APMAXUW.html).
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
pub trait SsePmaxuwEmitter<A, B> {
    fn sse_pmaxuw(&mut self, op0: A, op1: B);
}

impl<'a> SsePmaxuwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmaxuw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMAXUWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmaxuwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmaxuw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMAXUWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMINSB` (PMINSB). 
/// Performs a SIMD compare of the packed signed byte, word, or dword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMINSB%3APMINSW.html).
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
pub trait SsePminsbEmitter<A, B> {
    fn sse_pminsb(&mut self, op0: A, op1: B);
}

impl<'a> SsePminsbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pminsb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMINSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePminsbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pminsb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMINSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMINSD` (PMINSD). 
/// Performs a SIMD compare of the packed signed dword or qword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMINSD%3APMINSQ.html).
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
pub trait SsePminsdEmitter<A, B> {
    fn sse_pminsd(&mut self, op0: A, op1: B);
}

impl<'a> SsePminsdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pminsd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMINSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePminsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pminsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMINSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMINUD` (PMINUD). 
/// Performs a SIMD compare of the packed unsigned dword/qword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMINUD%3APMINUQ.html).
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
pub trait SsePminudEmitter<A, B> {
    fn sse_pminud(&mut self, op0: A, op1: B);
}

impl<'a> SsePminudEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pminud(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMINUDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePminudEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pminud(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMINUDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMINUW` (PMINUW). 
/// Performs a SIMD compare of the packed unsigned byte or word integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMINUB%3APMINUW.html).
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
pub trait SsePminuwEmitter<A, B> {
    fn sse_pminuw(&mut self, op0: A, op1: B);
}

impl<'a> SsePminuwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pminuw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMINUWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePminuwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pminuw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMINUWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVSXBD` (PMOVSXBD). 
/// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
pub trait SsePmovsxbdEmitter<A, B> {
    fn sse_pmovsxbd(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovsxbdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovsxbd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVSXBDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovsxbdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovsxbd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVSXBDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVSXBQ` (PMOVSXBQ). 
/// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
pub trait SsePmovsxbqEmitter<A, B> {
    fn sse_pmovsxbq(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovsxbqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovsxbq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVSXBQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovsxbqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovsxbq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVSXBQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVSXBW` (PMOVSXBW). 
/// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
pub trait SsePmovsxbwEmitter<A, B> {
    fn sse_pmovsxbw(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovsxbwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovsxbw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVSXBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovsxbwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovsxbw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVSXBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVSXDQ` (PMOVSXDQ). 
/// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
pub trait SsePmovsxdqEmitter<A, B> {
    fn sse_pmovsxdq(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovsxdqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovsxdq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVSXDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovsxdqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovsxdq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVSXDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVSXWD` (PMOVSXWD). 
/// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
pub trait SsePmovsxwdEmitter<A, B> {
    fn sse_pmovsxwd(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovsxwdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovsxwd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVSXWDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovsxwdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovsxwd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVSXWDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVSXWQ` (PMOVSXWQ). 
/// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
pub trait SsePmovsxwqEmitter<A, B> {
    fn sse_pmovsxwq(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovsxwqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovsxwq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVSXWQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovsxwqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovsxwq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVSXWQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVZXBD` (PMOVZXBD). 
/// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
pub trait SsePmovzxbdEmitter<A, B> {
    fn sse_pmovzxbd(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovzxbdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovzxbd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVZXBDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovzxbdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovzxbd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVZXBDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVZXBQ` (PMOVZXBQ). 
/// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
pub trait SsePmovzxbqEmitter<A, B> {
    fn sse_pmovzxbq(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovzxbqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovzxbq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVZXBQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovzxbqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovzxbq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVZXBQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVZXBW` (PMOVZXBW). 
/// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
pub trait SsePmovzxbwEmitter<A, B> {
    fn sse_pmovzxbw(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovzxbwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovzxbw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVZXBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovzxbwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovzxbw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVZXBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVZXDQ` (PMOVZXDQ). 
/// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
pub trait SsePmovzxdqEmitter<A, B> {
    fn sse_pmovzxdq(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovzxdqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovzxdq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVZXDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovzxdqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovzxdq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVZXDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVZXWD` (PMOVZXWD). 
/// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
pub trait SsePmovzxwdEmitter<A, B> {
    fn sse_pmovzxwd(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovzxwdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovzxwd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVZXWDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovzxwdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovzxwd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVZXWDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVZXWQ` (PMOVZXWQ). 
/// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
pub trait SsePmovzxwqEmitter<A, B> {
    fn sse_pmovzxwq(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovzxwqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmovzxwq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMOVZXWQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmovzxwqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmovzxwq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMOVZXWQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMULDQ` (PMULDQ). 
/// Multiplies packed signed doubleword integers in the even-numbered (zero-based reference) elements of the first source operand with the packed signed doubleword integers in the corresponding elements of the second source operand and stores packed signed quadword results in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULDQ.html).
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
pub trait SsePmuldqEmitter<A, B> {
    fn sse_pmuldq(&mut self, op0: A, op1: B);
}

impl<'a> SsePmuldqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmuldq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMULDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmuldqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmuldq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMULDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMULLD` (PMULLD). 
/// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
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
pub trait SsePmulldEmitter<A, B> {
    fn sse_pmulld(&mut self, op0: A, op1: B);
}

impl<'a> SsePmulldEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmulld(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMULLDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmulldEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmulld(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMULLDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PTEST` (PTEST). 
/// PTEST and VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND of the first source operand (first operand) and the second source operand (second operand). VPTEST sets the CF flag if all bits in the result are 0 of the bitwise AND of the second source operand (second operand) and the logical NOT of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PTEST.html).
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
pub trait SsePtestEmitter<A, B> {
    fn sse_ptest(&mut self, op0: A, op1: B);
}

impl<'a> SsePtestEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_ptest(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PTESTRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePtestEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_ptest(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PTESTRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_ROUNDPD` (ROUNDPD). 
/// Round the 2 double precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a double precision floating-point value.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDPD.html).
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
pub trait SseRoundpdEmitter<A, B, C> {
    fn sse_roundpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseRoundpdEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_roundpd(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_ROUNDPDRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseRoundpdEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_roundpd(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_ROUNDPDRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_ROUNDPS` (ROUNDPS). 
/// Round the 4 single precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a single precision floating-point value.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDPS.html).
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
pub trait SseRoundpsEmitter<A, B, C> {
    fn sse_roundps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseRoundpsEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_roundps(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_ROUNDPSRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseRoundpsEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_roundps(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_ROUNDPSRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_ROUNDSD` (ROUNDSD). 
/// Round the double precision floating-point value in the lower qword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a double precision floating-point input to an integer value and returns the integer result as a double precision floating-point value in the lowest position. The upper double precision floating-point value in the destination is retained.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDSD.html).
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
pub trait SseRoundsdEmitter<A, B, C> {
    fn sse_roundsd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseRoundsdEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_roundsd(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_ROUNDSDRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseRoundsdEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_roundsd(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_ROUNDSDRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_ROUNDSS` (ROUNDSS). 
/// Round the single precision floating-point value in the lowest dword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a single precision floating-point input to an integer value and returns the result as a single precision floating-point value in the lowest position. The upper three single precision floating-point values in the destination are retained.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDSS.html).
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
pub trait SseRoundssEmitter<A, B, C> {
    fn sse_roundss(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseRoundssEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_roundss(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_ROUNDSSRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseRoundssEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_roundss(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_ROUNDSSRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `SSE_BLENDPD` (BLENDPD). 
    /// Double-precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [3:0] determine whether the corresponding double precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is ”1”, then the double precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDPD.html).
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
    pub fn sse_blendpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseBlendpdEmitter<A, B, C> {
        <Self as SseBlendpdEmitter<A, B, C>>::sse_blendpd(self, op0, op1, op2);
    }
    /// `SSE_BLENDPS` (BLENDPS). 
    /// Packed single precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [7:0] determine whether the corresponding single precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is “1”, then the single precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDPS.html).
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
    pub fn sse_blendps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseBlendpsEmitter<A, B, C> {
        <Self as SseBlendpsEmitter<A, B, C>>::sse_blendps(self, op0, op1, op2);
    }
    /// `SSE_BLENDVPD` (BLENDVPD). 
    /// Conditionally copy each quadword data element of double precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each quadword element of the mask register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDVPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Xmm, Xmm, Xmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn sse_blendvpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseBlendvpdEmitter<A, B, C> {
        <Self as SseBlendvpdEmitter<A, B, C>>::sse_blendvpd(self, op0, op1, op2);
    }
    /// `SSE_BLENDVPS` (BLENDVPS). 
    /// Conditionally copy each dword data element of single precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each dword element of the mask register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDVPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Xmm |
    /// | 2 | Xmm, Xmm, Xmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn sse_blendvps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseBlendvpsEmitter<A, B, C> {
        <Self as SseBlendvpsEmitter<A, B, C>>::sse_blendvps(self, op0, op1, op2);
    }
    /// `SSE_DPPD` (DPPD). 
    /// Conditionally multiplies the packed double precision floating-point values in the destination operand (first operand) with the packed double precision floating-point values in the source (second operand) depending on a mask extracted from bits [5:4] of the immediate operand (third operand). If a condition mask bit is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DPPD.html).
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
    pub fn sse_dppd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseDppdEmitter<A, B, C> {
        <Self as SseDppdEmitter<A, B, C>>::sse_dppd(self, op0, op1, op2);
    }
    /// `SSE_DPPS` (DPPS). 
    /// Conditionally multiplies the packed single precision floating-point values in the destination operand (first operand) with the packed single precision floats in the source (second operand) depending on a mask extracted from the high 4 bits of the immediate byte (third operand). If a condition mask bit in imm8[7:4] is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DPPS.html).
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
    pub fn sse_dpps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseDppsEmitter<A, B, C> {
        <Self as SseDppsEmitter<A, B, C>>::sse_dpps(self, op0, op1, op2);
    }
    /// `SSE_EXTRACTPS` (EXTRACTPS). 
    /// Extracts a single precision floating-point value from the source operand (second operand) at the 32-bit offset specified from imm8. Immediate bits higher than the most significant offset for the vector length are ignored.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/EXTRACTPS.html).
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
    pub fn sse_extractps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseExtractpsEmitter<A, B, C> {
        <Self as SseExtractpsEmitter<A, B, C>>::sse_extractps(self, op0, op1, op2);
    }
    /// `SSE_INSERTPS` (INSERTPS). 
    /// (register source form)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/INSERTPS.html).
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
    pub fn sse_insertps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseInsertpsEmitter<A, B, C> {
        <Self as SseInsertpsEmitter<A, B, C>>::sse_insertps(self, op0, op1, op2);
    }
    /// `SSE_MOVNTDQA` (MOVNTDQA). 
    /// MOVNTDQA loads a double quadword from the source operand (second operand) to the destination operand (first operand) using a non-temporal hint if the memory source is WC (write combining) memory type. For WC memory type, the nontemporal hint may be implemented by loading a temporary internal buffer with the equivalent of an aligned cache line without filling this data to the cache. Any memory-type aliased lines in the cache will be snooped and flushed. Subsequent MOVNTDQA reads to unread portions of the WC cache line will receive data from the temporary internal buffer if data is available. The temporary internal buffer may be flushed by the processor at any time for any reason, for example
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTDQA.html).
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
    pub fn sse_movntdqa<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovntdqaEmitter<A, B> {
        <Self as SseMovntdqaEmitter<A, B>>::sse_movntdqa(self, op0, op1);
    }
    /// `SSE_MPSADBW` (MPSADBW). 
    /// (V)MPSADBW calculates packed word results of sum-absolute-difference (SAD) of unsigned bytes from two blocks of 32-bit dword elements, using two select fields in the immediate byte to select the offsets of the two blocks within the first source operand and the second operand. Packed SAD word results are calculated within each 128-bit lane. Each SAD word result is calculated between a stationary block_2 (whose offset within the second source operand is selected by a two bit select control, multiplied by 32 bits) and a sliding block_1 at consecutive byte-granular position within the first source operand. The offset of the first 32-bit block of block_1 is selectable using a one bit select control, multiplied by 32 bits.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MPSADBW.html).
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
    pub fn sse_mpsadbw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseMpsadbwEmitter<A, B, C> {
        <Self as SseMpsadbwEmitter<A, B, C>>::sse_mpsadbw(self, op0, op1, op2);
    }
    /// `SSE_PACKUSDW` (PACKUSDW). 
    /// Converts packed signed doubleword integers in the first and second source operands into packed unsigned word integers using unsigned saturation to handle overflow conditions. If the signed doubleword value is beyond the range of an unsigned word (that is, greater than FFFFH or less than 0000H), the saturated unsigned word integer value of FFFFH or 0000H, respectively, is stored in the destination.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PACKUSDW.html).
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
    pub fn sse_packusdw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePackusdwEmitter<A, B> {
        <Self as SsePackusdwEmitter<A, B>>::sse_packusdw(self, op0, op1);
    }
    /// `SSE_PBLENDVB` (PBLENDVB). 
    /// Conditionally copies byte elements from the source operand (second operand) to the destination operand (first operand) depending on mask bits defined in the implicit third register argument, XMM0. The mask bits are the most significant bit in each byte element of the XMM0 register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PBLENDVB.html).
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
    pub fn sse_pblendvb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePblendvbEmitter<A, B> {
        <Self as SsePblendvbEmitter<A, B>>::sse_pblendvb(self, op0, op1);
    }
    /// `SSE_PBLENDW` (PBLENDW). 
    /// Words from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source. If a bit in the mask, corresponding to a word, is “1", then the word is copied, else the word element in the destination operand is unchanged.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PBLENDW.html).
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
    pub fn sse_pblendw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePblendwEmitter<A, B, C> {
        <Self as SsePblendwEmitter<A, B, C>>::sse_pblendw(self, op0, op1, op2);
    }
    /// `SSE_PCMPEQQ` (PCMPEQQ). 
    /// Performs an SIMD compare for equality of the packed quadwords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQQ.html).
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
    pub fn sse_pcmpeqq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePcmpeqqEmitter<A, B> {
        <Self as SsePcmpeqqEmitter<A, B>>::sse_pcmpeqq(self, op0, op1);
    }
    /// `SSE_PCMPGTQ` (PCMPGTQ). 
    /// Performs an SIMD signed compare for the packed quadwords in the destination operand (first operand) and the source operand (second operand). If the data element in the first (destination) operand is greater than the corresponding element in the second (source) operand, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTQ.html).
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
    pub fn sse_pcmpgtq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePcmpgtqEmitter<A, B> {
        <Self as SsePcmpgtqEmitter<A, B>>::sse_pcmpgtq(self, op0, op1);
    }
    /// `SSE_PEXTRB` (PEXTRB). 
    /// Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html).
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
    pub fn sse_pextrb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePextrbEmitter<A, B, C> {
        <Self as SsePextrbEmitter<A, B, C>>::sse_pextrb(self, op0, op1, op2);
    }
    /// `SSE_PEXTRD` (PEXTRD). 
    /// Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html).
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
    pub fn sse_pextrd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePextrdEmitter<A, B, C> {
        <Self as SsePextrdEmitter<A, B, C>>::sse_pextrd(self, op0, op1, op2);
    }
    /// `SSE_PEXTRQ` (PEXTRQ). 
    /// Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html).
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
    pub fn sse_pextrq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePextrqEmitter<A, B, C> {
        <Self as SsePextrqEmitter<A, B, C>>::sse_pextrq(self, op0, op1, op2);
    }
    /// `SSE_PHMINPOSUW` (PHMINPOSUW). 
    /// Determine the minimum unsigned word value in the source operand (second operand) and place the unsigned word in the low word (bits 0-15) of the destination operand (first operand). The word index of the minimum value is stored in bits 16-18 of the destination operand. The remaining upper bits of the destination are set to zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHMINPOSUW.html).
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
    pub fn sse_phminposuw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePhminposuwEmitter<A, B> {
        <Self as SsePhminposuwEmitter<A, B>>::sse_phminposuw(self, op0, op1);
    }
    /// `SSE_PINSRB` (PINSRB). 
    /// Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Gpd, Imm |
    /// | 2 | Xmm, Mem, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn sse_pinsrb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePinsrbEmitter<A, B, C> {
        <Self as SsePinsrbEmitter<A, B, C>>::sse_pinsrb(self, op0, op1, op2);
    }
    /// `SSE_PINSRD` (PINSRD). 
    /// Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Gpd, Imm |
    /// | 2 | Xmm, Mem, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn sse_pinsrd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePinsrdEmitter<A, B, C> {
        <Self as SsePinsrdEmitter<A, B, C>>::sse_pinsrd(self, op0, op1, op2);
    }
    /// `SSE_PINSRQ` (PINSRQ). 
    /// Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Gpd, Imm |
    /// | 2 | Xmm, Mem, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn sse_pinsrq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePinsrqEmitter<A, B, C> {
        <Self as SsePinsrqEmitter<A, B, C>>::sse_pinsrq(self, op0, op1, op2);
    }
    /// `SSE_PMAXSB` (PMAXSB). 
    /// Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html).
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
    pub fn sse_pmaxsb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmaxsbEmitter<A, B> {
        <Self as SsePmaxsbEmitter<A, B>>::sse_pmaxsb(self, op0, op1);
    }
    /// `SSE_PMAXSD` (PMAXSD). 
    /// Performs a SIMD compare of the packed signed byte, word, dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMAXSB%3APMAXSW%3APMAXSD%3APMAXSQ.html).
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
    pub fn sse_pmaxsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmaxsdEmitter<A, B> {
        <Self as SsePmaxsdEmitter<A, B>>::sse_pmaxsd(self, op0, op1);
    }
    /// `SSE_PMAXUD` (PMAXUD). 
    /// Performs a SIMD compare of the packed unsigned dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMAXUD%3APMAXUQ.html).
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
    pub fn sse_pmaxud<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmaxudEmitter<A, B> {
        <Self as SsePmaxudEmitter<A, B>>::sse_pmaxud(self, op0, op1);
    }
    /// `SSE_PMAXUW` (PMAXUW). 
    /// Performs a SIMD compare of the packed unsigned byte, word integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMAXUB%3APMAXUW.html).
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
    pub fn sse_pmaxuw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmaxuwEmitter<A, B> {
        <Self as SsePmaxuwEmitter<A, B>>::sse_pmaxuw(self, op0, op1);
    }
    /// `SSE_PMINSB` (PMINSB). 
    /// Performs a SIMD compare of the packed signed byte, word, or dword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMINSB%3APMINSW.html).
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
    pub fn sse_pminsb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePminsbEmitter<A, B> {
        <Self as SsePminsbEmitter<A, B>>::sse_pminsb(self, op0, op1);
    }
    /// `SSE_PMINSD` (PMINSD). 
    /// Performs a SIMD compare of the packed signed dword or qword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMINSD%3APMINSQ.html).
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
    pub fn sse_pminsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePminsdEmitter<A, B> {
        <Self as SsePminsdEmitter<A, B>>::sse_pminsd(self, op0, op1);
    }
    /// `SSE_PMINUD` (PMINUD). 
    /// Performs a SIMD compare of the packed unsigned dword/qword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMINUD%3APMINUQ.html).
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
    pub fn sse_pminud<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePminudEmitter<A, B> {
        <Self as SsePminudEmitter<A, B>>::sse_pminud(self, op0, op1);
    }
    /// `SSE_PMINUW` (PMINUW). 
    /// Performs a SIMD compare of the packed unsigned byte or word integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMINUB%3APMINUW.html).
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
    pub fn sse_pminuw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePminuwEmitter<A, B> {
        <Self as SsePminuwEmitter<A, B>>::sse_pminuw(self, op0, op1);
    }
    /// `SSE_PMOVSXBD` (PMOVSXBD). 
    /// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
    pub fn sse_pmovsxbd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovsxbdEmitter<A, B> {
        <Self as SsePmovsxbdEmitter<A, B>>::sse_pmovsxbd(self, op0, op1);
    }
    /// `SSE_PMOVSXBQ` (PMOVSXBQ). 
    /// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
    pub fn sse_pmovsxbq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovsxbqEmitter<A, B> {
        <Self as SsePmovsxbqEmitter<A, B>>::sse_pmovsxbq(self, op0, op1);
    }
    /// `SSE_PMOVSXBW` (PMOVSXBW). 
    /// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
    pub fn sse_pmovsxbw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovsxbwEmitter<A, B> {
        <Self as SsePmovsxbwEmitter<A, B>>::sse_pmovsxbw(self, op0, op1);
    }
    /// `SSE_PMOVSXDQ` (PMOVSXDQ). 
    /// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
    pub fn sse_pmovsxdq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovsxdqEmitter<A, B> {
        <Self as SsePmovsxdqEmitter<A, B>>::sse_pmovsxdq(self, op0, op1);
    }
    /// `SSE_PMOVSXWD` (PMOVSXWD). 
    /// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
    pub fn sse_pmovsxwd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovsxwdEmitter<A, B> {
        <Self as SsePmovsxwdEmitter<A, B>>::sse_pmovsxwd(self, op0, op1);
    }
    /// `SSE_PMOVSXWQ` (PMOVSXWQ). 
    /// Legacy and VEX encoded versions: Packed byte, word, or dword integers in the low bytes of the source operand (second operand) are sign extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVSX.html).
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
    pub fn sse_pmovsxwq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovsxwqEmitter<A, B> {
        <Self as SsePmovsxwqEmitter<A, B>>::sse_pmovsxwq(self, op0, op1);
    }
    /// `SSE_PMOVZXBD` (PMOVZXBD). 
    /// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
    pub fn sse_pmovzxbd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovzxbdEmitter<A, B> {
        <Self as SsePmovzxbdEmitter<A, B>>::sse_pmovzxbd(self, op0, op1);
    }
    /// `SSE_PMOVZXBQ` (PMOVZXBQ). 
    /// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
    pub fn sse_pmovzxbq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovzxbqEmitter<A, B> {
        <Self as SsePmovzxbqEmitter<A, B>>::sse_pmovzxbq(self, op0, op1);
    }
    /// `SSE_PMOVZXBW` (PMOVZXBW). 
    /// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
    pub fn sse_pmovzxbw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovzxbwEmitter<A, B> {
        <Self as SsePmovzxbwEmitter<A, B>>::sse_pmovzxbw(self, op0, op1);
    }
    /// `SSE_PMOVZXDQ` (PMOVZXDQ). 
    /// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
    pub fn sse_pmovzxdq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovzxdqEmitter<A, B> {
        <Self as SsePmovzxdqEmitter<A, B>>::sse_pmovzxdq(self, op0, op1);
    }
    /// `SSE_PMOVZXWD` (PMOVZXWD). 
    /// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
    pub fn sse_pmovzxwd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovzxwdEmitter<A, B> {
        <Self as SsePmovzxwdEmitter<A, B>>::sse_pmovzxwd(self, op0, op1);
    }
    /// `SSE_PMOVZXWQ` (PMOVZXWQ). 
    /// Legacy, VEX, and EVEX encoded versions: Packed byte, word, or dword integers starting from the low bytes of the source operand (second operand) are zero extended to word, dword, or quadword integers and stored in packed signed bytes the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVZX.html).
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
    pub fn sse_pmovzxwq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovzxwqEmitter<A, B> {
        <Self as SsePmovzxwqEmitter<A, B>>::sse_pmovzxwq(self, op0, op1);
    }
    /// `SSE_PMULDQ` (PMULDQ). 
    /// Multiplies packed signed doubleword integers in the even-numbered (zero-based reference) elements of the first source operand with the packed signed doubleword integers in the corresponding elements of the second source operand and stores packed signed quadword results in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULDQ.html).
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
    pub fn sse_pmuldq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmuldqEmitter<A, B> {
        <Self as SsePmuldqEmitter<A, B>>::sse_pmuldq(self, op0, op1);
    }
    /// `SSE_PMULLD` (PMULLD). 
    /// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
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
    pub fn sse_pmulld<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmulldEmitter<A, B> {
        <Self as SsePmulldEmitter<A, B>>::sse_pmulld(self, op0, op1);
    }
    /// `SSE_PTEST` (PTEST). 
    /// PTEST and VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND of the first source operand (first operand) and the second source operand (second operand). VPTEST sets the CF flag if all bits in the result are 0 of the bitwise AND of the second source operand (second operand) and the logical NOT of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PTEST.html).
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
    pub fn sse_ptest<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePtestEmitter<A, B> {
        <Self as SsePtestEmitter<A, B>>::sse_ptest(self, op0, op1);
    }
    /// `SSE_ROUNDPD` (ROUNDPD). 
    /// Round the 2 double precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a double precision floating-point value.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDPD.html).
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
    pub fn sse_roundpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseRoundpdEmitter<A, B, C> {
        <Self as SseRoundpdEmitter<A, B, C>>::sse_roundpd(self, op0, op1, op2);
    }
    /// `SSE_ROUNDPS` (ROUNDPS). 
    /// Round the 4 single precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a single precision floating-point value.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDPS.html).
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
    pub fn sse_roundps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseRoundpsEmitter<A, B, C> {
        <Self as SseRoundpsEmitter<A, B, C>>::sse_roundps(self, op0, op1, op2);
    }
    /// `SSE_ROUNDSD` (ROUNDSD). 
    /// Round the double precision floating-point value in the lower qword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a double precision floating-point input to an integer value and returns the integer result as a double precision floating-point value in the lowest position. The upper double precision floating-point value in the destination is retained.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDSD.html).
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
    pub fn sse_roundsd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseRoundsdEmitter<A, B, C> {
        <Self as SseRoundsdEmitter<A, B, C>>::sse_roundsd(self, op0, op1, op2);
    }
    /// `SSE_ROUNDSS` (ROUNDSS). 
    /// Round the single precision floating-point value in the lowest dword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a single precision floating-point input to an integer value and returns the result as a single precision floating-point value in the lowest position. The upper three single precision floating-point values in the destination are retained.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDSS.html).
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
    pub fn sse_roundss<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseRoundssEmitter<A, B, C> {
        <Self as SseRoundssEmitter<A, B, C>>::sse_roundss(self, op0, op1, op2);
    }
}
