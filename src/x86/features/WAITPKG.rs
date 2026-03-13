use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `TPAUSE` (TPAUSE). 
/// TPAUSE instructs the processor to enter an implementation-dependent optimized state. There are two such optimized states to choose from: light-weight power/performance optimized state, and improved power/performance optimized state. The selection between the two is governed by the explicit input register bit[0] source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/TPAUSE.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// +---+----------+
/// ```
pub trait TpauseEmitter<A> {
    fn tpause(&mut self, op0: A);
}

impl<'a> TpauseEmitter<Gpd> for Assembler<'a> {
    fn tpause(&mut self, op0: Gpd) {
        self.emit(TPAUSER, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `UMONITOR` (UMONITOR). 
/// The UMONITOR instruction arms address monitoring hardware using an address specified in the source register (the address range that the monitoring hardware checks for store operations can be determined by using the CPUID monitor leaf function, EAX=05H). A store to an address within the specified address range triggers the monitoring hardware. The state of monitor hardware is used by UMWAIT.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UMONITOR.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// +---+----------+
/// ```
pub trait UmonitorEmitter<A> {
    fn umonitor(&mut self, op0: A);
}

impl<'a> UmonitorEmitter<Gpd> for Assembler<'a> {
    fn umonitor(&mut self, op0: Gpd) {
        self.emit(UMONITOR32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> UmonitorEmitter<Gpq> for Assembler<'a> {
    fn umonitor(&mut self, op0: Gpq) {
        self.emit(UMONITOR64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `UMWAIT` (UMWAIT). 
/// UMWAIT instructs the processor to enter an implementation-dependent optimized state while monitoring a range of addresses. The optimized state may be either a light-weight power/performance optimized state or an improved power/performance optimized state. The selection between the two states is governed by the explicit input register bit[0] source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UMWAIT.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// +---+----------+
/// ```
pub trait UmwaitEmitter<A> {
    fn umwait(&mut self, op0: A);
}

impl<'a> UmwaitEmitter<Gpd> for Assembler<'a> {
    fn umwait(&mut self, op0: Gpd) {
        self.emit(UMWAITR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `TPAUSE` (TPAUSE). 
    /// TPAUSE instructs the processor to enter an implementation-dependent optimized state. There are two such optimized states to choose from: light-weight power/performance optimized state, and improved power/performance optimized state. The selection between the two is governed by the explicit input register bit[0] source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/TPAUSE.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn tpause<A>(&mut self, op0: A)
    where Assembler<'a>: TpauseEmitter<A> {
        <Self as TpauseEmitter<A>>::tpause(self, op0);
    }
    /// `UMONITOR` (UMONITOR). 
    /// The UMONITOR instruction arms address monitoring hardware using an address specified in the source register (the address range that the monitoring hardware checks for store operations can be determined by using the CPUID monitor leaf function, EAX=05H). A store to an address within the specified address range triggers the monitoring hardware. The state of monitor hardware is used by UMWAIT.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UMONITOR.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn umonitor<A>(&mut self, op0: A)
    where Assembler<'a>: UmonitorEmitter<A> {
        <Self as UmonitorEmitter<A>>::umonitor(self, op0);
    }
    /// `UMWAIT` (UMWAIT). 
    /// UMWAIT instructs the processor to enter an implementation-dependent optimized state while monitoring a range of addresses. The optimized state may be either a light-weight power/performance optimized state or an improved power/performance optimized state. The selection between the two states is governed by the explicit input register bit[0] source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UMWAIT.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn umwait<A>(&mut self, op0: A)
    where Assembler<'a>: UmwaitEmitter<A> {
        <Self as UmwaitEmitter<A>>::umwait(self, op0);
    }
}
