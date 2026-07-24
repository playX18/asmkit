//! Top-level emit path: InstInfo lookup, signature matching, operand analysis, and
//! dispatch to the emit handlers (port of AsmJit's `Assembler::_emit` from
//! `x86assembler.cpp`, both 32-bit and 64-bit modes).
//!
//! AsmJit's `goto EmitXxx` targets become the [`Handler`] enum: the encoding arms
//! (a `match` on [`Encoding`]) only fill [`X86EmitState`] and select a handler, the
//! actual byte emission is done by the handlers in [`super::encoder`]. AsmJit's
//! `break` (no form matched) becomes `Err(InvalidInstruction)`.
//!
//! Derived from AsmJit (Zlib license) — this file is an altered version; see LICENSE notices.

// The encoding discriminants mirror `instdb::Encoding` variant names 1:1.
#![allow(non_upper_case_globals)]

use crate::core::buffer::CodeBuffer;
use crate::core::globals::InstOptions;
use crate::core::operand::{Operand, OperandSignature, OperandType, RegType, Sym};
use crate::{AsmError, X86Error};

use super::encoder::{
    X86EmitState, emit_address_override, emit_fpu_op, emit_jmp_call, emit_vex_evex_m,
    emit_vex_evex_r, emit_vex_op, emit_x86_m, emit_x86_op, emit_x86_op_implicit_mem,
    emit_x86_op_mov_abs, emit_x86_op_reg, emit_x86_r, emit_x86_r_from_m, fixup_gpb,
    is_implicit_mem, is_mmx_or_xmm, opcode_l_by_size, opcode_l_by_vmem, pack_reg_and_vvvvv,
    should_use_movabs, sign_extend_int32,
};
use super::encoder_tables::{MEM_INFO_TABLE, OPCODE_POP_SREG_TABLE, OPCODE_PUSH_SREG_TABLE};
use super::instdb::{
    ADDITIONAL_INFO_TABLE, ALT_OPCODE_TABLE, CPU_FEATURE_COUNT, CPU_FEATURE_NAMES, CommonInfo,
    CpuFeature, INST_COMMON_INFO_TABLE, INST_INFO_TABLE, INST_SIGNATURE_TABLE, InstFlags, InstId,
    InstInfo, MAIN_OPCODE_TABLE, Mode, OP_SIGNATURE_TABLE, OpFlags, OpSignature,
};
use super::opcode::Opcode;
use super::operands::{Gp, KReg, Mem, SReg};
use crate::core::operand::OperandCast;

/// Emit handler selected by an encoding arm (AsmJit's `goto EmitXxx` labels).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Handler {
    X86Op,
    X86OpMovAbs,
    X86OpReg,
    X86OpImplicitMem,
    X86R,
    X86RFromM,
    X86M,
    FpuOp,
    VexOp,
    VexEvexR,
    VexEvexM,
    JmpCall,
}

// AsmJit's operand-type codes used by ENC_OPSn (asmkit's `OperandType` numbering
// differs — `RegList` was inserted at 3 — so operands are mapped explicitly).
const OT_NONE: u32 = 0;
const OT_REG: u32 = 1;
const OT_MEM: u32 = 2;
const OT_IMM: u32 = 3;

/// Maps an operand to AsmJit's operand-type code for isign3/isign4 comparisons.
fn ot(op: &Operand) -> u32 {
    match op.op_type() {
        OperandType::None => OT_NONE,
        OperandType::Reg => OT_REG,
        OperandType::Mem => OT_MEM,
        OperandType::Imm => OT_IMM,
        // Label/Sym (and anything else) never match an ENC_OPSn form.
        _ => 7,
    }
}

macro_rules! ops1 {
    ($a:expr) => {
        $a
    };
}
macro_rules! ops2 {
    ($a:expr, $b:expr) => {
        $a + ($b << 3)
    };
}
macro_rules! ops3 {
    ($a:expr, $b:expr, $c:expr) => {
        $a + ($b << 3) + ($c << 6)
    };
}
macro_rules! ops4 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $a + ($b << 3) + ($c << 6) + ($d << 9)
    };
}

fn invalid(reason: &'static str) -> X86Error {
    X86Error::InvalidInstruction { opcode: 0, reason }
}

fn no_match() -> X86Error {
    invalid("instruction does not support the given operands")
}

fn size_mismatch() -> X86Error {
    invalid("operand size mismatch")
}

fn ambiguous_size() -> X86Error {
    invalid("ambiguous operand size")
}

fn invalid_imm(value: i64, size: usize) -> X86Error {
    X86Error::InvalidImmediate {
        value,
        size,
        reason: "immediate value does not fit the required size",
    }
}

fn is_int8(x: i64) -> bool {
    i8::try_from(x).is_ok()
}

fn is_int32(x: i64) -> bool {
    i32::try_from(x).is_ok()
}

fn is_uint32(x: i64) -> bool {
    u32::try_from(x).is_ok()
}

/// AsmJit's `alt_opcode_of`.
fn alt_opcode(inst_info: &InstInfo) -> Opcode {
    Opcode(ALT_OPCODE_TABLE[inst_info.alt_opcode_index as usize])
}

/// AsmJit's `FIXUP_GPB` over an operand (see [`fixup_gpb`]).
fn fixup_gpb_op(options: &mut InstOptions, op: &Operand, reg_id: &mut u32) {
    fixup_gpb(options, &Gp(op.as_::<super::operands::Reg>()), reg_id);
}

fn is_segment_reg(op: &Operand) -> bool {
    op.is_reg_type_of(RegType::X86SReg)
}

fn is_control_reg(op: &Operand) -> bool {
    op.is_reg_type_of(RegType::X86CReg)
}

fn is_debug_reg(op: &Operand) -> bool {
    op.is_reg_type_of(RegType::X86DReg)
}

fn is_mm_reg(op: &Operand) -> bool {
    op.is_reg_type_of(RegType::Extra)
}

fn is_gp_with_id(op: &Operand, id: u32) -> bool {
    op.is_gp() && op.id() == id
}

fn is_gpw_with_id(op: &Operand, id: u32) -> bool {
    op.is_reg_type_of(RegType::Gp16) && op.id() == id
}

fn is_gp32_with_id(op: &Operand, id: u32) -> bool {
    op.is_reg_type_of(RegType::Gp32) && op.id() == id
}

fn is_vec128_with_id(op: &Operand, id: u32) -> bool {
    op.is_vec128() && op.id() == id
}

/// `op_flag_from_reg_type_table` — OpFlags bits for a register type.
fn op_flags_from_reg_type(typ: RegType) -> u64 {
    match typ {
        RegType::Gp8Lo => OpFlags::REG_GPB_LO.bits(),
        RegType::Gp8Hi => OpFlags::REG_GPB_HI.bits(),
        RegType::Gp16 => OpFlags::REG_GPW.bits(),
        RegType::Gp32 => OpFlags::REG_GPD.bits(),
        RegType::Gp64 => OpFlags::REG_GPQ.bits(),
        RegType::Vec128 => OpFlags::REG_XMM.bits(),
        RegType::Vec256 => OpFlags::REG_YMM.bits(),
        RegType::Vec512 => OpFlags::REG_ZMM.bits(),
        RegType::Extra => OpFlags::REG_MM.bits(),
        RegType::Mask => OpFlags::REG_K_REG.bits(),
        RegType::X86SReg => OpFlags::REG_S_REG.bits(),
        RegType::X86CReg => OpFlags::REG_C_REG.bits(),
        RegType::X86DReg => OpFlags::REG_D_REG.bits(),
        RegType::X86St => OpFlags::REG_ST.bits(),
        RegType::X86Bnd => OpFlags::REG_BND.bits(),
        RegType::X86Tmm => OpFlags::REG_TMM.bits(),
        _ => 0,
    }
}

fn allowed_reg_ids(typ: RegType, is_32bit: bool) -> u32 {
    match (typ, is_32bit) {
        (RegType::PC, _) => 0x0000_0001,
        (RegType::Gp8Hi, _) => 0x0000_000F,
        (RegType::Gp8Lo, true) => 0x0000_000F,
        (RegType::Gp8Lo | RegType::Gp16 | RegType::Gp32, false) => 0x0000_FFFF,
        (RegType::Gp16 | RegType::Gp32, true) => 0x0000_00FF,
        (RegType::Gp64, false) => 0x0000_FFFF,
        (RegType::Vec128 | RegType::Vec256 | RegType::Vec512, false) => 0xFFFF_FFFF,
        (RegType::Vec128 | RegType::Vec256 | RegType::Vec512, true) => 0x0000_00FF,
        (RegType::Mask | RegType::Extra | RegType::X86St | RegType::X86Tmm, _) => 0x0000_00FF,
        (RegType::X86SReg, _) => 0x0000_007E,
        (RegType::X86CReg | RegType::X86DReg, _) => 0x0000_FFFF,
        (RegType::X86Bnd, _) => 0x0000_000F,
        _ => 0,
    }
}

fn validate_register(op: &Operand, operand_index: usize, is_32bit: bool) -> Result<(), X86Error> {
    let reg_type = op
        .signature
        .try_reg_type()
        .ok_or(X86Error::InvalidOperand {
            operand_index,
            reason: "invalid register type field",
        })?;
    let expected = super::operands::Reg::signature_of(reg_type);
    let mask = OperandSignature::REG_TYPE_MASK
        | OperandSignature::REG_GROUP_MASK
        | OperandSignature::SIZE_MASK;
    if op_flags_from_reg_type(reg_type) == 0 || op.signature.subset(mask) != expected.subset(mask) {
        return Err(X86Error::InvalidRegister {
            reg_id: op.id(),
            reg_type: "x86",
            reason: "invalid or inconsistent register type",
        });
    }

    let id = op.id();
    let allowed = allowed_reg_ids(reg_type, is_32bit);
    if id >= 32 || allowed & (1u32 << id) == 0 {
        return Err(X86Error::InvalidRegister {
            reg_id: id,
            reg_type: "x86",
            reason: "register id is not encodable in the target mode",
        });
    }
    Ok(())
}

fn validate_memory(op: &Operand, operand_index: usize, is_32bit: bool) -> Result<(), X86Error> {
    let mem = op.as_::<Mem>();
    let base_type = op
        .signature
        .try_mem_base_type()
        .ok_or(X86Error::InvalidOperand {
            operand_index,
            reason: "invalid memory base type field",
        })?;
    let index_type = op
        .signature
        .try_mem_index_type()
        .ok_or(X86Error::InvalidOperand {
            operand_index,
            reason: "invalid memory index type field",
        })?;

    if mem.try_addr_type().is_none() {
        return Err(X86Error::InvalidMemoryOperand {
            base: mem.has_base().then(|| mem.base_id()),
            index: mem.has_index().then(|| mem.index_id()),
            scale: mem.shift() as u8,
            offset: mem.offset(),
            reason: "invalid address type",
        });
    }
    if mem.try_broadcast().is_none() {
        return Err(X86Error::InvalidBroadcast {
            reason: "invalid broadcast field",
        });
    }
    if mem.segment_id() > SReg::GS {
        return Err(X86Error::InvalidMemoryOperand {
            base: mem.has_base().then(|| mem.base_id()),
            index: mem.has_index().then(|| mem.index_id()),
            scale: mem.shift() as u8,
            offset: mem.offset(),
            reason: "invalid segment register id",
        });
    }
    if !matches!(mem.size(), 0 | 1 | 2 | 4 | 6 | 8 | 10 | 16 | 32 | 64) {
        return Err(X86Error::InvalidMemoryOperand {
            base: mem.has_base().then(|| mem.base_id()),
            index: mem.has_index().then(|| mem.index_id()),
            scale: mem.shift() as u8,
            offset: mem.offset(),
            reason: "invalid memory operand size",
        });
    }

    let gp_limit = if is_32bit { 8 } else { 16 };
    match base_type {
        RegType::None | RegType::LabelTag | RegType::SymTag => {}
        RegType::PC if mem.base_id() == 0 => {}
        RegType::Gp16 if is_32bit && mem.base_id() < gp_limit => {}
        RegType::Gp32 if mem.base_id() < gp_limit => {}
        RegType::Gp64 if !is_32bit && mem.base_id() < gp_limit => {}
        _ => {
            return Err(X86Error::InvalidMemoryOperand {
                base: Some(mem.base_id()),
                index: mem.has_index().then(|| mem.index_id()),
                scale: mem.shift() as u8,
                offset: mem.offset(),
                reason: "invalid memory base register",
            });
        }
    }

    match index_type {
        RegType::None => {}
        RegType::Gp16 if is_32bit && mem.index_id() < gp_limit => {}
        RegType::Gp32 if mem.index_id() < gp_limit => {}
        RegType::Gp64 if !is_32bit && mem.index_id() < gp_limit => {}
        RegType::Vec128 | RegType::Vec256 | RegType::Vec512
            if mem.index_id() < if is_32bit { 8 } else { 32 } => {}
        _ => {
            return Err(X86Error::InvalidMemoryOperand {
                base: mem.has_base().then(|| mem.base_id()),
                index: Some(mem.index_id()),
                scale: mem.shift() as u8,
                offset: mem.offset(),
                reason: "invalid memory index register",
            });
        }
    }
    Ok(())
}

fn validate_raw_operand(
    buffer: &CodeBuffer,
    op: &Operand,
    operand_index: usize,
    is_32bit: bool,
) -> Result<(), X86Error> {
    let op_type = op.signature.try_op_type().ok_or(X86Error::InvalidOperand {
        operand_index,
        reason: "invalid operand type field",
    })?;
    match op_type {
        OperandType::None if op.signature.bits() == 0 => Ok(()),
        OperandType::Reg => validate_register(op, operand_index, is_32bit),
        OperandType::Mem => {
            validate_memory(op, operand_index, is_32bit)?;
            let mem = op.as_::<Mem>();
            if mem.has_base_sym() && buffer.symbol_name(Sym::from_id(mem.base_id())).is_none() {
                return Err(X86Error::InvalidOperand {
                    operand_index,
                    reason: "symbol is not declared in this buffer",
                });
            }
            Ok(())
        }
        OperandType::Sym if buffer.symbol_name(Sym::from_id(op.id())).is_some() => Ok(()),
        OperandType::Sym => Err(X86Error::InvalidOperand {
            operand_index,
            reason: "symbol is not declared in this buffer",
        }),
        OperandType::Imm | OperandType::Label => Ok(()),
        _ => Err(X86Error::InvalidOperand {
            operand_index,
            reason: "operand type is not supported by the x86 encoder",
        }),
    }
}

/// Translates one operand into an [`OpSignature`] (AsmJit's per-operand translation
/// in `validate`, physical registers only). In 32-bit mode also rejects operands
/// that are not encodable there (AsmJit's x86 validation data and the
/// `kInvalidUseOfGpq` / address-range checks).
fn translate_op(
    op: &Operand,
    common_info: &CommonInfo,
    is_32bit: bool,
) -> Result<OpSignature, X86Error> {
    let mut op_flags = 0u64;
    let mut reg_mask = 0u8;

    match op.op_type() {
        OperandType::Reg => {
            let reg_type = op.as_::<super::operands::Reg>().reg_type();
            let flags = op_flags_from_reg_type(reg_type);
            if flags == 0 {
                return Err(invalid("invalid register type"));
            }
            op_flags = flags;
            let id = op.id();
            if id < 8 {
                reg_mask = 1u8 << id;
            }
        }
        OperandType::Mem => {
            let m = op.as_::<Mem>();
            let mut mem_size = m.size();
            let base_type = m.base_type();
            let index_type = m.index_type();

            // AVX-512 broadcast {1tox}.
            if m.has_broadcast() {
                let bcst32 = common_info.has_avx512_flag(super::instdb::Avx512Flags::B32);
                let bcst64 = common_info.has_avx512_flag(super::instdb::Avx512Flags::B64);
                if mem_size != 0 {
                    if (bcst32 && mem_size != 4) || (bcst64 && mem_size != 8) {
                        return Err(X86Error::InvalidBroadcast {
                            reason: "memory size does not match the broadcast size",
                        });
                    }
                } else {
                    mem_size = if bcst64 {
                        8
                    } else if bcst32 {
                        4
                    } else {
                        2
                    };
                }
                mem_size <<= m.get_broadcast() as u32;
            }

            // Addressing register types depend on the mode (AsmJit's
            // `allowed_mem_base_regs` / `allowed_mem_index_regs`): 16|32-bit
            // registers in 32-bit mode, 32|64-bit ones in 64-bit mode.
            for addr_type in [base_type, index_type] {
                if matches!(addr_type, RegType::Gp16 | RegType::Gp32 | RegType::Gp64) {
                    let ok = if is_32bit {
                        addr_type != RegType::Gp64
                    } else {
                        addr_type != RegType::Gp16
                    };
                    if !ok {
                        return Err(invalid(
                            "addressing register size is not usable in the target mode",
                        ));
                    }
                }
            }

            // A base-less address is a 32|64-bit absolute address; make sure it is
            // encodable in the target mode (AsmJit's address-range validation).
            if base_type == RegType::None {
                let offset = m.offset();
                if !is_int32(offset) {
                    if is_32bit {
                        if !is_uint32(offset) {
                            return Err(invalid("absolute address does not fit 32 bits"));
                        }
                    } else if index_type != RegType::None
                        && (!is_uint32(offset) || index_type != RegType::Gp32)
                    {
                        return Err(invalid(
                            "absolute address with an index register must be a zero-extended 32-bit address",
                        ));
                    }
                }
            }

            if base_type != RegType::None
                && base_type != RegType::LabelTag
                && index_type == RegType::None
                && m.offset_lo32() == 0
            {
                op_flags |= OpFlags::FLAG_MEM_BASE.bits();
            }

            if index_type != RegType::None {
                if index_type == RegType::Vec128 {
                    op_flags |= OpFlags::VM32X.bits() | OpFlags::VM64X.bits();
                } else if index_type == RegType::Vec256 {
                    op_flags |= OpFlags::VM32Y.bits() | OpFlags::VM64Y.bits();
                } else if index_type == RegType::Vec512 {
                    op_flags |= OpFlags::VM32Z.bits() | OpFlags::VM64Z.bits();
                } else if base_type != RegType::None {
                    op_flags |= OpFlags::FLAG_MIB.bits();
                }
            }

            op_flags |= match mem_size {
                0 => OpFlags::MEM_UNSPECIFIED.bits(),
                1 => OpFlags::MEM8.bits(),
                2 => OpFlags::MEM16.bits(),
                4 => OpFlags::MEM32.bits(),
                6 => OpFlags::MEM48.bits(),
                8 => OpFlags::MEM64.bits(),
                10 => OpFlags::MEM80.bits(),
                16 => OpFlags::MEM128.bits(),
                32 => OpFlags::MEM256.bits(),
                64 => OpFlags::MEM512.bits(),
                _ => return Err(invalid("invalid memory operand size")),
            };
        }
        OperandType::Imm => {
            let imm_value = op.as_::<crate::core::operand::Imm>().value() as u64;
            if (imm_value as i64) >= 0 {
                op_flags = if imm_value <= 0x7 {
                    OpFlags::IMM_I64.bits()
                        | OpFlags::IMM_U64.bits()
                        | OpFlags::IMM_I32.bits()
                        | OpFlags::IMM_U32.bits()
                        | OpFlags::IMM_I16.bits()
                        | OpFlags::IMM_U16.bits()
                        | OpFlags::IMM_I8.bits()
                        | OpFlags::IMM_U8.bits()
                        | OpFlags::IMM_I4.bits()
                        | OpFlags::IMM_U4.bits()
                } else if imm_value <= 0xF {
                    OpFlags::IMM_I64.bits()
                        | OpFlags::IMM_U64.bits()
                        | OpFlags::IMM_I32.bits()
                        | OpFlags::IMM_U32.bits()
                        | OpFlags::IMM_I16.bits()
                        | OpFlags::IMM_U16.bits()
                        | OpFlags::IMM_I8.bits()
                        | OpFlags::IMM_U8.bits()
                        | OpFlags::IMM_U4.bits()
                } else if imm_value <= 0x7F {
                    OpFlags::IMM_I64.bits()
                        | OpFlags::IMM_U64.bits()
                        | OpFlags::IMM_I32.bits()
                        | OpFlags::IMM_U32.bits()
                        | OpFlags::IMM_I16.bits()
                        | OpFlags::IMM_U16.bits()
                        | OpFlags::IMM_I8.bits()
                        | OpFlags::IMM_U8.bits()
                } else if imm_value <= 0xFF {
                    OpFlags::IMM_I64.bits()
                        | OpFlags::IMM_U64.bits()
                        | OpFlags::IMM_I32.bits()
                        | OpFlags::IMM_U32.bits()
                        | OpFlags::IMM_I16.bits()
                        | OpFlags::IMM_U16.bits()
                        | OpFlags::IMM_U8.bits()
                } else if imm_value <= 0x7FFF {
                    OpFlags::IMM_I64.bits()
                        | OpFlags::IMM_U64.bits()
                        | OpFlags::IMM_I32.bits()
                        | OpFlags::IMM_U32.bits()
                        | OpFlags::IMM_I16.bits()
                        | OpFlags::IMM_U16.bits()
                } else if imm_value <= 0xFFFF {
                    OpFlags::IMM_I64.bits()
                        | OpFlags::IMM_U64.bits()
                        | OpFlags::IMM_I32.bits()
                        | OpFlags::IMM_U32.bits()
                        | OpFlags::IMM_I16.bits()
                } else if imm_value <= 0x7FFF_FFFF {
                    OpFlags::IMM_I64.bits()
                        | OpFlags::IMM_U64.bits()
                        | OpFlags::IMM_I32.bits()
                        | OpFlags::IMM_U32.bits()
                } else if imm_value <= 0xFFFF_FFFF {
                    OpFlags::IMM_I64.bits() | OpFlags::IMM_U64.bits() | OpFlags::IMM_U32.bits()
                } else if imm_value <= 0x7FFF_FFFF_FFFF_FFFF {
                    OpFlags::IMM_I64.bits() | OpFlags::IMM_U64.bits()
                } else {
                    OpFlags::IMM_U64.bits()
                };
            } else {
                let neg = (imm_value as i64).wrapping_neg() as u64;
                op_flags = if neg <= 0x8 {
                    OpFlags::IMM_I64.bits()
                        | OpFlags::IMM_I32.bits()
                        | OpFlags::IMM_I16.bits()
                        | OpFlags::IMM_I8.bits()
                        | OpFlags::IMM_I4.bits()
                } else if neg <= 0x80 {
                    OpFlags::IMM_I64.bits()
                        | OpFlags::IMM_I32.bits()
                        | OpFlags::IMM_I16.bits()
                        | OpFlags::IMM_I8.bits()
                } else if neg <= 0x8000 {
                    OpFlags::IMM_I64.bits() | OpFlags::IMM_I32.bits() | OpFlags::IMM_I16.bits()
                } else if neg <= 0x8000_0000 {
                    OpFlags::IMM_I64.bits() | OpFlags::IMM_I32.bits()
                } else {
                    OpFlags::IMM_I64.bits()
                };
            }
        }
        OperandType::Label => {
            op_flags = OpFlags::REL8.bits() | OpFlags::REL32.bits();
        }
        _ => return Err(invalid("invalid operand type")),
    }

    Ok(OpSignature::new(op_flags & 0x00FF_FFFF_FFFF_FFFF, reg_mask))
}

