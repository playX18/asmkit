use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CMPXCHG16BM`.
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
pub trait Cmpxchg16bmEmitter<A> {
    fn cmpxchg16bm(&mut self, op0: A);
}

impl<'a> Cmpxchg16bmEmitter<Mem> for Assembler<'a> {
    fn cmpxchg16bm(&mut self, op0: Mem) {
        self.emit(CMPXCHG16BM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `CMPXCHG8BM`.
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
pub trait Cmpxchg8bmEmitter<A> {
    fn cmpxchg8bm(&mut self, op0: A);
}

impl<'a> Cmpxchg8bmEmitter<Mem> for Assembler<'a> {
    fn cmpxchg8bm(&mut self, op0: Mem) {
        self.emit(CMPXCHG8BM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `CMPXCHGD`.
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
pub trait CmpxchgdEmitter<A> {
    fn cmpxchgd(&mut self, op0: A);
}

impl<'a> CmpxchgdEmitter<Mem> for Assembler<'a> {
    fn cmpxchgd(&mut self, op0: Mem) {
        self.emit(CMPXCHGD32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `CPUID` (CPUID). 
/// The ID flag (bit 21) in the EFLAGS register indicates support for the CPUID instruction. If a software procedure can set and clear this flag, the processor executing the procedure supports the CPUID instruction. This instruction operates the same in non-64-bit modes and 64-bit mode.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CPUID.html).
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
pub trait CpuidEmitter {
    fn cpuid(&mut self);
}

impl<'a> CpuidEmitter for Assembler<'a> {
    fn cpuid(&mut self) {
        self.emit(CPUID, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `RDMSR`.
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
pub trait RdmsrEmitter {
    fn rdmsr(&mut self);
}

impl<'a> RdmsrEmitter for Assembler<'a> {
    fn rdmsr(&mut self) {
        self.emit(RDMSR, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `RDTSC` (RDTSC). 
/// Reads the current value of the processor’s time-stamp counter (a 64-bit MSR) into the EDX:EAX registers. The EDX register is loaded with the high-order 32 bits of the MSR and the EAX register is loaded with the low-order 32 bits. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RDTSC.html).
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
pub trait RdtscEmitter {
    fn rdtsc(&mut self);
}

impl<'a> RdtscEmitter for Assembler<'a> {
    fn rdtsc(&mut self) {
        self.emit(RDTSC, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `RSM` (RSM). 
/// Returns program control from system management mode (SMM) to the application program or operating-system procedure that was interrupted when the processor received an SMM interrupt. The processor’s state is restored from the dump created upon entering SMM. If the processor detects invalid state information during state restoration, it enters the shutdown state. The following invalid information can cause a shutdown
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RSM.html).
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
pub trait RsmEmitter {
    fn rsm(&mut self);
}

impl<'a> RsmEmitter for Assembler<'a> {
    fn rsm(&mut self) {
        self.emit(RSM, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `WRMSR` (WRMSR). 
/// Writes the contents of registers EDX:EAX into the 64-bit model specific register (MSR) specified in the ECX register. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The contents of the EDX register are copied to high-order 32 bits of the selected MSR and the contents of the EAX register are copied to low-order 32 bits of the MSR. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are ignored.) Undefined or reserved bits in an MSR should be set to values previously read.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/WRMSR.html).
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
pub trait WrmsrEmitter {
    fn wrmsr(&mut self);
}

impl<'a> WrmsrEmitter for Assembler<'a> {
    fn wrmsr(&mut self) {
        self.emit(WRMSR, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `CMPXCHG16BM`.
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
    pub fn cmpxchg16bm<A>(&mut self, op0: A)
    where Assembler<'a>: Cmpxchg16bmEmitter<A> {
        <Self as Cmpxchg16bmEmitter<A>>::cmpxchg16bm(self, op0);
    }
    /// `CMPXCHG8BM`.
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
    pub fn cmpxchg8bm<A>(&mut self, op0: A)
    where Assembler<'a>: Cmpxchg8bmEmitter<A> {
        <Self as Cmpxchg8bmEmitter<A>>::cmpxchg8bm(self, op0);
    }
    /// `CMPXCHGD`.
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
    pub fn cmpxchgd<A>(&mut self, op0: A)
    where Assembler<'a>: CmpxchgdEmitter<A> {
        <Self as CmpxchgdEmitter<A>>::cmpxchgd(self, op0);
    }
    /// `CPUID` (CPUID). 
    /// The ID flag (bit 21) in the EFLAGS register indicates support for the CPUID instruction. If a software procedure can set and clear this flag, the processor executing the procedure supports the CPUID instruction. This instruction operates the same in non-64-bit modes and 64-bit mode.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CPUID.html).
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
    pub fn cpuid(&mut self)
    where Assembler<'a>: CpuidEmitter {
        <Self as CpuidEmitter>::cpuid(self);
    }
    /// `RDMSR`.
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
    pub fn rdmsr(&mut self)
    where Assembler<'a>: RdmsrEmitter {
        <Self as RdmsrEmitter>::rdmsr(self);
    }
    /// `RDTSC` (RDTSC). 
    /// Reads the current value of the processor’s time-stamp counter (a 64-bit MSR) into the EDX:EAX registers. The EDX register is loaded with the high-order 32 bits of the MSR and the EAX register is loaded with the low-order 32 bits. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RDTSC.html).
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
    pub fn rdtsc(&mut self)
    where Assembler<'a>: RdtscEmitter {
        <Self as RdtscEmitter>::rdtsc(self);
    }
    /// `RSM` (RSM). 
    /// Returns program control from system management mode (SMM) to the application program or operating-system procedure that was interrupted when the processor received an SMM interrupt. The processor’s state is restored from the dump created upon entering SMM. If the processor detects invalid state information during state restoration, it enters the shutdown state. The following invalid information can cause a shutdown
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RSM.html).
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
    pub fn rsm(&mut self)
    where Assembler<'a>: RsmEmitter {
        <Self as RsmEmitter>::rsm(self);
    }
    /// `WRMSR` (WRMSR). 
    /// Writes the contents of registers EDX:EAX into the 64-bit model specific register (MSR) specified in the ECX register. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The contents of the EDX register are copied to high-order 32 bits of the selected MSR and the contents of the EAX register are copied to low-order 32 bits of the MSR. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are ignored.) Undefined or reserved bits in an MSR should be set to values previously read.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/WRMSR.html).
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
    pub fn wrmsr(&mut self)
    where Assembler<'a>: WrmsrEmitter {
        <Self as WrmsrEmitter>::wrmsr(self);
    }
}
