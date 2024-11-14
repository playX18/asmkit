/* Copyright (c) 2008-2024 The AsmJit Authors

    This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

    Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

    The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
    Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
    This notice may not be removed or altered from any source distribution.

 */
//! X86 operands definition. 
use core::ops::{Add, Deref, Mul};
use derive_more::derive::{Deref, DerefMut};
use crate::{
    core::{
        arch_traits::{Arch, ArchTraits},
        operand::*,
        types::TypeId,
    },
    define_abstract_reg, define_final_reg, define_operand_cast, define_reg_traits,
};

define_reg_traits!(X86Rip, RegGroup::X86Rip, 0, TypeId::Void);
define_reg_traits!(X86GpbLo, RegGroup::Gp, 1, TypeId::Int8);
define_reg_traits!(X86GpbHi, RegGroup::Gp, 1, TypeId::Int8);
define_reg_traits!(X86Gpw, RegGroup::Gp, 2, TypeId::Int16);
define_reg_traits!(X86Gpd, RegGroup::Gp, 4, TypeId::Int32);
define_reg_traits!(X86Gpq, RegGroup::Gp, 8, TypeId::Int64);
define_reg_traits!(X86Xmm, RegGroup::Vec, 16, TypeId::Int32x4);
define_reg_traits!(X86Ymm, RegGroup::Vec, 32, TypeId::Int32x8);
define_reg_traits!(X86Zmm, RegGroup::Vec, 64, TypeId::Int32x16);
define_reg_traits!(X86KReg, RegGroup::X86K, 0, TypeId::Void);
define_reg_traits!(X86Mm, RegGroup::X86MM, 8, TypeId::Mmx64);
define_reg_traits!(X86SReg, RegGroup::X86SReg, 2, TypeId::Void);
define_reg_traits!(X86CReg, RegGroup::X86CReg, 0, TypeId::Void);
define_reg_traits!(X86DReg, RegGroup::X86DReg, 0, TypeId::Void);
define_reg_traits!(X86St, RegGroup::X86St, 10, TypeId::Float80);
define_reg_traits!(X86Bnd, RegGroup::X86Bnd, 16, TypeId::Void);
define_reg_traits!(X86Tmm, RegGroup::X86Tmm, 0, TypeId::Void);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Reg(pub BaseReg);

define_abstract_reg!(Reg, BaseReg);

