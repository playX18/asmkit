//! AArch64 instruction API: read/write information queries.
//!
//! Port of AsmJit's `a64instapi.cpp` (`InstInternal::query_rw_info`). Per-operand access
//! patterns come from a small pattern table indexed by [`InstInfo::rw_info_index`]; memory
//! operands get base/index access flags (incl. pre/post-index write-back) and vector element
//! accesses (`vN.T[i]`) narrow the byte masks to the addressed element.
//!
//! NZCV (PSTATE) flag effects are not part of AsmJit's query; asmkit generates them from the
//! instruction database `io` attributes (see `meta/asmjit_db/`) into
//! [`super::rwflags::RW_FLAGS_TABLE`], looked up here by real id. Flags are unioned across all
//! database forms of a name, so e.g. `b` conservatively reads NZCV via its conditional form.
//!
//! Derived from AsmJit (Zlib license) — this file is an altered version; see LICENSE notices.

use crate::AsmError;
use crate::core::inst::Inst;
use crate::core::rwinfo::{CpuRwFlags, INVALID_PHYS_ID, InstRwInfo, OpRwFlags, OpRwInfo};

use super::instdb::{INST_INFO_TABLE, InstFlag, InstId};
use super::operands::{Mem, Vec};

/// Operand access patterns indexed by `InstInfo::rw_info_index` (AsmJit's
/// `inst_rw_info_table`). Values are [`OpRwFlags`] read/write bits per operand position.
#[rustfmt::skip]
mod rw_pattern {
    pub const R: u8 = crate::core::rwinfo::OpRwFlags::READ.bits() as u8;
    pub const W: u8 = crate::core::rwinfo::OpRwFlags::WRITE.bits() as u8;
    pub const X: u8 = crate::core::rwinfo::OpRwFlags::RW.bits() as u8;
}

use rw_pattern::{R, W, X};

#[rustfmt::skip]
const RW_PATTERN_TABLE: [[u8; 6]; 17] = [
    [R, R, R, R, R, R], // RWI_R
    [R, W, R, R, R, R], // RWI_RW
    [R, X, R, R, R, R], // RWI_RX
    [R, R, W, R, R, R], // RWI_RRW
    [R, W, X, R, R, R], // RWI_RWX
    [W, R, R, R, R, R], // RWI_W
    [W, R, W, R, R, R], // RWI_WRW
    [W, R, X, R, R, R], // RWI_WRX
    [W, R, R, W, R, R], // RWI_WRRW
    [W, R, R, X, R, R], // RWI_WRRX
    [W, W, R, R, R, R], // RWI_WW
    [X, R, R, R, R, R], // RWI_X
    [X, R, X, R, R, R], // RWI_XRX
    [X, X, R, R, X, R], // RWI_XXRRX
    [W, R, R, R, R, R], // RWI_LDN
    [R, W, R, R, R, R], // RWI_STN
    [R, R, R, R, R, R], // RWI_TODO
];

/// Element size in bytes indexed by [`super::operands::VecElementType`] discriminant.
const ELEMENT_TYPE_SIZE: [u32; 8] = [0, 1, 2, 4, 8, 4, 4, 0];

