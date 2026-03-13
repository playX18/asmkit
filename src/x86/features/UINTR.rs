use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CLUI` (CLUI). 
/// CLUI clears the user interrupt flag (UIF). Its effect takes place immediately: a user interrupt cannot be delivered on the instruction boundary following CLUI.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CLUI.html).
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

/// `SENDUIPI` (SENDUIPI). 
/// The SENDUIPI instruction sends the user interprocessor interrupt (IPI) indicated by its register operand. (The operand always has 64 bits; operand-size overrides such as the prefix 66 are ignored.)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SENDUIPI.html).
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

/// `STUI` (STUI). 
/// STUI sets the user interrupt flag (UIF). Its effect takes place immediately; a user interrupt may be delivered on the instruction boundary following STUI. (This is in contrast with STI, whose effect is delayed by one instruction).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/STUI.html).
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

/// `UIRET` (UIRET). 
/// UIRET returns from the handling of a user interrupt. It can be executed regardless of CPL.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UIRET.html).
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
    /// `CLUI` (CLUI). 
    /// CLUI clears the user interrupt flag (UIF). Its effect takes place immediately: a user interrupt cannot be delivered on the instruction boundary following CLUI.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CLUI.html).
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
    where Assembler<'a>: CluiEmitter {
        <Self as CluiEmitter>::clui(self);
    }
    /// `SENDUIPI` (SENDUIPI). 
    /// The SENDUIPI instruction sends the user interprocessor interrupt (IPI) indicated by its register operand. (The operand always has 64 bits; operand-size overrides such as the prefix 66 are ignored.)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SENDUIPI.html).
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
    where Assembler<'a>: SenduipiEmitter<A> {
        <Self as SenduipiEmitter<A>>::senduipi(self, op0);
    }
    /// `STUI` (STUI). 
    /// STUI sets the user interrupt flag (UIF). Its effect takes place immediately; a user interrupt may be delivered on the instruction boundary following STUI. (This is in contrast with STI, whose effect is delayed by one instruction).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/STUI.html).
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
    where Assembler<'a>: StuiEmitter {
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
    where Assembler<'a>: TestuiEmitter {
        <Self as TestuiEmitter>::testui(self);
    }
    /// `UIRET` (UIRET). 
    /// UIRET returns from the handling of a user interrupt. It can be executed regardless of CPL.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/UIRET.html).
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
    where Assembler<'a>: UiretEmitter {
        <Self as UiretEmitter>::uiret(self);
    }
}
