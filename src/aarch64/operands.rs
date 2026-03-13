//! AArch64 operands definition.
use crate::{
    core::{operand::*, types::TypeId},
    define_abstract_reg, define_operand_cast,
};

use core::fmt;

macro_rules! impl_deref_for_wrapper {
    ($wrapper:ty, $target:ty) => {
        impl core::ops::Deref for $wrapper {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

pub struct A64Gpw;
impl RegTraits for A64Gpw {
    const TYPE: RegType = RegType::Gp32;
    const GROUP: RegGroup = RegGroup::Gp;
    const SIZE: u32 = 4;
    const TYPE_ID: TypeId = TypeId::Int32;
}

pub struct A64Gpx;
impl RegTraits for A64Gpx {
    const TYPE: RegType = RegType::Gp64;
    const GROUP: RegGroup = RegGroup::Gp;
    const SIZE: u32 = 8;
    const TYPE_ID: TypeId = TypeId::Int64;
}

pub struct A64VecB;
impl RegTraits for A64VecB {
    const TYPE: RegType = RegType::Vec8;
    const GROUP: RegGroup = RegGroup::Vec;
    const SIZE: u32 = 1;
    const TYPE_ID: TypeId = TypeId::Int8;
}

pub struct A64VecH;
impl RegTraits for A64VecH {
    const TYPE: RegType = RegType::Vec16;
    const GROUP: RegGroup = RegGroup::Vec;
    const SIZE: u32 = 2;
    const TYPE_ID: TypeId = TypeId::Int16;
}

pub struct A64VecS;
impl RegTraits for A64VecS {
    const TYPE: RegType = RegType::Vec32;
    const GROUP: RegGroup = RegGroup::Vec;
    const SIZE: u32 = 4;
    const TYPE_ID: TypeId = TypeId::Int32;
}

pub struct A64VecD;
impl RegTraits for A64VecD {
    const TYPE: RegType = RegType::Vec64;
    const GROUP: RegGroup = RegGroup::Vec;
    const SIZE: u32 = 8;
    const TYPE_ID: TypeId = TypeId::Int64;
}

pub struct A64VecQ;
impl RegTraits for A64VecQ {
    const TYPE: RegType = RegType::Vec128;
    const GROUP: RegGroup = RegGroup::Vec;
    const SIZE: u32 = 16;
    const TYPE_ID: TypeId = TypeId::Int32x4;
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Reg(pub BaseReg);

impl_deref_for_wrapper!(Reg, BaseReg);

define_abstract_reg!(Reg, BaseReg);

impl Reg {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        match typ {
            RegType::Gp32 => OperandSignature::new(A64Gpw::SIGNATURE),
            RegType::Gp64 => OperandSignature::new(A64Gpx::SIGNATURE),
            RegType::Vec8 => OperandSignature::new(A64VecB::SIGNATURE),
            RegType::Vec16 => OperandSignature::new(A64VecH::SIGNATURE),
            RegType::Vec32 => OperandSignature::new(A64VecS::SIGNATURE),
            RegType::Vec64 => OperandSignature::new(A64VecD::SIGNATURE),
            RegType::Vec128 => OperandSignature::new(A64VecQ::SIGNATURE),
            _ => OperandSignature::new(0),
        }
    }

    pub fn is_gp(&self) -> bool {
        self.is_group(RegGroup::Gp)
    }

    pub fn is_gp32(&self) -> bool {
        self.has_base_signature(A64Gpw::SIGNATURE)
    }

    pub fn is_gp64(&self) -> bool {
        self.has_base_signature(A64Gpx::SIGNATURE)
    }

    pub fn is_vec(&self) -> bool {
        self.is_group(RegGroup::Vec)
    }

    pub fn is_vec8(&self) -> bool {
        self.has_base_signature(A64VecB::SIGNATURE)
    }

    pub fn is_vec16(&self) -> bool {
        self.has_base_signature(A64VecH::SIGNATURE)
    }

    pub fn is_vec32(&self) -> bool {
        self.has_base_signature(A64VecS::SIGNATURE)
    }

    pub fn is_vec64(&self) -> bool {
        self.has_base_signature(A64VecD::SIGNATURE)
    }

    pub fn is_vec128(&self) -> bool {
        self.has_base_signature(A64VecQ::SIGNATURE)
    }

    pub fn set_reg_t<T: RegTraits>(&mut self, rid: u32) {
        self.set_signature(T::SIGNATURE.into());
        self.set_id(rid);
    }

    pub fn set_type_and_id(&mut self, typ: RegType, id: u32) {
        self.set_signature(Self::signature_of(typ));
        self.set_id(id);
    }

    pub const fn group_of_t<T: RegTraits>() -> RegGroup {
        T::GROUP
    }

    pub const fn type_id_of_t<T: RegTraits>() -> TypeId {
        T::TYPE_ID
    }

    pub fn is_sp(&self) -> bool {
        self.is_gp() && self.0.0.id() == Gp::ID_SP
    }

    pub fn is_zr(&self) -> bool {
        self.is_gp() && self.0.0.id() == Gp::ID_ZR
    }

    pub const SIGNATURE: u32 = BaseReg::SIGNATURE;
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Gp(pub Reg);

impl_deref_for_wrapper!(Gp, Reg);

define_abstract_reg!(Gp, Reg);

impl Gp {
    pub const ID_OS: u32 = 18;
    pub const ID_FP: u32 = 29;
    pub const ID_LR: u32 = 30;
    pub const ID_SP: u32 = 31;
    pub const ID_ZR: u32 = 63;

    pub const fn signature_of(typ: RegType) -> OperandSignature {
        Reg::signature_of(typ)
    }

    pub const fn make_r32(reg_id: u32) -> Self {
        Self(Reg::from_signature_and_id(
            OperandSignature::new(A64Gpw::SIGNATURE),
            reg_id,
        ))
    }

    pub const fn make_r64(reg_id: u32) -> Self {
        Self(Reg::from_signature_and_id(
            OperandSignature::new(A64Gpx::SIGNATURE),
            reg_id,
        ))
    }

    pub const fn make_w(reg_id: u32) -> Self {
        Self::make_r32(reg_id)
    }

    pub const fn make_x(reg_id: u32) -> Self {
        Self::make_r64(reg_id)
    }

    pub fn is_zr(&self) -> bool {
        self.id() == Self::ID_ZR
    }

    pub fn is_sp(&self) -> bool {
        self.id() == Self::ID_SP
    }

    pub fn r32(&self) -> Self {
        Self::make_r32(self.id())
    }

    pub fn r64(&self) -> Self {
        Self::make_r64(self.id())
    }

    pub fn w(&self) -> Self {
        self.r32()
    }

    pub fn x(&self) -> Self {
        self.r64()
    }

    pub const SIGNATURE: u32 = Reg::SIGNATURE;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
pub enum VecElementType {
    None = 0,
    B = 1,
    H = 2,
    S = 3,
    D = 4,
    B4 = 5,
    H2 = 6,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Vec(pub Reg);

impl_deref_for_wrapper!(Vec, Reg);

define_abstract_reg!(Vec, Reg);

impl Vec {
    pub const SIGNATURE_REG_ELEMENT_TYPE_SHIFT: u32 = 12;
    pub const SIGNATURE_REG_ELEMENT_TYPE_MASK: u32 = 0x07 << Self::SIGNATURE_REG_ELEMENT_TYPE_SHIFT;

    pub const SIGNATURE_REG_ELEMENT_FLAG_SHIFT: u32 = 15;
    pub const SIGNATURE_REG_ELEMENT_FLAG_MASK: u32 = 0x01 << Self::SIGNATURE_REG_ELEMENT_FLAG_SHIFT;

    pub const SIGNATURE_REG_ELEMENT_INDEX_SHIFT: u32 = 16;
    pub const SIGNATURE_REG_ELEMENT_INDEX_MASK: u32 =
        0x0F << Self::SIGNATURE_REG_ELEMENT_INDEX_SHIFT;

    pub const SIGNATURE_ELEMENT_B: u32 =
        (VecElementType::B as u32) << Self::SIGNATURE_REG_ELEMENT_TYPE_SHIFT;
    pub const SIGNATURE_ELEMENT_H: u32 =
        (VecElementType::H as u32) << Self::SIGNATURE_REG_ELEMENT_TYPE_SHIFT;
    pub const SIGNATURE_ELEMENT_S: u32 =
        (VecElementType::S as u32) << Self::SIGNATURE_REG_ELEMENT_TYPE_SHIFT;
    pub const SIGNATURE_ELEMENT_D: u32 =
        (VecElementType::D as u32) << Self::SIGNATURE_REG_ELEMENT_TYPE_SHIFT;
    pub const SIGNATURE_ELEMENT_B4: u32 =
        (VecElementType::B4 as u32) << Self::SIGNATURE_REG_ELEMENT_TYPE_SHIFT;
    pub const SIGNATURE_ELEMENT_H2: u32 =
        (VecElementType::H2 as u32) << Self::SIGNATURE_REG_ELEMENT_TYPE_SHIFT;

    const fn make_element_access_signature(
        element_type: VecElementType,
        element_index: u32,
    ) -> OperandSignature {
        OperandSignature::new(
            A64VecQ::SIGNATURE
                | Self::SIGNATURE_REG_ELEMENT_FLAG_MASK
                | ((element_type as u32) << Self::SIGNATURE_REG_ELEMENT_TYPE_SHIFT)
                | (element_index << Self::SIGNATURE_REG_ELEMENT_INDEX_SHIFT),
        )
    }

    pub const fn signature_of(typ: RegType) -> OperandSignature {
        Reg::signature_of(typ)
    }

    pub const fn make_v8(reg_id: u32) -> Self {
        Self(Reg::from_signature_and_id(
            OperandSignature::new(A64VecB::SIGNATURE),
            reg_id,
        ))
    }

    pub const fn make_v16(reg_id: u32) -> Self {
        Self(Reg::from_signature_and_id(
            OperandSignature::new(A64VecH::SIGNATURE),
            reg_id,
        ))
    }

    pub const fn make_v32(reg_id: u32) -> Self {
        Self(Reg::from_signature_and_id(
            OperandSignature::new(A64VecS::SIGNATURE),
            reg_id,
        ))
    }

    pub const fn make_v64(reg_id: u32) -> Self {
        Self(Reg::from_signature_and_id(
            OperandSignature::new(A64VecD::SIGNATURE),
            reg_id,
        ))
    }

    pub const fn make_v128(reg_id: u32) -> Self {
        Self(Reg::from_signature_and_id(
            OperandSignature::new(A64VecQ::SIGNATURE),
            reg_id,
        ))
    }

    pub const fn make_b(reg_id: u32) -> Self {
        Self::make_v8(reg_id)
    }

    pub const fn make_h(reg_id: u32) -> Self {
        Self::make_v16(reg_id)
    }

    pub const fn make_s(reg_id: u32) -> Self {
        Self::make_v32(reg_id)
    }

    pub const fn make_d(reg_id: u32) -> Self {
        Self::make_v64(reg_id)
    }

    pub const fn make_q(reg_id: u32) -> Self {
        Self::make_v128(reg_id)
    }

    pub const fn make_v32_with_element_type(element_type: VecElementType, reg_id: u32) -> Self {
        Self(Reg::from_signature_and_id(
            OperandSignature::new(
                A64VecS::SIGNATURE
                    | ((element_type as u32) << Self::SIGNATURE_REG_ELEMENT_TYPE_SHIFT),
            ),
            reg_id,
        ))
    }

    pub const fn make_v64_with_element_type(element_type: VecElementType, reg_id: u32) -> Self {
        Self(Reg::from_signature_and_id(
            OperandSignature::new(
                A64VecD::SIGNATURE
                    | ((element_type as u32) << Self::SIGNATURE_REG_ELEMENT_TYPE_SHIFT),
            ),
            reg_id,
        ))
    }

    pub const fn make_v128_with_element_type(element_type: VecElementType, reg_id: u32) -> Self {
        Self(Reg::from_signature_and_id(
            OperandSignature::new(
                A64VecQ::SIGNATURE
                    | ((element_type as u32) << Self::SIGNATURE_REG_ELEMENT_TYPE_SHIFT),
            ),
            reg_id,
        ))
    }

    pub const fn make_v128_with_element_index(
        element_type: VecElementType,
        element_index: u32,
        reg_id: u32,
    ) -> Self {
        Self(Reg::from_signature_and_id(
            Self::make_element_access_signature(element_type, element_index),
            reg_id,
        ))
    }

    pub fn has_element_type_or_index(&self) -> bool {
        self.0
			.0
			.0
			.signature
			.has_field::<{ Self::SIGNATURE_REG_ELEMENT_TYPE_MASK | Self::SIGNATURE_REG_ELEMENT_FLAG_MASK }>()
    }

    pub fn is_vec_b8(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .subset(REG_BASE_SIGNATURE_MASK | Self::SIGNATURE_REG_ELEMENT_TYPE_MASK)
            == OperandSignature::new(A64VecD::SIGNATURE | Self::SIGNATURE_ELEMENT_B)
    }

    pub fn is_vec_h4(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .subset(REG_BASE_SIGNATURE_MASK | Self::SIGNATURE_REG_ELEMENT_TYPE_MASK)
            == OperandSignature::new(A64VecD::SIGNATURE | Self::SIGNATURE_ELEMENT_H)
    }

    pub fn is_vec_s2(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .subset(REG_BASE_SIGNATURE_MASK | Self::SIGNATURE_REG_ELEMENT_TYPE_MASK)
            == OperandSignature::new(A64VecD::SIGNATURE | Self::SIGNATURE_ELEMENT_S)
    }

    pub fn is_vec_d1(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .subset(REG_BASE_SIGNATURE_MASK | Self::SIGNATURE_REG_ELEMENT_TYPE_MASK)
            == OperandSignature::new(A64VecD::SIGNATURE)
    }

    pub fn is_vec_b16(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .subset(REG_BASE_SIGNATURE_MASK | Self::SIGNATURE_REG_ELEMENT_TYPE_MASK)
            == OperandSignature::new(A64VecQ::SIGNATURE | Self::SIGNATURE_ELEMENT_B)
    }

    pub fn is_vec_h8(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .subset(REG_BASE_SIGNATURE_MASK | Self::SIGNATURE_REG_ELEMENT_TYPE_MASK)
            == OperandSignature::new(A64VecQ::SIGNATURE | Self::SIGNATURE_ELEMENT_H)
    }

    pub fn is_vec_s4(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .subset(REG_BASE_SIGNATURE_MASK | Self::SIGNATURE_REG_ELEMENT_TYPE_MASK)
            == OperandSignature::new(A64VecQ::SIGNATURE | Self::SIGNATURE_ELEMENT_S)
    }

    pub fn is_vec_d2(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .subset(REG_BASE_SIGNATURE_MASK | Self::SIGNATURE_REG_ELEMENT_TYPE_MASK)
            == OperandSignature::new(A64VecQ::SIGNATURE | Self::SIGNATURE_ELEMENT_D)
    }

    pub fn is_vec_b4x4(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .subset(REG_BASE_SIGNATURE_MASK | Self::SIGNATURE_REG_ELEMENT_TYPE_MASK)
            == OperandSignature::new(A64VecQ::SIGNATURE | Self::SIGNATURE_ELEMENT_B4)
    }

    pub fn is_vec_h2x4(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .subset(REG_BASE_SIGNATURE_MASK | Self::SIGNATURE_REG_ELEMENT_TYPE_MASK)
            == OperandSignature::new(A64VecQ::SIGNATURE | Self::SIGNATURE_ELEMENT_H2)
    }

    pub fn v8(&self) -> Self {
        Self::make_v8(self.id())
    }

    pub fn v16(&self) -> Self {
        Self::make_v16(self.id())
    }

    pub fn v32(&self) -> Self {
        Self::make_v32(self.id())
    }

    pub fn v64(&self) -> Self {
        Self::make_v64(self.id())
    }

    pub fn v128(&self) -> Self {
        Self::make_v128(self.id())
    }

    pub fn b(&self) -> Self {
        self.v8()
    }

    pub fn h(&self) -> Self {
        self.v16()
    }

    pub fn s(&self) -> Self {
        self.v32()
    }

    pub fn d(&self) -> Self {
        self.v64()
    }

    pub fn q(&self) -> Self {
        self.v128()
    }

    pub fn b_at(&self, element_index: u32) -> Self {
        Self::make_v128_with_element_index(VecElementType::B, element_index, self.id())
    }

    pub fn h_at(&self, element_index: u32) -> Self {
        Self::make_v128_with_element_index(VecElementType::H, element_index, self.id())
    }

    pub fn s_at(&self, element_index: u32) -> Self {
        Self::make_v128_with_element_index(VecElementType::S, element_index, self.id())
    }

    pub fn d_at(&self, element_index: u32) -> Self {
        Self::make_v128_with_element_index(VecElementType::D, element_index, self.id())
    }

    pub fn h2_at(&self, element_index: u32) -> Self {
        Self::make_v128_with_element_index(VecElementType::H2, element_index, self.id())
    }

    pub fn b4_at(&self, element_index: u32) -> Self {
        Self::make_v128_with_element_index(VecElementType::B4, element_index, self.id())
    }

    pub fn b8(&self) -> Self {
        Self::make_v64_with_element_type(VecElementType::B, self.id())
    }

    pub fn b16(&self) -> Self {
        Self::make_v128_with_element_type(VecElementType::B, self.id())
    }

    pub fn h2(&self) -> Self {
        Self::make_v32_with_element_type(VecElementType::H, self.id())
    }

    pub fn h4(&self) -> Self {
        Self::make_v64_with_element_type(VecElementType::H, self.id())
    }

    pub fn h8(&self) -> Self {
        Self::make_v128_with_element_type(VecElementType::H, self.id())
    }

    pub fn s2(&self) -> Self {
        Self::make_v64_with_element_type(VecElementType::S, self.id())
    }

    pub fn s4(&self) -> Self {
        Self::make_v128_with_element_type(VecElementType::S, self.id())
    }

    pub fn d2(&self) -> Self {
        Self::make_v128_with_element_type(VecElementType::D, self.id())
    }

    pub fn has_element_type(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .has_field::<{ Self::SIGNATURE_REG_ELEMENT_TYPE_MASK }>()
    }

    pub fn element_type(&self) -> VecElementType {
        match self
            .0
            .0
            .0
            .signature
            .get_field::<{ Self::SIGNATURE_REG_ELEMENT_TYPE_MASK }>()
        {
            1 => VecElementType::B,
            2 => VecElementType::H,
            3 => VecElementType::S,
            4 => VecElementType::D,
            5 => VecElementType::B4,
            6 => VecElementType::H2,
            _ => VecElementType::None,
        }
    }

    pub fn set_element_type(&mut self, element_type: VecElementType) {
        self.0
            .0
            .0
            .signature
            .set_field::<{ Self::SIGNATURE_REG_ELEMENT_TYPE_MASK }>(element_type as u32);
    }

    pub fn reset_element_type(&mut self) {
        self.0
            .0
            .0
            .signature
            .set_field::<{ Self::SIGNATURE_REG_ELEMENT_TYPE_MASK }>(0);
    }

    pub fn has_element_index(&self) -> bool {
        self.0
            .0
            .0
            .signature
            .has_field::<{ Self::SIGNATURE_REG_ELEMENT_FLAG_MASK }>()
    }

    pub fn element_index(&self) -> u32 {
        self.0
            .0
            .0
            .signature
            .get_field::<{ Self::SIGNATURE_REG_ELEMENT_INDEX_MASK }>()
    }

    pub fn set_element_index(&mut self, element_index: u32) {
        self.0.0.0.signature.bits |= Self::SIGNATURE_REG_ELEMENT_FLAG_MASK;
        self.0
            .0
            .0
            .signature
            .set_field::<{ Self::SIGNATURE_REG_ELEMENT_INDEX_MASK }>(element_index);
    }

    pub fn reset_element_index(&mut self) {
        self.0.0.0.signature.bits &=
            !(Self::SIGNATURE_REG_ELEMENT_FLAG_MASK | Self::SIGNATURE_REG_ELEMENT_INDEX_MASK);
    }

    pub fn at(&self, element_index: u32) -> Self {
        Self(Reg::from_signature_and_id(
            OperandSignature::new(
                (self.0.0.0.signature.bits & !Self::SIGNATURE_REG_ELEMENT_INDEX_MASK)
                    | (element_index << Self::SIGNATURE_REG_ELEMENT_INDEX_SHIFT)
                    | Self::SIGNATURE_REG_ELEMENT_FLAG_MASK,
            ),
            self.id(),
        ))
    }

    pub const SIGNATURE: u32 = Reg::SIGNATURE;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
pub enum ShiftOp {
    LSL = 0,
    LSR = 1,
    ASR = 2,
    ROR = 3,
    RRX = 4,
    MSL = 5,
    UXTB = 6,
    UXTH = 7,
    UXTW = 8,
    UXTX = 9,
    SXTB = 10,
    SXTH = 11,
    SXTW = 12,
    SXTX = 13,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Shift {
    op: ShiftOp,
    value: u32,
}

impl Shift {
    pub const fn new(op: ShiftOp, value: u32) -> Self {
        Self { op, value }
    }

    pub const fn op(&self) -> ShiftOp {
        self.op
    }

    pub const fn value(&self) -> u32 {
        self.value
    }
}

impl From<u32> for Shift {
    fn from(value: u32) -> Self {
        Self::new(ShiftOp::LSL, value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
pub enum OffsetMode {
    Fixed = 0,
    PreIndex = 1,
    PostIndex = 2,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Mem(pub BaseMem);

impl_deref_for_wrapper!(Mem, BaseMem);

define_operand_cast!(Mem, BaseMem);

type Signature = OperandSignature;

impl Default for Mem {
    fn default() -> Self {
        Self::new()
    }
}

impl Mem {
    pub const SIGNATURE_MEM_SHIFT_VALUE_SHIFT: u32 = 14;
    pub const SIGNATURE_MEM_SHIFT_VALUE_MASK: u32 = 0x1F << Self::SIGNATURE_MEM_SHIFT_VALUE_SHIFT;

    pub const SIGNATURE_MEM_SHIFT_OP_SHIFT: u32 = 20;
    pub const SIGNATURE_MEM_SHIFT_OP_MASK: u32 = 0x0F << Self::SIGNATURE_MEM_SHIFT_OP_SHIFT;

    pub const SIGNATURE_MEM_OFFSET_MODE_SHIFT: u32 = 24;
    pub const SIGNATURE_MEM_OFFSET_MODE_MASK: u32 = 0x03 << Self::SIGNATURE_MEM_OFFSET_MODE_SHIFT;

    pub const fn new() -> Self {
        Self(BaseMem::new())
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

    pub fn from_label_and_disp(base: &Label, off: i32, signature: OperandSignature) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem)
                | Signature::from_mem_base_type(RegType::LabelTag)
                | signature,
            base.id(),
            0,
            off,
        ))
    }

    pub fn from_base_and_disp(base: &Reg, off: i32, signature: OperandSignature) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem)
                | Signature::from_mem_base_type(base.typ())
                | signature,
            base.id(),
            0,
            off,
        ))
    }

    pub fn from_base_and_index(base: &Reg, index: &Reg, signature: OperandSignature) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem)
                | Signature::from_mem_base_type(base.typ())
                | Signature::from_mem_index_type(index.typ())
                | signature,
            base.id(),
            index.id(),
            0,
        ))
    }

    pub fn from_base_and_index_shift(
        base: &Reg,
        index: &Reg,
        shift: impl Into<Shift>,
        signature: OperandSignature,
    ) -> Self {
        let shift = shift.into();
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem)
                | Signature::from_mem_base_type(base.typ())
                | Signature::from_mem_index_type(index.typ())
                | Signature::from_value::<{ Self::SIGNATURE_MEM_SHIFT_OP_MASK }>(shift.op() as u32)
                | Signature::from_value::<{ Self::SIGNATURE_MEM_SHIFT_VALUE_MASK }>(shift.value())
                | signature,
            base.id(),
            index.id(),
            0,
        ))
    }

    pub fn from_u64(base: u64, signature: OperandSignature) -> Self {
        Self(BaseMem::from_base_and_index_disp(
            Signature::from_op_type(OperandType::Mem) | signature,
            (base >> 32) as u32,
            0,
            (base & 0xFFFF_FFFF) as u32 as i32,
        ))
    }

    pub fn clone_adjusted(&self, off: i64) -> Self {
        let mut result = *self;
        result.add_offset(off);
        result
    }

    pub fn pre(&self) -> Self {
        let mut result = *self;
        result.set_offset_mode(OffsetMode::PreIndex);
        result
    }

    pub fn pre_offset(&self, off: i64) -> Self {
        let mut result = *self;
        result.set_offset_mode(OffsetMode::PreIndex);
        result.add_offset(off);
        result
    }

    pub fn post(&self) -> Self {
        let mut result = *self;
        result.set_offset_mode(OffsetMode::PostIndex);
        result
    }

    pub fn post_offset(&self, off: i64) -> Self {
        let mut result = *self;
        result.set_offset_mode(OffsetMode::PostIndex);
        result.add_offset(off);
        result
    }

    pub fn base_reg(&self) -> Reg {
        Reg::from_type_and_id(self.base_type(), self.base_id())
    }

    pub fn index_reg(&self) -> Reg {
        Reg::from_type_and_id(self.index_type(), self.index_id())
    }

    pub fn set_index(&mut self, index: Reg, shift: impl Into<Shift>) {
        self.0.set_index(&index.0);
        self.set_shift(shift);
    }

    pub fn offset_mode(&self) -> OffsetMode {
        match self
            .0
            .signature
            .get_field::<{ Self::SIGNATURE_MEM_OFFSET_MODE_MASK }>()
        {
            1 => OffsetMode::PreIndex,
            2 => OffsetMode::PostIndex,
            _ => OffsetMode::Fixed,
        }
    }

    pub fn set_offset_mode(&mut self, mode: OffsetMode) {
        self.0
            .signature
            .set_field::<{ Self::SIGNATURE_MEM_OFFSET_MODE_MASK }>(mode as u32);
    }

    pub fn reset_offset_mode(&mut self) {
        self.set_offset_mode(OffsetMode::Fixed);
    }

    pub fn is_fixed_offset(&self) -> bool {
        self.offset_mode() == OffsetMode::Fixed
    }

    pub fn is_pre_or_post(&self) -> bool {
        self.offset_mode() != OffsetMode::Fixed
    }

    pub fn is_pre_index(&self) -> bool {
        self.offset_mode() == OffsetMode::PreIndex
    }

    pub fn is_post_index(&self) -> bool {
        self.offset_mode() == OffsetMode::PostIndex
    }

    pub fn make_pre_index(&mut self) {
        self.set_offset_mode(OffsetMode::PreIndex);
    }

    pub fn make_post_index(&mut self) {
        self.set_offset_mode(OffsetMode::PostIndex);
    }

    pub fn shift_op(&self) -> ShiftOp {
        match self
            .0
            .signature
            .get_field::<{ Self::SIGNATURE_MEM_SHIFT_OP_MASK }>()
        {
            1 => ShiftOp::LSR,
            2 => ShiftOp::ASR,
            3 => ShiftOp::ROR,
            4 => ShiftOp::RRX,
            5 => ShiftOp::MSL,
            6 => ShiftOp::UXTB,
            7 => ShiftOp::UXTH,
            8 => ShiftOp::UXTW,
            9 => ShiftOp::UXTX,
            10 => ShiftOp::SXTB,
            11 => ShiftOp::SXTH,
            12 => ShiftOp::SXTW,
            13 => ShiftOp::SXTX,
            _ => ShiftOp::LSL,
        }
    }

    pub fn set_shift_op(&mut self, op: ShiftOp) {
        self.0
            .signature
            .set_field::<{ Self::SIGNATURE_MEM_SHIFT_OP_MASK }>(op as u32);
    }

    pub fn reset_shift_op(&mut self) {
        self.set_shift_op(ShiftOp::LSL);
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

    pub fn set_shift(&mut self, shift: impl Into<Shift>) {
        let shift = shift.into();
        self.set_shift_op(shift.op());
        self.0
            .signature
            .set_field::<{ Self::SIGNATURE_MEM_SHIFT_VALUE_MASK }>(shift.value());
    }

    pub fn reset_shift(&mut self) {
        self.0
            .signature
            .set_field::<{ Self::SIGNATURE_MEM_SHIFT_VALUE_MASK }>(0);
    }
}

