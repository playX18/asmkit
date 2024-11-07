use derive_more::derive::{BitAnd, BitOr, BitXor, Deref, DerefMut, TryFrom};
use num_traits::{FromPrimitive, ToPrimitive};

use super::{globals::INVALID_ID, support::bitmask_from_bool, types::TypeId};
#[macro_export]
macro_rules! define_operand_cast {
    ($t: ty, $base: ty) => {
        impl OperandCast for $t {
            fn as_operand(&self) -> &Operand {
                (**self).as_operand()
            }

            fn from_operand(op: &Operand) -> Self {
                Self(<$base>::from_operand(op))
            }
        }
    };
}

/// Operand type used by [Operand_]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, derive_more::TryFrom)]
#[repr(u32)]
#[try_from(repr)]
pub enum OperandType {
    /// Not an operand or not initialized.
    None = 0,
    /// Operand is a register
    Reg = 1,
    /// Operand is a memory
    Mem = 2,
    /// Operand is a register-list
    RegList = 3,
    /// Operand is an immediate value
    Imm = 4,
    /// Operand is a label
    Label = 5,
    /// Operand is a symbol
    Sym,
}

/// Register mask is a convenience typedef that describes a mask where each bit describes a physical register id
/// in the same [RegGroup]. At the moment 32 bits are enough as AsmJit doesn't support any architecture that
/// would provide more than 32 registers for a register group.
pub type RegMask = u32;

/// Register type.
///
/// Provides a unique type that can be used to identify a register or its view.
#[derive(TryFrom, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
#[try_from(repr)]
pub enum RegType {
    /// No register - unused, invalid, multiple meanings.
    None,

    /// Label - local to current assembler label
    LabelTag,
    /// Symbol - global symbol, produces relocation
    SymTag,
    /// Program counter (PC) register.
    PC,

    /// 8-bit general-purpose register (low part).
    Gp8Lo,

    /// 8-bit general-purpose register (high part).
    Gp8Hi,

    /// General-purpose word register.
    Gp16,

    /// General-purpose doubleword register.
    Gp32,

    /// General-purpose quadword register (64-bit).
    Gp64,

    /// 128-bit vector register (SSE+).
    Vec8,

    /// 128-bit vector register (high part, SSE+).
    Vec16,

    /// 128-bit vector register (SSE+).
    Vec32,

    /// 256-bit vector register (AVX+).
    Vec64,

    /// 256-bit vector register (AVX+).
    Vec128,

    /// 512-bit vector register (AVX512+).
    Vec256,

    /// 1024-bit vector register (AVX512+).
    Vec512,

    /// 128-bit vector register with variable length (AVX512+).
    VecNLen,

    /// Mask register (AVX512+).
    Mask,

    /// Extra registers.
    Extra,

    X86SReg,
    X86CReg,
    X86DReg,
    X86St,
    X86Bnd,
    X86Tmm,
    MaxValue = 31,
}

#[allow(non_upper_case_globals)]
impl RegType {
    pub const X86Mm: Self = Self::Extra;
    pub const X86Rip: Self = Self::PC;
    pub const X86GpbLo: Self = Self::Gp8Lo;
    pub const X86GpbHi: Self = Self::Gp8Hi;
    pub const X86Gpw: Self = Self::Gp16;
    pub const X86Gpd: Self = Self::Gp32;
    pub const X86Gpq: Self = Self::Gp64;
    pub const X86Xmm: Self = Self::Vec128;
    pub const X86Ymm: Self = Self::Vec256;
    pub const X86Zmm: Self = Self::Vec512;
    pub const X86KReg: Self = Self::Mask;

    pub const RISCVPC: Self = Self::PC;
    pub const RISCV64Gp: Self = Self::Gp64;
    pub const RISCV32Gp: Self = Self::Gp32;
    pub const RISCVFp: Self = Self::Vec64;
    pub const RISCVVec: Self = Self::VecNLen;
}

#[derive(TryFrom, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
#[try_from(repr)]
pub enum RegGroup {
    Gp = 0,
    Vec,
    Mask,
    ExtraVirt3,
    PC,
    X86SReg,
    X86CReg,
    X86DReg,
    X86St,
    X86Bnd,
    X86Tmm,
}

impl RegGroup {
    pub const X86K: Self = Self::Mask;
    pub const X86MM: Self = Self::ExtraVirt3;
}

#[allow(non_upper_case_globals)]
impl RegGroup {
    pub const X86Rip: Self = Self::PC;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitOr, BitAnd, BitXor)]
pub struct OperandSignature {
    pub bits: u32,
}

impl From<u32> for OperandSignature {
    fn from(value: u32) -> Self {
        Self { bits: value }
    }
}

impl OperandSignature {
    /// Shift value for the operand type.
    pub const OP_TYPE_SHIFT: u32 = 0;
    /// Mask for the operand type.
    pub const OP_TYPE_MASK: u32 = 0x07u32 << Self::OP_TYPE_SHIFT;

