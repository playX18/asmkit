use crate::aarch64::opcodes::{Encoding, Opcode, INST_INFO};
use crate::core::buffer::{LabelUse, Reloc, RelocTarget};
use crate::core::operand::*;
use crate::core::{buffer::CodeBuffer, emitter::Emitter};
use crate::AsmError;

use super::emitter::A64EmitterExplicit;
pub struct Assembler<'a> {
    pub buffer: &'a mut CodeBuffer,
    last_error: Option<AsmError>,
}

fn imm_add(value: i64) -> u32 {
    let mut inst = 0;
    let mut uval = value as u64;
    if value < 0 {
        inst = 0x40000000;
        uval = !uval;
    }

    if (uval & 0xfff) == uval {
        return inst | (uval << 10) as u32;
    }

    if (uval & 0xfff000) == uval {
        return inst | 1 << 22 | (uval >> 2) as u32;
    }

    u32::MAX
}

fn imm_logical(mut value: u64, is64: bool) -> u32 {
    if !is64 {
        value = value | (value << 32);
    }

    if value == 0 || !value == 0 {
        return u32::MAX;
    }

    let clz = value.leading_zeros();
    let iclz = (!value).leading_zeros();
    let ctz = value.trailing_zeros();
    let ictz = (!value).trailing_zeros();
    let mut popcount = value.count_ones();
    let mut elog = 0;
    let mut shift = 64;
    let mut imms = (0x1780) | (popcount - 1);
    while elog < 6 {
        if (clz + ctz) == (shift - popcount) {
            return (((if ctz != 0 { shift - ctz } else { 0 }) << 6) | (imms & 0x103f)) << 10;
        } else if (iclz + ictz) == popcount {
            return ((iclz << 6) | (imms & 0x103f)) << 10;
        }
        elog += 1;
        popcount >>= 1;
        shift >>= 1;
        imms >>= 1;
        let mask = (1u64 << shift) - 1;
        if (value & mask) != (value >> shift & mask) {
            break;
        }
    }

    unreachable!()
}

fn imm_fmov32(value: f32) -> u32 {
    let vi = value.to_bits();
    if (vi & 0x7ffff) == 0 && ((vi >> 25 & 0x3f) - 0x1f) <= 1 {
        return (vi >> 19 & 0x7f) | (vi >> 24 & 0x80);
    }
    return 0xffffffff;
}

fn imm_fmov64(value: f64) -> u64 {
    let vi = value.to_bits();
    if (vi & 0xffffffffffff) != 0 && ((vi >> 54 & 0x1ff) - 0xff) <= 1 {
        return (vi >> 48 & 0x7f) | (vi >> 56 & 0x80);
    }
    return 0xffffffff;
}

const fn movi_encode(imm8: u32, op: u32, cmode: u32) -> u32 {
    let x = if op != 0 { 0x20000000 } else { 0 };
    x | ((cmode) << 12) | (((imm8) << 11) & 0x70000) | (((imm8) << 5) & 0x3e0)
}

