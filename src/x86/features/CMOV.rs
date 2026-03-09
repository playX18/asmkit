use super::super::opcodes::*;
use crate::core::emitter::*;
use crate::core::operand::*;
use crate::x86::assembler::*;
use crate::x86::operands::*;

/// A dummy operand that represents no register. Here just for simplicity.
const NOREG: Operand = Operand::new();

/// `CMOVA`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovaEmitter<A, B> {
    fn cmova(&mut self, op0: A, op1: B);
}

impl<'a> CmovaEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmova(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVA16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovaEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmova(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVA16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovaEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmova(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVA32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovaEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmova(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVA32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovaEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmova(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVA64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovaEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmova(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVA64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVBE`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovbeEmitter<A, B> {
    fn cmovbe(&mut self, op0: A, op1: B);
}

impl<'a> CmovbeEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovbe(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVBE16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovbeEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovbe(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVBE16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovbeEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovbe(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVBE32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovbeEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovbe(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVBE32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovbeEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovbe(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVBE64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovbeEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovbe(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVBE64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVC`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovcEmitter<A, B> {
    fn cmovc(&mut self, op0: A, op1: B);
}

impl<'a> CmovcEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovc(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVC16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovcEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovc(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVC16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovcEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovc(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVC32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovcEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovc(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVC32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovcEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovc(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVC64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovcEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovc(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVC64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVG`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovgEmitter<A, B> {
    fn cmovg(&mut self, op0: A, op1: B);
}

impl<'a> CmovgEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovg(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVG16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovgEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovg(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVG16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovgEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovg(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVG32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovgEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovg(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVG32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovgEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovg(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVG64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovgEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovg(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVG64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVGE`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovgeEmitter<A, B> {
    fn cmovge(&mut self, op0: A, op1: B);
}

impl<'a> CmovgeEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovge(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVGE16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovgeEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovge(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVGE16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovgeEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovge(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVGE32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovgeEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovge(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVGE32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovgeEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovge(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVGE64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovgeEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovge(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVGE64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVL`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovlEmitter<A, B> {
    fn cmovl(&mut self, op0: A, op1: B);
}

impl<'a> CmovlEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovl(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVL16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovlEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovl(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVL16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovlEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovl(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVL32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovlEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovl(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVL32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovlEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovl(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVL64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovlEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovl(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVL64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVLE`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovleEmitter<A, B> {
    fn cmovle(&mut self, op0: A, op1: B);
}

impl<'a> CmovleEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovle(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVLE16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovleEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovle(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVLE16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovleEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovle(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVLE32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovleEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovle(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVLE32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovleEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovle(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVLE64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovleEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovle(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVLE64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVNC`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovncEmitter<A, B> {
    fn cmovnc(&mut self, op0: A, op1: B);
}

impl<'a> CmovncEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovnc(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVNC16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovncEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovnc(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVNC16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovncEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovnc(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVNC32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovncEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovnc(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVNC32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovncEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovnc(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVNC64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovncEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovnc(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVNC64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVNO`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovnoEmitter<A, B> {
    fn cmovno(&mut self, op0: A, op1: B);
}

impl<'a> CmovnoEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovno(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVNO16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnoEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovno(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVNO16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnoEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovno(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVNO32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnoEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovno(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVNO32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnoEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovno(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVNO64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnoEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovno(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVNO64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVNP`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovnpEmitter<A, B> {
    fn cmovnp(&mut self, op0: A, op1: B);
}

impl<'a> CmovnpEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovnp(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVNP16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnpEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovnp(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVNP16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnpEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovnp(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVNP32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnpEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovnp(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVNP32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnpEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovnp(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVNP64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnpEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovnp(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVNP64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVNS`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovnsEmitter<A, B> {
    fn cmovns(&mut self, op0: A, op1: B);
}

impl<'a> CmovnsEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovns(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVNS16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnsEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovns(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVNS16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnsEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovns(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVNS32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnsEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovns(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVNS32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnsEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovns(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVNS64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnsEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovns(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVNS64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVNZ`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovnzEmitter<A, B> {
    fn cmovnz(&mut self, op0: A, op1: B);
}

impl<'a> CmovnzEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovnz(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVNZ16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnzEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovnz(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVNZ16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnzEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovnz(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVNZ32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnzEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovnz(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVNZ32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnzEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovnz(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVNZ64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovnzEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovnz(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVNZ64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVO`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovoEmitter<A, B> {
    fn cmovo(&mut self, op0: A, op1: B);
}

impl<'a> CmovoEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovo(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVO16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovoEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovo(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVO16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovoEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovo(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVO32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovoEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovo(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVO32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovoEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovo(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVO64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovoEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovo(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVO64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVP`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovpEmitter<A, B> {
    fn cmovp(&mut self, op0: A, op1: B);
}

impl<'a> CmovpEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovp(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVP16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovpEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovp(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVP16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovpEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovp(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVP32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovpEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovp(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVP32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovpEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovp(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVP64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovpEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovp(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVP64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVS`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovsEmitter<A, B> {
    fn cmovs(&mut self, op0: A, op1: B);
}