impl Reg {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }

    pub fn is_gpb(&self) -> bool {
        self.size() == 1
    }

    pub fn is_gpb_lo(&self) -> bool {
        self.has_base_signature(X86GpbLo::SIGNATURE)
    }

    pub fn is_gpb_hi(&self) -> bool {
        self.has_base_signature(X86GpbHi::SIGNATURE)
    }

    pub fn is_gpw(&self) -> bool {
        self.has_base_signature(X86Gpw::SIGNATURE)
    }

    pub fn is_gpd(&self) -> bool {
        self.has_base_signature(X86Gpd::SIGNATURE)
    }

    pub fn is_gpq(&self) -> bool {
        self.has_base_signature(X86Gpq::SIGNATURE)
    }

    pub fn is_gp32(&self) -> bool {
        self.has_base_signature(X86Gpd::SIGNATURE)
    }

    pub fn is_gp64(&self) -> bool {
        self.has_base_signature(X86Gpq::SIGNATURE)
    }

    pub fn is_xmm(&self) -> bool {
        self.has_base_signature(X86Xmm::SIGNATURE)
    }

    pub fn is_ymm(&self) -> bool {
        self.has_base_signature(X86Ymm::SIGNATURE)
    }

    pub fn is_zmm(&self) -> bool {
        self.has_base_signature(X86Zmm::SIGNATURE)
    }

    pub fn is_vec128(&self) -> bool {
        self.has_base_signature(X86Xmm::SIGNATURE)
    }

    pub fn is_vec256(&self) -> bool {
        self.has_base_signature(X86Ymm::SIGNATURE)
    }

    pub fn is_vec512(&self) -> bool {
        self.has_base_signature(X86Zmm::SIGNATURE)
    }

    pub fn is_mm(&self) -> bool {
        self.has_base_signature(X86Mm::SIGNATURE)
    }

    pub fn is_k_reg(&self) -> bool {
        self.has_base_signature(X86KReg::SIGNATURE)
    }

    pub fn is_s_reg(&self) -> bool {
        self.has_base_signature(X86SReg::SIGNATURE)
    }

    pub fn is_c_reg(&self) -> bool {
        self.has_base_signature(X86CReg::SIGNATURE)
    }

    pub fn is_d_reg(&self) -> bool {
        self.has_base_signature(X86DReg::SIGNATURE)
    }

    pub fn is_st(&self) -> bool {
        self.has_base_signature(X86St::SIGNATURE)
    }

    pub fn is_bnd(&self) -> bool {
        self.has_base_signature(X86Bnd::SIGNATURE)
    }

    pub fn is_tmm(&self) -> bool {
        self.has_base_signature(X86Tmm::SIGNATURE)
    }

    pub fn is_rip(&self) -> bool {
        self.has_base_signature(X86Rip::SIGNATURE)
    }

    pub fn set_reg_t<T: RegTraits>(&mut self, rid: u32) {
        self.set_signature(T::SIGNATURE.into());
        self.set_id(rid);
    }

    pub fn set_type_and_id(&mut self, typ: RegType, id: u32) {
        self.set_signature(Self::signature_of(typ));
        self.set_id(id);
    }

    pub fn group_of(typ: RegType) -> RegGroup {
        ArchTraits::by_arch(Arch::X86).reg_type_to_group(typ)
    }

    pub const fn type_id_of(typ: RegType) -> TypeId {
        ArchTraits::by_arch(Arch::X86).reg_type_to_type_id(typ)
    }

    pub const fn group_of_t<T: RegTraits>() -> RegGroup {
        T::GROUP
    }

    pub const fn type_id_of_t<T: RegTraits>() -> TypeId {
        T::TYPE_ID
    }

    pub const fn signature_of_vec_by_type(typ: TypeId) -> OperandSignature {
        let typ = typ as u32;

        if typ <= TypeId::VEC128_END {
            OperandSignature::new(X86Xmm::SIGNATURE)
        } else if typ <= TypeId::VEC256_END {
            OperandSignature::new(X86Ymm::SIGNATURE)
        } else {
            OperandSignature::new(X86Zmm::SIGNATURE)
        }
    }

    pub const fn signature_of_vec_by_size(size: u32) -> OperandSignature {
        if size <= 16 {
            OperandSignature::new(X86Xmm::SIGNATURE)
        } else if size <= 32 {
            OperandSignature::new(X86Ymm::SIGNATURE)
        } else {
            OperandSignature::new(X86Zmm::SIGNATURE)
        }
    }

    pub fn operand_is_gpb(op: &Operand) -> bool {
        op.signature.subset(
            OperandSignature::OP_TYPE_MASK
                | OperandSignature::REG_GROUP_MASK
                | OperandSignature::SIZE_MASK,
        ) == (OperandSignature::from_op_type(OperandType::Reg)
            | OperandSignature::from_reg_group(RegGroup::Gp)
            | OperandSignature::from_size(1))
    }

    pub fn operand_is_gpb_lo(op: &Operand) -> bool {
        op.as_::<Self>().is_gpb_lo()
    }
    pub fn operand_is_gpb_hi(op: &Operand) -> bool {
        op.as_::<Self>().is_gpb_hi()
    }
    pub fn operand_is_gpw(op: &Operand) -> bool {
        op.as_::<Self>().is_gpw()
    }
    pub fn operand_is_gpd(op: &Operand) -> bool {
        op.as_::<Self>().is_gpd()
    }
    pub fn operand_is_gpq(op: &Operand) -> bool {
        op.as_::<Self>().is_gpq()
    }
    pub fn operand_is_xmm(op: &Operand) -> bool {
        op.as_::<Self>().is_xmm()
    }
    pub fn operand_is_ymm(op: &Operand) -> bool {
        op.as_::<Self>().is_ymm()
    }
    pub fn operand_is_zmm(op: &Operand) -> bool {
        op.as_::<Self>().is_zmm()
    }
    pub fn operand_is_mm(op: &Operand) -> bool {
        op.as_::<Self>().is_mm()
    }
    pub fn operand_is_k_reg(op: &Operand) -> bool {
        op.as_::<Self>().is_k_reg()
    }
    pub fn operand_is_s_reg(op: &Operand) -> bool {
        op.as_::<Self>().is_s_reg()
    }
    pub fn operand_is_c_reg(op: &Operand) -> bool {
        op.as_::<Self>().is_c_reg()
    }
    pub fn operand_is_d_reg(op: &Operand) -> bool {
        op.as_::<Self>().is_d_reg()
    }
    pub fn operand_is_st(op: &Operand) -> bool {
        op.as_::<Self>().is_st()
    }
    pub fn operand_is_bnd(op: &Operand) -> bool {
        op.as_::<Self>().is_bnd()
    }
    pub fn operand_is_tmm(op: &Operand) -> bool {
        op.as_::<Self>().is_tmm()
    }
    pub fn operand_is_rip(op: &Operand) -> bool {
        op.as_::<Self>().is_rip()
    }

    pub fn operand_is_gpb_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_gpb(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_gpb_lo_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_gpb_lo(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_gpb_hi_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_gpb_hi(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_gpw_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_gpw(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_gpd_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_gpd(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_gpq_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_gpq(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_xmm_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_xmm(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_ymm_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_ymm(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_zmm_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_zmm(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_mm_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_mm(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_k_reg_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_k_reg(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_s_reg_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_s_reg(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_c_reg_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_c_reg(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_d_reg_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_d_reg(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_st_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_st(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_bnd_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_bnd(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_tmm_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_tmm(op) as u32 & (op.id() == rid) as u32) != 0
    }
    pub fn operand_is_rip_with_id(op: &Operand, rid: u32) -> bool {
        (Self::operand_is_rip(op) as u32 & (op.id() == rid) as u32) != 0
    }

    pub const SIGNATURE: u32 = BaseReg::SIGNATURE;
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Gp(pub Reg);

define_abstract_reg!(Gp, Reg);

impl Gp {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }

    pub const AX: u32 = 0;
    pub const CX: u32 = 1;
    pub const DX: u32 = 2;
    pub const BX: u32 = 3;
    pub const SP: u32 = 4;
    pub const BP: u32 = 5;
    pub const SI: u32 = 6;
    pub const DI: u32 = 7;
    pub const R8: u32 = 8;
    pub const R9: u32 = 9;
    pub const R10: u32 = 10;
    pub const R11: u32 = 11;
    pub const R12: u32 = 12;
    pub const R13: u32 = 13;
    pub const R14: u32 = 14;
    pub const R15: u32 = 15;

    pub const SIGNATURE: u32 = Reg::SIGNATURE;
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Vec(pub Reg);

define_abstract_reg!(Vec, Reg);

impl Vec {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }

    pub const SIGNATURE: u32 = Reg::SIGNATURE;
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SReg(pub Reg);

impl SReg {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }

    pub const ES: u32 = 1;
    pub const CS: u32 = 2;
    pub const SS: u32 = 3;
    pub const DS: u32 = 4;
    pub const FS: u32 = 5;
    pub const GS: u32 = 6;
}

define_final_reg!(SReg, Reg, X86SReg);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Gpb(pub Gp);

define_abstract_reg!(Gpb, Gp);

impl Gpb {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }

    pub const SIGNATURE: u32 = Gp::SIGNATURE;
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct GpbLo(pub Gpb);

impl GpbLo {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(GpbLo, Gpb, X86GpbLo);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct GpbHi(pub Gpb);

impl GpbHi {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(GpbHi, Gpb, X86GpbHi);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Gpw(pub Gp);

impl Gpw {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(Gpw, Gp, X86Gpw);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Gpd(pub Gp);

impl Gpd {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(Gpd, Gp, X86Gpd);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Gpq(pub Gp);

impl Gpq {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(Gpq, Gp, X86Gpq);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Xmm(pub Vec);

impl Xmm {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }

    pub fn half(&self) -> Xmm {
        Xmm::from_id(self.id())
    }
}

define_final_reg!(Xmm, Vec, X86Xmm);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Ymm(pub Vec);

impl Ymm {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }

    pub fn half(&self) -> Xmm {
        Xmm::from_id(self.id())
    }
}

define_final_reg!(Ymm, Vec, X86Ymm);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Zmm(pub Vec);

impl Zmm {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }

    pub fn half(&self) -> Ymm {
        Ymm::from_id(self.id())
    }
}

define_final_reg!(Zmm, Vec, X86Zmm);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mm(pub Reg);

impl Mm {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(Mm, Reg, X86Mm);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct KReg(pub Reg);

impl KReg {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(KReg, Reg, X86KReg);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CReg(pub Reg);

impl CReg {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(CReg, Reg, X86CReg);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DReg(pub Reg);

impl DReg {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(DReg, Reg, X86DReg);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct St(pub Reg);

impl St {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(St, Reg, X86St);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Bnd(pub Reg);

impl Bnd {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(Bnd, Reg, X86Bnd);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Tmm(pub Reg);

impl Tmm {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(Tmm, Reg, X86Tmm);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Rip(pub Reg);

impl Rip {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::X86).reg_type_to_signature(typ)
    }
}

define_final_reg!(Rip, Reg, X86Rip);

impl Gp {
    pub fn r8(&self) -> GpbLo {
        GpbLo::from_id(self.id())
    }

    pub fn r8lo(&self) -> GpbLo {
        GpbLo::from_id(self.id())
    }

    pub fn r8hi(&self) -> GpbHi {
        GpbHi::from_id(self.id())
    }

    pub fn r16(&self) -> Gpw {
        Gpw::from_id(self.id())
    }

    pub fn r32(&self) -> Gpd {
        Gpd::from_id(self.id())
    }

    pub fn r64(&self) -> Gpq {
        Gpq::from_id(self.id())
    }
}
impl Vec {
    pub fn xmm(&self) -> Xmm {
        Xmm::from_id(self.id())
    }

    pub fn ymm(&self) -> Ymm {
        Ymm::from_id(self.id())
    }

    pub fn zmm(&self) -> Zmm {
        Zmm::from_id(self.id())
    }

    pub fn v128(&self) -> Xmm {
        Xmm::from_id(self.id())
    }

    pub fn v256(&self) -> Ymm {
        Ymm::from_id(self.id())
    }

    pub fn v512(&self) -> Zmm {
        Zmm::from_id(self.id())
    }
}

pub mod regs {
    use super::*;
    pub const fn gpb(id: u32) -> GpbLo {
        GpbLo::from_id(id)
    }

    pub const fn gpb_lo(id: u32) -> GpbLo {
        GpbLo::from_id(id)
    }

    pub const fn gpb_hi(id: u32) -> GpbHi {
        GpbHi::from_id(id)
    }

    pub const fn gpw(id: u32) -> Gpw {
        Gpw::from_id(id)
    }

    pub const fn gpd(id: u32) -> Gpd {
        Gpd::from_id(id)
    }

    pub const fn gpq(id: u32) -> Gpq {
        Gpq::from_id(id)
    }

    pub const fn xmm(id: u32) -> Xmm {
        Xmm::from_id(id)
    }

    pub const fn ymm(id: u32) -> Ymm {
        Ymm::from_id(id)
    }

    pub const fn zmm(id: u32) -> Zmm {
        Zmm::from_id(id)
    }

    pub const fn mm(id: u32) -> Mm {
        Mm::from_id(id)
    }

    pub const fn k(id: u32) -> KReg {
        KReg::from_id(id)
    }

    pub const fn cr(id: u32) -> CReg {
        CReg::from_id(id)
    }

    pub const fn dr(id: u32) -> DReg {
        DReg::from_id(id)
    }

    pub const fn st(id: u32) -> St {
        St::from_id(id)
    }

    pub const fn bnd(id: u32) -> Bnd {
        Bnd::from_id(id)
    }

    pub const fn tmm(id: u32) -> Tmm {
        Tmm::from_id(id)
    }

    pub const fn rip() -> Rip {
        Rip::from_id(0x120)
    }

    pub const fn sreg(id: u32) -> SReg {
        SReg::from_id(id)
    }

    pub const AL: GpbLo = gpb_lo(Gp::AX);
    pub const CL: GpbLo = gpb_lo(Gp::CX);
    pub const DL: GpbLo = gpb_lo(Gp::DX);
    pub const BL: GpbLo = gpb_lo(Gp::BX);
    pub const SPL: GpbLo = gpb_lo(Gp::SP);
    pub const BPL: GpbLo = gpb_lo(Gp::BP);
    pub const SIL: GpbLo = gpb_lo(Gp::SI);
    pub const DIL: GpbLo = gpb_lo(Gp::DI);
    pub const R8B: GpbLo = gpb_lo(Gp::R8);
    pub const R9B: GpbLo = gpb_lo(Gp::R9);
    pub const R10B: GpbLo = gpb_lo(Gp::R10);
    pub const R11B: GpbLo = gpb_lo(Gp::R11);
    pub const R12B: GpbLo = gpb_lo(Gp::R12);
    pub const R13B: GpbLo = gpb_lo(Gp::R13);
    pub const R14B: GpbLo = gpb_lo(Gp::R14);
    pub const R15B: GpbLo = gpb_lo(Gp::R15);

    pub const AH: GpbHi = gpb_hi(Gp::AX);
    pub const CH: GpbHi = gpb_hi(Gp::CX);
    pub const DH: GpbHi = gpb_hi(Gp::DX);
    pub const BH: GpbHi = gpb_hi(Gp::BX);

    pub const AX: Gpw = gpw(Gp::AX);
    pub const CX: Gpw = gpw(Gp::CX);
    pub const DX: Gpw = gpw(Gp::DX);
    pub const BX: Gpw = gpw(Gp::BX);
    pub const SP: Gpw = gpw(Gp::SP);
    pub const BP: Gpw = gpw(Gp::BP);
    pub const SI: Gpw = gpw(Gp::SI);
    pub const DI: Gpw = gpw(Gp::DI);
    pub const R8W: Gpw = gpw(Gp::R8);
    pub const R9W: Gpw = gpw(Gp::R9);
    pub const R10W: Gpw = gpw(Gp::R10);
    pub const R11W: Gpw = gpw(Gp::R11);
    pub const R12W: Gpw = gpw(Gp::R12);
    pub const R13W: Gpw = gpw(Gp::R13);
    pub const R14W: Gpw = gpw(Gp::R14);
    pub const R15W: Gpw = gpw(Gp::R15);

    pub const EAX: Gpd = gpd(Gp::AX);
    pub const ECX: Gpd = gpd(Gp::CX);
    pub const EDX: Gpd = gpd(Gp::DX);
    pub const EBX: Gpd = gpd(Gp::BX);
    pub const ESP: Gpd = gpd(Gp::SP);
    pub const EBP: Gpd = gpd(Gp::BP);
    pub const ESI: Gpd = gpd(Gp::SI);
    pub const EDI: Gpd = gpd(Gp::DI);
    pub const R8D: Gpd = gpd(Gp::R8);
    pub const R9D: Gpd = gpd(Gp::R9);
    pub const R10D: Gpd = gpd(Gp::R10);
    pub const R11D: Gpd = gpd(Gp::R11);
    pub const R12D: Gpd = gpd(Gp::R12);
    pub const R13D: Gpd = gpd(Gp::R13);
    pub const R14D: Gpd = gpd(Gp::R14);
    pub const R15D: Gpd = gpd(Gp::R15);

    pub const RAX: Gpq = gpq(Gp::AX);
    pub const RCX: Gpq = gpq(Gp::CX);
    pub const RDX: Gpq = gpq(Gp::DX);
    pub const RBX: Gpq = gpq(Gp::BX);
    pub const RSP: Gpq = gpq(Gp::SP);
    pub const RBP: Gpq = gpq(Gp::BP);
    pub const RSI: Gpq = gpq(Gp::SI);
    pub const RDI: Gpq = gpq(Gp::DI);
    pub const R8: Gpq = gpq(Gp::R8);
    pub const R9: Gpq = gpq(Gp::R9);
    pub const R10: Gpq = gpq(Gp::R10);
    pub const R11: Gpq = gpq(Gp::R11);
    pub const R12: Gpq = gpq(Gp::R12);
    pub const R13: Gpq = gpq(Gp::R13);
    pub const R14: Gpq = gpq(Gp::R14);
    pub const R15: Gpq = gpq(Gp::R15);

    pub const XMM0: Xmm = xmm(0);
    pub const XMM1: Xmm = xmm(1);
    pub const XMM2: Xmm = xmm(2);
    pub const XMM3: Xmm = xmm(3);
    pub const XMM4: Xmm = xmm(4);
    pub const XMM5: Xmm = xmm(5);
    pub const XMM6: Xmm = xmm(6);
    pub const XMM7: Xmm = xmm(7);
    pub const XMM8: Xmm = xmm(8);
    pub const XMM9: Xmm = xmm(9);
    pub const XMM10: Xmm = xmm(10);
    pub const XMM11: Xmm = xmm(11);
    pub const XMM12: Xmm = xmm(12);
    pub const XMM13: Xmm = xmm(13);
    pub const XMM14: Xmm = xmm(14);
    pub const XMM15: Xmm = xmm(15);
    pub const XMM16: Xmm = xmm(16);
    pub const XMM17: Xmm = xmm(17);
    pub const XMM18: Xmm = xmm(18);
    pub const XMM19: Xmm = xmm(19);
    pub const XMM20: Xmm = xmm(20);
    pub const XMM21: Xmm = xmm(21);
    pub const XMM22: Xmm = xmm(22);
    pub const XMM23: Xmm = xmm(23);
    pub const XMM24: Xmm = xmm(24);
    pub const XMM25: Xmm = xmm(25);
    pub const XMM26: Xmm = xmm(26);
    pub const XMM27: Xmm = xmm(27);
    pub const XMM28: Xmm = xmm(28);
    pub const XMM29: Xmm = xmm(29);
    pub const XMM30: Xmm = xmm(30);
    pub const XMM31: Xmm = xmm(31);

    pub const YMM0: Ymm = ymm(0);
    pub const YMM1: Ymm = ymm(1);
    pub const YMM2: Ymm = ymm(2);
    pub const YMM3: Ymm = ymm(3);
    pub const YMM4: Ymm = ymm(4);
    pub const YMM5: Ymm = ymm(5);
    pub const YMM6: Ymm = ymm(6);
    pub const YMM7: Ymm = ymm(7);
    pub const YMM8: Ymm = ymm(8);
    pub const YMM9: Ymm = ymm(9);
    pub const YMM10: Ymm = ymm(10);
    pub const YMM11: Ymm = ymm(11);
    pub const YMM12: Ymm = ymm(12);
    pub const YMM13: Ymm = ymm(13);
    pub const YMM14: Ymm = ymm(14);
    pub const YMM15: Ymm = ymm(15);
    pub const YMM16: Ymm = ymm(16);
    pub const YMM17: Ymm = ymm(17);
    pub const YMM18: Ymm = ymm(18);
    pub const YMM19: Ymm = ymm(19);
    pub const YMM20: Ymm = ymm(20);
    pub const YMM21: Ymm = ymm(21);
    pub const YMM22: Ymm = ymm(22);
    pub const YMM23: Ymm = ymm(23);
    pub const YMM24: Ymm = ymm(24);
    pub const YMM25: Ymm = ymm(25);
    pub const YMM26: Ymm = ymm(26);
    pub const YMM27: Ymm = ymm(27);
    pub const YMM28: Ymm = ymm(28);
    pub const YMM29: Ymm = ymm(29);
    pub const YMM30: Ymm = ymm(30);
    pub const YMM31: Ymm = ymm(31);

    pub const ZMM0: Zmm = zmm(0);
    pub const ZMM1: Zmm = zmm(1);
    pub const ZMM2: Zmm = zmm(2);
    pub const ZMM3: Zmm = zmm(3);
    pub const ZMM4: Zmm = zmm(4);
    pub const ZMM5: Zmm = zmm(5);
    pub const ZMM6: Zmm = zmm(6);
    pub const ZMM7: Zmm = zmm(7);
    pub const ZMM8: Zmm = zmm(8);
    pub const ZMM9: Zmm = zmm(9);
    pub const ZMM10: Zmm = zmm(10);
    pub const ZMM11: Zmm = zmm(11);
    pub const ZMM12: Zmm = zmm(12);
    pub const ZMM13: Zmm = zmm(13);
    pub const ZMM14: Zmm = zmm(14);
    pub const ZMM15: Zmm = zmm(15);
    pub const ZMM16: Zmm = zmm(16);
    pub const ZMM17: Zmm = zmm(17);
    pub const ZMM18: Zmm = zmm(18);
    pub const ZMM19: Zmm = zmm(19);
    pub const ZMM20: Zmm = zmm(20);
    pub const ZMM21: Zmm = zmm(21);
    pub const ZMM22: Zmm = zmm(22);
    pub const ZMM23: Zmm = zmm(23);
    pub const ZMM24: Zmm = zmm(24);
    pub const ZMM25: Zmm = zmm(25);
    pub const ZMM26: Zmm = zmm(26);
    pub const ZMM27: Zmm = zmm(27);
    pub const ZMM28: Zmm = zmm(28);
    pub const ZMM29: Zmm = zmm(29);
    pub const ZMM30: Zmm = zmm(30);
    pub const ZMM31: Zmm = zmm(31);

    pub const MM0: Mm = mm(0);
    pub const MM1: Mm = mm(1);
    pub const MM2: Mm = mm(2);
    pub const MM3: Mm = mm(3);
    pub const MM4: Mm = mm(4);
    pub const MM5: Mm = mm(5);
    pub const MM6: Mm = mm(6);
    pub const MM7: Mm = mm(7);

    pub const K0: KReg = k(0);
    pub const K1: KReg = k(1);
    pub const K2: KReg = k(2);
    pub const K3: KReg = k(3);
    pub const K4: KReg = k(4);
    pub const K5: KReg = k(5);
    pub const K6: KReg = k(6);
    pub const K7: KReg = k(7);

    pub const CR0: CReg = cr(0);
    pub const CR2: CReg = cr(2);
    pub const CR3: CReg = cr(3);
    pub const CR4: CReg = cr(4);
    pub const CR8: CReg = cr(8);

    pub const DR0: DReg = dr(0);
    pub const DR1: DReg = dr(1);
    pub const DR2: DReg = dr(2);
    pub const DR3: DReg = dr(3);
    pub const DR6: DReg = dr(6);
    pub const DR7: DReg = dr(7);

    pub const ST0: St = st(0);
    pub const ST1: St = st(1);
    pub const ST2: St = st(2);
    pub const ST3: St = st(3);
    pub const ST4: St = st(4);
    pub const ST5: St = st(5);
    pub const ST6: St = st(6);
    pub const ST7: St = st(7);

    pub const BND0: Bnd = bnd(0);
    pub const BND1: Bnd = bnd(1);
    pub const BND2: Bnd = bnd(2);
    pub const BND3: Bnd = bnd(3);

    pub const TMM0: Tmm = tmm(0);
    pub const TMM1: Tmm = tmm(1);
    pub const TMM2: Tmm = tmm(2);
    pub const TMM3: Tmm = tmm(3);
    pub const TMM4: Tmm = tmm(4);
    pub const TMM5: Tmm = tmm(5);
    pub const TMM6: Tmm = tmm(6);
    pub const TMM7: Tmm = tmm(7);

    pub const RIP: Rip = rip();

    pub const ES: SReg = sreg(SReg::ES);
    pub const CS: SReg = sreg(SReg::CS);
    pub const SS: SReg = sreg(SReg::SS);
    pub const DS: SReg = sreg(SReg::DS);
    pub const FS: SReg = sreg(SReg::FS);
    pub const GS: SReg = sreg(SReg::GS);
}
pub use regs::*;

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mem(pub BaseMem);

define_operand_cast!(Mem, BaseMem);

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AddrType {
    Default = 0,
    Abs = 1,
    Rel = 2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Broadcast {
    None = 0,
    B1To2 = 1,
    B1To4 = 2,
    B1To8 = 3,
    B1To16 = 4,
    B1To32 = 5,
    B1To64 = 6,
}
type Signature = OperandSignature;

impl Mem {
    pub const SIGNATURE_MEM_ADDR_TYPE_SHIFT: u32 = 14;
    pub const SIGNATURE_MEM_ADDR_TYPE_MASK: u32 = 0x03 << Self::SIGNATURE_MEM_ADDR_TYPE_SHIFT;

    pub const SIGNATURE_MEM_SHIFT_VALUE_SHIFT: u32 = 16;
    pub const SIGNATURE_MEM_SHIFT_VALUE_MASK: u32 = 0x03 << Self::SIGNATURE_MEM_SHIFT_VALUE_SHIFT;

    pub const SIGNATURE_MEM_SEGMENT_SHIFT: u32 = 18;
    pub const SIGNATURE_MEM_SEGMENT_MASK: u32 = 0x07 << Self::SIGNATURE_MEM_SEGMENT_SHIFT;

    pub const SIGNATURE_MEM_BROADCAST_SHIFT: u32 = 21;
    pub const SIGNATURE_MEM_BROADCAST_MASK: u32 = 0x7 << Self::SIGNATURE_MEM_BROADCAST_SHIFT;

    pub const fn new() -> Self {
        Self(BaseMem::new())
    }

    pub fn base_reg(&self) -> Reg {
        Reg::from_type_and_id(self.base_type(), self.base_id())
    }

    pub fn index_reg(&self) -> Reg {
        Reg::from_type_and_id(self.index_type(), self.index_id())
    }

    pub fn set_index(&mut self, index: &BaseReg, shift: u32) {
        self.0.set_index(index);
        self.set_shift(shift);
    }

    pub fn has_size(&self) -> bool {
        self.0.signature.has_field::<{ Signature::SIZE_MASK }>()
    }

    pub fn has_size_of(&self, s: u32) -> bool {
        self.size() == s
    }

    pub fn size(&self) -> u32 {
        self.0.signature.get_field::<{ Signature::SIZE_MASK }>()
    }

    pub fn set_size(&mut self, size: u32) {
        self.0.signature.set_field::<{ Signature::SIZE_MASK }>(size);
    }

    pub fn addr_type(&self) -> AddrType {
        unsafe {
            core::mem::transmute(
                self.0
                    .signature
                    .get_field::<{ Self::SIGNATURE_MEM_ADDR_TYPE_MASK }>(),
            )
        }
    }

    pub fn set_addr_type(&mut self, addr_type: AddrType) {
        self.0
            .signature
            .set_field::<{ Self::SIGNATURE_MEM_ADDR_TYPE_MASK }>(addr_type as u32);
    }

    pub fn reset_addr_type(&mut self) {
        self.set_addr_type(AddrType::Default);
    }

    pub fn is_abs(&self) -> bool {
        self.addr_type() == AddrType::Abs
    }

    pub fn set_abs(&mut self) {
        self.set_addr_type(AddrType::Abs);
    }

    pub fn is_rel(&self) -> bool {
        self.addr_type() == AddrType::Rel
    }

    pub fn set_rel(&mut self) {
        self.set_addr_type(AddrType::Rel);
    }

    pub fn has_segment(&self) -> bool {
        self.0
            .signature
            .has_field::<{ Self::SIGNATURE_MEM_SEGMENT_MASK }>()
    }

    pub fn segment(&self) -> SReg {
        SReg::from_id(self.segment_id())
    }

    pub fn segment_id(&self) -> u32 {
        self.0
            .signature
            .get_field::<{ Self::SIGNATURE_MEM_SEGMENT_MASK }>()
    }

    pub fn set_segment(&mut self, seg: SReg) {
        self.set_segment_id(seg.id());
    }

    pub fn set_segment_id(&mut self, r_id: u32) {
        self.0
            .signature
            .set_field::<{ Self::SIGNATURE_MEM_SEGMENT_MASK }>(r_id);
    }

    pub fn reset_segment(&mut self) {
        self.set_segment_id(0);
    }

    pub fn has_shift(&self) -> bool {
        self.0
            .signature
            .has_field::<{ Self::SIGNATURE_MEM_SHIFT_VALUE_MASK }>()
    }

    pub fn shift(&self) -> u32 {
        self.0
            .signature
            .get_field::<{ Self::SIGNATURE_MEM_SHIFT_VALUE_MASK }>()
    }

    pub fn set_shift(&mut self, shift: u32) {
        self.0
            .signature
            .set_field::<{ Self::SIGNATURE_MEM_SHIFT_VALUE_MASK }>(shift);
    }

    pub fn reset_shift(&mut self) {
        self.set_shift(0);
    }

    pub fn has_broadcast(&self) -> bool {
        self.0
            .signature
            .has_field::<{ Self::SIGNATURE_MEM_BROADCAST_MASK }>()
    }

    pub fn get_broadcast(&self) -> Broadcast {
        unsafe {
            core::mem::transmute(
                self.0
                    .signature
                    .get_field::<{ Self::SIGNATURE_MEM_BROADCAST_MASK }>(),
            )
        }
    }

    pub fn set_broadcast(&mut self, b: Broadcast) {
        self.0
            .signature
            .set_field::<{ Self::SIGNATURE_MEM_BROADCAST_MASK }>(b as u32);
    }

    pub fn reset_broadcast(&mut self) {
        self.set_broadcast(Broadcast::None);
    }

    pub fn _1to1(&self) -> Self {
        self.clone_broadcasted(Broadcast::None)
    }

    pub fn _1to2(&self) -> Self {
        self.clone_broadcasted(Broadcast::B1To2)
    }

    pub fn _1to4(&self) -> Self {
        self.clone_broadcasted(Broadcast::B1To4)
    }

    pub fn _1to8(&self) -> Self {
        self.clone_broadcasted(Broadcast::B1To8)
    }

    pub fn _1to16(&self) -> Self {
        self.clone_broadcasted(Broadcast::B1To16)
    }

    pub fn _1to32(&self) -> Self {
        self.clone_broadcasted(Broadcast::B1To32)
    }

    pub fn _1to64(&self) -> Self {
        self.clone_broadcasted(Broadcast::B1To64)
    }

    pub fn base_and_index_types(&self) -> u32 {
        self.signature
            .get_field::<{ OperandSignature::MEM_BASE_INDEX_MASK }>()
    }

    pub fn clone_broadcasted(&self, bcst: Broadcast) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            OperandSignature::new(
                self.0.signature.bits() & !Self::SIGNATURE_MEM_BROADCAST_MASK as u32,
            ) | OperandSignature::new((bcst as u32) << Self::SIGNATURE_MEM_BROADCAST_SHIFT),
            self.0.base_id(),
            self.0.data[0],
            self.0.data[1] as _,
        ))
    }

    pub fn from_signature_base_and_index_id_disp(
        signature: OperandSignature,
        base_id: u32,
        index_id: u32,
        offset: i32,
    ) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            signature, base_id, index_id, offset,
        ))
    }

    pub fn from_sym_and_index_shift_disp(
        base: &Sym,
        index: &BaseReg,
        shift: u32,
        off: i32,
        size: u32,
        signature: OperandSignature,
    ) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem)
                | Signature::from_mem_base_type(RegType::SymTag)
                | Signature::from_mem_index_type(index.typ())
                | Signature::from_value::<{ Self::SIGNATURE_MEM_SHIFT_VALUE_MASK }>(shift)
                | Signature::from_size(size)
                | signature,
            base.id(),
            index.id(),
            off,
        ))
    }
    pub fn from_label_and_index_shift_disp(
        base: &Label,
        index: &BaseReg,
        shift: u32,
        off: i32,
        size: u32,
        signature: OperandSignature,
    ) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem)
                | Signature::from_mem_base_type(RegType::LabelTag)
                | Signature::from_mem_index_type(index.typ())
                | Signature::from_value::<{ Self::SIGNATURE_MEM_SHIFT_VALUE_MASK }>(shift)
                | Signature::from_size(size)
                | signature,
            base.id(),
            index.id(),
            off,
        ))
    }

    pub fn from_label_and_disp(
        base: &Label,
        off: i32,
        size: u32,
        signature: OperandSignature,
    ) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem)
                | Signature::from_mem_base_type(RegType::LabelTag)
                | Signature::from_size(size)
                | signature,
            base.id(),
            0,
            off,
        ))
    }

    pub fn from_sym_and_disp(base: &Sym, off: i32, size: u32, signature: OperandSignature) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem)
                | Signature::from_mem_base_type(RegType::SymTag)
                | Signature::from_size(size)
                | signature,
            base.id(),
            0,
            off,
        ))
    }

    pub fn from_base_and_disp(
        base: &BaseReg,
        off: i32,
        size: u32,
        signature: OperandSignature,
    ) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem)
                | Signature::from_mem_base_type(base.typ())
                | Signature::from_size(size)
                | signature,
            base.id(),
            0,
            off,
        ))
    }

    pub fn from_base_and_index_shift_disp(
        base: &BaseReg,
        index: &BaseReg,
        shift: u32,
        off: i32,
        size: u32,
        signature: OperandSignature,
    ) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem)
                | Signature::from_mem_base_type(base.typ())
                | Signature::from_mem_index_type(index.typ())
                | Signature::from_value::<{ Self::SIGNATURE_MEM_SHIFT_VALUE_MASK }>(shift)
                | Signature::from_size(size)
                | signature,
            base.id(),
            index.id(),
            off,
        ))
    }

    pub fn from_u64_and_index_shift_disp(
        base: u64,
        index: &BaseReg,
        shift: u32,
        size: u32,
        signature: OperandSignature,
    ) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem)
                | Signature::from_mem_index_type(index.typ())
                | Signature::from_value::<{ Self::SIGNATURE_MEM_SHIFT_VALUE_MASK }>(shift)
                | Signature::from_size(size)
                | signature,
            (base >> 32) as u32,
            index.id(),
            (base & 0xFFFFFFF) as _,
        ))
    }

    pub fn from_u64(base: u64, size: u32, signature: OperandSignature) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem) | Signature::from_size(size) | signature,
            (base >> 32) as u32,
            0,
            (base & 0xFFFFFFF) as _,
        ))
    }
}

