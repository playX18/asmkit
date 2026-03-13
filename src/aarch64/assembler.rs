#![allow(clippy::eq_op, clippy::erasing_op, dead_code, unused)]
use crate::AsmError;
use crate::aarch64::operands::*;
use crate::aarch64::{Gp, Reg, ShiftOp, instdb::*};
use crate::core::buffer::{Constant, LabelUse, Reloc, RelocDistance, RelocTarget};
use crate::core::globals::CondCode;
use crate::core::operand::*;
use crate::core::{buffer::CodeBuffer, emitter::Emitter, patch::PatchSiteId};

macro_rules! B {
    ($e: expr) => {
        1 << $e
    };
}

macro_rules! check_signature {
    ($op0: expr, $op1: expr) => {
        $op0.signature() == $op1.signature()
    };
    ($op0: expr, $op1: expr, $op2: expr) => {
        $op0.signature() == $op1.signature() && $op1.signature() == $op2.signature()
    };

    ($op0: expr, $op1: expr, $op2: expr, $op3: expr) => {
        $op0.signature() == $op1.signature()
            && $op1.signature() == $op2.signature()
            && $op2.signature() == $op3.signature()
    };
}

pub struct Assembler<'a> {
    pub buffer: &'a mut CodeBuffer,
    last_error: Option<AsmError>,
}

macro_rules! enc_ops {
    ($op0: ident) => {
        OperandType::$op0 as u32
    };

    ($op0: ident, $op1: ident) => {
        OperandType::$op0 as u32 | (OperandType::$op1 as u32) << 3
    };
    ($op0: ident, $op1: ident, $op2: ident) => {
        OperandType::$op0 as u32 | (OperandType::$op1 as u32) << 3 | (OperandType::$op2 as u32) << 6
    };
    ($op0: ident, $op1: ident, $op2: ident, $op3: ident) => {
        OperandType::$op0 as u32
            | (OperandType::$op1 as u32) << 3
            | (OperandType::$op2 as u32) << 6
            | (OperandType::$op3 as u32) << 9
    };
}

pub trait LoadConstantEmitter<DST, SRC> {
    fn load_constant(&mut self, dst: DST, src: SRC);
}

impl LoadConstantEmitter<Gp, Constant> for Assembler<'_> {
    fn load_constant(&mut self, dst: Gp, src: Constant) {
        let label = self.buffer.get_label_for_constant(src);
        self.load_constant(dst, label);
    }
}

impl LoadConstantEmitter<Gp, Label> for Assembler<'_> {
    fn load_constant(&mut self, dst: Gp, src: Label) {
        let label_id = src.id();
        let offset = self.buffer.label_offset(src);

        self.adrp(dst, src);
        self.buffer
            .use_label_at_offset(self.buffer.cur_offset(), src, LabelUse::A64AddAbsLo12);
        self.add(dst, dst, imm(0));
    }
}

impl LoadConstantEmitter<Gp, Sym> for Assembler<'_> {
    fn load_constant(&mut self, dst: Gp, src: Sym) {
        let distance = self.buffer.symbol_distance(src);

        if self.buffer.env().pic() {
            // When PIC is enabled, all syms are referenced through the GOT.
            self.buffer
                .add_reloc(Reloc::Aarch64AdrGotPage21, RelocTarget::Sym(src), 0);
            self.adrp(dst, imm(0));
            self.buffer
                .add_reloc(Reloc::Aarch64Ld64GotLo12Nc, RelocTarget::Sym(src), 0);
            self.ldr(dst, ptr(dst, 0));
        }

        match distance {
            RelocDistance::Near => {
                self.buffer
                    .add_reloc(Reloc::Aarch64AdrPrelPgHi21, RelocTarget::Sym(src), 0);
                self.adrp(dst, imm(0));
                self.buffer
                    .add_reloc(Reloc::Aarch64AddAbsLo12Nc, RelocTarget::Sym(src), 0);
                self.add(dst, dst, imm(0));
                return;
            }

            RelocDistance::Far => {
                // With absolute offsets we set up a load from a preallocated space, and then jump
                // over it.
                //
                // Emit the following code:
                //   ldr     rd, #8
                //   b       #0x10
                //   <8 byte space>
                let constant_start = self.buffer.get_label();
                let constant_end = self.buffer.get_label();
                self.ldr(dst, label_ptr(constant_start, 0));
                self.b(constant_end);
                self.buffer.bind_label(constant_start);
                self.buffer.add_reloc(Reloc::Abs8, RelocTarget::Sym(src), 0);
                self.buffer.write_u64(0);
                self.buffer.bind_label(constant_end);
            }
        }
    }
}

impl<'a> Assembler<'a> {
    pub fn new(buffer: &'a mut CodeBuffer) -> Self {
        Self {
            buffer,
            last_error: None,
        }
    }

    pub fn get_label(&mut self) -> Label {
        self.buffer.get_label()
    }

    pub fn bind_label(&mut self, label: Label) {
        self.buffer.bind_label(label);
    }
    /// A helper to load a constant address into a register.
    ///
    /// Supported variants are:
    /// ```text
    /// +------------------+
    /// |  DST  |  SRC     |
    /// +------------------+
    /// |  Gp   | Label    |
    /// |  Gp   | Sym      |
    /// |  Gp   | Constant |
    /// +------------------+
    /// ```
    ///
    /// Note that `Sym` is loaded based on `self.buffer.pic()` and its distance. If PIC is enabled
    /// then GOT is always used. Otherwise, if symbol is near it uses `adrp` + `add` combination, and
    /// for far symbols Abs8 reloc is used and data is embedded right into code.
    pub fn load_constant<DST, SRC>(&mut self, dst: DST, src: SRC)
    where
        Self: LoadConstantEmitter<DST, SRC>,
    {
        <Self as LoadConstantEmitter<DST, SRC>>::load_constant(self, dst, src);
    }

    pub fn last_error(&self) -> Option<AsmError> {
        self.last_error.clone()
    }

