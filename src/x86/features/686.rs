use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `FCMOVB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FcmovbEmitter<A> {
    fn fcmovb(&mut self, op0: A);
}

impl<'a> FcmovbEmitter<St> for Assembler<'a> {
    fn fcmovb(&mut self, op0: St) {
        self.emit(FCMOVBR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCMOVBE`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FcmovbeEmitter<A> {
    fn fcmovbe(&mut self, op0: A);
}

impl<'a> FcmovbeEmitter<St> for Assembler<'a> {
    fn fcmovbe(&mut self, op0: St) {
        self.emit(FCMOVBER, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCMOVE`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FcmoveEmitter<A> {
    fn fcmove(&mut self, op0: A);
}

impl<'a> FcmoveEmitter<St> for Assembler<'a> {
    fn fcmove(&mut self, op0: St) {
        self.emit(FCMOVER, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCMOVNB`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FcmovnbEmitter<A> {
    fn fcmovnb(&mut self, op0: A);
}

impl<'a> FcmovnbEmitter<St> for Assembler<'a> {
    fn fcmovnb(&mut self, op0: St) {
        self.emit(FCMOVNBR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCMOVNBE`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FcmovnbeEmitter<A> {
    fn fcmovnbe(&mut self, op0: A);
}

impl<'a> FcmovnbeEmitter<St> for Assembler<'a> {
    fn fcmovnbe(&mut self, op0: St) {
        self.emit(FCMOVNBER, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCMOVNE`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FcmovneEmitter<A> {
    fn fcmovne(&mut self, op0: A);
}

impl<'a> FcmovneEmitter<St> for Assembler<'a> {
    fn fcmovne(&mut self, op0: St) {
        self.emit(FCMOVNER, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCMOVNU`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FcmovnuEmitter<A> {
    fn fcmovnu(&mut self, op0: A);
}

impl<'a> FcmovnuEmitter<St> for Assembler<'a> {
    fn fcmovnu(&mut self, op0: St) {
        self.emit(FCMOVNUR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCMOVU`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FcmovuEmitter<A> {
    fn fcmovu(&mut self, op0: A);
}

impl<'a> FcmovuEmitter<St> for Assembler<'a> {
    fn fcmovu(&mut self, op0: St) {
        self.emit(FCMOVUR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCOMI`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FcomiEmitter<A> {
    fn fcomi(&mut self, op0: A);
}

impl<'a> FcomiEmitter<St> for Assembler<'a> {
    fn fcomi(&mut self, op0: St) {
        self.emit(FCOMIR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FCOMIP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FcomipEmitter<A, B> {
    fn fcomip(&mut self, op0: A, op1: B);
}

impl<'a> FcomipEmitter<St, St> for Assembler<'a> {
    fn fcomip(&mut self, op0: St, op1: St) {
        self.emit(FCOMIPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `FUCOMI`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St       |
/// +---+----------+
/// ```
pub trait FucomiEmitter<A> {
    fn fucomi(&mut self, op0: A);
}

impl<'a> FucomiEmitter<St> for Assembler<'a> {
    fn fucomi(&mut self, op0: St) {
        self.emit(FUCOMIR, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `FUCOMIP`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | St, St   |
/// +---+----------+
/// ```
pub trait FucomipEmitter<A, B> {
    fn fucomip(&mut self, op0: A, op1: B);
}

impl<'a> FucomipEmitter<St, St> for Assembler<'a> {
    fn fucomip(&mut self, op0: St, op1: St) {
        self.emit(
            FUCOMIPRR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `RDPMC`.
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
pub trait RdpmcEmitter {
    fn rdpmc(&mut self);
}

impl<'a> RdpmcEmitter for Assembler<'a> {
    fn rdpmc(&mut self) {
        self.emit(RDPMC, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `SYSENTER`.
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
pub trait SysenterEmitter {
    fn sysenter(&mut self);
}

impl<'a> SysenterEmitter for Assembler<'a> {
    fn sysenter(&mut self) {
        self.emit(SYSENTER, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `SYSEXIT`.
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
pub trait SysexitEmitter {
    fn sysexit(&mut self);
}

impl<'a> SysexitEmitter for Assembler<'a> {
    fn sysexit(&mut self) {
        self.emit(SYSEXIT, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `FCMOVB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcmovb<A>(&mut self, op0: A)
    where
        Assembler<'a>: FcmovbEmitter<A>,
    {
        <Self as FcmovbEmitter<A>>::fcmovb(self, op0);
    }
    /// `FCMOVBE`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcmovbe<A>(&mut self, op0: A)
    where
        Assembler<'a>: FcmovbeEmitter<A>,
    {
        <Self as FcmovbeEmitter<A>>::fcmovbe(self, op0);
    }
    /// `FCMOVE`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcmove<A>(&mut self, op0: A)
    where
        Assembler<'a>: FcmoveEmitter<A>,
    {
        <Self as FcmoveEmitter<A>>::fcmove(self, op0);
    }
    /// `FCMOVNB`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcmovnb<A>(&mut self, op0: A)
    where
        Assembler<'a>: FcmovnbEmitter<A>,
    {
        <Self as FcmovnbEmitter<A>>::fcmovnb(self, op0);
    }
    /// `FCMOVNBE`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcmovnbe<A>(&mut self, op0: A)
    where
        Assembler<'a>: FcmovnbeEmitter<A>,
    {
        <Self as FcmovnbeEmitter<A>>::fcmovnbe(self, op0);
    }
    /// `FCMOVNE`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcmovne<A>(&mut self, op0: A)
    where
        Assembler<'a>: FcmovneEmitter<A>,
    {
        <Self as FcmovneEmitter<A>>::fcmovne(self, op0);
    }
    /// `FCMOVNU`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcmovnu<A>(&mut self, op0: A)
    where
        Assembler<'a>: FcmovnuEmitter<A>,
    {
        <Self as FcmovnuEmitter<A>>::fcmovnu(self, op0);
    }
    /// `FCMOVU`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcmovu<A>(&mut self, op0: A)
    where
        Assembler<'a>: FcmovuEmitter<A>,
    {
        <Self as FcmovuEmitter<A>>::fcmovu(self, op0);
    }
    /// `FCOMI`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcomi<A>(&mut self, op0: A)
    where
        Assembler<'a>: FcomiEmitter<A>,
    {
        <Self as FcomiEmitter<A>>::fcomi(self, op0);
    }
    /// `FCOMIP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fcomip<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FcomipEmitter<A, B>,
    {
        <Self as FcomipEmitter<A, B>>::fcomip(self, op0, op1);
    }
    /// `FUCOMI`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St       |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fucomi<A>(&mut self, op0: A)
    where
        Assembler<'a>: FucomiEmitter<A>,
    {
        <Self as FucomiEmitter<A>>::fucomi(self, op0);
    }
    /// `FUCOMIP`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | St, St   |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn fucomip<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: FucomipEmitter<A, B>,
    {
        <Self as FucomipEmitter<A, B>>::fucomip(self, op0, op1);
    }
    /// `RDPMC`.
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
    pub fn rdpmc(&mut self)
    where
        Assembler<'a>: RdpmcEmitter,
    {
        <Self as RdpmcEmitter>::rdpmc(self);
    }
    /// `SYSENTER`.
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
    pub fn sysenter(&mut self)
    where
        Assembler<'a>: SysenterEmitter,
    {
        <Self as SysenterEmitter>::sysenter(self);
    }
    /// `SYSEXIT`.
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
    pub fn sysexit(&mut self)
    where
        Assembler<'a>: SysexitEmitter,
    {
        <Self as SysexitEmitter>::sysexit(self);
    }
}