    /// Shift value for the register type.
    pub const REG_TYPE_SHIFT: u32 = 3;
    /// Mask for the register type.
    pub const REG_TYPE_MASK: u32 = 0x1Fu32 << Self::REG_TYPE_SHIFT;

    /// Shift value for the register group.
    pub const REG_GROUP_SHIFT: u32 = 8;
    /// Mask for the register group.
    pub const REG_GROUP_MASK: u32 = 0x0Fu32 << Self::REG_GROUP_SHIFT;

    /// Shift value for the memory base type.
    pub const MEM_BASE_TYPE_SHIFT: u32 = 3;
    /// Mask for the memory base type.
    pub const MEM_BASE_TYPE_MASK: u32 = 0x1Fu32 << Self::MEM_BASE_TYPE_SHIFT;

    /// Shift value for the memory index type.
    pub const MEM_INDEX_TYPE_SHIFT: u32 = 8;
    /// Mask for the memory index type.
    pub const MEM_INDEX_TYPE_MASK: u32 = 0x1Fu32 << Self::MEM_INDEX_TYPE_SHIFT;

    /// Shift value for the memory base+index combined.
    pub const MEM_BASE_INDEX_SHIFT: u32 = 3;
    /// Mask for the memory base+index combined.
    pub const MEM_BASE_INDEX_MASK: u32 = 0x3FFu32 << Self::MEM_BASE_INDEX_SHIFT;

    pub const MEM_REG_HOME_SHIFT: u32 = 13;
    pub const MEM_REG_HOME_FLAG: u32 = 0x01 << Self::MEM_REG_HOME_SHIFT;

    /// Shift value for the predicate used by either registers or immediate values.
    pub const PREDICATE_SHIFT: u32 = 20;
    /// Mask for the predicate used by either registers or immediate values.
    pub const PREDICATE_MASK: u32 = 0x0Fu32 << Self::PREDICATE_SHIFT;

    /// Shift value for the operand size.
    pub const SIZE_SHIFT: u32 = 24;
    /// Mask for the operand size.
    pub const SIZE_MASK: u32 = 0xFFu32 << Self::SIZE_SHIFT;

    pub const fn new(bits: u32) -> Self {
        Self { bits }
    }

    pub const fn subset(&self, mask: u32) -> Self {
        Self {
            bits: self.bits & mask,
        }
    }
}

impl OperandSignature {
    pub fn reset(&mut self) -> () {
        self.bits = 0;
    }

    pub const fn bits(&self) -> u32 {
        self.bits
    }

    pub fn set_bits(&mut self, bits: u32) -> () {
        self.bits = bits;
    }

    pub const fn has_field<const K_FIELD_MASK: u32>(&self) -> bool {
        (self.bits & K_FIELD_MASK) != 0
    }

    pub const fn has_value<const K_FIELD_MASK: u32>(&self, value: u32) -> bool {
        (self.bits & K_FIELD_MASK) != value << K_FIELD_MASK.trailing_zeros()
    }

    pub const fn from_bits(bits: u32) -> Self {
        OperandSignature { bits }
    }

    pub const fn from_value<const K_FIELD_MASK: u32>(value: u32) -> Self {
        OperandSignature {
            bits: value << K_FIELD_MASK.trailing_zeros(),
        }
    }

    pub const fn from_op_type(op_type: OperandType) -> Self {
        OperandSignature {
            bits: (op_type as u32) << Self::OP_TYPE_SHIFT,
        }
    }

    pub const fn from_reg_type(reg_type: RegType) -> Self {
        OperandSignature {
            bits: (reg_type as u32) << Self::REG_TYPE_SHIFT,
        }
    }

    pub const fn from_reg_group(reg_group: RegGroup) -> Self {
        OperandSignature {
            bits: (reg_group as u32) << Self::REG_GROUP_SHIFT,
        }
    }

    pub const fn from_mem_base_type(base_type: RegType) -> Self {
        OperandSignature {
            bits: (base_type as u32) << Self::MEM_BASE_TYPE_SHIFT,
        }
    }

    pub const fn from_mem_index_type(index_type: RegType) -> Self {
        OperandSignature {
            bits: (index_type as u32) << Self::MEM_INDEX_TYPE_SHIFT,
        }
    }

    pub const fn from_predicate(predicate: u32) -> Self {
        OperandSignature {
            bits: predicate << Self::PREDICATE_SHIFT,
        }
    }

    pub const fn from_size(size: u32) -> Self {
        OperandSignature {
            bits: size << Self::SIZE_SHIFT,
        }
    }

    pub fn set_field<const K_FIELD_MASK: u32>(&mut self, value: u32) -> () {
        self.bits = (self.bits & !K_FIELD_MASK) | (value << K_FIELD_MASK.trailing_zeros());
    }

