use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `KADDB` (KADDB). 
/// Adds the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KADDW%3AKADDB%3AKADDQ%3AKADDD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KaddbEmitter<A, B, C> {
    fn kaddb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KaddbEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kaddb(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(KADDBKKK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `KADDW` (KADDW). 
/// Adds the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KADDW%3AKADDB%3AKADDQ%3AKADDD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KaddwEmitter<A, B, C> {
    fn kaddw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KaddwEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kaddw(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(KADDWKKK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `KANDB` (KANDB). 
/// Performs a bitwise AND between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KANDW%3AKANDB%3AKANDQ%3AKANDD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KandbEmitter<A, B, C> {
    fn kandb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KandbEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kandb(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(KANDBKKK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `KANDNB` (KANDNB). 
/// Performs a bitwise AND NOT between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KANDNW%3AKANDNB%3AKANDNQ%3AKANDND.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KandnbEmitter<A, B, C> {
    fn kandnb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KandnbEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kandnb(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(KANDNBKKK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `KMOVB` (KMOVB). 
/// Copies values from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be mask registers, memory location or general purpose. The instruction cannot be used to transfer data between general purpose registers and or memory locations.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KMOVW%3AKMOVB%3AKMOVQ%3AKMOVD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | Gpd, KReg  |
/// | 2 | KReg, Gpd  |
/// | 3 | KReg, KReg |
/// | 4 | KReg, Mem  |
/// | 5 | Mem, KReg  |
/// +---+------------+
/// ```
pub trait KmovbEmitter<A, B> {
    fn kmovb(&mut self, op0: A, op1: B);
}

impl<'a> KmovbEmitter<KReg, KReg> for Assembler<'a> {
    fn kmovb(&mut self, op0: KReg, op1: KReg) {
        self.emit(KMOVBKK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovbEmitter<KReg, Mem> for Assembler<'a> {
    fn kmovb(&mut self, op0: KReg, op1: Mem) {
        self.emit(KMOVBKM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovbEmitter<Mem, KReg> for Assembler<'a> {
    fn kmovb(&mut self, op0: Mem, op1: KReg) {
        self.emit(KMOVBMK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovbEmitter<KReg, Gpd> for Assembler<'a> {
    fn kmovb(&mut self, op0: KReg, op1: Gpd) {
        self.emit(KMOVBKR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> KmovbEmitter<Gpd, KReg> for Assembler<'a> {
    fn kmovb(&mut self, op0: Gpd, op1: KReg) {
        self.emit(KMOVBRK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `KNOTB` (KNOTB). 
/// Performs a bitwise NOT of vector mask k2 and writes the result into vector mask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KNOTW%3AKNOTB%3AKNOTQ%3AKNOTD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | KReg, KReg |
/// +---+------------+
/// ```
pub trait KnotbEmitter<A, B> {
    fn knotb(&mut self, op0: A, op1: B);
}

impl<'a> KnotbEmitter<KReg, KReg> for Assembler<'a> {
    fn knotb(&mut self, op0: KReg, op1: KReg) {
        self.emit(KNOTBKK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `KORB` (KORB). 
/// Performs a bitwise OR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KORW%3AKORB%3AKORQ%3AKORD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KorbEmitter<A, B, C> {
    fn korb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KorbEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn korb(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(KORBKKK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `KORTESTB` (KORTESTB). 
/// Performs a bitwise OR between the vector mask register k2, and the vector mask register k1, and sets CF and ZF based on the operation result.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KORTESTW%3AKORTESTB%3AKORTESTQ%3AKORTESTD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | KReg, KReg |
/// +---+------------+
/// ```
pub trait KortestbEmitter<A, B> {
    fn kortestb(&mut self, op0: A, op1: B);
}

impl<'a> KortestbEmitter<KReg, KReg> for Assembler<'a> {
    fn kortestb(&mut self, op0: KReg, op1: KReg) {
        self.emit(KORTESTBKK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `KSHIFTLB` (KSHIFTLB). 
/// Shifts 8/16/32/64 bits in the second operand (source operand) left by the count specified in immediate byte and place the least significant 8/16/32/64 bits of the result in the destination operand. The higher bits of the destination are zero-extended. The destination is set to zero if the count value is greater than 7 (for byte shift), 15 (for word shift), 31 (for doubleword shift) or 63 (for quadword shift).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KSHIFTLW%3AKSHIFTLB%3AKSHIFTLQ%3AKSHIFTLD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------------+
/// | # | Operands        |
/// +---+-----------------+
/// | 1 | KReg, KReg, Imm |
/// +---+-----------------+
/// ```
pub trait KshiftlbEmitter<A, B, C> {
    fn kshiftlb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KshiftlbEmitter<KReg, KReg, Imm> for Assembler<'a> {
    fn kshiftlb(&mut self, op0: KReg, op1: KReg, op2: Imm) {
        self.emit(KSHIFTLBKKI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `KSHIFTRB` (KSHIFTRB). 
/// Shifts 8/16/32/64 bits in the second operand (source operand) right by the count specified in immediate and place the least significant 8/16/32/64 bits of the result in the destination operand. The higher bits of the destination are zero-extended. The destination is set to zero if the count value is greater than 7 (for byte shift), 15 (for word shift), 31 (for doubleword shift) or 63 (for quadword shift).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KSHIFTRW%3AKSHIFTRB%3AKSHIFTRQ%3AKSHIFTRD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------------+
/// | # | Operands        |
/// +---+-----------------+
/// | 1 | KReg, KReg, Imm |
/// +---+-----------------+
/// ```
pub trait KshiftrbEmitter<A, B, C> {
    fn kshiftrb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KshiftrbEmitter<KReg, KReg, Imm> for Assembler<'a> {
    fn kshiftrb(&mut self, op0: KReg, op1: KReg, op2: Imm) {
        self.emit(KSHIFTRBKKI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `KTESTB` (KTESTB). 
/// Performs a bitwise comparison of the bits of the first source operand and corresponding bits in the second source operand. If the AND operation produces all zeros, the ZF is set else the ZF is clear. If the bitwise AND operation of the inverted first source operand with the second source operand produces all zeros the CF is set else the CF is clear. Only the EFLAGS register is updated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KTESTW%3AKTESTB%3AKTESTQ%3AKTESTD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | KReg, KReg |
/// +---+------------+
/// ```
pub trait KtestbEmitter<A, B> {
    fn ktestb(&mut self, op0: A, op1: B);
}

impl<'a> KtestbEmitter<KReg, KReg> for Assembler<'a> {
    fn ktestb(&mut self, op0: KReg, op1: KReg) {
        self.emit(KTESTBKK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `KTESTW` (KTESTW). 
/// Performs a bitwise comparison of the bits of the first source operand and corresponding bits in the second source operand. If the AND operation produces all zeros, the ZF is set else the ZF is clear. If the bitwise AND operation of the inverted first source operand with the second source operand produces all zeros the CF is set else the CF is clear. Only the EFLAGS register is updated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KTESTW%3AKTESTB%3AKTESTQ%3AKTESTD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------+
/// | # | Operands   |
/// +---+------------+
/// | 1 | KReg, KReg |
/// +---+------------+
/// ```
pub trait KtestwEmitter<A, B> {
    fn ktestw(&mut self, op0: A, op1: B);
}

impl<'a> KtestwEmitter<KReg, KReg> for Assembler<'a> {
    fn ktestw(&mut self, op0: KReg, op1: KReg) {
        self.emit(KTESTWKK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `KXNORB` (KXNORB). 
/// Performs a bitwise XNOR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KXNORW%3AKXNORB%3AKXNORQ%3AKXNORD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KxnorbEmitter<A, B, C> {
    fn kxnorb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KxnorbEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kxnorb(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(KXNORBKKK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `KXORB` (KXORB). 
/// Performs a bitwise XOR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KXORW%3AKXORB%3AKXORQ%3AKXORD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+------------------+
/// | # | Operands         |
/// +---+------------------+
/// | 1 | KReg, KReg, KReg |
/// +---+------------------+
/// ```
pub trait KxorbEmitter<A, B, C> {
    fn kxorb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> KxorbEmitter<KReg, KReg, KReg> for Assembler<'a> {
    fn kxorb(&mut self, op0: KReg, op1: KReg, op2: KReg) {
        self.emit(KXORBKKK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDNPD` (VANDNPD). 
/// Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandnpdEmitter<A, B, C> {
    fn vandnpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandnpdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandnpd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDNPD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandnpd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDNPD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandnpd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDNPD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandnpd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDNPD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandnpd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDNPD512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandnpd(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDNPD512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDNPD_MASK` (VANDNPD). 
/// Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandnpdMaskEmitter<A, B, C> {
    fn vandnpd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandnpdMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandnpd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDNPD128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandnpd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDNPD128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandnpd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDNPD256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandnpd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDNPD256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandnpd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDNPD512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandnpd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDNPD512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDNPD_MASKZ` (VANDNPD). 
/// Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandnpdMaskzEmitter<A, B, C> {
    fn vandnpd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandnpdMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandnpd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDNPD128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandnpd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDNPD128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandnpd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDNPD256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandnpd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDNPD256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandnpd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDNPD512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpdMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandnpd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDNPD512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDNPS` (VANDNPS). 
/// Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandnpsEmitter<A, B, C> {
    fn vandnps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandnpsEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandnps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDNPS128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandnps(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDNPS128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandnps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDNPS256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandnps(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDNPS256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandnps(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDNPS512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandnps(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDNPS512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDNPS_MASK` (VANDNPS). 
/// Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandnpsMaskEmitter<A, B, C> {
    fn vandnps_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandnpsMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandnps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDNPS128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandnps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDNPS128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandnps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDNPS256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandnps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDNPS256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandnps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDNPS512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandnps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDNPS512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDNPS_MASKZ` (VANDNPS). 
/// Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandnpsMaskzEmitter<A, B, C> {
    fn vandnps_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandnpsMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandnps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDNPS128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandnps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDNPS128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandnps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDNPS256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandnps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDNPS256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandnps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDNPS512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandnpsMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandnps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDNPS512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDPD` (VANDPD). 
/// Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandpdEmitter<A, B, C> {
    fn vandpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandpdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandpd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDPD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandpd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDPD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandpd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDPD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandpd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDPD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandpd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDPD512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandpd(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDPD512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDPD_MASK` (VANDPD). 
/// Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandpdMaskEmitter<A, B, C> {
    fn vandpd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandpdMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandpd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDPD128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandpd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDPD128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandpd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDPD256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandpd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDPD256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandpd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDPD512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandpd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDPD512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDPD_MASKZ` (VANDPD). 
/// Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandpdMaskzEmitter<A, B, C> {
    fn vandpd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandpdMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandpd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDPD128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandpd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDPD128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandpd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDPD256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandpd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDPD256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandpd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDPD512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpdMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandpd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDPD512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDPS` (VANDPS). 
/// Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandpsEmitter<A, B, C> {
    fn vandps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandpsEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDPS128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandps(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDPS128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDPS256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandps(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDPS256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandps(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDPS512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandps(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDPS512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDPS_MASK` (VANDPS). 
/// Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandpsMaskEmitter<A, B, C> {
    fn vandps_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandpsMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDPS128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDPS128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDPS256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDPS256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDPS512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDPS512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VANDPS_MASKZ` (VANDPS). 
/// Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VandpsMaskzEmitter<A, B, C> {
    fn vandps_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VandpsMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vandps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VANDPS128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vandps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VANDPS128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vandps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VANDPS256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vandps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VANDPS256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vandps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VANDPS512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VandpsMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vandps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VANDPS512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VBROADCASTF32X2` (VBROADCASTF32X2). 
/// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Ymm, Xmm |
/// | 3 | Zmm, Mem |
/// | 4 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait Vbroadcastf32x2Emitter<A, B> {
    fn vbroadcastf32x2(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcastf32x2Emitter<Ymm, Xmm> for Assembler<'a> {
    fn vbroadcastf32x2(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VBROADCASTF32X2_256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf32x2Emitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcastf32x2(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTF32X2_256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf32x2Emitter<Zmm, Xmm> for Assembler<'a> {
    fn vbroadcastf32x2(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(VBROADCASTF32X2_512RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf32x2Emitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcastf32x2(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTF32X2_512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTF32X2_MASK` (VBROADCASTF32X2). 
/// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Ymm, Xmm |
/// | 3 | Zmm, Mem |
/// | 4 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait Vbroadcastf32x2MaskEmitter<A, B> {
    fn vbroadcastf32x2_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcastf32x2MaskEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vbroadcastf32x2_mask(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VBROADCASTF32X2_256RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf32x2MaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcastf32x2_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTF32X2_256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf32x2MaskEmitter<Zmm, Xmm> for Assembler<'a> {
    fn vbroadcastf32x2_mask(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(VBROADCASTF32X2_512RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf32x2MaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcastf32x2_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTF32X2_512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTF32X2_MASKZ` (VBROADCASTF32X2). 
/// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Ymm, Xmm |
/// | 3 | Zmm, Mem |
/// | 4 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait Vbroadcastf32x2MaskzEmitter<A, B> {
    fn vbroadcastf32x2_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcastf32x2MaskzEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vbroadcastf32x2_maskz(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VBROADCASTF32X2_256RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf32x2MaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcastf32x2_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTF32X2_256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf32x2MaskzEmitter<Zmm, Xmm> for Assembler<'a> {
    fn vbroadcastf32x2_maskz(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(VBROADCASTF32X2_512RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf32x2MaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcastf32x2_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTF32X2_512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTF32X8` (VBROADCASTF32X8). 
/// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcastf32x8Emitter<A, B> {
    fn vbroadcastf32x8(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcastf32x8Emitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcastf32x8(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTF32X8_512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTF32X8_MASK` (VBROADCASTF32X8). 
/// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcastf32x8MaskEmitter<A, B> {
    fn vbroadcastf32x8_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcastf32x8MaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcastf32x8_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTF32X8_512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTF32X8_MASKZ` (VBROADCASTF32X8). 
/// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcastf32x8MaskzEmitter<A, B> {
    fn vbroadcastf32x8_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcastf32x8MaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcastf32x8_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTF32X8_512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTF64X2` (VBROADCASTF64X2). 
/// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcastf64x2Emitter<A, B> {
    fn vbroadcastf64x2(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcastf64x2Emitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcastf64x2(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTF64X2_256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf64x2Emitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcastf64x2(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTF64X2_512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTF64X2_MASK` (VBROADCASTF64X2). 
/// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcastf64x2MaskEmitter<A, B> {
    fn vbroadcastf64x2_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcastf64x2MaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcastf64x2_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTF64X2_256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf64x2MaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcastf64x2_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTF64X2_512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTF64X2_MASKZ` (VBROADCASTF64X2). 
/// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcastf64x2MaskzEmitter<A, B> {
    fn vbroadcastf64x2_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcastf64x2MaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcastf64x2_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTF64X2_256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf64x2MaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcastf64x2_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTF64X2_512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI32X2`.
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
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait Vbroadcasti32x2Emitter<A, B> {
    fn vbroadcasti32x2(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti32x2Emitter<Xmm, Xmm> for Assembler<'a> {
    fn vbroadcasti32x2(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VBROADCASTI32X2_128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2Emitter<Xmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x2(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VBROADCASTI32X2_128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2Emitter<Ymm, Xmm> for Assembler<'a> {
    fn vbroadcasti32x2(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VBROADCASTI32X2_256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2Emitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcasti32x2(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTI32X2_256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2Emitter<Zmm, Xmm> for Assembler<'a> {
    fn vbroadcasti32x2(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(VBROADCASTI32X2_512RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2Emitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x2(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI32X2_512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI32X2_MASK`.
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
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait Vbroadcasti32x2MaskEmitter<A, B> {
    fn vbroadcasti32x2_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti32x2MaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vbroadcasti32x2_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VBROADCASTI32X2_128RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2MaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x2_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VBROADCASTI32X2_128RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2MaskEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vbroadcasti32x2_mask(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VBROADCASTI32X2_256RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2MaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcasti32x2_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTI32X2_256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2MaskEmitter<Zmm, Xmm> for Assembler<'a> {
    fn vbroadcasti32x2_mask(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(VBROADCASTI32X2_512RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2MaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x2_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI32X2_512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI32X2_MASKZ`.
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
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Xmm |
/// +---+----------+
/// ```
pub trait Vbroadcasti32x2MaskzEmitter<A, B> {
    fn vbroadcasti32x2_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti32x2MaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vbroadcasti32x2_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VBROADCASTI32X2_128RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2MaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x2_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VBROADCASTI32X2_128RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2MaskzEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vbroadcasti32x2_maskz(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VBROADCASTI32X2_256RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2MaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcasti32x2_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTI32X2_256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2MaskzEmitter<Zmm, Xmm> for Assembler<'a> {
    fn vbroadcasti32x2_maskz(&mut self, op0: Zmm, op1: Xmm) {
        self.emit(VBROADCASTI32X2_512RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x2MaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x2_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI32X2_512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI32X4` (VBROADCASTI32X4). 
/// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcasti32x4Emitter<A, B> {
    fn vbroadcasti32x4(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti32x4Emitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcasti32x4(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTI32X4_256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x4Emitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x4(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI32X4_512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI32X4_MASK` (VBROADCASTI32X4). 
/// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcasti32x4MaskEmitter<A, B> {
    fn vbroadcasti32x4_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti32x4MaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcasti32x4_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTI32X4_256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x4MaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x4_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI32X4_512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI32X4_MASKZ` (VBROADCASTI32X4). 
/// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcasti32x4MaskzEmitter<A, B> {
    fn vbroadcasti32x4_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti32x4MaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcasti32x4_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTI32X4_256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti32x4MaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x4_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI32X4_512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI32X8` (VBROADCASTI32X8). 
/// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcasti32x8Emitter<A, B> {
    fn vbroadcasti32x8(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti32x8Emitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x8(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI32X8_512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI32X8_MASK` (VBROADCASTI32X8). 
/// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcasti32x8MaskEmitter<A, B> {
    fn vbroadcasti32x8_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti32x8MaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x8_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI32X8_512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI32X8_MASKZ` (VBROADCASTI32X8). 
/// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcasti32x8MaskzEmitter<A, B> {
    fn vbroadcasti32x8_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti32x8MaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti32x8_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI32X8_512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI64X2` (VBROADCASTI64X2). 
/// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcasti64x2Emitter<A, B> {
    fn vbroadcasti64x2(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti64x2Emitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcasti64x2(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTI64X2_256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti64x2Emitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti64x2(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI64X2_512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI64X2_MASK` (VBROADCASTI64X2). 
/// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcasti64x2MaskEmitter<A, B> {
    fn vbroadcasti64x2_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti64x2MaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcasti64x2_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTI64X2_256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti64x2MaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti64x2_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI64X2_512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VBROADCASTI64X2_MASKZ` (VBROADCASTI64X2). 
/// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Mem |
/// | 2 | Zmm, Mem |
/// +---+----------+
/// ```
pub trait Vbroadcasti64x2MaskzEmitter<A, B> {
    fn vbroadcasti64x2_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcasti64x2MaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcasti64x2_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTI64X2_256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcasti64x2MaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vbroadcasti64x2_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VBROADCASTI64X2_512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPD2QQ` (VCVTPD2QQ). 
/// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
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
pub trait Vcvtpd2qqEmitter<A, B> {
    fn vcvtpd2qq(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtpd2qqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtpd2qq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTPD2QQ128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtpd2qq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTPD2QQ128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vcvtpd2qq(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VCVTPD2QQ256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtpd2qq(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTPD2QQ256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtpd2qq(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTPD2QQ512RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvtpd2qq(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTPD2QQ512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPD2QQ_ER` (VCVTPD2QQ). 
/// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtpd2qqErEmitter<A, B> {
    fn vcvtpd2qq_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtpd2qqErEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtpd2qq_er(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTPD2QQ512RR_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPD2QQ_MASK` (VCVTPD2QQ). 
/// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
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
pub trait Vcvtpd2qqMaskEmitter<A, B> {
    fn vcvtpd2qq_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtpd2qqMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtpd2qq_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTPD2QQ128RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtpd2qq_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTPD2QQ128RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vcvtpd2qq_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VCVTPD2QQ256RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtpd2qq_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTPD2QQ256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtpd2qq_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTPD2QQ512RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvtpd2qq_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTPD2QQ512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPD2QQ_MASK_ER` (VCVTPD2QQ). 
/// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtpd2qqMaskErEmitter<A, B> {
    fn vcvtpd2qq_mask_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtpd2qqMaskErEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtpd2qq_mask_er(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTPD2QQ512RR_MASK_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPD2QQ_MASKZ` (VCVTPD2QQ). 
/// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
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
pub trait Vcvtpd2qqMaskzEmitter<A, B> {
    fn vcvtpd2qq_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtpd2qqMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtpd2qq_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTPD2QQ128RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtpd2qq_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTPD2QQ128RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vcvtpd2qq_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VCVTPD2QQ256RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtpd2qq_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTPD2QQ256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtpd2qq_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTPD2QQ512RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtpd2qqMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvtpd2qq_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTPD2QQ512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPD2QQ_MASKZ_ER` (VCVTPD2QQ). 
/// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtpd2qqMaskzErEmitter<A, B> {
    fn vcvtpd2qq_maskz_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtpd2qqMaskzErEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtpd2qq_maskz_er(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTPD2QQ512RR_MASKZ_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPS2QQ` (VCVTPS2QQ). 
/// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
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
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvtps2qqEmitter<A, B> {
    fn vcvtps2qq(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtps2qqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtps2qq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTPS2QQ128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtps2qq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTPS2QQ128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vcvtps2qq(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VCVTPS2QQ256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtps2qq(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTPS2QQ256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvtps2qq(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTPS2QQ512RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvtps2qq(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTPS2QQ512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPS2QQ_ER` (VCVTPS2QQ). 
/// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvtps2qqErEmitter<A, B> {
    fn vcvtps2qq_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtps2qqErEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvtps2qq_er(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTPS2QQ512RR_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPS2QQ_MASK` (VCVTPS2QQ). 
/// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
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
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvtps2qqMaskEmitter<A, B> {
    fn vcvtps2qq_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtps2qqMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtps2qq_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTPS2QQ128RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtps2qq_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTPS2QQ128RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqMaskEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vcvtps2qq_mask(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VCVTPS2QQ256RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtps2qq_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTPS2QQ256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqMaskEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvtps2qq_mask(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTPS2QQ512RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvtps2qq_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTPS2QQ512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPS2QQ_MASK_ER` (VCVTPS2QQ). 
/// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvtps2qqMaskErEmitter<A, B> {
    fn vcvtps2qq_mask_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtps2qqMaskErEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvtps2qq_mask_er(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTPS2QQ512RR_MASK_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPS2QQ_MASKZ` (VCVTPS2QQ). 
/// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
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
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvtps2qqMaskzEmitter<A, B> {
    fn vcvtps2qq_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtps2qqMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtps2qq_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTPS2QQ128RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtps2qq_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTPS2QQ128RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqMaskzEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vcvtps2qq_maskz(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VCVTPS2QQ256RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtps2qq_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTPS2QQ256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqMaskzEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvtps2qq_maskz(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTPS2QQ512RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtps2qqMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvtps2qq_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTPS2QQ512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTPS2QQ_MASKZ_ER` (VCVTPS2QQ). 
/// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvtps2qqMaskzErEmitter<A, B> {
    fn vcvtps2qq_maskz_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtps2qqMaskzErEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvtps2qq_maskz_er(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTPS2QQ512RR_MASKZ_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PD` (VCVTQQ2PD). 
/// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
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
pub trait Vcvtqq2pdEmitter<A, B> {
    fn vcvtqq2pd(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2pdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtqq2pd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTQQ2PD128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtqq2pd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTQQ2PD128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vcvtqq2pd(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VCVTQQ2PD256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtqq2pd(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTQQ2PD256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtqq2pd(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTQQ2PD512RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvtqq2pd(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTQQ2PD512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PD_ER` (VCVTQQ2PD). 
/// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtqq2pdErEmitter<A, B> {
    fn vcvtqq2pd_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2pdErEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtqq2pd_er(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTQQ2PD512RR_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PD_MASK` (VCVTQQ2PD). 
/// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
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
pub trait Vcvtqq2pdMaskEmitter<A, B> {
    fn vcvtqq2pd_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2pdMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtqq2pd_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTQQ2PD128RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtqq2pd_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTQQ2PD128RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vcvtqq2pd_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VCVTQQ2PD256RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtqq2pd_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTQQ2PD256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtqq2pd_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTQQ2PD512RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvtqq2pd_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTQQ2PD512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PD_MASK_ER` (VCVTQQ2PD). 
/// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtqq2pdMaskErEmitter<A, B> {
    fn vcvtqq2pd_mask_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2pdMaskErEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtqq2pd_mask_er(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTQQ2PD512RR_MASK_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PD_MASKZ` (VCVTQQ2PD). 
/// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
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
pub trait Vcvtqq2pdMaskzEmitter<A, B> {
    fn vcvtqq2pd_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2pdMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtqq2pd_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTQQ2PD128RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtqq2pd_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTQQ2PD128RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vcvtqq2pd_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VCVTQQ2PD256RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtqq2pd_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTQQ2PD256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtqq2pd_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTQQ2PD512RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2pdMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvtqq2pd_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTQQ2PD512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PD_MASKZ_ER` (VCVTQQ2PD). 
/// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtqq2pdMaskzErEmitter<A, B> {
    fn vcvtqq2pd_maskz_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2pdMaskzErEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvtqq2pd_maskz_er(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTQQ2PD512RR_MASKZ_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PS` (VCVTQQ2PS). 
/// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Xmm, Ymm |
/// | 4 | Ymm, Mem |
/// | 5 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtqq2psEmitter<A, B> {
    fn vcvtqq2ps(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2psEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtqq2ps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTQQ2PS128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtqq2ps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTQQ2PS128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vcvtqq2ps(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(VCVTQQ2PS256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vcvtqq2ps(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(VCVTQQ2PS512RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtqq2ps(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTQQ2PS512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PS_ER` (VCVTQQ2PS). 
/// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtqq2psErEmitter<A, B> {
    fn vcvtqq2ps_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2psErEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vcvtqq2ps_er(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(VCVTQQ2PS512RR_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PS_MASK` (VCVTQQ2PS). 
/// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Xmm, Ymm |
/// | 4 | Ymm, Mem |
/// | 5 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtqq2psMaskEmitter<A, B> {
    fn vcvtqq2ps_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2psMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtqq2ps_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTQQ2PS128RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtqq2ps_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTQQ2PS128RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psMaskEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vcvtqq2ps_mask(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(VCVTQQ2PS256RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psMaskEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vcvtqq2ps_mask(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(VCVTQQ2PS512RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtqq2ps_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTQQ2PS512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PS_MASK_ER` (VCVTQQ2PS). 
/// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtqq2psMaskErEmitter<A, B> {
    fn vcvtqq2ps_mask_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2psMaskErEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vcvtqq2ps_mask_er(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(VCVTQQ2PS512RR_MASK_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PS_MASKZ` (VCVTQQ2PS). 
/// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Mem |
/// | 2 | Xmm, Xmm |
/// | 3 | Xmm, Ymm |
/// | 4 | Ymm, Mem |
/// | 5 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtqq2psMaskzEmitter<A, B> {
    fn vcvtqq2ps_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2psMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvtqq2ps_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTQQ2PS128RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvtqq2ps_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTQQ2PS128RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psMaskzEmitter<Xmm, Ymm> for Assembler<'a> {
    fn vcvtqq2ps_maskz(&mut self, op0: Xmm, op1: Ymm) {
        self.emit(VCVTQQ2PS256RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psMaskzEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vcvtqq2ps_maskz(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(VCVTQQ2PS512RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvtqq2psMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvtqq2ps_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTQQ2PS512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTQQ2PS_MASKZ_ER` (VCVTQQ2PS). 
/// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Ymm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvtqq2psMaskzErEmitter<A, B> {
    fn vcvtqq2ps_maskz_er(&mut self, op0: A, op1: B);
}

impl<'a> Vcvtqq2psMaskzErEmitter<Ymm, Zmm> for Assembler<'a> {
    fn vcvtqq2ps_maskz_er(&mut self, op0: Ymm, op1: Zmm) {
        self.emit(VCVTQQ2PS512RR_MASKZ_ER, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPD2QQ` (VCVTTPD2QQ). 
/// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
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
pub trait Vcvttpd2qqEmitter<A, B> {
    fn vcvttpd2qq(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttpd2qqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvttpd2qq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTTPD2QQ128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvttpd2qq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTTPD2QQ128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vcvttpd2qq(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VCVTTPD2QQ256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvttpd2qq(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTTPD2QQ256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvttpd2qq(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTTPD2QQ512RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvttpd2qq(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTTPD2QQ512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPD2QQ_MASK` (VCVTTPD2QQ). 
/// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
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
pub trait Vcvttpd2qqMaskEmitter<A, B> {
    fn vcvttpd2qq_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttpd2qqMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvttpd2qq_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTTPD2QQ128RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvttpd2qq_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTTPD2QQ128RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqMaskEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vcvttpd2qq_mask(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VCVTTPD2QQ256RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvttpd2qq_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTTPD2QQ256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqMaskEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvttpd2qq_mask(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTTPD2QQ512RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvttpd2qq_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTTPD2QQ512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPD2QQ_MASK_SAE` (VCVTTPD2QQ). 
/// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvttpd2qqMaskSaeEmitter<A, B> {
    fn vcvttpd2qq_mask_sae(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttpd2qqMaskSaeEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvttpd2qq_mask_sae(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTTPD2QQ512RR_MASK_SAE, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPD2QQ_MASKZ` (VCVTTPD2QQ). 
/// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
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
pub trait Vcvttpd2qqMaskzEmitter<A, B> {
    fn vcvttpd2qq_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttpd2qqMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvttpd2qq_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTTPD2QQ128RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvttpd2qq_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTTPD2QQ128RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqMaskzEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vcvttpd2qq_maskz(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VCVTTPD2QQ256RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvttpd2qq_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTTPD2QQ256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqMaskzEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvttpd2qq_maskz(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTTPD2QQ512RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttpd2qqMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvttpd2qq_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTTPD2QQ512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPD2QQ_MASKZ_SAE` (VCVTTPD2QQ). 
/// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvttpd2qqMaskzSaeEmitter<A, B> {
    fn vcvttpd2qq_maskz_sae(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttpd2qqMaskzSaeEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvttpd2qq_maskz_sae(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTTPD2QQ512RR_MASKZ_SAE, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPD2QQ_SAE` (VCVTTPD2QQ). 
/// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Zmm |
/// +---+----------+
/// ```
pub trait Vcvttpd2qqSaeEmitter<A, B> {
    fn vcvttpd2qq_sae(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttpd2qqSaeEmitter<Zmm, Zmm> for Assembler<'a> {
    fn vcvttpd2qq_sae(&mut self, op0: Zmm, op1: Zmm) {
        self.emit(VCVTTPD2QQ512RR_SAE, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPS2QQ` (VCVTTPS2QQ). 
/// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
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
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvttps2qqEmitter<A, B> {
    fn vcvttps2qq(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttps2qqEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvttps2qq(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTTPS2QQ128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvttps2qq(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTTPS2QQ128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vcvttps2qq(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VCVTTPS2QQ256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvttps2qq(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTTPS2QQ256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvttps2qq(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTTPS2QQ512RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvttps2qq(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTTPS2QQ512RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPS2QQ_MASK` (VCVTTPS2QQ). 
/// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
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
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvttps2qqMaskEmitter<A, B> {
    fn vcvttps2qq_mask(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttps2qqMaskEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvttps2qq_mask(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTTPS2QQ128RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqMaskEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvttps2qq_mask(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTTPS2QQ128RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqMaskEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vcvttps2qq_mask(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VCVTTPS2QQ256RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqMaskEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvttps2qq_mask(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTTPS2QQ256RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqMaskEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvttps2qq_mask(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTTPS2QQ512RR_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqMaskEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvttps2qq_mask(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTTPS2QQ512RM_MASK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPS2QQ_MASK_SAE` (VCVTTPS2QQ). 
/// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvttps2qqMaskSaeEmitter<A, B> {
    fn vcvttps2qq_mask_sae(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttps2qqMaskSaeEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvttps2qq_mask_sae(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTTPS2QQ512RR_MASK_SAE, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPS2QQ_MASKZ` (VCVTTPS2QQ). 
/// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
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
/// | 4 | Ymm, Xmm |
/// | 5 | Zmm, Mem |
/// | 6 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvttps2qqMaskzEmitter<A, B> {
    fn vcvttps2qq_maskz(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttps2qqMaskzEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vcvttps2qq_maskz(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VCVTTPS2QQ128RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqMaskzEmitter<Xmm, Mem> for Assembler<'a> {
    fn vcvttps2qq_maskz(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VCVTTPS2QQ128RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqMaskzEmitter<Ymm, Xmm> for Assembler<'a> {
    fn vcvttps2qq_maskz(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VCVTTPS2QQ256RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqMaskzEmitter<Ymm, Mem> for Assembler<'a> {
    fn vcvttps2qq_maskz(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VCVTTPS2QQ256RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqMaskzEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvttps2qq_maskz(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTTPS2QQ512RR_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vcvttps2qqMaskzEmitter<Zmm, Mem> for Assembler<'a> {
    fn vcvttps2qq_maskz(&mut self, op0: Zmm, op1: Mem) {
        self.emit(VCVTTPS2QQ512RM_MASKZ, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPS2QQ_MASKZ_SAE` (VCVTTPS2QQ). 
/// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvttps2qqMaskzSaeEmitter<A, B> {
    fn vcvttps2qq_maskz_sae(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttps2qqMaskzSaeEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvttps2qq_maskz_sae(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTTPS2QQ512RR_MASKZ_SAE, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCVTTPS2QQ_SAE` (VCVTTPS2QQ). 
/// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Zmm, Ymm |
/// +---+----------+
/// ```
pub trait Vcvttps2qqSaeEmitter<A, B> {
    fn vcvttps2qq_sae(&mut self, op0: A, op1: B);
}

impl<'a> Vcvttps2qqSaeEmitter<Zmm, Ymm> for Assembler<'a> {
    fn vcvttps2qq_sae(&mut self, op0: Zmm, op1: Ymm) {
        self.emit(VCVTTPS2QQ512RR_SAE, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VFPCLASSPD` (VFPCLASSPD). 
/// The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Mem, Imm |
/// | 2 | KReg, Xmm, Imm |
/// | 3 | KReg, Ymm, Imm |
/// | 4 | KReg, Zmm, Imm |
/// +---+----------------+
/// ```
pub trait VfpclasspdEmitter<A, B, C> {
    fn vfpclasspd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VfpclasspdEmitter<KReg, Xmm, Imm> for Assembler<'a> {
    fn vfpclasspd(&mut self, op0: KReg, op1: Xmm, op2: Imm) {
        self.emit(VFPCLASSPD128KRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspdEmitter<KReg, Mem, Imm> for Assembler<'a> {
    fn vfpclasspd(&mut self, op0: KReg, op1: Mem, op2: Imm) {
        self.emit(VFPCLASSPD128KMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspdEmitter<KReg, Ymm, Imm> for Assembler<'a> {
    fn vfpclasspd(&mut self, op0: KReg, op1: Ymm, op2: Imm) {
        self.emit(VFPCLASSPD256KRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspdEmitter<KReg, Zmm, Imm> for Assembler<'a> {
    fn vfpclasspd(&mut self, op0: KReg, op1: Zmm, op2: Imm) {
        self.emit(VFPCLASSPD512KRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VFPCLASSPD_MASK` (VFPCLASSPD). 
/// The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Mem, Imm |
/// | 2 | KReg, Xmm, Imm |
/// | 3 | KReg, Ymm, Imm |
/// | 4 | KReg, Zmm, Imm |
/// +---+----------------+
/// ```
pub trait VfpclasspdMaskEmitter<A, B, C> {
    fn vfpclasspd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VfpclasspdMaskEmitter<KReg, Xmm, Imm> for Assembler<'a> {
    fn vfpclasspd_mask(&mut self, op0: KReg, op1: Xmm, op2: Imm) {
        self.emit(VFPCLASSPD128KRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspdMaskEmitter<KReg, Mem, Imm> for Assembler<'a> {
    fn vfpclasspd_mask(&mut self, op0: KReg, op1: Mem, op2: Imm) {
        self.emit(VFPCLASSPD128KMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspdMaskEmitter<KReg, Ymm, Imm> for Assembler<'a> {
    fn vfpclasspd_mask(&mut self, op0: KReg, op1: Ymm, op2: Imm) {
        self.emit(VFPCLASSPD256KRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspdMaskEmitter<KReg, Zmm, Imm> for Assembler<'a> {
    fn vfpclasspd_mask(&mut self, op0: KReg, op1: Zmm, op2: Imm) {
        self.emit(VFPCLASSPD512KRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VFPCLASSPS` (VFPCLASSPS). 
/// The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Mem, Imm |
/// | 2 | KReg, Xmm, Imm |
/// | 3 | KReg, Ymm, Imm |
/// | 4 | KReg, Zmm, Imm |
/// +---+----------------+
/// ```
pub trait VfpclasspsEmitter<A, B, C> {
    fn vfpclassps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VfpclasspsEmitter<KReg, Xmm, Imm> for Assembler<'a> {
    fn vfpclassps(&mut self, op0: KReg, op1: Xmm, op2: Imm) {
        self.emit(VFPCLASSPS128KRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspsEmitter<KReg, Mem, Imm> for Assembler<'a> {
    fn vfpclassps(&mut self, op0: KReg, op1: Mem, op2: Imm) {
        self.emit(VFPCLASSPS128KMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspsEmitter<KReg, Ymm, Imm> for Assembler<'a> {
    fn vfpclassps(&mut self, op0: KReg, op1: Ymm, op2: Imm) {
        self.emit(VFPCLASSPS256KRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspsEmitter<KReg, Zmm, Imm> for Assembler<'a> {
    fn vfpclassps(&mut self, op0: KReg, op1: Zmm, op2: Imm) {
        self.emit(VFPCLASSPS512KRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VFPCLASSPS_MASK` (VFPCLASSPS). 
/// The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Mem, Imm |
/// | 2 | KReg, Xmm, Imm |
/// | 3 | KReg, Ymm, Imm |
/// | 4 | KReg, Zmm, Imm |
/// +---+----------------+
/// ```
pub trait VfpclasspsMaskEmitter<A, B, C> {
    fn vfpclassps_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VfpclasspsMaskEmitter<KReg, Xmm, Imm> for Assembler<'a> {
    fn vfpclassps_mask(&mut self, op0: KReg, op1: Xmm, op2: Imm) {
        self.emit(VFPCLASSPS128KRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspsMaskEmitter<KReg, Mem, Imm> for Assembler<'a> {
    fn vfpclassps_mask(&mut self, op0: KReg, op1: Mem, op2: Imm) {
        self.emit(VFPCLASSPS128KMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspsMaskEmitter<KReg, Ymm, Imm> for Assembler<'a> {
    fn vfpclassps_mask(&mut self, op0: KReg, op1: Ymm, op2: Imm) {
        self.emit(VFPCLASSPS256KRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasspsMaskEmitter<KReg, Zmm, Imm> for Assembler<'a> {
    fn vfpclassps_mask(&mut self, op0: KReg, op1: Zmm, op2: Imm) {
        self.emit(VFPCLASSPS512KRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VFPCLASSSD` (VFPCLASSSD). 
/// The FPCLASSSD instruction checks the low double precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSSD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Mem, Imm |
/// | 2 | KReg, Xmm, Imm |
/// +---+----------------+
/// ```
pub trait VfpclasssdEmitter<A, B, C> {
    fn vfpclasssd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VfpclasssdEmitter<KReg, Xmm, Imm> for Assembler<'a> {
    fn vfpclasssd(&mut self, op0: KReg, op1: Xmm, op2: Imm) {
        self.emit(VFPCLASSSDKRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasssdEmitter<KReg, Mem, Imm> for Assembler<'a> {
    fn vfpclasssd(&mut self, op0: KReg, op1: Mem, op2: Imm) {
        self.emit(VFPCLASSSDKMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VFPCLASSSD_MASK` (VFPCLASSSD). 
/// The FPCLASSSD instruction checks the low double precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSSD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Mem, Imm |
/// | 2 | KReg, Xmm, Imm |
/// +---+----------------+
/// ```
pub trait VfpclasssdMaskEmitter<A, B, C> {
    fn vfpclasssd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VfpclasssdMaskEmitter<KReg, Xmm, Imm> for Assembler<'a> {
    fn vfpclasssd_mask(&mut self, op0: KReg, op1: Xmm, op2: Imm) {
        self.emit(VFPCLASSSDKRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclasssdMaskEmitter<KReg, Mem, Imm> for Assembler<'a> {
    fn vfpclasssd_mask(&mut self, op0: KReg, op1: Mem, op2: Imm) {
        self.emit(VFPCLASSSDKMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VFPCLASSSS` (VFPCLASSSS). 
/// The FPCLASSSS instruction checks the low single-precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSSS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Mem, Imm |
/// | 2 | KReg, Xmm, Imm |
/// +---+----------------+
/// ```
pub trait VfpclassssEmitter<A, B, C> {
    fn vfpclassss(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VfpclassssEmitter<KReg, Xmm, Imm> for Assembler<'a> {
    fn vfpclassss(&mut self, op0: KReg, op1: Xmm, op2: Imm) {
        self.emit(VFPCLASSSSKRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclassssEmitter<KReg, Mem, Imm> for Assembler<'a> {
    fn vfpclassss(&mut self, op0: KReg, op1: Mem, op2: Imm) {
        self.emit(VFPCLASSSSKMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VFPCLASSSS_MASK` (VFPCLASSSS). 
/// The FPCLASSSS instruction checks the low single-precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSSS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------------+
/// | # | Operands       |
/// +---+----------------+
/// | 1 | KReg, Mem, Imm |
/// | 2 | KReg, Xmm, Imm |
/// +---+----------------+
/// ```
pub trait VfpclassssMaskEmitter<A, B, C> {
    fn vfpclassss_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VfpclassssMaskEmitter<KReg, Xmm, Imm> for Assembler<'a> {
    fn vfpclassss_mask(&mut self, op0: KReg, op1: Xmm, op2: Imm) {
        self.emit(VFPCLASSSSKRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VfpclassssMaskEmitter<KReg, Mem, Imm> for Assembler<'a> {
    fn vfpclassss_mask(&mut self, op0: KReg, op1: Mem, op2: Imm) {
        self.emit(VFPCLASSSSKMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VINSERTF32X8` (VINSERTF32X8). 
/// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Mem, Imm |
/// | 2 | Zmm, Zmm, Ymm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinsertf32x8Emitter<A, B, C, D> {
    fn vinsertf32x8(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinsertf32x8Emitter<Zmm, Zmm, Ymm, Imm> for Assembler<'a> {
    fn vinsertf32x8(&mut self, op0: Zmm, op1: Zmm, op2: Ymm, op3: Imm) {
        self.emit(VINSERTF32X8_512RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf32x8Emitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinsertf32x8(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTF32X8_512RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VINSERTF32X8_MASK` (VINSERTF32X8). 
/// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Mem, Imm |
/// | 2 | Zmm, Zmm, Ymm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinsertf32x8MaskEmitter<A, B, C, D> {
    fn vinsertf32x8_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinsertf32x8MaskEmitter<Zmm, Zmm, Ymm, Imm> for Assembler<'a> {
    fn vinsertf32x8_mask(&mut self, op0: Zmm, op1: Zmm, op2: Ymm, op3: Imm) {
        self.emit(VINSERTF32X8_512RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf32x8MaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinsertf32x8_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTF32X8_512RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VINSERTF32X8_MASKZ` (VINSERTF32X8). 
/// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Mem, Imm |
/// | 2 | Zmm, Zmm, Ymm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinsertf32x8MaskzEmitter<A, B, C, D> {
    fn vinsertf32x8_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinsertf32x8MaskzEmitter<Zmm, Zmm, Ymm, Imm> for Assembler<'a> {
    fn vinsertf32x8_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Ymm, op3: Imm) {
        self.emit(VINSERTF32X8_512RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf32x8MaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinsertf32x8_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTF32X8_512RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VINSERTF64X2` (VINSERTF64X2). 
/// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Ymm, Ymm, Mem, Imm |
/// | 2 | Ymm, Ymm, Xmm, Imm |
/// | 3 | Zmm, Zmm, Mem, Imm |
/// | 4 | Zmm, Zmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinsertf64x2Emitter<A, B, C, D> {
    fn vinsertf64x2(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinsertf64x2Emitter<Ymm, Ymm, Xmm, Imm> for Assembler<'a> {
    fn vinsertf64x2(&mut self, op0: Ymm, op1: Ymm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTF64X2_256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf64x2Emitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vinsertf64x2(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VINSERTF64X2_256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf64x2Emitter<Zmm, Zmm, Xmm, Imm> for Assembler<'a> {
    fn vinsertf64x2(&mut self, op0: Zmm, op1: Zmm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTF64X2_512RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf64x2Emitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinsertf64x2(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTF64X2_512RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VINSERTF64X2_MASK` (VINSERTF64X2). 
/// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Ymm, Ymm, Mem, Imm |
/// | 2 | Ymm, Ymm, Xmm, Imm |
/// | 3 | Zmm, Zmm, Mem, Imm |
/// | 4 | Zmm, Zmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinsertf64x2MaskEmitter<A, B, C, D> {
    fn vinsertf64x2_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinsertf64x2MaskEmitter<Ymm, Ymm, Xmm, Imm> for Assembler<'a> {
    fn vinsertf64x2_mask(&mut self, op0: Ymm, op1: Ymm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTF64X2_256RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf64x2MaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vinsertf64x2_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VINSERTF64X2_256RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf64x2MaskEmitter<Zmm, Zmm, Xmm, Imm> for Assembler<'a> {
    fn vinsertf64x2_mask(&mut self, op0: Zmm, op1: Zmm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTF64X2_512RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf64x2MaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinsertf64x2_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTF64X2_512RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VINSERTF64X2_MASKZ` (VINSERTF64X2). 
/// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Ymm, Ymm, Mem, Imm |
/// | 2 | Ymm, Ymm, Xmm, Imm |
/// | 3 | Zmm, Zmm, Mem, Imm |
/// | 4 | Zmm, Zmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinsertf64x2MaskzEmitter<A, B, C, D> {
    fn vinsertf64x2_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinsertf64x2MaskzEmitter<Ymm, Ymm, Xmm, Imm> for Assembler<'a> {
    fn vinsertf64x2_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTF64X2_256RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf64x2MaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vinsertf64x2_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VINSERTF64X2_256RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf64x2MaskzEmitter<Zmm, Zmm, Xmm, Imm> for Assembler<'a> {
    fn vinsertf64x2_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTF64X2_512RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf64x2MaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinsertf64x2_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTF64X2_512RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VINSERTI32X8` (VINSERTI32X8). 
/// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Mem, Imm |
/// | 2 | Zmm, Zmm, Ymm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinserti32x8Emitter<A, B, C, D> {
    fn vinserti32x8(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinserti32x8Emitter<Zmm, Zmm, Ymm, Imm> for Assembler<'a> {
    fn vinserti32x8(&mut self, op0: Zmm, op1: Zmm, op2: Ymm, op3: Imm) {
        self.emit(VINSERTI32X8_512RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti32x8Emitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinserti32x8(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTI32X8_512RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VINSERTI32X8_MASK` (VINSERTI32X8). 
/// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Mem, Imm |
/// | 2 | Zmm, Zmm, Ymm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinserti32x8MaskEmitter<A, B, C, D> {
    fn vinserti32x8_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinserti32x8MaskEmitter<Zmm, Zmm, Ymm, Imm> for Assembler<'a> {
    fn vinserti32x8_mask(&mut self, op0: Zmm, op1: Zmm, op2: Ymm, op3: Imm) {
        self.emit(VINSERTI32X8_512RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti32x8MaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinserti32x8_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTI32X8_512RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VINSERTI32X8_MASKZ` (VINSERTI32X8). 
/// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Mem, Imm |
/// | 2 | Zmm, Zmm, Ymm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinserti32x8MaskzEmitter<A, B, C, D> {
    fn vinserti32x8_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinserti32x8MaskzEmitter<Zmm, Zmm, Ymm, Imm> for Assembler<'a> {
    fn vinserti32x8_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Ymm, op3: Imm) {
        self.emit(VINSERTI32X8_512RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti32x8MaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinserti32x8_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTI32X8_512RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VINSERTI64X2` (VINSERTI64X2). 
/// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Ymm, Ymm, Mem, Imm |
/// | 2 | Ymm, Ymm, Xmm, Imm |
/// | 3 | Zmm, Zmm, Mem, Imm |
/// | 4 | Zmm, Zmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinserti64x2Emitter<A, B, C, D> {
    fn vinserti64x2(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinserti64x2Emitter<Ymm, Ymm, Xmm, Imm> for Assembler<'a> {
    fn vinserti64x2(&mut self, op0: Ymm, op1: Ymm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTI64X2_256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti64x2Emitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vinserti64x2(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VINSERTI64X2_256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti64x2Emitter<Zmm, Zmm, Xmm, Imm> for Assembler<'a> {
    fn vinserti64x2(&mut self, op0: Zmm, op1: Zmm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTI64X2_512RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti64x2Emitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinserti64x2(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTI64X2_512RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VINSERTI64X2_MASK` (VINSERTI64X2). 
/// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Ymm, Ymm, Mem, Imm |
/// | 2 | Ymm, Ymm, Xmm, Imm |
/// | 3 | Zmm, Zmm, Mem, Imm |
/// | 4 | Zmm, Zmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinserti64x2MaskEmitter<A, B, C, D> {
    fn vinserti64x2_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinserti64x2MaskEmitter<Ymm, Ymm, Xmm, Imm> for Assembler<'a> {
    fn vinserti64x2_mask(&mut self, op0: Ymm, op1: Ymm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTI64X2_256RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti64x2MaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vinserti64x2_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VINSERTI64X2_256RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti64x2MaskEmitter<Zmm, Zmm, Xmm, Imm> for Assembler<'a> {
    fn vinserti64x2_mask(&mut self, op0: Zmm, op1: Zmm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTI64X2_512RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti64x2MaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinserti64x2_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTI64X2_512RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VINSERTI64X2_MASKZ` (VINSERTI64X2). 
/// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Ymm, Ymm, Mem, Imm |
/// | 2 | Ymm, Ymm, Xmm, Imm |
/// | 3 | Zmm, Zmm, Mem, Imm |
/// | 4 | Zmm, Zmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait Vinserti64x2MaskzEmitter<A, B, C, D> {
    fn vinserti64x2_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinserti64x2MaskzEmitter<Ymm, Ymm, Xmm, Imm> for Assembler<'a> {
    fn vinserti64x2_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTI64X2_256RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti64x2MaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vinserti64x2_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VINSERTI64X2_256RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti64x2MaskzEmitter<Zmm, Zmm, Xmm, Imm> for Assembler<'a> {
    fn vinserti64x2_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTI64X2_512RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinserti64x2MaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vinserti64x2_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VINSERTI64X2_512RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VORPD` (VORPD). 
/// Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VorpdEmitter<A, B, C> {
    fn vorpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VorpdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vorpd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VORPD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vorpd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VORPD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vorpd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VORPD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vorpd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VORPD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vorpd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VORPD512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vorpd(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VORPD512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VORPD_MASK` (VORPD). 
/// Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VorpdMaskEmitter<A, B, C> {
    fn vorpd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VorpdMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vorpd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VORPD128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vorpd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VORPD128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vorpd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VORPD256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vorpd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VORPD256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vorpd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VORPD512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vorpd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VORPD512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VORPD_MASKZ` (VORPD). 
/// Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VorpdMaskzEmitter<A, B, C> {
    fn vorpd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VorpdMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vorpd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VORPD128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vorpd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VORPD128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vorpd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VORPD256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vorpd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VORPD256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vorpd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VORPD512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpdMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vorpd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VORPD512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VORPS` (VORPS). 
/// Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VorpsEmitter<A, B, C> {
    fn vorps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VorpsEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vorps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VORPS128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vorps(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VORPS128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vorps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VORPS256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vorps(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VORPS256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vorps(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VORPS512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vorps(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VORPS512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VORPS_MASK` (VORPS). 
/// Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VorpsMaskEmitter<A, B, C> {
    fn vorps_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VorpsMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vorps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VORPS128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vorps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VORPS128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vorps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VORPS256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vorps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VORPS256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vorps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VORPS512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vorps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VORPS512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VORPS_MASKZ` (VORPS). 
/// Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VorpsMaskzEmitter<A, B, C> {
    fn vorps_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VorpsMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vorps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VORPS128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vorps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VORPS128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vorps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VORPS256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vorps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VORPS256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vorps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VORPS512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VorpsMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vorps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VORPS512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPMOVD2M` (VPMOVD2M). 
/// Converts a vector register to a mask register. Each element in the destination register is set to 1 or 0 depending on the value of most significant bit of the corresponding element in the source register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMOVB2M%3AVPMOVW2M%3AVPMOVD2M%3AVPMOVQ2M.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------+
/// | # | Operands  |
/// +---+-----------+
/// | 1 | KReg, Xmm |
/// | 2 | KReg, Ymm |
/// | 3 | KReg, Zmm |
/// +---+-----------+
/// ```
pub trait Vpmovd2mEmitter<A, B> {
    fn vpmovd2m(&mut self, op0: A, op1: B);
}

impl<'a> Vpmovd2mEmitter<KReg, Xmm> for Assembler<'a> {
    fn vpmovd2m(&mut self, op0: KReg, op1: Xmm) {
        self.emit(VPMOVD2M128KR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vpmovd2mEmitter<KReg, Ymm> for Assembler<'a> {
    fn vpmovd2m(&mut self, op0: KReg, op1: Ymm) {
        self.emit(VPMOVD2M256KR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vpmovd2mEmitter<KReg, Zmm> for Assembler<'a> {
    fn vpmovd2m(&mut self, op0: KReg, op1: Zmm) {
        self.emit(VPMOVD2M512KR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPMOVM2D` (VPMOVM2D). 
/// Converts a mask register to a vector register. Each element in the destination register is set to all 1’s or all 0’s depending on the value of the corresponding bit in the source mask register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMOVM2B%3AVPMOVM2W%3AVPMOVM2D%3AVPMOVM2Q.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------+
/// | # | Operands  |
/// +---+-----------+
/// | 1 | Xmm, KReg |
/// | 2 | Ymm, KReg |
/// | 3 | Zmm, KReg |
/// +---+-----------+
/// ```
pub trait Vpmovm2dEmitter<A, B> {
    fn vpmovm2d(&mut self, op0: A, op1: B);
}

impl<'a> Vpmovm2dEmitter<Xmm, KReg> for Assembler<'a> {
    fn vpmovm2d(&mut self, op0: Xmm, op1: KReg) {
        self.emit(VPMOVM2D128RK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vpmovm2dEmitter<Ymm, KReg> for Assembler<'a> {
    fn vpmovm2d(&mut self, op0: Ymm, op1: KReg) {
        self.emit(VPMOVM2D256RK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vpmovm2dEmitter<Zmm, KReg> for Assembler<'a> {
    fn vpmovm2d(&mut self, op0: Zmm, op1: KReg) {
        self.emit(VPMOVM2D512RK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPMOVM2Q` (VPMOVM2Q). 
/// Converts a mask register to a vector register. Each element in the destination register is set to all 1’s or all 0’s depending on the value of the corresponding bit in the source mask register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMOVM2B%3AVPMOVM2W%3AVPMOVM2D%3AVPMOVM2Q.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------+
/// | # | Operands  |
/// +---+-----------+
/// | 1 | Xmm, KReg |
/// | 2 | Ymm, KReg |
/// | 3 | Zmm, KReg |
/// +---+-----------+
/// ```
pub trait Vpmovm2qEmitter<A, B> {
    fn vpmovm2q(&mut self, op0: A, op1: B);
}

impl<'a> Vpmovm2qEmitter<Xmm, KReg> for Assembler<'a> {
    fn vpmovm2q(&mut self, op0: Xmm, op1: KReg) {
        self.emit(VPMOVM2Q128RK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vpmovm2qEmitter<Ymm, KReg> for Assembler<'a> {
    fn vpmovm2q(&mut self, op0: Ymm, op1: KReg) {
        self.emit(VPMOVM2Q256RK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vpmovm2qEmitter<Zmm, KReg> for Assembler<'a> {
    fn vpmovm2q(&mut self, op0: Zmm, op1: KReg) {
        self.emit(VPMOVM2Q512RK, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPMOVQ2M` (VPMOVQ2M). 
/// Converts a vector register to a mask register. Each element in the destination register is set to 1 or 0 depending on the value of most significant bit of the corresponding element in the source register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMOVB2M%3AVPMOVW2M%3AVPMOVD2M%3AVPMOVQ2M.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+-----------+
/// | # | Operands  |
/// +---+-----------+
/// | 1 | KReg, Xmm |
/// | 2 | KReg, Ymm |
/// | 3 | KReg, Zmm |
/// +---+-----------+
/// ```
pub trait Vpmovq2mEmitter<A, B> {
    fn vpmovq2m(&mut self, op0: A, op1: B);
}

impl<'a> Vpmovq2mEmitter<KReg, Xmm> for Assembler<'a> {
    fn vpmovq2m(&mut self, op0: KReg, op1: Xmm) {
        self.emit(VPMOVQ2M128KR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vpmovq2mEmitter<KReg, Ymm> for Assembler<'a> {
    fn vpmovq2m(&mut self, op0: KReg, op1: Ymm) {
        self.emit(VPMOVQ2M256KR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vpmovq2mEmitter<KReg, Zmm> for Assembler<'a> {
    fn vpmovq2m(&mut self, op0: KReg, op1: Zmm) {
        self.emit(VPMOVQ2M512KR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPMULLD` (VPMULLD). 
/// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpmulldEmitter<A, B, C> {
    fn vpmulld(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulldEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulld(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPMULLD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulld(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPMULLD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulld(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPMULLD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulld(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPMULLD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulld(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPMULLD512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulld(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPMULLD512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPMULLD_MASK` (VPMULLD). 
/// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpmulldMaskEmitter<A, B, C> {
    fn vpmulld_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulldMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulld_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPMULLD128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulld_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPMULLD128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulld_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPMULLD256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulld_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPMULLD256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulld_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPMULLD512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulld_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPMULLD512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPMULLD_MASKZ` (VPMULLD). 
/// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpmulldMaskzEmitter<A, B, C> {
    fn vpmulld_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmulldMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmulld_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPMULLD128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmulld_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPMULLD128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmulld_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPMULLD256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmulld_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPMULLD256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmulld_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPMULLD512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmulldMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmulld_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPMULLD512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPMULLQ` (VPMULLQ). 
/// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpmullqEmitter<A, B, C> {
    fn vpmullq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmullqEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmullq(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPMULLQ128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmullq(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPMULLQ128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmullq(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPMULLQ256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmullq(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPMULLQ256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmullq(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPMULLQ512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmullq(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPMULLQ512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPMULLQ_MASK` (VPMULLQ). 
/// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpmullqMaskEmitter<A, B, C> {
    fn vpmullq_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmullqMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmullq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPMULLQ128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmullq_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPMULLQ128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmullq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPMULLQ256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmullq_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPMULLQ256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmullq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPMULLQ512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmullq_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPMULLQ512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPMULLQ_MASKZ` (VPMULLQ). 
/// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VpmullqMaskzEmitter<A, B, C> {
    fn vpmullq_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpmullqMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpmullq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPMULLQ128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpmullq_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPMULLQ128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpmullq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPMULLQ256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpmullq_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPMULLQ256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vpmullq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VPMULLQ512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpmullqMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vpmullq_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VPMULLQ512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VRANGEPD` (VRANGEPD). 
/// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// | 3 | Ymm, Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Ymm, Imm |
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepdEmitter<A, B, C, D> {
    fn vrangepd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepdEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangepd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGEPD128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangepd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPD128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vrangepd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VRANGEPD256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vrangepd(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPD256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangepd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPD512RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vrangepd(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPD512RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGEPD_MASK` (VRANGEPD). 
/// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// | 3 | Ymm, Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Ymm, Imm |
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepdMaskEmitter<A, B, C, D> {
    fn vrangepd_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepdMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangepd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGEPD128RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangepd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPD128RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdMaskEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vrangepd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VRANGEPD256RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdMaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vrangepd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPD256RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdMaskEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangepd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPD512RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdMaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vrangepd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPD512RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGEPD_MASK_SAE` (VRANGEPD). 
/// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepdMaskSaeEmitter<A, B, C, D> {
    fn vrangepd_mask_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepdMaskSaeEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangepd_mask_sae(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPD512RRRI_MASK_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGEPD_MASKZ` (VRANGEPD). 
/// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// | 3 | Ymm, Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Ymm, Imm |
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepdMaskzEmitter<A, B, C, D> {
    fn vrangepd_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepdMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangepd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGEPD128RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangepd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPD128RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdMaskzEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vrangepd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VRANGEPD256RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdMaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vrangepd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPD256RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdMaskzEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangepd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPD512RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepdMaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vrangepd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPD512RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGEPD_MASKZ_SAE` (VRANGEPD). 
/// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepdMaskzSaeEmitter<A, B, C, D> {
    fn vrangepd_maskz_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepdMaskzSaeEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangepd_maskz_sae(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPD512RRRI_MASKZ_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGEPD_SAE` (VRANGEPD). 
/// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepdSaeEmitter<A, B, C, D> {
    fn vrangepd_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepdSaeEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangepd_sae(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPD512RRRI_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGEPS` (VRANGEPS). 
/// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// | 3 | Ymm, Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Ymm, Imm |
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepsEmitter<A, B, C, D> {
    fn vrangeps(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepsEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangeps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGEPS128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangeps(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPS128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vrangeps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VRANGEPS256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vrangeps(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPS256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangeps(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPS512RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vrangeps(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPS512RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGEPS_MASK` (VRANGEPS). 
/// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// | 3 | Ymm, Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Ymm, Imm |
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepsMaskEmitter<A, B, C, D> {
    fn vrangeps_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepsMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangeps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGEPS128RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangeps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPS128RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsMaskEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vrangeps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VRANGEPS256RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsMaskEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vrangeps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPS256RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsMaskEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangeps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPS512RRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsMaskEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vrangeps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPS512RRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGEPS_MASK_SAE` (VRANGEPS). 
/// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepsMaskSaeEmitter<A, B, C, D> {
    fn vrangeps_mask_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepsMaskSaeEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangeps_mask_sae(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPS512RRRI_MASK_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGEPS_MASKZ` (VRANGEPS). 
/// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// | 3 | Ymm, Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Ymm, Imm |
/// | 5 | Zmm, Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepsMaskzEmitter<A, B, C, D> {
    fn vrangeps_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepsMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangeps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGEPS128RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangeps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPS128RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsMaskzEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vrangeps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VRANGEPS256RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsMaskzEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vrangeps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPS256RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsMaskzEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangeps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPS512RRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangepsMaskzEmitter<Zmm, Zmm, Mem, Imm> for Assembler<'a> {
    fn vrangeps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VRANGEPS512RRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGEPS_MASKZ_SAE` (VRANGEPS). 
/// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepsMaskzSaeEmitter<A, B, C, D> {
    fn vrangeps_maskz_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepsMaskzSaeEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangeps_maskz_sae(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPS512RRRI_MASKZ_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGEPS_SAE` (VRANGEPS). 
/// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Zmm, Zmm, Zmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangepsSaeEmitter<A, B, C, D> {
    fn vrangeps_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangepsSaeEmitter<Zmm, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vrangeps_sae(&mut self, op0: Zmm, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VRANGEPS512RRRI_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESD` (VRANGESD). 
/// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangesdEmitter<A, B, C, D> {
    fn vrangesd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangesdEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangesd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESDRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangesdEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangesd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGESDRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESD_MASK` (VRANGESD). 
/// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangesdMaskEmitter<A, B, C, D> {
    fn vrangesd_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangesdMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangesd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESDRRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangesdMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangesd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGESDRRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESD_MASK_SAE` (VRANGESD). 
/// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangesdMaskSaeEmitter<A, B, C, D> {
    fn vrangesd_mask_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangesdMaskSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangesd_mask_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESDRRRI_MASK_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESD_MASKZ` (VRANGESD). 
/// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangesdMaskzEmitter<A, B, C, D> {
    fn vrangesd_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangesdMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangesd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESDRRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangesdMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangesd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGESDRRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESD_MASKZ_SAE` (VRANGESD). 
/// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangesdMaskzSaeEmitter<A, B, C, D> {
    fn vrangesd_maskz_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangesdMaskzSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangesd_maskz_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESDRRRI_MASKZ_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESD_SAE` (VRANGESD). 
/// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangesdSaeEmitter<A, B, C, D> {
    fn vrangesd_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangesdSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangesd_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESDRRRI_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESS` (VRANGESS). 
/// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangessEmitter<A, B, C, D> {
    fn vrangess(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangessEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangess(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESSRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangessEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangess(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGESSRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESS_MASK` (VRANGESS). 
/// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangessMaskEmitter<A, B, C, D> {
    fn vrangess_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangessMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangess_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESSRRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangessMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangess_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGESSRRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESS_MASK_SAE` (VRANGESS). 
/// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangessMaskSaeEmitter<A, B, C, D> {
    fn vrangess_mask_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangessMaskSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangess_mask_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESSRRRI_MASK_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESS_MASKZ` (VRANGESS). 
/// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangessMaskzEmitter<A, B, C, D> {
    fn vrangess_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangessMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangess_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESSRRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VrangessMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vrangess_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VRANGESSRRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESS_MASKZ_SAE` (VRANGESS). 
/// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangessMaskzSaeEmitter<A, B, C, D> {
    fn vrangess_maskz_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangessMaskzSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangess_maskz_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESSRRRI_MASKZ_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRANGESS_SAE` (VRANGESS). 
/// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VrangessSaeEmitter<A, B, C, D> {
    fn vrangess_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VrangessSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vrangess_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VRANGESSRRRI_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCEPD` (VREDUCEPD). 
/// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepdEmitter<A, B, C> {
    fn vreducepd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepdEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducepd(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VREDUCEPD128RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vreducepd(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPD128RMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vreducepd(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(VREDUCEPD256RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vreducepd(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPD256RMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreducepd(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPD512RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vreducepd(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPD512RMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCEPD_MASK` (VREDUCEPD). 
/// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepdMaskEmitter<A, B, C> {
    fn vreducepd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepdMaskEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducepd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VREDUCEPD128RRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdMaskEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vreducepd_mask(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPD128RMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdMaskEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vreducepd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(VREDUCEPD256RRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdMaskEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vreducepd_mask(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPD256RMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdMaskEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreducepd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPD512RRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdMaskEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vreducepd_mask(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPD512RMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCEPD_MASK_SAE` (VREDUCEPD). 
/// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepdMaskSaeEmitter<A, B, C> {
    fn vreducepd_mask_sae(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepdMaskSaeEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreducepd_mask_sae(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPD512RRI_MASK_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCEPD_MASKZ` (VREDUCEPD). 
/// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepdMaskzEmitter<A, B, C> {
    fn vreducepd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepdMaskzEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducepd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VREDUCEPD128RRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdMaskzEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vreducepd_maskz(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPD128RMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdMaskzEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vreducepd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(VREDUCEPD256RRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdMaskzEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vreducepd_maskz(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPD256RMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdMaskzEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreducepd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPD512RRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepdMaskzEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vreducepd_maskz(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPD512RMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCEPD_MASKZ_SAE` (VREDUCEPD). 
/// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepdMaskzSaeEmitter<A, B, C> {
    fn vreducepd_maskz_sae(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepdMaskzSaeEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreducepd_maskz_sae(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPD512RRI_MASKZ_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCEPD_SAE` (VREDUCEPD). 
/// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepdSaeEmitter<A, B, C> {
    fn vreducepd_sae(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepdSaeEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreducepd_sae(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPD512RRI_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCEPS` (VREDUCEPS). 
/// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepsEmitter<A, B, C> {
    fn vreduceps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepsEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreduceps(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VREDUCEPS128RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vreduceps(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPS128RMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vreduceps(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(VREDUCEPS256RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vreduceps(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPS256RMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreduceps(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPS512RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vreduceps(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPS512RMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCEPS_MASK` (VREDUCEPS). 
/// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepsMaskEmitter<A, B, C> {
    fn vreduceps_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepsMaskEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreduceps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VREDUCEPS128RRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsMaskEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vreduceps_mask(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPS128RMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsMaskEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vreduceps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(VREDUCEPS256RRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsMaskEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vreduceps_mask(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPS256RMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsMaskEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreduceps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPS512RRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsMaskEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vreduceps_mask(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPS512RMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCEPS_MASK_SAE` (VREDUCEPS). 
/// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepsMaskSaeEmitter<A, B, C> {
    fn vreduceps_mask_sae(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepsMaskSaeEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreduceps_mask_sae(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPS512RRI_MASK_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCEPS_MASKZ` (VREDUCEPS). 
/// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// | 3 | Ymm, Mem, Imm |
/// | 4 | Ymm, Ymm, Imm |
/// | 5 | Zmm, Mem, Imm |
/// | 6 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepsMaskzEmitter<A, B, C> {
    fn vreduceps_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepsMaskzEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreduceps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VREDUCEPS128RRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsMaskzEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vreduceps_maskz(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPS128RMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsMaskzEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vreduceps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(VREDUCEPS256RRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsMaskzEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vreduceps_maskz(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPS256RMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsMaskzEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreduceps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPS512RRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VreducepsMaskzEmitter<Zmm, Mem, Imm> for Assembler<'a> {
    fn vreduceps_maskz(&mut self, op0: Zmm, op1: Mem, op2: Imm) {
        self.emit(VREDUCEPS512RMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCEPS_MASKZ_SAE` (VREDUCEPS). 
/// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepsMaskzSaeEmitter<A, B, C> {
    fn vreduceps_maskz_sae(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepsMaskzSaeEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreduceps_maskz_sae(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPS512RRI_MASKZ_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCEPS_SAE` (VREDUCEPS). 
/// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Zmm, Zmm, Imm |
/// +---+---------------+
/// ```
pub trait VreducepsSaeEmitter<A, B, C> {
    fn vreduceps_sae(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VreducepsSaeEmitter<Zmm, Zmm, Imm> for Assembler<'a> {
    fn vreduceps_sae(&mut self, op0: Zmm, op1: Zmm, op2: Imm) {
        self.emit(VREDUCEPS512RRI_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VREDUCESD` (VREDUCESD). 
/// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducesdEmitter<A, B, C, D> {
    fn vreducesd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducesdEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducesd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESDRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VreducesdEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vreducesd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VREDUCESDRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCESD_MASK` (VREDUCESD). 
/// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducesdMaskEmitter<A, B, C, D> {
    fn vreducesd_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducesdMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducesd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESDRRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VreducesdMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vreducesd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VREDUCESDRRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCESD_MASK_SAE` (VREDUCESD). 
/// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducesdMaskSaeEmitter<A, B, C, D> {
    fn vreducesd_mask_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducesdMaskSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducesd_mask_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESDRRRI_MASK_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCESD_MASKZ` (VREDUCESD). 
/// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducesdMaskzEmitter<A, B, C, D> {
    fn vreducesd_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducesdMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducesd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESDRRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VreducesdMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vreducesd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VREDUCESDRRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCESD_MASKZ_SAE` (VREDUCESD). 
/// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducesdMaskzSaeEmitter<A, B, C, D> {
    fn vreducesd_maskz_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducesdMaskzSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducesd_maskz_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESDRRRI_MASKZ_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCESD_SAE` (VREDUCESD). 
/// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducesdSaeEmitter<A, B, C, D> {
    fn vreducesd_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducesdSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducesd_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESDRRRI_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCESS` (VREDUCESS). 
/// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducessEmitter<A, B, C, D> {
    fn vreducess(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducessEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducess(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESSRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VreducessEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vreducess(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VREDUCESSRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCESS_MASK` (VREDUCESS). 
/// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducessMaskEmitter<A, B, C, D> {
    fn vreducess_mask(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducessMaskEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducess_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESSRRRI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VreducessMaskEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vreducess_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VREDUCESSRRMI_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCESS_MASK_SAE` (VREDUCESS). 
/// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducessMaskSaeEmitter<A, B, C, D> {
    fn vreducess_mask_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducessMaskSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducess_mask_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESSRRRI_MASK_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCESS_MASKZ` (VREDUCESS). 
/// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducessMaskzEmitter<A, B, C, D> {
    fn vreducess_maskz(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducessMaskzEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducess_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESSRRRI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VreducessMaskzEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vreducess_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VREDUCESSRRMI_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCESS_MASKZ_SAE` (VREDUCESS). 
/// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducessMaskzSaeEmitter<A, B, C, D> {
    fn vreducess_maskz_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducessMaskzSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducess_maskz_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESSRRRI_MASKZ_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VREDUCESS_SAE` (VREDUCESS). 
/// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Xmm, Imm |
/// +---+--------------------+
/// ```
pub trait VreducessSaeEmitter<A, B, C, D> {
    fn vreducess_sae(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VreducessSaeEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vreducess_sae(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VREDUCESSRRRI_SAE, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VXORPD` (VXORPD). 
/// Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VxorpdEmitter<A, B, C> {
    fn vxorpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VxorpdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vxorpd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VXORPD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vxorpd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VXORPD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vxorpd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VXORPD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vxorpd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VXORPD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vxorpd(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VXORPD512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vxorpd(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VXORPD512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VXORPD_MASK` (VXORPD). 
/// Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VxorpdMaskEmitter<A, B, C> {
    fn vxorpd_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VxorpdMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vxorpd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VXORPD128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vxorpd_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VXORPD128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vxorpd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VXORPD256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vxorpd_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VXORPD256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vxorpd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VXORPD512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vxorpd_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VXORPD512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VXORPD_MASKZ` (VXORPD). 
/// Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VxorpdMaskzEmitter<A, B, C> {
    fn vxorpd_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VxorpdMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vxorpd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VXORPD128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vxorpd_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VXORPD128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vxorpd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VXORPD256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vxorpd_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VXORPD256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vxorpd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VXORPD512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpdMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vxorpd_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VXORPD512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VXORPS` (VXORPS). 
/// Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VxorpsEmitter<A, B, C> {
    fn vxorps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VxorpsEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vxorps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VXORPS128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vxorps(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VXORPS128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vxorps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VXORPS256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vxorps(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VXORPS256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vxorps(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VXORPS512RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vxorps(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VXORPS512RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VXORPS_MASK` (VXORPS). 
/// Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VxorpsMaskEmitter<A, B, C> {
    fn vxorps_mask(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VxorpsMaskEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vxorps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VXORPS128RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsMaskEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vxorps_mask(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VXORPS128RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsMaskEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vxorps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VXORPS256RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsMaskEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vxorps_mask(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VXORPS256RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsMaskEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vxorps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VXORPS512RRR_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsMaskEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vxorps_mask(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VXORPS512RRM_MASK, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VXORPS_MASKZ` (VXORPS). 
/// Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem |
/// | 4 | Ymm, Ymm, Ymm |
/// | 5 | Zmm, Zmm, Mem |
/// | 6 | Zmm, Zmm, Zmm |
/// +---+---------------+
/// ```
pub trait VxorpsMaskzEmitter<A, B, C> {
    fn vxorps_maskz(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VxorpsMaskzEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vxorps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VXORPS128RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsMaskzEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vxorps_maskz(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VXORPS128RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsMaskzEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vxorps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VXORPS256RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsMaskzEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vxorps_maskz(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VXORPS256RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsMaskzEmitter<Zmm, Zmm, Zmm> for Assembler<'a> {
    fn vxorps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Zmm) {
        self.emit(VXORPS512RRR_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VxorpsMaskzEmitter<Zmm, Zmm, Mem> for Assembler<'a> {
    fn vxorps_maskz(&mut self, op0: Zmm, op1: Zmm, op2: Mem) {
        self.emit(VXORPS512RRM_MASKZ, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `KADDB` (KADDB). 
    /// Adds the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KADDW%3AKADDB%3AKADDQ%3AKADDD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kaddb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: KaddbEmitter<A, B, C> {
        <Self as KaddbEmitter<A, B, C>>::kaddb(self, op0, op1, op2);
    }
    /// `KADDW` (KADDW). 
    /// Adds the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KADDW%3AKADDB%3AKADDQ%3AKADDD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kaddw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: KaddwEmitter<A, B, C> {
        <Self as KaddwEmitter<A, B, C>>::kaddw(self, op0, op1, op2);
    }
    /// `KANDB` (KANDB). 
    /// Performs a bitwise AND between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KANDW%3AKANDB%3AKANDQ%3AKANDD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kandb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: KandbEmitter<A, B, C> {
        <Self as KandbEmitter<A, B, C>>::kandb(self, op0, op1, op2);
    }
    /// `KANDNB` (KANDNB). 
    /// Performs a bitwise AND NOT between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KANDNW%3AKANDNB%3AKANDNQ%3AKANDND.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kandnb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: KandnbEmitter<A, B, C> {
        <Self as KandnbEmitter<A, B, C>>::kandnb(self, op0, op1, op2);
    }
    /// `KMOVB` (KMOVB). 
    /// Copies values from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be mask registers, memory location or general purpose. The instruction cannot be used to transfer data between general purpose registers and or memory locations.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KMOVW%3AKMOVB%3AKMOVQ%3AKMOVD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | Gpd, KReg  |
    /// | 2 | KReg, Gpd  |
    /// | 3 | KReg, KReg |
    /// | 4 | KReg, Mem  |
    /// | 5 | Mem, KReg  |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn kmovb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: KmovbEmitter<A, B> {
        <Self as KmovbEmitter<A, B>>::kmovb(self, op0, op1);
    }
    /// `KNOTB` (KNOTB). 
    /// Performs a bitwise NOT of vector mask k2 and writes the result into vector mask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KNOTW%3AKNOTB%3AKNOTQ%3AKNOTD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | KReg, KReg |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn knotb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: KnotbEmitter<A, B> {
        <Self as KnotbEmitter<A, B>>::knotb(self, op0, op1);
    }
    /// `KORB` (KORB). 
    /// Performs a bitwise OR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KORW%3AKORB%3AKORQ%3AKORD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn korb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: KorbEmitter<A, B, C> {
        <Self as KorbEmitter<A, B, C>>::korb(self, op0, op1, op2);
    }
    /// `KORTESTB` (KORTESTB). 
    /// Performs a bitwise OR between the vector mask register k2, and the vector mask register k1, and sets CF and ZF based on the operation result.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KORTESTW%3AKORTESTB%3AKORTESTQ%3AKORTESTD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | KReg, KReg |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn kortestb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: KortestbEmitter<A, B> {
        <Self as KortestbEmitter<A, B>>::kortestb(self, op0, op1);
    }
    /// `KSHIFTLB` (KSHIFTLB). 
    /// Shifts 8/16/32/64 bits in the second operand (source operand) left by the count specified in immediate byte and place the least significant 8/16/32/64 bits of the result in the destination operand. The higher bits of the destination are zero-extended. The destination is set to zero if the count value is greater than 7 (for byte shift), 15 (for word shift), 31 (for doubleword shift) or 63 (for quadword shift).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KSHIFTLW%3AKSHIFTLB%3AKSHIFTLQ%3AKSHIFTLD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------------+
    /// | # | Operands        |
    /// +---+-----------------+
    /// | 1 | KReg, KReg, Imm |
    /// +---+-----------------+
    /// ```
    #[inline]
    pub fn kshiftlb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: KshiftlbEmitter<A, B, C> {
        <Self as KshiftlbEmitter<A, B, C>>::kshiftlb(self, op0, op1, op2);
    }
    /// `KSHIFTRB` (KSHIFTRB). 
    /// Shifts 8/16/32/64 bits in the second operand (source operand) right by the count specified in immediate and place the least significant 8/16/32/64 bits of the result in the destination operand. The higher bits of the destination are zero-extended. The destination is set to zero if the count value is greater than 7 (for byte shift), 15 (for word shift), 31 (for doubleword shift) or 63 (for quadword shift).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KSHIFTRW%3AKSHIFTRB%3AKSHIFTRQ%3AKSHIFTRD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------------+
    /// | # | Operands        |
    /// +---+-----------------+
    /// | 1 | KReg, KReg, Imm |
    /// +---+-----------------+
    /// ```
    #[inline]
    pub fn kshiftrb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: KshiftrbEmitter<A, B, C> {
        <Self as KshiftrbEmitter<A, B, C>>::kshiftrb(self, op0, op1, op2);
    }
    /// `KTESTB` (KTESTB). 
    /// Performs a bitwise comparison of the bits of the first source operand and corresponding bits in the second source operand. If the AND operation produces all zeros, the ZF is set else the ZF is clear. If the bitwise AND operation of the inverted first source operand with the second source operand produces all zeros the CF is set else the CF is clear. Only the EFLAGS register is updated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KTESTW%3AKTESTB%3AKTESTQ%3AKTESTD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | KReg, KReg |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn ktestb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: KtestbEmitter<A, B> {
        <Self as KtestbEmitter<A, B>>::ktestb(self, op0, op1);
    }
    /// `KTESTW` (KTESTW). 
    /// Performs a bitwise comparison of the bits of the first source operand and corresponding bits in the second source operand. If the AND operation produces all zeros, the ZF is set else the ZF is clear. If the bitwise AND operation of the inverted first source operand with the second source operand produces all zeros the CF is set else the CF is clear. Only the EFLAGS register is updated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KTESTW%3AKTESTB%3AKTESTQ%3AKTESTD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------+
    /// | # | Operands   |
    /// +---+------------+
    /// | 1 | KReg, KReg |
    /// +---+------------+
    /// ```
    #[inline]
    pub fn ktestw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: KtestwEmitter<A, B> {
        <Self as KtestwEmitter<A, B>>::ktestw(self, op0, op1);
    }
    /// `KXNORB` (KXNORB). 
    /// Performs a bitwise XNOR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KXNORW%3AKXNORB%3AKXNORQ%3AKXNORD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kxnorb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: KxnorbEmitter<A, B, C> {
        <Self as KxnorbEmitter<A, B, C>>::kxnorb(self, op0, op1, op2);
    }
    /// `KXORB` (KXORB). 
    /// Performs a bitwise XOR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/KXORW%3AKXORB%3AKXORQ%3AKXORD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+------------------+
    /// | # | Operands         |
    /// +---+------------------+
    /// | 1 | KReg, KReg, KReg |
    /// +---+------------------+
    /// ```
    #[inline]
    pub fn kxorb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: KxorbEmitter<A, B, C> {
        <Self as KxorbEmitter<A, B, C>>::kxorb(self, op0, op1, op2);
    }
    /// `VANDNPD` (VANDNPD). 
    /// Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandnpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandnpdEmitter<A, B, C> {
        <Self as VandnpdEmitter<A, B, C>>::vandnpd(self, op0, op1, op2);
    }
    /// `VANDNPD_MASK` (VANDNPD). 
    /// Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandnpd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandnpdMaskEmitter<A, B, C> {
        <Self as VandnpdMaskEmitter<A, B, C>>::vandnpd_mask(self, op0, op1, op2);
    }
    /// `VANDNPD_MASKZ` (VANDNPD). 
    /// Performs a bitwise logical AND NOT of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandnpd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandnpdMaskzEmitter<A, B, C> {
        <Self as VandnpdMaskzEmitter<A, B, C>>::vandnpd_maskz(self, op0, op1, op2);
    }
    /// `VANDNPS` (VANDNPS). 
    /// Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandnps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandnpsEmitter<A, B, C> {
        <Self as VandnpsEmitter<A, B, C>>::vandnps(self, op0, op1, op2);
    }
    /// `VANDNPS_MASK` (VANDNPS). 
    /// Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandnps_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandnpsMaskEmitter<A, B, C> {
        <Self as VandnpsMaskEmitter<A, B, C>>::vandnps_mask(self, op0, op1, op2);
    }
    /// `VANDNPS_MASKZ` (VANDNPS). 
    /// Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDNPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandnps_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandnpsMaskzEmitter<A, B, C> {
        <Self as VandnpsMaskzEmitter<A, B, C>>::vandnps_maskz(self, op0, op1, op2);
    }
    /// `VANDPD` (VANDPD). 
    /// Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandpdEmitter<A, B, C> {
        <Self as VandpdEmitter<A, B, C>>::vandpd(self, op0, op1, op2);
    }
    /// `VANDPD_MASK` (VANDPD). 
    /// Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandpd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandpdMaskEmitter<A, B, C> {
        <Self as VandpdMaskEmitter<A, B, C>>::vandpd_mask(self, op0, op1, op2);
    }
    /// `VANDPD_MASKZ` (VANDPD). 
    /// Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandpd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandpdMaskzEmitter<A, B, C> {
        <Self as VandpdMaskzEmitter<A, B, C>>::vandpd_maskz(self, op0, op1, op2);
    }
    /// `VANDPS` (VANDPS). 
    /// Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandpsEmitter<A, B, C> {
        <Self as VandpsEmitter<A, B, C>>::vandps(self, op0, op1, op2);
    }
    /// `VANDPS_MASK` (VANDPS). 
    /// Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandps_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandpsMaskEmitter<A, B, C> {
        <Self as VandpsMaskEmitter<A, B, C>>::vandps_mask(self, op0, op1, op2);
    }
    /// `VANDPS_MASKZ` (VANDPS). 
    /// Performs a bitwise logical AND of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ANDPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vandps_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VandpsMaskzEmitter<A, B, C> {
        <Self as VandpsMaskzEmitter<A, B, C>>::vandps_maskz(self, op0, op1, op2);
    }
    /// `VBROADCASTF32X2` (VBROADCASTF32X2). 
    /// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Ymm, Xmm |
    /// | 3 | Zmm, Mem |
    /// | 4 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcastf32x2<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcastf32x2Emitter<A, B> {
        <Self as Vbroadcastf32x2Emitter<A, B>>::vbroadcastf32x2(self, op0, op1);
    }
    /// `VBROADCASTF32X2_MASK` (VBROADCASTF32X2). 
    /// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Ymm, Xmm |
    /// | 3 | Zmm, Mem |
    /// | 4 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcastf32x2_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcastf32x2MaskEmitter<A, B> {
        <Self as Vbroadcastf32x2MaskEmitter<A, B>>::vbroadcastf32x2_mask(self, op0, op1);
    }
    /// `VBROADCASTF32X2_MASKZ` (VBROADCASTF32X2). 
    /// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Ymm, Xmm |
    /// | 3 | Zmm, Mem |
    /// | 4 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcastf32x2_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcastf32x2MaskzEmitter<A, B> {
        <Self as Vbroadcastf32x2MaskzEmitter<A, B>>::vbroadcastf32x2_maskz(self, op0, op1);
    }
    /// `VBROADCASTF32X8` (VBROADCASTF32X8). 
    /// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcastf32x8<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcastf32x8Emitter<A, B> {
        <Self as Vbroadcastf32x8Emitter<A, B>>::vbroadcastf32x8(self, op0, op1);
    }
    /// `VBROADCASTF32X8_MASK` (VBROADCASTF32X8). 
    /// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcastf32x8_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcastf32x8MaskEmitter<A, B> {
        <Self as Vbroadcastf32x8MaskEmitter<A, B>>::vbroadcastf32x8_mask(self, op0, op1);
    }
    /// `VBROADCASTF32X8_MASKZ` (VBROADCASTF32X8). 
    /// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcastf32x8_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcastf32x8MaskzEmitter<A, B> {
        <Self as Vbroadcastf32x8MaskzEmitter<A, B>>::vbroadcastf32x8_maskz(self, op0, op1);
    }
    /// `VBROADCASTF64X2` (VBROADCASTF64X2). 
    /// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcastf64x2<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcastf64x2Emitter<A, B> {
        <Self as Vbroadcastf64x2Emitter<A, B>>::vbroadcastf64x2(self, op0, op1);
    }
    /// `VBROADCASTF64X2_MASK` (VBROADCASTF64X2). 
    /// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcastf64x2_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcastf64x2MaskEmitter<A, B> {
        <Self as Vbroadcastf64x2MaskEmitter<A, B>>::vbroadcastf64x2_mask(self, op0, op1);
    }
    /// `VBROADCASTF64X2_MASKZ` (VBROADCASTF64X2). 
    /// VBROADCASTSD/VBROADCASTSS/VBROADCASTF128 load floating-point values as one tuple from the source operand (second operand) in memory and broadcast to all elements of the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcastf64x2_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcastf64x2MaskzEmitter<A, B> {
        <Self as Vbroadcastf64x2MaskzEmitter<A, B>>::vbroadcastf64x2_maskz(self, op0, op1);
    }
    /// `VBROADCASTI32X2`.
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
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti32x2<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti32x2Emitter<A, B> {
        <Self as Vbroadcasti32x2Emitter<A, B>>::vbroadcasti32x2(self, op0, op1);
    }
    /// `VBROADCASTI32X2_MASK`.
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
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti32x2_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti32x2MaskEmitter<A, B> {
        <Self as Vbroadcasti32x2MaskEmitter<A, B>>::vbroadcasti32x2_mask(self, op0, op1);
    }
    /// `VBROADCASTI32X2_MASKZ`.
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
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti32x2_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti32x2MaskzEmitter<A, B> {
        <Self as Vbroadcasti32x2MaskzEmitter<A, B>>::vbroadcasti32x2_maskz(self, op0, op1);
    }
    /// `VBROADCASTI32X4` (VBROADCASTI32X4). 
    /// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti32x4<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti32x4Emitter<A, B> {
        <Self as Vbroadcasti32x4Emitter<A, B>>::vbroadcasti32x4(self, op0, op1);
    }
    /// `VBROADCASTI32X4_MASK` (VBROADCASTI32X4). 
    /// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti32x4_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti32x4MaskEmitter<A, B> {
        <Self as Vbroadcasti32x4MaskEmitter<A, B>>::vbroadcasti32x4_mask(self, op0, op1);
    }
    /// `VBROADCASTI32X4_MASKZ` (VBROADCASTI32X4). 
    /// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti32x4_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti32x4MaskzEmitter<A, B> {
        <Self as Vbroadcasti32x4MaskzEmitter<A, B>>::vbroadcasti32x4_maskz(self, op0, op1);
    }
    /// `VBROADCASTI32X8` (VBROADCASTI32X8). 
    /// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti32x8<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti32x8Emitter<A, B> {
        <Self as Vbroadcasti32x8Emitter<A, B>>::vbroadcasti32x8(self, op0, op1);
    }
    /// `VBROADCASTI32X8_MASK` (VBROADCASTI32X8). 
    /// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti32x8_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti32x8MaskEmitter<A, B> {
        <Self as Vbroadcasti32x8MaskEmitter<A, B>>::vbroadcasti32x8_mask(self, op0, op1);
    }
    /// `VBROADCASTI32X8_MASKZ` (VBROADCASTI32X8). 
    /// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti32x8_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti32x8MaskzEmitter<A, B> {
        <Self as Vbroadcasti32x8MaskzEmitter<A, B>>::vbroadcasti32x8_maskz(self, op0, op1);
    }
    /// `VBROADCASTI64X2` (VBROADCASTI64X2). 
    /// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti64x2<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti64x2Emitter<A, B> {
        <Self as Vbroadcasti64x2Emitter<A, B>>::vbroadcasti64x2(self, op0, op1);
    }
    /// `VBROADCASTI64X2_MASK` (VBROADCASTI64X2). 
    /// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti64x2_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti64x2MaskEmitter<A, B> {
        <Self as Vbroadcasti64x2MaskEmitter<A, B>>::vbroadcasti64x2_mask(self, op0, op1);
    }
    /// `VBROADCASTI64X2_MASKZ` (VBROADCASTI64X2). 
    /// Load integer data from the source operand (the second operand) and broadcast to all elements of the destination operand (the first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPBROADCAST.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Mem |
    /// | 2 | Zmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcasti64x2_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcasti64x2MaskzEmitter<A, B> {
        <Self as Vbroadcasti64x2MaskzEmitter<A, B>>::vbroadcasti64x2_maskz(self, op0, op1);
    }
    /// `VCVTPD2QQ` (VCVTPD2QQ). 
    /// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
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
    pub fn vcvtpd2qq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtpd2qqEmitter<A, B> {
        <Self as Vcvtpd2qqEmitter<A, B>>::vcvtpd2qq(self, op0, op1);
    }
    /// `VCVTPD2QQ_ER` (VCVTPD2QQ). 
    /// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtpd2qq_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtpd2qqErEmitter<A, B> {
        <Self as Vcvtpd2qqErEmitter<A, B>>::vcvtpd2qq_er(self, op0, op1);
    }
    /// `VCVTPD2QQ_MASK` (VCVTPD2QQ). 
    /// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
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
    pub fn vcvtpd2qq_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtpd2qqMaskEmitter<A, B> {
        <Self as Vcvtpd2qqMaskEmitter<A, B>>::vcvtpd2qq_mask(self, op0, op1);
    }
    /// `VCVTPD2QQ_MASK_ER` (VCVTPD2QQ). 
    /// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtpd2qq_mask_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtpd2qqMaskErEmitter<A, B> {
        <Self as Vcvtpd2qqMaskErEmitter<A, B>>::vcvtpd2qq_mask_er(self, op0, op1);
    }
    /// `VCVTPD2QQ_MASKZ` (VCVTPD2QQ). 
    /// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
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
    pub fn vcvtpd2qq_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtpd2qqMaskzEmitter<A, B> {
        <Self as Vcvtpd2qqMaskzEmitter<A, B>>::vcvtpd2qq_maskz(self, op0, op1);
    }
    /// `VCVTPD2QQ_MASKZ_ER` (VCVTPD2QQ). 
    /// Converts packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPD2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtpd2qq_maskz_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtpd2qqMaskzErEmitter<A, B> {
        <Self as Vcvtpd2qqMaskzErEmitter<A, B>>::vcvtpd2qq_maskz_er(self, op0, op1);
    }
    /// `VCVTPS2QQ` (VCVTPS2QQ). 
    /// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
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
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtps2qq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtps2qqEmitter<A, B> {
        <Self as Vcvtps2qqEmitter<A, B>>::vcvtps2qq(self, op0, op1);
    }
    /// `VCVTPS2QQ_ER` (VCVTPS2QQ). 
    /// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtps2qq_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtps2qqErEmitter<A, B> {
        <Self as Vcvtps2qqErEmitter<A, B>>::vcvtps2qq_er(self, op0, op1);
    }
    /// `VCVTPS2QQ_MASK` (VCVTPS2QQ). 
    /// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
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
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtps2qq_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtps2qqMaskEmitter<A, B> {
        <Self as Vcvtps2qqMaskEmitter<A, B>>::vcvtps2qq_mask(self, op0, op1);
    }
    /// `VCVTPS2QQ_MASK_ER` (VCVTPS2QQ). 
    /// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtps2qq_mask_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtps2qqMaskErEmitter<A, B> {
        <Self as Vcvtps2qqMaskErEmitter<A, B>>::vcvtps2qq_mask_er(self, op0, op1);
    }
    /// `VCVTPS2QQ_MASKZ` (VCVTPS2QQ). 
    /// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
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
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtps2qq_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtps2qqMaskzEmitter<A, B> {
        <Self as Vcvtps2qqMaskzEmitter<A, B>>::vcvtps2qq_maskz(self, op0, op1);
    }
    /// `VCVTPS2QQ_MASKZ_ER` (VCVTPS2QQ). 
    /// Converts eight packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTPS2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtps2qq_maskz_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtps2qqMaskzErEmitter<A, B> {
        <Self as Vcvtps2qqMaskzErEmitter<A, B>>::vcvtps2qq_maskz_er(self, op0, op1);
    }
    /// `VCVTQQ2PD` (VCVTQQ2PD). 
    /// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
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
    pub fn vcvtqq2pd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2pdEmitter<A, B> {
        <Self as Vcvtqq2pdEmitter<A, B>>::vcvtqq2pd(self, op0, op1);
    }
    /// `VCVTQQ2PD_ER` (VCVTQQ2PD). 
    /// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtqq2pd_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2pdErEmitter<A, B> {
        <Self as Vcvtqq2pdErEmitter<A, B>>::vcvtqq2pd_er(self, op0, op1);
    }
    /// `VCVTQQ2PD_MASK` (VCVTQQ2PD). 
    /// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
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
    pub fn vcvtqq2pd_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2pdMaskEmitter<A, B> {
        <Self as Vcvtqq2pdMaskEmitter<A, B>>::vcvtqq2pd_mask(self, op0, op1);
    }
    /// `VCVTQQ2PD_MASK_ER` (VCVTQQ2PD). 
    /// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtqq2pd_mask_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2pdMaskErEmitter<A, B> {
        <Self as Vcvtqq2pdMaskErEmitter<A, B>>::vcvtqq2pd_mask_er(self, op0, op1);
    }
    /// `VCVTQQ2PD_MASKZ` (VCVTQQ2PD). 
    /// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
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
    pub fn vcvtqq2pd_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2pdMaskzEmitter<A, B> {
        <Self as Vcvtqq2pdMaskzEmitter<A, B>>::vcvtqq2pd_maskz(self, op0, op1);
    }
    /// `VCVTQQ2PD_MASKZ_ER` (VCVTQQ2PD). 
    /// Converts packed quadword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtqq2pd_maskz_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2pdMaskzErEmitter<A, B> {
        <Self as Vcvtqq2pdMaskzErEmitter<A, B>>::vcvtqq2pd_maskz_er(self, op0, op1);
    }
    /// `VCVTQQ2PS` (VCVTQQ2PS). 
    /// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Xmm, Ymm |
    /// | 4 | Ymm, Mem |
    /// | 5 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtqq2ps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2psEmitter<A, B> {
        <Self as Vcvtqq2psEmitter<A, B>>::vcvtqq2ps(self, op0, op1);
    }
    /// `VCVTQQ2PS_ER` (VCVTQQ2PS). 
    /// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtqq2ps_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2psErEmitter<A, B> {
        <Self as Vcvtqq2psErEmitter<A, B>>::vcvtqq2ps_er(self, op0, op1);
    }
    /// `VCVTQQ2PS_MASK` (VCVTQQ2PS). 
    /// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Xmm, Ymm |
    /// | 4 | Ymm, Mem |
    /// | 5 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtqq2ps_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2psMaskEmitter<A, B> {
        <Self as Vcvtqq2psMaskEmitter<A, B>>::vcvtqq2ps_mask(self, op0, op1);
    }
    /// `VCVTQQ2PS_MASK_ER` (VCVTQQ2PS). 
    /// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtqq2ps_mask_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2psMaskErEmitter<A, B> {
        <Self as Vcvtqq2psMaskErEmitter<A, B>>::vcvtqq2ps_mask_er(self, op0, op1);
    }
    /// `VCVTQQ2PS_MASKZ` (VCVTQQ2PS). 
    /// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Mem |
    /// | 2 | Xmm, Xmm |
    /// | 3 | Xmm, Ymm |
    /// | 4 | Ymm, Mem |
    /// | 5 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtqq2ps_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2psMaskzEmitter<A, B> {
        <Self as Vcvtqq2psMaskzEmitter<A, B>>::vcvtqq2ps_maskz(self, op0, op1);
    }
    /// `VCVTQQ2PS_MASKZ_ER` (VCVTQQ2PS). 
    /// Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTQQ2PS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Ymm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvtqq2ps_maskz_er<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvtqq2psMaskzErEmitter<A, B> {
        <Self as Vcvtqq2psMaskzErEmitter<A, B>>::vcvtqq2ps_maskz_er(self, op0, op1);
    }
    /// `VCVTTPD2QQ` (VCVTTPD2QQ). 
    /// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
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
    pub fn vcvttpd2qq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttpd2qqEmitter<A, B> {
        <Self as Vcvttpd2qqEmitter<A, B>>::vcvttpd2qq(self, op0, op1);
    }
    /// `VCVTTPD2QQ_MASK` (VCVTTPD2QQ). 
    /// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
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
    pub fn vcvttpd2qq_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttpd2qqMaskEmitter<A, B> {
        <Self as Vcvttpd2qqMaskEmitter<A, B>>::vcvttpd2qq_mask(self, op0, op1);
    }
    /// `VCVTTPD2QQ_MASK_SAE` (VCVTTPD2QQ). 
    /// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvttpd2qq_mask_sae<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttpd2qqMaskSaeEmitter<A, B> {
        <Self as Vcvttpd2qqMaskSaeEmitter<A, B>>::vcvttpd2qq_mask_sae(self, op0, op1);
    }
    /// `VCVTTPD2QQ_MASKZ` (VCVTTPD2QQ). 
    /// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
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
    pub fn vcvttpd2qq_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttpd2qqMaskzEmitter<A, B> {
        <Self as Vcvttpd2qqMaskzEmitter<A, B>>::vcvttpd2qq_maskz(self, op0, op1);
    }
    /// `VCVTTPD2QQ_MASKZ_SAE` (VCVTTPD2QQ). 
    /// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvttpd2qq_maskz_sae<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttpd2qqMaskzSaeEmitter<A, B> {
        <Self as Vcvttpd2qqMaskzSaeEmitter<A, B>>::vcvttpd2qq_maskz_sae(self, op0, op1);
    }
    /// `VCVTTPD2QQ_SAE` (VCVTTPD2QQ). 
    /// Converts with truncation packed double precision floating-point values in the source operand (second operand) to packed quadword integers in the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPD2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Zmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvttpd2qq_sae<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttpd2qqSaeEmitter<A, B> {
        <Self as Vcvttpd2qqSaeEmitter<A, B>>::vcvttpd2qq_sae(self, op0, op1);
    }
    /// `VCVTTPS2QQ` (VCVTTPS2QQ). 
    /// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
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
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvttps2qq<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttps2qqEmitter<A, B> {
        <Self as Vcvttps2qqEmitter<A, B>>::vcvttps2qq(self, op0, op1);
    }
    /// `VCVTTPS2QQ_MASK` (VCVTTPS2QQ). 
    /// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
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
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvttps2qq_mask<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttps2qqMaskEmitter<A, B> {
        <Self as Vcvttps2qqMaskEmitter<A, B>>::vcvttps2qq_mask(self, op0, op1);
    }
    /// `VCVTTPS2QQ_MASK_SAE` (VCVTTPS2QQ). 
    /// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvttps2qq_mask_sae<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttps2qqMaskSaeEmitter<A, B> {
        <Self as Vcvttps2qqMaskSaeEmitter<A, B>>::vcvttps2qq_mask_sae(self, op0, op1);
    }
    /// `VCVTTPS2QQ_MASKZ` (VCVTTPS2QQ). 
    /// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
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
    /// | 4 | Ymm, Xmm |
    /// | 5 | Zmm, Mem |
    /// | 6 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvttps2qq_maskz<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttps2qqMaskzEmitter<A, B> {
        <Self as Vcvttps2qqMaskzEmitter<A, B>>::vcvttps2qq_maskz(self, op0, op1);
    }
    /// `VCVTTPS2QQ_MASKZ_SAE` (VCVTTPS2QQ). 
    /// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvttps2qq_maskz_sae<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttps2qqMaskzSaeEmitter<A, B> {
        <Self as Vcvttps2qqMaskzSaeEmitter<A, B>>::vcvttps2qq_maskz_sae(self, op0, op1);
    }
    /// `VCVTTPS2QQ_SAE` (VCVTTPS2QQ). 
    /// Converts with truncation packed single precision floating-point values in the source operand to eight signed quadword integers in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VCVTTPS2QQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Zmm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vcvttps2qq_sae<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vcvttps2qqSaeEmitter<A, B> {
        <Self as Vcvttps2qqSaeEmitter<A, B>>::vcvttps2qq_sae(self, op0, op1);
    }
    /// `VFPCLASSPD` (VFPCLASSPD). 
    /// The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Mem, Imm |
    /// | 2 | KReg, Xmm, Imm |
    /// | 3 | KReg, Ymm, Imm |
    /// | 4 | KReg, Zmm, Imm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vfpclasspd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VfpclasspdEmitter<A, B, C> {
        <Self as VfpclasspdEmitter<A, B, C>>::vfpclasspd(self, op0, op1, op2);
    }
    /// `VFPCLASSPD_MASK` (VFPCLASSPD). 
    /// The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:8/4/2] of the destination are cleared.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Mem, Imm |
    /// | 2 | KReg, Xmm, Imm |
    /// | 3 | KReg, Ymm, Imm |
    /// | 4 | KReg, Zmm, Imm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vfpclasspd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VfpclasspdMaskEmitter<A, B, C> {
        <Self as VfpclasspdMaskEmitter<A, B, C>>::vfpclasspd_mask(self, op0, op1, op2);
    }
    /// `VFPCLASSPS` (VFPCLASSPS). 
    /// The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Mem, Imm |
    /// | 2 | KReg, Xmm, Imm |
    /// | 3 | KReg, Ymm, Imm |
    /// | 4 | KReg, Zmm, Imm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vfpclassps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VfpclasspsEmitter<A, B, C> {
        <Self as VfpclasspsEmitter<A, B, C>>::vfpclassps(self, op0, op1, op2);
    }
    /// `VFPCLASSPS_MASK` (VFPCLASSPS). 
    /// The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result of each element is written to the corresponding bit in a mask register k2 according to the writemask k1. Bits [MAX_KL-1:16/8/4] of the destination are cleared.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Mem, Imm |
    /// | 2 | KReg, Xmm, Imm |
    /// | 3 | KReg, Ymm, Imm |
    /// | 4 | KReg, Zmm, Imm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vfpclassps_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VfpclasspsMaskEmitter<A, B, C> {
        <Self as VfpclasspsMaskEmitter<A, B, C>>::vfpclassps_mask(self, op0, op1, op2);
    }
    /// `VFPCLASSSD` (VFPCLASSSD). 
    /// The FPCLASSSD instruction checks the low double precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSSD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Mem, Imm |
    /// | 2 | KReg, Xmm, Imm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vfpclasssd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VfpclasssdEmitter<A, B, C> {
        <Self as VfpclasssdEmitter<A, B, C>>::vfpclasssd(self, op0, op1, op2);
    }
    /// `VFPCLASSSD_MASK` (VFPCLASSSD). 
    /// The FPCLASSSD instruction checks the low double precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSSD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Mem, Imm |
    /// | 2 | KReg, Xmm, Imm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vfpclasssd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VfpclasssdMaskEmitter<A, B, C> {
        <Self as VfpclasssdMaskEmitter<A, B, C>>::vfpclasssd_mask(self, op0, op1, op2);
    }
    /// `VFPCLASSSS` (VFPCLASSSS). 
    /// The FPCLASSSS instruction checks the low single-precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSSS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Mem, Imm |
    /// | 2 | KReg, Xmm, Imm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vfpclassss<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VfpclassssEmitter<A, B, C> {
        <Self as VfpclassssEmitter<A, B, C>>::vfpclassss(self, op0, op1, op2);
    }
    /// `VFPCLASSSS_MASK` (VFPCLASSSS). 
    /// The FPCLASSSS instruction checks the low single-precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte. Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against. The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element. The result is written to the low bit in a mask register k2 according to the writemask k1. Bits MAX_KL-1: 1 of the destination are cleared.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VFPCLASSSS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------------+
    /// | # | Operands       |
    /// +---+----------------+
    /// | 1 | KReg, Mem, Imm |
    /// | 2 | KReg, Xmm, Imm |
    /// +---+----------------+
    /// ```
    #[inline]
    pub fn vfpclassss_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VfpclassssMaskEmitter<A, B, C> {
        <Self as VfpclassssMaskEmitter<A, B, C>>::vfpclassss_mask(self, op0, op1, op2);
    }
    /// `VINSERTF32X8` (VINSERTF32X8). 
    /// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Mem, Imm |
    /// | 2 | Zmm, Zmm, Ymm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinsertf32x8<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinsertf32x8Emitter<A, B, C, D> {
        <Self as Vinsertf32x8Emitter<A, B, C, D>>::vinsertf32x8(self, op0, op1, op2, op3);
    }
    /// `VINSERTF32X8_MASK` (VINSERTF32X8). 
    /// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Mem, Imm |
    /// | 2 | Zmm, Zmm, Ymm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinsertf32x8_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinsertf32x8MaskEmitter<A, B, C, D> {
        <Self as Vinsertf32x8MaskEmitter<A, B, C, D>>::vinsertf32x8_mask(self, op0, op1, op2, op3);
    }
    /// `VINSERTF32X8_MASKZ` (VINSERTF32X8). 
    /// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Mem, Imm |
    /// | 2 | Zmm, Zmm, Ymm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinsertf32x8_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinsertf32x8MaskzEmitter<A, B, C, D> {
        <Self as Vinsertf32x8MaskzEmitter<A, B, C, D>>::vinsertf32x8_maskz(self, op0, op1, op2, op3);
    }
    /// `VINSERTF64X2` (VINSERTF64X2). 
    /// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Ymm, Ymm, Mem, Imm |
    /// | 2 | Ymm, Ymm, Xmm, Imm |
    /// | 3 | Zmm, Zmm, Mem, Imm |
    /// | 4 | Zmm, Zmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinsertf64x2<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinsertf64x2Emitter<A, B, C, D> {
        <Self as Vinsertf64x2Emitter<A, B, C, D>>::vinsertf64x2(self, op0, op1, op2, op3);
    }
    /// `VINSERTF64X2_MASK` (VINSERTF64X2). 
    /// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Ymm, Ymm, Mem, Imm |
    /// | 2 | Ymm, Ymm, Xmm, Imm |
    /// | 3 | Zmm, Zmm, Mem, Imm |
    /// | 4 | Zmm, Zmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinsertf64x2_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinsertf64x2MaskEmitter<A, B, C, D> {
        <Self as Vinsertf64x2MaskEmitter<A, B, C, D>>::vinsertf64x2_mask(self, op0, op1, op2, op3);
    }
    /// `VINSERTF64X2_MASKZ` (VINSERTF64X2). 
    /// VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The destination and first source operands are vector registers.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTF128%3AVINSERTF32x4%3AVINSERTF64x2%3AVINSERTF32x8%3AVINSERTF64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Ymm, Ymm, Mem, Imm |
    /// | 2 | Ymm, Ymm, Xmm, Imm |
    /// | 3 | Zmm, Zmm, Mem, Imm |
    /// | 4 | Zmm, Zmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinsertf64x2_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinsertf64x2MaskzEmitter<A, B, C, D> {
        <Self as Vinsertf64x2MaskzEmitter<A, B, C, D>>::vinsertf64x2_maskz(self, op0, op1, op2, op3);
    }
    /// `VINSERTI32X8` (VINSERTI32X8). 
    /// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Mem, Imm |
    /// | 2 | Zmm, Zmm, Ymm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinserti32x8<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinserti32x8Emitter<A, B, C, D> {
        <Self as Vinserti32x8Emitter<A, B, C, D>>::vinserti32x8(self, op0, op1, op2, op3);
    }
    /// `VINSERTI32X8_MASK` (VINSERTI32X8). 
    /// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Mem, Imm |
    /// | 2 | Zmm, Zmm, Ymm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinserti32x8_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinserti32x8MaskEmitter<A, B, C, D> {
        <Self as Vinserti32x8MaskEmitter<A, B, C, D>>::vinserti32x8_mask(self, op0, op1, op2, op3);
    }
    /// `VINSERTI32X8_MASKZ` (VINSERTI32X8). 
    /// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Mem, Imm |
    /// | 2 | Zmm, Zmm, Ymm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinserti32x8_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinserti32x8MaskzEmitter<A, B, C, D> {
        <Self as Vinserti32x8MaskzEmitter<A, B, C, D>>::vinserti32x8_maskz(self, op0, op1, op2, op3);
    }
    /// `VINSERTI64X2` (VINSERTI64X2). 
    /// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Ymm, Ymm, Mem, Imm |
    /// | 2 | Ymm, Ymm, Xmm, Imm |
    /// | 3 | Zmm, Zmm, Mem, Imm |
    /// | 4 | Zmm, Zmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinserti64x2<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinserti64x2Emitter<A, B, C, D> {
        <Self as Vinserti64x2Emitter<A, B, C, D>>::vinserti64x2(self, op0, op1, op2, op3);
    }
    /// `VINSERTI64X2_MASK` (VINSERTI64X2). 
    /// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Ymm, Ymm, Mem, Imm |
    /// | 2 | Ymm, Ymm, Xmm, Imm |
    /// | 3 | Zmm, Zmm, Mem, Imm |
    /// | 4 | Zmm, Zmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinserti64x2_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinserti64x2MaskEmitter<A, B, C, D> {
        <Self as Vinserti64x2MaskEmitter<A, B, C, D>>::vinserti64x2_mask(self, op0, op1, op2, op3);
    }
    /// `VINSERTI64X2_MASKZ` (VINSERTI64X2). 
    /// VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0]. The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand). The second source operand can be either an XMM register or a 128-bit memory location. The high 6/7bits of the immediate are ignored. The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VINSERTI128%3AVINSERTI32x4%3AVINSERTI64x2%3AVINSERTI32x8%3AVINSERTI64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Ymm, Ymm, Mem, Imm |
    /// | 2 | Ymm, Ymm, Xmm, Imm |
    /// | 3 | Zmm, Zmm, Mem, Imm |
    /// | 4 | Zmm, Zmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinserti64x2_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinserti64x2MaskzEmitter<A, B, C, D> {
        <Self as Vinserti64x2MaskzEmitter<A, B, C, D>>::vinserti64x2_maskz(self, op0, op1, op2, op3);
    }
    /// `VORPD` (VORPD). 
    /// Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vorpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VorpdEmitter<A, B, C> {
        <Self as VorpdEmitter<A, B, C>>::vorpd(self, op0, op1, op2);
    }
    /// `VORPD_MASK` (VORPD). 
    /// Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vorpd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VorpdMaskEmitter<A, B, C> {
        <Self as VorpdMaskEmitter<A, B, C>>::vorpd_mask(self, op0, op1, op2);
    }
    /// `VORPD_MASKZ` (VORPD). 
    /// Performs a bitwise logical OR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vorpd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VorpdMaskzEmitter<A, B, C> {
        <Self as VorpdMaskzEmitter<A, B, C>>::vorpd_maskz(self, op0, op1, op2);
    }
    /// `VORPS` (VORPS). 
    /// Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vorps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VorpsEmitter<A, B, C> {
        <Self as VorpsEmitter<A, B, C>>::vorps(self, op0, op1, op2);
    }
    /// `VORPS_MASK` (VORPS). 
    /// Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vorps_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VorpsMaskEmitter<A, B, C> {
        <Self as VorpsMaskEmitter<A, B, C>>::vorps_mask(self, op0, op1, op2);
    }
    /// `VORPS_MASKZ` (VORPS). 
    /// Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ORPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vorps_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VorpsMaskzEmitter<A, B, C> {
        <Self as VorpsMaskzEmitter<A, B, C>>::vorps_maskz(self, op0, op1, op2);
    }
    /// `VPMOVD2M` (VPMOVD2M). 
    /// Converts a vector register to a mask register. Each element in the destination register is set to 1 or 0 depending on the value of most significant bit of the corresponding element in the source register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMOVB2M%3AVPMOVW2M%3AVPMOVD2M%3AVPMOVQ2M.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------+
    /// | # | Operands  |
    /// +---+-----------+
    /// | 1 | KReg, Xmm |
    /// | 2 | KReg, Ymm |
    /// | 3 | KReg, Zmm |
    /// +---+-----------+
    /// ```
    #[inline]
    pub fn vpmovd2m<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vpmovd2mEmitter<A, B> {
        <Self as Vpmovd2mEmitter<A, B>>::vpmovd2m(self, op0, op1);
    }
    /// `VPMOVM2D` (VPMOVM2D). 
    /// Converts a mask register to a vector register. Each element in the destination register is set to all 1’s or all 0’s depending on the value of the corresponding bit in the source mask register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMOVM2B%3AVPMOVM2W%3AVPMOVM2D%3AVPMOVM2Q.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------+
    /// | # | Operands  |
    /// +---+-----------+
    /// | 1 | Xmm, KReg |
    /// | 2 | Ymm, KReg |
    /// | 3 | Zmm, KReg |
    /// +---+-----------+
    /// ```
    #[inline]
    pub fn vpmovm2d<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vpmovm2dEmitter<A, B> {
        <Self as Vpmovm2dEmitter<A, B>>::vpmovm2d(self, op0, op1);
    }
    /// `VPMOVM2Q` (VPMOVM2Q). 
    /// Converts a mask register to a vector register. Each element in the destination register is set to all 1’s or all 0’s depending on the value of the corresponding bit in the source mask register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMOVM2B%3AVPMOVM2W%3AVPMOVM2D%3AVPMOVM2Q.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------+
    /// | # | Operands  |
    /// +---+-----------+
    /// | 1 | Xmm, KReg |
    /// | 2 | Ymm, KReg |
    /// | 3 | Zmm, KReg |
    /// +---+-----------+
    /// ```
    #[inline]
    pub fn vpmovm2q<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vpmovm2qEmitter<A, B> {
        <Self as Vpmovm2qEmitter<A, B>>::vpmovm2q(self, op0, op1);
    }
    /// `VPMOVQ2M` (VPMOVQ2M). 
    /// Converts a vector register to a mask register. Each element in the destination register is set to 1 or 0 depending on the value of most significant bit of the corresponding element in the source register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPMOVB2M%3AVPMOVW2M%3AVPMOVD2M%3AVPMOVQ2M.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+-----------+
    /// | # | Operands  |
    /// +---+-----------+
    /// | 1 | KReg, Xmm |
    /// | 2 | KReg, Ymm |
    /// | 3 | KReg, Zmm |
    /// +---+-----------+
    /// ```
    #[inline]
    pub fn vpmovq2m<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vpmovq2mEmitter<A, B> {
        <Self as Vpmovq2mEmitter<A, B>>::vpmovq2m(self, op0, op1);
    }
    /// `VPMULLD` (VPMULLD). 
    /// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmulld<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpmulldEmitter<A, B, C> {
        <Self as VpmulldEmitter<A, B, C>>::vpmulld(self, op0, op1, op2);
    }
    /// `VPMULLD_MASK` (VPMULLD). 
    /// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmulld_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpmulldMaskEmitter<A, B, C> {
        <Self as VpmulldMaskEmitter<A, B, C>>::vpmulld_mask(self, op0, op1, op2);
    }
    /// `VPMULLD_MASKZ` (VPMULLD). 
    /// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmulld_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpmulldMaskzEmitter<A, B, C> {
        <Self as VpmulldMaskzEmitter<A, B, C>>::vpmulld_maskz(self, op0, op1, op2);
    }
    /// `VPMULLQ` (VPMULLQ). 
    /// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmullq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpmullqEmitter<A, B, C> {
        <Self as VpmullqEmitter<A, B, C>>::vpmullq(self, op0, op1, op2);
    }
    /// `VPMULLQ_MASK` (VPMULLQ). 
    /// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmullq_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpmullqMaskEmitter<A, B, C> {
        <Self as VpmullqMaskEmitter<A, B, C>>::vpmullq_mask(self, op0, op1, op2);
    }
    /// `VPMULLQ_MASKZ` (VPMULLQ). 
    /// Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand. The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMULLD%3APMULLQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpmullq_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpmullqMaskzEmitter<A, B, C> {
        <Self as VpmullqMaskzEmitter<A, B, C>>::vpmullq_maskz(self, op0, op1, op2);
    }
    /// `VRANGEPD` (VRANGEPD). 
    /// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// | 3 | Ymm, Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Ymm, Imm |
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangepd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepdEmitter<A, B, C, D> {
        <Self as VrangepdEmitter<A, B, C, D>>::vrangepd(self, op0, op1, op2, op3);
    }
    /// `VRANGEPD_MASK` (VRANGEPD). 
    /// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// | 3 | Ymm, Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Ymm, Imm |
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangepd_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepdMaskEmitter<A, B, C, D> {
        <Self as VrangepdMaskEmitter<A, B, C, D>>::vrangepd_mask(self, op0, op1, op2, op3);
    }
    /// `VRANGEPD_MASK_SAE` (VRANGEPD). 
    /// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangepd_mask_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepdMaskSaeEmitter<A, B, C, D> {
        <Self as VrangepdMaskSaeEmitter<A, B, C, D>>::vrangepd_mask_sae(self, op0, op1, op2, op3);
    }
    /// `VRANGEPD_MASKZ` (VRANGEPD). 
    /// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// | 3 | Ymm, Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Ymm, Imm |
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangepd_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepdMaskzEmitter<A, B, C, D> {
        <Self as VrangepdMaskzEmitter<A, B, C, D>>::vrangepd_maskz(self, op0, op1, op2, op3);
    }
    /// `VRANGEPD_MASKZ_SAE` (VRANGEPD). 
    /// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangepd_maskz_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepdMaskzSaeEmitter<A, B, C, D> {
        <Self as VrangepdMaskzSaeEmitter<A, B, C, D>>::vrangepd_maskz_sae(self, op0, op1, op2, op3);
    }
    /// `VRANGEPD_SAE` (VRANGEPD). 
    /// This instruction calculates 2/4/8 range operation outputs from two sets of packed input double precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangepd_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepdSaeEmitter<A, B, C, D> {
        <Self as VrangepdSaeEmitter<A, B, C, D>>::vrangepd_sae(self, op0, op1, op2, op3);
    }
    /// `VRANGEPS` (VRANGEPS). 
    /// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// | 3 | Ymm, Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Ymm, Imm |
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangeps<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepsEmitter<A, B, C, D> {
        <Self as VrangepsEmitter<A, B, C, D>>::vrangeps(self, op0, op1, op2, op3);
    }
    /// `VRANGEPS_MASK` (VRANGEPS). 
    /// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// | 3 | Ymm, Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Ymm, Imm |
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangeps_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepsMaskEmitter<A, B, C, D> {
        <Self as VrangepsMaskEmitter<A, B, C, D>>::vrangeps_mask(self, op0, op1, op2, op3);
    }
    /// `VRANGEPS_MASK_SAE` (VRANGEPS). 
    /// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangeps_mask_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepsMaskSaeEmitter<A, B, C, D> {
        <Self as VrangepsMaskSaeEmitter<A, B, C, D>>::vrangeps_mask_sae(self, op0, op1, op2, op3);
    }
    /// `VRANGEPS_MASKZ` (VRANGEPS). 
    /// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// | 3 | Ymm, Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Ymm, Imm |
    /// | 5 | Zmm, Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangeps_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepsMaskzEmitter<A, B, C, D> {
        <Self as VrangepsMaskzEmitter<A, B, C, D>>::vrangeps_maskz(self, op0, op1, op2, op3);
    }
    /// `VRANGEPS_MASKZ_SAE` (VRANGEPS). 
    /// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangeps_maskz_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepsMaskzSaeEmitter<A, B, C, D> {
        <Self as VrangepsMaskzSaeEmitter<A, B, C, D>>::vrangeps_maskz_sae(self, op0, op1, op2, op3);
    }
    /// `VRANGEPS_SAE` (VRANGEPS). 
    /// This instruction calculates 4/8/16 range operation outputs from two sets of packed input single-precision floating-point values in the first source operand (the second operand) and the second source operand (the third operand). The range outputs are written to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Zmm, Zmm, Zmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangeps_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangepsSaeEmitter<A, B, C, D> {
        <Self as VrangepsSaeEmitter<A, B, C, D>>::vrangeps_sae(self, op0, op1, op2, op3);
    }
    /// `VRANGESD` (VRANGESD). 
    /// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangesd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangesdEmitter<A, B, C, D> {
        <Self as VrangesdEmitter<A, B, C, D>>::vrangesd(self, op0, op1, op2, op3);
    }
    /// `VRANGESD_MASK` (VRANGESD). 
    /// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangesd_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangesdMaskEmitter<A, B, C, D> {
        <Self as VrangesdMaskEmitter<A, B, C, D>>::vrangesd_mask(self, op0, op1, op2, op3);
    }
    /// `VRANGESD_MASK_SAE` (VRANGESD). 
    /// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangesd_mask_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangesdMaskSaeEmitter<A, B, C, D> {
        <Self as VrangesdMaskSaeEmitter<A, B, C, D>>::vrangesd_mask_sae(self, op0, op1, op2, op3);
    }
    /// `VRANGESD_MASKZ` (VRANGESD). 
    /// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangesd_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangesdMaskzEmitter<A, B, C, D> {
        <Self as VrangesdMaskzEmitter<A, B, C, D>>::vrangesd_maskz(self, op0, op1, op2, op3);
    }
    /// `VRANGESD_MASKZ_SAE` (VRANGESD). 
    /// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangesd_maskz_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangesdMaskzSaeEmitter<A, B, C, D> {
        <Self as VrangesdMaskzSaeEmitter<A, B, C, D>>::vrangesd_maskz_sae(self, op0, op1, op2, op3);
    }
    /// `VRANGESD_SAE` (VRANGESD). 
    /// This instruction calculates a range operation output from two input double precision floating-point values in the low qword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low qword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangesd_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangesdSaeEmitter<A, B, C, D> {
        <Self as VrangesdSaeEmitter<A, B, C, D>>::vrangesd_sae(self, op0, op1, op2, op3);
    }
    /// `VRANGESS` (VRANGESS). 
    /// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangess<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangessEmitter<A, B, C, D> {
        <Self as VrangessEmitter<A, B, C, D>>::vrangess(self, op0, op1, op2, op3);
    }
    /// `VRANGESS_MASK` (VRANGESS). 
    /// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangess_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangessMaskEmitter<A, B, C, D> {
        <Self as VrangessMaskEmitter<A, B, C, D>>::vrangess_mask(self, op0, op1, op2, op3);
    }
    /// `VRANGESS_MASK_SAE` (VRANGESS). 
    /// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangess_mask_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangessMaskSaeEmitter<A, B, C, D> {
        <Self as VrangessMaskSaeEmitter<A, B, C, D>>::vrangess_mask_sae(self, op0, op1, op2, op3);
    }
    /// `VRANGESS_MASKZ` (VRANGESS). 
    /// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangess_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangessMaskzEmitter<A, B, C, D> {
        <Self as VrangessMaskzEmitter<A, B, C, D>>::vrangess_maskz(self, op0, op1, op2, op3);
    }
    /// `VRANGESS_MASKZ_SAE` (VRANGESS). 
    /// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangess_maskz_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangessMaskzSaeEmitter<A, B, C, D> {
        <Self as VrangessMaskzSaeEmitter<A, B, C, D>>::vrangess_maskz_sae(self, op0, op1, op2, op3);
    }
    /// `VRANGESS_SAE` (VRANGESS). 
    /// This instruction calculates a range operation output from two input single-precision floating-point values in the low dword element of the first source operand (the second operand) and second source operand (the third operand). The range output is written to the low dword element of the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VRANGESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vrangess_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VrangessSaeEmitter<A, B, C, D> {
        <Self as VrangessSaeEmitter<A, B, C, D>>::vrangess_sae(self, op0, op1, op2, op3);
    }
    /// `VREDUCEPD` (VREDUCEPD). 
    /// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreducepd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepdEmitter<A, B, C> {
        <Self as VreducepdEmitter<A, B, C>>::vreducepd(self, op0, op1, op2);
    }
    /// `VREDUCEPD_MASK` (VREDUCEPD). 
    /// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreducepd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepdMaskEmitter<A, B, C> {
        <Self as VreducepdMaskEmitter<A, B, C>>::vreducepd_mask(self, op0, op1, op2);
    }
    /// `VREDUCEPD_MASK_SAE` (VREDUCEPD). 
    /// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreducepd_mask_sae<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepdMaskSaeEmitter<A, B, C> {
        <Self as VreducepdMaskSaeEmitter<A, B, C>>::vreducepd_mask_sae(self, op0, op1, op2);
    }
    /// `VREDUCEPD_MASKZ` (VREDUCEPD). 
    /// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreducepd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepdMaskzEmitter<A, B, C> {
        <Self as VreducepdMaskzEmitter<A, B, C>>::vreducepd_maskz(self, op0, op1, op2);
    }
    /// `VREDUCEPD_MASKZ_SAE` (VREDUCEPD). 
    /// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreducepd_maskz_sae<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepdMaskzSaeEmitter<A, B, C> {
        <Self as VreducepdMaskzSaeEmitter<A, B, C>>::vreducepd_maskz_sae(self, op0, op1, op2);
    }
    /// `VREDUCEPD_SAE` (VREDUCEPD). 
    /// Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreducepd_sae<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepdSaeEmitter<A, B, C> {
        <Self as VreducepdSaeEmitter<A, B, C>>::vreducepd_sae(self, op0, op1, op2);
    }
    /// `VREDUCEPS` (VREDUCEPS). 
    /// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreduceps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepsEmitter<A, B, C> {
        <Self as VreducepsEmitter<A, B, C>>::vreduceps(self, op0, op1, op2);
    }
    /// `VREDUCEPS_MASK` (VREDUCEPS). 
    /// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreduceps_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepsMaskEmitter<A, B, C> {
        <Self as VreducepsMaskEmitter<A, B, C>>::vreduceps_mask(self, op0, op1, op2);
    }
    /// `VREDUCEPS_MASK_SAE` (VREDUCEPS). 
    /// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreduceps_mask_sae<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepsMaskSaeEmitter<A, B, C> {
        <Self as VreducepsMaskSaeEmitter<A, B, C>>::vreduceps_mask_sae(self, op0, op1, op2);
    }
    /// `VREDUCEPS_MASKZ` (VREDUCEPS). 
    /// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// | 3 | Ymm, Mem, Imm |
    /// | 4 | Ymm, Ymm, Imm |
    /// | 5 | Zmm, Mem, Imm |
    /// | 6 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreduceps_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepsMaskzEmitter<A, B, C> {
        <Self as VreducepsMaskzEmitter<A, B, C>>::vreduceps_maskz(self, op0, op1, op2);
    }
    /// `VREDUCEPS_MASKZ_SAE` (VREDUCEPS). 
    /// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreduceps_maskz_sae<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepsMaskzSaeEmitter<A, B, C> {
        <Self as VreducepsMaskzSaeEmitter<A, B, C>>::vreduceps_maskz_sae(self, op0, op1, op2);
    }
    /// `VREDUCEPS_SAE` (VREDUCEPS). 
    /// Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCEPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Zmm, Zmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vreduceps_sae<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VreducepsSaeEmitter<A, B, C> {
        <Self as VreducepsSaeEmitter<A, B, C>>::vreduceps_sae(self, op0, op1, op2);
    }
    /// `VREDUCESD` (VREDUCESD). 
    /// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducesd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducesdEmitter<A, B, C, D> {
        <Self as VreducesdEmitter<A, B, C, D>>::vreducesd(self, op0, op1, op2, op3);
    }
    /// `VREDUCESD_MASK` (VREDUCESD). 
    /// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducesd_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducesdMaskEmitter<A, B, C, D> {
        <Self as VreducesdMaskEmitter<A, B, C, D>>::vreducesd_mask(self, op0, op1, op2, op3);
    }
    /// `VREDUCESD_MASK_SAE` (VREDUCESD). 
    /// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducesd_mask_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducesdMaskSaeEmitter<A, B, C, D> {
        <Self as VreducesdMaskSaeEmitter<A, B, C, D>>::vreducesd_mask_sae(self, op0, op1, op2, op3);
    }
    /// `VREDUCESD_MASKZ` (VREDUCESD). 
    /// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducesd_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducesdMaskzEmitter<A, B, C, D> {
        <Self as VreducesdMaskzEmitter<A, B, C, D>>::vreducesd_maskz(self, op0, op1, op2, op3);
    }
    /// `VREDUCESD_MASKZ_SAE` (VREDUCESD). 
    /// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducesd_maskz_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducesdMaskzSaeEmitter<A, B, C, D> {
        <Self as VreducesdMaskzSaeEmitter<A, B, C, D>>::vreducesd_maskz_sae(self, op0, op1, op2, op3);
    }
    /// `VREDUCESD_SAE` (VREDUCESD). 
    /// Perform a reduction transformation of the binary encoded double precision floating-point value in the low qword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low qword element of the destination operand (the first operand) under the writemask k1. Bits 127:64 of the destination operand are copied from respective qword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducesd_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducesdSaeEmitter<A, B, C, D> {
        <Self as VreducesdSaeEmitter<A, B, C, D>>::vreducesd_sae(self, op0, op1, op2, op3);
    }
    /// `VREDUCESS` (VREDUCESS). 
    /// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducess<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducessEmitter<A, B, C, D> {
        <Self as VreducessEmitter<A, B, C, D>>::vreducess(self, op0, op1, op2, op3);
    }
    /// `VREDUCESS_MASK` (VREDUCESS). 
    /// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducess_mask<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducessMaskEmitter<A, B, C, D> {
        <Self as VreducessMaskEmitter<A, B, C, D>>::vreducess_mask(self, op0, op1, op2, op3);
    }
    /// `VREDUCESS_MASK_SAE` (VREDUCESS). 
    /// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducess_mask_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducessMaskSaeEmitter<A, B, C, D> {
        <Self as VreducessMaskSaeEmitter<A, B, C, D>>::vreducess_mask_sae(self, op0, op1, op2, op3);
    }
    /// `VREDUCESS_MASKZ` (VREDUCESS). 
    /// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducess_maskz<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducessMaskzEmitter<A, B, C, D> {
        <Self as VreducessMaskzEmitter<A, B, C, D>>::vreducess_maskz(self, op0, op1, op2, op3);
    }
    /// `VREDUCESS_MASKZ_SAE` (VREDUCESS). 
    /// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducess_maskz_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducessMaskzSaeEmitter<A, B, C, D> {
        <Self as VreducessMaskzSaeEmitter<A, B, C, D>>::vreducess_maskz_sae(self, op0, op1, op2, op3);
    }
    /// `VREDUCESS_SAE` (VREDUCESS). 
    /// Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1. Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VREDUCESS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Xmm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vreducess_sae<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VreducessSaeEmitter<A, B, C, D> {
        <Self as VreducessSaeEmitter<A, B, C, D>>::vreducess_sae(self, op0, op1, op2, op3);
    }
    /// `VXORPD` (VXORPD). 
    /// Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vxorpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VxorpdEmitter<A, B, C> {
        <Self as VxorpdEmitter<A, B, C>>::vxorpd(self, op0, op1, op2);
    }
    /// `VXORPD_MASK` (VXORPD). 
    /// Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vxorpd_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VxorpdMaskEmitter<A, B, C> {
        <Self as VxorpdMaskEmitter<A, B, C>>::vxorpd_mask(self, op0, op1, op2);
    }
    /// `VXORPD_MASKZ` (VXORPD). 
    /// Performs a bitwise logical XOR of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vxorpd_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VxorpdMaskzEmitter<A, B, C> {
        <Self as VxorpdMaskzEmitter<A, B, C>>::vxorpd_maskz(self, op0, op1, op2);
    }
    /// `VXORPS` (VXORPS). 
    /// Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vxorps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VxorpsEmitter<A, B, C> {
        <Self as VxorpsEmitter<A, B, C>>::vxorps(self, op0, op1, op2);
    }
    /// `VXORPS_MASK` (VXORPS). 
    /// Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vxorps_mask<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VxorpsMaskEmitter<A, B, C> {
        <Self as VxorpsMaskEmitter<A, B, C>>::vxorps_mask(self, op0, op1, op2);
    }
    /// `VXORPS_MASKZ` (VXORPS). 
    /// Performs a bitwise logical XOR of the four, eight or sixteen packed single-precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XORPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem |
    /// | 4 | Ymm, Ymm, Ymm |
    /// | 5 | Zmm, Zmm, Mem |
    /// | 6 | Zmm, Zmm, Zmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vxorps_maskz<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VxorpsMaskzEmitter<A, B, C> {
        <Self as VxorpsMaskzEmitter<A, B, C>>::vxorps_maskz(self, op0, op1, op2);
    }
}
