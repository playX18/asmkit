use crate::x86::assembler::*;
use crate::x86::operands::*;
use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `RDFSBASE` (RDFSBASE). 
/// Loads the general-purpose register indicated by the ModR/M:r/m field with the FS or GS segment base address.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RDFSBASE%3ARDGSBASE.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// +---+----------+
/// ```
pub trait RdfsbaseEmitter<A> {
    fn rdfsbase(&mut self, op0: A);
}

impl<'a> RdfsbaseEmitter<Gpd> for Assembler<'a> {
    fn rdfsbase(&mut self, op0: Gpd) {
        self.emit(RDFSBASE32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> RdfsbaseEmitter<Gpq> for Assembler<'a> {
    fn rdfsbase(&mut self, op0: Gpq) {
        self.emit(RDFSBASE64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `RDGSBASE` (RDGSBASE). 
/// Loads the general-purpose register indicated by the ModR/M:r/m field with the FS or GS segment base address.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RDFSBASE%3ARDGSBASE.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// +---+----------+
/// ```
pub trait RdgsbaseEmitter<A> {
    fn rdgsbase(&mut self, op0: A);
}

impl<'a> RdgsbaseEmitter<Gpd> for Assembler<'a> {
    fn rdgsbase(&mut self, op0: Gpd) {
        self.emit(RDGSBASE32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> RdgsbaseEmitter<Gpq> for Assembler<'a> {
    fn rdgsbase(&mut self, op0: Gpq) {
        self.emit(RDGSBASE64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `WRFSBASE` (WRFSBASE). 
/// Loads the FS or GS segment base address with the general-purpose register indicated by the modR/M:r/m field.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/WRFSBASE%3AWRGSBASE.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// +---+----------+
/// ```
pub trait WrfsbaseEmitter<A> {
    fn wrfsbase(&mut self, op0: A);
}

impl<'a> WrfsbaseEmitter<Gpd> for Assembler<'a> {
    fn wrfsbase(&mut self, op0: Gpd) {
        self.emit(WRFSBASE32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> WrfsbaseEmitter<Gpq> for Assembler<'a> {
    fn wrfsbase(&mut self, op0: Gpq) {
        self.emit(WRFSBASE64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

/// `WRGSBASE` (WRGSBASE). 
/// Loads the FS or GS segment base address with the general-purpose register indicated by the modR/M:r/m field.
///
///
/// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/WRFSBASE%3AWRGSBASE.html).
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd      |
/// | 2 | Gpq      |
/// +---+----------+
/// ```
pub trait WrgsbaseEmitter<A> {
    fn wrgsbase(&mut self, op0: A);
}

impl<'a> WrgsbaseEmitter<Gpd> for Assembler<'a> {
    fn wrgsbase(&mut self, op0: Gpd) {
        self.emit(WRGSBASE32R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}

impl<'a> WrgsbaseEmitter<Gpq> for Assembler<'a> {
    fn wrgsbase(&mut self, op0: Gpq) {
        self.emit(WRGSBASE64R, op0.as_operand(), &NOREG, &NOREG, &NOREG);
    }
}


impl<'a> Assembler<'a> {
    /// `RDFSBASE` (RDFSBASE). 
    /// Loads the general-purpose register indicated by the ModR/M:r/m field with the FS or GS segment base address.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RDFSBASE%3ARDGSBASE.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn rdfsbase<A>(&mut self, op0: A)
    where Assembler<'a>: RdfsbaseEmitter<A> {
        <Self as RdfsbaseEmitter<A>>::rdfsbase(self, op0);
    }
    /// `RDGSBASE` (RDGSBASE). 
    /// Loads the general-purpose register indicated by the ModR/M:r/m field with the FS or GS segment base address.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/RDFSBASE%3ARDGSBASE.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn rdgsbase<A>(&mut self, op0: A)
    where Assembler<'a>: RdgsbaseEmitter<A> {
        <Self as RdgsbaseEmitter<A>>::rdgsbase(self, op0);
    }
    /// `WRFSBASE` (WRFSBASE). 
    /// Loads the FS or GS segment base address with the general-purpose register indicated by the modR/M:r/m field.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/WRFSBASE%3AWRGSBASE.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn wrfsbase<A>(&mut self, op0: A)
    where Assembler<'a>: WrfsbaseEmitter<A> {
        <Self as WrfsbaseEmitter<A>>::wrfsbase(self, op0);
    }
    /// `WRGSBASE` (WRGSBASE). 
    /// Loads the FS or GS segment base address with the general-purpose register indicated by the modR/M:r/m field.
    ///
    ///
    /// For more details, see the [Intel manual](https://www.felixcloutier.com/x86/WRFSBASE%3AWRGSBASE.html).
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd      |
    /// | 2 | Gpq      |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn wrgsbase<A>(&mut self, op0: A)
    where Assembler<'a>: WrgsbaseEmitter<A> {
        <Self as WrgsbaseEmitter<A>>::wrgsbase(self, op0);
    }
}