/// Queries read/write information of the given instruction.
///
/// Returns [`AsmError::InvalidInstruction`] if the instruction id is not defined.
pub fn query_rw_info(inst: &Inst) -> Result<InstRwInfo, AsmError> {
    let real_id = InstId::extract_real_id(inst.id) as usize;
    if real_id >= INST_INFO_TABLE.len() {
        return Err(AsmError::InvalidInstruction);
    }

    let operands = inst.operands();
    let op_count = operands.len();

    let mut out = InstRwInfo::new();
    out.op_count = op_count as u8;
    debug_assert_eq!(super::rwflags::RW_FLAGS_TABLE.len(), INST_INFO_TABLE.len());
    let (read_flags, write_flags) = super::rwflags::RW_FLAGS_TABLE[real_id];
    out.read_flags = CpuRwFlags::from_bits_retain(read_flags);
    out.write_flags = CpuRwFlags::from_bits_retain(write_flags);

    let inst_info = &INST_INFO_TABLE[real_id];
    let rw_info = &RW_PATTERN_TABLE[inst_info.rw_info_index as usize];

    if inst_info.flags & (InstFlag::Consecutive as u16) != 0 && op_count > 2 {
        for (i, src) in operands.iter().enumerate() {
            let op = &mut out.operands[i];
            if !src.is_reg_or_mem() {
                *op = OpRwInfo::new();
                continue;
            }

            fill(
                op,
                if i < op_count - 1 {
                    rw_info[0]
                } else {
                    rw_info[1]
                },
            );

            if src.is_reg() {
                if i == 0 {
                    op.consecutive_lead_count = (op_count - 1) as u8;
                } else {
                    op.op_flags |= OpRwFlags::CONSECUTIVE;
                }
            } else {
                mem_flags(op, &src.as_::<Mem>());
            }
        }
    } else {
        for (i, src) in operands.iter().enumerate() {
            let op = &mut out.operands[i];
            if !src.is_reg_or_mem() {
                *op = OpRwInfo::new();
                continue;
            }

            fill(op, rw_info[i]);

            if src.is_reg() {
                let vec = src.as_::<Vec>();
                if vec.has_element_index() {
                    // Only part of the vector is accessed if element index [] is used.
                    let element_size = ELEMENT_TYPE_SIZE[vec.element_type() as usize];
                    let access_mask =
                        ((1u64 << element_size) - 1) << (vec.element_index() * element_size);
                    op.read_byte_mask &= access_mask;
                    op.write_byte_mask &= access_mask;
                }
            } else {
                mem_flags(op, &src.as_::<Mem>());
            }
        }
    }

    Ok(out)
}

/// Fills `op` from a read/write pattern byte (mirrors the AsmJit query: full byte masks,
/// no zext, no phys id).
fn fill(op: &mut OpRwInfo, rw: u8) {
    let flags = OpRwFlags::from_bits_retain(rw as u32) & !OpRwFlags::ZEXT;
    op.op_flags = flags;
    op.phys_id = INVALID_PHYS_ID;
    op.rm_size = 0;
    op.consecutive_lead_count = 0;
    op.read_byte_mask = if flags.contains(OpRwFlags::READ) {
        u64::MAX
    } else {
        0
    };
    op.write_byte_mask = if flags.contains(OpRwFlags::WRITE) {
        u64::MAX
    } else {
        0
    };
    op.extend_byte_mask = 0;
}