pub mod regs {
    use super::*;

    pub const fn w(id: u32) -> Gp {
        Gp::make_r32(id)
    }

    pub const fn gp32(id: u32) -> Gp {
        Gp::make_r32(id)
    }

    pub const fn x(id: u32) -> Gp {
        Gp::make_r64(id)
    }

    pub const fn gp64(id: u32) -> Gp {
        Gp::make_r64(id)
    }

    pub const fn b(id: u32) -> Vec {
        Vec::make_v8(id)
    }

    pub const fn h(id: u32) -> Vec {
        Vec::make_v16(id)
    }

    pub const fn s(id: u32) -> Vec {
        Vec::make_v32(id)
    }

    pub const fn d(id: u32) -> Vec {
        Vec::make_v64(id)
    }

    pub const fn q(id: u32) -> Vec {
        Vec::make_v128(id)
    }

    pub const fn v(id: u32) -> Vec {
        Vec::make_v128(id)
    }

    macro_rules! define_reg_consts {
		($ctor:ident, $ty:ty, { $($name:ident = $id:expr),* $(,)? }) => {
			$(pub const $name: $ty = $ctor($id);)*
		};
	}

    define_reg_consts!(w, Gp, {
        w0 = 0, w1 = 1, w2 = 2, w3 = 3, w4 = 4, w5 = 5, w6 = 6, w7 = 7,
        w8 = 8, w9 = 9, w10 = 10, w11 = 11, w12 = 12, w13 = 13, w14 = 14, w15 = 15,
        w16 = 16, w17 = 17, w18 = 18, w19 = 19, w20 = 20, w21 = 21, w22 = 22, w23 = 23,
        w24 = 24, w25 = 25, w26 = 26, w27 = 27, w28 = 28, w29 = 29, w30 = 30,
        wzr = Gp::ID_ZR, wsp = Gp::ID_SP
    });

