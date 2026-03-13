use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `XGETBV` (XGETBV). 
/// Reads the contents of the extended control register (XCR) specified in the ECX register into registers EDX:EAX. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The EDX register is loaded with the high-order 32 bits of the XCR and the EAX register is loaded with the low-order 32 bits. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.) If fewer than 64 bits are implemented in the XCR being read, the values returned to EDX:EAX in unimplemented bit locations are undefined.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XGETBV.html).
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
pub trait XgetbvEmitter {
    fn xgetbv(&mut self);
}

impl<'a> XgetbvEmitter for Assembler<'a> {
    fn xgetbv(&mut self) {
        self.emit(XGETBV, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}

/// `XRSTOR` (XRSTOR). 
/// Performs a full or partial restore of processor state components from the XSAVE area located at the memory address specified by the source operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components restored correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XRSTOR.html).
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
pub trait XrstorEmitter<A> {
    fn xrstor(&mut self, op0: A);
}

impl<'a> XrstorEmitter<Mem> for Assembler<'a> {
    fn xrstor(&mut self, op0: Mem) {
        self.emit(XRSTOR32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `XSAVE` (XSAVE). 
/// Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XSAVE.html).
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
pub trait XsaveEmitter<A> {
    fn xsave(&mut self, op0: A);
}

impl<'a> XsaveEmitter<Mem> for Assembler<'a> {
    fn xsave(&mut self, op0: Mem) {
        self.emit(XSAVE32M, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `XSETBV` (XSETBV). 
/// Writes the contents of registers EDX:EAX into the 64-bit extended control register (XCR) specified in the ECX register. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The contents of the EDX register are copied to high-order 32 bits of the selected XCR and the contents of the EAX register are copied to low-order 32 bits of the XCR. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are ignored.) Undefined or reserved bits in an XCR should be set to values previously read.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XSETBV.html).
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
pub trait XsetbvEmitter {
    fn xsetbv(&mut self);
}

impl<'a> XsetbvEmitter for Assembler<'a> {
    fn xsetbv(&mut self) {
        self.emit(XSETBV, &NOREG, &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `XGETBV` (XGETBV). 
    /// Reads the contents of the extended control register (XCR) specified in the ECX register into registers EDX:EAX. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The EDX register is loaded with the high-order 32 bits of the XCR and the EAX register is loaded with the low-order 32 bits. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.) If fewer than 64 bits are implemented in the XCR being read, the values returned to EDX:EAX in unimplemented bit locations are undefined.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XGETBV.html).
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
    pub fn xgetbv(&mut self)
    where Assembler<'a>: XgetbvEmitter {
        <Self as XgetbvEmitter>::xgetbv(self);
    }
    /// `XRSTOR` (XRSTOR). 
    /// Performs a full or partial restore of processor state components from the XSAVE area located at the memory address specified by the source operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components restored correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XRSTOR.html).
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
    pub fn xrstor<A>(&mut self, op0: A)
    where Assembler<'a>: XrstorEmitter<A> {
        <Self as XrstorEmitter<A>>::xrstor(self, op0);
    }
    /// `XSAVE` (XSAVE). 
    /// Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand. The implicit EDX:EAX register pair specifies a 64-bit instruction mask. The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XSAVE.html).
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
    pub fn xsave<A>(&mut self, op0: A)
    where Assembler<'a>: XsaveEmitter<A> {
        <Self as XsaveEmitter<A>>::xsave(self, op0);
    }
    /// `XSETBV` (XSETBV). 
    /// Writes the contents of registers EDX:EAX into the 64-bit extended control register (XCR) specified in the ECX register. (On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The contents of the EDX register are copied to high-order 32 bits of the selected XCR and the contents of the EAX register are copied to low-order 32 bits of the XCR. (On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are ignored.) Undefined or reserved bits in an XCR should be set to values previously read.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/XSETBV.html).
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
    pub fn xsetbv(&mut self)
    where Assembler<'a>: XsetbvEmitter {
        <Self as XsetbvEmitter>::xsetbv(self);
    }
}
