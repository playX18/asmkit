//! RISC-V architecture traits (both RV32 and RV64 variants).
use crate::core::{
    arch_traits::*,
    operand::{OperandSignature, RegTraits, RegType},
    types::TypeId,
};

use super::operands::{Gp, RISCV32Gp, RISCV64Gp, RISCVFp, RISCVPC, RISCVVec};

/// Register signatures indexed by [`RegType`] discriminant. RV64 variant.
#[rustfmt::skip]
const RISCV64_SIGNATURES: [OperandSignature; 32] = [
    OperandSignature::new(0),                      // None
    OperandSignature::new(0),                      // LabelTag
    OperandSignature::new(0),                      // SymTag
    OperandSignature::new(RISCVPC::SIGNATURE),     // PC
    OperandSignature::new(0),                      // Gp8Lo
    OperandSignature::new(0),                      // Gp8Hi
    OperandSignature::new(0),                      // Gp16
    OperandSignature::new(0),                      // Gp32
    OperandSignature::new(RISCV64Gp::SIGNATURE),   // Gp64
    OperandSignature::new(0),                      // Vec8
    OperandSignature::new(0),                      // Vec16
    OperandSignature::new(0),                      // Vec32
    OperandSignature::new(RISCVFp::SIGNATURE),     // Vec64 (RISCVFp)
    OperandSignature::new(0),                      // Vec128
    OperandSignature::new(0),                      // Vec256
    OperandSignature::new(0),                      // Vec512
    OperandSignature::new(RISCVVec::SIGNATURE),    // VecNLen (RISCVVec)
    OperandSignature::new(0),                      // Mask
    OperandSignature::new(0),                      // Extra
    OperandSignature::new(0),                      // X86SReg
    OperandSignature::new(0),                      // X86CReg
    OperandSignature::new(0),                      // X86DReg
    OperandSignature::new(0),                      // X86St
    OperandSignature::new(0),                      // X86Bnd
    OperandSignature::new(0),                      // X86Tmm
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
];

/// Register signatures indexed by [`RegType`] discriminant. RV32 variant.
#[rustfmt::skip]
const RISCV32_SIGNATURES: [OperandSignature; 32] = [
    OperandSignature::new(0),                      // None
    OperandSignature::new(0),                      // LabelTag
    OperandSignature::new(0),                      // SymTag
    OperandSignature::new(RISCVPC::SIGNATURE),     // PC
    OperandSignature::new(0),                      // Gp8Lo
    OperandSignature::new(0),                      // Gp8Hi
    OperandSignature::new(0),                      // Gp16
    OperandSignature::new(RISCV32Gp::SIGNATURE),   // Gp32
    OperandSignature::new(0),                      // Gp64
    OperandSignature::new(0),                      // Vec8
    OperandSignature::new(0),                      // Vec16
    OperandSignature::new(0),                      // Vec32
    OperandSignature::new(RISCVFp::SIGNATURE),     // Vec64 (RISCVFp)
    OperandSignature::new(0),                      // Vec128
    OperandSignature::new(0),                      // Vec256
    OperandSignature::new(0),                      // Vec512
    OperandSignature::new(RISCVVec::SIGNATURE),    // VecNLen (RISCVVec)
    OperandSignature::new(0),                      // Mask
    OperandSignature::new(0),                      // Extra
    OperandSignature::new(0),                      // X86SReg
    OperandSignature::new(0),                      // X86CReg
    OperandSignature::new(0),                      // X86DReg
    OperandSignature::new(0),                      // X86St
    OperandSignature::new(0),                      // X86Bnd
    OperandSignature::new(0),                      // X86Tmm
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
];

/// TypeIds of register types, indexed by [`RegType`] discriminant. RV64 variant.
///
/// Mirrors the `TYPE_ID` values of the register traits in `super::operands`.
#[rustfmt::skip]
const RISCV64_REG_TYPE_TO_TYPE_ID: [TypeId; 32] = [
    TypeId::Void,    // None
    TypeId::Void,    // LabelTag
    TypeId::Void,    // SymTag
    TypeId::Int64,   // PC
    TypeId::Void,    // Gp8Lo
    TypeId::Void,    // Gp8Hi
    TypeId::Void,    // Gp16
    TypeId::Void,    // Gp32
    TypeId::Int64,   // Gp64
    TypeId::Void,    // Vec8
    TypeId::Void,    // Vec16
    TypeId::Void,    // Vec32
    TypeId::Float64, // Vec64 (RISCVFp)
    TypeId::Void,    // Vec128
    TypeId::Void,    // Vec256
    TypeId::Void,    // Vec512
    TypeId::Int32x4, // VecNLen (RISCVVec)
    TypeId::Void,    // Mask
    TypeId::Void,    // Extra
    TypeId::Void,    // X86SReg
    TypeId::Void,    // X86CReg
    TypeId::Void,    // X86DReg
    TypeId::Void,    // X86St
    TypeId::Void,    // X86Bnd
    TypeId::Void,    // X86Tmm
    TypeId::Void,
    TypeId::Void,
    TypeId::Void,
    TypeId::Void,
    TypeId::Void,
    TypeId::Void,
    TypeId::Void,
];