    define_reg_consts!(x, Gp, {
        x0 = 0, x1 = 1, x2 = 2, x3 = 3, x4 = 4, x5 = 5, x6 = 6, x7 = 7,
        x8 = 8, x9 = 9, x10 = 10, x11 = 11, x12 = 12, x13 = 13, x14 = 14, x15 = 15,
        x16 = 16, x17 = 17, x18 = 18, x19 = 19, x20 = 20, x21 = 21, x22 = 22, x23 = 23,
        x24 = 24, x25 = 25, x26 = 26, x27 = 27, x28 = 28, x29 = 29, x30 = 30,
        xzr = Gp::ID_ZR, sp = Gp::ID_SP
    });

    pub const wos: Gp = w(Gp::ID_OS);
    pub const wfp: Gp = w(Gp::ID_FP);
    pub const wlr: Gp = w(Gp::ID_LR);
    pub const os: Gp = x(Gp::ID_OS);
    pub const fp: Gp = x(Gp::ID_FP);
    pub const lr: Gp = x(Gp::ID_LR);
    pub const zr: Gp = x(Gp::ID_ZR);

    define_reg_consts!(b, Vec, {
        b0 = 0, b1 = 1, b2 = 2, b3 = 3, b4 = 4, b5 = 5, b6 = 6, b7 = 7,
        b8 = 8, b9 = 9, b10 = 10, b11 = 11, b12 = 12, b13 = 13, b14 = 14, b15 = 15,
        b16 = 16, b17 = 17, b18 = 18, b19 = 19, b20 = 20, b21 = 21, b22 = 22, b23 = 23,
        b24 = 24, b25 = 25, b26 = 26, b27 = 27, b28 = 28, b29 = 29, b30 = 30, b31 = 31
    });

