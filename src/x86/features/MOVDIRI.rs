use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `MOVDIRI` (MOVDIRI). 
/// Moves the doubleword integer in the source operand (second operand) to the destination operand (first operand) using a direct-store operation. The source operand is a general purpose register. The destination operand is a 32-bit memory location. In 64-bit mode, the instruction’s default operation size is 32 bits. Use of the REX.R prefix permits access to additional registers (R8-R15). Use of the REX.W prefix promotes operation to 64 bits. See summary chart at the beginning of this section for encoding data and limits.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDIRI.html).
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
pub trait MovdiriEmitter<A, B> {
    fn movdiri(&mut self, op0: A, op1: B);
}

impl<'a> MovdiriEmitter<Mem, Gpd> for Assembler<'a> {
    fn movdiri(&mut self, op0: Mem, op1: Gpd) {
        self.emit(MOVDIRI32MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> MovdiriEmitter<Mem, Gpq> for Assembler<'a> {
    fn movdiri(&mut self, op0: Mem, op1: Gpq) {
        self.emit(MOVDIRI64MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `MOVDIRI` (MOVDIRI). 
    /// Moves the doubleword integer in the source operand (second operand) to the destination operand (first operand) using a direct-store operation. The source operand is a general purpose register. The destination operand is a 32-bit memory location. In 64-bit mode, the instruction’s default operation size is 32 bits. Use of the REX.R prefix permits access to additional registers (R8-R15). Use of the REX.W prefix promotes operation to 64 bits. See summary chart at the beginning of this section for encoding data and limits.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDIRI.html).
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
    pub fn movdiri<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: MovdiriEmitter<A, B> {
        <Self as MovdiriEmitter<A, B>>::movdiri(self, op0, op1);
    }
}
