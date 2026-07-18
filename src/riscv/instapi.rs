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
use crate::core::inst::Inst;
use crate::core::operand::Operand;
use crate::core::rwinfo::{CpuRwFlags, INVALID_PHYS_ID, InstRwInfo, OpRwFlags, OpRwInfo};

use super::instdb::{self, INST_INFO_TABLE, RW_PATTERN_TABLE, SIGNATURE_TABLE};
use super::operands::Reg;

/// Queries read/write information of the given instruction.
///
/// Returns [`AsmError::InvalidInstruction`] if the opcode is not defined.
pub fn query_rw_info(inst: &Inst) -> Result<InstRwInfo, AsmError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::buffer::CodeBuffer;
    use crate::core::builder::Builder;
    use crate::core::operand::{OperandCast, imm};
    use crate::riscv::assembler::Assembler;
    use crate::riscv::emitter::EmitterExplicit;
    use crate::riscv::opcodes::Opcode;
    use crate::riscv::operands::regs::*;
    use std::vec::Vec;

    fn inst_of(op: Opcode, ops: &[Operand]) -> Inst {
        Inst::with_operands(op as u32, ops)
    }

    #[test]
    fn add_w_rr() {
        let inst = inst_of(
            Opcode::ADD,
            &[*A0.as_operand(), *A1.as_operand(), *A2.as_operand()],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert_eq!(rw.op_count, 3);
        assert!(rw.operands[0].is_write_only());
        assert!(rw.operands[1].is_read_only());
        assert!(rw.operands[2].is_read_only());
        assert_eq!(rw.operands[0].write_byte_mask, u64::MAX);
        assert_eq!(rw.operands[0].phys_id, INVALID_PHYS_ID);
        assert_eq!(rw.read_flags, CpuRwFlags::NONE);
        assert_eq!(rw.write_flags, CpuRwFlags::NONE);
    }

    #[test]
    fn lw_loads_four_bytes() {
        let inst = inst_of(
            Opcode::LW,
            &[*A0.as_operand(), *A1.as_operand(), *imm(0).as_operand()],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert!(rw.operands[0].is_write_only());
        assert!(rw.operands[1].is_read_only());
        assert!(rw.operands[1].op_flags.contains(OpRwFlags::MEM_BASE_READ));
        assert_eq!(rw.operands[2].op_flags, OpRwFlags::NONE);
        let mem = &rw.extra_reg;
        assert!(mem.is_read_only());
        assert_eq!(mem.rm_size, 4);
        assert_eq!(mem.read_byte_mask, 0xF);
    }

    #[test]
    fn sw_stores_four_bytes() {
        let inst = inst_of(
            Opcode::SW,
            &[*A1.as_operand(), *A2.as_operand(), *imm(0).as_operand()],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert!(rw.operands[0].is_read_only());
        assert!(rw.operands[0].op_flags.contains(OpRwFlags::MEM_BASE_READ));
        assert!(rw.operands[1].is_read_only());
        let mem = &rw.extra_reg;
        assert!(mem.is_write_only());
        assert_eq!(mem.rm_size, 4);
        assert_eq!(mem.write_byte_mask, 0xF);
    }

    #[test]
    fn csrrw_accesses_csr() {
        let inst = inst_of(
            Opcode::CSRRW,
            &[*A0.as_operand(), *A1.as_operand(), *imm(0x001).as_operand()],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert!(rw.operands[0].is_write_only());
        assert!(rw.operands[1].is_read_only());
        // The csr immediate carries the CSR access itself (read-write).
        assert!(rw.operands[2].is_read_write());
        assert_eq!(rw.operands[2].read_byte_mask, 0);
    }

    #[test]
    fn amoadd_w_is_read_modify_write() {
        let inst = inst_of(
            Opcode::AMOADDW,
            &[
                *A0.as_operand(),
                *A1.as_operand(),
                *A2.as_operand(),
                *imm(0).as_operand(),
                *imm(0).as_operand(),
            ],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert!(rw.operands[0].is_write_only());
        assert!(rw.operands[1].op_flags.contains(OpRwFlags::MEM_BASE_READ));
        assert!(rw.operands[2].is_read_only());
        let mem = &rw.extra_reg;
        assert!(mem.is_read_write());
        assert_eq!(mem.rm_size, 4);
        assert_eq!(mem.read_byte_mask, 0xF);
        assert_eq!(mem.write_byte_mask, 0xF);
    }

    #[test]
    fn c_jal_writes_ra_implicitly() {
        let inst = inst_of(Opcode::CJAL, &[*imm(0).as_operand()]);
        let rw = query_rw_info(&inst).unwrap();
        assert_eq!(rw.operands[0].op_flags, OpRwFlags::NONE);
        let implicit = Opcode::CJAL.implicit_reg_effects();
        assert_eq!(implicit.write & (1 << 1), 1 << 1, "c.jal links to ra (x1)");
        assert_eq!(implicit.read, 0);
    }

    #[test]
    fn c_addi16sp_uses_def_sp() {
        let implicit = Opcode::CADDI16SP.implicit_reg_effects();
        assert_eq!(implicit.read & (1 << 2), 1 << 2, "c.addi16sp reads sp (x2)");
        assert_eq!(
            implicit.write & (1 << 2),
            1 << 2,
            "c.addi16sp writes sp (x2)"
        );
    }

    #[test]
    fn fadd_d_writes_fflags_reads_frm() {
        let inst = inst_of(
            Opcode::FADDD,
            &[
                *F10.as_operand(),
                *F11.as_operand(),
                *F12.as_operand(),
                *imm(7).as_operand(),
            ],
        );
        let rw = query_rw_info(&inst).unwrap();
        assert!(rw.operands[0].is_write_only());
        assert!(rw.write_flags.contains(CpuRwFlags::RISCV_FFLAGS));
        assert!(rw.read_flags.contains(CpuRwFlags::RISCV_FRM));
    }

    #[test]
    fn masked_vector_reads_v0_implicitly() {
        let implicit = Opcode::VADDVV.implicit_reg_effects();
        assert_eq!(implicit.read >> 32, 1, "masked vector ops read v0");
    }

    #[test]
    fn invalid_opcode_is_an_error() {
        assert!(query_rw_info(&Inst::new(Opcode::Invalid as u32)).is_err());
    }

    fn build_direct() -> Vec<u8> {
        let mut buf = CodeBuffer::new();
        let mut asm = Assembler::new(&mut buf);
        let done = asm.get_label();

        asm.add(A0, A1, A2);
        asm.beq(A0, A1, done);
        asm.addi(A0, A0, imm(1));
        asm.bind_label(done);
        asm.jalr(ZERO, RA, imm(0));

        buf.finish().data().to_vec()
    }

    fn build_deferred() -> Vec<u8> {
        let mut buf = CodeBuffer::new();
        let mut builder = Builder::new();
        let done = buf.get_label();

        builder.push_inst(Inst::with_operands(
            Opcode::ADD as u32,
            &[*A0.as_operand(), *A1.as_operand(), *A2.as_operand()],
        ));
        builder.push_inst(Inst::with_operands(
            Opcode::BEQ as u32,
            &[*A0.as_operand(), *A1.as_operand(), *done.as_operand()],
        ));
        builder.push_inst(Inst::with_operands(
            Opcode::ADDI as u32,
            &[*A0.as_operand(), *A0.as_operand(), *imm(1).as_operand()],
        ));
        builder.push_label(done);
        builder.push_inst(Inst::with_operands(
            Opcode::JALR as u32,
            &[*ZERO.as_operand(), *RA.as_operand(), *imm(0).as_operand()],
        ));

        {
            let mut asm = Assembler::new(&mut buf);
            builder.emit_into(&mut asm).unwrap();
        }
        buf.finish().data().to_vec()
    }

    #[test]
    fn builder_replay_matches_direct_assembly() {
        assert_eq!(build_direct(), build_deferred());
    }
}
