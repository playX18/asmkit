//! Top-level emit path: InstInfo lookup, signature validation, operand
//! analysis, and dispatch to the emit handlers (port of AsmJit's
//! `Assembler::_emit` from `a64assembler.cpp`).
//!
//! AsmJit's `goto EmitOp` / `goto EmitOp_DispImm` / `goto EmitOp_Rel` targets
//! become the [`Handler`] enum: the encoding arms (a `match` on [`Encoding`])
//! validate operands and compose the opcode in [`A64EmitState`], while the
//! actual word emission is done by the handlers in [`super::encoder`].
//!
//! Derived from AsmJit (Zlib license) — this file is an altered version; see LICENSE notices.

#![allow(clippy::eq_op, clippy::erasing_op, dead_code, unused)]
use crate::AsmError;
use crate::aarch64::encoder::*;
use crate::aarch64::encoder_tables::*;
use crate::aarch64::operands::*;
use crate::aarch64::{Assembler, Gp, Reg, ShiftOp, instdb::*};
use crate::core::arch_traits::Arch;
use crate::core::buffer::{Constant, LabelUse, Reloc, RelocDistance, RelocTarget};
use crate::core::globals::CondCode;
use crate::core::operand::*;
use crate::core::buffer::CodeBuffer;

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

/// Emit handler selected by an encoding arm (AsmJit's `goto EmitXxx` labels).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Handler {
    /// `EmitOp`: write the composed opcode word.
    Op,
    /// `EmitOp_DispImm`: pack the displacement immediate, then `EmitOp`.
    OpDispImm,
    /// `EmitOp_Rel`: resolve a label/immediate pc-relative operand, then
    /// `EmitOp_DispImm`. Falls through when `rm_rel` is not relative.
    OpRel,
    /// Multi-word emission (mov sequences in `multiple_op_data`).
    Multi,
}

