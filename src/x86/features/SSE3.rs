use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `FISTTP` (FISTTP). 
/// FISTTP converts the value in ST into a signed integer using truncation (chop) as rounding mode, transfers the result to the destination, and pop ST. FISTTP accepts word, short integer, and long integer destinations.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FISTTP.html).
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
pub trait FisttpEmitter<A> {
    fn fisttp(&mut self, op0: A);
}

impl<'a> FisttpEmitter<Mem> for Assembler<'a> {
    fn fisttp(&mut self, op0: Mem) {
        self.emit(FISTTPM32, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `SSE_ADDSUBPD` (ADDSUBPD). 
/// Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the corresponding double precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered double precision floating-point values from the second source operand from the corresponding double precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSUBPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseAddsubpdEmitter<A, B> {
    fn sse_addsubpd(&mut self, op0: A, op1: B);
}

impl<'a> SseAddsubpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_addsubpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ADDSUBPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseAddsubpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_addsubpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ADDSUBPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_ADDSUBPS` (ADDSUBPS). 
/// Adds odd-numbered single precision floating-point values of the first source operand (second operand) with the corresponding single precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered single precision floating-point values from the second source operand from the corresponding single precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSUBPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseAddsubpsEmitter<A, B> {
    fn sse_addsubps(&mut self, op0: A, op1: B);
}

impl<'a> SseAddsubpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_addsubps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_ADDSUBPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseAddsubpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_addsubps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_ADDSUBPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_HADDPD` (HADDPD). 
/// Adds the double precision floating-point values in the high and low quadwords of the destination operand and stores the result in the low quadword of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HADDPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseHaddpdEmitter<A, B> {
    fn sse_haddpd(&mut self, op0: A, op1: B);
}

impl<'a> SseHaddpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_haddpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_HADDPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseHaddpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_haddpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_HADDPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_HADDPS` (HADDPS). 
/// Adds the single precision floating-point values in the first and second dwords of the destination operand and stores the result in the first dword of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HADDPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseHaddpsEmitter<A, B> {
    fn sse_haddps(&mut self, op0: A, op1: B);
}

impl<'a> SseHaddpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_haddps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_HADDPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseHaddpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_haddps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_HADDPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_HSUBPD` (HSUBPD). 
/// The HSUBPD instruction subtracts horizontally the packed double precision floating-point numbers of both operands.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HSUBPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseHsubpdEmitter<A, B> {
    fn sse_hsubpd(&mut self, op0: A, op1: B);
}

impl<'a> SseHsubpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_hsubpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_HSUBPDRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseHsubpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_hsubpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_HSUBPDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_HSUBPS` (HSUBPS). 
/// Subtracts the single precision floating-point value in the second dword of the destination operand from the first dword of the destination operand and stores the result in the first dword of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HSUBPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseHsubpsEmitter<A, B> {
    fn sse_hsubps(&mut self, op0: A, op1: B);
}

impl<'a> SseHsubpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_hsubps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_HSUBPSRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseHsubpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_hsubps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_HSUBPSRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_LDDQU` (LDDQU). 
/// The instruction is functionally similar to (V)MOVDQU ymm/xmm, m256/m128 for loading from memory. That is: 32/16 bytes of data starting at an address specified by the source memory operand (second operand) are fetched from memory and placed in a destination register (first operand). The source operand need not be aligned on a 32/16-byte boundary. Up to 64/32 bytes may be loaded from memory; this is implementation dependent.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/LDDQU.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// +---+----------+
/// ```
pub trait SseLddquEmitter<A, B> {
    fn sse_lddqu(&mut self, op0: A, op1: B);
}

impl<'a> SseLddquEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_lddqu(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_LDDQURM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVDDUP` (MOVDDUP). 
/// For 256-bit or higher versions: Duplicates even-indexed double precision floating-point values from the source operand (the second operand) and into adjacent pair and store to the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDDUP.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovddupEmitter<A, B> {
    fn sse_movddup(&mut self, op0: A, op1: B);
}

impl<'a> SseMovddupEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movddup(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVDDUPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovddupEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movddup(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVDDUPRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVSHDUP` (MOVSHDUP). 
/// Duplicates odd-indexed single precision floating-point values from the source operand (the second operand) to adjacent element pair in the destination operand (the first operand). See Figure 4-3. The source operand is an XMM, YMM or ZMM register or 128, 256 or 512-bit memory location and the destination operand is an XMM, YMM or ZMM register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVSHDUP.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovshdupEmitter<A, B> {
    fn sse_movshdup(&mut self, op0: A, op1: B);
}

impl<'a> SseMovshdupEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movshdup(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVSHDUPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovshdupEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movshdup(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVSHDUPRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `SSE_MOVSLDUP` (MOVSLDUP). 
/// Duplicates even-indexed single precision floating-point values from the source operand (the second operand). See Figure 4-4. The source operand is an XMM, YMM or ZMM register or 128, 256 or 512-bit memory location and the destination operand is an XMM, YMM or ZMM register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVSLDUP.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait SseMovsldupEmitter<A, B> {
    fn sse_movsldup(&mut self, op0: A, op1: B);
}

