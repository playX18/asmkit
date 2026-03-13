use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `ENCLS`.
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
pub trait EnclsEmitter {
    fn encls(&mut self);
}

impl<'a> EnclsEmitter for Assembler<'a> {
    fn encls(&mut self) {
        self.emit(ENCLS, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `ENCLU`.
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
pub trait EncluEmitter {
    fn enclu(&mut self);
}

impl<'a> EncluEmitter for Assembler<'a> {
    fn enclu(&mut self) {
        self.emit(ENCLU, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `ENCLV` (ENCLV). 
/// The ENCLV instruction invokes the virtualization SGX leaf functions for managing enclaves in a virtualized environment. Software specifies the leaf function by setting the appropriate value in the register EAX as input. The registers RBX, RCX, and RDX have leaf-specific purpose, and may act as input, as output, or may be unused. In non 64-bit mode, the instruction ignores upper 32 bits of the RAX register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ENCLV.html).
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
pub trait EnclvEmitter {
    fn enclv(&mut self);
}

impl<'a> EnclvEmitter for Assembler<'a> {
    fn enclv(&mut self) {
        self.emit(ENCLV, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `ENCLS`.
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
    pub fn encls(&mut self)
    where Assembler<'a>: EnclsEmitter {
        <Self as EnclsEmitter>::encls(self);
    }
    /// `ENCLU`.
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
    pub fn enclu(&mut self)
    where Assembler<'a>: EncluEmitter {
        <Self as EncluEmitter>::enclu(self);
    }
    /// `ENCLV` (ENCLV). 
    /// The ENCLV instruction invokes the virtualization SGX leaf functions for managing enclaves in a virtualized environment. Software specifies the leaf function by setting the appropriate value in the register EAX as input. The registers RBX, RCX, and RDX have leaf-specific purpose, and may act as input, as output, or may be unused. In non 64-bit mode, the instruction ignores upper 32 bits of the RAX register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ENCLV.html).
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
    pub fn enclv(&mut self)
    where Assembler<'a>: EnclvEmitter {
        <Self as EnclvEmitter>::enclv(self);
    }
}
