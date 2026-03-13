use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `LFENCE` (LFENCE). 
/// Performs a serializing operation on all load-from-memory instructions that were issued prior the LFENCE instruction. Specifically, LFENCE does not execute until all prior instructions have completed locally, and no later instruction begins execution until LFENCE completes. In particular, an instruction that loads from memory and that precedes an LFENCE receives data from memory prior to completion of the LFENCE. (An LFENCE that follows an instruction that stores to memory might complete before the data being stored have become globally visible.) Instructions following an LFENCE may be fetched from memory before the LFENCE, but they will not execute (even speculatively) until the LFENCE completes.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/LFENCE.html).
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
pub trait LfenceEmitter {
    fn lfence(&mut self);
}

impl<'a> LfenceEmitter for Assembler<'a> {
    fn lfence(&mut self) {
        self.emit(LFENCE, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `MFENCE`.
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
pub trait MfenceEmitter {
    fn mfence(&mut self);
}

impl<'a> MfenceEmitter for Assembler<'a> {
    fn mfence(&mut self) {
        self.emit(MFENCE, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `MMX_CVTPD2PI` (CVTPD2PI). 
/// Converts two packed double precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPD2PI.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Xmm  |
/// +---+----------+
/// ```
pub trait MmxCvtpd2piEmitter<A, B> {
    fn mmx_cvtpd2pi(&mut self, op0: A, op1: B);
}

impl<'a> MmxCvtpd2piEmitter<Mm, Xmm> for Assembler<'a> {
    fn mmx_cvtpd2pi(&mut self, op0: Mm, op1: Xmm) {
        self.emit(MMX_CVTPD2PIRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxCvtpd2piEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_cvtpd2pi(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_CVTPD2PIRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_CVTPI2PD` (CVTPI2PD). 
/// Converts two packed signed doubleword integers in the source operand (second operand) to two packed double precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPI2PD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Mm  |
/// +---+----------+
/// ```
pub trait MmxCvtpi2pdEmitter<A, B> {
    fn mmx_cvtpi2pd(&mut self, op0: A, op1: B);
}

impl<'a> MmxCvtpi2pdEmitter<Xmm, Mm> for Assembler<'a> {
    fn mmx_cvtpi2pd(&mut self, op0: Xmm, op1: Mm) {
        self.emit(MMX_CVTPI2PDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxCvtpi2pdEmitter<Xmm, Mem> for Assembler<'a> {
    fn mmx_cvtpi2pd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(MMX_CVTPI2PDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_CVTPI2PS` (CVTPI2PS). 
/// Converts two packed signed doubleword integers in the source operand (second operand) to two packed single precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPI2PS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Mm  |
/// +---+----------+
/// ```
pub trait MmxCvtpi2psEmitter<A, B> {
    fn mmx_cvtpi2ps(&mut self, op0: A, op1: B);
}

impl<'a> MmxCvtpi2psEmitter<Xmm, Mm> for Assembler<'a> {
    fn mmx_cvtpi2ps(&mut self, op0: Xmm, op1: Mm) {
        self.emit(MMX_CVTPI2PSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxCvtpi2psEmitter<Xmm, Mem> for Assembler<'a> {
    fn mmx_cvtpi2ps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(MMX_CVTPI2PSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_CVTPS2PI` (CVTPS2PI). 
/// Converts two packed single precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPS2PI.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Xmm  |
/// +---+----------+
/// ```
pub trait MmxCvtps2piEmitter<A, B> {
    fn mmx_cvtps2pi(&mut self, op0: A, op1: B);
}

impl<'a> MmxCvtps2piEmitter<Mm, Xmm> for Assembler<'a> {
    fn mmx_cvtps2pi(&mut self, op0: Mm, op1: Xmm) {
        self.emit(MMX_CVTPS2PIRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxCvtps2piEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_cvtps2pi(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_CVTPS2PIRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_CVTTPD2PI` (CVTTPD2PI). 
/// Converts two packed double precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand). The source operand can be an XMM register or a 128-bit memory location. The destination operand is an MMX technology register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTPD2PI.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Xmm  |
/// +---+----------+
/// ```
pub trait MmxCvttpd2piEmitter<A, B> {
    fn mmx_cvttpd2pi(&mut self, op0: A, op1: B);
}

impl<'a> MmxCvttpd2piEmitter<Mm, Xmm> for Assembler<'a> {
    fn mmx_cvttpd2pi(&mut self, op0: Mm, op1: Xmm) {
        self.emit(MMX_CVTTPD2PIRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxCvttpd2piEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_cvttpd2pi(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_CVTTPD2PIRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MMX_CVTTPS2PI` (CVTTPS2PI). 
/// Converts two packed single precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is an MMX technology register. When the source operand is an XMM register, the two single precision floating-point values are contained in the low quadword of the register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTPS2PI.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mm, Mem  |
/// | 2 | Mm, Xmm  |
/// +---+----------+
/// ```
pub trait MmxCvttps2piEmitter<A, B> {
    fn mmx_cvttps2pi(&mut self, op0: A, op1: B);
}

impl<'a> MmxCvttps2piEmitter<Mm, Xmm> for Assembler<'a> {
    fn mmx_cvttps2pi(&mut self, op0: Mm, op1: Xmm) {
        self.emit(MMX_CVTTPS2PIRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MmxCvttps2piEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_cvttps2pi(&mut self, op0: Mm, op1: Mem) {
        self.emit(MMX_CVTTPS2PIRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `MOVNTI` (MOVNTI). 
/// Moves the doubleword integer in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to minimize cache pollution during the write to memory. The source operand is a general-purpose register. The destination operand is a 32-bit memory location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTI.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Gpd |
/// | 2 | Mem, Gpq |
/// +---+----------+
/// ```
pub trait MovntiEmitter<A, B> {
    fn movnti(&mut self, op0: A, op1: B);
}

impl<'a> MovntiEmitter<Mem, Gpd> for Assembler<'a> {
    fn movnti(&mut self, op0: Mem, op1: Gpd) {
        self.emit(MOVNTI32MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MovntiEmitter<Mem, Gpq> for Assembler<'a> {
    fn movnti(&mut self, op0: Mem, op1: Gpq) {
        self.emit(MOVNTI64MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_ADDPD` (ADDPD). 
/// Adds two, four or eight packed double precision floating-point values from the first source operand to the second source operand, and stores the packed double precision floating-point result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDPD.html).
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
pub trait SseAddpdEmitter<A, B> {
    fn sse_addpd(&mut self, op0: A, op1: B);
}

impl<'a> SseAddpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_addpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ADDPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseAddpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_addpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ADDPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_ADDSD` (ADDSD). 
/// Adds the low double precision floating-point values from the second source operand and the first source operand and stores the double precision floating-point result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSD.html).
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
pub trait SseAddsdEmitter<A, B> {
    fn sse_addsd(&mut self, op0: A, op1: B);
}

impl<'a> SseAddsdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_addsd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ADDSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseAddsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_addsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ADDSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_ANDNPD` (ANDNPD). 
/// Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPD.html).
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
pub trait SseAndnpdEmitter<A, B> {
    fn sse_andnpd(&mut self, op0: A, op1: B);
}

impl<'a> SseAndnpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_andnpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ANDNPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseAndnpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_andnpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ANDNPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_ANDPD` (ANDPD). 
/// Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPD.html).
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
pub trait SseAndpdEmitter<A, B> {
    fn sse_andpd(&mut self, op0: A, op1: B);
}

impl<'a> SseAndpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_andpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ANDPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseAndpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_andpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ANDPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CMPPD` (CMPPD). 
/// Performs a SIMD compare of the packed double precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each pair of packed values in the two source operands.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPPD.html).
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
pub trait SseCmppdEmitter<A, B, C> {
    fn sse_cmppd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseCmppdEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_cmppd(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_CMPPDRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseCmppdEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_cmppd(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_CMPPDRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_CMPSD` (CMPSD). 
/// Compares the byte, word, doubleword, or quadword specified with the first source operand with the byte, word, doubleword, or quadword specified with the second source operand and sets the status flags in the EFLAGS register according to the results.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPS%3ACMPSB%3ACMPSW%3ACMPSD%3ACMPSQ.html).
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
pub trait SseCmpsdEmitter<A, B, C> {
    fn sse_cmpsd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseCmpsdEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_cmpsd(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_CMPSDRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseCmpsdEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_cmpsd(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_CMPSDRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_COMISD` (COMISD). 
/// Compares the double precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/COMISD.html).
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
pub trait SseComisdEmitter<A, B> {
    fn sse_comisd(&mut self, op0: A, op1: B);
}

impl<'a> SseComisdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_comisd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_COMISDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseComisdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_comisd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_COMISDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTDQ2PD` (CVTDQ2PD). 
/// Converts two, four or eight packed signed doubleword integers in the source operand (the second operand) to two, four or eight packed double precision floating-point values in the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTDQ2PD.html).
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
pub trait SseCvtdq2pdEmitter<A, B> {
    fn sse_cvtdq2pd(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtdq2pdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_cvtdq2pd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_CVTDQ2PDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtdq2pdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvtdq2pd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTDQ2PDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTDQ2PS` (CVTDQ2PS). 
/// Converts four, eight or sixteen packed signed doubleword integers in the source operand to four, eight or sixteen packed single precision floating-point values in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTDQ2PS.html).
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
pub trait SseCvtdq2psEmitter<A, B> {
    fn sse_cvtdq2ps(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtdq2psEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_cvtdq2ps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_CVTDQ2PSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtdq2psEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvtdq2ps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTDQ2PSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTPD2DQ` (CVTPD2DQ). 
/// Converts packed double precision floating-point values in the source operand (second operand) to packed signed doubleword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPD2DQ.html).
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
pub trait SseCvtpd2dqEmitter<A, B> {
    fn sse_cvtpd2dq(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtpd2dqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_cvtpd2dq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_CVTPD2DQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtpd2dqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvtpd2dq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTPD2DQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTPD2PS` (CVTPD2PS). 
/// Converts two, four or eight packed double precision floating-point values in the source operand (second operand) to two, four or eight packed single precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPD2PS.html).
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
pub trait SseCvtpd2psEmitter<A, B> {
    fn sse_cvtpd2ps(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtpd2psEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_cvtpd2ps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_CVTPD2PSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtpd2psEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvtpd2ps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTPD2PSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTPS2DQ` (CVTPS2DQ). 
/// Converts four, eight or sixteen packed single precision floating-point values in the source operand to four, eight or sixteen signed doubleword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPS2DQ.html).
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
pub trait SseCvtps2dqEmitter<A, B> {
    fn sse_cvtps2dq(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtps2dqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_cvtps2dq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_CVTPS2DQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtps2dqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvtps2dq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTPS2DQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTPS2PD` (CVTPS2PD). 
/// Converts two, four or eight packed single precision floating-point values in the source operand (second operand) to two, four or eight packed double precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPS2PD.html).
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
pub trait SseCvtps2pdEmitter<A, B> {
    fn sse_cvtps2pd(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtps2pdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_cvtps2pd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_CVTPS2PDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtps2pdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvtps2pd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTPS2PDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTSD2SI` (CVTSD2SI). 
/// Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer in the destination operand (first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSD2SI.html).
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
pub trait SseCvtsd2siEmitter<A, B> {
    fn sse_cvtsd2si(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtsd2siEmitter<Gpd, Xmm> for Assembler<'a> {
    fn sse_cvtsd2si(&mut self, op0: Gpd, op1: Xmm) {
        self.emit(SSE_CVTSD2SI32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtsd2siEmitter<Gpd, Mem> for Assembler<'a> {
    fn sse_cvtsd2si(&mut self, op0: Gpd, op1: Mem) {
        self.emit(SSE_CVTSD2SI32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtsd2siEmitter<Gpq, Xmm> for Assembler<'a> {
    fn sse_cvtsd2si(&mut self, op0: Gpq, op1: Xmm) {
        self.emit(SSE_CVTSD2SI64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtsd2siEmitter<Gpq, Mem> for Assembler<'a> {
    fn sse_cvtsd2si(&mut self, op0: Gpq, op1: Mem) {
        self.emit(SSE_CVTSD2SI64RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTSD2SS` (CVTSD2SS). 
/// Converts a double precision floating-point value in the “convert-from” source operand (the second operand in SSE2 version, otherwise the third operand) to a single precision floating-point value in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSD2SS.html).
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
pub trait SseCvtsd2ssEmitter<A, B> {
    fn sse_cvtsd2ss(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtsd2ssEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_cvtsd2ss(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_CVTSD2SSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtsd2ssEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvtsd2ss(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTSD2SSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTSI2SD` (CVTSI2SD). 
/// Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a double precision floating-point value in the destination operand. The result is stored in the low quadword of the destination operand, and the high quadword left unchanged. When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSI2SD.html).
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
pub trait SseCvtsi2sdEmitter<A, B> {
    fn sse_cvtsi2sd(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtsi2sdEmitter<Xmm, Gpd> for Assembler<'a> {
    fn sse_cvtsi2sd(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(SSE_CVTSI2SD32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtsi2sdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvtsi2sd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTSI2SD32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtsi2sdEmitter<Xmm, Gpq> for Assembler<'a> {
    fn sse_cvtsi2sd(&mut self, op0: Xmm, op1: Gpq) {
        self.emit(SSE_CVTSI2SD64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTSS2SD` (CVTSS2SD). 
/// Converts a single precision floating-point value in the “convert-from” source operand to a double precision floating-point value in the destination operand. When the “convert-from” source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register. The result is stored in the low quadword of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSS2SD.html).
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
pub trait SseCvtss2sdEmitter<A, B> {
    fn sse_cvtss2sd(&mut self, op0: A, op1: B);
}

impl<'a> SseCvtss2sdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_cvtss2sd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_CVTSS2SDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvtss2sdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvtss2sd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTSS2SDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTTPD2DQ` (CVTTPD2DQ). 
/// Converts two, four or eight packed double precision floating-point values in the source operand (second operand) to two, four or eight packed signed doubleword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTPD2DQ.html).
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
pub trait SseCvttpd2dqEmitter<A, B> {
    fn sse_cvttpd2dq(&mut self, op0: A, op1: B);
}

impl<'a> SseCvttpd2dqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_cvttpd2dq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_CVTTPD2DQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvttpd2dqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvttpd2dq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTTPD2DQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTTPS2DQ` (CVTTPS2DQ). 
/// Converts four, eight or sixteen packed single precision floating-point values in the source operand to four, eight or sixteen signed doubleword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTPS2DQ.html).
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
pub trait SseCvttps2dqEmitter<A, B> {
    fn sse_cvttps2dq(&mut self, op0: A, op1: B);
}

impl<'a> SseCvttps2dqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_cvttps2dq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_CVTTPS2DQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvttps2dqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_cvttps2dq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_CVTTPS2DQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_CVTTSD2SI` (CVTTSD2SI). 
/// Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTSD2SI.html).
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
pub trait SseCvttsd2siEmitter<A, B> {
    fn sse_cvttsd2si(&mut self, op0: A, op1: B);
}

impl<'a> SseCvttsd2siEmitter<Gpd, Xmm> for Assembler<'a> {
    fn sse_cvttsd2si(&mut self, op0: Gpd, op1: Xmm) {
        self.emit(SSE_CVTTSD2SI32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvttsd2siEmitter<Gpd, Mem> for Assembler<'a> {
    fn sse_cvttsd2si(&mut self, op0: Gpd, op1: Mem) {
        self.emit(SSE_CVTTSD2SI32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvttsd2siEmitter<Gpq, Xmm> for Assembler<'a> {
    fn sse_cvttsd2si(&mut self, op0: Gpq, op1: Xmm) {
        self.emit(SSE_CVTTSD2SI64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseCvttsd2siEmitter<Gpq, Mem> for Assembler<'a> {
    fn sse_cvttsd2si(&mut self, op0: Gpq, op1: Mem) {
        self.emit(SSE_CVTTSD2SI64RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_DIVPD` (DIVPD). 
/// Performs a SIMD divide of the double precision floating-point values in the first source operand by the floating-point values in the second source operand (the third operand). Results are written to the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DIVPD.html).
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
pub trait SseDivpdEmitter<A, B> {
    fn sse_divpd(&mut self, op0: A, op1: B);
}

impl<'a> SseDivpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_divpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_DIVPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseDivpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_divpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_DIVPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_DIVSD` (DIVSD). 
/// Divides the low double precision floating-point value in the first source operand by the low double precision floating-point value in the second source operand, and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination are XMM registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DIVSD.html).
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
pub trait SseDivsdEmitter<A, B> {
    fn sse_divsd(&mut self, op0: A, op1: B);
}

impl<'a> SseDivsdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_divsd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_DIVSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseDivsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_divsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_DIVSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MASKMOVDQU` (MASKMOVDQU). 
/// Stores selected bytes from the source operand (first operand) into an 128-bit memory location. The mask operand (second operand) selects which bytes from the source operand are written to memory. The source and mask operands are XMM registers. The memory location specified by the effective address in the DI/EDI/RDI register (the default segment register is DS, but this may be overridden with a segment-override prefix). The memory location does not need to be aligned on a natural boundary. (The size of the store address depends on the address-size attribute.)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MASKMOVDQU.html).
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
pub trait SseMaskmovdquEmitter<A, B> {
    fn sse_maskmovdqu(&mut self, op0: A, op1: B);
}

impl<'a> SseMaskmovdquEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_maskmovdqu(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MASKMOVDQURR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MAXPD` (MAXPD). 
/// Performs a SIMD compare of the packed double precision floating-point values in the first source operand and the second source operand and returns the maximum value for each pair of values to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MAXPD.html).
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
pub trait SseMaxpdEmitter<A, B> {
    fn sse_maxpd(&mut self, op0: A, op1: B);
}

impl<'a> SseMaxpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_maxpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MAXPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMaxpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_maxpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MAXPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MAXSD` (MAXSD). 
/// Compares the low double precision floating-point values in the first source operand and the second source operand, and returns the maximum value to the low quadword of the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination operands are XMM registers. When the second source operand is a memory operand, only 64 bits are accessed.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MAXSD.html).
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
pub trait SseMaxsdEmitter<A, B> {
    fn sse_maxsd(&mut self, op0: A, op1: B);
}

impl<'a> SseMaxsdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_maxsd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MAXSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMaxsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_maxsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MAXSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MINPD` (MINPD). 
/// Performs a SIMD compare of the packed double precision floating-point values in the first source operand and the second source operand and returns the minimum value for each pair of values to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MINPD.html).
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
pub trait SseMinpdEmitter<A, B> {
    fn sse_minpd(&mut self, op0: A, op1: B);
}

impl<'a> SseMinpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_minpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MINPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMinpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_minpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MINPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MINSD` (MINSD). 
/// Compares the low double precision floating-point values in the first source operand and the second source operand, and returns the minimum value to the low quadword of the destination operand. When the source operand is a memory operand, only the 64 bits are accessed.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MINSD.html).
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
pub trait SseMinsdEmitter<A, B> {
    fn sse_minsd(&mut self, op0: A, op1: B);
}

impl<'a> SseMinsdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_minsd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MINSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMinsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_minsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MINSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVAPD` (MOVAPD). 
/// Moves 2, 4 or 8 double precision floating-point values from the source operand (second operand) to the destination operand (first operand). This instruction can be used to load an XMM, YMM or ZMM register from an 128-bit, 256-bit or 512-bit memory location, to store the contents of an XMM, YMM or ZMM register into a 128-bit, 256-bit or 512-bit memory location, or to move data between two XMM, two YMM or two ZMM registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVAPD.html).
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
pub trait SseMovapdEmitter<A, B> {
    fn sse_movapd(&mut self, op0: A, op1: B);
}

impl<'a> SseMovapdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movapd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVAPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovapdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movapd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVAPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovapdEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movapd(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVAPDMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVDQA` (MOVDQA). 
/// Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html).
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
pub trait SseMovdqaEmitter<A, B> {
    fn sse_movdqa(&mut self, op0: A, op1: B);
}

impl<'a> SseMovdqaEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movdqa(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVDQARR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovdqaEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movdqa(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVDQARM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovdqaEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movdqa(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVDQAMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVDQU` (MOVDQU). 
/// Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html).
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
pub trait SseMovdquEmitter<A, B> {
    fn sse_movdqu(&mut self, op0: A, op1: B);
}

impl<'a> SseMovdquEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movdqu(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVDQURR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovdquEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movdqu(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVDQURM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovdquEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movdqu(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVDQUMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVD_G2X` (MOVD). 
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
/// | 1 | Xmm, Gpd |
/// | 2 | Xmm, Mem |
/// +---+----------+
/// ```
pub trait SseMovdG2xEmitter<A, B> {
    fn sse_movd_g2x(&mut self, op0: A, op1: B);
}

impl<'a> SseMovdG2xEmitter<Xmm, Gpd> for Assembler<'a> {
    fn sse_movd_g2x(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(SSE_MOVD_G2XRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovdG2xEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movd_g2x(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVD_G2XRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVD_X2G` (MOVD). 
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
/// | 1 | Gpd, Xmm |
/// | 2 | Mem, Xmm |
/// +---+----------+
/// ```
pub trait SseMovdX2gEmitter<A, B> {
    fn sse_movd_x2g(&mut self, op0: A, op1: B);
}

impl<'a> SseMovdX2gEmitter<Gpd, Xmm> for Assembler<'a> {
    fn sse_movd_x2g(&mut self, op0: Gpd, op1: Xmm) {
        self.emit(SSE_MOVD_X2GRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovdX2gEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movd_x2g(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVD_X2GMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVHPD` (MOVHPD). 
/// This instruction cannot be used for register to register or memory to memory moves.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVHPD.html).
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
pub trait SseMovhpdEmitter<A, B> {
    fn sse_movhpd(&mut self, op0: A, op1: B);
}

impl<'a> SseMovhpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movhpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVHPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovhpdEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movhpd(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVHPDMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVLPD` (MOVLPD). 
/// This instruction cannot be used for register to register or memory to memory moves.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVLPD.html).
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
pub trait SseMovlpdEmitter<A, B> {
    fn sse_movlpd(&mut self, op0: A, op1: B);
}

impl<'a> SseMovlpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movlpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVLPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovlpdEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movlpd(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVLPDMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVMSKPD` (MOVMSKPD). 
/// Extracts the sign bits from the packed double precision floating-point values in the source operand (second operand), formats them into a 2-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM register, and the destination operand is a general-purpose register. The mask is stored in the 2 low-order bits of the destination operand. Zero-extend the upper bits of the destination.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVMSKPD.html).
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
pub trait SseMovmskpdEmitter<A, B> {
    fn sse_movmskpd(&mut self, op0: A, op1: B);
}

impl<'a> SseMovmskpdEmitter<Gpq, Xmm> for Assembler<'a> {
    fn sse_movmskpd(&mut self, op0: Gpq, op1: Xmm) {
        self.emit(SSE_MOVMSKPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVNTDQ` (MOVNTDQ). 
/// Moves the packed integers in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to prevent caching of the data during the write to memory. The source operand is an XMM register, YMM register or ZMM register, which is assumed to contain integer data (packed bytes, words, double-words, or quadwords). The destination operand is a 128-bit, 256-bit or 512-bit memory location. The memory operand must be aligned on a 16-byte (128-bit version), 32-byte (VEX.256 encoded version) or 64-byte (512-bit version) boundary otherwise a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTDQ.html).
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
pub trait SseMovntdqEmitter<A, B> {
    fn sse_movntdq(&mut self, op0: A, op1: B);
}

impl<'a> SseMovntdqEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movntdq(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVNTDQMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVNTPD` (MOVNTPD). 
/// Moves the packed double precision floating-point values in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to prevent caching of the data during the write to memory. The source operand is an XMM register, YMM register or ZMM register, which is assumed to contain packed double precision, floating-pointing data. The destination operand is a 128-bit, 256-bit or 512-bit memory location. The memory operand must be aligned on a 16-byte (128-bit version), 32-byte (VEX.256 encoded version) or 64-byte (EVEX.512 encoded version) boundary otherwise a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTPD.html).
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
pub trait SseMovntpdEmitter<A, B> {
    fn sse_movntpd(&mut self, op0: A, op1: B);
}

impl<'a> SseMovntpdEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movntpd(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVNTPDMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVNTSD`.
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
pub trait SseMovntsdEmitter<A, B> {
    fn sse_movntsd(&mut self, op0: A, op1: B);
}

impl<'a> SseMovntsdEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movntsd(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVNTSDMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVQ` (MOVQ). 
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
/// | 1 | Mem, Xmm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovqEmitter<A, B> {
    fn sse_movq(&mut self, op0: A, op1: B);
}

impl<'a> SseMovqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovqEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movq(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVQMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVQ_G2X` (MOVQ). 
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
/// | 1 | Xmm, Gpd |
/// | 2 | Xmm, Mem |
/// +---+----------+
/// ```
pub trait SseMovqG2xEmitter<A, B> {
    fn sse_movq_g2x(&mut self, op0: A, op1: B);
}

impl<'a> SseMovqG2xEmitter<Xmm, Gpd> for Assembler<'a> {
    fn sse_movq_g2x(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(SSE_MOVQ_G2XRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovqG2xEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movq_g2x(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVQ_G2XRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVQ_X2G` (MOVQ). 
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
/// | 1 | Gpd, Xmm |
/// | 2 | Mem, Xmm |
/// +---+----------+
/// ```
pub trait SseMovqX2gEmitter<A, B> {
    fn sse_movq_x2g(&mut self, op0: A, op1: B);
}

impl<'a> SseMovqX2gEmitter<Gpd, Xmm> for Assembler<'a> {
    fn sse_movq_x2g(&mut self, op0: Gpd, op1: Xmm) {
        self.emit(SSE_MOVQ_X2GRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovqX2gEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movq_x2g(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVQ_X2GMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVSD` (MOVSD). 
/// Moves the byte, word, or doubleword specified with the second operand (source operand) to the location specified with the first operand (destination operand). Both the source and destination operands are located in memory. The address of the source operand is read from the DS:ESI or the DS:SI registers (depending on the address-size attribute of the instruction, 32 or 16, respectively). The address of the destination operand is read from the ES:EDI or the ES:DI registers (again depending on the address-size attribute of the instruction). The DS segment may be overridden with a segment override prefix, but the ES segment cannot be overridden.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVS%3AMOVSB%3AMOVSW%3AMOVSD%3AMOVSQ.html).
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
pub trait SseMovsdEmitter<A, B> {
    fn sse_movsd(&mut self, op0: A, op1: B);
}

impl<'a> SseMovsdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movsd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovsdEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movsd(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVSDMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVUPD` (MOVUPD). 
/// Note: VEX.vvvv and EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVUPD.html).
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
pub trait SseMovupdEmitter<A, B> {
    fn sse_movupd(&mut self, op0: A, op1: B);
}

impl<'a> SseMovupdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movupd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVUPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovupdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movupd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVUPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovupdEmitter<Mem, Xmm> for Assembler<'a> {
    fn sse_movupd(&mut self, op0: Mem, op1: Xmm) {
        self.emit(SSE_MOVUPDMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MULPD` (MULPD). 
/// Multiply packed double precision floating-point values from the first source operand with corresponding values in the second source operand, and stores the packed double precision floating-point results in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MULPD.html).
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
pub trait SseMulpdEmitter<A, B> {
    fn sse_mulpd(&mut self, op0: A, op1: B);
}

impl<'a> SseMulpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_mulpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MULPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMulpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_mulpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MULPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MULSD` (MULSD). 
/// Multiplies the low double precision floating-point value in the second source operand by the low double precision floating-point value in the first source operand, and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source operand and the destination operands are XMM registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MULSD.html).
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
pub trait SseMulsdEmitter<A, B> {
    fn sse_mulsd(&mut self, op0: A, op1: B);
}

impl<'a> SseMulsdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_mulsd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MULSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMulsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_mulsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MULSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_ORPD` (ORPD). 
/// Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPD.html).
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
pub trait SseOrpdEmitter<A, B> {
    fn sse_orpd(&mut self, op0: A, op1: B);
}

impl<'a> SseOrpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_orpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ORPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseOrpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_orpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ORPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PACKSSDW` (PACKSSDW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePackssdwEmitter<A, B> {
    fn sse_packssdw(&mut self, op0: A, op1: B);
}

impl<'a> SsePackssdwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_packssdw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PACKSSDWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePackssdwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_packssdw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PACKSSDWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PACKSSWB` (PACKSSWB). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePacksswbEmitter<A, B> {
    fn sse_packsswb(&mut self, op0: A, op1: B);
}

impl<'a> SsePacksswbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_packsswb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PACKSSWBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePacksswbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_packsswb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PACKSSWBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PACKUSWB` (PACKUSWB). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePackuswbEmitter<A, B> {
    fn sse_packuswb(&mut self, op0: A, op1: B);
}

impl<'a> SsePackuswbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_packuswb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PACKUSWBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePackuswbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_packuswb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PACKUSWBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PADDB` (PADDB). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePaddbEmitter<A, B> {
    fn sse_paddb(&mut self, op0: A, op1: B);
}

impl<'a> SsePaddbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_paddb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PADDBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePaddbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_paddb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PADDBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PADDD` (PADDD). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePadddEmitter<A, B> {
    fn sse_paddd(&mut self, op0: A, op1: B);
}

impl<'a> SsePadddEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_paddd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PADDDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePadddEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_paddd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PADDDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PADDQ` (PADDQ). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePaddqEmitter<A, B> {
    fn sse_paddq(&mut self, op0: A, op1: B);
}

impl<'a> SsePaddqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_paddq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PADDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePaddqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_paddq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PADDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PADDSB` (PADDSB). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePaddsbEmitter<A, B> {
    fn sse_paddsb(&mut self, op0: A, op1: B);
}

impl<'a> SsePaddsbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_paddsb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PADDSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePaddsbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_paddsb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PADDSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PADDSW` (PADDSW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePaddswEmitter<A, B> {
    fn sse_paddsw(&mut self, op0: A, op1: B);
}

impl<'a> SsePaddswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_paddsw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PADDSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePaddswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_paddsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PADDSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PADDUSB` (PADDUSB). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePaddusbEmitter<A, B> {
    fn sse_paddusb(&mut self, op0: A, op1: B);
}

impl<'a> SsePaddusbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_paddusb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PADDUSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePaddusbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_paddusb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PADDUSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PADDUSW` (PADDUSW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePadduswEmitter<A, B> {
    fn sse_paddusw(&mut self, op0: A, op1: B);
}

impl<'a> SsePadduswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_paddusw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PADDUSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePadduswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_paddusw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PADDUSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PADDW` (PADDW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePaddwEmitter<A, B> {
    fn sse_paddw(&mut self, op0: A, op1: B);
}

impl<'a> SsePaddwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_paddw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PADDWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePaddwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_paddw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PADDWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PAND` (PAND). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePandEmitter<A, B> {
    fn sse_pand(&mut self, op0: A, op1: B);
}

impl<'a> SsePandEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pand(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PANDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePandEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pand(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PANDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PANDN` (PANDN). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePandnEmitter<A, B> {
    fn sse_pandn(&mut self, op0: A, op1: B);
}

impl<'a> SsePandnEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pandn(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PANDNRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePandnEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pandn(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PANDNRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PAVGB` (PAVGB). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePavgbEmitter<A, B> {
    fn sse_pavgb(&mut self, op0: A, op1: B);
}

impl<'a> SsePavgbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pavgb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PAVGBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePavgbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pavgb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PAVGBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PAVGW` (PAVGW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePavgwEmitter<A, B> {
    fn sse_pavgw(&mut self, op0: A, op1: B);
}

impl<'a> SsePavgwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pavgw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PAVGWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePavgwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pavgw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PAVGWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PCMPEQB` (PCMPEQB). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePcmpeqbEmitter<A, B> {
    fn sse_pcmpeqb(&mut self, op0: A, op1: B);
}

impl<'a> SsePcmpeqbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pcmpeqb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PCMPEQBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePcmpeqbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pcmpeqb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PCMPEQBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PCMPEQD` (PCMPEQD). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePcmpeqdEmitter<A, B> {
    fn sse_pcmpeqd(&mut self, op0: A, op1: B);
}

impl<'a> SsePcmpeqdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pcmpeqd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PCMPEQDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePcmpeqdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pcmpeqd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PCMPEQDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PCMPEQW` (PCMPEQW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePcmpeqwEmitter<A, B> {
    fn sse_pcmpeqw(&mut self, op0: A, op1: B);
}

impl<'a> SsePcmpeqwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pcmpeqw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PCMPEQWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePcmpeqwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pcmpeqw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PCMPEQWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PCMPGTB` (PCMPGTB). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePcmpgtbEmitter<A, B> {
    fn sse_pcmpgtb(&mut self, op0: A, op1: B);
}

impl<'a> SsePcmpgtbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pcmpgtb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PCMPGTBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePcmpgtbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pcmpgtb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PCMPGTBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PCMPGTD` (PCMPGTD). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePcmpgtdEmitter<A, B> {
    fn sse_pcmpgtd(&mut self, op0: A, op1: B);
}

impl<'a> SsePcmpgtdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pcmpgtd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PCMPGTDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePcmpgtdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pcmpgtd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PCMPGTDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PCMPGTW` (PCMPGTW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePcmpgtwEmitter<A, B> {
    fn sse_pcmpgtw(&mut self, op0: A, op1: B);
}

impl<'a> SsePcmpgtwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pcmpgtw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PCMPGTWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePcmpgtwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pcmpgtw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PCMPGTWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PEXTRW` (PEXTRW). 
/// Copies the word in the source operand (second operand) specified by the count operand (third operand) to the destination operand (first operand). The source operand can be an MMX technology register or an XMM register. The destination operand can be the low word of a general-purpose register or a 16-bit memory address. The count operand is an 8-bit immediate. When specifying a word location in an MMX technology register, the 2 least-significant bits of the count operand specify the location; for an XMM register, the 3 least-significant bits specify the location. The content of the destination register above bit 16 is cleared (set to all 0s).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRW.html).
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
pub trait SsePextrwEmitter<A, B, C> {
    fn sse_pextrw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePextrwEmitter<Gpd, Xmm, Imm> for Assembler<'a> {
    fn sse_pextrw(&mut self, op0: Gpd, op1: Xmm, op2: Imm) {
        self.emit(SSE_PEXTRWRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePextrwEmitter<Mem, Xmm, Imm> for Assembler<'a> {
    fn sse_pextrw(&mut self, op0: Mem, op1: Xmm, op2: Imm) {
        self.emit(SSE_PEXTRWMRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PINSRW` (PINSRW). 
/// Three operand MMX and SSE instructions
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRW.html).
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
pub trait SsePinsrwEmitter<A, B, C> {
    fn sse_pinsrw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePinsrwEmitter<Xmm, Gpd, Imm> for Assembler<'a> {
    fn sse_pinsrw(&mut self, op0: Xmm, op1: Gpd, op2: Imm) {
        self.emit(SSE_PINSRWRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePinsrwEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pinsrw(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PINSRWRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PMADDWD` (PMADDWD). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePmaddwdEmitter<A, B> {
    fn sse_pmaddwd(&mut self, op0: A, op1: B);
}

impl<'a> SsePmaddwdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmaddwd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMADDWDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmaddwdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmaddwd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMADDWDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMAXSW` (PMAXSW). 
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
pub trait SsePmaxswEmitter<A, B> {
    fn sse_pmaxsw(&mut self, op0: A, op1: B);
}

impl<'a> SsePmaxswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmaxsw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMAXSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmaxswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmaxsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMAXSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMAXUB` (PMAXUB). 
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
pub trait SsePmaxubEmitter<A, B> {
    fn sse_pmaxub(&mut self, op0: A, op1: B);
}

impl<'a> SsePmaxubEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmaxub(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMAXUBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmaxubEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmaxub(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMAXUBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMINSW` (PMINSW). 
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
pub trait SsePminswEmitter<A, B> {
    fn sse_pminsw(&mut self, op0: A, op1: B);
}

impl<'a> SsePminswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pminsw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMINSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePminswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pminsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMINSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMINUB` (PMINUB). 
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
pub trait SsePminubEmitter<A, B> {
    fn sse_pminub(&mut self, op0: A, op1: B);
}

impl<'a> SsePminubEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pminub(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMINUBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePminubEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pminub(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMINUBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMOVMSKB` (PMOVMSKB). 
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
/// | 1 | Gpq, Xmm |
/// +---+----------+
/// ```
pub trait SsePmovmskbEmitter<A, B> {
    fn sse_pmovmskb(&mut self, op0: A, op1: B);
}

impl<'a> SsePmovmskbEmitter<Gpq, Xmm> for Assembler<'a> {
    fn sse_pmovmskb(&mut self, op0: Gpq, op1: Xmm) {
        self.emit(SSE_PMOVMSKBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMULHUW` (PMULHUW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePmulhuwEmitter<A, B> {
    fn sse_pmulhuw(&mut self, op0: A, op1: B);
}

impl<'a> SsePmulhuwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmulhuw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMULHUWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmulhuwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmulhuw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMULHUWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMULHW` (PMULHW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePmulhwEmitter<A, B> {
    fn sse_pmulhw(&mut self, op0: A, op1: B);
}

impl<'a> SsePmulhwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmulhw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMULHWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmulhwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmulhw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMULHWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMULLW` (PMULLW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePmullwEmitter<A, B> {
    fn sse_pmullw(&mut self, op0: A, op1: B);
}

impl<'a> SsePmullwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmullw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMULLWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmullwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmullw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMULLWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PMULUDQ` (PMULUDQ). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePmuludqEmitter<A, B> {
    fn sse_pmuludq(&mut self, op0: A, op1: B);
}

impl<'a> SsePmuludqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pmuludq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PMULUDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePmuludqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmuludq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PMULUDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_POR` (POR). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePorEmitter<A, B> {
    fn sse_por(&mut self, op0: A, op1: B);
}

impl<'a> SsePorEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_por(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PORRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePorEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_por(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PORRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSADBW` (PSADBW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsadbwEmitter<A, B> {
    fn sse_psadbw(&mut self, op0: A, op1: B);
}

impl<'a> SsePsadbwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psadbw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSADBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsadbwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psadbw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSADBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSHUFD` (PSHUFD). 
/// Copies doublewords from source operand (second operand) and inserts them in the destination operand (first operand) at the locations selected with the order operand (third operand). Figure 4-16 shows the operation of the 256-bit VPSHUFD instruction and the encoding of the order operand. Each 2-bit field in the order operand selects the contents of one doubleword location within a 128-bit lane and copy to the target element in the destination operand. For example, bits 0 and 1 of the order operand targets the first doubleword element in the low and high 128-bit lane of the destination operand for 256-bit VPSHUFD. The encoded value of bits 1:0 of the order operand (see the field encoding in Figure 4-16) determines which doubleword element (from the respective 128-bit lane) of the source operand will be copied to doubleword 0 of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFD.html).
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
pub trait SsePshufdEmitter<A, B, C> {
    fn sse_pshufd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePshufdEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_pshufd(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_PSHUFDRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePshufdEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pshufd(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PSHUFDRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PSHUFHW` (PSHUFHW). 
/// Copies words from the high quadword of a 128-bit lane of the source operand and inserts them in the high quadword of the destination operand at word locations (of the respective lane) selected with the immediate operand. This 256-bit operation is similar to the in-lane operation used by the 256-bit VPSHUFD instruction, which is illustrated in Figure 4-16. For 128-bit operation, only the low 128-bit lane is operative. Each 2-bit field in the immediate operand selects the contents of one word location in the high quadword of the destination operand. The binary encodings of the immediate operand fields select words (0, 1, 2 or 3, 4) from the high quadword of the source operand to be copied to the destination operand. The low quadword of the source operand is copied to the low quadword of the destination operand, for each 128-bit lane.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFHW.html).
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
pub trait SsePshufhwEmitter<A, B, C> {
    fn sse_pshufhw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePshufhwEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_pshufhw(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_PSHUFHWRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePshufhwEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pshufhw(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PSHUFHWRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PSHUFLW` (PSHUFLW). 
/// Copies words from the low quadword of a 128-bit lane of the source operand and inserts them in the low quadword of the destination operand at word locations (of the respective lane) selected with the immediate operand. The 256-bit operation is similar to the in-lane operation used by the 256-bit VPSHUFD instruction, which is illustrated in Figure 4-16. For 128-bit operation, only the low 128-bit lane is operative. Each 2-bit field in the immediate operand selects the contents of one word location in the low quadword of the destination operand. The binary encodings of the immediate operand fields select words (0, 1, 2 or 3) from the low quadword of the source operand to be copied to the destination operand. The high quadword of the source operand is copied to the high quadword of the destination operand, for each 128-bit lane.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFLW.html).
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
pub trait SsePshuflwEmitter<A, B, C> {
    fn sse_pshuflw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SsePshuflwEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_pshuflw(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_PSHUFLWRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SsePshuflwEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_pshuflw(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_PSHUFLWRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_PSLLD` (PSLLD). 
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
/// | 1 | Xmm, Imm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePslldEmitter<A, B> {
    fn sse_pslld(&mut self, op0: A, op1: B);
}

impl<'a> SsePslldEmitter<Xmm, Imm> for Assembler<'a> {
    fn sse_pslld(&mut self, op0: Xmm, op1: Imm) {
        self.emit(SSE_PSLLDRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePslldEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pslld(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSLLDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePslldEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pslld(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSLLDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSLLDQ` (PSLLDQ). 
/// Shifts the destination operand (first operand) to the left by the number of bytes specified in the count operand (second operand). The empty low-order bytes are cleared (set to all 0s). If the value specified by the count operand is greater than 15, the destination operand is set to all 0s. The count operand is an 8-bit immediate.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSLLDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Imm |
/// +---+----------+
/// ```
pub trait SsePslldqEmitter<A, B> {
    fn sse_pslldq(&mut self, op0: A, op1: B);
}

impl<'a> SsePslldqEmitter<Xmm, Imm> for Assembler<'a> {
    fn sse_pslldq(&mut self, op0: Xmm, op1: Imm) {
        self.emit(SSE_PSLLDQRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSLLQ` (PSLLQ). 
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
/// | 1 | Xmm, Imm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsllqEmitter<A, B> {
    fn sse_psllq(&mut self, op0: A, op1: B);
}

impl<'a> SsePsllqEmitter<Xmm, Imm> for Assembler<'a> {
    fn sse_psllq(&mut self, op0: Xmm, op1: Imm) {
        self.emit(SSE_PSLLQRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsllqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psllq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSLLQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsllqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psllq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSLLQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSLLW` (PSLLW). 
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
/// | 1 | Xmm, Imm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsllwEmitter<A, B> {
    fn sse_psllw(&mut self, op0: A, op1: B);
}

impl<'a> SsePsllwEmitter<Xmm, Imm> for Assembler<'a> {
    fn sse_psllw(&mut self, op0: Xmm, op1: Imm) {
        self.emit(SSE_PSLLWRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsllwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psllw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSLLWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsllwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psllw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSLLWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSRAD` (PSRAD). 
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
/// | 1 | Xmm, Imm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsradEmitter<A, B> {
    fn sse_psrad(&mut self, op0: A, op1: B);
}

impl<'a> SsePsradEmitter<Xmm, Imm> for Assembler<'a> {
    fn sse_psrad(&mut self, op0: Xmm, op1: Imm) {
        self.emit(SSE_PSRADRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsradEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psrad(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSRADRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsradEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psrad(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSRADRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSRAW` (PSRAW). 
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
/// | 1 | Xmm, Imm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsrawEmitter<A, B> {
    fn sse_psraw(&mut self, op0: A, op1: B);
}

impl<'a> SsePsrawEmitter<Xmm, Imm> for Assembler<'a> {
    fn sse_psraw(&mut self, op0: Xmm, op1: Imm) {
        self.emit(SSE_PSRAWRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsrawEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psraw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSRAWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsrawEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psraw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSRAWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSRLD` (PSRLD). 
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
/// | 1 | Xmm, Imm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsrldEmitter<A, B> {
    fn sse_psrld(&mut self, op0: A, op1: B);
}

impl<'a> SsePsrldEmitter<Xmm, Imm> for Assembler<'a> {
    fn sse_psrld(&mut self, op0: Xmm, op1: Imm) {
        self.emit(SSE_PSRLDRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsrldEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psrld(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSRLDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsrldEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psrld(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSRLDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSRLDQ` (PSRLDQ). 
/// Shifts the destination operand (first operand) to the right by the number of bytes specified in the count operand (second operand). The empty high-order bytes are cleared (set to all 0s). If the value specified by the count operand is greater than 15, the destination operand is set to all 0s. The count operand is an 8-bit immediate.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRLDQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Imm |
/// +---+----------+
/// ```
pub trait SsePsrldqEmitter<A, B> {
    fn sse_psrldq(&mut self, op0: A, op1: B);
}

impl<'a> SsePsrldqEmitter<Xmm, Imm> for Assembler<'a> {
    fn sse_psrldq(&mut self, op0: Xmm, op1: Imm) {
        self.emit(SSE_PSRLDQRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSRLQ` (PSRLQ). 
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
/// | 1 | Xmm, Imm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsrlqEmitter<A, B> {
    fn sse_psrlq(&mut self, op0: A, op1: B);
}

impl<'a> SsePsrlqEmitter<Xmm, Imm> for Assembler<'a> {
    fn sse_psrlq(&mut self, op0: Xmm, op1: Imm) {
        self.emit(SSE_PSRLQRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsrlqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psrlq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSRLQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsrlqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psrlq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSRLQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSRLW` (PSRLW). 
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
/// | 1 | Xmm, Imm |
/// | 2 | Xmm, Mem |
/// | 3 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsrlwEmitter<A, B> {
    fn sse_psrlw(&mut self, op0: A, op1: B);
}

impl<'a> SsePsrlwEmitter<Xmm, Imm> for Assembler<'a> {
    fn sse_psrlw(&mut self, op0: Xmm, op1: Imm) {
        self.emit(SSE_PSRLWRI, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsrlwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psrlw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSRLWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsrlwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psrlw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSRLWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSUBB` (PSUBB). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsubbEmitter<A, B> {
    fn sse_psubb(&mut self, op0: A, op1: B);
}

impl<'a> SsePsubbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psubb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSUBBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsubbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psubb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSUBBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSUBD` (PSUBD). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsubdEmitter<A, B> {
    fn sse_psubd(&mut self, op0: A, op1: B);
}

impl<'a> SsePsubdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psubd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSUBDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsubdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psubd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSUBDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSUBQ` (PSUBQ). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsubqEmitter<A, B> {
    fn sse_psubq(&mut self, op0: A, op1: B);
}

impl<'a> SsePsubqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psubq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSUBQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsubqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psubq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSUBQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSUBSB` (PSUBSB). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsubsbEmitter<A, B> {
    fn sse_psubsb(&mut self, op0: A, op1: B);
}

impl<'a> SsePsubsbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psubsb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSUBSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsubsbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psubsb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSUBSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSUBSW` (PSUBSW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsubswEmitter<A, B> {
    fn sse_psubsw(&mut self, op0: A, op1: B);
}

impl<'a> SsePsubswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psubsw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSUBSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsubswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psubsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSUBSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSUBUSB` (PSUBUSB). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsubusbEmitter<A, B> {
    fn sse_psubusb(&mut self, op0: A, op1: B);
}

impl<'a> SsePsubusbEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psubusb(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSUBUSBRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsubusbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psubusb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSUBUSBRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSUBUSW` (PSUBUSW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsubuswEmitter<A, B> {
    fn sse_psubusw(&mut self, op0: A, op1: B);
}

impl<'a> SsePsubuswEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psubusw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSUBUSWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsubuswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psubusw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSUBUSWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PSUBW` (PSUBW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePsubwEmitter<A, B> {
    fn sse_psubw(&mut self, op0: A, op1: B);
}

impl<'a> SsePsubwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_psubw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PSUBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePsubwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psubw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PSUBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PUNPCKHBW` (PUNPCKHBW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePunpckhbwEmitter<A, B> {
    fn sse_punpckhbw(&mut self, op0: A, op1: B);
}

impl<'a> SsePunpckhbwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_punpckhbw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PUNPCKHBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePunpckhbwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_punpckhbw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PUNPCKHBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PUNPCKHDQ` (PUNPCKHDQ). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePunpckhdqEmitter<A, B> {
    fn sse_punpckhdq(&mut self, op0: A, op1: B);
}

impl<'a> SsePunpckhdqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_punpckhdq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PUNPCKHDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePunpckhdqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_punpckhdq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PUNPCKHDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PUNPCKHQDQ` (PUNPCKHQDQ). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePunpckhqdqEmitter<A, B> {
    fn sse_punpckhqdq(&mut self, op0: A, op1: B);
}

impl<'a> SsePunpckhqdqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_punpckhqdq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PUNPCKHQDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePunpckhqdqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_punpckhqdq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PUNPCKHQDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PUNPCKHWD` (PUNPCKHWD). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePunpckhwdEmitter<A, B> {
    fn sse_punpckhwd(&mut self, op0: A, op1: B);
}

impl<'a> SsePunpckhwdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_punpckhwd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PUNPCKHWDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePunpckhwdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_punpckhwd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PUNPCKHWDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PUNPCKLBW` (PUNPCKLBW). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePunpcklbwEmitter<A, B> {
    fn sse_punpcklbw(&mut self, op0: A, op1: B);
}

impl<'a> SsePunpcklbwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_punpcklbw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PUNPCKLBWRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePunpcklbwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_punpcklbw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PUNPCKLBWRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PUNPCKLDQ` (PUNPCKLDQ). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePunpckldqEmitter<A, B> {
    fn sse_punpckldq(&mut self, op0: A, op1: B);
}

impl<'a> SsePunpckldqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_punpckldq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PUNPCKLDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePunpckldqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_punpckldq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PUNPCKLDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PUNPCKLQDQ` (PUNPCKLQDQ). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePunpcklqdqEmitter<A, B> {
    fn sse_punpcklqdq(&mut self, op0: A, op1: B);
}

impl<'a> SsePunpcklqdqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_punpcklqdq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PUNPCKLQDQRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePunpcklqdqEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_punpcklqdq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PUNPCKLQDQRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PUNPCKLWD` (PUNPCKLWD). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePunpcklwdEmitter<A, B> {
    fn sse_punpcklwd(&mut self, op0: A, op1: B);
}

impl<'a> SsePunpcklwdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_punpcklwd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PUNPCKLWDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePunpcklwdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_punpcklwd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PUNPCKLWDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_PXOR` (PXOR). 
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
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SsePxorEmitter<A, B> {
    fn sse_pxor(&mut self, op0: A, op1: B);
}

impl<'a> SsePxorEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_pxor(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_PXORRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SsePxorEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pxor(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_PXORRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_SHUFPD` (SHUFPD). 
/// Selects a double precision floating-point value of an input pair using a bit control and move to a designated element of the destination operand. The low-to-high order of double precision element of the destination operand is interleaved between the first source operand and the second source operand at the granularity of input pair of 128 bits. Each bit in the imm8 byte, starting from bit 0, is the select control of the corresponding element of the destination to received the shuffled result of an input pair.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SHUFPD.html).
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
pub trait SseShufpdEmitter<A, B, C> {
    fn sse_shufpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseShufpdEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_shufpd(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(SSE_SHUFPDRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> SseShufpdEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_shufpd(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(SSE_SHUFPDRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `SSE_SQRTPD` (SQRTPD). 
/// Performs a SIMD computation of the square roots of the two, four or eight packed double precision floating-point values in the source operand (the second operand) stores the packed double precision floating-point results in the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SQRTPD.html).
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
pub trait SseSqrtpdEmitter<A, B> {
    fn sse_sqrtpd(&mut self, op0: A, op1: B);
}

impl<'a> SseSqrtpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_sqrtpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_SQRTPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseSqrtpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_sqrtpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_SQRTPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_SQRTSD` (SQRTSD). 
/// Computes the square root of the low double precision floating-point value in the second source operand and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination operands are XMM registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SQRTSD.html).
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
pub trait SseSqrtsdEmitter<A, B> {
    fn sse_sqrtsd(&mut self, op0: A, op1: B);
}

impl<'a> SseSqrtsdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_sqrtsd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_SQRTSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseSqrtsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_sqrtsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_SQRTSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_SUBPD` (SUBPD). 
/// Performs a SIMD subtract of the two, four or eight packed double precision floating-point values of the second Source operand from the first Source operand, and stores the packed double precision floating-point results in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SUBPD.html).
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
pub trait SseSubpdEmitter<A, B> {
    fn sse_subpd(&mut self, op0: A, op1: B);
}

impl<'a> SseSubpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_subpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_SUBPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseSubpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_subpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_SUBPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_SUBSD` (SUBSD). 
/// Subtract the low double precision floating-point value in the second source operand from the first source operand and stores the double precision floating-point result in the low quadword of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SUBSD.html).
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
pub trait SseSubsdEmitter<A, B> {
    fn sse_subsd(&mut self, op0: A, op1: B);
}

impl<'a> SseSubsdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_subsd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_SUBSDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseSubsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_subsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_SUBSDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_UCOMISD` (UCOMISD). 
/// Performs an unordered compare of the double precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UCOMISD.html).
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
pub trait SseUcomisdEmitter<A, B> {
    fn sse_ucomisd(&mut self, op0: A, op1: B);
}

impl<'a> SseUcomisdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_ucomisd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_UCOMISDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseUcomisdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_ucomisd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_UCOMISDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_UNPCKHPD` (UNPCKHPD). 
/// Performs an interleaved unpack of the high double precision floating-point values from the first source operand and the second source operand. See Figure 4-15 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 2B.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UNPCKHPD.html).
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
pub trait SseUnpckhpdEmitter<A, B> {
    fn sse_unpckhpd(&mut self, op0: A, op1: B);
}

impl<'a> SseUnpckhpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_unpckhpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_UNPCKHPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseUnpckhpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_unpckhpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_UNPCKHPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_UNPCKLPD` (UNPCKLPD). 
/// Performs an interleaved unpack of the low double precision floating-point values from the first source operand and the second source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UNPCKLPD.html).
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
pub trait SseUnpcklpdEmitter<A, B> {
    fn sse_unpcklpd(&mut self, op0: A, op1: B);
}

impl<'a> SseUnpcklpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_unpcklpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_UNPCKLPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseUnpcklpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_unpcklpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_UNPCKLPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_XORPD` (XORPD). 
/// Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPD.html).
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
pub trait SseXorpdEmitter<A, B> {
    fn sse_xorpd(&mut self, op0: A, op1: B);
}

impl<'a> SseXorpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_xorpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_XORPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseXorpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_xorpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_XORPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `LFENCE` (LFENCE). 
    /// Performs a serializing operation on all load-from-memory instructions that were issued prior the LFENCE instruction. Specifically, LFENCE does not execute until all prior instructions have completed locally, and no later instruction begins execution until LFENCE completes. In particular, an instruction that loads from memory and that precedes an LFENCE receives data from memory prior to completion of the LFENCE. (An LFENCE that follows an instruction that stores to memory might complete before the data being stored have become globally visible.) Instructions following an LFENCE may be fetched from memory before the LFENCE, but they will not execute (even speculatively) until the LFENCE completes.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/LFENCE.html).
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
    pub fn lfence(&mut self)
    where Assembler<'a>: LfenceEmitter {
        <Self as LfenceEmitter>::lfence(self);
    }
    /// `MFENCE`.
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
    pub fn mfence(&mut self)
    where Assembler<'a>: MfenceEmitter {
        <Self as MfenceEmitter>::mfence(self);
    }
    /// `MMX_CVTPD2PI` (CVTPD2PI). 
    /// Converts two packed double precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPD2PI.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Xmm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_cvtpd2pi<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxCvtpd2piEmitter<A, B> {
        <Self as MmxCvtpd2piEmitter<A, B>>::mmx_cvtpd2pi(self, op0, op1);
    }
    /// `MMX_CVTPI2PD` (CVTPI2PD). 
    /// Converts two packed signed doubleword integers in the source operand (second operand) to two packed double precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPI2PD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Mm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_cvtpi2pd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxCvtpi2pdEmitter<A, B> {
        <Self as MmxCvtpi2pdEmitter<A, B>>::mmx_cvtpi2pd(self, op0, op1);
    }
    /// `MMX_CVTPI2PS` (CVTPI2PS). 
    /// Converts two packed signed doubleword integers in the source operand (second operand) to two packed single precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPI2PS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Mm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_cvtpi2ps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxCvtpi2psEmitter<A, B> {
        <Self as MmxCvtpi2psEmitter<A, B>>::mmx_cvtpi2ps(self, op0, op1);
    }
    /// `MMX_CVTPS2PI` (CVTPS2PI). 
    /// Converts two packed single precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPS2PI.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Xmm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_cvtps2pi<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxCvtps2piEmitter<A, B> {
        <Self as MmxCvtps2piEmitter<A, B>>::mmx_cvtps2pi(self, op0, op1);
    }
    /// `MMX_CVTTPD2PI` (CVTTPD2PI). 
    /// Converts two packed double precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand). The source operand can be an XMM register or a 128-bit memory location. The destination operand is an MMX technology register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTPD2PI.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Xmm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_cvttpd2pi<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxCvttpd2piEmitter<A, B> {
        <Self as MmxCvttpd2piEmitter<A, B>>::mmx_cvttpd2pi(self, op0, op1);
    }
    /// `MMX_CVTTPS2PI` (CVTTPS2PI). 
    /// Converts two packed single precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is an MMX technology register. When the source operand is an XMM register, the two single precision floating-point values are contained in the low quadword of the register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTPS2PI.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mm, Mem  |
    /// | 2 | Mm, Xmm  |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn mmx_cvttps2pi<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MmxCvttps2piEmitter<A, B> {
        <Self as MmxCvttps2piEmitter<A, B>>::mmx_cvttps2pi(self, op0, op1);
    }
    /// `MOVNTI` (MOVNTI). 
    /// Moves the doubleword integer in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to minimize cache pollution during the write to memory. The source operand is a general-purpose register. The destination operand is a 32-bit memory location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTI.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Gpd |
    /// | 2 | Mem, Gpq |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn movnti<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MovntiEmitter<A, B> {
        <Self as MovntiEmitter<A, B>>::movnti(self, op0, op1);
    }
    /// `SSE_ADDPD` (ADDPD). 
    /// Adds two, four or eight packed double precision floating-point values from the first source operand to the second source operand, and stores the packed double precision floating-point result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDPD.html).
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
    pub fn sse_addpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseAddpdEmitter<A, B> {
        <Self as SseAddpdEmitter<A, B>>::sse_addpd(self, op0, op1);
    }
    /// `SSE_ADDSD` (ADDSD). 
    /// Adds the low double precision floating-point values from the second source operand and the first source operand and stores the double precision floating-point result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSD.html).
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
    pub fn sse_addsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseAddsdEmitter<A, B> {
        <Self as SseAddsdEmitter<A, B>>::sse_addsd(self, op0, op1);
    }
    /// `SSE_ANDNPD` (ANDNPD). 
    /// Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPD.html).
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
    pub fn sse_andnpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseAndnpdEmitter<A, B> {
        <Self as SseAndnpdEmitter<A, B>>::sse_andnpd(self, op0, op1);
    }
    /// `SSE_ANDPD` (ANDPD). 
    /// Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPD.html).
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
    pub fn sse_andpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseAndpdEmitter<A, B> {
        <Self as SseAndpdEmitter<A, B>>::sse_andpd(self, op0, op1);
    }
    /// `SSE_CMPPD` (CMPPD). 
    /// Performs a SIMD compare of the packed double precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each pair of packed values in the two source operands.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPPD.html).
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
    pub fn sse_cmppd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseCmppdEmitter<A, B, C> {
        <Self as SseCmppdEmitter<A, B, C>>::sse_cmppd(self, op0, op1, op2);
    }
    /// `SSE_CMPSD` (CMPSD). 
    /// Compares the byte, word, doubleword, or quadword specified with the first source operand with the byte, word, doubleword, or quadword specified with the second source operand and sets the status flags in the EFLAGS register according to the results.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPS%3ACMPSB%3ACMPSW%3ACMPSD%3ACMPSQ.html).
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
    pub fn sse_cmpsd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseCmpsdEmitter<A, B, C> {
        <Self as SseCmpsdEmitter<A, B, C>>::sse_cmpsd(self, op0, op1, op2);
    }
    /// `SSE_COMISD` (COMISD). 
    /// Compares the double precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/COMISD.html).
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
    pub fn sse_comisd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseComisdEmitter<A, B> {
        <Self as SseComisdEmitter<A, B>>::sse_comisd(self, op0, op1);
    }
    /// `SSE_CVTDQ2PD` (CVTDQ2PD). 
    /// Converts two, four or eight packed signed doubleword integers in the source operand (the second operand) to two, four or eight packed double precision floating-point values in the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTDQ2PD.html).
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
    pub fn sse_cvtdq2pd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtdq2pdEmitter<A, B> {
        <Self as SseCvtdq2pdEmitter<A, B>>::sse_cvtdq2pd(self, op0, op1);
    }
    /// `SSE_CVTDQ2PS` (CVTDQ2PS). 
    /// Converts four, eight or sixteen packed signed doubleword integers in the source operand to four, eight or sixteen packed single precision floating-point values in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTDQ2PS.html).
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
    pub fn sse_cvtdq2ps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtdq2psEmitter<A, B> {
        <Self as SseCvtdq2psEmitter<A, B>>::sse_cvtdq2ps(self, op0, op1);
    }
    /// `SSE_CVTPD2DQ` (CVTPD2DQ). 
    /// Converts packed double precision floating-point values in the source operand (second operand) to packed signed doubleword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPD2DQ.html).
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
    pub fn sse_cvtpd2dq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtpd2dqEmitter<A, B> {
        <Self as SseCvtpd2dqEmitter<A, B>>::sse_cvtpd2dq(self, op0, op1);
    }
    /// `SSE_CVTPD2PS` (CVTPD2PS). 
    /// Converts two, four or eight packed double precision floating-point values in the source operand (second operand) to two, four or eight packed single precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPD2PS.html).
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
    pub fn sse_cvtpd2ps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtpd2psEmitter<A, B> {
        <Self as SseCvtpd2psEmitter<A, B>>::sse_cvtpd2ps(self, op0, op1);
    }
    /// `SSE_CVTPS2DQ` (CVTPS2DQ). 
    /// Converts four, eight or sixteen packed single precision floating-point values in the source operand to four, eight or sixteen signed doubleword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPS2DQ.html).
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
    pub fn sse_cvtps2dq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtps2dqEmitter<A, B> {
        <Self as SseCvtps2dqEmitter<A, B>>::sse_cvtps2dq(self, op0, op1);
    }
    /// `SSE_CVTPS2PD` (CVTPS2PD). 
    /// Converts two, four or eight packed single precision floating-point values in the source operand (second operand) to two, four or eight packed double precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTPS2PD.html).
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
    pub fn sse_cvtps2pd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtps2pdEmitter<A, B> {
        <Self as SseCvtps2pdEmitter<A, B>>::sse_cvtps2pd(self, op0, op1);
    }
    /// `SSE_CVTSD2SI` (CVTSD2SI). 
    /// Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer in the destination operand (first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general-purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSD2SI.html).
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
    pub fn sse_cvtsd2si<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtsd2siEmitter<A, B> {
        <Self as SseCvtsd2siEmitter<A, B>>::sse_cvtsd2si(self, op0, op1);
    }
    /// `SSE_CVTSD2SS` (CVTSD2SS). 
    /// Converts a double precision floating-point value in the “convert-from” source operand (the second operand in SSE2 version, otherwise the third operand) to a single precision floating-point value in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSD2SS.html).
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
    pub fn sse_cvtsd2ss<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtsd2ssEmitter<A, B> {
        <Self as SseCvtsd2ssEmitter<A, B>>::sse_cvtsd2ss(self, op0, op1);
    }
    /// `SSE_CVTSI2SD` (CVTSI2SD). 
    /// Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the “convert-from” source operand to a double precision floating-point value in the destination operand. The result is stored in the low quadword of the destination operand, and the high quadword left unchanged. When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSI2SD.html).
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
    pub fn sse_cvtsi2sd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtsi2sdEmitter<A, B> {
        <Self as SseCvtsi2sdEmitter<A, B>>::sse_cvtsi2sd(self, op0, op1);
    }
    /// `SSE_CVTSS2SD` (CVTSS2SD). 
    /// Converts a single precision floating-point value in the “convert-from” source operand to a double precision floating-point value in the destination operand. When the “convert-from” source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register. The result is stored in the low quadword of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTSS2SD.html).
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
    pub fn sse_cvtss2sd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvtss2sdEmitter<A, B> {
        <Self as SseCvtss2sdEmitter<A, B>>::sse_cvtss2sd(self, op0, op1);
    }
    /// `SSE_CVTTPD2DQ` (CVTTPD2DQ). 
    /// Converts two, four or eight packed double precision floating-point values in the source operand (second operand) to two, four or eight packed signed doubleword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTPD2DQ.html).
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
    pub fn sse_cvttpd2dq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvttpd2dqEmitter<A, B> {
        <Self as SseCvttpd2dqEmitter<A, B>>::sse_cvttpd2dq(self, op0, op1);
    }
    /// `SSE_CVTTPS2DQ` (CVTTPS2DQ). 
    /// Converts four, eight or sixteen packed single precision floating-point values in the source operand to four, eight or sixteen signed doubleword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTPS2DQ.html).
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
    pub fn sse_cvttps2dq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvttps2dqEmitter<A, B> {
        <Self as SseCvttps2dqEmitter<A, B>>::sse_cvttps2dq(self, op0, op1);
    }
    /// `SSE_CVTTSD2SI` (CVTTSD2SI). 
    /// Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand). The source operand can be an XMM register or a 64-bit memory location. The destination operand is a general purpose register. When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CVTTSD2SI.html).
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
    pub fn sse_cvttsd2si<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseCvttsd2siEmitter<A, B> {
        <Self as SseCvttsd2siEmitter<A, B>>::sse_cvttsd2si(self, op0, op1);
    }
    /// `SSE_DIVPD` (DIVPD). 
    /// Performs a SIMD divide of the double precision floating-point values in the first source operand by the floating-point values in the second source operand (the third operand). Results are written to the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DIVPD.html).
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
    pub fn sse_divpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseDivpdEmitter<A, B> {
        <Self as SseDivpdEmitter<A, B>>::sse_divpd(self, op0, op1);
    }
    /// `SSE_DIVSD` (DIVSD). 
    /// Divides the low double precision floating-point value in the first source operand by the low double precision floating-point value in the second source operand, and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination are XMM registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DIVSD.html).
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
    pub fn sse_divsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseDivsdEmitter<A, B> {
        <Self as SseDivsdEmitter<A, B>>::sse_divsd(self, op0, op1);
    }
    /// `SSE_MASKMOVDQU` (MASKMOVDQU). 
    /// Stores selected bytes from the source operand (first operand) into an 128-bit memory location. The mask operand (second operand) selects which bytes from the source operand are written to memory. The source and mask operands are XMM registers. The memory location specified by the effective address in the DI/EDI/RDI register (the default segment register is DS, but this may be overridden with a segment-override prefix). The memory location does not need to be aligned on a natural boundary. (The size of the store address depends on the address-size attribute.)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MASKMOVDQU.html).
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
    pub fn sse_maskmovdqu<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMaskmovdquEmitter<A, B> {
        <Self as SseMaskmovdquEmitter<A, B>>::sse_maskmovdqu(self, op0, op1);
    }
    /// `SSE_MAXPD` (MAXPD). 
    /// Performs a SIMD compare of the packed double precision floating-point values in the first source operand and the second source operand and returns the maximum value for each pair of values to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MAXPD.html).
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
    pub fn sse_maxpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMaxpdEmitter<A, B> {
        <Self as SseMaxpdEmitter<A, B>>::sse_maxpd(self, op0, op1);
    }
    /// `SSE_MAXSD` (MAXSD). 
    /// Compares the low double precision floating-point values in the first source operand and the second source operand, and returns the maximum value to the low quadword of the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination operands are XMM registers. When the second source operand is a memory operand, only 64 bits are accessed.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MAXSD.html).
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
    pub fn sse_maxsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMaxsdEmitter<A, B> {
        <Self as SseMaxsdEmitter<A, B>>::sse_maxsd(self, op0, op1);
    }
    /// `SSE_MINPD` (MINPD). 
    /// Performs a SIMD compare of the packed double precision floating-point values in the first source operand and the second source operand and returns the minimum value for each pair of values to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MINPD.html).
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
    pub fn sse_minpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMinpdEmitter<A, B> {
        <Self as SseMinpdEmitter<A, B>>::sse_minpd(self, op0, op1);
    }
    /// `SSE_MINSD` (MINSD). 
    /// Compares the low double precision floating-point values in the first source operand and the second source operand, and returns the minimum value to the low quadword of the destination operand. When the source operand is a memory operand, only the 64 bits are accessed.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MINSD.html).
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
    pub fn sse_minsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMinsdEmitter<A, B> {
        <Self as SseMinsdEmitter<A, B>>::sse_minsd(self, op0, op1);
    }
    /// `SSE_MOVAPD` (MOVAPD). 
    /// Moves 2, 4 or 8 double precision floating-point values from the source operand (second operand) to the destination operand (first operand). This instruction can be used to load an XMM, YMM or ZMM register from an 128-bit, 256-bit or 512-bit memory location, to store the contents of an XMM, YMM or ZMM register into a 128-bit, 256-bit or 512-bit memory location, or to move data between two XMM, two YMM or two ZMM registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVAPD.html).
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
    pub fn sse_movapd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovapdEmitter<A, B> {
        <Self as SseMovapdEmitter<A, B>>::sse_movapd(self, op0, op1);
    }
    /// `SSE_MOVDQA` (MOVDQA). 
    /// Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html).
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
    pub fn sse_movdqa<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovdqaEmitter<A, B> {
        <Self as SseMovdqaEmitter<A, B>>::sse_movdqa(self, op0, op1);
    }
    /// `SSE_MOVDQU` (MOVDQU). 
    /// Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html).
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
    pub fn sse_movdqu<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovdquEmitter<A, B> {
        <Self as SseMovdquEmitter<A, B>>::sse_movdqu(self, op0, op1);
    }
    /// `SSE_MOVD_G2X` (MOVD). 
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
    /// | 1 | Xmm, Gpd |
    /// | 2 | Xmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movd_g2x<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovdG2xEmitter<A, B> {
        <Self as SseMovdG2xEmitter<A, B>>::sse_movd_g2x(self, op0, op1);
    }
    /// `SSE_MOVD_X2G` (MOVD). 
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
    /// | 1 | Gpd, Xmm |
    /// | 2 | Mem, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movd_x2g<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovdX2gEmitter<A, B> {
        <Self as SseMovdX2gEmitter<A, B>>::sse_movd_x2g(self, op0, op1);
    }
    /// `SSE_MOVHPD` (MOVHPD). 
    /// This instruction cannot be used for register to register or memory to memory moves.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVHPD.html).
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
    pub fn sse_movhpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovhpdEmitter<A, B> {
        <Self as SseMovhpdEmitter<A, B>>::sse_movhpd(self, op0, op1);
    }
    /// `SSE_MOVLPD` (MOVLPD). 
    /// This instruction cannot be used for register to register or memory to memory moves.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVLPD.html).
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
    pub fn sse_movlpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovlpdEmitter<A, B> {
        <Self as SseMovlpdEmitter<A, B>>::sse_movlpd(self, op0, op1);
    }
    /// `SSE_MOVMSKPD` (MOVMSKPD). 
    /// Extracts the sign bits from the packed double precision floating-point values in the source operand (second operand), formats them into a 2-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM register, and the destination operand is a general-purpose register. The mask is stored in the 2 low-order bits of the destination operand. Zero-extend the upper bits of the destination.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVMSKPD.html).
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
    pub fn sse_movmskpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovmskpdEmitter<A, B> {
        <Self as SseMovmskpdEmitter<A, B>>::sse_movmskpd(self, op0, op1);
    }
    /// `SSE_MOVNTDQ` (MOVNTDQ). 
    /// Moves the packed integers in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to prevent caching of the data during the write to memory. The source operand is an XMM register, YMM register or ZMM register, which is assumed to contain integer data (packed bytes, words, double-words, or quadwords). The destination operand is a 128-bit, 256-bit or 512-bit memory location. The memory operand must be aligned on a 16-byte (128-bit version), 32-byte (VEX.256 encoded version) or 64-byte (512-bit version) boundary otherwise a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTDQ.html).
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
    pub fn sse_movntdq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovntdqEmitter<A, B> {
        <Self as SseMovntdqEmitter<A, B>>::sse_movntdq(self, op0, op1);
    }
    /// `SSE_MOVNTPD` (MOVNTPD). 
    /// Moves the packed double precision floating-point values in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to prevent caching of the data during the write to memory. The source operand is an XMM register, YMM register or ZMM register, which is assumed to contain packed double precision, floating-pointing data. The destination operand is a 128-bit, 256-bit or 512-bit memory location. The memory operand must be aligned on a 16-byte (128-bit version), 32-byte (VEX.256 encoded version) or 64-byte (EVEX.512 encoded version) boundary otherwise a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVNTPD.html).
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
    pub fn sse_movntpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovntpdEmitter<A, B> {
        <Self as SseMovntpdEmitter<A, B>>::sse_movntpd(self, op0, op1);
    }
    /// `SSE_MOVNTSD`.
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
    pub fn sse_movntsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovntsdEmitter<A, B> {
        <Self as SseMovntsdEmitter<A, B>>::sse_movntsd(self, op0, op1);
    }
    /// `SSE_MOVQ` (MOVQ). 
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
    /// | 1 | Mem, Xmm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovqEmitter<A, B> {
        <Self as SseMovqEmitter<A, B>>::sse_movq(self, op0, op1);
    }
    /// `SSE_MOVQ_G2X` (MOVQ). 
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
    /// | 1 | Xmm, Gpd |
    /// | 2 | Xmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movq_g2x<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovqG2xEmitter<A, B> {
        <Self as SseMovqG2xEmitter<A, B>>::sse_movq_g2x(self, op0, op1);
    }
    /// `SSE_MOVQ_X2G` (MOVQ). 
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
    /// | 1 | Gpd, Xmm |
    /// | 2 | Mem, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movq_x2g<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovqX2gEmitter<A, B> {
        <Self as SseMovqX2gEmitter<A, B>>::sse_movq_x2g(self, op0, op1);
    }
    /// `SSE_MOVSD` (MOVSD). 
    /// Moves the byte, word, or doubleword specified with the second operand (source operand) to the location specified with the first operand (destination operand). Both the source and destination operands are located in memory. The address of the source operand is read from the DS:ESI or the DS:SI registers (depending on the address-size attribute of the instruction, 32 or 16, respectively). The address of the destination operand is read from the ES:EDI or the ES:DI registers (again depending on the address-size attribute of the instruction). The DS segment may be overridden with a segment override prefix, but the ES segment cannot be overridden.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVS%3AMOVSB%3AMOVSW%3AMOVSD%3AMOVSQ.html).
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
    pub fn sse_movsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovsdEmitter<A, B> {
        <Self as SseMovsdEmitter<A, B>>::sse_movsd(self, op0, op1);
    }
    /// `SSE_MOVUPD` (MOVUPD). 
    /// Note: VEX.vvvv and EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVUPD.html).
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
    pub fn sse_movupd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovupdEmitter<A, B> {
        <Self as SseMovupdEmitter<A, B>>::sse_movupd(self, op0, op1);
    }
    /// `SSE_MULPD` (MULPD). 
    /// Multiply packed double precision floating-point values from the first source operand with corresponding values in the second source operand, and stores the packed double precision floating-point results in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MULPD.html).
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
    pub fn sse_mulpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMulpdEmitter<A, B> {
        <Self as SseMulpdEmitter<A, B>>::sse_mulpd(self, op0, op1);
    }
    /// `SSE_MULSD` (MULSD). 
    /// Multiplies the low double precision floating-point value in the second source operand by the low double precision floating-point value in the first source operand, and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source operand and the destination operands are XMM registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MULSD.html).
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
    pub fn sse_mulsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMulsdEmitter<A, B> {
        <Self as SseMulsdEmitter<A, B>>::sse_mulsd(self, op0, op1);
    }
    /// `SSE_ORPD` (ORPD). 
    /// Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPD.html).
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
    pub fn sse_orpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseOrpdEmitter<A, B> {
        <Self as SseOrpdEmitter<A, B>>::sse_orpd(self, op0, op1);
    }
    /// `SSE_PACKSSDW` (PACKSSDW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_packssdw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePackssdwEmitter<A, B> {
        <Self as SsePackssdwEmitter<A, B>>::sse_packssdw(self, op0, op1);
    }
    /// `SSE_PACKSSWB` (PACKSSWB). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_packsswb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePacksswbEmitter<A, B> {
        <Self as SsePacksswbEmitter<A, B>>::sse_packsswb(self, op0, op1);
    }
    /// `SSE_PACKUSWB` (PACKUSWB). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_packuswb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePackuswbEmitter<A, B> {
        <Self as SsePackuswbEmitter<A, B>>::sse_packuswb(self, op0, op1);
    }
    /// `SSE_PADDB` (PADDB). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_paddb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePaddbEmitter<A, B> {
        <Self as SsePaddbEmitter<A, B>>::sse_paddb(self, op0, op1);
    }
    /// `SSE_PADDD` (PADDD). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_paddd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePadddEmitter<A, B> {
        <Self as SsePadddEmitter<A, B>>::sse_paddd(self, op0, op1);
    }
    /// `SSE_PADDQ` (PADDQ). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_paddq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePaddqEmitter<A, B> {
        <Self as SsePaddqEmitter<A, B>>::sse_paddq(self, op0, op1);
    }
    /// `SSE_PADDSB` (PADDSB). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_paddsb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePaddsbEmitter<A, B> {
        <Self as SsePaddsbEmitter<A, B>>::sse_paddsb(self, op0, op1);
    }
    /// `SSE_PADDSW` (PADDSW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_paddsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePaddswEmitter<A, B> {
        <Self as SsePaddswEmitter<A, B>>::sse_paddsw(self, op0, op1);
    }
    /// `SSE_PADDUSB` (PADDUSB). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_paddusb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePaddusbEmitter<A, B> {
        <Self as SsePaddusbEmitter<A, B>>::sse_paddusb(self, op0, op1);
    }
    /// `SSE_PADDUSW` (PADDUSW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_paddusw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePadduswEmitter<A, B> {
        <Self as SsePadduswEmitter<A, B>>::sse_paddusw(self, op0, op1);
    }
    /// `SSE_PADDW` (PADDW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_paddw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePaddwEmitter<A, B> {
        <Self as SsePaddwEmitter<A, B>>::sse_paddw(self, op0, op1);
    }
    /// `SSE_PAND` (PAND). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pand<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePandEmitter<A, B> {
        <Self as SsePandEmitter<A, B>>::sse_pand(self, op0, op1);
    }
    /// `SSE_PANDN` (PANDN). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pandn<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePandnEmitter<A, B> {
        <Self as SsePandnEmitter<A, B>>::sse_pandn(self, op0, op1);
    }
    /// `SSE_PAVGB` (PAVGB). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pavgb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePavgbEmitter<A, B> {
        <Self as SsePavgbEmitter<A, B>>::sse_pavgb(self, op0, op1);
    }
    /// `SSE_PAVGW` (PAVGW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pavgw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePavgwEmitter<A, B> {
        <Self as SsePavgwEmitter<A, B>>::sse_pavgw(self, op0, op1);
    }
    /// `SSE_PCMPEQB` (PCMPEQB). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pcmpeqb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePcmpeqbEmitter<A, B> {
        <Self as SsePcmpeqbEmitter<A, B>>::sse_pcmpeqb(self, op0, op1);
    }
    /// `SSE_PCMPEQD` (PCMPEQD). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pcmpeqd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePcmpeqdEmitter<A, B> {
        <Self as SsePcmpeqdEmitter<A, B>>::sse_pcmpeqd(self, op0, op1);
    }
    /// `SSE_PCMPEQW` (PCMPEQW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pcmpeqw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePcmpeqwEmitter<A, B> {
        <Self as SsePcmpeqwEmitter<A, B>>::sse_pcmpeqw(self, op0, op1);
    }
    /// `SSE_PCMPGTB` (PCMPGTB). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pcmpgtb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePcmpgtbEmitter<A, B> {
        <Self as SsePcmpgtbEmitter<A, B>>::sse_pcmpgtb(self, op0, op1);
    }
    /// `SSE_PCMPGTD` (PCMPGTD). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pcmpgtd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePcmpgtdEmitter<A, B> {
        <Self as SsePcmpgtdEmitter<A, B>>::sse_pcmpgtd(self, op0, op1);
    }
    /// `SSE_PCMPGTW` (PCMPGTW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pcmpgtw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePcmpgtwEmitter<A, B> {
        <Self as SsePcmpgtwEmitter<A, B>>::sse_pcmpgtw(self, op0, op1);
    }
    /// `SSE_PEXTRW` (PEXTRW). 
    /// Copies the word in the source operand (second operand) specified by the count operand (third operand) to the destination operand (first operand). The source operand can be an MMX technology register or an XMM register. The destination operand can be the low word of a general-purpose register or a 16-bit memory address. The count operand is an 8-bit immediate. When specifying a word location in an MMX technology register, the 2 least-significant bits of the count operand specify the location; for an XMM register, the 3 least-significant bits specify the location. The content of the destination register above bit 16 is cleared (set to all 0s).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRW.html).
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
    pub fn sse_pextrw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePextrwEmitter<A, B, C> {
        <Self as SsePextrwEmitter<A, B, C>>::sse_pextrw(self, op0, op1, op2);
    }
    /// `SSE_PINSRW` (PINSRW). 
    /// Three operand MMX and SSE instructions
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRW.html).
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
    pub fn sse_pinsrw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePinsrwEmitter<A, B, C> {
        <Self as SsePinsrwEmitter<A, B, C>>::sse_pinsrw(self, op0, op1, op2);
    }
    /// `SSE_PMADDWD` (PMADDWD). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pmaddwd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmaddwdEmitter<A, B> {
        <Self as SsePmaddwdEmitter<A, B>>::sse_pmaddwd(self, op0, op1);
    }
    /// `SSE_PMAXSW` (PMAXSW). 
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
    pub fn sse_pmaxsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmaxswEmitter<A, B> {
        <Self as SsePmaxswEmitter<A, B>>::sse_pmaxsw(self, op0, op1);
    }
    /// `SSE_PMAXUB` (PMAXUB). 
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
    pub fn sse_pmaxub<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmaxubEmitter<A, B> {
        <Self as SsePmaxubEmitter<A, B>>::sse_pmaxub(self, op0, op1);
    }
    /// `SSE_PMINSW` (PMINSW). 
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
    pub fn sse_pminsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePminswEmitter<A, B> {
        <Self as SsePminswEmitter<A, B>>::sse_pminsw(self, op0, op1);
    }
    /// `SSE_PMINUB` (PMINUB). 
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
    pub fn sse_pminub<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePminubEmitter<A, B> {
        <Self as SsePminubEmitter<A, B>>::sse_pminub(self, op0, op1);
    }
    /// `SSE_PMOVMSKB` (PMOVMSKB). 
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
    /// | 1 | Gpq, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pmovmskb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmovmskbEmitter<A, B> {
        <Self as SsePmovmskbEmitter<A, B>>::sse_pmovmskb(self, op0, op1);
    }
    /// `SSE_PMULHUW` (PMULHUW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pmulhuw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmulhuwEmitter<A, B> {
        <Self as SsePmulhuwEmitter<A, B>>::sse_pmulhuw(self, op0, op1);
    }
    /// `SSE_PMULHW` (PMULHW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pmulhw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmulhwEmitter<A, B> {
        <Self as SsePmulhwEmitter<A, B>>::sse_pmulhw(self, op0, op1);
    }
    /// `SSE_PMULLW` (PMULLW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pmullw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmullwEmitter<A, B> {
        <Self as SsePmullwEmitter<A, B>>::sse_pmullw(self, op0, op1);
    }
    /// `SSE_PMULUDQ` (PMULUDQ). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pmuludq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePmuludqEmitter<A, B> {
        <Self as SsePmuludqEmitter<A, B>>::sse_pmuludq(self, op0, op1);
    }
    /// `SSE_POR` (POR). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_por<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePorEmitter<A, B> {
        <Self as SsePorEmitter<A, B>>::sse_por(self, op0, op1);
    }
    /// `SSE_PSADBW` (PSADBW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psadbw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsadbwEmitter<A, B> {
        <Self as SsePsadbwEmitter<A, B>>::sse_psadbw(self, op0, op1);
    }
    /// `SSE_PSHUFD` (PSHUFD). 
    /// Copies doublewords from source operand (second operand) and inserts them in the destination operand (first operand) at the locations selected with the order operand (third operand). Figure 4-16 shows the operation of the 256-bit VPSHUFD instruction and the encoding of the order operand. Each 2-bit field in the order operand selects the contents of one doubleword location within a 128-bit lane and copy to the target element in the destination operand. For example, bits 0 and 1 of the order operand targets the first doubleword element in the low and high 128-bit lane of the destination operand for 256-bit VPSHUFD. The encoded value of bits 1:0 of the order operand (see the field encoding in Figure 4-16) determines which doubleword element (from the respective 128-bit lane) of the source operand will be copied to doubleword 0 of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFD.html).
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
    pub fn sse_pshufd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePshufdEmitter<A, B, C> {
        <Self as SsePshufdEmitter<A, B, C>>::sse_pshufd(self, op0, op1, op2);
    }
    /// `SSE_PSHUFHW` (PSHUFHW). 
    /// Copies words from the high quadword of a 128-bit lane of the source operand and inserts them in the high quadword of the destination operand at word locations (of the respective lane) selected with the immediate operand. This 256-bit operation is similar to the in-lane operation used by the 256-bit VPSHUFD instruction, which is illustrated in Figure 4-16. For 128-bit operation, only the low 128-bit lane is operative. Each 2-bit field in the immediate operand selects the contents of one word location in the high quadword of the destination operand. The binary encodings of the immediate operand fields select words (0, 1, 2 or 3, 4) from the high quadword of the source operand to be copied to the destination operand. The low quadword of the source operand is copied to the low quadword of the destination operand, for each 128-bit lane.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFHW.html).
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
    pub fn sse_pshufhw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePshufhwEmitter<A, B, C> {
        <Self as SsePshufhwEmitter<A, B, C>>::sse_pshufhw(self, op0, op1, op2);
    }
    /// `SSE_PSHUFLW` (PSHUFLW). 
    /// Copies words from the low quadword of a 128-bit lane of the source operand and inserts them in the low quadword of the destination operand at word locations (of the respective lane) selected with the immediate operand. The 256-bit operation is similar to the in-lane operation used by the 256-bit VPSHUFD instruction, which is illustrated in Figure 4-16. For 128-bit operation, only the low 128-bit lane is operative. Each 2-bit field in the immediate operand selects the contents of one word location in the low quadword of the destination operand. The binary encodings of the immediate operand fields select words (0, 1, 2 or 3) from the low quadword of the source operand to be copied to the destination operand. The high quadword of the source operand is copied to the high quadword of the destination operand, for each 128-bit lane.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSHUFLW.html).
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
    pub fn sse_pshuflw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SsePshuflwEmitter<A, B, C> {
        <Self as SsePshuflwEmitter<A, B, C>>::sse_pshuflw(self, op0, op1, op2);
    }
    /// `SSE_PSLLD` (PSLLD). 
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
    /// | 1 | Xmm, Imm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pslld<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePslldEmitter<A, B> {
        <Self as SsePslldEmitter<A, B>>::sse_pslld(self, op0, op1);
    }
    /// `SSE_PSLLDQ` (PSLLDQ). 
    /// Shifts the destination operand (first operand) to the left by the number of bytes specified in the count operand (second operand). The empty low-order bytes are cleared (set to all 0s). If the value specified by the count operand is greater than 15, the destination operand is set to all 0s. The count operand is an 8-bit immediate.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSLLDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Imm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pslldq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePslldqEmitter<A, B> {
        <Self as SsePslldqEmitter<A, B>>::sse_pslldq(self, op0, op1);
    }
    /// `SSE_PSLLQ` (PSLLQ). 
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
    /// | 1 | Xmm, Imm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psllq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsllqEmitter<A, B> {
        <Self as SsePsllqEmitter<A, B>>::sse_psllq(self, op0, op1);
    }
    /// `SSE_PSLLW` (PSLLW). 
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
    /// | 1 | Xmm, Imm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psllw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsllwEmitter<A, B> {
        <Self as SsePsllwEmitter<A, B>>::sse_psllw(self, op0, op1);
    }
    /// `SSE_PSRAD` (PSRAD). 
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
    /// | 1 | Xmm, Imm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psrad<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsradEmitter<A, B> {
        <Self as SsePsradEmitter<A, B>>::sse_psrad(self, op0, op1);
    }
    /// `SSE_PSRAW` (PSRAW). 
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
    /// | 1 | Xmm, Imm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psraw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsrawEmitter<A, B> {
        <Self as SsePsrawEmitter<A, B>>::sse_psraw(self, op0, op1);
    }
    /// `SSE_PSRLD` (PSRLD). 
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
    /// | 1 | Xmm, Imm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psrld<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsrldEmitter<A, B> {
        <Self as SsePsrldEmitter<A, B>>::sse_psrld(self, op0, op1);
    }
    /// `SSE_PSRLDQ` (PSRLDQ). 
    /// Shifts the destination operand (first operand) to the right by the number of bytes specified in the count operand (second operand). The empty high-order bytes are cleared (set to all 0s). If the value specified by the count operand is greater than 15, the destination operand is set to all 0s. The count operand is an 8-bit immediate.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSRLDQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Imm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psrldq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsrldqEmitter<A, B> {
        <Self as SsePsrldqEmitter<A, B>>::sse_psrldq(self, op0, op1);
    }
    /// `SSE_PSRLQ` (PSRLQ). 
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
    /// | 1 | Xmm, Imm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psrlq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsrlqEmitter<A, B> {
        <Self as SsePsrlqEmitter<A, B>>::sse_psrlq(self, op0, op1);
    }
    /// `SSE_PSRLW` (PSRLW). 
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
    /// | 1 | Xmm, Imm |
    /// | 2 | Xmm, Mem |
    /// | 3 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psrlw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsrlwEmitter<A, B> {
        <Self as SsePsrlwEmitter<A, B>>::sse_psrlw(self, op0, op1);
    }
    /// `SSE_PSUBB` (PSUBB). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psubb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsubbEmitter<A, B> {
        <Self as SsePsubbEmitter<A, B>>::sse_psubb(self, op0, op1);
    }
    /// `SSE_PSUBD` (PSUBD). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psubd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsubdEmitter<A, B> {
        <Self as SsePsubdEmitter<A, B>>::sse_psubd(self, op0, op1);
    }
    /// `SSE_PSUBQ` (PSUBQ). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psubq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsubqEmitter<A, B> {
        <Self as SsePsubqEmitter<A, B>>::sse_psubq(self, op0, op1);
    }
    /// `SSE_PSUBSB` (PSUBSB). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psubsb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsubsbEmitter<A, B> {
        <Self as SsePsubsbEmitter<A, B>>::sse_psubsb(self, op0, op1);
    }
    /// `SSE_PSUBSW` (PSUBSW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psubsw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsubswEmitter<A, B> {
        <Self as SsePsubswEmitter<A, B>>::sse_psubsw(self, op0, op1);
    }
    /// `SSE_PSUBUSB` (PSUBUSB). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psubusb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsubusbEmitter<A, B> {
        <Self as SsePsubusbEmitter<A, B>>::sse_psubusb(self, op0, op1);
    }
    /// `SSE_PSUBUSW` (PSUBUSW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psubusw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsubuswEmitter<A, B> {
        <Self as SsePsubuswEmitter<A, B>>::sse_psubusw(self, op0, op1);
    }
    /// `SSE_PSUBW` (PSUBW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_psubw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePsubwEmitter<A, B> {
        <Self as SsePsubwEmitter<A, B>>::sse_psubw(self, op0, op1);
    }
    /// `SSE_PUNPCKHBW` (PUNPCKHBW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_punpckhbw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePunpckhbwEmitter<A, B> {
        <Self as SsePunpckhbwEmitter<A, B>>::sse_punpckhbw(self, op0, op1);
    }
    /// `SSE_PUNPCKHDQ` (PUNPCKHDQ). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_punpckhdq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePunpckhdqEmitter<A, B> {
        <Self as SsePunpckhdqEmitter<A, B>>::sse_punpckhdq(self, op0, op1);
    }
    /// `SSE_PUNPCKHQDQ` (PUNPCKHQDQ). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_punpckhqdq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePunpckhqdqEmitter<A, B> {
        <Self as SsePunpckhqdqEmitter<A, B>>::sse_punpckhqdq(self, op0, op1);
    }
    /// `SSE_PUNPCKHWD` (PUNPCKHWD). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_punpckhwd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePunpckhwdEmitter<A, B> {
        <Self as SsePunpckhwdEmitter<A, B>>::sse_punpckhwd(self, op0, op1);
    }
    /// `SSE_PUNPCKLBW` (PUNPCKLBW). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_punpcklbw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePunpcklbwEmitter<A, B> {
        <Self as SsePunpcklbwEmitter<A, B>>::sse_punpcklbw(self, op0, op1);
    }
    /// `SSE_PUNPCKLDQ` (PUNPCKLDQ). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_punpckldq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePunpckldqEmitter<A, B> {
        <Self as SsePunpckldqEmitter<A, B>>::sse_punpckldq(self, op0, op1);
    }
    /// `SSE_PUNPCKLQDQ` (PUNPCKLQDQ). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_punpcklqdq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePunpcklqdqEmitter<A, B> {
        <Self as SsePunpcklqdqEmitter<A, B>>::sse_punpcklqdq(self, op0, op1);
    }
    /// `SSE_PUNPCKLWD` (PUNPCKLWD). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_punpcklwd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePunpcklwdEmitter<A, B> {
        <Self as SsePunpcklwdEmitter<A, B>>::sse_punpcklwd(self, op0, op1);
    }
    /// `SSE_PXOR` (PXOR). 
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
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_pxor<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SsePxorEmitter<A, B> {
        <Self as SsePxorEmitter<A, B>>::sse_pxor(self, op0, op1);
    }
    /// `SSE_SHUFPD` (SHUFPD). 
    /// Selects a double precision floating-point value of an input pair using a bit control and move to a designated element of the destination operand. The low-to-high order of double precision element of the destination operand is interleaved between the first source operand and the second source operand at the granularity of input pair of 128 bits. Each bit in the imm8 byte, starting from bit 0, is the select control of the corresponding element of the destination to received the shuffled result of an input pair.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SHUFPD.html).
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
    pub fn sse_shufpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: SseShufpdEmitter<A, B, C> {
        <Self as SseShufpdEmitter<A, B, C>>::sse_shufpd(self, op0, op1, op2);
    }
    /// `SSE_SQRTPD` (SQRTPD). 
    /// Performs a SIMD computation of the square roots of the two, four or eight packed double precision floating-point values in the source operand (the second operand) stores the packed double precision floating-point results in the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SQRTPD.html).
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
    pub fn sse_sqrtpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseSqrtpdEmitter<A, B> {
        <Self as SseSqrtpdEmitter<A, B>>::sse_sqrtpd(self, op0, op1);
    }
    /// `SSE_SQRTSD` (SQRTSD). 
    /// Computes the square root of the low double precision floating-point value in the second source operand and stores the double precision floating-point result in the destination operand. The second source operand can be an XMM register or a 64-bit memory location. The first source and destination operands are XMM registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SQRTSD.html).
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
    pub fn sse_sqrtsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseSqrtsdEmitter<A, B> {
        <Self as SseSqrtsdEmitter<A, B>>::sse_sqrtsd(self, op0, op1);
    }
    /// `SSE_SUBPD` (SUBPD). 
    /// Performs a SIMD subtract of the two, four or eight packed double precision floating-point values of the second Source operand from the first Source operand, and stores the packed double precision floating-point results in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SUBPD.html).
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
    pub fn sse_subpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseSubpdEmitter<A, B> {
        <Self as SseSubpdEmitter<A, B>>::sse_subpd(self, op0, op1);
    }
    /// `SSE_SUBSD` (SUBSD). 
    /// Subtract the low double precision floating-point value in the second source operand from the first source operand and stores the double precision floating-point result in the low quadword of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SUBSD.html).
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
    pub fn sse_subsd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseSubsdEmitter<A, B> {
        <Self as SseSubsdEmitter<A, B>>::sse_subsd(self, op0, op1);
    }
    /// `SSE_UCOMISD` (UCOMISD). 
    /// Performs an unordered compare of the double precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal). The OF, SF, and AF flags in the EFLAGS register are set to 0. The unordered result is returned if either source operand is a NaN (QNaN or SNaN).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UCOMISD.html).
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
    pub fn sse_ucomisd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseUcomisdEmitter<A, B> {
        <Self as SseUcomisdEmitter<A, B>>::sse_ucomisd(self, op0, op1);
    }
    /// `SSE_UNPCKHPD` (UNPCKHPD). 
    /// Performs an interleaved unpack of the high double precision floating-point values from the first source operand and the second source operand. See Figure 4-15 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 2B.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UNPCKHPD.html).
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
    pub fn sse_unpckhpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseUnpckhpdEmitter<A, B> {
        <Self as SseUnpckhpdEmitter<A, B>>::sse_unpckhpd(self, op0, op1);
    }
    /// `SSE_UNPCKLPD` (UNPCKLPD). 
    /// Performs an interleaved unpack of the low double precision floating-point values from the first source operand and the second source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UNPCKLPD.html).
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
    pub fn sse_unpcklpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseUnpcklpdEmitter<A, B> {
        <Self as SseUnpcklpdEmitter<A, B>>::sse_unpcklpd(self, op0, op1);
    }
    /// `SSE_XORPD` (XORPD). 
    /// Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPD.html).
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
    pub fn sse_xorpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseXorpdEmitter<A, B> {
        <Self as SseXorpdEmitter<A, B>>::sse_xorpd(self, op0, op1);
    }
}