    define_reg_consts!(h, Vec, {
        h0 = 0, h1 = 1, h2 = 2, h3 = 3, h4 = 4, h5 = 5, h6 = 6, h7 = 7,
        h8 = 8, h9 = 9, h10 = 10, h11 = 11, h12 = 12, h13 = 13, h14 = 14, h15 = 15,
        h16 = 16, h17 = 17, h18 = 18, h19 = 19, h20 = 20, h21 = 21, h22 = 22, h23 = 23,
        h24 = 24, h25 = 25, h26 = 26, h27 = 27, h28 = 28, h29 = 29, h30 = 30, h31 = 31
    });

    define_reg_consts!(s, Vec, {
        s0 = 0, s1 = 1, s2 = 2, s3 = 3, s4 = 4, s5 = 5, s6 = 6, s7 = 7,
        s8 = 8, s9 = 9, s10 = 10, s11 = 11, s12 = 12, s13 = 13, s14 = 14, s15 = 15,
        s16 = 16, s17 = 17, s18 = 18, s19 = 19, s20 = 20, s21 = 21, s22 = 22, s23 = 23,
        s24 = 24, s25 = 25, s26 = 26, s27 = 27, s28 = 28, s29 = 29, s30 = 30, s31 = 31
    });

    define_reg_consts!(d, Vec, {
        d0 = 0, d1 = 1, d2 = 2, d3 = 3, d4 = 4, d5 = 5, d6 = 6, d7 = 7,
        d8 = 8, d9 = 9, d10 = 10, d11 = 11, d12 = 12, d13 = 13, d14 = 14, d15 = 15,
        d16 = 16, d17 = 17, d18 = 18, d19 = 19, d20 = 20, d21 = 21, d22 = 22, d23 = 23,
        d24 = 24, d25 = 25, d26 = 26, d27 = 27, d28 = 28, d29 = 29, d30 = 30, d31 = 31
    });

