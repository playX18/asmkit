use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `LDMXCSR` (LDMXCSR). 
/// Loads the source operand into the MXCSR control/status register. The source operand is a 32-bit memory location. See “MXCSR Control and Status Register” in Chapter 10, of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for a description of the MXCSR register and its contents.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/LDMXCSR.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait LdmxcsrEmitter<A> {
    fn ldmxcsr(&mut self, op0: A);
}

impl<'a> LdmxcsrEmitter<Mem> for Assembler<'a> {
    fn ldmxcsr(&mut self, op0: Mem) {
        self.emit(LDMXCSRM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `MMX_MASKMOVQ` (MASKMOVQ). 
/// Stores selected bytes from the source operand (first operand) into a 64-bit memory location. The mask operand (second operand) selects which bytes from the source operand are written to memory. The source and mask operands are MMX technology registers. The memory location specified by the effective address in the DI/EDI/RDI register (the default segment register is DS, but this may be overridden with a segment-override prefix). The memory location does not need to be aligned on a natural boundary. (The size of the store address depends on the address-size attribute.)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MASKMOVQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxMaskmovqEmitter<A, B> {
    fn mmx_maskmovq(&mut self, op0: A, op1: B);
}

impl<'a> MmxMaskmovqEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_maskmovq(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_MASKMOVQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_MOVDQ2Q`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Xmm  |
/// +---+----------+
/// ```
pub trait MmxMovdq2qEmitter<A, B> {
    fn mmx_movdq2q(&mut self, op0: A, op1: B);
}

impl<'a> MmxMovdq2qEmitter<Mm, Xmm> for Assembler<'a> {
    fn mmx_movdq2q(&mut self, op0: Mm, op1: Xmm) {
        self.emit(MMX_MOVDQ2QRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_MOVNTQ` (MOVNTQ). 
/// Moves the quadword in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to minimize cache pollution during the write to memory. The source operand is an MMX technology register, which is assumed to contain packed integer data (packed bytes, words, or doublewords). The destination operand is a 64-bit memory location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Mm  |
/// +---+----------+
/// ```
pub trait MmxMovntqEmitter<A, B> {
    fn mmx_movntq(&mut self, op0: A, op1: B);
}

impl<'a> MmxMovntqEmitter<Mem, Mm> for Assembler<'a> {
    fn mmx_movntq(&mut self, op0: Mem, op1: Mm) {
        self.emit(MMX_MOVNTQMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_MOVQ2DQ` (MOVQ2DQ). 
/// Moves the quadword from the source operand (second operand) to the low quadword of the destination operand (first operand). The source operand is an MMX technology register and the destination operand is an XMM register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVQ2DQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mm  |
/// +---+----------+
/// ```
pub trait MmxMovq2dqEmitter<A, B> {
    fn mmx_movq2dq(&mut self, op0: A, op1: B);
}

impl<'a> MmxMovq2dqEmitter<Xmm, Mm> for Assembler<'a> {
    fn mmx_movq2dq(&mut self, op0: Xmm, op1: Mm) {
        self.emit(MMX_MOVQ2DQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PAVGB` (PAVGB). 
/// Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html).
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
pub trait MmxPavgbEmitter<A, B> {
    fn mmx_pavgb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPavgbEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pavgb(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PAVGBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPavgbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pavgb(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PAVGBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PAVGW` (PAVGW). 
/// Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html).
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
pub trait MmxPavgwEmitter<A, B> {
    fn mmx_pavgw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPavgwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pavgw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PAVGWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPavgwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pavgw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PAVGWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PEXTRW` (PEXTRW). 
/// Copies the word in the source operand (second operand) specified by the count operand (third operand) to the destination operand (first operand). The source operand can be an MMX technology register or an XMM register. The destination operand can be the low word of a general-purpose register or a 16-bit memory address. The count operand is an 8-bit immediate. When specifying a word location in an MMX technology register, the 2 least-significant bits of the count operand specify the location; for an XMM register, the 3 least-significant bits specify the location. The content of the destination register above bit 16 is cleared (set to all 0s).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------+
/// | # | Operands     |
/// +---+--------------+
/// | 1 | Gpq, Mm, Imm |
/// +---+--------------+
/// ```
pub trait MmxPextrwEmitter<A, B, C> {
    fn mmx_pextrw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> MmxPextrwEmitter<Gpq, Mm, Imm> for Assembler<'a> {
    fn mmx_pextrw(&mut self, op0: Gpq, op1: Mm, op2: Imm) {
        self.emit(MMX_PEXTRWRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `MMX_PINSRW` (PINSRW). 
/// Three operand MMX and SSE instructions
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRW.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------+
/// | # | Operands     |
/// +---+--------------+
/// | 1 | Mm, Gpd, Imm |
/// | 2 | Mm, Mem, Imm |
/// +---+--------------+
/// ```
pub trait MmxPinsrwEmitter<A, B, C> {
    fn mmx_pinsrw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> MmxPinsrwEmitter<Mm, Gpd, Imm> for Assembler<'a> {
    fn mmx_pinsrw(&mut self, op0: Mm, op1: Gpd, op2: Imm) {
        self.emit(MMX_PINSRWRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> MmxPinsrwEmitter<Mm, Mem, Imm> for Assembler<'a> {
    fn mmx_pinsrw(&mut self, op0: Mm, op1: Mem, op2: Imm) {
        self.emit(MMX_PINSRWRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `MMX_PMAXSW` (PMAXSW). 
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
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPmaxswEmitter<A, B> {
    fn mmx_pmaxsw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPmaxswEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pmaxsw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PMAXSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPmaxswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pmaxsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PMAXSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PMAXUB` (PMAXUB). 
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
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPmaxubEmitter<A, B> {
    fn mmx_pmaxub(&mut self, op0: A, op1: B);
}

impl<'a> MmxPmaxubEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pmaxub(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PMAXUBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPmaxubEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pmaxub(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PMAXUBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PMINSW` (PMINSW). 
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
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPminswEmitter<A, B> {
    fn mmx_pminsw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPminswEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pminsw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PMINSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPminswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pminsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PMINSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PMINUB` (PMINUB). 
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
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Mm   |
/// +---+----------+
/// ```
pub trait MmxPminubEmitter<A, B> {
    fn mmx_pminub(&mut self, op0: A, op1: B);
}

impl<'a> MmxPminubEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pminub(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PMINUBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPminubEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pminub(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PMINUBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PMOVMSKB` (PMOVMSKB). 
/// Creates a mask made up of the most significant bit of each byte of the source operand (second operand) and stores the result in the low byte or word of the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVMSKB.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpq, Mm  |
/// +---+----------+
/// ```
pub trait MmxPmovmskbEmitter<A, B> {
    fn mmx_pmovmskb(&mut self, op0: A, op1: B);
}

impl<'a> MmxPmovmskbEmitter<Gpq, Mm> for Assembler<'a> {
    fn mmx_pmovmskb(&mut self, op0: Gpq, op1: Mm) {
        self.emit(MMX_PMOVMSKBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PMULHUW` (PMULHUW). 
/// Performs a SIMD unsigned multiply of the packed unsigned word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each 32-bit intermediate results in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULHUW.html).
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
pub trait MmxPmulhuwEmitter<A, B> {
    fn mmx_pmulhuw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPmulhuwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_pmulhuw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PMULHUWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPmulhuwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pmulhuw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PMULHUWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSADBW` (PSADBW). 
/// Computes the absolute value of the difference of 8 unsigned byte integers from the source operand (second operand) and from the destination operand (first operand). These 8 differences are then summed to produce an unsigned word integer result that is stored in the destination operand. Figure 4-14 shows the operation of the PSADBW instruction when using 64-bit operands.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSADBW.html).
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
pub trait MmxPsadbwEmitter<A, B> {
    fn mmx_psadbw(&mut self, op0: A, op1: B);
}

impl<'a> MmxPsadbwEmitter<Mm, Mm> for Assembler<'a> {
    fn mmx_psadbw(&mut self, op0: Mm, op1: Mm) {
        self.emit(MMX_PSADBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxPsadbwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psadbw(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_PSADBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_PSHUFW` (PSHUFW). 
/// Copies words from the source operand (second operand) and inserts them in the destination operand (first operand) at word locations selected with the order operand (third operand). This operation is similar to the operation used by the PSHUFD instruction, which is illustrated in Figure 4-16. For the PSHUFW instruction, each 2-bit field in the order operand selects the contents of one word location in the destination operand. The encodings of the order operand fields select words from the source operand to be copied to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFW.html).
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
pub trait MmxPshufwEmitter<A, B, C> {
    fn mmx_pshufw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> MmxPshufwEmitter<Mm, Mm, Imm> for Assembler<'a> {
    fn mmx_pshufw(&mut self, op0: Mm, op1: Mm, op2: Imm) {
        self.emit(MMX_PSHUFWRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> MmxPshufwEmitter<Mm, Mem, Imm> for Assembler<'a> {
    fn mmx_pshufw(&mut self, op0: Mm, op1: Mem, op2: Imm) {
        self.emit(MMX_PSHUFWRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `PREFETCHNTA` (PREFETCHNTA). 
/// Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHh.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait PrefetchntaEmitter<A> {
    fn prefetchnta(&mut self, op0: A);
}

impl<'a> PrefetchntaEmitter<Mem> for Assembler<'a> {
    fn prefetchnta(&mut self, op0: Mem) {
        self.emit(PREFETCHNTAM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `PREFETCHT0` (PREFETCHT0). 
/// Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHh.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait Prefetcht0Emitter<A> {
    fn prefetcht0(&mut self, op0: A);
}

impl<'a> Prefetcht0Emitter<Mem> for Assembler<'a> {
    fn prefetcht0(&mut self, op0: Mem) {
        self.emit(PREFETCHT0M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `PREFETCHT1` (PREFETCHT1). 
/// Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHh.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait Prefetcht1Emitter<A> {
    fn prefetcht1(&mut self, op0: A);
}

impl<'a> Prefetcht1Emitter<Mem> for Assembler<'a> {
    fn prefetcht1(&mut self, op0: Mem) {
        self.emit(PREFETCHT1M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `PREFETCHT2` (PREFETCHT2). 
/// Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHh.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait Prefetcht2Emitter<A> {
    fn prefetcht2(&mut self, op0: A);
}

impl<'a> Prefetcht2Emitter<Mem> for Assembler<'a> {
    fn prefetcht2(&mut self, op0: Mem) {
        self.emit(PREFETCHT2M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `SFENCE` (SFENCE). 
/// Orders processor execution relative to all memory stores prior to the SFENCE instruction. The processor ensures that every store prior to SFENCE is globally visible before any store after SFENCE becomes globally visible. The SFENCE instruction is ordered with respect to memory stores, other SFENCE instructions, MFENCE instructions, and any serializing instructions (such as the CPUID instruction). It is not ordered with respect to memory loads or the LFENCE instruction.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SFENCE.html).
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
pub trait SfenceEmitter {
    fn sfence(&mut self);
}

impl<'a> SfenceEmitter for Assembler<'a> {
    fn sfence(&mut self) {
        self.emit(SFENCE, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `SSE_ADDPS` (ADDPS). 
/// Adds four, eight or sixteen packed single precision floating-point values from the first source operand with the second source operand, and stores the packed single precision floating-point result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDPS.html).
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
pub trait SseAddpsEmitter<A, B> {
    fn sse_addps(&mut self, op0: A, op1: B);
}

impl<'a> SseAddpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_addps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ADDPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseAddpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_addps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ADDPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_ADDSS` (ADDSS). 
/// Adds the low single precision floating-point values from the second source operand and the first source operand, and stores the double precision floating-point result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSS.html).
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
pub trait SseAddssEmitter<A, B> {
    fn sse_addss(&mut self, op0: A, op1: B);
}

impl<'a> SseAddssEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_addss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ADDSSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseAddssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_addss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ADDSSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_ANDNPS` (ANDNPS). 
/// Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPS.html).
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
pub trait SseAndnpsEmitter<A, B> {
    fn sse_andnps(&mut self, op0: A, op1: B);
}

impl<'a> SseAndnpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_andnps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ANDNPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseAndnpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_andnps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ANDNPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_ANDPS` (ANDPS). 
/// Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPS.html).
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
pub trait SseAndpsEmitter<A, B> {
    fn sse_andps(&mut self, op0: A, op1: B);
}

impl<'a> SseAndpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_andps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ANDPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseAndpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_andps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ANDPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CMPPS` (CMPPS). 
/// Performs a SIMD compare of the packed single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each of the pairs of packed values.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPPS.html).
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
pub trait SseCmppsEmitter<A, B, C> {
    fn sse_cmpps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseCmppsEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_cmpps(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_CMPPSRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseCmppsEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_cmpps(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_CMPPSRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_CMPSS` (CMPSS). 
/// Compares the low single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate operand) specifies the type of comparison performed.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPSS.html).
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
pub trait SseCmpssEmitter<A, B, C> {
    fn sse_cmpss(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseCmpssEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_cmpss(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_CMPSSRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseCmpssEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_cmpss(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_CMPSSRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_COMISS` (COMISS). 
/// Compares the single precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/COMISS.html).
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
pub trait SseComissEmitter<A, B> {
    fn sse_comiss(&mut self, op0: A, op1: B);
}

impl<'a> SseComissEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_comiss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_COMISSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseComissEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_comiss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_COMISSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTSI2SS` (CVTSI2SS). 
/// Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a single precision floating-point value in the destination operand (first operand). The “convert-from” source operand can be a general-purpose register or a memory location. The destination operand is an XMM register. The result is stored in the low doubleword of the destination operand, and the upper three doublewords are left unchanged. When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSI2SS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Gpd |
/// | 2 | Xmm, Gpq |
/// | 3 | Xmm, Mem |
/// +---+----------+
/// ```
pub trait SseCvtsi2ssEmitter<A, B> {
    fn sse_cvtsi2ss(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtsi2ssEmitter<Xmm, Gpd> for Assembler<'a> {
    fn sse_cvtsi2ss(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(SSE_CVTSI2SS32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtsi2ssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvtsi2ss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTSI2SS32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtsi2ssEmitter<Xmm, Gpq> for Assembler<'a> {
    fn sse_cvtsi2ss(&mut self, op0: Xmm, op1: Gpq) {
        self.emit(SSE_CVTSI2SS64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTSS2SI` (CVTSS2SI). 
/// Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSS2SI.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Mem |
/// | 2 | Gpd, Xmm |
/// | 3 | Gpq, Mem |
/// | 4 | Gpq, Xmm |
/// +---+----------+
/// ```
pub trait SseCvtss2siEmitter<A, B> {
    fn sse_cvtss2si(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtss2siEmitter<Gpd, Xmm> for Assembler<'a> {
    fn sse_cvtss2si(&mut self, op0: Gpd, op1: Xmm) {
        self.emit(SSE_CVTSS2SI32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtss2siEmitter<Gpd, Mem> for Assembler<'a> {
    fn sse_cvtss2si(&mut self, op0: Gpd, op1: Mem) {
        self.emit(SSE_CVTSS2SI32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtss2siEmitter<Gpq, Xmm> for Assembler<'a> {
    fn sse_cvtss2si(&mut self, op0: Gpq, op1: Xmm) {
        self.emit(SSE_CVTSS2SI64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtss2siEmitter<Gpq, Mem> for Assembler<'a> {
    fn sse_cvtss2si(&mut self, op0: Gpq, op1: Mem) {
        self.emit(SSE_CVTSS2SI64RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTTSS2SI` (CVTTSS2SI). 
/// Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 32-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTSS2SI.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Mem |
/// | 2 | Gpd, Xmm |
/// | 3 | Gpq, Mem |
/// | 4 | Gpq, Xmm |
/// +---+----------+
/// ```
pub trait SseCvttss2siEmitter<A, B> {
    fn sse_cvttss2si(&mut self, op0: A, op1: B);
}

impl<'a> SseCvttss2siEmitter<Gpd, Xmm> for Assembler<'a> {
    fn sse_cvttss2si(&mut self, op0: Gpd, op1: Xmm) {
        self.emit(SSE_CVTTSS2SI32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvttss2siEmitter<Gpd, Mem> for Assembler<'a> {
    fn sse_cvttss2si(&mut self, op0: Gpd, op1: Mem) {
        self.emit(SSE_CVTTSS2SI32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvttss2siEmitter<Gpq, Xmm> for Assembler<'a> {
    fn sse_cvttss2si(&mut self, op0: Gpq, op1: Xmm) {
        self.emit(SSE_CVTTSS2SI64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvttss2siEmitter<Gpq, Mem> for Assembler<'a> {
    fn sse_cvttss2si(&mut self, op0: Gpq, op1: Mem) {
        self.emit(SSE_CVTTSS2SI64RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_DIVPS` (DIVPS). 
/// Performs a SIMD divide of the four, eight or sixteen packed single precision floating-point values in the first source operand (the second operand) by the four, eight or sixteen packed single precision floating-point values in the second source operand (the third operand). Results are written to the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DIVPS.html).
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
pub trait SseDivpsEmitter<A, B> {
    fn sse_divps(&mut self, op0: A, op1: B);
}

impl<'a> SseDivpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_divps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_DIVPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseDivpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_divps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_DIVPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_DIVSS` (DIVSS). 
/// Divides the low single precision floating-point value in the first source operand by the low single precision floating-point value in the second source operand, and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DIVSS.html).
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
pub trait SseDivssEmitter<A, B> {
    fn sse_divss(&mut self, op0: A, op1: B);
}

impl<'a> SseDivssEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_divss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_DIVSSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseDivssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_divss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_DIVSSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MAXPS` (MAXPS). 
/// Performs a SIMD compare of the packed single precision floating-point values in the first source operand and the second source operand and returns the maximum value for each pair of values to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MAXPS.html).
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
pub trait SseMaxpsEmitter<A, B> {
    fn sse_maxps(&mut self, op0: A, op1: B);
}

impl<'a> SseMaxpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_maxps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MAXPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMaxpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_maxps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MAXPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MAXSS` (MAXSS). 
/// Compares the low single precision floating-point values in the first source operand and the second source operand, and returns the maximum value to the low doubleword of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MAXSS.html).
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
pub trait SseMaxssEmitter<A, B> {
    fn sse_maxss(&mut self, op0: A, op1: B);
}

impl<'a> SseMaxssEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_maxss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MAXSSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMaxssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_maxss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MAXSSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MINPS` (MINPS). 
/// Performs a SIMD compare of the packed single precision floating-point values in the first source operand and the second source operand and returns the minimum value for each pair of values to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MINPS.html).
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
pub trait SseMinpsEmitter<A, B> {
    fn sse_minps(&mut self, op0: A, op1: B);
}

impl<'a> SseMinpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_minps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MINPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMinpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_minps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MINPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MINSS` (MINSS). 
/// Compares the low single precision floating-point values in the first source operand and the second source operand and returns the minimum value to the low doubleword of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MINSS.html).
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
pub trait SseMinssEmitter<A, B> {
    fn sse_minss(&mut self, op0: A, op1: B);
}

impl<'a> SseMinssEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_minss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MINSSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMinssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_minss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MINSSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVAPS` (MOVAPS). 
/// Moves 4, 8 or 16 single precision floating-point values from the source operand (second operand) to the destination operand (first operand). This instruction can be used to load an XMM, YMM or ZMM register from an 128-bit, 256-bit or 512-bit memory location, to store the contents of an XMM, YMM or ZMM register into a 128-bit, 256-bit or 512-bit memory location, or to move data between two XMM, two YMM or two ZMM registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVAPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovapsEmitter<A, B> {
    fn sse_movaps(&mut self, op0: A, op1: B);
}

impl<'a> SseMovapsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movaps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVAPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovapsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movaps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVAPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovapsEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movaps(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVAPSMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVHLPS` (MOVHLPS). 
/// This instruction cannot be used for memory to register moves.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVHLPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovhlpsEmitter<A, B> {
    fn sse_movhlps(&mut self, op0: A, op1: B);
}

impl<'a> SseMovhlpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movhlps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVHLPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVHPS` (MOVHPS). 
/// This instruction cannot be used for register to register or memory to memory moves.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVHPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Xmm, Mem |
/// +---+----------+
/// ```
pub trait SseMovhpsEmitter<A, B> {
    fn sse_movhps(&mut self, op0: A, op1: B);
}

impl<'a> SseMovhpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movhps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVHPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovhpsEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movhps(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVHPSMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVLHPS` (MOVLHPS). 
/// This instruction cannot be used for memory to register moves.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVLHPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovlhpsEmitter<A, B> {
    fn sse_movlhps(&mut self, op0: A, op1: B);
}

impl<'a> SseMovlhpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movlhps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVLHPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVLPS` (MOVLPS). 
/// This instruction cannot be used for register to register or memory to memory moves.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVLPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Xmm, Mem |
/// +---+----------+
/// ```
pub trait SseMovlpsEmitter<A, B> {
    fn sse_movlps(&mut self, op0: A, op1: B);
}

impl<'a> SseMovlpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movlps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVLPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovlpsEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movlps(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVLPSMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVMSKPS` (MOVMSKPS). 
/// Extracts the sign bits from the packed single precision floating-point values in the source operand (second operand), formats them into a 4- or 8-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM or YMM register, and the destination operand is a general-purpose register. The mask is stored in the 4 or 8 low-order bits of the destination operand. The upper bits of the destination operand beyond the mask are filled with zeros.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVMSKPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpq, Xmm |
/// +---+----------+
/// ```
pub trait SseMovmskpsEmitter<A, B> {
    fn sse_movmskps(&mut self, op0: A, op1: B);
}

impl<'a> SseMovmskpsEmitter<Gpq, Xmm> for Assembler<'a> {
    fn sse_movmskps(&mut self, op0: Gpq, op1: Xmm) {
        self.emit(SSE_MOVMSKPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVNTPS` (MOVNTPS). 
/// Moves the packed single precision floating-point values in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to prevent caching of the data during the write to memory. The source operand is an XMM register, YMM register or ZMM register, which is assumed to contain packed single precision, floating-pointing. The destination operand is a 128-bit, 256-bit or 512-bit memory location. The memory operand must be aligned on a 16-byte (128-bit version), 32-byte (VEX.256 encoded version) or 64-byte (EVEX.512 encoded version) boundary otherwise a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// +---+----------+
/// ```
pub trait SseMovntpsEmitter<A, B> {
    fn sse_movntps(&mut self, op0: A, op1: B);
}

impl<'a> SseMovntpsEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movntps(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVNTPSMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVNTSS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// +---+----------+
/// ```
pub trait SseMovntssEmitter<A, B> {
    fn sse_movntss(&mut self, op0: A, op1: B);
}

impl<'a> SseMovntssEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movntss(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVNTSSMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVSS` (MOVSS). 
/// Moves a scalar single precision floating-point value from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be XMM registers or 32-bit memory locations. This instruction can be used to move a single precision floating-point value to and from the low doubleword of an XMM register and a 32-bit memory location, or to move a single precision floating-point value between the low doublewords of two XMM registers. The instruction cannot be used to transfer data between memory locations.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVSS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovssEmitter<A, B> {
    fn sse_movss(&mut self, op0: A, op1: B);
}

impl<'a> SseMovssEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVSSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVSSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovssEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movss(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVSSMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVUPS` (MOVUPS). 
/// Note: VEX.vvvv and EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVUPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovupsEmitter<A, B> {
    fn sse_movups(&mut self, op0: A, op1: B);
}

impl<'a> SseMovupsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movups(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVUPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovupsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movups(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVUPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovupsEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movups(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVUPSMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MULPS` (MULPS). 
/// Multiply the packed single precision floating-point values from the first source operand with the corresponding values in the second source operand, and stores the packed double precision floating-point results in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MULPS.html).
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
pub trait SseMulpsEmitter<A, B> {
    fn sse_mulps(&mut self, op0: A, op1: B);
}

impl<'a> SseMulpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_mulps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MULPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMulpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_mulps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MULPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MULSS` (MULSS). 
/// Multiplies the low single precision floating-point value from the second source operand by the low single precision floating-point value in the first source operand, and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location. The first source operand and the destination operands are XMM registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MULSS.html).
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
pub trait SseMulssEmitter<A, B> {
    fn sse_mulss(&mut self, op0: A, op1: B);
}

impl<'a> SseMulssEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_mulss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MULSSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMulssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_mulss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MULSSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_ORPS` (ORPS). 
/// Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPS.html).
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
pub trait SseOrpsEmitter<A, B> {
    fn sse_orps(&mut self, op0: A, op1: B);
}

impl<'a> SseOrpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_orps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ORPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseOrpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_orps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ORPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_RCPPS` (RCPPS). 
/// Performs a SIMD computation of the approximate reciprocals of the four packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RCPPS.html).
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
pub trait SseRcppsEmitter<A, B> {
    fn sse_rcpps(&mut self, op0: A, op1: B);
}

impl<'a> SseRcppsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_rcpps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_RCPPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseRcppsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_rcpps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_RCPPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_RCPSS` (RCPSS). 
/// Computes of an approximate reciprocal of the low single precision floating-point value in the source operand (second operand) and stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RCPSS.html).
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
pub trait SseRcpssEmitter<A, B> {
    fn sse_rcpss(&mut self, op0: A, op1: B);
}

impl<'a> SseRcpssEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_rcpss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_RCPSSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseRcpssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_rcpss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_RCPSSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_RSQRTPS` (RSQRTPS). 
/// Performs a SIMD computation of the approximate reciprocals of the square roots of the four packed single precision floating-point values in the source operand (second operand) and stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RSQRTPS.html).
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
pub trait SseRsqrtpsEmitter<A, B> {
    fn sse_rsqrtps(&mut self, op0: A, op1: B);
}

impl<'a> SseRsqrtpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_rsqrtps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_RSQRTPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseRsqrtpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_rsqrtps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_RSQRTPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_RSQRTSS` (RSQRTSS). 
/// Computes an approximate reciprocal of the square root of the low single precision floating-point value in the source operand (second operand) stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RSQRTSS.html).
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
pub trait SseRsqrtssEmitter<A, B> {
    fn sse_rsqrtss(&mut self, op0: A, op1: B);
}

impl<'a> SseRsqrtssEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_rsqrtss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_RSQRTSSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseRsqrtssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_rsqrtss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_RSQRTSSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_SHUFPS` (SHUFPS). 
/// Selects a single precision floating-point value of an input quadruplet using a two-bit control and move to a designated element of the destination operand. Each 64-bit element-pair of a 128-bit lane of the destination operand is interleaved between the corresponding lane of the first source operand and the second source operand at the granularity 128 bits. Each two bits in the imm8 byte, starting from bit 0, is the select control of the corresponding element of a 128-bit lane of the destination to received the shuffled result of an input quadruplet. The two lower elements of a 128-bit lane in the destination receives shuffle results from the quadruple of the first source operand. The next two elements of the destination receives shuffle results from the quadruple of the second source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SHUFPS.html).
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
pub trait SseShufpsEmitter<A, B, C> {
    fn sse_shufps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseShufpsEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_shufps(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_SHUFPSRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseShufpsEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_shufps(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_SHUFPSRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_SQRTPS` (SQRTPS). 
/// Performs a SIMD computation of the square roots of the four, eight or sixteen packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SQRTPS.html).
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
pub trait SseSqrtpsEmitter<A, B> {
    fn sse_sqrtps(&mut self, op0: A, op1: B);
}

impl<'a> SseSqrtpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_sqrtps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_SQRTPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseSqrtpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_sqrtps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_SQRTPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_SQRTSS` (SQRTSS). 
/// Computes the square root of the low single precision floating-point value in the second source operand and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location. The first source and destination operands is an XMM register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SQRTSS.html).
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
pub trait SseSqrtssEmitter<A, B> {
    fn sse_sqrtss(&mut self, op0: A, op1: B);
}

impl<'a> SseSqrtssEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_sqrtss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_SQRTSSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseSqrtssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_sqrtss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_SQRTSSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_SUBPS` (SUBPS). 
/// Performs a SIMD subtract of the packed single precision floating-point values in the second Source operand from the First Source operand, and stores the packed single precision floating-point results in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SUBPS.html).
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
pub trait SseSubpsEmitter<A, B> {
    fn sse_subps(&mut self, op0: A, op1: B);
}

impl<'a> SseSubpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_subps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_SUBPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseSubpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_subps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_SUBPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_SUBSS` (SUBSS). 
/// Subtract the low single precision floating-point value from the second source operand and the first source operand and store the double precision floating-point result in the low doubleword of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SUBSS.html).
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
pub trait SseSubssEmitter<A, B> {
    fn sse_subss(&mut self, op0: A, op1: B);
}

impl<'a> SseSubssEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_subss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_SUBSSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseSubssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_subss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_SUBSSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_UCOMISS` (UCOMISS). 
/// Compares the single precision floating-point values in the low doublewords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UCOMISS.html).
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
pub trait SseUcomissEmitter<A, B> {
    fn sse_ucomiss(&mut self, op0: A, op1: B);
}

impl<'a> SseUcomissEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_ucomiss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_UCOMISSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseUcomissEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_ucomiss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_UCOMISSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_UNPCKHPS` (UNPCKHPS). 
/// Performs an interleaved unpack of the high single precision floating-point values from the first source operand and the second source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UNPCKHPS.html).
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
pub trait SseUnpckhpsEmitter<A, B> {
    fn sse_unpckhps(&mut self, op0: A, op1: B);
}

impl<'a> SseUnpckhpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_unpckhps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_UNPCKHPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseUnpckhpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_unpckhps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_UNPCKHPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_UNPCKLPS` (UNPCKLPS). 
/// Performs an interleaved unpack of the low single precision floating-point values from the first source operand and the second source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UNPCKLPS.html).
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
pub trait SseUnpcklpsEmitter<A, B> {
    fn sse_unpcklps(&mut self, op0: A, op1: B);
}

impl<'a> SseUnpcklpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_unpcklps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_UNPCKLPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseUnpcklpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_unpcklps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_UNPCKLPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_XORPS` (XORPS). 
/// Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPS.html).
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
pub trait SseXorpsEmitter<A, B> {
    fn sse_xorps(&mut self, op0: A, op1: B);
}

impl<'a> SseXorpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_xorps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_XORPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseXorpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_xorps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_XORPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `STMXCSR` (STMXCSR). 
/// Stores the contents of the MXCSR control and status register to the destination operand. The destination operand is a 32-bit memory location. The reserved bits in the MXCSR register are stored as 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/STMXCSR.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem      |
/// +---+----------+
/// ```
pub trait StmxcsrEmitter<A> {
    fn stmxcsr(&mut self, op0: A);
}

impl<'a> StmxcsrEmitter<Mem> for Assembler<'a> {
    fn stmxcsr(&mut self, op0: Mem) {
        self.emit(STMXCSRM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `LDMXCSR` (LDMXCSR). 
    /// Loads the source operand into the MXCSR control/status register. The source operand is a 32-bit memory location. See “MXCSR Control and Status Register” in Chapter 10, of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for a description of the MXCSR register and its contents.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/LDMXCSR.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn ldmxcsr<A>(&mut self, op0: A)
    where Assembler<'a>: LdmxcsrEmitter<A> {
        <Self as LdmxcsrEmitter<A>>::ldmxcsr(self, op0);
    }
    /// `MMX_MASKMOVQ` (MASKMOVQ). 
    /// Stores selected bytes from the source operand (first operand) into a 64-bit memory location. The mask operand (second operand) selects which bytes from the source operand are written to memory. The source and mask operands are MMX technology registers. The memory location specified by the effective address in the DI/EDI/RDI register (the default segment register is DS, but this may be overridden with a segment-override prefix). The memory location does not need to be aligned on a natural boundary. (The size of the store address depends on the address-size attribute.)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MASKMOVQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_maskmovq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxMaskmovqEmitter<A, B> {
        <Self as MmxMaskmovqEmitter<A, B>>::mmx_maskmovq(self, op0, op1);
    }
    /// `MMX_MOVDQ2Q`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Xmm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_movdq2q<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxMovdq2qEmitter<A, B> {
        <Self as MmxMovdq2qEmitter<A, B>>::mmx_movdq2q(self, op0, op1);
    }
    /// `MMX_MOVNTQ` (MOVNTQ). 
    /// Moves the quadword in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to minimize cache pollution during the write to memory. The source operand is an MMX technology register, which is assumed to contain packed integer data (packed bytes, words, or doublewords). The destination operand is a 64-bit memory location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Mm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_movntq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxMovntqEmitter<A, B> {
        <Self as MmxMovntqEmitter<A, B>>::mmx_movntq(self, op0, op1);
    }
    /// `MMX_MOVQ2DQ` (MOVQ2DQ). 
    /// Moves the quadword from the source operand (second operand) to the low quadword of the destination operand (first operand). The source operand is an MMX technology register and the destination operand is an XMM register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVQ2DQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_movq2dq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxMovq2dqEmitter<A, B> {
        <Self as MmxMovq2dqEmitter<A, B>>::mmx_movq2dq(self, op0, op1);
    }
    /// `MMX_PAVGB` (PAVGB). 
    /// Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html).
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
    pub fn mmx_pavgb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPavgbEmitter<A, B> {
        <Self as MmxPavgbEmitter<A, B>>::mmx_pavgb(self, op0, op1);
    }
    /// `MMX_PAVGW` (PAVGW). 
    /// Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand. For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PAVGB%3APAVGW.html).
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
    pub fn mmx_pavgw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPavgwEmitter<A, B> {
        <Self as MmxPavgwEmitter<A, B>>::mmx_pavgw(self, op0, op1);
    }
    /// `MMX_PEXTRW` (PEXTRW). 
    /// Copies the word in the source operand (second operand) specified by the count operand (third operand) to the destination operand (first operand). The source operand can be an MMX technology register or an XMM register. The destination operand can be the low word of a general-purpose register or a 16-bit memory address. The count operand is an 8-bit immediate. When specifying a word location in an MMX technology register, the 2 least-significant bits of the count operand specify the location; for an XMM register, the 3 least-significant bits specify the location. The content of the destination register above bit 16 is cleared (set to all 0s).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------+
    /// | # | Operands     |
    /// +---+--------------+
    /// | 1 | Gpq, Mm, Imm |
    /// +---+--------------+
    /// ```
    #[inline]
    pub fn mmx_pextrw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: MmxPextrwEmitter<A, B, C> {
        <Self as MmxPextrwEmitter<A, B, C>>::mmx_pextrw(self, op0, op1, op2);
    }
    /// `MMX_PINSRW` (PINSRW). 
    /// Three operand MMX and SSE instructions
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRW.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------+
    /// | # | Operands     |
    /// +---+--------------+
    /// | 1 | Mm, Gpd, Imm |
    /// | 2 | Mm, Mem, Imm |
    /// +---+--------------+
    /// ```
    #[inline]
    pub fn mmx_pinsrw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: MmxPinsrwEmitter<A, B, C> {
        <Self as MmxPinsrwEmitter<A, B, C>>::mmx_pinsrw(self, op0, op1, op2);
    }
    /// `MMX_PMAXSW` (PMAXSW). 
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
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pmaxsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPmaxswEmitter<A, B> {
        <Self as MmxPmaxswEmitter<A, B>>::mmx_pmaxsw(self, op0, op1);
    }
    /// `MMX_PMAXUB` (PMAXUB). 
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
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pmaxub<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPmaxubEmitter<A, B> {
        <Self as MmxPmaxubEmitter<A, B>>::mmx_pmaxub(self, op0, op1);
    }
    /// `MMX_PMINSW` (PMINSW). 
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
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pminsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPminswEmitter<A, B> {
        <Self as MmxPminswEmitter<A, B>>::mmx_pminsw(self, op0, op1);
    }
    /// `MMX_PMINUB` (PMINUB). 
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
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Mm   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pminub<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPminubEmitter<A, B> {
        <Self as MmxPminubEmitter<A, B>>::mmx_pminub(self, op0, op1);
    }
    /// `MMX_PMOVMSKB` (PMOVMSKB). 
    /// Creates a mask made up of the most significant bit of each byte of the source operand (second operand) and stores the result in the low byte or word of the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVMSKB.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpq, Mm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_pmovmskb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPmovmskbEmitter<A, B> {
        <Self as MmxPmovmskbEmitter<A, B>>::mmx_pmovmskb(self, op0, op1);
    }
    /// `MMX_PMULHUW` (PMULHUW). 
    /// Performs a SIMD unsigned multiply of the packed unsigned word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each 32-bit intermediate results in the destination operand. (Figure 4-12 shows this operation when using 64-bit operands.)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULHUW.html).
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
    pub fn mmx_pmulhuw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPmulhuwEmitter<A, B> {
        <Self as MmxPmulhuwEmitter<A, B>>::mmx_pmulhuw(self, op0, op1);
    }
    /// `MMX_PSADBW` (PSADBW). 
    /// Computes the absolute value of the difference of 8 unsigned byte integers from the source operand (second operand) and from the destination operand (first operand). These 8 differences are then summed to produce an unsigned word integer result that is stored in the destination operand. Figure 4-14 shows the operation of the PSADBW instruction when using 64-bit operands.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSADBW.html).
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
    pub fn mmx_psadbw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxPsadbwEmitter<A, B> {
        <Self as MmxPsadbwEmitter<A, B>>::mmx_psadbw(self, op0, op1);
    }
    /// `MMX_PSHUFW` (PSHUFW). 
    /// Copies words from the source operand (second operand) and inserts them in the destination operand (first operand) at word locations selected with the order operand (third operand). This operation is similar to the operation used by the PSHUFD instruction, which is illustrated in Figure 4-16. For the PSHUFW instruction, each 2-bit field in the order operand selects the contents of one word location in the destination operand. The encodings of the order operand fields select words from the source operand to be copied to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFW.html).
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
    pub fn mmx_pshufw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: MmxPshufwEmitter<A, B, C> {
        <Self as MmxPshufwEmitter<A, B, C>>::mmx_pshufw(self, op0, op1, op2);
    }
    /// `PREFETCHNTA` (PREFETCHNTA). 
    /// Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHh.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn prefetchnta<A>(&mut self, op0: A)
    where Assembler<'a>: PrefetchntaEmitter<A> {
        <Self as PrefetchntaEmitter<A>>::prefetchnta(self, op0);
    }
    /// `PREFETCHT0` (PREFETCHT0). 
    /// Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHh.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn prefetcht0<A>(&mut self, op0: A)
    where Assembler<'a>: Prefetcht0Emitter<A> {
        <Self as Prefetcht0Emitter<A>>::prefetcht0(self, op0);
    }
    /// `PREFETCHT1` (PREFETCHT1). 
    /// Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHh.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn prefetcht1<A>(&mut self, op0: A)
    where Assembler<'a>: Prefetcht1Emitter<A> {
        <Self as Prefetcht1Emitter<A>>::prefetcht1(self, op0);
    }
    /// `PREFETCHT2` (PREFETCHT2). 
    /// Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by a locality hint
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHh.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn prefetcht2<A>(&mut self, op0: A)
    where Assembler<'a>: Prefetcht2Emitter<A> {
        <Self as Prefetcht2Emitter<A>>::prefetcht2(self, op0);
    }
    /// `SFENCE` (SFENCE). 
    /// Orders processor execution relative to all memory stores prior to the SFENCE instruction. The processor ensures that every store prior to SFENCE is globally visible before any store after SFENCE becomes globally visible. The SFENCE instruction is ordered with respect to memory stores, other SFENCE instructions, MFENCE instructions, and any serializing instructions (such as the CPUID instruction). It is not ordered with respect to memory loads or the LFENCE instruction.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SFENCE.html).
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
    pub fn sfence(&mut self)
    where Assembler<'a>: SfenceEmitter {
        <Self as SfenceEmitter>::sfence(self);
    }
    /// `SSE_ADDPS` (ADDPS). 
    /// Adds four, eight or sixteen packed single precision floating-point values from the first source operand with the second source operand, and stores the packed single precision floating-point result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDPS.html).
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
    pub fn sse_addps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseAddpsEmitter<A, B> {
        <Self as SseAddpsEmitter<A, B>>::sse_addps(self, op0, op1);
    }
    /// `SSE_ADDSS` (ADDSS). 
    /// Adds the low single precision floating-point values from the second source operand and the first source operand, and stores the double precision floating-point result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSS.html).
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
    pub fn sse_addss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseAddssEmitter<A, B> {
        <Self as SseAddssEmitter<A, B>>::sse_addss(self, op0, op1);
    }
    /// `SSE_ANDNPS` (ANDNPS). 
    /// Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPS.html).
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
    pub fn sse_andnps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseAndnpsEmitter<A, B> {
        <Self as SseAndnpsEmitter<A, B>>::sse_andnps(self, op0, op1);
    }
    /// `SSE_ANDPS` (ANDPS). 
    /// Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPS.html).
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
    pub fn sse_andps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseAndpsEmitter<A, B> {
        <Self as SseAndpsEmitter<A, B>>::sse_andps(self, op0, op1);
    }
    /// `SSE_CMPPS` (CMPPS). 
    /// Performs a SIMD compare of the packed single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each of the pairs of packed values.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPPS.html).
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
    pub fn sse_cmpps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseCmppsEmitter<A, B, C> {
        <Self as SseCmppsEmitter<A, B, C>>::sse_cmpps(self, op0, op1, op2);
    }
    /// `SSE_CMPSS` (CMPSS). 
    /// Compares the low single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate operand) specifies the type of comparison performed.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPSS.html).
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
    pub fn sse_cmpss<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseCmpssEmitter<A, B, C> {
        <Self as SseCmpssEmitter<A, B, C>>::sse_cmpss(self, op0, op1, op2);
    }
    /// `SSE_COMISS` (COMISS). 
    /// Compares the single precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/COMISS.html).
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
    pub fn sse_comiss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseComissEmitter<A, B> {
        <Self as SseComissEmitter<A, B>>::sse_comiss(self, op0, op1);
    }
    /// `SSE_CVTSI2SS` (CVTSI2SS). 
    /// Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a single precision floating-point value in the destination operand (first operand). The “convert-from” source operand can be a general-purpose register or a memory location. The destination operand is an XMM register. The result is stored in the low doubleword of the destination operand, and the upper three doublewords are left unchanged. When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSI2SS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Gpd |
    /// | 2 | Xmm, Gpq |
    /// | 3 | Xmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_cvtsi2ss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtsi2ssEmitter<A, B> {
        <Self as SseCvtsi2ssEmitter<A, B>>::sse_cvtsi2ss(self, op0, op1);
    }
    /// `SSE_CVTSS2SI` (CVTSS2SI). 
    /// Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSS2SI.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Mem |
    /// | 2 | Gpd, Xmm |
    /// | 3 | Gpq, Mem |
    /// | 4 | Gpq, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_cvtss2si<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtss2siEmitter<A, B> {
        <Self as SseCvtss2siEmitter<A, B>>::sse_cvtss2si(self, op0, op1);
    }
    /// `SSE_CVTTSS2SI` (CVTTSS2SI). 
    /// Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 32-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTSS2SI.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Mem |
    /// | 2 | Gpd, Xmm |
    /// | 3 | Gpq, Mem |
    /// | 4 | Gpq, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_cvttss2si<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvttss2siEmitter<A, B> {
        <Self as SseCvttss2siEmitter<A, B>>::sse_cvttss2si(self, op0, op1);
    }
    /// `SSE_DIVPS` (DIVPS). 
    /// Performs a SIMD divide of the four, eight or sixteen packed single precision floating-point values in the first source operand (the second operand) by the four, eight or sixteen packed single precision floating-point values in the second source operand (the third operand). Results are written to the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DIVPS.html).
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
    pub fn sse_divps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseDivpsEmitter<A, B> {
        <Self as SseDivpsEmitter<A, B>>::sse_divps(self, op0, op1);
    }
    /// `SSE_DIVSS` (DIVSS). 
    /// Divides the low single precision floating-point value in the first source operand by the low single precision floating-point value in the second source operand, and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DIVSS.html).
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
    pub fn sse_divss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseDivssEmitter<A, B> {
        <Self as SseDivssEmitter<A, B>>::sse_divss(self, op0, op1);
    }
    /// `SSE_MAXPS` (MAXPS). 
    /// Performs a SIMD compare of the packed single precision floating-point values in the first source operand and the second source operand and returns the maximum value for each pair of values to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MAXPS.html).
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
    pub fn sse_maxps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMaxpsEmitter<A, B> {
        <Self as SseMaxpsEmitter<A, B>>::sse_maxps(self, op0, op1);
    }
    /// `SSE_MAXSS` (MAXSS). 
    /// Compares the low single precision floating-point values in the first source operand and the second source operand, and returns the maximum value to the low doubleword of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MAXSS.html).
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
    pub fn sse_maxss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMaxssEmitter<A, B> {
        <Self as SseMaxssEmitter<A, B>>::sse_maxss(self, op0, op1);
    }
    /// `SSE_MINPS` (MINPS). 
    /// Performs a SIMD compare of the packed single precision floating-point values in the first source operand and the second source operand and returns the minimum value for each pair of values to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MINPS.html).
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
    pub fn sse_minps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMinpsEmitter<A, B> {
        <Self as SseMinpsEmitter<A, B>>::sse_minps(self, op0, op1);
    }
    /// `SSE_MINSS` (MINSS). 
    /// Compares the low single precision floating-point values in the first source operand and the second source operand and returns the minimum value to the low doubleword of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MINSS.html).
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
    pub fn sse_minss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMinssEmitter<A, B> {
        <Self as SseMinssEmitter<A, B>>::sse_minss(self, op0, op1);
    }
    /// `SSE_MOVAPS` (MOVAPS). 
    /// Moves 4, 8 or 16 single precision floating-point values from the source operand (second operand) to the destination operand (first operand). This instruction can be used to load an XMM, YMM or ZMM register from an 128-bit, 256-bit or 512-bit memory location, to store the contents of an XMM, YMM or ZMM register into a 128-bit, 256-bit or 512-bit memory location, or to move data between two XMM, two YMM or two ZMM registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVAPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movaps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovapsEmitter<A, B> {
        <Self as SseMovapsEmitter<A, B>>::sse_movaps(self, op0, op1);
    }
    /// `SSE_MOVHLPS` (MOVHLPS). 
    /// This instruction cannot be used for memory to register moves.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVHLPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movhlps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovhlpsEmitter<A, B> {
        <Self as SseMovhlpsEmitter<A, B>>::sse_movhlps(self, op0, op1);
    }
    /// `SSE_MOVHPS` (MOVHPS). 
    /// This instruction cannot be used for register to register or memory to memory moves.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVHPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Xmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movhps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovhpsEmitter<A, B> {
        <Self as SseMovhpsEmitter<A, B>>::sse_movhps(self, op0, op1);
    }
    /// `SSE_MOVLHPS` (MOVLHPS). 
    /// This instruction cannot be used for memory to register moves.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVLHPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movlhps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovlhpsEmitter<A, B> {
        <Self as SseMovlhpsEmitter<A, B>>::sse_movlhps(self, op0, op1);
    }
    /// `SSE_MOVLPS` (MOVLPS). 
    /// This instruction cannot be used for register to register or memory to memory moves.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVLPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Xmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movlps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovlpsEmitter<A, B> {
        <Self as SseMovlpsEmitter<A, B>>::sse_movlps(self, op0, op1);
    }
    /// `SSE_MOVMSKPS` (MOVMSKPS). 
    /// Extracts the sign bits from the packed single precision floating-point values in the source operand (second operand), formats them into a 4- or 8-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM or YMM register, and the destination operand is a general-purpose register. The mask is stored in the 4 or 8 low-order bits of the destination operand. The upper bits of the destination operand beyond the mask are filled with zeros.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVMSKPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpq, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movmskps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovmskpsEmitter<A, B> {
        <Self as SseMovmskpsEmitter<A, B>>::sse_movmskps(self, op0, op1);
    }
    /// `SSE_MOVNTPS` (MOVNTPS). 
    /// Moves the packed single precision floating-point values in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to prevent caching of the data during the write to memory. The source operand is an XMM register, YMM register or ZMM register, which is assumed to contain packed single precision, floating-pointing. The destination operand is a 128-bit, 256-bit or 512-bit memory location. The memory operand must be aligned on a 16-byte (128-bit version), 32-byte (VEX.256 encoded version) or 64-byte (EVEX.512 encoded version) boundary otherwise a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movntps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovntpsEmitter<A, B> {
        <Self as SseMovntpsEmitter<A, B>>::sse_movntps(self, op0, op1);
    }
    /// `SSE_MOVNTSS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movntss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovntssEmitter<A, B> {
        <Self as SseMovntssEmitter<A, B>>::sse_movntss(self, op0, op1);
    }
    /// `SSE_MOVSS` (MOVSS). 
    /// Moves a scalar single precision floating-point value from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be XMM registers or 32-bit memory locations. This instruction can be used to move a single precision floating-point value to and from the low doubleword of an XMM register and a 32-bit memory location, or to move a single precision floating-point value between the low doublewords of two XMM registers. The instruction cannot be used to transfer data between memory locations.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVSS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovssEmitter<A, B> {
        <Self as SseMovssEmitter<A, B>>::sse_movss(self, op0, op1);
    }
    /// `SSE_MOVUPS` (MOVUPS). 
    /// Note: VEX.vvvv and EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVUPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movups<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovupsEmitter<A, B> {
        <Self as SseMovupsEmitter<A, B>>::sse_movups(self, op0, op1);
    }
    /// `SSE_MULPS` (MULPS). 
    /// Multiply the packed single precision floating-point values from the first source operand with the corresponding values in the second source operand, and stores the packed double precision floating-point results in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MULPS.html).
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
    pub fn sse_mulps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMulpsEmitter<A, B> {
        <Self as SseMulpsEmitter<A, B>>::sse_mulps(self, op0, op1);
    }
    /// `SSE_MULSS` (MULSS). 
    /// Multiplies the low single precision floating-point value from the second source operand by the low single precision floating-point value in the first source operand, and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location. The first source operand and the destination operands are XMM registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MULSS.html).
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
    pub fn sse_mulss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMulssEmitter<A, B> {
        <Self as SseMulssEmitter<A, B>>::sse_mulss(self, op0, op1);
    }
    /// `SSE_ORPS` (ORPS). 
    /// Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPS.html).
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
    pub fn sse_orps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseOrpsEmitter<A, B> {
        <Self as SseOrpsEmitter<A, B>>::sse_orps(self, op0, op1);
    }
    /// `SSE_RCPPS` (RCPPS). 
    /// Performs a SIMD computation of the approximate reciprocals of the four packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RCPPS.html).
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
    pub fn sse_rcpps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseRcppsEmitter<A, B> {
        <Self as SseRcppsEmitter<A, B>>::sse_rcpps(self, op0, op1);
    }
    /// `SSE_RCPSS` (RCPSS). 
    /// Computes of an approximate reciprocal of the low single precision floating-point value in the source operand (second operand) and stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RCPSS.html).
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
    pub fn sse_rcpss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseRcpssEmitter<A, B> {
        <Self as SseRcpssEmitter<A, B>>::sse_rcpss(self, op0, op1);
    }
    /// `SSE_RSQRTPS` (RSQRTPS). 
    /// Performs a SIMD computation of the approximate reciprocals of the square roots of the four packed single precision floating-point values in the source operand (second operand) and stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RSQRTPS.html).
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
    pub fn sse_rsqrtps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseRsqrtpsEmitter<A, B> {
        <Self as SseRsqrtpsEmitter<A, B>>::sse_rsqrtps(self, op0, op1);
    }
    /// `SSE_RSQRTSS` (RSQRTSS). 
    /// Computes an approximate reciprocal of the square root of the low single precision floating-point value in the source operand (second operand) stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RSQRTSS.html).
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
    pub fn sse_rsqrtss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseRsqrtssEmitter<A, B> {
        <Self as SseRsqrtssEmitter<A, B>>::sse_rsqrtss(self, op0, op1);
    }
    /// `SSE_SHUFPS` (SHUFPS). 
    /// Selects a single precision floating-point value of an input quadruplet using a two-bit control and move to a designated element of the destination operand. Each 64-bit element-pair of a 128-bit lane of the destination operand is interleaved between the corresponding lane of the first source operand and the second source operand at the granularity 128 bits. Each two bits in the imm8 byte, starting from bit 0, is the select control of the corresponding element of a 128-bit lane of the destination to received the shuffled result of an input quadruplet. The two lower elements of a 128-bit lane in the destination receives shuffle results from the quadruple of the first source operand. The next two elements of the destination receives shuffle results from the quadruple of the second source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SHUFPS.html).
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
    pub fn sse_shufps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseShufpsEmitter<A, B, C> {
        <Self as SseShufpsEmitter<A, B, C>>::sse_shufps(self, op0, op1, op2);
    }
    /// `SSE_SQRTPS` (SQRTPS). 
    /// Performs a SIMD computation of the square roots of the four, eight or sixteen packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SQRTPS.html).
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
    pub fn sse_sqrtps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseSqrtpsEmitter<A, B> {
        <Self as SseSqrtpsEmitter<A, B>>::sse_sqrtps(self, op0, op1);
    }
    /// `SSE_SQRTSS` (SQRTSS). 
    /// Computes the square root of the low single precision floating-point value in the second source operand and stores the single precision floating-point result in the destination operand. The second source operand can be an XMM register or a 32-bit memory location. The first source and destination operands is an XMM register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SQRTSS.html).
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
    pub fn sse_sqrtss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseSqrtssEmitter<A, B> {
        <Self as SseSqrtssEmitter<A, B>>::sse_sqrtss(self, op0, op1);
    }
    /// `SSE_SUBPS` (SUBPS). 
    /// Performs a SIMD subtract of the packed single precision floating-point values in the second Source operand from the First Source operand, and stores the packed single precision floating-point results in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SUBPS.html).
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
    pub fn sse_subps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseSubpsEmitter<A, B> {
        <Self as SseSubpsEmitter<A, B>>::sse_subps(self, op0, op1);
    }
    /// `SSE_SUBSS` (SUBSS). 
    /// Subtract the low single precision floating-point value from the second source operand and the first source operand and store the double precision floating-point result in the low doubleword of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SUBSS.html).
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
    pub fn sse_subss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseSubssEmitter<A, B> {
        <Self as SseSubssEmitter<A, B>>::sse_subss(self, op0, op1);
    }
    /// `SSE_UCOMISS` (UCOMISS). 
    /// Compares the single precision floating-point values in the low doublewords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UCOMISS.html).
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
    pub fn sse_ucomiss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseUcomissEmitter<A, B> {
        <Self as SseUcomissEmitter<A, B>>::sse_ucomiss(self, op0, op1);
    }
    /// `SSE_UNPCKHPS` (UNPCKHPS). 
    /// Performs an interleaved unpack of the high single precision floating-point values from the first source operand and the second source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UNPCKHPS.html).
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
    pub fn sse_unpckhps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseUnpckhpsEmitter<A, B> {
        <Self as SseUnpckhpsEmitter<A, B>>::sse_unpckhps(self, op0, op1);
    }
    /// `SSE_UNPCKLPS` (UNPCKLPS). 
    /// Performs an interleaved unpack of the low single precision floating-point values from the first source operand and the second source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UNPCKLPS.html).
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
    pub fn sse_unpcklps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseUnpcklpsEmitter<A, B> {
        <Self as SseUnpcklpsEmitter<A, B>>::sse_unpcklps(self, op0, op1);
    }
    /// `SSE_XORPS` (XORPS). 
    /// Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPS.html).
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
    pub fn sse_xorps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseXorpsEmitter<A, B> {
        <Self as SseXorpsEmitter<A, B>>::sse_xorps(self, op0, op1);
    }
    /// `STMXCSR` (STMXCSR). 
    /// Stores the contents of the MXCSR control and status register to the destination operand. The destination operand is a 32-bit memory location. The reserved bits in the MXCSR register are stored as 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/STMXCSR.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn stmxcsr<A>(&mut self, op0: A)
    where Assembler<'a>: StmxcsrEmitter<A> {
        <Self as StmxcsrEmitter<A>>::stmxcsr(self, op0);
    }
}