/// AsmJit's `check_op_sig`.
fn check_op_sig(op: &OpSignature, reference: &OpSignature, imm_out_of_range: &mut bool) -> bool {
    // Fail if operand types are incompatible.
    let common_flags = op.flags & reference.flags;
    if common_flags & OpFlags::OP_MASK.bits() == 0 {
        // Mark temporarily `imm_out_of_range` so we can return a more descriptive error later.
        if (op.flags & OpFlags::IMM_MASK.bits() != 0)
            && (reference.flags & OpFlags::IMM_MASK.bits() != 0)
        {
            *imm_out_of_range = true;
            return true;
        }
        return false;
    }

    // Fail if some memory specific flags do not match.
    if common_flags & OpFlags::MEM_MASK.bits() != 0
        && (reference.flags & OpFlags::FLAG_MEM_BASE.bits() != 0)
        && (op.flags & OpFlags::FLAG_MEM_BASE.bits() == 0)
    {
        return false;
    }

    let op_vm = op.flags & OpFlags::VM_MASK.bits();
    let reference_vm = reference.flags & OpFlags::VM_MASK.bits();
    if (op_vm == 0) != (reference_vm == 0) || (op_vm != 0 && op_vm & reference_vm == 0) {
        return false;
    }
    if reference.flags & OpFlags::FLAG_MIB.bits() != 0 && op.flags & OpFlags::FLAG_MIB.bits() == 0 {
        return false;
    }

    // Fail if register indexes do not match.
    if common_flags & OpFlags::REG_MASK.bits() != 0
        && reference.reg_mask != 0
        && (op.reg_mask & reference.reg_mask) == 0
    {
        return false;
    }

    true
}

/// Validates operands against the instruction's signature records (port of the
/// signature-matching part of AsmJit's x86 `validate`). Returns
/// `InvalidInstruction` when no record matches, `InvalidImmediate` when only an
/// immediate was out of range. Signatures are filtered by the target mode
/// (AsmJit's `InstSignature::mode`), which rejects X64-only forms in 32-bit
/// mode and vice versa.
fn validate_signature(
    common_info: &CommonInfo,
    ops: &[Operand; 6],
    is_32bit: bool,
) -> Result<(), X86Error> {
    let signature_count = common_info.signature_count as usize;
    if signature_count == 0 {
        return Ok(());
    }

    // Count leading non-none operands; there must be no gaps.
    let mut op_count = 0usize;
    while op_count < ops.len() && !ops[op_count].is_none() {
        op_count += 1;
    }
    if ops[op_count..].iter().any(|op| !op.is_none()) {
        return Err(no_match());
    }

    let mut translated = [OpSignature::default(); 6];
    for (i, op) in ops[..op_count].iter().enumerate() {
        translated[i] = translate_op(op, common_info, is_32bit)?;
    }

    let signatures = &INST_SIGNATURE_TABLE[common_info.signature_index as usize
        ..common_info.signature_index as usize + signature_count];

    let mode = if is_32bit {
        Mode::X86 as u8
    } else {
        Mode::X64 as u8
    };

    let mut global_imm_out_of_range = false;
    for inst_signature in signatures {
        // Only match signatures that are compatible with the requested mode.
        if inst_signature.mode & mode == 0 {
            continue;
        }

        let inst_op_count = inst_signature.op_count as usize;
        let mut local_imm_out_of_range = false;
        let mut j = 0usize;

        if inst_op_count == op_count {
            while j < op_count {
                let reference =
                    &OP_SIGNATURE_TABLE[inst_signature.op_signature_indexes[j] as usize];
                if !check_op_sig(&translated[j], reference, &mut local_imm_out_of_range) {
                    break;
                }
                j += 1;
            }
        } else if inst_op_count - inst_signature.implicit_op_count as usize == op_count {
            let mut r = 0usize;
            while j < op_count && r < inst_op_count {
                // Skip implicit operands.
                loop {
                    let reference =
                        &OP_SIGNATURE_TABLE[inst_signature.op_signature_indexes[r] as usize];
                    if reference.flags & OpFlags::FLAG_IMPLICIT.bits() == 0 {
                        if !check_op_sig(&translated[j], reference, &mut local_imm_out_of_range) {
                            r = inst_op_count;
                        }
                        break;
                    }
                    r += 1;
                    if r >= inst_op_count {
                        break;
                    }
                }
                if r >= inst_op_count {
                    break;
                }
                j += 1;
                r += 1;
            }
        }

        if j == op_count {
            if !local_imm_out_of_range {
                return Ok(());
            }
            global_imm_out_of_range = true;
        }
    }

    if global_imm_out_of_range {
        Err(invalid_imm(0, 0))
    } else {
        Err(no_match())
    }
}

/// Shared tail of the `CaseX86M_NoSize` label.
fn case_x86m_no_size(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
) -> Result<Handler, X86Error> {
    let o0 = ops[0];
    st.rb_reg = o0.id();
    if isign3 == ops1!(OT_REG) {
        return Ok(Handler::X86R);
    }
    st.rm_rel = o0;
    if isign3 == ops1!(OT_MEM) {
        return Ok(Handler::X86M);
    }
    Err(no_match())
}

/// Shared tail of the `CaseX86M_GPB_MulDiv` label.
fn case_x86m_gpb_muldiv(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
) -> Result<Handler, X86Error> {
    let (o0, o1, o2) = (ops[0], ops[1], ops[2]);
    if isign3 > 0x7 {
        // [AX] <- [AX] div|mul r8.
        if isign3 == ops2!(OT_REG, OT_REG) {
            if !is_gpw_with_id(&o0, Gp::AX) || !super::operands::Reg::operand_is_gpb(&o1) {
                return Err(no_match());
            }
            st.rb_reg = o1.id();
            fixup_gpb_op(&mut st.options, &o1, &mut st.rb_reg);
            return Ok(Handler::X86R);
        }

        // [AX] <- [AX] div|mul m8.
        if isign3 == ops2!(OT_REG, OT_MEM) {
            if !is_gpw_with_id(&o0, Gp::AX) {
                return Err(no_match());
            }
            st.rm_rel = o1;
            return Ok(Handler::X86M);
        }

        // [?DX:?AX] <- [?DX:?AX] div|mul r16|r32|r64
        if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
            if o0.x86_rm_size() != o1.x86_rm_size() {
                return Err(no_match());
            }
            st.opcode.add_arith_by_size(o0.x86_rm_size());
            st.rb_reg = o2.id();
            return Ok(Handler::X86R);
        }

        // [?DX:?AX] <- [?DX:?AX] div|mul m16|m32|m64
        if isign3 == ops3!(OT_REG, OT_REG, OT_MEM) {
            if o0.x86_rm_size() != o1.x86_rm_size() {
                return Err(no_match());
            }
            st.opcode.add_arith_by_size(o0.x86_rm_size());
            st.rm_rel = o2;
            return Ok(Handler::X86M);
        }

        return Err(no_match());
    }

    // kEncodingX86M_GPB body.
    if isign3 == ops1!(OT_REG) {
        st.opcode.add_arith_by_size(o0.x86_rm_size());
        st.rb_reg = o0.id();
        if o0.x86_rm_size() != 1 {
            return Ok(Handler::X86R);
        }
        fixup_gpb_op(&mut st.options, &o0, &mut st.rb_reg);
        return Ok(Handler::X86R);
    }

    if isign3 == ops1!(OT_MEM) {
        if o0.x86_rm_size() == 0 {
            return Err(ambiguous_size());
        }
        st.opcode.add_arith_by_size(o0.x86_rm_size());
        st.rm_rel = o0;
        return Ok(Handler::X86M);
    }
    Err(no_match())
}

/// Shared tail of the `CaseExtRm` label.
fn case_ext_rm(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
) -> Result<Handler, X86Error> {
    let (o0, o1) = (ops[0], ops[1]);
    if isign3 == ops2!(OT_REG, OT_REG) {
        st.op_reg = o0.id();
        st.rb_reg = o1.id();
        return Ok(Handler::X86R);
    }
    if isign3 == ops2!(OT_REG, OT_MEM) {
        st.op_reg = o0.id();
        st.rm_rel = o1;
        return Ok(Handler::X86M);
    }
    Err(no_match())
}

/// Shared tail of the `CaseExtMovd` label.
fn case_ext_movd(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
    inst_info: &InstInfo,
) -> Result<Handler, X86Error> {
    let (o0, o1) = (ops[0], ops[1]);
    if is_mmx_or_xmm(o0.as_::<super::operands::Reg>().reg_type()) {
        st.op_reg = o0.id();
        st.opcode.add_66h_if(o0.is_vec128());

        // MM/XMM <- Gp
        if isign3 == ops2!(OT_REG, OT_REG) && o1.is_gp() {
            st.rb_reg = o1.id();
            return Ok(Handler::X86R);
        }

        // MM/XMM <- Mem
        if isign3 == ops2!(OT_REG, OT_MEM) {
            st.rm_rel = o1;
            return Ok(Handler::X86M);
        }
    }

    // The following instructions use the secondary opcode.
    if is_mmx_or_xmm(o1.as_::<super::operands::Reg>().reg_type()) {
        st.opcode = Opcode(st.opcode.get() & Opcode::W);
        st.opcode = Opcode(st.opcode.get() | alt_opcode(inst_info).get());
        st.op_reg = o1.id();
        st.opcode.add_66h_if(o1.is_vec128());

        // GP <- MM/XMM
        if isign3 == ops2!(OT_REG, OT_REG) && o0.is_gp() {
            st.rb_reg = o0.id();
            return Ok(Handler::X86R);
        }

        // Mem <- MM/XMM
        if isign3 == ops2!(OT_MEM, OT_REG) {
            st.rm_rel = o0;
            return Ok(Handler::X86M);
        }
    }
    Err(no_match())
}

