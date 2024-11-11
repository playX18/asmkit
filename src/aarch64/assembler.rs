use crate::aarch64::opcodes::{Encoding, Opcode, INST_INFO};
use crate::core::buffer::LabelUse;
use crate::core::operand::*;
use crate::core::{buffer::CodeBuffer, emitter::Emitter};
use crate::AsmError;

use super::emitter::A64EmitterExplicit;
pub struct Assembler<'a> {
    pub buffer: &'a mut CodeBuffer,
    last_error: Option<AsmError>,
}

impl<'a> Assembler<'a> {
    pub fn last_error(&self) -> Option<&AsmError> {
        self.last_error.as_ref()
    }

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
                self.buffer.put4(info.val | (0 & 0xf) << 8);
                return;
            }

            Encoding::Const15 => {
                self.buffer.put4(info.val | (15 & 0xf) << 8);
                return;
            }

            Encoding::FpConst0 => {
                if isign3 == enc_ops1!(Reg) {
                    let rn = ops[0].id();
                    self.buffer
                        .put4(info.val | (0 & 0x1f) << 16 | (rn & 0x1f) << 5);
                    return;
                }

                Some(AsmError::InvalidOperand)
            }

            Encoding::FpFp => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rm = ops[0].id();
                    let rn = ops[1].id();
                    self.buffer
                        .put4(info.val | (rm & 0x1f) << 16 | (rn & 0x1f) << 5);
                    return;
                }

                Some(AsmError::InvalidOperand)
            }

            Encoding::FpFpConst0 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    self.buffer
                        .put4(info.val | (0 & 0x7) << 16 | (rn & 0x1f) << 5 | (rd & 0x1f) << 0);
                    return;
                }

                Some(AsmError::InvalidOperand)
            }

            Encoding::FpFpFp => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();

                    self.buffer.put4(
                        info.val | ((rm & 0x1f) << 16) | (rn & 0x1f) << 5 | ((rd & 0x1f) << 0),
                    );
                    return;
                }

                Some(AsmError::InvalidOperand)
            }

            Encoding::FpFpFpCond => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();
                    let cond = ops[3].as_::<Imm>().value();

                    self.buffer.put4(
                        info.val
                            | ((rm & 0x1f) << 16)
                            | ((cond as u32 & 0xf) << 12)
                            | (rn & 0x1f) << 5
                            | ((rd & 0x1f) << 0),
                    );
                    return;
                }
                Some(AsmError::InvalidOperand)
            }

            Encoding::FpFpFpFp => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();
                    let ra = ops[3].id();

                    self.buffer.put4(
                        info.val
                            | ((rm & 0x1f) << 16)
                            | ((ra & 0x1f) << 10)
                            | (rn & 0x1f) << 5
                            | ((rd & 0x1f) << 0),
                    );
                    return;
                }

                Some(AsmError::InvalidOperand)
            }

            Encoding::FpFpFpImm => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();
                    let imm4 = ops[3].as_::<Imm>().value() as i32 as u32;

                    self.buffer.put4(
                        info.val
                            | ((rm & 0x1f) << 16)
                            | ((imm4 & 0xf) << 11)
                            | ((rn & 0x1f) << 5)
                            | ((rd & 0x1f) << 0),
                    );
                    return;
                }

                Some(AsmError::InvalidOperand)
            }

            Encoding::FpFpFpImmRotAdd => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();
                    let rot = ops[3].as_::<Imm>().value() as i32 as u32;
                    if rot == 90 || rot == 270 {
                        self.buffer.put4(
                            info.val
                                | ((rm & 0x1f) << 16)
                                | (((rot / 90 - 1) & 0x3) << 11)
                                | ((rn & 0x1f) << 5)
                                | ((rd & 0x1f) << 0),
                        );
                        return;
                    } else {
                        Some(AsmError::InvalidOperand)
                    }
                } else {
                    Some(AsmError::InvalidOperand)
                }
            }

            Encoding::FpFpFpImmRotMul => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();
                    let rot = ops[3].as_::<Imm>().value() as i32 as u32;
                    if rot < 360 && rot % 90 == 0 {
                        self.buffer.put4(
                            info.val
                                | ((rm & 0x1f) << 16)
                                | (((rot / 90) & 0x3) << 11)
                                | ((rn & 0x1f) << 5)
                                | ((rd & 0x1f) << 0),
                        );
                        return;
                    } else {
                        Some(AsmError::InvalidOperand)
                    }
                } else {
                    Some(AsmError::InvalidOperand)
                }
            }

            Encoding::FpFpFpVelElemIdx0_1 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let mrm = ops[2].id();
                    let elemidx = ops[3].as_::<Imm>().value() as u32;
                    // TODO: Broken
                    if mrm < 16 && elemidx < (1 << (4 - 1)) {
                        let v = info.val ^ mrm << 16
                            | ((((elemidx << 1) >> 2) & 0x1) << 21)
                            | ((((elemidx << 1) >> 1) & 0x1) << 20)
                            | (((((elemidx << 1) & 1) << 3) & 0xf) << 16)
                            | ((((elemidx << 1) >> 3) & 0x1) << 11)
                            | ((rn & 0x1f) << 5)
                            | ((rd & 0x1f) << 0);

                        self.buffer.put4(v);
                        return;
                    } else {
                        Some(AsmError::InvalidOperand)
                    }
                } else {
                    Some(AsmError::InvalidOperand)
                }
            }

            Encoding::FpFpFpVelElemIdx1 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let mrm = ops[2].id();
                   
                    let elemidx = ops[3].id() as u32;
                    let val = (info.val)
                        | ((((elemidx << 1) >> 2) & 0x1) << 21)
                        | ((((elemidx << 1) >> 1) & 0x1) << 20)
                        | (((0) & 0xf) << 16)
                        | ((((elemidx << 1) >> 3) & 0x1) << 11)
                        | ((mrm & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                }

                Some(AsmError::InvalidOperand)
            }

            _ => None,
        };
        self.last_error = err;
    }

    fn emit(&mut self, opcode: i64, op0: &Operand, op1: &Operand, op2: &Operand, op3: &Operand) {
        let _ = opcode;
        let _ = op0;
        let _ = op1;
        let _ = op2;
        let _ = op3;
        todo!()
    }
}

impl<'a> A64EmitterExplicit for Assembler<'a> {}