impl<'a> SseMovsldupEmitter<Xmm, Xmm> for Assembler<'a> {
    fn sse_movsldup(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(SSE_MOVSLDUPRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> SseMovsldupEmitter<Xmm, Mem> for Assembler<'a> {
    fn sse_movsldup(&mut self, op0: Xmm, op1: Mem) {
        self.emit(SSE_MOVSLDUPRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `FISTTP` (FISTTP). 
    /// FISTTP converts the value in ST into a signed integer using truncation (chop) as rounding mode, transfers the result to the destination, and pop ST. FISTTP accepts word, short integer, and long integer destinations.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/FISTTP.html).
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
    pub fn fisttp<A>(&mut self, op0: A)
    where Assembler<'a>: FisttpEmitter<A> {
        <Self as FisttpEmitter<A>>::fisttp(self, op0);
    }
    /// `SSE_ADDSUBPD` (ADDSUBPD). 
    /// Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the corresponding double precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered double precision floating-point values from the second source operand from the corresponding double precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSUBPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_addsubpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseAddsubpdEmitter<A, B> {
        <Self as SseAddsubpdEmitter<A, B>>::sse_addsubpd(self, op0, op1);
    }
    /// `SSE_ADDSUBPS` (ADDSUBPS). 
    /// Adds odd-numbered single precision floating-point values of the first source operand (second operand) with the corresponding single precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered single precision floating-point values from the second source operand from the corresponding single precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSUBPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_addsubps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseAddsubpsEmitter<A, B> {
        <Self as SseAddsubpsEmitter<A, B>>::sse_addsubps(self, op0, op1);
    }
    /// `SSE_HADDPD` (HADDPD). 
    /// Adds the double precision floating-point values in the high and low quadwords of the destination operand and stores the result in the low quadword of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HADDPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_haddpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseHaddpdEmitter<A, B> {
        <Self as SseHaddpdEmitter<A, B>>::sse_haddpd(self, op0, op1);
    }
    /// `SSE_HADDPS` (HADDPS). 
    /// Adds the single precision floating-point values in the first and second dwords of the destination operand and stores the result in the first dword of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HADDPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_haddps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseHaddpsEmitter<A, B> {
        <Self as SseHaddpsEmitter<A, B>>::sse_haddps(self, op0, op1);
    }
    /// `SSE_HSUBPD` (HSUBPD). 
    /// The HSUBPD instruction subtracts horizontally the packed double precision floating-point numbers of both operands.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HSUBPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_hsubpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseHsubpdEmitter<A, B> {
        <Self as SseHsubpdEmitter<A, B>>::sse_hsubpd(self, op0, op1);
    }
    /// `SSE_HSUBPS` (HSUBPS). 
    /// Subtracts the single precision floating-point value in the second dword of the destination operand from the first dword of the destination operand and stores the result in the first dword of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HSUBPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_hsubps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseHsubpsEmitter<A, B> {
        <Self as SseHsubpsEmitter<A, B>>::sse_hsubps(self, op0, op1);
    }
    /// `SSE_LDDQU` (LDDQU). 
    /// The instruction is functionally similar to (V)MOVDQU ymm/xmm, m256/m128 for loading from memory. That is: 32/16 bytes of data starting at an address specified by the source memory operand (second operand) are fetched from memory and placed in a destination register (first operand). The source operand need not be aligned on a 32/16-byte boundary. Up to 64/32 bytes may be loaded from memory; this is implementation dependent.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/LDDQU.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_lddqu<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseLddquEmitter<A, B> {
        <Self as SseLddquEmitter<A, B>>::sse_lddqu(self, op0, op1);
    }
    /// `SSE_MOVDDUP` (MOVDDUP). 
    /// For 256-bit or higher versions: Duplicates even-indexed double precision floating-point values from the source operand (the second operand) and into adjacent pair and store to the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDDUP.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movddup<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovddupEmitter<A, B> {
        <Self as SseMovddupEmitter<A, B>>::sse_movddup(self, op0, op1);
    }
    /// `SSE_MOVSHDUP` (MOVSHDUP). 
    /// Duplicates odd-indexed single precision floating-point values from the source operand (the second operand) to adjacent element pair in the destination operand (the first operand). See Figure 4-3. The source operand is an XMM, YMM or ZMM register or 128, 256 or 512-bit memory location and the destination operand is an XMM, YMM or ZMM register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVSHDUP.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movshdup<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovshdupEmitter<A, B> {
        <Self as SseMovshdupEmitter<A, B>>::sse_movshdup(self, op0, op1);
    }
    /// `SSE_MOVSLDUP` (MOVSLDUP). 
    /// Duplicates even-indexed single precision floating-point values from the source operand (the second operand). See Figure 4-4. The source operand is an XMM, YMM or ZMM register or 128, 256 or 512-bit memory location and the destination operand is an XMM, YMM or ZMM register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVSLDUP.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn sse_movsldup<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: SseMovsldupEmitter<A, B> {
        <Self as SseMovsldupEmitter<A, B>>::sse_movsldup(self, op0, op1);
    }
}
