use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `FCMOVB` (FCMOVB). 
/// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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

/// `FCMOVBE` (FCMOVBE). 
/// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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

/// `FCMOVE` (FCMOVE). 
/// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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

/// `FCMOVNB` (FCMOVNB). 
/// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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

/// `FCMOVNBE` (FCMOVNBE). 
/// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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

/// `FCMOVNE` (FCMOVNE). 
/// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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

/// `FCMOVNU` (FCMOVNU). 
/// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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

/// `FCMOVU` (FCMOVU). 
/// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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

/// `FCOMI` (FCOMI). 
/// Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html).
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

/// `FCOMIP` (FCOMIP). 
/// Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html).
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

/// `FUCOMI` (FUCOMI). 
/// Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html).
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

/// `FUCOMIP` (FUCOMIP). 
/// Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html).
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
        self.emit(FUCOMIPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `RDPMC` (RDPMC). 
/// Reads the contents of the performance monitoring counter (PMC) specified in ECX register into registers EDX:EAX. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The EDX register is loaded with the high-order 32 bits of the PMC and the EAX register is loaded with the low-order 32 bits. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.) If fewer than 64 bits are implemented in the PMC being read, unimplemented bits returned to EDX:EAX will have value zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RDPMC.html).
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

/// `SYSENTER` (SYSENTER). 
/// Executes a fast call to a level 0 system procedure or routine. SYSENTER is a companion instruction to SYSEXIT. The instruction is optimized to provide the maximum performance for system calls from user code running at privilege level 3 to operating system or executive procedures running at privilege level 0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SYSENTER.html).
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

/// `SYSEXIT` (SYSEXIT). 
/// Executes a fast return to privilege level 3 user code. SYSEXIT is a companion instruction to the SYSENTER instruction. The instruction is optimized to provide the maximum performance for returns from system procedures executing at protections levels 0 to user procedures executing at protection level 3. It must be executed from code executing at privilege level 0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SYSEXIT.html).
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
    /// `FCMOVB` (FCMOVB). 
    /// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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
    where Assembler<'a>: FcmovbEmitter<A> {
        <Self as FcmovbEmitter<A>>::fcmovb(self, op0);
    }
    /// `FCMOVBE` (FCMOVBE). 
    /// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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
    where Assembler<'a>: FcmovbeEmitter<A> {
        <Self as FcmovbeEmitter<A>>::fcmovbe(self, op0);
    }
    /// `FCMOVE` (FCMOVE). 
    /// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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
    where Assembler<'a>: FcmoveEmitter<A> {
        <Self as FcmoveEmitter<A>>::fcmove(self, op0);
    }
    /// `FCMOVNB` (FCMOVNB). 
    /// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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
    where Assembler<'a>: FcmovnbEmitter<A> {
        <Self as FcmovnbEmitter<A>>::fcmovnb(self, op0);
    }
    /// `FCMOVNBE` (FCMOVNBE). 
    /// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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
    where Assembler<'a>: FcmovnbeEmitter<A> {
        <Self as FcmovnbeEmitter<A>>::fcmovnbe(self, op0);
    }
    /// `FCMOVNE` (FCMOVNE). 
    /// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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
    where Assembler<'a>: FcmovneEmitter<A> {
        <Self as FcmovneEmitter<A>>::fcmovne(self, op0);
    }
    /// `FCMOVNU` (FCMOVNU). 
    /// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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
    where Assembler<'a>: FcmovnuEmitter<A> {
        <Self as FcmovnuEmitter<A>>::fcmovnu(self, op0);
    }
    /// `FCMOVU` (FCMOVU). 
    /// Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true. The condition for each mnemonic os given in the Description column above and in Chapter 8 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1. The source operand is always in the ST(i) register and the destination operand is always ST(0).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCMOVcc.html).
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
    where Assembler<'a>: FcmovuEmitter<A> {
        <Self as FcmovuEmitter<A>>::fcmovu(self, op0);
    }
    /// `FCOMI` (FCOMI). 
    /// Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html).
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
    where Assembler<'a>: FcomiEmitter<A> {
        <Self as FcomiEmitter<A>>::fcomi(self, op0);
    }
    /// `FCOMIP` (FCOMIP). 
    /// Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html).
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
    where Assembler<'a>: FcomipEmitter<A, B> {
        <Self as FcomipEmitter<A, B>>::fcomip(self, op0, op1);
    }
    /// `FUCOMI` (FUCOMI). 
    /// Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html).
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
    where Assembler<'a>: FucomiEmitter<A> {
        <Self as FucomiEmitter<A>>::fucomi(self, op0);
    }
    /// `FUCOMIP` (FUCOMIP). 
    /// Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below). The sign of zero is ignored for comparisons, so that –0.0 is equal to +0.0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FCOMI%3AFCOMIP%3AFUCOMI%3AFUCOMIP.html).
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
    where Assembler<'a>: FucomipEmitter<A, B> {
        <Self as FucomipEmitter<A, B>>::fucomip(self, op0, op1);
    }
    /// `RDPMC` (RDPMC). 
    /// Reads the contents of the performance monitoring counter (PMC) specified in ECX register into registers EDX:EAX. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The EDX register is loaded with the high-order 32 bits of the PMC and the EAX register is loaded with the low-order 32 bits. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.) If fewer than 64 bits are implemented in the PMC being read, unimplemented bits returned to EDX:EAX will have value zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RDPMC.html).
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
    where Assembler<'a>: RdpmcEmitter {
        <Self as RdpmcEmitter>::rdpmc(self);
    }
    /// `SYSENTER` (SYSENTER). 
    /// Executes a fast call to a level 0 system procedure or routine. SYSENTER is a companion instruction to SYSEXIT. The instruction is optimized to provide the maximum performance for system calls from user code running at privilege level 3 to operating system or executive procedures running at privilege level 0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SYSENTER.html).
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
    where Assembler<'a>: SysenterEmitter {
        <Self as SysenterEmitter>::sysenter(self);
    }
    /// `SYSEXIT` (SYSEXIT). 
    /// Executes a fast return to privilege level 3 user code. SYSEXIT is a companion instruction to the SYSENTER instruction. The instruction is optimized to provide the maximum performance for returns from system procedures executing at protections levels 0 to user procedures executing at protection level 3. It must be executed from code executing at privilege level 0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/SYSEXIT.html).
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
    where Assembler<'a>: SysexitEmitter {
        <Self as SysexitEmitter>::sysexit(self);
    }
}
