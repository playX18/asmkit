use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PREFETCHW` (PREFETCHW). 
/// Fetches the cache line of data from memory that contains the byte specified with the source operand to a location in the 1st or 2nd level cache and invalidates other cached instances of the line.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHW.html).
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
pub trait PrefetchwEmitter<A> {
    fn prefetchw(&mut self, op0: A);
}

impl<'a> PrefetchwEmitter<Mem> for Assembler<'a> {
    fn prefetchw(&mut self, op0: Mem) {
        self.emit(PREFETCHWM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `PREFETCHW` (PREFETCHW). 
    /// Fetches the cache line of data from memory that contains the byte specified with the source operand to a location in the 1st or 2nd level cache and invalidates other cached instances of the line.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHW.html).
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
    pub fn prefetchw<A>(&mut self, op0: A)
    where Assembler<'a>: PrefetchwEmitter<A> {
        <Self as PrefetchwEmitter<A>>::prefetchw(self, op0);
    }
}