impl Add<i32> for Gpq {
    type Output = Mem;
    fn add(self, rhs: i32) -> Self::Output {
        ptr(self, rhs, 0)
    }
}

impl Mul<i32> for Gpq {
    type Output = Mem;

    fn mul(self, rhs: i32) -> Self::Output {
        let shift = match rhs {
            0 | 1 => 0,
            2 => 1,
            4 => 2,
            8 => 3,
            16 => 4,
            _ => todo!(),
        };
        ptr_index(RAX, self, shift, 0, 0)
    }
}

impl<T: Deref<Target = Gp>> Mul<T> for Mem {
    type Output = Mem;

    fn mul(self, rhs: T) -> Self::Output {
        let mut this = self;
        let reg = rhs.deref();
        this.set_index_id(reg.id());
        this.set_index_type(reg.typ());

        this
    }
}

impl Add<i32> for Label {
    type Output = Mem;

    fn add(self, rhs: i32) -> Self::Output {
        label_ptr(self, rhs, 0)
    }
}

impl Add<Mem> for Gpq {
    type Output = Mem;

    fn add(self, mut rhs: Mem) -> Self::Output {
        rhs.set_base(&self);
        rhs
    }
}

impl Add<i32> for Mem {
    type Output = Mem;

    fn add(self, rhs: i32) -> Self::Output {
        let mut this = self;
        this.add_offset(rhs as _);
        this
    }
}

