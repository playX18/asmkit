use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `MOVDIR64B` (MOVDIR64B). 
/// Moves 64-bytes as direct-store with 64-byte write atomicity from source memory address to destination memory address. The source operand is a normal memory operand. The destination operand is a memory location specified in a general-purpose register. The register content is interpreted as an offset into ES segment without any segment override. In 64-bit mode, the register operand width is 64-bits (32-bits with 67H prefix). Outside of 64-bit mode, the register width is 32-bits when CS.D=1 (16-bits with 67H prefix), and 16-bits when CS.D=0 (32-bits with 67H prefix). MOVDIR64B requires the destination address to be 64-byte aligned. No alignment restriction is enforced for source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDIR64B.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait Movdir64bEmitter<A, B> {
    fn movdir64b(&mut self, op0: A, op1: B);
}

impl<'a> Movdir64bEmitter<Gpq, Mem> for Assembler<'a> {
    fn movdir64b(&mut self, op0: Gpq, op1: Mem) {
        self.emit(MOVDIR64BRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `MOVDIR64B` (MOVDIR64B). 
    /// Moves 64-bytes as direct-store with 64-byte write atomicity from source memory address to destination memory address. The source operand is a normal memory operand. The destination operand is a memory location specified in a general-purpose register. The register content is interpreted as an offset into ES segment without any segment override. In 64-bit mode, the register operand width is 64-bits (32-bits with 67H prefix). Outside of 64-bit mode, the register width is 32-bits when CS.D=1 (16-bits with 67H prefix), and 16-bits when CS.D=0 (32-bits with 67H prefix). MOVDIR64B requires the destination address to be 64-byte aligned. No alignment restriction is enforced for source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDIR64B.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn movdir64b<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Movdir64bEmitter<A, B> {
        <Self as Movdir64bEmitter<A, B>>::movdir64b(self, op0, op1);
    }
}
