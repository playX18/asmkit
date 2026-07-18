//! Temporary sanity check for the ArchTraits tables (deleted after verification).
#![cfg(all(feature = "x86", feature = "riscv", feature = "aarch64"))]
use asmkit::core::{
    arch_traits::{Arch, ArchTraits},
    operand::{OperandSignature, RegTraits, RegType},
    types::TypeId,
};

#[test]
fn x64_traits() {
    let t = ArchTraits::by_arch(Arch::X64);
    assert_eq!(t.sp_reg_id(), 4);
    assert_eq!(t.fp_reg_id(), 5);
    assert_eq!(t.link_reg_id(), 0xff);
    assert!(!t.has_link_reg());
    assert_eq!(t.ip_reg_id(), 0);
    assert_eq!(t.hw_stack_alignment(), 1);
    assert_eq!(t.min_stack_offset(), 0x7FFFFFFF);
    assert_eq!(t.max_stack_offset(), 0x7FFFFFFF);
    assert_eq!(
        t.reg_type_to_signature(RegType::PC),
        OperandSignature::new(asmkit::x86::X86Rip::SIGNATURE)
    );
    assert!(t.reg_type_to_signature(RegType::Gp8Lo).is_valid());
    assert!(t.reg_type_to_signature(RegType::Gp64).is_valid());
    assert!(t.reg_type_to_signature(RegType::Vec128).is_valid());
    assert!(t.reg_type_to_signature(RegType::Vec512).is_valid());
    assert!(t.reg_type_to_signature(RegType::Mask).is_valid());
    assert!(t.reg_type_to_signature(RegType::Extra).is_valid());
    assert!(t.reg_type_to_signature(RegType::X86Tmm).is_valid());
    assert!(!t.reg_type_to_signature(RegType::Vec8).is_valid());
    assert!(!t.reg_type_to_signature(RegType::Vec64).is_valid());
    assert_eq!(t.reg_type_to_type_id(RegType::Gp64), TypeId::Int64);
    assert_eq!(t.reg_type_to_type_id(RegType::Vec128), TypeId::Int32x4);
    assert_eq!(t.reg_type_to_type_id(RegType::Mask), TypeId::Mask64);
    assert_eq!(t.reg_type_to_type_id(RegType::X86St), TypeId::Float80);
    assert_eq!(t.reg_type_to_type_id(RegType::Extra), TypeId::Mmx64);
    assert!(t.type_id_to_reg_type[TypeId::Int64 as usize - 32] == RegType::Gp64);
    assert!(t.type_id_to_reg_type[TypeId::Int8 as usize - 32] == RegType::Gp8Lo);
    assert!(t.type_id_to_reg_type[TypeId::Float64 as usize - 32] == RegType::Vec128);
    assert!(t.type_id_to_reg_type[TypeId::Mask64 as usize - 32] == RegType::Mask);
    assert!(t.type_id_to_reg_type[TypeId::Mmx64 as usize - 32] == RegType::Extra);

    let t32 = ArchTraits::by_arch(Arch::X86);
    assert!(t32.type_id_to_reg_type[TypeId::Int64 as usize - 32] == RegType::None);
    assert!(t32.type_id_to_reg_type[TypeId::IntPtr as usize - 32] == RegType::Gp32);
}

#[test]
fn aarch64_traits() {
    let t = ArchTraits::by_arch(Arch::AArch64);
    assert_eq!(t.sp_reg_id(), 31);
    assert_eq!(t.fp_reg_id(), 29);
    assert_eq!(t.link_reg_id(), 30);
    assert!(t.has_link_reg());
    assert_eq!(t.ip_reg_id(), 0xff);
    assert_eq!(t.hw_stack_alignment(), 16);
    assert_eq!(t.min_stack_offset(), 4095);
    assert_eq!(t.max_stack_offset(), 65520);
    assert_eq!(
        t.reg_type_to_signature(RegType::Vec128),
        OperandSignature::new(asmkit::aarch64::A64VecQ::SIGNATURE)
    );
    assert!(t.reg_type_to_signature(RegType::Vec8).is_valid());
    assert!(!t.reg_type_to_signature(RegType::Vec256).is_valid());
    assert!(!t.reg_type_to_signature(RegType::Mask).is_valid());
    assert_eq!(t.reg_type_to_type_id(RegType::Gp64), TypeId::Int64);
    assert_eq!(t.reg_type_to_type_id(RegType::Vec128), TypeId::Int32x4);
    assert!(t.type_id_to_reg_type[TypeId::Int8 as usize - 32] == RegType::Gp32);
    assert!(t.type_id_to_reg_type[TypeId::Int64 as usize - 32] == RegType::Gp64);
    assert!(t.type_id_to_reg_type[TypeId::Float32 as usize - 32] == RegType::Vec32);
    assert!(t.type_id_to_reg_type[TypeId::Float64 as usize - 32] == RegType::Vec64);
}

#[test]
fn riscv_traits() {
    let t = ArchTraits::by_arch(Arch::RISCV64);
    assert_eq!(t.sp_reg_id(), 2);
    assert_eq!(t.fp_reg_id(), 8);
    assert_eq!(t.link_reg_id(), 1);
    assert!(t.has_link_reg());
    assert_eq!(t.ip_reg_id(), 0xff);
    assert_eq!(t.hw_stack_alignment(), 16);
    assert_eq!(t.min_stack_offset(), 0xFFFFF800);
    assert_eq!(t.max_stack_offset(), 2047);
    assert_eq!(
        t.reg_type_to_signature(RegType::Gp64),
        OperandSignature::new(asmkit::riscv::RISCV64Gp::SIGNATURE)
    );
    assert_eq!(
        t.reg_type_to_signature(RegType::Vec64),
        OperandSignature::new(asmkit::riscv::RISCVFp::SIGNATURE)
    );
    assert_eq!(
        t.reg_type_to_signature(RegType::VecNLen),
        OperandSignature::new(asmkit::riscv::RISCVVec::SIGNATURE)
    );
    assert!(t.reg_type_to_signature(RegType::PC).is_valid());
    assert_eq!(t.reg_type_to_type_id(RegType::Gp64), TypeId::Int64);
    assert_eq!(t.reg_type_to_type_id(RegType::Vec64), TypeId::Float64);
    assert!(t.type_id_to_reg_type[TypeId::Int64 as usize - 32] == RegType::Gp64);
    assert!(t.type_id_to_reg_type[TypeId::Float64 as usize - 32] == RegType::Vec64);

    let t32 = ArchTraits::by_arch(Arch::RISCV32);
    assert_eq!(t32.sp_reg_id(), 2);
    assert!(t32.reg_type_to_signature(RegType::Gp32).is_valid());
    assert!(!t32.reg_type_to_signature(RegType::Gp64).is_valid());
    assert!(t32.type_id_to_reg_type[TypeId::Int32 as usize - 32] == RegType::Gp32);
    assert!(t32.type_id_to_reg_type[TypeId::Int64 as usize - 32] == RegType::None);
}