/// TypeIds of register types, indexed by [`RegType`] discriminant. RV32 variant.
#[rustfmt::skip]
const RISCV32_REG_TYPE_TO_TYPE_ID: [TypeId; 32] = [
    TypeId::Void,    // None
    TypeId::Void,    // LabelTag
    TypeId::Void,    // SymTag
    TypeId::Int64,   // PC
    TypeId::Void,    // Gp8Lo
    TypeId::Void,    // Gp8Hi
    TypeId::Void,    // Gp16
    TypeId::Int32,   // Gp32
    TypeId::Void,    // Gp64
    TypeId::Void,    // Vec8
    TypeId::Void,    // Vec16
    TypeId::Void,    // Vec32
    TypeId::Float64, // Vec64 (RISCVFp)
    TypeId::Void,    // Vec128
    TypeId::Void,    // Vec256
    TypeId::Void,    // Vec512
    TypeId::Int32x4, // VecNLen (RISCVVec)
    TypeId::Void,    // Mask
    TypeId::Void,    // Extra
    TypeId::Void,    // X86SReg
    TypeId::Void,    // X86CReg
    TypeId::Void,    // X86DReg
    TypeId::Void,    // X86St
    TypeId::Void,    // X86Bnd
    TypeId::Void,    // X86Tmm
    TypeId::Void,
    TypeId::Void,
    TypeId::Void,
    TypeId::Void,
    TypeId::Void,
    TypeId::Void,
    TypeId::Void,
];

/// Register type used to hold a value of a given type, indexed by
/// `type_id as usize - 32` ([`TypeId::IntPtr`] is index 0). RV64 variant.
#[rustfmt::skip]
const RISCV64_TYPE_ID_TO_REG_TYPE: [RegType; 32] = [
    RegType::Gp64,  // IntPtr
    RegType::Gp64,  // UIntPtr
    RegType::Gp64,  // Int8
    RegType::Gp64,  // UInt8
    RegType::Gp64,  // Int16
    RegType::Gp64,  // UInt16
    RegType::Gp64,  // Int32
    RegType::Gp64,  // UInt32
    RegType::Gp64,  // Int64
    RegType::Gp64,  // UInt64
    RegType::Vec64, // Float32
    RegType::Vec64, // Float64
    RegType::None,  // Float80
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
];

/// RV32 variant of [`RISCV64_TYPE_ID_TO_REG_TYPE`].
#[rustfmt::skip]
const RISCV32_TYPE_ID_TO_REG_TYPE: [RegType; 32] = [
    RegType::Gp32,  // IntPtr
    RegType::Gp32,  // UIntPtr
    RegType::Gp32,  // Int8
    RegType::Gp32,  // UInt8
    RegType::Gp32,  // Int16
    RegType::Gp32,  // UInt16
    RegType::Gp32,  // Int32
    RegType::Gp32,  // UInt32
    RegType::None,  // Int64 (no 64-bit GP register on RV32)
    RegType::None,  // UInt64
    RegType::Vec64, // Float32
    RegType::Vec64, // Float64
    RegType::None,  // Float80
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
    RegType::None,
];

/// Addressable stack offset window of a RISC-V load/store (12-bit signed
/// immediate): -2048..=2047.
const MIN_STACK_OFFSET: u32 = 0xFFFFF800;
const MAX_STACK_OFFSET: u32 = 2047;

pub const RISCV64_ARCH_TRAITS: ArchTraits = ArchTraits {
    sp_reg_id: Gp::X2 as u8,   // sp
    fp_reg_id: Gp::X8 as u8,   // s0/fp
    link_reg_id: Gp::X1 as u8, // ra
    ip_reg_id: 0xff,
    hw_stack_alignment: 16,
    min_stack_offset: MIN_STACK_OFFSET,
    max_stack_offset: MAX_STACK_OFFSET,
    regs_signature: RISCV64_SIGNATURES,
    reg_type_to_type_id: RISCV64_REG_TYPE_TO_TYPE_ID,
    type_id_to_reg_type: RISCV64_TYPE_ID_TO_REG_TYPE,
};

/// RV32 has its own table (rather than sharing the RV64 one) so that GP
/// registers are described as 32-bit (`RegType::Gp32`/`TypeId::Int32`).
pub const RISCV32_ARCH_TRAITS: ArchTraits = ArchTraits {
    sp_reg_id: Gp::X2 as u8,   // sp
    fp_reg_id: Gp::X8 as u8,   // s0/fp
    link_reg_id: Gp::X1 as u8, // ra
    ip_reg_id: 0xff,
    hw_stack_alignment: 16,
    min_stack_offset: MIN_STACK_OFFSET,
    max_stack_offset: MAX_STACK_OFFSET,
    regs_signature: RISCV32_SIGNATURES,
    reg_type_to_type_id: RISCV32_REG_TYPE_TO_TYPE_ID,
    type_id_to_reg_type: RISCV32_TYPE_ID_TO_REG_TYPE,
};
