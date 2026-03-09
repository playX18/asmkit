use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `MMX_PABSB`.
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
        self.emit(
            MMX_PABSBRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPabsbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pabsb(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PABSBRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PABSD`.
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
        self.emit(
            MMX_PABSDRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPabsdEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pabsd(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PABSDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PABSW`.
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
        self.emit(
            MMX_PABSWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPabswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pabsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PABSWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PALIGNR`.
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
        self.emit(
            MMX_PALIGNRRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> MmxPalignrEmitter<Mm, Mem, Imm> for Assembler<'a> {
    fn mmx_palignr(&mut self, op0: Mm, op1: Mem, op2: Imm) {
        self.emit(
            MMX_PALIGNRRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `MMX_PHADDD`.
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
        self.emit(
            MMX_PHADDDRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPhadddEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phaddd(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PHADDDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PHADDSW`.
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
        self.emit(
            MMX_PHADDSWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPhaddswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phaddsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PHADDSWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PHADDW`.
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
        self.emit(
            MMX_PHADDWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPhaddwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phaddw(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PHADDWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PHSUBD`.
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
        self.emit(
            MMX_PHSUBDRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPhsubdEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phsubd(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PHSUBDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PHSUBSW`.
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
        self.emit(
            MMX_PHSUBSWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPhsubswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phsubsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PHSUBSWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PHSUBW`.
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
        self.emit(
            MMX_PHSUBWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPhsubwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_phsubw(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PHSUBWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PMADDUBSW`.
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
        self.emit(
            MMX_PMADDUBSWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPmaddubswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pmaddubsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PMADDUBSWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PMULHRSW`.
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
        self.emit(
            MMX_PMULHRSWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPmulhrswEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pmulhrsw(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PMULHRSWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PSHUFB`.
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
        self.emit(
            MMX_PSHUFBRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPshufbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_pshufb(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PSHUFBRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PSIGNB`.
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
        self.emit(
            MMX_PSIGNBRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPsignbEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psignb(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PSIGNBRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PSIGND`.
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
        self.emit(
            MMX_PSIGNDRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPsigndEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psignd(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PSIGNDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `MMX_PSIGNW`.
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
        self.emit(
            MMX_PSIGNWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> MmxPsignwEmitter<Mm, Mem> for Assembler<'a> {
    fn mmx_psignw(&mut self, op0: Mm, op1: Mem) {
        self.emit(
            MMX_PSIGNWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PABSB`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PABSBRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePabsbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pabsb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PABSBRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PABSD`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PABSDRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePabsdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pabsd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PABSDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PABSW`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PABSWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePabswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pabsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PABSWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PALIGNR`.
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
        self.emit(
            SSE_PALIGNRRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> SsePalignrEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn sse_palignr(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(
            SSE_PALIGNRRMI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

/// `SSE_PHADDD`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PHADDDRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePhadddEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phaddd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PHADDDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PHADDSW`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PHADDSWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePhaddswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phaddsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PHADDSWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PHADDW`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PHADDWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePhaddwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phaddw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PHADDWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PHSUBD`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PHSUBDRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePhsubdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phsubd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PHSUBDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PHSUBSW`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PHSUBSWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePhsubswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phsubsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PHSUBSWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PHSUBW`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PHSUBWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePhsubwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_phsubw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PHSUBWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PMADDUBSW`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PMADDUBSWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePmaddubswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmaddubsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PMADDUBSWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PMULHRSW`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PMULHRSWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePmulhrswEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pmulhrsw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PMULHRSWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PSHUFB`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PSHUFBRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePshufbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_pshufb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PSHUFBRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PSIGNB`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PSIGNBRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePsignbEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psignb(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PSIGNBRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PSIGND`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PSIGNDRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePsigndEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psignd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PSIGNDRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_PSIGNW`.
///
/// Supported operand variants:
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
        self.emit(
            SSE_PSIGNWRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SsePsignwEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_psignw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(
            SSE_PSIGNWRM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `MMX_PABSB`.
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
    where
        Assembler<'a>: MmxPabsbEmitter<A, B>,
    {
        <Self as MmxPabsbEmitter<A, B>>::mmx_pabsb(self, op0, op1);
    }
    /// `MMX_PABSD`.
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
    where
        Assembler<'a>: MmxPabsdEmitter<A, B>,
    {
        <Self as MmxPabsdEmitter<A, B>>::mmx_pabsd(self, op0, op1);
    }
    /// `MMX_PABSW`.
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
    where
        Assembler<'a>: MmxPabswEmitter<A, B>,
    {
        <Self as MmxPabswEmitter<A, B>>::mmx_pabsw(self, op0, op1);
    }
    /// `MMX_PALIGNR`.
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
    where
        Assembler<'a>: MmxPalignrEmitter<A, B, C>,
    {
        <Self as MmxPalignrEmitter<A, B, C>>::mmx_palignr(self, op0, op1, op2);
    }
    /// `MMX_PHADDD`.
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
    where
        Assembler<'a>: MmxPhadddEmitter<A, B>,
    {
        <Self as MmxPhadddEmitter<A, B>>::mmx_phaddd(self, op0, op1);
    }
    /// `MMX_PHADDSW`.
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
    where
        Assembler<'a>: MmxPhaddswEmitter<A, B>,
    {
        <Self as MmxPhaddswEmitter<A, B>>::mmx_phaddsw(self, op0, op1);
    }
    /// `MMX_PHADDW`.
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
    where
        Assembler<'a>: MmxPhaddwEmitter<A, B>,
    {
        <Self as MmxPhaddwEmitter<A, B>>::mmx_phaddw(self, op0, op1);
    }
    /// `MMX_PHSUBD`.
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
    where
        Assembler<'a>: MmxPhsubdEmitter<A, B>,
    {
        <Self as MmxPhsubdEmitter<A, B>>::mmx_phsubd(self, op0, op1);
    }
    /// `MMX_PHSUBSW`.
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
    where
        Assembler<'a>: MmxPhsubswEmitter<A, B>,
    {
        <Self as MmxPhsubswEmitter<A, B>>::mmx_phsubsw(self, op0, op1);
    }
    /// `MMX_PHSUBW`.
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
    where
        Assembler<'a>: MmxPhsubwEmitter<A, B>,
    {
        <Self as MmxPhsubwEmitter<A, B>>::mmx_phsubw(self, op0, op1);
    }
    /// `MMX_PMADDUBSW`.
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
    where
        Assembler<'a>: MmxPmaddubswEmitter<A, B>,
    {
        <Self as MmxPmaddubswEmitter<A, B>>::mmx_pmaddubsw(self, op0, op1);
    }
    /// `MMX_PMULHRSW`.
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
    where
        Assembler<'a>: MmxPmulhrswEmitter<A, B>,
    {
        <Self as MmxPmulhrswEmitter<A, B>>::mmx_pmulhrsw(self, op0, op1);
    }
    /// `MMX_PSHUFB`.
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
    where
        Assembler<'a>: MmxPshufbEmitter<A, B>,
    {
        <Self as MmxPshufbEmitter<A, B>>::mmx_pshufb(self, op0, op1);
    }
    /// `MMX_PSIGNB`.
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
    where
        Assembler<'a>: MmxPsignbEmitter<A, B>,
    {
        <Self as MmxPsignbEmitter<A, B>>::mmx_psignb(self, op0, op1);
    }
    /// `MMX_PSIGND`.
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
    where
        Assembler<'a>: MmxPsigndEmitter<A, B>,
    {
        <Self as MmxPsigndEmitter<A, B>>::mmx_psignd(self, op0, op1);
    }
    /// `MMX_PSIGNW`.
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
    where
        Assembler<'a>: MmxPsignwEmitter<A, B>,
    {
        <Self as MmxPsignwEmitter<A, B>>::mmx_psignw(self, op0, op1);
    }
    /// `SSE_PABSB`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePabsbEmitter<A, B>,
    {
        <Self as SsePabsbEmitter<A, B>>::sse_pabsb(self, op0, op1);
    }
    /// `SSE_PABSD`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePabsdEmitter<A, B>,
    {
        <Self as SsePabsdEmitter<A, B>>::sse_pabsd(self, op0, op1);
    }
    /// `SSE_PABSW`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePabswEmitter<A, B>,
    {
        <Self as SsePabswEmitter<A, B>>::sse_pabsw(self, op0, op1);
    }
    /// `SSE_PALIGNR`.
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
    where
        Assembler<'a>: SsePalignrEmitter<A, B, C>,
    {
        <Self as SsePalignrEmitter<A, B, C>>::sse_palignr(self, op0, op1, op2);
    }
    /// `SSE_PHADDD`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePhadddEmitter<A, B>,
    {
        <Self as SsePhadddEmitter<A, B>>::sse_phaddd(self, op0, op1);
    }
    /// `SSE_PHADDSW`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePhaddswEmitter<A, B>,
    {
        <Self as SsePhaddswEmitter<A, B>>::sse_phaddsw(self, op0, op1);
    }
    /// `SSE_PHADDW`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePhaddwEmitter<A, B>,
    {
        <Self as SsePhaddwEmitter<A, B>>::sse_phaddw(self, op0, op1);
    }
    /// `SSE_PHSUBD`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePhsubdEmitter<A, B>,
    {
        <Self as SsePhsubdEmitter<A, B>>::sse_phsubd(self, op0, op1);
    }
    /// `SSE_PHSUBSW`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePhsubswEmitter<A, B>,
    {
        <Self as SsePhsubswEmitter<A, B>>::sse_phsubsw(self, op0, op1);
    }
    /// `SSE_PHSUBW`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePhsubwEmitter<A, B>,
    {
        <Self as SsePhsubwEmitter<A, B>>::sse_phsubw(self, op0, op1);
    }
    /// `SSE_PMADDUBSW`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePmaddubswEmitter<A, B>,
    {
        <Self as SsePmaddubswEmitter<A, B>>::sse_pmaddubsw(self, op0, op1);
    }
    /// `SSE_PMULHRSW`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePmulhrswEmitter<A, B>,
    {
        <Self as SsePmulhrswEmitter<A, B>>::sse_pmulhrsw(self, op0, op1);
    }
    /// `SSE_PSHUFB`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePshufbEmitter<A, B>,
    {
        <Self as SsePshufbEmitter<A, B>>::sse_pshufb(self, op0, op1);
    }
    /// `SSE_PSIGNB`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePsignbEmitter<A, B>,
    {
        <Self as SsePsignbEmitter<A, B>>::sse_psignb(self, op0, op1);
    }
    /// `SSE_PSIGND`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePsigndEmitter<A, B>,
    {
        <Self as SsePsigndEmitter<A, B>>::sse_psignd(self, op0, op1);
    }
    /// `SSE_PSIGNW`.
    ///
    /// Supported operand variants:
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
    where
        Assembler<'a>: SsePsignwEmitter<A, B>,
    {
        <Self as SsePsignwEmitter<A, B>>::sse_psignw(self, op0, op1);
    }
}
