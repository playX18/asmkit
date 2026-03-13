use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `WBNOINVD` (WBNOINVD). 
/// The WBNOINVD instruction writes back all modified cache lines in the processor’s internal cache to main memory but does not invalidate (flush) the internal caches.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/WBNOINVD.html).
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
pub trait WbnoinvdEmitter {
    fn wbnoinvd(&mut self);
}

impl<'a> WbnoinvdEmitter for Assembler<'a> {
    fn wbnoinvd(&mut self) {
        self.emit(WBNOINVD, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `WBNOINVD` (WBNOINVD). 
    /// The WBNOINVD instruction writes back all modified cache lines in the processor’s internal cache to main memory but does not invalidate (flush) the internal caches.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/WBNOINVD.html).
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
    pub fn wbnoinvd(&mut self)
    where Assembler<'a>: WbnoinvdEmitter {
        <Self as WbnoinvdEmitter>::wbnoinvd(self);
    }
}
