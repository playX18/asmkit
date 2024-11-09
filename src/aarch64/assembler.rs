use crate::aarch64::opcodes::{Encoding, Opcode, INST_INFO};
use crate::core::buffer::{LabelUse, Reloc};
use crate::core::operand::*;
use crate::AsmError;
use crate::{
    core::{buffer::CodeBuffer, emitter::Emitter},
    riscv::EmitterExplicit,
};
pub struct Assembler<'a> {
    pub buffer: &'a mut CodeBuffer,
    last_error: Option<AsmError>,
}

impl<'a> Assembler<'a> {
    pub fn new(buffer: &'a mut CodeBuffer) -> Self {
        Assembler {
            buffer,
            last_error: None,
        }
    }

    fn use_label(&mut self, label: &Operand, kind: LabelUse) {
        let off = self.buffer.cur_offset();
        self.buffer
            .use_label_at_offset(off, label.as_::<Label>(), kind);
    }
}

macro_rules! enc_ops1 {
    ($op0:ident) => {
        OperandType::$op0 as u32
    };
}

macro_rules! enc_ops2 {
    ($op0:ident, $op1:ident) => {
        (OperandType::$op0 as u32) | ((OperandType::$op1 as u32) << 3)
    };
}

macro_rules! enc_ops3 {
    ($op0:ident, $op1:ident, $op2:ident) => {
        (OperandType::$op0 as u32)
            | ((OperandType::$op1 as u32) << 3)
            | ((OperandType::$op2 as u32) << 6)
    };
}

macro_rules! enc_ops4 {
    ($op0:ident, $op1:ident, $op2:ident, $op3:ident) => {
        (OperandType::$op0 as u32)
            | ((OperandType::$op1 as u32) << 3)
            | ((OperandType::$op2 as u32) << 6)
            | ((OperandType::$op3 as u32) << 9)
    };
}

impl<'a> Emitter for Assembler<'a> {
    fn emit_n(&mut self, opcode: i64, ops: &[&Operand]) {
        assert!(opcode < Opcode::LAST as i64);

        let opcode: Opcode = unsafe { core::mem::transmute(opcode as u16) };

        let info = &INST_INFO[opcode as usize];

        let isign3 = match ops {
            [] => 0,
            [op0] => op0.op_type() as u32,
            [op0, op1] => op0.op_type() as u32 + ((op1.op_type() as u32) << 3),
            [op0, op1, op2, ..] => {
                op0.op_type() as u32 + ((op1.op_type() as u32) << 3) + ((op2.op_type() as u32) << 6)
            }
        };

        let isign4 = match ops {
            [] => 0,
            [op0] => op0.op_type() as u32,
            [op0, op1] => op0.op_type() as u32 + ((op1.op_type() as u32) << 3),
            [op0, op1, op2] => {
                op0.op_type() as u32 + ((op1.op_type() as u32) << 3) + ((op2.op_type() as u32) << 6)
            }
            [op0, op1, op2, op3, ..] => {
                op0.op_type() as u32
                    + ((op1.op_type() as u32) << 3)
                    + ((op2.op_type() as u32) << 6)
                    + ((op3.op_type() as u32) << 9)
            }
        };


        let err = match info.encoding {
            Encoding::Empty => {
                self.buffer.put4(info.val);
                return;
            }

            Encoding::CondRelAddr19 => {
                if isign3 == enc_ops2!(Imm, Label) {
                    let cond = ops[0].as_::<Imm>().value() as u32;

                    self.use_label(ops[1], LabelUse::A64Branch19);
                    self.buffer
                        .put4(info.val | (0 & 0x7ffff) << 5 | ((cond & 0xf) << 0));
                    return;
                } else if isign3 == enc_ops2!(Imm, Imm) {
                    let cond = ops[0].as_::<Imm>().value() as u32;
                    let imm = ops[1].as_::<Imm>().value() as u32;
                    self.buffer
                        .put4(info.val | ((imm & 0x7ffff) << 5) | ((cond & 0xf) << 0));
                    return;
                } else {
                    Some(AsmError::InvalidOperand)
                }
            }

            Encoding::Const0 => {
                self.buffer
                     .put4(info.val | (0 & 0xf) << 8);
                return;
            }

            Encoding::Const15 => {
                self.buffer.put4(info.val | (15 & 0xf) << 8);
                return;
            }

            _ => None,
        };
        self.last_error = err;
    }

    fn emit(&mut self, opcode: i64, op0: &Operand, op1: &Operand, op2: &Operand, op3: &Operand) {
        todo!()
    }
}

impl<'a> EmitterExplicit for Assembler<'a> {}
