//! RISC-V instruction API: read/write information queries.
//!
//! Per-operand access patterns come from the generated [`super::instdb`] tables
//! (pattern-indexed like the AArch64 `RW_PATTERN_TABLE`), driven by the operand
//! order `Assembler::emit_n` consumes. Modeling specifics:
//!
//! - Loads/stores/AMOs have no explicit memory operand: the address base register
//!   (rs1) carries `OpRwFlags::READ | MEM_BASE_READ`, and the memory access itself
//!   is described by `InstRwInfo::extra_reg` (direction in `op_flags`, access width
//!   in `rm_size`, byte masks for the accessed bytes; width 0 means the size is not
//!   instruction-fixed, e.g. whole-register vector loads).
//! - The `csr` immediate operand of CSR instructions keeps R/W bits describing the
//!   CSR access (other immediates have no effects).
//! - Implicit CSR flag effects land in `read_flags`/`write_flags`
//!   (`CpuRwFlags::RISCV_FFLAGS`/`RISCV_FRM`/`RISCV_VXSAT`).
//! - Implicit fixed-register effects (ra/sp/v0) do not fit `InstRwInfo`; query them
//!   per opcode via [`Opcode::implicit_reg_effects`].
//!
//! Derived from riscv-opcodes (BSD-3-Clause); see meta/riscv.py.

use crate::AsmError;
use crate::core::arch_traits::Arch;
use crate::core::inst::Inst;
use crate::core::operand::Operand;
use crate::core::rwinfo::{CpuRwFlags, INVALID_PHYS_ID, InstRwInfo, OpRwFlags, OpRwInfo};

use super::instdb::{self, INST_INFO_TABLE, RW_PATTERN_TABLE, SIGNATURE_TABLE};
use super::operands::Reg;

/// Queries read/write information of the given instruction.
///
/// Returns [`AsmError::InvalidInstruction`] if the opcode is not defined.
pub fn query_rw_info(inst: &Inst) -> Result<InstRwInfo, AsmError> {
    if !matches!(inst.arch(), Arch::RISCV32 | Arch::RISCV64) {
        return Err(AsmError::InvalidArch);
    }
    let Some(info) = INST_INFO_TABLE.get(inst.id as usize) else {
        return Err(AsmError::InvalidInstruction);
    };

    let operands = inst.operands();
    let mut out = InstRwInfo::new();
    out.op_count = operands.len() as u8;
    out.read_flags = CpuRwFlags::from_bits_retain(info.read_flags);
    out.write_flags = CpuRwFlags::from_bits_retain(info.write_flags);

    let pattern = &RW_PATTERN_TABLE[info.rw_info_index as usize];
    let signature = &SIGNATURE_TABLE[info.signature_index as usize];
    for (i, src) in operands.iter().enumerate() {
        debug_assert!(
            class_matches(signature[i], src),
            "operand {i} class mismatch for opcode {}",
            super::opcodes::OPCODE_STR[inst.id as usize],
        );
        let rw = pattern[i];
        let op = &mut out.operands[i];
        if src.is_reg() {
            fill(op, rw);
        } else if rw != instdb::NONE {
            // Immediate operand that carries effects: the addressed CSR of CSR
            // instructions (R/W bits describe the CSR access; no byte masks).
            op.op_flags = OpRwFlags::from_bits_retain(rw) & OpRwFlags::RW;
            op.phys_id = INVALID_PHYS_ID;
        }
    }

    // The implicit memory access of loads/stores/AMOs (see module docs).
    match info.mem_access {
        instdb::MEM_NONE => {}
        instdb::MEM_LOAD => mem_access(&mut out.extra_reg, OpRwFlags::READ, info.mem_width),
        instdb::MEM_STORE => mem_access(&mut out.extra_reg, OpRwFlags::WRITE, info.mem_width),
        instdb::MEM_READ_MODIFY_WRITE => {
            mem_access(&mut out.extra_reg, OpRwFlags::RW, info.mem_width)
        }
        _ => unreachable!("generated table carries only MEM_* values"),
    }

    Ok(out)
}

/// Fills `op` from a read/write pattern value (full byte masks, no phys id).
fn fill(op: &mut OpRwInfo, rw: u32) {
    let flags = OpRwFlags::from_bits_retain(rw) & !OpRwFlags::ZEXT;
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

/// Fills `op` with an implicit memory access description (see module docs).
fn mem_access(op: &mut OpRwInfo, access: OpRwFlags, width: u8) {
    let byte_mask = if width == 0 { 0 } else { (1u64 << width) - 1 };
    op.op_flags = access;
    op.phys_id = INVALID_PHYS_ID;
    op.rm_size = width;
    op.consecutive_lead_count = 0;
    op.read_byte_mask = if access.contains(OpRwFlags::READ) {
        byte_mask
    } else {
        0
    };
    op.write_byte_mask = if access.contains(OpRwFlags::WRITE) {
        byte_mask
    } else {
        0
    };
    op.extend_byte_mask = 0;
}

/// Tests an operand against a signature class (`ANY`/`GP`/`FP`/`VEC`/`IMM`).
fn class_matches(class: u8, op: &Operand) -> bool {
    match class {
        instdb::GP | instdb::FP | instdb::VEC => {
            if !op.is_reg() {
                return false;
            }
            let reg = op.as_::<Reg>();
            match class {
                instdb::GP => reg.is_gp(),
                instdb::FP => reg.is_fp(),
                _ => reg.is_vec(),
            }
        }
        instdb::IMM => op.is_imm() || op.is_label() || op.is_sym(),
        _ => true,
    }
}
