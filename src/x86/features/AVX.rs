use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `VADDSUBPD` (VADDSUBPD). 
/// Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the corresponding double precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered double precision floating-point values from the second source operand from the corresponding double precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSUBPD.html).
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
/// +---+---------------+
/// ```
pub trait VaddsubpdEmitter<A, B, C> {
    fn vaddsubpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VaddsubpdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vaddsubpd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VADDSUBPD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VaddsubpdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vaddsubpd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VADDSUBPD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VaddsubpdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vaddsubpd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VADDSUBPD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VaddsubpdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vaddsubpd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VADDSUBPD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VADDSUBPS` (VADDSUBPS). 
/// Adds odd-numbered single precision floating-point values of the first source operand (second operand) with the corresponding single precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered single precision floating-point values from the second source operand from the corresponding single precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSUBPS.html).
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
/// +---+---------------+
/// ```
pub trait VaddsubpsEmitter<A, B, C> {
    fn vaddsubps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VaddsubpsEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vaddsubps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VADDSUBPS128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VaddsubpsEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vaddsubps(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VADDSUBPS128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VaddsubpsEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vaddsubps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VADDSUBPS256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VaddsubpsEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vaddsubps(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VADDSUBPS256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VBLENDPD` (VBLENDPD). 
/// Double-precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [3:0] determine whether the corresponding double precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is ”1”, then the double precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDPD.html).
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
/// +---+--------------------+
/// ```
pub trait VblendpdEmitter<A, B, C, D> {
    fn vblendpd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VblendpdEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vblendpd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VBLENDPD128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendpdEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vblendpd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VBLENDPD128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendpdEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vblendpd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VBLENDPD256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendpdEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vblendpd(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VBLENDPD256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VBLENDPS` (VBLENDPS). 
/// Packed single precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [7:0] determine whether the corresponding single precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is “1”, then the single precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDPS.html).
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
/// +---+--------------------+
/// ```
pub trait VblendpsEmitter<A, B, C, D> {
    fn vblendps(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VblendpsEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vblendps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VBLENDPS128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendpsEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vblendps(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VBLENDPS128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendpsEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vblendps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VBLENDPS256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendpsEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vblendps(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VBLENDPS256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VBLENDVPD` (VBLENDVPD). 
/// Conditionally copy each quadword data element of double precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each quadword element of the mask register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDVPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Xmm |
/// | 2 | Xmm, Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem, Ymm |
/// | 4 | Ymm, Ymm, Ymm, Ymm |
/// +---+--------------------+
/// ```
pub trait VblendvpdEmitter<A, B, C, D> {
    fn vblendvpd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VblendvpdEmitter<Xmm, Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vblendvpd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Xmm) {
        self.emit(VBLENDVPD128RRRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendvpdEmitter<Xmm, Xmm, Mem, Xmm> for Assembler<'a> {
    fn vblendvpd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Xmm) {
        self.emit(VBLENDVPD128RRMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendvpdEmitter<Ymm, Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vblendvpd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Ymm) {
        self.emit(VBLENDVPD256RRRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendvpdEmitter<Ymm, Ymm, Mem, Ymm> for Assembler<'a> {
    fn vblendvpd(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Ymm) {
        self.emit(VBLENDVPD256RRMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VBLENDVPS` (VBLENDVPS). 
/// Conditionally copy each dword data element of single precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each dword element of the mask register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDVPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Xmm |
/// | 2 | Xmm, Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem, Ymm |
/// | 4 | Ymm, Ymm, Ymm, Ymm |
/// +---+--------------------+
/// ```
pub trait VblendvpsEmitter<A, B, C, D> {
    fn vblendvps(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VblendvpsEmitter<Xmm, Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vblendvps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Xmm) {
        self.emit(VBLENDVPS128RRRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendvpsEmitter<Xmm, Xmm, Mem, Xmm> for Assembler<'a> {
    fn vblendvps(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Xmm) {
        self.emit(VBLENDVPS128RRMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendvpsEmitter<Ymm, Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vblendvps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Ymm) {
        self.emit(VBLENDVPS256RRRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VblendvpsEmitter<Ymm, Ymm, Mem, Ymm> for Assembler<'a> {
    fn vblendvps(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Ymm) {
        self.emit(VBLENDVPS256RRMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VBROADCASTF128` (VBROADCASTF128). 
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
/// +---+----------+
/// ```
pub trait Vbroadcastf128Emitter<A, B> {
    fn vbroadcastf128(&mut self, op0: A, op1: B);
}

impl<'a> Vbroadcastf128Emitter<Ymm, Xmm> for Assembler<'a> {
    fn vbroadcastf128(&mut self, op0: Ymm, op1: Xmm) {
        self.emit(VBROADCASTF128_256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Vbroadcastf128Emitter<Ymm, Mem> for Assembler<'a> {
    fn vbroadcastf128(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VBROADCASTF128_256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VCMPPD` (VCMPPD). 
/// Performs a SIMD compare of the packed double precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each pair of packed values in the two source operands.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +----+---------------------+
/// | #  | Operands            |
/// +----+---------------------+
/// | 1  | KReg, Xmm, Mem, Imm |
/// | 2  | KReg, Xmm, Xmm, Imm |
/// | 3  | KReg, Ymm, Mem, Imm |
/// | 4  | KReg, Ymm, Ymm, Imm |
/// | 5  | KReg, Zmm, Mem, Imm |
/// | 6  | KReg, Zmm, Zmm, Imm |
/// | 7  | Xmm, Xmm, Mem, Imm  |
/// | 8  | Xmm, Xmm, Xmm, Imm  |
/// | 9  | Ymm, Ymm, Mem, Imm  |
/// | 10 | Ymm, Ymm, Ymm, Imm  |
/// +----+---------------------+
/// ```
pub trait VcmppdEmitter<A, B, C, D> {
    fn vcmppd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VcmppdEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vcmppd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VCMPPD128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppdEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vcmppd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VCMPPD128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppdEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vcmppd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VCMPPD256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppdEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vcmppd(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VCMPPD256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppdEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vcmppd(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VCMPPD128KRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppdEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vcmppd(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VCMPPD128KRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppdEmitter<KReg, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vcmppd(&mut self, op0: KReg, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VCMPPD256KRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppdEmitter<KReg, Ymm, Mem, Imm> for Assembler<'a> {
    fn vcmppd(&mut self, op0: KReg, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VCMPPD256KRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppdEmitter<KReg, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vcmppd(&mut self, op0: KReg, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VCMPPD512KRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppdEmitter<KReg, Zmm, Mem, Imm> for Assembler<'a> {
    fn vcmppd(&mut self, op0: KReg, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VCMPPD512KRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VCMPPS` (VCMPPS). 
/// Performs a SIMD compare of the packed single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each of the pairs of packed values.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +----+---------------------+
/// | #  | Operands            |
/// +----+---------------------+
/// | 1  | KReg, Xmm, Mem, Imm |
/// | 2  | KReg, Xmm, Xmm, Imm |
/// | 3  | KReg, Ymm, Mem, Imm |
/// | 4  | KReg, Ymm, Ymm, Imm |
/// | 5  | KReg, Zmm, Mem, Imm |
/// | 6  | KReg, Zmm, Zmm, Imm |
/// | 7  | Xmm, Xmm, Mem, Imm  |
/// | 8  | Xmm, Xmm, Xmm, Imm  |
/// | 9  | Ymm, Ymm, Mem, Imm  |
/// | 10 | Ymm, Ymm, Ymm, Imm  |
/// +----+---------------------+
/// ```
pub trait VcmppsEmitter<A, B, C, D> {
    fn vcmpps(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VcmppsEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vcmpps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VCMPPS128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppsEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vcmpps(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VCMPPS128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppsEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vcmpps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VCMPPS256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppsEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vcmpps(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VCMPPS256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppsEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vcmpps(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VCMPPS128KRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppsEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vcmpps(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VCMPPS128KRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppsEmitter<KReg, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vcmpps(&mut self, op0: KReg, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VCMPPS256KRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppsEmitter<KReg, Ymm, Mem, Imm> for Assembler<'a> {
    fn vcmpps(&mut self, op0: KReg, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VCMPPS256KRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppsEmitter<KReg, Zmm, Zmm, Imm> for Assembler<'a> {
    fn vcmpps(&mut self, op0: KReg, op1: Zmm, op2: Zmm, op3: Imm) {
        self.emit(VCMPPS512KRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmppsEmitter<KReg, Zmm, Mem, Imm> for Assembler<'a> {
    fn vcmpps(&mut self, op0: KReg, op1: Zmm, op2: Mem, op3: Imm) {
        self.emit(VCMPPS512KRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VCMPSD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------------+
/// | # | Operands            |
/// +---+---------------------+
/// | 1 | KReg, Xmm, Mem, Imm |
/// | 2 | KReg, Xmm, Xmm, Imm |
/// | 3 | Xmm, Xmm, Mem, Imm  |
/// | 4 | Xmm, Xmm, Xmm, Imm  |
/// +---+---------------------+
/// ```
pub trait VcmpsdEmitter<A, B, C, D> {
    fn vcmpsd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VcmpsdEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vcmpsd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VCMPSDRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmpsdEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vcmpsd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VCMPSDRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmpsdEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vcmpsd(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VCMPSDKRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmpsdEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vcmpsd(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VCMPSDKRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VCMPSS` (VCMPSS). 
/// Compares the low single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate operand) specifies the type of comparison performed.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPSS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------------+
/// | # | Operands            |
/// +---+---------------------+
/// | 1 | KReg, Xmm, Mem, Imm |
/// | 2 | KReg, Xmm, Xmm, Imm |
/// | 3 | Xmm, Xmm, Mem, Imm  |
/// | 4 | Xmm, Xmm, Xmm, Imm  |
/// +---+---------------------+
/// ```
pub trait VcmpssEmitter<A, B, C, D> {
    fn vcmpss(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VcmpssEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vcmpss(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VCMPSSRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmpssEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vcmpss(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VCMPSSRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmpssEmitter<KReg, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vcmpss(&mut self, op0: KReg, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VCMPSSKRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VcmpssEmitter<KReg, Xmm, Mem, Imm> for Assembler<'a> {
    fn vcmpss(&mut self, op0: KReg, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VCMPSSKRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VDPPD` (VDPPD). 
/// Conditionally multiplies the packed double precision floating-point values in the destination operand (first operand) with the packed double precision floating-point values in the source (second operand) depending on a mask extracted from bits [5:4] of the immediate operand (third operand). If a condition mask bit is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DPPD.html).
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
pub trait VdppdEmitter<A, B, C, D> {
    fn vdppd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VdppdEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vdppd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VDPPD128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VdppdEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vdppd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VDPPD128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VDPPS` (VDPPS). 
/// Conditionally multiplies the packed single precision floating-point values in the destination operand (first operand) with the packed single precision floats in the source (second operand) depending on a mask extracted from the high 4 bits of the immediate byte (third operand). If a condition mask bit in imm8[7:4] is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DPPS.html).
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
/// +---+--------------------+
/// ```
pub trait VdppsEmitter<A, B, C, D> {
    fn vdpps(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VdppsEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vdpps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VDPPS128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VdppsEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vdpps(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VDPPS128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VdppsEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vdpps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VDPPS256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VdppsEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vdpps(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VDPPS256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VEXTRACTF128` (VEXTRACTF128). 
/// VEXTRACTF128/VEXTRACTF32x4 and VEXTRACTF64x2 extract 128-bits of single precision floating-point values from the source operand (the second operand) and store to the low 128-bit of the destination operand (the first operand). The 128-bit data extraction occurs at an 128-bit granular offset specified by imm8[0] (256-bit) or imm8[1:0] as the multiply factor. The destination may be either a vector register or an 128-bit memory location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VEXTRACTF128%3AVEXTRACTF32x4%3AVEXTRACTF64x2%3AVEXTRACTF32x8%3AVEXTRACTF64x4.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Ymm, Imm |
/// | 2 | Xmm, Ymm, Imm |
/// +---+---------------+
/// ```
pub trait Vextractf128Emitter<A, B, C> {
    fn vextractf128(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> Vextractf128Emitter<Xmm, Ymm, Imm> for Assembler<'a> {
    fn vextractf128(&mut self, op0: Xmm, op1: Ymm, op2: Imm) {
        self.emit(VEXTRACTF128RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> Vextractf128Emitter<Mem, Ymm, Imm> for Assembler<'a> {
    fn vextractf128(&mut self, op0: Mem, op1: Ymm, op2: Imm) {
        self.emit(VEXTRACTF128MRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VHADDPD` (VHADDPD). 
/// Adds the double precision floating-point values in the high and low quadwords of the destination operand and stores the result in the low quadword of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HADDPD.html).
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
/// +---+---------------+
/// ```
pub trait VhaddpdEmitter<A, B, C> {
    fn vhaddpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VhaddpdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vhaddpd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VHADDPD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhaddpdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vhaddpd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VHADDPD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhaddpdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vhaddpd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VHADDPD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhaddpdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vhaddpd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VHADDPD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VHADDPS` (VHADDPS). 
/// Adds the single precision floating-point values in the first and second dwords of the destination operand and stores the result in the first dword of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HADDPS.html).
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
/// +---+---------------+
/// ```
pub trait VhaddpsEmitter<A, B, C> {
    fn vhaddps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VhaddpsEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vhaddps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VHADDPS128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhaddpsEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vhaddps(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VHADDPS128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhaddpsEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vhaddps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VHADDPS256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhaddpsEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vhaddps(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VHADDPS256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VHSUBPD` (VHSUBPD). 
/// The HSUBPD instruction subtracts horizontally the packed double precision floating-point numbers of both operands.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HSUBPD.html).
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
/// +---+---------------+
/// ```
pub trait VhsubpdEmitter<A, B, C> {
    fn vhsubpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VhsubpdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vhsubpd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VHSUBPD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhsubpdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vhsubpd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VHSUBPD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhsubpdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vhsubpd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VHSUBPD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhsubpdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vhsubpd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VHSUBPD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VHSUBPS` (VHSUBPS). 
/// Subtracts the single precision floating-point value in the second dword of the destination operand from the first dword of the destination operand and stores the result in the first dword of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HSUBPS.html).
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
/// +---+---------------+
/// ```
pub trait VhsubpsEmitter<A, B, C> {
    fn vhsubps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VhsubpsEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vhsubps(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VHSUBPS128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhsubpsEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vhsubps(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VHSUBPS128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhsubpsEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vhsubps(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VHSUBPS256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VhsubpsEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vhsubps(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VHSUBPS256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VINSERTF128` (VINSERTF128). 
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
/// +---+--------------------+
/// ```
pub trait Vinsertf128Emitter<A, B, C, D> {
    fn vinsertf128(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vinsertf128Emitter<Ymm, Ymm, Xmm, Imm> for Assembler<'a> {
    fn vinsertf128(&mut self, op0: Ymm, op1: Ymm, op2: Xmm, op3: Imm) {
        self.emit(VINSERTF128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vinsertf128Emitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vinsertf128(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VINSERTF128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VLDDQU` (VLDDQU). 
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
/// | 2 | Ymm, Mem |
/// +---+----------+
/// ```
pub trait VlddquEmitter<A, B> {
    fn vlddqu(&mut self, op0: A, op1: B);
}

impl<'a> VlddquEmitter<Xmm, Mem> for Assembler<'a> {
    fn vlddqu(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VLDDQU128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VlddquEmitter<Ymm, Mem> for Assembler<'a> {
    fn vlddqu(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VLDDQU256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VLDMXCSR` (VLDMXCSR). 
/// Loads the source operand into the MXCSR control/status register. The source operand is a 32-bit memory location. See “MXCSR Control and Status Register” in Chapter 10, of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for a description of the MXCSR register and its contents.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/LDMXCSR.html).
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
pub trait VldmxcsrEmitter<A> {
    fn vldmxcsr(&mut self, op0: A);
}

impl<'a> VldmxcsrEmitter<Mem> for Assembler<'a> {
    fn vldmxcsr(&mut self, op0: Mem) {
        self.emit(VLDMXCSRM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `VMASKMOVDQU` (VMASKMOVDQU). 
/// Stores selected bytes from the source operand (first operand) into an 128-bit memory location. The mask operand (second operand) selects which bytes from the source operand are written to memory. The source and mask operands are XMM registers. The memory location specified by the effective address in the DI/EDI/RDI register (the default segment register is DS, but this may be overridden with a segment-override prefix). The memory location does not need to be aligned on a natural boundary. (The size of the store address depends on the address-size attribute.)
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MASKMOVDQU.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Xmm |
/// +---+----------+
/// ```
pub trait VmaskmovdquEmitter<A, B> {
    fn vmaskmovdqu(&mut self, op0: A, op1: B);
}

impl<'a> VmaskmovdquEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vmaskmovdqu(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VMASKMOVDQU128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMASKMOVPD` (VMASKMOVPD). 
/// Conditionally moves packed data elements from the second source operand into the corresponding data element of the destination operand, depending on the mask bits associated with each data element. The mask bits are specified in the first source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VMASKMOV.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Xmm, Xmm |
/// | 2 | Mem, Ymm, Ymm |
/// | 3 | Xmm, Xmm, Mem |
/// | 4 | Ymm, Ymm, Mem |
/// +---+---------------+
/// ```
pub trait VmaskmovpdEmitter<A, B, C> {
    fn vmaskmovpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VmaskmovpdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vmaskmovpd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VMASKMOVPD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VmaskmovpdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vmaskmovpd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VMASKMOVPD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VmaskmovpdEmitter<Mem, Xmm, Xmm> for Assembler<'a> {
    fn vmaskmovpd(&mut self, op0: Mem, op1: Xmm, op2: Xmm) {
        self.emit(VMASKMOVPD128MRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VmaskmovpdEmitter<Mem, Ymm, Ymm> for Assembler<'a> {
    fn vmaskmovpd(&mut self, op0: Mem, op1: Ymm, op2: Ymm) {
        self.emit(VMASKMOVPD256MRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VMASKMOVPS` (VMASKMOVPS). 
/// Conditionally moves packed data elements from the second source operand into the corresponding data element of the destination operand, depending on the mask bits associated with each data element. The mask bits are specified in the first source operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VMASKMOV.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Mem, Xmm, Xmm |
/// | 2 | Mem, Ymm, Ymm |
/// | 3 | Xmm, Xmm, Mem |
/// | 4 | Ymm, Ymm, Mem |
/// +---+---------------+
/// ```
pub trait VmaskmovpsEmitter<A, B, C> {
    fn vmaskmovps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VmaskmovpsEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vmaskmovps(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VMASKMOVPS128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VmaskmovpsEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vmaskmovps(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VMASKMOVPS256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VmaskmovpsEmitter<Mem, Xmm, Xmm> for Assembler<'a> {
    fn vmaskmovps(&mut self, op0: Mem, op1: Xmm, op2: Xmm) {
        self.emit(VMASKMOVPS128MRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VmaskmovpsEmitter<Mem, Ymm, Ymm> for Assembler<'a> {
    fn vmaskmovps(&mut self, op0: Mem, op1: Ymm, op2: Ymm) {
        self.emit(VMASKMOVPS256MRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VMOVD` (VMOVD). 
/// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Xmm, Mem |
/// +---+----------+
/// ```
pub trait VmovdEmitter<A, B> {
    fn vmovd(&mut self, op0: A, op1: B);
}

impl<'a> VmovdEmitter<Xmm, Mem> for Assembler<'a> {
    fn vmovd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VMOVDRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovdEmitter<Mem, Xmm> for Assembler<'a> {
    fn vmovd(&mut self, op0: Mem, op1: Xmm) {
        self.emit(VMOVDMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMOVDQA` (VMOVDQA). 
/// Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Xmm, Mem |
/// | 4 | Xmm, Xmm |
/// | 5 | Ymm, Mem |
/// | 6 | Ymm, Ymm |
/// +---+----------+
/// ```
pub trait VmovdqaEmitter<A, B> {
    fn vmovdqa(&mut self, op0: A, op1: B);
}

impl<'a> VmovdqaEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vmovdqa(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VMOVDQA128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovdqaEmitter<Xmm, Mem> for Assembler<'a> {
    fn vmovdqa(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VMOVDQA128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovdqaEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vmovdqa(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VMOVDQA256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovdqaEmitter<Ymm, Mem> for Assembler<'a> {
    fn vmovdqa(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VMOVDQA256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovdqaEmitter<Mem, Xmm> for Assembler<'a> {
    fn vmovdqa(&mut self, op0: Mem, op1: Xmm) {
        self.emit(VMOVDQA128MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovdqaEmitter<Mem, Ymm> for Assembler<'a> {
    fn vmovdqa(&mut self, op0: Mem, op1: Ymm) {
        self.emit(VMOVDQA256MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMOVDQU` (VMOVDQU). 
/// Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Mem, Xmm |
/// | 2 | Mem, Ymm |
/// | 3 | Xmm, Mem |
/// | 4 | Xmm, Xmm |
/// | 5 | Ymm, Mem |
/// | 6 | Ymm, Ymm |
/// +---+----------+
/// ```
pub trait VmovdquEmitter<A, B> {
    fn vmovdqu(&mut self, op0: A, op1: B);
}

impl<'a> VmovdquEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vmovdqu(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VMOVDQU128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovdquEmitter<Xmm, Mem> for Assembler<'a> {
    fn vmovdqu(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VMOVDQU128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovdquEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vmovdqu(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VMOVDQU256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovdquEmitter<Ymm, Mem> for Assembler<'a> {
    fn vmovdqu(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VMOVDQU256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovdquEmitter<Mem, Xmm> for Assembler<'a> {
    fn vmovdqu(&mut self, op0: Mem, op1: Xmm) {
        self.emit(VMOVDQU128MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovdquEmitter<Mem, Ymm> for Assembler<'a> {
    fn vmovdqu(&mut self, op0: Mem, op1: Ymm) {
        self.emit(VMOVDQU256MR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMOVD_G2X` (VMOVD). 
/// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Gpd |
/// +---+----------+
/// ```
pub trait VmovdG2xEmitter<A, B> {
    fn vmovd_g2x(&mut self, op0: A, op1: B);
}

impl<'a> VmovdG2xEmitter<Xmm, Gpd> for Assembler<'a> {
    fn vmovd_g2x(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(VMOVD_G2XRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMOVD_X2G` (VMOVD). 
/// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Xmm |
/// +---+----------+
/// ```
pub trait VmovdX2gEmitter<A, B> {
    fn vmovd_x2g(&mut self, op0: A, op1: B);
}

impl<'a> VmovdX2gEmitter<Gpd, Xmm> for Assembler<'a> {
    fn vmovd_x2g(&mut self, op0: Gpd, op1: Xmm) {
        self.emit(VMOVD_X2GRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMOVMSKPD` (VMOVMSKPD). 
/// Extracts the sign bits from the packed double precision floating-point values in the source operand (second operand), formats them into a 2-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM register, and the destination operand is a general-purpose register. The mask is stored in the 2 low-order bits of the destination operand. Zero-extend the upper bits of the destination.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVMSKPD.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Xmm |
/// | 2 | Gpd, Ymm |
/// +---+----------+
/// ```
pub trait VmovmskpdEmitter<A, B> {
    fn vmovmskpd(&mut self, op0: A, op1: B);
}

impl<'a> VmovmskpdEmitter<Gpd, Xmm> for Assembler<'a> {
    fn vmovmskpd(&mut self, op0: Gpd, op1: Xmm) {
        self.emit(VMOVMSKPD128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovmskpdEmitter<Gpd, Ymm> for Assembler<'a> {
    fn vmovmskpd(&mut self, op0: Gpd, op1: Ymm) {
        self.emit(VMOVMSKPD256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMOVMSKPS` (VMOVMSKPS). 
/// Extracts the sign bits from the packed single precision floating-point values in the source operand (second operand), formats them into a 4- or 8-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM or YMM register, and the destination operand is a general-purpose register. The mask is stored in the 4 or 8 low-order bits of the destination operand. The upper bits of the destination operand beyond the mask are filled with zeros.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVMSKPS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Xmm |
/// | 2 | Gpd, Ymm |
/// +---+----------+
/// ```
pub trait VmovmskpsEmitter<A, B> {
    fn vmovmskps(&mut self, op0: A, op1: B);
}

impl<'a> VmovmskpsEmitter<Gpd, Xmm> for Assembler<'a> {
    fn vmovmskps(&mut self, op0: Gpd, op1: Xmm) {
        self.emit(VMOVMSKPS128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovmskpsEmitter<Gpd, Ymm> for Assembler<'a> {
    fn vmovmskps(&mut self, op0: Gpd, op1: Ymm) {
        self.emit(VMOVMSKPS256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMOVQ_G2X` (VMOVQ). 
/// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Xmm, Gpd |
/// | 2 | Xmm, Gpq |
/// | 3 | Xmm, Mem |
/// +---+----------+
/// ```
pub trait VmovqG2xEmitter<A, B> {
    fn vmovq_g2x(&mut self, op0: A, op1: B);
}

impl<'a> VmovqG2xEmitter<Xmm, Gpd> for Assembler<'a> {
    fn vmovq_g2x(&mut self, op0: Xmm, op1: Gpd) {
        self.emit(VMOVQ_G2XRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovqG2xEmitter<Xmm, Mem> for Assembler<'a> {
    fn vmovq_g2x(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VMOVQ_G2XRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovqG2xEmitter<Xmm, Gpq> for Assembler<'a> {
    fn vmovq_g2x(&mut self, op0: Xmm, op1: Gpq) {
        self.emit(VMOVQ_G2XRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMOVQ_X2G` (VMOVQ). 
/// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Xmm |
/// | 2 | Gpq, Xmm |
/// | 3 | Mem, Xmm |
/// +---+----------+
/// ```
pub trait VmovqX2gEmitter<A, B> {
    fn vmovq_x2g(&mut self, op0: A, op1: B);
}

impl<'a> VmovqX2gEmitter<Gpd, Xmm> for Assembler<'a> {
    fn vmovq_x2g(&mut self, op0: Gpd, op1: Xmm) {
        self.emit(VMOVQ_X2GRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovqX2gEmitter<Mem, Xmm> for Assembler<'a> {
    fn vmovq_x2g(&mut self, op0: Mem, op1: Xmm) {
        self.emit(VMOVQ_X2GMR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VmovqX2gEmitter<Gpq, Xmm> for Assembler<'a> {
    fn vmovq_x2g(&mut self, op0: Gpq, op1: Xmm) {
        self.emit(VMOVQ_X2GRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VMPSADBW` (VMPSADBW). 
/// (V)MPSADBW calculates packed word results of sum-absolute-difference (SAD) of unsigned bytes from two blocks of 32-bit dword elements, using two select fields in the immediate byte to select the offsets of the two blocks within the first source operand and the second operand. Packed SAD word results are calculated within each 128-bit lane. Each SAD word result is calculated between a stationary block_2 (whose offset within the second source operand is selected by a two bit select control, multiplied by 32 bits) and a sliding block_1 at consecutive byte-granular position within the first source operand. The offset of the first 32-bit block of block_1 is selectable using a one bit select control, multiplied by 32 bits.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MPSADBW.html).
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
/// +---+--------------------+
/// ```
pub trait VmpsadbwEmitter<A, B, C, D> {
    fn vmpsadbw(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VmpsadbwEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vmpsadbw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VMPSADBW128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VmpsadbwEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vmpsadbw(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VMPSADBW128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VmpsadbwEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vmpsadbw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VMPSADBW256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VmpsadbwEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vmpsadbw(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VMPSADBW256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VPAND` (VPAND). 
/// Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PAND.html).
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
/// +---+---------------+
/// ```
pub trait VpandEmitter<A, B, C> {
    fn vpand(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpandEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpand(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPAND128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpandEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpand(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPAND128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpandEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpand(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPAND256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpandEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpand(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPAND256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPANDN` (VPANDN). 
/// Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PANDN.html).
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
/// +---+---------------+
/// ```
pub trait VpandnEmitter<A, B, C> {
    fn vpandn(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpandnEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpandn(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPANDN128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpandnEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpandn(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPANDN128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpandnEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpandn(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPANDN256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpandnEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpandn(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPANDN256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPBLENDVB` (VPBLENDVB). 
/// Conditionally copies byte elements from the source operand (second operand) to the destination operand (first operand) depending on mask bits defined in the implicit third register argument, XMM0. The mask bits are the most significant bit in each byte element of the XMM0 register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PBLENDVB.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Mem, Xmm |
/// | 2 | Xmm, Xmm, Xmm, Xmm |
/// | 3 | Ymm, Ymm, Mem, Ymm |
/// | 4 | Ymm, Ymm, Ymm, Ymm |
/// +---+--------------------+
/// ```
pub trait VpblendvbEmitter<A, B, C, D> {
    fn vpblendvb(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpblendvbEmitter<Xmm, Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpblendvb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Xmm) {
        self.emit(VPBLENDVB128RRRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpblendvbEmitter<Xmm, Xmm, Mem, Xmm> for Assembler<'a> {
    fn vpblendvb(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Xmm) {
        self.emit(VPBLENDVB128RRMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpblendvbEmitter<Ymm, Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpblendvb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Ymm) {
        self.emit(VPBLENDVB256RRRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpblendvbEmitter<Ymm, Ymm, Mem, Ymm> for Assembler<'a> {
    fn vpblendvb(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Ymm) {
        self.emit(VPBLENDVB256RRMR, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VPBLENDW` (VPBLENDW). 
/// Words from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source. If a bit in the mask, corresponding to a word, is “1", then the word is copied, else the word element in the destination operand is unchanged.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PBLENDW.html).
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
/// +---+--------------------+
/// ```
pub trait VpblendwEmitter<A, B, C, D> {
    fn vpblendw(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpblendwEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpblendw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VPBLENDW128RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpblendwEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpblendw(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VPBLENDW128RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpblendwEmitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vpblendw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VPBLENDW256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpblendwEmitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vpblendw(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VPBLENDW256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VPCMPEQB` (VPCMPEQB). 
/// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
///
/// Supported operand variants:
///
/// ```text
/// +----+----------------+
/// | #  | Operands       |
/// +----+----------------+
/// | 1  | KReg, Xmm, Mem |
/// | 2  | KReg, Xmm, Xmm |
/// | 3  | KReg, Ymm, Mem |
/// | 4  | KReg, Ymm, Ymm |
/// | 5  | KReg, Zmm, Mem |
/// | 6  | KReg, Zmm, Zmm |
/// | 7  | Xmm, Xmm, Mem  |
/// | 8  | Xmm, Xmm, Xmm  |
/// | 9  | Ymm, Ymm, Mem  |
/// | 10 | Ymm, Ymm, Ymm  |
/// +----+----------------+
/// ```
pub trait VpcmpeqbEmitter<A, B, C> {
    fn vpcmpeqb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpeqbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpeqb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPEQB128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpcmpeqb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPCMPEQB128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpeqb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPEQB256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpcmpeqb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPCMPEQB256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqbEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpeqb(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPEQB128KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqbEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vpcmpeqb(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(VPCMPEQB128KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqbEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpeqb(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPEQB256KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqbEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vpcmpeqb(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(VPCMPEQB256KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqbEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vpcmpeqb(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(VPCMPEQB512KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqbEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vpcmpeqb(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(VPCMPEQB512KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPCMPEQD` (VPCMPEQD). 
/// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
///
/// Supported operand variants:
///
/// ```text
/// +----+----------------+
/// | #  | Operands       |
/// +----+----------------+
/// | 1  | KReg, Xmm, Mem |
/// | 2  | KReg, Xmm, Xmm |
/// | 3  | KReg, Ymm, Mem |
/// | 4  | KReg, Ymm, Ymm |
/// | 5  | KReg, Zmm, Mem |
/// | 6  | KReg, Zmm, Zmm |
/// | 7  | Xmm, Xmm, Mem  |
/// | 8  | Xmm, Xmm, Xmm  |
/// | 9  | Ymm, Ymm, Mem  |
/// | 10 | Ymm, Ymm, Ymm  |
/// +----+----------------+
/// ```
pub trait VpcmpeqdEmitter<A, B, C> {
    fn vpcmpeqd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpeqdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpeqd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPEQD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpcmpeqd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPCMPEQD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpeqd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPEQD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpcmpeqd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPCMPEQD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqdEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpeqd(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPEQD128KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqdEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vpcmpeqd(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(VPCMPEQD128KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqdEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpeqd(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPEQD256KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqdEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vpcmpeqd(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(VPCMPEQD256KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqdEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vpcmpeqd(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(VPCMPEQD512KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqdEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vpcmpeqd(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(VPCMPEQD512KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPCMPEQQ` (VPCMPEQQ). 
/// Performs an SIMD compare for equality of the packed quadwords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +----+----------------+
/// | #  | Operands       |
/// +----+----------------+
/// | 1  | KReg, Xmm, Mem |
/// | 2  | KReg, Xmm, Xmm |
/// | 3  | KReg, Ymm, Mem |
/// | 4  | KReg, Ymm, Ymm |
/// | 5  | KReg, Zmm, Mem |
/// | 6  | KReg, Zmm, Zmm |
/// | 7  | Xmm, Xmm, Mem  |
/// | 8  | Xmm, Xmm, Xmm  |
/// | 9  | Ymm, Ymm, Mem  |
/// | 10 | Ymm, Ymm, Ymm  |
/// +----+----------------+
/// ```
pub trait VpcmpeqqEmitter<A, B, C> {
    fn vpcmpeqq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpeqqEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpeqq(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPEQQ128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqqEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpcmpeqq(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPCMPEQQ128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqqEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpeqq(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPEQQ256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqqEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpcmpeqq(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPCMPEQQ256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqqEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpeqq(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPEQQ128KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqqEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vpcmpeqq(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(VPCMPEQQ128KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqqEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpeqq(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPEQQ256KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqqEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vpcmpeqq(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(VPCMPEQQ256KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqqEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vpcmpeqq(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(VPCMPEQQ512KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqqEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vpcmpeqq(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(VPCMPEQQ512KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPCMPEQW` (VPCMPEQW). 
/// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
///
/// Supported operand variants:
///
/// ```text
/// +----+----------------+
/// | #  | Operands       |
/// +----+----------------+
/// | 1  | KReg, Xmm, Mem |
/// | 2  | KReg, Xmm, Xmm |
/// | 3  | KReg, Ymm, Mem |
/// | 4  | KReg, Ymm, Ymm |
/// | 5  | KReg, Zmm, Mem |
/// | 6  | KReg, Zmm, Zmm |
/// | 7  | Xmm, Xmm, Mem  |
/// | 8  | Xmm, Xmm, Xmm  |
/// | 9  | Ymm, Ymm, Mem  |
/// | 10 | Ymm, Ymm, Ymm  |
/// +----+----------------+
/// ```
pub trait VpcmpeqwEmitter<A, B, C> {
    fn vpcmpeqw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpeqwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpeqw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPEQW128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpcmpeqw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPCMPEQW128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpeqw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPEQW256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpcmpeqw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPCMPEQW256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqwEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpeqw(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPEQW128KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqwEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vpcmpeqw(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(VPCMPEQW128KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqwEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpeqw(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPEQW256KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqwEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vpcmpeqw(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(VPCMPEQW256KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqwEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vpcmpeqw(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(VPCMPEQW512KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpeqwEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vpcmpeqw(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(VPCMPEQW512KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPCMPESTRI` (VPCMPESTRI). 
/// The instruction compares and processes data from two string fragments based on the encoded value in the imm8 control byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates an index stored to the count register (ECX).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPESTRI.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// +---+---------------+
/// ```
pub trait VpcmpestriEmitter<A, B, C> {
    fn vpcmpestri(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpestriEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpestri(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VPCMPESTRIRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpestriEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpestri(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VPCMPESTRIRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPCMPESTRM` (VPCMPESTRM). 
/// The instruction compares data from two string fragments based on the encoded value in the imm8 contol byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates a mask stored to XMM0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPESTRM.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// +---+---------------+
/// ```
pub trait VpcmpestrmEmitter<A, B, C> {
    fn vpcmpestrm(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpestrmEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpestrm(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VPCMPESTRMRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpestrmEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpestrm(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VPCMPESTRMRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPCMPGTB` (VPCMPGTB). 
/// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
///
/// Supported operand variants:
///
/// ```text
/// +----+----------------+
/// | #  | Operands       |
/// +----+----------------+
/// | 1  | KReg, Xmm, Mem |
/// | 2  | KReg, Xmm, Xmm |
/// | 3  | KReg, Ymm, Mem |
/// | 4  | KReg, Ymm, Ymm |
/// | 5  | KReg, Zmm, Mem |
/// | 6  | KReg, Zmm, Zmm |
/// | 7  | Xmm, Xmm, Mem  |
/// | 8  | Xmm, Xmm, Xmm  |
/// | 9  | Ymm, Ymm, Mem  |
/// | 10 | Ymm, Ymm, Ymm  |
/// +----+----------------+
/// ```
pub trait VpcmpgtbEmitter<A, B, C> {
    fn vpcmpgtb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpgtbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpgtb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPGTB128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpcmpgtb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPCMPGTB128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpgtb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPGTB256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpcmpgtb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPCMPGTB256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtbEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpgtb(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPGTB128KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtbEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vpcmpgtb(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(VPCMPGTB128KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtbEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpgtb(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPGTB256KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtbEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vpcmpgtb(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(VPCMPGTB256KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtbEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vpcmpgtb(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(VPCMPGTB512KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtbEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vpcmpgtb(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(VPCMPGTB512KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPCMPGTD` (VPCMPGTD). 
/// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
///
/// Supported operand variants:
///
/// ```text
/// +----+----------------+
/// | #  | Operands       |
/// +----+----------------+
/// | 1  | KReg, Xmm, Mem |
/// | 2  | KReg, Xmm, Xmm |
/// | 3  | KReg, Ymm, Mem |
/// | 4  | KReg, Ymm, Ymm |
/// | 5  | KReg, Zmm, Mem |
/// | 6  | KReg, Zmm, Zmm |
/// | 7  | Xmm, Xmm, Mem  |
/// | 8  | Xmm, Xmm, Xmm  |
/// | 9  | Ymm, Ymm, Mem  |
/// | 10 | Ymm, Ymm, Ymm  |
/// +----+----------------+
/// ```
pub trait VpcmpgtdEmitter<A, B, C> {
    fn vpcmpgtd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpgtdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpgtd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPGTD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpcmpgtd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPCMPGTD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpgtd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPGTD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpcmpgtd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPCMPGTD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtdEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpgtd(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPGTD128KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtdEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vpcmpgtd(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(VPCMPGTD128KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtdEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpgtd(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPGTD256KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtdEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vpcmpgtd(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(VPCMPGTD256KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtdEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vpcmpgtd(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(VPCMPGTD512KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtdEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vpcmpgtd(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(VPCMPGTD512KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPCMPGTQ` (VPCMPGTQ). 
/// Performs an SIMD signed compare for the packed quadwords in the destination operand (first operand) and the source operand (second operand). If the data element in the first (destination) operand is greater than the corresponding element in the second (source) operand, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +----+----------------+
/// | #  | Operands       |
/// +----+----------------+
/// | 1  | KReg, Xmm, Mem |
/// | 2  | KReg, Xmm, Xmm |
/// | 3  | KReg, Ymm, Mem |
/// | 4  | KReg, Ymm, Ymm |
/// | 5  | KReg, Zmm, Mem |
/// | 6  | KReg, Zmm, Zmm |
/// | 7  | Xmm, Xmm, Mem  |
/// | 8  | Xmm, Xmm, Xmm  |
/// | 9  | Ymm, Ymm, Mem  |
/// | 10 | Ymm, Ymm, Ymm  |
/// +----+----------------+
/// ```
pub trait VpcmpgtqEmitter<A, B, C> {
    fn vpcmpgtq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpgtqEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpgtq(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPGTQ128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtqEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpcmpgtq(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPCMPGTQ128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtqEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpgtq(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPGTQ256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtqEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpcmpgtq(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPCMPGTQ256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtqEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpgtq(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPGTQ128KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtqEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vpcmpgtq(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(VPCMPGTQ128KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtqEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpgtq(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPGTQ256KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtqEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vpcmpgtq(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(VPCMPGTQ256KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtqEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vpcmpgtq(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(VPCMPGTQ512KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtqEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vpcmpgtq(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(VPCMPGTQ512KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPCMPGTW` (VPCMPGTW). 
/// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
///
/// Supported operand variants:
///
/// ```text
/// +----+----------------+
/// | #  | Operands       |
/// +----+----------------+
/// | 1  | KReg, Xmm, Mem |
/// | 2  | KReg, Xmm, Xmm |
/// | 3  | KReg, Ymm, Mem |
/// | 4  | KReg, Ymm, Ymm |
/// | 5  | KReg, Zmm, Mem |
/// | 6  | KReg, Zmm, Zmm |
/// | 7  | Xmm, Xmm, Mem  |
/// | 8  | Xmm, Xmm, Xmm  |
/// | 9  | Ymm, Ymm, Mem  |
/// | 10 | Ymm, Ymm, Ymm  |
/// +----+----------------+
/// ```
pub trait VpcmpgtwEmitter<A, B, C> {
    fn vpcmpgtw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpgtwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpgtw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPGTW128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpcmpgtw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPCMPGTW128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpgtw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPGTW256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpcmpgtw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPCMPGTW256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtwEmitter<KReg, Xmm, Xmm> for Assembler<'a> {
    fn vpcmpgtw(&mut self, op0: KReg, op1: Xmm, op2: Xmm) {
        self.emit(VPCMPGTW128KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtwEmitter<KReg, Xmm, Mem> for Assembler<'a> {
    fn vpcmpgtw(&mut self, op0: KReg, op1: Xmm, op2: Mem) {
        self.emit(VPCMPGTW128KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtwEmitter<KReg, Ymm, Ymm> for Assembler<'a> {
    fn vpcmpgtw(&mut self, op0: KReg, op1: Ymm, op2: Ymm) {
        self.emit(VPCMPGTW256KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtwEmitter<KReg, Ymm, Mem> for Assembler<'a> {
    fn vpcmpgtw(&mut self, op0: KReg, op1: Ymm, op2: Mem) {
        self.emit(VPCMPGTW256KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtwEmitter<KReg, Zmm, Zmm> for Assembler<'a> {
    fn vpcmpgtw(&mut self, op0: KReg, op1: Zmm, op2: Zmm) {
        self.emit(VPCMPGTW512KRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpgtwEmitter<KReg, Zmm, Mem> for Assembler<'a> {
    fn vpcmpgtw(&mut self, op0: KReg, op1: Zmm, op2: Mem) {
        self.emit(VPCMPGTW512KRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPCMPISTRI` (VPCMPISTRI). 
/// The instruction compares data from two strings based on the encoded value in the imm8 control byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates an index stored to ECX.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPISTRI.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// +---+---------------+
/// ```
pub trait VpcmpistriEmitter<A, B, C> {
    fn vpcmpistri(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpistriEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpistri(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VPCMPISTRIRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpistriEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpistri(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VPCMPISTRIRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPCMPISTRM` (VPCMPISTRM). 
/// The instruction compares data from two strings based on the encoded value in the imm8 byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”) generating a mask stored to XMM0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPISTRM.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Mem, Imm |
/// | 2 | Xmm, Xmm, Imm |
/// +---+---------------+
/// ```
pub trait VpcmpistrmEmitter<A, B, C> {
    fn vpcmpistrm(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpcmpistrmEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vpcmpistrm(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VPCMPISTRMRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpcmpistrmEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vpcmpistrm(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VPCMPISTRMRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPERM2F128` (VPERM2F128). 
/// Permute 128 bit floating-point-containing fields from the first source operand (second operand) and second source operand (third operand) using bits in the 8-bit immediate and store results in the destination operand (first operand). The first source operand is a YMM register, the second source operand is a YMM register or a 256-bit memory location, and the destination operand is a YMM register.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPERM2F128.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Ymm, Ymm, Mem, Imm |
/// | 2 | Ymm, Ymm, Ymm, Imm |
/// +---+--------------------+
/// ```
pub trait Vperm2f128Emitter<A, B, C, D> {
    fn vperm2f128(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> Vperm2f128Emitter<Ymm, Ymm, Ymm, Imm> for Assembler<'a> {
    fn vperm2f128(&mut self, op0: Ymm, op1: Ymm, op2: Ymm, op3: Imm) {
        self.emit(VPERM2F128_256RRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> Vperm2f128Emitter<Ymm, Ymm, Mem, Imm> for Assembler<'a> {
    fn vperm2f128(&mut self, op0: Ymm, op1: Ymm, op2: Mem, op3: Imm) {
        self.emit(VPERM2F128_256RRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VPEXTRD` (VPEXTRD). 
/// Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Xmm, Imm |
/// | 2 | Mem, Xmm, Imm |
/// +---+---------------+
/// ```
pub trait VpextrdEmitter<A, B, C> {
    fn vpextrd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpextrdEmitter<Gpd, Xmm, Imm> for Assembler<'a> {
    fn vpextrd(&mut self, op0: Gpd, op1: Xmm, op2: Imm) {
        self.emit(VPEXTRDRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpextrdEmitter<Mem, Xmm, Imm> for Assembler<'a> {
    fn vpextrd(&mut self, op0: Mem, op1: Xmm, op2: Imm) {
        self.emit(VPEXTRDMRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPEXTRQ` (VPEXTRQ). 
/// Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Gpd, Xmm, Imm |
/// | 2 | Gpq, Xmm, Imm |
/// | 3 | Mem, Xmm, Imm |
/// +---+---------------+
/// ```
pub trait VpextrqEmitter<A, B, C> {
    fn vpextrq(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpextrqEmitter<Gpd, Xmm, Imm> for Assembler<'a> {
    fn vpextrq(&mut self, op0: Gpd, op1: Xmm, op2: Imm) {
        self.emit(VPEXTRQRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpextrqEmitter<Mem, Xmm, Imm> for Assembler<'a> {
    fn vpextrq(&mut self, op0: Mem, op1: Xmm, op2: Imm) {
        self.emit(VPEXTRQMRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpextrqEmitter<Gpq, Xmm, Imm> for Assembler<'a> {
    fn vpextrq(&mut self, op0: Gpq, op1: Xmm, op2: Imm) {
        self.emit(VPEXTRQRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPHADDD` (VPHADDD). 
/// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
/// +---+---------------+
/// ```
pub trait VphadddEmitter<A, B, C> {
    fn vphaddd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VphadddEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vphaddd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPHADDD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphadddEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vphaddd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPHADDD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphadddEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vphaddd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPHADDD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphadddEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vphaddd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPHADDD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPHADDSW` (VPHADDSW). 
/// (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDSW.html).
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
/// +---+---------------+
/// ```
pub trait VphaddswEmitter<A, B, C> {
    fn vphaddsw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VphaddswEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vphaddsw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPHADDSW128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphaddswEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vphaddsw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPHADDSW128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphaddswEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vphaddsw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPHADDSW256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphaddswEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vphaddsw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPHADDSW256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPHADDW` (VPHADDW). 
/// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
/// +---+---------------+
/// ```
pub trait VphaddwEmitter<A, B, C> {
    fn vphaddw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VphaddwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vphaddw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPHADDW128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphaddwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vphaddw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPHADDW128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphaddwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vphaddw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPHADDW256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphaddwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vphaddw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPHADDW256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPHMINPOSUW` (VPHMINPOSUW). 
/// Determine the minimum unsigned word value in the source operand (second operand) and place the unsigned word in the low word (bits 0-15) of the destination operand (first operand). The word index of the minimum value is stored in bits 16-18 of the destination operand. The remaining upper bits of the destination are set to zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHMINPOSUW.html).
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
pub trait VphminposuwEmitter<A, B> {
    fn vphminposuw(&mut self, op0: A, op1: B);
}

impl<'a> VphminposuwEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vphminposuw(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VPHMINPOSUW128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VphminposuwEmitter<Xmm, Mem> for Assembler<'a> {
    fn vphminposuw(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VPHMINPOSUW128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPHSUBD` (VPHSUBD). 
/// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
/// +---+---------------+
/// ```
pub trait VphsubdEmitter<A, B, C> {
    fn vphsubd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VphsubdEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vphsubd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPHSUBD128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphsubdEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vphsubd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPHSUBD128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphsubdEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vphsubd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPHSUBD256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphsubdEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vphsubd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPHSUBD256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPHSUBSW` (VPHSUBSW). 
/// (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBSW.html).
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
/// +---+---------------+
/// ```
pub trait VphsubswEmitter<A, B, C> {
    fn vphsubsw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VphsubswEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vphsubsw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPHSUBSW128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphsubswEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vphsubsw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPHSUBSW128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphsubswEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vphsubsw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPHSUBSW256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphsubswEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vphsubsw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPHSUBSW256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPHSUBW` (VPHSUBW). 
/// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
/// +---+---------------+
/// ```
pub trait VphsubwEmitter<A, B, C> {
    fn vphsubw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VphsubwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vphsubw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPHSUBW128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphsubwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vphsubw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPHSUBW128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphsubwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vphsubw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPHSUBW256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VphsubwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vphsubw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPHSUBW256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPINSRD` (VPINSRD). 
/// Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Gpd, Imm |
/// | 2 | Xmm, Xmm, Mem, Imm |
/// +---+--------------------+
/// ```
pub trait VpinsrdEmitter<A, B, C, D> {
    fn vpinsrd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpinsrdEmitter<Xmm, Xmm, Gpd, Imm> for Assembler<'a> {
    fn vpinsrd(&mut self, op0: Xmm, op1: Xmm, op2: Gpd, op3: Imm) {
        self.emit(VPINSRDRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpinsrdEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpinsrd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VPINSRDRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VPINSRQ` (VPINSRQ). 
/// Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+--------------------+
/// | # | Operands           |
/// +---+--------------------+
/// | 1 | Xmm, Xmm, Gpd, Imm |
/// | 2 | Xmm, Xmm, Gpq, Imm |
/// | 3 | Xmm, Xmm, Mem, Imm |
/// +---+--------------------+
/// ```
pub trait VpinsrqEmitter<A, B, C, D> {
    fn vpinsrq(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VpinsrqEmitter<Xmm, Xmm, Gpd, Imm> for Assembler<'a> {
    fn vpinsrq(&mut self, op0: Xmm, op1: Xmm, op2: Gpd, op3: Imm) {
        self.emit(VPINSRQRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpinsrqEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vpinsrq(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VPINSRQRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VpinsrqEmitter<Xmm, Xmm, Gpq, Imm> for Assembler<'a> {
    fn vpinsrq(&mut self, op0: Xmm, op1: Xmm, op2: Gpq, op3: Imm) {
        self.emit(VPINSRQRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VPMOVMSKB` (VPMOVMSKB). 
/// Creates a mask made up of the most significant bit of each byte of the source operand (second operand) and stores the result in the low byte or word of the destination operand (first operand).
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVMSKB.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Xmm |
/// | 2 | Gpd, Ymm |
/// +---+----------+
/// ```
pub trait VpmovmskbEmitter<A, B> {
    fn vpmovmskb(&mut self, op0: A, op1: B);
}

impl<'a> VpmovmskbEmitter<Gpd, Xmm> for Assembler<'a> {
    fn vpmovmskb(&mut self, op0: Gpd, op1: Xmm) {
        self.emit(VPMOVMSKB128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VpmovmskbEmitter<Gpd, Ymm> for Assembler<'a> {
    fn vpmovmskb(&mut self, op0: Gpd, op1: Ymm) {
        self.emit(VPMOVMSKB256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPOR` (VPOR). 
/// Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/POR.html).
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
/// +---+---------------+
/// ```
pub trait VporEmitter<A, B, C> {
    fn vpor(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VporEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpor(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPOR128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VporEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpor(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPOR128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VporEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpor(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPOR256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VporEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpor(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPOR256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPSIGNB` (VPSIGNB). 
/// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
/// +---+---------------+
/// ```
pub trait VpsignbEmitter<A, B, C> {
    fn vpsignb(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsignbEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsignb(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPSIGNB128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpsignbEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsignb(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPSIGNB128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpsignbEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsignb(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPSIGNB256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpsignbEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsignb(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPSIGNB256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPSIGND` (VPSIGND). 
/// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
/// +---+---------------+
/// ```
pub trait VpsigndEmitter<A, B, C> {
    fn vpsignd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsigndEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsignd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPSIGND128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpsigndEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsignd(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPSIGND128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpsigndEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsignd(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPSIGND256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpsigndEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsignd(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPSIGND256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPSIGNW` (VPSIGNW). 
/// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
/// +---+---------------+
/// ```
pub trait VpsignwEmitter<A, B, C> {
    fn vpsignw(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpsignwEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpsignw(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPSIGNW128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpsignwEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpsignw(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPSIGNW128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpsignwEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpsignw(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPSIGNW256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpsignwEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpsignw(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPSIGNW256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VPTEST` (VPTEST). 
/// PTEST and VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND of the first source operand (first operand) and the second source operand (second operand). VPTEST sets the CF flag if all bits in the result are 0 of the bitwise AND of the second source operand (second operand) and the logical NOT of the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PTEST.html).
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
/// +---+----------+
/// ```
pub trait VptestEmitter<A, B> {
    fn vptest(&mut self, op0: A, op1: B);
}

impl<'a> VptestEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vptest(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VPTEST128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VptestEmitter<Xmm, Mem> for Assembler<'a> {
    fn vptest(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VPTEST128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VptestEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vptest(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VPTEST256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VptestEmitter<Ymm, Mem> for Assembler<'a> {
    fn vptest(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VPTEST256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VPXOR` (VPXOR). 
/// Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PXOR.html).
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
/// +---+---------------+
/// ```
pub trait VpxorEmitter<A, B, C> {
    fn vpxor(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VpxorEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vpxor(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VPXOR128RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpxorEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vpxor(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VPXOR128RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpxorEmitter<Ymm, Ymm, Ymm> for Assembler<'a> {
    fn vpxor(&mut self, op0: Ymm, op1: Ymm, op2: Ymm) {
        self.emit(VPXOR256RRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VpxorEmitter<Ymm, Ymm, Mem> for Assembler<'a> {
    fn vpxor(&mut self, op0: Ymm, op1: Ymm, op2: Mem) {
        self.emit(VPXOR256RRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VRCPPS` (VRCPPS). 
/// Performs a SIMD computation of the approximate reciprocals of the four packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RCPPS.html).
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
/// +---+----------+
/// ```
pub trait VrcppsEmitter<A, B> {
    fn vrcpps(&mut self, op0: A, op1: B);
}

impl<'a> VrcppsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vrcpps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VRCPPS128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VrcppsEmitter<Xmm, Mem> for Assembler<'a> {
    fn vrcpps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VRCPPS128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VrcppsEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vrcpps(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VRCPPS256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VrcppsEmitter<Ymm, Mem> for Assembler<'a> {
    fn vrcpps(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VRCPPS256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VRCPSS` (VRCPSS). 
/// Computes of an approximate reciprocal of the low single precision floating-point value in the source operand (second operand) and stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RCPSS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// +---+---------------+
/// ```
pub trait VrcpssEmitter<A, B, C> {
    fn vrcpss(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VrcpssEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vrcpss(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VRCPSSRRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VrcpssEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vrcpss(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VRCPSSRRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VROUNDPD` (VROUNDPD). 
/// Round the 2 double precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a double precision floating-point value.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDPD.html).
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
/// +---+---------------+
/// ```
pub trait VroundpdEmitter<A, B, C> {
    fn vroundpd(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VroundpdEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vroundpd(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VROUNDPD128RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VroundpdEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vroundpd(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VROUNDPD128RMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VroundpdEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vroundpd(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(VROUNDPD256RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VroundpdEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vroundpd(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(VROUNDPD256RMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VROUNDPS` (VROUNDPS). 
/// Round the 4 single precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a single precision floating-point value.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDPS.html).
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
/// +---+---------------+
/// ```
pub trait VroundpsEmitter<A, B, C> {
    fn vroundps(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VroundpsEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn vroundps(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(VROUNDPS128RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VroundpsEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn vroundps(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(VROUNDPS128RMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VroundpsEmitter<Ymm, Ymm, Imm> for Assembler<'a> {
    fn vroundps(&mut self, op0: Ymm, op1: Ymm, op2: Imm) {
        self.emit(VROUNDPS256RRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VroundpsEmitter<Ymm, Mem, Imm> for Assembler<'a> {
    fn vroundps(&mut self, op0: Ymm, op1: Mem, op2: Imm) {
        self.emit(VROUNDPS256RMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VROUNDSD` (VROUNDSD). 
/// Round the double precision floating-point value in the lower qword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a double precision floating-point input to an integer value and returns the integer result as a double precision floating-point value in the lowest position. The upper double precision floating-point value in the destination is retained.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDSD.html).
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
pub trait VroundsdEmitter<A, B, C, D> {
    fn vroundsd(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VroundsdEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vroundsd(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VROUNDSDRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VroundsdEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vroundsd(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VROUNDSDRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VROUNDSS` (VROUNDSS). 
/// Round the single precision floating-point value in the lowest dword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a single precision floating-point input to an integer value and returns the result as a single precision floating-point value in the lowest position. The upper three single precision floating-point values in the destination are retained.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDSS.html).
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
pub trait VroundssEmitter<A, B, C, D> {
    fn vroundss(&mut self, op0: A, op1: B, op2: C, op3: D);
}

impl<'a> VroundssEmitter<Xmm, Xmm, Xmm, Imm> for Assembler<'a> {
    fn vroundss(&mut self, op0: Xmm, op1: Xmm, op2: Xmm, op3: Imm) {
        self.emit(VROUNDSSRRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

impl<'a> VroundssEmitter<Xmm, Xmm, Mem, Imm> for Assembler<'a> {
    fn vroundss(&mut self, op0: Xmm, op1: Xmm, op2: Mem, op3: Imm) {
        self.emit(VROUNDSSRRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), op3.as_operand());
    }
}

/// `VRSQRTPS` (VRSQRTPS). 
/// Performs a SIMD computation of the approximate reciprocals of the square roots of the four packed single precision floating-point values in the source operand (second operand) and stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RSQRTPS.html).
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
/// +---+----------+
/// ```
pub trait VrsqrtpsEmitter<A, B> {
    fn vrsqrtps(&mut self, op0: A, op1: B);
}

impl<'a> VrsqrtpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vrsqrtps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VRSQRTPS128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VrsqrtpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn vrsqrtps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VRSQRTPS128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VrsqrtpsEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vrsqrtps(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VRSQRTPS256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VrsqrtpsEmitter<Ymm, Mem> for Assembler<'a> {
    fn vrsqrtps(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VRSQRTPS256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VRSQRTSS` (VRSQRTSS). 
/// Computes an approximate reciprocal of the square root of the low single precision floating-point value in the source operand (second operand) stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RSQRTSS.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+---------------+
/// | # | Operands      |
/// +---+---------------+
/// | 1 | Xmm, Xmm, Mem |
/// | 2 | Xmm, Xmm, Xmm |
/// +---+---------------+
/// ```
pub trait VrsqrtssEmitter<A, B, C> {
    fn vrsqrtss(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> VrsqrtssEmitter<Xmm, Xmm, Xmm> for Assembler<'a> {
    fn vrsqrtss(&mut self, op0: Xmm, op1: Xmm, op2: Xmm) {
        self.emit(VRSQRTSSRRR, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> VrsqrtssEmitter<Xmm, Xmm, Mem> for Assembler<'a> {
    fn vrsqrtss(&mut self, op0: Xmm, op1: Xmm, op2: Mem) {
        self.emit(VRSQRTSSRRM, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

/// `VSTMXCSR` (VSTMXCSR). 
/// Stores the contents of the MXCSR control and status register to the destination operand. The destination operand is a 32-bit memory location. The reserved bits in the MXCSR register are stored as 0s.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/STMXCSR.html).
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
pub trait VstmxcsrEmitter<A> {
    fn vstmxcsr(&mut self, op0: A);
}

impl<'a> VstmxcsrEmitter<Mem> for Assembler<'a> {
    fn vstmxcsr(&mut self, op0: Mem) {
        self.emit(VSTMXCSRM, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `VTESTPD` (VTESTPD). 
/// VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html).
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
/// +---+----------+
/// ```
pub trait VtestpdEmitter<A, B> {
    fn vtestpd(&mut self, op0: A, op1: B);
}

impl<'a> VtestpdEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vtestpd(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VTESTPD128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VtestpdEmitter<Xmm, Mem> for Assembler<'a> {
    fn vtestpd(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VTESTPD128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VtestpdEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vtestpd(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VTESTPD256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VtestpdEmitter<Ymm, Mem> for Assembler<'a> {
    fn vtestpd(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VTESTPD256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VTESTPS` (VTESTPS). 
/// VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html).
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
/// +---+----------+
/// ```
pub trait VtestpsEmitter<A, B> {
    fn vtestps(&mut self, op0: A, op1: B);
}

impl<'a> VtestpsEmitter<Xmm, Xmm> for Assembler<'a> {
    fn vtestps(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(VTESTPS128RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VtestpsEmitter<Xmm, Mem> for Assembler<'a> {
    fn vtestps(&mut self, op0: Xmm, op1: Mem) {
        self.emit(VTESTPS128RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VtestpsEmitter<Ymm, Ymm> for Assembler<'a> {
    fn vtestps(&mut self, op0: Ymm, op1: Ymm) {
        self.emit(VTESTPS256RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> VtestpsEmitter<Ymm, Mem> for Assembler<'a> {
    fn vtestps(&mut self, op0: Ymm, op1: Mem) {
        self.emit(VTESTPS256RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `VZEROALL` (VZEROALL). 
/// In 64-bit mode, the instruction zeroes XMM0-XMM15, YMM0-YMM15, and ZMM0-ZMM15. Outside 64-bit mode, it zeroes only XMM0-XMM7, YMM0-YMM7, and ZMM0-ZMM7. VZEROALL does not modify ZMM16-ZMM31.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VZEROALL.html).
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
pub trait VzeroallEmitter {
    fn vzeroall(&mut self);
}

impl<'a> VzeroallEmitter for Assembler<'a> {
    fn vzeroall(&mut self) {
        self.emit(VZEROALL, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `VZEROUPPER` (VZEROUPPER). 
/// In 64-bit mode, the instruction zeroes the bits in positions 128 and higher in YMM0-YMM15 and ZMM0-ZMM15. Outside 64-bit mode, it zeroes those bits only in YMM0-YMM7 and ZMM0-ZMM7. VZEROUPPER does not modify the lower 128 bits of these registers and it does not modify ZMM16-ZMM31.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VZEROUPPER.html).
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
pub trait VzeroupperEmitter {
    fn vzeroupper(&mut self);
}

impl<'a> VzeroupperEmitter for Assembler<'a> {
    fn vzeroupper(&mut self) {
        self.emit(VZEROUPPER, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `VADDSUBPD` (VADDSUBPD). 
    /// Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the corresponding double precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered double precision floating-point values from the second source operand from the corresponding double precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSUBPD.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vaddsubpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VaddsubpdEmitter<A, B, C> {
        <Self as VaddsubpdEmitter<A, B, C>>::vaddsubpd(self, op0, op1, op2);
    }
    /// `VADDSUBPS` (VADDSUBPS). 
    /// Adds odd-numbered single precision floating-point values of the first source operand (second operand) with the corresponding single precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered single precision floating-point values from the second source operand from the corresponding single precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ADDSUBPS.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vaddsubps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VaddsubpsEmitter<A, B, C> {
        <Self as VaddsubpsEmitter<A, B, C>>::vaddsubps(self, op0, op1, op2);
    }
    /// `VBLENDPD` (VBLENDPD). 
    /// Double-precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [3:0] determine whether the corresponding double precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is ”1”, then the double precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDPD.html).
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
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vblendpd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VblendpdEmitter<A, B, C, D> {
        <Self as VblendpdEmitter<A, B, C, D>>::vblendpd(self, op0, op1, op2, op3);
    }
    /// `VBLENDPS` (VBLENDPS). 
    /// Packed single precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand). The immediate bits [7:0] determine whether the corresponding single precision floating-point value in the destination is copied from the second source or first source. If a bit in the mask, corresponding to a word, is “1”, then the single precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDPS.html).
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
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vblendps<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VblendpsEmitter<A, B, C, D> {
        <Self as VblendpsEmitter<A, B, C, D>>::vblendps(self, op0, op1, op2, op3);
    }
    /// `VBLENDVPD` (VBLENDVPD). 
    /// Conditionally copy each quadword data element of double precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each quadword element of the mask register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDVPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Xmm |
    /// | 2 | Xmm, Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem, Ymm |
    /// | 4 | Ymm, Ymm, Ymm, Ymm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vblendvpd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VblendvpdEmitter<A, B, C, D> {
        <Self as VblendvpdEmitter<A, B, C, D>>::vblendvpd(self, op0, op1, op2, op3);
    }
    /// `VBLENDVPS` (VBLENDVPS). 
    /// Conditionally copy each dword data element of single precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand. The mask bits are the most significant bit in each dword element of the mask register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/BLENDVPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Xmm |
    /// | 2 | Xmm, Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem, Ymm |
    /// | 4 | Ymm, Ymm, Ymm, Ymm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vblendvps<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VblendvpsEmitter<A, B, C, D> {
        <Self as VblendvpsEmitter<A, B, C, D>>::vblendvps(self, op0, op1, op2, op3);
    }
    /// `VBROADCASTF128` (VBROADCASTF128). 
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
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vbroadcastf128<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: Vbroadcastf128Emitter<A, B> {
        <Self as Vbroadcastf128Emitter<A, B>>::vbroadcastf128(self, op0, op1);
    }
    /// `VCMPPD` (VCMPPD). 
    /// Performs a SIMD compare of the packed double precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each pair of packed values in the two source operands.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+---------------------+
    /// | #  | Operands            |
    /// +----+---------------------+
    /// | 1  | KReg, Xmm, Mem, Imm |
    /// | 2  | KReg, Xmm, Xmm, Imm |
    /// | 3  | KReg, Ymm, Mem, Imm |
    /// | 4  | KReg, Ymm, Ymm, Imm |
    /// | 5  | KReg, Zmm, Mem, Imm |
    /// | 6  | KReg, Zmm, Zmm, Imm |
    /// | 7  | Xmm, Xmm, Mem, Imm  |
    /// | 8  | Xmm, Xmm, Xmm, Imm  |
    /// | 9  | Ymm, Ymm, Mem, Imm  |
    /// | 10 | Ymm, Ymm, Ymm, Imm  |
    /// +----+---------------------+
    /// ```
    #[inline]
    pub fn vcmppd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VcmppdEmitter<A, B, C, D> {
        <Self as VcmppdEmitter<A, B, C, D>>::vcmppd(self, op0, op1, op2, op3);
    }
    /// `VCMPPS` (VCMPPS). 
    /// Performs a SIMD compare of the packed single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate byte) specifies the type of comparison performed on each of the pairs of packed values.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+---------------------+
    /// | #  | Operands            |
    /// +----+---------------------+
    /// | 1  | KReg, Xmm, Mem, Imm |
    /// | 2  | KReg, Xmm, Xmm, Imm |
    /// | 3  | KReg, Ymm, Mem, Imm |
    /// | 4  | KReg, Ymm, Ymm, Imm |
    /// | 5  | KReg, Zmm, Mem, Imm |
    /// | 6  | KReg, Zmm, Zmm, Imm |
    /// | 7  | Xmm, Xmm, Mem, Imm  |
    /// | 8  | Xmm, Xmm, Xmm, Imm  |
    /// | 9  | Ymm, Ymm, Mem, Imm  |
    /// | 10 | Ymm, Ymm, Ymm, Imm  |
    /// +----+---------------------+
    /// ```
    #[inline]
    pub fn vcmpps<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VcmppsEmitter<A, B, C, D> {
        <Self as VcmppsEmitter<A, B, C, D>>::vcmpps(self, op0, op1, op2, op3);
    }
    /// `VCMPSD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------------+
    /// | # | Operands            |
    /// +---+---------------------+
    /// | 1 | KReg, Xmm, Mem, Imm |
    /// | 2 | KReg, Xmm, Xmm, Imm |
    /// | 3 | Xmm, Xmm, Mem, Imm  |
    /// | 4 | Xmm, Xmm, Xmm, Imm  |
    /// +---+---------------------+
    /// ```
    #[inline]
    pub fn vcmpsd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VcmpsdEmitter<A, B, C, D> {
        <Self as VcmpsdEmitter<A, B, C, D>>::vcmpsd(self, op0, op1, op2, op3);
    }
    /// `VCMPSS` (VCMPSS). 
    /// Compares the low single precision floating-point values in the second source operand and the first source operand and returns the result of the comparison to the destination operand. The comparison predicate operand (immediate operand) specifies the type of comparison performed.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/CMPSS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------------+
    /// | # | Operands            |
    /// +---+---------------------+
    /// | 1 | KReg, Xmm, Mem, Imm |
    /// | 2 | KReg, Xmm, Xmm, Imm |
    /// | 3 | Xmm, Xmm, Mem, Imm  |
    /// | 4 | Xmm, Xmm, Xmm, Imm  |
    /// +---+---------------------+
    /// ```
    #[inline]
    pub fn vcmpss<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VcmpssEmitter<A, B, C, D> {
        <Self as VcmpssEmitter<A, B, C, D>>::vcmpss(self, op0, op1, op2, op3);
    }
    /// `VDPPD` (VDPPD). 
    /// Conditionally multiplies the packed double precision floating-point values in the destination operand (first operand) with the packed double precision floating-point values in the source (second operand) depending on a mask extracted from bits [5:4] of the immediate operand (third operand). If a condition mask bit is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DPPD.html).
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
    pub fn vdppd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VdppdEmitter<A, B, C, D> {
        <Self as VdppdEmitter<A, B, C, D>>::vdppd(self, op0, op1, op2, op3);
    }
    /// `VDPPS` (VDPPS). 
    /// Conditionally multiplies the packed single precision floating-point values in the destination operand (first operand) with the packed single precision floats in the source (second operand) depending on a mask extracted from the high 4 bits of the immediate byte (third operand). If a condition mask bit in imm8[7:4] is zero, the corresponding multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/DPPS.html).
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
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vdpps<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VdppsEmitter<A, B, C, D> {
        <Self as VdppsEmitter<A, B, C, D>>::vdpps(self, op0, op1, op2, op3);
    }
    /// `VEXTRACTF128` (VEXTRACTF128). 
    /// VEXTRACTF128/VEXTRACTF32x4 and VEXTRACTF64x2 extract 128-bits of single precision floating-point values from the source operand (the second operand) and store to the low 128-bit of the destination operand (the first operand). The 128-bit data extraction occurs at an 128-bit granular offset specified by imm8[0] (256-bit) or imm8[1:0] as the multiply factor. The destination may be either a vector register or an 128-bit memory location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VEXTRACTF128%3AVEXTRACTF32x4%3AVEXTRACTF64x2%3AVEXTRACTF32x8%3AVEXTRACTF64x4.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Ymm, Imm |
    /// | 2 | Xmm, Ymm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vextractf128<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: Vextractf128Emitter<A, B, C> {
        <Self as Vextractf128Emitter<A, B, C>>::vextractf128(self, op0, op1, op2);
    }
    /// `VHADDPD` (VHADDPD). 
    /// Adds the double precision floating-point values in the high and low quadwords of the destination operand and stores the result in the low quadword of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HADDPD.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vhaddpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VhaddpdEmitter<A, B, C> {
        <Self as VhaddpdEmitter<A, B, C>>::vhaddpd(self, op0, op1, op2);
    }
    /// `VHADDPS` (VHADDPS). 
    /// Adds the single precision floating-point values in the first and second dwords of the destination operand and stores the result in the first dword of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HADDPS.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vhaddps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VhaddpsEmitter<A, B, C> {
        <Self as VhaddpsEmitter<A, B, C>>::vhaddps(self, op0, op1, op2);
    }
    /// `VHSUBPD` (VHSUBPD). 
    /// The HSUBPD instruction subtracts horizontally the packed double precision floating-point numbers of both operands.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HSUBPD.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vhsubpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VhsubpdEmitter<A, B, C> {
        <Self as VhsubpdEmitter<A, B, C>>::vhsubpd(self, op0, op1, op2);
    }
    /// `VHSUBPS` (VHSUBPS). 
    /// Subtracts the single precision floating-point value in the second dword of the destination operand from the first dword of the destination operand and stores the result in the first dword of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/HSUBPS.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vhsubps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VhsubpsEmitter<A, B, C> {
        <Self as VhsubpsEmitter<A, B, C>>::vhsubps(self, op0, op1, op2);
    }
    /// `VINSERTF128` (VINSERTF128). 
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
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vinsertf128<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vinsertf128Emitter<A, B, C, D> {
        <Self as Vinsertf128Emitter<A, B, C, D>>::vinsertf128(self, op0, op1, op2, op3);
    }
    /// `VLDDQU` (VLDDQU). 
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
    /// | 2 | Ymm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vlddqu<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VlddquEmitter<A, B> {
        <Self as VlddquEmitter<A, B>>::vlddqu(self, op0, op1);
    }
    /// `VLDMXCSR` (VLDMXCSR). 
    /// Loads the source operand into the MXCSR control/status register. The source operand is a 32-bit memory location. See “MXCSR Control and Status Register” in Chapter 10, of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for a description of the MXCSR register and its contents.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/LDMXCSR.html).
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
    pub fn vldmxcsr<A>(&mut self, op0: A)
    where Assembler<'a>: VldmxcsrEmitter<A> {
        <Self as VldmxcsrEmitter<A>>::vldmxcsr(self, op0);
    }
    /// `VMASKMOVDQU` (VMASKMOVDQU). 
    /// Stores selected bytes from the source operand (first operand) into an 128-bit memory location. The mask operand (second operand) selects which bytes from the source operand are written to memory. The source and mask operands are XMM registers. The memory location specified by the effective address in the DI/EDI/RDI register (the default segment register is DS, but this may be overridden with a segment-override prefix). The memory location does not need to be aligned on a natural boundary. (The size of the store address depends on the address-size attribute.)
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MASKMOVDQU.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmaskmovdqu<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmaskmovdquEmitter<A, B> {
        <Self as VmaskmovdquEmitter<A, B>>::vmaskmovdqu(self, op0, op1);
    }
    /// `VMASKMOVPD` (VMASKMOVPD). 
    /// Conditionally moves packed data elements from the second source operand into the corresponding data element of the destination operand, depending on the mask bits associated with each data element. The mask bits are specified in the first source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VMASKMOV.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Xmm, Xmm |
    /// | 2 | Mem, Ymm, Ymm |
    /// | 3 | Xmm, Xmm, Mem |
    /// | 4 | Ymm, Ymm, Mem |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vmaskmovpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VmaskmovpdEmitter<A, B, C> {
        <Self as VmaskmovpdEmitter<A, B, C>>::vmaskmovpd(self, op0, op1, op2);
    }
    /// `VMASKMOVPS` (VMASKMOVPS). 
    /// Conditionally moves packed data elements from the second source operand into the corresponding data element of the destination operand, depending on the mask bits associated with each data element. The mask bits are specified in the first source operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VMASKMOV.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Mem, Xmm, Xmm |
    /// | 2 | Mem, Ymm, Ymm |
    /// | 3 | Xmm, Xmm, Mem |
    /// | 4 | Ymm, Ymm, Mem |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vmaskmovps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VmaskmovpsEmitter<A, B, C> {
        <Self as VmaskmovpsEmitter<A, B, C>>::vmaskmovps(self, op0, op1, op2);
    }
    /// `VMOVD` (VMOVD). 
    /// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Xmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmovdEmitter<A, B> {
        <Self as VmovdEmitter<A, B>>::vmovd(self, op0, op1);
    }
    /// `VMOVDQA` (VMOVDQA). 
    /// Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDQA%3AVMOVDQA32%3AVMOVDQA64.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Xmm, Mem |
    /// | 4 | Xmm, Xmm |
    /// | 5 | Ymm, Mem |
    /// | 6 | Ymm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovdqa<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmovdqaEmitter<A, B> {
        <Self as VmovdqaEmitter<A, B>>::vmovdqa(self, op0, op1);
    }
    /// `VMOVDQU` (VMOVDQU). 
    /// Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVDQU%3AVMOVDQU8%3AVMOVDQU16%3AVMOVDQU32%3AVMOVDQU64.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Mem, Xmm |
    /// | 2 | Mem, Ymm |
    /// | 3 | Xmm, Mem |
    /// | 4 | Xmm, Xmm |
    /// | 5 | Ymm, Mem |
    /// | 6 | Ymm, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovdqu<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmovdquEmitter<A, B> {
        <Self as VmovdquEmitter<A, B>>::vmovdqu(self, op0, op1);
    }
    /// `VMOVD_G2X` (VMOVD). 
    /// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Gpd |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovd_g2x<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmovdG2xEmitter<A, B> {
        <Self as VmovdG2xEmitter<A, B>>::vmovd_g2x(self, op0, op1);
    }
    /// `VMOVD_X2G` (VMOVD). 
    /// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovd_x2g<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmovdX2gEmitter<A, B> {
        <Self as VmovdX2gEmitter<A, B>>::vmovd_x2g(self, op0, op1);
    }
    /// `VMOVMSKPD` (VMOVMSKPD). 
    /// Extracts the sign bits from the packed double precision floating-point values in the source operand (second operand), formats them into a 2-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM register, and the destination operand is a general-purpose register. The mask is stored in the 2 low-order bits of the destination operand. Zero-extend the upper bits of the destination.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVMSKPD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Xmm |
    /// | 2 | Gpd, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovmskpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmovmskpdEmitter<A, B> {
        <Self as VmovmskpdEmitter<A, B>>::vmovmskpd(self, op0, op1);
    }
    /// `VMOVMSKPS` (VMOVMSKPS). 
    /// Extracts the sign bits from the packed single precision floating-point values in the source operand (second operand), formats them into a 4- or 8-bit mask, and stores the mask in the destination operand (first operand). The source operand is an XMM or YMM register, and the destination operand is a general-purpose register. The mask is stored in the 4 or 8 low-order bits of the destination operand. The upper bits of the destination operand beyond the mask are filled with zeros.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVMSKPS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Xmm |
    /// | 2 | Gpd, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovmskps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmovmskpsEmitter<A, B> {
        <Self as VmovmskpsEmitter<A, B>>::vmovmskps(self, op0, op1);
    }
    /// `VMOVQ_G2X` (VMOVQ). 
    /// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Xmm, Gpd |
    /// | 2 | Xmm, Gpq |
    /// | 3 | Xmm, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovq_g2x<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmovqG2xEmitter<A, B> {
        <Self as VmovqG2xEmitter<A, B>>::vmovq_g2x(self, op0, op1);
    }
    /// `VMOVQ_X2G` (VMOVQ). 
    /// Copies a doubleword from the source operand (second operand) to the destination operand (first operand). The source and destination operands can be general-purpose registers, MMX technology registers, XMM registers, or 32-bit memory locations. This instruction can be used to move a doubleword to and from the low doubleword of an MMX technology register and a general-purpose register or a 32-bit memory location, or to and from the low doubleword of an XMM register and a general-purpose register or a 32-bit memory location. The instruction cannot be used to transfer data between MMX technology registers, between XMM registers, between general-purpose registers, or between memory locations.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MOVD%3AMOVQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Xmm |
    /// | 2 | Gpq, Xmm |
    /// | 3 | Mem, Xmm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vmovq_x2g<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VmovqX2gEmitter<A, B> {
        <Self as VmovqX2gEmitter<A, B>>::vmovq_x2g(self, op0, op1);
    }
    /// `VMPSADBW` (VMPSADBW). 
    /// (V)MPSADBW calculates packed word results of sum-absolute-difference (SAD) of unsigned bytes from two blocks of 32-bit dword elements, using two select fields in the immediate byte to select the offsets of the two blocks within the first source operand and the second operand. Packed SAD word results are calculated within each 128-bit lane. Each SAD word result is calculated between a stationary block_2 (whose offset within the second source operand is selected by a two bit select control, multiplied by 32 bits) and a sliding block_1 at consecutive byte-granular position within the first source operand. The offset of the first 32-bit block of block_1 is selectable using a one bit select control, multiplied by 32 bits.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/MPSADBW.html).
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
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vmpsadbw<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VmpsadbwEmitter<A, B, C, D> {
        <Self as VmpsadbwEmitter<A, B, C, D>>::vmpsadbw(self, op0, op1, op2, op3);
    }
    /// `VPAND` (VPAND). 
    /// Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PAND.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpand<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpandEmitter<A, B, C> {
        <Self as VpandEmitter<A, B, C>>::vpand(self, op0, op1, op2);
    }
    /// `VPANDN` (VPANDN). 
    /// Performs a bitwise logical NOT operation on the first source operand, then performs bitwise AND with second source operand and stores the result in the destination operand. Each bit of the result is set to 1 if the corresponding bit in the first operand is 0 and the corresponding bit in the second operand is 1, otherwise it is set to 0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PANDN.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpandn<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpandnEmitter<A, B, C> {
        <Self as VpandnEmitter<A, B, C>>::vpandn(self, op0, op1, op2);
    }
    /// `VPBLENDVB` (VPBLENDVB). 
    /// Conditionally copies byte elements from the source operand (second operand) to the destination operand (first operand) depending on mask bits defined in the implicit third register argument, XMM0. The mask bits are the most significant bit in each byte element of the XMM0 register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PBLENDVB.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Mem, Xmm |
    /// | 2 | Xmm, Xmm, Xmm, Xmm |
    /// | 3 | Ymm, Ymm, Mem, Ymm |
    /// | 4 | Ymm, Ymm, Ymm, Ymm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vpblendvb<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VpblendvbEmitter<A, B, C, D> {
        <Self as VpblendvbEmitter<A, B, C, D>>::vpblendvb(self, op0, op1, op2, op3);
    }
    /// `VPBLENDW` (VPBLENDW). 
    /// Words from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand). The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source. If a bit in the mask, corresponding to a word, is “1", then the word is copied, else the word element in the destination operand is unchanged.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PBLENDW.html).
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
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vpblendw<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VpblendwEmitter<A, B, C, D> {
        <Self as VpblendwEmitter<A, B, C, D>>::vpblendw(self, op0, op1, op2, op3);
    }
    /// `VPCMPEQB` (VPCMPEQB). 
    /// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+----------------+
    /// | #  | Operands       |
    /// +----+----------------+
    /// | 1  | KReg, Xmm, Mem |
    /// | 2  | KReg, Xmm, Xmm |
    /// | 3  | KReg, Ymm, Mem |
    /// | 4  | KReg, Ymm, Ymm |
    /// | 5  | KReg, Zmm, Mem |
    /// | 6  | KReg, Zmm, Zmm |
    /// | 7  | Xmm, Xmm, Mem  |
    /// | 8  | Xmm, Xmm, Xmm  |
    /// | 9  | Ymm, Ymm, Mem  |
    /// | 10 | Ymm, Ymm, Ymm  |
    /// +----+----------------+
    /// ```
    #[inline]
    pub fn vpcmpeqb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpeqbEmitter<A, B, C> {
        <Self as VpcmpeqbEmitter<A, B, C>>::vpcmpeqb(self, op0, op1, op2);
    }
    /// `VPCMPEQD` (VPCMPEQD). 
    /// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+----------------+
    /// | #  | Operands       |
    /// +----+----------------+
    /// | 1  | KReg, Xmm, Mem |
    /// | 2  | KReg, Xmm, Xmm |
    /// | 3  | KReg, Ymm, Mem |
    /// | 4  | KReg, Ymm, Ymm |
    /// | 5  | KReg, Zmm, Mem |
    /// | 6  | KReg, Zmm, Zmm |
    /// | 7  | Xmm, Xmm, Mem  |
    /// | 8  | Xmm, Xmm, Xmm  |
    /// | 9  | Ymm, Ymm, Mem  |
    /// | 10 | Ymm, Ymm, Ymm  |
    /// +----+----------------+
    /// ```
    #[inline]
    pub fn vpcmpeqd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpeqdEmitter<A, B, C> {
        <Self as VpcmpeqdEmitter<A, B, C>>::vpcmpeqd(self, op0, op1, op2);
    }
    /// `VPCMPEQQ` (VPCMPEQQ). 
    /// Performs an SIMD compare for equality of the packed quadwords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+----------------+
    /// | #  | Operands       |
    /// +----+----------------+
    /// | 1  | KReg, Xmm, Mem |
    /// | 2  | KReg, Xmm, Xmm |
    /// | 3  | KReg, Ymm, Mem |
    /// | 4  | KReg, Ymm, Ymm |
    /// | 5  | KReg, Zmm, Mem |
    /// | 6  | KReg, Zmm, Zmm |
    /// | 7  | Xmm, Xmm, Mem  |
    /// | 8  | Xmm, Xmm, Xmm  |
    /// | 9  | Ymm, Ymm, Mem  |
    /// | 10 | Ymm, Ymm, Ymm  |
    /// +----+----------------+
    /// ```
    #[inline]
    pub fn vpcmpeqq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpeqqEmitter<A, B, C> {
        <Self as VpcmpeqqEmitter<A, B, C>>::vpcmpeqq(self, op0, op1, op2);
    }
    /// `VPCMPEQW` (VPCMPEQW). 
    /// Performs a SIMD compare for equality of the packed bytes, words, or doublewords in the destination operand (first operand) and the source operand (second operand). If a pair of data elements is equal, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPEQB%3APCMPEQW%3APCMPEQD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+----------------+
    /// | #  | Operands       |
    /// +----+----------------+
    /// | 1  | KReg, Xmm, Mem |
    /// | 2  | KReg, Xmm, Xmm |
    /// | 3  | KReg, Ymm, Mem |
    /// | 4  | KReg, Ymm, Ymm |
    /// | 5  | KReg, Zmm, Mem |
    /// | 6  | KReg, Zmm, Zmm |
    /// | 7  | Xmm, Xmm, Mem  |
    /// | 8  | Xmm, Xmm, Xmm  |
    /// | 9  | Ymm, Ymm, Mem  |
    /// | 10 | Ymm, Ymm, Ymm  |
    /// +----+----------------+
    /// ```
    #[inline]
    pub fn vpcmpeqw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpeqwEmitter<A, B, C> {
        <Self as VpcmpeqwEmitter<A, B, C>>::vpcmpeqw(self, op0, op1, op2);
    }
    /// `VPCMPESTRI` (VPCMPESTRI). 
    /// The instruction compares and processes data from two string fragments based on the encoded value in the imm8 control byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates an index stored to the count register (ECX).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPESTRI.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpcmpestri<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpestriEmitter<A, B, C> {
        <Self as VpcmpestriEmitter<A, B, C>>::vpcmpestri(self, op0, op1, op2);
    }
    /// `VPCMPESTRM` (VPCMPESTRM). 
    /// The instruction compares data from two string fragments based on the encoded value in the imm8 contol byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates a mask stored to XMM0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPESTRM.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpcmpestrm<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpestrmEmitter<A, B, C> {
        <Self as VpcmpestrmEmitter<A, B, C>>::vpcmpestrm(self, op0, op1, op2);
    }
    /// `VPCMPGTB` (VPCMPGTB). 
    /// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+----------------+
    /// | #  | Operands       |
    /// +----+----------------+
    /// | 1  | KReg, Xmm, Mem |
    /// | 2  | KReg, Xmm, Xmm |
    /// | 3  | KReg, Ymm, Mem |
    /// | 4  | KReg, Ymm, Ymm |
    /// | 5  | KReg, Zmm, Mem |
    /// | 6  | KReg, Zmm, Zmm |
    /// | 7  | Xmm, Xmm, Mem  |
    /// | 8  | Xmm, Xmm, Xmm  |
    /// | 9  | Ymm, Ymm, Mem  |
    /// | 10 | Ymm, Ymm, Ymm  |
    /// +----+----------------+
    /// ```
    #[inline]
    pub fn vpcmpgtb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpgtbEmitter<A, B, C> {
        <Self as VpcmpgtbEmitter<A, B, C>>::vpcmpgtb(self, op0, op1, op2);
    }
    /// `VPCMPGTD` (VPCMPGTD). 
    /// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+----------------+
    /// | #  | Operands       |
    /// +----+----------------+
    /// | 1  | KReg, Xmm, Mem |
    /// | 2  | KReg, Xmm, Xmm |
    /// | 3  | KReg, Ymm, Mem |
    /// | 4  | KReg, Ymm, Ymm |
    /// | 5  | KReg, Zmm, Mem |
    /// | 6  | KReg, Zmm, Zmm |
    /// | 7  | Xmm, Xmm, Mem  |
    /// | 8  | Xmm, Xmm, Xmm  |
    /// | 9  | Ymm, Ymm, Mem  |
    /// | 10 | Ymm, Ymm, Ymm  |
    /// +----+----------------+
    /// ```
    #[inline]
    pub fn vpcmpgtd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpgtdEmitter<A, B, C> {
        <Self as VpcmpgtdEmitter<A, B, C>>::vpcmpgtd(self, op0, op1, op2);
    }
    /// `VPCMPGTQ` (VPCMPGTQ). 
    /// Performs an SIMD signed compare for the packed quadwords in the destination operand (first operand) and the source operand (second operand). If the data element in the first (destination) operand is greater than the corresponding element in the second (source) operand, the corresponding data element in the destination is set to all 1s; otherwise, it is set to 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+----------------+
    /// | #  | Operands       |
    /// +----+----------------+
    /// | 1  | KReg, Xmm, Mem |
    /// | 2  | KReg, Xmm, Xmm |
    /// | 3  | KReg, Ymm, Mem |
    /// | 4  | KReg, Ymm, Ymm |
    /// | 5  | KReg, Zmm, Mem |
    /// | 6  | KReg, Zmm, Zmm |
    /// | 7  | Xmm, Xmm, Mem  |
    /// | 8  | Xmm, Xmm, Xmm  |
    /// | 9  | Ymm, Ymm, Mem  |
    /// | 10 | Ymm, Ymm, Ymm  |
    /// +----+----------------+
    /// ```
    #[inline]
    pub fn vpcmpgtq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpgtqEmitter<A, B, C> {
        <Self as VpcmpgtqEmitter<A, B, C>>::vpcmpgtq(self, op0, op1, op2);
    }
    /// `VPCMPGTW` (VPCMPGTW). 
    /// Performs an SIMD signed compare for the greater value of the packed byte, word, or doubleword integers in the destination operand (first operand) and the source operand (second operand). If a data element in the destination operand is greater than the corresponding date element in the source operand, the corresponding data element in the destination operand is set to all 1s; otherwise, it is set to all 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPGTB%3APCMPGTW%3APCMPGTD.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +----+----------------+
    /// | #  | Operands       |
    /// +----+----------------+
    /// | 1  | KReg, Xmm, Mem |
    /// | 2  | KReg, Xmm, Xmm |
    /// | 3  | KReg, Ymm, Mem |
    /// | 4  | KReg, Ymm, Ymm |
    /// | 5  | KReg, Zmm, Mem |
    /// | 6  | KReg, Zmm, Zmm |
    /// | 7  | Xmm, Xmm, Mem  |
    /// | 8  | Xmm, Xmm, Xmm  |
    /// | 9  | Ymm, Ymm, Mem  |
    /// | 10 | Ymm, Ymm, Ymm  |
    /// +----+----------------+
    /// ```
    #[inline]
    pub fn vpcmpgtw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpgtwEmitter<A, B, C> {
        <Self as VpcmpgtwEmitter<A, B, C>>::vpcmpgtw(self, op0, op1, op2);
    }
    /// `VPCMPISTRI` (VPCMPISTRI). 
    /// The instruction compares data from two strings based on the encoded value in the imm8 control byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”), and generates an index stored to ECX.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPISTRI.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpcmpistri<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpistriEmitter<A, B, C> {
        <Self as VpcmpistriEmitter<A, B, C>>::vpcmpistri(self, op0, op1, op2);
    }
    /// `VPCMPISTRM` (VPCMPISTRM). 
    /// The instruction compares data from two strings based on the encoded value in the imm8 byte (see Section 4.1, “Imm8 Control Byte Operation for PCMPESTRI / PCMPESTRM / PCMPISTRI / PCMPISTRM”) generating a mask stored to XMM0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PCMPISTRM.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Mem, Imm |
    /// | 2 | Xmm, Xmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpcmpistrm<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpcmpistrmEmitter<A, B, C> {
        <Self as VpcmpistrmEmitter<A, B, C>>::vpcmpistrm(self, op0, op1, op2);
    }
    /// `VPERM2F128` (VPERM2F128). 
    /// Permute 128 bit floating-point-containing fields from the first source operand (second operand) and second source operand (third operand) using bits in the 8-bit immediate and store results in the destination operand (first operand). The first source operand is a YMM register, the second source operand is a YMM register or a 256-bit memory location, and the destination operand is a YMM register.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VPERM2F128.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Ymm, Ymm, Mem, Imm |
    /// | 2 | Ymm, Ymm, Ymm, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vperm2f128<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: Vperm2f128Emitter<A, B, C, D> {
        <Self as Vperm2f128Emitter<A, B, C, D>>::vperm2f128(self, op0, op1, op2, op3);
    }
    /// `VPEXTRD` (VPEXTRD). 
    /// Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Xmm, Imm |
    /// | 2 | Mem, Xmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpextrd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpextrdEmitter<A, B, C> {
        <Self as VpextrdEmitter<A, B, C>>::vpextrd(self, op0, op1, op2);
    }
    /// `VPEXTRQ` (VPEXTRQ). 
    /// Extract a byte/dword/qword integer value from the source XMM register at a byte/dword/qword offset determined from imm8[3:0]. The destination can be a register or byte/dword/qword memory location. If the destination is a register, the upper bits of the register are zero extended.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PEXTRB%3APEXTRD%3APEXTRQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Gpd, Xmm, Imm |
    /// | 2 | Gpq, Xmm, Imm |
    /// | 3 | Mem, Xmm, Imm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpextrq<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpextrqEmitter<A, B, C> {
        <Self as VpextrqEmitter<A, B, C>>::vpextrq(self, op0, op1, op2);
    }
    /// `VPHADDD` (VPHADDD). 
    /// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vphaddd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VphadddEmitter<A, B, C> {
        <Self as VphadddEmitter<A, B, C>>::vphaddd(self, op0, op1, op2);
    }
    /// `VPHADDSW` (VPHADDSW). 
    /// (V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDSW.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vphaddsw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VphaddswEmitter<A, B, C> {
        <Self as VphaddswEmitter<A, B, C>>::vphaddsw(self, op0, op1, op2);
    }
    /// `VPHADDW` (VPHADDW). 
    /// (V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand). (V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHADDW%3APHADDD.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vphaddw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VphaddwEmitter<A, B, C> {
        <Self as VphaddwEmitter<A, B, C>>::vphaddw(self, op0, op1, op2);
    }
    /// `VPHMINPOSUW` (VPHMINPOSUW). 
    /// Determine the minimum unsigned word value in the source operand (second operand) and place the unsigned word in the low word (bits 0-15) of the destination operand (first operand). The word index of the minimum value is stored in bits 16-18 of the destination operand. The remaining upper bits of the destination are set to zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHMINPOSUW.html).
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
    pub fn vphminposuw<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VphminposuwEmitter<A, B> {
        <Self as VphminposuwEmitter<A, B>>::vphminposuw(self, op0, op1);
    }
    /// `VPHSUBD` (VPHSUBD). 
    /// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vphsubd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VphsubdEmitter<A, B, C> {
        <Self as VphsubdEmitter<A, B, C>>::vphsubd(self, op0, op1, op2);
    }
    /// `VPHSUBSW` (VPHSUBSW). 
    /// (V)PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed to the destination operand (first operand). When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBSW.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vphsubsw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VphsubswEmitter<A, B, C> {
        <Self as VphsubswEmitter<A, B, C>>::vphsubsw(self, op0, op1, op2);
    }
    /// `VPHSUBW` (VPHSUBW). 
    /// (V)PHSUBW performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands, and packs the signed 16-bit results to the destination operand (first operand). (V)PHSUBD performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant doubleword of each pair, and packs the signed 32-bit result to the destination operand. When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PHSUBW%3APHSUBD.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vphsubw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VphsubwEmitter<A, B, C> {
        <Self as VphsubwEmitter<A, B, C>>::vphsubw(self, op0, op1, op2);
    }
    /// `VPINSRD` (VPINSRD). 
    /// Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Gpd, Imm |
    /// | 2 | Xmm, Xmm, Mem, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vpinsrd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VpinsrdEmitter<A, B, C, D> {
        <Self as VpinsrdEmitter<A, B, C, D>>::vpinsrd(self, op0, op1, op2, op3);
    }
    /// `VPINSRQ` (VPINSRQ). 
    /// Copies a byte/dword/qword from the source operand (second operand) and inserts it in the destination operand (first operand) at the location specified with the count operand (third operand). (The other elements in the destination register are left untouched.) The source operand can be a general-purpose register or a memory location. (When the source operand is a general-purpose register, PINSRB copies the low byte of the register.) The destination operand is an XMM register. The count operand is an 8-bit immediate. When specifying a qword[dword, byte] location in an XMM register, the [2, 4] least-significant bit(s) of the count operand specify the location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PINSRB%3APINSRD%3APINSRQ.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+--------------------+
    /// | # | Operands           |
    /// +---+--------------------+
    /// | 1 | Xmm, Xmm, Gpd, Imm |
    /// | 2 | Xmm, Xmm, Gpq, Imm |
    /// | 3 | Xmm, Xmm, Mem, Imm |
    /// +---+--------------------+
    /// ```
    #[inline]
    pub fn vpinsrq<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VpinsrqEmitter<A, B, C, D> {
        <Self as VpinsrqEmitter<A, B, C, D>>::vpinsrq(self, op0, op1, op2, op3);
    }
    /// `VPMOVMSKB` (VPMOVMSKB). 
    /// Creates a mask made up of the most significant bit of each byte of the source operand (second operand) and stores the result in the low byte or word of the destination operand (first operand).
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PMOVMSKB.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Xmm |
    /// | 2 | Gpd, Ymm |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vpmovmskb<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VpmovmskbEmitter<A, B> {
        <Self as VpmovmskbEmitter<A, B>>::vpmovmskb(self, op0, op1);
    }
    /// `VPOR` (VPOR). 
    /// Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/POR.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpor<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VporEmitter<A, B, C> {
        <Self as VporEmitter<A, B, C>>::vpor(self, op0, op1, op2);
    }
    /// `VPSIGNB` (VPSIGNB). 
    /// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpsignb<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpsignbEmitter<A, B, C> {
        <Self as VpsignbEmitter<A, B, C>>::vpsignb(self, op0, op1, op2);
    }
    /// `VPSIGND` (VPSIGND). 
    /// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpsignd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpsigndEmitter<A, B, C> {
        <Self as VpsigndEmitter<A, B, C>>::vpsignd(self, op0, op1, op2);
    }
    /// `VPSIGNW` (VPSIGNW). 
    /// (V)PSIGNB/(V)PSIGNW/(V)PSIGND negates each data element of the destination operand (the first operand) if the signed integer value of the corresponding data element in the source operand (the second operand) is less than zero. If the signed integer value of a data element in the source operand is positive, the corresponding data element in the destination operand is unchanged. If a data element in the source operand is zero, the corresponding data element in the destination operand is set to zero.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PSIGNB%3APSIGNW%3APSIGND.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpsignw<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpsignwEmitter<A, B, C> {
        <Self as VpsignwEmitter<A, B, C>>::vpsignw(self, op0, op1, op2);
    }
    /// `VPTEST` (VPTEST). 
    /// PTEST and VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND of the first source operand (first operand) and the second source operand (second operand). VPTEST sets the CF flag if all bits in the result are 0 of the bitwise AND of the second source operand (second operand) and the logical NOT of the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PTEST.html).
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
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vptest<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VptestEmitter<A, B> {
        <Self as VptestEmitter<A, B>>::vptest(self, op0, op1);
    }
    /// `VPXOR` (VPXOR). 
    /// Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand. Each bit of the result is 1 if the corresponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/PXOR.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vpxor<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VpxorEmitter<A, B, C> {
        <Self as VpxorEmitter<A, B, C>>::vpxor(self, op0, op1, op2);
    }
    /// `VRCPPS` (VRCPPS). 
    /// Performs a SIMD computation of the approximate reciprocals of the four packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RCPPS.html).
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
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vrcpps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VrcppsEmitter<A, B> {
        <Self as VrcppsEmitter<A, B>>::vrcpps(self, op0, op1);
    }
    /// `VRCPSS` (VRCPSS). 
    /// Computes of an approximate reciprocal of the low single precision floating-point value in the source operand (second operand) and stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RCPSS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vrcpss<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VrcpssEmitter<A, B, C> {
        <Self as VrcpssEmitter<A, B, C>>::vrcpss(self, op0, op1, op2);
    }
    /// `VROUNDPD` (VROUNDPD). 
    /// Round the 2 double precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a double precision floating-point value.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDPD.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vroundpd<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VroundpdEmitter<A, B, C> {
        <Self as VroundpdEmitter<A, B, C>>::vroundpd(self, op0, op1, op2);
    }
    /// `VROUNDPS` (VROUNDPS). 
    /// Round the 4 single precision floating-point values in the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the results in the destination operand (first operand). The rounding process rounds each input floating-point value to an integer value and returns the integer result as a single precision floating-point value.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDPS.html).
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
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vroundps<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VroundpsEmitter<A, B, C> {
        <Self as VroundpsEmitter<A, B, C>>::vroundps(self, op0, op1, op2);
    }
    /// `VROUNDSD` (VROUNDSD). 
    /// Round the double precision floating-point value in the lower qword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a double precision floating-point input to an integer value and returns the integer result as a double precision floating-point value in the lowest position. The upper double precision floating-point value in the destination is retained.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDSD.html).
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
    pub fn vroundsd<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VroundsdEmitter<A, B, C, D> {
        <Self as VroundsdEmitter<A, B, C, D>>::vroundsd(self, op0, op1, op2, op3);
    }
    /// `VROUNDSS` (VROUNDSS). 
    /// Round the single precision floating-point value in the lowest dword of the source operand (second operand) using the rounding mode specified in the immediate operand (third operand) and place the result in the destination operand (first operand). The rounding process rounds a single precision floating-point input to an integer value and returns the result as a single precision floating-point value in the lowest position. The upper three single precision floating-point values in the destination are retained.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/ROUNDSS.html).
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
    pub fn vroundss<A, B, C, D>(&mut self, op0: A, op1: B, op2: C, op3: D)
    where Assembler<'a>: VroundssEmitter<A, B, C, D> {
        <Self as VroundssEmitter<A, B, C, D>>::vroundss(self, op0, op1, op2, op3);
    }
    /// `VRSQRTPS` (VRSQRTPS). 
    /// Performs a SIMD computation of the approximate reciprocals of the square roots of the four packed single precision floating-point values in the source operand (second operand) and stores the packed single precision floating-point results in the destination operand. The source operand can be an XMM register or a 128-bit memory location. The destination operand is an XMM register. See Figure 10-5 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a SIMD single precision floating-point operation.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RSQRTPS.html).
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
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vrsqrtps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VrsqrtpsEmitter<A, B> {
        <Self as VrsqrtpsEmitter<A, B>>::vrsqrtps(self, op0, op1);
    }
    /// `VRSQRTSS` (VRSQRTSS). 
    /// Computes an approximate reciprocal of the square root of the low single precision floating-point value in the source operand (second operand) stores the single precision floating-point result in the destination operand. The source operand can be an XMM register or a 32-bit memory location. The destination operand is an XMM register. The three high-order doublewords of the destination operand remain unchanged. See Figure 10-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for an illustration of a scalar single precision floating-point operation.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RSQRTSS.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+---------------+
    /// | # | Operands      |
    /// +---+---------------+
    /// | 1 | Xmm, Xmm, Mem |
    /// | 2 | Xmm, Xmm, Xmm |
    /// +---+---------------+
    /// ```
    #[inline]
    pub fn vrsqrtss<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: VrsqrtssEmitter<A, B, C> {
        <Self as VrsqrtssEmitter<A, B, C>>::vrsqrtss(self, op0, op1, op2);
    }
    /// `VSTMXCSR` (VSTMXCSR). 
    /// Stores the contents of the MXCSR control and status register to the destination operand. The destination operand is a 32-bit memory location. The reserved bits in the MXCSR register are stored as 0s.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/STMXCSR.html).
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
    pub fn vstmxcsr<A>(&mut self, op0: A)
    where Assembler<'a>: VstmxcsrEmitter<A> {
        <Self as VstmxcsrEmitter<A>>::vstmxcsr(self, op0);
    }
    /// `VTESTPD` (VTESTPD). 
    /// VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html).
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
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vtestpd<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VtestpdEmitter<A, B> {
        <Self as VtestpdEmitter<A, B>>::vtestpd(self, op0, op1);
    }
    /// `VTESTPS` (VTESTPS). 
    /// VTESTPS performs a bitwise comparison of all the sign bits of the packed single-precision elements in the first source operation and corresponding sign bits in the second source operand. If the AND of the source sign bits with the dest sign bits produces all zeros, the ZF is set else the ZF is clear. If the AND of the source sign bits with the inverted dest sign bits produces all zeros the CF is set else the CF is clear. An attempt to execute VTESTPS with VEX.W=1 will cause #UD.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VTESTPD%3AVTESTPS.html).
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
    /// +---+----------+
    /// ```
    #[inline]
    pub fn vtestps<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: VtestpsEmitter<A, B> {
        <Self as VtestpsEmitter<A, B>>::vtestps(self, op0, op1);
    }
    /// `VZEROALL` (VZEROALL). 
    /// In 64-bit mode, the instruction zeroes XMM0-XMM15, YMM0-YMM15, and ZMM0-ZMM15. Outside 64-bit mode, it zeroes only XMM0-XMM7, YMM0-YMM7, and ZMM0-ZMM7. VZEROALL does not modify ZMM16-ZMM31.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VZEROALL.html).
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
    pub fn vzeroall(&mut self)
    where Assembler<'a>: VzeroallEmitter {
        <Self as VzeroallEmitter>::vzeroall(self);
    }
    /// `VZEROUPPER` (VZEROUPPER). 
    /// In 64-bit mode, the instruction zeroes the bits in positions 128 and higher in YMM0-YMM15 and ZMM0-ZMM15. Outside 64-bit mode, it zeroes those bits only in YMM0-YMM7 and ZMM0-ZMM7. VZEROUPPER does not modify the lower 128 bits of these registers and it does not modify ZMM16-ZMM31.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/VZEROUPPER.html).
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
    pub fn vzeroupper(&mut self)
    where Assembler<'a>: VzeroupperEmitter {
        <Self as VzeroupperEmitter>::vzeroupper(self);
    }
}
