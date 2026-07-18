//! AArch64 architecture traits.
use crate::core::{
    arch_traits::*,
    operand::{OperandSignature, RegTraits, RegType},
    types::TypeId,
};

use super::operands::{A64Gpw, A64Gpx, A64VecB, A64VecD, A64VecH, A64VecQ, A64VecS, Gp};

/// Register signatures indexed by [`RegType`] discriminant.
#[rustfmt::skip]
const SIGNATURES: [OperandSignature; 32] = [
    OperandSignature::new(0),                    // None
    OperandSignature::new(0),                    // LabelTag
    OperandSignature::new(0),                    // SymTag
    OperandSignature::new(0),                    // PC
    OperandSignature::new(0),                    // Gp8Lo
    OperandSignature::new(0),                    // Gp8Hi
    OperandSignature::new(0),                    // Gp16
    OperandSignature::new(A64Gpw::SIGNATURE),    // Gp32
    OperandSignature::new(A64Gpx::SIGNATURE),    // Gp64
    OperandSignature::new(A64VecB::SIGNATURE),   // Vec8
    OperandSignature::new(A64VecH::SIGNATURE),   // Vec16
    OperandSignature::new(A64VecS::SIGNATURE),   // Vec32
    OperandSignature::new(A64VecD::SIGNATURE),   // Vec64
    OperandSignature::new(A64VecQ::SIGNATURE),   // Vec128
    OperandSignature::new(0),                    // Vec256
    OperandSignature::new(0),                    // Vec512
    OperandSignature::new(0),                    // VecNLen
    OperandSignature::new(0),                    // Mask
    OperandSignature::new(0),                    // Extra
    OperandSignature::new(0),                    // X86SReg
    OperandSignature::new(0),                    // X86CReg
    OperandSignature::new(0),                    // X86DReg
    OperandSignature::new(0),                    // X86St
    OperandSignature::new(0),                    // X86Bnd
    OperandSignature::new(0),                    // X86Tmm
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
];

/// TypeIds of register types, indexed by [`RegType`] discriminant.
///
/// Mirrors the `TYPE_ID` values of the register traits in `super::operands`.
#[rustfmt::skip]
const REG_TYPE_TO_TYPE_ID: [TypeId; 32] = [
    TypeId::Void,    // None
    TypeId::Void,    // LabelTag
    TypeId::Void,    // SymTag
    TypeId::Void,    // PC
    TypeId::Void,    // Gp8Lo
    TypeId::Void,    // Gp8Hi
    TypeId::Void,    // Gp16
    TypeId::Int32,   // Gp32 (W)
    TypeId::Int64,   // Gp64 (X)
    TypeId::Int8,    // Vec8 (B)
    TypeId::Int16,   // Vec16 (H)
    TypeId::Int32,   // Vec32 (S)
    TypeId::Int64,   // Vec64 (D)
    TypeId::Int32x4, // Vec128 (Q/V)
    TypeId::Void,    // Vec256
    TypeId::Void,    // Vec512
    TypeId::Void,    // VecNLen
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
/// `type_id as usize - 32` ([`TypeId::IntPtr`] is index 0).
#[rustfmt::skip]
const TYPE_ID_TO_REG_TYPE: [RegType; 32] = [
    RegType::Gp64, // IntPtr
    RegType::Gp64, // UIntPtr
    RegType::Gp32, // Int8
    RegType::Gp32, // UInt8
    RegType::Gp32, // Int16
    RegType::Gp32, // UInt16
    RegType::Gp32, // Int32
    RegType::Gp32, // UInt32
    RegType::Gp64, // Int64
    RegType::Gp64, // UInt64
    RegType::Vec32,// Float32
    RegType::Vec64,// Float64
    RegType::None, // Float80
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

pub const AARCH64_ARCH_TRAITS: ArchTraits = ArchTraits {
    sp_reg_id: Gp::ID_SP as u8,   // x31 (SP)
    fp_reg_id: Gp::ID_FP as u8,   // x29 (FP)
    link_reg_id: Gp::ID_LR as u8, // x30 (LR)
    ip_reg_id: 0xff,
    hw_stack_alignment: 16,
    // Byte addressing is the worst case, vec.q addressing the best
    // (mirrors asmjit's a64_arch_traits).
    min_stack_offset: 4095,
    max_stack_offset: 65520,
    regs_signature: SIGNATURES,
    reg_type_to_type_id: REG_TYPE_TO_TYPE_ID,
    type_id_to_reg_type: TYPE_ID_TO_REG_TYPE,
};
