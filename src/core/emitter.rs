use crate::AsmError;

use super::operand::Operand;

pub trait Emitter {
    fn emit(
        &mut self,
        opcode: i64,
        op0: &Operand,
        op1: &Operand,
        op2: &Operand,
        op3: &Operand,
    ) -> Result<(), AsmError>;
}
