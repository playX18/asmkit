/* Copyright (c) 2008-2024 The AsmJit Authors

   This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

   Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

   The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
   Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
   This notice may not be removed or altered from any source distribution.
*/

//! X86 instruction API: read/write information queries.
//!
//! [AsmKit] This file is a derived work: it was translated from AsmJit's
//! `asmjit/x86/x86instapi.cpp` (`x86::InstInternal::query_rw_info`) by hand.
//!
//! Per-operand access patterns come from the generated RW tables in
//! [`super::instdb`]; special instruction categories (mov, imul, string ops,
//! vector narrowing/widening, ...) are handled by code, mirroring the C++
//! structure. asmkit-specific adaptations:
//!
//! - AsmJit's `rw_reg_group_byte_mask_table` is indexed by its own `RegGroup`
//!   numbering; here it is ported symbolically as [`reg_group_byte_mask`].
//! - AsmJit takes the target architecture (X86 vs X64) to derive
//!   `native_gp_size`; asmkit's [`Inst`] carries no mode, so the query assumes
//!   X64 (`NATIVE_GP_SIZE == 8`): 32-bit GP writes zero-extend to 64 bits.
//! - AsmJit applies same-register hints (`InstSameRegHint::kRO/kWO`) in the
//!   register-allocation pass on tied registers; asmkit folds the rule into
//!   the query itself — when all operands are the same physical register of
//!   the same type, `kRO` makes every operand read-only and `kWO` write-only
//!   (see [`apply_same_reg_hint`]).

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

/// GP register size of the X64 architecture (AsmJit derives this from `Arch`).
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

/// Generates a trailing bit-mask that has `n` least significant bits set
/// (port of AsmJit's `Support::lsb_mask<uint64_t>`).
const fn lsb_mask_u64(n: u32) -> u64 {
    if n >= 64 {
        u64::MAX
    } else {
        (1u64 << n).wrapping_sub(1)
    }
}

/// Fills all trailing bits up to and including the most significant bit of `value`
/// (port of AsmJit's `Support::fill_trailing_bits`).
const fn fill_trailing_bits(value: u64) -> u64 {
    let leading = (value | 1).leading_zeros();
    ((u64::MAX >> 1) >> leading) | value
}

/// Maximum byte mask touched by a write to a register of the given group, used to clamp
/// zero-extension masks (symbolic port of AsmJit's `rw_reg_group_byte_mask_table`, which is
/// indexed by AsmJit's own `RegGroup` numbering).
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
/// from the flags (port of AsmJit's `OpRWInfo::reset`; unlike [`OpRwInfo::reset`] this
/// handles a zero `register_size` exactly like AsmJit's `lsb_mask` — an empty mask).
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

/// Port of AsmJit's `rw_zero_extend_gp`: 32-bit GP writes zero-extend on X64.
fn rw_zero_extend_gp(op: &mut OpRwInfo, reg: &Operand, native_gp_size: u32) {
    if reg.x86_rm_size() + 4 == native_gp_size {
        op.op_flags |= OpRwFlags::ZEXT;
        op.extend_byte_mask = !op.write_byte_mask & 0xFF;
    }
}

/// Port of AsmJit's `rw_zero_extend_avx_vec`: writing a 128/256-bit vector zero-extends
/// the rest of the architectural 512-bit register.
fn rw_zero_extend_avx_vec(op: &mut OpRwInfo) {
    let msk = !fill_trailing_bits(op.write_byte_mask);
    if msk != 0 {
        op.op_flags |= OpRwFlags::ZEXT;
        op.extend_byte_mask = msk;
    }
}

/// Port of AsmJit's `rw_zero_extend_non_vec`: zero extension clamped by the register
/// group's byte mask.
fn rw_zero_extend_non_vec(op: &mut OpRwInfo, reg: &Operand) {
    let msk =
        !fill_trailing_bits(op.write_byte_mask) & reg_group_byte_mask(reg.signature.reg_group());
    if msk != 0 {
        op.op_flags |= OpRwFlags::ZEXT;
        op.extend_byte_mask = msk;
    }
}

/// Port of AsmJit's `rw_handle_avx512`: an AVX-512 `{k}` extra register is always read;
/// unless zeroing (`{z}` option or implicit-z instructions) the destination is also read
/// (merge semantics).
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

/// Port of AsmJit's `has_same_reg_type` (only called when all operands are registers).
fn has_same_reg_type(operands: &[Operand]) -> bool {
    debug_assert!(!operands.is_empty());
    let reg_type = operands[0].signature.reg_type();
    operands[1..]
        .iter()
        .all(|op| op.signature.reg_type() == reg_type)
}

/// Applies the same-register hint from [`CommonInfo`] to the query result.
///
/// In AsmJit this transformation lives in `x86rapass.cpp` (a single tied register becomes
/// read-only or write-only when all operands share it); asmkit exposes it at query level:
/// `kRO` clears write access on every operand, `kWO` clears read access (`xor x, x` writes
/// `x` without reading it, including the zero extension).
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

                    // Handle 'pmovmskb|vpmovmskb'.
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

#[cfg(all(test, feature = "x86"))]
mod tests {
    use super::*;
    use crate::core::globals::InstOptions;
    use crate::core::inst::Inst;
    use crate::core::operand::{OperandCast, imm};
    use crate::x86::operands::{Mem, ptr, regs, u64_ptr_abs};

    use alloc::format;
    use alloc::string::String;
    use alloc::vec::Vec;

    fn inst_of(id: InstId, ops: &[Operand]) -> Inst {
        Inst::with_operands(id as u32, ops)
    }

    /// Instruction with an AVX-512 `{k}` extra register (and optionally `{z}`).
    fn inst_k(id: InstId, zeroing: bool, ops: &[Operand]) -> Inst {
        let mut inst = inst_of(id, ops);
        if zeroing {
            inst.options = InstOptions::X86_ZMASK;
        }
        inst.extra_reg = *regs::K1.as_operand();
        inst
    }

    fn vsib() -> Mem {
        Mem::from_base_and_index_shift_disp(&regs::RAX, &regs::XMM1, 2, 0, 0, 0.into())
    }

