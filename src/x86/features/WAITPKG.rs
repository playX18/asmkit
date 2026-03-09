use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `TPAUSE`.
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

/// `UMONITOR`.
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

/// `UMWAIT`.
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
    /// `TPAUSE`.
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
    where
        Assembler<'a>: TpauseEmitter<A>,
    {
        <Self as TpauseEmitter<A>>::tpause(self, op0);
    }
    /// `UMONITOR`.
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
    where
        Assembler<'a>: UmonitorEmitter<A>,
    {
        <Self as UmonitorEmitter<A>>::umonitor(self, op0);
    }
    /// `UMWAIT`.
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
    where
        Assembler<'a>: UmwaitEmitter<A>,
    {
        <Self as UmwaitEmitter<A>>::umwait(self, op0);
    }
}