impl<'a> CmovsEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovs(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVS16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovsEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovs(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVS16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovsEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovs(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVS32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovsEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovs(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVS32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovsEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovs(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVS64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovsEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovs(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVS64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVZ`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovzEmitter<A, B> {
    fn cmovz(&mut self, op0: A, op1: B);
}

impl<'a> CmovzEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovz(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVZ16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovzEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovz(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVZ16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovzEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovz(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVZ32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovzEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovz(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVZ32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovzEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovz(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVZ64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovzEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovz(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVZ64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

/// `CMOVCC`.
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
/// | 5 | Gpw, Gpw |
/// | 6 | Gpw, Mem |
/// +---+----------+
/// ```
pub trait CmovccEmitter<A, B> {
    fn cmovcc(&mut self, op0: A, op1: B);
}

impl<'a> CmovccEmitter<Gpw, Gpw> for Assembler<'a> {
    fn cmovcc(&mut self, op0: Gpw, op1: Gpw) {
        self.emit(
            CMOVCC16RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovccEmitter<Gpw, Mem> for Assembler<'a> {
    fn cmovcc(&mut self, op0: Gpw, op1: Mem) {
        self.emit(
            CMOVCC16RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovccEmitter<Gpd, Gpd> for Assembler<'a> {
    fn cmovcc(&mut self, op0: Gpd, op1: Gpd) {
        self.emit(
            CMOVCC32RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovccEmitter<Gpd, Mem> for Assembler<'a> {
    fn cmovcc(&mut self, op0: Gpd, op1: Mem) {
        self.emit(
            CMOVCC32RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovccEmitter<Gpq, Gpq> for Assembler<'a> {
    fn cmovcc(&mut self, op0: Gpq, op1: Gpq) {
        self.emit(
            CMOVCC64RR,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> CmovccEmitter<Gpq, Mem> for Assembler<'a> {
    fn cmovcc(&mut self, op0: Gpq, op1: Mem) {
        self.emit(
            CMOVCC64RM,
            op0.as_operand(),
            op1.as_operand(),
            &NOREG,
            &NOREG,
        );
    }
}

impl<'a> Assembler<'a> {
    /// `CMOVA`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmova<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovaEmitter<A, B>,
    {
        <Self as CmovaEmitter<A, B>>::cmova(self, op0, op1);
    }
    /// `CMOVBE`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovbe<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovbeEmitter<A, B>,
    {
        <Self as CmovbeEmitter<A, B>>::cmovbe(self, op0, op1);
    }
    /// `CMOVC`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovc<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovcEmitter<A, B>,
    {
        <Self as CmovcEmitter<A, B>>::cmovc(self, op0, op1);
    }
    /// `CMOVG`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovg<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovgEmitter<A, B>,
    {
        <Self as CmovgEmitter<A, B>>::cmovg(self, op0, op1);
    }
    /// `CMOVGE`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovge<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovgeEmitter<A, B>,
    {
        <Self as CmovgeEmitter<A, B>>::cmovge(self, op0, op1);
    }
    /// `CMOVL`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovl<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovlEmitter<A, B>,
    {
        <Self as CmovlEmitter<A, B>>::cmovl(self, op0, op1);
    }
    /// `CMOVLE`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovle<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovleEmitter<A, B>,
    {
        <Self as CmovleEmitter<A, B>>::cmovle(self, op0, op1);
    }
    /// `CMOVNC`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovnc<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovncEmitter<A, B>,
    {
        <Self as CmovncEmitter<A, B>>::cmovnc(self, op0, op1);
    }
    /// `CMOVNO`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovno<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovnoEmitter<A, B>,
    {
        <Self as CmovnoEmitter<A, B>>::cmovno(self, op0, op1);
    }
    /// `CMOVNP`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovnp<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovnpEmitter<A, B>,
    {
        <Self as CmovnpEmitter<A, B>>::cmovnp(self, op0, op1);
    }
    /// `CMOVNS`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovns<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovnsEmitter<A, B>,
    {
        <Self as CmovnsEmitter<A, B>>::cmovns(self, op0, op1);
    }
    /// `CMOVNZ`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovnz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovnzEmitter<A, B>,
    {
        <Self as CmovnzEmitter<A, B>>::cmovnz(self, op0, op1);
    }
    /// `CMOVO`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovo<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovoEmitter<A, B>,
    {
        <Self as CmovoEmitter<A, B>>::cmovo(self, op0, op1);
    }
    /// `CMOVP`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovp<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovpEmitter<A, B>,
    {
        <Self as CmovpEmitter<A, B>>::cmovp(self, op0, op1);
    }
    /// `CMOVS`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovs<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovsEmitter<A, B>,
    {
        <Self as CmovsEmitter<A, B>>::cmovs(self, op0, op1);
    }
    /// `CMOVZ`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovz<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovzEmitter<A, B>,
    {
        <Self as CmovzEmitter<A, B>>::cmovz(self, op0, op1);
    }
    /// `CMOVCC`.
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
    /// | 5 | Gpw, Gpw |
    /// | 6 | Gpw, Mem |
    /// +---+----------+
    /// ```
    #[inline]
    pub fn cmovcc<A, B>(&mut self, op0: A, op1: B)
    where
        Assembler<'a>: CmovccEmitter<A, B>,
    {
        <Self as CmovccEmitter<A, B>>::cmovcc(self, op0, op1);
    }
}
