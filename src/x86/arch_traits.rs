use crate::core::{
    arch_traits::*,
    operand::{OperandSignature, RegTraits, RegType},
    types::TypeId,
};

use super::operands::{
    X86Bnd, X86CReg, X86DReg, X86GpbHi, X86GpbLo, X86Gpd, X86Gpq, X86Gpw, X86KReg, X86Mm, X86Rip,
    X86SReg, X86St, X86Tmm, X86Xmm, X86Ymm, X86Zmm,
};

const SIGNATURES: [OperandSignature; 32] = [
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(X86Rip::SIGNATURE),
    OperandSignature::new(X86GpbLo::SIGNATURE),
    OperandSignature::new(X86GpbHi::SIGNATURE),
    OperandSignature::new(X86Gpw::SIGNATURE),
    OperandSignature::new(X86Gpd::SIGNATURE),
    OperandSignature::new(X86Gpq::SIGNATURE),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(X86Xmm::SIGNATURE),
    OperandSignature::new(X86Ymm::SIGNATURE),
    OperandSignature::new(X86Zmm::SIGNATURE),
    OperandSignature::new(0),
    OperandSignature::new(X86KReg::SIGNATURE),
    OperandSignature::new(X86Mm::SIGNATURE),
    OperandSignature::new(X86SReg::SIGNATURE),
    OperandSignature::new(X86CReg::SIGNATURE),
    OperandSignature::new(X86DReg::SIGNATURE),
    OperandSignature::new(X86St::SIGNATURE),
    OperandSignature::new(X86Bnd::SIGNATURE),
    OperandSignature::new(X86Tmm::SIGNATURE),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
    OperandSignature::new(0),
];

pub const X86_ARCH_TRAITS: ArchTraits = ArchTraits {
    fp_reg_id: 0xff,
    sp_reg_id: 0xff,
    ip_reg_id: 0xff,
    link_reg_id: 0xff,
    hw_stack_alignment: 1,
    min_stack_offset: 0x7FFFFFFF,
    max_stack_offset: 0x7FFFFFFF,
    regs_signature: SIGNATURES,
    reg_type_to_type_id: [TypeId::Void; 32],
    type_id_to_reg_type: [RegType::Gp32; 32],
};

pub const X64_ARCH_TRAITS: ArchTraits = ArchTraits {
    fp_reg_id: 0xff,
    sp_reg_id: 0xff,
    ip_reg_id: 0xff,
    link_reg_id: 0xff,
    hw_stack_alignment: 1,
    min_stack_offset: 0x7FFFFFFF,
    max_stack_offset: 0x7FFFFFFF,
    regs_signature: SIGNATURES,
    reg_type_to_type_id: [TypeId::Void; 32],
    type_id_to_reg_type: [RegType::Gp32; 32],
};