    pub fn emit_n(&mut self, id: impl Into<u32>, ops: &[&Operand]) {
        let id = id.into();
        let inst_cc = InstId::extract_cc(id);
        let inst_id = InstId::extract_real_id(id);

        let inst_info = &INST_INFO_TABLE[inst_id as usize];
        let mut encoding_index = inst_info.encoding_data_index as usize;

        let mut opcode = Opc(0);

        let isign4;
        let inst_flags;

        const NOREG: &Operand = &Operand::new();

        let op0 = *ops.get(0).unwrap_or(&NOREG);
        let op1 = *ops.get(1).unwrap_or(&NOREG);
        let op2 = *ops.get(2).unwrap_or(&NOREG);
        let op3 = *ops.get(3).unwrap_or(&NOREG);
        let op4 = *ops.get(4).unwrap_or(&NOREG);
        let op5 = *ops.get(5).unwrap_or(&NOREG);

        let mut multiple_op_data = [0u32; 4];
        let mut multiple_op_count = 0;

        isign4 = op0.op_type() as u32
            + ((op1.op_type() as u32) << 3)
            + ((op2.op_type() as u32) << 6)
            + ((op3.op_type() as u32) << 9);
        inst_flags = inst_info.flags;
        let mut offset_format = OffsetFormat::new(OffsetType::SignedOffset, 0, 0, 0, 0, 0, 0, 0);
        let mut offset_value = 0;
        let mut label_use: Option<(u32, LabelUse)> = None;
        let mut reloc: Option<Reloc> = None;
        let mut rm_rel = &Operand::new();

        macro_rules! emit_disp_imm {
            () => {
                if (offset_value & ((1 << offset_format.imm_discard_lsb()) - 1)) != 0 {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                let disp_imm64 = (offset_value as i64) >> offset_format.imm_discard_lsb() as i64;
                let disp_imm32 = (disp_imm64 & (1 << offset_format.imm_bit_count()) - 1) as u32;

                match offset_format.typ() {
                    OffsetType::SignedOffset => {
                        opcode.add_imm(disp_imm32 as _, offset_format.imm_bit_shift() as _);
                        return self.buffer.write_u32(opcode.get());
                    }

                    _ => {
                        let imm_lo = disp_imm32 & 0x3;
                        let imm_hi = disp_imm32 >> 2;
                        opcode.add_imm(imm_lo, 29);
                        opcode.add_imm(imm_hi, 5);
                        return self.buffer.write_u32(opcode.get());
                    }
                }
            };
        }

        macro_rules! emit_rel {
            () => {
                if rm_rel.is_label() || (rm_rel.is_mem() && rm_rel.as_::<Mem>().has_base_label()) {
                    let label_id;
                    let mut label_offset = 0;

                    if rm_rel.is_label() {
                        label_id = rm_rel.as_::<Label>().id();
                    } else {
                        label_id = rm_rel.as_::<Mem>().base_id();
                        label_offset = rm_rel.as_::<Mem>().offset();
                    }

                    if self.buffer.is_bound(Label::from_id(label_id)) {
                        offset_value = self.buffer.label_offset(Label::from_id(label_id)) as i64
                            + label_offset
                            - self.buffer.cur_offset() as i64;
                        emit_disp_imm!();
                    } else {
                        let offset = self.buffer.cur_offset();
                        self.buffer.use_label_at_offset(
                            offset,
                            Label::from_id(label_id),
                            match offset_format.typ() {
                                OffsetType::Adrp => LabelUse::A64Adrp21,
                                OffsetType::Adr => LabelUse::A64Adr21,
                                OffsetType::Ldr => LabelUse::A64Ldr19,
                                OffsetType::SignedOffset => {
                                    if offset_format.imm_bit_count() == 26 {
                                        LabelUse::A64Branch26
                                    } else if offset_format.imm_bit_count() == 19 {
                                        LabelUse::A64Branch19
                                    } else if offset_format.imm_bit_count() == 14 {
                                        LabelUse::A64Branch14
                                    } else {
                                        panic!("Invalid offset format for label use")
                                    }
                                }
                            },
                        );

                        return self.buffer.write_u32(opcode.get());
                    }
                }

                if rm_rel.is_imm() {
                    let target_offset = rm_rel.as_::<Imm>().value_as::<u64>();
                    let mut pc = self.buffer.cur_offset() as u64 + 4;
                    if offset_format.typ() == OffsetType::Adrp {
                        pc &= !(4096 - 1);
                    }
                    offset_value = target_offset as i64 - pc as i64;
                    emit_disp_imm!();
                }
            };
        }

        macro_rules! emit_rd0 {
            () => {
                opcode.add_reg(op0.id(), 0);
                return self.buffer.write_u32(opcode.get());
            };
        }

        macro_rules! emit_rn5 {
            () => {
                opcode.add_reg(op0.id(), 5);
                return self.buffer.write_u32(opcode.get());
            };
        }

        macro_rules! emit_rn5_rm16 {
            () => {
                opcode.add_reg(op0.id(), 5);
                opcode.add_reg(op1.id(), 16);
                return self.buffer.write_u32(opcode.get());
            };
        }

        macro_rules! emit_rd0_rn5 {
            () => {
                opcode.add_reg(op0.id(), 0);
                opcode.add_reg(op1.id(), 5);
                return self.buffer.write_u32(opcode.get());
            };
        }

        macro_rules! emit_rd0_rn5_rm16_ra10 {
            () => {
                opcode.add_reg(op0.id(), 0);
                opcode.add_reg(op1.id(), 5);
                opcode.add_reg(op2.id(), 16);
                opcode.add_reg(op3.id(), 10);
                return self.buffer.write_u32(opcode.get());
            };
        }

        macro_rules! emit_rd0_rn5_rm16 {
            () => {
                opcode.add_reg(op0.id(), 0);
                opcode.add_reg(op1.id(), 5);
                opcode.add_reg(op2.id(), 16);
                return self.buffer.write_u32(opcode.get());
            };
        }

        macro_rules! emit_mem_base_rn5 {
            () => {
                opcode.add_reg(op0.as_::<Mem>().base_id(), 5);
                return self.buffer.write_u32(opcode.get());
            };
        }

        macro_rules! emit_mem_base_index_rn5_rm16 {
            () => {
                opcode.add_reg(op0.as_::<Mem>().base_id(), 5);
                opcode.add_reg(op0.as_::<Mem>().index_id(), 16);
                return self.buffer.write_u32(opcode.get());
            };
        }

        macro_rules! emit_mem_base_no_imm_rn5 {
            () => {
                opcode.add_reg(rm_rel.as_::<Mem>().base_id(), 5);
                return self.buffer.write_u32(opcode.get());
            };
        }

        let mut encoding = Encoding::try_from(inst_info.encoding).expect("Invalid encoding index");

        macro_rules! simd_insn {
            () => {
                if isign4 == enc_ops!(Reg, Reg) && op0.as_::<Reg>().is_vec128() {
                    if !op0.as_::<Vec>().has_element_index() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let element_type = op0.as_::<Vec>().element_type() as u32;
                    let dst_index = op0.as_::<Vec>().element_index();
                    let lsb_index = element_type - 1;
                    let imm5 = ((dst_index << 1) | 1) << lsb_index;
                    if imm5 > 31 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if op1.as_::<Reg>().is_gp() {
                        // INS - Vec[N] <- GP register.
                        opcode.reset(0b0100111000000000000111 << 10);
                        opcode.add_imm(imm5, 16);
                        emit_rd0_rn5!();
                        return;
                    } else if op1.as_::<Reg>().is_vec128() && op1.as_::<Vec>().has_element_index() {
                        // INS - Vec[N] <- Vec[M].
                        if op0.as_::<Vec>().element_type() != op1.as_::<Vec>().element_type() {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                        let src_index = op1.as_::<Vec>().element_index();
                        if op0.as_::<Reg>().reg_type() != op1.as_::<Reg>().reg_type() {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                        let imm4 = src_index << lsb_index;
                        if imm4 > 15 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        opcode.reset(0b0110111000000000000001 << 10);
                        opcode.add_imm(imm5, 16);
                        opcode.add_imm(imm4, 11);
                        emit_rd0_rn5!();
                        return;
                    }
                }
            };
        }

        macro_rules! simd_dup {
            () => {
                if isign4 == enc_ops!(Reg, Reg) {
                    let k_valid_encodings = B!(VecElementType::B as u32 + 0)
                        | B!(VecElementType::H as u32 + 0)
                        | B!(VecElementType::S as u32 + 0)
                        | B!(VecElementType::B as u32 + 8)
                        | B!(VecElementType::H as u32 + 8)
                        | B!(VecElementType::S as u32 + 8)
                        | B!(VecElementType::D as u32 + 8);

                    let q = op0.as_::<Reg>().reg_type() as u32 - RegType::Vec64 as u32;

                    if op1.as_::<Reg>().is_gp() {
                        let element_type = op0.as_::<Vec>().element_type() as u32;
                        if q > 1 || !bit_test(k_valid_encodings, (q << 3) | element_type) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        let lsb_index = element_type - 1;
                        let imm5 = 1u32 << lsb_index;

                        opcode.reset(0b0000111000000000000011 << 10);
                        opcode.add_imm(q, 30);
                        opcode.add_imm(imm5, 16);
                        emit_rd0_rn5!();
                    } else if !op1.as_::<Reg>().is_vec() || !op1.as_::<Vec>().has_element_index() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    } else {
                        let dst_index = op1.as_::<Vec>().element_index();
                        if !op0.as_::<Vec>().has_element_type() {
                            let lsb_index =
                                op0.as_::<Reg>().reg_type() as u32 - RegType::Vec8 as u32;
                            if lsb_index
                                != op1.as_::<Vec>().element_type() as u32 - VecElementType::B as u32
                                || lsb_index > 3
                            {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            let imm5 = ((dst_index << 1) | 1u32) << lsb_index;
                            if imm5 > 31 {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            opcode.reset(0b0101111000000000000001 << 10);
                            opcode.add_imm(imm5, 16);
                            emit_rd0_rn5!();
                        } else {
                            let element_type = op0.as_::<Vec>().element_type() as u32;
                            if q > 1 || !bit_test(k_valid_encodings, (q << 3) | element_type) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            let lsb_index = element_type - 1;
                            let imm5 = ((dst_index << 1) | 1u32) << lsb_index;
                            if imm5 > 31 {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            opcode.reset(0b0000111000000000000001 << 10);
                            opcode.add_imm(q, 30);
                            opcode.add_imm(imm5, 16);
                            emit_rd0_rn5!();
                        }
                    }
                }
            };
        }

        macro_rules! simd_umov {
            () => {
                let op_data = &SIMD_SMOV_UMOV[encoding_index];
                if isign4 == enc_ops!(Reg, Reg)
                    && op0.as_::<Reg>().is_gp()
                    && op1.as_::<Reg>().is_vec()
                {
                    let size_op = element_type_to_size_op(
                        op_data.vec_op_type,
                        op1.as_::<Reg>().reg_type(),
                        op1.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !op1.as_::<Vec>().has_element_index() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let x = op0.as_::<Gp>().is_gp64() as u32;
                    let gp_must_be_x = (size_op.size() >= 3u32 - op_data.is_signed as u32) as u32;
                    if op_data.is_signed != 0 {
                        if gp_must_be_x != 0 && x == 0 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                    } else {
                        if x != gp_must_be_x {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                    }
                    let element_index = op1.as_::<Vec>().element_index();
                    let max_element_index = 15u32 >> size_op.size();
                    if element_index > max_element_index {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let imm5 = (1u32 | (element_index << 1)) << size_op.size();
                    opcode.reset((op_data.opcode as u32) << 10);
                    opcode.add_imm(x, 30);
                    opcode.add_imm(imm5, 16);
                    emit_rd0_rn5!();
                    return;
                }
            };
        }

        match encoding {
            Encoding::BaseOp => {
                let op_data = &BASE_OP[encoding_index];
                if isign4 == 0 {
                    opcode.reset(op_data.opcode);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseOpX16 => {
                let op_data = &BASE_OP_X16[encoding_index];
                if isign4 == enc_ops!(Reg) && op0.as_::<Reg>().is_gp64() && op0.id() == 16 {
                    opcode.reset(op_data.opcode);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseOpImm => {
                let op_data = &BASE_OP_IMM[encoding_index];
                if isign4 == enc_ops!(Imm) {
                    let imm = op0.as_::<Imm>().value();
                    let imm_max = 1i64 << op_data.imm_bits as u32;
                    if imm >= imm_max as i64 {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }
                    opcode.reset(op_data.opcode);
                    opcode.add_imm(imm as i32 as u32, op_data.imm_offset as _);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseR => {
                let op_data = &BASE_R[encoding_index];
                if isign4 == enc_ops!(Reg) {
                    opcode.reset(op_data.opcode);
                    opcode.add_reg(op0.id(), op_data.r_shift);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseRR => {
                let op_data = &BASE_RR[encoding_index];
                if isign4 == enc_ops!(Reg, Reg) {
                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.a_type, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_type(op1, op_data.b_type) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if op_data.uniform != 0 && !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_gp_id(op0, op_data.a_hi_id) || !check_gp_id(op1, op_data.b_hi_id) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    opcode.reset(op_data.opcode);
                    opcode.add_imm(x, 31);
                    opcode.add_reg(op1.id(), op_data.b_shift);
                    opcode.add_reg(op0.id(), op_data.a_shift);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseRRR => {
                let op_data = &BASE_RRR[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.a_type, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_type(op1, op_data.b_type) || !check_gp_type(op2, op_data.c_type) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if op_data.uniform != 0 && !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id(op0, op_data.a_hi_id)
                        || !check_gp_id(op1, op_data.b_hi_id)
                        || !check_gp_id(op2, op_data.c_hi_id)
                    {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    opcode.reset(op_data.opcode());
                    opcode.add_imm(x, 31);
                    opcode.add_reg(op2.id(), 16);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseRRRR => {
                let op_data = &BASE_RRRR[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg, Reg) {
                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.a_type, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_type(op1, op_data.b_type)
                        || !check_gp_type(op2, op_data.c_type)
                        || !check_gp_type(op3, op_data.d_type)
                    {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if op_data.uniform != 0 && !check_signature!(op0, op1, op2, op3) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id(op0, op_data.a_hi_id)
                        || !check_gp_id(op1, op_data.b_hi_id)
                        || !check_gp_id(op2, op_data.c_hi_id)
                        || !check_gp_id(op3, op_data.d_hi_id)
                    {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    opcode.reset(op_data.opcode());
                    opcode.add_imm(x, 31);
                    opcode.add_reg(op2.id(), 16);
                    opcode.add_reg(op3.id(), 10);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseRRII => {
                let op_data = &BASE_RRII[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Imm, Imm) {
                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.a_type, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_type(op1, op_data.b_type) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_gp_id(op0, op_data.a_hi_id) || !check_gp_id(op1, op_data.b_hi_id) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let imm2 = op2.as_::<Imm>().value();
                    let imm3 = op3.as_::<Imm>().value();

                    if imm2 >= (op_data.a_imm_size + op_data.a_imm_discard_lsb) as u32 as i64
                        || imm3 >= (op_data.b_imm_size + op_data.b_imm_discard_lsb) as u32 as i64
                    {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let a_imm = imm2 as u32 >> op_data.a_imm_discard_lsb;
                    let b_imm = imm3 as u32 >> op_data.b_imm_discard_lsb;

                    opcode.reset(op_data.opcode());
                    opcode.add_imm(a_imm, op_data.a_imm_offset);
                    opcode.add_imm(b_imm, op_data.b_imm_offset);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseMov => {
                let x = (op0.as_::<Reg>().typ() as u32).wrapping_sub(RegType::Gp32 as u32);
                if x > 1 {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                if isign4 == enc_ops!(Reg, Reg) {
                    if !op0.as_::<Reg>().is_gp() || !op1.as_::<Reg>().is_gp() {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let has_sp = op0.as_::<Reg>().is_sp() || op1.as_::<Reg>().is_sp();

                    if has_sp {
                        if !check_gp_id2(op0, op1, 31) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        opcode.reset(0b00010001000000000000000000000000);
                        opcode.add_imm(x, 31);
                        opcode.add_reg(op1.id(), 5);
                        opcode.add_reg(op0.id(), 0);
                        return self.buffer.write_u32(opcode.get());
                    } else {
                        if !check_gp_id2(op0, op1, 63) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        opcode.reset(0b00101010000000000000001111100000);
                        opcode.add_imm(x, 31);
                        opcode.add_reg(op1.id(), 16);
                        opcode.add_reg(op0.id(), 0);
                        return self.buffer.write_u32(opcode.get());
                    }
                }

                if isign4 == enc_ops!(Reg, Imm) {
                    if !op0.as_::<Reg>().is_gp() {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let mut imm_value = op1.as_::<Imm>().value_as::<u64>();
                    if x == 0 {
                        imm_value &= 0xFFFFFFFF;
                    }

                    // Prefer a single MOVN/MOVZ instruction over a logical instruction.
                    multiple_op_count =
                        encode_mov_sequence64(&mut multiple_op_data, imm_value, op0.id() & 31, x);
                    if multiple_op_count == 1 && !op0.as_::<Gp>().is_sp() {
                        opcode.reset(multiple_op_data[0]);
                        return self.buffer.write_u32(opcode.get());
                    }

                    if !op0.as_::<Gp>().is_zr() {
                        if let Some(logical_imm) =
                            encode_logical_imm(imm_value, if x != 0 { 64 } else { 32 })
                        {
                            opcode.reset(0b00110010000000000000001111100000);
                            opcode.add_imm(x, 31);
                            opcode.add_logical_imm(&logical_imm);
                            opcode.add_reg(op0.id(), 0);
                            return self.buffer.write_u32(opcode.get());
                        }
                    }

                    for i in 0..multiple_op_count {
                        self.buffer.write_u32(multiple_op_data[i]);
                    }
                    return;
                }
            }

            Encoding::BaseMovKNZ => {
                let op_data = &BASE_MOV_KNZ[encoding_index];

                let x = op0.as_::<Reg>().typ() as u32 - RegType::Gp32 as u32;
                if x > 1 {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                if !check_gp_id(op0, 63) {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                opcode.reset(op_data.opcode);
                opcode.add_imm(x, 31);

                if isign4 == enc_ops!(Reg, Imm) {
                    let imm16 = op1.as_::<Imm>().value_as::<u64>();
                    if imm16 > 0xFFFF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    opcode.add_imm(imm16 as u32, 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }

                if isign4 == enc_ops!(Reg, Imm, Imm) {
                    let imm16 = op1.as_::<Imm>().value_as::<u64>();
                    let shift_type = op2.as_::<Imm>().predicate();
                    let shift_value = op2.as_::<Imm>().value_as::<u64>();

                    if imm16 > 0xFFFF || shift_value > 48 || shift_type != ShiftOp::LSL as u32 {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let hw = (shift_value as u32) >> 4;

                    if hw << 4 != shift_value as u32 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.add_imm(hw, 21);
                    opcode.add_imm(imm16 as u32, 5);
                    opcode.add_reg(op0.id(), 0);

                    if x == 0 && hw > 1 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseAdr => {
                let op_data = &BASE_ADR[encoding_index];
                if isign4 == enc_ops!(Reg, Label)
                    || isign4 == enc_ops!(Reg, Sym)
                    || isign4 == enc_ops!(Reg, Imm)
                {
                    if !op0.as_::<Reg>().is_gp() {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_gp_id(op0, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(op_data.opcode());
                    opcode.add_reg(op0.id(), 0);
                    offset_format.reset_to_imm_type(
                        OffsetType::try_from(op_data.offset_type).expect("Invalid offset type"),
                        4,
                        5,
                        21,
                        0,
                    );

                    if inst_id == InstId::Adrp as u32 {
                        offset_format.imm_discard_lsb = 12;
                    }
                    rm_rel = op1;
                    emit_rel!();
                }
            }

            Encoding::BaseAddSub => {
                let op_data = &BASE_ADD_SUB[encoding_index];

                let mut x = 0;
                if !check_gp_typex2(op0, op1, 3, &mut x) {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                if isign4 == enc_ops!(Reg, Reg, Imm) || isign4 == enc_ops!(Reg, Reg, Imm, Imm) {
                    opcode.reset((op_data.immediate_op as u32) << 24);

                    // ADD | SUB (immediate) - ZR is not allowed.
                    // ADDS|SUBS (immediate) - ZR allowed in Rd, SP allowed in Rn.
                    let a_hi_id = if opcode.get() & 1 << 29 != 0 { 63 } else { 31 };
                    let b_hi_id = 31;
                    if !check_gp_id(op0, a_hi_id) || !check_gp_id(op1, b_hi_id) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let mut imm = op2.as_::<Imm>().value_as::<u64>();
                    let mut shift = 0;

                    if isign4 == enc_ops!(Reg, Reg, Imm, Imm) {
                        if op3.as_::<Imm>().predicate() != ShiftOp::LSL as u32 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        if op3.as_::<Imm>().value() != 0 && op3.as_::<Imm>().value() != 12 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        shift = (op3.as_::<Imm>().value() != 0) as u32;
                    }

                    if imm > 0xfff {
                        if shift != 0 || (imm & !(0xfff << 12)) != 0 {
                            self.last_error = Some(AsmError::TooLarge);
                            return;
                        }

                        shift = 1;
                        imm >>= 12;
                    }

                    opcode.add_imm(x, 31);
                    opcode.add_imm(shift, 12);
                    opcode.add_imm(imm as u32, 10);
                    opcode.add_reg(op0.id(), 5);
                    opcode.add_reg(op1.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }

                if isign4 == enc_ops!(Reg, Reg, Reg) || isign4 == enc_ops!(Reg, Reg, Reg, Imm) {
                    let op_size = if x != 0 { 64 } else { 32 };
                    let mut shift = 0;
                    let mut shift_type = ShiftOp::LSL as u32;

                    if isign4 == enc_ops!(Reg, Reg, Reg, Imm) {
                        shift_type = op3.as_::<Imm>().predicate();
                        shift = op3.as_::<Imm>().value() as u32;
                    }

                    if !check_gp_id(op2, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if shift_type <= ShiftOp::ASR as u32 {
                        let has_sp = op0.as_::<Gp>().is_sp() || op1.as_::<Gp>().is_sp();

                        if !has_sp {
                            if !check_signature!(op1, op2) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            if !check_gp_id3(op0, op1, op2, 63) {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            if shift >= op_size {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            opcode.reset((op_data.shifted_op as u32) << 21);
                            opcode.add_imm(x, 31);
                            opcode.add_imm(shift_type, 22);
                            opcode.add_reg(op2.id(), 16);
                            opcode.add_imm(shift, 10);
                            opcode.add_reg(op1.id(), 5);
                            opcode.add_reg(op0.id(), 0);
                            return self.buffer.write_u32(opcode.get());
                        }

                        if shift_type != ShiftOp::LSL as u32 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        shift_type = if x != 0 {
                            ShiftOp::UXTX as u32
                        } else {
                            ShiftOp::UXTW as u32
                        };
                    }

                    opcode.reset((op_data.extended_op as u32) << 21);
                    shift_type -= ShiftOp::UXTB as u32;

                    if shift_type > 7 || shift > 4 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if (opcode.get() & (1 << 29)) == 0 {
                        if !check_gp_id2(op0, op1, 31) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                    } else {
                        if !check_gp_id(op0, 63) || !check_gp_id(op1, 31) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                    }

                    opcode.add_imm(x, 31);
                    opcode.add_reg(op2.id(), 16);
                    opcode.add_imm(shift_type, 13);
                    opcode.add_imm(shift, 10);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseLogical => {
                let op_data = &BASE_LOGICAL[encoding_index];

                let mut x = 0;
                if !check_gp_typex2(op0, op1, 3, &mut x) {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                if !check_signature!(op0, op1) {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                let op_size = if x != 0 { 64 } else { 32 };

                if isign4 == enc_ops!(Reg, Reg, Imm) && op_data.immediate_op != 0 {
                    opcode.reset((op_data.immediate_op as u32) << 23);

                    let imm_mask = lsb_mask::<u64>(op_size);
                    let mut imm_value = op2.as_::<Imm>().value_as::<u64>();

                    if op_data.negate_imm != 0 {
                        imm_value ^= imm_mask;
                    }

                    let Some(logical_imm) = encode_logical_imm(imm_value, op_size) else {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    };

                    let op_ands = 0x3 << 29;
                    let is_ands = (opcode.get() & op_ands) == op_ands;

                    if !check_gp_id(op0, if is_ands { 63 } else { 31 }) || !check_gp_id(op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.add_imm(x, 31);
                    opcode.add_logical_imm(&logical_imm);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }

                if !check_signature!(op1, op2) {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if !check_gp_id3(op0, op1, op2, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset((op_data.shifted_op as u32) << 21);
                    opcode.add_imm(x, 31);
                    opcode.add_reg(op2.id(), 16);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }

                if isign4 == enc_ops!(Reg, Reg, Reg, Imm) {
                    if !check_gp_id3(op0, op1, op2, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let shift_type = op3.as_::<Imm>().predicate();
                    let op_shift = op3.as_::<Imm>().value() as u32;

                    if shift_type > 0x3 || op_shift >= op_size {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset((op_data.shifted_op as u32) << 21);
                    opcode.add_imm(x, 31);
                    opcode.add_imm(shift_type, 22);
                    opcode.add_reg(op2.id(), 16);
                    opcode.add_imm(op_shift, 10);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseCmpCmn => {
                let op_data = &BASE_CMP_CMN[encoding_index];

                let mut x = 0;
                if !check_gp_typex(op0, 3, &mut x) {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                if isign4 == enc_ops!(Reg, Imm) {
                    if !check_gp_id(op0, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let imm12 = op1.as_::<Imm>();
                    let mut imm_shift = 0;
                    let mut imm_value = imm12.value_as::<u64>();

                    if imm_value > 0xfff {
                        if (imm_value & !(0xfff << 12)) != 0 {
                            self.last_error = Some(AsmError::TooLarge);
                            return;
                        }
                        imm_shift = 1;
                        imm_value >>= 12;
                    }

                    opcode.reset((op_data.immediate_op as u32) << 24);
                    opcode.add_imm(x, 31);
                    opcode.add_imm(imm_shift, 22);
                    opcode.add_imm(imm_value as u32, 10);
                    opcode.add_reg(op0.id(), 5);
                    opcode.add_reg(63, 0);
                    return self.buffer.write_u32(opcode.get());
                }

                if isign4 == enc_ops!(Reg, Reg) || isign4 == enc_ops!(Reg, Reg, Imm) {
                    let op_size = if x != 0 { 64 } else { 32 };
                    let mut shift_type = 0;
                    let mut shift_value = 0;

                    if isign4 == enc_ops!(Reg, Reg, Imm) {
                        shift_type = op2.as_::<Imm>().predicate();
                        shift_value = op2.as_::<Imm>().value() as u32;
                    }

                    let has_sp = op0.as_::<Gp>().is_sp() || op1.as_::<Gp>().is_sp();

                    if shift_type <= ShiftOp::ASR as u32 {
                        if !has_sp {
                            if !check_signature!(op0, op1) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            if shift_value >= op_size {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            opcode.reset((op_data.shifted_op as u32) << 21);
                            opcode.add_imm(x, 31);
                            opcode.add_imm(shift_type, 22);
                            opcode.add_reg(op1.id(), 5);
                            opcode.add_imm(shift_value, 10);
                            opcode.add_reg(op0.id(), 5);
                            opcode.add_reg(63, 0);
                            return self.buffer.write_u32(opcode.get());
                        }

                        if shift_type != ShiftOp::LSL as u32 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        shift_type = if x != 0 {
                            ShiftOp::UXTX as u32
                        } else {
                            ShiftOp::UXTW as u32
                        }
                    }

                    shift_type -= ShiftOp::UXTB as u32;

                    if shift_type > 7 || shift_value > 4 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset((op_data.extended_op as u32) << 21);
                    opcode.add_imm(x, 31);
                    opcode.add_reg(op1.id(), 16);
                    opcode.add_imm(shift_type, 13);
                    opcode.add_imm(shift_value, 10);
                    opcode.add_reg(op0.id(), 5);
                    opcode.add_reg(63, 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseMvnNeg => {
                let op_data = &BASE_MVN_NEG[encoding_index];

                let mut x = 0;
                if !check_gp_typex2(op0, op1, 3, &mut x) {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                opcode.reset(op_data.opcode);
                opcode.add_imm(x, 31);
                opcode.add_reg(op1.id(), 16);
                opcode.add_reg(op0.id(), 0);

                if isign4 == enc_ops!(Reg, Reg) {
                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    return self.buffer.write_u32(opcode.get());
                }

                if isign4 == enc_ops!(Reg, Reg, Imm) {
                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let op_size = if x != 0 { 64 } else { 32 };
                    let shift_type = op2.as_::<Imm>().predicate();
                    let shift_value = op2.as_::<Imm>().value() as u32;

                    if shift_type > ShiftOp::ROR as u32 || shift_value >= op_size {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.add_imm(shift_type, 22);
                    opcode.add_imm(shift_value, 10);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseTst => {
                let op_data = &BASE_TST[encoding_index];

                let mut x = 0;
                if !check_gp_typex(op0, 3, &mut x) {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                if isign4 == enc_ops!(Reg, Imm) && op_data.immediate_op != 0 {
                    if !check_gp_id(op0, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let op_size = if x != 0 { 64 } else { 32 };
                    let imm_mask = lsb_mask::<u64>(op_size);
                    let imm_value = op1.as_::<Imm>().value_as::<u64>();

                    let Some(logical_imm) = encode_logical_imm(imm_value & imm_mask, op_size)
                    else {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    };

                    opcode.reset((op_data.immediate_op as u32) << 22);
                    opcode.add_logical_imm(&logical_imm);
                    opcode.add_imm(x, 31);
                    opcode.add_reg(op0.id(), 5);
                    opcode.add_reg(63, 0);
                    return self.buffer.write_u32(opcode.get());
                }

                opcode.reset((op_data.shifted_op as u32) << 21);
                opcode.add_imm(x, 31);
                opcode.add_reg(op1.id(), 16);
                opcode.add_reg(op0.id(), 5);
                opcode.add_reg(63, 0);

                if isign4 == enc_ops!(Reg, Reg) {
                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    return self.buffer.write_u32(opcode.get());
                }

                if isign4 == enc_ops!(Reg, Reg, Imm) {
                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let shift_type = op2.as_::<Imm>().predicate();
                    let shift_value = op2.as_::<Imm>().value() as u32;

                    if shift_type > 0x3 || shift_value >= (if x != 0 { 64 } else { 32 }) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.add_imm(shift_type, 22);
                    opcode.add_imm(shift_value, 10);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseBfc => {
                let op_data = &BASE_BFC[encoding_index];
                if isign4 == enc_ops!(Reg, Imm, Imm) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let lsb = op1.as_::<Imm>().value_as::<u64>();
                    let width = op2.as_::<Imm>().value_as::<u64>();
                    let op_size = if x != 0 { 64 } else { 32 };

                    if lsb >= op_size || width == 0 || width > op_size {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let lsb32 = 0u32.wrapping_sub(lsb as u32) & (op_size as u32 - 1);
                    let width32 = width as u32 - 1;

                    opcode.reset(op_data.opcode);
                    opcode.add_imm(x, 31);
                    opcode.add_imm(x, 22);
                    opcode.add_imm(lsb32, 16);
                    opcode.add_imm(width32, 10);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseBfi => {
                let op_data = &BASE_BFI[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Imm, Imm) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let lsb = op2.as_::<Imm>().value_as::<u64>();
                    let width = op3.as_::<Imm>().value_as::<u64>();
                    let op_size = if x != 0 { 64 } else { 32 };

                    if lsb >= op_size as u64 || width == 0 || width > op_size as u64 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let imm_l = 0u32.wrapping_sub(lsb as u32) & (op_size as u32 - 1);
                    let imm_w = width as u32 - 1;

                    opcode.reset(op_data.opcode);
                    opcode.add_imm(x, 31);
                    opcode.add_imm(x, 22);
                    opcode.add_imm(imm_l, 16);
                    opcode.add_imm(imm_w, 10);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseBfm => {
                let op_data = &BASE_BFM[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Imm, Imm) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let imm_r = op2.as_::<Imm>().value_as::<u64>();
                    let imm_s = op3.as_::<Imm>().value_as::<u64>();
                    let op_size = if x != 0 { 64 } else { 32 };

                    if (imm_r | imm_s) >= op_size as u64 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(op_data.opcode);
                    opcode.add_imm(x, 31);
                    opcode.add_imm(x, 22);
                    opcode.add_imm(imm_r as u32, 16);
                    opcode.add_imm(imm_s as u32, 10);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseBfx => {
                let op_data = &BASE_BFX[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Imm, Imm) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let lsb = op2.as_::<Imm>().value_as::<u64>();
                    let width = op3.as_::<Imm>().value_as::<u64>();
                    let op_size = if x != 0 { 64 } else { 32 };

                    if lsb >= op_size as u64 || width == 0 || width > op_size as u64 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let lsb32 = lsb as u32;
                    let width32 = lsb32 + width as u32 - 1;

                    if width32 >= op_size as u32 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(op_data.opcode);
                    opcode.add_imm(x, 31);
                    opcode.add_imm(x, 22);
                    opcode.add_imm(lsb32, 16);
                    opcode.add_imm(width32, 10);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseExtend => {
                let op_data = &BASE_EXTEND[encoding_index];

                if isign4 == enc_ops!(Reg, Reg) {
                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.reg_type, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !op1.as_::<Reg>().is_gp32() {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(op_data.opcode());
                    opcode.add_imm(x, 31);
                    opcode.add_imm(x, 22);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseExtract => {
                let op_data = &BASE_EXTRACT[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg, Imm) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id3(op0, op1, op2, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let lsb = op3.as_::<Imm>().value_as::<u64>();
                    let op_size = if x != 0 { 64 } else { 32 };

                    if lsb >= op_size as u64 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(op_data.opcode);
                    opcode.add_imm(x, 31);
                    opcode.add_imm(x, 22);
                    opcode.add_reg(op2.id(), 16);
                    opcode.add_imm(lsb as u32, 10);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseRev => {
                if isign4 == enc_ops!(Reg, Reg) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(0b01011010110000000000100000000000);
                    opcode.add_imm(x, 31);
                    opcode.add_imm(x, 10);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseShift => {
                let op_data = &BASE_SHIFT[encoding_index];

                let mut x = 0;
                if !check_gp_typex(op0, 3, &mut x) {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id3(op0, op1, op2, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(op_data.register_op);
                    opcode.add_imm(x, 31);
                    opcode.add_reg(op2.id(), 16);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }

                if isign4 == enc_ops!(Reg, Reg, Imm) && op_data.immediate_op != 0 {
                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let imm_r = op2.as_::<Imm>().value_as::<u64>();
                    let op_size = if x != 0 { 64 } else { 32 };

                    if imm_r >= op_size as u64 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(op_data.immediate_op);
                    opcode.add_imm(x, 31);
                    opcode.add_imm(x, 22);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);

                    if opcode.get() & (1 << 10) != 0 {
                        opcode.add_imm(x, 15);
                        opcode.add_imm(imm_r as u32, 16);
                        return self.buffer.write_u32(opcode.get());
                    }

                    if op_data.ror == 0 {
                        let ubfm_imm_r = (0u32).wrapping_sub(imm_r as u32) & (op_size as u32 - 1);
                        let ubfm_imm_s = op_size as u32 - 1 - imm_r as u32;

                        opcode.add_imm(ubfm_imm_r, 16);
                        opcode.add_imm(ubfm_imm_s, 10);
                        return self.buffer.write_u32(opcode.get());
                    } else {
                        opcode.add_imm(imm_r as u32, 10);
                        opcode.add_reg(op1.id(), 16);
                        return self.buffer.write_u32(opcode.get());
                    }
                }
            }

            Encoding::BaseCCmp => {
                let op_data = &BASE_C_CMP[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Imm, Imm) || isign4 == enc_ops!(Reg, Imm, Imm, Imm)
                {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_gp_id(op0, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let nzcv = op2.as_::<Imm>().value_as::<u64>();
                    let cond = op3.as_::<Imm>().value_as::<u64>();

                    if (nzcv | cond) > 0xF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    opcode.reset(op_data.opcode);
                    opcode.add_imm(x, 31);
                    opcode.add_imm(cond_code_to_opcode_field(cond as u32), 12);
                    opcode.add_imm(nzcv as u32, 0);

                    if isign4 == enc_ops!(Reg, Reg, Imm, Imm) {
                        if !check_signature!(op0, op1) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if !check_gp_id(op1, 31) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        opcode.add_reg(op1.id(), 16);
                        opcode.add_reg(op0.id(), 5);
                        return self.buffer.write_u32(opcode.get());
                    } else {
                        let imm5 = op1.as_::<Imm>().value_as::<u64>();
                        if imm5 > 0x1F {
                            self.last_error = Some(AsmError::TooLarge);
                            return;
                        }

                        opcode.add_imm(1, 11);
                        opcode.add_imm(imm5 as u32, 16);
                        opcode.add_reg(op0.id(), 5);
                        return self.buffer.write_u32(opcode.get());
                    }
                }
            }

            Encoding::BaseCInc => {
                let op_data = &BASE_C_INC[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Imm) {
                    let mut x = 0;
                    if !check_gp_typex2(op0, op1, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id2(op0, op1, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let cond = op2.as_::<Imm>().value_as::<u64>();
                    if cond.wrapping_sub(2) > 0xE {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    opcode.reset(op_data.opcode);
                    opcode.add_imm(x, 31);
                    opcode.add_reg(op1.id(), 16);
                    opcode.add_imm(cond_code_to_opcode_field((cond as u32) ^ 1), 12);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseCSel => {
                let op_data = &BASE_C_SEL[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg, Imm) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id3(op0, op1, op2, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let cond = op3.as_::<Imm>().value_as::<u64>();
                    if cond > 0xF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    opcode.reset(op_data.opcode);
                    opcode.add_imm(x, 31);
                    opcode.add_reg(op2.id(), 16);
                    opcode.add_imm(cond_code_to_opcode_field(cond as u32), 12);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseCSet => {
                let op_data = &BASE_C_SET[encoding_index];

                if isign4 == enc_ops!(Reg, Imm) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id(op0, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let cond = op1.as_::<Imm>().value_as::<u64>();
                    if cond.wrapping_sub(2) >= 0xE {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    opcode.reset(op_data.opcode);
                    opcode.add_imm(x, 31);
                    opcode.add_imm(cond_code_to_opcode_field((cond as u32) ^ 1), 12);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseMinMax => {
                let op_data = &BASE_MIN_MAX[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    opcode.reset(op_data.register_op);
                    opcode.add_imm(x, 31);
                    opcode.add_reg(op2.id(), 16);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }

                if isign4 == enc_ops!(Reg, Reg, Imm) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let imm = op2.as_::<Imm>().value_as::<u64>();

                    if (op_data.immediate_op & (1u32 << 18)) != 0 {
                        if imm > 0xFF {
                            self.last_error = Some(AsmError::TooLarge);
                            return;
                        }
                    } else {
                        if (imm as i64) < -128 || (imm as i64) > 127 {
                            self.last_error = Some(AsmError::TooLarge);
                            return;
                        }
                    }

                    opcode.reset(op_data.immediate_op);
                    opcode.add_imm(x, 31);
                    opcode.add_imm((imm & 0xFF) as u32, 10);
                    opcode.add_reg(op1.id(), 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseAtDcIcTlbi => {
                let op_data = &BASE_AT_DC_IC_TLBI[encoding_index];

                if isign4 == enc_ops!(Imm) || isign4 == enc_ops!(Imm, Reg) {
                    if op_data.mandatory_reg != 0 && isign4 != enc_ops!(Imm, Reg) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if op0.as_::<Imm>().value_as::<u64>() > 0x7FFF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let imm = op0.as_::<Imm>().value_as::<u32>();
                    if (imm & op_data.imm_verify_mask) != op_data.imm_verify_data {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let mut rt = 31;
                    if op1.is_reg() {
                        if !op1.as_::<Reg>().is_gp64() {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if !check_gp_id(op1, 63) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        rt = op1.id() & 31;
                    }

                    opcode.reset(0b11010101000010000000000000000000);
                    opcode.add_imm(imm, 5);
                    opcode.add_reg(rt, 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseMrs => {
                if isign4 == enc_ops!(Reg, Imm) {
                    if !op0.as_::<Reg>().is_gp64() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id(op0, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if op1.as_::<Imm>().value_as::<u64>() > 0xFFFF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let imm = op1.as_::<Imm>().value_as::<u32>();
                    if (imm & (1 << 15)) == 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(0b11010101001100000000000000000000);
                    opcode.add_imm(imm, 5);
                    opcode.add_reg(op0.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseMsr => {
                if isign4 == enc_ops!(Imm, Reg) {
                    if !op1.as_::<Reg>().is_gp64() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if op0.as_::<Imm>().value_as::<u64>() > 0xFFFF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let imm = op0.as_::<Imm>().value_as::<u32>();
                    if (imm & (1 << 15)) == 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_gp_id(op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(0b11010101000100000000000000000000);
                    opcode.add_imm(imm, 5);
                    opcode.add_reg(op1.id(), 0);
                    return self.buffer.write_u32(opcode.get());
                }

                if isign4 == enc_ops!(Imm, Imm) {
                    if op0.as_::<Imm>().value_as::<u64>() > 0x1F {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    if op1.as_::<Imm>().value_as::<u64>() > 0xF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let op = op0.as_::<Imm>().value_as::<u32>();
                    let crm = op1.as_::<Imm>().value_as::<u32>();

                    let op1_val = op >> 3;
                    let op2_val = op & 0x7;

                    opcode.reset(0b11010101000000000100000000011111);
                    opcode.add_imm(op1_val, 16);
                    opcode.add_imm(crm, 8);
                    opcode.add_imm(op2_val, 5);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseSys => {
                if isign4 == enc_ops!(Imm, Imm, Imm, Imm) {
                    if op0.as_::<Imm>().value_as::<u64>() > 0x7
                        || op1.as_::<Imm>().value_as::<u64>() > 0xF
                        || op2.as_::<Imm>().value_as::<u64>() > 0xF
                        || op3.as_::<Imm>().value_as::<u64>() > 0x7
                    {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let op1_val = op0.as_::<Imm>().value_as::<u32>();
                    let crn = op1.as_::<Imm>().value_as::<u32>();
                    let crm = op2.as_::<Imm>().value_as::<u32>();
                    let op2_val = op3.as_::<Imm>().value_as::<u32>();
                    let mut rt = 31;

                    let op4 = *ops.get(4).unwrap_or(&NOREG);
                    if op4.is_reg() {
                        if !op4.as_::<Reg>().is_gp64() {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if !check_gp_id(op4, 63) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        rt = op4.id() & 31;
                    } else if !op4.is_none() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    opcode.reset(0b11010101000010000000000000000000);
                    opcode.add_imm(op1_val, 16);
                    opcode.add_imm(crn, 12);
                    opcode.add_imm(crm, 8);
                    opcode.add_imm(op2_val, 5);
                    opcode.add_reg(rt, 0);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseBranchReg => {
                let op_data = &BASE_BRANCH_REG[encoding_index];
                if isign4 == enc_ops!(Reg) {
                    if !op0.as_::<Reg>().is_gp64() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id(op0, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(op_data.opcode);
                    opcode.add_reg(op0.id(), 5);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseBranchRel => {
                let op_data = &BASE_BRANCH_REL[encoding_index];
                if isign4 == enc_ops!(Label) || isign4 == enc_ops!(Imm) {
                    opcode.reset(op_data.opcode);
                    rm_rel = op0;

                    if inst_cc as u32 != 0 || (opcode.0 & (1 << 30)) != 0 {
                        if opcode.has_x() {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        opcode.0 |= 1 << 30;
                        opcode.add_imm(cond_code_to_opcode_field(inst_cc as u32), 0);
                        offset_format.reset_to_imm_type(OffsetType::SignedOffset, 4, 5, 19, 2);
                        rm_rel = op0;
                        emit_rel!();
                    }

                    offset_format.reset_to_imm_type(OffsetType::SignedOffset, 4, 0, 26, 2);
                    rm_rel = op0;
                    emit_rel!();
                }
            }

            Encoding::BaseBranchCmp => {
                let op_data = &BASE_BRANCH_CMP[encoding_index];
                if isign4 == enc_ops!(Reg, Label) || isign4 == enc_ops!(Reg, Imm) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id(op0, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(op_data.opcode);
                    opcode.add_imm(x, 31);
                    opcode.add_reg(op0.id(), 0);
                    offset_format.reset_to_imm_type(OffsetType::SignedOffset, 4, 5, 19, 2);

                    rm_rel = op1;
                    emit_rel!();
                }
            }

            Encoding::BaseBranchTst => {
                let op_data = &BASE_BRANCH_TST[encoding_index];
                if isign4 == enc_ops!(Reg, Imm, Label) || isign4 == enc_ops!(Reg, Imm, Imm) {
                    let mut x = 0;
                    if !check_gp_typex(op0, 3, &mut x) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_gp_id(op0, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let mut imm = op1.as_::<Imm>().value_as::<u64>();

                    opcode.reset(op_data.opcode);
                    if imm >= 32 {
                        if x == 0 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        opcode.add_imm(x, 31);
                        imm &= 0x1F;
                    }

                    opcode.add_reg(op0.id(), 0);
                    opcode.add_imm(imm as u32, 19);
                    offset_format.reset_to_imm_type(OffsetType::SignedOffset, 4, 5, 14, 2);

                    rm_rel = op2;
                    emit_rel!();
                }
            }

            Encoding::BasePrfm => {
                let op_data = &BASE_PRFM[encoding_index];
                if isign4 == enc_ops!(Imm, Mem) {
                    let m = op1.as_::<Mem>();
                    rm_rel = op1;

                    let imm_shift = 3u32;

                    if op0.as_::<Imm>().value_as::<u64>() > 0x1F {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let offset = m.offset();
                    let prfop = op0.as_::<Imm>().value_as::<u32>();

                    if m.has_base_reg() {
                        if m.has_index() {
                            let opt = SHIFT_OP_TO_LD_ST_OP_MAP[m.shift_op() as usize];
                            if opt == 0xFF {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            let shift = m.shift();
                            let s = if shift != 0 { 1 } else { 0 };

                            if s != 0 && shift != imm_shift {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            opcode.reset((op_data.register_op as u32) << 21);
                            opcode.add_imm(opt as u32, 13);
                            opcode.add_imm(s, 12);
                            opcode.0 |= 1 << 11;
                            opcode.add_imm(prfop, 0);
                            opcode.add_reg(m.base_id(), 5);
                            opcode.add_reg(m.index_id(), 16);
                            return self.buffer.write_u32(opcode.get());
                        }

                        let offset32 = offset as i32;
                        let imm12 = (offset32 as u32) >> imm_shift;

                        if imm12 < (1 << 12) && ((imm12 << imm_shift) as i32) == offset32 {
                            opcode.reset((op_data.s_offset_op as u32) << 22);
                            opcode.add_imm(imm12, 10);
                            opcode.add_imm(prfop, 0);
                            opcode.add_reg(m.base_id(), 5);
                            return self.buffer.write_u32(opcode.get());
                        }

                        if offset32 >= -256 && offset32 < 256 {
                            opcode.reset((op_data.u_offset_op as u32) << 21);
                            opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                            opcode.add_imm(prfop, 0);
                            opcode.add_reg(m.base_id(), 5);
                            return self.buffer.write_u32(opcode.get());
                        }

                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    } else {
                        opcode.reset((op_data.literal_op as u32) << 24);
                        opcode.add_imm(prfop, 0);
                        offset_format.reset_to_imm_type(OffsetType::SignedOffset, 4, 5, 19, 2);
                        rm_rel = op1;
                        emit_rel!();
                    }
                }
            }

            Encoding::BaseLdSt => {
                let op_data = &BASE_LD_ST[encoding_index];
                if isign4 == enc_ops!(Reg, Mem) {
                    let m = op1.as_::<Mem>();
                    rm_rel = op1;

                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.reg_type, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_gp_id(op0, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let x_shift_mask = if op_data.u_offset_shift == 2 { 1 } else { 0 };
                    let imm_shift = (op_data.u_offset_shift as u32) + (x & x_shift_mask);

                    let offset = m.offset();

                    if m.has_base_reg() {
                        if m.has_index() {
                            let opt = SHIFT_OP_TO_LD_ST_OP_MAP[m.shift_op() as usize];
                            if opt == 0xFF {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            let shift = m.shift();
                            let s = if shift != 0 { 1 } else { 0 };

                            if s != 0 && shift != imm_shift {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            opcode.reset((op_data.register_op as u32) << 21);
                            opcode.xor_imm(x, op_data.x_offset as u32);
                            opcode.add_imm(opt as u32, 13);
                            opcode.add_imm(s, 12);
                            opcode.0 |= 1 << 11;
                            opcode.add_reg(op0.id(), 0);
                            opcode.add_reg(m.base_id(), 5);
                            opcode.add_reg(m.index_id(), 16);
                            return self.buffer.write_u32(opcode.get());
                        }

                        let offset32 = offset as i32;
                        let imm12 = (offset32 as u32) >> imm_shift;

                        if imm12 < (1 << 12) && ((imm12 << imm_shift) as i32) == offset32 {
                            opcode.reset((op_data.u_offset_op as u32) << 22);
                            opcode.xor_imm(x, op_data.x_offset as u32);
                            opcode.add_imm(imm12, 10);
                            opcode.add_reg(op0.id(), 0);
                            opcode.add_reg(m.base_id(), 5);
                            return self.buffer.write_u32(opcode.get());
                        }

                        if offset32 >= -256 && offset32 < 256 {
                            opcode.reset((op_data.u_offset_op as u32) << 22);
                            opcode.xor_imm(x, op_data.x_offset as u32);
                            opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                            opcode.add_reg(op0.id(), 0);
                            opcode.add_reg(m.base_id(), 5);
                            return self.buffer.write_u32(opcode.get());
                        }

                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    } else {
                        if op_data.literal_op == 0 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        opcode.reset((op_data.literal_op as u32) << 24);
                        opcode.xor_imm(x, op_data.x_offset);
                        opcode.add_reg(op0.id(), 0);
                        offset_format.reset_to_imm_type(OffsetType::Ldr, 4, 5, 19, 2);
                        emit_rel!();
                    }
                }
            }

            Encoding::BaseLdpStp => {
                let op_data = &BASE_LDP_STP[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Mem) {
                    let m = op2.as_::<Mem>();
                    rm_rel = op2;

                    let mut x = 0;
                    if !check_gp_typex2(op0, op1, op_data.reg_type, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_gp_id2(op0, op1, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let offset_shift = op_data.offset_shift as u32 + x;
                    let offset32 = (m.offset_lo32() as i32) >> offset_shift;

                    if (offset32 as u32) << offset_shift != m.offset_lo32() as u32 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    const I7_MAX: i32 = (1 << 6) - 1;
                    const I7_MIN: i32 = -(1 << 6);

                    if offset32 < I7_MIN || offset32 > I7_MAX {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if m.is_pre_or_post() && offset32 != 0 {
                        if op_data.pre_post_op == 0 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        opcode.reset((op_data.pre_post_op as u32) << 22);
                        opcode.add_imm(m.is_pre_index() as u32, 24);
                    } else {
                        opcode.reset((op_data.offset_op as u32) << 22);
                    }
                    opcode.add_imm(x, op_data.x_offset as u32);
                    opcode.add_imm((offset32 as u32) & 0x7F, 15);
                    opcode.add_reg(op1.id(), 10);
                    opcode.add_reg(op0.id(), 0);
                    opcode.add_reg(m.base_id(), 5);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseStx => {
                let op_data = &BASE_STX[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Mem) {
                    let m = op2.as_::<Mem>();
                    let mut x = 0;
                    if !op0.as_::<Reg>().is_gp32() || !check_gp_typex(op1, op_data.reg_type, &mut x)
                    {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_id2(op0, op1, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    opcode.reset(op_data.opcode());
                    opcode.add_imm(x, op_data.x_offset as _);
                    opcode.add_reg(op0.id(), 16);
                    opcode.add_reg(op1.id(), 0);
                    rm_rel = op2;
                    opcode.add_reg(m.base_id(), 5);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseLdxp => {
                let op_data = &BASE_LDXP[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Mem) {
                    let m = op2.as_::<Mem>();
                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.reg_type, &mut x) || !check_signature!(op0, op1)
                    {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_id2(op0, op1, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    opcode.reset(op_data.opcode());
                    opcode.add_imm(x, op_data.x_offset as _);
                    opcode.add_reg(op1.id(), 10);
                    opcode.add_reg(op0.id(), 0);
                    rm_rel = op2;
                    opcode.add_reg(m.base_id(), 5);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseStxp => {
                let op_data = &BASE_STXP[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg, Mem) {
                    let m = op3.as_::<Mem>();
                    let mut x = 0;
                    if !op0.as_::<Reg>().is_gp32()
                        || !check_gp_typex(op1, op_data.reg_type, &mut x)
                        || !check_signature!(op1, op2)
                    {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_id3(op0, op1, op2, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    opcode.reset(op_data.opcode());
                    opcode.add_imm(x, op_data.x_offset as _);
                    opcode.add_reg(op0.id(), 16);
                    opcode.add_reg(op2.id(), 10);
                    opcode.add_reg(op1.id(), 0);
                    rm_rel = op3;
                    opcode.add_reg(m.base_id(), 5);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseRMNoImm => {
                let op_data = &BASE_RM_NO_IMM[encoding_index];
                if isign4 == enc_ops!(Reg, Mem) {
                    let m = op1.as_::<Mem>();
                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.reg_type, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_id(op0, op_data.reg_hi_id) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    opcode.reset(op_data.opcode());
                    opcode.add_imm(x, op_data.x_offset as _);
                    opcode.add_reg(op0.id(), 0);
                    rm_rel = op1;

                    emit_mem_base_no_imm_rn5!();
                }
            }

            Encoding::BaseRMSImm9 => {
                let op_data = &BASE_RM_SIMM9[encoding_index];
                if isign4 == enc_ops!(Reg, Mem) {
                    let m = op1.as_::<Mem>();
                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.reg_type, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_id(op0, op_data.reg_hi_id) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if m.has_base_reg() && !m.has_index() {
                        let offset32 = m.offset() as i32 >> op_data.imm_shift;
                        if (offset32 << op_data.imm_shift) != m.offset() as i32 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        if offset32 < -256 || offset32 > 255 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        if m.is_fixed_offset() {
                            opcode.reset(op_data.offset_op());
                        } else {
                            opcode.reset(op_data.pre_post_op());
                            opcode.xor_imm(m.is_pre_index() as u32, 11);
                        }
                        opcode.xor_imm(x, op_data.x_offset as u32);
                        opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                        opcode.add_reg(op0.id(), 0);
                        opcode.add_reg(m.base_id(), 5);
                        return self.buffer.write_u32(opcode.get());
                    }
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::BaseRMSImm10 => {
                let op_data = &BASE_RM_SIMM10[encoding_index];
                if isign4 == enc_ops!(Reg, Mem) {
                    let m = op1.as_::<Mem>();
                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.reg_type, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_id(op0, op_data.reg_hi_id) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if m.has_base_reg() && !m.has_index() {
                        let offset32 = m.offset() as i32 >> op_data.imm_shift;
                        if (offset32 << op_data.imm_shift) != m.offset() as i32 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        if offset32 < -512 || offset32 > 511 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        let offset32 = (offset32 as u32) & 0x3FF;
                        opcode.reset(op_data.opcode());
                        opcode.xor_imm(m.is_pre_index() as u32, 11);
                        opcode.xor_imm(x, op_data.x_offset as u32);
                        opcode.add_imm(offset32 >> 9, 22);
                        opcode.add_imm(offset32, 12);
                        opcode.add_reg(op0.id(), 0);
                        opcode.add_reg(m.base_id(), 5);
                        return self.buffer.write_u32(opcode.get());
                    }
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::BaseAtomicOp => {
                let op_data = &BASE_ATOMIC_OP[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Mem) {
                    let m = op2.as_::<Mem>();
                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.reg_type, &mut x) || !check_signature!(op0, op1)
                    {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_id2(op0, op1, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    opcode.reset(op_data.opcode());
                    opcode.add_imm(x, op_data.x_offset as _);
                    opcode.add_reg(op0.id(), 16);
                    opcode.add_reg(op1.id(), 0);
                    rm_rel = op2;
                    opcode.add_reg(m.base_id(), 5);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseAtomicSt => {
                let op_data = &BASE_ATOMIC_ST[encoding_index];
                if isign4 == enc_ops!(Reg, Mem) {
                    let m = op1.as_::<Mem>();
                    let mut x = 0;
                    if !check_gp_typex(op0, op_data.reg_type, &mut x) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if !check_gp_id(op0, 31) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    opcode.reset(op_data.opcode());
                    opcode.add_imm(x, op_data.x_offset as _);
                    opcode.add_reg(op0.id(), 16);
                    opcode.add_reg(31, 0);
                    rm_rel = op1;
                    opcode.add_reg(m.base_id(), 5);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::BaseAtomicCasp => {
                let op_data = &BASE_ATOMIC_CASP[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg, Reg) {
                    let op4 = *ops.get(4).unwrap_or(&NOREG);
                    if op4.is_mem() {
                        let m = op4.as_::<Mem>();
                        let mut x = 0;
                        if !check_gp_typex(op0, op_data.reg_type, &mut x) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        if !check_signature!(op0, op1, op2, op3) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        let id0 = op0.id();
                        let id2 = op2.id();
                        if (id0 & 1) != 0 || (id2 & 1) != 0 || id0 == id2 || id0 > 30 || id2 > 30 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        if (id0 + 1) != op1.id() || (id2 + 1) != op3.id() {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        opcode.reset(op_data.opcode());
                        opcode.add_imm(x, op_data.x_offset as _);
                        opcode.add_reg(op0.id(), 16);
                        opcode.add_reg(op2.id(), 0);
                        rm_rel = &op4;
                        emit_mem_base_no_imm_rn5!();
                    }
                }
            }

            Encoding::FSimdSV => {
                let op_data = &F_SIMD_SV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg) {
                    let q = op1.as_::<Reg>().typ() as u32;
                    let q = if q >= RegType::Vec64 as u32 {
                        q - RegType::Vec64 as u32
                    } else {
                        u32::MAX
                    };
                    if q > 1 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if op0.as_::<Vec>().has_element_type() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let sz = op0.as_::<Reg>().typ() as u32;
                    let sz = if sz >= RegType::Vec16 as u32 {
                        sz - RegType::Vec16 as u32
                    } else {
                        u32::MAX
                    };
                    let element_sz = op1.as_::<Vec>().element_type() as u32;
                    let element_sz = if element_sz >= VecElementType::H as u32 {
                        element_sz - VecElementType::H as u32
                    } else {
                        u32::MAX
                    };

                    if (sz | element_sz) > 1 || sz != element_sz {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if sz != 0 && q == 0 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    opcode.reset((op_data.opcode as u32) << 10);
                    if sz == 0 {
                        opcode.0 ^= 1 << 29;
                    }
                    opcode.add_imm(q, 30);
                    opcode.add_reg(op0.id(), 0);
                    opcode.add_reg(op1.id(), 5);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::FSimdVV => {
                let op_data = &F_SIMD_VV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg) {
                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if let Some(fp_opcode) = pick_fp_opcode(
                        op0.as_::<Vec>(),
                        op_data.scalar_op(),
                        op_data.scalar_hf(),
                        op_data.vector_op(),
                        op_data.vector_hf(),
                        &mut 0,
                    ) {
                        opcode.reset(fp_opcode.0);
                        emit_rd0_rn5!();
                    }

                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }
            }

            Encoding::FSimdVVV => {
                let op_data = &F_SIMD_VVV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if let Some(fp_opcode) = pick_fp_opcode(
                        op0.as_::<Vec>(),
                        op_data.scalar_op(),
                        op_data.scalar_hf(),
                        op_data.vector_op(),
                        op_data.vector_hf(),
                        &mut 0,
                    ) {
                        opcode.reset(fp_opcode.0);
                        emit_rd0_rn5_rm16!();
                    }

                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }
            }

            Encoding::FSimdVVVe => {
                let op_data = &F_SIMD_VVVE[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if !op2.as_::<Vec>().has_element_index() {
                        if !check_signature!(op0, op1, op2) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if let Some(fp_opcode) = pick_fp_opcode(
                            op0.as_::<Vec>(),
                            op_data.scalar_op(),
                            op_data.scalar_hf(),
                            op_data.vector_op(),
                            op_data.vector_hf(),
                            &mut 0,
                        ) {
                            opcode.reset(fp_opcode.0);

                            emit_rd0_rn5_rm16!();
                        }

                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    } else {
                        if !check_signature!(op0, op1) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        let q = op1.as_::<Reg>().is_vec128() as u32;
                        let mut sz = 0;
                        if let Some((fp_opcode)) = pick_fp_opcode(
                            op0.as_::<Vec>(),
                            op_data.element_scalar_op(),
                            5,
                            op_data.element_vector_op(),
                            5,
                            &mut sz,
                        ) {
                            if sz == 0 && op2.as_::<Reg>().id() > 15 {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            let element_index = op2.as_::<Vec>().element_index();
                            if element_index > (7u32 >> sz) {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            let hlm = element_index << sz;
                            opcode.reset(fp_opcode.0);
                            opcode.add_imm(q, 30);
                            opcode.add_imm(hlm & 3, 20);
                            opcode.add_imm(hlm >> 2, 11);
                            emit_rd0_rn5_rm16!();
                        }

                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                }
            }

            Encoding::FSimdVVVV => {
                let op_data = &F_SIMD_VVVV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg, Reg) {
                    if !check_signature!(op0, op1, op2, op3) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if let Some(fp_opcode) = pick_fp_opcode(
                        op0.as_::<Vec>(),
                        op_data.scalar_op(),
                        op_data.scalar_hf(),
                        op_data.vector_op(),
                        op_data.vector_hf(),
                        &mut 0,
                    ) {
                        opcode.reset(fp_opcode.0);
                        emit_rd0_rn5_rm16_ra10!();
                    }

                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }
            }

            Encoding::SimdFcadd => {
                let op_data = &SIMD_FCADD[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg, Imm) {
                    if !check_signature!(op0, op1, op2) || op0.as_::<Vec>().has_element_index() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let q = (op0.as_::<Reg>().is_vec128() as u32).saturating_sub(1);
                    if q > 1 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let mut sz = op0.as_::<Vec>().element_type() as u32;
                    sz = sz.saturating_sub(1);
                    if sz == 0 || sz > 3 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let mut rot = 0u32;
                    let imm_val = op3.as_::<Imm>().value();
                    if imm_val == 270 {
                        rot = 1;
                    } else if imm_val != 90 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.reset(op_data.opcode());
                    opcode.add_imm(q, 30);
                    opcode.add_imm(sz, 22);
                    opcode.add_imm(rot, 12);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::SimdFccmpFccmpe => {
                let op_data = &SIMD_FCCMP_FCCMPE[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Imm, Imm) {
                    let mut sz = op0.as_::<Reg>().typ() as u32;
                    sz = sz.saturating_sub(4);
                    if sz > 2 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_signature!(op0, op1) || op0.as_::<Vec>().has_element_type() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let nzcv = op2.as_::<Imm>().value();
                    let cond = op3.as_::<Imm>().value();

                    if (nzcv | cond) > 0xFi64 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let type_field = (sz - 1) & 0x3u32;

                    opcode.reset(op_data.opcode());
                    opcode.add_imm(type_field, 22);
                    opcode.add_imm(cond_code_to_opcode_field(cond as u32), 12);
                    opcode.add_imm(nzcv as u32, 0);
                    emit_rn5_rm16!();
                }
            }

            Encoding::SimdFcm => {
                let op_data = &SIMD_FCM[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) && op_data.has_register_op() {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if let Some(fp_opcode) = pick_fp_opcode(
                        op0.as_::<Vec>(),
                        op_data.register_scalar_op(),
                        op_data.register_scalar_hf(),
                        op_data.register_vector_op(),
                        op_data.register_vector_hf(),
                        &mut 0,
                    ) {
                        opcode.reset(fp_opcode.0);
                        emit_rd0_rn5_rm16!();
                    }

                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                if isign4 == enc_ops!(Reg, Reg, Imm) && op_data.has_zero_op() {
                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if op2.as_::<Imm>().value() != 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if let Some(fp_opcode) = pick_fp_opcode(
                        op0.as_::<Vec>(),
                        op_data.zero_scalar_op(),
                        5,
                        op_data.zero_vector_op(),
                        5,
                        &mut 0,
                    ) {
                        opcode.reset(fp_opcode.0);
                        emit_rd0_rn5!();
                    }

                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }
            }

            Encoding::SimdFcmla => {
                let op_data = &SIMD_FCMLA[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg, Imm) {
                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let q = (op0.as_::<Reg>().is_vec128() as u32).saturating_sub(1);
                    if q > 1 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let mut sz = op0.as_::<Vec>().element_type() as u32;
                    sz = sz.saturating_sub(1);
                    if sz == 0 || sz > 3 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let mut rot = 0u32;
                    match op3.as_::<Imm>().value() {
                        0 => rot = 0,
                        90 => rot = 1,
                        180 => rot = 2,
                        270 => rot = 3,
                        _ => {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                    }

                    if !op2.as_::<Vec>().has_element_index() {
                        if !check_signature!(op1, op2) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        opcode.reset(op_data.regular_op());
                        opcode.add_imm(q, 30);
                        opcode.add_imm(sz, 22);
                        opcode.add_imm(rot, 11);
                        emit_rd0_rn5_rm16!();
                    } else {
                        if op0.as_::<Vec>().element_type() != op2.as_::<Vec>().element_type() {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        if !((sz == 1) || (q == 1 && sz == 2)) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        let element_index = op2.as_::<Vec>().element_index();
                        let hl_field_shift = if sz == 1 { 0u32 } else { 1u32 };
                        let max_element_index = if q == 1 && sz == 1 { 3u32 } else { 1u32 };

                        if element_index > max_element_index {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        let hl = element_index << hl_field_shift;

                        opcode.reset(op_data.element_op());
                        opcode.add_imm(q, 30);
                        opcode.add_imm(sz, 22);
                        opcode.add_imm(hl & 1u32, 21);
                        opcode.add_imm(hl >> 1, 11);
                        opcode.add_imm(rot, 13);
                        emit_rd0_rn5_rm16!();
                    }
                }
            }

            Encoding::SimdFcmpFcmpe => {
                let op_data = &SIMD_FCMP_FCMPE[encoding_index];

                let sz = (op0.as_::<Reg>().typ() as u32).saturating_sub(RegType::Vec16 as u32);
                let type_field = (sz - 1) & 0x3u32;

                if sz > 2 {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                if op0.as_::<Vec>().has_element_type() {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                opcode.reset(op_data.opcode());
                opcode.add_imm(type_field, 22);

                if isign4 == enc_ops!(Reg, Reg) {
                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    emit_rd0_rn5_rm16!();
                } else if isign4 == enc_ops!(Reg, Imm) {
                    if op1.as_::<Imm>().value() != 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    opcode.0 |= 0x8;
                    emit_rd0_rn5!();
                }
            }

            Encoding::SimdFcsel => {
                if isign4 == enc_ops!(Reg, Reg, Reg, Imm) {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let sz = (op0.as_::<Reg>().typ() as u32).saturating_sub(RegType::Vec16 as u32);
                    let type_field = (sz - 1) & 0x3u32;

                    if sz > 2 || op0.as_::<Vec>().has_element_type() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let cond = op3.as_::<Imm>().value() as u32;
                    if cond > 0xFu32 {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }

                    opcode.reset(0b00011110001000000000110000000000u32);
                    opcode.add_imm(type_field, 22);
                    opcode.add_imm(cond, 12);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::SimdFcvt => {
                if isign4 == enc_ops!(Reg, Reg) {
                    let dst_sz =
                        (op0.as_::<Reg>().reg_type() as u32).saturating_sub(RegType::Vec16 as u32);
                    let src_sz =
                        (op1.as_::<Reg>().reg_type() as u32).saturating_sub(RegType::Vec16 as u32);

                    if (dst_sz | src_sz) > 3 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if op0.as_::<Vec>().has_element_type() || op1.as_::<Vec>().has_element_type() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    // Table that provides 'type' and 'opc' according to the dst/src combination.
                    let table: [u8; 16] = [
                        0xFFu8, // H <- H (Invalid).
                        0x03u8, // H <- S (type=00 opc=11).
                        0x13u8, // H <- D (type=01 opc=11).
                        0xFFu8, // H <- Q (Invalid).
                        0x30u8, // S <- H (type=11 opc=00).
                        0xFFu8, // S <- S (Invalid).
                        0x10u8, // S <- D (type=01 opc=00).
                        0xFFu8, // S <- Q (Invalid).
                        0x31u8, // D <- H (type=11 opc=01).
                        0x01u8, // D <- S (type=00 opc=01).
                        0xFFu8, // D <- D (Invalid).
                        0xFFu8, // D <- Q (Invalid).
                        0xFFu8, // Q <- H (Invalid).
                        0xFFu8, // Q <- S (Invalid).
                        0xFFu8, // Q <- D (Invalid).
                        0xFFu8, // Q <- Q (Invalid).
                    ];

                    let type_opc = table[((dst_sz << 2) | src_sz) as usize];
                    if type_opc == 0xFFu8 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    opcode.reset(0b0001111000100010010000 << 10);
                    opcode.add_imm((type_opc as u32) >> 4, 22);
                    opcode.add_imm((type_opc as u32) & 15, 15);
                    emit_rd0_rn5!();
                }
            }

            Encoding::SimdFcvtLN => {
                let op_data = &SIMD_FCVT_LN[encoding_index];

                if isign4 == enc_ops!(Reg, Reg) {
                    // Scalar form - only FCVTXN.
                    if op0.as_::<Vec>().is_vec32() && op1.as_::<Vec>().is_vec64() {
                        if op_data.has_scalar() == 0 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if op0.as_::<Vec>().has_element_type()
                            || op1.as_::<Vec>().has_element_type()
                        {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        opcode.reset(op_data.scalar_op());
                        opcode.0 |= 0x400000; // sz bit must be 1
                        emit_rd0_rn5!();
                        return;
                    }

                    opcode.reset(op_data.vector_op());

                    let is_long = (inst_flags & InstFlag::Long as u16) != 0;
                    let (rl, rn) = if is_long {
                        (op0.as_::<Vec>(), op1.as_::<Vec>())
                    } else {
                        (op1.as_::<Vec>(), op0.as_::<Vec>())
                    };

                    let q = (rn.reg_type() as u32).saturating_sub(RegType::Vec64 as u32);
                    if (opcode.has_q() as u32) != q {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if rl.is_vec_s4()
                        && rn.element_type() == VecElementType::H
                        && op_data.is_cvtxn() == 0
                    {
                        emit_rd0_rn5!();
                        return;
                    }

                    if rl.is_vec_d2() && rn.element_type() == VecElementType::S {
                        opcode.0 |= 0x400000;
                        emit_rd0_rn5!();
                        return;
                    }

                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }
            }

            Encoding::SimdFcvtSV => {
                let op_data = &SIMD_FCVT_SV[encoding_index];

                // So we can support both IntToFloat and FloatToInt conversions.
                let is_float_to_int = op_data.is_float_to_int();
                let (op_gp, op_vec) = if is_float_to_int != 0 {
                    (&op0, &op1)
                } else {
                    (&op1, &op0)
                };

                if isign4 == enc_ops!(Reg, Reg) {
                    if op_gp.as_::<Reg>().is_gp() && op_vec.as_::<Reg>().is_vec() {
                        let x = op_gp.as_::<Reg>().is_gp64() as u32;
                        let type_field = (op_vec.as_::<Reg>().reg_type() as u32)
                            .saturating_sub(RegType::Vec16 as u32);

                        if type_field > 2u32 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        let type_val = (type_field - 1u32) & 0x3;
                        opcode.reset(op_data.general_op());
                        opcode.add_imm(type_val, 22);
                        opcode.add_imm(x, 31);
                        emit_rd0_rn5!();
                    } else if op0.as_::<Reg>().is_vec() && op1.as_::<Reg>().is_vec() {
                        if !check_signature!(op0, op1) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if let Some(fp_opcode) = pick_fp_opcode(
                            op0.as_::<Vec>(),
                            op_data.scalar_int_op(),
                            5,
                            op_data.vector_int_op(),
                            5,
                            &mut 0,
                        ) {
                            opcode.reset(fp_opcode.0);
                            emit_rd0_rn5!();
                        } else {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                    }
                } else if isign4 == enc_ops!(Reg, Reg, Imm) && op_data.is_fixed_point() {
                    let scale_val = op2.as_::<Imm>().value() as u32;
                    if scale_val >= 64 {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }

                    if scale_val == 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if op_gp.as_::<Reg>().is_gp() && op_vec.as_::<Reg>().is_vec() {
                        let x = op_gp.as_::<Reg>().is_gp64() as u32;
                        let type_field = (op_vec.as_::<Reg>().reg_type() as u32)
                            .saturating_sub(RegType::Vec16 as u32);

                        let scale_limit = 32u32 << x;
                        if scale_val > scale_limit {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        let type_val = (type_field - 1u32) & 0x3;
                        opcode.reset(op_data.general_op() ^ 0x200000);
                        opcode.add_imm(type_val, 22);
                        opcode.add_imm(x, 31);
                        opcode.add_imm(64u32 - scale_val, 10);
                        emit_rd0_rn5!();
                    } else if op0.as_::<Reg>().is_vec() && op1.as_::<Reg>().is_vec() {
                        if !check_signature!(op0, op1) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        let mut sz = 0u32;
                        if let Some(fp_opcode) = pick_fp_opcode(
                            op0.as_::<Vec>(),
                            op_data.scalar_fp_op(),
                            5,
                            op_data.vector_fp_op(),
                            5,
                            &mut sz,
                        ) {
                            let scale_limit = 16u32 << sz;
                            if scale_val > scale_limit {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            let imm = (!(scale_val) + 1) & ((1u32 << (sz + 4 + 1)) - 1);
                            opcode.reset(fp_opcode.0);
                            opcode.add_imm(imm, 16);
                            emit_rd0_rn5!();
                        } else {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                    }
                }
            }

            Encoding::SimdFmlal => {
                let op_data = &SIMD_FMLAL[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    let mut q =
                        (op0.as_::<Reg>().reg_type() as u32).saturating_sub(RegType::Vec64 as u32);
                    let q_is_optional = op_data.optional_q() != 0;

                    if q_is_optional {
                        if q > 1 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                    } else {
                        if q != 1 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        q = 0;
                    }

                    if (op0.as_::<Reg>().reg_type() as u32)
                        != (op1.as_::<Reg>().reg_type() as u32) + if q_is_optional { 1 } else { 0 }
                        || (op0.as_::<Vec>().element_type() as u32) != op_data.ta as u32
                        || (op1.as_::<Vec>().element_type() as u32) != op_data.tb as u32
                    {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !op2.as_::<Vec>().has_element_index() {
                        if !check_signature!(&op1, &op2) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        opcode.reset(op_data.vector_op());
                        opcode.add_imm(q, 30);
                        emit_rd0_rn5_rm16!();
                    } else {
                        if (op2.as_::<Vec>().element_type() as u32) != op_data.t_element as u32 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if op2.as_::<Reg>().id() > 15 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        let element_index = op2.as_::<Vec>().element_index();
                        if element_index > 7u32 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        opcode.reset(op_data.element_op());
                        opcode.add_imm(q, 30);
                        opcode.add_imm(element_index & 3u32, 20);
                        opcode.add_imm(element_index >> 2, 11);
                        emit_rd0_rn5_rm16!();
                    }
                }
            }

            Encoding::SimdFmov => {
                if isign4 == enc_ops!(Reg, Reg) {
                    // FMOV Gp <-> Vec opcode:
                    opcode.reset(0b00011110001001100000000000000000);

                    if (op0.as_::<Reg>().is_gp() && op1.as_::<Reg>().is_vec()) {
                        // FMOV Wd, Hn      (sf=0 type=11 rmode=00 op=110)
                        // FMOV Xd, Hn      (sf=1 type=11 rmode=00 op=110)
                        // FMOV Wd, Sn      (sf=0 type=00 rmode=00 op=110)
                        // FMOV Xd, Dn      (sf=1 type=11 rmode=00 op=110)
                        // FMOV Xd, Vn.d[1] (sf=1 type=10 rmode=01 op=110)
                        let x = op0.as_::<Reg>().is_gp64();
                        let sz = (op1.as_::<Reg>().reg_type() as u32)
                            .saturating_sub(RegType::Vec16 as u32);

                        let mut typ = (sz - 1) & 0x3;
                        let mut r_mode_op = 0b00110;

                        if (op1.as_::<Vec>().has_element_index()) {
                            // Special case.
                            if (!x
                                || !op1.as_::<Vec>().is_vec_d2()
                                || op1.as_::<Vec>().element_index() != 1)
                            {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }
                            typ = 0b10;
                            r_mode_op = 0b01110;
                        } else {
                            // Must be scalar.
                            if (sz > 2) {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }

                            if (op1.as_::<Vec>().has_element_type()) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            if (op1.as_::<Vec>().is_vec32() && x) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            if (op1.as_::<Vec>().is_vec64() && !x) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }
                        }

                        opcode.add_imm(x as u32, 31);
                        opcode.add_imm(typ, 22);
                        opcode.add_imm(r_mode_op, 16);

                        emit_rd0_rn5!();
                    }

                    if (op0.as_::<Reg>().is_vec() && op1.as_::<Reg>().is_gp()) {
                        // FMOV Hd, Wn      (sf=0 type=11 rmode=00 op=111)
                        // FMOV Hd, Xn      (sf=1 type=11 rmode=00 op=111)
                        // FMOV Sd, Wn      (sf=0 type=00 rmode=00 op=111)
                        // FMOV Dd, Xn      (sf=1 type=11 rmode=00 op=111)
                        // FMOV Vd.d[1], Xn (sf=1 type=10 rmode=01 op=111)
                        let x = op1.as_::<Reg>().is_gp64();
                        let sz = (op0.as_::<Reg>().reg_type() as u32)
                            .saturating_sub(RegType::Vec16 as u32);

                        let mut typ = (sz - 1) & 0x3;
                        let mut r_mode_op = 0b00111;

                        if (op0.as_::<Vec>().has_element_index()) {
                            // Special case.
                            if (!x
                                || !op0.as_::<Vec>().is_vec_d2()
                                || op0.as_::<Vec>().element_index() != 1)
                            {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }
                            typ = 0b10;
                            r_mode_op = 0b01111;
                        } else {
                            // Must be scalar.
                            if (sz > 2) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            if (op0.as_::<Vec>().has_element_type()) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            if (op0.as_::<Vec>().is_vec32() && x) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            if (op0.as_::<Vec>().is_vec64() && !x) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }
                        }

                        opcode.add_imm(x as u32, 31);
                        opcode.add_imm(typ, 22);
                        opcode.add_imm(r_mode_op, 16);
                        emit_rd0_rn5!();
                    }

                    if check_signature!(op0, op1) {
                        let sz = (op0.as_::<Reg>().reg_type() as u32)
                            .saturating_sub(RegType::Vec16 as u32);
                        if sz > 2 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if op0.as_::<Vec>().has_element_type() {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        let typ = (sz - 1) & 0x3;
                        opcode.reset(0b00011110001000000100000000000000);
                        opcode.add_imm(typ, 22);
                        emit_rd0_rn5!();
                    }
                }

                if isign4 == enc_ops!(Reg, Imm) {
                    if op0.as_::<Reg>().is_vec() {
                        let fp_value = if op1.as_::<Imm>().is_double() {
                            op1.as_::<Imm>().value_f64()
                        } else if op1.as_::<Imm>().is_int32() {
                            op1.as_::<Imm>().value_as::<i32>() as f64
                        } else {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        };

                        if !is_fp64_imm8(fp_value.to_bits()) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        let imm8 = encode_fp64_to_imm8(fp_value.to_bits());

                        if !op0.as_::<Vec>().has_element_type() {
                            let sz = (op0.as_::<Reg>().reg_type() as u32)
                                .saturating_sub(RegType::Vec16 as u32);
                            let typ = (sz - 1) & 0x3;
                            if sz > 2 {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            opcode.reset(0b00011110001000000001000000000000);
                            opcode.add_imm(typ, 22);
                            opcode.add_imm(imm8, 13);
                            emit_rd0!();
                        } else {
                            let q = (op0.as_::<Reg>().reg_type() as u32)
                                .saturating_sub(RegType::Vec64 as u32);
                            let sz = (op0.as_::<Vec>().element_type() as u32)
                                .saturating_sub(VecElementType::H as u32);

                            if q > 1 || sz > 2 {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            const SZ_BITS_TABLE: [u32; 3] = [1 << 11, 0, 1 << 29];
                            opcode.reset(0b00001111000000001111010000000000);
                            opcode ^= SZ_BITS_TABLE[sz as usize];
                            opcode.add_imm(q, 30);
                            opcode.add_imm(imm8 >> 5, 16);
                            opcode.add_imm(imm8 & 31, 5);
                            emit_rd0!();
                        }
                    }
                }
            }

            Encoding::FSimdPair => {
                let op_data = &F_SIMD_PAIR[encoding_index];

                if isign4 == enc_ops!(Reg, Reg) {
                    // This operation is only defined for:
                    //   hD, vS.2h (16-bit)
                    //   sD, vS.2s (32-bit)
                    //   dD, vS.2d (64-bit)
                    let sz =
                        (op0.as_::<Reg>().reg_type() as u32).saturating_sub(RegType::Vec16 as u32);
                    if sz > 2 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    const SZ_SIGNATURES: [u32; 3] = [
                        A64VecS::SIGNATURE | (Vec::SIGNATURE_ELEMENT_H as u32),
                        A64VecD::SIGNATURE | (Vec::SIGNATURE_ELEMENT_S as u32),
                        A64VecQ::SIGNATURE | (Vec::SIGNATURE_ELEMENT_D as u32),
                    ];

                    if op0.signature().bits() != SZ_SIGNATURES[sz as usize] {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    const SZ_BITS_TABLE: [u32; 3] = [1 << 29, 0, 1 << 22];

                    opcode.reset(op_data.scalar_op());
                    opcode ^= SZ_BITS_TABLE[sz as usize];
                    emit_rd0_rn5!();
                }

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let q =
                        (op0.as_::<Reg>().reg_type() as u32).saturating_sub(RegType::Vec64 as u32);
                    if q > 1 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    const SZ_BITS_TABLE: [u32; 3] =
                        [(1 << 22) | (1 << 21) | (1 << 15) | (1 << 14), 0, 1 << 22];
                    opcode.reset(op_data.scalar_op());
                    opcode ^= SZ_BITS_TABLE[q as usize];
                    opcode.add_imm(q, 30);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::ISimdSV => {
                let op_data = &I_SIMD_SV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg) {
                    let l = (inst_flags & InstFlag::Long as u16) != 0;
                    if (op0.as_::<Vec>().reg_type() as u32).saturating_sub(RegType::Vec8 as u32)
                        != (op1.as_::<Vec>().element_type() as u32)
                            .saturating_sub(VecElementType::B as u32)
                            + l as u32
                    {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let size_op = element_type_to_size_op(
                        op_data.vec_op_type,
                        op1.as_::<Reg>().reg_type(),
                        op1.as_::<Vec>().element_type(),
                    );

                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    opcode.reset(op_data.opcode());
                    opcode.add_imm(size_op.q(), 30);
                    opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5!();
                }
            }

            Encoding::ISimdVV => {
                let op_data = &I_SIMD_VV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg) {
                    let sop = significant_simd_op(op0, op1, inst_flags as u32);
                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let size_op = element_type_to_size_op(
                        op_data.vec_op_type,
                        sop.as_::<Reg>().reg_type(),
                        sop.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    opcode.reset(op_data.opcode());
                    opcode.add_imm(size_op.qs(), 30);
                    opcode.add_imm(size_op.scalar(), 28);
                    opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5!();
                }
            }

            Encoding::ISimdVVx => {
                let op_data = &I_SIMD_VVX[encoding_index];
                if isign4 == enc_ops!(Reg, Reg) {
                    if op0.signature().bits() != op_data.op0_signature
                        || op1.signature().bits() != op_data.op1_signature
                    {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    opcode.reset(op_data.opcode());
                    emit_rd0_rn5!();
                }
            }

            Encoding::ISimdVVV => {
                let op_data = &I_SIMD_VVV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    let sop = significant_simd_op(op0, op1, inst_flags as u32);
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let size_op = element_type_to_size_op(
                        op_data.vec_op_type,
                        sop.as_::<Reg>().reg_type(),
                        sop.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    opcode.reset(op_data.opcode());
                    opcode.add_imm(size_op.qs(), 30);
                    opcode.add_imm(size_op.scalar(), 28);
                    opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::ISimdVVVx => {
                let op_data = &I_SIMD_VVVX[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if op0.signature().bits() != op_data.op0_signature
                        || op1.signature().bits() != op_data.op1_signature
                        || op2.signature().bits() != op_data.op2_signature
                    {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    opcode.reset(op_data.opcode());
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::ISimdWWV => {
                let op_data = &I_SIMD_WWV[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    let size_op = element_type_to_size_op(
                        op_data.vec_op_type,
                        op2.as_::<Reg>().reg_type(),
                        op2.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !check_signature!(op0, op1)
                        || !op0.as_::<Reg>().is_vec128()
                        || (op0.as_::<Vec>().element_type() as u32)
                            != (op2.as_::<Vec>().element_type() as u32 + 1)
                    {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    opcode.reset(op_data.opcode());
                    opcode.add_imm(size_op.qs(), 30);
                    opcode.add_imm(size_op.scalar(), 28);
                    opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::ISimdVVVe => {
                let op_data = &I_SIMD_VVVE[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    let sop = significant_simd_op(op0, op1, inst_flags as u32);
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !op2.as_::<Vec>().has_element_index() {
                        let size_op = element_type_to_size_op(
                            op_data.regular_vec_type,
                            sop.as_::<Reg>().reg_type(),
                            sop.as_::<Vec>().element_type(),
                        );
                        if !size_op.is_valid() {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                        if !check_signature!(op1, op2) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                        opcode.reset((op_data.regular_op as u32) << 10);
                        opcode.add_imm(size_op.qs(), 30);
                        opcode.add_imm(size_op.scalar(), 28);
                        opcode.add_imm(size_op.size(), 22);
                        emit_rd0_rn5_rm16!();
                    } else {
                        let size_op = element_type_to_size_op(
                            op_data.element_vec_type,
                            sop.as_::<Reg>().reg_type(),
                            sop.as_::<Vec>().element_type(),
                        );
                        if !size_op.is_valid() {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                        let element_index = op2.as_::<Vec>().element_index();
                        let mut lmh = LMHImm {
                            lm: 0,
                            h: 0,
                            max_rm_id: 0,
                        };
                        if !encode_lmh(size_op.size(), element_index, &mut lmh) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        if op2.as_::<Reg>().id() > lmh.max_rm_id {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        opcode.reset((op_data.element_op as u32) << 10);
                        opcode.add_imm(size_op.q(), 30);
                        opcode.add_imm(size_op.size(), 22);
                        opcode.add_imm(lmh.lm, 20);
                        opcode.add_imm(lmh.h, 11);
                        emit_rd0_rn5_rm16!();
                    }
                }
            }

            Encoding::ISimdVVVI => {
                let op_data = &I_SIMD_VVVI[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg, Imm) {
                    let sop = significant_simd_op(op0, op1, inst_flags as u32);
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let size_op = element_type_to_size_op(
                        op_data.vec_op_type,
                        sop.as_::<Reg>().reg_type(),
                        sop.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let imm_value = op3.as_::<Imm>().value_as::<u64>();
                    let mut imm_size = op_data.imm_size;
                    if op_data.imm64_has_one_bit_less != 0 && size_op.q() == 0 {
                        imm_size -= 1;
                    }
                    let imm_max = 1u64 << imm_size;
                    if imm_value >= imm_max {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }
                    opcode.reset(op_data.opcode());
                    opcode.add_imm(size_op.qs(), 30);
                    opcode.add_imm(size_op.scalar(), 28);
                    opcode.add_imm(size_op.size(), 22);
                    opcode.add_imm(imm_value as u32, op_data.imm_shift);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::ISimdVVVV => {
                let op_data = &I_SIMD_VVVV[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg, Reg) {
                    let sop = significant_simd_op(op0, op1, inst_flags as u32);
                    if !check_signature!(op0, op1, op2, op3) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let size_op = element_type_to_size_op(
                        op_data.vec_op_type,
                        sop.as_::<Reg>().reg_type(),
                        sop.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    opcode.reset((op_data.opcode as u32) << 10);
                    opcode.add_imm(size_op.qs(), 30);
                    opcode.add_imm(size_op.scalar(), 28);
                    opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5_rm16_ra10!();
                }
            }

            Encoding::ISimdVVVVx => {
                let op_data = &I_SIMD_VVVVX[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg, Reg) {
                    if op0.signature().bits() != op_data.op0_signature
                        || op1.signature().bits() != op_data.op1_signature
                        || op2.signature().bits() != op_data.op2_signature
                        || op3.signature().bits() != op_data.op3_signature
                    {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    opcode.reset((op_data.opcode as u32) << 10);
                    emit_rd0_rn5_rm16_ra10!();
                }
            }

            Encoding::ISimdPair => {
                let op_data = &I_SIMD_PAIR[encoding_index];
                if isign4 == enc_ops!(Reg, Reg) && op_data.opcode2 != 0 {
                    if op0.as_::<Vec>().is_vec_d1() && op1.as_::<Vec>().is_vec_d2() {
                        opcode.reset((op_data.opcode2 as u32) << 10);
                        opcode.add_imm(0x3, 22);
                        emit_rd0_rn5!();
                    }
                }
                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let size_op = element_type_to_size_op(
                        op_data.op_type3,
                        op0.as_::<Reg>().reg_type(),
                        op0.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    opcode.reset((op_data.opcode3 as u32) << 10);
                    opcode.add_imm(size_op.qs(), 30);
                    opcode.add_imm(size_op.scalar(), 28);
                    opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::SimdBicOrr => {
                let op_data = &SIMD_BIC_ORR[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let size_op = element_type_to_size_op(
                        0, // kVO_V_B
                        op0.as_::<Reg>().reg_type(),
                        op0.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    opcode.reset((op_data.register_op as u32) << 10);
                    opcode.add_imm(size_op.q(), 30);
                    emit_rd0_rn5_rm16!();
                }
                if isign4 == enc_ops!(Reg, Imm) || isign4 == enc_ops!(Reg, Imm, Imm) {
                    let size_op = element_type_to_size_op(
                        5, // kVO_V_HS
                        op0.as_::<Reg>().reg_type(),
                        op0.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if op1.as_::<Imm>().value_as::<u64>() > 0xFFFFFFFF {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }
                    let mut imm = op1.as_::<Imm>().value_as::<u32>();
                    let mut shift = 0u32;
                    let max_shift = (8u32 << size_op.size()) - 8u32;
                    if isign4 == enc_ops!(Reg, Imm, Imm) {
                        if op2.as_::<Imm>().predicate() != ShiftOp::LSL as u32 {
                            self.last_error = Some(AsmError::InvalidImmediate);
                            return;
                        }
                        if imm > 0xFF || op2.as_::<Imm>().value_as::<u64>() > max_shift as u64 {
                            self.last_error = Some(AsmError::InvalidImmediate);
                            return;
                        }
                        shift = op2.as_::<Imm>().value_as::<u32>();
                        if (shift & 0x7) != 0 {
                            self.last_error = Some(AsmError::InvalidImmediate);
                            return;
                        }
                    } else if imm != 0 {
                        shift = imm.trailing_zeros() & !0x7;
                        imm >>= shift;
                        if imm > 0xFF || shift > max_shift {
                            self.last_error = Some(AsmError::InvalidImmediate);
                            return;
                        }
                    }
                    let mut cmode = 0x1 | ((shift / 8) << 1);
                    if size_op.size() == 1 {
                        cmode |= 1 << 3;
                    }
                    let abc = (imm >> 5) & 0x7;
                    let defgh = imm & 0x1F;
                    opcode.reset((op_data.immediate_op as u32) << 10);
                    opcode.add_imm(size_op.q(), 30);
                    opcode.add_imm(abc, 16);
                    opcode.add_imm(cmode, 12);
                    opcode.add_imm(defgh, 5);
                    emit_rd0!();
                }
            }

            Encoding::SimdCmp => {
                let op_data = &SIMD_CMP[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) && op_data.register_op != 0 {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let size_op = element_type_to_size_op(
                        op_data.vec_op_type,
                        op0.as_::<Reg>().reg_type(),
                        op0.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    opcode.reset((op_data.register_op as u32) << 10);
                    opcode.add_imm(size_op.qs(), 30);
                    opcode.add_imm(size_op.scalar(), 28);
                    opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5_rm16!();
                }

                if isign4 == enc_ops!(Reg, Reg, Imm) && op_data.zero_op != 0 {
                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if op2.as_::<Imm>().value() != 0 {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }

                    let size_op = element_type_to_size_op(
                        op_data.vec_op_type,
                        op0.as_::<Reg>().reg_type(),
                        op0.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    opcode.reset((op_data.zero_op as u32) << 10);
                    opcode.add_imm(size_op.qs(), 30);
                    opcode.add_imm(size_op.scalar(), 28);
                    opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5!();
                }
            }

            Encoding::SimdDot => {
                let op_data = &SIMD_DOT[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    let q = op0.as_::<Reg>().reg_type() as u32 - RegType::Vec64 as u32;
                    let size = 2u32;

                    if q > 1 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !op2.as_::<Vec>().has_element_index() {
                        if op_data.vector_op == 0 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if op0.as_::<Reg>().reg_type() != op1.as_::<Reg>().reg_type()
                            || op1.as_::<Reg>().reg_type() != op2.as_::<Reg>().reg_type()
                        {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if op0.as_::<Vec>().element_type() as u32 != op_data.ta as u32
                            || op1.as_::<Vec>().element_type() as u32 != op_data.tb as u32
                            || op2.as_::<Vec>().element_type() as u32 != op_data.tb as u32
                        {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        opcode.reset((op_data.vector_op as u32) << 10);
                        opcode.add_imm(q, 30);
                        emit_rd0_rn5_rm16!();
                    } else {
                        if op_data.element_op == 0 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if op0.as_::<Reg>().reg_type() != op1.as_::<Reg>().reg_type()
                            || !op2.as_::<Reg>().is_vec128()
                        {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if op0.as_::<Vec>().element_type() as u32 != op_data.ta as u32
                            || op1.as_::<Vec>().element_type() as u32 != op_data.tb as u32
                            || op2.as_::<Vec>().element_type() as u32 != op_data.t_element as u32
                        {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        let element_index = op2.as_::<Vec>().element_index();
                        let mut lmh = LMHImm {
                            lm: 0,
                            h: 0,
                            max_rm_id: 0,
                        };
                        if !encode_lmh(size, element_index, &mut lmh) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        if op2.as_::<Reg>().id() > lmh.max_rm_id {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        opcode.reset((op_data.element_op as u32) << 10);
                        opcode.add_imm(q, 30);
                        opcode.add_imm(lmh.lm, 20);
                        opcode.add_imm(lmh.h, 11);
                        emit_rd0_rn5_rm16!();
                    }
                }
            }

            Encoding::SimdDup => {
                simd_dup!();
            }

            Encoding::SimdIns => {
                simd_insn!();
            }

            Encoding::SimdMov => {
                if isign4 == enc_ops!(Reg, Reg) {
                    if op0.as_::<Reg>().is_vec() && op1.as_::<Reg>().is_vec() {
                        // INS v.x[index], v.x[index].
                        if op0.as_::<Vec>().has_element_index()
                            && op1.as_::<Vec>().has_element_index()
                        {
                            // SimdIns encoding.

                            encoding = Encoding::SimdIns;
                            // Recurse to SimdIns.
                            simd_insn!();
                            return;
                        }
                        // DUP {b|h|s|d}, v.{b|h|s|d}[index].
                        if op1.as_::<Vec>().has_element_index() {
                            encoding = Encoding::SimdDup;
                            simd_dup!();
                            return;
                        }
                        if !check_signature!(op0, op1) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                        let q = op0.as_::<Reg>().reg_type() as u32 - RegType::Vec64 as u32;
                        if q > 1 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                        opcode.reset(0b0000111010100000000111 << 10);
                        opcode.add_imm(q, 30);
                        opcode.add_reg(op1.id(), 16);
                        emit_rd0_rn5!();
                        return;
                    }
                    if op0.as_::<Reg>().is_vec() && op1.as_::<Reg>().is_gp() {
                        // INS v.x[index], Rn.
                        if op0.as_::<Vec>().has_element_index() {
                            encoding = Encoding::SimdIns;
                            simd_insn!();
                            return;
                        }
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if op0.as_::<Reg>().is_gp() && op1.as_::<Reg>().is_vec() {
                        // UMOV Rd, V.{s|d}[index].
                        encoding_index = 1;
                        encoding = Encoding::SimdSmovUmov;
                        simd_umov!();
                        return;
                    }
                }
            }

            Encoding::SimdMoviMvni => {
                let op_data = &SIMD_MOVI_MVNI[encoding_index];
                if isign4 == enc_ops!(Reg, Imm) || isign4 == enc_ops!(Reg, Imm, Imm) {
                    let mut size_op = element_type_to_size_op(
                        20,
                        op0.as_::<Reg>().reg_type(),
                        op0.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let mut imm64 = op1.as_::<Imm>().value_as::<u64>();
                    let mut imm8 = 0u32;
                    let mut cmode = 0u32;
                    let inverted = op_data.inverted;
                    let mut op = 0u32;
                    let mut shift = 0u32;
                    let mut shift_op = ShiftOp::LSL as u32;
                    if size_op.size() == 3 {
                        if op2.is_imm() && op2.as_::<Imm>().value() != 0 {
                            self.last_error = Some(AsmError::InvalidImmediate);
                            return;
                        }
                        if is_byte_mask_imm(imm64) {
                            imm8 = encode_imm64_byte_mask_to_imm8(imm64);
                        } else {
                            if (imm64 >> 32) == (imm64 & 0xFFFFFFFF) {
                                imm64 &= 0xFFFFFFFF;
                                size_op.decrement_size();
                            } else {
                                self.last_error = Some(AsmError::InvalidImmediate);
                                return;
                            }
                        }
                    }
                    if size_op.size() < 3 {
                        if imm64 > 0xFFFFFFFF {
                            self.last_error = Some(AsmError::InvalidImmediate);
                            return;
                        }
                        imm8 = imm64 as u32;
                        if size_op.size() == 2 {
                            if (imm8 >> 16) == (imm8 & 0xFFFF) {
                                imm8 >>= 16;
                                size_op.decrement_size();
                            }
                        }
                        if size_op.size() == 1 {
                            if imm8 > 0xFFFF {
                                self.last_error = Some(AsmError::InvalidImmediate);
                                return;
                            }
                            if (imm8 >> 8) == (imm8 & 0xFF) {
                                imm8 >>= 8;
                                size_op.decrement_size();
                            }
                        }
                        let max_shift = (8u32 << size_op.size()) - 8u32;
                        if op2.is_imm() {
                            if imm8 > 0xFF || op2.as_::<Imm>().value_as::<u64>() > max_shift as u64
                            {
                                self.last_error = Some(AsmError::InvalidImmediate);
                                return;
                            }
                            shift = op2.as_::<Imm>().value_as::<u32>();
                            shift_op = op2.as_::<Imm>().predicate();
                        } else if imm8 != 0 {
                            shift = imm8.trailing_zeros() & !0x7;
                            imm8 >>= shift;
                            if imm8 > 0xFF || shift > max_shift {
                                self.last_error = Some(AsmError::InvalidImmediate);
                                return;
                            }
                        }
                        if (shift & 0x7) != 0 {
                            self.last_error = Some(AsmError::InvalidImmediate);
                            return;
                        }
                    }
                    shift /= 8;
                    match size_op.size() {
                        0 => {
                            if shift_op != ShiftOp::LSL as u32 {
                                self.last_error = Some(AsmError::InvalidImmediate);
                                return;
                            }
                            if inverted != 0 {
                                imm8 = !imm8 & 0xFF;
                            }
                            cmode = B!(3) | B!(2) | B!(1);
                        }
                        1 => {
                            if shift_op != ShiftOp::LSL as u32 {
                                self.last_error = Some(AsmError::InvalidImmediate);
                                return;
                            }
                            cmode = B!(3) | (shift << 1);
                            op = inverted;
                        }
                        2 => {
                            if shift_op == ShiftOp::LSL as u32 {
                                cmode = shift << 1;
                            } else if shift_op == ShiftOp::MSL as u32 {
                                if shift == 0 || shift > 2 {
                                    self.last_error = Some(AsmError::InvalidImmediate);
                                    return;
                                }
                                cmode = B!(3) | B!(2) | (shift - 1);
                            } else {
                                self.last_error = Some(AsmError::InvalidImmediate);
                                return;
                            }
                            op = inverted;
                        }
                        3 => {
                            if inverted != 0 {
                                imm8 = !imm8 & 0xFF;
                            }
                            op = 1;
                            cmode = B!(3) | B!(2) | B!(1);
                        }
                        _ => {}
                    }
                    let abc = (imm8 >> 5) & 0x7;
                    let defgh = imm8 & 0x1F;
                    opcode.reset((op_data.opcode as u32) << 10);
                    opcode.add_imm(size_op.q(), 30);
                    opcode.add_imm(op, 29);
                    opcode.add_imm(abc, 16);
                    opcode.add_imm(cmode, 12);
                    opcode.add_imm(defgh, 5);
                    emit_rd0!();
                    return;
                }
            }

            Encoding::SimdShift => {
                let op_data = &SIMD_SHIFT[encoding_index];
                let sop = significant_simd_op(op0, op1, inst_flags as u32);
                let size_op = element_type_to_size_op(
                    op_data.vec_op_type,
                    sop.as_::<Reg>().reg_type(),
                    sop.as_::<Vec>().element_type(),
                );
                if !size_op.is_valid() {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }
                if isign4 == enc_ops!(Reg, Reg, Imm) && op_data.immediate_op != 0 {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if op2.as_::<Imm>().value_as::<u64>() > 63 {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }
                    let lsb_shift = size_op.size() + 3;
                    let lsb_mask = (1u32 << lsb_shift) - 1;
                    let mut imm = op2.as_::<Imm>().value_as::<u32>();
                    if op_data.inverted_imm != 0 {
                        if imm == 0 || imm > (1u32 << lsb_shift) {
                            self.last_error = Some(AsmError::InvalidImmediate);
                            return;
                        }
                        imm = (!imm + 1) & lsb_mask;
                    }
                    if imm > lsb_mask {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }
                    imm |= 1u32 << lsb_shift;
                    opcode.reset((op_data.immediate_op as u32) << 10);
                    opcode.add_imm(size_op.qs(), 30);
                    opcode.add_imm(size_op.scalar(), 28);
                    opcode.add_imm(imm, 16);
                    emit_rd0_rn5!();
                    return;
                }
                if isign4 == enc_ops!(Reg, Reg, Reg) && op_data.register_op != 0 {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    opcode.reset((op_data.register_op as u32) << 10);
                    opcode.add_imm(size_op.qs(), 30);
                    opcode.add_imm(size_op.scalar(), 28);
                    opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5_rm16!();
                    return;
                }
            }

            Encoding::SimdShiftES => {
                let op_data = &SIMD_SHIFT_ES[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Imm) {
                    let size_op = element_type_to_size_op(
                        op_data.vec_op_type,
                        op1.as_::<Reg>().reg_type(),
                        op1.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let shift = op2.as_::<Imm>().value_as::<u64>();
                    let shift_op = op2.as_::<Imm>().predicate();
                    if shift != (8u64 << size_op.size()) || shift_op != ShiftOp::LSL as u32 {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }
                    opcode.reset((op_data.opcode as u32) << 10);
                    opcode.add_imm(size_op.q(), 30);
                    opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5!();
                    return;
                }
            }

            Encoding::SimdSm3tt => {
                let op_data = &SIMD_SM3TT[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if op0.as_::<Vec>().is_vec_s4()
                        && op1.as_::<Vec>().is_vec_s4()
                        && op2.as_::<Vec>().is_vec_s4()
                        && op2.as_::<Vec>().has_element_index()
                    {
                        let imm2 = op2.as_::<Vec>().element_index();
                        if imm2 > 3 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        opcode.reset((op_data.opcode as u32) << 10);
                        opcode.add_imm(imm2, 12);
                        emit_rd0_rn5_rm16!();
                        return;
                    }
                }
            }

            Encoding::SimdSmovUmov => {
                simd_umov!();
            }

            Encoding::SimdSxtlUxtl => {
                let op_data = &SIMD_SXTL_UXTL[encoding_index];
                if isign4 == enc_ops!(Reg, Reg) {
                    let size_op = element_type_to_size_op(
                        op_data.vec_op_type,
                        op1.as_::<Reg>().reg_type(),
                        op1.as_::<Vec>().element_type(),
                    );
                    if !size_op.is_valid() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    opcode.reset((op_data.opcode as u32) << 10);
                    opcode.add_imm(size_op.q(), 30);
                    opcode.add_imm(1u32, size_op.size() + 19);
                    emit_rd0_rn5!();
                    return;
                }
            }

            Encoding::SimdTblTbx => {
                let op_data = &SIMD_TBL_TBX[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg) || isign4 == enc_ops!(Reg, Reg, Reg, Reg) {
                    opcode.reset((op_data.opcode as u32) << 10);

                    let q = op0.as_::<Reg>().reg_type() as u32 - RegType::Vec64 as u32;
                    if q > 1 || op0.as_::<Vec>().has_element_index() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !op1.as_::<Vec>().is_vec_b16() || op1.as_::<Vec>().has_element_index() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let len =
                        (!op3.is_none() as u32) + (!op4.is_none() as u32) + (!op5.is_none() as u32);
                    opcode.add_imm(q, 30);
                    opcode.add_imm(len, 13);

                    match len {
                        0 => {
                            if !check_signature!(op0, op2) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }
                            if op2.id() > 31 {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }
                            opcode.add_reg(op2.id(), 16);
                            emit_rd0_rn5!();
                            return;
                        }
                        1 => {
                            if !check_signature!(op0, op3) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }
                            if op3.id() > 31 {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }
                            opcode.add_reg(op3.id(), 16);
                            emit_rd0_rn5!();
                            return;
                        }
                        2 => {
                            if !check_signature!(op0, op4) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }
                            if op4.id() > 31 {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }
                            opcode.add_reg(op4.id(), 16);
                            emit_rd0_rn5!();
                            return;
                        }
                        3 => {
                            if !check_signature!(op0, op5) {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }
                            if op5.id() > 31 {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }
                            opcode.add_reg(op5.id(), 16);
                            emit_rd0_rn5!();
                            return;
                        }
                        _ => {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                    }
                }
            }

            Encoding::SimdLdSt => {
                let op_data = &SIMD_LD_ST[encoding_index];
                if isign4 == enc_ops!(Reg, Mem) {
                    let m = op1.as_::<Mem>();
                    rm_rel = op1;

                    let xsz = op0.as_::<Reg>().reg_type() as u32 - RegType::Vec8 as u32;
                    if xsz > 4 || op0.as_::<Vec>().has_element_index() {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_vec_id(op0) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    // TODO: check_mem_base_index_rel(m)
                    let offset = m.offset();
                    if m.has_base_reg() {
                        if m.has_index() {
                            let opt = SHIFT_OP_TO_LD_ST_OP_MAP[m.shift_op() as usize];
                            if opt == 0xFF {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }
                            let shift = m.shift();
                            let s = if shift != 0 { 1 } else { 0 };
                            if s != 0 && shift != xsz {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }
                            opcode.reset((op_data.register_op as u32) << 21);
                            opcode.add_imm(xsz & 3, 30);
                            opcode.add_imm(xsz >> 2, 23);
                            opcode.add_imm(opt as u32, 13);
                            opcode.add_imm(s, 12);
                            opcode.0 |= 1 << 11;
                            opcode.add_reg(op0.id(), 0);
                            // Emit mem base index
                            opcode.add_reg(m.base_id(), 5);
                            opcode.add_reg(m.index_id(), 16);
                            return self.buffer.write_u32(opcode.get());
                        }

                        let offset32 = offset as i32;
                        if m.is_pre_or_post() {
                            if offset32 < -256 || offset32 > 255 {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }
                            opcode.reset((op_data.pre_post_op as u32) << 21);
                            opcode.add_imm(xsz & 3, 30);
                            opcode.add_imm(xsz >> 2, 23);
                            opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                            opcode.add_imm(m.is_pre_index() as u32, 11);
                            opcode.0 |= 1 << 10;
                            opcode.add_reg(op0.id(), 0);
                            opcode.add_reg(m.base_id(), 5);
                            return self.buffer.write_u32(opcode.get());
                        } else {
                            let imm12 = (offset32 as u32) >> xsz;
                            if imm12 >= (1 << 12) || ((imm12 << xsz) as i32) != offset32 {
                                // Fallback to SimdLdurStur
                                let op_data_ldur = &SIMD_LDUR_STUR[encoding_index];
                                if m.has_base_reg() && !m.has_index() && !m.is_pre_or_post() {
                                    if offset32 < -256 || offset32 > 255 {
                                        self.last_error = Some(AsmError::InvalidOperand);
                                        return;
                                    }
                                    opcode.reset((op_data_ldur.opcode as u32) << 10);
                                    opcode.add_imm(xsz & 3, 30);
                                    opcode.add_imm(xsz >> 2, 23);
                                    opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                                    opcode.add_reg(op0.id(), 0);
                                    opcode.add_reg(m.base_id(), 5);
                                    return self.buffer.write_u32(opcode.get());
                                }
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }
                            opcode.reset((op_data.u_offset_op as u32) << 22);
                            opcode.add_imm(xsz & 3, 30);
                            opcode.add_imm(xsz >> 2, 23);
                            opcode.add_imm(imm12, 10);
                            opcode.add_reg(op0.id(), 0);
                            opcode.add_reg(m.base_id(), 5);
                            return self.buffer.write_u32(opcode.get());
                        }
                    } else {
                        if op_data.literal_op == 0 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        if xsz < 2 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        let opc = xsz - 2;
                        opcode.reset((op_data.literal_op as u32) << 24);
                        opcode.add_imm(opc, 30);
                        opcode.add_reg(op0.id(), 0);
                        offset_format.reset_to_imm_type(OffsetType::SignedOffset, 4, 5, 19, 2);
                        rm_rel = op1;
                        emit_rel!();
                    }
                }
            }

            Encoding::SimdLdpStp => {
                let op_data = &SIMD_LDP_STP[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Mem) {
                    let m = op2.as_::<Mem>();
                    rm_rel = op2;

                    let opc = op0.as_::<Reg>().reg_type() as u32 - RegType::Vec32 as u32;
                    if opc > 2
                        || op0.as_::<Vec>().has_element_type()
                        || op0.as_::<Vec>().has_element_index()
                    {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !check_signature!(op0, op1) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !check_vec_id2(op0, op1) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if m.base_type() != RegType::Gp64 || m.has_index() {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let offset_shift = 2 + opc;
                    let offset32 = m.offset() as i32 >> offset_shift;
                    if ((offset32 << offset_shift) as i32) != m.offset() as i32 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if offset32 < -64 || offset32 > 63 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if m.is_pre_or_post() && offset32 != 0 {
                        if op_data.pre_post_op == 0 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        opcode.reset((op_data.pre_post_op as u32) << 22);
                        opcode.add_imm(m.is_pre_index() as u32, 24);
                    } else {
                        opcode.reset((op_data.offset_op as u32) << 22);
                    }
                    opcode.add_imm(opc, 30);
                    opcode.add_imm((offset32 as u32) & 0x7F, 15);
                    opcode.add_reg(op1.id(), 10);
                    opcode.add_reg(op0.id(), 0);
                    opcode.add_reg(m.base_id(), 5);
                    return self.buffer.write_u32(opcode.get());
                }
            }

            Encoding::SimdLdurStur => {
                let op_data = &SIMD_LDUR_STUR[encoding_index];
                if isign4 == enc_ops!(Reg, Mem) {
                    let m = op1.as_::<Mem>();
                    rm_rel = op1;

                    let sz = op0.as_::<Reg>().reg_type() as u32 - RegType::Vec8 as u32;
                    if sz > 4
                        || op0.as_::<Vec>().has_element_type()
                        || op0.as_::<Vec>().has_element_index()
                    {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !check_vec_id(op0) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    if m.has_base_reg() && !m.has_index() && !m.is_pre_or_post() {
                        let offset32 = m.offset() as i32;
                        if offset32 < -256 || offset32 > 255 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        opcode.reset((op_data.opcode as u32) << 10);
                        opcode.add_imm(sz & 3, 30);
                        opcode.add_imm(sz >> 2, 23);
                        opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                        opcode.add_reg(op0.id(), 0);
                        opcode.add_reg(m.base_id(), 5);
                        return self.buffer.write_u32(opcode.get());
                    }
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::SimdLdNStN => {
                let op_data = &SIMD_LD_N_ST_N[encoding_index];
                let o4 = *ops.get(4).unwrap_or(&NOREG);

                let mut n = 1;

                if isign4 == enc_ops!(Reg, Mem) {
                    if op_data.n != 1 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    rm_rel = op1;
                } else if isign4 == enc_ops!(Reg, Reg, Mem) {
                    if op_data.n != 1 && op_data.n != 2 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !check_signature!(op0, op1) || op0.id() + 1 != op1.id() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    n = 2;
                    rm_rel = op2;
                } else if isign4 == enc_ops!(Reg, Reg, Reg, Mem) && o4.is_none() {
                    if op_data.n != 1 && op_data.n != 3 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !check_signature!(op0, op1, op2)
                        || op0.id() + 1 != op1.id()
                        || op1.id() + 1 != op2.id()
                    {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    n = 3;
                    rm_rel = op3;
                } else if isign4 == enc_ops!(Reg, Reg, Reg, Reg) && o4.is_mem() {
                    if op_data.n != 1 && op_data.n != 4 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if !check_signature!(op0, op1, op2, op3)
                        || op0.id() + 1 != op1.id()
                        || op1.id() + 1 != op2.id()
                        || op2.id() + 1 != op3.id()
                    {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    n = 4;
                    rm_rel = &o4;
                } else {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                let v = op0.as_::<Vec>();
                let m = rm_rel.as_::<Mem>();

                let mut q = 0u32;
                let mut rm = 0u32;
                let mut rn = m.base_id();
                let sz = (v.element_type() as u32).wrapping_sub(VecElementType::B as u32);
                let mut opc_s_size = sz;
                let mut offset_possibility = 0u32;

                if sz > 3 {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                if m.base_type() != RegType::Gp64 {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                if rn > 30 && rn != Gp::ID_SP {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                rn &= 31;

                if op_data.replicate != 0 {
                    if n != op_data.n {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if v.has_element_index() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    q = (v.reg_type() as u32).wrapping_sub(RegType::Vec64 as u32);
                    if q > 1 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    opcode.reset((op_data.single_op as u32) << 10);
                    offset_possibility = (1u32 << sz) * n;
                } else if v.has_element_index() {
                    if n != op_data.n {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    const OPC_S_SIZE_BY_SZ_TABLE: [u32; 4] =
                        [0u32 << 3, 2u32 << 3, 4u32 << 3, (4u32 << 3) | 1u32];
                    opcode.reset((op_data.single_op as u32) << 10);
                    opc_s_size = OPC_S_SIZE_BY_SZ_TABLE[sz as usize];
                    offset_possibility = (1u32 << sz) * op_data.n;
                    let element_index = v.element_index();
                    let max_element_index = 15u32 >> sz;
                    if element_index > max_element_index {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let element_index_shifted = element_index << sz;
                    q = element_index_shifted >> 3;
                    opc_s_size |= element_index_shifted & 0x7;
                } else {
                    const OPC_S_SIZE_BY_N_TABLE: [u32; 5] =
                        [0u32, 0x7u32 << 2, 0xAu32 << 2, 0x6u32 << 2, 0x2u32 << 2];
                    q = (v.reg_type() as u32).wrapping_sub(RegType::Vec64 as u32);
                    if q > 1 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if op_data.n == 1 {
                        opc_s_size |= OPC_S_SIZE_BY_N_TABLE[n as usize];
                    }
                    opcode.reset((op_data.multiple_op as u32) << 10);
                    offset_possibility = (8u32 << q) * n;
                }

                if m.has_index() {
                    if m.has_offset() || !m.is_post_index() {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    rm = m.index_id();
                    if rm > 30 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    opcode.0 |= 1 << 23;
                } else {
                    if m.has_offset() {
                        if m.offset() != offset_possibility as i64 || !m.is_post_index() {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        rm = 31;
                        opcode.0 |= 1 << 23;
                    }
                }

                opcode.add_imm(q, 30);
                opcode.add_imm(rm, 16);
                opcode.add_imm(opc_s_size, 10);
                opcode.add_imm(rn, 5);

                opcode.add_reg(op0.id(), 0);
                return self.buffer.write_u32(opcode.get());
            }

            Encoding::None | Encoding::Count => (),
        }

        self.last_error = Some(AsmError::UnsupportedInstruction {
            reason: "Unsupported instruction encoding or operand types",
        });
    }
}

impl InstId {
    pub const ARM_COND: u32 = 0x78000000;
    pub const REAL_ID: u32 = 65535;
    pub const fn with_cc(self, cond: CondCode) -> u32 {
        let x = self as u32;
        x | (cond as u32) << Self::ARM_COND.trailing_zeros()
    }

    pub const fn extract_cc(inst: u32) -> CondCode {
        unsafe {
            core::mem::transmute(((inst & Self::ARM_COND) >> Self::ARM_COND.trailing_zeros()) as u8)
        }
    }

    pub const fn extract_real_id(inst: u32) -> u32 {
        inst & Self::REAL_ID
    }
}

pub const BLE: u32 = InstId::B.with_cc(CondCode::GE);

impl Into<u32> for InstId {
    fn into(self) -> u32 {
        self as u32
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
struct Opc(u32);

impl Opc {
    const N: u32 = 1 << 2;
    const Q: u32 = 1 << 30;
    const X: u32 = 1 << 31;

    pub fn reset(&mut self, value: u32) {
        self.0 = value;
    }

    pub fn get(&self) -> u32 {
        self.0
    }

    pub const fn has_q(&self) -> bool {
        (self.0 & Self::Q) != 0
    }
    pub const fn has_x(&self) -> bool {
        (self.0 & Self::X) != 0
    }

    pub fn add_imm(&mut self, value: u32, bit_index: u32) -> &mut Self {
        self.0 |= value << bit_index;
        self
    }

    pub fn xor_imm(&mut self, value: u32, bit_index: u32) -> &mut Self {
        self.0 ^= value << bit_index;
        self
    }

    pub fn add_if(&mut self, condition: bool, value: u32, bit_index: u32) -> &mut Self {
        if condition {
            self.0 |= value << bit_index;
        }
        self
    }

    pub fn add_logical_imm(&mut self, logical_imm: &LogicalImm) -> &mut Self {
        self.add_imm(logical_imm.n, 22)
            .add_imm(logical_imm.s, 10)
            .add_imm(logical_imm.r, 16);
        self
    }

    pub fn add_reg(&mut self, id: u32, bit_index: u32) -> &mut Self {
        self.0 |= (id & 31) << bit_index;
        self
    }
}

impl core::ops::BitOr<u32> for Opc {
    type Output = Self;

    fn bitor(self, rhs: u32) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl core::ops::BitOrAssign<u32> for Opc {
    fn bitor_assign(&mut self, rhs: u32) {
        self.0 |= rhs;
    }
}

impl core::ops::BitAnd<u32> for Opc {
    type Output = Self;

    fn bitand(self, rhs: u32) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl core::ops::BitAndAssign<u32> for Opc {
    fn bitand_assign(&mut self, rhs: u32) {
        self.0 &= rhs;
    }
}

impl core::ops::Not for Opc {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl core::ops::BitXor<u32> for Opc {
    type Output = Self;

    fn bitxor(self, rhs: u32) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

impl core::ops::BitXorAssign<u32> for Opc {
    fn bitxor_assign(&mut self, rhs: u32) {
        self.0 ^= rhs;
    }
}

impl core::ops::Shl<u32> for Opc {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl core::ops::ShlAssign<u32> for Opc {
    fn shl_assign(&mut self, rhs: u32) {
        self.0 <<= rhs;
    }
}

impl core::ops::Shr<u32> for Opc {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl core::ops::ShrAssign<u32> for Opc {
    fn shr_assign(&mut self, rhs: u32) {
        self.0 >>= rhs;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct LogicalImm {
    pub n: u32,
    pub s: u32,
    pub r: u32,
}

fn check_gp_type(op: &Operand, allowed: u32) -> bool {
    let typ = op.as_::<Reg>().typ() as u32;
    let mask = allowed << RegType::Gp32 as u32;
    bit_test(mask, typ)
}

fn check_gp_typex(op: &Operand, allowed: u32, x: &mut u32) -> bool {
    let typ = op.as_::<Reg>().typ() as u32;
    *x = (typ - RegType::Gp32 as u32) & allowed;
    bit_test(allowed << RegType::Gp32 as u32, typ)
}

fn check_gp_typex2(o0: &Operand, o1: &Operand, allowed: u32, x: &mut u32) -> bool {
    check_gp_typex(o0, allowed, x) && check_signature!(o0, o1)
}

fn check_gp_typex3(o0: &Operand, o1: &Operand, o2: &Operand, allowed: u32, x: &mut u32) -> bool {
    check_gp_typex(o0, allowed, x) && check_signature!(o0, o1, o2)
}

fn check_gp_id(op: &Operand, hi_id: u32) -> bool {
    op.id() < 31 || op.id() == hi_id
}

fn check_gp_id2(o0: &Operand, o1: &Operand, hi_id: u32) -> bool {
    let id0 = o0.id();
    let id1 = o1.id();
    (id0 < 31 || id0 == hi_id) && (id1 < 31 || id1 == hi_id)
}

fn check_gp_id3(o0: &Operand, o1: &Operand, o2: &Operand, hi_id: u32) -> bool {
    let id0 = o0.id();
    let id1 = o1.id();
    let id2 = o2.id();
    (id0 < 31 || id0 == hi_id) && (id1 < 31 || id1 == hi_id) && (id2 < 31 || id2 == hi_id)
}

fn check_vec_id(o0: &Operand) -> bool {
    let id = o0.id();
    id < 31
}

fn check_vec_id2(o0: &Operand, o1: &Operand) -> bool {
    let id0 = o0.id();
    let id1 = o1.id();
    id0 < 31 && id1 < 31
}

fn check_vec_id3(o0: &Operand, o1: &Operand, o2: &Operand) -> bool {
    let id0 = o0.id();
    let id1 = o1.id();
    let id2 = o2.id();
    id0 < 31 && id1 < 31 && id2 < 31
}

fn bit_test(value: u32, n: u32) -> bool {
    /*
        SUPPORT_INLINE constexpr bool bit_test(const T& value, const N& n) noexcept {
      return (as_std_uint(value) & (as_std_uint(T(1)) << as_std_uint(n))) != 0u;
    } */
    value & (1 << n) != 0
}

fn encode_mov_sequence64(out: &mut [u32; 4], mut imm: u64, rd: u32, x: u32) -> usize {
    const MOVZ: u32 = 0b11010010100000000000000000000000;
    const MOVN: u32 = 0b10010010100000000000000000000000;
    const MOVK: u32 = 0b11110010100000000000000000000000;

    if imm <= 0xFFFFFFFF {
        return encode_mov_sequence32(out, imm as u32, rd, x);
    }

    let zhw = count_zero_half_words_64(imm);
    let ohw = count_zero_half_words_64(!imm);

    if zhw >= ohw {
        let mut op = MOVZ;
        let mut count = 0;
        for hw_index in 0..4 {
            let hw_imm = (imm & 0xFFFF) as u32;
            if hw_imm == 0 {
                imm >>= 16;
                continue;
            }
            out[count] = op | (hw_index << 21) | (hw_imm << 5) | rd;
            op = MOVK;
            count += 1;

            imm >>= 16;
        }

        return count;
    }

    let mut op = MOVN;
    let mut count = 0;
    let mut neg_mask = 0xFFFF;

    for hw_index in 0..4 {
        let hw_imm = (imm & 0xFFFF) as u32;
        if hw_imm == 0xFFFF {
            imm >>= 16;
            continue;
        }

        out[count] = op | (hw_index << 21) | ((hw_imm ^ neg_mask) << 5) | rd;
        count += 1;
        op = MOVK;
        neg_mask = 0;
        imm >>= 16;
    }

    count
}

fn encode_mov_sequence32(out: &mut [u32], imm: u32, rd: u32, x: u32) -> usize {
    let movz = 0b11010010100000000000000000000000 | (x << 31);
    let movn = 0b10010010100000000000000000000000;
    let movk = 0b11110010100000000000000000000000;
    if (imm & 0xFFFF0000) == 0 {
        out[0] = movz | (0 << 21) | ((imm & 0xffff) << 5) | rd;
        return 1;
    }

    if (imm & 0xFFFF0000) == 0xFFFF0000 {
        out[0] = movn | (0 << 21) | ((!imm & 0xFFFF) << 5) | rd;
        return 1;
    }

    if (imm & 0x0000FFFF) == 0x00000000 {
        out[0] = movz | (1 << 21) | ((imm >> 16) << 5) | rd;
        return 1;
    }

    if (imm & 0x0000FFFF) == 0x0000FFFF {
        out[0] = movn | (1 << 21) | ((!imm >> 16) << 5) | rd;
        return 1;
    }

    out[0] = movz | (0 << 21) | ((imm & 0xFFFF) << 5) | rd;
    out[1] = movk | (1 << 21) | ((imm >> 16) << 5) | rd;
    return 2;
}

pub const fn count_zero_half_words_64(imm: u64) -> u32 {
    let mut count = 0;
    if (imm & 0x000000000000FFFF) == 0 {
        count += 1;
    }
    if (imm & 0x00000000FFFF0000) == 0 {
        count += 1;
    }
    if (imm & 0x0000FFFF00000000) == 0 {
        count += 1;
    }
    if (imm & 0xFFFF000000000000) == 0 {
        count += 1;
    }
    count
}

/// Encodes the given `imm` value of the given `width` to a logical immediate value represented as N, S, and R fields
/// and writes these fields to `out`.
///
/// Encoding Table:
///
/// ```text
/// +---+--------+--------+------+
/// | N |  ImmS  |  ImmR  | Size |
/// +---+--------+--------+------+
/// | 1 | ssssss | rrrrrr |  64  |
/// | 0 | 0sssss | .rrrrr |  32  |
/// | 0 | 10ssss | ..rrrr |  16  |
/// | 0 | 110sss | ...rrr |  8   |
/// | 0 | 1110ss | ....rr |  4   |
/// | 0 | 11110s | .....r |  2   |
/// +---+--------+--------+------+
/// ```
pub const fn encode_logical_imm(mut imm: u64, mut width: u32) -> Option<LogicalImm> {
    loop {
        width /= 2;
        let mask = (1u64 << width) - 1;
        if (imm & mask) != (imm >> width) & mask {
            width *= 2;
            break;
        }
        if width <= 2 {
            break;
        }
    }

    let lsb_mask = lsb_mask::<u64>(width);
    imm &= lsb_mask;

    // Patterns of all zeros and all ones are not encodable.
    if imm == 0 || lsb_mask == imm {
        return None;
    }

    // Inspect the pattern and get the most important bit indexes.
    //
    //         o_index <-+      +-> z_index
    //                  |      |
    // |..zeros..|o_count|z_count|..ones..|
    // |000000000|111111|000000|11111111|
    let z_index = (!imm).trailing_zeros();
    let z_imm = imm ^ ((1 << z_index) - 1);
    let z_count = if z_imm != 0 {
        z_imm.trailing_zeros()
    } else {
        width
    } - z_index;

    let o_index = z_index + z_count;
    let o_imm = !(z_imm ^ ((1 << o_index) - 1));
    let o_count = if o_imm != 0 {
        o_imm.trailing_zeros()
    } else {
        width
    } - o_index;

    let must_be_zero = o_imm ^ !((1 << (o_index + o_count)) - 1);
    if must_be_zero != 0 || (z_index > 0 && width - (o_index + o_count) != 0) {
        return None;
    }

    Some(LogicalImm {
        n: if width == 64 { 1 } else { 0 },
        s: (o_count + z_index - 1) | 0u32.wrapping_sub(width * 2) & 0x3f,
        r: width - o_index,
    })
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub(super) enum OffsetType {
    SignedOffset,
    Adr,
    Adrp,
    Ldr,
}

impl TryFrom<u8> for OffsetType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SignedOffset),
            1 => Ok(Self::Adr),
            2 => Ok(Self::Adrp),
            3 => Ok(Self::Ldr),
            _ => Err(()),
        }
    }
}

struct OffsetFormat {
    typ: OffsetType,
    flags: u8,
    region_size: u8,
    value_size: u8,
    value_offset: u8,
    imm_bit_count: u8,
    imm_bit_shift: u8,
    imm_discard_lsb: u8,
}

impl OffsetFormat {
    pub const fn new(
        typ: OffsetType,
        flags: u8,
        region_size: u8,
        value_size: u8,
        value_offset: u8,
        imm_bit_count: u8,
        imm_bit_shift: u8,
        imm_discard_lsb: u8,
    ) -> Self {
        Self {
            typ,
            flags,
            region_size,
            value_size,
            value_offset,
            imm_bit_count,
            imm_bit_shift,
            imm_discard_lsb,
        }
    }

    pub fn reset_to_imm_type(
        &mut self,
        typ: OffsetType,
        value_size: usize,
        imm_bit_shift: u32,
        imm_bit_count: u32,
        imm_discard_lsb: u32,
    ) {
        self.typ = typ;
        self.value_size = value_size as u8;
        self.region_size = value_size as u8;
        self.imm_bit_shift = imm_bit_shift as u8;
        self.imm_bit_count = imm_bit_count as u8;
        self.imm_discard_lsb = imm_discard_lsb as u8;
        self.flags = 0;
        self.value_offset = 0;
    }

    fn set_region(&mut self, region_size: usize, value_offset: usize) {
        self.region_size = region_size as u8;
        self.value_offset = value_offset as u8;
    }

    fn set_leading_and_trailing_size(&mut self, leading_size: usize, trailing_size: usize) {
        self.region_size = (leading_size + trailing_size + self.value_size as usize) as u8;
        self.value_offset = leading_size as u8;
    }

    fn typ(&self) -> OffsetType {
        self.typ
    }

    fn flags(&self) -> u8 {
        self.flags
    }

    fn region_size(&self) -> usize {
        self.region_size as usize
    }

    fn value_size(&self) -> usize {
        self.value_size as usize
    }

    fn value_offset(&self) -> usize {
        self.value_offset as usize
    }

    fn imm_bit_count(&self) -> usize {
        self.imm_bit_count as usize
    }

    fn imm_bit_shift(&self) -> usize {
        self.imm_bit_shift as usize
    }

    fn imm_discard_lsb(&self) -> usize {
        self.imm_discard_lsb as usize
    }
}

const fn lsb_mask<T>(n: u32) -> u64 {
    if size_of::<T>() < size_of::<u64>() {
        (1 << n) - 1
    } else {
        if n != 0 {
            (!0u64).wrapping_shr((size_of::<T>() as u32 * 8) - n)
        } else {
            0
        }
    }
}

const fn cond_code_to_opcode_field(cond: u32) -> u32 {
    (cond.wrapping_sub(2)) & 0xf
}

const SHIFT_OP_TO_LD_ST_OP_MAP: [u8; 16] = [
    3, 255, 255, 255, 255, 255, 255, 255, 2, 255, 255, 255, 6, 7, 255, 255,
];

const fn is_byte_mask_imm(imm: u64) -> bool {
    let mask = 0x0101010101010101 & u64::MAX;
    imm == (imm & mask) * 255
}

const fn encode_imm64_byte_mask_to_imm8(imm: u64) -> u32 {
    (((imm >> (7  - 0)) & 0b00000011) | // [.......G|H.......]
     ((imm >> (23 - 2)) & 0b00001100) | // [.......E|F.......]
     ((imm >> (39 - 4)) & 0b00110000) | // [.......C|D.......]
     ((imm >> (55 - 6)) & 0b11000000)) as u32
}

macro_rules! is_fp_imm8_generic {
    ($t: ty: $val: expr, $num_b_bits: expr, $num_cdefgh_bits: expr, $num_zero_bits: expr) => {{
        let all_bs_mask = lsb_mask::<u32>($num_b_bits);
        let b0_pattern = 1u32 << ($num_b_bits - 1);
        let b1_pattern = all_bs_mask as u32 ^ b0_pattern;

        let imm_z = $val & lsb_mask::<$t>($num_zero_bits as _) as $t;
        let imm_b = ($val >> ($num_zero_bits + $num_cdefgh_bits)) as u32 & all_bs_mask as u32;
        imm_z == 0 && (imm_b == b0_pattern || imm_b == b1_pattern)
    }};
}

pub const fn is_fp16_imm8(val: u32) -> bool {
    is_fp_imm8_generic!(u32: val, 3, 6, 6)
}

pub const fn is_fp32_imm8(val: u32) -> bool {
    is_fp_imm8_generic!(u32: val, 6, 6, 19)
}

pub const fn is_fp64_imm8(val: u64) -> bool {
    is_fp_imm8_generic!(u64: val, 9, 6, 48)
}

macro_rules! encode_fp_to_imm8_generic {
    ($t: ty: $val: expr, $num_b_bits: expr, $num_cdefgh_bits: expr, $num_zero_bits: expr) => {{
        let bits = ($val >> $num_zero_bits) as u32;
        ((bits >> ($num_b_bits + $num_cdefgh_bits - 7)) & 0x80) | (bits & 0x7f)
    }};
}

pub const fn encode_fp64_to_imm8(val: u64) -> u32 {
    encode_fp_to_imm8_generic!(u64: val, 9, 6, 48)
}

fn pick_fp_opcode(
    reg: Vec,
    s_op: u32,
    s_hf: u32,
    v_op: u32,
    v_hf: u32,
    sz_out: &mut u32,
) -> Option<Opc> {
    const QBIT_INDEX: usize = 30;

    struct EncodeFpOpcodeBits {
        size_mask: u32,
        mask: [u32; 3],
    }

    static SZ_BITS_TABLE: [EncodeFpOpcodeBits; 6] = [
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1),
            mask: [0, 0, 1 << 22],
        },
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1) | (1 << 0),
            mask: [0, 0, 0],
        },
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1) | (1 << 0),
            mask: [1 << 23 | 1 << 22, 0, 1 << 22],
        },
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1) | (1 << 0),
            mask: [(1 << 22) | (1 << 20) | (1 << 19), 0, 0],
        },
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1) | (1 << 0),
            mask: [1 << 22 | (1 << 21) | (1 << 15) | (1 << 14), 0, 1 << 22],
        },
        EncodeFpOpcodeBits {
            size_mask: (1 << 2) | (1 << 1) | (1 << 0),
            mask: [1 << 23, 0, 1 << 22],
        },
    ];

    let mut op = Opc(0);
    if reg.has_element_type() {
        let sz = reg.typ() as u32 - RegType::Vec16 as u32;
        if sz > 2 || !bit_test32(SZ_BITS_TABLE[s_hf as usize].size_mask, sz) {
            return None;
        }

        op.reset(SZ_BITS_TABLE[s_hf as usize].mask[sz as usize] ^ s_op);
        *sz_out = sz;

        return (s_op != 0).then_some(op);
    } else {
        let q = reg.typ() as u32 - RegType::Vec64 as u32;
        let sz = (reg.element_type() as u32).saturating_sub(VecElementType::H as u32);

        if q > 1 || sz > 2 || !bit_test32(SZ_BITS_TABLE[v_hf as usize].size_mask, sz) {
            return None;
        }

        op.reset(SZ_BITS_TABLE[v_hf as usize].mask[sz as usize] ^ (v_op | (q << QBIT_INDEX)));
        *sz_out = sz;
        return (v_op != 0).then_some(op);
    }
}

const fn bit_test32(value: u32, n: u32) -> bool {
    value & (1 << n) != 0
}

struct SizeOpTable {
    array: [SizeOp; ((RegType::Vec128 as usize - RegType::Vec8 as usize + 1) + 1) * 40],
}

impl SizeOpTable {
    const fn len() -> usize {
        ((RegType::Vec128 as usize - RegType::Vec8 as usize + 1) + 1) * 40
    }
    const fn bin() -> Self {
        let mut i = 0;
        let mut array = [SizeOp::new(SizeOp::K_INVALID); Self::len()];
        while i < Self::len() {
            array[i] = Self::bin_at(i);
            i += 1;
        }
        Self { array }
    }

    const fn any() -> Self {
        let mut i = 0;
        let mut array = [SizeOp::new(SizeOp::K_INVALID); Self::len()];
        while i < Self::len() {
            array[i] = Self::any_at(i);
            i += 1;
        }
        Self { array }
    }

    const fn bin_at(x: usize) -> SizeOp {
        if x == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
            | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K00)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K00_Q)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::B as usize)
        {
            SizeOp::new(SizeOp::K00)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::B as usize)
        {
            SizeOp::new(SizeOp::K00_Q)
        } else {
            SizeOp::new(SizeOp::K_INVALID)
        }
    }

    const fn any_at(x: usize) -> SizeOp {
        if x == (((RegType::Vec8 as usize - RegType::Vec8 as usize) << 3)
            | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K00_S)
        } else if x
            == (((RegType::Vec16 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K01_S)
        } else if x
            == (((RegType::Vec32 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K10_S)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::None as usize)
        {
            SizeOp::new(SizeOp::K11_S)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::B as usize)
        {
            SizeOp::new(SizeOp::K00)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::B as usize)
        {
            SizeOp::new(SizeOp::K00_Q)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::H as usize)
        {
            SizeOp::new(SizeOp::K01)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::H as usize)
        {
            SizeOp::new(SizeOp::K01_Q)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::S as usize)
        {
            SizeOp::new(SizeOp::K10)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::S as usize)
        {
            SizeOp::new(SizeOp::K10_Q)
        } else if x
            == (((RegType::Vec64 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::D as usize)
        {
            SizeOp::new(SizeOp::K11_S)
        } else if x
            == (((RegType::Vec128 as usize - RegType::Vec8 as usize) << 3)
                | VecElementType::D as usize)
        {
            SizeOp::new(SizeOp::K11_Q)
        } else {
            SizeOp::new(SizeOp::K_INVALID)
        }
    }
}

static SIZE_OP_TABLE: [SizeOpTable; 2] = [SizeOpTable::bin(), SizeOpTable::any()];

const TABLE_BIN: usize = 0;
const TABLE_ANY: usize = 1;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
struct SizeOp(u8);

impl SizeOp {
    pub const fn new(val: u8) -> Self {
        Self(val)
    }

    const K128_BIT_SHIFT: u8 = 0;
    const K_SCALAR_SHIFT: u8 = 1;
    const K_SIZE_SHIFT: u8 = 2;

    const K_Q: u8 = 1u8 << Self::K128_BIT_SHIFT;
    const K_S: u8 = 1u8 << Self::K_SCALAR_SHIFT;

    const K00: u8 = 0 << Self::K_SIZE_SHIFT;
    const K01: u8 = 1 << Self::K_SIZE_SHIFT;
    const K10: u8 = 2 << Self::K_SIZE_SHIFT;
    const K11: u8 = 3 << Self::K_SIZE_SHIFT;

    const K00_Q: u8 = Self::K00 | Self::K_Q;
    const K01_Q: u8 = Self::K01 | Self::K_Q;
    const K10_Q: u8 = Self::K10 | Self::K_Q;
    const K11_Q: u8 = Self::K11 | Self::K_Q;

    const K00_S: u8 = Self::K00 | Self::K_S;
    const K01_S: u8 = Self::K01 | Self::K_S;
    const K10_S: u8 = Self::K10 | Self::K_S;
    const K11_S: u8 = Self::K11 | Self::K_S;

    const K_INVALID: u8 = 0xFF;

    const K_SZ_Q: u8 = (0x3u8 << Self::K_SIZE_SHIFT) | Self::K_Q;
    const K_SZ_S: u8 = (0x3u8 << Self::K_SIZE_SHIFT) | Self::K_S;
    const K_SZ_QS: u8 = (0x3u8 << Self::K_SIZE_SHIFT) | Self::K_Q | Self::K_S;

    const fn is_valid(self) -> bool {
        self.0 != Self::K_INVALID
    }

    const fn make_invalid(&mut self) {
        self.0 = Self::K_INVALID;
    }

    const fn q(&self) -> u32 {
        (self.0 >> Self::K128_BIT_SHIFT) as u32 & 1
    }

    const fn qs(&self) -> u32 {
        (((self.0 >> Self::K128_BIT_SHIFT) as u32) | ((self.0 >> Self::K_SCALAR_SHIFT) as u32)) & 1
    }

    const fn scalar(&self) -> u32 {
        (self.0 >> Self::K_SCALAR_SHIFT) as u32 & 1
    }

    const fn size(&self) -> u32 {
        (self.0 >> Self::K_SIZE_SHIFT) as u32 & 0x3
    }

    const fn decrement_size(&mut self) {
        self.0 = (self.0 as u32 - (1u32 << Self::K_SIZE_SHIFT)) as u8;
    }
}

#[derive(Copy, Clone, Debug)]
struct SizeOpMap {
    table_id: u8,
    size_op_mask: u8,
    accept_mask: u16,
}

static SIZE_OP_MAP: [SizeOpMap; 23] = [
    // kVO_V_B
    SizeOpMap {
        table_id: TABLE_BIN as u8,
        size_op_mask: SizeOp::K_Q,
        accept_mask: (B!(SizeOp::K00) | B!(SizeOp::K00_Q)) as u16,
    },
    // kVO_V_BH
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00) | B!(SizeOp::K00_Q) | B!(SizeOp::K01) | B!(SizeOp::K01_Q))
            as u16,
    },
    // kVO_V_BH_4S
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_BHS
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_BHS_D2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)
            | B!(SizeOp::K11_Q)) as u16,
    },
    // kVO_V_HS
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K01) | B!(SizeOp::K01_Q) | B!(SizeOp::K10) | B!(SizeOp::K10_Q))
            as u16,
    },
    // kVO_V_S
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_Q,
        accept_mask: (B!(SizeOp::K10) | B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_B8H4
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00) | B!(SizeOp::K01)) as u16,
    },
    // kVO_V_B8H4S2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00) | B!(SizeOp::K01) | B!(SizeOp::K10)) as u16,
    },
    // kVO_V_B8D1
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_Q,
        accept_mask: (B!(SizeOp::K00) | B!(SizeOp::K11_S)) as u16,
    },
    // kVO_V_H4S2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K01) | B!(SizeOp::K10)) as u16,
    },
    // kVO_V_B16
    SizeOpMap {
        table_id: TABLE_BIN as u8,
        size_op_mask: SizeOp::K_Q,
        accept_mask: (B!(SizeOp::K00_Q)) as u16,
    },
    // kVO_V_B16H8
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00_Q) | B!(SizeOp::K01_Q)) as u16,
    },
    // kVO_V_B16H8S4
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00_Q) | B!(SizeOp::K01_Q) | B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_B16D2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00_Q) | B!(SizeOp::K11_Q)) as u16,
    },
    // kVO_V_H8S4
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K01_Q) | B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_S4
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: 0,
        accept_mask: (B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_D2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: 0,
        accept_mask: (B!(SizeOp::K11_Q)) as u16,
    },
    // kVO_SV_BHS
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K00_S)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K01_S)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)
            | B!(SizeOp::K10_S)) as u16,
    },
    // kVO_SV_B8H4S2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_S)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_S)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_S)) as u16,
    },
    // kVO_SV_HS
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K01_S)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)
            | B!(SizeOp::K10_S)) as u16,
    },
    // kVO_V_Any
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)
            | B!(SizeOp::K11_S)
            | B!(SizeOp::K11_Q)) as u16,
    },
    // kVO_SV_Any
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K00_S)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K01_S)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)
            | B!(SizeOp::K10_S)
            | B!(SizeOp::K11)
            | B!(SizeOp::K11_Q)
            | B!(SizeOp::K11_S)) as u16,
    },
];

const fn significant_simd_op<'a>(o0: &'a Operand, o1: &'a Operand, inst_flags: u32) -> &'a Operand {
    if (inst_flags & InstFlag::Long as u32) == 0 {
        o0
    } else {
        o1
    }
}

const fn element_type_to_size_op(
    vec_op_type: u32,
    reg_type: RegType,
    element_type: VecElementType,
) -> SizeOp {
    let map = &SIZE_OP_MAP[vec_op_type as usize];
    let table = &SIZE_OP_TABLE[map.table_id as usize];

    let a = reg_type as usize - RegType::Vec8 as usize;
    let b = RegType::Vec128 as usize - RegType::Vec8 as usize;

    let index = /*(reg_type as usize - RegType::Vec8 as usize)
        .min(RegType::Vec128 as usize - RegType::Vec8 as usize)
        << 3
        | (element_type as usize);*/
        if a < b {
            (a << 3) | (element_type as usize)
        } else {
            (b << 3) | (element_type as usize)
        };
    let op = table.array[index];
    let mut modified_op = SizeOp::new(op.0 & map.size_op_mask);

    if !bit_test32(map.accept_mask as u32, op.0 as u32) {
        modified_op.make_invalid();
    }

    modified_op
}

struct LMHImm {
    lm: u32,
    h: u32,
    max_rm_id: u32,
}

fn encode_lmh(size_field: u32, element_index: u32, out: &mut LMHImm) -> bool {
    if size_field != 1 && size_field != 2 {
        return false;
    }

    let h_shift = 3u32.saturating_sub(size_field);
    let lm_shift = size_field.saturating_sub(1u32);
    let max_element_index = 15u32 >> size_field;

    out.h = element_index >> h_shift;
    out.lm = (element_index << lm_shift) & 0x3u32;
    out.max_rm_id = (8u32 << size_field).saturating_sub(1);

    element_index <= max_element_index
}
