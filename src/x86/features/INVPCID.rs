use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `INVPCID` (INVPCID). 
/// Invalidates mappings in the translation lookaside buffers (TLBs) and paging-structure caches based on process-context identifier (PCID). (See Section 4.10, “Caching Translation Information,” in the Intel 64 and IA-32 Architecture Software Developer’s Manual, Volume 3A.) Invalidation is based on the INVPCID type specified in the register operand and the INVPCID descriptor specified in the memory operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/INVPCID.html).
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
pub trait InvpcidEmitter<A, B> {
    fn invpcid(&mut self, op0: A, op1: B);
}

impl<'a> InvpcidEmitter<Gpq, Mem> for Assembler<'a> {
    fn invpcid(&mut self, op0: Gpq, op1: Mem) {
        self.emit(INVPCIDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `INVPCID` (INVPCID). 
    /// Invalidates mappings in the translation lookaside buffers (TLBs) and paging-structure caches based on process-context identifier (PCID). (See Section 4.10, “Caching Translation Information,” in the Intel 64 and IA-32 Architecture Software Developer’s Manual, Volume 3A.) Invalidation is based on the INVPCID type specified in the register operand and the INVPCID descriptor specified in the memory operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/INVPCID.html).
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
    pub fn invpcid<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: InvpcidEmitter<A, B> {
        <Self as InvpcidEmitter<A, B>>::invpcid(self, op0, op1);
    }
}
