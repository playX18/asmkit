use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XRESLDTRK` (XRESLDTRK). 
/// The instruction marks the end of an Intel TSX (RTM) suspend load address tracking region. If the instruction is used inside a suspend load address tracking region it will end the suspend region and all following load addresses will be added to the transaction read set. If this instruction is used inside an active transaction but not in a suspend region it will cause transaction abort.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XRESLDTRK.html).
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
pub trait XresldtrkEmitter {
    fn xresldtrk(&mut self);
}

impl<'a> XresldtrkEmitter for Assembler<'a> {
    fn xresldtrk(&mut self) {
        self.emit(XRESLDTRK, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `XSUSLDTRK` (XSUSLDTRK). 
/// The instruction marks the start of an Intel TSX (RTM) suspend load address tracking region. If the instruction is used inside a transactional region, subsequent loads are not added to the read set of the transaction. If the instruction is used inside a suspend load address tracking region it will cause transaction abort.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XSUSLDTRK.html).
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
pub trait XsusldtrkEmitter {
    fn xsusldtrk(&mut self);
}

impl<'a> XsusldtrkEmitter for Assembler<'a> {
    fn xsusldtrk(&mut self) {
        self.emit(XSUSLDTRK, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `XRESLDTRK` (XRESLDTRK). 
    /// The instruction marks the end of an Intel TSX (RTM) suspend load address tracking region. If the instruction is used inside a suspend load address tracking region it will end the suspend region and all following load addresses will be added to the transaction read set. If this instruction is used inside an active transaction but not in a suspend region it will cause transaction abort.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XRESLDTRK.html).
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
    pub fn xresldtrk(&mut self)
    where Assembler<'a>: XresldtrkEmitter {
        <Self as XresldtrkEmitter>::xresldtrk(self);
    }
    /// `XSUSLDTRK` (XSUSLDTRK). 
    /// The instruction marks the start of an Intel TSX (RTM) suspend load address tracking region. If the instruction is used inside a transactional region, subsequent loads are not added to the read set of the transaction. If the instruction is used inside a suspend load address tracking region it will cause transaction abort.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XSUSLDTRK.html).
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
    pub fn xsusldtrk(&mut self)
    where Assembler<'a>: XsusldtrkEmitter {
        <Self as XsusldtrkEmitter>::xsusldtrk(self);
    }
}