    define_reg_consts!(q, Vec, {
        q0 = 0, q1 = 1, q2 = 2, q3 = 3, q4 = 4, q5 = 5, q6 = 6, q7 = 7,
        q8 = 8, q9 = 9, q10 = 10, q11 = 11, q12 = 12, q13 = 13, q14 = 14, q15 = 15,
        q16 = 16, q17 = 17, q18 = 18, q19 = 19, q20 = 20, q21 = 21, q22 = 22, q23 = 23,
        q24 = 24, q25 = 25, q26 = 26, q27 = 27, q28 = 28, q29 = 29, q30 = 30, q31 = 31
    });

    define_reg_consts!(v, Vec, {
        v0 = 0, v1 = 1, v2 = 2, v3 = 3, v4 = 4, v5 = 5, v6 = 6, v7 = 7,
        v8 = 8, v9 = 9, v10 = 10, v11 = 11, v12 = 12, v13 = 13, v14 = 14, v15 = 15,
        v16 = 16, v17 = 17, v18 = 18, v19 = 19, v20 = 20, v21 = 21, v22 = 22, v23 = 23,
        v24 = 24, v25 = 25, v26 = 26, v27 = 27, v28 = 28, v29 = 29, v30 = 30, v31 = 31
    });
}

pub use regs::*;

