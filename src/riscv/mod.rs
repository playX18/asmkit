pub mod assembler;
pub mod emitter;
pub mod opcodes;
pub mod operands;

pub use crate::core::operand::imm;
pub use assembler::*;
pub use emitter::EmitterExplicit;
pub use opcodes::{Inst, InstructionValue, Opcode};
pub use operands::*;
pub use regs::*;

/// Return the length (in bytes) of an instruction given the low 16 bits of it.
///
/// The current spec reserves a bit pattern for instructions of length >= 192 bits, but for
/// simplicity this function just returns 24 in that case. The largest instructions currently
/// defined are 4 bytes so it will likely be a long time until this diffence matters.
pub fn instruction_length(i: u16) -> usize {
    if i & 0b11 != 0b11 {
        2
    } else if i & 0b11100 != 0b11100 {
        4
    } else if i & 0b111111 == 0b011111 {
        6
    } else if i & 0b1111111 == 0b011111 {
        8
    } else {
        10 + 2 * ((i >> 12) & 0b111) as usize
    }
}
