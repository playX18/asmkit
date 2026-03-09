use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `ENQCMD`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Mem |
/// | 2 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait EnqcmdEmitter<A, B> {
    fn enqcmd(&mut self, op0: A, op1: B);
}

impl<'a> EnqcmdEmitter<Gpd, Mem> for Assembler<'a> {
    fn enqcmd(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            ENQCMD32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> EnqcmdEmitter<Gpq, Mem> for Assembler<'a> {
    fn enqcmd(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            ENQCMD64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `ENQCMDS`.
///
/// Supported operand variants:
///
/// ```text
/// +---+----------+
/// | # | Operands |
/// +---+----------+
/// | 1 | Gpd, Mem |
/// | 2 | Gpq, Mem |
/// +---+----------+
/// ```
pub trait EnqcmdsEmitter<A, B> {
    fn enqcmds(&mut self, op0: A, op1: B);
}

impl<'a> EnqcmdsEmitter<Gpd, Mem> for Assembler<'a> {
    fn enqcmds(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            ENQCMDS32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> EnqcmdsEmitter<Gpq, Mem> for Assembler<'a> {
    fn enqcmds(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            ENQCMDS64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `ENQCMD`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Mem |
    /// | 2 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn enqcmd<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: EnqcmdEmitter<A, B>,
    {
        <Self as EnqcmdEmitter<A, B>>::enqcmd(self, op0, op1);
    }
    /// `ENQCMDS`.
    ///
    /// Supported operand variants:
    ///
    /// ```text
    /// +---+----------+
    /// | # | Operands |
    /// +---+----------+
    /// | 1 | Gpd, Mem |
    /// | 2 | Gpq, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn enqcmds<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: EnqcmdsEmitter<A, B>,
    {
        <Self as EnqcmdsEmitter<A, B>>::enqcmds(self, op0, op1);
    }
}
