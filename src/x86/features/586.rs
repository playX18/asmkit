use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

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

/// `CPUID`.
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

/// `RDTSC`.
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

/// `RSM`.
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

/// `WRMSR`.
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
    where
        Assembler<'a>: Cmpxchg16bmEmitter<A>,
    {
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
    where
        Assembler<'a>: Cmpxchg8bmEmitter<A>,
    {
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
    where
        Assembler<'a>: CmpxchgdEmitter<A>,
    {
        <Self as CmpxchgdEmitter<A>>::cmpxchgd(self, op0);
    }
    /// `CPUID`.
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
    where
        Assembler<'a>: CpuidEmitter,
    {
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
    where
        Assembler<'a>: RdmsrEmitter,
    {
        <Self as RdmsrEmitter>::rdmsr(self);
    }
    /// `RDTSC`.
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
    where
        Assembler<'a>: RdtscEmitter,
    {
        <Self as RdtscEmitter>::rdtsc(self);
    }
    /// `RSM`.
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
    where
        Assembler<'a>: RsmEmitter,
    {
        <Self as RsmEmitter>::rsm(self);
    }
    /// `WRMSR`.
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
    where
        Assembler<'a>: WrmsrEmitter,
    {
        <Self as WrmsrEmitter>::wrmsr(self);
    }
}