    pub const fn get_field<const K_FIELD_MASK: u32>(&self) -> u32 {
        (self.bits >> K_FIELD_MASK.trailing_zeros())
            & (K_FIELD_MASK >> K_FIELD_MASK.trailing_zeros())
    }

    pub const fn is_valid(&self) -> bool {
        self.bits != 0
    }

    pub fn op_type(&self) -> OperandType {
        OperandType::try_from(self.get_field::<{ Self::OP_TYPE_MASK }>()).unwrap()
    }

    pub fn reg_type(&self) -> RegType {
        RegType::try_from(self.get_field::<{ Self::REG_TYPE_MASK }>()).unwrap()
    }

    pub fn reg_group(&self) -> RegGroup {
        RegGroup::try_from(self.get_field::<{ Self::REG_GROUP_MASK }>()).unwrap()
    }

    pub fn mem_base_type(&self) -> RegType {
        RegType::try_from(self.get_field::<{ Self::MEM_BASE_TYPE_MASK }>()).unwrap()
    }

    pub fn mem_index_type(&self) -> RegType {
        RegType::try_from(self.get_field::<{ Self::MEM_INDEX_TYPE_MASK }>()).unwrap()
    }

    pub fn predicate(&self) -> u32 {
        self.get_field::<{ Self::PREDICATE_MASK }>()
    }

    pub fn size(&self) -> u32 {
        self.get_field::<{ Self::SIZE_MASK }>()
    }

    pub fn set_op_type(&mut self, op_type: OperandType) {
        self.set_field::<{ Self::OP_TYPE_MASK }>(op_type as _);
    }

    pub fn set_reg_type(&mut self, reg_type: RegType) {
        self.set_field::<{ Self::REG_TYPE_MASK }>(reg_type as _);
    }

    pub fn set_reg_group(&mut self, reg_group: RegGroup) {
        self.set_field::<{ Self::REG_GROUP_MASK }>(reg_group as _);
    }

    pub fn set_mem_base_type(&mut self, base_type: RegType) {
        self.set_field::<{ Self::MEM_BASE_TYPE_MASK }>(base_type as _);
    }

    pub fn set_mem_index_type(&mut self, index_type: RegType) {
        self.set_field::<{ Self::MEM_INDEX_TYPE_MASK }>(index_type as _);
    }

    pub fn set_predicate(&mut self, predicate: u32) {
        self.set_field::<{ Self::PREDICATE_MASK }>(predicate);
    }

    pub fn set_size(&mut self, size: u32) {
        self.set_field::<{ Self::SIZE_MASK }>(size);
    }
}

pub const DATA_MEM_INDEX_ID: usize = 0;
pub const DATA_MEM_OFFSET_LO: usize = 1;
pub const DATA_IMM_VALUE_LO: usize = if cfg!(target_endian = "little") { 0 } else { 1 };
pub const DATA_IMM_VALUE_HI: usize = if DATA_IMM_VALUE_LO == 0 { 1 } else { 0 };

pub const VIRT_ID_MIN: u32 = 400;
pub const VIRT_ID_MAX: u32 = u32::MAX - 1;
pub const VIRT_ID_COUNT: u32 = VIRT_ID_MAX - VIRT_ID_MIN + 1;

/// Base struct representing an operand in AsmJit (non-default constructed version).
///
/// This struct is used internally and all types that are valid operands downscat to
/// this type eventually and it's also possible to "upcast" it to a operand type e.g `Gp`.
///
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Operand {
    pub signature: OperandSignature,
    pub base_id: u32,
    pub data: [u32; 2],
}

pub const fn is_virt_id(id: u32) -> bool {
    id - VIRT_ID_MIN < VIRT_ID_COUNT
}

pub const fn index_to_virt_id(id: u32) -> u32 {
    id + VIRT_ID_MIN
}

pub const fn virt_id_to_index(id: u32) -> u32 {
    id - VIRT_ID_MIN
}

impl Operand {
    pub const fn new() -> Self {
        Self {
            base_id: INVALID_ID,
            signature: OperandSignature::new(0),
            data: [0; 2],
        }
    }

    pub const fn make_reg(sig: OperandSignature, id: u32) -> Self {
        Self {
            base_id: id,
            signature: sig,
            data: [0; 2],
        }
    }

    pub fn reset(&mut self) {
        self.signature.reset();
        self.base_id = 0;
        self.data = [0; 2]
    }

    pub fn has_signature(&self, other: OperandSignature) -> bool {
        self.signature == other
    }

    pub const fn signature(&self) -> OperandSignature {
        self.signature
    }

    pub fn set_signature(&mut self, sig: OperandSignature) {
        self.signature = sig;
    }

    pub fn set_id(&mut self, id: u32) {
        self.base_id = id;
    }

    pub fn op_type(&self) -> OperandType {
        self.signature.op_type()
    }

    pub const fn is_none(&self) -> bool {
        self.signature.bits == 0
    }