    /// The same instruction list as the C++ oracle harness
    /// (`target/tmp/x86_rwinfo_oracle.cpp`).
    fn oracle_cases() -> Vec<(&'static str, Inst)> {
        use regs::*;
        alloc::vec![
            // GP arithmetic (partial writes, zext, EFLAGS).
            (
                "add_rax_rcx",
                inst_of(InstId::Add, &[*RAX.as_operand(), *RCX.as_operand()])
            ),
            (
                "add_eax_ecx",
                inst_of(InstId::Add, &[*EAX.as_operand(), *ECX.as_operand()])
            ),
            (
                "add_al_cl",
                inst_of(InstId::Add, &[*AL.as_operand(), *CL.as_operand()])
            ),
            (
                "add_ax_cx",
                inst_of(InstId::Add, &[*AX.as_operand(), *CX.as_operand()])
            ),
            (
                "add_rax_m64",
                inst_of(
                    InstId::Add,
                    &[*RAX.as_operand(), *ptr(RBX, 0, 8).as_operand()]
                )
            ),
            (
                "add_m64_rax",
                inst_of(
                    InstId::Add,
                    &[*ptr(RBX, 0, 8).as_operand(), *RAX.as_operand()]
                )
            ),
            (
                "adc_rax_rcx",
                inst_of(InstId::Adc, &[*RAX.as_operand(), *RCX.as_operand()])
            ),
            (
                "cmp_rax_rcx",
                inst_of(InstId::Cmp, &[*RAX.as_operand(), *RCX.as_operand()])
            ),
            (
                "test_eax_ecx",
                inst_of(InstId::Test, &[*EAX.as_operand(), *ECX.as_operand()])
            ),
            (
                "xor_eax_eax",
                inst_of(InstId::Xor, &[*EAX.as_operand(), *EAX.as_operand()])
            ),
            (
                "xor_rax_rax",
                inst_of(InstId::Xor, &[*RAX.as_operand(), *RAX.as_operand()])
            ),
            (
                "and_eax_eax",
                inst_of(InstId::And, &[*EAX.as_operand(), *EAX.as_operand()])
            ),
            (
                "sub_rax_rax",
                inst_of(InstId::Sub, &[*RAX.as_operand(), *RAX.as_operand()])
            ),
            // mul/imul (implicit dx:ax, all three imul forms).
            ("mul_rbx", inst_of(InstId::Mul, &[*RBX.as_operand()])),
            ("mul_ebx", inst_of(InstId::Mul, &[*EBX.as_operand()])),
            (
                "imul_rax_rcx",
                inst_of(InstId::Imul, &[*RAX.as_operand(), *RCX.as_operand()])
            ),
            (
                "imul_eax_ecx",
                inst_of(InstId::Imul, &[*EAX.as_operand(), *ECX.as_operand()])
            ),
            (
                "imul_ax_cl",
                inst_of(InstId::Imul, &[*AX.as_operand(), *CL.as_operand()])
            ),
            (
                "imul_rax_rcx_5",
                inst_of(
                    InstId::Imul,
                    &[*RAX.as_operand(), *RCX.as_operand(), *imm(5).as_operand()]
                )
            ),
            (
                "imul_dx_ax_bx",
                inst_of(
                    InstId::Imul,
                    &[*DX.as_operand(), *AX.as_operand(), *BX.as_operand()]
                )
            ),
            // Stack & shifts (implicit CL).
            ("push_rax", inst_of(InstId::Push, &[*RAX.as_operand()])),
            ("pop_rax", inst_of(InstId::Pop, &[*RAX.as_operand()])),
            (
                "shl_rax_cl",
                inst_of(InstId::Shl, &[*RAX.as_operand(), *CL.as_operand()])
            ),
            (
                "shl_rax_3",
                inst_of(InstId::Shl, &[*RAX.as_operand(), *imm(3).as_operand()])
            ),
            // Mov family (kMovOp, segment/control regs, moffs, movabs).
            (
                "mov_rax_rcx",
                inst_of(InstId::Mov, &[*RAX.as_operand(), *RCX.as_operand()])
            ),
            (
                "mov_eax_ecx",
                inst_of(InstId::Mov, &[*EAX.as_operand(), *ECX.as_operand()])
            ),
            (
                "mov_rax_m64",
                inst_of(
                    InstId::Mov,
                    &[*RAX.as_operand(), *ptr(RBX, 0, 8).as_operand()]
                )
            ),
            (
                "mov_m64_rax",
                inst_of(
                    InstId::Mov,
                    &[*ptr(RBX, 0, 8).as_operand(), *RAX.as_operand()]
                )
            ),
            (
                "mov_eax_42",
                inst_of(InstId::Mov, &[*EAX.as_operand(), *imm(42).as_operand()])
            ),
            (
                "mov_cr0_rax",
                inst_of(InstId::Mov, &[*CR0.as_operand(), *RAX.as_operand()])
            ),
            (
                "mov_rax_cr0",
                inst_of(InstId::Mov, &[*RAX.as_operand(), *CR0.as_operand()])
            ),
            (
                "mov_ax_es",
                inst_of(InstId::Mov, &[*AX.as_operand(), *ES.as_operand()])
            ),
            (
                "mov_es_ax",
                inst_of(InstId::Mov, &[*ES.as_operand(), *AX.as_operand()])
            ),
            (
                "movabs_rax_m64abs",
                inst_of(
                    InstId::Movabs,
                    &[*RAX.as_operand(), *u64_ptr_abs(0x11223344, 8).as_operand()]
                )
            ),
            (
                "movabs_m64abs_rax",
                inst_of(
                    InstId::Movabs,
                    &[*u64_ptr_abs(0x11223344, 8).as_operand(), *RAX.as_operand()]
                )
            ),
            (
                "movabs_rax_imm",
                inst_of(
                    InstId::Movabs,
                    &[*RAX.as_operand(), *imm(0x1122334455667788i64).as_operand()]
                )
            ),
            // AVX vectors (zext to MAXVL, zmm, EVEX {k} merge vs zeroing).
            (
                "vaddps_xmm",
                inst_of(
                    InstId::Vaddps,
                    &[*XMM0.as_operand(), *XMM1.as_operand(), *XMM2.as_operand()]
                )
            ),
            (
                "vaddps_ymm",
                inst_of(
                    InstId::Vaddps,
                    &[*YMM0.as_operand(), *YMM1.as_operand(), *YMM2.as_operand()]
                )
            ),
            (
                "vaddps_zmm",
                inst_of(
                    InstId::Vaddps,
                    &[*ZMM0.as_operand(), *ZMM1.as_operand(), *ZMM2.as_operand()]
                )
            ),
            (
                "vaddps_xmm_k1",
                inst_k(
                    InstId::Vaddps,
                    false,
                    &[*XMM0.as_operand(), *XMM1.as_operand(), *XMM2.as_operand()]
                )
            ),
            (
                "vaddps_xmm_k1z",
                inst_k(
                    InstId::Vaddps,
                    true,
                    &[*XMM0.as_operand(), *XMM1.as_operand(), *XMM2.as_operand()]
                )
            ),
            (
                "vaddps_zmm_k1z",
                inst_k(
                    InstId::Vaddps,
                    true,
                    &[*ZMM0.as_operand(), *ZMM1.as_operand(), *ZMM2.as_operand()]
                )
            ),
            // Special categories: Vmovddup, Punpcklxx, GenericEx (vpternlogd).
            (
                "vmovddup_xmm_xmm",
                inst_of(InstId::Vmovddup, &[*XMM0.as_operand(), *XMM1.as_operand()])
            ),
            (
                "vmovddup_xmm_m64",
                inst_of(
                    InstId::Vmovddup,
                    &[*XMM0.as_operand(), *ptr(RBX, 0, 8).as_operand()]
                )
            ),
            (
                "vmovddup_ymm_ymm",
                inst_of(InstId::Vmovddup, &[*YMM0.as_operand(), *YMM1.as_operand()])
            ),
            (
                "punpcklbw_xmm_xmm",
                inst_of(InstId::Punpcklbw, &[*XMM0.as_operand(), *XMM1.as_operand()])
            ),
            (
                "punpcklbw_mm_mm",
                inst_of(InstId::Punpcklbw, &[*MM0.as_operand(), *MM1.as_operand()])
            ),
            (
                "vpternlogd_fe",
                inst_of(
                    InstId::Vpternlogd,
                    &[
                        *XMM0.as_operand(),
                        *XMM1.as_operand(),
                        *XMM2.as_operand(),
                        *imm(0xFE).as_operand()
                    ]
                )
            ),
            (
                "vpternlogd_cc",
                inst_of(
                    InstId::Vpternlogd,
                    &[
                        *XMM0.as_operand(),
                        *XMM1.as_operand(),
                        *XMM2.as_operand(),
                        *imm(0xCC).as_operand()
                    ]
                )
            ),
            // cmov (reads EFLAGS), lea.
            (
                "cmovz_rax_rcx",
                inst_of(InstId::Cmovz, &[*RAX.as_operand(), *RCX.as_operand()])
            ),
            (
                "lea_rax_m8",
                inst_of(
                    InstId::Lea,
                    &[*RAX.as_operand(), *ptr(RBX, 8, 0).as_operand()]
                )
            ),
            // Movh64.
            (
                "movhps_xmm_m64",
                inst_of(
                    InstId::Movhps,
                    &[*XMM0.as_operand(), *ptr(RBX, 0, 8).as_operand()]
                )
            ),
            (
                "movhps_m64_xmm",
                inst_of(
                    InstId::Movhps,
                    &[*ptr(RBX, 0, 8).as_operand(), *XMM0.as_operand()]
                )
            ),
            // Gather (VSIB) + EVEX two-op with {k}.
            (
                "vgatherdps_vsib",
                inst_of(
                    InstId::Vgatherdps,
                    &[*XMM0.as_operand(), *vsib().as_operand()]
                )
            ),
            (
                "vpgatherdd_k1_vsib",
                inst_k(
                    InstId::Vpgatherdd,
                    false,
                    &[*XMM0.as_operand(), *vsib().as_operand()]
                )
            ),
            // String ops (implicit zAX/zSI/zDI through RW info).
            (
                "movsb",
                inst_of(
                    InstId::Movs,
                    &[*ptr(RDI, 0, 1).as_operand(), *ptr(RSI, 0, 1).as_operand()]
                )
            ),
            (
                "stosb",
                inst_of(
                    InstId::Stos,
                    &[*ptr(RDI, 0, 1).as_operand(), *AL.as_operand()]
                )
            ),
            (
                "lodsb",
                inst_of(
                    InstId::Lods,
                    &[*AL.as_operand(), *ptr(RSI, 0, 1).as_operand()]
                )
            ),
            (
                "cmpsb",
                inst_of(
                    InstId::Cmps,
                    &[*ptr(RSI, 0, 1).as_operand(), *ptr(RDI, 0, 1).as_operand()]
                )
            ),
            (
                "scasb",
                inst_of(
                    InstId::Scas,
                    &[*AL.as_operand(), *ptr(RDI, 0, 1).as_operand()]
                )
            ),
            // Mask registers, consecutive registers, flag-only instructions.
            (
                "kaddb_k1_k2_k3",
                inst_of(
                    InstId::Kaddb,
                    &[*K1.as_operand(), *K2.as_operand(), *K3.as_operand()]
                )
            ),
            (
                "vp2intersectd_k1k2",
                inst_of(
                    InstId::Vp2intersectd,
                    &[
                        *K1.as_operand(),
                        *K2.as_operand(),
                        *ZMM0.as_operand(),
                        *ZMM1.as_operand()
                    ]
                )
            ),
            ("stc", inst_of(InstId::Stc, &[])),
            ("clc", inst_of(InstId::Clc, &[])),
            // Vmovmskpd / Vmov1_8 / Vmov4_1 / Vmov1_2 / Vmaskmov categories.
            (
                "vmovmskpd_eax",
                inst_of(InstId::Vmovmskpd, &[*EAX.as_operand(), *XMM0.as_operand()])
            ),
            (
                "pmovmskb_eax",
                inst_of(InstId::Pmovmskb, &[*EAX.as_operand(), *XMM0.as_operand()])
            ),
            (
                "vpmovzxbd_xmm",
                inst_of(InstId::Vpmovzxbd, &[*XMM0.as_operand(), *XMM1.as_operand()])
            ),
            (
                "vcvtpd2dq_xmm_ymm",
                inst_of(InstId::Vcvtpd2dq, &[*XMM0.as_operand(), *YMM1.as_operand()])
            ),
            (
                "vmaskmovpd_load",
                inst_of(
                    InstId::Vmaskmovpd,
                    &[
                        *XMM0.as_operand(),
                        *XMM1.as_operand(),
                        *ptr(RBX, 0, 16).as_operand()
                    ]
                )
            ),
            (
                "vmaskmovpd_store",
                inst_of(
                    InstId::Vmaskmovpd,
                    &[
                        *ptr(RBX, 0, 16).as_operand(),
                        *XMM0.as_operand(),
                        *XMM1.as_operand()
                    ]
                )
            ),
            // RWInfoRm flags: Pextrw / FeatureIfRMI / MovssMovsd.
            (
                "pextrw_eax_mm1",
                inst_of(
                    InstId::Pextrw,
                    &[*EAX.as_operand(), *MM1.as_operand(), *imm(1).as_operand()]
                )
            ),
            (
                "pextrw_eax_xmm1",
                inst_of(
                    InstId::Pextrw,
                    &[*EAX.as_operand(), *XMM1.as_operand(), *imm(1).as_operand()]
                )
            ),
            (
                "vpslld_imm",
                inst_of(
                    InstId::Vpslld,
                    &[*XMM1.as_operand(), *XMM2.as_operand(), *imm(8).as_operand()]
                )
            ),
            (
                "vpslld_reg",
                inst_of(
                    InstId::Vpslld,
                    &[*XMM1.as_operand(), *XMM2.as_operand(), *XMM3.as_operand()]
                )
            ),
            (
                "movss_xmm_xmm",
                inst_of(InstId::Movss, &[*XMM0.as_operand(), *XMM1.as_operand()])
            ),
            (
                "movss_xmm_m32",
                inst_of(
                    InstId::Movss,
                    &[*XMM0.as_operand(), *ptr(RBX, 0, 4).as_operand()]
                )
            ),
            (
                "movsd_xmm_xmm",
                inst_of(InstId::Movsd, &[*XMM0.as_operand(), *XMM1.as_operand()])
            ),
        ]
    }

