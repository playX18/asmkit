use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XABORT` (XABORT). 
/// XABORT forces an RTM abort. Following an RTM abort, the logical processor resumes execution at the fallback address computed through the outermost XBEGIN instruction. The EAX register is updated to reflect an XABORT instruction caused the abort, and the imm8 argument will be provided in bits 31:24 of EAX.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XABORT.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Imm      |
/// +---+----------+
/// ```
pub trait XabortEmitter<A> {
    fn xabort(&mut self, op0: A);
}

impl<'a> XabortEmitter<Imm> for Assembler<'a> {
    fn xabort(&mut self, op0: Imm) {
        self.emit(XABORTI, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `XBEGIN` (XBEGIN). 
/// The XBEGIN instruction specifies the start of an RTM code region. If the logical processor was not already in transactional execution, then the XBEGIN instruction causes the logical processor to transition into transactional execution. The XBEGIN instruction that transitions the logical processor into transactional execution is referred to as the outermost XBEGIN instruction. The instruction also specifies a relative offset to compute the address of the fallback code path following a transactional abort. (Use of the 16-bit operand size does not cause this address to be truncated to 16 bits, unlike a near jump to a relative offset.)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XBEGIN.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Imm      |
/// | 2 | Label    |
/// | 3 | Sym      |
/// +---+----------+
/// ```
pub trait XbeginEmitter<A> {
    fn xbegin(&mut self, op0: A);
}

impl<'a> XbeginEmitter<Imm> for Assembler<'a> {
    fn xbegin(&mut self, op0: Imm) {
        self.emit(XBEGIN, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> XbeginEmitter<Sym> for Assembler<'a> {
    fn xbegin(&mut self, op0: Sym) {
        self.emit(XBEGIN, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> XbeginEmitter<Label> for Assembler<'a> {
    fn xbegin(&mut self, op0: Label) {
        self.emit(XBEGIN, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `XEND` (XEND). 
/// The instruction marks the end of an RTM code region. If this corresponds to the outermost scope (that is, including this XEND instruction, the number of XBEGIN instructions is the same as number of XEND instructions), the logical processor will attempt to commit the logical processor state atomically. If the commit fails, the logical processor will rollback all architectural register and memory updates performed during the RTM execution. The logical processor will resume execution at the fallback address computed from the outermost XBEGIN instruction. The EAX register is updated to reflect RTM abort information.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XEND.html).
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
pub trait XendEmitter {
    fn xend(&mut self);
}

impl<'a> XendEmitter for Assembler<'a> {
    fn xend(&mut self) {
        self.emit(XEND, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `XTEST` (XTEST). 
/// The XTEST instruction queries the transactional execution status. If the instruction executes inside a transactionally executing RTM region or a transactionally executing HLE region, then the ZF flag is cleared, else it is set.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XTEST.html).
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
pub trait XtestEmitter {
    fn xtest(&mut self);
}

impl<'a> XtestEmitter for Assembler<'a> {
    fn xtest(&mut self) {
        self.emit(XTEST, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `XABORT` (XABORT). 
    /// XABORT forces an RTM abort. Following an RTM abort, the logical processor resumes execution at the fallback address computed through the outermost XBEGIN instruction. The EAX register is updated to reflect an XABORT instruction caused the abort, and the imm8 argument will be provided in bits 31:24 of EAX.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XABORT.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Imm      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn xabort<A>(&mut self, op0: A)
    where Assembler<'a>: XabortEmitter<A> {
        <Self as XabortEmitter<A>>::xabort(self, op0);
    }
    /// `XBEGIN` (XBEGIN). 
    /// The XBEGIN instruction specifies the start of an RTM code region. If the logical processor was not already in transactional execution, then the XBEGIN instruction causes the logical processor to transition into transactional execution. The XBEGIN instruction that transitions the logical processor into transactional execution is referred to as the outermost XBEGIN instruction. The instruction also specifies a relative offset to compute the address of the fallback code path following a transactional abort. (Use of the 16-bit operand size does not cause this address to be truncated to 16 bits, unlike a near jump to a relative offset.)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XBEGIN.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Imm      |
    /// | 2 | Label    |
    /// | 3 | Sym      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn xbegin<A>(&mut self, op0: A)
    where Assembler<'a>: XbeginEmitter<A> {
        <Self as XbeginEmitter<A>>::xbegin(self, op0);
    }
    /// `XEND` (XEND). 
    /// The instruction marks the end of an RTM code region. If this corresponds to the outermost scope (that is, including this XEND instruction, the number of XBEGIN instructions is the same as number of XEND instructions), the logical processor will attempt to commit the logical processor state atomically. If the commit fails, the logical processor will rollback all architectural register and memory updates performed during the RTM execution. The logical processor will resume execution at the fallback address computed from the outermost XBEGIN instruction. The EAX register is updated to reflect RTM abort information.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XEND.html).
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
    pub fn xend(&mut self)
    where Assembler<'a>: XendEmitter {
        <Self as XendEmitter>::xend(self);
    }
    /// `XTEST` (XTEST). 
    /// The XTEST instruction queries the transactional execution status. If the instruction executes inside a transactionally executing RTM region or a transactionally executing HLE region, then the ZF flag is cleared, else it is set.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XTEST.html).
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
    pub fn xtest(&mut self)
    where Assembler<'a>: XtestEmitter {
        <Self as XtestEmitter>::xtest(self);
    }
}
