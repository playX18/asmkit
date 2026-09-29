/* Copyright (c) 2008-2024 The AsmJit Authors

   This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

   Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

   The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
   Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
   This notice may not be removed or altered from any source distribution.
*/

//! X86 instruction API: read/write information queries.
//!
//! Per-operand access patterns come from the generated RW tables in
//! [`super::instdb`]; special instruction categories (mov, imul, string ops,
//! vector narrowing/widening, ...) are handled by code.
//!
//! [`Inst`] carries no architecture mode, so the query assumes X64
//! (`NATIVE_GP_SIZE == 8`): 32-bit GP writes zero-extend to 64 bits.
//! Same-register hints (`InstSameRegHint::kRO/kWO`) are folded into the query
//! itself: when all operands are the same physical register of the same
//! type, `kRO` makes every operand read-only and `kWO` write-only (see
//! [`apply_same_reg_hint`]).

use crate::AsmError;
use crate::core::arch_traits::Arch;
use crate::core::globals::InstOptions;
use crate::core::inst::Inst;
use crate::core::operand::{Imm, Operand, OperandType, RegGroup, RegType};
use crate::core::rwinfo::{
    CpuRwFlags, INVALID_PHYS_ID, InstRwFlags, InstRwInfo, InstSameRegHint, OpRwFlags, OpRwInfo,
};

use super::instdb::{
    ADDITIONAL_INFO_TABLE, Avx512Flags, CommonInfo, INST_COMMON_INFO_TABLE, INST_FLAGS_TABLE,
    INST_INFO_TABLE, InstId, InstInfo, RW_FLAGS_INFO_TABLE, RW_INFO_A_TABLE, RW_INFO_B_TABLE,
    RW_INFO_INDEX_A_TABLE, RW_INFO_INDEX_B_TABLE, RW_INFO_OP_TABLE, RW_INFO_RM_TABLE,
    RwInfoCategory, RwInfoRmCategory, RwInfoRmFlags,
};
use super::operands::{Gp, Mem};

/// GP register size of the X64 architecture.
const NATIVE_GP_SIZE: u32 = 8;

const R: OpRwFlags = OpRwFlags::READ;
const W: OpRwFlags = OpRwFlags::WRITE;
const X: OpRwFlags = OpRwFlags::RW;
const REG_M: OpRwFlags = OpRwFlags::REG_MEM;
const REG_PHYS: OpRwFlags = OpRwFlags::REG_PHYS_ID;
const MIB_READ: OpRwFlags =
    OpRwFlags::from_bits_retain(OpRwFlags::MEM_BASE_READ.bits() | OpRwFlags::MEM_INDEX_READ.bits());

/// Queries read/write information of the given instruction.
///
/// Returns [`AsmError::InvalidInstruction`] if the instruction id is not defined or the
/// operand combination is not recognized by a special-category handler.
pub fn query_rw_info(inst: &Inst) -> Result<InstRwInfo, AsmError> {
    if !matches!(inst.arch(), Arch::X86 | Arch::X64) {
        return Err(AsmError::InvalidArch);
    }
    let inst_id = inst.id as usize;
    if inst_id >= INST_INFO_TABLE.len() {
        return Err(AsmError::InvalidInstruction);
    }

    let inst_info = &INST_INFO_TABLE[inst_id];
    let common_info = &INST_COMMON_INFO_TABLE[inst_info.common_info_index as usize];

    let mut out = InstRwInfo::new();
    query_rw_info_internal(inst, inst_info, common_info, &mut out)?;
    apply_same_reg_hint(inst, common_info, &mut out);
    Ok(out)
}

/// Generates a trailing bit-mask that has `n` least significant bits set.
const fn lsb_mask_u64(n: u32) -> u64 {
    if n >= 64 {
        u64::MAX
    } else {
        (1u64 << n).wrapping_sub(1)
    }
}

/// Fills all trailing bits up to and including the most significant bit of `value`.
const fn fill_trailing_bits(value: u64) -> u64 {
    let leading = (value | 1).leading_zeros();
    ((u64::MAX >> 1) >> leading) | value
}

/// Maximum byte mask touched by a write to a register of the given group, used to clamp
/// zero-extension masks.
const fn reg_group_byte_mask(group: RegGroup) -> u64 {
    match group {
        RegGroup::Gp => 0xFF,
        RegGroup::Vec => u64::MAX,
        RegGroup::X86MM => 0xFF,
        RegGroup::X86K => 0xFF,
        RegGroup::X86SReg => 0x03,
        RegGroup::X86CReg => 0xFF,
        RegGroup::X86DReg => 0xFF,
        RegGroup::X86St => 0x03FF,
        RegGroup::X86Bnd => 0xFFFF,
        RegGroup::X86Rip => 0xFF,
        // AsmJit's table predates TMM registers; they never carry a ZExt flag.
        RegGroup::X86Tmm => 0,
    }
}