    fn fmt_op(op: &OpRwInfo) -> String {
        format!(
            "(f={:08x} p={:02X} rm={:02x} c={} r={:016x} w={:016x} x={:016x})",
            op.op_flags.bits(),
            op.phys_id,
            op.rm_size,
            op.consecutive_lead_count,
            op.read_byte_mask,
            op.write_byte_mask,
            op.extend_byte_mask
        )
    }

    fn fmt_case(name: &str, rw: &InstRwInfo) -> String {
        let mut s = format!(
            "{} if={:08x} rmf={:02x} rf={:08x} wf={:08x} x={}",
            name,
            rw.inst_flags.bits(),
            rw.rm_feature,
            rw.read_flags.bits(),
            rw.write_flags.bits(),
            fmt_op(&rw.extra_reg)
        );
        for (i, op) in rw.operands().iter().enumerate() {
            s.push_str(&format!(" o{}={}", i, fmt_op(op)));
        }
        s
    }

    /// Text output of the C++ oracle (`x86::InstInternal::query_rw_info` on the pinned
    /// asmjit, plus the same-register hint rule that asmkit folds into the query — see
    /// [`apply_same_reg_hint`]; generated by `target/tmp/x86_rwinfo_oracle`).
    const ORACLE: &[&str] = &[
        "add_rax_rcx if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000007 p=FF rm=08 c=0 r=00000000000000ff w=00000000000000ff x=0000000000000000) o1=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "add_eax_ecx if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000017 p=FF rm=04 c=0 r=000000000000000f w=000000000000000f x=00000000000000f0) o1=(f=00000005 p=FF rm=04 c=0 r=000000000000000f w=0000000000000000 x=0000000000000000)",
        "add_al_cl if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000007 p=FF rm=01 c=0 r=0000000000000001 w=0000000000000001 x=0000000000000000) o1=(f=00000005 p=FF rm=01 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000)",
        "add_ax_cx if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000007 p=FF rm=02 c=0 r=0000000000000003 w=0000000000000003 x=0000000000000000) o1=(f=00000005 p=FF rm=02 c=0 r=0000000000000003 w=0000000000000000 x=0000000000000000)",
        "add_rax_m64 if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000007 p=FF rm=08 c=0 r=00000000000000ff w=00000000000000ff x=0000000000000000) o1=(f=00001001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "add_m64_rax if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00001003 p=FF rm=00 c=0 r=00000000000000ff w=00000000000000ff x=0000000000000000) o1=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "adc_rax_rcx if=00000000 rmf=00 rf=00000002 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000007 p=FF rm=08 c=0 r=00000000000000ff w=00000000000000ff x=0000000000000000) o1=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "cmp_rax_rcx if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000) o1=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "test_eax_ecx if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000005 p=FF rm=04 c=0 r=000000000000000f w=0000000000000000 x=0000000000000000) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000000f w=0000000000000000 x=0000000000000000)",
        "xor_eax_eax if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000016 p=FF rm=04 c=0 r=0000000000000000 w=000000000000000f x=00000000000000f0) o1=(f=00000004 p=FF rm=04 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "xor_rax_rax if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000006 p=FF rm=08 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000004 p=FF rm=08 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "and_eax_eax if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000005 p=FF rm=04 c=0 r=000000000000000f w=0000000000000000 x=0000000000000000) o1=(f=00000005 p=FF rm=04 c=0 r=000000000000000f w=0000000000000000 x=0000000000000000)",
        "sub_rax_rax if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000006 p=FF rm=08 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000004 p=FF rm=08 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "mul_rbx if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000102 p=02 rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000)",
        "mul_ebx if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000112 p=02 rm=00 c=0 r=0000000000000000 w=000000000000000f x=00000000000000f0)",
        "imul_rax_rcx if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000003 p=FF rm=00 c=0 r=00000000000000ff w=00000000000000ff x=0000000000000000) o1=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "imul_eax_ecx if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000013 p=FF rm=00 c=0 r=000000000000000f w=000000000000000f x=00000000000000f0) o1=(f=00000005 p=FF rm=04 c=0 r=000000000000000f w=0000000000000000 x=0000000000000000)",
        "imul_ax_cl if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000103 p=00 rm=00 c=0 r=0000000000000001 w=0000000000000003 x=0000000000000000) o1=(f=00000005 p=FF rm=01 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000)",
        "imul_rax_rcx_5 if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000) o2=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "imul_dx_ax_bx if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000102 p=02 rm=00 c=0 r=0000000000000000 w=0000000000000003 x=0000000000000000) o1=(f=00000103 p=00 rm=00 c=0 r=0000000000000003 w=0000000000000003 x=0000000000000000) o2=(f=00000005 p=FF rm=02 c=0 r=0000000000000003 w=0000000000000000 x=0000000000000000)",
        "push_rax if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "pop_rax if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000006 p=FF rm=08 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000)",
        "shl_rax_cl if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000007 p=FF rm=08 c=0 r=00000000000000ff w=00000000000000ff x=0000000000000000) o1=(f=00000101 p=01 rm=00 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000)",
        "shl_rax_3 if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000007 p=FF rm=08 c=0 r=00000000000000ff w=00000000000000ff x=0000000000000000) o1=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "mov_rax_rcx if=00000001 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000006 p=FF rm=08 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "mov_eax_ecx if=00000001 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000016 p=FF rm=04 c=0 r=0000000000000000 w=000000000000000f x=00000000000000f0) o1=(f=00000005 p=FF rm=04 c=0 r=000000000000000f w=0000000000000000 x=0000000000000000)",
        "mov_rax_m64 if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00005001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "mov_m64_rax if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00005002 p=FF rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "mov_eax_42 if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000016 p=FF rm=04 c=0 r=0000000000000000 w=000000000000000f x=00000000000000f0) o1=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "mov_cr0_rax if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "mov_rax_cr0 if=00000000 rmf=00 rf=00000000 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "mov_ax_es if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000006 p=FF rm=02 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000001 p=FF rm=00 c=0 r=0000000000000003 w=0000000000000000 x=0000000000000000)",
        "mov_es_ax if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000003 x=0000000000000000) o1=(f=00000005 p=FF rm=02 c=0 r=0000000000000003 w=0000000000000000 x=0000000000000000)",
        "movabs_rax_m64abs if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000102 p=00 rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00005001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "movabs_m64abs_rax if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00005002 p=FF rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000101 p=00 rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "movabs_rax_imm if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "vaddps_xmm if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=000000000000ffff x=ffffffffffff0000) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o2=(f=00000005 p=FF rm=10 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000)",
        "vaddps_ymm if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=00000000ffffffff x=ffffffff00000000) o1=(f=00000001 p=FF rm=00 c=0 r=00000000ffffffff w=0000000000000000 x=0000000000000000) o2=(f=00000005 p=FF rm=20 c=0 r=00000000ffffffff w=0000000000000000 x=0000000000000000)",
        "vaddps_zmm if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=ffffffffffffffff x=0000000000000000) o1=(f=00000001 p=FF rm=00 c=0 r=ffffffffffffffff w=0000000000000000 x=0000000000000000) o2=(f=00000005 p=FF rm=40 c=0 r=ffffffffffffffff w=0000000000000000 x=0000000000000000)",
        "vaddps_xmm_k1 if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000) o0=(f=00000013 p=FF rm=00 c=0 r=000000000000ffff w=000000000000ffff x=ffffffffffff0000) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o2=(f=00000005 p=FF rm=10 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000)",
        "vaddps_xmm_k1z if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=000000000000ffff x=ffffffffffff0000) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o2=(f=00000005 p=FF rm=10 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000)",
        "vaddps_zmm_k1z if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=ffffffffffffffff x=0000000000000000) o1=(f=00000001 p=FF rm=00 c=0 r=ffffffffffffffff w=0000000000000000 x=0000000000000000) o2=(f=00000005 p=FF rm=40 c=0 r=ffffffffffffffff w=0000000000000000 x=0000000000000000)",
        "vmovddup_xmm_xmm if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=000000000000ffff x=ffffffffffff0000) o1=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "vmovddup_xmm_m64 if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=000000000000ffff x=ffffffffffff0000) o1=(f=00005001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "vmovddup_ymm_ymm if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=00000000ffffffff x=ffffffff00000000) o1=(f=00000005 p=FF rm=20 c=0 r=0000000000ff00ff w=0000000000000000 x=0000000000000000)",
        "punpcklbw_xmm_xmm if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000003 p=FF rm=00 c=0 r=0000000000000f0f w=000000000000ffff x=0000000000000000) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000f0f x=0000000000000000)",
        "punpcklbw_mm_mm if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000003 p=FF rm=00 c=0 r=000000000000000f w=00000000000000ff x=0000000000000000) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000000f w=0000000000000000 x=0000000000000000)",
        "vpternlogd_fe if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000013 p=FF rm=00 c=0 r=000000000000ffff w=000000000000ffff x=ffffffffffff0000) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o2=(f=00000005 p=FF rm=10 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o3=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "vpternlogd_cc if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=000000000000ffff x=ffffffffffff0000) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o2=(f=00000005 p=FF rm=10 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o3=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "cmovz_rax_rcx if=00000000 rmf=00 rf=00000004 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000003 p=FF rm=00 c=0 r=00000000000000ff w=00000000000000ff x=0000000000000000) o1=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "lea_rax_m8 if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00001001 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "movhps_xmm_m64 if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=000000000000ff00 x=0000000000000000) o1=(f=00005001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
        "movhps_m64_xmm if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00005002 p=FF rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ff00 w=0000000000000000 x=0000000000000000)",
        "vgatherdps_vsib if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000013 p=FF rm=00 c=0 r=000000000000ffff w=000000000000ffff x=ffffffffffff0000) o1=(f=00005001 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "vpgatherdd_k1_vsib if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000) o0=(f=00000013 p=FF rm=00 c=0 r=000000000000ffff w=000000000000ffff x=ffffffffffff0000) o1=(f=00005001 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "movsb if=00000000 rmf=00 rf=00000400 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00023202 p=07 rm=00 c=0 r=0000000000000000 w=0000000000000001 x=0000000000000000) o1=(f=00023201 p=06 rm=00 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000)",
        "stosb if=00000000 rmf=00 rf=00000400 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00023202 p=07 rm=00 c=0 r=0000000000000000 w=0000000000000001 x=0000000000000000) o1=(f=00000101 p=00 rm=00 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000)",
        "lodsb if=00000000 rmf=00 rf=00000400 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000102 p=00 rm=00 c=0 r=0000000000000000 w=0000000000000001 x=0000000000000000) o1=(f=00023201 p=06 rm=00 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000)",
        "cmpsb if=00000000 rmf=00 rf=00000400 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00023201 p=06 rm=00 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000) o1=(f=00023201 p=07 rm=00 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000)",
        "scasb if=00000000 rmf=00 rf=00000400 wf=0000030f x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000101 p=00 rm=00 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000) o1=(f=00023201 p=07 rm=00 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000)",
        "kaddb_k1_k2_k3 if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000001 x=00000000000000fe) o1=(f=00000001 p=FF rm=00 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000) o2=(f=00000001 p=FF rm=00 c=0 r=0000000000000001 w=0000000000000000 x=0000000000000000)",
        "vp2intersectd_k1k2 if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=0000001a p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=00000000000000ff) o2=(f=00000001 p=FF rm=00 c=0 r=ffffffffffffffff w=0000000000000000 x=0000000000000000) o3=(f=00000005 p=FF rm=40 c=0 r=ffffffffffffffff w=0000000000000000 x=0000000000000000)",
        "stc if=00000000 rmf=00 rf=00000000 wf=00000002 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "clc if=00000000 rmf=00 rf=00000000 wf=00000002 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "vmovmskpd_eax if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000002 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000001 x=00000000000000fe) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000)",
        "pmovmskb_eax if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000003 x=00000000000000fc) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000)",
        "vpmovzxbd_xmm if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=000000000000ffff x=ffffffffffff0000) o1=(f=00000005 p=FF rm=04 c=0 r=000000000000000f w=0000000000000000 x=0000000000000000)",
        "vcvtpd2dq_xmm_ymm if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=000000000000ffff x=ffffffffffff0000) o1=(f=00000005 p=FF rm=20 c=0 r=00000000ffffffff w=0000000000000000 x=0000000000000000)",
        "vmaskmovpd_load if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=000000000000ffff x=ffffffffffff0000) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o2=(f=00005001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000)",
        "vmaskmovpd_store if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00005003 p=FF rm=00 c=0 r=000000000000ffff w=000000000000ffff x=0000000000000000) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o2=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000)",
        "pextrw_eax_mm1 if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000003 x=00000000000000fc) o1=(f=00000001 p=FF rm=00 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000) o2=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "pextrw_eax_xmm1 if=00000000 rmf=63 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000016 p=FF rm=02 c=0 r=0000000000000000 w=0000000000000003 x=00000000000000fc) o1=(f=00000001 p=FF rm=00 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o2=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "vpslld_imm if=00000000 rmf=77 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=000000000000ffff x=ffffffffffff0000) o1=(f=00000005 p=FF rm=10 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o2=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000)",
        "vpslld_reg if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000012 p=FF rm=00 c=0 r=0000000000000000 w=000000000000ffff x=ffffffffffff0000) o1=(f=00000005 p=FF rm=10 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000) o2=(f=00000005 p=FF rm=10 c=0 r=000000000000ffff w=0000000000000000 x=0000000000000000)",
        "movss_xmm_xmm if=00000001 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000016 p=FF rm=04 c=0 r=0000000000000000 w=000000000000000f x=0000000000000000) o1=(f=00000005 p=FF rm=04 c=0 r=000000000000000f w=0000000000000000 x=0000000000000000)",
        "movss_xmm_m32 if=00000000 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000016 p=FF rm=04 c=0 r=0000000000000000 w=000000000000000f x=fffffffffffffff0) o1=(f=00001001 p=FF rm=00 c=0 r=000000000000000f w=0000000000000000 x=0000000000000000)",
        "movsd_xmm_xmm if=00000001 rmf=00 rf=00000000 wf=00000000 x=(f=00000000 p=FF rm=00 c=0 r=0000000000000000 w=0000000000000000 x=0000000000000000) o0=(f=00000016 p=FF rm=08 c=0 r=0000000000000000 w=00000000000000ff x=0000000000000000) o1=(f=00000005 p=FF rm=08 c=0 r=00000000000000ff w=0000000000000000 x=0000000000000000)",
    ];

    /// Dumps the replay of the oracle instruction list in the oracle's text format;
    /// kept for regenerating [`ORACLE`] (run with `--ignored --nocapture`).
    #[test]
    #[ignore]
    fn print_oracle_replay() {
        for (name, inst) in oracle_cases() {
            let rw = query_rw_info(&inst).unwrap();
            std::println!("{}", fmt_case(name, &rw));
        }
    }

    /// Replays the full oracle instruction list and compares against the C++ output.
    #[test]
    fn query_matches_cpp_oracle() {
        let cases = oracle_cases();
        assert_eq!(cases.len(), ORACLE.len());
        for ((name, inst), expected) in cases.iter().zip(ORACLE.iter()) {
            let rw = query_rw_info(inst).unwrap();
            let actual = fmt_case(name, &rw);
            assert_eq!(&actual, expected, "case {name}");
        }
    }

    fn query(inst: &Inst) -> InstRwInfo {
        query_rw_info(inst).unwrap()
    }

    #[test]
    fn partial_write_masks() {
        let rw = query(&inst_of(
            InstId::Add,
            &[*regs::AL.as_operand(), *regs::CL.as_operand()],
        ));
        assert_eq!(rw.operands[0].read_byte_mask, 0x1);
        assert_eq!(rw.operands[0].write_byte_mask, 0x1);
        assert!(!rw.operands[0].op_flags.contains(OpRwFlags::ZEXT));

        let rw = query(&inst_of(
            InstId::Add,
            &[*regs::AX.as_operand(), *regs::CX.as_operand()],
        ));
        assert_eq!(rw.operands[0].write_byte_mask, 0x3);
        assert!(!rw.operands[0].op_flags.contains(OpRwFlags::ZEXT));

        // 32-bit GP write zero-extends to 64 bits on X64.
        let rw = query(&inst_of(
            InstId::Add,
            &[*regs::EAX.as_operand(), *regs::ECX.as_operand()],
        ));
        assert_eq!(rw.operands[0].write_byte_mask, 0xF);
        assert!(rw.operands[0].op_flags.contains(OpRwFlags::ZEXT));
        assert_eq!(rw.operands[0].extend_byte_mask, 0xF0);

        let rw = query(&inst_of(
            InstId::Add,
            &[*regs::RAX.as_operand(), *regs::RCX.as_operand()],
        ));
        assert_eq!(rw.operands[0].write_byte_mask, 0xFF);
        assert!(!rw.operands[0].op_flags.contains(OpRwFlags::ZEXT));
    }

    #[test]
    fn implicit_phys_ids() {
        // mul: the implicit form reuses the 3-operand row — the explicit operand maps to
        // the DX slot (AsmJit models `mul`/`div` through the explicit xDX:xAX forms).
        let rw = query(&inst_of(InstId::Mul, &[*regs::RBX.as_operand()]));
        assert!(rw.operands[0].op_flags.contains(OpRwFlags::REG_PHYS_ID));
        assert_eq!(rw.operands[0].phys_id, Gp::DX as u8);

        // imul dx, ax, bx: extended multiplication writes dx:ax.
        let rw = query(&inst_of(
            InstId::Imul,
            &[
                *regs::DX.as_operand(),
                *regs::AX.as_operand(),
                *regs::BX.as_operand(),
            ],
        ));
        assert_eq!(rw.operands[0].phys_id, Gp::DX as u8);
        assert_eq!(rw.operands[1].phys_id, Gp::AX as u8);
        assert!(rw.operands[1].is_read_write());

        // shl rax, cl: the count register must be CL.
        let rw = query(&inst_of(
            InstId::Shl,
            &[*regs::RAX.as_operand(), *regs::CL.as_operand()],
        ));
        assert!(rw.operands[1].op_flags.contains(OpRwFlags::REG_PHYS_ID));
        assert_eq!(rw.operands[1].phys_id, Gp::CX as u8);

        // movsb: [zDI] written, [zSI] read, both with post-modify of the base register.
        let rw = query(&inst_of(
            InstId::Movs,
            &[
                *ptr(regs::RDI, 0, 1).as_operand(),
                *ptr(regs::RSI, 0, 1).as_operand(),
            ],
        ));
        assert!(rw.operands[0].op_flags.contains(OpRwFlags::MEM_PHYS_ID));
        assert_eq!(rw.operands[0].phys_id, Gp::DI as u8);
        assert!(
            rw.operands[0]
                .op_flags
                .contains(OpRwFlags::MEM_BASE_POST_MODIFY)
        );
        assert_eq!(rw.operands[1].phys_id, Gp::SI as u8);
        assert_eq!(rw.read_flags, CpuRwFlags::X86_DF);
    }

    #[test]
    fn eflags_read_write() {
        let rw = query(&inst_of(
            InstId::Add,
            &[*regs::RAX.as_operand(), *regs::RCX.as_operand()],
        ));
        assert_eq!(rw.read_flags, CpuRwFlags::NONE);
        assert_eq!(
            rw.write_flags,
            CpuRwFlags::X86_OF
                | CpuRwFlags::X86_SF
                | CpuRwFlags::X86_ZF
                | CpuRwFlags::X86_AF
                | CpuRwFlags::X86_PF
                | CpuRwFlags::X86_CF
        );

        // adc reads the carry flag.
        let rw = query(&inst_of(
            InstId::Adc,
            &[*regs::RAX.as_operand(), *regs::RCX.as_operand()],
        ));
        assert_eq!(rw.read_flags, CpuRwFlags::X86_CF);

        // cmp: first operand is read-only.
        let rw = query(&inst_of(
            InstId::Cmp,
            &[*regs::RAX.as_operand(), *regs::RCX.as_operand()],
        ));
        assert!(rw.operands[0].is_read_only());
        assert!(rw.operands[1].is_read_only());

        // stc/clc write CF only.
        for id in [InstId::Stc, InstId::Clc] {
            let rw = query(&inst_of(id, &[]));
            assert_eq!(rw.op_count, 0);
            assert_eq!(rw.write_flags, CpuRwFlags::X86_CF);
        }

        // cmov reads flags according to its condition (cmovz reads ZF).
        let rw = query(&inst_of(
            InstId::Cmovz,
            &[*regs::RAX.as_operand(), *regs::RCX.as_operand()],
        ));
        assert_eq!(rw.read_flags, CpuRwFlags::X86_ZF);
    }

    #[test]
    fn mov_op_and_zext() {
        // Register-to-register move of the same type keeps kMovOp.
        let rw = query(&inst_of(
            InstId::Mov,
            &[*regs::EAX.as_operand(), *regs::ECX.as_operand()],
        ));
        assert!(rw.is_mov_op());
        assert!(rw.operands[0].op_flags.contains(OpRwFlags::ZEXT));
        assert_eq!(rw.operands[0].extend_byte_mask, 0xF0);

        let rw = query(&inst_of(
            InstId::Mov,
            &[*regs::RAX.as_operand(), *regs::RCX.as_operand()],
        ));
        assert!(rw.is_mov_op());
        assert!(rw.operands[0].is_write_only());

        // movss xmm, xmm is also a same-type reg-reg move, but doesn't zero-extend.
        let rw = query(&inst_of(
            InstId::Movss,
            &[*regs::XMM0.as_operand(), *regs::XMM1.as_operand()],
        ));
        assert!(rw.is_mov_op());
        assert_eq!(rw.operands[0].extend_byte_mask, 0);

        // Not a move: memory form and immediate form lose kMovOp.
        let rw = query(&inst_of(
            InstId::Mov,
            &[*regs::EAX.as_operand(), *imm(42).as_operand()],
        ));
        assert!(!rw.is_mov_op());
    }

    #[test]
    fn same_reg_hints() {
        // xor x, x writes x without reading it (WO hint).
        let rw = query(&inst_of(
            InstId::Xor,
            &[*regs::EAX.as_operand(), *regs::EAX.as_operand()],
        ));
        assert!(rw.operands[0].is_write_only());
        assert_eq!(rw.operands[0].read_byte_mask, 0);
        assert_eq!(rw.operands[0].write_byte_mask, 0xF);
        // The zero extension still happens.
        assert!(rw.operands[0].op_flags.contains(OpRwFlags::ZEXT));
        assert_eq!(rw.operands[0].extend_byte_mask, 0xF0);

        // and x, x reads x without changing it (RO hint).
        let rw = query(&inst_of(
            InstId::And,
            &[*regs::EAX.as_operand(), *regs::EAX.as_operand()],
        ));
        assert!(rw.operands[0].is_read_only());
        assert_eq!(rw.operands[0].write_byte_mask, 0);
        assert_eq!(rw.operands[0].extend_byte_mask, 0);

        // Different registers: no hint applied.
        let rw = query(&inst_of(
            InstId::Xor,
            &[*regs::EAX.as_operand(), *regs::ECX.as_operand()],
        ));
        assert!(rw.operands[0].is_read_write());
    }

    #[test]
    fn avx512_mask_merge_vs_zero() {
        // Merge masking {k}: the extra register is read and the destination becomes RW.
        let rw = query(&inst_k(
            InstId::Vaddps,
            false,
            &[
                *regs::XMM0.as_operand(),
                *regs::XMM1.as_operand(),
                *regs::XMM2.as_operand(),
            ],
        ));
        assert!(rw.extra_reg.is_read_only());
        assert_eq!(rw.extra_reg.read_byte_mask, 0xFF);
        assert!(rw.operands[0].is_read_write());
        assert_eq!(rw.operands[0].read_byte_mask, 0xFFFF);

        // Zeroing {k}{z}: the destination stays write-only.
        let rw = query(&inst_k(
            InstId::Vaddps,
            true,
            &[
                *regs::XMM0.as_operand(),
                *regs::XMM1.as_operand(),
                *regs::XMM2.as_operand(),
            ],
        ));
        assert!(rw.extra_reg.is_read_only());
        assert!(rw.operands[0].is_write_only());
        assert_eq!(rw.operands[0].read_byte_mask, 0);
        // And the AVX zero extension of the upper lanes is still tracked.
        assert_eq!(rw.operands[0].extend_byte_mask, 0xFFFFFFFFFFFF0000);
    }

    #[test]
    fn avx_vector_zext() {
        let rw = query(&inst_of(
            InstId::Vaddps,
            &[
                *regs::XMM0.as_operand(),
                *regs::XMM1.as_operand(),
                *regs::XMM2.as_operand(),
            ],
        ));
        assert!(rw.operands[0].op_flags.contains(OpRwFlags::ZEXT));
        assert_eq!(rw.operands[0].extend_byte_mask, 0xFFFFFFFFFFFF0000);

        // Full-width zmm writes don't zero-extend (nothing left to extend).
        let rw = query(&inst_of(
            InstId::Vaddps,
            &[
                *regs::ZMM0.as_operand(),
                *regs::ZMM1.as_operand(),
                *regs::ZMM2.as_operand(),
            ],
        ));
        assert!(!rw.operands[0].op_flags.contains(OpRwFlags::ZEXT));
        assert_eq!(rw.operands[0].extend_byte_mask, 0);
    }

    #[test]
    fn consecutive_vp2intersect() {
        let rw = query(&inst_of(
            InstId::Vp2intersectd,
            &[
                *regs::K1.as_operand(),
                *regs::K2.as_operand(),
                *regs::ZMM0.as_operand(),
                *regs::ZMM1.as_operand(),
            ],
        ));
        // The second K register is the consecutive (+1) register of the pair.
        assert!(rw.operands[1].op_flags.contains(OpRwFlags::CONSECUTIVE));
        assert!(rw.operands[0].is_write_only());
        assert!(rw.operands[2].is_read_only());
        assert!(rw.operands[3].is_read_only());
    }

    #[test]
    fn lea_mem_base_read() {
        // kMemFake is vestigial in current AsmJit (nothing sets it, not even LEA); the
        // LEA memory operand reports just a base register read and no data access.
        let rw = query(&inst_of(
            InstId::Lea,
            &[*regs::RAX.as_operand(), *ptr(regs::RBX, 8, 0).as_operand()],
        ));
        assert!(rw.operands[0].is_write_only());
        let mem = &rw.operands[1];
        assert!(mem.op_flags.contains(OpRwFlags::MEM_BASE_READ));
        assert!(!mem.op_flags.contains(OpRwFlags::MEM_FAKE));
        assert_eq!(mem.read_byte_mask, 0);
        assert_eq!(mem.write_byte_mask, 0);
    }

    #[test]
    fn vsib_gather_mem_flags() {
        let rw = query(&inst_of(
            InstId::Vgatherdps,
            &[*regs::XMM0.as_operand(), *vsib().as_operand()],
        ));
        let mem = &rw.operands[1];
        assert!(mem.op_flags.contains(OpRwFlags::MEM_BASE_READ));
        assert!(mem.op_flags.contains(OpRwFlags::MEM_INDEX_READ));
    }

    #[test]
    fn rm_feature_gating() {
        // pextrw with an MM register: no feature required for the RM replacement.
        let rw = query(&inst_of(
            InstId::Pextrw,
            &[
                *regs::EAX.as_operand(),
                *regs::MM1.as_operand(),
                *imm(1).as_operand(),
            ],
        ));
        assert_eq!(rw.rm_feature, 0);

        // pextrw with an XMM register: SSE4.1 is required for the memory form.
        let rw = query(&inst_of(
            InstId::Pextrw,
            &[
                *regs::EAX.as_operand(),
                *regs::XMM1.as_operand(),
                *imm(1).as_operand(),
            ],
        ));
        assert_eq!(
            rw.rm_feature,
            super::super::instdb::CpuFeature::SSE4_1 as u8
        );

        // AVX shift with immediate: RM replacement requires AVX512_F.
        let rw = query(&inst_of(
            InstId::Vpslld,
            &[
                *regs::XMM1.as_operand(),
                *regs::XMM2.as_operand(),
                *imm(8).as_operand(),
            ],
        ));
        assert_eq!(
            rw.rm_feature,
            super::super::instdb::CpuFeature::AVX512_F as u8
        );

        // Same instruction with a register count: no feature required.
        let rw = query(&inst_of(
            InstId::Vpslld,
            &[
                *regs::XMM1.as_operand(),
                *regs::XMM2.as_operand(),
                *regs::XMM3.as_operand(),
            ],
        ));
        assert_eq!(rw.rm_feature, 0);
    }

    #[test]
    fn vpternlogd_predicate_analysis() {
        // Predicate 0xCC: high nibble equals low nibble — destination is not read.
        let rw = query(&inst_of(
            InstId::Vpternlogd,
            &[
                *regs::XMM0.as_operand(),
                *regs::XMM1.as_operand(),
                *regs::XMM2.as_operand(),
                *imm(0xCC).as_operand(),
            ],
        ));
        assert!(rw.operands[0].is_write_only());
        assert_eq!(rw.operands[0].read_byte_mask, 0);

        // Predicate 0xFE: destination value participates.
        let rw = query(&inst_of(
            InstId::Vpternlogd,
            &[
                *regs::XMM0.as_operand(),
                *regs::XMM1.as_operand(),
                *regs::XMM2.as_operand(),
                *imm(0xFE).as_operand(),
            ],
        ));
        assert!(rw.operands[0].is_read_write());
    }

    #[test]
    fn instruction_id_bounds() {
        // Id 0 (`InstId::None`) is "defined" in AsmJit's sense (`inst_id < kIdCount`) and
        // maps to an empty record, like in AsmJit.
        assert!(query_rw_info(&inst_of(InstId::None, &[])).is_ok());
        // Out-of-range ids are rejected.
        let inst = Inst::new(u32::MAX);
        assert!(query_rw_info(&inst).is_err());
    }
}