/// Shared tail of the `CaseVexRm` label.
fn case_vex_rm(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
) -> Result<Handler, X86Error> {
    let (o0, o1) = (ops[0], ops[1]);
    if isign3 == ops2!(OT_REG, OT_REG) {
        st.op_reg = o0.id();
        st.rb_reg = o1.id();
        return Ok(Handler::VexEvexR);
    }
    if isign3 == ops2!(OT_REG, OT_MEM) {
        st.op_reg = o0.id();
        st.rm_rel = o1;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Shared tail of the `CaseVexRmi` label.
fn case_vex_rmi(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
) -> Result<Handler, X86Error> {
    let (o0, o1, o2) = (ops[0], ops[1], ops[2]);
    st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
    st.imm_size = 1;

    if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
        st.op_reg = o0.id();
        st.rb_reg = o1.id();
        return Ok(Handler::VexEvexR);
    }
    if isign3 == ops3!(OT_REG, OT_MEM, OT_IMM) {
        st.op_reg = o0.id();
        st.rm_rel = o1;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Shared tail of the `CaseVexMri` label.
fn case_vex_mri(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
) -> Result<Handler, X86Error> {
    let (o0, o1, o2) = (ops[0], ops[1], ops[2]);
    st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
    st.imm_size = 1;

    if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
        st.op_reg = o1.id();
        st.rb_reg = o0.id();
        return Ok(Handler::VexEvexR);
    }
    if isign3 == ops3!(OT_MEM, OT_REG, OT_IMM) {
        st.op_reg = o1.id();
        st.rm_rel = o0;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Shared tail of the `CaseVexRvm` label.
fn case_vex_rvm(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
) -> Result<Handler, X86Error> {
    let (o0, o1, o2) = (ops[0], ops[1], ops[2]);
    if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
        // CaseVexRvm_R.
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rb_reg = o2.id();
        return Ok(Handler::VexEvexR);
    }
    if isign3 == ops3!(OT_REG, OT_REG, OT_MEM) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rm_rel = o2;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Shared tail of the `CaseVexVmi_AfterImm` label.
fn case_vex_vmi_after_imm(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
) -> Result<Handler, X86Error> {
    let (o0, o1) = (ops[0], ops[1]);
    if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
        st.op_reg = pack_reg_and_vvvvv(st.op_reg, o0.id());
        st.rb_reg = o1.id();
        return Ok(Handler::VexEvexR);
    }
    if isign3 == ops3!(OT_REG, OT_MEM, OT_IMM) {
        st.op_reg = pack_reg_and_vvvvv(st.op_reg, o0.id());
        st.rm_rel = o1;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Shared tail of the `CaseFpuArith_Reg` label.
fn case_fpu_arith_reg(st: &mut X86EmitState) -> Handler {
    st.opcode = Opcode(
        (0xD8 << Opcode::FPU_2B_SHIFT)
            + ((st.opcode.get() >> Opcode::FPU_2B_SHIFT) & 0xFF)
            + st.rb_reg,
    );
    Handler::FpuOp
}

/// Shared tail of the `CaseFpuArith_Mem` label.
fn case_fpu_arith_mem(st: &mut X86EmitState, ops: &[Operand; 6]) -> Handler {
    // 0xD8/0xDC, depends on the size of the memory operand; op_reg is valid.
    st.opcode = Opcode(if ops[0].x86_rm_size() == 4 {
        0xD8
    } else {
        0xDC
    });
    // Clear compressed displacement before going to EmitX86M.
    st.opcode = Opcode(st.opcode.get() & !Opcode::CDSHL_MASK);
    st.rm_rel = ops[0];
    Handler::X86M
}

/// Shared tail of the `CaseX86PushPop_Gp` label.
fn case_push_pop_gp(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    inst_info: &InstInfo,
) -> Result<Handler, X86Error> {
    let o0 = ops[0];
    // We allow 2 byte, 4 byte, and 8 byte register sizes, although PUSH and POP only
    // allow 2 bytes or native size. On 64-bit we simply PUSH/POP 64-bit register even
    // if 32-bit register was given.
    if o0.x86_rm_size() < 2 {
        return Err(no_match());
    }
    st.opcode = alt_opcode(inst_info);
    st.opcode.add_66h_by_size(o0.x86_rm_size());
    st.op_reg = o0.id();
    Ok(Handler::X86OpReg)
}

// Encoding discriminants as matchable constants (`Encoding::None` is handled
// before the match, `Encoding::Count` is never stored).
macro_rules! enc_consts {
    ($($name:ident),* $(,)?) => {
        $(const $name: u8 = super::instdb::Encoding::$name as u8;)*
    };
}
enc_consts!(
    X86Op,
    X86Op_Mod11RM,
    X86Op_Mod11RM_I8,
    X86Op_xAddr,
    X86Op_xAX,
    X86Op_xDX_xAX,
    X86Op_MemZAX,
    X86I_xAX,
    X86M,
    X86M_NoMemSize,
    X86M_NoSize,
    X86M_GPB,
    X86M_GPB_MulDiv,
    X86M_Only,
    X86M_Only_EDX_EAX,
    X86M_Nop,
    X86R_Native,
    X86R_FromM,
    X86R32_EDX_EAX,
    X86Rm,
    X86Rm_Raw66H,
    X86Rm_NoSize,
    X86Mr,
    X86Mr_NoSize,
    X86Arith,
    X86Bswap,
    X86Bt,
    X86Call,
    X86Cmpxchg,
    X86Cmpxchg8b_16b,
    X86Crc,
    X86Enter,
    X86Imul,
    X86In,
    X86Ins,
    X86IncDec,
    X86Int,
    X86Jcc,
    X86JecxzLoop,
    X86Jmp,
    X86JmpRel,
    X86LcallLjmp,
    X86Lea,
    X86Mov,
    X86Movabs,
    X86MovsxMovzx,
    X86MovntiMovdiri,
    X86EnqcmdMovdir64b,
    X86Out,
    X86Outs,
    X86Push,
    X86Pushw,
    X86Pop,
    X86Ret,
    X86Rot,
    X86Set,
    X86ShldShrd,
    X86StrRm,
    X86StrMr,
    X86StrMm,
    X86Test,
    X86Xadd,
    X86Xchg,
    X86Fence,
    X86Bndmov,
    FpuOp,
    FpuArith,
    FpuCom,
    FpuFldFst,
    FpuM,
    FpuR,
    FpuRDef,
    FpuStsw,
    ExtRm,
    ExtRm_XMM0,
    ExtRm_ZDI,
    ExtRm_P,
    ExtRm_Wx,
    ExtRm_Wx_GpqOnly,
    ExtRmRi,
    ExtRmRi_P,
    ExtRmi,
    ExtRmi_P,
    ExtPextrw,
    ExtExtract,
    ExtMov,
    ExtMovbe,
    ExtMovd,
    ExtMovq,
    ExtExtrq,
    ExtInsertq,
    Ext3dNow,
    VexOp,
    VexOpMod,
    VexKmov,
    VexR_Wx,
    VexM,
    VexMr_Lx,
    VexMr_VM,
    VexMri,
    VexMri_Lx,
    VexMri_Vpextrw,
    VexMvr_Wx,
    VexRm,
    VexRm_ZDI,
    VexRm_Wx,
    VexRm_Lx,
    VexRm_Lx_Narrow,
    VexRm_Lx_Bcst,
    VexRm_VM,
    VexRmi,
    VexRmi_Wx,
    VexRmi_Lx,
    VexRvm,
    VexRvm_Wx,
    VexRvm_ZDX_Wx,
    VexRvm_Lx,
    VexRvm_Lx_KEvex,
    VexRvm_Lx_2xK,
    VexRvmr,
    VexRvmr_Lx,
    VexRvmi,
    VexRvmi_KEvex,
    VexRvmi_Lx,
    VexRvmi_Lx_KEvex,
    VexRmv,
    VexRmv_Wx,
    VexRmv_VM,
    VexRmvRm_VM,
    VexRmvi,
    VexRmMr,
    VexRmMr_Lx,
    VexRvmRmv,
    VexRvmRmi,
    VexRvmRmi_Lx,
    VexRvmRmvRmi,
    VexRvmMr,
    VexRvmMvr,
    VexRvmMvr_Lx,
    VexRvmVmi,
    VexRvmVmi_Lx,
    VexRvmVmi_Lx_MEvex,
    VexVm,
    VexVm_Wx,
    VexVmi,
    VexVmi_Lx,
    VexVmi4_Wx,
    VexVmi_Lx_MEvex,
    VexRvrmRvmr,
    VexRvrmRvmr_Lx,
    VexRvrmiRvmri_Lx,
    VexMovdMovq,
    VexMovssMovsd,
    Fma4,
    Fma4_Lx,
    AmxCfg,
    AmxR,
    AmxRm,
    AmxMr,
    AmxRmv,
);

/// Operand analysis: fills `st` and selects the emit handler. Port of the encoding
/// switch in AsmJit's `Assembler::_emit`; `break` maps to `Err(no_match())`,
/// fallthrough arms call the shared `case_*` tails above.
#[allow(clippy::too_many_arguments)]
fn analyze(
    buf: &mut CodeBuffer,
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    mut isign3: u32,
    inst_info: &InstInfo,
    common_info: &CommonInfo,
    inst_id: u32,
) -> Result<Handler, X86Error> {
    let (o0, o1, o2, o3, o4) = (ops[0], ops[1], ops[2], ops[3], ops[4]);
    let long_form = st.options.contains(InstOptions::LONG_FORM);

    match inst_info.encoding {
        X86Op => Ok(Handler::X86Op),

        X86Op_Mod11RM => {
            st.rb_reg = st.opcode.extract_mod_rm();
            Ok(Handler::X86R)
        }

        X86Op_Mod11RM_I8 => {
            // The first operand must be immediate, we don't care of other operands as
            // they could be implicit.
            if !o0.is_imm() {
                return Err(no_match());
            }
            st.rb_reg = st.opcode.extract_mod_rm();
            st.imm_value = o0.as_::<crate::core::operand::Imm>().value() as u8 as i64;
            st.imm_size = 1;
            Ok(Handler::X86R)
        }

        X86Op_xAddr => {
            if !o0.is_reg() {
                return Err(no_match());
            }
            let rm_info =
                MEM_INFO_TABLE[o0.as_::<super::operands::Reg>().reg_type() as usize] as u32;
            emit_address_override(buf, rm_info & (st.address_override_mask() as u32) != 0);
            Ok(Handler::X86Op)
        }

        X86Op_xAX => {
            if isign3 == 0 {
                return Ok(Handler::X86Op);
            }
            if isign3 == ops1!(OT_REG) && o0.id() == Gp::AX {
                return Ok(Handler::X86Op);
            }
            Err(no_match())
        }

        X86Op_xDX_xAX => {
            if isign3 == 0 {
                return Ok(Handler::X86Op);
            }
            if isign3 == ops2!(OT_REG, OT_REG) && o0.id() == Gp::DX && o1.id() == Gp::AX {
                return Ok(Handler::X86Op);
            }
            Err(no_match())
        }

        X86Op_MemZAX => {
            if isign3 == 0 {
                return Ok(Handler::X86Op);
            }
            st.rm_rel = o0;
            if isign3 == ops1!(OT_MEM) && is_implicit_mem(&o0, Gp::AX) {
                return Ok(Handler::X86OpImplicitMem);
            }
            Err(no_match())
        }

        X86I_xAX => {
            // Implicit form.
            if isign3 == ops1!(OT_IMM) {
                st.imm_value = o0.as_::<crate::core::operand::Imm>().value() as u8 as i64;
                st.imm_size = 1;
                return Ok(Handler::X86Op);
            }
            // Explicit form.
            if isign3 == ops2!(OT_REG, OT_IMM) && o0.id() == Gp::AX {
                st.imm_value = o1.as_::<crate::core::operand::Imm>().value() as u8 as i64;
                st.imm_size = 1;
                return Ok(Handler::X86Op);
            }
            Err(no_match())
        }

        X86M_NoMemSize => {
            if o0.is_reg() {
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
            }
            case_x86m_no_size(st, ops, isign3)
        }

        X86M => {
            st.opcode.add_prefix_by_size(o0.x86_rm_size());
            case_x86m_no_size(st, ops, isign3)
        }

        X86M_NoSize => case_x86m_no_size(st, ops, isign3),

        X86M_GPB_MulDiv | X86M_GPB => case_x86m_gpb_muldiv(st, ops, isign3),

        X86M_Only_EDX_EAX => {
            if isign3 == ops3!(OT_MEM, OT_REG, OT_REG)
                && is_gp32_with_id(&o1, Gp::DX)
                && is_gp32_with_id(&o2, Gp::AX)
            {
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            // Fall through to X86M_Only.
            if isign3 == ops1!(OT_MEM) {
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86M_Only => {
            if isign3 == ops1!(OT_MEM) {
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86M_Nop => {
            if isign3 == ops1!(OT_NONE) {
                return Ok(Handler::X86Op);
            }

            // Single operand NOP instruction "0F 1F /0".
            st.opcode = Opcode(Opcode::K000F00 | 0x1F);
            st.op_reg = 0;

            if isign3 == ops1!(OT_REG) {
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }

            if isign3 == ops1!(OT_MEM) {
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }

            // Two operand NOP instruction "0F 1F /r".
            st.op_reg = o1.id();
            st.opcode.add_prefix_by_size(o1.x86_rm_size());

            if isign3 == ops2!(OT_REG, OT_REG) {
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }

            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86R_FromM => {
            if isign3 == ops1!(OT_MEM) {
                st.rm_rel = o0;
                st.rb_reg = o0.id();
                return Ok(Handler::X86RFromM);
            }
            Err(no_match())
        }

        X86R32_EDX_EAX => {
            // Explicit form: R32, EDX, EAX.
            if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
                if !is_gp32_with_id(&o1, Gp::DX) || !is_gp32_with_id(&o2, Gp::AX) {
                    return Err(no_match());
                }
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }
            // Implicit form: R32.
            if isign3 == ops1!(OT_REG) {
                if !o0.is_gp32() {
                    return Err(no_match());
                }
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }
            Err(no_match())
        }

        X86R_Native => {
            if isign3 == ops1!(OT_REG) {
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }
            Err(no_match())
        }

        X86Rm => {
            st.opcode.add_prefix_by_size(o0.x86_rm_size());
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Rm_NoSize => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Rm_Raw66H => {
            // We normally emit either [66|F2|F3], this instruction requires 66+[F2|F3].
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                if o0.x86_rm_size() == 2 {
                    buf.put1(0x66);
                } else {
                    st.opcode.add_w_by_size(o0.x86_rm_size());
                }
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                if o0.x86_rm_size() == 2 {
                    buf.put1(0x66);
                } else {
                    st.opcode.add_w_by_size(o0.x86_rm_size());
                }
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Mr => {
            st.opcode.add_prefix_by_size(o1.x86_rm_size());
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.rb_reg = o0.id();
                st.op_reg = o1.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.rm_rel = o0;
                st.op_reg = o1.id();
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Mr_NoSize => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.rb_reg = o0.id();
                st.op_reg = o1.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.rm_rel = o0;
                st.op_reg = o1.id();
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Arith => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.opcode.add_arith_by_size(o0.x86_rm_size());
                if o0.x86_rm_size() != o1.x86_rm_size() {
                    return Err(size_mismatch());
                }
                st.rb_reg = o0.id();
                st.op_reg = o1.id();
                if o0.x86_rm_size() == 1 {
                    fixup_gpb_op(&mut st.options, &o0, &mut st.rb_reg);
                    fixup_gpb_op(&mut st.options, &o1, &mut st.op_reg);
                }
                // MOD/MR: The default encoding used if not instructed otherwise.
                if !st.options.contains(InstOptions::X86_MOD_RM) {
                    return Ok(Handler::X86R);
                }
                // MOD/RM: Alternative encoding selected via instruction options.
                st.opcode.add(2);
                core::mem::swap(&mut st.op_reg, &mut st.rb_reg);
                return Ok(Handler::X86R);
            }

            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.opcode.add(2);
                st.opcode.add_arith_by_size(o0.x86_rm_size());
                st.op_reg = o0.id();
                st.rm_rel = o1;
                if o0.x86_rm_size() != 1 {
                    return Ok(Handler::X86M);
                }
                fixup_gpb_op(&mut st.options, &o0, &mut st.op_reg);
                return Ok(Handler::X86M);
            }

            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.opcode.add_arith_by_size(o1.x86_rm_size());
                st.op_reg = o1.id();
                st.rm_rel = o0;
                if o1.x86_rm_size() != 1 {
                    return Ok(Handler::X86M);
                }
                fixup_gpb_op(&mut st.options, &o1, &mut st.op_reg);
                return Ok(Handler::X86M);
            }

            // The remaining instructions use 0x80 opcode.
            st.opcode = Opcode(0x80);

            if isign3 == ops2!(OT_REG, OT_IMM) {
                let mut size = o0.x86_rm_size();
                st.rb_reg = o0.id();
                st.imm_value = o1.as_::<crate::core::operand::Imm>().value();

                if size == 1 {
                    fixup_gpb_op(&mut st.options, &o0, &mut st.rb_reg);
                    st.imm_size = 1;
                } else {
                    if size == 2 {
                        st.opcode = Opcode(st.opcode.get() | Opcode::PP_66);
                    } else if size == 4 {
                        // Sign extend so is_int8 returns the right result.
                        st.imm_value = sign_extend_int32(st.imm_value as u64) as i64;
                    } else if size == 8 {
                        let can_transform_to_32bit =
                            inst_id == InstId::And as u32 && is_uint32(st.imm_value);
                        if !is_int32(st.imm_value) {
                            // We force this as otherwise we would have to fail.
                            if can_transform_to_32bit {
                                size = 4;
                            } else {
                                return Err(invalid_imm(st.imm_value, 4));
                            }
                        }
                        st.opcode.add_w_by_size(size);
                    }

                    st.imm_size = size.min(4) as u8;
                    if is_int8(st.imm_value) && !long_form {
                        st.imm_size = 1;
                    }
                }

                // Short form - AL, AX, EAX, RAX.
                if st.rb_reg == 0 && (size == 1 || st.imm_size != 1) && !long_form {
                    st.opcode = Opcode(
                        (st.opcode.get() & (Opcode::PP_66 | Opcode::W))
                            | ((st.op_reg << 3) | (0x04 + (size != 1) as u32)),
                    );
                    st.imm_size = size.min(4) as u8;
                    return Ok(Handler::X86Op);
                }

                st.opcode.add(if size != 1 {
                    (st.imm_size != 1) as u32 + (st.imm_size == 1) as u32 * 3
                } else {
                    0
                });
                return Ok(Handler::X86R);
            }

            if isign3 == ops2!(OT_MEM, OT_IMM) {
                let mem_size = o0.x86_rm_size();
                if mem_size == 0 {
                    return Err(ambiguous_size());
                }
                st.imm_value = o1.as_::<crate::core::operand::Imm>().value();
                st.imm_size = mem_size.min(4) as u8;

                // Sign extend so is_int8 returns the right result.
                if mem_size == 4 {
                    st.imm_value = sign_extend_int32(st.imm_value as u64) as i64;
                }
                if is_int8(st.imm_value) && !long_form {
                    st.imm_size = 1;
                }

                st.opcode.add(if mem_size != 1 {
                    (st.imm_size != 1) as u32 + (st.imm_size == 1) as u32 * 3
                } else {
                    0
                });
                st.opcode.add_prefix_by_size(mem_size);
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Bswap => {
            if isign3 == ops1!(OT_REG) {
                if o0.x86_rm_size() == 1 {
                    return Err(no_match());
                }
                st.op_reg = o0.id();
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                return Ok(Handler::X86OpReg);
            }
            Err(no_match())
        }

        X86Bt => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.opcode.add_prefix_by_size(o1.x86_rm_size());
                st.op_reg = o1.id();
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.opcode.add_prefix_by_size(o1.x86_rm_size());
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }

            // The remaining instructions use the secondary opcode/r.
            st.imm_value = o1.as_::<crate::core::operand::Imm>().value();
            st.imm_size = 1;
            st.opcode = alt_opcode(inst_info);
            st.opcode.add_prefix_by_size(o0.x86_rm_size());
            st.op_reg = st.opcode.extract_mod_o();

            if isign3 == ops2!(OT_REG, OT_IMM) {
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_MEM, OT_IMM) {
                if o0.x86_rm_size() == 0 {
                    return Err(ambiguous_size());
                }
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Call => {
            if isign3 == ops1!(OT_REG) {
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }
            st.rm_rel = o0;
            if isign3 == ops1!(OT_MEM) {
                return Ok(Handler::X86M);
            }
            // Call with 32-bit displacement use 0xE8 opcode. Call with 8-bit
            // displacement is not encodable so the alternative opcode field in X86DB
            // must be zero.
            st.opcode = Opcode(0xE8);
            st.op_reg = 0;
            Ok(Handler::JmpCall)
        }

        X86Cmpxchg => {
            // Convert explicit to implicit.
            if isign3 & (0x7 << 6) != 0 {
                if !o2.is_gp() || o2.id() != Gp::AX {
                    return Err(no_match());
                }
                isign3 &= 0x3F;
            }

            if isign3 == ops2!(OT_REG, OT_REG) {
                if o0.x86_rm_size() != o1.x86_rm_size() {
                    return Err(size_mismatch());
                }
                st.opcode.add_arith_by_size(o0.x86_rm_size());
                st.rb_reg = o0.id();
                st.op_reg = o1.id();
                if o0.x86_rm_size() != 1 {
                    return Ok(Handler::X86R);
                }
                fixup_gpb_op(&mut st.options, &o0, &mut st.rb_reg);
                fixup_gpb_op(&mut st.options, &o1, &mut st.op_reg);
                return Ok(Handler::X86R);
            }

            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.opcode.add_arith_by_size(o1.x86_rm_size());
                st.op_reg = o1.id();
                st.rm_rel = o0;
                if o1.x86_rm_size() != 1 {
                    return Ok(Handler::X86M);
                }
                fixup_gpb_op(&mut st.options, &o1, &mut st.op_reg);
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Cmpxchg8b_16b => {
            if isign3 == ops3!(OT_MEM, OT_REG, OT_REG) && o3.is_reg() && o4.is_reg() {
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            if isign3 == ops1!(OT_MEM) {
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Crc => {
            st.op_reg = o0.id();
            st.opcode.add_w_by_size(o0.x86_rm_size());
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.rb_reg = o1.id();
                if o1.x86_rm_size() == 1 {
                    fixup_gpb_op(&mut st.options, &o1, &mut st.rb_reg);
                    return Ok(Handler::X86R);
                } else {
                    // This seems to be the only exception of encoding '66F2' prefix.
                    if o1.x86_rm_size() == 2 {
                        buf.put1(0x66);
                    }
                    st.opcode.add(1);
                    return Ok(Handler::X86R);
                }
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.rm_rel = o1;
                if o1.x86_rm_size() == 0 {
                    return Err(ambiguous_size());
                }
                // This seems to be the only exception of encoding '66F2' prefix.
                if o1.x86_rm_size() == 2 {
                    buf.put1(0x66);
                }
                st.opcode.add((o1.x86_rm_size() != 1) as u32);
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Enter => {
            if isign3 == ops2!(OT_IMM, OT_IMM) {
                let iw = o0.as_::<crate::core::operand::Imm>().value() as u16 as u32;
                let ib = o1.as_::<crate::core::operand::Imm>().value() as u8 as u32;
                st.imm_value = (iw | (ib << 16)) as i64;
                st.imm_size = 3;
                return Ok(Handler::X86Op);
            }
            Err(no_match())
        }

        X86Imul => {
            // First process all forms distinct of `kEncodingX86M_GPB_MulDiv`.
            if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
                st.opcode = Opcode(0x6B);
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                if !is_int8(st.imm_value) || long_form {
                    st.opcode.add(-2i32 as u32);
                    st.imm_size = if o0.x86_rm_size() == 2 { 2 } else { 4 };
                }
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                return Ok(Handler::X86R);
            }

            if isign3 == ops3!(OT_REG, OT_MEM, OT_IMM) {
                st.opcode = Opcode(0x6B);
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                // Sign extend so is_int8 returns the right result.
                if o0.x86_rm_size() == 4 {
                    st.imm_value = sign_extend_int32(st.imm_value as u64) as i64;
                }
                if !is_int8(st.imm_value) || long_form {
                    st.opcode.add(-2i32 as u32);
                    st.imm_size = if o0.x86_rm_size() == 2 { 2 } else { 4 };
                }
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }

            if isign3 == ops2!(OT_REG, OT_REG) {
                // Must be explicit 'ax, r8' form.
                if o1.x86_rm_size() == 1 {
                    return case_x86m_gpb_muldiv(st, ops, isign3);
                }
                if o0.x86_rm_size() != o1.x86_rm_size() {
                    return Err(size_mismatch());
                }
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                st.opcode = Opcode(Opcode::K000F00 | 0xAF);
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                return Ok(Handler::X86R);
            }

            if isign3 == ops2!(OT_REG, OT_MEM) {
                // Must be explicit 'ax, m8' form.
                if o1.x86_rm_size() == 1 {
                    return case_x86m_gpb_muldiv(st, ops, isign3);
                }
                st.op_reg = o0.id();
                st.rm_rel = o1;
                st.opcode = Opcode(Opcode::K000F00 | 0xAF);
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                return Ok(Handler::X86M);
            }

            // Shorthand to imul 'reg, reg, imm'.
            if isign3 == ops2!(OT_REG, OT_IMM) {
                st.opcode = Opcode(0x6B);
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                st.imm_value = o1.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                // Sign extend so is_int8 returns the right result.
                if o0.x86_rm_size() == 4 {
                    st.imm_value = sign_extend_int32(st.imm_value as u64) as i64;
                }
                if !is_int8(st.imm_value) || long_form {
                    st.opcode.add(-2i32 as u32);
                    st.imm_size = if o0.x86_rm_size() == 2 { 2 } else { 4 };
                }
                st.op_reg = o0.id();
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }

            // Try implicit form.
            case_x86m_gpb_muldiv(st, ops, isign3)
        }

        X86In => {
            if isign3 == ops2!(OT_REG, OT_IMM) {
                if o0.id() != Gp::AX {
                    return Err(no_match());
                }
                st.imm_value = o1.as_::<crate::core::operand::Imm>().value() as u8 as i64;
                st.imm_size = 1;
                st.opcode = alt_opcode(inst_info);
                st.opcode.add((o0.x86_rm_size() != 1) as u32);
                st.opcode.add_66h_by_size(o0.x86_rm_size());
                return Ok(Handler::X86Op);
            }
            if isign3 == ops2!(OT_REG, OT_REG) {
                if o0.id() != Gp::AX || o1.id() != Gp::DX {
                    return Err(no_match());
                }
                st.opcode.add((o0.x86_rm_size() != 1) as u32);
                st.opcode.add_66h_by_size(o0.x86_rm_size());
                return Ok(Handler::X86Op);
            }
            Err(no_match())
        }

        X86Ins => {
            if isign3 == ops2!(OT_MEM, OT_REG) {
                if !is_implicit_mem(&o0, Gp::DI) || o1.id() != Gp::DX {
                    return Err(no_match());
                }
                let size = o0.x86_rm_size();
                if size == 0 {
                    return Err(ambiguous_size());
                }
                st.rm_rel = o0;
                st.opcode.add((size != 1) as u32);
                st.opcode.add_66h_by_size(size);
                return Ok(Handler::X86OpImplicitMem);
            }
            Err(no_match())
        }

        X86IncDec => {
            if isign3 == ops1!(OT_REG) {
                st.rb_reg = o0.id();
                if o0.x86_rm_size() == 1 {
                    fixup_gpb_op(&mut st.options, &o0, &mut st.rb_reg);
                    return Ok(Handler::X86R);
                }
                if st.is_32bit {
                    // INC r16|r32 is only encodable in 32-bit mode (collides with REX).
                    st.opcode = alt_opcode(inst_info);
                    st.opcode.add(st.rb_reg & 0x07);
                    st.opcode.add_66h_by_size(o0.x86_rm_size());
                    return Ok(Handler::X86Op);
                }
                st.opcode.add_arith_by_size(o0.x86_rm_size());
                return Ok(Handler::X86R);
            }
            if isign3 == ops1!(OT_MEM) {
                if o0.x86_rm_size() == 0 {
                    return Err(ambiguous_size());
                }
                st.opcode.add_arith_by_size(o0.x86_rm_size());
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Int => {
            if isign3 == ops1!(OT_IMM) {
                st.imm_value = o0.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                return Ok(Handler::X86Op);
            }
            Err(no_match())
        }

        X86Jcc => {
            // Branch prediction prefixes (EncodingOptions::kPredictedJumps) are not
            // supported by asmkit.
            st.rm_rel = o0;
            st.op_reg = 0;
            Ok(Handler::JmpCall)
        }

        X86JecxzLoop => {
            st.rm_rel = o0;
            // Explicit jecxz|loop [r|e]cx, dst
            if o0.is_reg() {
                if !is_gp_with_id(&o0, Gp::CX) {
                    return Err(no_match());
                }
                emit_address_override(
                    buf,
                    (st.is_32bit && o0.x86_rm_size() == 2)
                        || (!st.is_32bit && o0.x86_rm_size() == 4),
                );
                st.rm_rel = o1;
            }
            st.op_reg = 0;
            Ok(Handler::JmpCall)
        }

        X86Jmp => {
            if isign3 == ops1!(OT_REG) {
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }
            st.rm_rel = o0;
            if isign3 == ops1!(OT_MEM) {
                return Ok(Handler::X86M);
            }
            // Jump encoded with 32-bit displacement use 0xE9 opcode. Jump encoded with
            // 8-bit displacement's opcode is stored as an alternative opcode.
            st.opcode = Opcode(0xE9);
            st.op_reg = 0;
            Ok(Handler::JmpCall)
        }

        X86JmpRel => {
            st.rm_rel = o0;
            Ok(Handler::JmpCall)
        }

        X86LcallLjmp => {
            if isign3 == ops1!(OT_MEM) {
                st.rm_rel = o0;
                let mem = o0.as_::<Mem>();
                let mut mem_size = mem.size();
                if mem_size == 0 {
                    mem_size = st.register_size(); // Native register size.
                } else {
                    mem_size = mem_size.wrapping_sub(2);
                    if mem_size != 2 && mem_size != 4 && mem_size != st.register_size() {
                        return Err(invalid("invalid far pointer size"));
                    }
                }
                st.opcode.add_prefix_by_size(mem_size);
                return Ok(Handler::X86M);
            }
            if isign3 == ops2!(OT_IMM, OT_IMM) {
                // The imm,imm (far pointer) form is 32-bit only.
                if !st.is_32bit {
                    return Err(no_match());
                }
                let imm0 = o0.as_::<crate::core::operand::Imm>().value();
                let imm1 = o1.as_::<crate::core::operand::Imm>().value();
                if imm0 as u64 > 0xFFFF {
                    return Err(invalid_imm(imm0, 2));
                }
                if imm1 as u64 > 0xFFFF_FFFF {
                    return Err(invalid_imm(imm1, 4));
                }
                st.opcode = alt_opcode(inst_info);
                st.imm_value = ((imm1 as u64) | ((imm0 as u64) << 32)) as i64;
                st.imm_size = 6;
                return Ok(Handler::X86Op);
            }
            Err(no_match())
        }

        X86Lea => {
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Mov => {
            // Reg <- Reg
            if isign3 == ops2!(OT_REG, OT_REG) {
                // AsmJit uses segment registers indexed from 1 to 6, leaving zero as
                // "no segment register used". We have to fix this (decrement the index
                // of the register) when emitting MOV instructions which move to/from a
                // segment register. The segment register is always `op_reg`, because
                // the MOV instruction uses either RM or MR encoding.

                // GP <- ??
                if o0.is_gp() {
                    st.rb_reg = o0.id();
                    st.op_reg = o1.id();

                    // GP <- GP
                    if o1.is_gp() {
                        let op_size = o0.x86_rm_size();
                        if op_size != o1.x86_rm_size() {
                            return Err(no_match());
                        }
                        if op_size == 1 {
                            fixup_gpb_op(&mut st.options, &o0, &mut st.rb_reg);
                            fixup_gpb_op(&mut st.options, &o1, &mut st.op_reg);
                            st.opcode = Opcode(0x88);
                            if !st.options.contains(InstOptions::X86_MOD_RM) {
                                return Ok(Handler::X86R);
                            }
                            st.opcode.add(2);
                            core::mem::swap(&mut st.op_reg, &mut st.rb_reg);
                            return Ok(Handler::X86R);
                        } else {
                            st.opcode = Opcode(0x89);
                            st.opcode.add_prefix_by_size(op_size);
                            if !st.options.contains(InstOptions::X86_MOD_RM) {
                                return Ok(Handler::X86R);
                            }
                            st.opcode.add(2);
                            core::mem::swap(&mut st.op_reg, &mut st.rb_reg);
                            return Ok(Handler::X86R);
                        }
                    }

                    // GP <- SReg
                    if is_segment_reg(&o1) {
                        st.opcode = Opcode(0x8C);
                        st.opcode.add_prefix_by_size(o0.x86_rm_size());
                        st.op_reg -= 1;
                        return Ok(Handler::X86R);
                    }

                    // GP <- CReg
                    if is_control_reg(&o1) {
                        st.opcode = Opcode(Opcode::K000F00 | 0x20);
                        // Use `LOCK MOV` in 32-bit mode if CR8+ register is accessed
                        // (AMD extension).
                        if st.op_reg & 0x8 != 0 && st.is_32bit {
                            buf.put1(0xF0);
                            st.op_reg &= 0x7;
                        }
                        return Ok(Handler::X86R);
                    }

                    // GP <- DReg
                    if is_debug_reg(&o1) {
                        st.opcode = Opcode(Opcode::K000F00 | 0x21);
                        return Ok(Handler::X86R);
                    }
                } else {
                    st.op_reg = o0.id();
                    st.rb_reg = o1.id();

                    // ?? <- GP
                    if !o1.is_gp() {
                        return Err(no_match());
                    }

                    // SReg <- GP
                    if is_segment_reg(&o0) {
                        st.opcode = Opcode(0x8E);
                        st.opcode.add_prefix_by_size(o1.x86_rm_size());
                        st.op_reg -= 1;
                        return Ok(Handler::X86R);
                    }

                    // CReg <- GP
                    if is_control_reg(&o0) {
                        st.opcode = Opcode(Opcode::K000F00 | 0x22);
                        // Use `LOCK MOV` in 32-bit mode if CR8+ register is accessed
                        // (AMD extension).
                        if st.op_reg & 0x8 != 0 && st.is_32bit {
                            buf.put1(0xF0);
                            st.op_reg &= 0x7;
                        }
                        return Ok(Handler::X86R);
                    }

                    // DReg <- GP
                    if is_debug_reg(&o0) {
                        st.opcode = Opcode(Opcode::K000F00 | 0x23);
                        return Ok(Handler::X86R);
                    }
                }
                return Err(no_match());
            }

            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;

                // SReg <- Mem
                if is_segment_reg(&o0) {
                    st.opcode = Opcode(0x8E);
                    st.opcode.add_prefix_by_size(o1.x86_rm_size());
                    st.op_reg -= 1;
                    return Ok(Handler::X86M);
                }
                // Reg <- Mem
                st.opcode = Opcode(0);
                st.opcode.add_arith_by_size(o0.x86_rm_size());

                // Handle a special form of `mov al|ax|eax|rax, [ptr64]` that doesn't
                // use MOD.
                if st.op_reg == Gp::AX
                    && !o1.as_::<Mem>().has_base_or_index()
                    && should_use_movabs(
                        st.is_32bit,
                        o0.x86_rm_size(),
                        st.options,
                        &o1.as_::<Mem>(),
                    )
                {
                    st.opcode.add(0xA0);
                    st.imm_value = o1.as_::<Mem>().offset();
                    return Ok(Handler::X86OpMovAbs);
                }

                if o0.x86_rm_size() == 1 {
                    fixup_gpb_op(&mut st.options, &o0, &mut st.op_reg);
                }
                st.opcode.add(0x8A);
                return Ok(Handler::X86M);
            }

            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.op_reg = o1.id();
                st.rm_rel = o0;

                // Mem <- SReg
                if is_segment_reg(&o1) {
                    st.opcode = Opcode(0x8C);
                    st.opcode.add_prefix_by_size(o0.x86_rm_size());
                    st.op_reg -= 1;
                    return Ok(Handler::X86M);
                }
                // Mem <- Reg
                st.opcode = Opcode(0);
                st.opcode.add_arith_by_size(o1.x86_rm_size());

                // Handle a special form of `mov [ptr64], al|ax|eax|rax` that doesn't
                // use MOD.
                if st.op_reg == Gp::AX
                    && !o0.as_::<Mem>().has_base_or_index()
                    && should_use_movabs(
                        st.is_32bit,
                        o1.x86_rm_size(),
                        st.options,
                        &o0.as_::<Mem>(),
                    )
                {
                    st.opcode.add(0xA2);
                    st.imm_value = o0.as_::<Mem>().offset();
                    return Ok(Handler::X86OpMovAbs);
                }

                if o1.x86_rm_size() == 1 {
                    fixup_gpb_op(&mut st.options, &o1, &mut st.op_reg);
                }
                st.opcode.add(0x88);
                return Ok(Handler::X86M);
            }

            if isign3 == ops2!(OT_REG, OT_IMM) {
                st.op_reg = o0.id();
                st.imm_size = o0.x86_rm_size() as u8;

                if st.imm_size == 1 {
                    fixup_gpb_op(&mut st.options, &o0, &mut st.op_reg);
                    st.opcode = Opcode(0xB0);
                    st.imm_value = o1.as_::<crate::core::operand::Imm>().value() as u8 as i64;
                    return Ok(Handler::X86OpReg);
                }

                // 64-bit immediate in 64-bit mode is allowed.
                st.imm_value = o1.as_::<crate::core::operand::Imm>().value();

                // Optimize the instruction size by using a 32-bit immediate if possible.
                if st.imm_size == 8 && !long_form && is_int32(st.imm_value) {
                    // Sign-extend, uses 'C7 /0' opcode.
                    st.rb_reg = st.op_reg;
                    st.opcode = Opcode(Opcode::W | 0xC7);
                    st.op_reg = 0;
                    st.imm_size = 4;
                    return Ok(Handler::X86R);
                }

                st.opcode = Opcode(0xB8);
                st.opcode.add_prefix_by_size(st.imm_size as u32);
                return Ok(Handler::X86OpReg);
            }

            if isign3 == ops2!(OT_MEM, OT_IMM) {
                let mem_size = o0.x86_rm_size();
                if mem_size == 0 {
                    return Err(ambiguous_size());
                }
                st.opcode = Opcode(0xC6 + (mem_size != 1) as u32);
                st.opcode.add_prefix_by_size(mem_size);
                st.op_reg = 0;
                st.rm_rel = o0;
                st.imm_value = o1.as_::<crate::core::operand::Imm>().value();
                st.imm_size = mem_size.min(4) as u8;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Movabs => {
            // Reg <- Mem
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                st.opcode = Opcode(0xA0);
                st.opcode.add_arith_by_size(o0.x86_rm_size());
                if !o0.is_gp() || st.op_reg != Gp::AX {
                    return Err(no_match());
                }
                if o1.as_::<Mem>().has_base_or_index() {
                    return Err(invalid(
                        "movabs requires a memory operand without base and index",
                    ));
                }
                if o1.as_::<Mem>().addr_type() == super::operands::AddrType::Rel {
                    return Err(invalid("movabs requires an absolute address"));
                }
                st.imm_value = o1.as_::<Mem>().offset();
                return Ok(Handler::X86OpMovAbs);
            }

            // Mem <- Reg
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.op_reg = o1.id();
                st.rm_rel = o0;
                st.opcode = Opcode(0xA2);
                st.opcode.add_arith_by_size(o1.x86_rm_size());
                if !o1.is_gp() || st.op_reg != Gp::AX {
                    return Err(no_match());
                }
                if o0.as_::<Mem>().has_base_or_index() {
                    return Err(invalid(
                        "movabs requires a memory operand without base and index",
                    ));
                }
                st.imm_value = o0.as_::<Mem>().offset();
                return Ok(Handler::X86OpMovAbs);
            }

            // Reg <- Imm.
            if isign3 == ops2!(OT_REG, OT_IMM) {
                if !o0.is_gp64() {
                    return Err(no_match());
                }
                st.op_reg = o0.id();
                st.opcode = Opcode(0xB8);
                st.imm_size = 8;
                st.imm_value = o1.as_::<crate::core::operand::Imm>().value();
                st.opcode.add_prefix_by_size(8);
                return Ok(Handler::X86OpReg);
            }
            Err(no_match())
        }

        X86MovsxMovzx => {
            st.opcode.add((o1.x86_rm_size() != 1) as u32);
            st.opcode.add_prefix_by_size(o0.x86_rm_size());
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                if o1.x86_rm_size() != 1 {
                    return Ok(Handler::X86R);
                }
                fixup_gpb_op(&mut st.options, &o1, &mut st.rb_reg);
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86MovntiMovdiri => {
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.opcode.add_w_if(o1.is_gp64());
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86EnqcmdMovdir64b => {
            if isign3 == ops2!(OT_MEM, OT_MEM) {
                let m0 = o0.as_::<Mem>();
                let m1 = o1.as_::<Mem>();
                // This is the only required validation, the rest is handled afterwards.
                if m0.base_type() != m1.base_type()
                    || m0.has_index()
                    || m0.has_offset()
                    || (m0.has_segment() && m0.segment_id() != SReg::ES)
                {
                    return Err(no_match());
                }
                // The first memory operand is passed via register, the second memory
                // operand is RM.
                st.op_reg = m0.base_id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Out => {
            if isign3 == ops2!(OT_IMM, OT_REG) {
                if o1.id() != Gp::AX {
                    return Err(no_match());
                }
                st.opcode = alt_opcode(inst_info);
                st.opcode.add((o1.x86_rm_size() != 1) as u32);
                st.opcode.add_66h_by_size(o1.x86_rm_size());
                st.imm_value = o0.as_::<crate::core::operand::Imm>().value() as u8 as i64;
                st.imm_size = 1;
                return Ok(Handler::X86Op);
            }
            if isign3 == ops2!(OT_REG, OT_REG) {
                if o0.id() != Gp::DX || o1.id() != Gp::AX {
                    return Err(no_match());
                }
                st.opcode.add((o1.x86_rm_size() != 1) as u32);
                st.opcode.add_66h_by_size(o1.x86_rm_size());
                return Ok(Handler::X86Op);
            }
            Err(no_match())
        }

        X86Outs => {
            if isign3 == ops2!(OT_REG, OT_MEM) {
                if o0.id() != Gp::DX || !is_implicit_mem(&o1, Gp::SI) {
                    return Err(no_match());
                }
                let size = o1.x86_rm_size();
                if size == 0 {
                    return Err(ambiguous_size());
                }
                st.rm_rel = o1;
                st.opcode.add((size != 1) as u32);
                st.opcode.add_66h_by_size(size);
                return Ok(Handler::X86OpImplicitMem);
            }
            Err(no_match())
        }

        X86Pushw => {
            if isign3 == ops1!(OT_IMM) {
                st.imm_value = o0.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 2;
                st.opcode = Opcode(0x68 | Opcode::PP_66);
                return Ok(Handler::X86Op);
            }
            Err(no_match())
        }

        X86Push => {
            if isign3 == ops1!(OT_REG) {
                if is_segment_reg(&o0) {
                    let segment = o0.id() as usize;
                    if segment >= OPCODE_PUSH_SREG_TABLE.len() {
                        return Err(invalid("invalid segment register"));
                    }
                    st.opcode = Opcode(OPCODE_PUSH_SREG_TABLE[segment]);
                    return Ok(Handler::X86Op);
                }
                return case_push_pop_gp(st, ops, inst_info);
            }
            if isign3 == ops1!(OT_IMM) {
                st.imm_value = o0.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 4;
                if is_int8(st.imm_value) && !long_form {
                    st.imm_size = 1;
                }
                st.opcode = Opcode(if st.imm_size == 1 { 0x6A } else { 0x68 });
                return Ok(Handler::X86Op);
            }
            // Fall through to the X86Pop body.
            x86_pop_body(st, ops, isign3, inst_info)
        }

        X86Pop => x86_pop_body(st, ops, isign3, inst_info),

        X86Ret => {
            if isign3 == 0 {
                // 'ret' without immediate, change C2 to C3.
                st.opcode.add(1);
                return Ok(Handler::X86Op);
            }
            if isign3 == ops1!(OT_IMM) {
                st.imm_value = o0.as_::<crate::core::operand::Imm>().value();
                if st.imm_value == 0 && !long_form {
                    // 'ret' without immediate, change C2 to C3.
                    st.opcode.add(1);
                    return Ok(Handler::X86Op);
                }
                st.imm_size = 2;
                return Ok(Handler::X86Op);
            }
            Err(no_match())
        }

        X86Rot => {
            if o0.is_reg() {
                st.opcode.add_arith_by_size(o0.x86_rm_size());
                st.rb_reg = o0.id();
                if o0.x86_rm_size() == 1 {
                    fixup_gpb_op(&mut st.options, &o0, &mut st.rb_reg);
                }

                if isign3 == ops2!(OT_REG, OT_REG) {
                    if o1.id() != Gp::CX {
                        return Err(no_match());
                    }
                    st.opcode.add(2);
                    return Ok(Handler::X86R);
                }

                if isign3 == ops2!(OT_REG, OT_IMM) {
                    st.imm_value = o1.as_::<crate::core::operand::Imm>().value() & 0xFF;
                    st.imm_size = 0;
                    if st.imm_value == 1 && !long_form {
                        return Ok(Handler::X86R);
                    }
                    st.opcode.add(-0x10i32 as u32);
                    st.imm_size = 1;
                    return Ok(Handler::X86R);
                }
            } else {
                if o0.x86_rm_size() == 0 {
                    return Err(ambiguous_size());
                }
                st.opcode.add_arith_by_size(o0.x86_rm_size());

                if isign3 == ops2!(OT_MEM, OT_REG) {
                    if o1.id() != Gp::CX {
                        return Err(no_match());
                    }
                    st.opcode.add(2);
                    st.rm_rel = o0;
                    return Ok(Handler::X86M);
                }

                if isign3 == ops2!(OT_MEM, OT_IMM) {
                    st.rm_rel = o0;
                    st.imm_value = o1.as_::<crate::core::operand::Imm>().value() & 0xFF;
                    st.imm_size = 0;
                    if st.imm_value == 1 && !long_form {
                        return Ok(Handler::X86M);
                    }
                    st.opcode.add(-0x10i32 as u32);
                    st.imm_size = 1;
                    return Ok(Handler::X86M);
                }
            }
            Err(no_match())
        }

        X86Set => {
            if isign3 == ops1!(OT_REG) {
                st.rb_reg = o0.id();
                fixup_gpb_op(&mut st.options, &o0, &mut st.rb_reg);
                return Ok(Handler::X86R);
            }
            if isign3 == ops1!(OT_MEM) {
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86ShldShrd => {
            if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                st.op_reg = o1.id();
                st.rb_reg = o0.id();
                st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                return Ok(Handler::X86R);
            }
            if isign3 == ops3!(OT_MEM, OT_REG, OT_IMM) {
                st.opcode.add_prefix_by_size(o1.x86_rm_size());
                st.op_reg = o1.id();
                st.rm_rel = o0;
                st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                return Ok(Handler::X86M);
            }

            // The following instructions use opcode + 1.
            st.opcode.add(1);

            if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
                if o2.id() != Gp::CX {
                    return Err(no_match());
                }
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                st.op_reg = o1.id();
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops3!(OT_MEM, OT_REG, OT_REG) {
                if o2.id() != Gp::CX {
                    return Err(no_match());
                }
                st.opcode.add_prefix_by_size(o1.x86_rm_size());
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86StrRm => {
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.rm_rel = o1;
                if o1.as_::<Mem>().offset_lo32() != 0 || !o0.is_gp() || o0.id() != Gp::AX {
                    return Err(no_match());
                }
                let size = o0.x86_rm_size();
                if o1.x86_rm_size() != 0 && o1.x86_rm_size() != size {
                    return Err(size_mismatch());
                }
                st.opcode.add_arith_by_size(size);
                return Ok(Handler::X86OpImplicitMem);
            }
            Err(no_match())
        }

        X86StrMr => {
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.rm_rel = o0;
                if o0.as_::<Mem>().offset_lo32() != 0 || !is_gp_with_id(&o1, Gp::AX) {
                    return Err(no_match());
                }
                let size = o1.x86_rm_size();
                if o0.x86_rm_size() != 0 && o0.x86_rm_size() != size {
                    return Err(size_mismatch());
                }
                st.opcode.add_arith_by_size(size);
                return Ok(Handler::X86OpImplicitMem);
            }
            Err(no_match())
        }

        X86StrMm => {
            if isign3 == ops2!(OT_MEM, OT_MEM) {
                if o0.as_::<Mem>().base_and_index_types() != o1.as_::<Mem>().base_and_index_types()
                {
                    return Err(no_match());
                }
                st.rm_rel = o1;
                if o0.as_::<Mem>().has_offset() {
                    return Err(no_match());
                }
                let size = o1.x86_rm_size();
                if size == 0 {
                    return Err(ambiguous_size());
                }
                if o0.x86_rm_size() != size {
                    return Err(size_mismatch());
                }
                st.opcode.add_arith_by_size(size);
                return Ok(Handler::X86OpImplicitMem);
            }
            Err(no_match())
        }

        X86Test => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                if o0.x86_rm_size() != o1.x86_rm_size() {
                    return Err(size_mismatch());
                }
                st.opcode.add_arith_by_size(o0.x86_rm_size());
                st.rb_reg = o0.id();
                st.op_reg = o1.id();
                if o0.x86_rm_size() != 1 {
                    return Ok(Handler::X86R);
                }
                fixup_gpb_op(&mut st.options, &o0, &mut st.rb_reg);
                fixup_gpb_op(&mut st.options, &o1, &mut st.op_reg);
                return Ok(Handler::X86R);
            }

            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.opcode.add_arith_by_size(o1.x86_rm_size());
                st.op_reg = o1.id();
                st.rm_rel = o0;
                if o1.x86_rm_size() != 1 {
                    return Ok(Handler::X86M);
                }
                fixup_gpb_op(&mut st.options, &o1, &mut st.op_reg);
                return Ok(Handler::X86M);
            }

            // The following instructions use the secondary opcode.
            st.opcode = alt_opcode(inst_info);
            st.op_reg = st.opcode.extract_mod_o();

            if isign3 == ops2!(OT_REG, OT_IMM) {
                st.opcode.add_arith_by_size(o0.x86_rm_size());
                st.rb_reg = o0.id();
                if o0.x86_rm_size() == 1 {
                    fixup_gpb_op(&mut st.options, &o0, &mut st.rb_reg);
                    st.imm_value = o1.as_::<crate::core::operand::Imm>().value() as u8 as i64;
                    st.imm_size = 1;
                } else {
                    st.imm_value = o1.as_::<crate::core::operand::Imm>().value();
                    st.imm_size = o0.x86_rm_size().min(4) as u8;
                }

                // Short form - AL, AX, EAX, RAX.
                if st.rb_reg == 0 && !long_form {
                    st.opcode = Opcode(
                        (st.opcode.get() & (Opcode::PP_66 | Opcode::W))
                            | (0xA8 + (o0.x86_rm_size() != 1) as u32),
                    );
                    return Ok(Handler::X86Op);
                }
                return Ok(Handler::X86R);
            }

            if isign3 == ops2!(OT_MEM, OT_IMM) {
                if o0.x86_rm_size() == 0 {
                    return Err(ambiguous_size());
                }
                st.opcode.add_arith_by_size(o0.x86_rm_size());
                st.rm_rel = o0;
                st.imm_value = o1.as_::<crate::core::operand::Imm>().value();
                st.imm_size = o0.x86_rm_size().min(4) as u8;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Xadd | X86Xchg => {
            if isign3 == ops2!(OT_REG, OT_MEM) && inst_info.encoding == X86Xchg {
                st.opcode.add_arith_by_size(o0.x86_rm_size());
                st.op_reg = o0.id();
                st.rm_rel = o1;
                if o0.x86_rm_size() != 1 {
                    return Ok(Handler::X86M);
                }
                fixup_gpb_op(&mut st.options, &o0, &mut st.op_reg);
                return Ok(Handler::X86M);
            }

            if isign3 == ops2!(OT_REG, OT_REG) {
                st.rb_reg = o0.id();
                st.op_reg = o1.id();
                let op_size = o0.x86_rm_size();
                if op_size != o1.x86_rm_size() {
                    return Err(size_mismatch());
                }
                if op_size == 1 {
                    fixup_gpb_op(&mut st.options, &o0, &mut st.rb_reg);
                    fixup_gpb_op(&mut st.options, &o1, &mut st.op_reg);
                    return Ok(Handler::X86R);
                }

                // Special cases for 'xchg ?ax, reg'.
                if inst_id == InstId::Xchg as u32 && (st.op_reg == 0 || st.rb_reg == 0) {
                    if !st.is_32bit && st.op_reg == st.rb_reg && op_size >= 4 {
                        if op_size == 8 {
                            // Encode 'xchg rax, rax' as '90' (REX and other prefixes
                            // are optional).
                            st.opcode = Opcode(st.opcode.get() & Opcode::W);
                            st.opcode = Opcode(st.opcode.get() | 0x90);
                            return Ok(Handler::X86OpReg);
                        }
                        // Encode 'xchg eax, eax' by using a generic path.
                    } else if !long_form {
                        // The special encoding encodes only one register, which is
                        // non-zero.
                        st.op_reg += st.rb_reg;
                        st.opcode.add_arith_by_size(op_size);
                        st.opcode = Opcode(st.opcode.get() & (Opcode::W | Opcode::PP_66));
                        st.opcode = Opcode(st.opcode.get() | 0x90);
                        return Ok(Handler::X86OpReg);
                    }
                }

                st.opcode.add_arith_by_size(op_size);
                return Ok(Handler::X86R);
            }

            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.opcode.add_arith_by_size(o1.x86_rm_size());
                st.op_reg = o1.id();
                st.rm_rel = o0;
                if o1.x86_rm_size() == 1 {
                    fixup_gpb_op(&mut st.options, &o1, &mut st.op_reg);
                }
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        X86Fence => {
            st.rb_reg = 0;
            Ok(Handler::X86R)
        }

        X86Bndmov => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                // ModRM encoding:
                if !st.options.contains(InstOptions::X86_MOD_MR) {
                    return Ok(Handler::X86R);
                }
                // ModMR encoding:
                st.opcode = alt_opcode(inst_info);
                core::mem::swap(&mut st.op_reg, &mut st.rb_reg);
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.opcode = alt_opcode(inst_info);
                st.rm_rel = o0;
                st.op_reg = o1.id();
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        FpuOp => Ok(Handler::FpuOp),

        FpuArith => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                // We switch to the alternative opcode if the first operand is zero.
                if st.op_reg == 0 {
                    return Ok(case_fpu_arith_reg(st));
                } else if st.rb_reg == 0 {
                    st.rb_reg = st.op_reg;
                    st.opcode = Opcode(
                        (0xDC << Opcode::FPU_2B_SHIFT) + (st.opcode.get() & 0xFF) + st.rb_reg,
                    );
                    return Ok(Handler::FpuOp);
                } else {
                    return Err(no_match());
                }
            }
            if isign3 == ops1!(OT_MEM) {
                return Ok(case_fpu_arith_mem(st, ops));
            }
            Err(no_match())
        }

        FpuCom => {
            if isign3 == 0 {
                st.rb_reg = 1;
                return Ok(case_fpu_arith_reg(st));
            }
            if isign3 == ops1!(OT_REG) {
                st.rb_reg = o0.id();
                return Ok(case_fpu_arith_reg(st));
            }
            if isign3 == ops1!(OT_MEM) {
                return Ok(case_fpu_arith_mem(st, ops));
            }
            Err(no_match())
        }

        FpuFldFst => {
            if isign3 == ops1!(OT_MEM) {
                st.rm_rel = o0;
                if o0.x86_rm_size() == 4 && common_info.has_flag(InstFlags::FPU_M32) {
                    return Ok(Handler::X86M);
                }
                if o0.x86_rm_size() == 8 && common_info.has_flag(InstFlags::FPU_M64) {
                    st.opcode.add(4);
                    return Ok(Handler::X86M);
                }
                if o0.x86_rm_size() == 10 && common_info.has_flag(InstFlags::FPU_M80) {
                    st.opcode = alt_opcode(inst_info);
                    st.op_reg = st.opcode.extract_mod_o();
                    return Ok(Handler::X86M);
                }
            }
            if isign3 == ops1!(OT_REG) {
                if inst_id == InstId::Fld as u32 {
                    st.opcode = Opcode((0xD9 << Opcode::FPU_2B_SHIFT) + 0xC0 + o0.id());
                    return Ok(Handler::FpuOp);
                }
                if inst_id == InstId::Fst as u32 {
                    st.opcode = Opcode((0xDD << Opcode::FPU_2B_SHIFT) + 0xD0 + o0.id());
                    return Ok(Handler::FpuOp);
                }
                if inst_id == InstId::Fstp as u32 {
                    st.opcode = Opcode((0xDD << Opcode::FPU_2B_SHIFT) + 0xD8 + o0.id());
                    return Ok(Handler::FpuOp);
                }
            }
            Err(no_match())
        }

        FpuM => {
            if isign3 == ops1!(OT_MEM) {
                // Clear compressed displacement before going to EmitX86M.
                st.opcode = Opcode(st.opcode.get() & !Opcode::CDSHL_MASK);
                st.rm_rel = o0;
                if o0.x86_rm_size() == 2 && common_info.has_flag(InstFlags::FPU_M16) {
                    st.opcode.add(4);
                    return Ok(Handler::X86M);
                }
                if o0.x86_rm_size() == 4 && common_info.has_flag(InstFlags::FPU_M32) {
                    return Ok(Handler::X86M);
                }
                if o0.x86_rm_size() == 8 && common_info.has_flag(InstFlags::FPU_M64) {
                    st.opcode = Opcode(alt_opcode(inst_info).get() & !Opcode::CDSHL_MASK);
                    st.op_reg = st.opcode.extract_mod_o();
                    return Ok(Handler::X86M);
                }
            }
            Err(no_match())
        }

        FpuRDef => {
            if isign3 == 0 {
                st.opcode.add(1);
                return Ok(Handler::FpuOp);
            }
            // Fall through to FpuR.
            if isign3 == ops1!(OT_REG) {
                st.opcode.add(o0.id());
                return Ok(Handler::FpuOp);
            }
            Err(no_match())
        }

        FpuR => {
            if isign3 == ops1!(OT_REG) {
                st.opcode.add(o0.id());
                return Ok(Handler::FpuOp);
            }
            Err(no_match())
        }

        FpuStsw => {
            if isign3 == ops1!(OT_REG) {
                if o0.id() != Gp::AX {
                    return Err(no_match());
                }
                st.opcode = alt_opcode(inst_info);
                return Ok(Handler::FpuOp);
            }
            if isign3 == ops1!(OT_MEM) {
                // Clear compressed displacement before going to EmitX86M.
                st.opcode = Opcode(st.opcode.get() & !Opcode::CDSHL_MASK);
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        ExtPextrw => {
            if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
                st.opcode.add_66h_if(o1.is_vec128());
                st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops3!(OT_MEM, OT_REG, OT_IMM) {
                // Secondary opcode of 'pextrw' instruction (SSE4.1).
                st.opcode = alt_opcode(inst_info);
                st.opcode.add_66h_if(o1.is_vec128());
                st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        ExtExtract => {
            if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
                st.opcode.add_66h_if(o1.is_vec128());
                st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                st.op_reg = o1.id();
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops3!(OT_MEM, OT_REG, OT_IMM) {
                st.opcode.add_66h_if(o1.is_vec128());
                st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        ExtMov => {
            // GP|MM|XMM <- GP|MM|XMM
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                if !st.options.contains(InstOptions::X86_MOD_MR) || inst_info.alt_opcode_index == 0
                {
                    return Ok(Handler::X86R);
                }
                st.opcode = alt_opcode(inst_info);
                core::mem::swap(&mut st.op_reg, &mut st.rb_reg);
                return Ok(Handler::X86R);
            }
            // GP|MM|XMM <- Mem
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            // The following instruction uses opcode[1].
            st.opcode = alt_opcode(inst_info);
            // Mem <- GP|MM|XMM
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        ExtMovbe => {
            if isign3 == ops2!(OT_REG, OT_MEM) {
                if o0.x86_rm_size() == 1 {
                    return Err(no_match());
                }
                st.opcode.add_prefix_by_size(o0.x86_rm_size());
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            // The following instruction uses the secondary opcode.
            st.opcode = alt_opcode(inst_info);
            if isign3 == ops2!(OT_MEM, OT_REG) {
                if o1.x86_rm_size() == 1 {
                    return Err(no_match());
                }
                st.opcode.add_prefix_by_size(o1.x86_rm_size());
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        ExtMovd => case_ext_movd(st, ops, isign3, inst_info),

        ExtMovq => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();

                // MM <- MM
                if is_mm_reg(&o0) && is_mm_reg(&o1) {
                    st.opcode = Opcode(Opcode::K000F00 | 0x6F);
                    if !st.options.contains(InstOptions::X86_MOD_MR) {
                        return Ok(Handler::X86R);
                    }
                    st.opcode.add(0x10);
                    core::mem::swap(&mut st.op_reg, &mut st.rb_reg);
                    return Ok(Handler::X86R);
                }

                // XMM <- XMM
                if o0.is_vec128() && o1.is_vec128() {
                    st.opcode = Opcode(Opcode::KF30F00 | 0x7E);
                    if !st.options.contains(InstOptions::X86_MOD_MR) {
                        return Ok(Handler::X86R);
                    }
                    st.opcode = Opcode(Opcode::K660F00 | 0xD6);
                    core::mem::swap(&mut st.op_reg, &mut st.rb_reg);
                    return Ok(Handler::X86R);
                }
            }

            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                // MM <- Mem
                if is_mm_reg(&o0) {
                    st.opcode = Opcode(Opcode::K000F00 | 0x6F);
                    return Ok(Handler::X86M);
                }
                // XMM <- Mem
                if o0.is_vec128() {
                    st.opcode = Opcode(Opcode::KF30F00 | 0x7E);
                    return Ok(Handler::X86M);
                }
            }

            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.op_reg = o1.id();
                st.rm_rel = o0;
                // Mem <- MM
                if is_mm_reg(&o1) {
                    st.opcode = Opcode(Opcode::K000F00 | 0x7F);
                    return Ok(Handler::X86M);
                }
                // Mem <- XMM
                if o1.is_vec128() {
                    st.opcode = Opcode(Opcode::K660F00 | 0xD6);
                    return Ok(Handler::X86M);
                }
            }

            // MOVQ in other case is simply a MOVD instruction promoted to 64-bit.
            st.opcode = Opcode(st.opcode.get() | Opcode::W);
            case_ext_movd(st, ops, isign3, inst_info)
        }

        ExtRm_XMM0 => {
            if !o2.is_none() && !is_vec128_with_id(&o2, 0) {
                return Err(no_match());
            }
            case_ext_rm(st, ops, isign3 & 0x3F)
        }

        ExtRm_ZDI => {
            if !o2.is_none() && !is_implicit_mem(&o2, Gp::DI) {
                return Err(no_match());
            }
            case_ext_rm(st, ops, isign3 & 0x3F)
        }

        ExtRm_Wx => {
            st.opcode.add_w_if(o1.x86_rm_size() == 8);
            st.opcode.add_w_if(o0.is_gp64());
            case_ext_rm(st, ops, isign3)
        }

        ExtRm_Wx_GpqOnly => {
            st.opcode.add_w_if(o0.is_gp64());
            case_ext_rm(st, ops, isign3)
        }

        ExtRm => case_ext_rm(st, ops, isign3),

        ExtRm_P => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.opcode.add_66h_if(o0.is_vec128() || o1.is_vec128());
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.opcode.add_66h_if(o0.is_vec128());
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        ExtRmRi => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            // The following instruction uses the secondary opcode.
            st.opcode = alt_opcode(inst_info);
            st.op_reg = st.opcode.extract_mod_o();
            if isign3 == ops2!(OT_REG, OT_IMM) {
                st.imm_value = o1.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }
            Err(no_match())
        }

        ExtRmRi_P => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.opcode.add_66h_if(o0.is_vec128() || o1.is_vec128());
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.opcode.add_66h_if(o0.is_vec128());
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            // The following instruction uses the secondary opcode.
            st.opcode = alt_opcode(inst_info);
            st.op_reg = st.opcode.extract_mod_o();
            if isign3 == ops2!(OT_REG, OT_IMM) {
                st.opcode.add_66h_if(o0.is_vec128());
                st.imm_value = o1.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                st.rb_reg = o0.id();
                return Ok(Handler::X86R);
            }
            Err(no_match())
        }

        ExtRmi => {
            st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
            st.imm_size = 1;
            if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops3!(OT_REG, OT_MEM, OT_IMM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        ExtRmi_P => {
            st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
            st.imm_size = 1;
            if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
                st.opcode.add_66h_if(o0.is_vec128() || o1.is_vec128());
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops3!(OT_REG, OT_MEM, OT_IMM) {
                st.opcode.add_66h_if(o0.is_vec128());
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        ExtExtrq => {
            st.op_reg = o0.id();
            st.rb_reg = o1.id();
            if isign3 == ops2!(OT_REG, OT_REG) {
                return Ok(Handler::X86R);
            }
            if isign3 == ops3!(OT_REG, OT_IMM, OT_IMM) {
                // This variant of the instruction uses the secondary opcode.
                st.opcode = alt_opcode(inst_info);
                st.rb_reg = st.op_reg;
                st.op_reg = st.opcode.extract_mod_o();
                st.imm_value = (o1.as_::<crate::core::operand::Imm>().value() as u8 as u32
                    + ((o2.as_::<crate::core::operand::Imm>().value() as u8 as u32) << 8))
                    as i64;
                st.imm_size = 2;
                return Ok(Handler::X86R);
            }
            Err(no_match())
        }

        ExtInsertq => {
            let isign4 = isign3 + (ot(&o3) << 9);
            st.op_reg = o0.id();
            st.rb_reg = o1.id();
            if isign4 == ops2!(OT_REG, OT_REG) {
                return Ok(Handler::X86R);
            }
            if isign4 == ops4!(OT_REG, OT_REG, OT_IMM, OT_IMM) {
                // This variant of the instruction uses the secondary opcode.
                st.opcode = alt_opcode(inst_info);
                st.imm_value = (o2.as_::<crate::core::operand::Imm>().value() as u8 as u32
                    + ((o3.as_::<crate::core::operand::Imm>().value() as u8 as u32) << 8))
                    as i64;
                st.imm_size = 2;
                return Ok(Handler::X86R);
            }
            Err(no_match())
        }

        Ext3dNow => {
            // Every 3dNow instruction starts with 0x0F0F and the actual opcode is
            // stored as 8-bit immediate.
            st.imm_value = (st.opcode.get() & 0xFF) as i64;
            st.imm_size = 1;
            st.opcode = Opcode(Opcode::K000F00 | 0x0F);
            st.op_reg = o0.id();
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.rb_reg = o1.id();
                return Ok(Handler::X86R);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.rm_rel = o1;
                return Ok(Handler::X86M);
            }
            Err(no_match())
        }

        VexOp => Ok(Handler::VexOp),

        VexOpMod => {
            st.rb_reg = 0;
            Ok(Handler::VexEvexR)
        }

        VexKmov => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();

                // Form 'k, reg'.
                if o1.is_gp() {
                    st.opcode = alt_opcode(inst_info);
                    return Ok(Handler::VexEvexR);
                }
                // Form 'reg, k'.
                if o0.is_gp() {
                    st.opcode = Opcode(alt_opcode(inst_info).get() + 1);
                    return Ok(Handler::VexEvexR);
                }
                // Form 'k, k'.
                if !st.options.contains(InstOptions::X86_MOD_MR) {
                    return Ok(Handler::VexEvexR);
                }
                st.opcode.add(1);
                core::mem::swap(&mut st.op_reg, &mut st.rb_reg);
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.opcode.add(1);
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexR_Wx => {
            if isign3 == ops1!(OT_REG) {
                st.rb_reg = o0.id();
                st.opcode.add_w_if(o0.is_gp64());
                return Ok(Handler::VexEvexR);
            }
            Err(no_match())
        }

        VexM => {
            if isign3 == ops1!(OT_MEM) {
                st.rm_rel = o0;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexMr_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o1.id();
                st.rb_reg = o0.id();
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexMr_VM => {
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.opcode = Opcode(
                    st.opcode.get() | opcode_l_by_vmem(&o0).max(opcode_l_by_size(o1.x86_rm_size())),
                );
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexMri_Vpextrw => {
            // Use 'vpextrw reg, xmm1, i8' when possible.
            if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
                st.opcode = Opcode(Opcode::K660F00 | 0xC5);
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
                st.imm_size = 1;
                return Ok(Handler::VexEvexR);
            }
            case_vex_mri(st, ops, isign3)
        }

        VexMvr_Wx => {
            if isign3 == ops3!(OT_MEM, OT_REG, OT_REG) {
                st.opcode.add_w_if(o1.is_gp64());
                st.op_reg = pack_reg_and_vvvvv(o1.id(), o2.id());
                st.rm_rel = o0;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexMri_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            case_vex_mri(st, ops, isign3)
        }

        VexMri => case_vex_mri(st, ops, isign3),

        VexRm_ZDI => {
            if !o2.is_none() && !is_implicit_mem(&o2, Gp::DI) {
                return Err(no_match());
            }
            case_vex_rm(st, ops, isign3 & 0x3F)
        }

        VexRm_Wx => {
            st.opcode.add_w_if(o0.is_gp64() || o1.is_gp64());
            case_vex_rm(st, ops, isign3)
        }

        VexRm_Lx_Narrow => {
            if o1.x86_rm_size() != 0 {
                st.opcode = Opcode(st.opcode.get() | opcode_l_by_size(o1.x86_rm_size()));
            } else if o0.x86_rm_size() == 32 {
                st.opcode = Opcode(st.opcode.get() | (2 << Opcode::LL_SHIFT));
            }
            case_vex_rm(st, ops, isign3)
        }

        VexRm_Lx_Bcst => {
            if isign3 == ops2!(OT_REG, OT_REG) && o1.is_gp() {
                st.opcode = Opcode(
                    alt_opcode(inst_info).get()
                        | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()),
                );
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                return Ok(Handler::VexEvexR);
            }
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            case_vex_rm(st, ops, isign3)
        }

        VexRm_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            case_vex_rm(st, ops, isign3)
        }

        VexRm => case_vex_rm(st, ops, isign3),

        VexRm_VM => {
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.opcode = Opcode(
                    st.opcode.get() | opcode_l_by_vmem(&o1).max(opcode_l_by_size(o0.x86_rm_size())),
                );
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexRmi_Wx => {
            st.opcode.add_w_if(o0.is_gp64() || o1.is_gp64());
            case_vex_rmi(st, ops, isign3)
        }

        VexRmi_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            case_vex_rmi(st, ops, isign3)
        }

        VexRmi => case_vex_rmi(st, ops, isign3),

        VexRvm => case_vex_rvm(st, ops, isign3),

        VexRvm_ZDX_Wx => {
            if !o3.is_none() && !is_gp_with_id(&o3, Gp::DX) {
                return Err(no_match());
            }
            st.opcode.add_w_if(o0.is_gp64() || o2.x86_rm_size() == 8);
            case_vex_rvm(st, ops, isign3)
        }

        VexRvm_Wx => {
            st.opcode.add_w_if(o0.is_gp64() || o2.x86_rm_size() == 8);
            case_vex_rvm(st, ops, isign3)
        }

        VexRvm_Lx_KEvex => {
            st.opcode.force_evex_if(o0.is_mask());
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            case_vex_rvm(st, ops, isign3)
        }

        VexRvm_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            case_vex_rvm(st, ops, isign3)
        }

        VexRvm_Lx_2xK => {
            if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
                // Two registers are encoded as a single register.
                //   - First K register must be even.
                //   - Second K register must be first+1.
                if (o0.id() & 1) != 0 || o0.id() + 1 != o1.id() {
                    return Err(invalid("expected two consecutive mask registers"));
                }
                st.opcode = Opcode(st.opcode.get() | opcode_l_by_size(o2.x86_rm_size()));
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                if o3.is_reg() {
                    st.rb_reg = o3.id();
                    return Ok(Handler::VexEvexR);
                }
                if o3.is_mem() {
                    st.rm_rel = o3;
                    return Ok(Handler::VexEvexM);
                }
            }
            Err(no_match())
        }

        VexRvmr_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            vex_rvmr(st, ops, isign3)
        }

        VexRvmr => vex_rvmr(st, ops, isign3),

        VexRvmi_KEvex => {
            st.opcode.force_evex_if(o0.is_mask());
            vex_rvmi(st, ops, isign3)
        }

        VexRvmi_Lx_KEvex => {
            st.opcode.force_evex_if(o0.is_mask());
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            vex_rvmi(st, ops, isign3)
        }

        VexRvmi_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            vex_rvmi(st, ops, isign3)
        }

        VexRvmi => vex_rvmi(st, ops, isign3),

        VexRmv_Wx => {
            st.opcode.add_w_if(o0.is_gp64() || o2.is_gp64());
            if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rb_reg = o1.id();
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops3!(OT_REG, OT_MEM, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexRmv => {
            if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rb_reg = o1.id();
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops3!(OT_REG, OT_MEM, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexRmvRm_VM => {
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.opcode = alt_opcode(inst_info);
                st.opcode = Opcode(
                    st.opcode.get() | opcode_l_by_vmem(&o1).max(opcode_l_by_size(o0.x86_rm_size())),
                );
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            // Fall through to VexRmv_VM.
            if isign3 == ops3!(OT_REG, OT_MEM, OT_REG) {
                st.opcode = Opcode(
                    st.opcode.get()
                        | opcode_l_by_vmem(&o1)
                            .max(opcode_l_by_size(o0.x86_rm_size() | o2.x86_rm_size())),
                );
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexRmv_VM => {
            if isign3 == ops3!(OT_REG, OT_MEM, OT_REG) {
                st.opcode = Opcode(
                    st.opcode.get()
                        | opcode_l_by_vmem(&o1)
                            .max(opcode_l_by_size(o0.x86_rm_size() | o2.x86_rm_size())),
                );
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexRmvi => {
            let isign4 = isign3 + (ot(&o3) << 9);
            st.imm_value = o3.as_::<crate::core::operand::Imm>().value();
            st.imm_size = 1;
            if isign4 == ops4!(OT_REG, OT_REG, OT_REG, OT_IMM) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rb_reg = o1.id();
                return Ok(Handler::VexEvexR);
            }
            if isign4 == ops4!(OT_REG, OT_MEM, OT_REG, OT_IMM) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexMovdMovq => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                if o0.is_gp() {
                    st.opcode = alt_opcode(inst_info);
                    st.opcode.add_w_by_size(o0.x86_rm_size());
                    st.op_reg = o1.id();
                    st.rb_reg = o0.id();
                    return Ok(Handler::VexEvexR);
                }
                if o1.is_gp() {
                    st.opcode.add_w_by_size(o1.x86_rm_size());
                    st.op_reg = o0.id();
                    st.rb_reg = o1.id();
                    return Ok(Handler::VexEvexR);
                }
                // If this is a 'W' version (movq) then allow also vmovq 'xmm|xmm' form.
                if st.opcode.get() & Opcode::EVEX_W_MASK != 0 {
                    st.opcode = Opcode(
                        (st.opcode.get() & !(Opcode::PP_VEX_MASK | Opcode::MM_MASK | 0xFF))
                            | (Opcode::KF30F00 | 0x7E),
                    );
                    st.op_reg = o0.id();
                    st.rb_reg = o1.id();
                    return Ok(Handler::VexEvexR);
                }
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                if st.opcode.get() & Opcode::EVEX_W_MASK != 0 {
                    st.opcode = Opcode(
                        (st.opcode.get() & !(Opcode::PP_VEX_MASK | Opcode::MM_MASK | 0xFF))
                            | (Opcode::KF30F00 | 0x7E),
                    );
                }
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            // The following instruction uses the secondary opcode.
            st.opcode = alt_opcode(inst_info);
            if isign3 == ops2!(OT_MEM, OT_REG) {
                if st.opcode.get() & Opcode::EVEX_W_MASK != 0 {
                    st.opcode = Opcode(
                        (st.opcode.get() & !(Opcode::PP_VEX_MASK | Opcode::MM_MASK | 0xFF))
                            | (Opcode::K660F00 | 0xD6),
                    );
                }
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexRmMr_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            vex_rm_mr(st, ops, isign3, inst_info)
        }

        VexRmMr => vex_rm_mr(st, ops, isign3, inst_info),

        VexRvmRmv => {
            if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rb_reg = o1.id();
                if !st.options.contains(InstOptions::X86_MOD_MR) {
                    return Ok(Handler::VexEvexR);
                }
                st.opcode.add_w();
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
                st.rb_reg = o2.id();
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops3!(OT_REG, OT_MEM, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            if isign3 == ops3!(OT_REG, OT_REG, OT_MEM) {
                st.opcode.add_w();
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
                st.rm_rel = o2;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexRvmRmi_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            vex_rvm_rmi(st, ops, isign3, inst_info)
        }

        VexRvmRmi => vex_rvm_rmi(st, ops, isign3, inst_info),

        VexRvmRmvRmi => {
            if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rb_reg = o1.id();
                if !st.options.contains(InstOptions::X86_MOD_MR) {
                    return Ok(Handler::VexEvexR);
                }
                st.opcode.add_w();
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
                st.rb_reg = o2.id();
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops3!(OT_REG, OT_MEM, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            if isign3 == ops3!(OT_REG, OT_REG, OT_MEM) {
                st.opcode.add_w();
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
                st.rm_rel = o2;
                return Ok(Handler::VexEvexM);
            }
            // The following instructions use the secondary opcode.
            st.opcode = alt_opcode(inst_info);
            st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
            st.imm_size = 1;
            if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
                st.op_reg = o0.id();
                st.rb_reg = o1.id();
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops3!(OT_REG, OT_MEM, OT_IMM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexRvmMr => {
            if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
                st.rb_reg = o2.id();
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops3!(OT_REG, OT_REG, OT_MEM) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
                st.rm_rel = o2;
                return Ok(Handler::VexEvexM);
            }
            // The following instructions use the secondary opcode.
            st.opcode = alt_opcode(inst_info);
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = o1.id();
                st.rb_reg = o0.id();
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexRvmMvr_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            vex_rvm_mvr(st, ops, isign3, inst_info)
        }

        VexRvmMvr => vex_rvm_mvr(st, ops, isign3, inst_info),

        VexRvmVmi_Lx_MEvex => {
            st.opcode.force_evex_if(o1.is_mem());
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            vex_rvm_vmi(st, ops, isign3, inst_info)
        }

        VexRvmVmi_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            vex_rvm_vmi(st, ops, isign3, inst_info)
        }

        VexRvmVmi => vex_rvm_vmi(st, ops, isign3, inst_info),

        VexVm_Wx => {
            st.opcode.add_w_if(o0.is_gp64() || o1.is_gp64());
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(st.op_reg, o0.id());
                st.rb_reg = o1.id();
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = pack_reg_and_vvvvv(st.op_reg, o0.id());
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexVm => {
            if isign3 == ops2!(OT_REG, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(st.op_reg, o0.id());
                st.rb_reg = o1.id();
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = pack_reg_and_vvvvv(st.op_reg, o0.id());
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexVmi_Lx_MEvex => {
            if isign3 == ops3!(OT_REG, OT_MEM, OT_IMM) {
                st.opcode.force_evex();
            }
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            vex_vmi(st, ops, isign3)
        }

        VexVmi_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            vex_vmi(st, ops, isign3)
        }

        VexVmi => vex_vmi(st, ops, isign3),

        VexVmi4_Wx => {
            st.opcode.add_w_if(o0.is_gp64() || o1.x86_rm_size() == 8);
            st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
            st.imm_size = 4;
            case_vex_vmi_after_imm(st, ops, isign3)
        }

        VexRvrmRvmr_Lx => {
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            vex_rvrm_rvmr(st, ops, isign3)
        }

        VexRvrmRvmr => vex_rvrm_rvmr(st, ops, isign3),

        VexRvrmiRvmri_Lx => {
            if !o4.is_imm() {
                return Err(no_match());
            }
            let isign4 = isign3 + (ot(&o3) << 9);
            st.opcode = Opcode(
                st.opcode.get()
                    | opcode_l_by_size(
                        o0.x86_rm_size() | o1.x86_rm_size() | o2.x86_rm_size() | o3.x86_rm_size(),
                    ),
            );
            st.imm_value = (o4.as_::<crate::core::operand::Imm>().value() as u8 & 0x0F) as i64;
            st.imm_size = 1;

            if isign4 == ops4!(OT_REG, OT_REG, OT_REG, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
                st.rb_reg = o2.id();
                st.imm_value |= (o3.id() << 4) as i64;
                return Ok(Handler::VexEvexR);
            }
            if isign4 == ops4!(OT_REG, OT_REG, OT_REG, OT_MEM) {
                st.opcode.add_w();
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
                st.rm_rel = o3;
                st.imm_value |= (o2.id() << 4) as i64;
                return Ok(Handler::VexEvexM);
            }
            if isign4 == ops4!(OT_REG, OT_REG, OT_MEM, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
                st.rm_rel = o2;
                st.imm_value |= (o3.id() << 4) as i64;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        VexMovssMovsd => {
            if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
                // CaseVexRvm_R.
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
                st.rb_reg = o2.id();
                return Ok(Handler::VexEvexR);
            }
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.opcode = alt_opcode(inst_info);
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        Fma4_Lx => {
            // It's fine to just check the first operand, second is just for sanity.
            st.opcode =
                Opcode(st.opcode.get() | opcode_l_by_size(o0.x86_rm_size() | o1.x86_rm_size()));
            fma4(st, ops, isign3)
        }

        Fma4 => fma4(st, ops, isign3),

        AmxCfg => {
            if isign3 == ops1!(OT_MEM) {
                st.rm_rel = o0;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        AmxR => {
            if isign3 == ops1!(OT_REG) {
                st.op_reg = o0.id();
                st.rb_reg = 0;
                return Ok(Handler::VexEvexR);
            }
            Err(no_match())
        }

        AmxRm => {
            if isign3 == ops2!(OT_REG, OT_MEM) {
                st.op_reg = o0.id();
                st.rm_rel = o1;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        AmxMr => {
            if isign3 == ops2!(OT_MEM, OT_REG) {
                st.op_reg = o1.id();
                st.rm_rel = o0;
                return Ok(Handler::VexEvexM);
            }
            Err(no_match())
        }

        AmxRmv => {
            if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
                st.op_reg = pack_reg_and_vvvvv(o0.id(), o2.id());
                st.rb_reg = o1.id();
                return Ok(Handler::VexEvexR);
            }
            Err(no_match())
        }

        _ => Err(no_match()),
    }
}

/// Body of `kEncodingX86Pop` (also the fallthrough tail of `kEncodingX86Push`).
fn x86_pop_body(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
    inst_info: &InstInfo,
) -> Result<Handler, X86Error> {
    let o0 = ops[0];
    if isign3 == ops1!(OT_REG) {
        if is_segment_reg(&o0) {
            let segment = o0.id();
            if segment == SReg::CS || segment as usize >= OPCODE_POP_SREG_TABLE.len() {
                return Err(invalid("invalid segment register"));
            }
            st.opcode = Opcode(OPCODE_POP_SREG_TABLE[segment as usize]);
            return Ok(Handler::X86Op);
        }
        return case_push_pop_gp(st, ops, inst_info);
    }
    if isign3 == ops1!(OT_MEM) {
        if o0.x86_rm_size() == 0 {
            return Err(ambiguous_size());
        }
        if o0.x86_rm_size() != 2 && o0.x86_rm_size() != st.register_size() {
            return Err(no_match());
        }
        st.opcode.add_66h_by_size(o0.x86_rm_size());
        st.rm_rel = o0;
        return Ok(Handler::X86M);
    }
    Err(no_match())
}

/// Body shared by `kEncodingVexRvmr` and `kEncodingVexRvmr_Lx`.
fn vex_rvmr(st: &mut X86EmitState, ops: &[Operand; 6], isign3: u32) -> Result<Handler, X86Error> {
    let (o0, o1, o2, o3) = (ops[0], ops[1], ops[2], ops[3]);
    let isign4 = isign3 + (ot(&o3) << 9);
    st.imm_value = (o3.id() << 4) as i64;
    st.imm_size = 1;

    if isign4 == ops4!(OT_REG, OT_REG, OT_REG, OT_REG) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rb_reg = o2.id();
        return Ok(Handler::VexEvexR);
    }
    if isign4 == ops4!(OT_REG, OT_REG, OT_MEM, OT_REG) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rm_rel = o2;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Body shared by the `VexRvmi*` encodings.
fn vex_rvmi(st: &mut X86EmitState, ops: &[Operand; 6], isign3: u32) -> Result<Handler, X86Error> {
    let (o0, o1, o2, o3) = (ops[0], ops[1], ops[2], ops[3]);
    let isign4 = isign3 + (ot(&o3) << 9);
    st.imm_value = o3.as_::<crate::core::operand::Imm>().value();
    st.imm_size = 1;

    if isign4 == ops4!(OT_REG, OT_REG, OT_REG, OT_IMM) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rb_reg = o2.id();
        return Ok(Handler::VexEvexR);
    }
    if isign4 == ops4!(OT_REG, OT_REG, OT_MEM, OT_IMM) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rm_rel = o2;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Body shared by `kEncodingVexRmMr` and `kEncodingVexRmMr_Lx`.
fn vex_rm_mr(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
    inst_info: &InstInfo,
) -> Result<Handler, X86Error> {
    let (o0, o1) = (ops[0], ops[1]);
    if isign3 == ops2!(OT_REG, OT_REG) {
        st.op_reg = o0.id();
        st.rb_reg = o1.id();
        return Ok(Handler::VexEvexR);
    }
    if isign3 == ops2!(OT_REG, OT_MEM) {
        st.op_reg = o0.id();
        st.rm_rel = o1;
        return Ok(Handler::VexEvexM);
    }
    // The following instruction uses the secondary opcode.
    st.opcode = Opcode(st.opcode.get() & Opcode::LL_MASK);
    st.opcode = Opcode(st.opcode.get() | alt_opcode(inst_info).get());
    if isign3 == ops2!(OT_MEM, OT_REG) {
        st.op_reg = o1.id();
        st.rm_rel = o0;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Body shared by `kEncodingVexRvmRmi` and `kEncodingVexRvmRmi_Lx`.
fn vex_rvm_rmi(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
    inst_info: &InstInfo,
) -> Result<Handler, X86Error> {
    let (o0, o1, o2) = (ops[0], ops[1], ops[2]);
    if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rb_reg = o2.id();
        return Ok(Handler::VexEvexR);
    }
    if isign3 == ops3!(OT_REG, OT_REG, OT_MEM) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rm_rel = o2;
        return Ok(Handler::VexEvexM);
    }
    // The following instructions use the secondary opcode.
    st.opcode = Opcode(st.opcode.get() & Opcode::LL_MASK);
    st.opcode = Opcode(st.opcode.get() | alt_opcode(inst_info).get());
    st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
    st.imm_size = 1;
    if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
        st.op_reg = o0.id();
        st.rb_reg = o1.id();
        return Ok(Handler::VexEvexR);
    }
    if isign3 == ops3!(OT_REG, OT_MEM, OT_IMM) {
        st.op_reg = o0.id();
        st.rm_rel = o1;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Body shared by `kEncodingVexRvmMvr` and `kEncodingVexRvmMvr_Lx`.
fn vex_rvm_mvr(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
    inst_info: &InstInfo,
) -> Result<Handler, X86Error> {
    let (o0, o1, o2) = (ops[0], ops[1], ops[2]);
    if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rb_reg = o2.id();
        return Ok(Handler::VexEvexR);
    }
    if isign3 == ops3!(OT_REG, OT_REG, OT_MEM) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rm_rel = o2;
        return Ok(Handler::VexEvexM);
    }
    // The following instructions use the secondary opcode.
    st.opcode = Opcode(st.opcode.get() & Opcode::LL_MASK);
    st.opcode = Opcode(st.opcode.get() | alt_opcode(inst_info).get());
    if isign3 == ops3!(OT_MEM, OT_REG, OT_REG) {
        st.op_reg = pack_reg_and_vvvvv(o2.id(), o1.id());
        st.rm_rel = o0;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Body shared by the `VexRvmVmi*` encodings.
fn vex_rvm_vmi(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
    inst_info: &InstInfo,
) -> Result<Handler, X86Error> {
    let (o0, o1, o2) = (ops[0], ops[1], ops[2]);
    if isign3 == ops3!(OT_REG, OT_REG, OT_REG) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rb_reg = o2.id();
        return Ok(Handler::VexEvexR);
    }
    if isign3 == ops3!(OT_REG, OT_REG, OT_MEM) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rm_rel = o2;
        return Ok(Handler::VexEvexM);
    }
    // The following instructions use the secondary opcode.
    st.opcode = Opcode(st.opcode.get() & (Opcode::LL_MASK | Opcode::MM_FORCE_EVEX));
    st.opcode = Opcode(st.opcode.get() | alt_opcode(inst_info).get());
    st.op_reg = st.opcode.extract_mod_o();
    st.imm_value = o2.as_::<crate::core::operand::Imm>().value();
    st.imm_size = 1;
    if isign3 == ops3!(OT_REG, OT_REG, OT_IMM) {
        st.op_reg = pack_reg_and_vvvvv(st.op_reg, o0.id());
        st.rb_reg = o1.id();
        return Ok(Handler::VexEvexR);
    }
    if isign3 == ops3!(OT_REG, OT_MEM, OT_IMM) {
        st.op_reg = pack_reg_and_vvvvv(st.op_reg, o0.id());
        st.rm_rel = o1;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Body shared by the `VexVmi*` encodings.
fn vex_vmi(st: &mut X86EmitState, ops: &[Operand; 6], isign3: u32) -> Result<Handler, X86Error> {
    st.imm_value = ops[2].as_::<crate::core::operand::Imm>().value();
    st.imm_size = 1;
    case_vex_vmi_after_imm(st, ops, isign3)
}

/// Body shared by `kEncodingVexRvrmRvmr` and `kEncodingVexRvrmRvmr_Lx`.
fn vex_rvrm_rvmr(
    st: &mut X86EmitState,
    ops: &[Operand; 6],
    isign3: u32,
) -> Result<Handler, X86Error> {
    let (o0, o1, o2, o3) = (ops[0], ops[1], ops[2], ops[3]);
    let isign4 = isign3 + (ot(&o3) << 9);

    if isign4 == ops4!(OT_REG, OT_REG, OT_REG, OT_REG) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rb_reg = o2.id();
        st.imm_value = (o3.id() << 4) as i64;
        st.imm_size = 1;
        return Ok(Handler::VexEvexR);
    }
    if isign4 == ops4!(OT_REG, OT_REG, OT_REG, OT_MEM) {
        st.opcode.add_w();
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rm_rel = o3;
        st.imm_value = (o2.id() << 4) as i64;
        st.imm_size = 1;
        return Ok(Handler::VexEvexM);
    }
    if isign4 == ops4!(OT_REG, OT_REG, OT_MEM, OT_REG) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rm_rel = o2;
        st.imm_value = (o3.id() << 4) as i64;
        st.imm_size = 1;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Body shared by `kEncodingFma4` and `kEncodingFma4_Lx`.
fn fma4(st: &mut X86EmitState, ops: &[Operand; 6], isign3: u32) -> Result<Handler, X86Error> {
    let (o0, o1, o2, o3) = (ops[0], ops[1], ops[2], ops[3]);
    let isign4 = isign3 + (ot(&o3) << 9);

    if isign4 == ops4!(OT_REG, OT_REG, OT_REG, OT_REG) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        if !st.options.contains(InstOptions::X86_MOD_MR) {
            // MOD/RM - Encoding preferred by LLVM.
            st.opcode.add_w();
            st.rb_reg = o3.id();
            st.imm_value = (o2.id() << 4) as i64;
            st.imm_size = 1;
            return Ok(Handler::VexEvexR);
        } else {
            // MOD/MR - Alternative encoding.
            st.rb_reg = o2.id();
            st.imm_value = (o3.id() << 4) as i64;
            st.imm_size = 1;
            return Ok(Handler::VexEvexR);
        }
    }
    if isign4 == ops4!(OT_REG, OT_REG, OT_REG, OT_MEM) {
        st.opcode.add_w();
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rm_rel = o3;
        st.imm_value = (o2.id() << 4) as i64;
        st.imm_size = 1;
        return Ok(Handler::VexEvexM);
    }
    if isign4 == ops4!(OT_REG, OT_REG, OT_MEM, OT_REG) {
        st.op_reg = pack_reg_and_vvvvv(o0.id(), o1.id());
        st.rm_rel = o2;
        st.imm_value = (o3.id() << 4) as i64;
        st.imm_size = 1;
        return Ok(Handler::VexEvexM);
    }
    Err(no_match())
}

/// Decoded prefix state collected by the assembler's prefix setters.
#[derive(Clone, Copy, Debug)]
pub struct PendingPrefixes {
    pub options: InstOptions,
    /// Segment override (SReg id, 0 = none) applied to the first memory operand.
    pub segment_id: u32,
    /// AVX-512 {k} mask register id (0 = none).
    pub mask_id: u32,
}

impl Default for PendingPrefixes {
    fn default() -> Self {
        Self {
            options: InstOptions::NONE,
            segment_id: 0,
            mask_id: 0,
        }
    }
}

/// Resolves AsmJit's instruction-wide feature union to the requirements of the
/// selected operand/prefix form.
fn validate_cpu_features(
    buf: &CodeBuffer,
    inst_id: InstId,
    common_info: CommonInfo,
    options: InstOptions,
    mask_id: u32,
    ops: &[Operand; 6],
) -> Result<(), AsmError> {
    let inst_info = INST_INFO_TABLE[inst_id as usize];
    let features = ADDITIONAL_INFO_TABLE[inst_info.additional_info_index as usize].features;
    let mut required = [false; CPU_FEATURE_COUNT];
    for feature in features {
        if feature != 0 {
            required[feature as usize] = true;
        }
    }

    let mut has_xmm = false;
    let mut has_ymm = false;
    let mut has_zmm = false;
    let mut has_mask = mask_id != 0;
    let mut high_vec_used = false;
    for op in ops {
        let (reg_type, reg_id) = if op.is_reg() {
            (op.signature.try_reg_type(), op.id())
        } else if op.is_mem() {
            let mem = op.as_::<Mem>();
            (Some(mem.index_type()), mem.index_id())
        } else {
            (None, 0)
        };
        match reg_type {
            Some(RegType::Vec128) => has_xmm = true,
            Some(RegType::Vec256) => has_ymm = true,
            Some(RegType::Vec512) => has_zmm = true,
            Some(RegType::Mask) => has_mask = true,
            _ => continue,
        }
        high_vec_used |= reg_id >= 16;
    }

    // MMX/SSE overlap.
    if (required[CpuFeature::MMX as usize] || required[CpuFeature::MMX2 as usize])
        && (required[CpuFeature::SSE as usize] || required[CpuFeature::SSE2 as usize])
    {
        if has_xmm {
            required[CpuFeature::MMX as usize] = false;
            required[CpuFeature::MMX2 as usize] = false;
        } else {
            required[CpuFeature::SSE as usize] = false;
            required[CpuFeature::SSE2 as usize] = false;
            required[CpuFeature::SSE4_1 as usize] = false;
        }
        if inst_id == InstId::Pextrw {
            if ops[0].is_mem() {
                required[CpuFeature::SSE2 as usize] = false;
            } else {
                required[CpuFeature::SSE4_1 as usize] = false;
            }
        }
    }

    // PCLMULQDQ/VPCLMULQDQ overlap.
    if required[CpuFeature::VPCLMULQDQ as usize] {
        if has_zmm || options.contains(InstOptions::X86_EVEX) {
            required[CpuFeature::AVX as usize] = false;
            required[CpuFeature::PCLMULQDQ as usize] = false;
        } else if has_ymm {
            required[CpuFeature::AVX512_F as usize] = false;
            required[CpuFeature::AVX512_VL as usize] = false;
        } else {
            required[CpuFeature::AVX512_F as usize] = false;
            required[CpuFeature::AVX512_VL as usize] = false;
            required[CpuFeature::VPCLMULQDQ as usize] = false;
        }
    }

    // AVX/AVX2 overlap.
    if required[CpuFeature::AVX as usize] && required[CpuFeature::AVX2 as usize] {
        let avx2 = if matches!(inst_id, InstId::Vbroadcastss | InstId::Vbroadcastsd) {
            !ops[1].is_mem()
        } else {
            has_ymm || has_zmm
        };
        required[if avx2 {
            CpuFeature::AVX as usize
        } else {
            CpuFeature::AVX2 as usize
        }] = false;
    }

    let has_avx = [
        CpuFeature::AVX,
        CpuFeature::AVX_IFMA,
        CpuFeature::AVX_NE_CONVERT,
        CpuFeature::AVX_VNNI,
        CpuFeature::AVX2,
        CpuFeature::F16C,
        CpuFeature::FMA,
    ]
    .iter()
    .any(|feature| required[*feature as usize]);
    let has_avx512 = [
        CpuFeature::AVX512_BF16,
        CpuFeature::AVX512_BW,
        CpuFeature::AVX512_DQ,
        CpuFeature::AVX512_F,
        CpuFeature::AVX512_IFMA,
        CpuFeature::AVX512_VNNI,
    ]
    .iter()
    .any(|feature| required[*feature as usize]);

    if has_avx && has_avx512 {
        let mut use_evex = options.intersects(
            InstOptions::X86_EVEX
                | InstOptions::X86_ZMASK
                | InstOptions::X86_ER
                | InstOptions::X86_SAE,
        ) || has_mask
            || has_zmm
            || high_vec_used;

        use_evex |= match inst_id {
            InstId::Vpbroadcastb
            | InstId::Vpbroadcastd
            | InstId::Vpbroadcastq
            | InstId::Vpbroadcastw => ops[1].is_gp(),
            InstId::Vcvtpd2dq | InstId::Vcvtpd2ps | InstId::Vcvttpd2dq => ops[0].is_vec256(),
            InstId::Vgatherdpd
            | InstId::Vgatherdps
            | InstId::Vgatherqpd
            | InstId::Vgatherqps
            | InstId::Vpgatherdd
            | InstId::Vpgatherdq
            | InstId::Vpgatherqd
            | InstId::Vpgatherqq => ops[2].is_none(),
            InstId::Vpslldq
            | InstId::Vpslld
            | InstId::Vpsllq
            | InstId::Vpsllw
            | InstId::Vpsrad
            | InstId::Vpsraq
            | InstId::Vpsraw
            | InstId::Vpsrld
            | InstId::Vpsrldq
            | InstId::Vpsrlq
            | InstId::Vpsrlw => ops[1].is_mem(),
            InstId::Vpermpd => !ops[2].is_imm(),
            InstId::Vpermq => ops[1].is_mem() || !ops[2].is_imm(),
            _ => false,
        };
        if common_info.has_flag(InstFlags::PREFER_EVEX)
            && !options.intersects(InstOptions::X86_VEX | InstOptions::X86_VEX3)
        {
            use_evex = true;
        }

        let alternatives: &[CpuFeature] = if use_evex {
            &[
                CpuFeature::AVX,
                CpuFeature::AVX_IFMA,
                CpuFeature::AVX_NE_CONVERT,
                CpuFeature::AVX_VNNI,
                CpuFeature::AVX2,
                CpuFeature::F16C,
                CpuFeature::FMA,
            ]
        } else {
            &[
                CpuFeature::AVX512_BF16,
                CpuFeature::AVX512_BW,
                CpuFeature::AVX512_DQ,
                CpuFeature::AVX512_F,
                CpuFeature::AVX512_IFMA,
                CpuFeature::AVX512_VL,
                CpuFeature::AVX512_VNNI,
            ]
        };
        for feature in alternatives {
            required[*feature as usize] = false;
        }
    }

    if has_zmm {
        required[CpuFeature::AVX512_VL as usize] = false;
    }

    for (feature, is_required) in required.into_iter().enumerate() {
        if is_required && !buf.env().x86_feature_id(feature as u8) {
            return Err(AsmError::MissingCpuFeature {
                feature: CPU_FEATURE_NAMES[feature],
            });
        }
    }
    Ok(())
}

/// Emits one instruction: looks up the InstInfo, validates the operand signature,
/// emits pending LOCK/REP prefixes, runs the operand analysis, and dispatches to the
/// selected emit handler. Port of AsmJit's `Assembler::_emit`; `is_32bit` selects
/// the 32-bit X86 mode (AsmJit's `Assembler::is_32bit()`).
pub fn emit_n(
    buf: &mut CodeBuffer,
    inst_id: u32,
    ops: &[&Operand],
    prefixes: PendingPrefixes,
    is_32bit: bool,
) -> Result<(), AsmError> {
    if inst_id == 0 || inst_id as usize >= INST_INFO_TABLE.len() {
        return Err(invalid("unknown instruction id").into());
    }

    if ops.len() > 6 {
        return Err(invalid("instructions support at most six operands").into());
    }

    let inst_info = INST_INFO_TABLE[inst_id as usize];
    let common_info = INST_COMMON_INFO_TABLE[inst_info.common_info_index as usize];

    let mut ops_array = [Operand::new(); 6];
    for (i, op) in ops.iter().enumerate() {
        ops_array[i] = **op;
        validate_raw_operand(buf, op, i, is_32bit)?;
    }

    let options = prefixes.options;
    if options.contains(InstOptions::X86_MOD_MR) && options.contains(InstOptions::X86_MOD_RM) {
        return Err(X86Error::InvalidPrefix {
            prefix: options.bits() as u64,
            reason: "ModMR and ModRM selection options conflict",
        }
        .into());
    }
    if options.contains(InstOptions::X86_VEX) && options.contains(InstOptions::X86_EVEX) {
        return Err(X86Error::InvalidPrefix {
            prefix: options.bits() as u64,
            reason: "VEX and EVEX selection options conflict",
        }
        .into());
    }
    if options.contains(InstOptions::X86_REP) && options.contains(InstOptions::X86_REPNE) {
        return Err(X86Error::InvalidPrefix {
            prefix: 0,
            reason: "REP and REPNE prefixes conflict",
        }
        .into());
    }
    if options.contains(InstOptions::X86_LOCK)
        && options.intersects(InstOptions::X86_REP | InstOptions::X86_REPNE)
    {
        return Err(X86Error::InvalidPrefix {
            prefix: 0,
            reason: "LOCK and REP prefixes conflict",
        }
        .into());
    }
    if options.contains(InstOptions::X86_ER) && options.contains(InstOptions::X86_SAE) {
        return Err(X86Error::InvalidRoundingControl {
            rc: options.bits() as u64,
            reason: "embedded rounding and standalone SAE conflict",
        }
        .into());
    }
    if options.intersects(InstOptions::X86_ER_MASK) && !options.contains(InstOptions::X86_ER) {
        return Err(X86Error::InvalidRoundingControl {
            rc: options.bits() as u64,
            reason: "rounding mode requires embedded rounding",
        }
        .into());
    }

    // Apply a valid pending segment override to the first memory operand.
    if prefixes.segment_id != 0 && prefixes.segment_id > SReg::GS {
        return Err(X86Error::InvalidPrefix {
            prefix: prefixes.segment_id as u64,
            reason: "invalid segment override",
        }
        .into());
    }
    if prefixes.segment_id != 0 {
        let mut applied = false;
        for op in ops_array.iter_mut() {
            if op.is_mem() {
                let mut mem = op.as_::<Mem>();
                if mem.has_segment() && mem.segment_id() != prefixes.segment_id {
                    return Err(X86Error::InvalidPrefix {
                        prefix: prefixes.segment_id as u64,
                        reason: "segment override conflicts with the memory operand",
                    }
                    .into());
                }
                mem.set_segment_id(prefixes.segment_id);
                *op = *mem.as_operand();
                applied = true;
                break;
            }
        }
        if !applied {
            return Err(X86Error::InvalidPrefix {
                prefix: prefixes.segment_id as u64,
                reason: "segment override requires a memory operand",
            }
            .into());
        }
    }

    // Validate operands against the instruction's signature records.
    validate_signature(&common_info, &ops_array, is_32bit)?;

    // SAFETY: `InstId` is a dense `repr(u32)` enum and the bounds check at the
    // start of this function excluded both `None` and every value at/after `_Count`.
    let inst_id_enum = unsafe { core::mem::transmute::<u32, InstId>(inst_id) };
    validate_cpu_features(
        buf,
        inst_id_enum,
        common_info,
        options,
        prefixes.mask_id,
        &ops_array,
    )?;

    // LOCK prefix (XACQUIRE/XRELEASE are not exposed by asmkit's prefix setters).
    if options.contains(InstOptions::X86_LOCK)
        && (!common_info.has_flag(InstFlags::LOCK) || !ops_array[0].is_mem())
    {
        return Err(X86Error::InvalidPrefix {
            prefix: 0xF0,
            reason: "LOCK requires a lockable memory-destination form",
        }
        .into());
    }

    // REP and REPNE prefixes.
    if options.intersects(InstOptions::X86_REP | InstOptions::X86_REPNE)
        && !common_info.has_flag(InstFlags::REP)
        && !common_info.has_flag(InstFlags::REP_IGNORED)
    {
        return Err(X86Error::InvalidPrefix {
            prefix: 0xF3,
            reason: "instruction cannot be used with a REP prefix",
        }
        .into());
    }

    if prefixes.mask_id > 7 {
        return Err(X86Error::InvalidMasking {
            mask_reg: prefixes.mask_id,
            reason: "mask register id must be in k0..k7",
        }
        .into());
    }
    if options.contains(InstOptions::X86_ZMASK) && prefixes.mask_id == 0 {
        return Err(X86Error::InvalidMasking {
            mask_reg: 0,
            reason: "zeroing requires a nonzero mask register",
        }
        .into());
    }
    if prefixes.mask_id != 0 && !common_info.has_flag(InstFlags::EVEX) {
        return Err(X86Error::InvalidMasking {
            mask_reg: prefixes.mask_id,
            reason: "instruction has no EVEX masking form",
        }
        .into());
    }
    if options.contains(InstOptions::X86_ZMASK) && !ops_array[0].is_reg() {
        return Err(X86Error::InvalidMasking {
            mask_reg: prefixes.mask_id,
            reason: "zeroing requires a register destination",
        }
        .into());
    }
    if options.contains(InstOptions::X86_ER)
        && !common_info.has_avx512_flag(super::instdb::Avx512Flags::ER)
    {
        return Err(X86Error::InvalidRoundingControl {
            rc: options.bits() as u64,
            reason: "instruction does not support embedded rounding",
        }
        .into());
    }
    if options.contains(InstOptions::X86_SAE)
        && !common_info.has_avx512_flag(super::instdb::Avx512Flags::SAE)
    {
        return Err(X86Error::InvalidRoundingControl {
            rc: options.bits() as u64,
            reason: "instruction does not support suppress-all-exceptions",
        }
        .into());
    }
    if options.intersects(InstOptions::X86_ER | InstOptions::X86_SAE)
        && ops_array.iter().any(Operand::is_mem)
    {
        return Err(X86Error::InvalidRoundingControl {
            rc: options.bits() as u64,
            reason: "rounding control is not encodable with a memory operand",
        }
        .into());
    }

    if options.contains(InstOptions::X86_LOCK) {
        buf.put1(0xF0);
    }
    if options.intersects(InstOptions::X86_REP | InstOptions::X86_REPNE) {
        buf.put1(if options.contains(InstOptions::X86_REPNE) {
            0xF2
        } else {
            0xF3
        });
    }

    // This sequence seems to be the fastest (AsmJit).
    let mut st = X86EmitState {
        is_32bit,
        opcode: Opcode(MAIN_OPCODE_TABLE[inst_info.main_opcode_index as usize]),
        options,
        inst_id,
        inst_info,
        common_info,
        extra_reg: if prefixes.mask_id != 0 {
            *KReg::from_id(prefixes.mask_id).as_operand()
        } else {
            *KReg::from_id(0).as_operand()
        },
        ..Default::default()
    };
    st.op_reg = st.opcode.extract_mod_o();
    st.opcode = Opcode(st.opcode.get() | inst_info.main_opcode_value as u32);

    // Signature of the first 3 operands.
    let isign3 = ot(&ops_array[0]) + (ot(&ops_array[1]) << 3) + (ot(&ops_array[2]) << 6);
    st.isign3 = isign3;

    if inst_info.encoding == super::instdb::Encoding::None as u8 {
        // AsmJit's `kEncodingNone` jumps to EmitDone.
        return Ok(());
    }

    let handler = analyze(
        buf,
        &mut st,
        &ops_array,
        isign3,
        &inst_info,
        &common_info,
        inst_id,
    )?;

    if st.rm_rel.is_label() {
        st.label_id = st.rm_rel.id();
    }

    match handler {
        Handler::X86Op => emit_x86_op(buf, &mut st),
        Handler::X86OpMovAbs => emit_x86_op_mov_abs(buf, &mut st),
        Handler::X86OpReg => emit_x86_op_reg(buf, &mut st),
        Handler::X86OpImplicitMem => emit_x86_op_implicit_mem(buf, &mut st),
        Handler::X86R => emit_x86_r(buf, &mut st),
        Handler::X86RFromM => emit_x86_r_from_m(buf, &mut st),
        Handler::X86M => emit_x86_m(buf, &mut st),
        Handler::FpuOp => emit_fpu_op(buf, &mut st),
        Handler::VexOp => emit_vex_op(buf, &mut st),
        Handler::VexEvexR => emit_vex_evex_r(buf, &mut st),
        Handler::VexEvexM => emit_vex_evex_m(buf, &mut st),
        Handler::JmpCall => emit_jmp_call(buf, &mut st),
    }
    .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::arch_traits::Arch;
    use crate::core::operand::OperandCast;
    use crate::core::target::Environment;
    use crate::x86::operands::regs::*;
    use crate::x86::operands::{Mem, dword_ptr};

    fn emit(
        inst: InstId,
        ops: &[&Operand],
        prefixes: PendingPrefixes,
    ) -> (Vec<u8>, Option<X86Error>) {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let error = emit_n(&mut buf, inst as u32, ops, prefixes, false)
            .err()
            .map(|error| match error {
                AsmError::X86(error) => error,
                error => panic!("unexpected error: {error}"),
            });
        (buf.data().to_vec(), error)
    }

    fn assert_default_feature_can_emit_and_can_be_disabled(
        feature: CpuFeature,
        inst: InstId,
        ops: &[&Operand],
    ) {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        emit_n(
            &mut buf,
            inst as u32,
            ops,
            PendingPrefixes::default(),
            false,
        )
        .unwrap();
        assert!(!buf.data().is_empty(), "{feature:?} did not emit {inst:?}");

        let mut env = Environment::new(Arch::X64);
        env.set_x86_feature(feature, false);
        let mut buf = CodeBuffer::new(env);
        assert_eq!(
            emit_n(
                &mut buf,
                inst as u32,
                ops,
                PendingPrefixes::default(),
                false
            ),
            Err(AsmError::MissingCpuFeature {
                feature: CPU_FEATURE_NAMES[feature as usize],
            }),
            "{feature:?} did not gate {inst:?}",
        );
        assert!(buf.data().is_empty());
    }

    #[test]
    fn default_vector_extensions_emit_and_are_feature_gated() {
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX,
            InstId::Vaddsubpd,
            &[XMM1.as_operand(), XMM2.as_operand(), XMM3.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX2,
            InstId::Vpaddd,
            &[YMM1.as_operand(), YMM2.as_operand(), YMM3.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_BF16,
            InstId::Vcvtne2ps2bf16,
            &[ZMM1.as_operand(), ZMM2.as_operand(), ZMM3.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_BITALG,
            InstId::Vpopcntb,
            &[ZMM1.as_operand(), ZMM2.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_BW,
            InstId::Vpmovm2b,
            &[ZMM1.as_operand(), K2.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_CD,
            InstId::Vplzcntd,
            &[ZMM1.as_operand(), ZMM2.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_DQ,
            InstId::Vpmullq,
            &[ZMM1.as_operand(), ZMM2.as_operand(), ZMM3.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_F,
            InstId::Kxorw,
            &[K1.as_operand(), K2.as_operand(), K3.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_FP16,
            InstId::Vaddph,
            &[ZMM1.as_operand(), ZMM2.as_operand(), ZMM3.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_IFMA,
            InstId::Vpmadd52luq,
            &[ZMM1.as_operand(), ZMM2.as_operand(), ZMM3.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_VBMI,
            InstId::Vpermb,
            &[ZMM1.as_operand(), ZMM2.as_operand(), ZMM3.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_VBMI2,
            InstId::Vpcompressb,
            &[ZMM1.as_operand(), ZMM2.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_VL,
            InstId::Vpmovm2d,
            &[YMM1.as_operand(), K2.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_VNNI,
            InstId::Vpdpbusd,
            &[ZMM1.as_operand(), ZMM2.as_operand(), ZMM3.as_operand()],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_VP2INTERSECT,
            InstId::Vp2intersectd,
            &[
                K0.as_operand(),
                K1.as_operand(),
                ZMM3.as_operand(),
                ZMM4.as_operand(),
            ],
        );
        assert_default_feature_can_emit_and_can_be_disabled(
            CpuFeature::AVX512_VPOPCNTDQ,
            InstId::Vpopcntd,
            &[ZMM1.as_operand(), ZMM2.as_operand()],
        );
    }

    #[test]
    fn singleton_feature_requirement_is_checked_before_emission() {
        let mut env =
            crate::core::target::Environment::baseline(crate::core::arch_traits::Arch::X64);
        let mut buf = CodeBuffer::new(env);
        let error = emit_n(
            &mut buf,
            InstId::Vaddsubpd as u32,
            &[XMM1.as_operand(), XMM2.as_operand(), XMM3.as_operand()],
            PendingPrefixes::default(),
            false,
        )
        .unwrap_err();
        assert_eq!(error, AsmError::MissingCpuFeature { feature: "AVX" });
        assert!(buf.data().is_empty());

        env.set_x86_feature(super::super::instdb::CpuFeature::AVX, true);
        let mut buf = CodeBuffer::new(env);
        emit_n(
            &mut buf,
            InstId::Vaddsubpd as u32,
            &[XMM1.as_operand(), XMM2.as_operand(), XMM3.as_operand()],
            PendingPrefixes::default(),
            false,
        )
        .unwrap();
        assert!(!buf.data().is_empty());

        let mut env =
            crate::core::target::Environment::baseline(crate::core::arch_traits::Arch::X64);
        let mem = super::super::operands::ptr(RAX, 0, 16);
        let mut buf = CodeBuffer::new(env);
        let error = emit_n(
            &mut buf,
            InstId::Vbroadcasti128 as u32,
            &[YMM1.as_operand(), mem.as_operand()],
            PendingPrefixes::default(),
            false,
        )
        .unwrap_err();
        assert_eq!(error, AsmError::MissingCpuFeature { feature: "AVX2" });
        assert!(buf.data().is_empty());

        env.set_x86_feature(super::super::instdb::CpuFeature::AVX2, true);
        let mut buf = CodeBuffer::new(env);
        emit_n(
            &mut buf,
            InstId::Vbroadcasti128 as u32,
            &[YMM1.as_operand(), mem.as_operand()],
            PendingPrefixes::default(),
            false,
        )
        .unwrap();
        assert!(!buf.data().is_empty());

        env.set_x86_feature(super::super::instdb::CpuFeature::AVX2, false);
        let mut buf = CodeBuffer::new(env);
        let error = emit_n(
            &mut buf,
            InstId::Kxorw as u32,
            &[K1.as_operand(), K2.as_operand(), K3.as_operand()],
            PendingPrefixes::default(),
            false,
        )
        .unwrap_err();
        assert_eq!(
            error,
            AsmError::MissingCpuFeature {
                feature: "AVX512_F"
            }
        );
        assert!(buf.data().is_empty());

        env.set_x86_feature(super::super::instdb::CpuFeature::AVX512_F, true);
        let mut buf = CodeBuffer::new(env);
        emit_n(
            &mut buf,
            InstId::Kxorw as u32,
            &[K1.as_operand(), K2.as_operand(), K3.as_operand()],
            PendingPrefixes::default(),
            false,
        )
        .unwrap();
        assert!(!buf.data().is_empty());
    }

    #[test]
    fn multi_feature_requirements_follow_the_selected_form() {
        use super::super::instdb::CpuFeature;

        let mut env =
            crate::core::target::Environment::baseline(crate::core::arch_traits::Arch::X64);
        env.set_x86_feature(CpuFeature::FPU, true);
        let mut buf = CodeBuffer::new(env);
        let error = emit_n(
            &mut buf,
            InstId::Fcmovb as u32,
            &[ST1.as_operand()],
            PendingPrefixes::default(),
            false,
        )
        .unwrap_err();
        assert_eq!(error, AsmError::MissingCpuFeature { feature: "CMOV" });
        assert!(buf.data().is_empty());

        env.set_x86_feature(CpuFeature::CMOV, true);
        let mut buf = CodeBuffer::new(env);
        emit_n(
            &mut buf,
            InstId::Fcmovb as u32,
            &[ST1.as_operand()],
            PendingPrefixes::default(),
            false,
        )
        .unwrap();
        assert!(!buf.data().is_empty());

        let mut env =
            crate::core::target::Environment::baseline(crate::core::arch_traits::Arch::X64);
        env.set_x86_feature(CpuFeature::AVX, true);
        let mut buf = CodeBuffer::new(env);
        emit_n(
            &mut buf,
            InstId::Vpaddd as u32,
            &[XMM1.as_operand(), XMM2.as_operand(), XMM3.as_operand()],
            PendingPrefixes::default(),
            false,
        )
        .unwrap();
        assert!(!buf.data().is_empty());

        let mut buf = CodeBuffer::new(env);
        let error = emit_n(
            &mut buf,
            InstId::Vpaddd as u32,
            &[YMM1.as_operand(), YMM2.as_operand(), YMM3.as_operand()],
            PendingPrefixes::default(),
            false,
        )
        .unwrap_err();
        assert_eq!(error, AsmError::MissingCpuFeature { feature: "AVX2" });
        assert!(buf.data().is_empty());
    }

    #[test]
    fn malformed_raw_operands_are_rejected_without_writes() {
        let mut bad_type = Operand::new();
        bad_type.signature.bits = 7;
        let (bytes, error) = emit(InstId::Mov, &[&bad_type], PendingPrefixes::default());
        assert!(matches!(error, Some(X86Error::InvalidOperand { .. })));
        assert!(bytes.is_empty());

        let bad_reg = *super::super::operands::gpq(16).as_operand();
        let (bytes, error) = emit(
            InstId::Mov,
            &[RAX.as_operand(), &bad_reg],
            PendingPrefixes::default(),
        );
        assert!(matches!(error, Some(X86Error::InvalidRegister { .. })));
        assert!(bytes.is_empty());

        let bad_mem = RAX * 16;
        let (bytes, error) = emit(
            InstId::Mov,
            &[RAX.as_operand(), bad_mem.as_operand()],
            PendingPrefixes::default(),
        );
        assert!(matches!(error, Some(X86Error::InvalidMemoryOperand { .. })));
        assert!(bytes.is_empty());
    }

    #[test]
    fn vsib_and_plain_sib_index_kinds_do_not_cross_match() {
        let vector_index = Mem::from_base_and_index_shift_disp(&RBX, &XMM1, 0, 0, 4, 0.into());
        let (bytes, error) = emit(
            InstId::Mov,
            &[EAX.as_operand(), vector_index.as_operand()],
            PendingPrefixes::default(),
        );
        assert!(error.is_some());
        assert!(bytes.is_empty());

        let scalar_index = Mem::from_base_and_index_shift_disp(&RDX, &RCX, 0, 0, 8, 0.into());
        let (bytes, error) = emit(
            InstId::Vgatherdpd,
            &[YMM1.as_operand(), scalar_index.as_operand()],
            PendingPrefixes::default(),
        );
        assert!(error.is_some());
        assert!(bytes.is_empty());
    }

    #[test]
    fn invalid_prefix_forms_are_rejected_before_writes() {
        let lock = PendingPrefixes {
            options: InstOptions::X86_LOCK,
            ..PendingPrefixes::default()
        };
        let (bytes, error) = emit(InstId::Add, &[RAX.as_operand(), RBX.as_operand()], lock);
        assert!(matches!(error, Some(X86Error::InvalidPrefix { .. })));
        assert!(bytes.is_empty());

        let conflicting_rep = PendingPrefixes {
            options: InstOptions::X86_REP | InstOptions::X86_REPNE,
            ..PendingPrefixes::default()
        };
        let (bytes, error) = emit(InstId::Movs, &[], conflicting_rep);
        assert!(matches!(error, Some(X86Error::InvalidPrefix { .. })));
        assert!(bytes.is_empty());

        let segment_without_memory = PendingPrefixes {
            segment_id: SReg::FS,
            ..PendingPrefixes::default()
        };
        let (bytes, error) = emit(InstId::Ret, &[], segment_without_memory);
        assert!(matches!(error, Some(X86Error::InvalidPrefix { .. })));
        assert!(bytes.is_empty());

        let zero_without_mask = PendingPrefixes {
            options: InstOptions::X86_ZMASK,
            ..PendingPrefixes::default()
        };
        let (bytes, error) = emit(
            InstId::Vaddpd,
            &[ZMM1.as_operand(), ZMM1.as_operand(), ZMM2.as_operand()],
            zero_without_mask,
        );
        assert!(matches!(error, Some(X86Error::InvalidMasking { .. })));
        assert!(bytes.is_empty());

        for options in [
            InstOptions::X86_MOD_MR | InstOptions::X86_MOD_RM,
            InstOptions::X86_VEX | InstOptions::X86_EVEX,
            InstOptions::X86_ER | InstOptions::X86_SAE,
            InstOptions::X86_RD_SAE,
        ] {
            let prefixes = PendingPrefixes {
                options,
                ..PendingPrefixes::default()
            };
            let (bytes, error) = emit(InstId::Ret, &[], prefixes);
            assert!(error.is_some());
            assert!(bytes.is_empty());
        }
    }

    #[test]
    fn valid_lock_form_is_unchanged() {
        let mem = dword_ptr(RBX, 0);
        let prefixes = PendingPrefixes {
            options: InstOptions::X86_LOCK,
            ..PendingPrefixes::default()
        };
        let (bytes, error) = emit(InstId::Add, &[mem.as_operand(), EAX.as_operand()], prefixes);
        assert_eq!(error, None);
        assert_eq!(bytes, [0xF0, 0x01, 0x03]);
    }
}