fn imm_simdmovi(value: u64) -> u32 {
    let imm8 = || {
        return movi_encode((value & 0xff) as u32, 0, 0xe);
    };
    let mask64 = || {
        let mut imm8 = 0u32;
        for i in 0..8 {
            let byte = value >> 8 * i & 0xff;
            if byte == 0xff {
                imm8 |= 1 << i;
            } else if byte != 0 {
                return u32::MAX;
            }
        }

        return movi_encode(imm8, 1, 0xe);
    };

    if value == 0 || !value == 0 {
        return imm8();
    }
    if (value & u32::MAX as u64) != value >> 32 {
        return mask64();
    }

    if (value & 0xffff) != (value >> 16 & 0xffff) {
        let value32 = value as u32;
        let clz = value32.leading_zeros() >> 3;
        let iclz = (!value32).leading_zeros() >> 3;
        let ctz = value32.trailing_zeros() >> 3;
        let ictz = (!value32).trailing_zeros() >> 3;

        if clz + ctz >= 3 {
            return movi_encode((value >> ctz * 8) as u32, 0, ctz * 2);
        }
        if iclz + ictz >= 3 {
            return movi_encode((!value >> ictz * 8) as u32, 1, ictz * 2);
        }
        if clz + ictz >= 3 {
            return movi_encode((value >> ictz * 8) as u32, 0, 0xc + ictz - 1);
        }

        if iclz + ctz >= 3 {
            return movi_encode((!value >> ctz * 8) as u32, 1, 0xc + ctz - 1);
        }

        return mask64();
    }

    if (value & 0xff) != (value >> 8 & 0xff) {
        let low8 = value & 0xff;
        let high8 = value >> 8 & 0xff;

        if high8 == 0 {
            return movi_encode(low8 as _, 0, 0x8);
        }

        if high8 == 0xff {
            return movi_encode((!low8) as u32, 1, 0x8);
        }

        if low8 == 0 {
            return movi_encode(high8 as _, 0, 0xa);
        }

        if low8 == 0xff {
            return movi_encode((!high8) as u32, 1, 0xa);
        }
    }

    imm8()
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

        let mut err = None;

        match info.encoding {
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
            }

            Encoding::FpFp => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rm = ops[0].id();
                    let rn = ops[1].id();
                    self.buffer
                        .put4(info.val | (rm & 0x1f) << 16 | (rn & 0x1f) << 5);
                    return;
                }
            }

            Encoding::FpFpConst0 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    self.buffer
                        .put4(info.val | (0 & 0x7) << 16 | (rn & 0x1f) << 5 | (rd & 0x1f) << 0);
                    return;
                }
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
                    }
                } else {
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
                    }
                } else {
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
                    }
                } else {
                }
            }

            Encoding::FpFpFpVelElemIdx1 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let mrm = ops[2].id();

                    let elemidx = ops[3].id() as u32;
                    let val = (info.val ^ mrm << 16)
                        | ((((elemidx << 1) >> 2) & 0x1) << 21)
                        | ((((elemidx << 1) >> 1) & 0x1) << 20)
                        | (((0) & 0xf) << 16)
                        | ((((elemidx << 1) >> 3) & 0x1) << 11)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpFpVelElemIdx2 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let mrm = ops[2].id();

                    let elemidx = ops[3].id() as u32;
                    let val = (info.val ^ mrm << 16)
                        | ((((elemidx << 1) >> 2) & 0x1) << 21)
                        | ((((elemidx << 1) >> 1) & 0x1) << 20)
                        | (((0) & 0xf) << 16)
                        | ((((elemidx << 1) >> 3) & 0x1) << 11)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpFpVelElemIdx3 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let mrm = ops[2].id();

                    let elemidx = ops[3].id() as u32;
                    let val = (info.val ^ mrm << 16)
                        | ((((elemidx << 2) >> 2) & 0x1) << 21)
                        | ((((elemidx << 2) >> 1) & 0x1) << 20)
                        | (((0) & 0xf) << 16)
                        | ((((elemidx << 2) >> 3) & 0x1) << 11)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpFpVelElemIdxLim2_2ImmRotMul => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) && ops.len() == 5 && ops[4].is_imm() {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let mrm = ops[2].id();

                    let elemidx = ops[3].id() as u32;
                    let rot = ops[4].id() as u32;
                    let val = (info.val ^ mrm << 16)
                        | ((((elemidx << 2) >> 2) & 0x1) << 21)
                        | ((((elemidx << 2) >> 1) & 0x1) << 20)
                        | (((0) & 0xf) << 16)
                        | (((rot / 90) & 0x3) << 13)
                        | ((((elemidx << 2) >> 3) & 0x1) << 11)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpFpVelElemIdxLim2_4ImmRotMul => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) && ops.len() == 5 && ops[4].is_imm() {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let mrm = ops[2].id();

                    let elemidx = ops[3].id() as u32;
                    let rot = ops[4].id() as u32;
                    let val = (info.val ^ mrm << 16)
                        | ((((elemidx << 2) >> 2) & 0x1) << 21)
                        | ((((elemidx << 2) >> 1) & 0x1) << 20)
                        | (((0) & 0xf) << 16)
                        | (((rot / 90) & 0x3) << 13)
                        | ((((elemidx << 2) >> 3) & 0x1) << 11)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                }
            }
            //
            Encoding::FpFpFpVelElemIdxLim3_4ImmRotMul => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) && ops.len() == 5 && ops[4].is_imm() {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let mrm = ops[2].id();

                    let elemidx = ops[3].id() as u32;
                    let rot = ops[4].id() as u32;
                    let val = (info.val ^ mrm << 16)
                        | ((((elemidx << 3) >> 2) & 0x1) << 21)
                        | ((((elemidx << 3) >> 1) & 0x1) << 20)
                        | (((0) & 0xf) << 16)
                        | (((rot / 90) & 0x3) << 13)
                        | ((((elemidx << 3) >> 3) & 0x1) << 11)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                }
            }
            Encoding::FpFpGpSImm7_2 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rt2 = ops[1].id();
                    let rn = ops[2].id();
                    let imm7 = ops[3].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | (((imm7 / (1 << 2)) & 0x7f) << 15)
                        | ((rt2 & 0x1f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpGpSImm7_3 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rt2 = ops[1].id();
                    let rn = ops[2].id();
                    let imm7 = ops[3].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | (((imm7 / (1 << 3)) & 0x7f) << 15)
                        | ((rt2 & 0x1f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpGpSImm7_4 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rt2 = ops[1].id();
                    let rn = ops[2].id();
                    let imm7 = ops[3].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | (((imm7 / (1 << 4)) & 0x7f) << 15)
                        | ((rt2 & 0x1f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmCond => {
                if isign4 == enc_ops4!(Reg, Reg, Imm, Imm) {
                    let rn = ops[0].id();
                    let rm = ops[1].id();
                    let nzcv = ops[2].as_::<Imm>().value() as u32;
                    let cond = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | ((rm & 0x1f) << 16)
                        | ((cond & 0xf) << 12)
                        | ((rn & 0x1f) << 5)
                        | ((nzcv & 0xf) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmShiftl16 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as u32;

                    let val = (info.val ^ imm << 16)
                        | ((0 & 0xf) << 19)
                        | ((0 & 0x7) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmShiftl32 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as u32;

                    let val = (info.val ^ imm << 16)
                        | ((0 & 0xf) << 19)
                        | ((0 & 0x7) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmShiftl64 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as u32;

                    let val = ((info.val ^ imm) << 16)
                        | ((0 & 0xf) << 19)
                        | ((0 & 0x7) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmShiftl8 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32 as u32;

                    let val = (info.val ^ imm << 16)
                        | ((0 & 0xf) << 19)
                        | ((0 & 0x7) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmShiftr8 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as u32;

                    let val = (info.val ^ 8u32.wrapping_sub(imm << 16))
                        | ((0 & 0xf) << 19)
                        | ((0 & 0x7) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmShiftr16 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as u32;

                    let val = (info.val ^ 16u32.wrapping_sub(imm << 16))
                        | ((0 & 0xf) << 19)
                        | ((0 & 0x7) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmShiftr32 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as u32;

                    let val = (info.val ^ 32u32.wrapping_sub(imm << 16))
                        | ((0 & 0xf) << 19)
                        | ((0 & 0x7) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmShiftr64 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as u32;

                    let val = (info.val ^ 64u32.wrapping_sub(imm << 16))
                        | ((0 & 0xf) << 19)
                        | ((0 & 0x7) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmVIdx0_1 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm5 = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm5 << (0 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmVIdx1_1 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm5 = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm5 << (1 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpFpImmVIdx2_1 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm5 = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm5 << (2 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }
            Encoding::FpFpImmVIdx3_1 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm5 = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm5 << (3 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpGpFcvtFixScale => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let fbits = ops[2].as_::<Imm>().value() as u32;
                    let val = info.val
                        | (((64 - fbits) & 0x3f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpGpGp => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();

                    let val =
                        info.val | ((rm & 0x1f) << 16) | ((rn & 0x1f) << 5) | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpGpGpBool => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();
                    let sc = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | ((rm & 0x1f) << 16)
                        | ((sc & 0x1) << 12)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpGpSImm9_0 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let imm9 = ops[2].as_::<Imm>().value() as i32 as u32;
                    let val = info.val
                        | (((imm9 / (1 << 0)) & 0x1ff) << 12)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpGpUImm12_0 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let imm12 = ops[2].as_::<Imm>().value() as i32 as u32;
                    let val = info.val
                        | (((imm12 >> 0) & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpGpUImm12_1 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let imm12 = ops[2].as_::<Imm>().value() as i32 as u32;
                    let val = info.val
                        | (((imm12 >> 1) & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpGpUImm12_2 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let imm12 = ops[2].as_::<Imm>().value() as i32 as u32;
                    let val = info.val
                        | (((imm12 >> 2) & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpGpUImm12_3 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let imm12 = ops[2].as_::<Imm>().value() as i32 as u32;
                    let val = info.val
                        | (((imm12 >> 3) & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpGpUImm12_4 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let imm12 = ops[2].as_::<Imm>().value() as i32 as u32;
                    let val = info.val
                        | (((imm12 >> 4) & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpGpZero => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();

                    let val =
                        info.val | (((31) & 0x1f) << 16) | ((rn & 0x1f) << 5) | ((rt & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmFMov32 => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value_f32();
                    let val = info.val | ((imm_fmov32(imm) & 0xff) << 13) | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmFMov64 => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value_f64();
                    let val =
                        info.val | (((imm_fmov64(imm) & 0xff) as u32) << 13) | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmSIMD8Fmov => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value_f32();
                    let val = info.val
                        | (((imm_fmov32(imm) >> 5) & 0x7) << 16)
                        | (((imm_fmov32(imm) & 0x1f) & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmSIMD8Lsl => {
                if isign3 == enc_ops3!(Reg, Imm, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as u32;
                    let lsl = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm >> 5) & 0x7) << 16)
                        | (((lsl >> 2 | 1) & 0x1f) << 12)
                        | (((imm & 0x1f) & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmSIMD8Movi => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm64 = ops[1].as_::<Imm>().value() as u64;

                    let val = (info.val ^ imm_simdmovi(imm64))
                        | ((0 & 0x1) << 29)
                        | ((0 & 0x7) << 16)
                        | ((0 & 0xf) << 12)
                        | ((0 & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmVIdx0_1FpImmVIdx0_0 => {
                if isign4 == enc_ops4!(Reg, Imm, Reg, Imm) {
                    let rd = ops[0].id();
                    let imm5 = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();
                    let imm4 = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm5 << (0 + 1)) & 0x1f) << 16)
                        | (((imm4 << (0 + 0)) & 0xf) << 11)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmVIdx0_1Gp => {
                if enc_ops3!(Reg, Imm, Reg) == isign3 {
                    let rd = ops[0].id();
                    let imm5 = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();

                    let val = info.val
                        | (((imm5 << (0 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmVIdx1_1FpImmVIdx1_0 => {
                if isign4 == enc_ops4!(Reg, Imm, Reg, Imm) {
                    let rd = ops[0].id();
                    let imm5 = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();
                    let imm4 = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm5 << (1 + 1)) & 0x1f) << 16)
                        | (((imm4 << (0 + 0)) & 0xf) << 11)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmVIdx1_1Gp => {
                if enc_ops3!(Reg, Imm, Reg) == isign3 {
                    let rd = ops[0].id();
                    let imm5 = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();

                    let val = info.val
                        | (((imm5 << (1 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmVIdx2_1FpImmVIdx2_0 => {
                if isign4 == enc_ops4!(Reg, Imm, Reg, Imm) {
                    let rd = ops[0].id();
                    let imm5 = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();
                    let imm4 = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm5 << (2 + 1)) & 0x1f) << 16)
                        | (((imm4 << (2 + 0)) & 0xf) << 11)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmVIdx2_1Gp => {
                if enc_ops3!(Reg, Imm, Reg) == isign3 {
                    let rd = ops[0].id();
                    let imm5 = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();

                    let val = info.val
                        | (((imm5 << (2 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmVIdx3_1FpImmVIdx3_0 => {
                if isign4 == enc_ops4!(Reg, Imm, Reg, Imm) {
                    let rd = ops[0].id();
                    let imm5 = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();
                    let imm4 = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm5 << (3 + 1)) & 0x1f) << 16)
                        | (((imm4 << (3 + 0)) & 0xf) << 11)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpImmVIdx3_1Gp => {
                if enc_ops3!(Reg, Imm, Reg) == isign3 {
                    let rd = ops[0].id();
                    let imm5 = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();

                    let val = info.val
                        | (((imm5 << (3 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpMemSIMDIdx0Gp => {
                if isign3 == enc_ops3!(Reg, Imm, Reg) {
                    let rt = ops[0].id();
                    let elemidx = ops[1].as_::<Imm>().value() as usize as u32;
                    let rn = ops[2].id();

                    let val = info.val
                        | ((((elemidx << 0) >> 3) & 0x1) << 30)
                        | ((((elemidx << 0) >> 2) & 0x1) << 12)
                        | (((elemidx << 0) & 0x3) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpMemSIMDIdx0GpGp => {
                if isign4 == enc_ops4!(Reg, Imm, Reg, Reg) {
                    let rt = ops[0].id();
                    let elemidx = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();
                    let rm = ops[3].id();

                    let val = info.val
                        | ((((elemidx << 0) >> 3) & 0x1) << 30)
                        | ((rm & 0x1f) << 16)
                        | ((((elemidx << 0) >> 2) & 0x1) << 12)
                        | (((elemidx << 0) & 0x3) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpMemSIMDIdx0GpZero => {
                if isign3 == enc_ops3!(Reg, Imm, Reg) {
                    let rt = ops[0].id();
                    let elemidx = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();

                    let val = info.val
                        | ((((elemidx << 0) >> 3) & 0x1) << 30)
                        | (((31) & 0x1f) << 16)
                        | ((((elemidx << 0) >> 2) & 0x1) << 12)
                        | (((elemidx << 0) & 0x3) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpMemSIMDIdx1GpGp => {
                if isign4 == enc_ops4!(Reg, Imm, Reg, Reg) {
                    let rt = ops[0].id();
                    let elemidx = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();
                    let rm = ops[3].id();

                    let val = info.val
                        | ((((elemidx << 1) >> 3) & 0x1) << 30)
                        | ((rm & 0x1f) << 16)
                        | ((((elemidx << 1) >> 2) & 0x1) << 12)
                        | (((elemidx << 1) & 0x3) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpMemSIMDIdx1GpZero => {
                if isign3 == enc_ops3!(Reg, Imm, Reg) {
                    let rt = ops[0].id();
                    let elemidx = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();

                    let val = info.val
                        | ((((elemidx << 1) >> 3) & 0x1) << 30)
                        | (((31) & 0x1f) << 16)
                        | ((((elemidx << 1) >> 2) & 0x1) << 12)
                        | (((elemidx << 1) & 0x3) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpMemSIMDIdx2Gp => {
                if isign4 == enc_ops4!(Reg, Imm, Reg, Reg) {
                    let rt = ops[0].id();
                    let elemidx = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[1].id();
                    let rm = ops[2].id();

                    let val = info.val
                        | ((((elemidx << 2) >> 3) & 0x1) << 30)
                        | ((rm & 0x1f) << 16)
                        | ((((elemidx << 2) >> 2) & 0x1) << 12)
                        | (((elemidx << 2) & 0x3) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpMemSIMDIdx2GpGp => {
                if isign4 == enc_ops4!(Reg, Imm, Reg, Reg) {
                    let rt = ops[0].id();
                    let elemidx = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();
                    let rm = ops[3].id();

                    let val = info.val
                        | ((((elemidx << 2) >> 3) & 0x1) << 30)
                        | ((rm & 0x1f) << 16)
                        | ((((elemidx << 2) >> 2) & 0x1) << 12)
                        | (((elemidx << 2) & 0x3) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpMemSIMDIdx2GpZero => {
                if isign3 == enc_ops3!(Reg, Imm, Reg) {
                    let rt = ops[0].id();
                    let elemidx = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();

                    let val = info.val
                        | ((((elemidx << 2) >> 3) & 0x1) << 30)
                        | (((31) & 0x1f) << 16)
                        | ((((elemidx << 2) >> 2) & 0x1) << 12)
                        | (((elemidx << 2) & 0x3) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpMemSIMDIdx3Gp => {
                if isign4 == enc_ops4!(Reg, Imm, Reg, Reg) {
                    let rt = ops[0].id();
                    let elemidx = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[1].id();
                    let rm = ops[2].id();

                    let val = info.val
                        | ((((elemidx << 3) >> 3) & 0x1) << 30)
                        | ((rm & 0x1f) << 16)
                        | ((((elemidx << 3) >> 2) & 0x1) << 12)
                        | (((elemidx << 3) & 0x3) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpMemSIMDIdx3GpGp => {
                if isign4 == enc_ops4!(Reg, Imm, Reg, Reg) {
                    let rt = ops[0].id();
                    let elemidx = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();
                    let rm = ops[3].id();

                    let val = info.val
                        | ((((elemidx << 3) >> 3) & 0x1) << 30)
                        | ((rm & 0x1f) << 16)
                        | ((((elemidx << 3) >> 2) & 0x1) << 12)
                        | (((elemidx << 3) & 0x3) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpMemSIMDIdx3GpZero => {
                if isign3 == enc_ops3!(Reg, Imm, Reg) {
                    let rt = ops[0].id();
                    let elemidx = ops[1].as_::<Imm>().value() as u32;
                    let rn = ops[2].id();

                    let val = info.val
                        | ((((elemidx << 3) >> 3) & 0x1) << 30)
                        | (((31) & 0x1f) << 16)
                        | ((((elemidx << 3) >> 2) & 0x1) << 12)
                        | (((elemidx << 3) & 0x3) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::FpRelAddr19 => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rt = ops[0].id();
                    let imm19 = ops[1].as_::<Imm>().value() as isize as i32 as u32;

                    let val = info.val | ((imm19 & 0x7ffff) << 5) | ((rt & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                } else if isign3 == enc_ops2!(Reg, Label) {
                    let rt = ops[0].id();

                    self.use_label(ops[1], LabelUse::A64Branch19);

                    let val = info.val | ((0 & 0x7ffff) << 5) | ((rt & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpFp => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();

                    let val = info.val | ((rn & 0x1f) << 5) | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpFpFcvtFixScale => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let fbits = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((64 - fbits) & &0x3f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpFpImmVIdx0_1 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm5 = ops[2].as_::<Imm>().value() as u32;
                    let val = info.val
                        | (((imm5 << (0 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpFpImmVIdx1_1 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm5 = ops[2].as_::<Imm>().value() as u32;
                    let val = info.val
                        | (((imm5 << (1 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpFpImmVIdx2_1 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm5 = ops[2].as_::<Imm>().value() as u32;
                    let val = info.val
                        | (((imm5 << (2 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpFpImmVIdx3_1 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm5 = ops[2].as_::<Imm>().value() as u32;
                    let val = info.val
                        | (((imm5 << (3 + 1)) & 0x1f) << 16)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpConst0Const0 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();

                    let val = info.val
                        | ((0 & 0x3) << 22)
                        | ((0 & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpConst0Const15 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();

                    let val = info.val
                        | ((0 & 0x3) << 16)
                        | ((15 & 0x3f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpConst0Const31 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();

                    let val = info.val
                        | ((0 & 0x3) << 16)
                        | ((31 & 0x3f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }
            Encoding::GpGpConst0Const7 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();

                    let val = info.val
                        | ((0 & 0x3) << 16)
                        | ((7 & 0x3f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpGpBool => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();
                    let sc = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | ((rm & 0x1f) << 16)
                        | ((sc & 0x1) << 12)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpGpCond => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();
                    let sc = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | ((rm & 0x1f) << 16)
                        | ((sc & 0xf) << 12)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpGpGp => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Reg) {
                    let rs = ops[0].id();
                    let rt = ops[1].id();
                    let rt2 = ops[2].id();
                    let rn = ops[3].id();

                    let val = info.val
                        | ((rs & 0x1f) << 16)
                        | ((rt2 & 0x1f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpGpImm => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();
                    let imm3 = ops[3].as_::<Imm>().value() as i32 as u32;

                    self.buffer.put4(
                        info.val
                            | ((rm & 0x1f) << 16)
                            | ((imm3 & 0x7) << 10)
                            | ((rn & 0x1f) << 5)
                            | ((rd & 0x1f) << 0),
                    );
                    return;
                }
            }

            Encoding::GpGpGpSImm7_2 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rt2 = ops[1].id();
                    let rn = ops[2].id();
                    let imm7 = ops[3].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | (((imm7 / (1 << 2)) & 0x7f) << 15)
                        | ((rt2 & 0x1f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpGpSImm7_3 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rt2 = ops[1].id();
                    let rn = ops[2].id();
                    let imm7 = ops[3].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | (((imm7 / (1 << 3)) & 0x7f) << 15)
                        | ((rt2 & 0x1f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }
            Encoding::GpGpGpSImm7_4 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rt2 = ops[1].id();
                    let rn = ops[2].id();
                    let imm7 = ops[3].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | (((imm7 / (1 << 4)) & 0x7f) << 15)
                        | ((rt2 & 0x1f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpGpZero => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();

                    let val = info.val
                        | ((rm & 0x1f) << 16)
                        | ((31 & 0x1f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpImmAdd32 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value();

                    let val = (info.val ^ imm_add(imm))
                        | ((0 & 0x3) << 22)
                        | ((0 & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpImmAdd64 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value();

                    let val = (info.val ^ imm_add(imm))
                        | ((0 & 0x3) << 22)
                        | ((0 & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpImmCond => {
                if isign4 == enc_ops4!(Reg, Reg, Imm, Imm) {
                    let rn = ops[0].id();
                    let rm = ops[1].id();
                    let nzcv = ops[2].as_::<Imm>().value() as u32;
                    let cond = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | ((rm & 0x1f) << 16)
                        | ((cond & 0xf) << 12)
                        | ((rn & 0x1f) << 5)
                        | ((nzcv & 0xf) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            // rd, rn, immr
            Encoding::GpGpImmConst31 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let immr = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | ((immr & 0x3f) << 16)
                        | ((31 & 0x3f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpImmConst63 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let immr = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | ((immr & 0x3f) << 16)
                        | ((63 & 0x3f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            // Rd, Rn, immr, imms
            Encoding::GpGpImmImm => {
                if isign4 == enc_ops4!(Reg, Reg, Imm, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let immr = ops[2].as_::<Imm>().value() as u32;
                    let imms = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | ((immr & 0x3f) << 16)
                        | ((imms & 0x3f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            // rt, rn, simm9
            Encoding::GpGpImmLDraut => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let simm9 = ops[2].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | ((((simm9 >> 12) & 1) & 0x1) << 22)
                        | ((((simm9 & 0xff8) >> 3) & 0x1ff) << 12)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            // rd, rn, imm
            Encoding::GpGpImmLogical32 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as u64;

                    let val = (info.val ^ imm_logical(imm, (32 >> 6) != 0))
                        | ((0 & 0x3f) << 16)
                        | ((0 & 0x3f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            // rd, rn, imm
            Encoding::GpGpImmLogical64 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as u64;

                    let val = (info.val ^ imm_logical(imm, (64 >> 6) != 0))
                        | ((0 & 0x3f) << 16)
                        | ((0 & 0x3f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            // rd, rn, cond
            Encoding::GpGpInvCond => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let cond = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | ((rn & 0x1f) << 16)
                        | (((cond ^ 1) & 0xf) << 12)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            // rs, rt, rn
            Encoding::GpGpLs64Gp => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let rs = ops[0].id();
                    let rt = ops[1].id();
                    let rn = ops[2].id();

                    let val =
                        info.val | ((rs & 0x1f) << 16) | ((rt & 0x1f) << 5) | ((rn & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            // rd, rn, imm8
            Encoding::GpGpSImm8_0 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm8 = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm8 / (1 << 0)) & 0xff) << 10)
                        | (((rn) & 0x1f) << 5)
                        | (((rd) & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpSImm9_0 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm9 = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm9 / (1 << 0)) & 0x1ff) << 12)
                        | (((rn) & 0x1f) << 5)
                        | (((rd) & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpSImm9_4 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm9 = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm9 / (1 << 4)) & 0x1ff) << 12)
                        | (((rn) & 0x1f) << 5)
                        | (((rd) & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpUImm12_0 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let imm12 = ops[2].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | (((imm12 >> 0) & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpUImm12_1 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let imm12 = ops[2].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | (((imm12 >> 1) & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpUImm12_2 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let imm12 = ops[2].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | (((imm12 >> 2) & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGpUImm12_3 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rt = ops[0].id();
                    let rn = ops[1].id();
                    let imm12 = ops[2].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | (((imm12 >> 3) & 0xfff) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            // rd, rn, uimm6, uimm4
            Encoding::GpGpUImm6_4UImm4_0Const0 => {
                if isign4 == enc_ops4!(Reg, Reg, Imm, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm6 = ops[2].as_::<Imm>().value() as u32;
                    let imm4 = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0)
                        | ((0 & 0x3) << 14)
                        | (((imm4 >> 0) & 0xf) << 10)
                        | (((imm6 >> 4) & 0x3f) << 16);

                    self.buffer.put4(val);
                    return;
                }
            }

            // rd, rn, imm8
            Encoding::GpGpUImm8_0 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let imm8 = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm8 >> 0) & 0xff) << 12)
                        | ((rn & 0x1f) << 5)
                        | ((rd & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            // rs, rt, rn
            Encoding::GpGpZeroGp => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let rs = ops[0].id();
                    let rn = ops[1].id();
                    let rt = ops[2].id();

                    let val = info.val
                        | ((31 & 0x1f) << 10)
                        | ((rn & 0x1f) << 5)
                        | ((rt & 0x1f) << 0)
                        | ((rs & 0x1f) << 16);

                    self.buffer.put4(val);
                    return;
                }
            }

            // rd, rn, lsb, width
            Encoding::GpGplsbwidth => {
                if isign4 == enc_ops4!(Reg, Reg, Imm, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let lsb = ops[2].as_::<Imm>().value() as u32;
                    let width = ops[3].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((lsb + width - 1) & 0x3f) << 10)
                        | (((rn) & 0x1f) << 5)
                        | (((rd) & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGplsl32 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let lsl32 = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((32 - 1 - lsl32) & 0x3f) << 10)
                        | (((rn) & 0x1f) << 5)
                        | (((rd) & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpGplsl64 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let lsl64 = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((64 - 1 - lsl64) & 0x3f) << 10)
                        | (((rn) & 0x1f) << 5)
                        | (((rd) & 0x1f) << 0);
                    self.buffer.put4(val);
                    return;
                }
            }

            // rd, target
            Encoding::GpImmAddr => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let target = ops[1].as_::<Imm>().value() as i32 as u32;

                    let val = info.val
                        | (((target & 3) & 0x3) << 29)
                        | (((target >> 2) & 0x7ffff) << 5)
                        | ((rd & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                } else if isign3 == enc_ops2!(Reg, Label) {
                    let rd = ops[0].id();
                    self.use_label(&ops[1], LabelUse::A64Adr21);
                    let val = info.val | (rd & 0x1f) << 0;
                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpImmAddrP => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let target = ops[1].as_::<Imm>().value() as u64;

                    let val = info.val
                        | ((((target & !0xfffu64) >> 12) as u32 & 3) << 29)
                        | ((((target & !0xfffu64) >> 14) as u32 & 0x7ffff) << 5)
                        | ((rd & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                } else if isign3 == enc_ops2!(Reg, Label) {
                    let rd = ops[0].id();
                    self.use_label(&ops[1], LabelUse::A64Adr21);
                    let val = info.val | (rd & 0x1f) << 0;
                    self.buffer.put4(val);
                    return;
                } else if isign3 == enc_ops2!(Reg, Sym) {
                    let rd = ops[0].id();
                    self.buffer.add_reloc(
                        Reloc::Aarch64AdrGotPage21,
                        RelocTarget::Sym(ops[1].as_::<Sym>()),
                        0,
                    );
                    let val = info.val | (rd & 0x1f) << 0;
                    self.buffer.put4(val);
                    return;
                }
            }

            // rn, imm5, nzcv, cond
            Encoding::GpImmImmCond => {
                if isign4 == enc_ops4!(Reg, Imm, Imm, Imm) {
                    let rn = ops[0].id();
                    let imm5 = ops[1].as_::<Imm>().value() as u32;
                    let nzcv = ops[2].as_::<Imm>().value() as u32;
                    let cond = ops[3].as_::<Imm>().value() as u32;
                    let val = info.val
                        | ((imm5 & 0x1f) << 16)
                        | ((cond & 0xf) << 12)
                        | ((rn & 0x1f) << 5)
                        | ((nzcv & 0xf) << 0);

                    self.buffer.put4(val);
                    return;
                }
            }

            // rt, imm19
            Encoding::GpRelAddr19 => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rt = ops[0].id();
                    let imm19 = ops[1].as_::<Imm>().value() as i32 as u32;

                    let val = info.val | (((imm19 & 0x7ffff) << 5) & 0x7ffff0) | ((rt & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                } else if isign3 == enc_ops2!(Reg, Label) {
                    let rt = ops[0].id();
                    self.use_label(&ops[1], LabelUse::A64Ldr19);
                    let val = info.val | (rt & 0x1f) << 0;
                    self.buffer.put4(val);
                    return;
                }
            }
            // rt, bit, imm14
            Encoding::GpTBZRelAddr14 => {
                if isign3 == enc_ops3!(Reg, Imm, Label) {
                    let rt = ops[0].id();
                    let bit = ops[1].as_::<Imm>().value() as u32;

                    self.use_label(ops[2], LabelUse::A64Branch14);
                    let val = info.val
                        | (((bit >> 5) & 0x1) << 31)
                        | (((bit & 0x1f) & 0x1f) << 19)
                        | (((0 >> 2) & 0x3fff) << 5)
                        | ((rt & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                } else if isign3 == enc_ops3!(Reg, Imm, Imm) {
                    let rt = ops[0].id();
                    let bit = ops[1].as_::<Imm>().value() as u32;
                    let imm14 = ops[2].as_::<Imm>();

                    let val = info.val
                        | (((bit >> 5) & 0x1) << 31)
                        | (((bit & 0x1f) & 0x1f) << 19)
                        | (((imm14.value() as i32 as u32 >> 2) & 0x3fff) << 5)
                        | ((rt & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                }
            }

            // rd, imm16 e.g MOVNw
            Encoding::GpUImm16_0 => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm16 = ops[1].as_::<Imm>().value() as u32;

                    let val = info.val | (((imm16 >> 0) & 0xffff) << 5) | ((rd & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                }
            }

            // rd, imm16, hw
            Encoding::GpUImm16_0Imm => {
                if isign3 == enc_ops3!(Reg, Imm, Imm) {
                    let rd = ops[0].id();
                    let imm16 = ops[1].as_::<Imm>().value() as u32;
                    let hw = ops[2].as_::<Imm>().value() as u32;

                    let val = info.val
                        | (((imm16 >> 0) & 0xffff) << 5)
                        | (((hw >> 0) & 0x3) << 21)
                        | ((rd & 0x1f) << 0);

                    self.buffer.put4(val);
                    return;
                }
            }

            Encoding::GpZeroGpConst0 => {
                todo!()
            }

            Encoding::GpGpGp => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();

                    self.buffer.put4(
                        info.val
                            | ((rm & 0x1f) << 16)
                            | (0 << 10)
                            | (rn & 0x1f) << 5
                            | ((rd & 0x1f) << 0),
                    );
                    return;
                }
            }

            Encoding::GpGpGpConst0 => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let rd = ops[0].id();
                    let rn = ops[1].id();
                    let rm = ops[2].id();

                    self.buffer.put4(
                        info.val
                            | ((rm & 0x1f) << 16)
                            | (0 << 10)
                            | (rn & 0x1f) << 5
                            | ((rd & 0x1f) << 0),
                    );
                    return;
                }
            }

            _ => panic!("{:?}", info.encoding),
        };
        if err.is_none() {
            err = Some(AsmError::InvalidOperand);
        }
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