/// Resets `op` to `op_flags`, `register_size`, and `phys_id`, computing full byte masks
/// from the flags. Unlike [`OpRwInfo::reset`], a zero `register_size` yields an empty mask.
fn reset_op(op: &mut OpRwInfo, op_flags: OpRwFlags, register_size: u32, phys_id: u8) {
    op.op_flags = op_flags;
    op.phys_id = phys_id;
    op.rm_size = if op_flags.contains(OpRwFlags::REG_MEM) {
        register_size as u8
    } else {
        0
    };
    op.consecutive_lead_count = 0;

    let mask = lsb_mask_u64(register_size.min(64));
    op.read_byte_mask = if op_flags.contains(OpRwFlags::READ) {
        mask
    } else {
        0
    };
    op.write_byte_mask = if op_flags.contains(OpRwFlags::WRITE) {
        mask
    } else {
        0
    };
    op.extend_byte_mask = 0;
}

/// 32-bit GP writes zero-extend on X64.
fn rw_zero_extend_gp(op: &mut OpRwInfo, reg: &Operand, native_gp_size: u32) {
    if reg.x86_rm_size() + 4 == native_gp_size {
        op.op_flags |= OpRwFlags::ZEXT;
        op.extend_byte_mask = !op.write_byte_mask & 0xFF;
    }
}

/// Writing a 128/256-bit vector zero-extends the rest of the architectural 512-bit register.
fn rw_zero_extend_avx_vec(op: &mut OpRwInfo) {
    let msk = !fill_trailing_bits(op.write_byte_mask);
    if msk != 0 {
        op.op_flags |= OpRwFlags::ZEXT;
        op.extend_byte_mask = msk;
    }
}

/// Zero extension clamped by the register group's byte mask.
fn rw_zero_extend_non_vec(op: &mut OpRwInfo, reg: &Operand) {
    let msk =
        !fill_trailing_bits(op.write_byte_mask) & reg_group_byte_mask(reg.signature.reg_group());
    if msk != 0 {
        op.op_flags |= OpRwFlags::ZEXT;
        op.extend_byte_mask = msk;
    }
}

/// An AVX-512 `{k}` extra register is always read; unless zeroing (`{z}` option or
/// implicit-z instructions) the destination is also read (merge semantics).
fn rw_handle_avx512(inst: &Inst, common_info: &CommonInfo, out: &mut InstRwInfo) {
    if inst.extra_reg.is_reg() && inst.extra_reg.is_reg_type_of(RegType::Mask) && out.op_count > 0 {
        out.extra_reg.op_flags |= OpRwFlags::READ;
        out.extra_reg.read_byte_mask = 0xFF;
        if !inst.options.contains(InstOptions::X86_ZMASK)
            && !common_info.has_avx512_flag(Avx512Flags::IMPLICIT_Z)
        {
            out.operands[0].op_flags |= OpRwFlags::READ;
            out.operands[0].read_byte_mask |= out.operands[0].write_byte_mask;
        }
    }
}

/// Only called when all operands are registers.
fn has_same_reg_type(operands: &[Operand]) -> bool {
    debug_assert!(!operands.is_empty());
    let reg_type = operands[0].signature.reg_type();
    operands[1..]
        .iter()
        .all(|op| op.signature.reg_type() == reg_type)
}

/// Applies the same-register hint from [`CommonInfo`] to the query result: when all
/// operands share a tied register, `kRO` clears write access on every operand, `kWO`
/// clears read access (`xor x, x` writes `x` without reading it, including the zero
/// extension).
fn apply_same_reg_hint(inst: &Inst, common_info: &CommonInfo, out: &mut InstRwInfo) {
    let hint = common_info.same_reg_hint;
    if hint == InstSameRegHint::None || out.op_count < 2 {
        return;
    }

    let operands = inst.operands();
    if !operands[0].is_reg() {
        return;
    }
    let id0 = operands[0].id();
    let type0 = operands[0].signature.reg_type();
    let all_same = operands[1..]
        .iter()
        .all(|op| op.is_reg() && op.id() == id0 && op.signature.reg_type() == type0);
    if !all_same {
        return;
    }

    for op in out.operands_mut() {
        match hint {
            InstSameRegHint::RO => {
                op.op_flags &= !(OpRwFlags::WRITE | OpRwFlags::ZEXT);
                op.write_byte_mask = 0;
                op.extend_byte_mask = 0;
            }
            InstSameRegHint::WO => {
                op.op_flags &= !OpRwFlags::READ;
                op.read_byte_mask = 0;
            }
            InstSameRegHint::None => {}
        }
    }
}

