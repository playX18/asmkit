use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CLUI`.
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
pub trait CluiEmitter {
    fn clui(&mut self);
}

impl<'a> CluiEmitter for Assembler<'a> {
    fn clui(&mut self) {
        self.emit(CLUI, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `SENDUIPI`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpq      |
/// +---+----------+
/// ```
pub trait SenduipiEmitter<A> {
    fn senduipi(&mut self, op0: A);
}

impl<'a> SenduipiEmitter<Gpq> for Assembler<'a> {
    fn senduipi(&mut self, op0: Gpq) {
        self.emit(SENDUIPIR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `STUI`.
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
pub trait StuiEmitter {
    fn stui(&mut self);
}

impl<'a> StuiEmitter for Assembler<'a> {
    fn stui(&mut self) {
        self.emit(STUI, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `TESTUI`.
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
pub trait TestuiEmitter {
    fn testui(&mut self);
}

impl<'a> TestuiEmitter for Assembler<'a> {
    fn testui(&mut self) {
        self.emit(TESTUI, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `UIRET`.
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
pub trait UiretEmitter {
    fn uiret(&mut self);
}

impl<'a> UiretEmitter for Assembler<'a> {
    fn uiret(&mut self) {
        self.emit(UIRET, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `CLUI`.
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
    pub fn clui(&mut self)
    where
        Assembler<'a>: CluiEmitter,
    {
        <Self as CluiEmitter>::clui(self);
    }
    /// `SENDUIPI`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpq      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn senduipi<A>(&mut self, op0: A)
    where
        Assembler<'a>: SenduipiEmitter<A>,
    {
        <Self as SenduipiEmitter<A>>::senduipi(self, op0);
    }
    /// `STUI`.
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
    pub fn stui(&mut self)
    where
        Assembler<'a>: StuiEmitter,
    {
        <Self as StuiEmitter>::stui(self);
    }
    /// `TESTUI`.
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
    pub fn testui(&mut self)
    where
        Assembler<'a>: TestuiEmitter,
    {
        <Self as TestuiEmitter>::testui(self);
    }
    /// `UIRET`.
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
    pub fn uiret(&mut self)
    where
        Assembler<'a>: UiretEmitter,
    {
        <Self as UiretEmitter>::uiret(self);
    }
}
