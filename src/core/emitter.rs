use super::operand::Operand;

const NOREG: Operand = Operand::new();

pub trait Emitter {
    fn emit(&mut self, opcode: i64, op0: &Operand, op1: &Operand, op2: &Operand, op3: &Operand);

    fn emit_n(&mut self, opcode: i64, ops: &[&Operand]) {
        match ops {
            [op0] => self.emit(opcode, op0, &NOREG, &NOREG, &NOREG),

            [op0, op1] => self.emit(opcode, op0, op1, &NOREG, &NOREG),

            [op0, op1, op2] => self.emit(opcode, op0, op1, op2, &NOREG),
            [op0, op1, op2, op3] => self.emit(opcode, op0, op1, op2, op3),
            _ => unreachable!(),
        }
    }
}