impl<'a> Assembler<'a> {
    /// Port of AsmJit's `Assembler::_emit`: validates the instruction's
    /// operand signature, composes the opcode, and dispatches to the emit
    /// handlers. Errors are recorded in `last_error`.
    pub(crate) fn _emit(&mut self, id: u32, ops: &[&Operand]) {
        if id & !(InstId::REAL_ID | InstId::ARM_COND) != 0 {
            self.last_error = Some(AsmError::InvalidInstruction);
            return;
        }

        let inst_cc = InstId::extract_cc(id);
        let inst_id = InstId::extract_real_id(id);

        if inst_id == InstId::None as u32 || inst_id >= InstId::_Count as u32 {
            self.last_error = Some(AsmError::InvalidInstruction);
            return;
        }

        let inst_info = &INST_INFO_TABLE[inst_id as usize];
        let mut encoding_index = inst_info.encoding_data_index as usize;

        let mut st = A64EmitState::new();

        let isign4;
        let inst_flags;

        const NOREG: &Operand = &Operand::new();

        let op0 = *ops.get(0).unwrap_or(&NOREG);
        let op1 = *ops.get(1).unwrap_or(&NOREG);
        let op2 = *ops.get(2).unwrap_or(&NOREG);
        let op3 = *ops.get(3).unwrap_or(&NOREG);
        let op4 = *ops.get(4).unwrap_or(&NOREG);
        let op5 = *ops.get(5).unwrap_or(&NOREG);

        isign4 = op0.op_type() as u32
            + ((op1.op_type() as u32) << 3)
            + ((op2.op_type() as u32) << 6)
            + ((op3.op_type() as u32) << 9);
        inst_flags = inst_info.flags;

        macro_rules! check_features {
            () => {{
                let (required, context) =
                    required_features_for_form(inst_id as usize, st.opcode.get(), ops);
                if !self.environment().supports_aarch64_features(required) {
                    self.last_error = Some(AsmError::MissingCpuFeature { feature: context });
                    return;
                }
            }};
        }

        macro_rules! emit_op {
            () => {{
                check_features!();
                self.emit_handler(Handler::Op, &mut st);
                return;
            }};
        }

        macro_rules! emit_disp_imm {
            () => {{
                check_features!();
                self.emit_handler(Handler::OpDispImm, &mut st);
                return;
            }};
        }

        macro_rules! emit_rel {
            () => {{
                check_features!();
                if self.emit_handler(Handler::OpRel, &mut st) {
                    return;
                }
            }};
        }

        macro_rules! emit_rd0 {
            () => {
                st.opcode.add_reg(op0.id(), 0);
                emit_op!();
            };
        }

        macro_rules! emit_rn5 {
            () => {
                st.opcode.add_reg(op0.id(), 5);
                emit_op!();
            };
        }

        macro_rules! emit_rn5_rm16 {
            () => {
                st.opcode.add_reg(op0.id(), 5);
                st.opcode.add_reg(op1.id(), 16);
                emit_op!();
            };
        }

        macro_rules! emit_rd0_rn5 {
            () => {
                st.opcode.add_reg(op0.id(), 0);
                st.opcode.add_reg(op1.id(), 5);
                emit_op!();
            };
        }

        macro_rules! emit_rd0_rn5_rm16_ra10 {
            () => {
                st.opcode.add_reg(op0.id(), 0);
                st.opcode.add_reg(op1.id(), 5);
                st.opcode.add_reg(op2.id(), 16);
                st.opcode.add_reg(op3.id(), 10);
                emit_op!();
            };
        }

        macro_rules! emit_rd0_rn5_rm16 {
            () => {
                st.opcode.add_reg(op0.id(), 0);
                st.opcode.add_reg(op1.id(), 5);
                st.opcode.add_reg(op2.id(), 16);
                emit_op!();
            };
        }

        macro_rules! emit_mem_base_rn5 {
            () => {
                st.opcode.add_reg(op0.as_::<Mem>().base_id(), 5);
                emit_op!();
            };
        }

        macro_rules! emit_mem_base_index_rn5_rm16 {
            () => {
                st.opcode.add_reg(op0.as_::<Mem>().base_id(), 5);
                st.opcode.add_reg(op0.as_::<Mem>().index_id(), 16);
                emit_op!();
            };
        }

        macro_rules! emit_mem_base_no_imm_rn5 {
            () => {
                st.opcode.add_reg(st.rm_rel.as_::<Mem>().base_id(), 5);
                emit_op!();
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
                        st.opcode.reset(0b0100111000000000000111 << 10);
                        st.opcode.add_imm(imm5, 16);
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
                        st.opcode.reset(0b0110111000000000000001 << 10);
                        st.opcode.add_imm(imm5, 16);
                        st.opcode.add_imm(imm4, 11);
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

                    let q =
                        (op0.as_::<Reg>().reg_type() as u32).wrapping_sub(RegType::Vec64 as u32);

                    if op1.as_::<Reg>().is_gp() {
                        let element_type = op0.as_::<Vec>().element_type() as u32;
                        if q > 1 || !bit_test(k_valid_encodings, (q << 3) | element_type) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        let lsb_index = element_type - 1;
                        let imm5 = 1u32 << lsb_index;

                        st.opcode.reset(0b0000111000000000000011 << 10);
                        st.opcode.add_imm(q, 30);
                        st.opcode.add_imm(imm5, 16);
                        emit_rd0_rn5!();
                    } else if !op1.as_::<Reg>().is_vec() || !op1.as_::<Vec>().has_element_index() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    } else {
                        let dst_index = op1.as_::<Vec>().element_index();
                        if !op0.as_::<Vec>().has_element_type() {
                            let lsb_index = (op0.as_::<Reg>().reg_type() as u32)
                                .wrapping_sub(RegType::Vec8 as u32);
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

                            st.opcode.reset(0b0101111000000000000001 << 10);
                            st.opcode.add_imm(imm5, 16);
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

                            st.opcode.reset(0b0000111000000000000001 << 10);
                            st.opcode.add_imm(q, 30);
                            st.opcode.add_imm(imm5, 16);
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
                    st.opcode.reset((op_data.opcode as u32) << 10);
                    st.opcode.add_imm(x, 30);
                    st.opcode.add_imm(imm5, 16);
                    emit_rd0_rn5!();
                    return;
                }
            };
        }

        match encoding {
            Encoding::BaseOp => {
                let op_data = &BASE_OP[encoding_index];
                if isign4 == 0 {
                    st.opcode.reset(op_data.opcode);
                    emit_op!();
                }
            }

            Encoding::BaseOpX16 => {
                let op_data = &BASE_OP_X16[encoding_index];
                if isign4 == enc_ops!(Reg) && op0.as_::<Reg>().is_gp64() && op0.id() == 16 {
                    st.opcode.reset(op_data.opcode);
                    emit_op!();
                }
            }

            Encoding::BaseOpImm => {
                let op_data = &BASE_OP_IMM[encoding_index];
                if isign4 == enc_ops!(Imm) {
                    let imm = op0.as_::<Imm>().value() as u64;
                    let imm_max = 1u64 << op_data.imm_bits as u32;
                    if imm >= imm_max {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }
                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(imm as u32, op_data.imm_offset as _);
                    emit_op!();
                }
            }

            Encoding::BaseR => {
                let op_data = &BASE_R[encoding_index];
                if isign4 == enc_ops!(Reg) {
                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_reg(op0.id(), op_data.r_shift);
                    emit_op!();
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
                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op1.id(), op_data.b_shift);
                    st.opcode.add_reg(op0.id(), op_data.a_shift);
                    emit_op!();
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
                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op2.id(), 16);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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
                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op2.id(), 16);
                    st.opcode.add_reg(op3.id(), 10);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    let imm2 = op2.as_::<Imm>().value() as u64;
                    let imm3 = op3.as_::<Imm>().value() as u64;

                    if imm2 >= 1u64 << (op_data.a_imm_size + op_data.a_imm_discard_lsb)
                        || imm3 >= 1u64 << (op_data.b_imm_size + op_data.b_imm_discard_lsb)
                    {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let a_imm = imm2 as u32 >> op_data.a_imm_discard_lsb;
                    let b_imm = imm3 as u32 >> op_data.b_imm_discard_lsb;

                    if (a_imm << op_data.a_imm_discard_lsb) != imm2 as u32
                        || (b_imm << op_data.b_imm_discard_lsb) != imm3 as u32
                    {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(a_imm, op_data.a_imm_offset);
                    st.opcode.add_imm(b_imm, op_data.b_imm_offset);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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
                        st.opcode.reset(0b00010001000000000000000000000000);
                        st.opcode.add_imm(x, 31);
                        st.opcode.add_reg(op1.id(), 5);
                        st.opcode.add_reg(op0.id(), 0);
                        emit_op!();
                    } else {
                        if !check_gp_id2(op0, op1, 63) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        st.opcode.reset(0b00101010000000000000001111100000);
                        st.opcode.add_imm(x, 31);
                        st.opcode.add_reg(op1.id(), 16);
                        st.opcode.add_reg(op0.id(), 0);
                        emit_op!();
                    }
                }

                if isign4 == enc_ops!(Reg, Imm) {
                    if !op0.as_::<Reg>().is_gp() {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let mut imm_value = op1.as_::<Imm>().value() as u64;
                    if x == 0 {
                        imm_value &= 0xFFFFFFFF;
                    }

                    // Prefer a single MOVN/MOVZ instruction over a logical instruction.
                    st.multiple_op_count = encode_mov_sequence64(
                        &mut st.multiple_op_data,
                        imm_value,
                        op0.id() & 31,
                        x,
                    );
                    if st.multiple_op_count == 1 && !op0.as_::<Gp>().is_sp() {
                        st.opcode.reset(st.multiple_op_data[0]);
                        emit_op!();
                    }

                    if !op0.as_::<Gp>().is_zr() {
                        if let Some(logical_imm) =
                            encode_logical_imm(imm_value, if x != 0 { 64 } else { 32 })
                        {
                            st.opcode.reset(0b00110010000000000000001111100000);
                            st.opcode.add_imm(x, 31);
                            st.opcode.add_logical_imm(&logical_imm);
                            st.opcode.add_reg(op0.id(), 0);
                            emit_op!();
                        }
                    }

                    check_features!();
                    self.emit_handler(Handler::Multi, &mut st);
                    return;
                }
            }

            Encoding::BaseMovKNZ => {
                let op_data = &BASE_MOV_KNZ[encoding_index];

                let x = (op0.as_::<Reg>().typ() as u32).wrapping_sub(RegType::Gp32 as u32);
                if x > 1 {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                if !check_gp_id(op0, 63) {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                st.opcode.reset(op_data.opcode);
                st.opcode.add_imm(x, 31);

                if isign4 == enc_ops!(Reg, Imm) {
                    let imm16 = op1.as_::<Imm>().value() as u64;
                    if imm16 > 0xFFFF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    st.opcode.add_imm(imm16 as u32, 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
                }

                if isign4 == enc_ops!(Reg, Imm, Imm) {
                    let imm16 = op1.as_::<Imm>().value() as u64;
                    let shift_type = op2.as_::<Imm>().predicate();
                    let shift_value = op2.as_::<Imm>().value() as u64;

                    if imm16 > 0xFFFF || shift_value > 48 || shift_type != ShiftOp::LSL as u32 {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let hw = (shift_value as u32) >> 4;

                    if hw << 4 != shift_value as u32 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    st.opcode.add_imm(hw, 21);
                    st.opcode.add_imm(imm16 as u32, 5);
                    st.opcode.add_reg(op0.id(), 0);

                    if x == 0 && hw > 1 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    emit_op!();
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

                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_reg(op0.id(), 0);
                    st.offset_format.reset_to_imm_type(
                        OffsetType::try_from(op_data.offset_type).expect("Invalid offset type"),
                        4,
                        5,
                        21,
                        0,
                    );

                    if inst_id == InstId::Adrp as u32 {
                        st.offset_format.imm_discard_lsb = 12;
                    }
                    st.rm_rel = *op1;
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
                    st.opcode.reset((op_data.immediate_op as u32) << 24);

                    // ADD | SUB (immediate) - ZR is not allowed.
                    // ADDS|SUBS (immediate) - ZR allowed in Rd, SP allowed in Rn.
                    let a_hi_id = if st.opcode.get() & 1 << 29 != 0 {
                        63
                    } else {
                        31
                    };
                    let b_hi_id = 31;
                    if !check_gp_id(op0, a_hi_id) || !check_gp_id(op1, b_hi_id) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let mut imm = op2.as_::<Imm>().value() as u64;
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

                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm(shift, 12);
                    st.opcode.add_imm(imm as u32, 10);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                            st.opcode.reset((op_data.shifted_op as u32) << 21);
                            st.opcode.add_imm(x, 31);
                            st.opcode.add_imm(shift_type, 22);
                            st.opcode.add_reg(op2.id(), 16);
                            st.opcode.add_imm(shift, 10);
                            st.opcode.add_reg(op1.id(), 5);
                            st.opcode.add_reg(op0.id(), 0);
                            emit_op!();
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

                    st.opcode.reset((op_data.extended_op as u32) << 21);
                    shift_type = shift_type.wrapping_sub(ShiftOp::UXTB as u32);

                    if shift_type > 7 || shift > 4 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if (st.opcode.get() & (1 << 29)) == 0 {
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

                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op2.id(), 16);
                    st.opcode.add_imm(shift_type, 13);
                    st.opcode.add_imm(shift, 10);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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
                    st.opcode.reset((op_data.immediate_op as u32) << 23);

                    let imm_mask = lsb_mask::<u64>(op_size);
                    let mut imm_value = op2.as_::<Imm>().value() as u64;

                    if op_data.negate_imm != 0 {
                        imm_value ^= imm_mask;
                    }

                    let Some(logical_imm) = encode_logical_imm(imm_value, op_size) else {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    };

                    let op_ands = 0x3 << 29;
                    let is_ands = (st.opcode.get() & op_ands) == op_ands;

                    if !check_gp_id(op0, if is_ands { 63 } else { 31 }) || !check_gp_id(op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    st.opcode.add_imm(x, 31);
                    st.opcode.add_logical_imm(&logical_imm);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    st.opcode.reset((op_data.shifted_op as u32) << 21);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op2.id(), 16);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    st.opcode.reset((op_data.shifted_op as u32) << 21);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm(shift_type, 22);
                    st.opcode.add_reg(op2.id(), 16);
                    st.opcode.add_imm(op_shift, 10);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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
                    let mut imm_value = imm12.value() as u64;

                    if imm_value > 0xfff {
                        if (imm_value & !(0xfff << 12)) != 0 {
                            self.last_error = Some(AsmError::TooLarge);
                            return;
                        }
                        imm_shift = 1;
                        imm_value >>= 12;
                    }

                    st.opcode.reset((op_data.immediate_op as u32) << 24);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm(imm_shift, 22);
                    st.opcode.add_imm(imm_value as u32, 10);
                    st.opcode.add_reg(op0.id(), 5);
                    st.opcode.add_reg(63, 0);
                    emit_op!();
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

                            st.opcode.reset((op_data.shifted_op as u32) << 21);
                            st.opcode.add_imm(x, 31);
                            st.opcode.add_imm(shift_type, 22);
                            st.opcode.add_reg(op1.id(), 5);
                            st.opcode.add_imm(shift_value, 10);
                            st.opcode.add_reg(op0.id(), 5);
                            st.opcode.add_reg(63, 0);
                            emit_op!();
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

                    shift_type = shift_type.wrapping_sub(ShiftOp::UXTB as u32);

                    if shift_type > 7 || shift_value > 4 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    st.opcode.reset((op_data.extended_op as u32) << 21);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op1.id(), 16);
                    st.opcode.add_imm(shift_type, 13);
                    st.opcode.add_imm(shift_value, 10);
                    st.opcode.add_reg(op0.id(), 5);
                    st.opcode.add_reg(63, 0);
                    emit_op!();
                }
            }

            Encoding::BaseMvnNeg => {
                let op_data = &BASE_MVN_NEG[encoding_index];

                let mut x = 0;
                if !check_gp_typex2(op0, op1, 3, &mut x) {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                st.opcode.reset(op_data.opcode);
                st.opcode.add_imm(x, 31);
                st.opcode.add_reg(op1.id(), 16);
                st.opcode.add_reg(op0.id(), 0);

                if isign4 == enc_ops!(Reg, Reg) {
                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    emit_op!();
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

                    st.opcode.add_imm(shift_type, 22);
                    st.opcode.add_imm(shift_value, 10);
                    emit_op!();
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
                    let imm_value = op1.as_::<Imm>().value() as u64;

                    let Some(logical_imm) = encode_logical_imm(imm_value & imm_mask, op_size)
                    else {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    };

                    st.opcode.reset((op_data.immediate_op as u32) << 22);
                    st.opcode.add_logical_imm(&logical_imm);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op0.id(), 5);
                    st.opcode.add_reg(63, 0);
                    emit_op!();
                }

                st.opcode.reset((op_data.shifted_op as u32) << 21);
                st.opcode.add_imm(x, 31);
                st.opcode.add_reg(op1.id(), 16);
                st.opcode.add_reg(op0.id(), 5);
                st.opcode.add_reg(63, 0);

                if isign4 == enc_ops!(Reg, Reg) {
                    if !check_gp_id2(op0, op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    emit_op!();
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

                    st.opcode.add_imm(shift_type, 22);
                    st.opcode.add_imm(shift_value, 10);
                    emit_op!();
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

                    let lsb = op1.as_::<Imm>().value() as u64;
                    let width = op2.as_::<Imm>().value() as u64;
                    let op_size = if x != 0 { 64 } else { 32 };

                    if lsb >= op_size || width == 0 || width > op_size {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let lsb32 = 0u32.wrapping_sub(lsb as u32) & (op_size as u32 - 1);
                    let width32 = width as u32 - 1;

                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm(x, 22);
                    st.opcode.add_imm(lsb32, 16);
                    st.opcode.add_imm(width32, 10);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    let lsb = op2.as_::<Imm>().value() as u64;
                    let width = op3.as_::<Imm>().value() as u64;
                    let op_size = if x != 0 { 64 } else { 32 };

                    if lsb >= op_size as u64 || width == 0 || width > op_size as u64 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let imm_l = 0u32.wrapping_sub(lsb as u32) & (op_size as u32 - 1);
                    let imm_w = width as u32 - 1;

                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm(x, 22);
                    st.opcode.add_imm(imm_l, 16);
                    st.opcode.add_imm(imm_w, 10);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    let imm_r = op2.as_::<Imm>().value() as u64;
                    let imm_s = op3.as_::<Imm>().value() as u64;
                    let op_size = if x != 0 { 64 } else { 32 };

                    if (imm_r | imm_s) >= op_size as u64 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm(x, 22);
                    st.opcode.add_imm(imm_r as u32, 16);
                    st.opcode.add_imm(imm_s as u32, 10);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    let lsb = op2.as_::<Imm>().value() as u64;
                    let width = op3.as_::<Imm>().value() as u64;
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

                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm(x, 22);
                    st.opcode.add_imm(lsb32, 16);
                    st.opcode.add_imm(width32, 10);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm(x, 22);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    let lsb = op3.as_::<Imm>().value() as u64;
                    let op_size = if x != 0 { 64 } else { 32 };

                    if lsb >= op_size as u64 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm(x, 22);
                    st.opcode.add_reg(op2.id(), 16);
                    st.opcode.add_imm(lsb as u32, 10);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    st.opcode.reset(0b01011010110000000000100000000000);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm(x, 10);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    st.opcode.reset(op_data.register_op);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op2.id(), 16);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    let imm_r = op2.as_::<Imm>().value() as u64;
                    let op_size = if x != 0 { 64 } else { 32 };

                    if imm_r >= op_size as u64 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    st.opcode.reset(op_data.immediate_op);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm(x, 22);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);

                    if st.opcode.get() & (1 << 10) != 0 {
                        st.opcode.add_imm(x, 15);
                        st.opcode.add_imm(imm_r as u32, 16);
                        emit_op!();
                    }

                    if op_data.ror == 0 {
                        let ubfm_imm_r = (0u32).wrapping_sub(imm_r as u32) & (op_size as u32 - 1);
                        let ubfm_imm_s = op_size as u32 - 1 - imm_r as u32;

                        st.opcode.add_imm(ubfm_imm_r, 16);
                        st.opcode.add_imm(ubfm_imm_s, 10);
                        emit_op!();
                    } else {
                        st.opcode.add_imm(imm_r as u32, 10);
                        st.opcode.add_reg(op1.id(), 16);
                        emit_op!();
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

                    let nzcv = op2.as_::<Imm>().value() as u64;
                    let cond = op3.as_::<Imm>().value() as u64;

                    if (nzcv | cond) > 0xF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(x, 31);
                    st.opcode
                        .add_imm(cond_code_to_opcode_field(cond as u32), 12);
                    st.opcode.add_imm(nzcv as u32, 0);

                    if isign4 == enc_ops!(Reg, Reg, Imm, Imm) {
                        if !check_signature!(op0, op1) {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if !check_gp_id(op1, 31) {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        st.opcode.add_reg(op1.id(), 16);
                        st.opcode.add_reg(op0.id(), 5);
                        emit_op!();
                    } else {
                        let imm5 = op1.as_::<Imm>().value() as u64;
                        if imm5 > 0x1F {
                            self.last_error = Some(AsmError::TooLarge);
                            return;
                        }

                        st.opcode.add_imm(1, 11);
                        st.opcode.add_imm(imm5 as u32, 16);
                        st.opcode.add_reg(op0.id(), 5);
                        emit_op!();
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

                    let cond = op2.as_::<Imm>().value() as u64;
                    if cond.wrapping_sub(2) > 0xE {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op1.id(), 16);
                    st.opcode
                        .add_imm(cond_code_to_opcode_field((cond as u32) ^ 1), 12);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    let cond = op3.as_::<Imm>().value() as u64;
                    if cond > 0xF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op2.id(), 16);
                    st.opcode
                        .add_imm(cond_code_to_opcode_field(cond as u32), 12);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    let cond = op1.as_::<Imm>().value() as u64;
                    if cond.wrapping_sub(2) >= 0xE {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(x, 31);
                    st.opcode
                        .add_imm(cond_code_to_opcode_field((cond as u32) ^ 1), 12);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    st.opcode.reset(op_data.register_op);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op2.id(), 16);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
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

                    let imm = op2.as_::<Imm>().value() as u64;

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

                    st.opcode.reset(op_data.immediate_op);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_imm((imm & 0xFF) as u32, 10);
                    st.opcode.add_reg(op1.id(), 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
                }
            }

            Encoding::BaseAtDcIcTlbi => {
                let op_data = &BASE_AT_DC_IC_TLBI[encoding_index];

                if isign4 == enc_ops!(Imm) || isign4 == enc_ops!(Imm, Reg) {
                    if op_data.mandatory_reg != 0 && isign4 != enc_ops!(Imm, Reg) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if op0.as_::<Imm>().value() as u64 > 0x7FFF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let imm = op0.as_::<Imm>().value() as u32;
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

                    st.opcode.reset(0b11010101000010000000000000000000);
                    st.opcode.add_imm(imm, 5);
                    st.opcode.add_reg(rt, 0);
                    emit_op!();
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

                    if op1.as_::<Imm>().value() as u64 > 0xFFFF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let imm = op1.as_::<Imm>().value() as u32;
                    if (imm & (1 << 15)) == 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    st.opcode.reset(0b11010101001100000000000000000000);
                    st.opcode.add_imm(imm, 5);
                    st.opcode.add_reg(op0.id(), 0);
                    emit_op!();
                }
            }

            Encoding::BaseMsr => {
                if isign4 == enc_ops!(Imm, Reg) {
                    if !op1.as_::<Reg>().is_gp64() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if op0.as_::<Imm>().value() as u64 > 0xFFFF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let imm = op0.as_::<Imm>().value() as u32;
                    if (imm & (1 << 15)) == 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    if !check_gp_id(op1, 63) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    st.opcode.reset(0b11010101000100000000000000000000);
                    st.opcode.add_imm(imm, 5);
                    st.opcode.add_reg(op1.id(), 0);
                    emit_op!();
                }

                if isign4 == enc_ops!(Imm, Imm) {
                    if op0.as_::<Imm>().value() as u64 > 0x1F {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    if op1.as_::<Imm>().value() as u64 > 0xF {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let op = op0.as_::<Imm>().value() as u32;
                    let crm = op1.as_::<Imm>().value() as u32;

                    let op1_val = op >> 3;
                    let op2_val = op & 0x7;

                    st.opcode.reset(0b11010101000000000100000000011111);
                    st.opcode.add_imm(op1_val, 16);
                    st.opcode.add_imm(crm, 8);
                    st.opcode.add_imm(op2_val, 5);
                    emit_op!();
                }
            }

            Encoding::BaseSys => {
                if isign4 == enc_ops!(Imm, Imm, Imm, Imm) {
                    if op0.as_::<Imm>().value() as u64 > 0x7
                        || op1.as_::<Imm>().value() as u64 > 0xF
                        || op2.as_::<Imm>().value() as u64 > 0xF
                        || op3.as_::<Imm>().value() as u64 > 0x7
                    {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let op1_val = op0.as_::<Imm>().value() as u32;
                    let crn = op1.as_::<Imm>().value() as u32;
                    let crm = op2.as_::<Imm>().value() as u32;
                    let op2_val = op3.as_::<Imm>().value() as u32;
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

                    st.opcode.reset(0b11010101000010000000000000000000);
                    st.opcode.add_imm(op1_val, 16);
                    st.opcode.add_imm(crn, 12);
                    st.opcode.add_imm(crm, 8);
                    st.opcode.add_imm(op2_val, 5);
                    st.opcode.add_reg(rt, 0);
                    emit_op!();
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

                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_reg(op0.id(), 5);
                    emit_op!();
                }
            }

            Encoding::BaseBranchRel => {
                let op_data = &BASE_BRANCH_REL[encoding_index];
                if isign4 == enc_ops!(Label) || isign4 == enc_ops!(Imm) {
                    st.opcode.reset(op_data.opcode);
                    st.rm_rel = *op0;

                    if inst_cc as u32 != 0 || (st.opcode.0 & (1 << 30)) != 0 {
                        if st.opcode.has_x() {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        st.opcode.0 |= 1 << 30;
                        st.opcode
                            .add_imm(cond_code_to_opcode_field(inst_cc as u32), 0);
                        st.offset_format
                            .reset_to_imm_type(OffsetType::SignedOffset, 4, 5, 19, 2);
                        st.rm_rel = *op0;
                        emit_rel!();
                    }

                    st.offset_format
                        .reset_to_imm_type(OffsetType::SignedOffset, 4, 0, 26, 2);
                    st.rm_rel = *op0;
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

                    st.opcode.reset(op_data.opcode);
                    st.opcode.add_imm(x, 31);
                    st.opcode.add_reg(op0.id(), 0);
                    st.offset_format
                        .reset_to_imm_type(OffsetType::SignedOffset, 4, 5, 19, 2);

                    st.rm_rel = *op1;
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

                    let mut imm = op1.as_::<Imm>().value() as u64;

                    st.opcode.reset(op_data.opcode);
                    if imm >= 32 {
                        if x == 0 {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        st.opcode.add_imm(x, 31);
                        imm &= 0x1F;
                    }

                    st.opcode.add_reg(op0.id(), 0);
                    st.opcode.add_imm(imm as u32, 19);
                    st.offset_format
                        .reset_to_imm_type(OffsetType::SignedOffset, 4, 5, 14, 2);

                    st.rm_rel = *op2;
                    emit_rel!();
                }
            }

            Encoding::BasePrfm => {
                let op_data = &BASE_PRFM[encoding_index];
                if isign4 == enc_ops!(Imm, Mem) {
                    let m = op1.as_::<Mem>();
                    st.rm_rel = *op1;

                    let imm_shift = 3u32;

                    if op0.as_::<Imm>().value() as u64 > 0x1F {
                        self.last_error = Some(AsmError::TooLarge);
                        return;
                    }

                    let offset = m.offset();
                    let prfop = op0.as_::<Imm>().value() as u32;

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

                            st.opcode.reset((op_data.register_op as u32) << 21);
                            st.opcode.add_imm(opt as u32, 13);
                            st.opcode.add_imm(s, 12);
                            st.opcode.0 |= 1 << 11;
                            st.opcode.add_imm(prfop, 0);
                            st.opcode.add_reg(m.base_id(), 5);
                            st.opcode.add_reg(m.index_id(), 16);
                            emit_op!();
                        }

                        let offset32 = offset as i32;
                        let imm12 = (offset32 as u32) >> imm_shift;

                        if imm12 < (1 << 12) && ((imm12 << imm_shift) as i32) == offset32 {
                            st.opcode.reset((op_data.s_offset_op as u32) << 22);
                            st.opcode.add_imm(imm12, 10);
                            st.opcode.add_imm(prfop, 0);
                            st.opcode.add_reg(m.base_id(), 5);
                            emit_op!();
                        }

                        if offset32 >= -256 && offset32 < 256 {
                            st.opcode.reset((op_data.u_offset_op as u32) << 21);
                            st.opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                            st.opcode.add_imm(prfop, 0);
                            st.opcode.add_reg(m.base_id(), 5);
                            emit_op!();
                        }

                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    } else {
                        st.opcode.reset((op_data.literal_op as u32) << 24);
                        st.opcode.add_imm(prfop, 0);
                        st.offset_format
                            .reset_to_imm_type(OffsetType::SignedOffset, 4, 5, 19, 2);
                        st.rm_rel = *op1;
                        emit_rel!();
                    }
                }
            }

            Encoding::BaseLdSt => {
                let op_data = &BASE_LD_ST[encoding_index];
                if isign4 == enc_ops!(Reg, Mem) {
                    let m = op1.as_::<Mem>();
                    st.rm_rel = *op1;

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

                            st.opcode.reset((op_data.register_op as u32) << 21);
                            st.opcode.xor_imm(x, op_data.x_offset as u32);
                            st.opcode.add_imm(opt as u32, 13);
                            st.opcode.add_imm(s, 12);
                            st.opcode.0 |= 1 << 11;
                            st.opcode.add_reg(op0.id(), 0);
                            st.opcode.add_reg(m.base_id(), 5);
                            st.opcode.add_reg(m.index_id(), 16);
                            emit_op!();
                        }

                        let offset32 = offset as i32;
                        let imm12 = (offset32 as u32) >> imm_shift;

                        if imm12 < (1 << 12) && ((imm12 << imm_shift) as i32) == offset32 {
                            st.opcode.reset((op_data.u_offset_op as u32) << 22);
                            st.opcode.xor_imm(x, op_data.x_offset as u32);
                            st.opcode.add_imm(imm12, 10);
                            st.opcode.add_reg(op0.id(), 0);
                            st.opcode.add_reg(m.base_id(), 5);
                            emit_op!();
                        }

                        if offset32 >= -256 && offset32 < 256 {
                            st.opcode.reset((op_data.u_offset_op as u32) << 22);
                            st.opcode.xor_imm(x, op_data.x_offset as u32);
                            st.opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                            st.opcode.add_reg(op0.id(), 0);
                            st.opcode.add_reg(m.base_id(), 5);
                            emit_op!();
                        }

                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    } else {
                        if op_data.literal_op == 0 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        st.opcode.reset((op_data.literal_op as u32) << 24);
                        st.opcode.xor_imm(x, op_data.x_offset);
                        st.opcode.add_reg(op0.id(), 0);
                        st.offset_format
                            .reset_to_imm_type(OffsetType::Ldr, 4, 5, 19, 2);
                        emit_rel!();
                    }
                }
            }

            Encoding::BaseLdpStp => {
                let op_data = &BASE_LDP_STP[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Mem) {
                    let m = op2.as_::<Mem>();
                    st.rm_rel = *op2;

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

                        st.opcode.reset((op_data.pre_post_op as u32) << 22);
                        st.opcode.add_imm(m.is_pre_index() as u32, 24);
                    } else {
                        st.opcode.reset((op_data.offset_op as u32) << 22);
                    }
                    st.opcode.add_imm(x, op_data.x_offset as u32);
                    st.opcode.add_imm((offset32 as u32) & 0x7F, 15);
                    st.opcode.add_reg(op1.id(), 10);
                    st.opcode.add_reg(op0.id(), 0);
                    st.opcode.add_reg(m.base_id(), 5);
                    emit_op!();
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
                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(x, op_data.x_offset as _);
                    st.opcode.add_reg(op0.id(), 16);
                    st.opcode.add_reg(op1.id(), 0);
                    st.rm_rel = *op2;
                    st.opcode.add_reg(m.base_id(), 5);
                    emit_op!();
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
                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(x, op_data.x_offset as _);
                    st.opcode.add_reg(op1.id(), 10);
                    st.opcode.add_reg(op0.id(), 0);
                    st.rm_rel = *op2;
                    st.opcode.add_reg(m.base_id(), 5);
                    emit_op!();
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
                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(x, op_data.x_offset as _);
                    st.opcode.add_reg(op0.id(), 16);
                    st.opcode.add_reg(op2.id(), 10);
                    st.opcode.add_reg(op1.id(), 0);
                    st.rm_rel = *op3;
                    st.opcode.add_reg(m.base_id(), 5);
                    emit_op!();
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
                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(x, op_data.x_offset as _);
                    st.opcode.add_reg(op0.id(), 0);
                    st.rm_rel = *op1;

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
                            st.opcode.reset(op_data.offset_op());
                        } else {
                            st.opcode.reset(op_data.pre_post_op());
                            st.opcode.xor_imm(m.is_pre_index() as u32, 11);
                        }
                        st.opcode.xor_imm(x, op_data.x_offset as u32);
                        st.opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                        st.opcode.add_reg(op0.id(), 0);
                        st.opcode.add_reg(m.base_id(), 5);
                        emit_op!();
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
                        st.opcode.reset(op_data.opcode());
                        st.opcode.xor_imm(m.is_pre_index() as u32, 11);
                        st.opcode.xor_imm(x, op_data.x_offset as u32);
                        st.opcode.add_imm(offset32 >> 9, 22);
                        st.opcode.add_imm(offset32, 12);
                        st.opcode.add_reg(op0.id(), 0);
                        st.opcode.add_reg(m.base_id(), 5);
                        emit_op!();
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
                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(x, op_data.x_offset as _);
                    st.opcode.add_reg(op0.id(), 16);
                    st.opcode.add_reg(op1.id(), 0);
                    st.rm_rel = *op2;
                    st.opcode.add_reg(m.base_id(), 5);
                    emit_op!();
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
                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(x, op_data.x_offset as _);
                    st.opcode.add_reg(op0.id(), 16);
                    st.opcode.add_reg(31, 0);
                    st.rm_rel = *op1;
                    st.opcode.add_reg(m.base_id(), 5);
                    emit_op!();
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
                        st.opcode.reset(op_data.opcode());
                        st.opcode.add_imm(x, op_data.x_offset as _);
                        st.opcode.add_reg(op0.id(), 16);
                        st.opcode.add_reg(op2.id(), 0);
                        st.rm_rel = *op4;
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

                    st.opcode.reset((op_data.opcode as u32) << 10);
                    if sz == 0 {
                        st.opcode.0 ^= 1 << 29;
                    }
                    st.opcode.add_imm(q, 30);
                    st.opcode.add_reg(op0.id(), 0);
                    st.opcode.add_reg(op1.id(), 5);
                    emit_op!();
                }
            }

            Encoding::FSimdVV => {
                let op_data = &F_SIMD_VV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg) {
                    if !match_signature2(op0, op1, inst_flags as u32) {
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
                        st.opcode.reset(fp_opcode.0);
                        emit_rd0_rn5!();
                    }

                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }
            }

            Encoding::FSimdVVV => {
                let op_data = &F_SIMD_VVV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if !match_signature3(op0, op1, op2, inst_flags as u32) {
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
                        st.opcode.reset(fp_opcode.0);
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
                        if !match_signature3(op0, op1, op2, inst_flags as u32) {
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
                            st.opcode.reset(fp_opcode.0);

                            emit_rd0_rn5_rm16!();
                        }

                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    } else {
                        if !match_signature2(op0, op1, inst_flags as u32) {
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
                            st.opcode.reset(fp_opcode.0);
                            st.opcode.add_imm(q, 30);
                            st.opcode.add_imm(hlm & 3, 20);
                            st.opcode.add_imm(hlm >> 2, 11);
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
                    if !match_signature4(op0, op1, op2, op3, inst_flags as u32) {
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
                        st.opcode.reset(fp_opcode.0);
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

                    let q = (op0.as_::<Reg>().is_vec128() as u32).wrapping_sub(1);
                    if q > 1 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let mut sz = op0.as_::<Vec>().element_type() as u32;
                    sz = sz.wrapping_sub(1);
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

                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(q, 30);
                    st.opcode.add_imm(sz, 22);
                    st.opcode.add_imm(rot, 12);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::SimdFccmpFccmpe => {
                let op_data = &SIMD_FCCMP_FCCMPE[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Imm, Imm) {
                    let sz = (op0.as_::<Reg>().typ() as u32).wrapping_sub(RegType::Vec16 as u32);
                    if sz > 2 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    if !check_signature!(op0, op1) || op0.as_::<Vec>().has_element_type() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let nzcv = op2.as_::<Imm>().value() as u64;
                    let cond = op3.as_::<Imm>().value() as u64;

                    if (nzcv | cond) > 0xF {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    let type_field = sz.wrapping_sub(1) & 0x3;

                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(type_field, 22);
                    st.opcode
                        .add_imm(cond_code_to_opcode_field(cond as u32), 12);
                    st.opcode.add_imm(nzcv as u32, 0);
                    emit_rn5_rm16!();
                }
            }

            Encoding::SimdFcm => {
                let op_data = &SIMD_FCM[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) && op_data.has_register_op() {
                    if !match_signature3(op0, op1, op2, inst_flags as u32) {
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
                        st.opcode.reset(fp_opcode.0);
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
                        st.opcode.reset(fp_opcode.0);
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

                    let q = (op0.as_::<Reg>().is_vec128() as u32).wrapping_sub(1);
                    if q > 1 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let mut sz = op0.as_::<Vec>().element_type() as u32;
                    sz = sz.wrapping_sub(1);
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

                        st.opcode.reset(op_data.regular_op());
                        st.opcode.add_imm(q, 30);
                        st.opcode.add_imm(sz, 22);
                        st.opcode.add_imm(rot, 11);
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

                        st.opcode.reset(op_data.element_op());
                        st.opcode.add_imm(q, 30);
                        st.opcode.add_imm(sz, 22);
                        st.opcode.add_imm(hl & 1u32, 21);
                        st.opcode.add_imm(hl >> 1, 11);
                        st.opcode.add_imm(rot, 13);
                        emit_rd0_rn5_rm16!();
                    }
                }
            }

            Encoding::SimdFcmpFcmpe => {
                let op_data = &SIMD_FCMP_FCMPE[encoding_index];

                let sz = (op0.as_::<Reg>().typ() as u32).wrapping_sub(RegType::Vec16 as u32);
                let type_field = sz.wrapping_sub(1) & 0x3u32;

                if sz > 2 {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                if op0.as_::<Vec>().has_element_type() {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                st.opcode.reset(op_data.opcode());
                st.opcode.add_imm(type_field, 22);

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

                    st.opcode.0 |= 0x8;
                    emit_rd0_rn5!();
                }
            }

            Encoding::SimdFcsel => {
                if isign4 == enc_ops!(Reg, Reg, Reg, Imm) {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let sz = (op0.as_::<Reg>().typ() as u32).wrapping_sub(RegType::Vec16 as u32);
                    let type_field = sz.wrapping_sub(1) & 0x3u32;

                    if sz > 2 || op0.as_::<Vec>().has_element_type() {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let cond = op3.as_::<Imm>().value() as u32;
                    if cond > 0xFu32 {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }

                    st.opcode.reset(0b00011110001000000000110000000000u32);
                    st.opcode.add_imm(type_field, 22);
                    st.opcode.add_imm(cond, 12);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::SimdFcvt => {
                if isign4 == enc_ops!(Reg, Reg) {
                    let dst_sz =
                        (op0.as_::<Reg>().reg_type() as u32).wrapping_sub(RegType::Vec16 as u32);
                    let src_sz =
                        (op1.as_::<Reg>().reg_type() as u32).wrapping_sub(RegType::Vec16 as u32);

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

                    st.opcode.reset(0b0001111000100010010000 << 10);
                    st.opcode.add_imm((type_opc as u32) >> 4, 22);
                    st.opcode.add_imm((type_opc as u32) & 15, 15);
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

                        st.opcode.reset(op_data.scalar_op());
                        st.opcode.0 |= 0x400000; // sz bit must be 1
                        emit_rd0_rn5!();
                        return;
                    }

                    st.opcode.reset(op_data.vector_op());

                    let is_long = (inst_flags & InstFlag::Long as u16) != 0;
                    let (rl, rn) = if is_long {
                        (op0.as_::<Vec>(), op1.as_::<Vec>())
                    } else {
                        (op1.as_::<Vec>(), op0.as_::<Vec>())
                    };

                    let q = (rn.reg_type() as u32).wrapping_sub(RegType::Vec64 as u32);
                    if (st.opcode.has_q() as u32) != q {
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
                        st.opcode.0 |= 0x400000;
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
                            .wrapping_sub(RegType::Vec16 as u32);

                        if type_field > 2u32 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        let type_val = (type_field - 1u32) & 0x3;
                        st.opcode.reset(op_data.general_op());
                        st.opcode.add_imm(type_val, 22);
                        st.opcode.add_imm(x, 31);
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
                            st.opcode.reset(fp_opcode.0);
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
                            .wrapping_sub(RegType::Vec16 as u32);

                        let scale_limit = 32u32 << x;
                        if scale_val > scale_limit {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }

                        let type_val = (type_field - 1u32) & 0x3;
                        st.opcode.reset(op_data.general_op() ^ 0x200000);
                        st.opcode.add_imm(type_val, 22);
                        st.opcode.add_imm(x, 31);
                        st.opcode.add_imm(64u32 - scale_val, 10);
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
                            st.opcode.reset(fp_opcode.0);
                            st.opcode.add_imm(imm, 16);
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
                        (op0.as_::<Reg>().reg_type() as u32).wrapping_sub(RegType::Vec64 as u32);
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

                        st.opcode.reset(op_data.vector_op());
                        st.opcode.add_imm(q, 30);
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

                        st.opcode.reset(op_data.element_op());
                        st.opcode.add_imm(q, 30);
                        st.opcode.add_imm(element_index & 3u32, 20);
                        st.opcode.add_imm(element_index >> 2, 11);
                        emit_rd0_rn5_rm16!();
                    }
                }
            }

            Encoding::SimdFmov => {
                if isign4 == enc_ops!(Reg, Reg) {
                    // FMOV Gp <-> Vec st.opcode:
                    st.opcode.reset(0b00011110001001100000000000000000);

                    if (op0.as_::<Reg>().is_gp() && op1.as_::<Reg>().is_vec()) {
                        // FMOV Wd, Hn      (sf=0 type=11 rmode=00 op=110)
                        // FMOV Xd, Hn      (sf=1 type=11 rmode=00 op=110)
                        // FMOV Wd, Sn      (sf=0 type=00 rmode=00 op=110)
                        // FMOV Xd, Dn      (sf=1 type=11 rmode=00 op=110)
                        // FMOV Xd, Vn.d[1] (sf=1 type=10 rmode=01 op=110)
                        let x = op0.as_::<Reg>().is_gp64();
                        let sz = (op1.as_::<Reg>().reg_type() as u32)
                            .wrapping_sub(RegType::Vec16 as u32);

                        let mut typ = sz.wrapping_sub(1) & 0x3;
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

                        st.opcode.add_imm(x as u32, 31);
                        st.opcode.add_imm(typ, 22);
                        st.opcode.add_imm(r_mode_op, 16);

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
                            .wrapping_sub(RegType::Vec16 as u32);

                        let mut typ = sz.wrapping_sub(1) & 0x3;
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

                        st.opcode.add_imm(x as u32, 31);
                        st.opcode.add_imm(typ, 22);
                        st.opcode.add_imm(r_mode_op, 16);
                        emit_rd0_rn5!();
                    }

                    if check_signature!(op0, op1) {
                        let sz = (op0.as_::<Reg>().reg_type() as u32)
                            .wrapping_sub(RegType::Vec16 as u32);
                        if sz > 2 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        if op0.as_::<Vec>().has_element_type() {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }

                        let typ = sz.wrapping_sub(1) & 0x3;
                        st.opcode.reset(0b00011110001000000100000000000000);
                        st.opcode.add_imm(typ, 22);
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
                                .wrapping_sub(RegType::Vec16 as u32);
                            let typ = sz.wrapping_sub(1) & 0x3;
                            if sz > 2 {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            st.opcode.reset(0b00011110001000000001000000000000);
                            st.opcode.add_imm(typ, 22);
                            st.opcode.add_imm(imm8, 13);
                            emit_rd0!();
                        } else {
                            let q = (op0.as_::<Reg>().reg_type() as u32)
                                .wrapping_sub(RegType::Vec64 as u32);
                            let sz = (op0.as_::<Vec>().element_type() as u32)
                                .wrapping_sub(VecElementType::H as u32);

                            if q > 1 || sz > 2 {
                                self.last_error = Some(AsmError::InvalidInstruction);
                                return;
                            }

                            const SZ_BITS_TABLE: [u32; 3] = [1 << 11, 0, 1 << 29];
                            st.opcode.reset(0b00001111000000001111010000000000);
                            st.opcode ^= SZ_BITS_TABLE[sz as usize];
                            st.opcode.add_imm(q, 30);
                            st.opcode.add_imm(imm8 >> 5, 16);
                            st.opcode.add_imm(imm8 & 31, 5);
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
                        (op0.as_::<Reg>().reg_type() as u32).wrapping_sub(RegType::Vec16 as u32);
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

                    st.opcode.reset(op_data.scalar_op());
                    st.opcode ^= SZ_BITS_TABLE[sz as usize];
                    emit_rd0_rn5!();
                }

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if !check_signature!(op0, op1, op2) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    let q =
                        (op0.as_::<Reg>().reg_type() as u32).wrapping_sub(RegType::Vec64 as u32);
                    if q > 1 {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }

                    const SZ_BITS_TABLE: [u32; 3] =
                        [(1 << 22) | (1 << 21) | (1 << 15) | (1 << 14), 0, 1 << 22];
                    st.opcode.reset(op_data.scalar_op());
                    st.opcode ^= SZ_BITS_TABLE[q as usize];
                    st.opcode.add_imm(q, 30);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::ISimdSV => {
                let op_data = &I_SIMD_SV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg) {
                    let l = (inst_flags & InstFlag::Long as u16) != 0;
                    if (op0.as_::<Vec>().reg_type() as u32).wrapping_sub(RegType::Vec8 as u32)
                        != (op1.as_::<Vec>().element_type() as u32)
                            .wrapping_sub(VecElementType::B as u32)
                            .wrapping_add(l as u32)
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

                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(size_op.q(), 30);
                    st.opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5!();
                }
            }

            Encoding::ISimdVV => {
                let op_data = &I_SIMD_VV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg) {
                    let sop = significant_simd_op(op0, op1, inst_flags as u32);
                    if !match_signature2(op0, op1, inst_flags as u32) {
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

                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(size_op.qs(), 30);
                    st.opcode.add_imm(size_op.scalar(), 28);
                    st.opcode.add_imm(size_op.size(), 22);
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
                    st.opcode.reset(op_data.opcode());
                    emit_rd0_rn5!();
                }
            }

            Encoding::ISimdVVV => {
                let op_data = &I_SIMD_VVV[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    let sop = significant_simd_op(op0, op1, inst_flags as u32);
                    if !match_signature3(op0, op1, op2, inst_flags as u32) {
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

                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(size_op.qs(), 30);
                    st.opcode.add_imm(size_op.scalar(), 28);
                    st.opcode.add_imm(size_op.size(), 22);
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

                    st.opcode.reset(op_data.opcode());
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
                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(size_op.qs(), 30);
                    st.opcode.add_imm(size_op.scalar(), 28);
                    st.opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::ISimdVVVe => {
                let op_data = &I_SIMD_VVVE[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    let sop = significant_simd_op(op0, op1, inst_flags as u32);
                    if !match_signature2(op0, op1, inst_flags as u32) {
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
                        st.opcode.reset((op_data.regular_op as u32) << 10);
                        st.opcode.add_imm(size_op.qs(), 30);
                        st.opcode.add_imm(size_op.scalar(), 28);
                        st.opcode.add_imm(size_op.size(), 22);
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
                        st.opcode.reset((op_data.element_op as u32) << 10);
                        st.opcode.add_imm(size_op.q(), 30);
                        st.opcode.add_imm(size_op.size(), 22);
                        st.opcode.add_imm(lmh.lm, 20);
                        st.opcode.add_imm(lmh.h, 11);
                        emit_rd0_rn5_rm16!();
                    }
                }
            }

            Encoding::ISimdVVVI => {
                let op_data = &I_SIMD_VVVI[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg, Imm) {
                    let sop = significant_simd_op(op0, op1, inst_flags as u32);
                    if !match_signature3(op0, op1, op2, inst_flags as u32) {
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
                    let imm_value = op3.as_::<Imm>().value() as u64;
                    let mut imm_size = op_data.imm_size;
                    if op_data.imm64_has_one_bit_less != 0 && size_op.q() == 0 {
                        imm_size -= 1;
                    }
                    let imm_max = 1u64 << imm_size;
                    if imm_value >= imm_max {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }
                    st.opcode.reset(op_data.opcode());
                    st.opcode.add_imm(size_op.qs(), 30);
                    st.opcode.add_imm(size_op.scalar(), 28);
                    st.opcode.add_imm(size_op.size(), 22);
                    st.opcode.add_imm(imm_value as u32, op_data.imm_shift);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::ISimdVVVV => {
                let op_data = &I_SIMD_VVVV[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg, Reg) {
                    let sop = significant_simd_op(op0, op1, inst_flags as u32);
                    if !match_signature4(op0, op1, op2, op3, inst_flags as u32) {
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
                    st.opcode.reset((op_data.opcode as u32) << 10);
                    st.opcode.add_imm(size_op.qs(), 30);
                    st.opcode.add_imm(size_op.scalar(), 28);
                    st.opcode.add_imm(size_op.size(), 22);
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
                    st.opcode.reset((op_data.opcode as u32) << 10);
                    emit_rd0_rn5_rm16_ra10!();
                }
            }

            Encoding::ISimdPair => {
                let op_data = &I_SIMD_PAIR[encoding_index];
                if isign4 == enc_ops!(Reg, Reg) && op_data.opcode2 != 0 {
                    if op0.as_::<Vec>().is_vec_d1() && op1.as_::<Vec>().is_vec_d2() {
                        st.opcode.reset((op_data.opcode2 as u32) << 10);
                        st.opcode.add_imm(0x3, 22);
                        emit_rd0_rn5!();
                    }
                }
                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if !match_signature3(op0, op1, op2, inst_flags as u32) {
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
                    st.opcode.reset((op_data.opcode3 as u32) << 10);
                    st.opcode.add_imm(size_op.qs(), 30);
                    st.opcode.add_imm(size_op.scalar(), 28);
                    st.opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5_rm16!();
                }
            }

            Encoding::SimdBicOrr => {
                let op_data = &SIMD_BIC_ORR[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    if !match_signature3(op0, op1, op2, inst_flags as u32) {
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
                    st.opcode.reset((op_data.register_op as u32) << 10);
                    st.opcode.add_imm(size_op.q(), 30);
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
                    if op1.as_::<Imm>().value() as u64 > 0xFFFFFFFF {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }
                    let mut imm = op1.as_::<Imm>().value() as u32;
                    let mut shift = 0u32;
                    let max_shift = (8u32 << size_op.size()) - 8u32;
                    if isign4 == enc_ops!(Reg, Imm, Imm) {
                        if op2.as_::<Imm>().predicate() != ShiftOp::LSL as u32 {
                            self.last_error = Some(AsmError::InvalidImmediate);
                            return;
                        }
                        if imm > 0xFF || op2.as_::<Imm>().value() as u64 > max_shift as u64 {
                            self.last_error = Some(AsmError::InvalidImmediate);
                            return;
                        }
                        shift = op2.as_::<Imm>().value() as u32;
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
                    st.opcode.reset((op_data.immediate_op as u32) << 10);
                    st.opcode.add_imm(size_op.q(), 30);
                    st.opcode.add_imm(abc, 16);
                    st.opcode.add_imm(cmode, 12);
                    st.opcode.add_imm(defgh, 5);
                    emit_rd0!();
                }
            }

            Encoding::SimdCmp => {
                let op_data = &SIMD_CMP[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) && op_data.register_op != 0 {
                    if !match_signature3(op0, op1, op2, inst_flags as u32) {
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

                    st.opcode.reset((op_data.register_op as u32) << 10);
                    st.opcode.add_imm(size_op.qs(), 30);
                    st.opcode.add_imm(size_op.scalar(), 28);
                    st.opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5_rm16!();
                }

                if isign4 == enc_ops!(Reg, Reg, Imm) && op_data.zero_op != 0 {
                    if !match_signature2(op0, op1, inst_flags as u32) {
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

                    st.opcode.reset((op_data.zero_op as u32) << 10);
                    st.opcode.add_imm(size_op.qs(), 30);
                    st.opcode.add_imm(size_op.scalar(), 28);
                    st.opcode.add_imm(size_op.size(), 22);
                    emit_rd0_rn5!();
                }
            }

            Encoding::SimdDot => {
                let op_data = &SIMD_DOT[encoding_index];

                if isign4 == enc_ops!(Reg, Reg, Reg) {
                    let q =
                        (op0.as_::<Reg>().reg_type() as u32).wrapping_sub(RegType::Vec64 as u32);
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

                        st.opcode.reset((op_data.vector_op as u32) << 10);
                        st.opcode.add_imm(q, 30);
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

                        st.opcode.reset((op_data.element_op as u32) << 10);
                        st.opcode.add_imm(q, 30);
                        st.opcode.add_imm(lmh.lm, 20);
                        st.opcode.add_imm(lmh.h, 11);
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
                        let q = (op0.as_::<Reg>().reg_type() as u32)
                            .wrapping_sub(RegType::Vec64 as u32);
                        if q > 1 {
                            self.last_error = Some(AsmError::InvalidInstruction);
                            return;
                        }
                        st.opcode.reset(0b0000111010100000000111 << 10);
                        st.opcode.add_imm(q, 30);
                        st.opcode.add_reg(op1.id(), 16);
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
                    let mut imm64 = op1.as_::<Imm>().value() as u64;
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
                            if imm8 > 0xFF || op2.as_::<Imm>().value() as u64 > max_shift as u64 {
                                self.last_error = Some(AsmError::InvalidImmediate);
                                return;
                            }
                            shift = op2.as_::<Imm>().value() as u32;
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
                    st.opcode.reset((op_data.opcode as u32) << 10);
                    st.opcode.add_imm(size_op.q(), 30);
                    st.opcode.add_imm(op, 29);
                    st.opcode.add_imm(abc, 16);
                    st.opcode.add_imm(cmode, 12);
                    st.opcode.add_imm(defgh, 5);
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
                    if !match_signature2(op0, op1, inst_flags as u32) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    if op2.as_::<Imm>().value() as u64 > 63 {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }
                    let lsb_shift = size_op.size() + 3;
                    let lsb_mask = (1u32 << lsb_shift) - 1;
                    let mut imm = op2.as_::<Imm>().value() as u32;
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
                    st.opcode.reset((op_data.immediate_op as u32) << 10);
                    st.opcode.add_imm(size_op.qs(), 30);
                    st.opcode.add_imm(size_op.scalar(), 28);
                    st.opcode.add_imm(imm, 16);
                    emit_rd0_rn5!();
                    return;
                }
                if isign4 == enc_ops!(Reg, Reg, Reg) && op_data.register_op != 0 {
                    if !match_signature3(op0, op1, op2, inst_flags as u32) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    st.opcode.reset((op_data.register_op as u32) << 10);
                    st.opcode.add_imm(size_op.qs(), 30);
                    st.opcode.add_imm(size_op.scalar(), 28);
                    st.opcode.add_imm(size_op.size(), 22);
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
                    if !match_signature2(op0, op1, inst_flags as u32) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    let shift = op2.as_::<Imm>().value() as u64;
                    let shift_op = op2.as_::<Imm>().predicate();
                    if shift != (8u64 << size_op.size()) || shift_op != ShiftOp::LSL as u32 {
                        self.last_error = Some(AsmError::InvalidImmediate);
                        return;
                    }
                    st.opcode.reset((op_data.opcode as u32) << 10);
                    st.opcode.add_imm(size_op.q(), 30);
                    st.opcode.add_imm(size_op.size(), 22);
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
                        st.opcode.reset((op_data.opcode as u32) << 10);
                        st.opcode.add_imm(imm2, 12);
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
                    if !match_signature2(op0, op1, inst_flags as u32) {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    st.opcode.reset((op_data.opcode as u32) << 10);
                    st.opcode.add_imm(size_op.q(), 30);
                    st.opcode.add_imm(1u32, size_op.size() + 19);
                    emit_rd0_rn5!();
                    return;
                }
            }

            Encoding::SimdTblTbx => {
                let op_data = &SIMD_TBL_TBX[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Reg) || isign4 == enc_ops!(Reg, Reg, Reg, Reg) {
                    st.opcode.reset((op_data.opcode as u32) << 10);

                    let q =
                        (op0.as_::<Reg>().reg_type() as u32).wrapping_sub(RegType::Vec64 as u32);
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
                    st.opcode.add_imm(q, 30);
                    st.opcode.add_imm(len, 13);

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
                            st.opcode.add_reg(op2.id(), 16);
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
                            st.opcode.add_reg(op3.id(), 16);
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
                            st.opcode.add_reg(op4.id(), 16);
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
                            st.opcode.add_reg(op5.id(), 16);
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
                    st.rm_rel = *op1;

                    let xsz =
                        (op0.as_::<Reg>().reg_type() as u32).wrapping_sub(RegType::Vec8 as u32);
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
                            st.opcode.reset((op_data.register_op as u32) << 21);
                            st.opcode.add_imm(xsz & 3, 30);
                            st.opcode.add_imm(xsz >> 2, 23);
                            st.opcode.add_imm(opt as u32, 13);
                            st.opcode.add_imm(s, 12);
                            st.opcode.0 |= 1 << 11;
                            st.opcode.add_reg(op0.id(), 0);
                            st.opcode.add_reg(m.base_id(), 5);
                            st.opcode.add_reg(m.index_id(), 16);
                            emit_op!();
                        }

                        let offset32 = offset as i32;
                        if m.is_pre_or_post() {
                            if offset32 < -256 || offset32 > 255 {
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }
                            st.opcode.reset((op_data.pre_post_op as u32) << 21);
                            st.opcode.add_imm(xsz & 3, 30);
                            st.opcode.add_imm(xsz >> 2, 23);
                            st.opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                            st.opcode.add_imm(m.is_pre_index() as u32, 11);
                            st.opcode.0 |= 1 << 10;
                            st.opcode.add_reg(op0.id(), 0);
                            st.opcode.add_reg(m.base_id(), 5);
                            emit_op!();
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
                                    st.opcode.reset((op_data_ldur.opcode as u32) << 10);
                                    st.opcode.add_imm(xsz & 3, 30);
                                    st.opcode.add_imm(xsz >> 2, 23);
                                    st.opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                                    st.opcode.add_reg(op0.id(), 0);
                                    st.opcode.add_reg(m.base_id(), 5);
                                    emit_op!();
                                }
                                self.last_error = Some(AsmError::InvalidOperand);
                                return;
                            }
                            st.opcode.reset((op_data.u_offset_op as u32) << 22);
                            st.opcode.add_imm(xsz & 3, 30);
                            st.opcode.add_imm(xsz >> 2, 23);
                            st.opcode.add_imm(imm12, 10);
                            st.opcode.add_reg(op0.id(), 0);
                            st.opcode.add_reg(m.base_id(), 5);
                            emit_op!();
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
                        st.opcode.reset((op_data.literal_op as u32) << 24);
                        st.opcode.add_imm(opc, 30);
                        st.opcode.add_reg(op0.id(), 0);
                        st.offset_format
                            .reset_to_imm_type(OffsetType::SignedOffset, 4, 5, 19, 2);
                        st.rm_rel = *op1;
                        emit_rel!();
                    }
                }
            }

            Encoding::SimdLdpStp => {
                let op_data = &SIMD_LDP_STP[encoding_index];
                if isign4 == enc_ops!(Reg, Reg, Mem) {
                    let m = op2.as_::<Mem>();
                    st.rm_rel = *op2;

                    let opc =
                        (op0.as_::<Reg>().reg_type() as u32).wrapping_sub(RegType::Vec32 as u32);
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
                        st.opcode.reset((op_data.pre_post_op as u32) << 22);
                        st.opcode.add_imm(m.is_pre_index() as u32, 24);
                    } else {
                        st.opcode.reset((op_data.offset_op as u32) << 22);
                    }
                    st.opcode.add_imm(opc, 30);
                    st.opcode.add_imm((offset32 as u32) & 0x7F, 15);
                    st.opcode.add_reg(op1.id(), 10);
                    st.opcode.add_reg(op0.id(), 0);
                    st.opcode.add_reg(m.base_id(), 5);
                    emit_op!();
                }
            }

            Encoding::SimdLdurStur => {
                let op_data = &SIMD_LDUR_STUR[encoding_index];
                if isign4 == enc_ops!(Reg, Mem) {
                    let m = op1.as_::<Mem>();
                    st.rm_rel = *op1;

                    let sz =
                        (op0.as_::<Reg>().reg_type() as u32).wrapping_sub(RegType::Vec8 as u32);
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
                        st.opcode.reset((op_data.opcode as u32) << 10);
                        st.opcode.add_imm(sz & 3, 30);
                        st.opcode.add_imm(sz >> 2, 23);
                        st.opcode.add_imm((offset32 as u32) & 0x1FF, 12);
                        st.opcode.add_reg(op0.id(), 0);
                        st.opcode.add_reg(m.base_id(), 5);
                        emit_op!();
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
                    st.rm_rel = *op1;
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
                    st.rm_rel = *op2;
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
                    st.rm_rel = *op3;
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
                    st.rm_rel = *o4;
                } else {
                    self.last_error = Some(AsmError::InvalidInstruction);
                    return;
                }

                let v = op0.as_::<Vec>();
                let m = st.rm_rel.as_::<Mem>();

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
                    st.opcode.reset((op_data.single_op as u32) << 10);
                    offset_possibility = (1u32 << sz) * n;
                } else if v.has_element_index() {
                    if n != op_data.n {
                        self.last_error = Some(AsmError::InvalidInstruction);
                        return;
                    }
                    const OPC_S_SIZE_BY_SZ_TABLE: [u32; 4] =
                        [0u32 << 3, 2u32 << 3, 4u32 << 3, (4u32 << 3) | 1u32];
                    st.opcode.reset((op_data.single_op as u32) << 10);
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
                    st.opcode.reset((op_data.multiple_op as u32) << 10);
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
                    st.opcode.0 |= 1 << 23;
                } else {
                    if m.has_offset() {
                        if m.offset() != offset_possibility as i64 || !m.is_post_index() {
                            self.last_error = Some(AsmError::InvalidOperand);
                            return;
                        }
                        rm = 31;
                        st.opcode.0 |= 1 << 23;
                    }
                }

                st.opcode.add_imm(q, 30);
                st.opcode.add_imm(rm, 16);
                st.opcode.add_imm(opc_s_size, 10);
                st.opcode.add_imm(rn, 5);

                st.opcode.add_reg(op0.id(), 0);
                emit_op!();
            }

            Encoding::None | Encoding::Count => (),
        }

        self.last_error = Some(AsmError::UnsupportedInstruction {
            reason: "Unsupported instruction encoding or operand types",
        });
    }
}

// @generated AArch64 target features begin
/// AArch64 architectural features present in the pinned AsmJit ISA metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum CpuFeature {
    Aes,
    Asimd,
    Bf16,
    Bti,
    Chk,
    Clrbhb,
    Crc32,
    Cssc,
    Dgh,
    Dotprod,
    Fcma,
    Fhm,
    Flagm,
    Flagm2,
    Fp16,
    Fp8,
    Frintts,
    I8Mm,
    Jscvt,
    Lor,
    Lse,
    Mte,
    Mte2,
    Pauth,
    Ras,
    Rdm,
    Sha1,
    Sha256,
    Sha3,
    Sha512,
    Sm3,
    Sm4,
}

pub const CPU_FEATURE_COUNT: usize = 32;
pub const CPU_FEATURE_NAMES: [&str; CPU_FEATURE_COUNT] = [
    "AES", "ASIMD", "BF16", "BTI", "CHK", "CLRBHB", "CRC32", "CSSC", "DGH", "DOTPROD", "FCMA",
    "FHM", "FLAGM", "FLAGM2", "FP16", "FP8", "FRINTTS", "I8MM", "JSCVT", "LOR", "LSE", "MTE",
    "MTE2", "PAUTH", "RAS", "RDM", "SHA1", "SHA256", "SHA3", "SHA512", "SM3", "SM4",
];

pub const ALL_CPU_FEATURES: [CpuFeature; CPU_FEATURE_COUNT] = [
    CpuFeature::Aes,
    CpuFeature::Asimd,
    CpuFeature::Bf16,
    CpuFeature::Bti,
    CpuFeature::Chk,
    CpuFeature::Clrbhb,
    CpuFeature::Crc32,
    CpuFeature::Cssc,
    CpuFeature::Dgh,
    CpuFeature::Dotprod,
    CpuFeature::Fcma,
    CpuFeature::Fhm,
    CpuFeature::Flagm,
    CpuFeature::Flagm2,
    CpuFeature::Fp16,
    CpuFeature::Fp8,
    CpuFeature::Frintts,
    CpuFeature::I8Mm,
    CpuFeature::Jscvt,
    CpuFeature::Lor,
    CpuFeature::Lse,
    CpuFeature::Mte,
    CpuFeature::Mte2,
    CpuFeature::Pauth,
    CpuFeature::Ras,
    CpuFeature::Rdm,
    CpuFeature::Sha1,
    CpuFeature::Sha256,
    CpuFeature::Sha3,
    CpuFeature::Sha512,
    CpuFeature::Sm3,
    CpuFeature::Sm4,
];

impl CpuFeature {
    pub const fn name(self) -> &'static str {
        CPU_FEATURE_NAMES[self as usize]
    }
}

const FEATURE_FORM_SIGNATURE_MASK: u32 = OperandSignature::OP_TYPE_MASK
    | OperandSignature::REG_TYPE_MASK
    | Vec::SIGNATURE_REG_ELEMENT_TYPE_MASK
    | Vec::SIGNATURE_REG_ELEMENT_FLAG_MASK;

const fn feature_reg_signature(
    reg_type: RegType,
    element_type: VecElementType,
    element_access: bool,
) -> u32 {
    OperandType::Reg as u32
        | (reg_type as u32) << OperandSignature::REG_TYPE_SHIFT
        | (element_type as u32) << Vec::SIGNATURE_REG_ELEMENT_TYPE_SHIFT
        | (element_access as u32) << Vec::SIGNATURE_REG_ELEMENT_FLAG_SHIFT
}

struct InstFeatureForm {
    opcode_mask: u32,
    opcode_value: u32,
    operand_signatures: [u32; 6],
    required: u64,
    context: &'static str,
}

impl InstFeatureForm {
    fn matches(&self, opcode: u32, ops: &[&Operand]) -> bool {
        if opcode & self.opcode_mask != self.opcode_value {
            return false;
        }
        self.operand_signatures
            .iter()
            .enumerate()
            .all(|(index, expected)| {
                let actual = ops.get(index).map_or(0, |op| op.signature.bits());
                actual & FEATURE_FORM_SIGNATURE_MASK == *expected
            })
    }
}

/// Conservative required-feature masks, indexed by `InstId as usize`.
pub static INST_FEATURE_MASKS: [u64; InstId::_Count as usize] = [
    0x0000000000000000, // None
    0x0000000000000080, // Abs
    0x0000000000000000, // Adc
    0x0000000000000000, // Adcs
    0x0000000000000000, // Add
    0x0000000000200000, // Addg
    0x0000000000000000, // Adds
    0x0000000000000000, // Adr
    0x0000000000000000, // Adrp
    0x0000000000000000, // And
    0x0000000000000000, // Ands
    0x0000000000000000, // Asr
    0x0000000000000000, // Asrv
    0x0000000000000000, // At
    0x0000000000800000, // Autda
    0x0000000000800000, // Autdza
    0x0000000000800000, // Autdb
    0x0000000000800000, // Autdzb
    0x0000000000800000, // Autia
    0x0000000000800000, // Autia1716
    0x0000000000800000, // Autiasp
    0x0000000000800000, // Autiaz
    0x0000000000800000, // Autib
    0x0000000000800000, // Autib1716
    0x0000000000800000, // Autibsp
    0x0000000000800000, // Autibz
    0x0000000000800000, // Autiza
    0x0000000000800000, // Autizb
    0x0000000000002000, // Axflag
    0x0000000000000000, // B
    0x0000000000000000, // Bc
    0x0000000000000000, // Bfc
    0x0000000000000000, // Bfi
    0x0000000000000000, // Bfm
    0x0000000000000000, // Bfxil
    0x0000000000000000, // Bic
    0x0000000000000000, // Bics
    0x0000000000000000, // Bl
    0x0000000000000000, // Blr
    0x0000000000000000, // Br
    0x0000000000000000, // Brk
    0x0000000000000008, // Bti
    0x0000000000100000, // Cas
    0x0000000000100000, // Casa
    0x0000000000100000, // Casab
    0x0000000000100000, // Casah
    0x0000000000100000, // Casal
    0x0000000000100000, // Casalb
    0x0000000000100000, // Casalh
    0x0000000000100000, // Casb
    0x0000000000100000, // Cash
    0x0000000000100000, // Casl
    0x0000000000100000, // Caslb
    0x0000000000100000, // Caslh
    0x0000000000100000, // Casp
    0x0000000000100000, // Caspa
    0x0000000000100000, // Caspal
    0x0000000000100000, // Caspl
    0x0000000000000000, // Cbnz
    0x0000000000000000, // Cbz
    0x0000000000000000, // Ccmn
    0x0000000000000000, // Ccmp
    0x0000000000001000, // Cfinv
    0x0000000000000010, // Chkfeat
    0x0000000000000000, // Cinc
    0x0000000000000000, // Cinv
    0x0000000000000020, // Clrbhb
    0x0000000000000000, // Clrex
    0x0000000000000000, // Cls
    0x0000000000000000, // Clz
    0x0000000000000000, // Cmn
    0x0000000000000000, // Cmp
    0x0000000000200000, // Cmpp
    0x0000000000000000, // Cneg
    0x0000000000000080, // Cnt
    0x0000000000000040, // Crc32b
    0x0000000000000040, // Crc32cb
    0x0000000000000040, // Crc32ch
    0x0000000000000040, // Crc32cw
    0x0000000000000040, // Crc32cx
    0x0000000000000040, // Crc32h
    0x0000000000000040, // Crc32w
    0x0000000000000040, // Crc32x
    0x0000000000000000, // Csdb
    0x0000000000000000, // Csel
    0x0000000000000000, // Cset
    0x0000000000000000, // Csetm
    0x0000000000000000, // Csinc
    0x0000000000000000, // Csinv
    0x0000000000000000, // Csneg
    0x0000000000000080, // Ctz
    0x0000000000000000, // Dc
    0x0000000000000000, // Dcps1
    0x0000000000000000, // Dcps2
    0x0000000000000000, // Dcps3
    0x0000000000000100, // Dgh
    0x0000000000000000, // Dmb
    0x0000000000000000, // Drps
    0x0000000000000000, // Dsb
    0x0000000000000000, // Eon
    0x0000000000000000, // Eor
    0x0000000001000000, // Esb
    0x0000000000000000, // Extr
    0x0000000000000000, // Eret
    0x0000000000200000, // Gmi
    0x0000000000000000, // Hint
    0x0000000000000000, // Hlt
    0x0000000000000000, // Hvc
    0x0000000000000000, // Ic
    0x0000000000000000, // Isb
    0x0000000000100000, // Ldadd
    0x0000000000100000, // Ldadda
    0x0000000000100000, // Ldaddab
    0x0000000000100000, // Ldaddah
    0x0000000000100000, // Ldaddal
    0x0000000000100000, // Ldaddalb
    0x0000000000100000, // Ldaddalh
    0x0000000000100000, // Ldaddb
    0x0000000000100000, // Ldaddh
    0x0000000000100000, // Ldaddl
    0x0000000000100000, // Ldaddlb
    0x0000000000100000, // Ldaddlh
    0x0000000000000000, // Ldar
    0x0000000000000000, // Ldarb
    0x0000000000000000, // Ldarh
    0x0000000000000000, // Ldaxp
    0x0000000000000000, // Ldaxr
    0x0000000000000000, // Ldaxrb
    0x0000000000000000, // Ldaxrh
    0x0000000000100000, // Ldclr
    0x0000000000100000, // Ldclra
    0x0000000000100000, // Ldclrab
    0x0000000000100000, // Ldclrah
    0x0000000000100000, // Ldclral
    0x0000000000100000, // Ldclralb
    0x0000000000100000, // Ldclralh
    0x0000000000100000, // Ldclrb
    0x0000000000100000, // Ldclrh
    0x0000000000100000, // Ldclrl
    0x0000000000100000, // Ldclrlb
    0x0000000000100000, // Ldclrlh
    0x0000000000100000, // Ldeor
    0x0000000000100000, // Ldeora
    0x0000000000100000, // Ldeorab
    0x0000000000100000, // Ldeorah
    0x0000000000100000, // Ldeoral
    0x0000000000100000, // Ldeoralb
    0x0000000000100000, // Ldeoralh
    0x0000000000100000, // Ldeorb
    0x0000000000100000, // Ldeorh
    0x0000000000100000, // Ldeorl
    0x0000000000100000, // Ldeorlb
    0x0000000000100000, // Ldeorlh
    0x0000000000200000, // Ldg
    0x0000000000400000, // Ldgm
    0x0000000000080000, // Ldlar
    0x0000000000080000, // Ldlarb
    0x0000000000080000, // Ldlarh
    0x0000000000000000, // Ldnp
    0x0000000000000000, // Ldp
    0x0000000000000000, // Ldpsw
    0x0000000000000000, // Ldr
    0x0000000000800000, // Ldraa
    0x0000000000800000, // Ldrab
    0x0000000000000000, // Ldrb
    0x0000000000000000, // Ldrh
    0x0000000000000000, // Ldrsb
    0x0000000000000000, // Ldrsh
    0x0000000000000000, // Ldrsw
    0x0000000000100000, // Ldset
    0x0000000000100000, // Ldseta
    0x0000000000100000, // Ldsetab
    0x0000000000100000, // Ldsetah
    0x0000000000100000, // Ldsetal
    0x0000000000100000, // Ldsetalb
    0x0000000000100000, // Ldsetalh
    0x0000000000100000, // Ldsetb
    0x0000000000100000, // Ldseth
    0x0000000000100000, // Ldsetl
    0x0000000000100000, // Ldsetlb
    0x0000000000100000, // Ldsetlh
    0x0000000000100000, // Ldsmax
    0x0000000000100000, // Ldsmaxa
    0x0000000000100000, // Ldsmaxab
    0x0000000000100000, // Ldsmaxah
    0x0000000000100000, // Ldsmaxal
    0x0000000000100000, // Ldsmaxalb
    0x0000000000100000, // Ldsmaxalh
    0x0000000000100000, // Ldsmaxb
    0x0000000000100000, // Ldsmaxh
    0x0000000000100000, // Ldsmaxl
    0x0000000000100000, // Ldsmaxlb
    0x0000000000100000, // Ldsmaxlh
    0x0000000000100000, // Ldsmin
    0x0000000000100000, // Ldsmina
    0x0000000000100000, // Ldsminab
    0x0000000000100000, // Ldsminah
    0x0000000000100000, // Ldsminal
    0x0000000000100000, // Ldsminalb
    0x0000000000100000, // Ldsminalh
    0x0000000000100000, // Ldsminb
    0x0000000000100000, // Ldsminh
    0x0000000000100000, // Ldsminl
    0x0000000000100000, // Ldsminlb
    0x0000000000100000, // Ldsminlh
    0x0000000000000000, // Ldtr
    0x0000000000000000, // Ldtrb
    0x0000000000000000, // Ldtrh
    0x0000000000000000, // Ldtrsb
    0x0000000000000000, // Ldtrsh
    0x0000000000000000, // Ldtrsw
    0x0000000000100000, // Ldumax
    0x0000000000100000, // Ldumaxa
    0x0000000000100000, // Ldumaxab
    0x0000000000100000, // Ldumaxah
    0x0000000000100000, // Ldumaxal
    0x0000000000100000, // Ldumaxalb
    0x0000000000100000, // Ldumaxalh
    0x0000000000100000, // Ldumaxb
    0x0000000000100000, // Ldumaxh
    0x0000000000100000, // Ldumaxl
    0x0000000000100000, // Ldumaxlb
    0x0000000000100000, // Ldumaxlh
    0x0000000000100000, // Ldumin
    0x0000000000100000, // Ldumina
    0x0000000000100000, // Lduminab
    0x0000000000100000, // Lduminah
    0x0000000000100000, // Lduminal
    0x0000000000100000, // Lduminalb
    0x0000000000100000, // Lduminalh
    0x0000000000100000, // Lduminb
    0x0000000000100000, // Lduminh
    0x0000000000100000, // Lduminl
    0x0000000000100000, // Lduminlb
    0x0000000000100000, // Lduminlh
    0x0000000000000000, // Ldur
    0x0000000000000000, // Ldurb
    0x0000000000000000, // Ldurh
    0x0000000000000000, // Ldursb
    0x0000000000000000, // Ldursh
    0x0000000000000000, // Ldursw
    0x0000000000000000, // Ldxp
    0x0000000000000000, // Ldxr
    0x0000000000000000, // Ldxrb
    0x0000000000000000, // Ldxrh
    0x0000000000000000, // Lsl
    0x0000000000000000, // Lslv
    0x0000000000000000, // Lsr
    0x0000000000000000, // Lsrv
    0x0000000000000000, // Madd
    0x0000000000000000, // Mneg
    0x0000000000000000, // Mov
    0x0000000000000000, // Movk
    0x0000000000000000, // Movn
    0x0000000000000000, // Movz
    0x0000000000000000, // Mrs
    0x0000000000000000, // Msr
    0x0000000000000000, // Msub
    0x0000000000000000, // Mul
    0x0000000000000000, // Mvn
    0x0000000000000000, // Neg
    0x0000000000000000, // Negs
    0x0000000000000000, // Ngc
    0x0000000000000000, // Ngcs
    0x0000000000000000, // Nop
    0x0000000000000000, // Orn
    0x0000000000000000, // Orr
    0x0000000000800000, // Pacda
    0x0000000000800000, // Pacdb
    0x0000000000800000, // Pacdza
    0x0000000000800000, // Pacdzb
    0x0000000000800000, // Pacga
    0x0000000000000000, // Prfm
    0x0000000000000000, // Pssbb
    0x0000000000000000, // Rbit
    0x0000000000000000, // Ret
    0x0000000000000000, // Rev
    0x0000000000000000, // Rev16
    0x0000000000000000, // Rev32
    0x0000000000000000, // Rev64
    0x0000000000000000, // Ror
    0x0000000000000000, // Rorv
    0x0000000000000000, // Sbc
    0x0000000000000000, // Sbcs
    0x0000000000000000, // Sbfiz
    0x0000000000000000, // Sbfm
    0x0000000000000000, // Sbfx
    0x0000000000000000, // Sdiv
    0x0000000000001000, // Setf8
    0x0000000000001000, // Setf16
    0x0000000000000000, // Sev
    0x0000000000000000, // Sevl
    0x0000000000000000, // Smaddl
    0x0000000000000080, // Smax
    0x0000000000000000, // Smc
    0x0000000000000080, // Smin
    0x0000000000000000, // Smnegl
    0x0000000000000000, // Smsubl
    0x0000000000000000, // Smulh
    0x0000000000000000, // Smull
    0x0000000000000000, // Ssbb
    0x0000000000200000, // St2g
    0x0000000000100000, // Stadd
    0x0000000000100000, // Staddl
    0x0000000000100000, // Staddb
    0x0000000000100000, // Staddlb
    0x0000000000100000, // Staddh
    0x0000000000100000, // Staddlh
    0x0000000000100000, // Stclr
    0x0000000000100000, // Stclrl
    0x0000000000100000, // Stclrb
    0x0000000000100000, // Stclrlb
    0x0000000000100000, // Stclrh
    0x0000000000100000, // Stclrlh
    0x0000000000100000, // Steor
    0x0000000000100000, // Steorl
    0x0000000000100000, // Steorb
    0x0000000000100000, // Steorlb
    0x0000000000100000, // Steorh
    0x0000000000100000, // Steorlh
    0x0000000000200000, // Stg
    0x0000000000400000, // Stgm
    0x0000000000200000, // Stgp
    0x0000000000080000, // Stllr
    0x0000000000080000, // Stllrb
    0x0000000000080000, // Stllrh
    0x0000000000000000, // Stlr
    0x0000000000000000, // Stlrb
    0x0000000000000000, // Stlrh
    0x0000000000000000, // Stlxp
    0x0000000000000000, // Stlxr
    0x0000000000000000, // Stlxrb
    0x0000000000000000, // Stlxrh
    0x0000000000000000, // Stnp
    0x0000000000000000, // Stp
    0x0000000000000000, // Str
    0x0000000000000000, // Strb
    0x0000000000000000, // Strh
    0x0000000000100000, // Stset
    0x0000000000100000, // Stsetl
    0x0000000000100000, // Stsetb
    0x0000000000100000, // Stsetlb
    0x0000000000100000, // Stseth
    0x0000000000100000, // Stsetlh
    0x0000000000100000, // Stsmax
    0x0000000000100000, // Stsmaxl
    0x0000000000100000, // Stsmaxb
    0x0000000000100000, // Stsmaxlb
    0x0000000000100000, // Stsmaxh
    0x0000000000100000, // Stsmaxlh
    0x0000000000100000, // Stsmin
    0x0000000000100000, // Stsminl
    0x0000000000100000, // Stsminb
    0x0000000000100000, // Stsminlb
    0x0000000000100000, // Stsminh
    0x0000000000100000, // Stsminlh
    0x0000000000000000, // Sttr
    0x0000000000000000, // Sttrb
    0x0000000000000000, // Sttrh
    0x0000000000100000, // Stumax
    0x0000000000100000, // Stumaxl
    0x0000000000100000, // Stumaxb
    0x0000000000100000, // Stumaxlb
    0x0000000000100000, // Stumaxh
    0x0000000000100000, // Stumaxlh
    0x0000000000100000, // Stumin
    0x0000000000100000, // Stuminl
    0x0000000000100000, // Stuminb
    0x0000000000100000, // Stuminlb
    0x0000000000100000, // Stuminh
    0x0000000000100000, // Stuminlh
    0x0000000000000000, // Stur
    0x0000000000000000, // Sturb
    0x0000000000000000, // Sturh
    0x0000000000000000, // Stxp
    0x0000000000000000, // Stxr
    0x0000000000000000, // Stxrb
    0x0000000000000000, // Stxrh
    0x0000000000200000, // Stz2g
    0x0000000000200000, // Stzg
    0x0000000000400000, // Stzgm
    0x0000000000000000, // Sub
    0x0000000000200000, // Subg
    0x0000000000200000, // Subp
    0x0000000000200000, // Subps
    0x0000000000000000, // Subs
    0x0000000000000000, // Svc
    0x0000000000100000, // Swp
    0x0000000000100000, // Swpa
    0x0000000000100000, // Swpab
    0x0000000000100000, // Swpah
    0x0000000000100000, // Swpal
    0x0000000000100000, // Swpalb
    0x0000000000100000, // Swpalh
    0x0000000000100000, // Swpb
    0x0000000000100000, // Swph
    0x0000000000100000, // Swpl
    0x0000000000100000, // Swplb
    0x0000000000100000, // Swplh
    0x0000000000000000, // Sxtb
    0x0000000000000000, // Sxth
    0x0000000000000000, // Sxtw
    0x0000000000000000, // Sys
    0x0000000000000000, // Tlbi
    0x0000000000000000, // Tst
    0x0000000000000000, // Tbnz
    0x0000000000000000, // Tbz
    0x0000000000000000, // Ubfiz
    0x0000000000000000, // Ubfm
    0x0000000000000000, // Ubfx
    0x0000000000000000, // Udf
    0x0000000000000000, // Udiv
    0x0000000000000000, // Umaddl
    0x0000000000000080, // Umax
    0x0000000000000080, // Umin
    0x0000000000000000, // Umnegl
    0x0000000000000000, // Umull
    0x0000000000000000, // Umulh
    0x0000000000000000, // Umsubl
    0x0000000000000000, // Uxtb
    0x0000000000000000, // Uxth
    0x0000000000000000, // Wfe
    0x0000000000000000, // Wfi
    0x0000000000002000, // Xaflag
    0x0000000000800000, // Xpacd
    0x0000000000800000, // Xpaci
    0x0000000000800000, // Xpaclri
    0x0000000000000000, // Yield
    0x0000000000000002, // Abs_v
    0x0000000000000002, // Add_v
    0x0000000000000002, // Addhn_v
    0x0000000000000002, // Addhn2_v
    0x0000000000000002, // Addp_v
    0x0000000000000002, // Addv_v
    0x0000000000000003, // Aesd_v
    0x0000000000000003, // Aese_v
    0x0000000000000003, // Aesimc_v
    0x0000000000000003, // Aesmc_v
    0x0000000000000002, // And_v
    0x0000000010000002, // Bcax_v
    0x0000000000000006, // Bfcvt_v
    0x0000000000000006, // Bfcvtn_v
    0x0000000000000006, // Bfcvtn2_v
    0x0000000000000006, // Bfdot_v
    0x0000000000000006, // Bfmlalb_v
    0x0000000000000006, // Bfmlalt_v
    0x0000000000000006, // Bfmmla_v
    0x0000000000000002, // Bic_v
    0x0000000000000002, // Bif_v
    0x0000000000000002, // Bit_v
    0x0000000000000002, // Bsl_v
    0x0000000000000002, // Cls_v
    0x0000000000000002, // Clz_v
    0x0000000000000002, // Cmeq_v
    0x0000000000000002, // Cmge_v
    0x0000000000000002, // Cmgt_v
    0x0000000000000002, // Cmhi_v
    0x0000000000000002, // Cmhs_v
    0x0000000000000002, // Cmle_v
    0x0000000000000002, // Cmlt_v
    0x0000000000000002, // Cmtst_v
    0x0000000000000002, // Cnt_v
    0x0000000000000002, // Dup_v
    0x0000000000000002, // Eor_v
    0x0000000010000002, // Eor3_v
    0x0000000000000002, // Ext_v
    0x0000000000004002, // Fabd_v
    0x0000000000004002, // Fabs_v
    0x0000000000004002, // Facge_v
    0x0000000000004002, // Facgt_v
    0x0000000000004002, // Fadd_v
    0x0000000000004002, // Faddp_v
    0x0000000000000402, // Fcadd_v
    0x0000000000004002, // Fccmp_v
    0x0000000000004002, // Fccmpe_v
    0x0000000000004002, // Fcmeq_v
    0x0000000000004002, // Fcmge_v
    0x0000000000004002, // Fcmgt_v
    0x0000000000000402, // Fcmla_v
    0x0000000000004002, // Fcmle_v
    0x0000000000004002, // Fcmlt_v
    0x0000000000004002, // Fcmp_v
    0x0000000000004002, // Fcmpe_v
    0x0000000000004002, // Fcsel_v
    0x0000000000000002, // Fcvt_v
    0x0000000000004002, // Fcvtas_v
    0x0000000000004002, // Fcvtau_v
    0x0000000000000002, // Fcvtl_v
    0x0000000000000002, // Fcvtl2_v
    0x0000000000004002, // Fcvtms_v
    0x0000000000004002, // Fcvtmu_v
    0x0000000000008002, // Fcvtn_v
    0x0000000000008002, // Fcvtn2_v
    0x0000000000004002, // Fcvtns_v
    0x0000000000004002, // Fcvtnu_v
    0x0000000000004002, // Fcvtps_v
    0x0000000000004002, // Fcvtpu_v
    0x0000000000000000, // Fcvtxn_v
    0x0000000000000000, // Fcvtxn2_v
    0x0000000000004002, // Fcvtzs_v
    0x0000000000004002, // Fcvtzu_v
    0x0000000000004002, // Fdiv_v
    0x0000000000040002, // Fjcvtzs_v
    0x0000000000004002, // Fmadd_v
    0x0000000000004002, // Fmax_v
    0x0000000000004002, // Fmaxnm_v
    0x0000000000004002, // Fmaxnmp_v
    0x0000000000004002, // Fmaxnmv_v
    0x0000000000004002, // Fmaxp_v
    0x0000000000004002, // Fmaxv_v
    0x0000000000004002, // Fmin_v
    0x0000000000004002, // Fminnm_v
    0x0000000000004002, // Fminnmp_v
    0x0000000000004002, // Fminnmv_v
    0x0000000000004002, // Fminp_v
    0x0000000000004002, // Fminv_v
    0x0000000000004002, // Fmla_v
    0x0000000000000802, // Fmlal_v
    0x0000000000000802, // Fmlal2_v
    0x0000000000004002, // Fmls_v
    0x0000000000000802, // Fmlsl_v
    0x0000000000000802, // Fmlsl2_v
    0x0000000000004002, // Fmov_v
    0x0000000000004002, // Fmsub_v
    0x0000000000004002, // Fmul_v
    0x0000000000004002, // Fmulx_v
    0x0000000000004002, // Fneg_v
    0x0000000000004002, // Fnmadd_v
    0x0000000000004002, // Fnmsub_v
    0x0000000000004002, // Fnmul_v
    0x0000000000004002, // Frecpe_v
    0x0000000000004002, // Frecps_v
    0x0000000000004002, // Frecpx_v
    0x0000000000010002, // Frint32x_v
    0x0000000000010002, // Frint32z_v
    0x0000000000010002, // Frint64x_v
    0x0000000000010002, // Frint64z_v
    0x0000000000004002, // Frinta_v
    0x0000000000004002, // Frinti_v
    0x0000000000004002, // Frintm_v
    0x0000000000004002, // Frintn_v
    0x0000000000004002, // Frintp_v
    0x0000000000004002, // Frintx_v
    0x0000000000004002, // Frintz_v
    0x0000000000004002, // Frsqrte_v
    0x0000000000004002, // Frsqrts_v
    0x0000000000004002, // Fsqrt_v
    0x0000000000004002, // Fsub_v
    0x0000000000000002, // Ins_v
    0x0000000000000002, // Ld1_v
    0x0000000000000002, // Ld1r_v
    0x0000000000000002, // Ld2_v
    0x0000000000000002, // Ld2r_v
    0x0000000000000002, // Ld3_v
    0x0000000000000002, // Ld3r_v
    0x0000000000000002, // Ld4_v
    0x0000000000000002, // Ld4r_v
    0x0000000000000002, // Ldnp_v
    0x0000000000000002, // Ldp_v
    0x0000000000000002, // Ldr_v
    0x0000000000000002, // Ldur_v
    0x0000000000000002, // Mla_v
    0x0000000000000002, // Mls_v
    0x0000000000000002, // Mov_v
    0x0000000000000002, // Movi_v
    0x0000000000000002, // Mul_v
    0x0000000000000002, // Mvn_v
    0x0000000000000002, // Mvni_v
    0x0000000000000002, // Neg_v
    0x0000000000000002, // Not_v
    0x0000000000000002, // Orn_v
    0x0000000000000002, // Orr_v
    0x0000000000000002, // Pmul_v
    0x0000000000000002, // Pmull_v
    0x0000000000000002, // Pmull2_v
    0x0000000000000002, // Raddhn_v
    0x0000000000000002, // Raddhn2_v
    0x0000000010000002, // Rax1_v
    0x0000000000000002, // Rbit_v
    0x0000000000000002, // Rev16_v
    0x0000000000000002, // Rev32_v
    0x0000000000000002, // Rev64_v
    0x0000000000000002, // Rshrn_v
    0x0000000000000002, // Rshrn2_v
    0x0000000000000002, // Rsubhn_v
    0x0000000000000002, // Rsubhn2_v
    0x0000000000000002, // Saba_v
    0x0000000000000002, // Sabal_v
    0x0000000000000002, // Sabal2_v
    0x0000000000000002, // Sabd_v
    0x0000000000000002, // Sabdl_v
    0x0000000000000002, // Sabdl2_v
    0x0000000000000002, // Sadalp_v
    0x0000000000000002, // Saddl_v
    0x0000000000000002, // Saddl2_v
    0x0000000000000002, // Saddlp_v
    0x0000000000000002, // Saddlv_v
    0x0000000000000002, // Saddw_v
    0x0000000000000002, // Saddw2_v
    0x0000000000004002, // Scvtf_v
    0x0000000000000202, // Sdot_v
    0x0000000004000002, // Sha1c_v
    0x0000000004000002, // Sha1h_v
    0x0000000004000002, // Sha1m_v
    0x0000000004000002, // Sha1p_v
    0x0000000004000002, // Sha1su0_v
    0x0000000004000002, // Sha1su1_v
    0x0000000008000002, // Sha256h_v
    0x0000000008000002, // Sha256h2_v
    0x0000000008000002, // Sha256su0_v
    0x0000000008000002, // Sha256su1_v
    0x0000000020000002, // Sha512h_v
    0x0000000020000002, // Sha512h2_v
    0x0000000020000002, // Sha512su0_v
    0x0000000020000002, // Sha512su1_v
    0x0000000000000002, // Shadd_v
    0x0000000000000002, // Shl_v
    0x0000000000000002, // Shll_v
    0x0000000000000002, // Shll2_v
    0x0000000000000002, // Shrn_v
    0x0000000000000002, // Shrn2_v
    0x0000000000000002, // Shsub_v
    0x0000000000000002, // Sli_v
    0x0000000040000002, // Sm3partw1_v
    0x0000000040000002, // Sm3partw2_v
    0x0000000040000002, // Sm3ss1_v
    0x0000000040000002, // Sm3tt1a_v
    0x0000000040000002, // Sm3tt1b_v
    0x0000000040000002, // Sm3tt2a_v
    0x0000000040000002, // Sm3tt2b_v
    0x0000000080000002, // Sm4e_v
    0x0000000080000002, // Sm4ekey_v
    0x0000000000000002, // Smax_v
    0x0000000000000002, // Smaxp_v
    0x0000000000000002, // Smaxv_v
    0x0000000000000002, // Smin_v
    0x0000000000000002, // Sminp_v
    0x0000000000000002, // Sminv_v
    0x0000000000000002, // Smlal_v
    0x0000000000000002, // Smlal2_v
    0x0000000000000002, // Smlsl_v
    0x0000000000000002, // Smlsl2_v
    0x0000000000020002, // Smmla_v
    0x0000000000000002, // Smov_v
    0x0000000000000002, // Smull_v
    0x0000000000000002, // Smull2_v
    0x0000000000000002, // Sqabs_v
    0x0000000000000002, // Sqadd_v
    0x0000000000000002, // Sqdmlal_v
    0x0000000000000002, // Sqdmlal2_v
    0x0000000000000002, // Sqdmlsl_v
    0x0000000000000002, // Sqdmlsl2_v
    0x0000000000000002, // Sqdmulh_v
    0x0000000000000002, // Sqdmull_v
    0x0000000000000002, // Sqdmull2_v
    0x0000000000000002, // Sqneg_v
    0x0000000002000002, // Sqrdmlah_v
    0x0000000002000002, // Sqrdmlsh_v
    0x0000000000000002, // Sqrdmulh_v
    0x0000000000000002, // Sqrshl_v
    0x0000000000000002, // Sqrshrn_v
    0x0000000000000002, // Sqrshrn2_v
    0x0000000000000002, // Sqrshrun_v
    0x0000000000000002, // Sqrshrun2_v
    0x0000000000000002, // Sqshl_v
    0x0000000000000002, // Sqshlu_v
    0x0000000000000002, // Sqshrn_v
    0x0000000000000002, // Sqshrn2_v
    0x0000000000000002, // Sqshrun_v
    0x0000000000000002, // Sqshrun2_v
    0x0000000000000002, // Sqsub_v
    0x0000000000000002, // Sqxtn_v
    0x0000000000000002, // Sqxtn2_v
    0x0000000000000002, // Sqxtun_v
    0x0000000000000002, // Sqxtun2_v
    0x0000000000000002, // Srhadd_v
    0x0000000000000002, // Sri_v
    0x0000000000000002, // Srshl_v
    0x0000000000000002, // Srshr_v
    0x0000000000000002, // Srsra_v
    0x0000000000000002, // Sshl_v
    0x0000000000000002, // Sshll_v
    0x0000000000000002, // Sshll2_v
    0x0000000000000002, // Sshr_v
    0x0000000000000002, // Ssra_v
    0x0000000000000002, // Ssubl_v
    0x0000000000000002, // Ssubl2_v
    0x0000000000000002, // Ssubw_v
    0x0000000000000002, // Ssubw2_v
    0x0000000000000002, // St1_v
    0x0000000000000002, // St2_v
    0x0000000000000002, // St3_v
    0x0000000000000002, // St4_v
    0x0000000000000002, // Stnp_v
    0x0000000000000002, // Stp_v
    0x0000000000000002, // Str_v
    0x0000000000000002, // Stur_v
    0x0000000000000002, // Sub_v
    0x0000000000000002, // Subhn_v
    0x0000000000000002, // Subhn2_v
    0x0000000000020002, // Sudot_v
    0x0000000000000002, // Suqadd_v
    0x0000000000000002, // Sxtl_v
    0x0000000000000002, // Sxtl2_v
    0x0000000000000002, // Tbl_v
    0x0000000000000002, // Tbx_v
    0x0000000000000002, // Trn1_v
    0x0000000000000002, // Trn2_v
    0x0000000000000002, // Uaba_v
    0x0000000000000002, // Uabal_v
    0x0000000000000002, // Uabal2_v
    0x0000000000000002, // Uabd_v
    0x0000000000000002, // Uabdl_v
    0x0000000000000002, // Uabdl2_v
    0x0000000000000002, // Uadalp_v
    0x0000000000000002, // Uaddl_v
    0x0000000000000002, // Uaddl2_v
    0x0000000000000002, // Uaddlp_v
    0x0000000000000002, // Uaddlv_v
    0x0000000000000002, // Uaddw_v
    0x0000000000000002, // Uaddw2_v
    0x0000000000004002, // Ucvtf_v
    0x0000000000000202, // Udot_v
    0x0000000000000002, // Uhadd_v
    0x0000000000000002, // Uhsub_v
    0x0000000000000002, // Umax_v
    0x0000000000000002, // Umaxp_v
    0x0000000000000002, // Umaxv_v
    0x0000000000000002, // Umin_v
    0x0000000000000002, // Uminp_v
    0x0000000000000002, // Uminv_v
    0x0000000000000002, // Umlal_v
    0x0000000000000002, // Umlal2_v
    0x0000000000000002, // Umlsl_v
    0x0000000000000002, // Umlsl2_v
    0x0000000000020002, // Ummla_v
    0x0000000000000002, // Umov_v
    0x0000000000000002, // Umull_v
    0x0000000000000002, // Umull2_v
    0x0000000000000002, // Uqadd_v
    0x0000000000000002, // Uqrshl_v
    0x0000000000000002, // Uqrshrn_v
    0x0000000000000002, // Uqrshrn2_v
    0x0000000000000002, // Uqshl_v
    0x0000000000000002, // Uqshrn_v
    0x0000000000000002, // Uqshrn2_v
    0x0000000000000002, // Uqsub_v
    0x0000000000000002, // Uqxtn_v
    0x0000000000000002, // Uqxtn2_v
    0x0000000000000002, // Urecpe_v
    0x0000000000000002, // Urhadd_v
    0x0000000000000002, // Urshl_v
    0x0000000000000002, // Urshr_v
    0x0000000000000002, // Ursqrte_v
    0x0000000000000002, // Ursra_v
    0x0000000000020002, // Usdot_v
    0x0000000000000002, // Ushl_v
    0x0000000000000002, // Ushll_v
    0x0000000000000002, // Ushll2_v
    0x0000000000000002, // Ushr_v
    0x0000000000020002, // Usmmla_v
    0x0000000000000002, // Usqadd_v
    0x0000000000000002, // Usra_v
    0x0000000000000002, // Usubl_v
    0x0000000000000002, // Usubl2_v
    0x0000000000000002, // Usubw_v
    0x0000000000000002, // Usubw2_v
    0x0000000000000002, // Uxtl_v
    0x0000000000000002, // Uxtl2_v
    0x0000000000000002, // Uzp1_v
    0x0000000000000002, // Uzp2_v
    0x0000000010000002, // Xar_v
    0x0000000000000002, // Xtn_v
    0x0000000000000002, // Xtn2_v
    0x0000000000000002, // Zip1_v
    0x0000000000000002, // Zip2_v
];

/// Requirements common to every form, indexed by `InstId as usize`.
static INST_BASE_FEATURE_MASKS: [u64; InstId::_Count as usize] = [
    0x0000000000000000, // None
    0x0000000000000080, // Abs
    0x0000000000000000, // Adc
    0x0000000000000000, // Adcs
    0x0000000000000000, // Add
    0x0000000000200000, // Addg
    0x0000000000000000, // Adds
    0x0000000000000000, // Adr
    0x0000000000000000, // Adrp
    0x0000000000000000, // And
    0x0000000000000000, // Ands
    0x0000000000000000, // Asr
    0x0000000000000000, // Asrv
    0x0000000000000000, // At
    0x0000000000800000, // Autda
    0x0000000000800000, // Autdza
    0x0000000000800000, // Autdb
    0x0000000000800000, // Autdzb
    0x0000000000800000, // Autia
    0x0000000000800000, // Autia1716
    0x0000000000800000, // Autiasp
    0x0000000000800000, // Autiaz
    0x0000000000800000, // Autib
    0x0000000000800000, // Autib1716
    0x0000000000800000, // Autibsp
    0x0000000000800000, // Autibz
    0x0000000000800000, // Autiza
    0x0000000000800000, // Autizb
    0x0000000000002000, // Axflag
    0x0000000000000000, // B
    0x0000000000000000, // Bc
    0x0000000000000000, // Bfc
    0x0000000000000000, // Bfi
    0x0000000000000000, // Bfm
    0x0000000000000000, // Bfxil
    0x0000000000000000, // Bic
    0x0000000000000000, // Bics
    0x0000000000000000, // Bl
    0x0000000000000000, // Blr
    0x0000000000000000, // Br
    0x0000000000000000, // Brk
    0x0000000000000008, // Bti
    0x0000000000100000, // Cas
    0x0000000000100000, // Casa
    0x0000000000100000, // Casab
    0x0000000000100000, // Casah
    0x0000000000100000, // Casal
    0x0000000000100000, // Casalb
    0x0000000000100000, // Casalh
    0x0000000000100000, // Casb
    0x0000000000100000, // Cash
    0x0000000000100000, // Casl
    0x0000000000100000, // Caslb
    0x0000000000100000, // Caslh
    0x0000000000100000, // Casp
    0x0000000000100000, // Caspa
    0x0000000000100000, // Caspal
    0x0000000000100000, // Caspl
    0x0000000000000000, // Cbnz
    0x0000000000000000, // Cbz
    0x0000000000000000, // Ccmn
    0x0000000000000000, // Ccmp
    0x0000000000001000, // Cfinv
    0x0000000000000010, // Chkfeat
    0x0000000000000000, // Cinc
    0x0000000000000000, // Cinv
    0x0000000000000020, // Clrbhb
    0x0000000000000000, // Clrex
    0x0000000000000000, // Cls
    0x0000000000000000, // Clz
    0x0000000000000000, // Cmn
    0x0000000000000000, // Cmp
    0x0000000000200000, // Cmpp
    0x0000000000000000, // Cneg
    0x0000000000000080, // Cnt
    0x0000000000000040, // Crc32b
    0x0000000000000040, // Crc32cb
    0x0000000000000040, // Crc32ch
    0x0000000000000040, // Crc32cw
    0x0000000000000040, // Crc32cx
    0x0000000000000040, // Crc32h
    0x0000000000000040, // Crc32w
    0x0000000000000040, // Crc32x
    0x0000000000000000, // Csdb
    0x0000000000000000, // Csel
    0x0000000000000000, // Cset
    0x0000000000000000, // Csetm
    0x0000000000000000, // Csinc
    0x0000000000000000, // Csinv
    0x0000000000000000, // Csneg
    0x0000000000000080, // Ctz
    0x0000000000000000, // Dc
    0x0000000000000000, // Dcps1
    0x0000000000000000, // Dcps2
    0x0000000000000000, // Dcps3
    0x0000000000000100, // Dgh
    0x0000000000000000, // Dmb
    0x0000000000000000, // Drps
    0x0000000000000000, // Dsb
    0x0000000000000000, // Eon
    0x0000000000000000, // Eor
    0x0000000001000000, // Esb
    0x0000000000000000, // Extr
    0x0000000000000000, // Eret
    0x0000000000200000, // Gmi
    0x0000000000000000, // Hint
    0x0000000000000000, // Hlt
    0x0000000000000000, // Hvc
    0x0000000000000000, // Ic
    0x0000000000000000, // Isb
    0x0000000000100000, // Ldadd
    0x0000000000100000, // Ldadda
    0x0000000000100000, // Ldaddab
    0x0000000000100000, // Ldaddah
    0x0000000000100000, // Ldaddal
    0x0000000000100000, // Ldaddalb
    0x0000000000100000, // Ldaddalh
    0x0000000000100000, // Ldaddb
    0x0000000000100000, // Ldaddh
    0x0000000000100000, // Ldaddl
    0x0000000000100000, // Ldaddlb
    0x0000000000100000, // Ldaddlh
    0x0000000000000000, // Ldar
    0x0000000000000000, // Ldarb
    0x0000000000000000, // Ldarh
    0x0000000000000000, // Ldaxp
    0x0000000000000000, // Ldaxr
    0x0000000000000000, // Ldaxrb
    0x0000000000000000, // Ldaxrh
    0x0000000000100000, // Ldclr
    0x0000000000100000, // Ldclra
    0x0000000000100000, // Ldclrab
    0x0000000000100000, // Ldclrah
    0x0000000000100000, // Ldclral
    0x0000000000100000, // Ldclralb
    0x0000000000100000, // Ldclralh
    0x0000000000100000, // Ldclrb
    0x0000000000100000, // Ldclrh
    0x0000000000100000, // Ldclrl
    0x0000000000100000, // Ldclrlb
    0x0000000000100000, // Ldclrlh
    0x0000000000100000, // Ldeor
    0x0000000000100000, // Ldeora
    0x0000000000100000, // Ldeorab
    0x0000000000100000, // Ldeorah
    0x0000000000100000, // Ldeoral
    0x0000000000100000, // Ldeoralb
    0x0000000000100000, // Ldeoralh
    0x0000000000100000, // Ldeorb
    0x0000000000100000, // Ldeorh
    0x0000000000100000, // Ldeorl
    0x0000000000100000, // Ldeorlb
    0x0000000000100000, // Ldeorlh
    0x0000000000200000, // Ldg
    0x0000000000400000, // Ldgm
    0x0000000000080000, // Ldlar
    0x0000000000080000, // Ldlarb
    0x0000000000080000, // Ldlarh
    0x0000000000000000, // Ldnp
    0x0000000000000000, // Ldp
    0x0000000000000000, // Ldpsw
    0x0000000000000000, // Ldr
    0x0000000000800000, // Ldraa
    0x0000000000800000, // Ldrab
    0x0000000000000000, // Ldrb
    0x0000000000000000, // Ldrh
    0x0000000000000000, // Ldrsb
    0x0000000000000000, // Ldrsh
    0x0000000000000000, // Ldrsw
    0x0000000000100000, // Ldset
    0x0000000000100000, // Ldseta
    0x0000000000100000, // Ldsetab
    0x0000000000100000, // Ldsetah
    0x0000000000100000, // Ldsetal
    0x0000000000100000, // Ldsetalb
    0x0000000000100000, // Ldsetalh
    0x0000000000100000, // Ldsetb
    0x0000000000100000, // Ldseth
    0x0000000000100000, // Ldsetl
    0x0000000000100000, // Ldsetlb
    0x0000000000100000, // Ldsetlh
    0x0000000000100000, // Ldsmax
    0x0000000000100000, // Ldsmaxa
    0x0000000000100000, // Ldsmaxab
    0x0000000000100000, // Ldsmaxah
    0x0000000000100000, // Ldsmaxal
    0x0000000000100000, // Ldsmaxalb
    0x0000000000100000, // Ldsmaxalh
    0x0000000000100000, // Ldsmaxb
    0x0000000000100000, // Ldsmaxh
    0x0000000000100000, // Ldsmaxl
    0x0000000000100000, // Ldsmaxlb
    0x0000000000100000, // Ldsmaxlh
    0x0000000000100000, // Ldsmin
    0x0000000000100000, // Ldsmina
    0x0000000000100000, // Ldsminab
    0x0000000000100000, // Ldsminah
    0x0000000000100000, // Ldsminal
    0x0000000000100000, // Ldsminalb
    0x0000000000100000, // Ldsminalh
    0x0000000000100000, // Ldsminb
    0x0000000000100000, // Ldsminh
    0x0000000000100000, // Ldsminl
    0x0000000000100000, // Ldsminlb
    0x0000000000100000, // Ldsminlh
    0x0000000000000000, // Ldtr
    0x0000000000000000, // Ldtrb
    0x0000000000000000, // Ldtrh
    0x0000000000000000, // Ldtrsb
    0x0000000000000000, // Ldtrsh
    0x0000000000000000, // Ldtrsw
    0x0000000000100000, // Ldumax
    0x0000000000100000, // Ldumaxa
    0x0000000000100000, // Ldumaxab
    0x0000000000100000, // Ldumaxah
    0x0000000000100000, // Ldumaxal
    0x0000000000100000, // Ldumaxalb
    0x0000000000100000, // Ldumaxalh
    0x0000000000100000, // Ldumaxb
    0x0000000000100000, // Ldumaxh
    0x0000000000100000, // Ldumaxl
    0x0000000000100000, // Ldumaxlb
    0x0000000000100000, // Ldumaxlh
    0x0000000000100000, // Ldumin
    0x0000000000100000, // Ldumina
    0x0000000000100000, // Lduminab
    0x0000000000100000, // Lduminah
    0x0000000000100000, // Lduminal
    0x0000000000100000, // Lduminalb
    0x0000000000100000, // Lduminalh
    0x0000000000100000, // Lduminb
    0x0000000000100000, // Lduminh
    0x0000000000100000, // Lduminl
    0x0000000000100000, // Lduminlb
    0x0000000000100000, // Lduminlh
    0x0000000000000000, // Ldur
    0x0000000000000000, // Ldurb
    0x0000000000000000, // Ldurh
    0x0000000000000000, // Ldursb
    0x0000000000000000, // Ldursh
    0x0000000000000000, // Ldursw
    0x0000000000000000, // Ldxp
    0x0000000000000000, // Ldxr
    0x0000000000000000, // Ldxrb
    0x0000000000000000, // Ldxrh
    0x0000000000000000, // Lsl
    0x0000000000000000, // Lslv
    0x0000000000000000, // Lsr
    0x0000000000000000, // Lsrv
    0x0000000000000000, // Madd
    0x0000000000000000, // Mneg
    0x0000000000000000, // Mov
    0x0000000000000000, // Movk
    0x0000000000000000, // Movn
    0x0000000000000000, // Movz
    0x0000000000000000, // Mrs
    0x0000000000000000, // Msr
    0x0000000000000000, // Msub
    0x0000000000000000, // Mul
    0x0000000000000000, // Mvn
    0x0000000000000000, // Neg
    0x0000000000000000, // Negs
    0x0000000000000000, // Ngc
    0x0000000000000000, // Ngcs
    0x0000000000000000, // Nop
    0x0000000000000000, // Orn
    0x0000000000000000, // Orr
    0x0000000000800000, // Pacda
    0x0000000000800000, // Pacdb
    0x0000000000800000, // Pacdza
    0x0000000000800000, // Pacdzb
    0x0000000000800000, // Pacga
    0x0000000000000000, // Prfm
    0x0000000000000000, // Pssbb
    0x0000000000000000, // Rbit
    0x0000000000000000, // Ret
    0x0000000000000000, // Rev
    0x0000000000000000, // Rev16
    0x0000000000000000, // Rev32
    0x0000000000000000, // Rev64
    0x0000000000000000, // Ror
    0x0000000000000000, // Rorv
    0x0000000000000000, // Sbc
    0x0000000000000000, // Sbcs
    0x0000000000000000, // Sbfiz
    0x0000000000000000, // Sbfm
    0x0000000000000000, // Sbfx
    0x0000000000000000, // Sdiv
    0x0000000000001000, // Setf8
    0x0000000000001000, // Setf16
    0x0000000000000000, // Sev
    0x0000000000000000, // Sevl
    0x0000000000000000, // Smaddl
    0x0000000000000080, // Smax
    0x0000000000000000, // Smc
    0x0000000000000080, // Smin
    0x0000000000000000, // Smnegl
    0x0000000000000000, // Smsubl
    0x0000000000000000, // Smulh
    0x0000000000000000, // Smull
    0x0000000000000000, // Ssbb
    0x0000000000200000, // St2g
    0x0000000000100000, // Stadd
    0x0000000000100000, // Staddl
    0x0000000000100000, // Staddb
    0x0000000000100000, // Staddlb
    0x0000000000100000, // Staddh
    0x0000000000100000, // Staddlh
    0x0000000000100000, // Stclr
    0x0000000000100000, // Stclrl
    0x0000000000100000, // Stclrb
    0x0000000000100000, // Stclrlb
    0x0000000000100000, // Stclrh
    0x0000000000100000, // Stclrlh
    0x0000000000100000, // Steor
    0x0000000000100000, // Steorl
    0x0000000000100000, // Steorb
    0x0000000000100000, // Steorlb
    0x0000000000100000, // Steorh
    0x0000000000100000, // Steorlh
    0x0000000000200000, // Stg
    0x0000000000400000, // Stgm
    0x0000000000200000, // Stgp
    0x0000000000080000, // Stllr
    0x0000000000080000, // Stllrb
    0x0000000000080000, // Stllrh
    0x0000000000000000, // Stlr
    0x0000000000000000, // Stlrb
    0x0000000000000000, // Stlrh
    0x0000000000000000, // Stlxp
    0x0000000000000000, // Stlxr
    0x0000000000000000, // Stlxrb
    0x0000000000000000, // Stlxrh
    0x0000000000000000, // Stnp
    0x0000000000000000, // Stp
    0x0000000000000000, // Str
    0x0000000000000000, // Strb
    0x0000000000000000, // Strh
    0x0000000000100000, // Stset
    0x0000000000100000, // Stsetl
    0x0000000000100000, // Stsetb
    0x0000000000100000, // Stsetlb
    0x0000000000100000, // Stseth
    0x0000000000100000, // Stsetlh
    0x0000000000100000, // Stsmax
    0x0000000000100000, // Stsmaxl
    0x0000000000100000, // Stsmaxb
    0x0000000000100000, // Stsmaxlb
    0x0000000000100000, // Stsmaxh
    0x0000000000100000, // Stsmaxlh
    0x0000000000100000, // Stsmin
    0x0000000000100000, // Stsminl
    0x0000000000100000, // Stsminb
    0x0000000000100000, // Stsminlb
    0x0000000000100000, // Stsminh
    0x0000000000100000, // Stsminlh
    0x0000000000000000, // Sttr
    0x0000000000000000, // Sttrb
    0x0000000000000000, // Sttrh
    0x0000000000100000, // Stumax
    0x0000000000100000, // Stumaxl
    0x0000000000100000, // Stumaxb
    0x0000000000100000, // Stumaxlb
    0x0000000000100000, // Stumaxh
    0x0000000000100000, // Stumaxlh
    0x0000000000100000, // Stumin
    0x0000000000100000, // Stuminl
    0x0000000000100000, // Stuminb
    0x0000000000100000, // Stuminlb
    0x0000000000100000, // Stuminh
    0x0000000000100000, // Stuminlh
    0x0000000000000000, // Stur
    0x0000000000000000, // Sturb
    0x0000000000000000, // Sturh
    0x0000000000000000, // Stxp
    0x0000000000000000, // Stxr
    0x0000000000000000, // Stxrb
    0x0000000000000000, // Stxrh
    0x0000000000200000, // Stz2g
    0x0000000000200000, // Stzg
    0x0000000000400000, // Stzgm
    0x0000000000000000, // Sub
    0x0000000000200000, // Subg
    0x0000000000200000, // Subp
    0x0000000000200000, // Subps
    0x0000000000000000, // Subs
    0x0000000000000000, // Svc
    0x0000000000100000, // Swp
    0x0000000000100000, // Swpa
    0x0000000000100000, // Swpab
    0x0000000000100000, // Swpah
    0x0000000000100000, // Swpal
    0x0000000000100000, // Swpalb
    0x0000000000100000, // Swpalh
    0x0000000000100000, // Swpb
    0x0000000000100000, // Swph
    0x0000000000100000, // Swpl
    0x0000000000100000, // Swplb
    0x0000000000100000, // Swplh
    0x0000000000000000, // Sxtb
    0x0000000000000000, // Sxth
    0x0000000000000000, // Sxtw
    0x0000000000000000, // Sys
    0x0000000000000000, // Tlbi
    0x0000000000000000, // Tst
    0x0000000000000000, // Tbnz
    0x0000000000000000, // Tbz
    0x0000000000000000, // Ubfiz
    0x0000000000000000, // Ubfm
    0x0000000000000000, // Ubfx
    0x0000000000000000, // Udf
    0x0000000000000000, // Udiv
    0x0000000000000000, // Umaddl
    0x0000000000000080, // Umax
    0x0000000000000080, // Umin
    0x0000000000000000, // Umnegl
    0x0000000000000000, // Umull
    0x0000000000000000, // Umulh
    0x0000000000000000, // Umsubl
    0x0000000000000000, // Uxtb
    0x0000000000000000, // Uxth
    0x0000000000000000, // Wfe
    0x0000000000000000, // Wfi
    0x0000000000002000, // Xaflag
    0x0000000000800000, // Xpacd
    0x0000000000800000, // Xpaci
    0x0000000000800000, // Xpaclri
    0x0000000000000000, // Yield
    0x0000000000000002, // Abs_v
    0x0000000000000002, // Add_v
    0x0000000000000002, // Addhn_v
    0x0000000000000002, // Addhn2_v
    0x0000000000000002, // Addp_v
    0x0000000000000002, // Addv_v
    0x0000000000000003, // Aesd_v
    0x0000000000000003, // Aese_v
    0x0000000000000003, // Aesimc_v
    0x0000000000000003, // Aesmc_v
    0x0000000000000002, // And_v
    0x0000000010000002, // Bcax_v
    0x0000000000000006, // Bfcvt_v
    0x0000000000000006, // Bfcvtn_v
    0x0000000000000006, // Bfcvtn2_v
    0x0000000000000006, // Bfdot_v
    0x0000000000000006, // Bfmlalb_v
    0x0000000000000006, // Bfmlalt_v
    0x0000000000000006, // Bfmmla_v
    0x0000000000000002, // Bic_v
    0x0000000000000002, // Bif_v
    0x0000000000000002, // Bit_v
    0x0000000000000002, // Bsl_v
    0x0000000000000002, // Cls_v
    0x0000000000000002, // Clz_v
    0x0000000000000002, // Cmeq_v
    0x0000000000000002, // Cmge_v
    0x0000000000000002, // Cmgt_v
    0x0000000000000002, // Cmhi_v
    0x0000000000000002, // Cmhs_v
    0x0000000000000002, // Cmle_v
    0x0000000000000002, // Cmlt_v
    0x0000000000000002, // Cmtst_v
    0x0000000000000002, // Cnt_v
    0x0000000000000002, // Dup_v
    0x0000000000000002, // Eor_v
    0x0000000010000002, // Eor3_v
    0x0000000000000002, // Ext_v
    0x0000000000000002, // Fabd_v
    0x0000000000000002, // Fabs_v
    0x0000000000000002, // Facge_v
    0x0000000000000002, // Facgt_v
    0x0000000000000002, // Fadd_v
    0x0000000000000002, // Faddp_v
    0x0000000000000402, // Fcadd_v
    0x0000000000000002, // Fccmp_v
    0x0000000000000002, // Fccmpe_v
    0x0000000000000002, // Fcmeq_v
    0x0000000000000002, // Fcmge_v
    0x0000000000000002, // Fcmgt_v
    0x0000000000000402, // Fcmla_v
    0x0000000000000002, // Fcmle_v
    0x0000000000000002, // Fcmlt_v
    0x0000000000000002, // Fcmp_v
    0x0000000000000002, // Fcmpe_v
    0x0000000000000002, // Fcsel_v
    0x0000000000000002, // Fcvt_v
    0x0000000000000002, // Fcvtas_v
    0x0000000000000002, // Fcvtau_v
    0x0000000000000002, // Fcvtl_v
    0x0000000000000002, // Fcvtl2_v
    0x0000000000000002, // Fcvtms_v
    0x0000000000000002, // Fcvtmu_v
    0x0000000000000002, // Fcvtn_v
    0x0000000000000002, // Fcvtn2_v
    0x0000000000000002, // Fcvtns_v
    0x0000000000000002, // Fcvtnu_v
    0x0000000000000002, // Fcvtps_v
    0x0000000000000002, // Fcvtpu_v
    0x0000000000000000, // Fcvtxn_v
    0x0000000000000000, // Fcvtxn2_v
    0x0000000000000002, // Fcvtzs_v
    0x0000000000000002, // Fcvtzu_v
    0x0000000000000002, // Fdiv_v
    0x0000000000040002, // Fjcvtzs_v
    0x0000000000000002, // Fmadd_v
    0x0000000000000002, // Fmax_v
    0x0000000000000002, // Fmaxnm_v
    0x0000000000000002, // Fmaxnmp_v
    0x0000000000000002, // Fmaxnmv_v
    0x0000000000000002, // Fmaxp_v
    0x0000000000000002, // Fmaxv_v
    0x0000000000000002, // Fmin_v
    0x0000000000000002, // Fminnm_v
    0x0000000000000002, // Fminnmp_v
    0x0000000000000002, // Fminnmv_v
    0x0000000000000002, // Fminp_v
    0x0000000000000002, // Fminv_v
    0x0000000000000002, // Fmla_v
    0x0000000000000802, // Fmlal_v
    0x0000000000000802, // Fmlal2_v
    0x0000000000000002, // Fmls_v
    0x0000000000000802, // Fmlsl_v
    0x0000000000000802, // Fmlsl2_v
    0x0000000000000002, // Fmov_v
    0x0000000000000002, // Fmsub_v
    0x0000000000000002, // Fmul_v
    0x0000000000000002, // Fmulx_v
    0x0000000000000002, // Fneg_v
    0x0000000000000002, // Fnmadd_v
    0x0000000000000002, // Fnmsub_v
    0x0000000000000002, // Fnmul_v
    0x0000000000000002, // Frecpe_v
    0x0000000000000002, // Frecps_v
    0x0000000000000002, // Frecpx_v
    0x0000000000010002, // Frint32x_v
    0x0000000000010002, // Frint32z_v
    0x0000000000010002, // Frint64x_v
    0x0000000000010002, // Frint64z_v
    0x0000000000000002, // Frinta_v
    0x0000000000000002, // Frinti_v
    0x0000000000000002, // Frintm_v
    0x0000000000000002, // Frintn_v
    0x0000000000000002, // Frintp_v
    0x0000000000000002, // Frintx_v
    0x0000000000000002, // Frintz_v
    0x0000000000000002, // Frsqrte_v
    0x0000000000000002, // Frsqrts_v
    0x0000000000000002, // Fsqrt_v
    0x0000000000000002, // Fsub_v
    0x0000000000000002, // Ins_v
    0x0000000000000002, // Ld1_v
    0x0000000000000002, // Ld1r_v
    0x0000000000000002, // Ld2_v
    0x0000000000000002, // Ld2r_v
    0x0000000000000002, // Ld3_v
    0x0000000000000002, // Ld3r_v
    0x0000000000000002, // Ld4_v
    0x0000000000000002, // Ld4r_v
    0x0000000000000002, // Ldnp_v
    0x0000000000000002, // Ldp_v
    0x0000000000000002, // Ldr_v
    0x0000000000000002, // Ldur_v
    0x0000000000000002, // Mla_v
    0x0000000000000002, // Mls_v
    0x0000000000000002, // Mov_v
    0x0000000000000002, // Movi_v
    0x0000000000000002, // Mul_v
    0x0000000000000002, // Mvn_v
    0x0000000000000002, // Mvni_v
    0x0000000000000002, // Neg_v
    0x0000000000000002, // Not_v
    0x0000000000000002, // Orn_v
    0x0000000000000002, // Orr_v
    0x0000000000000002, // Pmul_v
    0x0000000000000002, // Pmull_v
    0x0000000000000002, // Pmull2_v
    0x0000000000000002, // Raddhn_v
    0x0000000000000002, // Raddhn2_v
    0x0000000010000002, // Rax1_v
    0x0000000000000002, // Rbit_v
    0x0000000000000002, // Rev16_v
    0x0000000000000002, // Rev32_v
    0x0000000000000002, // Rev64_v
    0x0000000000000002, // Rshrn_v
    0x0000000000000002, // Rshrn2_v
    0x0000000000000002, // Rsubhn_v
    0x0000000000000002, // Rsubhn2_v
    0x0000000000000002, // Saba_v
    0x0000000000000002, // Sabal_v
    0x0000000000000002, // Sabal2_v
    0x0000000000000002, // Sabd_v
    0x0000000000000002, // Sabdl_v
    0x0000000000000002, // Sabdl2_v
    0x0000000000000002, // Sadalp_v
    0x0000000000000002, // Saddl_v
    0x0000000000000002, // Saddl2_v
    0x0000000000000002, // Saddlp_v
    0x0000000000000002, // Saddlv_v
    0x0000000000000002, // Saddw_v
    0x0000000000000002, // Saddw2_v
    0x0000000000000002, // Scvtf_v
    0x0000000000000202, // Sdot_v
    0x0000000004000002, // Sha1c_v
    0x0000000004000002, // Sha1h_v
    0x0000000004000002, // Sha1m_v
    0x0000000004000002, // Sha1p_v
    0x0000000004000002, // Sha1su0_v
    0x0000000004000002, // Sha1su1_v
    0x0000000008000002, // Sha256h_v
    0x0000000008000002, // Sha256h2_v
    0x0000000008000002, // Sha256su0_v
    0x0000000008000002, // Sha256su1_v
    0x0000000020000002, // Sha512h_v
    0x0000000020000002, // Sha512h2_v
    0x0000000020000002, // Sha512su0_v
    0x0000000020000002, // Sha512su1_v
    0x0000000000000002, // Shadd_v
    0x0000000000000002, // Shl_v
    0x0000000000000002, // Shll_v
    0x0000000000000002, // Shll2_v
    0x0000000000000002, // Shrn_v
    0x0000000000000002, // Shrn2_v
    0x0000000000000002, // Shsub_v
    0x0000000000000002, // Sli_v
    0x0000000040000002, // Sm3partw1_v
    0x0000000040000002, // Sm3partw2_v
    0x0000000040000002, // Sm3ss1_v
    0x0000000040000002, // Sm3tt1a_v
    0x0000000040000002, // Sm3tt1b_v
    0x0000000040000002, // Sm3tt2a_v
    0x0000000040000002, // Sm3tt2b_v
    0x0000000080000002, // Sm4e_v
    0x0000000080000002, // Sm4ekey_v
    0x0000000000000002, // Smax_v
    0x0000000000000002, // Smaxp_v
    0x0000000000000002, // Smaxv_v
    0x0000000000000002, // Smin_v
    0x0000000000000002, // Sminp_v
    0x0000000000000002, // Sminv_v
    0x0000000000000002, // Smlal_v
    0x0000000000000002, // Smlal2_v
    0x0000000000000002, // Smlsl_v
    0x0000000000000002, // Smlsl2_v
    0x0000000000020002, // Smmla_v
    0x0000000000000002, // Smov_v
    0x0000000000000002, // Smull_v
    0x0000000000000002, // Smull2_v
    0x0000000000000002, // Sqabs_v
    0x0000000000000002, // Sqadd_v
    0x0000000000000002, // Sqdmlal_v
    0x0000000000000002, // Sqdmlal2_v
    0x0000000000000002, // Sqdmlsl_v
    0x0000000000000002, // Sqdmlsl2_v
    0x0000000000000002, // Sqdmulh_v
    0x0000000000000002, // Sqdmull_v
    0x0000000000000002, // Sqdmull2_v
    0x0000000000000002, // Sqneg_v
    0x0000000002000002, // Sqrdmlah_v
    0x0000000002000002, // Sqrdmlsh_v
    0x0000000000000002, // Sqrdmulh_v
    0x0000000000000002, // Sqrshl_v
    0x0000000000000002, // Sqrshrn_v
    0x0000000000000002, // Sqrshrn2_v
    0x0000000000000002, // Sqrshrun_v
    0x0000000000000002, // Sqrshrun2_v
    0x0000000000000002, // Sqshl_v
    0x0000000000000002, // Sqshlu_v
    0x0000000000000002, // Sqshrn_v
    0x0000000000000002, // Sqshrn2_v
    0x0000000000000002, // Sqshrun_v
    0x0000000000000002, // Sqshrun2_v
    0x0000000000000002, // Sqsub_v
    0x0000000000000002, // Sqxtn_v
    0x0000000000000002, // Sqxtn2_v
    0x0000000000000002, // Sqxtun_v
    0x0000000000000002, // Sqxtun2_v
    0x0000000000000002, // Srhadd_v
    0x0000000000000002, // Sri_v
    0x0000000000000002, // Srshl_v
    0x0000000000000002, // Srshr_v
    0x0000000000000002, // Srsra_v
    0x0000000000000002, // Sshl_v
    0x0000000000000002, // Sshll_v
    0x0000000000000002, // Sshll2_v
    0x0000000000000002, // Sshr_v
    0x0000000000000002, // Ssra_v
    0x0000000000000002, // Ssubl_v
    0x0000000000000002, // Ssubl2_v
    0x0000000000000002, // Ssubw_v
    0x0000000000000002, // Ssubw2_v
    0x0000000000000002, // St1_v
    0x0000000000000002, // St2_v
    0x0000000000000002, // St3_v
    0x0000000000000002, // St4_v
    0x0000000000000002, // Stnp_v
    0x0000000000000002, // Stp_v
    0x0000000000000002, // Str_v
    0x0000000000000002, // Stur_v
    0x0000000000000002, // Sub_v
    0x0000000000000002, // Subhn_v
    0x0000000000000002, // Subhn2_v
    0x0000000000020002, // Sudot_v
    0x0000000000000002, // Suqadd_v
    0x0000000000000002, // Sxtl_v
    0x0000000000000002, // Sxtl2_v
    0x0000000000000002, // Tbl_v
    0x0000000000000002, // Tbx_v
    0x0000000000000002, // Trn1_v
    0x0000000000000002, // Trn2_v
    0x0000000000000002, // Uaba_v
    0x0000000000000002, // Uabal_v
    0x0000000000000002, // Uabal2_v
    0x0000000000000002, // Uabd_v
    0x0000000000000002, // Uabdl_v
    0x0000000000000002, // Uabdl2_v
    0x0000000000000002, // Uadalp_v
    0x0000000000000002, // Uaddl_v
    0x0000000000000002, // Uaddl2_v
    0x0000000000000002, // Uaddlp_v
    0x0000000000000002, // Uaddlv_v
    0x0000000000000002, // Uaddw_v
    0x0000000000000002, // Uaddw2_v
    0x0000000000000002, // Ucvtf_v
    0x0000000000000202, // Udot_v
    0x0000000000000002, // Uhadd_v
    0x0000000000000002, // Uhsub_v
    0x0000000000000002, // Umax_v
    0x0000000000000002, // Umaxp_v
    0x0000000000000002, // Umaxv_v
    0x0000000000000002, // Umin_v
    0x0000000000000002, // Uminp_v
    0x0000000000000002, // Uminv_v
    0x0000000000000002, // Umlal_v
    0x0000000000000002, // Umlal2_v
    0x0000000000000002, // Umlsl_v
    0x0000000000000002, // Umlsl2_v
    0x0000000000020002, // Ummla_v
    0x0000000000000002, // Umov_v
    0x0000000000000002, // Umull_v
    0x0000000000000002, // Umull2_v
    0x0000000000000002, // Uqadd_v
    0x0000000000000002, // Uqrshl_v
    0x0000000000000002, // Uqrshrn_v
    0x0000000000000002, // Uqrshrn2_v
    0x0000000000000002, // Uqshl_v
    0x0000000000000002, // Uqshrn_v
    0x0000000000000002, // Uqshrn2_v
    0x0000000000000002, // Uqsub_v
    0x0000000000000002, // Uqxtn_v
    0x0000000000000002, // Uqxtn2_v
    0x0000000000000002, // Urecpe_v
    0x0000000000000002, // Urhadd_v
    0x0000000000000002, // Urshl_v
    0x0000000000000002, // Urshr_v
    0x0000000000000002, // Ursqrte_v
    0x0000000000000002, // Ursra_v
    0x0000000000020002, // Usdot_v
    0x0000000000000002, // Ushl_v
    0x0000000000000002, // Ushll_v
    0x0000000000000002, // Ushll2_v
    0x0000000000000002, // Ushr_v
    0x0000000000020002, // Usmmla_v
    0x0000000000000002, // Usqadd_v
    0x0000000000000002, // Usra_v
    0x0000000000000002, // Usubl_v
    0x0000000000000002, // Usubl2_v
    0x0000000000000002, // Usubw_v
    0x0000000000000002, // Usubw2_v
    0x0000000000000002, // Uxtl_v
    0x0000000000000002, // Uxtl2_v
    0x0000000000000002, // Uzp1_v
    0x0000000000000002, // Uzp2_v
    0x0000000010000002, // Xar_v
    0x0000000000000002, // Xtn_v
    0x0000000000000002, // Xtn2_v
    0x0000000000000002, // Zip1_v
    0x0000000000000002, // Zip2_v
];

static INST_BASE_FEATURE_CONTEXT: [&str; InstId::_Count as usize] = [
    "",
    "abs requires: CSSC",
    "",
    "",
    "",
    "addg requires: MTE",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "autda requires: PAUTH",
    "autdza requires: PAUTH",
    "autdb requires: PAUTH",
    "autdzb requires: PAUTH",
    "autia requires: PAUTH",
    "autia1716 requires: PAUTH",
    "autiasp requires: PAUTH",
    "autiaz requires: PAUTH",
    "autib requires: PAUTH",
    "autib1716 requires: PAUTH",
    "autibsp requires: PAUTH",
    "autibz requires: PAUTH",
    "autiza requires: PAUTH",
    "autizb requires: PAUTH",
    "axflag requires: FLAGM2",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "bti requires: BTI",
    "cas requires: LSE",
    "casa requires: LSE",
    "casab requires: LSE",
    "casah requires: LSE",
    "casal requires: LSE",
    "casalb requires: LSE",
    "casalh requires: LSE",
    "casb requires: LSE",
    "cash requires: LSE",
    "casl requires: LSE",
    "caslb requires: LSE",
    "caslh requires: LSE",
    "casp requires: LSE",
    "caspa requires: LSE",
    "caspal requires: LSE",
    "caspl requires: LSE",
    "",
    "",
    "",
    "",
    "cfinv requires: FLAGM",
    "chkfeat requires: CHK",
    "",
    "",
    "clrbhb requires: CLRBHB",
    "",
    "",
    "",
    "",
    "",
    "cmpp requires: MTE",
    "",
    "cnt requires: CSSC",
    "crc32b requires: CRC32",
    "crc32cb requires: CRC32",
    "crc32ch requires: CRC32",
    "crc32cw requires: CRC32",
    "crc32cx requires: CRC32",
    "crc32h requires: CRC32",
    "crc32w requires: CRC32",
    "crc32x requires: CRC32",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "ctz requires: CSSC",
    "",
    "",
    "",
    "",
    "dgh requires: DGH",
    "",
    "",
    "",
    "",
    "",
    "esb requires: RAS",
    "",
    "",
    "gmi requires: MTE",
    "",
    "",
    "",
    "",
    "",
    "ldadd requires: LSE",
    "ldadda requires: LSE",
    "ldaddab requires: LSE",
    "ldaddah requires: LSE",
    "ldaddal requires: LSE",
    "ldaddalb requires: LSE",
    "ldaddalh requires: LSE",
    "ldaddb requires: LSE",
    "ldaddh requires: LSE",
    "ldaddl requires: LSE",
    "ldaddlb requires: LSE",
    "ldaddlh requires: LSE",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "ldclr requires: LSE",
    "ldclra requires: LSE",
    "ldclrab requires: LSE",
    "ldclrah requires: LSE",
    "ldclral requires: LSE",
    "ldclralb requires: LSE",
    "ldclralh requires: LSE",
    "ldclrb requires: LSE",
    "ldclrh requires: LSE",
    "ldclrl requires: LSE",
    "ldclrlb requires: LSE",
    "ldclrlh requires: LSE",
    "ldeor requires: LSE",
    "ldeora requires: LSE",
    "ldeorab requires: LSE",
    "ldeorah requires: LSE",
    "ldeoral requires: LSE",
    "ldeoralb requires: LSE",
    "ldeoralh requires: LSE",
    "ldeorb requires: LSE",
    "ldeorh requires: LSE",
    "ldeorl requires: LSE",
    "ldeorlb requires: LSE",
    "ldeorlh requires: LSE",
    "ldg requires: MTE",
    "ldgm requires: MTE2",
    "ldlar requires: LOR",
    "ldlarb requires: LOR",
    "ldlarh requires: LOR",
    "",
    "",
    "",
    "",
    "ldraa requires: PAUTH",
    "ldrab requires: PAUTH",
    "",
    "",
    "",
    "",
    "",
    "ldset requires: LSE",
    "ldseta requires: LSE",
    "ldsetab requires: LSE",
    "ldsetah requires: LSE",
    "ldsetal requires: LSE",
    "ldsetalb requires: LSE",
    "ldsetalh requires: LSE",
    "ldsetb requires: LSE",
    "ldseth requires: LSE",
    "ldsetl requires: LSE",
    "ldsetlb requires: LSE",
    "ldsetlh requires: LSE",
    "ldsmax requires: LSE",
    "ldsmaxa requires: LSE",
    "ldsmaxab requires: LSE",
    "ldsmaxah requires: LSE",
    "ldsmaxal requires: LSE",
    "ldsmaxalb requires: LSE",
    "ldsmaxalh requires: LSE",
    "ldsmaxb requires: LSE",
    "ldsmaxh requires: LSE",
    "ldsmaxl requires: LSE",
    "ldsmaxlb requires: LSE",
    "ldsmaxlh requires: LSE",
    "ldsmin requires: LSE",
    "ldsmina requires: LSE",
    "ldsminab requires: LSE",
    "ldsminah requires: LSE",
    "ldsminal requires: LSE",
    "ldsminalb requires: LSE",
    "ldsminalh requires: LSE",
    "ldsminb requires: LSE",
    "ldsminh requires: LSE",
    "ldsminl requires: LSE",
    "ldsminlb requires: LSE",
    "ldsminlh requires: LSE",
    "",
    "",
    "",
    "",
    "",
    "",
    "ldumax requires: LSE",
    "ldumaxa requires: LSE",
    "ldumaxab requires: LSE",
    "ldumaxah requires: LSE",
    "ldumaxal requires: LSE",
    "ldumaxalb requires: LSE",
    "ldumaxalh requires: LSE",
    "ldumaxb requires: LSE",
    "ldumaxh requires: LSE",
    "ldumaxl requires: LSE",
    "ldumaxlb requires: LSE",
    "ldumaxlh requires: LSE",
    "ldumin requires: LSE",
    "ldumina requires: LSE",
    "lduminab requires: LSE",
    "lduminah requires: LSE",
    "lduminal requires: LSE",
    "lduminalb requires: LSE",
    "lduminalh requires: LSE",
    "lduminb requires: LSE",
    "lduminh requires: LSE",
    "lduminl requires: LSE",
    "lduminlb requires: LSE",
    "lduminlh requires: LSE",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "pacda requires: PAUTH",
    "pacdb requires: PAUTH",
    "pacdza requires: PAUTH",
    "pacdzb requires: PAUTH",
    "pacga requires: PAUTH",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "setf8 requires: FLAGM",
    "setf16 requires: FLAGM",
    "",
    "",
    "",
    "smax requires: CSSC",
    "",
    "smin requires: CSSC",
    "",
    "",
    "",
    "",
    "",
    "st2g requires: MTE",
    "stadd requires: LSE",
    "staddl requires: LSE",
    "staddb requires: LSE",
    "staddlb requires: LSE",
    "staddh requires: LSE",
    "staddlh requires: LSE",
    "stclr requires: LSE",
    "stclrl requires: LSE",
    "stclrb requires: LSE",
    "stclrlb requires: LSE",
    "stclrh requires: LSE",
    "stclrlh requires: LSE",
    "steor requires: LSE",
    "steorl requires: LSE",
    "steorb requires: LSE",
    "steorlb requires: LSE",
    "steorh requires: LSE",
    "steorlh requires: LSE",
    "stg requires: MTE",
    "stgm requires: MTE2",
    "stgp requires: MTE",
    "stllr requires: LOR",
    "stllrb requires: LOR",
    "stllrh requires: LOR",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "stset requires: LSE",
    "stsetl requires: LSE",
    "stsetb requires: LSE",
    "stsetlb requires: LSE",
    "stseth requires: LSE",
    "stsetlh requires: LSE",
    "stsmax requires: LSE",
    "stsmaxl requires: LSE",
    "stsmaxb requires: LSE",
    "stsmaxlb requires: LSE",
    "stsmaxh requires: LSE",
    "stsmaxlh requires: LSE",
    "stsmin requires: LSE",
    "stsminl requires: LSE",
    "stsminb requires: LSE",
    "stsminlb requires: LSE",
    "stsminh requires: LSE",
    "stsminlh requires: LSE",
    "",
    "",
    "",
    "stumax requires: LSE",
    "stumaxl requires: LSE",
    "stumaxb requires: LSE",
    "stumaxlb requires: LSE",
    "stumaxh requires: LSE",
    "stumaxlh requires: LSE",
    "stumin requires: LSE",
    "stuminl requires: LSE",
    "stuminb requires: LSE",
    "stuminlb requires: LSE",
    "stuminh requires: LSE",
    "stuminlh requires: LSE",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "stz2g requires: MTE",
    "stzg requires: MTE",
    "stzgm requires: MTE2",
    "",
    "subg requires: MTE",
    "subp requires: MTE",
    "subps requires: MTE",
    "",
    "",
    "swp requires: LSE",
    "swpa requires: LSE",
    "swpab requires: LSE",
    "swpah requires: LSE",
    "swpal requires: LSE",
    "swpalb requires: LSE",
    "swpalh requires: LSE",
    "swpb requires: LSE",
    "swph requires: LSE",
    "swpl requires: LSE",
    "swplb requires: LSE",
    "swplh requires: LSE",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "umax requires: CSSC",
    "umin requires: CSSC",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "xaflag requires: FLAGM2",
    "xpacd requires: PAUTH",
    "xpaci requires: PAUTH",
    "xpaclri requires: PAUTH",
    "",
    "abs requires: ASIMD",
    "add requires: ASIMD",
    "addhn requires: ASIMD",
    "addhn2 requires: ASIMD",
    "addp requires: ASIMD",
    "addv requires: ASIMD",
    "aesd requires: AES, ASIMD",
    "aese requires: AES, ASIMD",
    "aesimc requires: AES, ASIMD",
    "aesmc requires: AES, ASIMD",
    "and requires: ASIMD",
    "bcax requires: ASIMD, SHA3",
    "bfcvt requires: ASIMD, BF16",
    "bfcvtn requires: ASIMD, BF16",
    "bfcvtn2 requires: ASIMD, BF16",
    "bfdot requires: ASIMD, BF16",
    "bfmlalb requires: ASIMD, BF16",
    "bfmlalt requires: ASIMD, BF16",
    "bfmmla requires: ASIMD, BF16",
    "bic requires: ASIMD",
    "bif requires: ASIMD",
    "bit requires: ASIMD",
    "bsl requires: ASIMD",
    "cls requires: ASIMD",
    "clz requires: ASIMD",
    "cmeq requires: ASIMD",
    "cmge requires: ASIMD",
    "cmgt requires: ASIMD",
    "cmhi requires: ASIMD",
    "cmhs requires: ASIMD",
    "cmle requires: ASIMD",
    "cmlt requires: ASIMD",
    "cmtst requires: ASIMD",
    "cnt requires: ASIMD",
    "dup requires: ASIMD",
    "eor requires: ASIMD",
    "eor3 requires: ASIMD, SHA3",
    "ext requires: ASIMD",
    "fabd requires: ASIMD",
    "fabs requires: ASIMD",
    "facge requires: ASIMD",
    "facgt requires: ASIMD",
    "fadd requires: ASIMD",
    "faddp requires: ASIMD",
    "fcadd requires: ASIMD, FCMA",
    "fccmp requires: ASIMD",
    "fccmpe requires: ASIMD",
    "fcmeq requires: ASIMD",
    "fcmge requires: ASIMD",
    "fcmgt requires: ASIMD",
    "fcmla requires: ASIMD, FCMA",
    "fcmle requires: ASIMD",
    "fcmlt requires: ASIMD",
    "fcmp requires: ASIMD",
    "fcmpe requires: ASIMD",
    "fcsel requires: ASIMD",
    "fcvt requires: ASIMD",
    "fcvtas requires: ASIMD",
    "fcvtau requires: ASIMD",
    "fcvtl requires: ASIMD",
    "fcvtl2 requires: ASIMD",
    "fcvtms requires: ASIMD",
    "fcvtmu requires: ASIMD",
    "fcvtn requires: ASIMD",
    "fcvtn2 requires: ASIMD",
    "fcvtns requires: ASIMD",
    "fcvtnu requires: ASIMD",
    "fcvtps requires: ASIMD",
    "fcvtpu requires: ASIMD",
    "",
    "",
    "fcvtzs requires: ASIMD",
    "fcvtzu requires: ASIMD",
    "fdiv requires: ASIMD",
    "fjcvtzs requires: ASIMD, JSCVT",
    "fmadd requires: ASIMD",
    "fmax requires: ASIMD",
    "fmaxnm requires: ASIMD",
    "fmaxnmp requires: ASIMD",
    "fmaxnmv requires: ASIMD",
    "fmaxp requires: ASIMD",
    "fmaxv requires: ASIMD",
    "fmin requires: ASIMD",
    "fminnm requires: ASIMD",
    "fminnmp requires: ASIMD",
    "fminnmv requires: ASIMD",
    "fminp requires: ASIMD",
    "fminv requires: ASIMD",
    "fmla requires: ASIMD",
    "fmlal requires: ASIMD, FHM",
    "fmlal2 requires: ASIMD, FHM",
    "fmls requires: ASIMD",
    "fmlsl requires: ASIMD, FHM",
    "fmlsl2 requires: ASIMD, FHM",
    "fmov requires: ASIMD",
    "fmsub requires: ASIMD",
    "fmul requires: ASIMD",
    "fmulx requires: ASIMD",
    "fneg requires: ASIMD",
    "fnmadd requires: ASIMD",
    "fnmsub requires: ASIMD",
    "fnmul requires: ASIMD",
    "frecpe requires: ASIMD",
    "frecps requires: ASIMD",
    "frecpx requires: ASIMD",
    "frint32x requires: ASIMD, FRINTTS",
    "frint32z requires: ASIMD, FRINTTS",
    "frint64x requires: ASIMD, FRINTTS",
    "frint64z requires: ASIMD, FRINTTS",
    "frinta requires: ASIMD",
    "frinti requires: ASIMD",
    "frintm requires: ASIMD",
    "frintn requires: ASIMD",
    "frintp requires: ASIMD",
    "frintx requires: ASIMD",
    "frintz requires: ASIMD",
    "frsqrte requires: ASIMD",
    "frsqrts requires: ASIMD",
    "fsqrt requires: ASIMD",
    "fsub requires: ASIMD",
    "ins requires: ASIMD",
    "ld1 requires: ASIMD",
    "ld1r requires: ASIMD",
    "ld2 requires: ASIMD",
    "ld2r requires: ASIMD",
    "ld3 requires: ASIMD",
    "ld3r requires: ASIMD",
    "ld4 requires: ASIMD",
    "ld4r requires: ASIMD",
    "ldnp requires: ASIMD",
    "ldp requires: ASIMD",
    "ldr requires: ASIMD",
    "ldur requires: ASIMD",
    "mla requires: ASIMD",
    "mls requires: ASIMD",
    "mov requires: ASIMD",
    "movi requires: ASIMD",
    "mul requires: ASIMD",
    "mvn requires: ASIMD",
    "mvni requires: ASIMD",
    "neg requires: ASIMD",
    "not requires: ASIMD",
    "orn requires: ASIMD",
    "orr requires: ASIMD",
    "pmul requires: ASIMD",
    "pmull requires: ASIMD",
    "pmull2 requires: ASIMD",
    "raddhn requires: ASIMD",
    "raddhn2 requires: ASIMD",
    "rax1 requires: ASIMD, SHA3",
    "rbit requires: ASIMD",
    "rev16 requires: ASIMD",
    "rev32 requires: ASIMD",
    "rev64 requires: ASIMD",
    "rshrn requires: ASIMD",
    "rshrn2 requires: ASIMD",
    "rsubhn requires: ASIMD",
    "rsubhn2 requires: ASIMD",
    "saba requires: ASIMD",
    "sabal requires: ASIMD",
    "sabal2 requires: ASIMD",
    "sabd requires: ASIMD",
    "sabdl requires: ASIMD",
    "sabdl2 requires: ASIMD",
    "sadalp requires: ASIMD",
    "saddl requires: ASIMD",
    "saddl2 requires: ASIMD",
    "saddlp requires: ASIMD",
    "saddlv requires: ASIMD",
    "saddw requires: ASIMD",
    "saddw2 requires: ASIMD",
    "scvtf requires: ASIMD",
    "sdot requires: ASIMD, DOTPROD",
    "sha1c requires: ASIMD, SHA1",
    "sha1h requires: ASIMD, SHA1",
    "sha1m requires: ASIMD, SHA1",
    "sha1p requires: ASIMD, SHA1",
    "sha1su0 requires: ASIMD, SHA1",
    "sha1su1 requires: ASIMD, SHA1",
    "sha256h requires: ASIMD, SHA256",
    "sha256h2 requires: ASIMD, SHA256",
    "sha256su0 requires: ASIMD, SHA256",
    "sha256su1 requires: ASIMD, SHA256",
    "sha512h requires: ASIMD, SHA512",
    "sha512h2 requires: ASIMD, SHA512",
    "sha512su0 requires: ASIMD, SHA512",
    "sha512su1 requires: ASIMD, SHA512",
    "shadd requires: ASIMD",
    "shl requires: ASIMD",
    "shll requires: ASIMD",
    "shll2 requires: ASIMD",
    "shrn requires: ASIMD",
    "shrn2 requires: ASIMD",
    "shsub requires: ASIMD",
    "sli requires: ASIMD",
    "sm3partw1 requires: ASIMD, SM3",
    "sm3partw2 requires: ASIMD, SM3",
    "sm3ss1 requires: ASIMD, SM3",
    "sm3tt1a requires: ASIMD, SM3",
    "sm3tt1b requires: ASIMD, SM3",
    "sm3tt2a requires: ASIMD, SM3",
    "sm3tt2b requires: ASIMD, SM3",
    "sm4e requires: ASIMD, SM4",
    "sm4ekey requires: ASIMD, SM4",
    "smax requires: ASIMD",
    "smaxp requires: ASIMD",
    "smaxv requires: ASIMD",
    "smin requires: ASIMD",
    "sminp requires: ASIMD",
    "sminv requires: ASIMD",
    "smlal requires: ASIMD",
    "smlal2 requires: ASIMD",
    "smlsl requires: ASIMD",
    "smlsl2 requires: ASIMD",
    "smmla requires: ASIMD, I8MM",
    "smov requires: ASIMD",
    "smull requires: ASIMD",
    "smull2 requires: ASIMD",
    "sqabs requires: ASIMD",
    "sqadd requires: ASIMD",
    "sqdmlal requires: ASIMD",
    "sqdmlal2 requires: ASIMD",
    "sqdmlsl requires: ASIMD",
    "sqdmlsl2 requires: ASIMD",
    "sqdmulh requires: ASIMD",
    "sqdmull requires: ASIMD",
    "sqdmull2 requires: ASIMD",
    "sqneg requires: ASIMD",
    "sqrdmlah requires: ASIMD, RDM",
    "sqrdmlsh requires: ASIMD, RDM",
    "sqrdmulh requires: ASIMD",
    "sqrshl requires: ASIMD",
    "sqrshrn requires: ASIMD",
    "sqrshrn2 requires: ASIMD",
    "sqrshrun requires: ASIMD",
    "sqrshrun2 requires: ASIMD",
    "sqshl requires: ASIMD",
    "sqshlu requires: ASIMD",
    "sqshrn requires: ASIMD",
    "sqshrn2 requires: ASIMD",
    "sqshrun requires: ASIMD",
    "sqshrun2 requires: ASIMD",
    "sqsub requires: ASIMD",
    "sqxtn requires: ASIMD",
    "sqxtn2 requires: ASIMD",
    "sqxtun requires: ASIMD",
    "sqxtun2 requires: ASIMD",
    "srhadd requires: ASIMD",
    "sri requires: ASIMD",
    "srshl requires: ASIMD",
    "srshr requires: ASIMD",
    "srsra requires: ASIMD",
    "sshl requires: ASIMD",
    "sshll requires: ASIMD",
    "sshll2 requires: ASIMD",
    "sshr requires: ASIMD",
    "ssra requires: ASIMD",
    "ssubl requires: ASIMD",
    "ssubl2 requires: ASIMD",
    "ssubw requires: ASIMD",
    "ssubw2 requires: ASIMD",
    "st1 requires: ASIMD",
    "st2 requires: ASIMD",
    "st3 requires: ASIMD",
    "st4 requires: ASIMD",
    "stnp requires: ASIMD",
    "stp requires: ASIMD",
    "str requires: ASIMD",
    "stur requires: ASIMD",
    "sub requires: ASIMD",
    "subhn requires: ASIMD",
    "subhn2 requires: ASIMD",
    "sudot requires: ASIMD, I8MM",
    "suqadd requires: ASIMD",
    "sxtl requires: ASIMD",
    "sxtl2 requires: ASIMD",
    "tbl requires: ASIMD",
    "tbx requires: ASIMD",
    "trn1 requires: ASIMD",
    "trn2 requires: ASIMD",
    "uaba requires: ASIMD",
    "uabal requires: ASIMD",
    "uabal2 requires: ASIMD",
    "uabd requires: ASIMD",
    "uabdl requires: ASIMD",
    "uabdl2 requires: ASIMD",
    "uadalp requires: ASIMD",
    "uaddl requires: ASIMD",
    "uaddl2 requires: ASIMD",
    "uaddlp requires: ASIMD",
    "uaddlv requires: ASIMD",
    "uaddw requires: ASIMD",
    "uaddw2 requires: ASIMD",
    "ucvtf requires: ASIMD",
    "udot requires: ASIMD, DOTPROD",
    "uhadd requires: ASIMD",
    "uhsub requires: ASIMD",
    "umax requires: ASIMD",
    "umaxp requires: ASIMD",
    "umaxv requires: ASIMD",
    "umin requires: ASIMD",
    "uminp requires: ASIMD",
    "uminv requires: ASIMD",
    "umlal requires: ASIMD",
    "umlal2 requires: ASIMD",
    "umlsl requires: ASIMD",
    "umlsl2 requires: ASIMD",
    "ummla requires: ASIMD, I8MM",
    "umov requires: ASIMD",
    "umull requires: ASIMD",
    "umull2 requires: ASIMD",
    "uqadd requires: ASIMD",
    "uqrshl requires: ASIMD",
    "uqrshrn requires: ASIMD",
    "uqrshrn2 requires: ASIMD",
    "uqshl requires: ASIMD",
    "uqshrn requires: ASIMD",
    "uqshrn2 requires: ASIMD",
    "uqsub requires: ASIMD",
    "uqxtn requires: ASIMD",
    "uqxtn2 requires: ASIMD",
    "urecpe requires: ASIMD",
    "urhadd requires: ASIMD",
    "urshl requires: ASIMD",
    "urshr requires: ASIMD",
    "ursqrte requires: ASIMD",
    "ursra requires: ASIMD",
    "usdot requires: ASIMD, I8MM",
    "ushl requires: ASIMD",
    "ushll requires: ASIMD",
    "ushll2 requires: ASIMD",
    "ushr requires: ASIMD",
    "usmmla requires: ASIMD, I8MM",
    "usqadd requires: ASIMD",
    "usra requires: ASIMD",
    "usubl requires: ASIMD",
    "usubl2 requires: ASIMD",
    "usubw requires: ASIMD",
    "usubw2 requires: ASIMD",
    "uxtl requires: ASIMD",
    "uxtl2 requires: ASIMD",
    "uzp1 requires: ASIMD",
    "uzp2 requires: ASIMD",
    "xar requires: ASIMD, SHA3",
    "xtn requires: ASIMD",
    "xtn2 requires: ASIMD",
    "zip1 requires: ASIMD",
    "zip2 requires: ASIMD",
];

static INST_FEATURE_FORM_OFFSETS: [u16; InstId::_Count as usize + 1] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 6, 9, 12, 15, 18, 18, 19, 20, 26,
    32, 38, 38, 41, 44, 46, 48, 49, 49, 54, 59, 59, 59, 64, 69, 72, 73, 78, 83, 88, 93, 93, 93,
    103, 113, 116, 116, 117, 120, 123, 126, 128, 131, 133, 136, 139, 142, 144, 147, 149, 154, 154,
    154, 159, 159, 159, 167, 168, 174, 180, 183, 184, 185, 186, 189, 192, 193, 193, 193, 193, 193,
    196, 199, 202, 205, 208, 211, 214, 217, 220, 223, 226, 226, 226, 226, 226, 226, 226, 226, 226,
    226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226,
    226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226, 226,
    226, 226, 226, 226, 226, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236,
    236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236,
    236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236,
    236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236,
    236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236,
    236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236,
    236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 246, 246, 246, 246, 246, 246,
    246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246,
    246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246,
    246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246,
];

static INST_FEATURE_FORMS: [InstFeatureForm; 246] = [
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x7ec01400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fabd Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2ec01400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fabd Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6ec01400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fabd Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee0c000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fabs Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0ef8f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fabs Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4ef8f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fabs Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x7e402c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "facge Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2e402c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "facge Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6e402c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "facge Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x7ec02c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "facgt Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2ec02c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "facgt Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6ec02c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "facgt Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x1ee02800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fadd Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0e401400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fadd Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4e401400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fadd Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5e30d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec32, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "faddp Hd, Vn.2H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2e401400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "faddp Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6e401400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "faddp Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe00c10,
        opcode_value: 0x1ee00400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            OperandType::Imm as u32,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fccmp Hn, Hm, #nzcv, #cond requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe00c10,
        opcode_value: 0x1ee00410,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            OperandType::Imm as u32,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fccmpe Hn, Hm, #nzcv, #cond requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5ef8d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmeq Hd, Hn, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0ef8d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmeq Vd.4H, Vn.4H, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4ef8d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmeq Vd.8H, Vn.8H, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x5e402400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmeq Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0e402400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmeq Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4e402400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmeq Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x7ef8c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmge Hd, Hn, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2ef8c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmge Vd.4H, Vn.4H, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6ef8c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmge Vd.8H, Vn.8H, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x7e402400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmge Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2e402400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmge Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6e402400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmge Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5ef8c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmgt Hd, Hn, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0ef8c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmgt Vd.4H, Vn.4H, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4ef8c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmgt Vd.8H, Vn.8H, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x7ec02400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmgt Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2ec02400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmgt Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6ec02400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmgt Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x7ef8d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmle Hd, Hn, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2ef8d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmle Vd.4H, Vn.4H, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6ef8d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmle Vd.8H, Vn.8H, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5ef8e800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmlt Hd, Hn, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0ef8e800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmlt Vd.4H, Vn.4H, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4ef8e800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmlt Vd.8H, Vn.8H, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc1f,
        opcode_value: 0x1ee02008,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmp Hn, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc1f,
        opcode_value: 0x1ee02000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmp Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc1f,
        opcode_value: 0x1ee02018,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmpe Hn, #0 requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc1f,
        opcode_value: 0x1ee02010,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcmpe Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe00c00,
        opcode_value: 0x1ee00c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcsel Hd, Hn, Hm, #cond requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee40000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtas Wd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ee40000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtas Xd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5e79c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtas Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0e79c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtas Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4e79c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtas Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee50000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtau Wd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ee50000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtau Xd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x7e79c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtau Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2e79c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtau Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6e79c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtau Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ef00000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtms Wd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ef00000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtms Xd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5e79b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtms Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0e79b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtms Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4e79b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtms Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ef10000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtmu Wd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ef10000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtmu Xd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x7e79b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtmu Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2e79b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtmu Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6e79b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtmu Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0e40f400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::B, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000008002,
        context: "fcvtn Vd.8B, Vn.4H, Vm.4H requires: ASIMD, FP8",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4e40f400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::B, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000008002,
        context: "fcvtn Vd.16B, Vn.8H, Vm.8H requires: ASIMD, FP8",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0e00f400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::B, false),
            feature_reg_signature(RegType::Vec128, VecElementType::S, false),
            feature_reg_signature(RegType::Vec128, VecElementType::S, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000008002,
        context: "fcvtn Vd.8B, Vn.4S, Vm.4S requires: ASIMD, FP8",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4e00f400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::B, false),
            feature_reg_signature(RegType::Vec128, VecElementType::S, false),
            feature_reg_signature(RegType::Vec128, VecElementType::S, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000008002,
        context: "fcvtn2 Vx.16B, Vn.4S, Vm.4S requires: ASIMD, FP8",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee00000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtns Wd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ee00000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtns Xd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5e79a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtns Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0e79a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtns Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4e79a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtns Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee10000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtnu Wd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ee10000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtnu Xd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x7e79a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtnu Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2e79a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtnu Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6e79a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtnu Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee80000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtps Wd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ee80000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtps Xd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5ef9a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtps Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0ef9a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtps Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4ef9a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtps Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee90000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtpu Wd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ee90000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtpu Xd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x7ef9a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtpu Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2ef9a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtpu Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6ef9a800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtpu Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ef80000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzs Wd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ef80000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzs Xd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5ef9b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzs Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0ef9b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzs Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4ef9b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzs Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffff0000,
        opcode_value: 0x1ed80000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzs Wd, Hn, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffff0000,
        opcode_value: 0x9ed80000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzs Xd, Hn, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x5f00fc00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzs Hd, Hn, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x0f00fc00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzs Vd.4H, Vn.4H, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x4f00fc00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzs Vd.8H, Vn.8H, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ef90000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzu Wd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ef90000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzu Xd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x7ef9b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzu Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2ef9b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzu Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6ef9b800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzu Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffff0000,
        opcode_value: 0x1ed90000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzu Wd, Hn, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffff0000,
        opcode_value: 0x9ed90000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzu Xd, Hn, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x7f00fc00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzu Hd, Hn, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x2f00fc00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzu Vd.4H, Vn.4H, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x6f00fc00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fcvtzu Vd.8H, Vn.8H, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x1ee01800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fdiv Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2e403c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fdiv Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6e403c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fdiv Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe08000,
        opcode_value: 0x1fc00000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmadd Hd, Hn, Hm, Ha requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x1ee04800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmax Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0e403400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmax Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4e403400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmax Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x1ee06800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxnm Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0e400400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxnm Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4e400400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxnm Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5e30c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec32, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxnmp Hd, Vn.2H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2e400400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxnmp Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6e400400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxnmp Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0e30c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxnmv Hd, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4e30c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxnmv Hd, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5e30f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec32, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxp Hd, Vn.2H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2e403400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxp Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6e403400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxp Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0e30f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxv Hd, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4e30f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmaxv Hd, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x1ee05800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmin Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0ec03400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmin Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4ec03400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmin Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x1ee07800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminnm Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0ec00400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminnm Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4ec00400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminnm Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5eb0c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec32, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminnmp Hd, Vn.2H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2ec00400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminnmp Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6ec00400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminnmp Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0eb0c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminnmv Hd, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4eb0c800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminnmv Hd, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5eb0f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec32, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminp Hd, Vn.2H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2ec03400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminp Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6ec03400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminp Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0eb0f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminv Hd, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4eb0f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fminv Hd, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0e400c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmla Vx.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4e400c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmla Vx.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x5f001000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmla Hx, Hn, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x0f001000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmla Vx.4H, Vn.4H, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x4f001000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmla Vx.8H, Vn.8H, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0ec00c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmls Vx.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4ec00c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmls Vx.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x5f005000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmls Hx, Hn, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x0f005000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmls Vx.4H, Vn.4H, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x4f005000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmls Vx.8H, Vn.8H, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee60000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmov Wd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ee60000,
        operand_signatures: [
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmov Xd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee70000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmov Hd, Wn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ee70000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmov Hd, Xn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee04000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmov Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe01fe0,
        opcode_value: 0x1ee01000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmov Hd, #fimm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfff8fc00,
        opcode_value: 0x0f00fc00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmov Vd.4H, #fimm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfff8fc00,
        opcode_value: 0x4f00fc00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmov Vd.8H, #fimm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe08000,
        opcode_value: 0x1fc08000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmsub Hd, Hn, Hm, Ha requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x1ee00800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmul Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x2e401c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmul Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x6e401c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmul Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x5f009000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmul Hd, Hn, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x0f009000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmul Vd.4H, Vn.4H, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x4f009000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmul Vd.8H, Vn.8H, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x5e401c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmulx Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0e401c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmulx Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4e401c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmulx Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x7f009000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmulx Hd, Hn, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x2f009000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmulx Vd.4H, Vn.4H, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffc0f400,
        opcode_value: 0x6f009000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, true),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fmulx Vd.8H, Vn.8H, Vm.H[#idx] requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee14000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fneg Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2ef8f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fneg Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6ef8f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fneg Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe08000,
        opcode_value: 0x1fe00000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fnmadd Hd, Hn, Hm, Ha requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe08000,
        opcode_value: 0x1fe08000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fnmsub Hd, Hn, Hm, Ha requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x1ee08800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fnmul Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5ef9d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frecpe Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0ef9d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frecpe Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4ef9d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frecpe Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x5e403c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frecps Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0e403c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frecps Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4e403c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frecps Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5ef9f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frecpx Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee64000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frinta Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2e798800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frinta Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6e798800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frinta Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee7c000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frinti Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2ef99800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frinti Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6ef99800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frinti Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee54000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintm Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0e799800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintm Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4e799800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintm Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee44000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintn Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0e798800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintn Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4e798800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintn Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee4c000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintp Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0ef98800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintp Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4ef98800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintp Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee74000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintx Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2e799800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintx Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6e799800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintx Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee5c000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintz Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0ef99800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintz Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4ef99800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frintz Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x7ef9d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frsqrte Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2ef9d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frsqrte Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6ef9d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frsqrte Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x5ec03c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frsqrts Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0ec03c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frsqrts Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4ec03c00,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "frsqrts Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee1c000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fsqrt Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2ef9f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fsqrt Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6ef9f800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fsqrt Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x1ee03800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fsub Hd, Hn, Hm requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x0ec01400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fsub Vd.4H, Vn.4H, Vm.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffe0fc00,
        opcode_value: 0x4ec01400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "fsub Vd.8H, Vn.8H, Vm.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee20000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "scvtf Hd, Wn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ee20000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "scvtf Hd, Xn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x5e79d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "scvtf Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x0e79d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "scvtf Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x4e79d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "scvtf Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffff0000,
        opcode_value: 0x1ec20000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "scvtf Hd, Wn, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffff0000,
        opcode_value: 0x9ec20000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "scvtf Hd, Xn, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x5f00e400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "scvtf Hd, Hn, #bits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x0f00e400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "scvtf Vd.4H, Vn.4H, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x4f00e400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "scvtf Vd.8H, Vn.8H, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x1ee30000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "ucvtf Hd, Wn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x9ee30000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "ucvtf Hd, Xn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x7e79d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "ucvtf Hd, Hn requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x2e79d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "ucvtf Vd.4H, Vn.4H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xfffffc00,
        opcode_value: 0x6e79d800,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            0,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "ucvtf Vd.8H, Vn.8H requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffff0000,
        opcode_value: 0x1ec30000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Gp32, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "ucvtf Hd, Wn, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xffff0000,
        opcode_value: 0x9ec30000,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Gp64, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "ucvtf Hd, Xn, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x7f00e400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            feature_reg_signature(RegType::Vec16, VecElementType::None, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "ucvtf Hd, Hn, #bits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x2f00e400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            feature_reg_signature(RegType::Vec64, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "ucvtf Vd.4H, Vn.4H, #fbits requires: ASIMD, FP16",
    },
    InstFeatureForm {
        opcode_mask: 0xff80fc00,
        opcode_value: 0x6f00e400,
        operand_signatures: [
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            feature_reg_signature(RegType::Vec128, VecElementType::H, false),
            OperandType::Imm as u32,
            0,
            0,
            0,
        ],
        required: 0x0000000000004002,
        context: "ucvtf Vd.8H, Vn.8H, #fbits requires: ASIMD, FP16",
    },
];

fn required_features_for_form(
    inst_id: usize,
    opcode: u32,
    ops: &[&Operand],
) -> (u64, &'static str) {
    let start = INST_FEATURE_FORM_OFFSETS[inst_id] as usize;
    let end = INST_FEATURE_FORM_OFFSETS[inst_id + 1] as usize;
    for form in &INST_FEATURE_FORMS[start..end] {
        if form.matches(opcode, ops) {
            return (form.required, form.context);
        }
    }
    (
        INST_BASE_FEATURE_MASKS[inst_id],
        INST_BASE_FEATURE_CONTEXT[inst_id],
    )
}

/// One instruction carrying each represented feature.
pub static CPU_FEATURE_REPRESENTATIVE: [InstId; CPU_FEATURE_COUNT] = [
    InstId::Aesd_v,
    InstId::Abs_v,
    InstId::Bfcvt_v,
    InstId::Bti,
    InstId::Chkfeat,
    InstId::Clrbhb,
    InstId::Crc32b,
    InstId::Abs,
    InstId::Dgh,
    InstId::Sdot_v,
    InstId::Fcadd_v,
    InstId::Fmlal_v,
    InstId::Cfinv,
    InstId::Axflag,
    InstId::Fabd_v,
    InstId::Fcvtn_v,
    InstId::Frint32x_v,
    InstId::Smmla_v,
    InstId::Fjcvtzs_v,
    InstId::Ldlar,
    InstId::Cas,
    InstId::Addg,
    InstId::Ldgm,
    InstId::Autda,
    InstId::Esb,
    InstId::Sqrdmlah_v,
    InstId::Sha1c_v,
    InstId::Sha256h_v,
    InstId::Bcax_v,
    InstId::Sha512h_v,
    InstId::Sm3partw1_v,
    InstId::Sm4e_v,
];
// @generated AArch64 target features end