/// The query itself, split from [`query_rw_info`] so special categories can return early
/// exactly like the C++ code does.
fn query_rw_info_internal(
    inst: &Inst,
    inst_info: &InstInfo,
    common_info: &CommonInfo,
    out: &mut InstRwInfo,
) -> Result<(), AsmError> {
    let operands = inst.operands();
    let op_count = operands.len();

    let additional_info = &ADDITIONAL_INFO_TABLE[inst_info.additional_info_index as usize];
    let rw_flags = &RW_FLAGS_INFO_TABLE[additional_info.rw_flags_index as usize];

    // There are two data tables, one for `op_count == 2` and the second for
    // `op_count != 2` (AsmJit: two tables are needed so the index fits into 8 bits and
    // because 2-operand forms can have different RW semantics than 3+-operand forms).
    let inst_id = inst.id as usize;
    let inst_rw_info = if op_count == 2 {
        &RW_INFO_A_TABLE[RW_INFO_INDEX_A_TABLE[inst_id] as usize]
    } else {
        &RW_INFO_B_TABLE[RW_INFO_INDEX_B_TABLE[inst_id] as usize]
    };
    let inst_rm_info = &RW_INFO_RM_TABLE[inst_rw_info.rm_info as usize];

    out.inst_flags = INST_FLAGS_TABLE[additional_info.inst_flags_index as usize];
    out.op_count = op_count as u8;
    out.rm_feature = inst_rm_info.rm_feature;
    out.extra_reg = OpRwInfo::new();
    out.read_flags = CpuRwFlags::from_bits_retain(rw_flags.read_flags);
    out.write_flags = CpuRwFlags::from_bits_retain(rw_flags.write_flags);

    let mut op_type_mask: u32 = 0;

    if (inst_rw_info.category as u8) <= (RwInfoCategory::GenericEx as u8) {
        let mut rm_ops_mask: u32 = 0;
        let mut rm_max_size: u32 = 0;

        for i in 0..op_count {
            let src_op = &operands[i];
            let rw_op_data = &RW_INFO_OP_TABLE[inst_rw_info.op_info_index[i] as usize];

            op_type_mask |= 1u32 << (src_op.op_type() as u32);

            let op = &mut out.operands[i];
            if !src_op.is_reg_or_mem() {
                *op = OpRwInfo::new();
                continue;
            }

            op.op_flags = rw_op_data.flags & !OpRwFlags::ZEXT;
            op.phys_id = rw_op_data.phys_id;
            op.rm_size = 0;

            let mut r_byte_mask = rw_op_data.r_byte_mask;
            let mut w_byte_mask = rw_op_data.w_byte_mask;

            if op.is_read() && r_byte_mask == 0 {
                r_byte_mask = lsb_mask_u64(src_op.x86_rm_size());
            }

            if op.is_write() && w_byte_mask == 0 {
                w_byte_mask = lsb_mask_u64(src_op.x86_rm_size());
            }

            op.read_byte_mask = r_byte_mask;
            op.write_byte_mask = w_byte_mask;
            op.extend_byte_mask = 0;
            op.consecutive_lead_count = rw_op_data.consecutive_lead_count;

            if src_op.is_reg() {
                // Zero extension.
                if op.is_write() {
                    if src_op.is_gp() {
                        // GP registers on X64 are special:
                        //   - 8-bit and 16-bit writes aren't zero extended.
                        //   - 32-bit writes ARE zero extended.
                        rw_zero_extend_gp(op, src_op, NATIVE_GP_SIZE);
                    } else if rw_op_data.flags.contains(OpRwFlags::ZEXT) {
                        // Otherwise follow ZExt.
                        rw_zero_extend_non_vec(op, src_op);
                    }
                }

                // Aggregate values required to calculate valid Reg/M info.
                rm_max_size = rm_max_size.max(src_op.x86_rm_size());
                rm_ops_mask |= 1u32 << i;
            } else {
                let mem_op = src_op.as_::<Mem>();
                // The RW flags of BASE+INDEX are either provided by the data, which means
                // that the instruction is border-case, or they are deduced from the operand.
                if mem_op.has_base_reg() && !op.op_flags.contains(OpRwFlags::MEM_BASE_RW) {
                    op.op_flags |= OpRwFlags::MEM_BASE_READ;
                }
                if mem_op.has_index_reg() && !op.op_flags.contains(OpRwFlags::MEM_INDEX_RW) {
                    op.op_flags |= OpRwFlags::MEM_INDEX_READ;
                }
            }
        }

        // Only keep MovOp if the instruction is actually register to register move of the
        // same kind.
        if out.inst_flags.contains(InstRwFlags::MOV_OP) {
            let reg_bit = 1u32 << (OperandType::Reg as u32);
            if !(op_count >= 2 && op_type_mask == reg_bit && has_same_reg_type(operands)) {
                out.inst_flags &= !InstRwFlags::MOV_OP;
            }
        }

        // Special cases require more logic.
        let rm_flags = RwInfoRmFlags::from_bits_retain(inst_rm_info.flags);
        if rm_flags.intersects(
            RwInfoRmFlags::MOVSS_MOVSD | RwInfoRmFlags::PEXTRW | RwInfoRmFlags::FEATURE_IF_RMI,
        ) {
            if rm_flags.contains(RwInfoRmFlags::MOVSS_MOVSD) {
                if op_count == 2 && operands[0].is_reg() && operands[1].is_reg() {
                    // Doesn't zero extend the destination.
                    out.operands[0].extend_byte_mask = 0;
                }
            } else if rm_flags.contains(RwInfoRmFlags::PEXTRW) {
                if op_count == 3 && operands[1].is_reg_type_of(RegType::X86Mm) {
                    out.rm_feature = 0;
                    rm_ops_mask = 0;
                }
            } else if rm_flags.contains(RwInfoRmFlags::FEATURE_IF_RMI)
                && (op_count != 3 || !operands[2].is_imm())
            {
                out.rm_feature = 0;
            }
        }

        rm_ops_mask &= inst_rm_info.rm_ops_mask as u32;
        if rm_ops_mask != 0 && !inst.options.contains(InstOptions::X86_ER) {
            let mut mask = rm_ops_mask;
            while mask != 0 {
                let i = mask.trailing_zeros() as usize;
                mask &= mask - 1;

                let op = &mut out.operands[i];
                op.op_flags |= REG_M;

                match inst_rm_info.category {
                    RwInfoRmCategory::Fixed => op.rm_size = inst_rm_info.fixed_size,
                    RwInfoRmCategory::Consistent => op.rm_size = operands[i].x86_rm_size() as u8,
                    RwInfoRmCategory::Half => op.rm_size = (rm_max_size / 2) as u8,
                    RwInfoRmCategory::Quarter => op.rm_size = (rm_max_size / 4) as u8,
                    RwInfoRmCategory::Eighth => op.rm_size = (rm_max_size / 8) as u8,
                    RwInfoRmCategory::None => {}
                }
            }
        }

        // Special cases per instruction.
        if inst_rw_info.category == RwInfoCategory::GenericEx {
            match inst.id {
                id if (id == InstId::Vpternlogd as u32 || id == InstId::Vpternlogq as u32)
                    && op_count == 4
                    && operands[3].is_imm() =>
                {
                    let predicate = operands[3].as_::<Imm>().value() as u8;

                    if (predicate >> 4) == (predicate & 0xF) {
                        out.operands[0].op_flags &= !OpRwFlags::READ;
                        out.operands[0].read_byte_mask = 0;
                    }
                }
                _ => {}
            }
        }

        rw_handle_avx512(inst, common_info, out);
        return Ok(());
    }

    match inst_rw_info.category {
        RwInfoCategory::Mov => {
            // Special case for 'mov' instruction. Here there are some variants that we have
            // to handle as 'mov' can be used to move between GP, segment, control and debug
            // registers. Moving between GP registers also allow to use memory operand.

            // We will again set the flag if it's actually a move from GP to GP register,
            // otherwise this flag cannot be set.
            out.inst_flags &= !InstRwFlags::MOV_OP;

            if op_count == 2 {
                if operands[0].is_reg() && operands[1].is_reg() {
                    let o0_gp = operands[0].is_gp();
                    let o1_gp = operands[1].is_gp();
                    let o0_sreg = operands[0].is_reg_type_of(RegType::X86SReg);
                    let o1_sreg = operands[1].is_reg_type_of(RegType::X86SReg);
                    let o0_cd = operands[0].is_reg_type_of(RegType::X86CReg)
                        || operands[0].is_reg_type_of(RegType::X86DReg);
                    let o1_cd = operands[1].is_reg_type_of(RegType::X86CReg)
                        || operands[1].is_reg_type_of(RegType::X86DReg);

                    if o0_gp && o1_gp {
                        reset_op(
                            &mut out.operands[0],
                            W | REG_M,
                            operands[0].x86_rm_size(),
                            INVALID_PHYS_ID,
                        );
                        reset_op(
                            &mut out.operands[1],
                            R | REG_M,
                            operands[1].x86_rm_size(),
                            INVALID_PHYS_ID,
                        );

                        rw_zero_extend_gp(&mut out.operands[0], &operands[0], NATIVE_GP_SIZE);
                        out.inst_flags |= InstRwFlags::MOV_OP;
                        return Ok(());
                    }

                    if o0_gp && o1_sreg {
                        reset_op(
                            &mut out.operands[0],
                            W | REG_M,
                            NATIVE_GP_SIZE,
                            INVALID_PHYS_ID,
                        );
                        out.operands[0].rm_size = 2;
                        reset_op(&mut out.operands[1], R, 2, INVALID_PHYS_ID);
                        return Ok(());
                    }

                    if o0_sreg && o1_gp {
                        reset_op(&mut out.operands[0], W, 2, INVALID_PHYS_ID);
                        reset_op(&mut out.operands[1], R | REG_M, 2, INVALID_PHYS_ID);
                        out.operands[1].rm_size = 2;
                        return Ok(());
                    }

                    if (o0_gp && o1_cd) || (o0_cd && o1_gp) {
                        reset_op(&mut out.operands[0], W, NATIVE_GP_SIZE, INVALID_PHYS_ID);
                        reset_op(&mut out.operands[1], R, NATIVE_GP_SIZE, INVALID_PHYS_ID);
                        out.write_flags = CpuRwFlags::X86_OF
                            | CpuRwFlags::X86_SF
                            | CpuRwFlags::X86_ZF
                            | CpuRwFlags::X86_AF
                            | CpuRwFlags::X86_PF
                            | CpuRwFlags::X86_CF;
                        return Ok(());
                    }
                }

                if operands[0].is_reg() && operands[1].is_mem() {
                    let o1 = operands[1].as_::<Mem>();

                    if operands[0].is_gp() {
                        if !o1.is_offset_64bit() {
                            reset_op(
                                &mut out.operands[0],
                                W,
                                operands[0].x86_rm_size(),
                                INVALID_PHYS_ID,
                            );
                        } else {
                            reset_op(
                                &mut out.operands[0],
                                W | REG_PHYS,
                                operands[0].x86_rm_size(),
                                Gp::AX as u8,
                            );
                        }

                        reset_op(
                            &mut out.operands[1],
                            R | MIB_READ,
                            operands[0].x86_rm_size(),
                            INVALID_PHYS_ID,
                        );
                        rw_zero_extend_gp(&mut out.operands[0], &operands[0], NATIVE_GP_SIZE);
                        return Ok(());
                    }

                    if operands[0].is_reg_type_of(RegType::X86SReg) {
                        reset_op(&mut out.operands[0], W, 2, INVALID_PHYS_ID);
                        reset_op(&mut out.operands[1], R, 2, INVALID_PHYS_ID);
                        return Ok(());
                    }
                }

                if operands[0].is_mem() && operands[1].is_reg() {
                    let o0 = operands[0].as_::<Mem>();

                    if operands[1].is_gp() {
                        reset_op(
                            &mut out.operands[0],
                            W | MIB_READ,
                            operands[1].x86_rm_size(),
                            INVALID_PHYS_ID,
                        );
                        if !o0.is_offset_64bit() {
                            reset_op(
                                &mut out.operands[1],
                                R,
                                operands[1].x86_rm_size(),
                                INVALID_PHYS_ID,
                            );
                        } else {
                            reset_op(
                                &mut out.operands[1],
                                R | REG_PHYS,
                                operands[1].x86_rm_size(),
                                Gp::AX as u8,
                            );
                        }
                        return Ok(());
                    }

                    if operands[1].is_reg_type_of(RegType::X86SReg) {
                        reset_op(&mut out.operands[0], W | MIB_READ, 2, INVALID_PHYS_ID);
                        reset_op(&mut out.operands[1], R, 2, INVALID_PHYS_ID);
                        return Ok(());
                    }
                }

                if operands[0].is_gp() && operands[1].is_imm() {
                    reset_op(
                        &mut out.operands[0],
                        W | REG_M,
                        operands[0].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    out.operands[1] = OpRwInfo::new();

                    rw_zero_extend_gp(&mut out.operands[0], &operands[0], NATIVE_GP_SIZE);
                    return Ok(());
                }

                if operands[0].is_mem() && operands[1].is_imm() {
                    // AsmJit reads `operands[0].as<Reg>().size()` here; the size field is
                    // shared between register and memory signatures, so this is the same
                    // value as the memory operand's size.
                    reset_op(
                        &mut out.operands[0],
                        W | MIB_READ,
                        operands[0].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    out.operands[1] = OpRwInfo::new();
                    return Ok(());
                }
            }
        }

        RwInfoCategory::Movabs => {
            if op_count == 2 {
                if operands[0].is_gp() && operands[1].is_mem() {
                    reset_op(
                        &mut out.operands[0],
                        W | REG_PHYS,
                        operands[0].x86_rm_size(),
                        Gp::AX as u8,
                    );
                    reset_op(
                        &mut out.operands[1],
                        R | MIB_READ,
                        operands[0].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    rw_zero_extend_gp(&mut out.operands[0], &operands[0], NATIVE_GP_SIZE);
                    return Ok(());
                }

                if operands[0].is_mem() && operands[1].is_gp() {
                    reset_op(
                        &mut out.operands[0],
                        W | MIB_READ,
                        operands[1].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    reset_op(
                        &mut out.operands[1],
                        R | REG_PHYS,
                        operands[1].x86_rm_size(),
                        Gp::AX as u8,
                    );
                    return Ok(());
                }

                if operands[0].is_gp() && operands[1].is_imm() {
                    reset_op(
                        &mut out.operands[0],
                        W,
                        operands[0].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    out.operands[1] = OpRwInfo::new();

                    rw_zero_extend_gp(&mut out.operands[0], &operands[0], NATIVE_GP_SIZE);
                    return Ok(());
                }
            }
        }

        RwInfoCategory::Imul => {
            // Special case for 'imul' instruction.
            //
            // There are 3 variants in general:
            //
            //   1. Standard multiplication: 'A = A * B'.
            //   2. Multiplication with imm: 'A = B * C'.
            //   3. Extended multiplication: 'A:B = B * C'.

            if op_count == 2 {
                if operands[0].is_reg() && operands[1].is_imm() {
                    reset_op(
                        &mut out.operands[0],
                        X,
                        operands[0].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    out.operands[1] = OpRwInfo::new();

                    rw_zero_extend_gp(&mut out.operands[0], &operands[0], NATIVE_GP_SIZE);
                    return Ok(());
                }

                if operands[0].is_reg_type_of(RegType::Gp16) && operands[1].x86_rm_size() == 1 {
                    // imul ax, r8/m8 <- AX = AL * r8/m8
                    reset_op(&mut out.operands[0], X | REG_PHYS, 2, Gp::AX as u8);
                    out.operands[0].read_byte_mask = lsb_mask_u64(1);
                    reset_op(&mut out.operands[1], R | REG_M, 1, INVALID_PHYS_ID);
                } else {
                    // imul r?, r?/m?
                    reset_op(
                        &mut out.operands[0],
                        X,
                        operands[0].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    reset_op(
                        &mut out.operands[1],
                        R | REG_M,
                        operands[0].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    rw_zero_extend_gp(&mut out.operands[0], &operands[0], NATIVE_GP_SIZE);
                }

                if operands[1].is_mem() {
                    out.operands[1].op_flags |= MIB_READ;
                }
                return Ok(());
            }

            if op_count == 3 {
                if operands[2].is_imm() {
                    reset_op(
                        &mut out.operands[0],
                        W,
                        operands[0].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    reset_op(
                        &mut out.operands[1],
                        R | REG_M,
                        operands[1].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    out.operands[2] = OpRwInfo::new();

                    rw_zero_extend_gp(&mut out.operands[0], &operands[0], NATIVE_GP_SIZE);
                    if operands[1].is_mem() {
                        out.operands[1].op_flags |= MIB_READ;
                    }
                    return Ok(());
                } else {
                    reset_op(
                        &mut out.operands[0],
                        W | REG_PHYS,
                        operands[0].x86_rm_size(),
                        Gp::DX as u8,
                    );
                    reset_op(
                        &mut out.operands[1],
                        X | REG_PHYS,
                        operands[1].x86_rm_size(),
                        Gp::AX as u8,
                    );
                    reset_op(
                        &mut out.operands[2],
                        R | REG_M,
                        operands[2].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );

                    rw_zero_extend_gp(&mut out.operands[0], &operands[0], NATIVE_GP_SIZE);
                    rw_zero_extend_gp(&mut out.operands[1], &operands[1], NATIVE_GP_SIZE);
                    if operands[2].is_mem() {
                        out.operands[2].op_flags |= MIB_READ;
                    }
                    return Ok(());
                }
            }
        }

        RwInfoCategory::Movh64 => {
            // Special case for 'movhpd|movhps' instructions. Note that this is only required
            // for legacy (non-AVX) variants as AVX instructions use either 2 or 3 operands
            // that are in Generic category.
            if op_count == 2 {
                if operands[0].is_vec() && operands[1].is_mem() {
                    reset_op(&mut out.operands[0], W, 8, INVALID_PHYS_ID);
                    out.operands[0].write_byte_mask = lsb_mask_u64(8) << 8;
                    reset_op(&mut out.operands[1], R | MIB_READ, 8, INVALID_PHYS_ID);
                    return Ok(());
                }

                if operands[0].is_mem() && operands[1].is_vec() {
                    reset_op(&mut out.operands[0], W | MIB_READ, 8, INVALID_PHYS_ID);
                    reset_op(&mut out.operands[1], R, 8, INVALID_PHYS_ID);
                    out.operands[1].read_byte_mask = lsb_mask_u64(8) << 8;
                    return Ok(());
                }
            }
        }

        RwInfoCategory::Punpcklxx => {
            // Special case for 'punpcklbw|punpckldq|punpcklwd' instructions.
            if op_count == 2 {
                if operands[0].is_vec128() {
                    reset_op(&mut out.operands[0], X, 16, INVALID_PHYS_ID);
                    out.operands[0].read_byte_mask = 0x0F0F;
                    out.operands[0].write_byte_mask = 0xFFFF;
                    reset_op(&mut out.operands[1], R, 16, INVALID_PHYS_ID);
                    out.operands[1].write_byte_mask = 0x0F0F;

                    if operands[1].is_vec128() {
                        return Ok(());
                    }

                    if operands[1].is_mem() {
                        out.operands[1].op_flags |= MIB_READ;
                        return Ok(());
                    }
                }

                if operands[0].is_reg_type_of(RegType::X86Mm) {
                    reset_op(&mut out.operands[0], X, 8, INVALID_PHYS_ID);
                    out.operands[0].read_byte_mask = 0x0F;
                    out.operands[0].write_byte_mask = 0xFF;
                    reset_op(&mut out.operands[1], R, 4, INVALID_PHYS_ID);
                    out.operands[1].read_byte_mask = 0x0F;

                    if operands[1].is_reg_type_of(RegType::X86Mm) {
                        return Ok(());
                    }

                    if operands[1].is_mem() {
                        out.operands[1].op_flags |= MIB_READ;
                        return Ok(());
                    }
                }
            }
        }

        RwInfoCategory::Vmaskmov => {
            // Special case for 'vmaskmovpd|vmaskmovps|vpmaskmovd|vpmaskmovq' instructions.
            if op_count == 3 {
                if operands[0].is_vec() && operands[1].is_vec() && operands[2].is_mem() {
                    reset_op(
                        &mut out.operands[0],
                        W,
                        operands[0].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    reset_op(
                        &mut out.operands[1],
                        R,
                        operands[1].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    reset_op(
                        &mut out.operands[2],
                        R | MIB_READ,
                        operands[1].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );

                    rw_zero_extend_avx_vec(&mut out.operands[0]);
                    return Ok(());
                }

                if operands[0].is_mem() && operands[1].is_vec() && operands[2].is_vec() {
                    reset_op(
                        &mut out.operands[0],
                        X | MIB_READ,
                        operands[1].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    reset_op(
                        &mut out.operands[1],
                        R,
                        operands[1].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    reset_op(
                        &mut out.operands[2],
                        R,
                        operands[2].x86_rm_size(),
                        INVALID_PHYS_ID,
                    );
                    return Ok(());
                }
            }
        }

        RwInfoCategory::Vmovddup => {
            // Special case for 'vmovddup' instruction. This instruction has an interesting
            // semantic as 128-bit XMM version only uses 64-bit memory operand (m64),
            // however, 256/512-bit versions use 256/512-bit memory operand, respectively.
            if op_count == 2 {
                if operands[0].is_vec() && operands[1].is_vec() {
                    let o0_size = operands[0].x86_rm_size();
                    let o1_size = if o0_size == 16 { 8 } else { o0_size };

                    reset_op(&mut out.operands[0], W, o0_size, INVALID_PHYS_ID);
                    reset_op(&mut out.operands[1], R | REG_M, o1_size, INVALID_PHYS_ID);
                    out.operands[1].read_byte_mask &= 0x00FF00FF00FF00FF;

                    rw_zero_extend_avx_vec(&mut out.operands[0]);
                    rw_handle_avx512(inst, common_info, out);
                    return Ok(());
                }

                if operands[0].is_vec() && operands[1].is_mem() {
                    let o0_size = operands[0].x86_rm_size();
                    let o1_size = if o0_size == 16 { 8 } else { o0_size };

                    reset_op(&mut out.operands[0], W, o0_size, INVALID_PHYS_ID);
                    reset_op(&mut out.operands[1], R | MIB_READ, o1_size, INVALID_PHYS_ID);

                    rw_zero_extend_avx_vec(&mut out.operands[0]);
                    rw_handle_avx512(inst, common_info, out);
                    return Ok(());
                }
            }
        }

        RwInfoCategory::Vmovmskpd | RwInfoCategory::Vmovmskps => {
            // Special case for 'vmovmskpd|vmovmskps' instructions.
            if op_count == 2 && operands[0].is_gp() && operands[1].is_vec() {
                reset_op(&mut out.operands[0], W, 1, INVALID_PHYS_ID);
                out.operands[0].extend_byte_mask = lsb_mask_u64(NATIVE_GP_SIZE - 1) << 1;
                reset_op(
                    &mut out.operands[1],
                    R,
                    operands[1].x86_rm_size(),
                    INVALID_PHYS_ID,
                );
                return Ok(());
            }
        }

        RwInfoCategory::Vmov1_2 | RwInfoCategory::Vmov1_4 | RwInfoCategory::Vmov1_8 => {
            // Special case for instructions where the destination is 1:N (narrowing).
            //
            // Vmov1_2: vcvtpd2dq|vcvttpd2dq, vcvtpd2udq|vcvttpd2udq, vcvtpd2ps|vcvtps2ph,
            //          vcvtqq2ps|vcvtuqq2ps, vpmovwb|vpmovswb|vpmovuswb,
            //          vpmovdw|vpmovsdw|vpmovusdw, vpmovqd|vpmovsqd|vpmovusqd
            // Vmov1_4: vpmovdb|vpmovsdb|vpmovusdb, vpmovqw|vpmovsqw|vpmovusqw
            // Vmov1_8: pmovmskb|vpmovmskb, vpmovqb|vpmovsqb|vpmovusqb
            let shift = inst_rw_info.category as u32 - RwInfoCategory::Vmov1_2 as u32 + 1;

            if op_count >= 2 {
                if op_count >= 3 {
                    if op_count > 3 {
                        return Err(AsmError::InvalidInstruction);
                    }
                    out.operands[2] = OpRwInfo::new();
                }

                if operands[0].is_reg() && operands[1].is_reg() {
                    let size1 = operands[1].x86_rm_size();
                    let size0 = size1 >> shift;

                    reset_op(&mut out.operands[0], W, size0, INVALID_PHYS_ID);
                    reset_op(&mut out.operands[1], R, size1, INVALID_PHYS_ID);

                    if inst_rm_info.rm_ops_mask & 0x1 != 0 {
                        out.operands[0].op_flags |= REG_M;
                        out.operands[0].rm_size = size0 as u8;
                    }

                    if inst_rm_info.rm_ops_mask & 0x2 != 0 {
                        out.operands[1].op_flags |= REG_M;
                        out.operands[1].rm_size = size1 as u8;
                    }

                    if operands[0].is_gp() {
                        rw_zero_extend_gp(&mut out.operands[0], &operands[0], NATIVE_GP_SIZE);
                    }

                    if operands[0].is_vec() {
                        rw_zero_extend_avx_vec(&mut out.operands[0]);
                    }

                    rw_handle_avx512(inst, common_info, out);
                    return Ok(());
                }

                if operands[0].is_reg() && operands[1].is_mem() {
                    let rm1 = operands[1].x86_rm_size();
                    let size1 = if rm1 != 0 { rm1 } else { 16 };
                    let size0 = size1 >> shift;

                    reset_op(&mut out.operands[0], W, size0, INVALID_PHYS_ID);
                    reset_op(&mut out.operands[1], R | MIB_READ, size1, INVALID_PHYS_ID);

                    if operands[0].is_vec() {
                        rw_zero_extend_avx_vec(&mut out.operands[0]);
                    }

                    return Ok(());
                }

                if operands[0].is_mem() && operands[1].is_reg() {
                    let size1 = operands[1].x86_rm_size();
                    let size0 = size1 >> shift;

                    reset_op(&mut out.operands[0], W | MIB_READ, size0, INVALID_PHYS_ID);
                    reset_op(&mut out.operands[1], R, size1, INVALID_PHYS_ID);

                    rw_handle_avx512(inst, common_info, out);
                    return Ok(());
                }
            }
        }

        RwInfoCategory::Vmov2_1 | RwInfoCategory::Vmov4_1 | RwInfoCategory::Vmov8_1 => {
            // Special case for instructions where the destination is N:1 (widening).
            //
            // Vmov2_1: vcvtdq2pd|vcvtudq2pd, vcvtps2pd|vcvtph2ps, vcvtps2qq|vcvtps2uqq,
            //          vcvttps2qq|vcvttps2uqq, vpmovsxbw|vpmovzxbw, vpmovsxwd|vpmovzxwd,
            //          vpmovsxdq|vpmovzxdq
            // Vmov4_1: vpmovsxbd|vpmovzxbd, vpmovsxwq|vpmovzxwq
            // Vmov8_1: vpmovsxbq|vpmovzxbq
            let shift = inst_rw_info.category as u32 - RwInfoCategory::Vmov2_1 as u32 + 1;

            if op_count >= 2 {
                if op_count >= 3 {
                    if op_count > 3 {
                        return Err(AsmError::InvalidInstruction);
                    }
                    out.operands[2] = OpRwInfo::new();
                }

                let size0 = operands[0].x86_rm_size();
                let size1 = size0 >> shift;

                reset_op(&mut out.operands[0], W, size0, INVALID_PHYS_ID);
                reset_op(&mut out.operands[1], R, size1, INVALID_PHYS_ID);

                if operands[0].is_vec() {
                    rw_zero_extend_avx_vec(&mut out.operands[0]);
                }

                if operands[0].is_reg() && operands[1].is_reg() {
                    if inst_rm_info.rm_ops_mask & 0x1 != 0 {
                        out.operands[0].op_flags |= REG_M;
                        out.operands[0].rm_size = size0 as u8;
                    }

                    if inst_rm_info.rm_ops_mask & 0x2 != 0 {
                        out.operands[1].op_flags |= REG_M;
                        out.operands[1].rm_size = size1 as u8;
                    }

                    rw_handle_avx512(inst, common_info, out);
                    return Ok(());
                }

                if operands[0].is_reg() && operands[1].is_mem() {
                    out.operands[1].op_flags |= MIB_READ;

                    rw_handle_avx512(inst, common_info, out);
                    return Ok(());
                }
            }
        }

        _ => {}
    }

    Err(AsmError::InvalidInstruction)
}