macro_rules! mem_ptr {
    ($name: ident, $size: literal) => {
        paste::paste! {
            pub fn $name(base: impl Deref<Target = Gp>, offset: i32) -> Mem {
                Mem::from_base_and_disp(base.deref(), offset, $size, 0.into())
            }

            pub fn [<$name _index>](base: impl Deref<Target = Gp>, index: impl Deref<Target = Gp>, shift: u32, offset: i32) -> Mem {
                Mem::from_base_and_index_shift_disp(base.deref(), index.deref(), shift, offset, $size, 0.into())
            }

            pub fn [<$name _label>](base: Label, offset: i32) -> Mem {
                Mem::from_label_and_disp(&base, offset, $size, 0.into())
            }

            pub fn [<$name _label_index>](base: Label, index: impl Deref<Target = Gp>, shift: u32, offset: i32) -> Mem {
                Mem::from_label_and_index_shift_disp(&base, index.deref(), shift, offset, $size, 0.into())
            }

            pub fn [<$name _sym>](base: Sym, offset: i32) -> Mem {
                Mem::from_sym_and_disp(&base, offset, $size, 0.into())
            }

            pub fn [<$name _sym_index>](base: Sym, index: impl Deref<Target = Gp>, shift: u32, offset: i32) -> Mem {
                Mem::from_sym_and_index_shift_disp(&base, index.deref(), shift, offset, $size, 0.into())
            }


            pub fn [<$name _rip>](offset: i32) -> Mem {
                Mem::from_base_and_disp(&RIP, offset, $size, 0.into())
            }

            pub fn [<$name _u64>](base: u64) -> Mem {
                Mem::from_u64(base, $size, 0.into())
            }

            pub fn [<$name _u64_index>](base: u64, index: impl Deref<Target = Gp>, shift: u32) -> Mem {
                Mem::from_u64_and_index_shift_disp(base, index.deref(), shift, $size, 0.into())
            }

            pub fn [<$name _u64_abs>](base: u64) -> Mem {
                Mem::from_u64(base, $size, OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_ADDR_TYPE_MASK }>(AddrType::Abs as _))
            }

            pub fn [<$name _u64_index_abs>](base: u64, index: impl Deref<Target = Gp>, shift: u32) -> Mem {
                Mem::from_u64_and_index_shift_disp(base, index.deref(), shift, $size, OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_ADDR_TYPE_MASK }>(AddrType::Abs as _))
            }

            pub fn [<$name _u64_rel>](base: u64) -> Mem {
                Mem::from_u64(base, $size, OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_ADDR_TYPE_MASK }>(AddrType::Rel as _))
            }

            pub fn [<$name _u64_index_rel>](base: u64, index: impl Deref<Target = Gp>, shift: u32) -> Mem {
                Mem::from_u64_and_index_shift_disp(base, index.deref(), shift, $size, OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_ADDR_TYPE_MASK }>(AddrType::Rel as _))
            }
        }
    }
}
mem_ptr!(ptr8, 1);
mem_ptr!(ptr16, 2);
mem_ptr!(ptr32, 4);
mem_ptr!(ptr48, 6);
mem_ptr!(ptr64, 8);
mem_ptr!(ptr80, 10);
mem_ptr!(ptr128, 16);
mem_ptr!(ptr256, 32);
mem_ptr!(ptr512, 64);

