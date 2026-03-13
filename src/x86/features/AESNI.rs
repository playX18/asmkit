use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `AESDEC` (AESDEC). 
/// This instruction performs a single round of the AES decryption flow using the Equivalent Inverse Cipher, using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESDEC.html).
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
pub trait AesdecEmitter<A, B> {
    fn aesdec(&mut self, op0: A, op1: B);
}

impl<'a> AesdecEmitter<Xmm, Xmm> for Assembler<'a> {
    fn aesdec(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(AESDECRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> AesdecEmitter<Xmm, Mem> for Assembler<'a> {
    fn aesdec(&mut self, op0: Xmm, op1: Mem) {
        self.emit(AESDECRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `AESDECLAST` (AESDECLAST). 
/// This instruction performs the last round of the AES decryption flow using the Equivalent Inverse Cipher, using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESDECLAST.html).
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
pub trait AesdeclastEmitter<A, B> {
    fn aesdeclast(&mut self, op0: A, op1: B);
}

impl<'a> AesdeclastEmitter<Xmm, Xmm> for Assembler<'a> {
    fn aesdeclast(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(AESDECLASTRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> AesdeclastEmitter<Xmm, Mem> for Assembler<'a> {
    fn aesdeclast(&mut self, op0: Xmm, op1: Mem) {
        self.emit(AESDECLASTRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `AESENC` (AESENC). 
/// This instruction performs a single round of an AES encryption flow using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESENC.html).
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
pub trait AesencEmitter<A, B> {
    fn aesenc(&mut self, op0: A, op1: B);
}

impl<'a> AesencEmitter<Xmm, Xmm> for Assembler<'a> {
    fn aesenc(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(AESENCRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> AesencEmitter<Xmm, Mem> for Assembler<'a> {
    fn aesenc(&mut self, op0: Xmm, op1: Mem) {
        self.emit(AESENCRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `AESENCLAST` (AESENCLAST). 
/// This instruction performs the last round of an AES encryption flow using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESENCLAST.html).
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
pub trait AesenclastEmitter<A, B> {
    fn aesenclast(&mut self, op0: A, op1: B);
}

impl<'a> AesenclastEmitter<Xmm, Xmm> for Assembler<'a> {
    fn aesenclast(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(AESENCLASTRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> AesenclastEmitter<Xmm, Mem> for Assembler<'a> {
    fn aesenclast(&mut self, op0: Xmm, op1: Mem) {
        self.emit(AESENCLASTRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `AESIMC` (AESIMC). 
/// Perform the InvMixColumns transformation on the source operand and store the result in the destination operand. The destination operand is an XMM register. The source operand can be an XMM register or a 128-bit memory location.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESIMC.html).
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
pub trait AesimcEmitter<A, B> {
    fn aesimc(&mut self, op0: A, op1: B);
}

impl<'a> AesimcEmitter<Xmm, Xmm> for Assembler<'a> {
    fn aesimc(&mut self, op0: Xmm, op1: Xmm) {
        self.emit(AESIMCRR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> AesimcEmitter<Xmm, Mem> for Assembler<'a> {
    fn aesimc(&mut self, op0: Xmm, op1: Mem) {
        self.emit(AESIMCRM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `AESKEYGENASSIST` (AESKEYGENASSIST). 
/// Assist in expanding the AES cipher key, by computing steps towards generating a round key for encryption, using 128-bit data specified in the source operand and an 8-bit round constant specified as an immediate, store the result in the destination operand.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESKEYGENASSIST.html).
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
pub trait AeskeygenassistEmitter<A, B, C> {
    fn aeskeygenassist(&mut self, op0: A, op1: B, op2: C);
}

impl<'a> AeskeygenassistEmitter<Xmm, Xmm, Imm> for Assembler<'a> {
    fn aeskeygenassist(&mut self, op0: Xmm, op1: Xmm, op2: Imm) {
        self.emit(AESKEYGENASSISTRRI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}

impl<'a> AeskeygenassistEmitter<Xmm, Mem, Imm> for Assembler<'a> {
    fn aeskeygenassist(&mut self, op0: Xmm, op1: Mem, op2: Imm) {
        self.emit(AESKEYGENASSISTRMI, op0.as_operand(), op1.as_operand(), op2.as_operand(), &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `AESDEC` (AESDEC). 
    /// This instruction performs a single round of the AES decryption flow using the Equivalent Inverse Cipher, using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESDEC.html).
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
    pub fn aesdec<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: AesdecEmitter<A, B> {
        <Self as AesdecEmitter<A, B>>::aesdec(self, op0, op1);
    }
    /// `AESDECLAST` (AESDECLAST). 
    /// This instruction performs the last round of the AES decryption flow using the Equivalent Inverse Cipher, using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESDECLAST.html).
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
    pub fn aesdeclast<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: AesdeclastEmitter<A, B> {
        <Self as AesdeclastEmitter<A, B>>::aesdeclast(self, op0, op1);
    }
    /// `AESENC` (AESENC). 
    /// This instruction performs a single round of an AES encryption flow using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESENC.html).
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
    pub fn aesenc<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: AesencEmitter<A, B> {
        <Self as AesencEmitter<A, B>>::aesenc(self, op0, op1);
    }
    /// `AESENCLAST` (AESENCLAST). 
    /// This instruction performs the last round of an AES encryption flow using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESENCLAST.html).
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
    pub fn aesenclast<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: AesenclastEmitter<A, B> {
        <Self as AesenclastEmitter<A, B>>::aesenclast(self, op0, op1);
    }
    /// `AESIMC` (AESIMC). 
    /// Perform the InvMixColumns transformation on the source operand and store the result in the destination operand. The destination operand is an XMM register. The source operand can be an XMM register or a 128-bit memory location.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESIMC.html).
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
    pub fn aesimc<A, B>(&mut self, op0: A, op1: B)
    where Assembler<'a>: AesimcEmitter<A, B> {
        <Self as AesimcEmitter<A, B>>::aesimc(self, op0, op1);
    }
    /// `AESKEYGENASSIST` (AESKEYGENASSIST). 
    /// Assist in expanding the AES cipher key, by computing steps towards generating a round key for encryption, using 128-bit data specified in the source operand and an 8-bit round constant specified as an immediate, store the result in the destination operand.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/AESKEYGENASSIST.html).
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
    pub fn aeskeygenassist<A, B, C>(&mut self, op0: A, op1: B, op2: C)
    where Assembler<'a>: AeskeygenassistEmitter<A, B, C> {
        <Self as AeskeygenassistEmitter<A, B, C>>::aeskeygenassist(self, op0, op1, op2);
    }
}