    pub fn is_reg(&self) -> bool {
        self.op_type() == OperandType::Reg
    }

    pub fn is_reg_list(&self) -> bool {
        self.op_type() == OperandType::RegList
    }

    pub fn is_mem(&self) -> bool {
        self.op_type() == OperandType::Mem
    }

    pub fn is_imm(&self) -> bool {
        self.op_type() == OperandType::Imm
    }

    pub fn is_label(&self) -> bool {
        self.op_type() == OperandType::Label
    }

    pub fn is_sym(&self) -> bool {
        self.op_type() == OperandType::Sym
    }

    pub fn is_phys_reg(&self) -> bool {
        self.is_reg() && self.base_id < 0xff
    }

    pub fn is_virt_reg(&self) -> bool {
        self.is_reg() && self.base_id > 0xff
    }

    pub const fn id(&self) -> u32 {
        self.base_id
    }

    pub fn is_reg_type_of(&self, typ: RegType) -> bool {
        self.signature
            .subset(OperandSignature::OP_TYPE_MASK | OperandSignature::REG_TYPE_MASK)
            == OperandSignature::from_reg_type(typ)
    }

    pub fn is_reg_group_of(&self, group: RegGroup) -> bool {
        self.signature
            .subset(OperandSignature::OP_TYPE_MASK | OperandSignature::REG_GROUP_MASK)
            == OperandSignature::from_reg_group(group)
    }

    pub fn is_gp(&self) -> bool {
        self.is_reg_group_of(RegGroup::Gp)
    }

    pub fn is_gp32(&self) -> bool {
        self.is_reg_type_of(RegType::Gp32)
    }

    pub fn is_gp64(&self) -> bool {
        self.is_reg_type_of(RegType::Gp64)
    }

    pub fn is_vec(&self) -> bool {
        self.is_reg_group_of(RegGroup::Vec)
    }

    pub fn is_vec8(&self) -> bool {
        self.is_reg_type_of(RegType::Vec8)
    }

    pub fn is_vec16(&self) -> bool {
        self.is_reg_type_of(RegType::Vec16)
    }

    pub fn is_vec32(&self) -> bool {
        self.is_reg_type_of(RegType::Vec32)
    }

    pub fn is_vec64(&self) -> bool {
        self.is_reg_type_of(RegType::Vec64)
    }

    pub fn is_vec128(&self) -> bool {
        self.is_reg_type_of(RegType::Vec128)
    }

    pub fn is_vec256(&self) -> bool {
        self.is_reg_type_of(RegType::Vec256)
    }

    pub fn is_vec512(&self) -> bool {
        self.is_reg_type_of(RegType::Vec512)
    }

    pub fn is_mask(&self) -> bool {
        self.is_reg_group_of(RegGroup::Mask)
    }

    pub fn is_reg_list_of(&self, typ: RegType) -> bool {
        self.signature
            .subset(OperandSignature::OP_TYPE_MASK | OperandSignature::REG_TYPE_MASK)
            == OperandSignature::from_reg_type(typ)
    }

    pub fn is_reg_or_mem(&self) -> bool {
        self.op_type() >= OperandType::Reg && self.op_type() <= OperandType::Mem
    }

    pub fn is_reg_or_reg_list_or_mem(&self) -> bool {
        self.op_type() >= OperandType::RegList && self.op_type() <= OperandType::RegList
    }

    pub fn x86_rm_size(&self) -> u32 {
        self.signature.size()
    }
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, Eq)]
pub struct Label(pub Operand);

impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

impl Ord for Label {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id().cmp(&other.id())
    }
}

define_operand_cast!(Label, Operand);

impl Label {
    pub const fn new() -> Self {
        Self(Operand {
            signature: OperandSignature::from_op_type(OperandType::Label),
            base_id: INVALID_ID,
            data: [0; 2],
        })
    }

    pub const fn from_id(id: u32) -> Self {
        Self(Operand {
            signature: OperandSignature::from_op_type(OperandType::Label),
            base_id: id,
            data: [0; 2],
        })
    }

    pub const fn is_valid(&self) -> bool {
        self.0.base_id != INVALID_ID
    }

    pub fn set_id(&mut self, id: u32) {
        self.base_id = id;
    }
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, Eq)]
pub struct Sym(pub Operand);

impl PartialOrd for Sym {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

impl Ord for Sym {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id().cmp(&other.id())
    }
}

define_operand_cast!(Sym, Operand);

impl Sym {
    pub const fn new() -> Self {
        Self(Operand {
            signature: OperandSignature::from_op_type(OperandType::Sym),
            base_id: INVALID_ID,
            data: [0; 2],
        })
    }

    pub const fn from_id(id: u32) -> Self {
        Self(Operand {
            signature: OperandSignature::from_op_type(OperandType::Label),
            base_id: id,
            data: [0; 2],
        })
    }