mem_ptr!(byte_ptr, 1);
mem_ptr!(word_ptr, 2);
mem_ptr!(dword_ptr, 4);
mem_ptr!(fword_ptr, 6);
mem_ptr!(qword_ptr, 8);
mem_ptr!(tbyte_ptr, 10);
mem_ptr!(tword_ptr, 10);
mem_ptr!(oword_ptr, 16);
mem_ptr!(dqword_ptr, 16);
mem_ptr!(qqword_ptr, 32);
mem_ptr!(xmmword_ptr, 16);
mem_ptr!(ymmword_ptr, 32);
mem_ptr!(zmmword_ptr, 64);

pub fn ptr(base: impl Deref<Target = Gp>, offset: i32, size: u32) -> Mem {
    Mem::from_base_and_disp(base.deref(), offset, size, 0.into())
}

pub fn ptr_index(
    base: impl Deref<Target = Gp>,
    index: impl Deref<Target = Gp>,
    shift: u32,
    offset: i32,
    size: u32,
) -> Mem {
    Mem::from_base_and_index_shift_disp(base.deref(), index.deref(), shift, offset, size, 0.into())
}

pub fn label_ptr(base: Label, offset: i32, size: u32) -> Mem {
    Mem::from_label_and_disp(&base, offset, size, 0.into())
}
pub fn label_ptr_index(
    base: Label,
    index: impl Deref<Target = Gp>,
    shift: u32,
    offset: i32,
    size: u32,
) -> Mem {
    Mem::from_label_and_index_shift_disp(&base, index.deref(), shift, offset, size, 0.into())
}