pub const fn lsl(value: u32) -> Shift {
    Shift::new(ShiftOp::LSL, value)
}

pub const fn lsr(value: u32) -> Shift {
    Shift::new(ShiftOp::LSR, value)
}

pub const fn asr(value: u32) -> Shift {
    Shift::new(ShiftOp::ASR, value)
}

pub const fn ror(value: u32) -> Shift {
    Shift::new(ShiftOp::ROR, value)
}

pub const fn rrx() -> Shift {
    Shift::new(ShiftOp::RRX, 0)
}

pub const fn msl(value: u32) -> Shift {
    Shift::new(ShiftOp::MSL, value)
}

pub const fn uxtb(value: u32) -> Shift {
    Shift::new(ShiftOp::UXTB, value)
}

pub const fn uxth(value: u32) -> Shift {
    Shift::new(ShiftOp::UXTH, value)
}

pub const fn uxtw(value: u32) -> Shift {
    Shift::new(ShiftOp::UXTW, value)
}

pub const fn uxtx(value: u32) -> Shift {
    Shift::new(ShiftOp::UXTX, value)
}

pub const fn sxtb(value: u32) -> Shift {
    Shift::new(ShiftOp::SXTB, value)
}

pub const fn sxth(value: u32) -> Shift {
    Shift::new(ShiftOp::SXTH, value)
}

