use crate::core::{
    arch_traits::*,
    operand::{OperandSignature, RegTraits, RegType},
    types::TypeId,
};

use super::operands::{
    X86Bnd, X86CReg, X86DReg, X86GpbHi, X86GpbLo, X86Gpd, X86Gpq, X86Gpw, X86KReg, X86Mm, X86Rip,
    X86SReg, X86St, X86Tmm, X86Xmm, X86Ymm, X86Zmm,
};

/// Register signatures indexed by [`RegType`] discriminant.
#[rustfmt::skip]
const SIGNATURES: [OperandSignature; 32] = [
    OperandSignature::new(0),                      // None
    OperandSignature::new(0),                      // LabelTag
    OperandSignature::new(0),                      // SymTag
    OperandSignature::new(X86Rip::SIGNATURE),      // PC
    OperandSignature::new(X86GpbLo::SIGNATURE),    // Gp8Lo
    OperandSignature::new(X86GpbHi::SIGNATURE),    // Gp8Hi
    OperandSignature::new(X86Gpw::SIGNATURE),      // Gp16
    OperandSignature::new(X86Gpd::SIGNATURE),      // Gp32
    OperandSignature::new(X86Gpq::SIGNATURE),      // Gp64
    OperandSignature::new(0),                      // Vec8
    OperandSignature::new(0),                      // Vec16
    OperandSignature::new(0),                      // Vec32
    OperandSignature::new(0),                      // Vec64
    OperandSignature::new(X86Xmm::SIGNATURE),      // Vec128
    OperandSignature::new(X86Ymm::SIGNATURE),      // Vec256
    OperandSignature::new(X86Zmm::SIGNATURE),      // Vec512
    OperandSignature::new(0),                      // VecNLen
    OperandSignature::new(X86KReg::SIGNATURE),     // Mask
    OperandSignature::new(X86Mm::SIGNATURE),       // Extra (X86Mm)
    OperandSignature::new(X86SReg::SIGNATURE),     // X86SReg
    OperandSignature::new(X86CReg::SIGNATURE),     // X86CReg
    OperandSignature::new(X86DReg::SIGNATURE),     // X86DReg
    OperandSignature::new(X86St::SIGNATURE),       // X86St
    OperandSignature::new(X86Bnd::SIGNATURE),      // X86Bnd
    OperandSignature::new(X86Tmm::SIGNATURE),      // X86Tmm
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
/// Mirrors the `TYPE_ID` values of the register traits in `super::operands`,
/// except `Mask` which is mapped to [`TypeId::Mask64`].
#[rustfmt::skip]
const REG_TYPE_TO_TYPE_ID: [TypeId; 32] = [
    TypeId::Void,    // None
    TypeId::Void,    // LabelTag
    TypeId::Void,    // SymTag
    TypeId::Void,    // PC (X86Rip)
    TypeId::Int8,    // Gp8Lo
    TypeId::Int8,    // Gp8Hi
    TypeId::Int16,   // Gp16
    TypeId::Int32,   // Gp32
    TypeId::Int64,   // Gp64
    TypeId::Void,    // Vec8
    TypeId::Void,    // Vec16
    TypeId::Void,    // Vec32
    TypeId::Void,    // Vec64
    TypeId::Int32x4, // Vec128
    TypeId::Int32x8, // Vec256
    TypeId::Int32x16,// Vec512
    TypeId::Void,    // VecNLen
    TypeId::Mask64,  // Mask (X86KReg)
    TypeId::Mmx64,   // Extra (X86Mm)
    TypeId::Void,    // X86SReg
    TypeId::Void,    // X86CReg
    TypeId::Void,    // X86DReg
    TypeId::Float80, // X86St
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
/// `type_id as usize - 32` ([`TypeId::IntPtr`] is index 0). 32-bit variant.
#[rustfmt::skip]
const X86_TYPE_ID_TO_REG_TYPE: [RegType; 32] = [
    RegType::Gp32,  // IntPtr
    RegType::Gp32,  // UIntPtr
    RegType::Gp8Lo, // Int8
    RegType::Gp8Lo, // UInt8
    RegType::Gp16,  // Int16
    RegType::Gp16,  // UInt16
    RegType::Gp32,  // Int32
    RegType::Gp32,  // UInt32
    RegType::None,  // Int64 (no 64-bit GP register on X86)
    RegType::None,  // UInt64
    RegType::Vec128,// Float32
    RegType::Vec128,// Float64
    RegType::None,  // Float80
    RegType::Mask,  // Mask8
    RegType::Mask,  // Mask16
    RegType::Mask,  // Mask32
    RegType::Mask,  // Mask64
    RegType::Extra, // Mmx32 (X86Mm)
    RegType::Extra, // Mmx64 (X86Mm)
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

/// 64-bit variant of [`X86_TYPE_ID_TO_REG_TYPE`].
#[rustfmt::skip]
const X64_TYPE_ID_TO_REG_TYPE: [RegType; 32] = [
    RegType::Gp64,  // IntPtr
    RegType::Gp64,  // UIntPtr
    RegType::Gp8Lo, // Int8
    RegType::Gp8Lo, // UInt8
    RegType::Gp16,  // Int16
    RegType::Gp16,  // UInt16
    RegType::Gp32,  // Int32
    RegType::Gp32,  // UInt32
    RegType::Gp64,  // Int64
    RegType::Gp64,  // UInt64
    RegType::Vec128,// Float32
    RegType::Vec128,// Float64
    RegType::None,  // Float80
    RegType::Mask,  // Mask8
    RegType::Mask,  // Mask16
    RegType::Mask,  // Mask32
    RegType::Mask,  // Mask64
    RegType::Extra, // Mmx32 (X86Mm)
    RegType::Extra, // Mmx64 (X86Mm)
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

pub const X86_ARCH_TRAITS: ArchTraits = ArchTraits {
    fp_reg_id: 5, // EBP
    sp_reg_id: 4, // ESP
    ip_reg_id: 0, // EIP
    link_reg_id: 0xff,
    hw_stack_alignment: 1,
    min_stack_offset: 0x7FFFFFFF,
    max_stack_offset: 0x7FFFFFFF,
    regs_signature: SIGNATURES,
    reg_type_to_type_id: REG_TYPE_TO_TYPE_ID,
    type_id_to_reg_type: X86_TYPE_ID_TO_REG_TYPE,
};

pub const X64_ARCH_TRAITS: ArchTraits = ArchTraits {
    fp_reg_id: 5, // RBP
    sp_reg_id: 4, // RSP
    ip_reg_id: 0, // RIP
    link_reg_id: 0xff,
    hw_stack_alignment: 1,
    min_stack_offset: 0x7FFFFFFF,
    max_stack_offset: 0x7FFFFFFF,
    regs_signature: SIGNATURES,
    reg_type_to_type_id: REG_TYPE_TO_TYPE_ID,
    type_id_to_reg_type: X64_TYPE_ID_TO_REG_TYPE,
};