    pub const fn is_valid(&self) -> bool {
        self.0.base_id != INVALID_ID
    }

    pub fn set_id(&mut self, id: u32) {
        self.base_id = id;
    }
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BaseReg(pub Operand);

pub const REG_SIGNATURE: OperandSignature = OperandSignature::from_op_type(OperandType::Reg);
pub const REG_TYPE_NONE: u32 = RegType::None as u32;
pub const REG_BASE_SIGNATURE_MASK: u32 = OperandSignature::OP_TYPE_MASK
    | OperandSignature::REG_TYPE_MASK
    | OperandSignature::REG_GROUP_MASK
    | OperandSignature::SIZE_MASK;

impl BaseReg {
    pub const SIGNATURE: u32 = REG_BASE_SIGNATURE_MASK;

    pub const fn new() -> Self {
        Self(Operand {
            signature: OperandSignature::from_op_type(OperandType::Reg),
            base_id: 0xff,
            data: [0; 2],
        })
    }

    pub const fn from_signature_and_id(signature: OperandSignature, id: u32) -> Self {
        Self(Operand {
            signature,
            base_id: id,
            data: [0; 2],
        })
    }

    pub const fn base_signature(&self) -> OperandSignature {
        OperandSignature {
            bits: self.0.signature.bits & REG_BASE_SIGNATURE_MASK,
        }
    }

    pub fn has_base_signature(&self, signature: impl Into<OperandSignature>) -> bool {
        self.base_signature().bits == signature.into().bits
    }

    pub fn is_valid(&self) -> bool {
        self.signature().is_valid() && self.base_id != 0xff
    }

    pub fn is_phys_reg(&self) -> bool {
        self.base_id < 0xff
    }

    pub fn is_virt_reg(&self) -> bool {
        self.base_id > 0xff
    }

    pub fn is_type(&self, typ: RegType) -> bool {
        self.signature.subset(OperandSignature::REG_TYPE_MASK)
            == OperandSignature::from_reg_type(typ)
    }

    pub fn is_group(&self, group: RegGroup) -> bool {
        self.signature.subset(OperandSignature::REG_GROUP_MASK)
            == OperandSignature::from_reg_group(group)
    }

    pub fn is_gp(&self) -> bool {
        self.is_group(RegGroup::Gp)
    }

    pub fn is_vec(&self) -> bool {
        self.is_group(RegGroup::Vec)
    }

    pub fn is_mask(&self) -> bool {
        self.is_group(RegGroup::Mask)
    }

    pub fn operand_is_gp(op: &Operand) -> bool {
        op.signature()
            .subset(OperandSignature::OP_TYPE_MASK | OperandSignature::REG_GROUP_MASK)
            == (OperandSignature::from_op_type(OperandType::Reg)
                | OperandSignature::from_reg_group(RegGroup::Gp))
    }

    pub fn operand_is_vec(op: &Operand) -> bool {
        op.signature()
            .subset(OperandSignature::OP_TYPE_MASK | OperandSignature::REG_GROUP_MASK)
            == (OperandSignature::from_op_type(OperandType::Reg)
                | OperandSignature::from_reg_group(RegGroup::Vec))
    }

    pub fn operand_is_gp_with_id(op: &Operand, id: u32) -> bool {
        Self::operand_is_gp(op) && op.id() == id
    }

    pub fn is_vec_with_id(op: &Operand, id: u32) -> bool {
        Self::operand_is_vec(op) && op.id() == id
    }

    pub fn is_reg_of_type(&self, typ: RegType) -> bool {
        self.is_type(typ)
    }

    pub fn is_reg_of_type_and_id(&self, typ: RegType, id: u32) -> bool {
        self.is_type(typ) && self.base_id == id
    }

    pub fn typ(&self) -> RegType {
        self.signature.reg_type()
    }

    pub fn group(&self) -> RegGroup {
        self.signature.reg_group()
    }

    pub fn has_size(&self) -> bool {
        self.signature
            .has_field::<{ OperandSignature::SIZE_MASK }>()
    }

    pub fn size(&self) -> u32 {
        self.signature.size()
    }

    pub fn predicate(&self) -> u32 {
        self.signature.predicate()
    }

    pub fn set_predicate(&mut self, predicate: u32) {
        self.signature
            .set_field::<{ OperandSignature::PREDICATE_MASK }>(predicate);
    }

    pub fn reset_predicate(&mut self) {
        self.set_predicate(0);
    }
}

pub trait RegTraits {
    const VALID: u32 = 1;
    const TYPE: RegType;
    const GROUP: RegGroup;
    const SIZE: u32;
    const TYPE_ID: TypeId;

