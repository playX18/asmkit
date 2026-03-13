use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CLAC` (CLAC). 
/// Clears the AC flag bit in EFLAGS register. This disables any alignment checking of user-mode data accesses. Ifthe SMAP bit is set in the CR4 register, this disallows explicit supervisor-mode data accesses to user-mode pages.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CLAC.html).
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
pub trait ClacEmitter {
    fn clac(&mut self);
}

impl<'a> ClacEmitter for Assembler<'a> {
    fn clac(&mut self) {
        self.emit(CLAC, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `STAC` (STAC). 
/// Sets the AC flag bit in EFLAGS register. This may enable alignment checking of user-mode data accesses. This allows explicit supervisor-mode data accesses to user-mode pages even if the SMAP bit is set in the CR4 register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/STAC.html).
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
pub trait StacEmitter {
    fn stac(&mut self);
}

impl<'a> StacEmitter for Assembler<'a> {
    fn stac(&mut self) {
        self.emit(STAC, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `CLAC` (CLAC). 
    /// Clears the AC flag bit in EFLAGS register. This disables any alignment checking of user-mode data accesses. Ifthe SMAP bit is set in the CR4 register, this disallows explicit supervisor-mode data accesses to user-mode pages.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CLAC.html).
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
    pub fn clac(&mut self)
    where Assembler<'a>: ClacEmitter {
        <Self as ClacEmitter>::clac(self);
    }
    /// `STAC` (STAC). 
    /// Sets the AC flag bit in EFLAGS register. This may enable alignment checking of user-mode data accesses. This allows explicit supervisor-mode data accesses to user-mode pages even if the SMAP bit is set in the CR4 register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/STAC.html).
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
    pub fn stac(&mut self)
    where Assembler<'a>: StacEmitter {
        <Self as StacEmitter>::stac(self);
    }
}
