use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VPOPCNTD` (VPOPCNTD). 
/// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpopcntdEmitter<A, B> {
    fn vpopcntd(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VPOPCNTD128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VPOPCNTD128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntd(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VPOPCNTD256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntd(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VPOPCNTD256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntd(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VPOPCNTD512RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntd(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VPOPCNTD512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPOPCNTD_MASK` (VPOPCNTD). 
/// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpopcntdMaskEmitter<A, B> {
    fn vpopcntd_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntdMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntd_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VPOPCNTD128RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntd_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VPOPCNTD128RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntd_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VPOPCNTD256RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntd_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VPOPCNTD256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntd_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VPOPCNTD512RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntd_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VPOPCNTD512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPOPCNTD_MASKZ` (VPOPCNTD). 
/// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpopcntdMaskzEmitter<A, B> {
    fn vpopcntd_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntdMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntd_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VPOPCNTD128RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntd_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VPOPCNTD128RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntd_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VPOPCNTD256RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntd_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VPOPCNTD256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntd_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VPOPCNTD512RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntdMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntd_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VPOPCNTD512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPOPCNTQ` (VPOPCNTQ). 
/// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpopcntqEmitter<A, B> {
    fn vpopcntq(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VPOPCNTQ128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VPOPCNTQ128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntq(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VPOPCNTQ256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntq(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VPOPCNTQ256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntq(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VPOPCNTQ512RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntq(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VPOPCNTQ512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPOPCNTQ_MASK` (VPOPCNTQ). 
/// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpopcntqMaskEmitter<A, B> {
    fn vpopcntq_mask(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntqMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntq_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VPOPCNTQ128RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntq_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VPOPCNTQ128RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntq_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VPOPCNTQ256RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntq_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VPOPCNTQ256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntq_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VPOPCNTQ512RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntq_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VPOPCNTQ512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPOPCNTQ_MASKZ` (VPOPCNTQ). 
/// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Ymm, Mem |
/// | 4 | Ymm, Ymm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait VpopcntqMaskzEmitter<A, B> {
    fn vpopcntq_maskz(&mut self, op0: A, op1: B);
}

impl<'a> VpopcntqMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vpopcntq_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VPOPCNTQ128RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vpopcntq_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VPOPCNTQ128RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vpopcntq_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VPOPCNTQ256RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vpopcntq_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VPOPCNTQ256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vpopcntq_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VPOPCNTQ512RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpopcntqMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vpopcntq_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VPOPCNTQ512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `VPOPCNTD` (VPOPCNTD). 
    /// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpopcntd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VpopcntdEmitter<A, B> {
        <Self as VpopcntdEmitter<A, B>>::vpopcntd(self, op0, op1);
    }
    /// `VPOPCNTD_MASK` (VPOPCNTD). 
    /// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpopcntd_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VpopcntdMaskEmitter<A, B> {
        <Self as VpopcntdMaskEmitter<A, B>>::vpopcntd_mask(self, op0, op1);
    }
    /// `VPOPCNTD_MASKZ` (VPOPCNTD). 
    /// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpopcntd_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VpopcntdMaskzEmitter<A, B> {
        <Self as VpopcntdMaskzEmitter<A, B>>::vpopcntd_maskz(self, op0, op1);
    }
    /// `VPOPCNTQ` (VPOPCNTQ). 
    /// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpopcntq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VpopcntqEmitter<A, B> {
        <Self as VpopcntqEmitter<A, B>>::vpopcntq(self, op0, op1);
    }
    /// `VPOPCNTQ_MASK` (VPOPCNTQ). 
    /// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpopcntq_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VpopcntqMaskEmitter<A, B> {
        <Self as VpopcntqMaskEmitter<A, B>>::vpopcntq_mask(self, op0, op1);
    }
    /// `VPOPCNTQ_MASKZ` (VPOPCNTQ). 
    /// This instruction counts the number of bits set to one in each byte, word, dword or qword element of its source (e.g., zmm2 or memory) and places the results in the destination register (zmm1). This instruction supports memory fault suppression.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPOPCNT.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Ymm, Mem |
    /// | 4 | Ymm, Ymm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpopcntq_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VpopcntqMaskzEmitter<A, B> {
        <Self as VpopcntqMaskzEmitter<A, B>>::vpopcntq_maskz(self, op0, op1);
    }
}
