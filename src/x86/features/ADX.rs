use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `ADCX`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Gpd |
/// | 2 | Gpd, Mem |
/// | 3 | Gpq, Gpq |
/// | 4 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait AdcxEmitter<A, B> {
    fn adcx(&mut self, op0: A, op1: B);
}

impl<'a> AdcxEmitter<Gpd, Gpd> for Assembler<'a> {
    fn adcx(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(ADCX32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> AdcxEmitter<Gpd, Mem> for Assembler<'a> {
    fn adcx(&mut self, op0: Gpd, op1: Mem) {
        self.emit(ADCX32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> AdcxEmitter<Gpq, Gpq> for Assembler<'a> {
    fn adcx(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(ADCX64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> AdcxEmitter<Gpq, Mem> for Assembler<'a> {
    fn adcx(&mut self, op0: Gpq, op1: Mem) {
        self.emit(ADCX64RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

/// `ADOX`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Gpd |
/// | 2 | Gpd, Mem |
/// | 3 | Gpq, Gpq |
/// | 4 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait AdoxEmitter<A, B> {
    fn adox(&mut self, op0: A, op1: B);
}

impl<'a> AdoxEmitter<Gpd, Gpd> for Assembler<'a> {
    fn adox(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(ADOX32RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> AdoxEmitter<Gpd, Mem> for Assembler<'a> {
    fn adox(&mut self, op0: Gpd, op1: Mem) {
        self.emit(ADOX32RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> AdoxEmitter<Gpq, Gpq> for Assembler<'a> {
    fn adox(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(ADOX64RR, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> AdoxEmitter<Gpq, Mem> for Assembler<'a> {
    fn adox(&mut self, op0: Gpq, op1: Mem) {
        self.emit(ADOX64RM, op0.as_operand(), op1.as_operand(), &NOREG, &NOREG);
    }
}

impl<'a> Assembler<'a> {
    /// `ADCX`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Gpd |
    /// | 2 | Gpd, Mem |
    /// | 3 | Gpq, Gpq |
    /// | 4 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn adcx<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: AdcxEmitter<A, B>,
    {
        <Self as AdcxEmitter<A, B>>::adcx(self, op0, op1);
    }
    /// `ADOX`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Gpd |
    /// | 2 | Gpd, Mem |
    /// | 3 | Gpq, Gpq |
    /// | 4 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn adox<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: AdoxEmitter<A, B>,
    {
        <Self as AdoxEmitter<A, B>>::adox(self, op0, op1);
    }
}