    const SIGNATURE: u32 = OperandSignature::from_op_type(OperandType::Reg).bits
        | OperandSignature::from_reg_type(Self::TYPE).bits
        | OperandSignature::from_reg_group(Self::GROUP).bits
        | OperandSignature::from_size(Self::SIZE).bits;
}

#[macro_export]
macro_rules! define_abstract_reg {
    ($reg: ty, $base: ty) => {
        impl $reg {
            pub const fn new() -> Self {
                Self(<$base>::from_signature_and_id(
                    OperandSignature {
                        bits: <$base>::SIGNATURE,
                    },
                    0xff,
                ))
            }

            pub const fn from_signature_and_id(signature: OperandSignature, id: u32) -> Self {
                Self(<$base>::from_signature_and_id(signature, id))
            }

            pub const fn from_type_and_id(typ: RegType, id: u32) -> Self {
                Self::from_signature_and_id(Self::signature_of(typ), id)
            }
        }

        define_operand_cast!($reg, $base);
    };
}

#[macro_export]
macro_rules! define_final_reg {
    ($reg: ty, $base: ty, $traits: ty) => {
        define_abstract_reg!($reg, $base);
        impl $reg {
            pub const THIS_TYPE: RegType = <$traits>::TYPE;
            pub const THIS_GROUP: RegGroup = <$traits>::GROUP;
            pub const THIS_SIZE: u32 = <$traits>::SIZE;
            pub const SIGNATURE: u32 = <$traits>::SIGNATURE;

            pub const fn from_id(id: u32) -> Self {
                Self(<$base>::from_signature_and_id(
                    OperandSignature::new(Self::SIGNATURE),
                    id,
                ))
            }
        }
    };
}

#[macro_export]
macro_rules! define_reg_traits {
    ($reg_type: ident, $group: path, $size: expr, $type_id: path) => {
        pub struct $reg_type;

        impl RegTraits for $reg_type {
            const TYPE: RegType = RegType::$reg_type;
            const GROUP: RegGroup = $group;
            const SIZE: u32 = $size;
            const TYPE_ID: TypeId = $type_id;
        }
    };
}

/// A helper trait to help cast [Operand] to Architecture dependent
/// operands and vice-versa.
pub trait OperandCast {
    fn as_operand(&self) -> &Operand;
    fn from_operand(op: &Operand) -> Self;
}

impl OperandCast for Operand {
    fn as_operand(&self) -> &Operand {
        self
    }

    fn from_operand(op: &Operand) -> Self {
        *op
    }
}

define_operand_cast!(BaseReg, Operand);

impl Operand {
    pub fn as_<T>(&self) -> T
    where
        T: OperandCast,
    {
        T::from_operand(self)
    }
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BaseMem(pub Operand);

define_operand_cast!(BaseMem, Operand);

impl BaseMem {
    pub fn from_base_disp(base_reg: impl AsRef<BaseReg>, offset: i32) -> Self {
        let base = base_reg.as_ref();
        Self(Operand {
            signature: OperandSignature::from_op_type(OperandType::Mem)
                | OperandSignature::from_mem_base_type(base.typ()),
            base_id: base.id(),
            data: [0, offset as _],
        })
    }

    pub fn from_base_and_index_disp(
        u0: OperandSignature,
        base_id: u32,
        index_id: u32,
        offset: i32,
    ) -> Self {
        Self(Operand {
            signature: u0,
            base_id,
            data: [index_id, offset as _],
        })
    }

    pub fn reset(&mut self) {
        self.signature = OperandSignature::from_op_type(OperandType::Mem);
        self.base_id = 0;
        self.data = [0; 2]
    }

    pub const fn new() -> Self {
        Self(Operand {
            signature: OperandSignature::from_op_type(OperandType::Mem),
            base_id: 0,
            data: [0; 2],
        })
    }

    pub fn is_reg_home(&self) -> bool {
        self.signature
            .has_field::<{ OperandSignature::MEM_REG_HOME_FLAG }>()
    }

    pub fn set_reg_home(&mut self) {
        self.signature.bits |= OperandSignature::MEM_REG_HOME_FLAG;
    }

    pub fn clear_reg_home(&mut self) {
        self.signature.bits &= !OperandSignature::MEM_REG_HOME_FLAG;
    }

    pub fn has_base(&self) -> bool {
        self.signature.bits & OperandSignature::MEM_BASE_TYPE_MASK != 0
    }

    pub fn has_index(&self) -> bool {
        self.signature.bits & OperandSignature::MEM_INDEX_TYPE_MASK != 0
    }

    pub fn has_base_or_index(&self) -> bool {
        self.has_base() || self.has_index()
    }

    pub fn has_base_and_index(&self) -> bool {
        self.has_base() && self.has_index()
    }

    pub fn has_base_label(&self) -> bool {
        self.signature.subset(OperandSignature::MEM_BASE_TYPE_MASK)
            == OperandSignature::from_reg_type(RegType::LabelTag)
    }

    pub fn has_base_sym(&self) -> bool {
        self.signature.subset(OperandSignature::MEM_BASE_TYPE_MASK)
            == OperandSignature::from_reg_type(RegType::SymTag)
    }

