use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `INVEPT`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait InveptEmitter<A, B> {
    fn invept(&mut self, op0: A, op1: B);
}

impl<'a> InveptEmitter<Gpq, Mem> for Assembler<'a> {
    fn invept(&mut self, op0: Gpq, op1: Mem) {
        self.emit(INVEPTRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `INVVPID`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait InvvpidEmitter<A, B> {
    fn invvpid(&mut self, op0: A, op1: B);
}

impl<'a> InvvpidEmitter<Gpq, Mem> for Assembler<'a> {
    fn invvpid(&mut self, op0: Gpq, op1: Mem) {
        self.emit(INVVPIDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMCALL`.
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
pub trait VmcallEmitter {
    fn vmcall(&mut self);
}

impl<'a> VmcallEmitter for Assembler<'a> {
    fn vmcall(&mut self) {
        self.emit(VMCALL, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `VMCLEAR`.
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
pub trait VmclearEmitter<A> {
    fn vmclear(&mut self, op0: A);
}

impl<'a> VmclearEmitter<Mem> for Assembler<'a> {
    fn vmclear(&mut self, op0: Mem) {
        self.emit(VMCLEARM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `VMFUNC`.
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
pub trait VmfuncEmitter {
    fn vmfunc(&mut self);
}

impl<'a> VmfuncEmitter for Assembler<'a> {
    fn vmfunc(&mut self) {
        self.emit(VMFUNC, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `VMLAUNCH`.
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
pub trait VmlaunchEmitter {
    fn vmlaunch(&mut self);
}

impl<'a> VmlaunchEmitter for Assembler<'a> {
    fn vmlaunch(&mut self) {
        self.emit(VMLAUNCH, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `VMPTRLD`.
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
pub trait VmptrldEmitter<A> {
    fn vmptrld(&mut self, op0: A);
}

impl<'a> VmptrldEmitter<Mem> for Assembler<'a> {
    fn vmptrld(&mut self, op0: Mem) {
        self.emit(VMPTRLDM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `VMPTRST`.
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
pub trait VmptrstEmitter<A> {
    fn vmptrst(&mut self, op0: A);
}

impl<'a> VmptrstEmitter<Mem> for Assembler<'a> {
    fn vmptrst(&mut self, op0: Mem) {
        self.emit(VMPTRSTM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `VMREAD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpq, Gpq |
/// | 2 | Mem, Gpq |
/// +---+----------+
/// ```
pub trait VmreadEmitter<A, B> {
    fn vmread(&mut self, op0: A, op1: B);
}

impl<'a> VmreadEmitter<Gpq, Gpq> for Assembler<'a> {
    fn vmread(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(VMREADRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmreadEmitter<Mem, Gpq> for Assembler<'a> {
    fn vmread(&mut self, op0: Mem, op1: Gpq) {
        self.emit(VMREADMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMRESUME`.
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
pub trait VmresumeEmitter {
    fn vmresume(&mut self);
}

impl<'a> VmresumeEmitter for Assembler<'a> {
    fn vmresume(&mut self) {
        self.emit(VMRESUME, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `VMWRITE`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpq, Gpq |
/// | 2 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait VmwriteEmitter<A, B> {
    fn vmwrite(&mut self, op0: A, op1: B);
}

impl<'a> VmwriteEmitter<Gpq, Gpq> for Assembler<'a> {
    fn vmwrite(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(VMWRITERR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmwriteEmitter<Gpq, Mem> for Assembler<'a> {
    fn vmwrite(&mut self, op0: Gpq, op1: Mem) {
        self.emit(VMWRITERM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMXOFF`.
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
pub trait VmxoffEmitter {
    fn vmxoff(&mut self);
}

impl<'a> VmxoffEmitter for Assembler<'a> {
    fn vmxoff(&mut self) {
        self.emit(VMXOFF, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `VMXON`.
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
pub trait VmxonEmitter<A> {
    fn vmxon(&mut self, op0: A);
}

impl<'a> VmxonEmitter<Mem> for Assembler<'a> {
    fn vmxon(&mut self, op0: Mem) {
        self.emit(VMXONM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `INVEPT`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn invept<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: InveptEmitter<A, B> {
        <Self as InveptEmitter<A, B>>::invept(self, op0, op1);
    }
    /// `INVVPID`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn invvpid<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: InvvpidEmitter<A, B> {
        <Self as InvvpidEmitter<A, B>>::invvpid(self, op0, op1);
    }
    /// `VMCALL`.
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
    pub fn vmcall(&mut self)
    where Assembler<'a>: VmcallEmitter {
        <Self as VmcallEmitter>::vmcall(self);
    }
    /// `VMCLEAR`.
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
    pub fn vmclear<A>(&mut self, op0: A)
    where Assembler<'a>: VmclearEmitter<A> {
        <Self as VmclearEmitter<A>>::vmclear(self, op0);
    }
    /// `VMFUNC`.
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
    pub fn vmfunc(&mut self)
    where Assembler<'a>: VmfuncEmitter {
        <Self as VmfuncEmitter>::vmfunc(self);
    }
    /// `VMLAUNCH`.
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
    pub fn vmlaunch(&mut self)
    where Assembler<'a>: VmlaunchEmitter {
        <Self as VmlaunchEmitter>::vmlaunch(self);
    }
    /// `VMPTRLD`.
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
    pub fn vmptrld<A>(&mut self, op0: A)
    where Assembler<'a>: VmptrldEmitter<A> {
        <Self as VmptrldEmitter<A>>::vmptrld(self, op0);
    }
    /// `VMPTRST`.
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
    pub fn vmptrst<A>(&mut self, op0: A)
    where Assembler<'a>: VmptrstEmitter<A> {
        <Self as VmptrstEmitter<A>>::vmptrst(self, op0);
    }
    /// `VMREAD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpq, Gpq |
    /// | 2 | Mem, Gpq |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmread<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmreadEmitter<A, B> {
        <Self as VmreadEmitter<A, B>>::vmread(self, op0, op1);
    }
    /// `VMRESUME`.
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
    pub fn vmresume(&mut self)
    where Assembler<'a>: VmresumeEmitter {
        <Self as VmresumeEmitter>::vmresume(self);
    }
    /// `VMWRITE`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpq, Gpq |
    /// | 2 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmwrite<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmwriteEmitter<A, B> {
        <Self as VmwriteEmitter<A, B>>::vmwrite(self, op0, op1);
    }
    /// `VMXOFF`.
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
    pub fn vmxoff(&mut self)
    where Assembler<'a>: VmxoffEmitter {
        <Self as VmxoffEmitter>::vmxoff(self);
    }
    /// `VMXON`.
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
    pub fn vmxon<A>(&mut self, op0: A)
    where Assembler<'a>: VmxonEmitter<A> {
        <Self as VmxonEmitter<A>>::vmxon(self, op0);
    }
}