pub const fn sxtw(value: u32) -> Shift {
    Shift::new(ShiftOp::SXTW, value)
}

pub const fn sxtx(value: u32) -> Shift {
    Shift::new(ShiftOp::SXTX, value)
}

pub fn ptr(base: Gp, offset: i32) -> Mem {
    Mem::from_base_and_disp(&base.0, offset, OperandSignature::new(0))
}

pub fn ptr_pre(base: Gp, offset: i32) -> Mem {
    Mem::from_base_and_disp(
        &base.0,
        offset,
        OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_OFFSET_MODE_MASK }>(
            OffsetMode::PreIndex as u32,
        ),
    )
}

pub fn ptr_post(base: Gp, offset: i32) -> Mem {
    Mem::from_base_and_disp(
        &base.0,
        offset,
        OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_OFFSET_MODE_MASK }>(
            OffsetMode::PostIndex as u32,
        ),
    )
}

pub fn ptr_reg(base: Gp, index: Gp) -> Mem {
    Mem::from_base_and_index(&base.0, &index.0, OperandSignature::new(0))
}

pub fn ptr_reg_pre(base: Gp, index: Gp) -> Mem {
    Mem::from_base_and_index(
        &base.0,
        &index.0,
        OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_OFFSET_MODE_MASK }>(
            OffsetMode::PreIndex as u32,
        ),
    )
}

pub fn ptr_reg_post(base: Gp, index: Gp) -> Mem {
    Mem::from_base_and_index(
        &base.0,
        &index.0,
        OperandSignature::from_value::<{ Mem::SIGNATURE_MEM_OFFSET_MODE_MASK }>(
            OffsetMode::PostIndex as u32,
        ),
    )
}

pub fn ptr_shift(base: Gp, index: Gp, shift: impl Into<Shift>) -> Mem {
    Mem::from_base_and_index_shift(&base.0, &index.0, shift, OperandSignature::new(0))
}

pub fn label_ptr(base: Label, offset: i32) -> Mem {
    Mem::from_label_and_disp(&base, offset, OperandSignature::new(0))
}