    pub fn has_base_reg(&self) -> bool {
        self.signature.subset(OperandSignature::MEM_BASE_TYPE_MASK)
            > OperandSignature::from_reg_type(RegType::SymTag)
    }

    pub fn has_index_reg(&self) -> bool {
        self.signature.subset(OperandSignature::MEM_INDEX_TYPE_MASK)
            > OperandSignature::from_reg_type(RegType::SymTag)
    }

    pub fn base_type(&self) -> RegType {
        self.signature.mem_base_type()
    }

    pub fn index_type(&self) -> RegType {
        self.signature.mem_index_type()
    }

    pub fn base_id(&self) -> u32 {
        self.base_id
    }

    pub fn index_id(&self) -> u32 {
        self.data[DATA_MEM_INDEX_ID]
    }

    pub fn set_base_type(&mut self, base_type: RegType) {
        self.signature.set_mem_base_type(base_type);
    }

    pub fn set_index_type(&mut self, index_type: RegType) {
        self.signature.set_mem_index_type(index_type);
    }

    pub fn set_base_id(&mut self, id: u32) {
        self.base_id = id;
    }

    pub fn set_index_id(&mut self, id: u32) {
        self.data[DATA_MEM_INDEX_ID] = id;
    }

    pub fn set_base(&mut self, base: &BaseReg) {
        let base_reg = base;
        self.set_base_type(base_reg.typ());
        self.set_base_id(base_reg.id());
    }

    pub fn set_index(&mut self, index: &BaseReg) {
        let index_reg = index;
        self.set_index_type(index_reg.typ());
        self.set_index_id(index_reg.id());
    }
    pub fn reset_base(&mut self) {
        self.signature.bits &= !OperandSignature::MEM_BASE_TYPE_MASK;
        self.base_id = 0;
    }

    pub fn reset_index(&mut self) {
        self.signature.bits &= !OperandSignature::MEM_INDEX_TYPE_MASK;
        self.data[DATA_MEM_INDEX_ID] = 0;
    }

    pub fn is_offset_64bit(&self) -> bool {
        // If this is true then `hasBase()` must always report false.
        self.base_type() == RegType::None
    }

    pub fn has_offset(&self) -> bool {
        (self.data[DATA_MEM_OFFSET_LO] | (self.base_id & bitmask_from_bool(self.is_offset_64bit())))
            != 0
    }

    pub fn offset(&self) -> i64 {
        if self.is_offset_64bit() {
            (self.data[DATA_MEM_OFFSET_LO] as u64 | (self.base_id as u64) << 32) as i64
        } else {
            self.data[DATA_MEM_OFFSET_LO] as i32 as i64
        }
    }
    pub fn offset_lo32(&self) -> i32 {
        self.data[DATA_MEM_OFFSET_LO] as i32
    }

    pub fn offset_hi32(&self) -> i32 {
        if self.is_offset_64bit() {
            self.base_id as i32
        } else {
            0
        }
    }
    pub fn set_offset(&mut self, offset: i64) {
        let lo = (offset as u64 & 0xFFFFFFFF) as u32;
        let hi = (offset as u64 >> 32) as u32;
        let hi_msk = bitmask_from_bool(self.is_offset_64bit());

        self.data[DATA_MEM_OFFSET_LO] = lo;
        self.base_id = (hi & hi_msk) | (self.base_id & !hi_msk);
    }
    pub fn set_offset_lo32(&mut self, offset: i32) {
        self.data[DATA_MEM_OFFSET_LO] = offset as u32;
    }

    pub fn add_offset(&mut self, offset: i64) {
        if self.is_offset_64bit() {
            self.set_offset(self.offset() + offset);
        } else {
            self.set_offset_lo32(self.offset_lo32() + offset as i32);
        }
    }

    pub fn add_offset_lo32(&mut self, offset: i32) {
        self.set_offset_lo32(self.offset_lo32() + offset);
    }

    pub fn reset_offset(&mut self) {
        self.set_offset(0);
    }

