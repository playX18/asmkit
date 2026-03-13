use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `INVLPGA`.
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
pub trait InvlpgaEmitter {
    fn invlpga(&mut self);
}

impl<'a> InvlpgaEmitter for Assembler<'a> {
    fn invlpga(&mut self) {
        self.emit(INVLPGA, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `VMLOAD`.
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
pub trait VmloadEmitter {
    fn vmload(&mut self);
}

impl<'a> VmloadEmitter for Assembler<'a> {
    fn vmload(&mut self) {
        self.emit(VMLOAD, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `VMMCALL`.
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
pub trait VmmcallEmitter {
    fn vmmcall(&mut self);
}

impl<'a> VmmcallEmitter for Assembler<'a> {
    fn vmmcall(&mut self) {
        self.emit(VMMCALL, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `VMRUN`.
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
pub trait VmrunEmitter {
    fn vmrun(&mut self);
}

impl<'a> VmrunEmitter for Assembler<'a> {
    fn vmrun(&mut self) {
        self.emit(VMRUN, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `VMSAVE`.
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
pub trait VmsaveEmitter {
    fn vmsave(&mut self);
}

impl<'a> VmsaveEmitter for Assembler<'a> {
    fn vmsave(&mut self) {
        self.emit(VMSAVE, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `INVLPGA`.
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
    pub fn invlpga(&mut self)
    where Assembler<'a>: InvlpgaEmitter {
        <Self as InvlpgaEmitter>::invlpga(self);
    }
    /// `VMLOAD`.
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
    pub fn vmload(&mut self)
    where Assembler<'a>: VmloadEmitter {
        <Self as VmloadEmitter>::vmload(self);
    }
    /// `VMMCALL`.
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
    pub fn vmmcall(&mut self)
    where Assembler<'a>: VmmcallEmitter {
        <Self as VmmcallEmitter>::vmmcall(self);
    }
    /// `VMRUN`.
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
    pub fn vmrun(&mut self)
    where Assembler<'a>: VmrunEmitter {
        <Self as VmrunEmitter>::vmrun(self);
    }
    /// `VMSAVE`.
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
    pub fn vmsave(&mut self)
    where Assembler<'a>: VmsaveEmitter {
        <Self as VmsaveEmitter>::vmsave(self);
    }
}