/// Adds memory base/index access flags (incl. pre/post-index write-back) to `op`.
fn mem_flags(op: &mut OpRwInfo, mem: &Mem) {
    if mem.has_base() {
        op.op_flags |= OpRwFlags::MEM_BASE_READ;
        if (mem.has_index() || mem.has_offset()) && mem.is_pre_or_post() {
            op.op_flags |= OpRwFlags::MEM_BASE_WRITE;
        }
    }
    if mem.has_index() {
        op.op_flags |= OpRwFlags::MEM_INDEX_READ;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aarch64::operands::*;
    use crate::core::inst::Inst;
    use crate::core::operand::OperandCast;
    use crate::core::rwinfo::CpuRwFlags;

    fn inst_of(id: InstId, ops: &[crate::core::operand::Operand]) -> Inst {
        Inst::with_operands(id as u32, ops)
    }

    fn nzcv() -> CpuRwFlags {
        CpuRwFlags::A64_N | CpuRwFlags::A64_Z | CpuRwFlags::A64_C | CpuRwFlags::A64_V
    }

    #[test]
    fn add_gp() {
        let inst = inst_of(
            InstId::Add,
            &[*x0.as_operand(), *x1.as_operand(), *x2.as_operand()],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert_eq!(rw.op_count, 3);
        assert!(rw.operands[0].is_write_only());
        assert_eq!(rw.operands[0].write_byte_mask, u64::MAX);
        assert!(rw.operands[1].is_read_only());
        assert!(rw.operands[2].is_read_only());
        assert_eq!(rw.operands[0].phys_id, INVALID_PHYS_ID);
    }

    #[test]
    fn casp_consecutive() {
        let inst = inst_of(
            InstId::Casp,
            &[
                *x0.as_operand(),
                *x1.as_operand(),
                *x2.as_operand(),
                *x3.as_operand(),
                *ptr(x4, 0).as_operand(),
            ],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert_eq!(rw.operands[0].consecutive_lead_count, 4);
        for op in &rw.operands[1..4] {
            assert!(op.op_flags.contains(OpRwFlags::CONSECUTIVE));
            assert!(op.is_read_write());
        }
        let mem = &rw.operands[4];
        assert!(mem.op_flags.contains(OpRwFlags::MEM_BASE_READ));
        assert!(!mem.op_flags.contains(OpRwFlags::MEM_BASE_WRITE));
    }

    #[test]
    fn str_pre_index_writeback() {
        let inst = inst_of(
            InstId::Str,
            &[*x0.as_operand(), *ptr(x1, 0).pre_offset(-16).as_operand()],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert!(rw.operands[0].is_read_only());
        let mem = &rw.operands[1];
        assert!(mem.is_write_only());
        assert!(mem.op_flags.contains(OpRwFlags::MEM_BASE_READ));
        assert!(mem.op_flags.contains(OpRwFlags::MEM_BASE_WRITE));
    }

    #[test]
    fn vector_element_access_narrows_masks() {
        let v = Vec::make_v128_with_element_index(VecElementType::S, 2, 0);
        let inst = inst_of(
            InstId::Add,
            &[*v.as_operand(), *x1.as_operand(), *x2.as_operand()],
        );
        let rw = query_rw_info(&inst).unwrap();
        // 4-byte S element at index 2: byte mask 0xF << 8.
        assert_eq!(rw.operands[0].write_byte_mask, 0xF00);
    }

    #[test]
    fn adds_gp() {
        let inst = inst_of(
            InstId::Adds,
            &[*x0.as_operand(), *x1.as_operand(), *x2.as_operand()],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert_eq!(rw.op_count, 3);
        assert!(rw.operands[0].is_write_only());
        assert!(rw.operands[1].is_read_only());
        assert!(rw.operands[2].is_read_only());
        assert_eq!(rw.read_flags, CpuRwFlags::NONE);
        assert_eq!(rw.write_flags, nzcv());
    }

    #[test]
    fn b_cond_label() {
        let inst = Inst::with_operands(
            InstId::B.with_cc(crate::core::globals::CondCode::EQ),
            &[*crate::core::operand::imm(0).as_operand()],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert_eq!(rw.op_count, 1);
        assert!(
            !rw.operands[0]
                .op_flags
                .intersects(OpRwFlags::READ | OpRwFlags::WRITE)
        );
        assert_eq!(rw.read_flags, nzcv());
        assert_eq!(rw.write_flags, CpuRwFlags::NONE);
    }

    #[test]
    fn bl_label() {
        let inst = inst_of(InstId::Bl, &[*crate::core::operand::imm(0).as_operand()]);
        let rw = query_rw_info(&inst).unwrap();
        assert_eq!(rw.op_count, 1);
        assert!(
            !rw.operands[0]
                .op_flags
                .intersects(OpRwFlags::READ | OpRwFlags::WRITE)
        );
    }

    #[test]
    fn ldp_pair() {
        let inst = inst_of(
            InstId::Ldp,
            &[*x0.as_operand(), *x1.as_operand(), *ptr(x2, 0).as_operand()],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert!(rw.operands[0].is_write_only());
        assert!(rw.operands[1].is_write_only());
        let mem = &rw.operands[2];
        assert!(mem.is_read_only());
        assert!(mem.op_flags.contains(OpRwFlags::MEM_BASE_READ));
        assert!(!mem.op_flags.contains(OpRwFlags::MEM_BASE_WRITE));
    }

    #[test]
    fn stp_pair() {
        let inst = inst_of(
            InstId::Stp,
            &[*x0.as_operand(), *x1.as_operand(), *ptr(x2, 0).as_operand()],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert!(rw.operands[0].is_read_only());
        assert!(rw.operands[1].is_read_only());
        let mem = &rw.operands[2];
        assert!(mem.is_write_only());
        assert!(mem.op_flags.contains(OpRwFlags::MEM_BASE_READ));
        assert!(!mem.op_flags.contains(OpRwFlags::MEM_BASE_WRITE));
    }

    #[test]
    fn cfinv_carry() {
        let inst = Inst::with_operands(InstId::Cfinv as u32, &[]);
        let rw = query_rw_info(&inst).unwrap();
        assert_eq!(rw.read_flags, CpuRwFlags::A64_C);
        assert_eq!(rw.write_flags, CpuRwFlags::A64_C);
    }
}
