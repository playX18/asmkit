use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CLDEMOTE` (CLDEMOTE). 
/// Hints to hardware that the cache line that contains the linear address specified with the memory operand should be moved (“demoted”) from the cache(s) closest to the processor core to a level more distant from the processor core. This may accelerate subsequent accesses to the line by other cores in the same coherence domain, especially if the line was written by the core that demotes the line. Moving the line in such a manner is a performance optimization, i.e., it is a hint which does not modify architectural state. Hardware may choose which level in the cache hierarchy to retain the line (e.g., L3 in typical server designs). The source operand is a byte memory location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CLDEMOTE.html).
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
pub trait CldemoteEmitter<A> {
    fn cldemote(&mut self, op0: A);
}

impl<'a> CldemoteEmitter<Mem> for Assembler<'a> {
    fn cldemote(&mut self, op0: Mem) {
        self.emit(CLDEMOTEM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `CLDEMOTE` (CLDEMOTE). 
    /// Hints to hardware that the cache line that contains the linear address specified with the memory operand should be moved (“demoted”) from the cache(s) closest to the processor core to a level more distant from the processor core. This may accelerate subsequent accesses to the line by other cores in the same coherence domain, especially if the line was written by the core that demotes the line. Moving the line in such a manner is a performance optimization, i.e., it is a hint which does not modify architectural state. Hardware may choose which level in the cache hierarchy to retain the line (e.g., L3 in typical server designs). The source operand is a byte memory location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CLDEMOTE.html).
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
    pub fn cldemote<A>(&mut self, op0: A)
    where Assembler<'a>: CldemoteEmitter<A> {
        <Self as CldemoteEmitter<A>>::cldemote(self, op0);
    }
}