pub fn sym_ptr(base: Sym, offset: i32, size: u32) -> Mem {
    Mem::from_sym_and_disp(&base, offset, size, 0.into())
}
pub fn sym_ptr_index(
    base: Sym,
    index: impl Deref<Target = Gp>,
    shift: u32,
    offset: i32,
    size: u32,
) -> Mem {
    Mem::from_sym_and_index_shift_disp(&base, index.deref(), shift, offset, size, 0.into())
}

pub fn rip_rel(offset: i32, size: u32) -> Mem {
    Mem::from_base_and_disp(&RIP, offset, size, 0.into())
}

pub fn u64_ptr(base: u64, size: u32) -> Mem {
    Mem::from_u64(base, size, 0.into())
}

pub fn u64_ptr_index(base: u64, index: impl Deref<Target = Gp>, shift: u32, size: u32) -> Mem {
    Mem::from_u64_and_index_shift_disp(base, index.deref(), shift, size, 0.into())
}

pub fn u64_ptr_abs(base: u64, size: u32) -> Mem {
    Mem::from_u64(
        base,
        size,
        OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_ADDR_TYPE_MASK }>(AddrType::Abs as _),
    )
}

pub fn u64_ptr_index_abs(base: u64, index: impl Deref<Target = Gp>, shift: u32, size: u32) -> Mem {
    Mem::from_u64_and_index_shift_disp(
        base,
        index.deref(),
        shift,
        size,
        OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_ADDR_TYPE_MASK }>(AddrType::Abs as _),
    )
}