    pub fn reset_offset_lo32(&mut self) {
        self.set_offset_lo32(0);
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ImmType {
    Int = 0,
    Double = 1,
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Imm(pub Operand);

define_operand_cast!(Imm, Operand);

impl Imm {
    pub const fn new() -> Self {
        Self(Operand {
            signature: OperandSignature::from_op_type(OperandType::Imm),
            base_id: 0,
            data: [0; 2],
        })
    }
    /*
    pub fn from_shift(shift: arm::Shift) -> Self {
        Self(Operand {
            signature: OperandSignature::from_op_type(OperandType::Imm)
                | OperandSignature::from_predicate(shift.op() as u32),
            base_id: 0,
            data: [shift.value() as u32, (shift.value() >> 32) as u32],
        })
    }*/

    pub fn from_value<T: Into<i64>>(val: T, predicate: u32) -> Self {
        let value = val.into();
        Self(Operand {
            signature: OperandSignature::from_op_type(OperandType::Imm)
                | OperandSignature::from_predicate(predicate),
            base_id: 0,
            data: [value as u32, (value >> 32) as u32],
        })
    }

    pub fn from_float(val: f32, predicate: u32) -> Self {
        let mut imm = Self::new();
        imm.set_value_float(val);
        imm.set_predicate(predicate);
        imm
    }

    pub fn from_double(val: f64, predicate: u32) -> Self {
        let mut imm = Self::new();
        imm.set_value_float(val);
        imm.set_predicate(predicate);
        imm
    }

    pub fn typ(&self) -> ImmType {
        if self
            .signature()
            .get_field::<{ OperandSignature::PREDICATE_MASK }>()
            == 0
        {
            ImmType::Int
        } else {
            ImmType::Double
        }
    }

    pub fn set_type(&mut self, typ: ImmType) {
        self.signature
            .set_field::<{ OperandSignature::PREDICATE_MASK }>(typ as u32);
    }

    pub fn reset_type(&mut self) {
        self.set_type(ImmType::Int);
    }

    pub fn predicate(&self) -> u32 {
        self.signature()
            .get_field::<{ OperandSignature::PREDICATE_MASK }>()
    }

    pub fn set_predicate(&mut self, predicate: u32) {
        self.signature
            .set_field::<{ OperandSignature::PREDICATE_MASK }>(predicate);
    }

    pub fn reset_predicate(&mut self) {
        self.set_predicate(0);
    }

    pub fn value(&self) -> i64 {
        (((self.data[DATA_IMM_VALUE_HI] as u64) << 32) | (self.data[DATA_IMM_VALUE_LO] as u64))
            as i64
    }

    pub fn is_int(&self) -> bool {
        self.typ() == ImmType::Int
    }

    pub fn is_double(&self) -> bool {
        self.typ() == ImmType::Double
    }

    pub fn is_int8(&self) -> bool {
        self.is_int() && (-128..=127).contains(&self.value())
    }

    pub fn is_uint8(&self) -> bool {
        self.is_int() && (0..=255).contains(&self.value())
    }

    pub fn is_int16(&self) -> bool {
        self.is_int() && (-32768..=32767).contains(&self.value())
    }

    pub fn is_uint16(&self) -> bool {
        self.is_int() && (0..=65535).contains(&self.value())
    }

    pub fn is_int32(&self) -> bool {
        self.is_int() && (-2147483648..=2147483647).contains(&self.value())
    }

    pub fn is_uint32(&self) -> bool {
        self.is_int() && self.data[DATA_IMM_VALUE_HI] == 0
    }

    pub fn value_as<T: FromPrimitive>(&self) -> T {
        T::from_i64(self.value()).unwrap()
    }

    pub fn int32_lo(&self) -> i32 {
        self.data[DATA_IMM_VALUE_LO] as i32
    }

    pub fn int32_hi(&self) -> i32 {
        self.data[DATA_IMM_VALUE_HI] as i32
    }

    pub fn uint32_lo(&self) -> u32 {
        self.data[DATA_IMM_VALUE_LO]
    }

    pub fn uint32_hi(&self) -> u32 {
        self.data[DATA_IMM_VALUE_HI]
    }

    pub fn set_value<T: ToPrimitive>(&mut self, val: T) {
        let value = val.to_i64().unwrap();
        self.data[DATA_IMM_VALUE_LO] = value as u32;
        self.data[DATA_IMM_VALUE_HI] = (value >> 32) as u32;
        self.set_type(ImmType::Int);
    }
    pub fn set_value_float<T: Into<f64>>(&mut self, val: T) {
        let value = f64::to_bits(val.into());
        self.data[DATA_IMM_VALUE_LO] = value as u32;
        self.data[DATA_IMM_VALUE_HI] = (value >> 32) as u32;
        self.set_type(ImmType::Double);
    }

    pub fn clone(&self) -> Self {
        *self
    }

    pub fn sign_extend_8_bits(&mut self) {
        self.set_value(self.value_as::<i8>() as i64);
    }

    pub fn sign_extend_16_bits(&mut self) {
        self.set_value(self.value_as::<i16>() as i64);
    }

    pub fn sign_extend_32_bits(&mut self) {
        self.set_value(self.value_as::<i32>() as i64);
    }

    pub fn zero_extend_8_bits(&mut self) {
        self.set_value(self.value_as::<u8>() as u64);
    }

    pub fn zero_extend_16_bits(&mut self) {
        self.set_value(self.value_as::<u16>() as u64);
    }

    pub fn zero_extend_32_bits(&mut self) {
        self.data[DATA_IMM_VALUE_HI] = 0;
    }
}

pub fn imm<T: Into<i64>>(val: T) -> Imm {
    Imm::from_value(val, 0)
}
