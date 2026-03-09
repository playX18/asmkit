use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `SSE_EXTRQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Imm |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseExtrqEmitter<A, B> {
    fn sse_extrq(&mut self, op0: A, op1: B);
}

impl<'a> SseExtrqEmitter<Xmm, Imm> for Assembler<'a> {
    fn sse_extrq(&mut self, op0: Xmm, op1: Imm) {
        self.emit(
            SSE_EXTRQRI,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> SseExtrqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_extrq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SSE_EXTRQRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_INSERTQ`.
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
pub trait SseInsertqEmitter_2<A, B> {
    fn sse_insertq_2(&mut self, op0: A, op1: B);
}

impl<'a> SseInsertqEmitter_2<Xmm, Xmm> for Assembler<'a> {
    fn sse_insertq_2(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(
            SSE_INSERTQRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `SSE_INSERTQ`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Imm |
/// +---+---------------+
/// ```
pub trait SseInsertqEmitter_3<A, B, C> {
    fn sse_insertq_3(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> SseInsertqEmitter_3<Xmm, Xmm, Imm> for Assembler<'a> {
    fn sse_insertq_3(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(
            SSE_INSERTQRRI,
            op0.as_operand(),
            op1.as_operand(),
            op2.as_operand(),
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `SSE_EXTRQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Imm |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_extrq<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseExtrqEmitter<A, B>,
    {
        <Self as SseExtrqEmitter<A, B>>::sse_extrq(self, op0, op1);
    }
    /// `SSE_INSERTQ`.
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
    pub fn sse_insertq_2<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: SseInsertqEmitter_2<A, B>,
    {
        <Self as SseInsertqEmitter_2<A, B>>::sse_insertq_2(self, op0, op1);
    }
    /// `SSE_INSERTQ`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn sse_insertq_3<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where
        Assembler<'a>: SseInsertqEmitter_3<A, B, C>,
    {
        <Self as SseInsertqEmitter_3<A, B, C>>::sse_insertq_3(self, op0, op1, op2);
    }
}
