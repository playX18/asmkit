use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `PREFETCHWT1` (PREFETCHWT1). 
/// Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by an intent to write hint (so that data is brought into ‘Exclusive’ state via a request for ownership) and a locality hint
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHWT1.html).
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
pub trait Prefetchwt1Emitter<A> {
    fn prefetchwt1(&mut self, op0: A);
}

impl<'a> Prefetchwt1Emitter<Mem> for Assembler<'a> {
    fn prefetchwt1(&mut self, op0: Mem) {
        self.emit(PREFETCHWT1M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `PREFETCHWT1` (PREFETCHWT1). 
    /// Fetches the line of data from memory that contains the byte specified with the source operand to a location in the cache hierarchy specified by an intent to write hint (so that data is brought into ‘Exclusive’ state via a request for ownership) and a locality hint
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PREFETCHWT1.html).
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
    pub fn prefetchwt1<A>(&mut self, op0: A)
    where Assembler<'a>: Prefetchwt1Emitter<A> {
        <Self as Prefetchwt1Emitter<A>>::prefetchwt1(self, op0);
    }
}
