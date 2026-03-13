use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CRC32` (CRC32). 
/// Starting with an initial value in the first operand (destination operand), accumulates a CRC32 (polynomial 11EDC6F41H) value for the second operand (source operand) and stores the result in the destination operand. The source operand can be a register or a memory location. The destination operand must be an r32 or r64 register. If the destination is an r64 register, then the 32-bit result is stored in the least significant double word and 00000000H is stored in the most significant double word of the r64 register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CRC32.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | Gpd, GpbLo |
/// | 2 | Gpd, Gpd   |
/// | 3 | Gpd, Gpq   |
/// | 4 | Gpd, Gpw   |
/// | 5 | Gpd, Mem   |
/// +---+------------+
/// ```
pub trait Crc32Emitter<A, B> {
    fn crc32(&mut self, op0: A, op1: B);
}

impl<'a> Crc32Emitter<Gpd, GpbLo> for Assembler<'a> {
    fn crc32(&mut self, op0: Gpd, op1: GpbLo) {
        self.emit(CRC32_8RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Crc32Emitter<Gpd, Mem> for Assembler<'a> {
    fn crc32(&mut self, op0: Gpd, op1: Mem) {
        self.emit(CRC32_8RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Crc32Emitter<Gpd, Gpw> for Assembler<'a> {
    fn crc32(&mut self, op0: Gpd, op1: Gpw) {
        self.emit(CRC32_16RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Crc32Emitter<Gpd, Gpd> for Assembler<'a> {
    fn crc32(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(CRC32_32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Crc32Emitter<Gpd, Gpq> for Assembler<'a> {
    fn crc32(&mut self, op0: Gpd, op1: Gpq) {
        self.emit(CRC32_64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PCMPESTRI`.
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
pub trait SsePcmpestriEmitter<A, B, C> {
    fn sse_pcmpestri(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePcmpestriEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_pcmpestri(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_PCMPESTRIRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePcmpestriEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pcmpestri(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PCMPESTRIRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PCMPESTRM`.
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
pub trait SsePcmpestrmEmitter<A, B, C> {
    fn sse_pcmpestrm(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePcmpestrmEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_pcmpestrm(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_PCMPESTRMRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePcmpestrmEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pcmpestrm(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PCMPESTRMRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PCMPISTRI`.
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
pub trait SsePcmpistriEmitter<A, B, C> {
    fn sse_pcmpistri(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePcmpistriEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_pcmpistri(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_PCMPISTRIRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePcmpistriEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pcmpistri(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PCMPISTRIRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PCMPISTRM`.
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
pub trait SsePcmpistrmEmitter<A, B, C> {
    fn sse_pcmpistrm(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePcmpistrmEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_pcmpistrm(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_PCMPISTRMRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePcmpistrmEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pcmpistrm(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PCMPISTRMRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `CRC32` (CRC32). 
    /// Starting with an initial value in the first operand (destination operand), accumulates a CRC32 (polynomial 11EDC6F41H) value for the second operand (source operand) and stores the result in the destination operand. The source operand can be a register or a memory location. The destination operand must be an r32 or r64 register. If the destination is an r64 register, then the 32-bit result is stored in the least significant double word and 00000000H is stored in the most significant double word of the r64 register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CRC32.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | Gpd, GpbLo |
    /// | 2 | Gpd, Gpd   |
    /// | 3 | Gpd, Gpq   |
    /// | 4 | Gpd, Gpw   |
    /// | 5 | Gpd, Mem   |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn crc32<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Crc32Emitter<A, B> {
        <Self as Crc32Emitter<A, B>>::crc32(self, op0, op1);
    }
    /// `SSE_PCMPESTRI`.
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
    pub fn sse_pcmpestri<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePcmpestriEmitter<A, B, C> {
        <Self as SsePcmpestriEmitter<A, B, C>>::sse_pcmpestri(self, op0, op1, op2);
    }
    /// `SSE_PCMPESTRM`.
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
    pub fn sse_pcmpestrm<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePcmpestrmEmitter<A, B, C> {
        <Self as SsePcmpestrmEmitter<A, B, C>>::sse_pcmpestrm(self, op0, op1, op2);
    }
    /// `SSE_PCMPISTRI`.
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
    pub fn sse_pcmpistri<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePcmpistriEmitter<A, B, C> {
        <Self as SsePcmpistriEmitter<A, B, C>>::sse_pcmpistri(self, op0, op1, op2);
    }
    /// `SSE_PCMPISTRM`.
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
    pub fn sse_pcmpistrm<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePcmpistrmEmitter<A, B, C> {
        <Self as SsePcmpistrmEmitter<A, B, C>>::sse_pcmpistrm(self, op0, op1, op2);
    }
}