pub fn abs_ptr(base: u64) -> Mem {
    Mem::from_u64(base, OperandSignature::new(0))
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_valid() {
            return write!(f, "reg_invalid");
        }

        if self.is_gp32() || self.is_gp64() {
            write!(f, "{}", Gp::from_type_and_id(self.typ(), self.id()))
        } else if self.is_vec() {
            write!(f, "{}", Vec::from_type_and_id(self.typ(), self.id()))
        } else {
            write!(f, "reg{}", self.id())
        }
    }
}

impl fmt::Display for Gp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_valid() {
            return write!(f, "gp_invalid");
        }

        if self.is_gp32() {
            match self.id() {
                Self::ID_SP => write!(f, "wsp"),
                Self::ID_ZR => write!(f, "wzr"),
                id => write!(f, "w{}", id),
            }
        } else {
            match self.id() {
                Self::ID_SP => write!(f, "sp"),
                Self::ID_ZR => write!(f, "xzr"),
                id => write!(f, "x{}", id),
            }
        }
    }
}

impl fmt::Display for Vec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_valid() {
            return write!(f, "vec_invalid");
        }

        if self.has_element_index() {
            let suffix = match self.element_type() {
                VecElementType::B => "b",
                VecElementType::H => "h",
                VecElementType::S => "s",
                VecElementType::D => "d",
                VecElementType::B4 => "b4",
                VecElementType::H2 => "h2",
                VecElementType::None => "",
            };
            if suffix.is_empty() {
                write!(f, "v{}[{}]", self.id(), self.element_index())
            } else {
                write!(f, "v{}.{}[{}]", self.id(), suffix, self.element_index())
            }
        } else if self.has_element_type() {
            let suffix = match (self.typ(), self.element_type()) {
                (RegType::Vec32, VecElementType::H) => "2h",
                (RegType::Vec64, VecElementType::B) => "8b",
                (RegType::Vec64, VecElementType::H) => "4h",
                (RegType::Vec64, VecElementType::S) => "2s",
                (RegType::Vec64, VecElementType::D) | (RegType::Vec64, VecElementType::None) => {
                    "1d"
                }
                (RegType::Vec128, VecElementType::B) => "16b",
                (RegType::Vec128, VecElementType::H) => "8h",
                (RegType::Vec128, VecElementType::S) => "4s",
                (RegType::Vec128, VecElementType::D) => "2d",
                (RegType::Vec128, VecElementType::B4) => "4b",
                (RegType::Vec128, VecElementType::H2) => "2h",
                _ => "",
            };

            if suffix.is_empty() {
                write!(f, "v{}", self.id())
            } else {
                write!(f, "v{}.{}", self.id(), suffix)
            }
        } else {
            match self.typ() {
                RegType::Vec8 => write!(f, "b{}", self.id()),
                RegType::Vec16 => write!(f, "h{}", self.id()),
                RegType::Vec32 => write!(f, "s{}", self.id()),
                RegType::Vec64 => write!(f, "d{}", self.id()),
                RegType::Vec128 => write!(f, "v{}", self.id()),
                _ => write!(f, "vec{}", self.id()),
            }
        }
    }
}

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self.op() {
            ShiftOp::LSL => "lsl",
            ShiftOp::LSR => "lsr",
            ShiftOp::ASR => "asr",
            ShiftOp::ROR => "ror",
            ShiftOp::RRX => "rrx",
            ShiftOp::MSL => "msl",
            ShiftOp::UXTB => "uxtb",
            ShiftOp::UXTH => "uxth",
            ShiftOp::UXTW => "uxtw",
            ShiftOp::UXTX => "uxtx",
            ShiftOp::SXTB => "sxtb",
            ShiftOp::SXTH => "sxth",
            ShiftOp::SXTW => "sxtw",
            ShiftOp::SXTX => "sxtx",
        };

        if matches!(self.op(), ShiftOp::RRX) {
            write!(f, "{}", op)
        } else {
            write!(f, "{} #{}", op, self.value())
        }
    }
}

impl fmt::Display for OffsetMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OffsetMode::Fixed => write!(f, "fixed"),
            OffsetMode::PreIndex => write!(f, "pre-index"),
            OffsetMode::PostIndex => write!(f, "post-index"),
        }
    }
}

impl fmt::Display for Mem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.has_base() && !self.has_index() {
            let addr = ((self.base_id() as u64) << 32) | (self.offset() as u32 as u64);
            return write!(f, "[0x{addr:x}]");
        }

        let has_offset = self.has_offset() && self.offset() != 0;
        let has_index = self.has_index() && self.has_index_reg();

        match self.offset_mode() {
            OffsetMode::PostIndex => {
                write!(f, "[")?;
                if self.has_base_label() {
                    write!(f, "label{}", self.base_id())?;
                } else {
                    write!(f, "{}", self.base_reg())?;
                }
                write!(f, "]")?;

                if has_index {
                    write!(f, ", {}", self.index_reg())?;
                    if self.has_shift() {
                        write!(f, ", {}", Shift::new(self.shift_op(), self.shift()))?;
                    }
                } else if has_offset {
                    write!(f, ", #{}", self.offset())?;
                }

                Ok(())
            }
            OffsetMode::Fixed | OffsetMode::PreIndex => {
                write!(f, "[")?;
                if self.has_base_label() {
                    write!(f, "label{}", self.base_id())?;
                } else {
                    write!(f, "{}", self.base_reg())?;
                }

                if has_index {
                    write!(f, ", {}", self.index_reg())?;
                    if self.has_shift() {
                        write!(f, ", {}", Shift::new(self.shift_op(), self.shift()))?;
                    }
                } else if has_offset {
                    write!(f, ", #{}", self.offset())?;
                }

                write!(f, "]")?;

                if self.is_pre_index() {
                    write!(f, "!")?;
                }

                Ok(())
            }
        }
    }
}
