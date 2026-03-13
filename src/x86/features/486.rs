use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `BSWAP` (BSWAP). 
/// Reverses the byte order of a 32-bit or 64-bit (destination) register. This instruction is provided for converting little-endian values to big-endian format and vice versa. To swap bytes in a word value (16-bit register), use the XCHG instruction. When the BSWAP instruction references a 16-bit register, the result is undefined.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BSWAP.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// | 3 | Gpw      |
/// +---+----------+
/// ```
pub trait BswapEmitter<A> {
    fn bswap(&mut self, op0: A);
}

impl<'a> BswapEmitter<Gpw> for Assembler<'a> {
    fn bswap(&mut self, op0: Gpw) {
        self.emit(BSWAP16R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> BswapEmitter<Gpd> for Assembler<'a> {
    fn bswap(&mut self, op0: Gpd) {
        self.emit(BSWAP32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> BswapEmitter<Gpq> for Assembler<'a> {
    fn bswap(&mut self, op0: Gpq) {
        self.emit(BSWAP64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `CMPXCHG` (CMPXCHG). 
/// Compares the value in the AL, AX, EAX, or RAX register with the first operand (destination operand). If the two values are equal, the second operand (source operand) is loaded into the destination operand. Otherwise, the destination operand is loaded into the AL, AX, EAX or RAX register. RAX register is available only in 64-bit mode.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPXCHG.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------+
/// | # | Operands     |
/// +---+--------------+
/// | 1 | GpbLo, GpbLo |
/// | 2 | Gpd, Gpd     |
/// | 3 | Gpq, Gpq     |
/// | 4 | Gpw, Gpw     |
/// | 5 | Mem, GpbLo   |
/// | 6 | Mem, Gpd     |
/// | 7 | Mem, Gpq     |
/// | 8 | Mem, Gpw     |
/// +---+--------------+
/// ```
pub trait CmpxchgEmitter<A, B> {
    fn cmpxchg(&mut self, op0: A, op1: B);
}

impl<'a> CmpxchgEmitter<GpbLo, GpbLo> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: GpbLo, op1: GpbLo) {
        self.emit(CMPXCHG8RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> CmpxchgEmitter<Mem, GpbLo> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Mem, op1: GpbLo) {
        self.emit(CMPXCHG8MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> CmpxchgEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(CMPXCHG16RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> CmpxchgEmitter<Mem, Gpw> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Mem, op1: Gpw) {
        self.emit(CMPXCHG16MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> CmpxchgEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(CMPXCHG32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> CmpxchgEmitter<Mem, Gpd> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Mem, op1: Gpd) {
        self.emit(CMPXCHG32MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> CmpxchgEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(CMPXCHG64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> CmpxchgEmitter<Mem, Gpq> for Assembler<'a> {
    fn cmpxchg(&mut self, op0: Mem, op1: Gpq) {
        self.emit(CMPXCHG64MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `INVD` (INVD). 
/// Invalidates (flushes) the processor’s internal caches and issues a special-function bus cycle that directs external caches to also flush themselves. Data held in internal caches is not written back to main memory.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/INVD.html).
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
pub trait InvdEmitter {
    fn invd(&mut self);
}

impl<'a> InvdEmitter for Assembler<'a> {
    fn invd(&mut self) {
        self.emit(INVD, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `INVLPG`.
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
pub trait InvlpgEmitter<A> {
    fn invlpg(&mut self, op0: A);
}

impl<'a> InvlpgEmitter<Mem> for Assembler<'a> {
    fn invlpg(&mut self, op0: Mem) {
        self.emit(INVLPG8M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `WBINVD` (WBINVD). 
/// Writes back all modified cache lines in the processor’s internal cache to main memory and invalidates (flushes) the internal caches. The instruction then issues a special-function bus cycle that directs external caches to also write back modified data and another bus cycle to indicate that the external caches should be invalidated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/WBINVD.html).
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
pub trait WbinvdEmitter {
    fn wbinvd(&mut self);
}

impl<'a> WbinvdEmitter for Assembler<'a> {
    fn wbinvd(&mut self) {
        self.emit(WBINVD, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `XADD` (XADD). 
/// Exchanges the first operand (destination operand) with the second operand (source operand), then loads the sum of the two values into the destination operand. The destination operand can be a register or a memory location; the source operand is a register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XADD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------+
/// | # | Operands     |
/// +---+--------------+
/// | 1 | GpbLo, GpbLo |
/// | 2 | Gpd, Gpd     |
/// | 3 | Gpq, Gpq     |
/// | 4 | Gpw, Gpw     |
/// | 5 | Mem, GpbLo   |
/// | 6 | Mem, Gpd     |
/// | 7 | Mem, Gpq     |
/// | 8 | Mem, Gpw     |
/// +---+--------------+
/// ```
pub trait XaddEmitter<A, B> {
    fn xadd(&mut self, op0: A, op1: B);
}

impl<'a> XaddEmitter<GpbLo, GpbLo> for Assembler<'a> {
    fn xadd(&mut self, op0: GpbLo, op1: GpbLo) {
        self.emit(XADD8RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Mem, GpbLo> for Assembler<'a> {
    fn xadd(&mut self, op0: Mem, op1: GpbLo) {
        self.emit(XADD8MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Gpw, Gpw> for Assembler<'a> {
    fn xadd(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(XADD16RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Mem, Gpw> for Assembler<'a> {
    fn xadd(&mut self, op0: Mem, op1: Gpw) {
        self.emit(XADD16MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Gpd, Gpd> for Assembler<'a> {
    fn xadd(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(XADD32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Mem, Gpd> for Assembler<'a> {
    fn xadd(&mut self, op0: Mem, op1: Gpd) {
        self.emit(XADD32MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Gpq, Gpq> for Assembler<'a> {
    fn xadd(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(XADD64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> XaddEmitter<Mem, Gpq> for Assembler<'a> {
    fn xadd(&mut self, op0: Mem, op1: Gpq) {
        self.emit(XADD64MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `BSWAP` (BSWAP). 
    /// Reverses the byte order of a 32-bit or 64-bit (destination) register. This instruction is provided for converting little-endian values to big-endian format and vice versa. To swap bytes in a word value (16-bit register), use the XCHG instruction. When the BSWAP instruction references a 16-bit register, the result is undefined.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BSWAP.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// | 3 | Gpw      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn bswap<A>(&mut self, op0: A)
    where Assembler<'a>: BswapEmitter<A> {
        <Self as BswapEmitter<A>>::bswap(self, op0);
    }
    /// `CMPXCHG` (CMPXCHG). 
    /// Compares the value in the AL, AX, EAX, or RAX register with the first operand (destination operand). If the two values are equal, the second operand (source operand) is loaded into the destination operand. Otherwise, the destination operand is loaded into the AL, AX, EAX or RAX register. RAX register is available only in 64-bit mode.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPXCHG.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------+
    /// | # | Operands     |
    /// +---+--------------+
    /// | 1 | GpbLo, GpbLo |
    /// | 2 | Gpd, Gpd     |
    /// | 3 | Gpq, Gpq     |
    /// | 4 | Gpw, Gpw     |
    /// | 5 | Mem, GpbLo   |
    /// | 6 | Mem, Gpd     |
    /// | 7 | Mem, Gpq     |
    /// | 8 | Mem, Gpw     |
    /// +---+--------------+
    /// ```
    #[inline]
    pub fn cmpxchg<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: CmpxchgEmitter<A, B> {
        <Self as CmpxchgEmitter<A, B>>::cmpxchg(self, op0, op1);
    }
    /// `INVD` (INVD). 
    /// Invalidates (flushes) the processor’s internal caches and issues a special-function bus cycle that directs external caches to also flush themselves. Data held in internal caches is not written back to main memory.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/INVD.html).
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
    pub fn invd(&mut self)
    where Assembler<'a>: InvdEmitter {
        <Self as InvdEmitter>::invd(self);
    }
    /// `INVLPG`.
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
    pub fn invlpg<A>(&mut self, op0: A)
    where Assembler<'a>: InvlpgEmitter<A> {
        <Self as InvlpgEmitter<A>>::invlpg(self, op0);
    }
    /// `WBINVD` (WBINVD). 
    /// Writes back all modified cache lines in the processor’s internal cache to main memory and invalidates (flushes) the internal caches. The instruction then issues a special-function bus cycle that directs external caches to also write back modified data and another bus cycle to indicate that the external caches should be invalidated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/WBINVD.html).
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
    pub fn wbinvd(&mut self)
    where Assembler<'a>: WbinvdEmitter {
        <Self as WbinvdEmitter>::wbinvd(self);
    }
    /// `XADD` (XADD). 
    /// Exchanges the first operand (destination operand) with the second operand (source operand), then loads the sum of the two values into the destination operand. The destination operand can be a register or a memory location; the source operand is a register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XADD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------+
    /// | # | Operands     |
    /// +---+--------------+
    /// | 1 | GpbLo, GpbLo |
    /// | 2 | Gpd, Gpd     |
    /// | 3 | Gpq, Gpq     |
    /// | 4 | Gpw, Gpw     |
    /// | 5 | Mem, GpbLo   |
    /// | 6 | Mem, Gpd     |
    /// | 7 | Mem, Gpq     |
    /// | 8 | Mem, Gpw     |
    /// +---+--------------+
    /// ```
    #[inline]
    pub fn xadd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: XaddEmitter<A, B> {
        <Self as XaddEmitter<A, B>>::xadd(self, op0, op1);
    }
}
