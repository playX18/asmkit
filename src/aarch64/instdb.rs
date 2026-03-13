/* Copyright (c) 2008-2024 The AsmJit Authors

   This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

   Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

   The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
   Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
   This notice may not be removed or altered from any source distribution.
*/

use super::assembler::*;
use super::operands::*;
use crate::core::operand::RegType;

pub const W: u32 = 0x1;
pub const X: u32 = 0x2;
pub const WX: u32 = W | X;

pub const ZR: u32 = Gp::ID_ZR as u32;
pub const SP: u32 = Gp::ID_SP as u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RwInfo {
    R,
    RW,
    RX,
    RRW,
    RWX,
    W,
    WRW,
    WRX,
    WRRW,
    WRRX,
    WW,
    X,
    XRX,
    XXRRX,

    LDn,
    STn,
}

pub const RWI_R: u16 = RwInfo::R as u16;
pub const RWI_RW: u16 = RwInfo::RW as u16;
pub const RWI_RX: u16 = RwInfo::RX as u16;
pub const RWI_RRW: u16 = RwInfo::RRW as u16;
pub const RWI_RWX: u16 = RwInfo::RWX as u16;
pub const RWI_W: u16 = RwInfo::W as u16;
pub const RWI_WRW: u16 = RwInfo::WRW as u16;
pub const RWI_WRX: u16 = RwInfo::WRX as u16;
pub const RWI_WRRW: u16 = RwInfo::WRRW as u16;
pub const RWI_WRRX: u16 = RwInfo::WRRX as u16;
pub const RWI_WW: u16 = RwInfo::WW as u16;
pub const RWI_X: u16 = RwInfo::X as u16;
pub const RWI_XRX: u16 = RwInfo::XRX as u16;
pub const RWI_XXRRX: u16 = RwInfo::XXRRX as u16;
pub const RWI_LDN: u16 = RwInfo::LDn as u16;
pub const RWI_STN: u16 = RwInfo::STn as u16;

impl RwInfo {
    pub const SPECIAL_START: Self = Self::LDn;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstElementType {
    None = VecElementType::None as isize,
    B = VecElementType::B as isize,
    H = VecElementType::H as isize,
    S = VecElementType::S as isize,
    D = VecElementType::D as isize,
    _2H = VecElementType::H2 as isize,
    _4B = VecElementType::B4 as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpType {
    X,
    W,
    XSp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum OpSignature {
    GpW = Reg::signature_of(RegType::Gp32).bits,
    GpX = Reg::signature_of(RegType::Gp64).bits,
    B = Reg::signature_of(RegType::Vec8).bits,
    H = Reg::signature_of(RegType::Vec16).bits,
    S = Reg::signature_of(RegType::Vec32).bits,
    D = Reg::signature_of(RegType::Vec64).bits,
    Q = Reg::signature_of(RegType::Vec128).bits,
    V8B = Self::D as u32 | Vec::SIGNATURE_ELEMENT_B,
    V4H = Self::D as u32 | Vec::SIGNATURE_ELEMENT_H,
    V2S = Self::D as u32 | Vec::SIGNATURE_ELEMENT_S,

    V16B = Self::Q as u32 | Vec::SIGNATURE_ELEMENT_B,
    V8H = Self::Q as u32 | Vec::SIGNATURE_ELEMENT_H,
    V4S = Self::Q as u32 | Vec::SIGNATURE_ELEMENT_S,
    V2D = Self::Q as u32 | Vec::SIGNATURE_ELEMENT_D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HFConv {
    N,
    None,
    A,
    B,
    C,
    D,
    Count,
}

pub enum VOType {
    VB,
    VBH,
    VBH4S,
    VBHS,
    VBHSD2,
    VHS,
    VS,

    VB8H4,
    VB8H4S2,
    VB8D1,
    VH4S2,

    VB16,
    VB16H8,
    VB16H8S4,
    VB16D2,
    VH8S4,
    VS4,
    VD2,

    SVBHS,
    SVB8H4S2,
    SVHS,
    VAny,
    SVAny,

    Count,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Encoding {
    None = 0,
    BaseAddSub,
    BaseAdr,
    BaseAtDcIcTlbi,
    BaseAtomicCasp,
    BaseAtomicOp,
    BaseAtomicSt,
    BaseBfc,
    BaseBfi,
    BaseBfm,
    BaseBfx,
    BaseBranchCmp,
    BaseBranchReg,
    BaseBranchRel,
    BaseBranchTst,
    BaseCCmp,
    BaseCInc,
    BaseCSel,
    BaseCSet,
    BaseCmpCmn,
    BaseExtend,
    BaseExtract,
    BaseLdSt,
    BaseLdpStp,
    BaseLdxp,
    BaseLogical,
    BaseMinMax,
    BaseMov,
    BaseMovKNZ,
    BaseMrs,
    BaseMsr,
    BaseMvnNeg,
    BaseOp,
    BaseOpImm,
    BaseOpX16,
    BasePrfm,
    BaseR,
    BaseRMNoImm,
    BaseRMSImm10,
    BaseRMSImm9,
    BaseRR,
    BaseRRII,
    BaseRRR,
    BaseRRRR,
    BaseRev,
    BaseShift,
    BaseStx,
    BaseStxp,
    BaseSys,
    BaseTst,
    FSimdPair,
    FSimdSV,
    FSimdVV,
    FSimdVVV,
    FSimdVVVV,
    FSimdVVVe,
    ISimdPair,
    ISimdSV,
    ISimdVV,
    ISimdVVV,
    ISimdVVVI,
    ISimdVVVV,
    ISimdVVVVx,
    ISimdVVVe,
    ISimdVVVx,
    ISimdVVx,
    ISimdWWV,
    SimdBicOrr,
    SimdCmp,
    SimdDot,
    SimdDup,
    SimdFcadd,
    SimdFccmpFccmpe,
    SimdFcm,
    SimdFcmla,
    SimdFcmpFcmpe,
    SimdFcsel,
    SimdFcvt,
    SimdFcvtLN,
    SimdFcvtSV,
    SimdFmlal,
    SimdFmov,
    SimdIns,
    SimdLdNStN,
    SimdLdSt,
    SimdLdpStp,
    SimdLdurStur,
    SimdMov,
    SimdMoviMvni,
    SimdShift,
    SimdShiftES,
    SimdSm3tt,
    SimdSmovUmov,
    SimdSxtlUxtl,
    SimdTblTbx,

    Count,
}

impl TryFrom<u8> for Encoding {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        unsafe {
            if value >= Self::Count as u8 {
                Err(())
            } else {
                Ok(::core::mem::transmute(value))
            }
        }
    }
}

const HF_C: u32 = HFConv::C as u32;

macro_rules! impl_const_new_zero {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        impl $name {
            pub const fn new($($field: $ty),*) -> Self {
                Self {
                    $($field),*
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseOp {
    pub opcode: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseOpX16 {
    pub opcode: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseOpImm {
    pub opcode: u32,
    pub imm_bits: u16,
    pub imm_offset: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseR {
    pub opcode: u32,
    pub reg_type: u32,
    pub reg_hi_id: u32,
    pub r_shift: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseRR {
    pub opcode: u32,
    pub a_type: u32,
    pub a_hi_id: u32,
    pub a_shift: u32,
    pub b_type: u32,
    pub b_hi_id: u32,
    pub b_shift: u32,
    pub uniform: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseRRR {
    opcode: u32,
    pub a_type: u32,
    pub a_hi_id: u32,
    pub b_type: u32,
    pub b_hi_id: u32,
    pub c_type: u32,
    pub c_hi_id: u32,
    pub uniform: u32,
}

impl BaseRRR {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseRRRR {
    opcode: u32,
    pub a_type: u32,
    pub a_hi_id: u32,
    pub b_type: u32,
    pub b_hi_id: u32,
    pub c_type: u32,
    pub c_hi_id: u32,
    pub d_type: u32,
    pub d_hi_id: u32,
    pub uniform: u32,
}

impl BaseRRRR {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseRRII {
    opcode: u32,
    pub a_type: u32,
    pub a_hi_id: u32,
    pub b_type: u32,
    pub b_hi_id: u32,
    pub a_imm_size: u32,
    pub a_imm_discard_lsb: u32,
    pub a_imm_offset: u32,
    pub b_imm_size: u32,
    pub b_imm_discard_lsb: u32,
    pub b_imm_offset: u32,
}

impl BaseRRII {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseAtDcIcTlbi {
    pub imm_verify_mask: u32,
    pub imm_verify_data: u32,
    pub mandatory_reg: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseAdcSbc {
    pub opcode: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseMinMax {
    pub register_op: u32,
    pub immediate_op: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseAddSub {
    pub shifted_op: u32,
    pub extended_op: u32,
    pub immediate_op: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseAdr {
    opcode: u32,
    pub offset_type: u8,
}

impl BaseAdr {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseBfm {
    pub opcode: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseCmpCmn {
    pub shifted_op: u32,
    pub extended_op: u32,
    pub immediate_op: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseExtend {
    opcode: u32,
    pub reg_type: u32,
    pub u: u32,
}

impl BaseExtend {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseLogical {
    pub shifted_op: u32,
    pub immediate_op: u32,
    pub negate_imm: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseMvnNeg {
    pub opcode: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseShift {
    pub register_op: u32,
    pub immediate_op: u32,
    pub ror: u32,
}

impl BaseShift {
    pub const fn register_op(&self) -> u32 {
        self.register_op << 10
    }

    pub const fn immediate_op(&self) -> u32 {
        self.immediate_op << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseTst {
    pub shifted_op: u32,
    pub immediate_op: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseRMNoImm {
    opcode: u32,
    pub reg_type: u32,
    pub reg_hi_id: u32,
    pub x_offset: u32,
}

impl BaseRMNoImm {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseRMSImm9 {
    pub offset_op: u32,
    pub pre_post_op: u32,
    pub reg_type: u32,
    pub reg_hi_id: u32,
    pub x_offset: u32,
    pub imm_shift: u32,
}

impl BaseRMSImm9 {
    pub const fn offset_op(&self) -> u32 {
        self.offset_op << 10
    }

    pub const fn pre_post_op(&self) -> u32 {
        self.pre_post_op << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseRMSImm10 {
    opcode: u32,
    pub reg_type: u32,
    pub reg_hi_id: u32,
    pub x_offset: u32,
    pub imm_shift: u32,
}

impl BaseRMSImm10 {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BasePrfm {
    pub register_op: u32,
    pub s_offset_op: u32,
    pub u_offset_op: u32,
    pub literal_op: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseLdSt {
    pub u_offset_op: u32,
    pub pre_post_op: u32,
    pub register_op: u32,
    pub literal_op: u32,
    pub reg_type: u32,
    pub x_offset: u32,
    pub u_offset_shift: u32,
    pub u_alt_inst_id: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseLdpStp {
    pub offset_op: u32,
    pub pre_post_op: u32,
    pub reg_type: u32,
    pub x_offset: u32,
    pub offset_shift: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseStx {
    opcode: u32,
    pub reg_type: u32,
    pub x_offset: u32,
}

impl BaseStx {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseLdxp {
    opcode: u32,
    pub reg_type: u32,
    pub x_offset: u32,
}

impl BaseLdxp {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseStxp {
    opcode: u32,
    pub reg_type: u32,
    pub x_offset: u32,
}

impl BaseStxp {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseAtomicOp {
    opcode: u32,
    pub reg_type: u32,
    pub x_offset: u32,
    pub zr_reg: u32,
}

impl BaseAtomicOp {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseAtomicSt {
    opcode: u32,
    pub reg_type: u32,
    pub x_offset: u32,
}

impl BaseAtomicSt {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseAtomicCasp {
    opcode: u32,
    pub reg_type: u32,
    pub x_offset: u32,
}

impl BaseAtomicCasp {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

pub type BaseBranchReg = BaseOp;
pub type BaseBranchRel = BaseOp;
pub type BaseBranchCmp = BaseOp;
pub type BaseBranchTst = BaseOp;
pub type BaseExtract = BaseOp;
pub type BaseBfc = BaseOp;
pub type BaseBfi = BaseOp;
pub type BaseBfx = BaseOp;
pub type BaseCCmp = BaseOp;
pub type BaseCInc = BaseOp;
pub type BaseCSet = BaseOp;
pub type BaseCSel = BaseOp;
pub type BaseMovKNZ = BaseOp;
pub type BaseMull = BaseOp;

#[derive(Debug, Clone, Copy, Default)]
pub struct FSimdGeneric {
    scalar_op: u32,
    scalar_hf: u32,
    vector_op: u32,
    vector_hf: u32,
}

impl FSimdGeneric {
    pub const fn scalar_op(&self) -> u32 {
        self.scalar_op << 10
    }

    pub const fn vector_op(&self) -> u32 {
        self.vector_op << 10
    }

    pub const fn scalar_hf(&self) -> u32 {
        self.scalar_hf
    }

    pub const fn vector_hf(&self) -> u32 {
        self.vector_hf
    }
}

pub type FSimdVV = FSimdGeneric;
pub type FSimdVVV = FSimdGeneric;
pub type FSimdVVVV = FSimdGeneric;

#[derive(Debug, Clone, Copy, Default)]
pub struct FSimdSV {
    pub opcode: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FSimdVVVe {
    scalar_op: u32,
    scalar_hf: u32,
    vector_op: u32,
    element_op: u32,
}

impl FSimdVVVe {
    pub const fn scalar_op(&self) -> u32 {
        self.scalar_op << 10
    }

    pub const fn scalar_hf(&self) -> u32 {
        self.scalar_hf
    }

    pub const fn vector_op(&self) -> u32 {
        self.vector_op << 10
    }

    pub const fn vector_hf(&self) -> u32 {
        HF_C
    }

    pub const fn element_scalar_op(&self) -> u32 {
        (self.element_op << 10) | (0x5 << 28)
    }

    pub const fn element_vector_op(&self) -> u32 {
        self.element_op << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdFcadd {
    opcode: u32,
}

impl SimdFcadd {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdFcmla {
    regular_op: u32,
    element_op: u32,
}

impl SimdFcmla {
    pub const fn regular_op(&self) -> u32 {
        self.regular_op << 10
    }

    pub const fn element_op(&self) -> u32 {
        self.element_op << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdFccmpFccmpe {
    opcode: u32,
}

impl SimdFccmpFccmpe {
    pub const fn opcode(&self) -> u32 {
        self.opcode
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdFcm {
    register_op: u32,
    register_hf: u32,
    zero_op: u32,
}

impl SimdFcm {
    pub const fn has_register_op(&self) -> bool {
        self.register_op != 0
    }

    pub const fn has_zero_op(&self) -> bool {
        self.zero_op != 0
    }

    pub const fn register_scalar_op(&self) -> u32 {
        (self.register_op << 10) | (0x5 << 28)
    }

    pub const fn register_vector_op(&self) -> u32 {
        self.register_op << 10
    }

    pub const fn register_scalar_hf(&self) -> u32 {
        self.register_hf
    }

    pub const fn register_vector_hf(&self) -> u32 {
        self.register_hf
    }

    pub const fn zero_scalar_op(&self) -> u32 {
        (self.zero_op << 10) | (0x5 << 28)
    }

    pub const fn zero_vector_op(&self) -> u32 {
        self.zero_op << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdFcmpFcmpe {
    opcode: u32,
}

impl SimdFcmpFcmpe {
    pub const fn opcode(&self) -> u32 {
        self.opcode
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdFcvtLN {
    opcode: u32,
    is_cvtxn: u32,
    has_scalar: u32,
}

impl SimdFcvtLN {
    pub const fn scalar_op(&self) -> u32 {
        (self.opcode << 10) | (0x5 << 28)
    }

    pub const fn vector_op(&self) -> u32 {
        self.opcode << 10
    }

    pub const fn is_cvtxn(&self) -> u32 {
        self.is_cvtxn
    }

    pub const fn has_scalar(&self) -> u32 {
        self.has_scalar
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdFcvtSV {
    vector_int_op: u32,
    vector_fp_op: u32,
    general_op: u32,
    is_float_to_int: u32,
}

impl SimdFcvtSV {
    pub const fn scalar_int_op(&self) -> u32 {
        (self.vector_int_op << 10) | (0x5 << 28)
    }

    pub const fn vector_int_op(&self) -> u32 {
        self.vector_int_op << 10
    }

    pub const fn scalar_fp_op(&self) -> u32 {
        (self.vector_fp_op << 10) | (0x5 << 28)
    }

    pub const fn vector_fp_op(&self) -> u32 {
        self.vector_fp_op << 10
    }

    pub const fn general_op(&self) -> u32 {
        self.general_op << 10
    }

    pub const fn is_float_to_int(&self) -> u32 {
        self.is_float_to_int
    }

    pub const fn is_fixed_point(&self) -> bool {
        self.vector_fp_op != 0
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdFmlal {
    pub vector_op: u32,
    pub element_op: u32,
    pub optional_q: u8,
    pub ta: u8,
    pub tb: u8,
    pub t_element: u8,
}

impl SimdFmlal {
    pub const fn vector_op(&self) -> u32 {
        self.vector_op << 10
    }

    pub const fn element_op(&self) -> u32 {
        self.element_op << 10
    }

    pub const fn optional_q(&self) -> u32 {
        self.optional_q as u32
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FSimdPair {
    pub scalar_op: u32,
    pub vector_op: u32,
}

impl FSimdPair {
    pub const fn scalar_op(&self) -> u32 {
        self.scalar_op << 10
    }

    pub const fn vector_op(&self) -> u32 {
        self.vector_op << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ISimdVV {
    opcode: u32,
    pub vec_op_type: u32,
}

impl ISimdVV {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ISimdVVx {
    opcode: u32,
    pub op0_signature: u32,
    pub op1_signature: u32,
}

impl ISimdVVx {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ISimdSV {
    opcode: u32,
    pub vec_op_type: u32,
}

impl ISimdSV {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ISimdVVV {
    opcode: u32,
    pub vec_op_type: u32,
}

impl ISimdVVV {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ISimdVVVx {
    opcode: u32,
    pub op0_signature: u32,
    pub op1_signature: u32,
    pub op2_signature: u32,
}

impl ISimdVVVx {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ISimdWWV {
    opcode: u32,
    pub vec_op_type: u32,
}

impl ISimdWWV {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ISimdVVVe {
    pub regular_op: u32,
    pub regular_vec_type: u32,
    pub element_op: u32,
    pub element_vec_type: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ISimdVVVI {
    opcode: u32,
    pub vec_op_type: u32,
    pub imm_size: u32,
    pub imm_shift: u32,
    pub imm64_has_one_bit_less: u32,
}

impl ISimdVVVI {
    pub const fn opcode(&self) -> u32 {
        self.opcode << 10
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ISimdVVVV {
    pub opcode: u32,
    pub vec_op_type: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ISimdVVVVx {
    pub opcode: u32,
    pub op0_signature: u32,
    pub op1_signature: u32,
    pub op2_signature: u32,
    pub op3_signature: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdBicOrr {
    pub register_op: u32,
    pub immediate_op: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdCmp {
    pub register_op: u32,
    pub zero_op: u32,
    pub vec_op_type: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdDot {
    pub vector_op: u32,
    pub element_op: u32,
    pub ta: u8,
    pub tb: u8,
    pub t_element: u8,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdMoviMvni {
    pub opcode: u32,
    pub inverted: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdLdSt {
    pub u_offset_op: u32,
    pub pre_post_op: u32,
    pub register_op: u32,
    pub literal_op: u32,
    pub u_alt_inst_id: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdLdNStN {
    pub single_op: u32,
    pub multiple_op: u32,
    pub n: u32,
    pub replicate: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdLdpStp {
    pub offset_op: u32,
    pub pre_post_op: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdLdurStur {
    pub opcode: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ISimdPair {
    pub opcode2: u32,
    pub opcode3: u32,
    pub op_type3: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdShift {
    pub register_op: u32,
    pub immediate_op: u32,
    pub inverted_imm: u32,
    pub vec_op_type: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdShiftES {
    pub opcode: u32,
    pub vec_op_type: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdSm3tt {
    pub opcode: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdSmovUmov {
    pub opcode: u32,
    pub vec_op_type: u32,
    pub is_signed: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdSxtlUxtl {
    pub opcode: u32,
    pub vec_op_type: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimdTblTbx {
    pub opcode: u32,
}

impl_const_new_zero!(BaseOp { opcode: u32 });
impl_const_new_zero!(BaseOpX16 { opcode: u32 });
impl_const_new_zero!(BaseOpImm {
    opcode: u32,
    imm_bits: u16,
    imm_offset: u16
});
impl_const_new_zero!(BaseR {
    opcode: u32,
    reg_type: u32,
    reg_hi_id: u32,
    r_shift: u32
});
impl_const_new_zero!(BaseRR {
    opcode: u32,
    a_type: u32,
    a_hi_id: u32,
    a_shift: u32,
    b_type: u32,
    b_hi_id: u32,
    b_shift: u32,
    uniform: u32,
});
impl_const_new_zero!(BaseRRR {
    opcode: u32,
    a_type: u32,
    a_hi_id: u32,
    b_type: u32,
    b_hi_id: u32,
    c_type: u32,
    c_hi_id: u32,
    uniform: u32,
});
impl_const_new_zero!(BaseRRRR {
    opcode: u32,
    a_type: u32,
    a_hi_id: u32,
    b_type: u32,
    b_hi_id: u32,
    c_type: u32,
    c_hi_id: u32,
    d_type: u32,
    d_hi_id: u32,
    uniform: u32,
});
impl_const_new_zero!(BaseRRII {
    opcode: u32,
    a_type: u32,
    a_hi_id: u32,
    b_type: u32,
    b_hi_id: u32,
    a_imm_size: u32,
    a_imm_discard_lsb: u32,
    a_imm_offset: u32,
    b_imm_size: u32,
    b_imm_discard_lsb: u32,
    b_imm_offset: u32,
});
impl_const_new_zero!(BaseAtDcIcTlbi {
    imm_verify_mask: u32,
    imm_verify_data: u32,
    mandatory_reg: u32,
});
impl_const_new_zero!(BaseAdcSbc { opcode: u32 });
impl_const_new_zero!(BaseMinMax {
    register_op: u32,
    immediate_op: u32
});
impl_const_new_zero!(BaseAddSub {
    shifted_op: u32,
    extended_op: u32,
    immediate_op: u32
});
impl_const_new_zero!(BaseAdr {
    opcode: u32,
    offset_type: u8
});
impl_const_new_zero!(BaseBfm { opcode: u32 });
impl_const_new_zero!(BaseCmpCmn {
    shifted_op: u32,
    extended_op: u32,
    immediate_op: u32
});
impl_const_new_zero!(BaseExtend {
    opcode: u32,
    reg_type: u32,
    u: u32
});
impl_const_new_zero!(BaseLogical {
    shifted_op: u32,
    immediate_op: u32,
    negate_imm: u32
});
impl_const_new_zero!(BaseMvnNeg { opcode: u32 });
impl_const_new_zero!(BaseShift {
    register_op: u32,
    immediate_op: u32,
    ror: u32
});
impl_const_new_zero!(BaseTst {
    shifted_op: u32,
    immediate_op: u32
});
impl_const_new_zero!(BaseRMNoImm {
    opcode: u32,
    reg_type: u32,
    reg_hi_id: u32,
    x_offset: u32
});
impl_const_new_zero!(BaseRMSImm9 {
    offset_op: u32,
    pre_post_op: u32,
    reg_type: u32,
    reg_hi_id: u32,
    x_offset: u32,
    imm_shift: u32,
});
impl_const_new_zero!(BaseRMSImm10 {
    opcode: u32,
    reg_type: u32,
    reg_hi_id: u32,
    x_offset: u32,
    imm_shift: u32,
});
impl_const_new_zero!(BasePrfm {
    register_op: u32,
    s_offset_op: u32,
    u_offset_op: u32,
    literal_op: u32
});
impl_const_new_zero!(BaseLdSt {
    u_offset_op: u32,
    pre_post_op: u32,
    register_op: u32,
    literal_op: u32,
    reg_type: u32,
    x_offset: u32,
    u_offset_shift: u32,
    u_alt_inst_id: u32,
});
impl_const_new_zero!(BaseLdpStp {
    offset_op: u32,
    pre_post_op: u32,
    reg_type: u32,
    x_offset: u32,
    offset_shift: u32,
});
impl_const_new_zero!(BaseStx {
    opcode: u32,
    reg_type: u32,
    x_offset: u32
});
impl_const_new_zero!(BaseLdxp {
    opcode: u32,
    reg_type: u32,
    x_offset: u32
});
impl_const_new_zero!(BaseStxp {
    opcode: u32,
    reg_type: u32,
    x_offset: u32
});
impl_const_new_zero!(BaseAtomicOp {
    opcode: u32,
    reg_type: u32,
    x_offset: u32,
    zr_reg: u32
});
impl_const_new_zero!(BaseAtomicSt {
    opcode: u32,
    reg_type: u32,
    x_offset: u32
});
impl_const_new_zero!(BaseAtomicCasp {
    opcode: u32,
    reg_type: u32,
    x_offset: u32
});

impl_const_new_zero!(FSimdGeneric {
    scalar_op: u32,
    scalar_hf: u32,
    vector_op: u32,
    vector_hf: u32
});
impl_const_new_zero!(FSimdSV { opcode: u32 });
impl_const_new_zero!(FSimdVVVe {
    scalar_op: u32,
    scalar_hf: u32,
    vector_op: u32,
    element_op: u32
});
impl_const_new_zero!(SimdFcadd { opcode: u32 });
impl_const_new_zero!(SimdFcmla {
    regular_op: u32,
    element_op: u32
});
impl_const_new_zero!(SimdFccmpFccmpe { opcode: u32 });
impl_const_new_zero!(SimdFcm {
    register_op: u32,
    register_hf: u32,
    zero_op: u32
});
impl_const_new_zero!(SimdFcmpFcmpe { opcode: u32 });
impl_const_new_zero!(SimdFcvtLN {
    opcode: u32,
    is_cvtxn: u32,
    has_scalar: u32
});
impl_const_new_zero!(SimdFcvtSV {
    vector_int_op: u32,
    vector_fp_op: u32,
    general_op: u32,
    is_float_to_int: u32,
});
impl_const_new_zero!(SimdFmlal {
    vector_op: u32,
    element_op: u32,
    optional_q: u8,
    ta: u8,
    tb: u8,
    t_element: u8,
});
impl_const_new_zero!(FSimdPair {
    scalar_op: u32,
    vector_op: u32
});

impl_const_new_zero!(ISimdVV {
    opcode: u32,
    vec_op_type: u32
});
impl_const_new_zero!(ISimdVVx {
    opcode: u32,
    op0_signature: u32,
    op1_signature: u32
});
impl_const_new_zero!(ISimdSV {
    opcode: u32,
    vec_op_type: u32
});
impl_const_new_zero!(ISimdVVV {
    opcode: u32,
    vec_op_type: u32
});
impl_const_new_zero!(ISimdVVVx {
    opcode: u32,
    op0_signature: u32,
    op1_signature: u32,
    op2_signature: u32,
});
impl_const_new_zero!(ISimdWWV {
    opcode: u32,
    vec_op_type: u32
});
impl_const_new_zero!(ISimdVVVe {
    regular_op: u32,
    regular_vec_type: u32,
    element_op: u32,
    element_vec_type: u32,
});
impl_const_new_zero!(ISimdVVVI {
    opcode: u32,
    vec_op_type: u32,
    imm_size: u32,
    imm_shift: u32,
    imm64_has_one_bit_less: u32,
});
impl_const_new_zero!(ISimdVVVV {
    opcode: u32,
    vec_op_type: u32
});
impl_const_new_zero!(ISimdVVVVx {
    opcode: u32,
    op0_signature: u32,
    op1_signature: u32,
    op2_signature: u32,
    op3_signature: u32,
});
impl_const_new_zero!(SimdBicOrr {
    register_op: u32,
    immediate_op: u32
});
impl_const_new_zero!(SimdCmp {
    register_op: u32,
    zero_op: u32,
    vec_op_type: u32
});
impl_const_new_zero!(SimdDot {
    vector_op: u32,
    element_op: u32,
    ta: u8,
    tb: u8,
    t_element: u8
});
impl_const_new_zero!(SimdMoviMvni {
    opcode: u32,
    inverted: u32
});
impl_const_new_zero!(SimdLdSt {
    u_offset_op: u32,
    pre_post_op: u32,
    register_op: u32,
    literal_op: u32,
    u_alt_inst_id: u32,
});
impl_const_new_zero!(SimdLdNStN {
    single_op: u32,
    multiple_op: u32,
    n: u32,
    replicate: u32
});
impl_const_new_zero!(SimdLdpStp {
    offset_op: u32,
    pre_post_op: u32
});
impl_const_new_zero!(SimdLdurStur { opcode: u32 });
impl_const_new_zero!(ISimdPair {
    opcode2: u32,
    opcode3: u32,
    op_type3: u32
});
impl_const_new_zero!(SimdShift {
    register_op: u32,
    immediate_op: u32,
    inverted_imm: u32,
    vec_op_type: u32
});
impl_const_new_zero!(SimdShiftES {
    opcode: u32,
    vec_op_type: u32
});
impl_const_new_zero!(SimdSm3tt { opcode: u32 });
impl_const_new_zero!(SimdSmovUmov {
    opcode: u32,
    vec_op_type: u32,
    is_signed: u32
});
impl_const_new_zero!(SimdSxtlUxtl {
    opcode: u32,
    vec_op_type: u32
});
impl_const_new_zero!(SimdTblTbx { opcode: u32 });

#[derive(Debug, Clone, Copy)]
pub struct InstInfo {
    pub encoding: u8,
    pub encoding_data_index: u8,
    pub reserved: u16,
    pub rw_info_index: u16,
    pub flags: u16,
}

impl InstInfo {
    pub const fn new(
        encoding: u8,
        encoding_data_index: u8,
        reserved: u16,
        rw_info_index: u16,
        flags: u16,
    ) -> Self {
        Self {
            encoding,
            encoding_data_index,
            reserved,
            rw_info_index,
            flags,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InstFlag {
    Cond = 0x00000001,
    Pair = 0x00000002,
    Long = 0x00000004,
    Narrow = 0x00000008,
    VH0_15 = 0x00000010,
    Consecutive = 0x00000080,
}

macro_rules! F {
    ($name: ident) => {
        InstFlag::$name as u16
    };
}

macro_rules! INST {
    ($id: ident, $opcode_encoding: ident, $opcode_data: expr, $rw_info_index: expr, $flags: expr, $opcode_data_index: expr) => {
        InstInfo::new(
            Encoding::$opcode_encoding as u8,
            $opcode_data_index as u8,
            0,
            $rw_info_index,
            $flags,
        )
    };
}

macro_rules! TABLE {
    ($name: ident = {
        $(INST(
            $id: ident,
            $opcode_encoding: ident,
            $opcode_data: expr,
            $rw_info_index: expr,
            $flags: expr,
            $opcode_data_index: expr
        ),)*
    }) => {
        pub static $name: &[InstInfo] = &[
            $(
                INST!(
                    $id,
                    $opcode_encoding,
                    $opcode_data,
                    $rw_info_index,
                    $flags,
                    $opcode_data_index)
            ),*
        ];
    };
}

TABLE!(INST_INFO_TABLE

  = {
// +------------------+---------------------+--------------------------------------------------------------------------------------+-----------+---------------------------+----+
// | Instruction Id   | Encoding            | Opcode Data                                                                          | RW Info   | Instruction Flags         |DatX|
// +------------------+---------------------+--------------------------------------------------------------------------------------+-----------+---------------------------+----+
// ${InstInfo:Begin}
INST(None             , None               , (_)                                                                                   , 0         , 0                         , 0  ), // #0
INST(Abs              , BaseRR             , (0b01011010110000000010000000000000, kWX, kZR, 0, kWX, kZR, 5, true)                  , RWI_W    , 0                         , 0  ), // #1
INST(Adc              , BaseRRR            , (0b0001101000000000000000, kWX, kZR, kWX, kZR, kWX, kZR, true)                        , RWI_W    , 0                         , 0  ), // #2
INST(Adcs             , BaseRRR            , (0b0011101000000000000000, kWX, kZR, kWX, kZR, kWX, kZR, true)                        , RWI_W    , 0                         , 1  ), // #3
INST(Add              , BaseAddSub         , (0b0001011000, 0b0001011001, 0b0010001)                                               , RWI_W    , 0                         , 0  ), // #4
INST(Addg             , BaseRRII           , (0b1001000110000000000000, kX, kSP, kX, kSP, 6, 4, 16, 4, 0, 10)                      , RWI_W    , 0                         , 0  ), // #5
INST(Adds             , BaseAddSub         , (0b0101011000, 0b0101011001, 0b0110001)                                               , RWI_W    , 0                         , 1  ), // #6
INST(Adr              , BaseAdr            , (0b0001000000000000000000, OffsetType::kAArch64_ADR)                                  , RWI_W    , 0                         , 0  ), // #7
INST(Adrp             , BaseAdr            , (0b1001000000000000000000, OffsetType::kAArch64_ADRP)                                 , RWI_W    , 0                         , 1  ), // #8
INST(And              , BaseLogical        , (0b0001010000, 0b00100100, 0)                                                         , RWI_W    , 0                         , 0  ), // #9
INST(Ands             , BaseLogical        , (0b1101010000, 0b11100100, 0)                                                         , RWI_W    , 0                         , 1  ), // #10
INST(Asr              , BaseShift          , (0b0001101011000000001010, 0b0001001100000000011111, 0)                               , RWI_W    , 0                         , 0  ), // #11
INST(Asrv             , BaseShift          , (0b0001101011000000001010, 0b0000000000000000000000, 0)                               , RWI_W    , 0                         , 1  ), // #12
INST(At               , BaseAtDcIcTlbi     , (0b00011111110000, 0b00001111000000, true)                                            , RWI_RX   , 0                         , 0  ), // #13
INST(Autda            , BaseRR             , (0b11011010110000010001100000000000, kX, kZR, 0, kX, kSP, 5, true)                    , RWI_X    , 0                         , 1  ), // #14
INST(Autdza           , BaseR              , (0b11011010110000010011101111100000, kX, kZR, 0)                                      , RWI_X    , 0                         , 0  ), // #15
INST(Autdb            , BaseRR             , (0b11011010110000010001110000000000, kX, kZR, 0, kX, kSP, 5, true)                    , RWI_X    , 0                         , 2  ), // #16
INST(Autdzb           , BaseR              , (0b11011010110000010011111111100000, kX, kZR, 0)                                      , RWI_X    , 0                         , 1  ), // #17
INST(Autia            , BaseRR             , (0b11011010110000010001000000000000, kX, kZR, 0, kX, kSP, 5, true)                    , RWI_X    , 0                         , 3  ), // #18
INST(Autia1716        , BaseOp             , (0b11010101000000110010000110011111)                                                  , 0         , 0                         , 0  ), // #19
INST(Autiasp          , BaseOp             , (0b11010101000000110010001110111111)                                                  , 0         , 0                         , 1  ), // #20
INST(Autiaz           , BaseOp             , (0b11010101000000110010001110011111)                                                  , 0         , 0                         , 2  ), // #21
INST(Autib            , BaseRR             , (0b11011010110000010001010000000000, kX, kZR, 0, kX, kSP, 5, true)                    , RWI_X    , 0                         , 4  ), // #22
INST(Autib1716        , BaseOp             , (0b11010101000000110010000111011111)                                                  , 0         , 0                         , 3  ), // #23
INST(Autibsp          , BaseOp             , (0b11010101000000110010001111111111)                                                  , 0         , 0                         , 4  ), // #24
INST(Autibz           , BaseOp             , (0b11010101000000110010001111011111)                                                  , 0         , 0                         , 5  ), // #25
INST(Autiza           , BaseR              , (0b11011010110000010011001111100000, kX, kZR, 0)                                      , RWI_X    , 0                         , 2  ), // #26
INST(Autizb           , BaseR              , (0b11011010110000010011011111100000, kX, kZR, 0)                                      , RWI_X    , 0                         , 3  ), // #27
INST(Axflag           , BaseOp             , (0b11010101000000000100000001011111)                                                  , 0         , 0                         , 6  ), // #28
INST(B                , BaseBranchRel      , (0b00010100000000000000000000000000)                                                  , 0         , F!(Cond)                   , 0  ), // #29
INST(Bc               , BaseBranchRel      , (0b00010100000000000000000000010000)                                                  , 0         , F!(Cond)                   , 1  ), // #30
INST(Bfc              , BaseBfc            , (0b00110011000000000000001111100000)                                                  , RWI_X    , 0                         , 0  ), // #31
INST(Bfi              , BaseBfi            , (0b00110011000000000000000000000000)                                                  , RWI_X    , 0                         , 0  ), // #32
INST(Bfm              , BaseBfm            , (0b00110011000000000000000000000000)                                                  , RWI_X    , 0                         , 0  ), // #33
INST(Bfxil            , BaseBfx            , (0b00110011000000000000000000000000)                                                  , RWI_X    , 0                         , 0  ), // #34
INST(Bic              , BaseLogical        , (0b0001010001, 0b00100100, 1)                                                         , RWI_W    , 0                         , 2  ), // #35
INST(Bics             , BaseLogical        , (0b1101010001, 0b11100100, 1)                                                         , RWI_W    , 0                         , 3  ), // #36
INST(Bl               , BaseBranchRel      , (0b10010100000000000000000000000000)                                                  , 0         , 0                         , 2  ), // #37
INST(Blr              , BaseBranchReg      , (0b11010110001111110000000000000000)                                                  , RWI_R    , 0                         , 0  ), // #38
INST(Br               , BaseBranchReg      , (0b11010110000111110000000000000000)                                                  , RWI_R    , 0                         , 1  ), // #39
INST(Brk              , BaseOpImm          , (0b11010100001000000000000000000000, 16, 5)                                           , 0         , 0                         , 0  ), // #40
INST(Bti              , BaseOpImm          , (0b11010101000000110010010000011111, 2, 6)                                            , 0         , 0                         , 1  ), // #41
INST(Cas              , BaseAtomicOp       , (0b1000100010100000011111, kWX, 30, 0)                                                , RWI_XRX  , 0                         , 0  ), // #42
INST(Casa             , BaseAtomicOp       , (0b1000100011100000011111, kWX, 30, 1)                                                , RWI_XRX  , 0                         , 1  ), // #43
INST(Casab            , BaseAtomicOp       , (0b0000100011100000011111, kW , 0 , 1)                                                , RWI_XRX  , 0                         , 2  ), // #44
INST(Casah            , BaseAtomicOp       , (0b0100100011100000011111, kW , 0 , 1)                                                , RWI_XRX  , 0                         , 3  ), // #45
INST(Casal            , BaseAtomicOp       , (0b1000100011100000111111, kWX, 30, 1)                                                , RWI_XRX  , 0                         , 4  ), // #46
INST(Casalb           , BaseAtomicOp       , (0b0000100011100000111111, kW , 0 , 1)                                                , RWI_XRX  , 0                         , 5  ), // #47
INST(Casalh           , BaseAtomicOp       , (0b0100100011100000111111, kW , 0 , 1)                                                , RWI_XRX  , 0                         , 6  ), // #48
INST(Casb             , BaseAtomicOp       , (0b0000100010100000011111, kW , 0 , 0)                                                , RWI_XRX  , 0                         , 7  ), // #49
INST(Cash             , BaseAtomicOp       , (0b0100100010100000011111, kW , 0 , 0)                                                , RWI_XRX  , 0                         , 8  ), // #50
INST(Casl             , BaseAtomicOp       , (0b1000100010100000111111, kWX, 30, 0)                                                , RWI_XRX  , 0                         , 9  ), // #51
INST(Caslb            , BaseAtomicOp       , (0b0000100010100000111111, kW , 0 , 0)                                                , RWI_XRX  , 0                         , 10 ), // #52
INST(Caslh            , BaseAtomicOp       , (0b0100100010100000111111, kW , 0 , 0)                                                , RWI_XRX  , 0                         , 11 ), // #53
INST(Casp             , BaseAtomicCasp     , (0b0000100000100000011111, kWX, 30)                                                   , RWI_XXRRX, F!(Consecutive)            , 0  ), // #54
INST(Caspa            , BaseAtomicCasp     , (0b0000100001100000011111, kWX, 30)                                                   , RWI_XXRRX, F!(Consecutive)            , 1  ), // #55
INST(Caspal           , BaseAtomicCasp     , (0b0000100001100000111111, kWX, 30)                                                   , RWI_XXRRX, F!(Consecutive)            , 2  ), // #56
INST(Caspl            , BaseAtomicCasp     , (0b0000100000100000111111, kWX, 30)                                                   , RWI_XXRRX, F!(Consecutive)            , 3  ), // #57
INST(Cbnz             , BaseBranchCmp      , (0b00110101000000000000000000000000)                                                  , RWI_R    , 0                         , 0  ), // #58
INST(Cbz              , BaseBranchCmp      , (0b00110100000000000000000000000000)                                                  , RWI_R    , 0                         , 1  ), // #59
INST(Ccmn             , BaseCCmp           , (0b00111010010000000000000000000000)                                                  , RWI_R    , 0                         , 0  ), // #60
INST(Ccmp             , BaseCCmp           , (0b01111010010000000000000000000000)                                                  , RWI_R    , 0                         , 1  ), // #61
INST(Cfinv            , BaseOp             , (0b11010101000000000100000000011111)                                                  , 0         , 0                         , 7  ), // #62
INST(Chkfeat          , BaseOpX16          , (0b11010101000000110010010100011111)                                                  , 0         , 0                         , 0  ), // #63
INST(Cinc             , BaseCInc           , (0b00011010100000000000010000000000)                                                  , RWI_W    , 0                         , 0  ), // #64
INST(Cinv             , BaseCInc           , (0b01011010100000000000000000000000)                                                  , RWI_W    , 0                         , 1  ), // #65
INST(Clrbhb           , BaseOp             , (0b11010101000000110010001011011111)                                                  , 0         , 0                         , 8  ), // #66
INST(Clrex            , BaseOpImm          , (0b11010101000000110011000001011111, 4, 8)                                            , 0         , 0                         , 2  ), // #67
INST(Cls              , BaseRR             , (0b01011010110000000001010000000000, kWX, kZR, 0, kWX, kZR, 5, true)                  , RWI_W    , 0                         , 5  ), // #68
INST(Clz              , BaseRR             , (0b01011010110000000001000000000000, kWX, kZR, 0, kWX, kZR, 5, true)                  , RWI_W    , 0                         , 6  ), // #69
INST(Cmn              , BaseCmpCmn         , (0b0101011000, 0b0101011001, 0b0110001)                                               , RWI_R    , 0                         , 0  ), // #70
INST(Cmp              , BaseCmpCmn         , (0b1101011000, 0b1101011001, 0b1110001)                                               , RWI_R    , 0                         , 1  ), // #71
INST(Cmpp             , BaseRR             , (0b10111010110000000000000000011111, kX, kSP, 5, kX, kSP, 16, true)                   , RWI_R    , 0                         , 7  ), // #72
INST(Cneg             , BaseCInc           , (0b01011010100000000000010000000000)                                                  , RWI_W    , 0                         , 2  ), // #73
INST(Cnt              , BaseRR             , (0b01011010110000000001110000000000, kWX, kZR, 0, kWX, kZR, 5, true)                  , RWI_W    , 0                         , 8  ), // #74
INST(Crc32b           , BaseRRR            , (0b0001101011000000010000, kW, kZR, kW, kZR, kW, kZR, false)                          , RWI_W    , 0                         , 2  ), // #75
INST(Crc32cb          , BaseRRR            , (0b0001101011000000010100, kW, kZR, kW, kZR, kW, kZR, false)                          , RWI_W    , 0                         , 3  ), // #76
INST(Crc32ch          , BaseRRR            , (0b0001101011000000010101, kW, kZR, kW, kZR, kW, kZR, false)                          , RWI_W    , 0                         , 4  ), // #77
INST(Crc32cw          , BaseRRR            , (0b0001101011000000010110, kW, kZR, kW, kZR, kW, kZR, false)                          , RWI_W    , 0                         , 5  ), // #78
INST(Crc32cx          , BaseRRR            , (0b1001101011000000010111, kW, kZR, kW, kZR, kX, kZR, false)                          , RWI_W    , 0                         , 6  ), // #79
INST(Crc32h           , BaseRRR            , (0b0001101011000000010001, kW, kZR, kW, kZR, kW, kZR, false)                          , RWI_W    , 0                         , 7  ), // #80
INST(Crc32w           , BaseRRR            , (0b0001101011000000010010, kW, kZR, kW, kZR, kW, kZR, false)                          , RWI_W    , 0                         , 8  ), // #81
INST(Crc32x           , BaseRRR            , (0b1001101011000000010011, kW, kZR, kW, kZR, kX, kZR, false)                          , RWI_W    , 0                         , 9  ), // #82
INST(Csdb             , BaseOp             , (0b11010101000000110010001010011111)                                                  , 0         , 0                         , 9  ), // #83
INST(Csel             , BaseCSel           , (0b00011010100000000000000000000000)                                                  , RWI_W    , 0                         , 0  ), // #84
INST(Cset             , BaseCSet           , (0b00011010100111110000011111100000)                                                  , RWI_W    , 0                         , 0  ), // #85
INST(Csetm            , BaseCSet           , (0b01011010100111110000001111100000)                                                  , RWI_W    , 0                         , 1  ), // #86
INST(Csinc            , BaseCSel           , (0b00011010100000000000010000000000)                                                  , RWI_W    , 0                         , 1  ), // #87
INST(Csinv            , BaseCSel           , (0b01011010100000000000000000000000)                                                  , RWI_W    , 0                         , 2  ), // #88
INST(Csneg            , BaseCSel           , (0b01011010100000000000010000000000)                                                  , RWI_W    , 0                         , 3  ), // #89
INST(Ctz              , BaseRR             , (0b01011010110000000001100000000000, kWX, kZR, 0, kWX, kZR, 5, true)                  , RWI_W    , 0                         , 9  ), // #90
INST(Dc               , BaseAtDcIcTlbi     , (0b00011110000000, 0b00001110000000, true)                                            , RWI_RX   , 0                         , 1  ), // #91
INST(Dcps1            , BaseOpImm          , (0b11010100101000000000000000000001, 16, 5)                                           , 0         , 0                         , 3  ), // #92
INST(Dcps2            , BaseOpImm          , (0b11010100101000000000000000000010, 16, 5)                                           , 0         , 0                         , 4  ), // #93
INST(Dcps3            , BaseOpImm          , (0b11010100101000000000000000000011, 16, 5)                                           , 0         , 0                         , 5  ), // #94
INST(Dgh              , BaseOp             , (0b11010101000000110010000011011111)                                                  , 0         , 0                         , 10 ), // #95
INST(Dmb              , BaseOpImm          , (0b11010101000000110011000010111111, 4, 8)                                            , 0         , 0                         , 6  ), // #96
INST(Drps             , BaseOp             , (0b11010110101111110000001111100000)                                                  , 0         , 0                         , 11 ), // #97
INST(Dsb              , BaseOpImm          , (0b11010101000000110011000010011111, 4, 8)                                            , 0         , 0                         , 7  ), // #98
INST(Eon              , BaseLogical        , (0b1001010001, 0b10100100, 1)                                                         , RWI_W    , 0                         , 4  ), // #99
INST(Eor              , BaseLogical        , (0b1001010000, 0b10100100, 0)                                                         , RWI_W    , 0                         , 5  ), // #100
INST(Esb              , BaseOp             , (0b11010101000000110010001000011111)                                                  , 0         , 0                         , 12 ), // #101
INST(Extr             , BaseExtract        , (0b00010011100000000000000000000000)                                                  , RWI_W    , 0                         , 0  ), // #102
INST(Eret             , BaseOp             , (0b11010110100111110000001111100000)                                                  , 0         , 0                         , 13 ), // #103
INST(Gmi              , BaseRRR            , (0b1001101011000000000101, kX , kZR, kX , kSP, kX , kZR, true)                        , RWI_W    , 0                         , 10 ), // #104
INST(Hint             , BaseOpImm          , (0b11010101000000110010000000011111, 7, 5)                                            , 0         , 0                         , 8  ), // #105
INST(Hlt              , BaseOpImm          , (0b11010100010000000000000000000000, 16, 5)                                           , 0         , 0                         , 9  ), // #106
INST(Hvc              , BaseOpImm          , (0b11010100000000000000000000000010, 16, 5)                                           , 0         , 0                         , 10 ), // #107
INST(Ic               , BaseAtDcIcTlbi     , (0b00011110000000, 0b00001110000000, false)                                           , RWI_RX   , 0                         , 2  ), // #108
INST(Isb              , BaseOpImm          , (0b11010101000000110011000011011111, 4, 8)                                            , 0         , 0                         , 11 ), // #109
INST(Ldadd            , BaseAtomicOp       , (0b1011100000100000000000, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 12 ), // #110
INST(Ldadda           , BaseAtomicOp       , (0b1011100010100000000000, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 13 ), // #111
INST(Ldaddab          , BaseAtomicOp       , (0b0011100010100000000000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 14 ), // #112
INST(Ldaddah          , BaseAtomicOp       , (0b0111100010100000000000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 15 ), // #113
INST(Ldaddal          , BaseAtomicOp       , (0b1011100011100000000000, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 16 ), // #114
INST(Ldaddalb         , BaseAtomicOp       , (0b0011100011100000000000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 17 ), // #115
INST(Ldaddalh         , BaseAtomicOp       , (0b0111100011100000000000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 18 ), // #116
INST(Ldaddb           , BaseAtomicOp       , (0b0011100000100000000000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 19 ), // #117
INST(Ldaddh           , BaseAtomicOp       , (0b0111100000100000000000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 20 ), // #118
INST(Ldaddl           , BaseAtomicOp       , (0b1011100001100000000000, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 21 ), // #119
INST(Ldaddlb          , BaseAtomicOp       , (0b0011100001100000000000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 22 ), // #120
INST(Ldaddlh          , BaseAtomicOp       , (0b0111100001100000000000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 23 ), // #121
INST(Ldar             , BaseRMNoImm       , (0b1000100011011111111111, kWX, kZR, 30)                                              , RWI_W    , 0                         , 0  ), // #122
INST(Ldarb            , BaseRMNoImm       , (0b0000100011011111111111, kW , kZR, 0 )                                              , RWI_W    , 0                         , 1  ), // #123
INST(Ldarh            , BaseRMNoImm       , (0b0100100011011111111111, kW , kZR, 0 )                                              , RWI_W    , 0                         , 2  ), // #124
INST(Ldaxp            , BaseLdxp           , (0b1000100001111111100000, kWX, 30)                                                   , RWI_WW   , 0                         , 0  ), // #125
INST(Ldaxr            , BaseRMNoImm       , (0b1000100001011111111111, kWX, kZR, 30)                                              , RWI_W    , 0                         , 3  ), // #126
INST(Ldaxrb           , BaseRMNoImm       , (0b0000100001011111111111, kW , kZR, 0 )                                              , RWI_W    , 0                         , 4  ), // #127
INST(Ldaxrh           , BaseRMNoImm       , (0b0100100001011111111111, kW , kZR, 0 )                                              , RWI_W    , 0                         , 5  ), // #128
INST(Ldclr            , BaseAtomicOp       , (0b1011100000100000000100, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 24 ), // #129
INST(Ldclra           , BaseAtomicOp       , (0b1011100010100000000100, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 25 ), // #130
INST(Ldclrab          , BaseAtomicOp       , (0b0011100010100000000100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 26 ), // #131
INST(Ldclrah          , BaseAtomicOp       , (0b0111100010100000000100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 27 ), // #132
INST(Ldclral          , BaseAtomicOp       , (0b1011100011100000000100, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 28 ), // #133
INST(Ldclralb         , BaseAtomicOp       , (0b0011100011100000000100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 29 ), // #134
INST(Ldclralh         , BaseAtomicOp       , (0b0111100011100000000100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 30 ), // #135
INST(Ldclrb           , BaseAtomicOp       , (0b0011100000100000000100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 31 ), // #136
INST(Ldclrh           , BaseAtomicOp       , (0b0111100000100000000100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 32 ), // #137
INST(Ldclrl           , BaseAtomicOp       , (0b1011100001100000000100, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 33 ), // #138
INST(Ldclrlb          , BaseAtomicOp       , (0b0011100001100000000100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 34 ), // #139
INST(Ldclrlh          , BaseAtomicOp       , (0b0111100001100000000100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 35 ), // #140
INST(Ldeor            , BaseAtomicOp       , (0b1011100000100000001000, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 36 ), // #141
INST(Ldeora           , BaseAtomicOp       , (0b1011100010100000001000, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 37 ), // #142
INST(Ldeorab          , BaseAtomicOp       , (0b0011100010100000001000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 38 ), // #143
INST(Ldeorah          , BaseAtomicOp       , (0b0111100010100000001000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 39 ), // #144
INST(Ldeoral          , BaseAtomicOp       , (0b1011100011100000001000, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 40 ), // #145
INST(Ldeoralb         , BaseAtomicOp       , (0b0011100011100000001000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 41 ), // #146
INST(Ldeoralh         , BaseAtomicOp       , (0b0111100011100000001000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 42 ), // #147
INST(Ldeorb           , BaseAtomicOp       , (0b0011100000100000001000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 43 ), // #148
INST(Ldeorh           , BaseAtomicOp       , (0b0111100000100000001000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 44 ), // #149
INST(Ldeorl           , BaseAtomicOp       , (0b1011100001100000001000, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 45 ), // #150
INST(Ldeorlb          , BaseAtomicOp       , (0b0011100001100000001000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 46 ), // #151
INST(Ldeorlh          , BaseAtomicOp       , (0b0111100001100000001000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 47 ), // #152
INST(Ldg              , BaseRMSImm9       , (0b1101100101100000000000, 0b0000000000000000000000, kX , kZR, 0, 4)                  , RWI_W    , 0                         , 0  ), // #153
INST(Ldgm             , BaseRMNoImm       , (0b1101100111100000000000, kX , kZR, 0 )                                              , RWI_W    , 0                         , 6  ), // #154
INST(Ldlar            , BaseRMNoImm       , (0b1000100011011111011111, kWX, kZR, 30)                                              , RWI_W    , 0                         , 7  ), // #155
INST(Ldlarb           , BaseRMNoImm       , (0b0000100011011111011111, kW , kZR, 0 )                                              , RWI_W    , 0                         , 8  ), // #156
INST(Ldlarh           , BaseRMNoImm       , (0b0100100011011111011111, kW , kZR, 0 )                                              , RWI_W    , 0                         , 9  ), // #157
INST(Ldnp             , BaseLdpStp         , (0b0010100001, 0           , kWX, 31, 2)                                              , RWI_WW   , 0                         , 0  ), // #158
INST(Ldp              , BaseLdpStp         , (0b0010100101, 0b0010100011, kWX, 31, 2)                                              , RWI_WW   , 0                         , 1  ), // #159
INST(Ldpsw            , BaseLdpStp         , (0b0110100101, 0b0110100011, kX , 0 , 2)                                              , RWI_WW   , 0                         , 2  ), // #160
INST(Ldr              , BaseLdSt           , (0b1011100101, 0b10111000010, 0b10111000011, 0b00011000, kWX, 30, 2, InstId::Ldur)   , RWI_W    , 0                         , 0  ), // #161
INST(Ldraa            , BaseRMSImm10      , (0b1111100000100000000001, kX , kZR, 0, 3)                                            , RWI_W    , 0                         , 0  ), // #162
INST(Ldrab            , BaseRMSImm10      , (0b1111100010100000000001, kX , kZR, 0, 3)                                            , RWI_W    , 0                         , 1  ), // #163
INST(Ldrb             , BaseLdSt           , (0b0011100101, 0b00111000010, 0b00111000011, 0         , kW , 0 , 0, InstId::Ldurb)  , RWI_W    , 0                         , 1  ), // #164
INST(Ldrh             , BaseLdSt           , (0b0111100101, 0b01111000010, 0b01111000011, 0         , kW , 0 , 1, InstId::Ldurh)  , RWI_W    , 0                         , 2  ), // #165
INST(Ldrsb            , BaseLdSt           , (0b0011100111, 0b00111000100, 0b00111000111, 0         , kWX, 22, 0, InstId::Ldursb) , RWI_W    , 0                         , 3  ), // #166
INST(Ldrsh            , BaseLdSt           , (0b0111100111, 0b01111000100, 0b01111000111, 0         , kWX, 22, 1, InstId::Ldursh) , RWI_W    , 0                         , 4  ), // #167
INST(Ldrsw            , BaseLdSt           , (0b1011100110, 0b10111000100, 0b10111000101, 0b10011000, kX , 0 , 2, InstId::Ldursw) , RWI_W    , 0                         , 5  ), // #168
INST(Ldset            , BaseAtomicOp       , (0b1011100000100000001100, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 48 ), // #169
INST(Ldseta           , BaseAtomicOp       , (0b1011100010100000001100, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 49 ), // #170
INST(Ldsetab          , BaseAtomicOp       , (0b0011100010100000001100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 50 ), // #171
INST(Ldsetah          , BaseAtomicOp       , (0b0111100010100000001100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 51 ), // #172
INST(Ldsetal          , BaseAtomicOp       , (0b1011100011100000001100, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 52 ), // #173
INST(Ldsetalb         , BaseAtomicOp       , (0b0011100011100000001100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 53 ), // #174
INST(Ldsetalh         , BaseAtomicOp       , (0b0111100011100000001100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 54 ), // #175
INST(Ldsetb           , BaseAtomicOp       , (0b0011100000100000001100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 55 ), // #176
INST(Ldseth           , BaseAtomicOp       , (0b0111100000100000001100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 56 ), // #177
INST(Ldsetl           , BaseAtomicOp       , (0b1011100001100000001100, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 57 ), // #178
INST(Ldsetlb          , BaseAtomicOp       , (0b0011100001100000001100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 58 ), // #179
INST(Ldsetlh          , BaseAtomicOp       , (0b0111100001100000001100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 59 ), // #180
INST(Ldsmax           , BaseAtomicOp       , (0b1011100000100000010000, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 60 ), // #181
INST(Ldsmaxa          , BaseAtomicOp       , (0b1011100010100000010000, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 61 ), // #182
INST(Ldsmaxab         , BaseAtomicOp       , (0b0011100010100000010000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 62 ), // #183
INST(Ldsmaxah         , BaseAtomicOp       , (0b0111100010100000010000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 63 ), // #184
INST(Ldsmaxal         , BaseAtomicOp       , (0b1011100011100000010000, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 64 ), // #185
INST(Ldsmaxalb        , BaseAtomicOp       , (0b0011100011100000010000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 65 ), // #186
INST(Ldsmaxalh        , BaseAtomicOp       , (0b0111100011100000010000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 66 ), // #187
INST(Ldsmaxb          , BaseAtomicOp       , (0b0011100000100000010000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 67 ), // #188
INST(Ldsmaxh          , BaseAtomicOp       , (0b0111100000100000010000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 68 ), // #189
INST(Ldsmaxl          , BaseAtomicOp       , (0b1011100001100000010000, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 69 ), // #190
INST(Ldsmaxlb         , BaseAtomicOp       , (0b0011100001100000010000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 70 ), // #191
INST(Ldsmaxlh         , BaseAtomicOp       , (0b0111100001100000010000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 71 ), // #192
INST(Ldsmin           , BaseAtomicOp       , (0b1011100000100000010100, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 72 ), // #193
INST(Ldsmina          , BaseAtomicOp       , (0b1011100010100000010100, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 73 ), // #194
INST(Ldsminab         , BaseAtomicOp       , (0b0011100010100000010100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 74 ), // #195
INST(Ldsminah         , BaseAtomicOp       , (0b0111100010100000010100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 75 ), // #196
INST(Ldsminal         , BaseAtomicOp       , (0b1011100011100000010100, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 76 ), // #197
INST(Ldsminalb        , BaseAtomicOp       , (0b0011100011100000010100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 77 ), // #198
INST(Ldsminalh        , BaseAtomicOp       , (0b0111100011100000010100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 78 ), // #199
INST(Ldsminb          , BaseAtomicOp       , (0b0011100000100000010100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 79 ), // #200
INST(Ldsminh          , BaseAtomicOp       , (0b0111100000100000010100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 80 ), // #201
INST(Ldsminl          , BaseAtomicOp       , (0b1011100001100000010100, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 81 ), // #202
INST(Ldsminlb         , BaseAtomicOp       , (0b0011100001100000010100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 82 ), // #203
INST(Ldsminlh         , BaseAtomicOp       , (0b0111100001100000010100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 83 ), // #204
INST(Ldtr             , BaseRMSImm9       , (0b1011100001000000000010, 0b0000000000000000000000, kWX, kZR, 30, 0)                 , RWI_W    , 0                         , 1  ), // #205
INST(Ldtrb            , BaseRMSImm9       , (0b0011100001000000000010, 0b0000000000000000000000, kW , kZR, 0 , 0)                 , RWI_W    , 0                         , 2  ), // #206
INST(Ldtrh            , BaseRMSImm9       , (0b0111100001000000000010, 0b0000000000000000000000, kW , kZR, 0 , 0)                 , RWI_W    , 0                         , 3  ), // #207
INST(Ldtrsb           , BaseRMSImm9       , (0b0011100011000000000010, 0b0000000000000000000000, kWX, kZR, 22, 0)                 , RWI_W    , 0                         , 4  ), // #208
INST(Ldtrsh           , BaseRMSImm9       , (0b0111100011000000000010, 0b0000000000000000000000, kWX, kZR, 22, 0)                 , RWI_W    , 0                         , 5  ), // #209
INST(Ldtrsw           , BaseRMSImm9       , (0b1011100010000000000010, 0b0000000000000000000000, kX , kZR, 0 , 0)                 , RWI_W    , 0                         , 6  ), // #210
INST(Ldumax           , BaseAtomicOp       , (0b1011100000100000011000, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 84 ), // #211
INST(Ldumaxa          , BaseAtomicOp       , (0b1011100010100000011000, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 85 ), // #212
INST(Ldumaxab         , BaseAtomicOp       , (0b0011100010100000011000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 86 ), // #213
INST(Ldumaxah         , BaseAtomicOp       , (0b0111100010100000011000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 87 ), // #214
INST(Ldumaxal         , BaseAtomicOp       , (0b1011100011100000011000, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 88 ), // #215
INST(Ldumaxalb        , BaseAtomicOp       , (0b0011100011100000011000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 89 ), // #216
INST(Ldumaxalh        , BaseAtomicOp       , (0b0111100011100000011000, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 90 ), // #217
INST(Ldumaxb          , BaseAtomicOp       , (0b0011100000100000011000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 91 ), // #218
INST(Ldumaxh          , BaseAtomicOp       , (0b0111100000100000011000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 92 ), // #219
INST(Ldumaxl          , BaseAtomicOp       , (0b1011100001100000011000, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 93 ), // #220
INST(Ldumaxlb         , BaseAtomicOp       , (0b0011100001100000011000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 94 ), // #221
INST(Ldumaxlh         , BaseAtomicOp       , (0b0111100001100000011000, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 95 ), // #222
INST(Ldumin           , BaseAtomicOp       , (0b1011100000100000011100, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 96 ), // #223
INST(Ldumina          , BaseAtomicOp       , (0b1011100010100000011100, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 97 ), // #224
INST(Lduminab         , BaseAtomicOp       , (0b0011100010100000011100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 98 ), // #225
INST(Lduminah         , BaseAtomicOp       , (0b0111100010100000011100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 99 ), // #226
INST(Lduminal         , BaseAtomicOp       , (0b1011100011100000011100, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 100), // #227
INST(Lduminalb        , BaseAtomicOp       , (0b0011100011100000011100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 101), // #228
INST(Lduminalh        , BaseAtomicOp       , (0b0111100011100000011100, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 102), // #229
INST(Lduminb          , BaseAtomicOp       , (0b0011100000100000011100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 103), // #230
INST(Lduminh          , BaseAtomicOp       , (0b0111100000100000011100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 104), // #231
INST(Lduminl          , BaseAtomicOp       , (0b1011100001100000011100, kWX, 30, 0)                                                , RWI_WRX  , 0                         , 105), // #232
INST(Lduminlb         , BaseAtomicOp       , (0b0011100001100000011100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 106), // #233
INST(Lduminlh         , BaseAtomicOp       , (0b0111100001100000011100, kW , 0 , 0)                                                , RWI_WRX  , 0                         , 107), // #234
INST(Ldur             , BaseRMSImm9       , (0b1011100001000000000000, 0b0000000000000000000000, kWX, kZR, 30, 0)                 , RWI_W    , 0                         , 7  ), // #235
INST(Ldurb            , BaseRMSImm9       , (0b0011100001000000000000, 0b0000000000000000000000, kW , kZR, 0 , 0)                 , RWI_W    , 0                         , 8  ), // #236
INST(Ldurh            , BaseRMSImm9       , (0b0111100001000000000000, 0b0000000000000000000000, kW , kZR, 0 , 0)                 , RWI_W    , 0                         , 9  ), // #237
INST(Ldursb           , BaseRMSImm9       , (0b0011100011000000000000, 0b0000000000000000000000, kWX, kZR, 22, 0)                 , RWI_W    , 0                         , 10 ), // #238
INST(Ldursh           , BaseRMSImm9       , (0b0111100011000000000000, 0b0000000000000000000000, kWX, kZR, 22, 0)                 , RWI_W    , 0                         , 11 ), // #239
INST(Ldursw           , BaseRMSImm9       , (0b1011100010000000000000, 0b0000000000000000000000, kX , kZR, 0 , 0)                 , RWI_W    , 0                         , 12 ), // #240
INST(Ldxp             , BaseLdxp           , (0b1000100001111111000000, kWX, 30)                                                   , RWI_WW   , 0                         , 1  ), // #241
INST(Ldxr             , BaseRMNoImm       , (0b1000100001011111011111, kWX, kZR, 30)                                              , RWI_W    , 0                         , 10 ), // #242
INST(Ldxrb            , BaseRMNoImm       , (0b0000100001011111011111, kW , kZR, 0 )                                              , RWI_W    , 0                         , 11 ), // #243
INST(Ldxrh            , BaseRMNoImm       , (0b0100100001011111011111, kW , kZR, 0 )                                              , RWI_W    , 0                         , 12 ), // #244
INST(Lsl              , BaseShift          , (0b0001101011000000001000, 0b0101001100000000000000, 0)                               , RWI_W    , 0                         , 2  ), // #245
INST(Lslv             , BaseShift          , (0b0001101011000000001000, 0b0000000000000000000000, 0)                               , RWI_W    , 0                         , 3  ), // #246
INST(Lsr              , BaseShift          , (0b0001101011000000001001, 0b0101001100000000011111, 0)                               , RWI_W    , 0                         , 4  ), // #247
INST(Lsrv             , BaseShift          , (0b0001101011000000001001, 0b0000000000000000000000, 0)                               , RWI_W    , 0                         , 5  ), // #248
INST(Madd             , BaseRRRR           , (0b0001101100000000000000, kWX, kZR, kWX, kZR, kWX, kZR, kWX, kZR, true)              , RWI_W    , 0                         , 0  ), // #249
INST(Mneg             , BaseRRR            , (0b0001101100000000111111, kWX, kZR, kWX, kZR, kWX, kZR, true)                        , RWI_W    , 0                         , 11 ), // #250
INST(Mov              , BaseMov            , (_)                                                                                   , RWI_W    , 0                         , 0  ), // #251
INST(Movk             , BaseMovKNZ         , (0b01110010100000000000000000000000)                                                  , RWI_X    , 0                         , 0  ), // #252
INST(Movn             , BaseMovKNZ         , (0b00010010100000000000000000000000)                                                  , RWI_W    , 0                         , 1  ), // #253
INST(Movz             , BaseMovKNZ         , (0b01010010100000000000000000000000)                                                  , RWI_W    , 0                         , 2  ), // #254
INST(Mrs              , BaseMrs            , (_)                                                                                   , RWI_W    , 0                         , 0  ), // #255
INST(Msr              , BaseMsr            , (_)                                                                                   , RWI_W    , 0                         , 0  ), // #256
INST(Msub             , BaseRRRR           , (0b0001101100000000100000, kWX, kZR, kWX, kZR, kWX, kZR, kWX, kZR, true)              , RWI_W    , 0                         , 1  ), // #257
INST(Mul              , BaseRRR            , (0b0001101100000000011111, kWX, kZR, kWX, kZR, kWX, kZR, true)                        , RWI_W    , 0                         , 12 ), // #258
INST(Mvn              , BaseMvnNeg         , (0b00101010001000000000001111100000)                                                  , RWI_W    , 0                         , 0  ), // #259
INST(Neg              , BaseMvnNeg         , (0b01001011000000000000001111100000)                                                  , RWI_W    , 0                         , 1  ), // #260
INST(Negs             , BaseMvnNeg         , (0b01101011000000000000001111100000)                                                  , RWI_W    , 0                         , 2  ), // #261
INST(Ngc              , BaseRR             , (0b01011010000000000000001111100000, kWX, kZR, 0, kWX, kZR, 16, true)                 , RWI_W    , 0                         , 10 ), // #262
INST(Ngcs             , BaseRR             , (0b01111010000000000000001111100000, kWX, kZR, 0, kWX, kZR, 16, true)                 , RWI_W    , 0                         , 11 ), // #263
INST(Nop              , BaseOp             , (0b11010101000000110010000000011111)                                                  , 0         , 0                         , 14 ), // #264
INST(Orn              , BaseLogical        , (0b0101010001, 0b01100100, 1)                                                         , RWI_W    , 0                         , 6  ), // #265
INST(Orr              , BaseLogical        , (0b0101010000, 0b01100100, 0)                                                         , RWI_W    , 0                         , 7  ), // #266
INST(Pacda            , BaseRR             , (0b11011010110000010000100000000000, kX, kZR, 0, kX, kSP, 5, true)                    , RWI_X    , 0                         , 12 ), // #267
INST(Pacdb            , BaseRR             , (0b11011010110000010000110000000000, kX, kZR, 0, kX, kSP, 5, true)                    , RWI_X    , 0                         , 13 ), // #268
INST(Pacdza           , BaseR              , (0b11011010110000010010101111100000, kX, kZR, 0)                                      , RWI_X    , 0                         , 4  ), // #269
INST(Pacdzb           , BaseR              , (0b11011010110000010010111111100000, kX, kZR, 0)                                      , RWI_X    , 0                         , 5  ), // #270
INST(Pacga            , BaseRRR            , (0b1001101011000000001100, kX, kZR, kX, kZR, kX, kSP, false)                          , RWI_W    , 0                         , 13 ), // #271
INST(Prfm             , BasePrfm           , (0b11111000101, 0b1111100110, 0b11111000100, 0b11011000)                              , RWI_R    , 0                         , 0  ), // #272
INST(Pssbb            , BaseOp             , (0b11010101000000110011010010011111)                                                  , 0         , 0                         , 15 ), // #273
INST(Rbit             , BaseRR             , (0b01011010110000000000000000000000, kWX, kZR, 0, kWX, kZR, 5, true)                  , RWI_W    , 0                         , 14 ), // #274
INST(Ret              , BaseBranchReg      , (0b11010110010111110000000000000000)                                                  , RWI_R    , 0                         , 2  ), // #275
INST(Rev              , BaseRev            , (_)                                                                                   , RWI_W    , 0                         , 0  ), // #276
INST(Rev16            , BaseRR             , (0b01011010110000000000010000000000, kWX, kZR, 0, kWX, kZR, 5, true)                  , RWI_W    , 0                         , 15 ), // #277
INST(Rev32            , BaseRR             , (0b11011010110000000000100000000000, kWX, kZR, 0, kWX, kZR, 5, true)                  , RWI_W    , 0                         , 16 ), // #278
INST(Rev64            , BaseRR             , (0b11011010110000000000110000000000, kWX, kZR, 0, kWX, kZR, 5, true)                  , RWI_W    , 0                         , 17 ), // #279
INST(Ror              , BaseShift          , (0b0001101011000000001011, 0b0001001110000000000000, 1)                               , RWI_W    , 0                         , 6  ), // #280
INST(Rorv             , BaseShift          , (0b0001101011000000001011, 0b0000000000000000000000, 1)                               , RWI_W    , 0                         , 7  ), // #281
INST(Sbc              , BaseRRR            , (0b0101101000000000000000, kWX, kZR, kWX, kZR, kWX, kZR, true)                        , RWI_W    , 0                         , 14 ), // #282
INST(Sbcs             , BaseRRR            , (0b0111101000000000000000, kWX, kZR, kWX, kZR, kWX, kZR, true)                        , RWI_W    , 0                         , 15 ), // #283
INST(Sbfiz            , BaseBfi            , (0b00010011000000000000000000000000)                                                  , RWI_W    , 0                         , 1  ), // #284
INST(Sbfm             , BaseBfm            , (0b00010011000000000000000000000000)                                                  , RWI_W    , 0                         , 1  ), // #285
INST(Sbfx             , BaseBfx            , (0b00010011000000000000000000000000)                                                  , RWI_W    , 0                         , 1  ), // #286
INST(Sdiv             , BaseRRR            , (0b0001101011000000000011, kWX, kZR, kWX, kZR, kWX, kZR, true)                        , RWI_W    , 0                         , 16 ), // #287
INST(Setf8            , BaseR              , (0b00111010000000000000100000001101, kW, kZR, 5)                                      , 0         , 0                         , 6  ), // #288
INST(Setf16           , BaseR              , (0b00111010000000000100100000001101, kW, kZR, 5)                                      , 0         , 0                         , 7  ), // #289
INST(Sev              , BaseOp             , (0b11010101000000110010000010011111)                                                  , 0         , 0                         , 16 ), // #290
INST(Sevl             , BaseOp             , (0b11010101000000110010000010111111)                                                  , 0         , 0                         , 17 ), // #291
INST(Smaddl           , BaseRRRR           , (0b1001101100100000000000, kX , kZR, kW , kZR, kW , kZR, kX , kZR, false)             , RWI_W    , 0                         , 2  ), // #292
INST(Smax             , BaseMinMax         , (0b00011010110000000110000000000000, 0b00010001110000000000000000000000)              , RWI_W    , 0                         , 0  ), // #293
INST(Smc              , BaseOpImm          , (0b11010100000000000000000000000011, 16, 5)                                           , 0         , 0                         , 12 ), // #294
INST(Smin             , BaseMinMax         , (0b00011010110000000110100000000000, 0b00010001110010000000000000000000)              , RWI_W    , 0                         , 1  ), // #295
INST(Smnegl           , BaseRRR            , (0b1001101100100000111111, kX , kZR, kW , kZR, kW , kZR, false)                       , RWI_W    , 0                         , 17 ), // #296
INST(Smsubl           , BaseRRRR           , (0b1001101100100000100000, kX , kZR, kW , kZR, kW , kZR, kX , kZR, false)             , RWI_W    , 0                         , 3  ), // #297
INST(Smulh            , BaseRRR            , (0b1001101101000000011111, kX , kZR, kX , kZR, kX , kZR, true)                        , RWI_W    , 0                         , 18 ), // #298
INST(Smull            , BaseRRR            , (0b1001101100100000011111, kX , kZR, kW , kZR, kW , kZR, false)                       , RWI_W    , 0                         , 19 ), // #299
INST(Ssbb             , BaseOp             , (0b11010101000000110011000010011111)                                                  , 0         , 0                         , 18 ), // #300
INST(St2g             , BaseRMSImm9       , (0b1101100110100000000010, 0b1101100110100000000001, kX, kSP, 0, 4)                   , RWI_RW   , 0                         , 13 ), // #301
INST(Stadd            , BaseAtomicSt       , (0b1011100000100000000000, kWX, 30)                                                   , RWI_RX   , 0                         , 0  ), // #302
INST(Staddl           , BaseAtomicSt       , (0b1011100001100000000000, kWX, 30)                                                   , RWI_RX   , 0                         , 1  ), // #303
INST(Staddb           , BaseAtomicSt       , (0b0011100000100000000000, kW , 0 )                                                   , RWI_RX   , 0                         , 2  ), // #304
INST(Staddlb          , BaseAtomicSt       , (0b0011100001100000000000, kW , 0 )                                                   , RWI_RX   , 0                         , 3  ), // #305
INST(Staddh           , BaseAtomicSt       , (0b0111100000100000000000, kW , 0 )                                                   , RWI_RX   , 0                         , 4  ), // #306
INST(Staddlh          , BaseAtomicSt       , (0b0111100001100000000000, kW , 0 )                                                   , RWI_RX   , 0                         , 5  ), // #307
INST(Stclr            , BaseAtomicSt       , (0b1011100000100000000100, kWX, 30)                                                   , RWI_RX   , 0                         , 6  ), // #308
INST(Stclrl           , BaseAtomicSt       , (0b1011100001100000000100, kWX, 30)                                                   , RWI_RX   , 0                         , 7  ), // #309
INST(Stclrb           , BaseAtomicSt       , (0b0011100000100000000100, kW , 0 )                                                   , RWI_RX   , 0                         , 8  ), // #310
INST(Stclrlb          , BaseAtomicSt       , (0b0011100001100000000100, kW , 0 )                                                   , RWI_RX   , 0                         , 9  ), // #311
INST(Stclrh           , BaseAtomicSt       , (0b0111100000100000000100, kW , 0 )                                                   , RWI_RX   , 0                         , 10 ), // #312
INST(Stclrlh          , BaseAtomicSt       , (0b0111100001100000000100, kW , 0 )                                                   , RWI_RX   , 0                         , 11 ), // #313
INST(Steor            , BaseAtomicSt       , (0b1011100000100000001000, kWX, 30)                                                   , RWI_RX   , 0                         , 12 ), // #314
INST(Steorl           , BaseAtomicSt       , (0b1011100001100000001000, kWX, 30)                                                   , RWI_RX   , 0                         , 13 ), // #315
INST(Steorb           , BaseAtomicSt       , (0b0011100000100000001000, kW , 0 )                                                   , RWI_RX   , 0                         , 14 ), // #316
INST(Steorlb          , BaseAtomicSt       , (0b0011100001100000001000, kW , 0 )                                                   , RWI_RX   , 0                         , 15 ), // #317
INST(Steorh           , BaseAtomicSt       , (0b0111100000100000001000, kW , 0 )                                                   , RWI_RX   , 0                         , 16 ), // #318
INST(Steorlh          , BaseAtomicSt       , (0b0111100001100000001000, kW , 0 )                                                   , RWI_RX   , 0                         , 17 ), // #319
INST(Stg              , BaseRMSImm9       , (0b1101100100100000000010, 0b1101100100100000000001, kX, kSP, 0, 4)                   , RWI_RW   , 0                         , 14 ), // #320
INST(Stgm             , BaseRMNoImm       , (0b1101100110100000000000, kX , kZR, 0 )                                              , RWI_RW   , 0                         , 13 ), // #321
INST(Stgp             , BaseLdpStp         , (0b0110100100, 0b0110100010, kX, 0, 4)                                                , RWI_RRW  , 0                         , 3  ), // #322
INST(Stllr            , BaseRMNoImm       , (0b1000100010011111011111, kWX, kZR, 30)                                              , RWI_RW   , 0                         , 14 ), // #323
INST(Stllrb           , BaseRMNoImm       , (0b0000100010011111011111, kW , kZR, 0 )                                              , RWI_RW   , 0                         , 15 ), // #324
INST(Stllrh           , BaseRMNoImm       , (0b0100100010011111011111, kW , kZR, 0 )                                              , RWI_RW   , 0                         , 16 ), // #325
INST(Stlr             , BaseRMNoImm       , (0b1000100010011111111111, kWX, kZR, 30)                                              , RWI_RW   , 0                         , 17 ), // #326
INST(Stlrb            , BaseRMNoImm       , (0b0000100010011111111111, kW , kZR, 0 )                                              , RWI_RW   , 0                         , 18 ), // #327
INST(Stlrh            , BaseRMNoImm       , (0b0100100010011111111111, kW , kZR, 0 )                                              , RWI_RW   , 0                         , 19 ), // #328
INST(Stlxp            , BaseStxp           , (0b1000100000100000100000, kWX, 30)                                                   , RWI_WRRX , 0                         , 0  ), // #329
INST(Stlxr            , BaseAtomicOp       , (0b1000100000000000111111, kWX, 30, 1)                                                , RWI_WRX  , 0                         , 108), // #330
INST(Stlxrb           , BaseAtomicOp       , (0b0000100000000000111111, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 109), // #331
INST(Stlxrh           , BaseAtomicOp       , (0b0100100000000000111111, kW , 0 , 1)                                                , RWI_WRX  , 0                         , 110), // #332
INST(Stnp             , BaseLdpStp         , (0b0010100000, 0           , kWX, 31, 2)                                              , RWI_RRW  , 0                         , 4  ), // #333
INST(Stp              , BaseLdpStp         , (0b0010100100, 0b0010100010, kWX, 31, 2)                                              , RWI_RRW  , 0                         , 5  ), // #334
INST(Str              , BaseLdSt           , (0b1011100100, 0b10111000000, 0b10111000001, 0         , kWX, 30, 2, InstId::Stur)   , RWI_RW   , 0                         , 6  ), // #335
INST(Strb             , BaseLdSt           , (0b0011100100, 0b00111000000, 0b00111000001, 0         , kW , 30, 0, InstId::Sturb)  , RWI_RW   , 0                         , 7  ), // #336
INST(Strh             , BaseLdSt           , (0b0111100100, 0b01111000000, 0b01111000001, 0         , kWX, 30, 1, InstId::Sturh)  , RWI_RW   , 0                         , 8  ), // #337
INST(Stset            , BaseAtomicSt       , (0b1011100000100000001100, kWX, 30)                                                   , RWI_RX   , 0                         , 18 ), // #338
INST(Stsetl           , BaseAtomicSt       , (0b1011100001100000001100, kWX, 30)                                                   , RWI_RX   , 0                         , 19 ), // #339
INST(Stsetb           , BaseAtomicSt       , (0b0011100000100000001100, kW , 0 )                                                   , RWI_RX   , 0                         , 20 ), // #340
INST(Stsetlb          , BaseAtomicSt       , (0b0011100001100000001100, kW , 0 )                                                   , RWI_RX   , 0                         , 21 ), // #341
INST(Stseth           , BaseAtomicSt       , (0b0111100000100000001100, kW , 0 )                                                   , RWI_RX   , 0                         , 22 ), // #342
INST(Stsetlh          , BaseAtomicSt       , (0b0111100001100000001100, kW , 0 )                                                   , RWI_RX   , 0                         , 23 ), // #343
INST(Stsmax           , BaseAtomicSt       , (0b1011100000100000010000, kWX, 30)                                                   , RWI_RX   , 0                         , 24 ), // #344
INST(Stsmaxl          , BaseAtomicSt       , (0b1011100001100000010000, kWX, 30)                                                   , RWI_RX   , 0                         , 25 ), // #345
INST(Stsmaxb          , BaseAtomicSt       , (0b0011100000100000010000, kW , 0 )                                                   , RWI_RX   , 0                         , 26 ), // #346
INST(Stsmaxlb         , BaseAtomicSt       , (0b0011100001100000010000, kW , 0 )                                                   , RWI_RX   , 0                         , 27 ), // #347
INST(Stsmaxh          , BaseAtomicSt       , (0b0111100000100000010000, kW , 0 )                                                   , RWI_RX   , 0                         , 28 ), // #348
INST(Stsmaxlh         , BaseAtomicSt       , (0b0111100001100000010000, kW , 0 )                                                   , RWI_RX   , 0                         , 29 ), // #349
INST(Stsmin           , BaseAtomicSt       , (0b1011100000100000010100, kWX, 30)                                                   , RWI_RX   , 0                         , 30 ), // #350
INST(Stsminl          , BaseAtomicSt       , (0b1011100001100000010100, kWX, 30)                                                   , RWI_RX   , 0                         , 31 ), // #351
INST(Stsminb          , BaseAtomicSt       , (0b0011100000100000010100, kW , 0 )                                                   , RWI_RX   , 0                         , 32 ), // #352
INST(Stsminlb         , BaseAtomicSt       , (0b0011100001100000010100, kW , 0 )                                                   , RWI_RX   , 0                         , 33 ), // #353
INST(Stsminh          , BaseAtomicSt       , (0b0111100000100000010100, kW , 0 )                                                   , RWI_RX   , 0                         , 34 ), // #354
INST(Stsminlh         , BaseAtomicSt       , (0b0111100001100000010100, kW , 0 )                                                   , RWI_RX   , 0                         , 35 ), // #355
INST(Sttr             , BaseRMSImm9       , (0b1011100000000000000010, 0b0000000000000000000000, kWX, kZR, 30, 0)                 , RWI_RW   , 0                         , 15 ), // #356
INST(Sttrb            , BaseRMSImm9       , (0b0011100000000000000010, 0b0000000000000000000000, kW , kZR, 0 , 0)                 , RWI_RW   , 0                         , 16 ), // #357
INST(Sttrh            , BaseRMSImm9       , (0b0111100000000000000010, 0b0000000000000000000000, kW , kZR, 0 , 0)                 , RWI_RW   , 0                         , 17 ), // #358
INST(Stumax           , BaseAtomicSt       , (0b1011100000100000011000, kWX, 30)                                                   , RWI_RX   , 0                         , 36 ), // #359
INST(Stumaxl          , BaseAtomicSt       , (0b1011100001100000011000, kWX, 30)                                                   , RWI_RX   , 0                         , 37 ), // #360
INST(Stumaxb          , BaseAtomicSt       , (0b0011100000100000011000, kW , 0 )                                                   , RWI_RX   , 0                         , 38 ), // #361
INST(Stumaxlb         , BaseAtomicSt       , (0b0011100001100000011000, kW , 0 )                                                   , RWI_RX   , 0                         , 39 ), // #362
INST(Stumaxh          , BaseAtomicSt       , (0b0111100000100000011000, kW , 0 )                                                   , RWI_RX   , 0                         , 40 ), // #363
INST(Stumaxlh         , BaseAtomicSt       , (0b0111100001100000011000, kW , 0 )                                                   , RWI_RX   , 0                         , 41 ), // #364
INST(Stumin           , BaseAtomicSt       , (0b1011100000100000011100, kWX, 30)                                                   , RWI_RX   , 0                         , 42 ), // #365
INST(Stuminl          , BaseAtomicSt       , (0b1011100001100000011100, kWX, 30)                                                   , RWI_RX   , 0                         , 43 ), // #366
INST(Stuminb          , BaseAtomicSt       , (0b0011100000100000011100, kW , 0 )                                                   , RWI_RX   , 0                         , 44 ), // #367
INST(Stuminlb         , BaseAtomicSt       , (0b0011100001100000011100, kW , 0 )                                                   , RWI_RX   , 0                         , 45 ), // #368
INST(Stuminh          , BaseAtomicSt       , (0b0111100000100000011100, kW , 0 )                                                   , RWI_RX   , 0                         , 46 ), // #369
INST(Stuminlh         , BaseAtomicSt       , (0b0111100001100000011100, kW , 0 )                                                   , RWI_RX   , 0                         , 47 ), // #370
INST(Stur             , BaseRMSImm9       , (0b1011100000000000000000, 0b0000000000000000000000, kWX, kZR, 30, 0)                 , RWI_RW   , 0                         , 18 ), // #371
INST(Sturb            , BaseRMSImm9       , (0b0011100000000000000000, 0b0000000000000000000000, kW , kZR, 0 , 0)                 , RWI_RW   , 0                         , 19 ), // #372
INST(Sturh            , BaseRMSImm9       , (0b0111100000000000000000, 0b0000000000000000000000, kW , kZR, 0 , 0)                 , RWI_RW   , 0                         , 20 ), // #373
INST(Stxp             , BaseStxp           , (0b1000100000100000000000, kWX, 30)                                                   , RWI_WRRW , 0                         , 1  ), // #374
INST(Stxr             , BaseStx            , (0b1000100000000000011111, kWX, 30)                                                   , RWI_WRW  , 0                         , 0  ), // #375
INST(Stxrb            , BaseStx            , (0b0000100000000000011111, kW , 0 )                                                   , RWI_WRW  , 0                         , 1  ), // #376
INST(Stxrh            , BaseStx            , (0b0100100000000000011111, kW , 0 )                                                   , RWI_WRW  , 0                         , 2  ), // #377
INST(Stz2g            , BaseRMSImm9       , (0b1101100111100000000010, 0b1101100111100000000001, kX , kSP, 0, 4)                  , RWI_RW   , 0                         , 21 ), // #378
INST(Stzg             , BaseRMSImm9       , (0b1101100101100000000010, 0b1101100101100000000001, kX , kSP, 0, 4)                  , RWI_RW   , 0                         , 22 ), // #379
INST(Stzgm            , BaseRMNoImm       , (0b1101100100100000000000, kX , kZR, 0)                                               , RWI_RW   , 0                         , 20 ), // #380
INST(Sub              , BaseAddSub         , (0b1001011000, 0b1001011001, 0b1010001)                                               , RWI_W    , 0                         , 2  ), // #381
INST(Subg             , BaseRRII           , (0b1101000110000000000000, kX, kSP, kX, kSP, 6, 4, 16, 4, 0, 10)                      , RWI_W    , 0                         , 1  ), // #382
INST(Subp             , BaseRRR            , (0b1001101011000000000000, kX, kZR, kX, kSP, kX, kSP, false)                          , RWI_W    , 0                         , 20 ), // #383
INST(Subps            , BaseRRR            , (0b1011101011000000000000, kX, kZR, kX, kSP, kX, kSP, false)                          , RWI_W    , 0                         , 21 ), // #384
INST(Subs             , BaseAddSub         , (0b1101011000, 0b1101011001, 0b1110001)                                               , RWI_W    , 0                         , 3  ), // #385
INST(Svc              , BaseOpImm          , (0b11010100000000000000000000000001, 16, 5)                                           , 0         , 0                         , 13 ), // #386
INST(Swp              , BaseAtomicOp       , (0b1011100000100000100000, kWX, 30, 1)                                                , RWI_RWX  , 0                         , 111), // #387
INST(Swpa             , BaseAtomicOp       , (0b1011100010100000100000, kWX, 30, 1)                                                , RWI_RWX  , 0                         , 112), // #388
INST(Swpab            , BaseAtomicOp       , (0b0011100010100000100000, kW , 0 , 1)                                                , RWI_RWX  , 0                         , 113), // #389
INST(Swpah            , BaseAtomicOp       , (0b0111100010100000100000, kW , 0 , 1)                                                , RWI_RWX  , 0                         , 114), // #390
INST(Swpal            , BaseAtomicOp       , (0b1011100011100000100000, kWX, 30, 1)                                                , RWI_RWX  , 0                         , 115), // #391
INST(Swpalb           , BaseAtomicOp       , (0b0011100011100000100000, kW , 0 , 1)                                                , RWI_RWX  , 0                         , 116), // #392
INST(Swpalh           , BaseAtomicOp       , (0b0111100011100000100000, kW , 0 , 1)                                                , RWI_RWX  , 0                         , 117), // #393
INST(Swpb             , BaseAtomicOp       , (0b0011100000100000100000, kW , 0 , 1)                                                , RWI_RWX  , 0                         , 118), // #394
INST(Swph             , BaseAtomicOp       , (0b0111100000100000100000, kW , 0 , 1)                                                , RWI_RWX  , 0                         , 119), // #395
INST(Swpl             , BaseAtomicOp       , (0b1011100001100000100000, kWX, 30, 1)                                                , RWI_RWX  , 0                         , 120), // #396
INST(Swplb            , BaseAtomicOp       , (0b0011100001100000100000, kW , 0 , 1)                                                , RWI_RWX  , 0                         , 121), // #397
INST(Swplh            , BaseAtomicOp       , (0b0111100001100000100000, kW , 0 , 1)                                                , RWI_RWX  , 0                         , 122), // #398
INST(Sxtb             , BaseExtend         , (0b0001001100000000000111, kWX, 0)                                                    , RWI_W    , 0                         , 0  ), // #399
INST(Sxth             , BaseExtend         , (0b0001001100000000001111, kWX, 0)                                                    , RWI_W    , 0                         , 1  ), // #400
INST(Sxtw             , BaseExtend         , (0b1001001101000000011111, kX , 0)                                                    , RWI_W    , 0                         , 2  ), // #401
INST(Sys              , BaseSys            , (_)                                                                                   , RWI_W    , 0                         , 0  ), // #402
INST(Tlbi             , BaseAtDcIcTlbi     , (0b00011110000000, 0b00010000000000, false)                                           , RWI_RX   , 0                         , 3  ), // #403
INST(Tst              , BaseTst            , (0b1101010000, 0b111001000)                                                           , RWI_R    , 0                         , 0  ), // #404
INST(Tbnz             , BaseBranchTst      , (0b00110111000000000000000000000000)                                                  , RWI_R    , 0                         , 0  ), // #405
INST(Tbz              , BaseBranchTst      , (0b00110110000000000000000000000000)                                                  , RWI_R    , 0                         , 1  ), // #406
INST(Ubfiz            , BaseBfi            , (0b01010011000000000000000000000000)                                                  , RWI_W    , 0                         , 2  ), // #407
INST(Ubfm             , BaseBfm            , (0b01010011000000000000000000000000)                                                  , RWI_W    , 0                         , 2  ), // #408
INST(Ubfx             , BaseBfx            , (0b01010011000000000000000000000000)                                                  , RWI_W    , 0                         , 2  ), // #409
INST(Udf              , BaseOpImm          , (0b00000000000000000000000000000000, 16, 0)                                           , 0         , 0                         , 14 ), // #410
INST(Udiv             , BaseRRR            , (0b0001101011000000000010, kWX, kZR, kWX, kZR, kWX, kZR, true)                        , RWI_W    , 0                         , 22 ), // #411
INST(Umaddl           , BaseRRRR           , (0b1001101110100000000000, kX , kZR, kW , kZR, kW , kZR, kX , kZR, false)             , RWI_W    , 0                         , 4  ), // #412
INST(Umax             , BaseMinMax         , (0b00011010110000000110010000000000, 0b00010001110001000000000000000000)              , RWI_W    , 0                         , 2  ), // #413
INST(Umin             , BaseMinMax         , (0b00011010110000000110110000000000, 0b00010001110011000000000000000000)              , RWI_W    , 0                         , 3  ), // #414
INST(Umnegl           , BaseRRR            , (0b1001101110100000111111, kX , kZR, kW , kZR, kW , kZR, false)                       , RWI_W    , 0                         , 23 ), // #415
INST(Umull            , BaseRRR            , (0b1001101110100000011111, kX , kZR, kW , kZR, kW , kZR, false)                       , RWI_W    , 0                         , 24 ), // #416
INST(Umulh            , BaseRRR            , (0b1001101111000000011111, kX , kZR, kX , kZR, kX , kZR, false)                       , RWI_W    , 0                         , 25 ), // #417
INST(Umsubl           , BaseRRRR           , (0b1001101110100000100000, kX , kZR, kW , kZR, kW , kZR, kX , kZR, false)             , RWI_W    , 0                         , 5  ), // #418
INST(Uxtb             , BaseExtend         , (0b0101001100000000000111, kW, 1)                                                     , RWI_W    , 0                         , 3  ), // #419
INST(Uxth             , BaseExtend         , (0b0101001100000000001111, kW, 1)                                                     , RWI_W    , 0                         , 4  ), // #420
INST(Wfe              , BaseOp             , (0b11010101000000110010000001011111)                                                  , 0         , 0                         , 19 ), // #421
INST(Wfi              , BaseOp             , (0b11010101000000110010000001111111)                                                  , 0         , 0                         , 20 ), // #422
INST(Xaflag           , BaseOp             , (0b11010101000000000100000000111111)                                                  , 0         , 0                         , 21 ), // #423
INST(Xpacd            , BaseR              , (0b11011010110000010100011111100000, kX, kZR, 0)                                      , RWI_X    , 0                         , 8  ), // #424
INST(Xpaci            , BaseR              , (0b11011010110000010100001111100000, kX, kZR, 0)                                      , RWI_X    , 0                         , 9  ), // #425
INST(Xpaclri          , BaseOp             , (0b11010101000000110010000011111111)                                                  , RWI_X    , 0                         , 22 ), // #426
INST(Yield            , BaseOp             , (0b11010101000000110010000000111111)                                                  , 0         , 0                         , 23 ), // #427
INST(Abs_v            , ISimdVV            , (0b0000111000100000101110, kVO_V_Any)                                                 , RWI_W    , 0                         , 0  ), // #428
INST(Add_v            , ISimdVVV           , (0b0000111000100000100001, kVO_V_Any)                                                 , RWI_W    , 0                         , 0  ), // #429
INST(Addhn_v          , ISimdVVV           , (0b0000111000100000010000, kVO_V_B8H4S2)                                              , RWI_W    , F!(Narrow)                 , 1  ), // #430
INST(Addhn2_v         , ISimdVVV           , (0b0100111000100000010000, kVO_V_B16H8S4)                                             , RWI_W    , F!(Narrow)                 , 2  ), // #431
INST(Addp_v           , ISimdPair          , (0b0101111000110001101110, 0b0000111000100000101111, kVO_V_Any)                       , RWI_W    , F!(Pair)                   , 0  ), // #432
INST(Addv_v           , ISimdSV            , (0b0000111000110001101110, kVO_V_BH_4S)                                               , RWI_W    , 0                         , 0  ), // #433
INST(Aesd_v           , ISimdVVx           , (0b0100111000101000010110, kOp_V16B, kOp_V16B)                                        , RWI_X    , 0                         , 0  ), // #434
INST(Aese_v           , ISimdVVx           , (0b0100111000101000010010, kOp_V16B, kOp_V16B)                                        , RWI_X    , 0                         , 1  ), // #435
INST(Aesimc_v         , ISimdVVx           , (0b0100111000101000011110, kOp_V16B, kOp_V16B)                                        , RWI_W    , 0                         , 2  ), // #436
INST(Aesmc_v          , ISimdVVx           , (0b0100111000101000011010, kOp_V16B, kOp_V16B)                                        , RWI_W    , 0                         , 3  ), // #437
INST(And_v            , ISimdVVV           , (0b0000111000100000000111, kVO_V_B)                                                   , RWI_W    , 0                         , 3  ), // #438
INST(Bcax_v           , ISimdVVVV          , (0b1100111000100000000000, kVO_V_B16)                                                 , RWI_W    , 0                         , 0  ), // #439
INST(Bfcvt_v          , ISimdVVx           , (0b0001111001100011010000, kOp_H, kOp_S)                                              , RWI_W    , 0                         , 4  ), // #440
INST(Bfcvtn_v         , ISimdVVx           , (0b0000111010100001011010, kOp_V4H, kOp_V4S)                                          , RWI_W    , F!(Narrow)                 , 5  ), // #441
INST(Bfcvtn2_v        , ISimdVVx           , (0b0100111010100001011010, kOp_V8H, kOp_V4S)                                          , RWI_W    , F!(Narrow)                 , 6  ), // #442
INST(Bfdot_v          , SimdDot            , (0b0010111001000000111111, 0b0000111101000000111100, kET_S, kET_H, kET_2H)            , RWI_X    , 0                         , 0  ), // #443
INST(Bfmlalb_v        , SimdFmlal          , (0b0010111011000000111111, 0b0000111111000000111100, 0, kET_S, kET_H, kET_H)          , RWI_X    , F!(VH0_15)                 , 0  ), // #444
INST(Bfmlalt_v        , SimdFmlal          , (0b0110111011000000111111, 0b0100111111000000111100, 0, kET_S, kET_H, kET_H)          , RWI_X    , F!(VH0_15)                 , 1  ), // #445
INST(Bfmmla_v         , ISimdVVVx          , (0b0110111001000000111011, kOp_V4S, kOp_V8H, kOp_V8H)                                 , RWI_X    , F!(Long)                   , 0  ), // #446
INST(Bic_v            , SimdBicOrr         , (0b0000111001100000000111, 0b0010111100000000000001)                                  , RWI_W    , 0                         , 0  ), // #447
INST(Bif_v            , ISimdVVV           , (0b0010111011100000000111, kVO_V_B)                                                   , RWI_X    , 0                         , 4  ), // #448
INST(Bit_v            , ISimdVVV           , (0b0010111010100000000111, kVO_V_B)                                                   , RWI_X    , 0                         , 5  ), // #449
INST(Bsl_v            , ISimdVVV           , (0b0010111001100000000111, kVO_V_B)                                                   , RWI_X    , 0                         , 6  ), // #450
INST(Cls_v            , ISimdVV            , (0b0000111000100000010010, kVO_V_BHS)                                                 , RWI_W    , 0                         , 1  ), // #451
INST(Clz_v            , ISimdVV            , (0b0010111000100000010010, kVO_V_BHS)                                                 , RWI_W    , 0                         , 2  ), // #452
INST(Cmeq_v           , SimdCmp            , (0b0010111000100000100011, 0b0000111000100000100110, kVO_V_Any)                       , RWI_W    , 0                         , 0  ), // #453
INST(Cmge_v           , SimdCmp            , (0b0000111000100000001111, 0b0010111000100000100010, kVO_V_Any)                       , RWI_W    , 0                         , 1  ), // #454
INST(Cmgt_v           , SimdCmp            , (0b0000111000100000001101, 0b0000111000100000100010, kVO_V_Any)                       , RWI_W    , 0                         , 2  ), // #455
INST(Cmhi_v           , SimdCmp            , (0b0010111000100000001101, 0b0000000000000000000000, kVO_V_Any)                       , RWI_W    , 0                         , 3  ), // #456
INST(Cmhs_v           , SimdCmp            , (0b0010111000100000001111, 0b0000000000000000000000, kVO_V_Any)                       , RWI_W    , 0                         , 4  ), // #457
INST(Cmle_v           , SimdCmp            , (0b0000000000000000000000, 0b0010111000100000100110, kVO_V_Any)                       , RWI_W    , 0                         , 5  ), // #458
INST(Cmlt_v           , SimdCmp            , (0b0000000000000000000000, 0b0000111000100000101010, kVO_V_Any)                       , RWI_W    , 0                         , 6  ), // #459
INST(Cmtst_v          , ISimdVVV           , (0b0000111000100000100011, kVO_V_Any)                                                 , RWI_W    , 0                         , 7  ), // #460
INST(Cnt_v            , ISimdVV            , (0b0000111000100000010110, kVO_V_B)                                                   , RWI_W    , 0                         , 3  ), // #461
INST(Dup_v            , SimdDup            , (_)                                                                                   , RWI_W    , 0                         , 0  ), // #462
INST(Eor_v            , ISimdVVV           , (0b0010111000100000000111, kVO_V_B)                                                   , RWI_W    , 0                         , 8  ), // #463
INST(Eor3_v           , ISimdVVVV          , (0b1100111000000000000000, kVO_V_B16)                                                 , RWI_W    , 0                         , 1  ), // #464
INST(Ext_v            , ISimdVVVI          , (0b0010111000000000000000, kVO_V_B, 4, 11, 1)                                         , RWI_W    , 0                         , 0  ), // #465
INST(Fabd_v           , FSimdVVV           , (0b0111111010100000110101, kHF_C, 0b0010111010100000110101, kHF_C)                    , RWI_W    , 0                         , 0  ), // #466
INST(Fabs_v           , FSimdVV            , (0b0001111000100000110000, kHF_A, 0b0000111010100000111110, kHF_B)                    , RWI_W    , 0                         , 0  ), // #467
INST(Facge_v          , FSimdVVV           , (0b0111111000100000111011, kHF_C, 0b0010111000100000111011, kHF_C)                    , RWI_W    , 0                         , 1  ), // #468
INST(Facgt_v          , FSimdVVV           , (0b0111111010100000111011, kHF_C, 0b0010111010100000111011, kHF_C)                    , RWI_W    , 0                         , 2  ), // #469
INST(Fadd_v           , FSimdVVV           , (0b0001111000100000001010, kHF_A, 0b0000111000100000110101, kHF_C)                    , RWI_W    , 0                         , 3  ), // #470
INST(Faddp_v          , FSimdPair          , (0b0111111000110000110110, 0b0010111000100000110101)                                  , RWI_W    , 0                         , 0  ), // #471
INST(Fcadd_v          , SimdFcadd          , (0b0010111000000000111001)                                                            , RWI_W    , 0                         , 0  ), // #472
INST(Fccmp_v          , SimdFccmpFccmpe    , (0b00011110001000000000010000000000)                                                  , RWI_R    , 0                         , 0  ), // #473
INST(Fccmpe_v         , SimdFccmpFccmpe    , (0b00011110001000000000010000010000)                                                  , RWI_R    , 0                         , 1  ), // #474
INST(Fcmeq_v          , SimdFcm            , (0b0000111000100000111001, kHF_C, 0b0000111010100000110110)                           , RWI_W    , 0                         , 0  ), // #475
INST(Fcmge_v          , SimdFcm            , (0b0010111000100000111001, kHF_C, 0b0010111010100000110010)                           , RWI_W    , 0                         , 1  ), // #476
INST(Fcmgt_v          , SimdFcm            , (0b0010111010100000111001, kHF_C, 0b0000111010100000110010)                           , RWI_W    , 0                         , 2  ), // #477
INST(Fcmla_v          , SimdFcmla          , (0b0010111000000000110001, 0b0010111100000000000100)                                  , RWI_X    , 0                         , 0  ), // #478
INST(Fcmle_v          , SimdFcm            , (0b0000000000000000000000, kHF_C, 0b0010111010100000110110)                           , RWI_W    , 0                         , 3  ), // #479
INST(Fcmlt_v          , SimdFcm            , (0b0000000000000000000000, kHF_C, 0b0000111010100000111010)                           , RWI_W    , 0                         , 4  ), // #480
INST(Fcmp_v           , SimdFcmpFcmpe      , (0b00011110001000000010000000000000)                                                  , RWI_R    , 0                         , 0  ), // #481
INST(Fcmpe_v          , SimdFcmpFcmpe      , (0b00011110001000000010000000010000)                                                  , RWI_R    , 0                         , 1  ), // #482
INST(Fcsel_v          , SimdFcsel          , (_)                                                                                   , RWI_W    , 0                         , 0  ), // #483
INST(Fcvt_v           , SimdFcvt           , (_)                                                                                   , RWI_W    , 0                         , 0  ), // #484
INST(Fcvtas_v         , SimdFcvtSV         , (0b0000111000100001110010, 0b0000000000000000000000, 0b0001111000100100000000, 1)     , RWI_W    , 0                         , 0  ), // #485
INST(Fcvtau_v         , SimdFcvtSV         , (0b0010111000100001110010, 0b0000000000000000000000, 0b0001111000100101000000, 1)     , RWI_W    , 0                         , 1  ), // #486
INST(Fcvtl_v          , SimdFcvtLN         , (0b0000111000100001011110, 0, 0)                                                      , RWI_W    , F!(Long)                   , 0  ), // #487
INST(Fcvtl2_v         , SimdFcvtLN         , (0b0100111000100001011110, 0, 0)                                                      , RWI_W    , F!(Long)                   , 1  ), // #488
INST(Fcvtms_v         , SimdFcvtSV         , (0b0000111000100001101110, 0b0000000000000000000000, 0b0001111000110000000000, 1)     , RWI_W    , 0                         , 2  ), // #489
INST(Fcvtmu_v         , SimdFcvtSV         , (0b0010111000100001101110, 0b0000000000000000000000, 0b0001111000110001000000, 1)     , RWI_W    , 0                         , 3  ), // #490
INST(Fcvtn_v          , SimdFcvtLN         , (0b0000111000100001011010, 0, 0)                                                      , RWI_W    , F!(Narrow)                 , 2  ), // #491
INST(Fcvtn2_v         , SimdFcvtLN         , (0b0100111000100001011010, 0, 0)                                                      , RWI_X    , F!(Narrow)                 , 3  ), // #492
INST(Fcvtns_v         , SimdFcvtSV         , (0b0000111000100001101010, 0b0000000000000000000000, 0b0001111000100000000000, 1)     , RWI_W    , 0                         , 4  ), // #493
INST(Fcvtnu_v         , SimdFcvtSV         , (0b0010111000100001101010, 0b0000000000000000000000, 0b0001111000100001000000, 1)     , RWI_W    , 0                         , 5  ), // #494
INST(Fcvtps_v         , SimdFcvtSV         , (0b0000111010100001101010, 0b0000000000000000000000, 0b0001111000101000000000, 1)     , RWI_W    , 0                         , 6  ), // #495
INST(Fcvtpu_v         , SimdFcvtSV         , (0b0010111010100001101010, 0b0000000000000000000000, 0b0001111000101001000000, 1)     , RWI_W    , 0                         , 7  ), // #496
INST(Fcvtxn_v         , SimdFcvtLN         , (0b0010111000100001011010, 1, 1)                                                      , RWI_W    , F!(Narrow)                 , 4  ), // #497
INST(Fcvtxn2_v        , SimdFcvtLN         , (0b0110111000100001011010, 1, 0)                                                      , RWI_X    , F!(Narrow)                 , 5  ), // #498
INST(Fcvtzs_v         , SimdFcvtSV         , (0b0000111010100001101110, 0b0000111100000000111111, 0b0001111000111000000000, 1)     , RWI_W    , 0                         , 8  ), // #499
INST(Fcvtzu_v         , SimdFcvtSV         , (0b0010111010100001101110, 0b0010111100000000111111, 0b0001111000111001000000, 1)     , RWI_W    , 0                         , 9  ), // #500
INST(Fdiv_v           , FSimdVVV           , (0b0001111000100000000110, kHF_A, 0b0010111000100000111111, kHF_C)                    , RWI_W    , 0                         , 4  ), // #501
INST(Fjcvtzs_v        , ISimdVVx           , (0b0001111001111110000000, kOp_GpW, kOp_D)                                            , RWI_W    , 0                         , 7  ), // #502
INST(Fmadd_v          , FSimdVVVV          , (0b0001111100000000000000, kHF_A, 0b0000000000000000000000, kHF_N)                    , RWI_W    , 0                         , 0  ), // #503
INST(Fmax_v           , FSimdVVV           , (0b0001111000100000010010, kHF_A, 0b0000111000100000111101, kHF_C)                    , RWI_W    , 0                         , 5  ), // #504
INST(Fmaxnm_v         , FSimdVVV           , (0b0001111000100000011010, kHF_A, 0b0000111000100000110001, kHF_C)                    , RWI_W    , 0                         , 6  ), // #505
INST(Fmaxnmp_v        , FSimdPair          , (0b0111111000110000110010, 0b0010111000100000110001)                                  , RWI_W    , 0                         , 1  ), // #506
INST(Fmaxnmv_v        , FSimdSV            , (0b0010111000110000110010)                                                            , RWI_W    , 0                         , 0  ), // #507
INST(Fmaxp_v          , FSimdPair          , (0b0111111000110000111110, 0b0010111000100000111101)                                  , RWI_W    , 0                         , 2  ), // #508
INST(Fmaxv_v          , FSimdSV            , (0b0010111000110000111110)                                                            , RWI_W    , 0                         , 1  ), // #509
INST(Fmin_v           , FSimdVVV           , (0b0001111000100000010110, kHF_A, 0b0000111010100000111101, kHF_C)                    , RWI_W    , 0                         , 7  ), // #510
INST(Fminnm_v         , FSimdVVV           , (0b0001111000100000011110, kHF_A, 0b0000111010100000110001, kHF_C)                    , RWI_W    , 0                         , 8  ), // #511
INST(Fminnmp_v        , FSimdPair          , (0b0111111010110000110010, 0b0010111010100000110001)                                  , RWI_W    , 0                         , 3  ), // #512
INST(Fminnmv_v        , FSimdSV            , (0b0010111010110000110010)                                                            , RWI_W    , 0                         , 2  ), // #513
INST(Fminp_v          , FSimdPair          , (0b0111111010110000111110, 0b0010111010100000111101)                                  , RWI_W    , 0                         , 4  ), // #514
INST(Fminv_v          , FSimdSV            , (0b0010111010110000111110)                                                            , RWI_W    , 0                         , 3  ), // #515
INST(Fmla_v           , FSimdVVVe          , (0b0000000000000000000000, kHF_N, 0b0000111000100000110011, 0b0000111110000000000100) , RWI_X    , F!(VH0_15)                 , 0  ), // #516
INST(Fmlal_v          , SimdFmlal          , (0b0000111000100000111011, 0b0000111110000000000000, 1, kET_S, kET_H, kET_H)          , RWI_X    , F!(VH0_15)                 , 2  ), // #517
INST(Fmlal2_v         , SimdFmlal          , (0b0010111000100000110011, 0b0010111110000000100000, 1, kET_S, kET_H, kET_H)          , RWI_X    , F!(VH0_15)                 , 3  ), // #518
INST(Fmls_v           , FSimdVVVe          , (0b0000000000000000000000, kHF_N, 0b0000111010100000110011, 0b0000111110000000010100) , RWI_X    , F!(VH0_15)                 , 1  ), // #519
INST(Fmlsl_v          , SimdFmlal          , (0b0000111010100000111011, 0b0000111110000000010000, 1, kET_S, kET_H, kET_H)          , RWI_X    , F!(VH0_15)                 , 4  ), // #520
INST(Fmlsl2_v         , SimdFmlal          , (0b0010111010100000110011, 0b0010111110000000110000, 1, kET_S, kET_H, kET_H)          , RWI_X    , F!(VH0_15)                 , 5  ), // #521
INST(Fmov_v           , SimdFmov           , (_)                                                                                   , RWI_W    , 0                         , 0  ), // #522
INST(Fmsub_v          , FSimdVVVV          , (0b0001111100000000100000, kHF_A, 0b0000000000000000000000, kHF_N)                    , RWI_W    , 0                         , 1  ), // #523
INST(Fmul_v           , FSimdVVVe          , (0b0001111000100000000010, kHF_A, 0b0010111000100000110111, 0b0000111110000000100100) , RWI_W    , F!(VH0_15)                 , 2  ), // #524
INST(Fmulx_v          , FSimdVVVe          , (0b0101111000100000110111, kHF_C, 0b0000111000100000110111, 0b0010111110000000100100) , RWI_W    , F!(VH0_15)                 , 3  ), // #525
INST(Fneg_v           , FSimdVV            , (0b0001111000100001010000, kHF_A, 0b0010111010100000111110, kHF_B)                    , RWI_W    , 0                         , 1  ), // #526
INST(Fnmadd_v         , FSimdVVVV          , (0b0001111100100000000000, kHF_A, 0b0000000000000000000000, kHF_N)                    , RWI_W    , 0                         , 2  ), // #527
INST(Fnmsub_v         , FSimdVVVV          , (0b0001111100100000100000, kHF_A, 0b0000000000000000000000, kHF_N)                    , RWI_W    , 0                         , 3  ), // #528
INST(Fnmul_v          , FSimdVVV           , (0b0001111000100000100010, kHF_A, 0b0000000000000000000000, kHF_N)                    , RWI_W    , 0                         , 9  ), // #529
INST(Frecpe_v         , FSimdVV            , (0b0101111010100001110110, kHF_B, 0b0000111010100001110110, kHF_B)                    , RWI_W    , 0                         , 2  ), // #530
INST(Frecps_v         , FSimdVVV           , (0b0101111000100000111111, kHF_C, 0b0000111000100000111111, kHF_C)                    , RWI_W    , 0                         , 10 ), // #531
INST(Frecpx_v         , FSimdVV            , (0b0101111010100001111110, kHF_B, 0b0000000000000000000000, kHF_N)                    , RWI_W    , 0                         , 3  ), // #532
INST(Frint32x_v       , FSimdVV            , (0b0001111000101000110000, kHF_N, 0b0010111000100001111010, kHF_N)                    , RWI_W    , 0                         , 4  ), // #533
INST(Frint32z_v       , FSimdVV            , (0b0001111000101000010000, kHF_N, 0b0000111000100001111010, kHF_N)                    , RWI_W    , 0                         , 5  ), // #534
INST(Frint64x_v       , FSimdVV            , (0b0001111000101001110000, kHF_N, 0b0010111000100001111110, kHF_N)                    , RWI_W    , 0                         , 6  ), // #535
INST(Frint64z_v       , FSimdVV            , (0b0001111000101001010000, kHF_N, 0b0000111000100001111110, kHF_N)                    , RWI_W    , 0                         , 7  ), // #536
INST(Frinta_v         , FSimdVV            , (0b0001111000100110010000, kHF_A, 0b0010111000100001100010, kHF_B)                    , RWI_W    , 0                         , 8  ), // #537
INST(Frinti_v         , FSimdVV            , (0b0001111000100111110000, kHF_A, 0b0010111010100001100110, kHF_B)                    , RWI_W    , 0                         , 9  ), // #538
INST(Frintm_v         , FSimdVV            , (0b0001111000100101010000, kHF_A, 0b0000111000100001100110, kHF_B)                    , RWI_W    , 0                         , 10 ), // #539
INST(Frintn_v         , FSimdVV            , (0b0001111000100100010000, kHF_A, 0b0000111000100001100010, kHF_B)                    , RWI_W    , 0                         , 11 ), // #540
INST(Frintp_v         , FSimdVV            , (0b0001111000100100110000, kHF_A, 0b0000111010100001100010, kHF_B)                    , RWI_W    , 0                         , 12 ), // #541
INST(Frintx_v         , FSimdVV            , (0b0001111000100111010000, kHF_A, 0b0010111000100001100110, kHF_B)                    , RWI_W    , 0                         , 13 ), // #542
INST(Frintz_v         , FSimdVV            , (0b0001111000100101110000, kHF_A, 0b0000111010100001100110, kHF_B)                    , RWI_W    , 0                         , 14 ), // #543
INST(Frsqrte_v        , FSimdVV            , (0b0111111010100001110110, kHF_B, 0b0010111010100001110110, kHF_B)                    , RWI_W    , 0                         , 15 ), // #544
INST(Frsqrts_v        , FSimdVVV           , (0b0101111010100000111111, kHF_C, 0b0000111010100000111111, kHF_C)                    , RWI_W    , 0                         , 11 ), // #545
INST(Fsqrt_v          , FSimdVV            , (0b0001111000100001110000, kHF_A, 0b0010111010100001111110, kHF_B)                    , RWI_W    , 0                         , 16 ), // #546
INST(Fsub_v           , FSimdVVV           , (0b0001111000100000001110, kHF_A, 0b0000111010100000110101, kHF_C)                    , RWI_W    , 0                         , 12 ), // #547
INST(Ins_v            , SimdIns            , (_)                                                                                   , RWI_X    , 0                         , 0  ), // #548
INST(Ld1_v            , SimdLdNStN         , (0b0000110101000000000000, 0b0000110001000000001000, 1, 0)                            , RWI_LDN  , F!(Consecutive)            , 0  ), // #549
INST(Ld1r_v           , SimdLdNStN         , (0b0000110101000000110000, 0b0000000000000000000000, 1, 1)                            , RWI_LDN  , F!(Consecutive)            , 1  ), // #550
INST(Ld2_v            , SimdLdNStN         , (0b0000110101100000000000, 0b0000110001000000100000, 2, 0)                            , RWI_LDN  , F!(Consecutive)            , 2  ), // #551
INST(Ld2r_v           , SimdLdNStN         , (0b0000110101100000110000, 0b0000000000000000000000, 2, 1)                            , RWI_LDN  , F!(Consecutive)            , 3  ), // #552
INST(Ld3_v            , SimdLdNStN         , (0b0000110101000000001000, 0b0000110001000000010000, 3, 0)                            , RWI_LDN  , F!(Consecutive)            , 4  ), // #553
INST(Ld3r_v           , SimdLdNStN         , (0b0000110101000000111000, 0b0000000000000000000000, 3, 1)                            , RWI_LDN  , F!(Consecutive)            , 5  ), // #554
INST(Ld4_v            , SimdLdNStN         , (0b0000110101100000001000, 0b0000110001000000000000, 4, 0)                            , RWI_LDN  , F!(Consecutive)            , 6  ), // #555
INST(Ld4r_v           , SimdLdNStN         , (0b0000110101100000111000, 0b0000000000000000000000, 4, 1)                            , RWI_LDN  , F!(Consecutive)            , 7  ), // #556
INST(Ldnp_v           , SimdLdpStp         , (0b0010110001, 0b0000000000)                                                          , RWI_WW   , 0                         , 0  ), // #557
INST(Ldp_v            , SimdLdpStp         , (0b0010110101, 0b0010110011)                                                          , RWI_WW   , 0                         , 1  ), // #558
INST(Ldr_v            , SimdLdSt           , (0b0011110101, 0b00111100010, 0b00111100011, 0b00011100, InstId::Ldur_v)             , RWI_W    , 0                         , 0  ), // #559
INST(Ldur_v           , SimdLdurStur       , (0b0011110001000000000000)                                                            , RWI_W    , 0                         , 0  ), // #560
INST(Mla_v            , ISimdVVVe          , (0b0000111000100000100101, kVO_V_BHS, 0b0010111100000000000000, kVO_V_HS)             , RWI_X    , F!(VH0_15)                 , 0  ), // #561
INST(Mls_v            , ISimdVVVe          , (0b0010111000100000100101, kVO_V_BHS, 0b0010111100000000010000, kVO_V_HS)             , RWI_X    , F!(VH0_15)                 , 1  ), // #562
INST(Mov_v            , SimdMov            , (_)                                                                                   , RWI_W    , 0                         , 0  ), // #563
INST(Movi_v           , SimdMoviMvni       , (0b0000111100000000000001, 0)                                                         , RWI_W    , 0                         , 0  ), // #564
INST(Mul_v            , ISimdVVVe          , (0b0000111000100000100111, kVO_V_BHS, 0b0000111100000000100000, kVO_V_HS)             , RWI_W    , F!(VH0_15)                 , 2  ), // #565
INST(Mvn_v            , ISimdVV            , (0b0010111000100000010110, kVO_V_B)                                                   , RWI_W    , 0                         , 4  ), // #566
INST(Mvni_v           , SimdMoviMvni       , (0b0000111100000000000001, 1)                                                         , RWI_W    , 0                         , 1  ), // #567
INST(Neg_v            , ISimdVV            , (0b0010111000100000101110, kVO_V_Any)                                                 , RWI_W    , 0                         , 5  ), // #568
INST(Not_v            , ISimdVV            , (0b0010111000100000010110, kVO_V_B)                                                   , RWI_W    , 0                         , 6  ), // #569
INST(Orn_v            , ISimdVVV           , (0b0000111011100000000111, kVO_V_B)                                                   , RWI_W    , 0                         , 9  ), // #570
INST(Orr_v            , SimdBicOrr         , (0b0000111010100000000111, 0b0000111100000000000001)                                  , RWI_W    , 0                         , 1  ), // #571
INST(Pmul_v           , ISimdVVV           , (0b0010111000100000100111, kVO_V_B)                                                   , RWI_W    , 0                         , 10 ), // #572
INST(Pmull_v          , ISimdVVV           , (0b0000111000100000111000, kVO_V_B8D1)                                                , RWI_W    , F!(Long)                   , 11 ), // #573
INST(Pmull2_v         , ISimdVVV           , (0b0100111000100000111000, kVO_V_B16D2)                                               , RWI_W    , F!(Long)                   , 12 ), // #574
INST(Raddhn_v         , ISimdVVV           , (0b0010111000100000010000, kVO_V_B8H4S2)                                              , RWI_W    , F!(Narrow)                 , 13 ), // #575
INST(Raddhn2_v        , ISimdVVV           , (0b0110111000100000010000, kVO_V_B16H8S4)                                             , RWI_X    , F!(Narrow)                 , 14 ), // #576
INST(Rax1_v           , ISimdVVV           , (0b1100111001100000100011, kVO_V_D2)                                                  , RWI_W    , 0                         , 15 ), // #577
INST(Rbit_v           , ISimdVV            , (0b0010111001100000010110, kVO_V_B)                                                   , RWI_W    , 0                         , 7  ), // #578
INST(Rev16_v          , ISimdVV            , (0b0000111000100000000110, kVO_V_B)                                                   , RWI_W    , 0                         , 8  ), // #579
INST(Rev32_v          , ISimdVV            , (0b0010111000100000000010, kVO_V_BH)                                                  , RWI_W    , 0                         , 9  ), // #580
INST(Rev64_v          , ISimdVV            , (0b0000111000100000000010, kVO_V_BHS)                                                 , RWI_W    , 0                         , 10 ), // #581
INST(Rshrn_v          , SimdShift          , (0b0000000000000000000000, 0b0000111100000000100011, 1, kVO_V_B8H4S2)                 , RWI_W    , F!(Narrow)                 , 0  ), // #582
INST(Rshrn2_v         , SimdShift          , (0b0000000000000000000000, 0b0100111100000000100011, 1, kVO_V_B16H8S4)                , RWI_X    , F!(Narrow)                 , 1  ), // #583
INST(Rsubhn_v         , ISimdVVV           , (0b0010111000100000011000, kVO_V_B8H4S2)                                              , RWI_W    , F!(Narrow)                 , 16 ), // #584
INST(Rsubhn2_v        , ISimdVVV           , (0b0110111000100000011000, kVO_V_B16H8S4)                                             , RWI_X    , F!(Narrow)                 , 17 ), // #585
INST(Saba_v           , ISimdVVV           , (0b0000111000100000011111, kVO_V_BHS)                                                 , RWI_X    , 0                         , 18 ), // #586
INST(Sabal_v          , ISimdVVV           , (0b0000111000100000010100, kVO_V_B8H4S2)                                              , RWI_X    , F!(Long)                   , 19 ), // #587
INST(Sabal2_v         , ISimdVVV           , (0b0100111000100000010100, kVO_V_B16H8S4)                                             , RWI_X    , F!(Long)                   , 20 ), // #588
INST(Sabd_v           , ISimdVVV           , (0b0000111000100000011101, kVO_V_BHS)                                                 , RWI_W    , 0                         , 21 ), // #589
INST(Sabdl_v          , ISimdVVV           , (0b0000111000100000011100, kVO_V_B8H4S2)                                              , RWI_W    , F!(Long)                   , 22 ), // #590
INST(Sabdl2_v         , ISimdVVV           , (0b0100111000100000011100, kVO_V_B16H8S4)                                             , RWI_W    , F!(Long)                   , 23 ), // #591
INST(Sadalp_v         , ISimdVV            , (0b0000111000100000011010, kVO_V_BHS)                                                 , RWI_X    , F!(Long) | F!(Pair)         , 11 ), // #592
INST(Saddl_v          , ISimdVVV           , (0b0000111000100000000000, kVO_V_B8H4S2)                                              , RWI_W    , F!(Long)                   , 24 ), // #593
INST(Saddl2_v         , ISimdVVV           , (0b0100111000100000000000, kVO_V_B16H8S4)                                             , RWI_W    , F!(Long)                   , 25 ), // #594
INST(Saddlp_v         , ISimdVV            , (0b0000111000100000001010, kVO_V_BHS)                                                 , RWI_W    , F!(Long) | F!(Pair)         , 12 ), // #595
INST(Saddlv_v         , ISimdSV            , (0b0000111000110000001110, kVO_V_BH_4S)                                               , RWI_W    , F!(Long)                   , 1  ), // #596
INST(Saddw_v          , ISimdWWV           , (0b0000111000100000000100, kVO_V_B8H4S2)                                              , RWI_W    , 0                         , 0  ), // #597
INST(Saddw2_v         , ISimdWWV           , (0b0000111000100000000100, kVO_V_B16H8S4)                                             , RWI_W    , 0                         , 1  ), // #598
INST(Scvtf_v          , SimdFcvtSV         , (0b0000111000100001110110, 0b0000111100000000111001, 0b0001111000100010000000, 0)     , RWI_W    , 0                         , 10 ), // #599
INST(Sdot_v           , SimdDot            , (0b0000111010000000100101, 0b0000111110000000111000, kET_S, kET_B, kET_4B)            , RWI_X    , 0                         , 1  ), // #600
INST(Sha1c_v          , ISimdVVVx          , (0b0101111000000000000000, kOp_Q, kOp_S, kOp_V4S)                                     , RWI_X    , 0                         , 1  ), // #601
INST(Sha1h_v          , ISimdVVx           , (0b0101111000101000000010, kOp_S, kOp_S)                                              , RWI_W    , 0                         , 8  ), // #602
INST(Sha1m_v          , ISimdVVVx          , (0b0101111000000000001000, kOp_Q, kOp_S, kOp_V4S)                                     , RWI_X    , 0                         , 2  ), // #603
INST(Sha1p_v          , ISimdVVVx          , (0b0101111000000000000100, kOp_Q, kOp_S, kOp_V4S)                                     , RWI_X    , 0                         , 3  ), // #604
INST(Sha1su0_v        , ISimdVVVx          , (0b0101111000000000001100, kOp_V4S, kOp_V4S, kOp_V4S)                                 , RWI_X    , 0                         , 4  ), // #605
INST(Sha1su1_v        , ISimdVVx           , (0b0101111000101000000110, kOp_V4S, kOp_V4S)                                          , RWI_X    , 0                         , 9  ), // #606
INST(Sha256h_v        , ISimdVVVx          , (0b0101111000000000010000, kOp_Q, kOp_Q, kOp_V4S)                                     , RWI_X    , 0                         , 5  ), // #607
INST(Sha256h2_v       , ISimdVVVx          , (0b0101111000000000010100, kOp_Q, kOp_Q, kOp_V4S)                                     , RWI_X    , 0                         , 6  ), // #608
INST(Sha256su0_v      , ISimdVVx           , (0b0101111000101000001010, kOp_V4S, kOp_V4S)                                          , RWI_X    , 0                         , 10 ), // #609
INST(Sha256su1_v      , ISimdVVVx          , (0b0101111000000000011000, kOp_V4S, kOp_V4S, kOp_V4S)                                 , RWI_X    , 0                         , 7  ), // #610
INST(Sha512h_v        , ISimdVVVx          , (0b1100111001100000100000, kOp_Q, kOp_Q, kOp_V2D)                                     , RWI_X    , 0                         , 8  ), // #611
INST(Sha512h2_v       , ISimdVVVx          , (0b1100111001100000100001, kOp_Q, kOp_Q, kOp_V2D)                                     , RWI_X    , 0                         , 9  ), // #612
INST(Sha512su0_v      , ISimdVVx           , (0b1100111011000000100000, kOp_V2D, kOp_V2D)                                          , RWI_X    , 0                         , 11 ), // #613
INST(Sha512su1_v      , ISimdVVVx          , (0b1100111001100000100010, kOp_V2D, kOp_V2D, kOp_V2D)                                 , RWI_X    , 0                         , 10 ), // #614
INST(Shadd_v          , ISimdVVV           , (0b0000111000100000000001, kVO_V_BHS)                                                 , RWI_W    , 0                         , 26 ), // #615
INST(Shl_v            , SimdShift          , (0b0000000000000000000000, 0b0000111100000000010101, 0, kVO_V_Any)                    , RWI_W    , 0                         , 2  ), // #616
INST(Shll_v           , SimdShiftES        , (0b0010111000100001001110, kVO_V_B8H4S2)                                              , RWI_W    , F!(Long)                   , 0  ), // #617
INST(Shll2_v          , SimdShiftES        , (0b0110111000100001001110, kVO_V_B16H8S4)                                             , RWI_W    , F!(Long)                   , 1  ), // #618
INST(Shrn_v           , SimdShift          , (0b0000000000000000000000, 0b0000111100000000100001, 1, kVO_V_B8H4S2)                 , RWI_W    , F!(Narrow)                 , 3  ), // #619
INST(Shrn2_v          , SimdShift          , (0b0000000000000000000000, 0b0100111100000000100001, 1, kVO_V_B16H8S4)                , RWI_X    , F!(Narrow)                 , 4  ), // #620
INST(Shsub_v          , ISimdVVV           , (0b0000111000100000001001, kVO_V_BHS)                                                 , RWI_W    , 0                         , 27 ), // #621
INST(Sli_v            , SimdShift          , (0b0000000000000000000000, 0b0010111100000000010101, 0, kVO_V_Any)                    , RWI_X    , 0                         , 5  ), // #622
INST(Sm3partw1_v      , ISimdVVVx          , (0b1100111001100000110000, kOp_V4S, kOp_V4S, kOp_V4S)                                 , RWI_X    , 0                         , 11 ), // #623
INST(Sm3partw2_v      , ISimdVVVx          , (0b1100111001100000110001, kOp_V4S, kOp_V4S, kOp_V4S)                                 , RWI_X    , 0                         , 12 ), // #624
INST(Sm3ss1_v         , ISimdVVVVx         , (0b1100111001000000000000, kOp_V4S, kOp_V4S, kOp_V4S, kOp_V4S)                        , RWI_W    , 0                         , 0  ), // #625
INST(Sm3tt1a_v        , SimdSm3tt          , (0b1100111001000000100000)                                                            , RWI_X    , 0                         , 0  ), // #626
INST(Sm3tt1b_v        , SimdSm3tt          , (0b1100111001000000100001)                                                            , RWI_X    , 0                         , 1  ), // #627
INST(Sm3tt2a_v        , SimdSm3tt          , (0b1100111001000000100010)                                                            , RWI_X    , 0                         , 2  ), // #628
INST(Sm3tt2b_v        , SimdSm3tt          , (0b1100111001000000100011)                                                            , RWI_X    , 0                         , 3  ), // #629
INST(Sm4e_v           , ISimdVVx           , (0b1100111011000000100001, kOp_V4S, kOp_V4S)                                          , RWI_X    , 0                         , 12 ), // #630
INST(Sm4ekey_v        , ISimdVVVx          , (0b1100111001100000110010, kOp_V4S, kOp_V4S, kOp_V4S)                                 , RWI_X    , 0                         , 13 ), // #631
INST(Smax_v           , ISimdVVV           , (0b0000111000100000011001, kVO_V_BHS)                                                 , RWI_W    , 0                         , 28 ), // #632
INST(Smaxp_v          , ISimdVVV           , (0b0000111000100000101001, kVO_V_BHS)                                                 , RWI_W    , 0                         , 29 ), // #633
INST(Smaxv_v          , ISimdSV            , (0b0000111000110000101010, kVO_V_BH_4S)                                               , RWI_W    , 0                         , 2  ), // #634
INST(Smin_v           , ISimdVVV           , (0b0000111000100000011011, kVO_V_BHS)                                                 , RWI_W    , 0                         , 30 ), // #635
INST(Sminp_v          , ISimdVVV           , (0b0000111000100000101011, kVO_V_BHS)                                                 , RWI_W    , 0                         , 31 ), // #636
INST(Sminv_v          , ISimdSV            , (0b0000111000110001101010, kVO_V_BH_4S)                                               , RWI_W    , 0                         , 3  ), // #637
INST(Smlal_v          , ISimdVVVe          , (0b0000111000100000100000, kVO_V_B8H4S2, 0b0000111100000000001000, kVO_V_H4S2)        , RWI_X    , F!(Long) | F!(VH0_15)       , 3  ), // #638
INST(Smlal2_v         , ISimdVVVe          , (0b0100111000100000100000, kVO_V_B16H8S4, 0b0100111100000000001000, kVO_V_H8S4)       , RWI_X    , F!(Long) | F!(VH0_15)       , 4  ), // #639
INST(Smlsl_v          , ISimdVVVe          , (0b0000111000100000101000, kVO_V_B8H4S2, 0b0000111100000000011000, kVO_V_H4S2)        , RWI_X    , F!(Long) | F!(VH0_15)       , 5  ), // #640
INST(Smlsl2_v         , ISimdVVVe          , (0b0100111000100000101000, kVO_V_B16H8S4, 0b0100111100000000011000, kVO_V_H8S4)       , RWI_X    , F!(Long) | F!(VH0_15)       , 6  ), // #641
INST(Smmla_v          , ISimdVVVx          , (0b0100111010000000101001, kOp_V4S, kOp_V16B, kOp_V16B)                               , RWI_X    , 0                         , 14 ), // #642
INST(Smov_v           , SimdSmovUmov       , (0b0000111000000000001011, kVO_V_BHS, 1)                                              , RWI_W    , 0                         , 0  ), // #643
INST(Smull_v          , ISimdVVVe          , (0b0000111000100000110000, kVO_V_B8H4S2, 0b0000111100000000101000, kVO_V_H4S2)        , RWI_W    , F!(Long) | F!(VH0_15)       , 7  ), // #644
INST(Smull2_v         , ISimdVVVe          , (0b0100111000100000110000, kVO_V_B16H8S4, 0b0100111100000000101000, kVO_V_H8S4)       , RWI_W    , F!(Long) | F!(VH0_15)       , 8  ), // #645
INST(Sqabs_v          , ISimdVV            , (0b0000111000100000011110, kVO_SV_Any)                                                , RWI_W    , 0                         , 13 ), // #646
INST(Sqadd_v          , ISimdVVV           , (0b0000111000100000000011, kVO_SV_Any)                                                , RWI_W    , 0                         , 32 ), // #647
INST(Sqdmlal_v        , ISimdVVVe          , (0b0000111000100000100100, kVO_SV_BHS, 0b0000111100000000001100, kVO_V_H4S2)          , RWI_X    , F!(Long) | F!(VH0_15)       , 9  ), // #648
INST(Sqdmlal2_v       , ISimdVVVe          , (0b0100111000100000100100, kVO_V_B16H8S4, 0b0100111100000000001100, kVO_V_H8S4)       , RWI_X    , F!(Long) | F!(VH0_15)       , 10 ), // #649
INST(Sqdmlsl_v        , ISimdVVVe          , (0b0000111000100000101100, kVO_SV_BHS, 0b0000111100000000011100, kVO_V_H4S2)          , RWI_X    , F!(Long) | F!(VH0_15)       , 11 ), // #650
INST(Sqdmlsl2_v       , ISimdVVVe          , (0b0100111000100000101100, kVO_V_B16H8S4, 0b0100111100000000011100, kVO_V_H8S4)       , RWI_X    , F!(Long) | F!(VH0_15)       , 12 ), // #651
INST(Sqdmulh_v        , ISimdVVVe          , (0b0000111000100000101101, kVO_SV_HS, 0b0000111100000000110000, kVO_SV_HS)            , RWI_W    , F!(VH0_15)                 , 13 ), // #652
INST(Sqdmull_v        , ISimdVVVe          , (0b0000111000100000110100, kVO_SV_BHS, 0b0000111100000000101100, kVO_V_H4S2)          , RWI_W    , F!(Long) | F!(VH0_15)       , 14 ), // #653
INST(Sqdmull2_v       , ISimdVVVe          , (0b0100111000100000110100, kVO_V_B16H8S4, 0b0100111100000000101100, kVO_V_H8S4)       , RWI_W    , F!(Long) | F!(VH0_15)       , 15 ), // #654
INST(Sqneg_v          , ISimdVV            , (0b0010111000100000011110, kVO_SV_Any)                                                , RWI_W    , 0                         , 14 ), // #655
INST(Sqrdmlah_v       , ISimdVVVe          , (0b0010111000000000100001, kVO_SV_HS, 0b0010111100000000110100, kVO_SV_HS)            , RWI_X    , F!(VH0_15)                 , 16 ), // #656
INST(Sqrdmlsh_v       , ISimdVVVe          , (0b0010111000000000100011, kVO_SV_HS, 0b0010111100000000111100, kVO_SV_HS)            , RWI_X    , F!(VH0_15)                 , 17 ), // #657
INST(Sqrdmulh_v       , ISimdVVVe          , (0b0010111000100000101101, kVO_SV_HS, 0b0000111100000000110100, kVO_SV_HS)            , RWI_W    , F!(VH0_15)                 , 18 ), // #658
INST(Sqrshl_v         , SimdShift          , (0b0000111000100000010111, 0b0000000000000000000000, 1, kVO_SV_Any)                   , RWI_W    , 0                         , 6  ), // #659
INST(Sqrshrn_v        , SimdShift          , (0b0000000000000000000000, 0b0000111100000000100111, 1, kVO_SV_B8H4S2)                , RWI_W    , F!(Narrow)                 , 7  ), // #660
INST(Sqrshrn2_v       , SimdShift          , (0b0000000000000000000000, 0b0100111100000000100111, 1, kVO_V_B16H8S4)                , RWI_X    , F!(Narrow)                 , 8  ), // #661
INST(Sqrshrun_v       , SimdShift          , (0b0000000000000000000000, 0b0010111100000000100011, 1, kVO_SV_B8H4S2)                , RWI_W    , F!(Narrow)                 , 9  ), // #662
INST(Sqrshrun2_v      , SimdShift          , (0b0000000000000000000000, 0b0110111100000000100011, 1, kVO_V_B16H8S4)                , RWI_X    , F!(Narrow)                 , 10 ), // #663
INST(Sqshl_v          , SimdShift          , (0b0000111000100000010011, 0b0000111100000000011101, 0, kVO_SV_Any)                   , RWI_W    , 0                         , 11 ), // #664
INST(Sqshlu_v         , SimdShift          , (0b0000000000000000000000, 0b0010111100000000011001, 0, kVO_SV_Any)                   , RWI_W    , 0                         , 12 ), // #665
INST(Sqshrn_v         , SimdShift          , (0b0000000000000000000000, 0b0000111100000000100101, 1, kVO_SV_B8H4S2)                , RWI_W    , F!(Narrow)                 , 13 ), // #666
INST(Sqshrn2_v        , SimdShift          , (0b0000000000000000000000, 0b0100111100000000100101, 1, kVO_V_B16H8S4)                , RWI_X    , F!(Narrow)                 , 14 ), // #667
INST(Sqshrun_v        , SimdShift          , (0b0000000000000000000000, 0b0010111100000000100001, 1, kVO_SV_B8H4S2)                , RWI_W    , F!(Narrow)                 , 15 ), // #668
INST(Sqshrun2_v       , SimdShift          , (0b0000000000000000000000, 0b0110111100000000100001, 1, kVO_V_B16H8S4)                , RWI_X    , F!(Narrow)                 , 16 ), // #669
INST(Sqsub_v          , ISimdVVV           , (0b0000111000100000001011, kVO_SV_Any)                                                , RWI_W    , 0                         , 33 ), // #670
INST(Sqxtn_v          , ISimdVV            , (0b0000111000100001010010, kVO_SV_B8H4S2)                                             , RWI_W    , F!(Narrow)                 , 15 ), // #671
INST(Sqxtn2_v         , ISimdVV            , (0b0100111000100001010010, kVO_V_B16H8S4)                                             , RWI_X    , F!(Narrow)                 , 16 ), // #672
INST(Sqxtun_v         , ISimdVV            , (0b0010111000100001001010, kVO_SV_B8H4S2)                                             , RWI_W    , F!(Narrow)                 , 17 ), // #673
INST(Sqxtun2_v        , ISimdVV            , (0b0110111000100001001010, kVO_V_B16H8S4)                                             , RWI_X    , F!(Narrow)                 , 18 ), // #674
INST(Srhadd_v         , ISimdVVV           , (0b0000111000100000000101, kVO_V_BHS)                                                 , RWI_W    , 0                         , 34 ), // #675
INST(Sri_v            , SimdShift          , (0b0000000000000000000000, 0b0010111100000000010001, 1, kVO_V_Any)                    , RWI_W    , 0                         , 17 ), // #676
INST(Srshl_v          , SimdShift          , (0b0000111000100000010101, 0b0000000000000000000000, 0, kVO_V_Any)                    , RWI_W    , 0                         , 18 ), // #677
INST(Srshr_v          , SimdShift          , (0b0000000000000000000000, 0b0000111100000000001001, 1, kVO_V_Any)                    , RWI_W    , 0                         , 19 ), // #678
INST(Srsra_v          , SimdShift          , (0b0000000000000000000000, 0b0000111100000000001101, 1, kVO_V_Any)                    , RWI_X    , 0                         , 20 ), // #679
INST(Sshl_v           , SimdShift          , (0b0000111000100000010001, 0b0000000000000000000000, 0, kVO_V_Any)                    , RWI_W    , 0                         , 21 ), // #680
INST(Sshll_v          , SimdShift          , (0b0000000000000000000000, 0b0000111100000000101001, 0, kVO_V_B8H4S2)                 , RWI_W    , F!(Long)                   , 22 ), // #681
INST(Sshll2_v         , SimdShift          , (0b0000000000000000000000, 0b0100111100000000101001, 0, kVO_V_B16H8S4)                , RWI_W    , F!(Long)                   , 23 ), // #682
INST(Sshr_v           , SimdShift          , (0b0000000000000000000000, 0b0000111100000000000001, 1, kVO_V_Any)                    , RWI_W    , 0                         , 24 ), // #683
INST(Ssra_v           , SimdShift          , (0b0000000000000000000000, 0b0000111100000000000101, 1, kVO_V_Any)                    , RWI_X    , 0                         , 25 ), // #684
INST(Ssubl_v          , ISimdVVV           , (0b0000111000100000001000, kVO_V_B8H4S2)                                              , RWI_W    , F!(Long)                   , 35 ), // #685
INST(Ssubl2_v         , ISimdVVV           , (0b0100111000100000001000, kVO_V_B16H8S4)                                             , RWI_W    , F!(Long)                   , 36 ), // #686
INST(Ssubw_v          , ISimdWWV           , (0b0000111000100000001100, kVO_V_B8H4S2)                                              , RWI_W    , 0                         , 2  ), // #687
INST(Ssubw2_v         , ISimdWWV           , (0b0000111000100000001100, kVO_V_B16H8S4)                                             , RWI_X    , 0                         , 3  ), // #688
INST(St1_v            , SimdLdNStN         , (0b0000110100000000000000, 0b0000110000000000001000, 1, 0)                            , RWI_STN  , F!(Consecutive)            , 8  ), // #689
INST(St2_v            , SimdLdNStN         , (0b0000110100100000000000, 0b0000110000000000100000, 2, 0)                            , RWI_STN  , F!(Consecutive)            , 9  ), // #690
INST(St3_v            , SimdLdNStN         , (0b0000110100000000001000, 0b0000110000000000010000, 3, 0)                            , RWI_STN  , F!(Consecutive)            , 10 ), // #691
INST(St4_v            , SimdLdNStN         , (0b0000110100100000001000, 0b0000110000000000000000, 4, 0)                            , RWI_STN  , F!(Consecutive)            , 11 ), // #692
INST(Stnp_v           , SimdLdpStp         , (0b0010110000, 0b0000000000)                                                          , RWI_RRW  , 0                         , 2  ), // #693
INST(Stp_v            , SimdLdpStp         , (0b0010110100, 0b0010110010)                                                          , RWI_RRW  , 0                         , 3  ), // #694
INST(Str_v            , SimdLdSt           , (0b0011110100, 0b00111100000, 0b00111100001, 0b00000000, InstId::Stur_v)             , RWI_RW   , 0                         , 1  ), // #695
INST(Stur_v           , SimdLdurStur       , (0b0011110000000000000000)                                                            , RWI_RW   , 0                         , 1  ), // #696
INST(Sub_v            , ISimdVVV           , (0b0010111000100000100001, kVO_V_Any)                                                 , RWI_W    , 0                         , 37 ), // #697
INST(Subhn_v          , ISimdVVV           , (0b0000111000100000011000, kVO_V_B8H4S2)                                              , RWI_W    , F!(Narrow)                 , 38 ), // #698
INST(Subhn2_v         , ISimdVVV           , (0b0000111000100000011000, kVO_V_B16H8S4)                                             , RWI_X    , F!(Narrow)                 , 39 ), // #699
INST(Sudot_v          , SimdDot            , (0b0000000000000000000000, 0b0000111100000000111100, kET_S, kET_B, kET_4B)            , RWI_X    , 0                         , 2  ), // #700
INST(Suqadd_v         , ISimdVV            , (0b0000111000100000001110, kVO_SV_Any)                                                , RWI_X    , 0                         , 19 ), // #701
INST(Sxtl_v           , SimdSxtlUxtl       , (0b0000111100000000101001, kVO_V_B8H4S2)                                              , RWI_W    , F!(Long)                   , 0  ), // #702
INST(Sxtl2_v          , SimdSxtlUxtl       , (0b0100111100000000101001, kVO_V_B16H8S4)                                             , RWI_W    , F!(Long)                   , 1  ), // #703
INST(Tbl_v            , SimdTblTbx         , (0b0000111000000000000000)                                                            , RWI_W    , 0                         , 0  ), // #704
INST(Tbx_v            , SimdTblTbx         , (0b0000111000000000000100)                                                            , RWI_W    , 0                         , 1  ), // #705
INST(Trn1_v           , ISimdVVV           , (0b0000111000000000001010, kVO_V_BHS_D2)                                              , RWI_W    , 0                         , 40 ), // #706
INST(Trn2_v           , ISimdVVV           , (0b0000111000000000011010, kVO_V_BHS_D2)                                              , RWI_W    , 0                         , 41 ), // #707
INST(Uaba_v           , ISimdVVV           , (0b0010111000100000011111, kVO_V_BHS)                                                 , RWI_X    , 0                         , 42 ), // #708
INST(Uabal_v          , ISimdVVV           , (0b0010111000100000010100, kVO_V_B8H4S2)                                              , RWI_X    , F!(Long)                   , 43 ), // #709
INST(Uabal2_v         , ISimdVVV           , (0b0110111000100000010100, kVO_V_B16H8S4)                                             , RWI_X    , F!(Long)                   , 44 ), // #710
INST(Uabd_v           , ISimdVVV           , (0b0010111000100000011101, kVO_V_BHS)                                                 , RWI_W    , 0                         , 45 ), // #711
INST(Uabdl_v          , ISimdVVV           , (0b0010111000100000011100, kVO_V_B8H4S2)                                              , RWI_W    , F!(Long)                   , 46 ), // #712
INST(Uabdl2_v         , ISimdVVV           , (0b0110111000100000011100, kVO_V_B16H8S4)                                             , RWI_W    , F!(Long)                   , 47 ), // #713
INST(Uadalp_v         , ISimdVV            , (0b0010111000100000011010, kVO_V_BHS)                                                 , RWI_X    , F!(Long) | F!(Pair)         , 20 ), // #714
INST(Uaddl_v          , ISimdVVV           , (0b0010111000100000000000, kVO_V_B8H4S2)                                              , RWI_W    , F!(Long)                   , 48 ), // #715
INST(Uaddl2_v         , ISimdVVV           , (0b0110111000100000000000, kVO_V_B16H8S4)                                             , RWI_W    , F!(Long)                   , 49 ), // #716
INST(Uaddlp_v         , ISimdVV            , (0b0010111000100000001010, kVO_V_BHS)                                                 , RWI_W    , F!(Long) | F!(Pair)         , 21 ), // #717
INST(Uaddlv_v         , ISimdSV            , (0b0010111000110000001110, kVO_V_BH_4S)                                               , RWI_W    , F!(Long)                   , 4  ), // #718
INST(Uaddw_v          , ISimdWWV           , (0b0010111000100000000100, kVO_V_B8H4S2)                                              , RWI_W    , 0                         , 4  ), // #719
INST(Uaddw2_v         , ISimdWWV           , (0b0010111000100000000100, kVO_V_B16H8S4)                                             , RWI_W    , 0                         , 5  ), // #720
INST(Ucvtf_v          , SimdFcvtSV         , (0b0010111000100001110110, 0b0010111100000000111001, 0b0001111000100011000000, 0)     , RWI_W    , 0                         , 11 ), // #721
INST(Udot_v           , SimdDot            , (0b0010111010000000100101, 0b0010111110000000111000, kET_S, kET_B, kET_4B)            , RWI_X    , 0                         , 3  ), // #722
INST(Uhadd_v          , ISimdVVV           , (0b0010111000100000000001, kVO_V_BHS)                                                 , RWI_W    , 0                         , 50 ), // #723
INST(Uhsub_v          , ISimdVVV           , (0b0010111000100000001001, kVO_V_BHS)                                                 , RWI_W    , 0                         , 51 ), // #724
INST(Umax_v           , ISimdVVV           , (0b0010111000100000011001, kVO_V_BHS)                                                 , RWI_W    , 0                         , 52 ), // #725
INST(Umaxp_v          , ISimdVVV           , (0b0010111000100000101001, kVO_V_BHS)                                                 , RWI_W    , 0                         , 53 ), // #726
INST(Umaxv_v          , ISimdSV            , (0b0010111000110000101010, kVO_V_BH_4S)                                               , RWI_W    , 0                         , 5  ), // #727
INST(Umin_v           , ISimdVVV           , (0b0010111000100000011011, kVO_V_BHS)                                                 , RWI_W    , 0                         , 54 ), // #728
INST(Uminp_v          , ISimdVVV           , (0b0010111000100000101011, kVO_V_BHS)                                                 , RWI_W    , 0                         , 55 ), // #729
INST(Uminv_v          , ISimdSV            , (0b0010111000110001101010, kVO_V_BH_4S)                                               , RWI_W    , 0                         , 6  ), // #730
INST(Umlal_v          , ISimdVVVe          , (0b0010111000100000100000, kVO_V_B8H4S2, 0b0010111100000000001000, kVO_V_H4S2)        , RWI_X    , F!(Long) | F!(VH0_15)       , 19 ), // #731
INST(Umlal2_v         , ISimdVVVe          , (0b0110111000100000100000, kVO_V_B16H8S4, 0b0010111100000000001000, kVO_V_H8S4)       , RWI_X    , F!(Long) | F!(VH0_15)       , 20 ), // #732
INST(Umlsl_v          , ISimdVVVe          , (0b0010111000100000101000, kVO_V_B8H4S2, 0b0010111100000000011000, kVO_V_H4S2)        , RWI_X    , F!(Long) | F!(VH0_15)       , 21 ), // #733
INST(Umlsl2_v         , ISimdVVVe          , (0b0110111000100000101000, kVO_V_B16H8S4, 0b0110111100000000011000, kVO_V_H8S4)       , RWI_X    , F!(Long) | F!(VH0_15)       , 22 ), // #734
INST(Ummla_v          , ISimdVVVx          , (0b0110111010000000101001, kOp_V4S, kOp_V16B, kOp_V16B)                               , RWI_X    , 0                         , 15 ), // #735
INST(Umov_v           , SimdSmovUmov       , (0b0000111000000000001111, kVO_V_Any, 0)                                              , RWI_W    , 0                         , 1  ), // #736
INST(Umull_v          , ISimdVVVe          , (0b0010111000100000110000, kVO_V_B8H4S2, 0b0010111100000000101000, kVO_V_H4S2)        , RWI_W    , F!(Long) | F!(VH0_15)       , 23 ), // #737
INST(Umull2_v         , ISimdVVVe          , (0b0110111000100000110000, kVO_V_B16H8S4, 0b0110111100000000101000, kVO_V_H8S4)       , RWI_W    , F!(Long) | F!(VH0_15)       , 24 ), // #738
INST(Uqadd_v          , ISimdVVV           , (0b0010111000100000000011, kVO_SV_Any)                                                , RWI_W    , 0                         , 56 ), // #739
INST(Uqrshl_v         , SimdShift          , (0b0010111000100000010111, 0b0000000000000000000000, 0, kVO_SV_Any)                   , RWI_W    , 0                         , 26 ), // #740
INST(Uqrshrn_v        , SimdShift          , (0b0000000000000000000000, 0b0010111100000000100111, 1, kVO_SV_B8H4S2)                , RWI_W    , F!(Narrow)                 , 27 ), // #741
INST(Uqrshrn2_v       , SimdShift          , (0b0000000000000000000000, 0b0110111100000000100111, 1, kVO_V_B16H8S4)                , RWI_X    , F!(Narrow)                 , 28 ), // #742
INST(Uqshl_v          , SimdShift          , (0b0010111000100000010011, 0b0010111100000000011101, 0, kVO_SV_Any)                   , RWI_W    , 0                         , 29 ), // #743
INST(Uqshrn_v         , SimdShift          , (0b0000000000000000000000, 0b0010111100000000100101, 1, kVO_SV_B8H4S2)                , RWI_W    , F!(Narrow)                 , 30 ), // #744
INST(Uqshrn2_v        , SimdShift          , (0b0000000000000000000000, 0b0110111100000000100101, 1, kVO_V_B16H8S4)                , RWI_X    , F!(Narrow)                 , 31 ), // #745
INST(Uqsub_v          , ISimdVVV           , (0b0010111000100000001011, kVO_SV_Any)                                                , RWI_W    , 0                         , 57 ), // #746
INST(Uqxtn_v          , ISimdVV            , (0b0010111000100001010010, kVO_SV_B8H4S2)                                             , RWI_W    , F!(Narrow)                 , 22 ), // #747
INST(Uqxtn2_v         , ISimdVV            , (0b0110111000100001010010, kVO_V_B16H8S4)                                             , RWI_X    , F!(Narrow)                 , 23 ), // #748
INST(Urecpe_v         , ISimdVV            , (0b0000111010100001110010, kVO_V_S)                                                   , RWI_W    , 0                         , 24 ), // #749
INST(Urhadd_v         , ISimdVVV           , (0b0010111000100000000101, kVO_V_BHS)                                                 , RWI_W    , 0                         , 58 ), // #750
INST(Urshl_v          , SimdShift          , (0b0010111000100000010101, 0b0000000000000000000000, 0, kVO_V_Any)                    , RWI_W    , 0                         , 32 ), // #751
INST(Urshr_v          , SimdShift          , (0b0000000000000000000000, 0b0010111100000000001001, 1, kVO_V_Any)                    , RWI_W    , 0                         , 33 ), // #752
INST(Ursqrte_v        , ISimdVV            , (0b0010111010100001110010, kVO_V_S)                                                   , RWI_W    , 0                         , 25 ), // #753
INST(Ursra_v          , SimdShift          , (0b0000000000000000000000, 0b0010111100000000001101, 1, kVO_V_Any)                    , RWI_X    , 0                         , 34 ), // #754
INST(Usdot_v          , SimdDot            , (0b0000111010000000100111, 0b0000111110000000111100, kET_S, kET_B, kET_4B)            , RWI_X    , 0                         , 4  ), // #755
INST(Ushl_v           , SimdShift          , (0b0010111000100000010001, 0b0000000000000000000000, 0, kVO_V_Any)                    , RWI_W    , 0                         , 35 ), // #756
INST(Ushll_v          , SimdShift          , (0b0000000000000000000000, 0b0010111100000000101001, 0, kVO_V_B8H4S2)                 , RWI_W    , F!(Long)                   , 36 ), // #757
INST(Ushll2_v         , SimdShift          , (0b0000000000000000000000, 0b0110111100000000101001, 0, kVO_V_B16H8S4)                , RWI_W    , F!(Long)                   , 37 ), // #758
INST(Ushr_v           , SimdShift          , (0b0000000000000000000000, 0b0010111100000000000001, 1, kVO_V_Any)                    , RWI_W    , 0                         , 38 ), // #759
INST(Usmmla_v         , ISimdVVVx          , (0b0100111010000000101011, kOp_V4S, kOp_V16B, kOp_V16B)                               , RWI_X    , 0                         , 16 ), // #760
INST(Usqadd_v         , ISimdVV            , (0b0010111000100000001110, kVO_SV_Any)                                                , RWI_X    , 0                         , 26 ), // #761
INST(Usra_v           , SimdShift          , (0b0000000000000000000000, 0b0010111100000000000101, 1, kVO_V_Any)                    , RWI_X    , 0                         , 39 ), // #762
INST(Usubl_v          , ISimdVVV           , (0b0010111000100000001000, kVO_V_B8H4S2)                                              , RWI_W    , F!(Long)                   , 59 ), // #763
INST(Usubl2_v         , ISimdVVV           , (0b0110111000100000001000, kVO_V_B16H8S4)                                             , RWI_W    , F!(Long)                   , 60 ), // #764
INST(Usubw_v          , ISimdWWV           , (0b0010111000100000001100, kVO_V_B8H4S2)                                              , RWI_W    , 0                         , 6  ), // #765
INST(Usubw2_v         , ISimdWWV           , (0b0010111000100000001100, kVO_V_B16H8S4)                                             , RWI_W    , 0                         , 7  ), // #766
INST(Uxtl_v           , SimdSxtlUxtl       , (0b0010111100000000101001, kVO_V_B8H4S2)                                              , RWI_W    , F!(Long)                   , 2  ), // #767
INST(Uxtl2_v          , SimdSxtlUxtl       , (0b0110111100000000101001, kVO_V_B16H8S4)                                             , RWI_W    , F!(Long)                   , 3  ), // #768
INST(Uzp1_v           , ISimdVVV           , (0b0000111000000000000110, kVO_V_BHS_D2)                                              , RWI_W    , 0                         , 61 ), // #769
INST(Uzp2_v           , ISimdVVV           , (0b0000111000000000010110, kVO_V_BHS_D2)                                              , RWI_W    , 0                         , 62 ), // #770
INST(Xar_v            , ISimdVVVI          , (0b1100111001100000100011, kVO_V_D2, 6, 10, 0)                                        , RWI_W    , 0                         , 1  ), // #771
INST(Xtn_v            , ISimdVV            , (0b0000111000100001001010, kVO_V_B8H4S2)                                              , RWI_W    , F!(Narrow)                 , 27 ), // #772
INST(Xtn2_v           , ISimdVV            , (0b0100111000100001001010, kVO_V_B16H8S4)                                             , RWI_X    , F!(Narrow)                 , 28 ), // #773
INST(Zip1_v           , ISimdVVV           , (0b0000111000000000001110, kVO_V_BHS_D2)                                              , RWI_W    , 0                         , 63 ), // #774
INST(Zip2_v           , ISimdVVV           , (0b0000111000000000011110, kVO_V_BHS_D2)                                              , RWI_W    , 0                         , 64 ),  // #775

  });

pub const BASE_ADD_SUB: [BaseAddSub; 4] = [
    BaseAddSub::new(0b0001011000, 0b0001011001, 0b0010001), // add
    BaseAddSub::new(0b0101011000, 0b0101011001, 0b0110001), // adds
    BaseAddSub::new(0b1001011000, 0b1001011001, 0b1010001), // sub
    BaseAddSub::new(0b1101011000, 0b1101011001, 0b1110001), // subs
];

macro_rules! table_new {
        ($ty:ident, { $({ $($e:expr),* $(,)? }),* $(,)? }) => {
                [$( $ty::new($($e as u64 as _),*) ),*]
        };
}

const kW: u32 = W;
const kX: u32 = X;
const kWX: u32 = WX;
const kZR: u32 = ZR;
const kSP: u32 = SP;

const kHF_N: u32 = HFConv::N as u32;
const kHF_A: u32 = HFConv::A as u32;
const kHF_B: u32 = HFConv::B as u32;
const kHF_C: u32 = HFConv::C as u32;

const kET_B: u8 = InstElementType::B as u8;
const kET_H: u8 = InstElementType::H as u8;
const kET_S: u8 = InstElementType::S as u8;
const kET_2H: u8 = InstElementType::_2H as u8;
const kET_4B: u8 = InstElementType::_4B as u8;

const kOp_GpW: u32 = OpSignature::GpW as u32;
const kOp_H: u32 = OpSignature::H as u32;
const kOp_S: u32 = OpSignature::S as u32;
const kOp_D: u32 = OpSignature::D as u32;
const kOp_Q: u32 = OpSignature::Q as u32;
const kOp_V4H: u32 = OpSignature::V4H as u32;
const kOp_V8H: u32 = OpSignature::V8H as u32;
const kOp_V4S: u32 = OpSignature::V4S as u32;
const kOp_V2D: u32 = OpSignature::V2D as u32;
const kOp_V16B: u32 = OpSignature::V16B as u32;

const kVO_V_B: u32 = VOType::VB as u32;
const kVO_V_BH: u32 = VOType::VBH as u32;
const kVO_V_BH_4S: u32 = VOType::VBH4S as u32;
const kVO_V_BHS: u32 = VOType::VBHS as u32;
const kVO_V_BHS_D2: u32 = VOType::VBHSD2 as u32;
const kVO_V_HS: u32 = VOType::VHS as u32;
const kVO_V_S: u32 = VOType::VS as u32;
const kVO_V_B8H4S2: u32 = VOType::VB8H4S2 as u32;
const kVO_V_B8D1: u32 = VOType::VB8D1 as u32;
const kVO_V_H4S2: u32 = VOType::VH4S2 as u32;
const kVO_V_B16: u32 = VOType::VB16 as u32;
const kVO_V_B16H8S4: u32 = VOType::VB16H8S4 as u32;
const kVO_V_B16D2: u32 = VOType::VB16D2 as u32;
const kVO_V_H8S4: u32 = VOType::VH8S4 as u32;
const kVO_V_D2: u32 = VOType::VD2 as u32;
const kVO_SV_BHS: u32 = VOType::SVBHS as u32;
const kVO_SV_B8H4S2: u32 = VOType::SVB8H4S2 as u32;
const kVO_SV_HS: u32 = VOType::SVHS as u32;
const kVO_V_Any: u32 = VOType::VAny as u32;
const kVO_SV_Any: u32 = VOType::SVAny as u32;

pub const BASE_ADR: [BaseAdr; 2] = table_new!(BaseAdr, {
    { 0b0001000000000000000000, OffsetType::Adr as u8 }, // adr
    { 0b1001000000000000000000, OffsetType::Adrp as u8 }  // adrp
});

pub const BASE_AT_DC_IC_TLBI: [BaseAtDcIcTlbi; 4] = table_new!(BaseAtDcIcTlbi, {
    { 0b00011111110000, 0b00001111000000, true }, // at
    { 0b00011110000000, 0b00001110000000, true }, // dc
    { 0b00011110000000, 0b00001110000000, false }, // ic
    { 0b00011110000000, 0b00010000000000, false }  // tlbi
});

pub const BASE_ATOMIC_CASP: [BaseAtomicCasp; 4] = table_new!(BaseAtomicCasp, {
    { 0b0000100000100000011111, kWX, 30 }, // casp
    { 0b0000100001100000011111, kWX, 30 }, // caspa
    { 0b0000100001100000111111, kWX, 30 }, // caspal
    { 0b0000100000100000111111, kWX, 30 }  // caspl
});

pub const BASE_ATOMIC_OP: [BaseAtomicOp; 123] = table_new!(BaseAtomicOp, {
    { 0b1000100010100000011111, kWX, 30, 0 }, // cas
    { 0b1000100011100000011111, kWX, 30, 1 }, // casa
    { 0b0000100011100000011111, kW , 0 , 1 }, // casab
    { 0b0100100011100000011111, kW , 0 , 1 }, // casah
    { 0b1000100011100000111111, kWX, 30, 1 }, // casal
    { 0b0000100011100000111111, kW , 0 , 1 }, // casalb
    { 0b0100100011100000111111, kW , 0 , 1 }, // casalh
    { 0b0000100010100000011111, kW , 0 , 0 }, // casb
    { 0b0100100010100000011111, kW , 0 , 0 }, // cash
    { 0b1000100010100000111111, kWX, 30, 0 }, // casl
    { 0b0000100010100000111111, kW , 0 , 0 }, // caslb
    { 0b0100100010100000111111, kW , 0 , 0 }, // caslh
    { 0b1011100000100000000000, kWX, 30, 0 }, // ldadd
    { 0b1011100010100000000000, kWX, 30, 1 }, // ldadda
    { 0b0011100010100000000000, kW , 0 , 1 }, // ldaddab
    { 0b0111100010100000000000, kW , 0 , 1 }, // ldaddah
    { 0b1011100011100000000000, kWX, 30, 1 }, // ldaddal
    { 0b0011100011100000000000, kW , 0 , 1 }, // ldaddalb
    { 0b0111100011100000000000, kW , 0 , 1 }, // ldaddalh
    { 0b0011100000100000000000, kW , 0 , 0 }, // ldaddb
    { 0b0111100000100000000000, kW , 0 , 0 }, // ldaddh
    { 0b1011100001100000000000, kWX, 30, 0 }, // ldaddl
    { 0b0011100001100000000000, kW , 0 , 0 }, // ldaddlb
    { 0b0111100001100000000000, kW , 0 , 0 }, // ldaddlh
    { 0b1011100000100000000100, kWX, 30, 0 }, // ldclr
    { 0b1011100010100000000100, kWX, 30, 1 }, // ldclra
    { 0b0011100010100000000100, kW , 0 , 1 }, // ldclrab
    { 0b0111100010100000000100, kW , 0 , 1 }, // ldclrah
    { 0b1011100011100000000100, kWX, 30, 1 }, // ldclral
    { 0b0011100011100000000100, kW , 0 , 1 }, // ldclralb
    { 0b0111100011100000000100, kW , 0 , 1 }, // ldclralh
    { 0b0011100000100000000100, kW , 0 , 0 }, // ldclrb
    { 0b0111100000100000000100, kW , 0 , 0 }, // ldclrh
    { 0b1011100001100000000100, kWX, 30, 0 }, // ldclrl
    { 0b0011100001100000000100, kW , 0 , 0 }, // ldclrlb
    { 0b0111100001100000000100, kW , 0 , 0 }, // ldclrlh
    { 0b1011100000100000001000, kWX, 30, 0 }, // ldeor
    { 0b1011100010100000001000, kWX, 30, 1 }, // ldeora
    { 0b0011100010100000001000, kW , 0 , 1 }, // ldeorab
    { 0b0111100010100000001000, kW , 0 , 1 }, // ldeorah
    { 0b1011100011100000001000, kWX, 30, 1 }, // ldeoral
    { 0b0011100011100000001000, kW , 0 , 1 }, // ldeoralb
    { 0b0111100011100000001000, kW , 0 , 1 }, // ldeoralh
    { 0b0011100000100000001000, kW , 0 , 0 }, // ldeorb
    { 0b0111100000100000001000, kW , 0 , 0 }, // ldeorh
    { 0b1011100001100000001000, kWX, 30, 0 }, // ldeorl
    { 0b0011100001100000001000, kW , 0 , 0 }, // ldeorlb
    { 0b0111100001100000001000, kW , 0 , 0 }, // ldeorlh
    { 0b1011100000100000001100, kWX, 30, 0 }, // ldset
    { 0b1011100010100000001100, kWX, 30, 1 }, // ldseta
    { 0b0011100010100000001100, kW , 0 , 1 }, // ldsetab
    { 0b0111100010100000001100, kW , 0 , 1 }, // ldsetah
    { 0b1011100011100000001100, kWX, 30, 1 }, // ldsetal
    { 0b0011100011100000001100, kW , 0 , 1 }, // ldsetalb
    { 0b0111100011100000001100, kW , 0 , 1 }, // ldsetalh
    { 0b0011100000100000001100, kW , 0 , 0 }, // ldsetb
    { 0b0111100000100000001100, kW , 0 , 0 }, // ldseth
    { 0b1011100001100000001100, kWX, 30, 0 }, // ldsetl
    { 0b0011100001100000001100, kW , 0 , 0 }, // ldsetlb
    { 0b0111100001100000001100, kW , 0 , 0 }, // ldsetlh
    { 0b1011100000100000010000, kWX, 30, 0 }, // ldsmax
    { 0b1011100010100000010000, kWX, 30, 1 }, // ldsmaxa
    { 0b0011100010100000010000, kW , 0 , 1 }, // ldsmaxab
    { 0b0111100010100000010000, kW , 0 , 1 }, // ldsmaxah
    { 0b1011100011100000010000, kWX, 30, 1 }, // ldsmaxal
    { 0b0011100011100000010000, kW , 0 , 1 }, // ldsmaxalb
    { 0b0111100011100000010000, kW , 0 , 1 }, // ldsmaxalh
    { 0b0011100000100000010000, kW , 0 , 0 }, // ldsmaxb
    { 0b0111100000100000010000, kW , 0 , 0 }, // ldsmaxh
    { 0b1011100001100000010000, kWX, 30, 0 }, // ldsmaxl
    { 0b0011100001100000010000, kW , 0 , 0 }, // ldsmaxlb
    { 0b0111100001100000010000, kW , 0 , 0 }, // ldsmaxlh
    { 0b1011100000100000010100, kWX, 30, 0 }, // ldsmin
    { 0b1011100010100000010100, kWX, 30, 1 }, // ldsmina
    { 0b0011100010100000010100, kW , 0 , 1 }, // ldsminab
    { 0b0111100010100000010100, kW , 0 , 1 }, // ldsminah
    { 0b1011100011100000010100, kWX, 30, 1 }, // ldsminal
    { 0b0011100011100000010100, kW , 0 , 1 }, // ldsminalb
    { 0b0111100011100000010100, kW , 0 , 1 }, // ldsminalh
    { 0b0011100000100000010100, kW , 0 , 0 }, // ldsminb
    { 0b0111100000100000010100, kW , 0 , 0 }, // ldsminh
    { 0b1011100001100000010100, kWX, 30, 0 }, // ldsminl
    { 0b0011100001100000010100, kW , 0 , 0 }, // ldsminlb
    { 0b0111100001100000010100, kW , 0 , 0 }, // ldsminlh
    { 0b1011100000100000011000, kWX, 30, 0 }, // ldumax
    { 0b1011100010100000011000, kWX, 30, 1 }, // ldumaxa
    { 0b0011100010100000011000, kW , 0 , 1 }, // ldumaxab
    { 0b0111100010100000011000, kW , 0 , 1 }, // ldumaxah
    { 0b1011100011100000011000, kWX, 30, 1 }, // ldumaxal
    { 0b0011100011100000011000, kW , 0 , 1 }, // ldumaxalb
    { 0b0111100011100000011000, kW , 0 , 1 }, // ldumaxalh
    { 0b0011100000100000011000, kW , 0 , 0 }, // ldumaxb
    { 0b0111100000100000011000, kW , 0 , 0 }, // ldumaxh
    { 0b1011100001100000011000, kWX, 30, 0 }, // ldumaxl
    { 0b0011100001100000011000, kW , 0 , 0 }, // ldumaxlb
    { 0b0111100001100000011000, kW , 0 , 0 }, // ldumaxlh
    { 0b1011100000100000011100, kWX, 30, 0 }, // ldumin
    { 0b1011100010100000011100, kWX, 30, 1 }, // ldumina
    { 0b0011100010100000011100, kW , 0 , 1 }, // lduminab
    { 0b0111100010100000011100, kW , 0 , 1 }, // lduminah
    { 0b1011100011100000011100, kWX, 30, 1 }, // lduminal
    { 0b0011100011100000011100, kW , 0 , 1 }, // lduminalb
    { 0b0111100011100000011100, kW , 0 , 1 }, // lduminalh
    { 0b0011100000100000011100, kW , 0 , 0 }, // lduminb
    { 0b0111100000100000011100, kW , 0 , 0 }, // lduminh
    { 0b1011100001100000011100, kWX, 30, 0 }, // lduminl
    { 0b0011100001100000011100, kW , 0 , 0 }, // lduminlb
    { 0b0111100001100000011100, kW , 0 , 0 }, // lduminlh
    { 0b1000100000000000111111, kWX, 30, 1 }, // stlxr
    { 0b0000100000000000111111, kW , 0 , 1 }, // stlxrb
    { 0b0100100000000000111111, kW , 0 , 1 }, // stlxrh
    { 0b1011100000100000100000, kWX, 30, 1 }, // swp
    { 0b1011100010100000100000, kWX, 30, 1 }, // swpa
    { 0b0011100010100000100000, kW , 0 , 1 }, // swpab
    { 0b0111100010100000100000, kW , 0 , 1 }, // swpah
    { 0b1011100011100000100000, kWX, 30, 1 }, // swpal
    { 0b0011100011100000100000, kW , 0 , 1 }, // swpalb
    { 0b0111100011100000100000, kW , 0 , 1 }, // swpalh
    { 0b0011100000100000100000, kW , 0 , 1 }, // swpb
    { 0b0111100000100000100000, kW , 0 , 1 }, // swph
    { 0b1011100001100000100000, kWX, 30, 1 }, // swpl
    { 0b0011100001100000100000, kW , 0 , 1 }, // swplb
    { 0b0111100001100000100000, kW , 0 , 1 }  // swplh
});

pub const BASE_ATOMIC_ST: [BaseAtomicSt; 48] = table_new!(BaseAtomicSt, {
    { 0b1011100000100000000000, kWX, 30 }, // stadd
    { 0b1011100001100000000000, kWX, 30 }, // staddl
    { 0b0011100000100000000000, kW , 0  }, // staddb
    { 0b0011100001100000000000, kW , 0  }, // staddlb
    { 0b0111100000100000000000, kW , 0  }, // staddh
    { 0b0111100001100000000000, kW , 0  }, // staddlh
    { 0b1011100000100000000100, kWX, 30 }, // stclr
    { 0b1011100001100000000100, kWX, 30 }, // stclrl
    { 0b0011100000100000000100, kW , 0  }, // stclrb
    { 0b0011100001100000000100, kW , 0  }, // stclrlb
    { 0b0111100000100000000100, kW , 0  }, // stclrh
    { 0b0111100001100000000100, kW , 0  }, // stclrlh
    { 0b1011100000100000001000, kWX, 30 }, // steor
    { 0b1011100001100000001000, kWX, 30 }, // steorl
    { 0b0011100000100000001000, kW , 0  }, // steorb
    { 0b0011100001100000001000, kW , 0  }, // steorlb
    { 0b0111100000100000001000, kW , 0  }, // steorh
    { 0b0111100001100000001000, kW , 0  }, // steorlh
    { 0b1011100000100000001100, kWX, 30 }, // stset
    { 0b1011100001100000001100, kWX, 30 }, // stsetl
    { 0b0011100000100000001100, kW , 0  }, // stsetb
    { 0b0011100001100000001100, kW , 0  }, // stsetlb
    { 0b0111100000100000001100, kW , 0  }, // stseth
    { 0b0111100001100000001100, kW , 0  }, // stsetlh
    { 0b1011100000100000010000, kWX, 30 }, // stsmax
    { 0b1011100001100000010000, kWX, 30 }, // stsmaxl
    { 0b0011100000100000010000, kW , 0  }, // stsmaxb
    { 0b0011100001100000010000, kW , 0  }, // stsmaxlb
    { 0b0111100000100000010000, kW , 0  }, // stsmaxh
    { 0b0111100001100000010000, kW , 0  }, // stsmaxlh
    { 0b1011100000100000010100, kWX, 30 }, // stsmin
    { 0b1011100001100000010100, kWX, 30 }, // stsminl
    { 0b0011100000100000010100, kW , 0  }, // stsminb
    { 0b0011100001100000010100, kW , 0  }, // stsminlb
    { 0b0111100000100000010100, kW , 0  }, // stsminh
    { 0b0111100001100000010100, kW , 0  }, // stsminlh
    { 0b1011100000100000011000, kWX, 30 }, // stumax
    { 0b1011100001100000011000, kWX, 30 }, // stumaxl
    { 0b0011100000100000011000, kW , 0  }, // stumaxb
    { 0b0011100001100000011000, kW , 0  }, // stumaxlb
    { 0b0111100000100000011000, kW , 0  }, // stumaxh
    { 0b0111100001100000011000, kW , 0  }, // stumaxlh
    { 0b1011100000100000011100, kWX, 30 }, // stumin
    { 0b1011100001100000011100, kWX, 30 }, // stuminl
    { 0b0011100000100000011100, kW , 0  }, // stuminb
    { 0b0011100001100000011100, kW , 0  }, // stuminlb
    { 0b0111100000100000011100, kW , 0  }, // stuminh
    { 0b0111100001100000011100, kW , 0  }  // stuminlh
});

pub const BASE_BFC: [BaseBfc; 1] = table_new!(BaseBfc, {
    { 0b00110011000000000000001111100000 } // bfc
});

pub const BASE_BFI: [BaseBfi; 3] = table_new!(BaseBfi, {
    { 0b00110011000000000000000000000000 }, // bfi
    { 0b00010011000000000000000000000000 }, // sbfiz
    { 0b01010011000000000000000000000000 }  // ubfiz
});

pub const BASE_BFM: [BaseBfm; 3] = table_new!(BaseBfm, {
    { 0b00110011000000000000000000000000 }, // bfm
    { 0b00010011000000000000000000000000 }, // sbfm
    { 0b01010011000000000000000000000000 }  // ubfm
});

pub const BASE_BFX: [BaseBfx; 3] = table_new!(BaseBfx, {
    { 0b00110011000000000000000000000000 }, // bfxil
    { 0b00010011000000000000000000000000 }, // sbfx
    { 0b01010011000000000000000000000000 }  // ubfx
});

pub const BASE_BRANCH_CMP: [BaseBranchCmp; 2] = table_new!(BaseBranchCmp, {
    { 0b00110101000000000000000000000000 }, // cbnz
    { 0b00110100000000000000000000000000 }  // cbz
});

pub const BASE_BRANCH_REG: [BaseBranchReg; 3] = table_new!(BaseBranchReg, {
    { 0b11010110001111110000000000000000u32 as i32 }, // blr
    { 0b11010110000111110000000000000000u32 as i32 }, // br
    { 0b11010110010111110000000000000000u32 as i32 }  // ret
});

pub const BASE_BRANCH_REL: [BaseBranchRel; 3] = table_new!(BaseBranchRel, {
    { 0b00010100000000000000000000000000 }, // b
    { 0b00010100000000000000000000010000 }, // bc
    { 0b10010100000000000000000000000000u32 as i32 }  // bl
});

pub const BASE_BRANCH_TST: [BaseBranchTst; 2] = table_new!(BaseBranchTst, {
    { 0b00110111000000000000000000000000 }, // tbnz
    { 0b00110110000000000000000000000000 }  // tbz
});

pub const BASE_C_CMP: [BaseCCmp; 2] = table_new!(BaseCCmp, {
    { 0b00111010010000000000000000000000 }, // ccmn
    { 0b01111010010000000000000000000000 }  // ccmp
});

pub const BASE_C_INC: [BaseCInc; 3] = table_new!(BaseCInc, {
    { 0b00011010100000000000010000000000 }, // cinc
    { 0b01011010100000000000000000000000 }, // cinv
    { 0b01011010100000000000010000000000 }  // cneg
});

pub const BASE_C_SEL: [BaseCSel; 4] = table_new!(BaseCSel, {
    { 0b00011010100000000000000000000000 }, // csel
    { 0b00011010100000000000010000000000 }, // csinc
    { 0b01011010100000000000000000000000 }, // csinv
    { 0b01011010100000000000010000000000 }  // csneg
});

pub const BASE_C_SET: [BaseCSet; 2] = table_new!(BaseCSet, {
    { 0b00011010100111110000011111100000 }, // cset
    { 0b01011010100111110000001111100000 }  // csetm
});

pub const BASE_CMP_CMN: [BaseCmpCmn; 2] = table_new!(BaseCmpCmn, {
    { 0b0101011000, 0b0101011001, 0b0110001 }, // cmn
    { 0b1101011000, 0b1101011001, 0b1110001 }  // cmp
});

pub const BASE_EXTEND: [BaseExtend; 5] = table_new!(BaseExtend, {
    { 0b0001001100000000000111, kWX, 0 }, // sxtb
    { 0b0001001100000000001111, kWX, 0 }, // sxth
    { 0b1001001101000000011111, kX , 0 }, // sxtw
    { 0b0101001100000000000111, kW, 1 }, // uxtb
    { 0b0101001100000000001111, kW, 1 }  // uxth
});

pub const BASE_EXTRACT: [BaseExtract; 1] = table_new!(BaseExtract, {
    { 0b00010011100000000000000000000000 } // extr
});

pub const BASE_LD_ST: [BaseLdSt; 9] = table_new!(BaseLdSt, {
    { 0b1011100101, 0b10111000010, 0b10111000011, 0b00011000, kWX, 30, 2, InstId::Ldur }, // ldr
    { 0b0011100101, 0b00111000010, 0b00111000011, 0         , kW , 0 , 0, InstId::Ldurb }, // ldrb
    { 0b0111100101, 0b01111000010, 0b01111000011, 0         , kW , 0 , 1, InstId::Ldurh }, // ldrh
    { 0b0011100111, 0b00111000100, 0b00111000111, 0         , kWX, 22, 0, InstId::Ldursb }, // ldrsb
    { 0b0111100111, 0b01111000100, 0b01111000111, 0         , kWX, 22, 1, InstId::Ldursh }, // ldrsh
    { 0b1011100110, 0b10111000100, 0b10111000101, 0b10011000, kX , 0 , 2, InstId::Ldursw }, // ldrsw
    { 0b1011100100, 0b10111000000, 0b10111000001, 0         , kWX, 30, 2, InstId::Stur }, // str
    { 0b0011100100, 0b00111000000, 0b00111000001, 0         , kW , 30, 0, InstId::Sturb }, // strb
    { 0b0111100100, 0b01111000000, 0b01111000001, 0         , kWX, 30, 1, InstId::Sturh }  // strh
});

pub const BASE_LDP_STP: [BaseLdpStp; 6] = table_new!(BaseLdpStp, {
    { 0b0010100001, 0           , kWX, 31, 2 }, // ldnp
    { 0b0010100101, 0b0010100011, kWX, 31, 2 }, // ldp
    { 0b0110100101, 0b0110100011, kX , 0 , 2 }, // ldpsw
    { 0b0110100100, 0b0110100010, kX, 0, 4 }, // stgp
    { 0b0010100000, 0           , kWX, 31, 2 }, // stnp
    { 0b0010100100, 0b0010100010, kWX, 31, 2 }  // stp
});

pub const BASE_LDXP: [BaseLdxp; 2] = table_new!(BaseLdxp, {
    { 0b1000100001111111100000, kWX, 30 }, // ldaxp
    { 0b1000100001111111000000, kWX, 30 }  // ldxp
});

pub const BASE_LOGICAL: [BaseLogical; 8] = table_new!(BaseLogical, {
    { 0b0001010000, 0b00100100, 0 }, // and
    { 0b1101010000, 0b11100100, 0 }, // ands
    { 0b0001010001, 0b00100100, 1 }, // bic
    { 0b1101010001, 0b11100100, 1 }, // bics
    { 0b1001010001, 0b10100100, 1 }, // eon
    { 0b1001010000, 0b10100100, 0 }, // eor
    { 0b0101010001, 0b01100100, 1 }, // orn
    { 0b0101010000, 0b01100100, 0 }  // orr
});

pub const BASE_MIN_MAX: [BaseMinMax; 4] = table_new!(BaseMinMax, {
    { 0b00011010110000000110000000000000, 0b00010001110000000000000000000000 }, // smax
    { 0b00011010110000000110100000000000, 0b00010001110010000000000000000000 }, // smin
    { 0b00011010110000000110010000000000, 0b00010001110001000000000000000000 }, // umax
    { 0b00011010110000000110110000000000, 0b00010001110011000000000000000000 }  // umin
});

pub const BASE_MOV_KNZ: [BaseMovKNZ; 3] = table_new!(BaseMovKNZ, {
    { 0b01110010100000000000000000000000 }, // movk
    { 0b00010010100000000000000000000000 }, // movn
    { 0b01010010100000000000000000000000 }  // movz
});

pub const BASE_MVN_NEG: [BaseMvnNeg; 3] = table_new!(BaseMvnNeg, {
    { 0b00101010001000000000001111100000 }, // mvn
    { 0b01001011000000000000001111100000 }, // neg
    { 0b01101011000000000000001111100000 }  // negs
});

pub const BASE_OP: [BaseOp; 24] = table_new!(BaseOp, {
    { 0b11010101000000110010000110011111 }, // autia1716
    { 0b11010101000000110010001110111111 }, // autiasp
    { 0b11010101000000110010001110011111 }, // autiaz
    { 0b11010101000000110010000111011111 }, // autib1716
    { 0b11010101000000110010001111111111 }, // autibsp
    { 0b11010101000000110010001111011111 }, // autibz
    { 0b11010101000000000100000001011111 }, // axflag
    { 0b11010101000000000100000000011111 }, // cfinv
    { 0b11010101000000110010001011011111 }, // clrbhb
    { 0b11010101000000110010001010011111 }, // csdb
    { 0b11010101000000110010000011011111 }, // dgh
    { 0b11010110101111110000001111100000 }, // drps
    { 0b11010101000000110010001000011111 }, // esb
    { 0b11010110100111110000001111100000 }, // eret
    { 0b11010101000000110010000000011111 }, // nop
    { 0b11010101000000110011010010011111 }, // pssbb
    { 0b11010101000000110010000010011111 }, // sev
    { 0b11010101000000110010000010111111 }, // sevl
    { 0b11010101000000110011000010011111 }, // ssbb
    { 0b11010101000000110010000001011111 }, // wfe
    { 0b11010101000000110010000001111111 }, // wfi
    { 0b11010101000000000100000000111111 }, // xaflag
    { 0b11010101000000110010000011111111 }, // xpaclri
    { 0b11010101000000110010000000111111 }  // yield
});

pub const BASE_OP_IMM: [BaseOpImm; 15] = table_new!(BaseOpImm, {
    { 0b11010100001000000000000000000000, 16, 5 }, // brk
    { 0b11010101000000110010010000011111, 2, 6 }, // bti
    { 0b11010101000000110011000001011111, 4, 8 }, // clrex
    { 0b11010100101000000000000000000001, 16, 5 }, // dcps1
    { 0b11010100101000000000000000000010, 16, 5 }, // dcps2
    { 0b11010100101000000000000000000011, 16, 5 }, // dcps3
    { 0b11010101000000110011000010111111, 4, 8 }, // dmb
    { 0b11010101000000110011000010011111, 4, 8 }, // dsb
    { 0b11010101000000110010000000011111, 7, 5 }, // hint
    { 0b11010100010000000000000000000000, 16, 5 }, // hlt
    { 0b11010100000000000000000000000010, 16, 5 }, // hvc
    { 0b11010101000000110011000011011111, 4, 8 }, // isb
    { 0b11010100000000000000000000000011, 16, 5 }, // smc
    { 0b11010100000000000000000000000001, 16, 5 }, // svc
    { 0b00000000000000000000000000000000, 16, 0 }  // udf
});

pub const BASE_OP_X16: [BaseOpX16; 1] = table_new!(BaseOpX16, {
    { 0b11010101000000110010010100011111 } // chkfeat
});

pub const BASE_PRFM: [BasePrfm; 1] = table_new!(BasePrfm, {
    { 0b11111000101, 0b1111100110, 0b11111000100, 0b11011000 }  // prfm
});

pub const BASE_R: [BaseR; 10] = table_new!(BaseR, {
    { 0b11011010110000010011101111100000, kX, kZR, 0 }, // autdza
    { 0b11011010110000010011111111100000, kX, kZR, 0 }, // autdzb
    { 0b11011010110000010011001111100000, kX, kZR, 0 }, // autiza
    { 0b11011010110000010011011111100000, kX, kZR, 0 }, // autizb
    { 0b11011010110000010010101111100000, kX, kZR, 0 }, // pacdza
    { 0b11011010110000010010111111100000, kX, kZR, 0 }, // pacdzb
    { 0b00111010000000000000100000001101, kW, kZR, 5 }, // setf8
    { 0b00111010000000000100100000001101, kW, kZR, 5 }, // setf16
    { 0b11011010110000010100011111100000, kX, kZR, 0 }, // xpacd
    { 0b11011010110000010100001111100000, kX, kZR, 0 }  // xpaci
});

pub const BASE_RM_NO_IMM: [BaseRMNoImm; 21] = table_new!(BaseRMNoImm, {
    { 0b1000100011011111111111, kWX, kZR, 30 }, // ldar
    { 0b0000100011011111111111, kW , kZR, 0  }, // ldarb
    { 0b0100100011011111111111, kW , kZR, 0  }, // ldarh
    { 0b1000100001011111111111, kWX, kZR, 30 }, // ldaxr
    { 0b0000100001011111111111, kW , kZR, 0  }, // ldaxrb
    { 0b0100100001011111111111, kW , kZR, 0  }, // ldaxrh
    { 0b1101100111100000000000, kX , kZR, 0  }, // ldgm
    { 0b1000100011011111011111, kWX, kZR, 30 }, // ldlar
    { 0b0000100011011111011111, kW , kZR, 0  }, // ldlarb
    { 0b0100100011011111011111, kW , kZR, 0  }, // ldlarh
    { 0b1000100001011111011111, kWX, kZR, 30 }, // ldxr
    { 0b0000100001011111011111, kW , kZR, 0  }, // ldxrb
    { 0b0100100001011111011111, kW , kZR, 0  }, // ldxrh
    { 0b1101100110100000000000, kX , kZR, 0  }, // stgm
    { 0b1000100010011111011111, kWX, kZR, 30 }, // stllr
    { 0b0000100010011111011111, kW , kZR, 0  }, // stllrb
    { 0b0100100010011111011111, kW , kZR, 0  }, // stllrh
    { 0b1000100010011111111111, kWX, kZR, 30 }, // stlr
    { 0b0000100010011111111111, kW , kZR, 0  }, // stlrb
    { 0b0100100010011111111111, kW , kZR, 0  }, // stlrh
    { 0b1101100100100000000000, kX , kZR, 0 }  // stzgm
});

pub const BASE_RM_SIMM10: [BaseRMSImm10; 2] = table_new!(BaseRMSImm10, {
    { 0b1111100000100000000001, kX , kZR, 0, 3 }, // ldraa
    { 0b1111100010100000000001, kX , kZR, 0, 3 }  // ldrab
});

pub const BASE_RM_SIMM9: [BaseRMSImm9; 23] = table_new!(BaseRMSImm9, {
    { 0b1101100101100000000000, 0b0000000000000000000000, kX , kZR, 0, 4 }, // ldg
    { 0b1011100001000000000010, 0b0000000000000000000000, kWX, kZR, 30, 0 }, // ldtr
    { 0b0011100001000000000010, 0b0000000000000000000000, kW , kZR, 0 , 0 }, // ldtrb
    { 0b0111100001000000000010, 0b0000000000000000000000, kW , kZR, 0 , 0 }, // ldtrh
    { 0b0011100011000000000010, 0b0000000000000000000000, kWX, kZR, 22, 0 }, // ldtrsb
    { 0b0111100011000000000010, 0b0000000000000000000000, kWX, kZR, 22, 0 }, // ldtrsh
    { 0b1011100010000000000010, 0b0000000000000000000000, kX , kZR, 0 , 0 }, // ldtrsw
    { 0b1011100001000000000000, 0b0000000000000000000000, kWX, kZR, 30, 0 }, // ldur
    { 0b0011100001000000000000, 0b0000000000000000000000, kW , kZR, 0 , 0 }, // ldurb
    { 0b0111100001000000000000, 0b0000000000000000000000, kW , kZR, 0 , 0 }, // ldurh
    { 0b0011100011000000000000, 0b0000000000000000000000, kWX, kZR, 22, 0 }, // ldursb
    { 0b0111100011000000000000, 0b0000000000000000000000, kWX, kZR, 22, 0 }, // ldursh
    { 0b1011100010000000000000, 0b0000000000000000000000, kX , kZR, 0 , 0 }, // ldursw
    { 0b1101100110100000000010, 0b1101100110100000000001, kX, kSP, 0, 4 }, // st2g
    { 0b1101100100100000000010, 0b1101100100100000000001, kX, kSP, 0, 4 }, // stg
    { 0b1011100000000000000010, 0b0000000000000000000000, kWX, kZR, 30, 0 }, // sttr
    { 0b0011100000000000000010, 0b0000000000000000000000, kW , kZR, 0 , 0 }, // sttrb
    { 0b0111100000000000000010, 0b0000000000000000000000, kW , kZR, 0 , 0 }, // sttrh
    { 0b1011100000000000000000, 0b0000000000000000000000, kWX, kZR, 30, 0 }, // stur
    { 0b0011100000000000000000, 0b0000000000000000000000, kW , kZR, 0 , 0 }, // sturb
    { 0b0111100000000000000000, 0b0000000000000000000000, kW , kZR, 0 , 0 }, // sturh
    { 0b1101100111100000000010, 0b1101100111100000000001, kX , kSP, 0, 4 }, // stz2g
    { 0b1101100101100000000010, 0b1101100101100000000001, kX , kSP, 0, 4 }  // stzg
});

pub const BASE_RR: [BaseRR; 18] = table_new!(BaseRR, {
    { 0b01011010110000000010000000000000, kWX, kZR, 0, kWX, kZR, 5, true }, // abs
    { 0b11011010110000010001100000000000, kX, kZR, 0, kX, kSP, 5, true }, // autda
    { 0b11011010110000010001110000000000, kX, kZR, 0, kX, kSP, 5, true }, // autdb
    { 0b11011010110000010001000000000000, kX, kZR, 0, kX, kSP, 5, true }, // autia
    { 0b11011010110000010001010000000000, kX, kZR, 0, kX, kSP, 5, true }, // autib
    { 0b01011010110000000001010000000000, kWX, kZR, 0, kWX, kZR, 5, true }, // cls
    { 0b01011010110000000001000000000000, kWX, kZR, 0, kWX, kZR, 5, true }, // clz
    { 0b10111010110000000000000000011111, kX, kSP, 5, kX, kSP, 16, true }, // cmpp
    { 0b01011010110000000001110000000000, kWX, kZR, 0, kWX, kZR, 5, true }, // cnt
    { 0b01011010110000000001100000000000, kWX, kZR, 0, kWX, kZR, 5, true }, // ctz
    { 0b01011010000000000000001111100000, kWX, kZR, 0, kWX, kZR, 16, true }, // ngc
    { 0b01111010000000000000001111100000, kWX, kZR, 0, kWX, kZR, 16, true }, // ngcs
    { 0b11011010110000010000100000000000, kX, kZR, 0, kX, kSP, 5, true }, // pacda
    { 0b11011010110000010000110000000000, kX, kZR, 0, kX, kSP, 5, true }, // pacdb
    { 0b01011010110000000000000000000000, kWX, kZR, 0, kWX, kZR, 5, true }, // rbit
    { 0b01011010110000000000010000000000, kWX, kZR, 0, kWX, kZR, 5, true }, // rev16
    { 0b11011010110000000000100000000000, kWX, kZR, 0, kWX, kZR, 5, true }, // rev32
    { 0b11011010110000000000110000000000, kWX, kZR, 0, kWX, kZR, 5, true }  // rev64
});

pub const BASE_RRII: [BaseRRII; 2] = table_new!(BaseRRII, {
    { 0b1001000110000000000000, kX, kSP, kX, kSP, 6, 4, 16, 4, 0, 10 }, // addg
    { 0b1101000110000000000000, kX, kSP, kX, kSP, 6, 4, 16, 4, 0, 10 }  // subg
});

pub const BASE_RRR: [BaseRRR; 26] = table_new!(BaseRRR, {
    { 0b0001101000000000000000, kWX, kZR, kWX, kZR, kWX, kZR, true }, // adc
    { 0b0011101000000000000000, kWX, kZR, kWX, kZR, kWX, kZR, true }, // adcs
    { 0b0001101011000000010000, kW, kZR, kW, kZR, kW, kZR, false }, // crc32b
    { 0b0001101011000000010100, kW, kZR, kW, kZR, kW, kZR, false }, // crc32cb
    { 0b0001101011000000010101, kW, kZR, kW, kZR, kW, kZR, false }, // crc32ch
    { 0b0001101011000000010110, kW, kZR, kW, kZR, kW, kZR, false }, // crc32cw
    { 0b1001101011000000010111, kW, kZR, kW, kZR, kX, kZR, false }, // crc32cx
    { 0b0001101011000000010001, kW, kZR, kW, kZR, kW, kZR, false }, // crc32h
    { 0b0001101011000000010010, kW, kZR, kW, kZR, kW, kZR, false }, // crc32w
    { 0b1001101011000000010011, kW, kZR, kW, kZR, kX, kZR, false }, // crc32x
    { 0b1001101011000000000101, kX , kZR, kX , kSP, kX , kZR, true }, // gmi
    { 0b0001101100000000111111, kWX, kZR, kWX, kZR, kWX, kZR, true }, // mneg
    { 0b0001101100000000011111, kWX, kZR, kWX, kZR, kWX, kZR, true }, // mul
    { 0b1001101011000000001100, kX, kZR, kX, kZR, kX, kSP, false }, // pacga
    { 0b0101101000000000000000, kWX, kZR, kWX, kZR, kWX, kZR, true }, // sbc
    { 0b0111101000000000000000, kWX, kZR, kWX, kZR, kWX, kZR, true }, // sbcs
    { 0b0001101011000000000011, kWX, kZR, kWX, kZR, kWX, kZR, true }, // sdiv
    { 0b1001101100100000111111, kX , kZR, kW , kZR, kW , kZR, false }, // smnegl
    { 0b1001101101000000011111, kX , kZR, kX , kZR, kX , kZR, true }, // smulh
    { 0b1001101100100000011111, kX , kZR, kW , kZR, kW , kZR, false }, // smull
    { 0b1001101011000000000000, kX, kZR, kX, kSP, kX, kSP, false }, // subp
    { 0b1011101011000000000000, kX, kZR, kX, kSP, kX, kSP, false }, // subps
    { 0b0001101011000000000010, kWX, kZR, kWX, kZR, kWX, kZR, true }, // udiv
    { 0b1001101110100000111111, kX , kZR, kW , kZR, kW , kZR, false }, // umnegl
    { 0b1001101110100000011111, kX , kZR, kW , kZR, kW , kZR, false }, // umull
    { 0b1001101111000000011111, kX , kZR, kX , kZR, kX , kZR, false }  // umulh
});

pub const BASE_RRRR: [BaseRRRR; 6] = table_new!(BaseRRRR, {
    { 0b0001101100000000000000, kWX, kZR, kWX, kZR, kWX, kZR, kWX, kZR, true }, // madd
    { 0b0001101100000000100000, kWX, kZR, kWX, kZR, kWX, kZR, kWX, kZR, true }, // msub
    { 0b1001101100100000000000, kX , kZR, kW , kZR, kW , kZR, kX , kZR, false }, // smaddl
    { 0b1001101100100000100000, kX , kZR, kW , kZR, kW , kZR, kX , kZR, false }, // smsubl
    { 0b1001101110100000000000, kX , kZR, kW , kZR, kW , kZR, kX , kZR, false }, // umaddl
    { 0b1001101110100000100000, kX , kZR, kW , kZR, kW , kZR, kX , kZR, false }  // umsubl
});

pub const BASE_SHIFT: [BaseShift; 8] = table_new!(BaseShift, {
    { 0b0001101011000000001010, 0b0001001100000000011111, 0 }, // asr
    { 0b0001101011000000001010, 0b0000000000000000000000, 0 }, // asrv
    { 0b0001101011000000001000, 0b0101001100000000000000, 0 }, // lsl
    { 0b0001101011000000001000, 0b0000000000000000000000, 0 }, // lslv
    { 0b0001101011000000001001, 0b0101001100000000011111, 0 }, // lsr
    { 0b0001101011000000001001, 0b0000000000000000000000, 0 }, // lsrv
    { 0b0001101011000000001011, 0b0001001110000000000000, 1 }, // ror
    { 0b0001101011000000001011, 0b0000000000000000000000, 1 }  // rorv
});

pub const BASE_STX: [BaseStx; 3] = table_new!(BaseStx, {
    { 0b1000100000000000011111, kWX, 30 }, // stxr
    { 0b0000100000000000011111, kW , 0  }, // stxrb
    { 0b0100100000000000011111, kW , 0  }  // stxrh
});

pub const BASE_STXP: [BaseStxp; 2] = table_new!(BaseStxp, {
    { 0b1000100000100000100000, kWX, 30 }, // stlxp
    { 0b1000100000100000000000, kWX, 30 }  // stxp
});

pub const BASE_TST: [BaseTst; 1] = table_new!(BaseTst, {
    { 0b1101010000, 0b111001000 }  // tst
});

pub const F_SIMD_PAIR: [FSimdPair; 5] = table_new!(FSimdPair, {
    { 0b0111111000110000110110, 0b0010111000100000110101 }, // faddp_v
    { 0b0111111000110000110010, 0b0010111000100000110001 }, // fmaxnmp_v
    { 0b0111111000110000111110, 0b0010111000100000111101 }, // fmaxp_v
    { 0b0111111010110000110010, 0b0010111010100000110001 }, // fminnmp_v
    { 0b0111111010110000111110, 0b0010111010100000111101 }  // fminp_v
});

pub const F_SIMD_SV: [FSimdSV; 4] = table_new!(FSimdSV, {
    { 0b0010111000110000110010 }, // fmaxnmv_v
    { 0b0010111000110000111110 }, // fmaxv_v
    { 0b0010111010110000110010 }, // fminnmv_v
    { 0b0010111010110000111110 }  // fminv_v
});

pub const F_SIMD_VV: [FSimdVV; 17] = table_new!(FSimdVV, {
    { 0b0001111000100000110000, kHF_A, 0b0000111010100000111110, kHF_B }, // fabs_v
    { 0b0001111000100001010000, kHF_A, 0b0010111010100000111110, kHF_B }, // fneg_v
    { 0b0101111010100001110110, kHF_B, 0b0000111010100001110110, kHF_B }, // frecpe_v
    { 0b0101111010100001111110, kHF_B, 0b0000000000000000000000, kHF_N }, // frecpx_v
    { 0b0001111000101000110000, kHF_N, 0b0010111000100001111010, kHF_N }, // frint32x_v
    { 0b0001111000101000010000, kHF_N, 0b0000111000100001111010, kHF_N }, // frint32z_v
    { 0b0001111000101001110000, kHF_N, 0b0010111000100001111110, kHF_N }, // frint64x_v
    { 0b0001111000101001010000, kHF_N, 0b0000111000100001111110, kHF_N }, // frint64z_v
    { 0b0001111000100110010000, kHF_A, 0b0010111000100001100010, kHF_B }, // frinta_v
    { 0b0001111000100111110000, kHF_A, 0b0010111010100001100110, kHF_B }, // frinti_v
    { 0b0001111000100101010000, kHF_A, 0b0000111000100001100110, kHF_B }, // frintm_v
    { 0b0001111000100100010000, kHF_A, 0b0000111000100001100010, kHF_B }, // frintn_v
    { 0b0001111000100100110000, kHF_A, 0b0000111010100001100010, kHF_B }, // frintp_v
    { 0b0001111000100111010000, kHF_A, 0b0010111000100001100110, kHF_B }, // frintx_v
    { 0b0001111000100101110000, kHF_A, 0b0000111010100001100110, kHF_B }, // frintz_v
    { 0b0111111010100001110110, kHF_B, 0b0010111010100001110110, kHF_B }, // frsqrte_v
    { 0b0001111000100001110000, kHF_A, 0b0010111010100001111110, kHF_B }  // fsqrt_v
});

pub const F_SIMD_VVV: [FSimdVVV; 13] = table_new!(FSimdVVV, {
    { 0b0111111010100000110101, kHF_C, 0b0010111010100000110101, kHF_C }, // fabd_v
    { 0b0111111000100000111011, kHF_C, 0b0010111000100000111011, kHF_C }, // facge_v
    { 0b0111111010100000111011, kHF_C, 0b0010111010100000111011, kHF_C }, // facgt_v
    { 0b0001111000100000001010, kHF_A, 0b0000111000100000110101, kHF_C }, // fadd_v
    { 0b0001111000100000000110, kHF_A, 0b0010111000100000111111, kHF_C }, // fdiv_v
    { 0b0001111000100000010010, kHF_A, 0b0000111000100000111101, kHF_C }, // fmax_v
    { 0b0001111000100000011010, kHF_A, 0b0000111000100000110001, kHF_C }, // fmaxnm_v
    { 0b0001111000100000010110, kHF_A, 0b0000111010100000111101, kHF_C }, // fmin_v
    { 0b0001111000100000011110, kHF_A, 0b0000111010100000110001, kHF_C }, // fminnm_v
    { 0b0001111000100000100010, kHF_A, 0b0000000000000000000000, kHF_N }, // fnmul_v
    { 0b0101111000100000111111, kHF_C, 0b0000111000100000111111, kHF_C }, // frecps_v
    { 0b0101111010100000111111, kHF_C, 0b0000111010100000111111, kHF_C }, // frsqrts_v
    { 0b0001111000100000001110, kHF_A, 0b0000111010100000110101, kHF_C }  // fsub_v
});

pub const F_SIMD_VVVV: [FSimdVVVV; 4] = table_new!(FSimdVVVV, {
    { 0b0001111100000000000000, kHF_A, 0b0000000000000000000000, kHF_N }, // fmadd_v
    { 0b0001111100000000100000, kHF_A, 0b0000000000000000000000, kHF_N }, // fmsub_v
    { 0b0001111100100000000000, kHF_A, 0b0000000000000000000000, kHF_N }, // fnmadd_v
    { 0b0001111100100000100000, kHF_A, 0b0000000000000000000000, kHF_N }  // fnmsub_v
});

pub const F_SIMD_VVVE: [FSimdVVVe; 4] = table_new!(FSimdVVVe, {
    { 0b0000000000000000000000, kHF_N, 0b0000111000100000110011, 0b0000111110000000000100 }, // fmla_v
    { 0b0000000000000000000000, kHF_N, 0b0000111010100000110011, 0b0000111110000000010100 }, // fmls_v
    { 0b0001111000100000000010, kHF_A, 0b0010111000100000110111, 0b0000111110000000100100 }, // fmul_v
    { 0b0101111000100000110111, kHF_C, 0b0000111000100000110111, 0b0010111110000000100100 }  // fmulx_v
});

pub const I_SIMD_PAIR: [ISimdPair; 1] = table_new!(ISimdPair, {
    { 0b0101111000110001101110, 0b0000111000100000101111, kVO_V_Any }  // addp_v
});

pub const I_SIMD_SV: [ISimdSV; 7] = table_new!(ISimdSV, {
    { 0b0000111000110001101110, kVO_V_BH_4S }, // addv_v
    { 0b0000111000110000001110, kVO_V_BH_4S }, // saddlv_v
    { 0b0000111000110000101010, kVO_V_BH_4S }, // smaxv_v
    { 0b0000111000110001101010, kVO_V_BH_4S }, // sminv_v
    { 0b0010111000110000001110, kVO_V_BH_4S }, // uaddlv_v
    { 0b0010111000110000101010, kVO_V_BH_4S }, // umaxv_v
    { 0b0010111000110001101010, kVO_V_BH_4S }  // uminv_v
});

pub const I_SIMD_VV: [ISimdVV; 29] = table_new!(ISimdVV, {
    { 0b0000111000100000101110, kVO_V_Any }, // abs_v
    { 0b0000111000100000010010, kVO_V_BHS }, // cls_v
    { 0b0010111000100000010010, kVO_V_BHS }, // clz_v
    { 0b0000111000100000010110, kVO_V_B }, // cnt_v
    { 0b0010111000100000010110, kVO_V_B }, // mvn_v
    { 0b0010111000100000101110, kVO_V_Any }, // neg_v
    { 0b0010111000100000010110, kVO_V_B }, // not_v
    { 0b0010111001100000010110, kVO_V_B }, // rbit_v
    { 0b0000111000100000000110, kVO_V_B }, // rev16_v
    { 0b0010111000100000000010, kVO_V_BH }, // rev32_v
    { 0b0000111000100000000010, kVO_V_BHS }, // rev64_v
    { 0b0000111000100000011010, kVO_V_BHS }, // sadalp_v
    { 0b0000111000100000001010, kVO_V_BHS }, // saddlp_v
    { 0b0000111000100000011110, kVO_SV_Any }, // sqabs_v
    { 0b0010111000100000011110, kVO_SV_Any }, // sqneg_v
    { 0b0000111000100001010010, kVO_SV_B8H4S2 }, // sqxtn_v
    { 0b0100111000100001010010, kVO_V_B16H8S4 }, // sqxtn2_v
    { 0b0010111000100001001010, kVO_SV_B8H4S2 }, // sqxtun_v
    { 0b0110111000100001001010, kVO_V_B16H8S4 }, // sqxtun2_v
    { 0b0000111000100000001110, kVO_SV_Any }, // suqadd_v
    { 0b0010111000100000011010, kVO_V_BHS }, // uadalp_v
    { 0b0010111000100000001010, kVO_V_BHS }, // uaddlp_v
    { 0b0010111000100001010010, kVO_SV_B8H4S2 }, // uqxtn_v
    { 0b0110111000100001010010, kVO_V_B16H8S4 }, // uqxtn2_v
    { 0b0000111010100001110010, kVO_V_S }, // urecpe_v
    { 0b0010111010100001110010, kVO_V_S }, // ursqrte_v
    { 0b0010111000100000001110, kVO_SV_Any }, // usqadd_v
    { 0b0000111000100001001010, kVO_V_B8H4S2 }, // xtn_v
    { 0b0100111000100001001010, kVO_V_B16H8S4 }  // xtn2_v
});

pub const I_SIMD_VVV: [ISimdVVV; 65] = table_new!(ISimdVVV, {
    { 0b0000111000100000100001, kVO_V_Any }, // add_v
    { 0b0000111000100000010000, kVO_V_B8H4S2 }, // addhn_v
    { 0b0100111000100000010000, kVO_V_B16H8S4 }, // addhn2_v
    { 0b0000111000100000000111, kVO_V_B }, // and_v
    { 0b0010111011100000000111, kVO_V_B }, // bif_v
    { 0b0010111010100000000111, kVO_V_B }, // bit_v
    { 0b0010111001100000000111, kVO_V_B }, // bsl_v
    { 0b0000111000100000100011, kVO_V_Any }, // cmtst_v
    { 0b0010111000100000000111, kVO_V_B }, // eor_v
    { 0b0000111011100000000111, kVO_V_B }, // orn_v
    { 0b0010111000100000100111, kVO_V_B }, // pmul_v
    { 0b0000111000100000111000, kVO_V_B8D1 }, // pmull_v
    { 0b0100111000100000111000, kVO_V_B16D2 }, // pmull2_v
    { 0b0010111000100000010000, kVO_V_B8H4S2 }, // raddhn_v
    { 0b0110111000100000010000, kVO_V_B16H8S4 }, // raddhn2_v
    { 0b1100111001100000100011, kVO_V_D2 }, // rax1_v
    { 0b0010111000100000011000, kVO_V_B8H4S2 }, // rsubhn_v
    { 0b0110111000100000011000, kVO_V_B16H8S4 }, // rsubhn2_v
    { 0b0000111000100000011111, kVO_V_BHS }, // saba_v
    { 0b0000111000100000010100, kVO_V_B8H4S2 }, // sabal_v
    { 0b0100111000100000010100, kVO_V_B16H8S4 }, // sabal2_v
    { 0b0000111000100000011101, kVO_V_BHS }, // sabd_v
    { 0b0000111000100000011100, kVO_V_B8H4S2 }, // sabdl_v
    { 0b0100111000100000011100, kVO_V_B16H8S4 }, // sabdl2_v
    { 0b0000111000100000000000, kVO_V_B8H4S2 }, // saddl_v
    { 0b0100111000100000000000, kVO_V_B16H8S4 }, // saddl2_v
    { 0b0000111000100000000001, kVO_V_BHS }, // shadd_v
    { 0b0000111000100000001001, kVO_V_BHS }, // shsub_v
    { 0b0000111000100000011001, kVO_V_BHS }, // smax_v
    { 0b0000111000100000101001, kVO_V_BHS }, // smaxp_v
    { 0b0000111000100000011011, kVO_V_BHS }, // smin_v
    { 0b0000111000100000101011, kVO_V_BHS }, // sminp_v
    { 0b0000111000100000000011, kVO_SV_Any }, // sqadd_v
    { 0b0000111000100000001011, kVO_SV_Any }, // sqsub_v
    { 0b0000111000100000000101, kVO_V_BHS }, // srhadd_v
    { 0b0000111000100000001000, kVO_V_B8H4S2 }, // ssubl_v
    { 0b0100111000100000001000, kVO_V_B16H8S4 }, // ssubl2_v
    { 0b0010111000100000100001, kVO_V_Any }, // sub_v
    { 0b0000111000100000011000, kVO_V_B8H4S2 }, // subhn_v
    { 0b0000111000100000011000, kVO_V_B16H8S4 }, // subhn2_v
    { 0b0000111000000000001010, kVO_V_BHS_D2 }, // trn1_v
    { 0b0000111000000000011010, kVO_V_BHS_D2 }, // trn2_v
    { 0b0010111000100000011111, kVO_V_BHS }, // uaba_v
    { 0b0010111000100000010100, kVO_V_B8H4S2 }, // uabal_v
    { 0b0110111000100000010100, kVO_V_B16H8S4 }, // uabal2_v
    { 0b0010111000100000011101, kVO_V_BHS }, // uabd_v
    { 0b0010111000100000011100, kVO_V_B8H4S2 }, // uabdl_v
    { 0b0110111000100000011100, kVO_V_B16H8S4 }, // uabdl2_v
    { 0b0010111000100000000000, kVO_V_B8H4S2 }, // uaddl_v
    { 0b0110111000100000000000, kVO_V_B16H8S4 }, // uaddl2_v
    { 0b0010111000100000000001, kVO_V_BHS }, // uhadd_v
    { 0b0010111000100000001001, kVO_V_BHS }, // uhsub_v
    { 0b0010111000100000011001, kVO_V_BHS }, // umax_v
    { 0b0010111000100000101001, kVO_V_BHS }, // umaxp_v
    { 0b0010111000100000011011, kVO_V_BHS }, // umin_v
    { 0b0010111000100000101011, kVO_V_BHS }, // uminp_v
    { 0b0010111000100000000011, kVO_SV_Any }, // uqadd_v
    { 0b0010111000100000001011, kVO_SV_Any }, // uqsub_v
    { 0b0010111000100000000101, kVO_V_BHS }, // urhadd_v
    { 0b0010111000100000001000, kVO_V_B8H4S2 }, // usubl_v
    { 0b0110111000100000001000, kVO_V_B16H8S4 }, // usubl2_v
    { 0b0000111000000000000110, kVO_V_BHS_D2 }, // uzp1_v
    { 0b0000111000000000010110, kVO_V_BHS_D2 }, // uzp2_v
    { 0b0000111000000000001110, kVO_V_BHS_D2 }, // zip1_v
    { 0b0000111000000000011110, kVO_V_BHS_D2 }  // zip2_v
});

pub const I_SIMD_VVVI: [ISimdVVVI; 2] = table_new!(ISimdVVVI, {
    { 0b0010111000000000000000, kVO_V_B, 4, 11, 1 }, // ext_v
    { 0b1100111001100000100011, kVO_V_D2, 6, 10, 0 }  // xar_v
});

pub const I_SIMD_VVVV: [ISimdVVVV; 2] = table_new!(ISimdVVVV, {
    { 0b1100111000100000000000, kVO_V_B16 }, // bcax_v
    { 0b1100111000000000000000, kVO_V_B16 }  // eor3_v
});

pub const I_SIMD_VVVVX: [ISimdVVVVx; 1] = table_new!(ISimdVVVVx, {
    { 0b1100111001000000000000, kOp_V4S, kOp_V4S, kOp_V4S, kOp_V4S }  // sm3ss1_v
});

pub const I_SIMD_VVVE: [ISimdVVVe; 25] = table_new!(ISimdVVVe, {
    { 0b0000111000100000100101, kVO_V_BHS, 0b0010111100000000000000, kVO_V_HS }, // mla_v
    { 0b0010111000100000100101, kVO_V_BHS, 0b0010111100000000010000, kVO_V_HS }, // mls_v
    { 0b0000111000100000100111, kVO_V_BHS, 0b0000111100000000100000, kVO_V_HS }, // mul_v
    { 0b0000111000100000100000, kVO_V_B8H4S2, 0b0000111100000000001000, kVO_V_H4S2 }, // smlal_v
    { 0b0100111000100000100000, kVO_V_B16H8S4, 0b0100111100000000001000, kVO_V_H8S4 }, // smlal2_v
    { 0b0000111000100000101000, kVO_V_B8H4S2, 0b0000111100000000011000, kVO_V_H4S2 }, // smlsl_v
    { 0b0100111000100000101000, kVO_V_B16H8S4, 0b0100111100000000011000, kVO_V_H8S4 }, // smlsl2_v
    { 0b0000111000100000110000, kVO_V_B8H4S2, 0b0000111100000000101000, kVO_V_H4S2 }, // smull_v
    { 0b0100111000100000110000, kVO_V_B16H8S4, 0b0100111100000000101000, kVO_V_H8S4 }, // smull2_v
    { 0b0000111000100000100100, kVO_SV_BHS, 0b0000111100000000001100, kVO_V_H4S2 }, // sqdmlal_v
    { 0b0100111000100000100100, kVO_V_B16H8S4, 0b0100111100000000001100, kVO_V_H8S4 }, // sqdmlal2_v
    { 0b0000111000100000101100, kVO_SV_BHS, 0b0000111100000000011100, kVO_V_H4S2 }, // sqdmlsl_v
    { 0b0100111000100000101100, kVO_V_B16H8S4, 0b0100111100000000011100, kVO_V_H8S4 }, // sqdmlsl2_v
    { 0b0000111000100000101101, kVO_SV_HS, 0b0000111100000000110000, kVO_SV_HS }, // sqdmulh_v
    { 0b0000111000100000110100, kVO_SV_BHS, 0b0000111100000000101100, kVO_V_H4S2 }, // sqdmull_v
    { 0b0100111000100000110100, kVO_V_B16H8S4, 0b0100111100000000101100, kVO_V_H8S4 }, // sqdmull2_v
    { 0b0010111000000000100001, kVO_SV_HS, 0b0010111100000000110100, kVO_SV_HS }, // sqrdmlah_v
    { 0b0010111000000000100011, kVO_SV_HS, 0b0010111100000000111100, kVO_SV_HS }, // sqrdmlsh_v
    { 0b0010111000100000101101, kVO_SV_HS, 0b0000111100000000110100, kVO_SV_HS }, // sqrdmulh_v
    { 0b0010111000100000100000, kVO_V_B8H4S2, 0b0010111100000000001000, kVO_V_H4S2 }, // umlal_v
    { 0b0110111000100000100000, kVO_V_B16H8S4, 0b0010111100000000001000, kVO_V_H8S4 }, // umlal2_v
    { 0b0010111000100000101000, kVO_V_B8H4S2, 0b0010111100000000011000, kVO_V_H4S2 }, // umlsl_v
    { 0b0110111000100000101000, kVO_V_B16H8S4, 0b0110111100000000011000, kVO_V_H8S4 }, // umlsl2_v
    { 0b0010111000100000110000, kVO_V_B8H4S2, 0b0010111100000000101000, kVO_V_H4S2 }, // umull_v
    { 0b0110111000100000110000, kVO_V_B16H8S4, 0b0110111100000000101000, kVO_V_H8S4 }  // umull2_v
});

pub const I_SIMD_VVVX: [ISimdVVVx; 17] = table_new!(ISimdVVVx, {
    { 0b0110111001000000111011, kOp_V4S, kOp_V8H, kOp_V8H }, // bfmmla_v
    { 0b0101111000000000000000, kOp_Q, kOp_S, kOp_V4S }, // sha1c_v
    { 0b0101111000000000001000, kOp_Q, kOp_S, kOp_V4S }, // sha1m_v
    { 0b0101111000000000000100, kOp_Q, kOp_S, kOp_V4S }, // sha1p_v
    { 0b0101111000000000001100, kOp_V4S, kOp_V4S, kOp_V4S }, // sha1su0_v
    { 0b0101111000000000010000, kOp_Q, kOp_Q, kOp_V4S }, // sha256h_v
    { 0b0101111000000000010100, kOp_Q, kOp_Q, kOp_V4S }, // sha256h2_v
    { 0b0101111000000000011000, kOp_V4S, kOp_V4S, kOp_V4S }, // sha256su1_v
    { 0b1100111001100000100000, kOp_Q, kOp_Q, kOp_V2D }, // sha512h_v
    { 0b1100111001100000100001, kOp_Q, kOp_Q, kOp_V2D }, // sha512h2_v
    { 0b1100111001100000100010, kOp_V2D, kOp_V2D, kOp_V2D }, // sha512su1_v
    { 0b1100111001100000110000, kOp_V4S, kOp_V4S, kOp_V4S }, // sm3partw1_v
    { 0b1100111001100000110001, kOp_V4S, kOp_V4S, kOp_V4S }, // sm3partw2_v
    { 0b1100111001100000110010, kOp_V4S, kOp_V4S, kOp_V4S }, // sm4ekey_v
    { 0b0100111010000000101001, kOp_V4S, kOp_V16B, kOp_V16B }, // smmla_v
    { 0b0110111010000000101001, kOp_V4S, kOp_V16B, kOp_V16B }, // ummla_v
    { 0b0100111010000000101011, kOp_V4S, kOp_V16B, kOp_V16B }  // usmmla_v
});

pub const I_SIMD_VVX: [ISimdVVx; 13] = table_new!(ISimdVVx, {
    { 0b0100111000101000010110, kOp_V16B, kOp_V16B }, // aesd_v
    { 0b0100111000101000010010, kOp_V16B, kOp_V16B }, // aese_v
    { 0b0100111000101000011110, kOp_V16B, kOp_V16B }, // aesimc_v
    { 0b0100111000101000011010, kOp_V16B, kOp_V16B }, // aesmc_v
    { 0b0001111001100011010000, kOp_H, kOp_S }, // bfcvt_v
    { 0b0000111010100001011010, kOp_V4H, kOp_V4S }, // bfcvtn_v
    { 0b0100111010100001011010, kOp_V8H, kOp_V4S }, // bfcvtn2_v
    { 0b0001111001111110000000, kOp_GpW, kOp_D }, // fjcvtzs_v
    { 0b0101111000101000000010, kOp_S, kOp_S }, // sha1h_v
    { 0b0101111000101000000110, kOp_V4S, kOp_V4S }, // sha1su1_v
    { 0b0101111000101000001010, kOp_V4S, kOp_V4S }, // sha256su0_v
    { 0b1100111011000000100000, kOp_V2D, kOp_V2D }, // sha512su0_v
    { 0b1100111011000000100001, kOp_V4S, kOp_V4S }  // sm4e_v
});

pub const I_SIMD_WWV: [ISimdWWV; 8] = table_new!(ISimdWWV, {
    { 0b0000111000100000000100, kVO_V_B8H4S2 }, // saddw_v
    { 0b0000111000100000000100, kVO_V_B16H8S4 }, // saddw2_v
    { 0b0000111000100000001100, kVO_V_B8H4S2 }, // ssubw_v
    { 0b0000111000100000001100, kVO_V_B16H8S4 }, // ssubw2_v
    { 0b0010111000100000000100, kVO_V_B8H4S2 }, // uaddw_v
    { 0b0010111000100000000100, kVO_V_B16H8S4 }, // uaddw2_v
    { 0b0010111000100000001100, kVO_V_B8H4S2 }, // usubw_v
    { 0b0010111000100000001100, kVO_V_B16H8S4 }  // usubw2_v
});

pub const SIMD_BIC_ORR: [SimdBicOrr; 2] = table_new!(SimdBicOrr, {
    { 0b0000111001100000000111, 0b0010111100000000000001 }, // bic_v
    { 0b0000111010100000000111, 0b0000111100000000000001 }  // orr_v
});

pub const SIMD_CMP: [SimdCmp; 7] = table_new!(SimdCmp, {
    { 0b0010111000100000100011, 0b0000111000100000100110, kVO_V_Any }, // cmeq_v
    { 0b0000111000100000001111, 0b0010111000100000100010, kVO_V_Any }, // cmge_v
    { 0b0000111000100000001101, 0b0000111000100000100010, kVO_V_Any }, // cmgt_v
    { 0b0010111000100000001101, 0b0000000000000000000000, kVO_V_Any }, // cmhi_v
    { 0b0010111000100000001111, 0b0000000000000000000000, kVO_V_Any }, // cmhs_v
    { 0b0000000000000000000000, 0b0010111000100000100110, kVO_V_Any }, // cmle_v
    { 0b0000000000000000000000, 0b0000111000100000101010, kVO_V_Any }  // cmlt_v
});

pub const SIMD_DOT: [SimdDot; 5] = table_new!(SimdDot, {
    { 0b0010111001000000111111, 0b0000111101000000111100, kET_S, kET_H, kET_2H }, // bfdot_v
    { 0b0000111010000000100101, 0b0000111110000000111000, kET_S, kET_B, kET_4B }, // sdot_v
    { 0b0000000000000000000000, 0b0000111100000000111100, kET_S, kET_B, kET_4B }, // sudot_v
    { 0b0010111010000000100101, 0b0010111110000000111000, kET_S, kET_B, kET_4B }, // udot_v
    { 0b0000111010000000100111, 0b0000111110000000111100, kET_S, kET_B, kET_4B }  // usdot_v
});

pub const SIMD_FCADD: [SimdFcadd; 1] = table_new!(SimdFcadd, {
    { 0b0010111000000000111001 } // fcadd_v
});

pub const SIMD_FCCMP_FCCMPE: [SimdFccmpFccmpe; 2] = table_new!(SimdFccmpFccmpe, {
    { 0b00011110001000000000010000000000 }, // fccmp_v
    { 0b00011110001000000000010000010000 }  // fccmpe_v
});

pub const SIMD_FCM: [SimdFcm; 5] = table_new!(SimdFcm, {
    { 0b0000111000100000111001, kHF_C, 0b0000111010100000110110 }, // fcmeq_v
    { 0b0010111000100000111001, kHF_C, 0b0010111010100000110010 }, // fcmge_v
    { 0b0010111010100000111001, kHF_C, 0b0000111010100000110010 }, // fcmgt_v
    { 0b0000000000000000000000, kHF_C, 0b0010111010100000110110 }, // fcmle_v
    { 0b0000000000000000000000, kHF_C, 0b0000111010100000111010 }  // fcmlt_v
});

pub const SIMD_FCMLA: [SimdFcmla; 1] = table_new!(SimdFcmla, {
    { 0b0010111000000000110001, 0b0010111100000000000100 }  // fcmla_v
});

pub const SIMD_FCMP_FCMPE: [SimdFcmpFcmpe; 2] = table_new!(SimdFcmpFcmpe, {
    { 0b00011110001000000010000000000000 }, // fcmp_v
    { 0b00011110001000000010000000010000 }  // fcmpe_v
});

pub const SIMD_FCVT_LN: [SimdFcvtLN; 6] = table_new!(SimdFcvtLN, {
    { 0b0000111000100001011110, 0, 0 }, // fcvtl_v
    { 0b0100111000100001011110, 0, 0 }, // fcvtl2_v
    { 0b0000111000100001011010, 0, 0 }, // fcvtn_v
    { 0b0100111000100001011010, 0, 0 }, // fcvtn2_v
    { 0b0010111000100001011010, 1, 1 }, // fcvtxn_v
    { 0b0110111000100001011010, 1, 0 }  // fcvtxn2_v
});

pub const SIMD_FCVT_SV: [SimdFcvtSV; 12] = table_new!(SimdFcvtSV, {
    { 0b0000111000100001110010, 0b0000000000000000000000, 0b0001111000100100000000, 1 }, // fcvtas_v
    { 0b0010111000100001110010, 0b0000000000000000000000, 0b0001111000100101000000, 1 }, // fcvtau_v
    { 0b0000111000100001101110, 0b0000000000000000000000, 0b0001111000110000000000, 1 }, // fcvtms_v
    { 0b0010111000100001101110, 0b0000000000000000000000, 0b0001111000110001000000, 1 }, // fcvtmu_v
    { 0b0000111000100001101010, 0b0000000000000000000000, 0b0001111000100000000000, 1 }, // fcvtns_v
    { 0b0010111000100001101010, 0b0000000000000000000000, 0b0001111000100001000000, 1 }, // fcvtnu_v
    { 0b0000111010100001101010, 0b0000000000000000000000, 0b0001111000101000000000, 1 }, // fcvtps_v
    { 0b0010111010100001101010, 0b0000000000000000000000, 0b0001111000101001000000, 1 }, // fcvtpu_v
    { 0b0000111010100001101110, 0b0000111100000000111111, 0b0001111000111000000000, 1 }, // fcvtzs_v
    { 0b0010111010100001101110, 0b0010111100000000111111, 0b0001111000111001000000, 1 }, // fcvtzu_v
    { 0b0000111000100001110110, 0b0000111100000000111001, 0b0001111000100010000000, 0 }, // scvtf_v
    { 0b0010111000100001110110, 0b0010111100000000111001, 0b0001111000100011000000, 0 }  // ucvtf_v
});

pub const SIMD_FMLAL: [SimdFmlal; 6] = table_new!(SimdFmlal, {
    { 0b0010111011000000111111, 0b0000111111000000111100, 0, kET_S, kET_H, kET_H }, // bfmlalb_v
    { 0b0110111011000000111111, 0b0100111111000000111100, 0, kET_S, kET_H, kET_H }, // bfmlalt_v
    { 0b0000111000100000111011, 0b0000111110000000000000, 1, kET_S, kET_H, kET_H }, // fmlal_v
    { 0b0010111000100000110011, 0b0010111110000000100000, 1, kET_S, kET_H, kET_H }, // fmlal2_v
    { 0b0000111010100000111011, 0b0000111110000000010000, 1, kET_S, kET_H, kET_H }, // fmlsl_v
    { 0b0010111010100000110011, 0b0010111110000000110000, 1, kET_S, kET_H, kET_H }  // fmlsl2_v
});

pub const SIMD_LD_N_ST_N: [SimdLdNStN; 12] = table_new!(SimdLdNStN, {
    { 0b0000110101000000000000, 0b0000110001000000001000, 1, 0 }, // ld1_v
    { 0b0000110101000000110000, 0b0000000000000000000000, 1, 1 }, // ld1r_v
    { 0b0000110101100000000000, 0b0000110001000000100000, 2, 0 }, // ld2_v
    { 0b0000110101100000110000, 0b0000000000000000000000, 2, 1 }, // ld2r_v
    { 0b0000110101000000001000, 0b0000110001000000010000, 3, 0 }, // ld3_v
    { 0b0000110101000000111000, 0b0000000000000000000000, 3, 1 }, // ld3r_v
    { 0b0000110101100000001000, 0b0000110001000000000000, 4, 0 }, // ld4_v
    { 0b0000110101100000111000, 0b0000000000000000000000, 4, 1 }, // ld4r_v
    { 0b0000110100000000000000, 0b0000110000000000001000, 1, 0 }, // st1_v
    { 0b0000110100100000000000, 0b0000110000000000100000, 2, 0 }, // st2_v
    { 0b0000110100000000001000, 0b0000110000000000010000, 3, 0 }, // st3_v
    { 0b0000110100100000001000, 0b0000110000000000000000, 4, 0 }  // st4_v
});

pub const SIMD_LD_ST: [SimdLdSt; 2] = table_new!(SimdLdSt, {
    { 0b0011110101, 0b00111100010, 0b00111100011, 0b00011100, InstId::Ldur_v }, // ldr_v
    { 0b0011110100, 0b00111100000, 0b00111100001, 0b00000000, InstId::Stur_v }  // str_v
});

pub const SIMD_LDP_STP: [SimdLdpStp; 4] = table_new!(SimdLdpStp, {
    { 0b0010110001, 0b0000000000 }, // ldnp_v
    { 0b0010110101, 0b0010110011 }, // ldp_v
    { 0b0010110000, 0b0000000000 }, // stnp_v
    { 0b0010110100, 0b0010110010 }  // stp_v
});

pub const SIMD_LDUR_STUR: [SimdLdurStur; 2] = table_new!(SimdLdurStur, {
    { 0b0011110001000000000000 }, // ldur_v
    { 0b0011110000000000000000 }  // stur_v
});

pub const SIMD_MOVI_MVNI: [SimdMoviMvni; 2] = table_new!(SimdMoviMvni, {
    { 0b0000111100000000000001, 0 }, // movi_v
    { 0b0000111100000000000001, 1 }  // mvni_v
});

pub const SIMD_SHIFT: [SimdShift; 40] = table_new!(SimdShift, {
    { 0b0000000000000000000000, 0b0000111100000000100011, 1, kVO_V_B8H4S2 }, // rshrn_v
    { 0b0000000000000000000000, 0b0100111100000000100011, 1, kVO_V_B16H8S4 }, // rshrn2_v
    { 0b0000000000000000000000, 0b0000111100000000010101, 0, kVO_V_Any }, // shl_v
    { 0b0000000000000000000000, 0b0000111100000000100001, 1, kVO_V_B8H4S2 }, // shrn_v
    { 0b0000000000000000000000, 0b0100111100000000100001, 1, kVO_V_B16H8S4 }, // shrn2_v
    { 0b0000000000000000000000, 0b0010111100000000010101, 0, kVO_V_Any }, // sli_v
    { 0b0000111000100000010111, 0b0000000000000000000000, 1, kVO_SV_Any }, // sqrshl_v
    { 0b0000000000000000000000, 0b0000111100000000100111, 1, kVO_SV_B8H4S2 }, // sqrshrn_v
    { 0b0000000000000000000000, 0b0100111100000000100111, 1, kVO_V_B16H8S4 }, // sqrshrn2_v
    { 0b0000000000000000000000, 0b0010111100000000100011, 1, kVO_SV_B8H4S2 }, // sqrshrun_v
    { 0b0000000000000000000000, 0b0110111100000000100011, 1, kVO_V_B16H8S4 }, // sqrshrun2_v
    { 0b0000111000100000010011, 0b0000111100000000011101, 0, kVO_SV_Any }, // sqshl_v
    { 0b0000000000000000000000, 0b0010111100000000011001, 0, kVO_SV_Any }, // sqshlu_v
    { 0b0000000000000000000000, 0b0000111100000000100101, 1, kVO_SV_B8H4S2 }, // sqshrn_v
    { 0b0000000000000000000000, 0b0100111100000000100101, 1, kVO_V_B16H8S4 }, // sqshrn2_v
    { 0b0000000000000000000000, 0b0010111100000000100001, 1, kVO_SV_B8H4S2 }, // sqshrun_v
    { 0b0000000000000000000000, 0b0110111100000000100001, 1, kVO_V_B16H8S4 }, // sqshrun2_v
    { 0b0000000000000000000000, 0b0010111100000000010001, 1, kVO_V_Any }, // sri_v
    { 0b0000111000100000010101, 0b0000000000000000000000, 0, kVO_V_Any }, // srshl_v
    { 0b0000000000000000000000, 0b0000111100000000001001, 1, kVO_V_Any }, // srshr_v
    { 0b0000000000000000000000, 0b0000111100000000001101, 1, kVO_V_Any }, // srsra_v
    { 0b0000111000100000010001, 0b0000000000000000000000, 0, kVO_V_Any }, // sshl_v
    { 0b0000000000000000000000, 0b0000111100000000101001, 0, kVO_V_B8H4S2 }, // sshll_v
    { 0b0000000000000000000000, 0b0100111100000000101001, 0, kVO_V_B16H8S4 }, // sshll2_v
    { 0b0000000000000000000000, 0b0000111100000000000001, 1, kVO_V_Any }, // sshr_v
    { 0b0000000000000000000000, 0b0000111100000000000101, 1, kVO_V_Any }, // ssra_v
    { 0b0010111000100000010111, 0b0000000000000000000000, 0, kVO_SV_Any }, // uqrshl_v
    { 0b0000000000000000000000, 0b0010111100000000100111, 1, kVO_SV_B8H4S2 }, // uqrshrn_v
    { 0b0000000000000000000000, 0b0110111100000000100111, 1, kVO_V_B16H8S4 }, // uqrshrn2_v
    { 0b0010111000100000010011, 0b0010111100000000011101, 0, kVO_SV_Any }, // uqshl_v
    { 0b0000000000000000000000, 0b0010111100000000100101, 1, kVO_SV_B8H4S2 }, // uqshrn_v
    { 0b0000000000000000000000, 0b0110111100000000100101, 1, kVO_V_B16H8S4 }, // uqshrn2_v
    { 0b0010111000100000010101, 0b0000000000000000000000, 0, kVO_V_Any }, // urshl_v
    { 0b0000000000000000000000, 0b0010111100000000001001, 1, kVO_V_Any }, // urshr_v
    { 0b0000000000000000000000, 0b0010111100000000001101, 1, kVO_V_Any }, // ursra_v
    { 0b0010111000100000010001, 0b0000000000000000000000, 0, kVO_V_Any }, // ushl_v
    { 0b0000000000000000000000, 0b0010111100000000101001, 0, kVO_V_B8H4S2 }, // ushll_v
    { 0b0000000000000000000000, 0b0110111100000000101001, 0, kVO_V_B16H8S4 }, // ushll2_v
    { 0b0000000000000000000000, 0b0010111100000000000001, 1, kVO_V_Any }, // ushr_v
    { 0b0000000000000000000000, 0b0010111100000000000101, 1, kVO_V_Any }  // usra_v
});

pub const SIMD_SHIFT_ES: [SimdShiftES; 2] = table_new!(SimdShiftES, {
    { 0b0010111000100001001110, kVO_V_B8H4S2 }, // shll_v
    { 0b0110111000100001001110, kVO_V_B16H8S4 }  // shll2_v
});

pub const SIMD_SM3TT: [SimdSm3tt; 4] = table_new!(SimdSm3tt, {
    { 0b1100111001000000100000 }, // sm3tt1a_v
    { 0b1100111001000000100001 }, // sm3tt1b_v
    { 0b1100111001000000100010 }, // sm3tt2a_v
    { 0b1100111001000000100011 }  // sm3tt2b_v
});

pub const SIMD_SMOV_UMOV: [SimdSmovUmov; 2] = table_new!(SimdSmovUmov, {
    { 0b0000111000000000001011, kVO_V_BHS, 1 }, // smov_v
    { 0b0000111000000000001111, kVO_V_Any, 0 }  // umov_v
});

pub const SIMD_SXTL_UXTL: [SimdSxtlUxtl; 4] = table_new!(SimdSxtlUxtl, {
    { 0b0000111100000000101001, kVO_V_B8H4S2 }, // sxtl_v
    { 0b0100111100000000101001, kVO_V_B16H8S4 }, // sxtl2_v
    { 0b0010111100000000101001, kVO_V_B8H4S2 }, // uxtl_v
    { 0b0110111100000000101001, kVO_V_B16H8S4 }  // uxtl2_v
});

pub const SIMD_TBL_TBX: [SimdTblTbx; 2] = table_new!(SimdTblTbx, {
    { 0b0000111000000000000000 }, // tbl_v
    { 0b0000111000000000000100 }  // tbx_v
});

pub static INST_NAME_STRING_TABLE: &[u8] = b"autia1716autibldsmaxalhldsminalldumaxallduminalsha256su0sha512su1sm3partwsqrshrunldaddalldclralldeoralldsetallbstsmaxstsminstumaxstuminfrint32z64x64zh2sqdmlalsl2sqdmulsqrdmlaulhn2sqshruuqrshrspchkfeacrc32cstaddstclrsteorstsetxpaclbfcvtbfmlaltfcvtxfjcvtzfmaxnmfminnmfrsqrraddrsubsha1sm3tt12a2bsm4ekeysqxtuuqshrursqrsetfrev8";
#[rustfmt::skip]
pub static INST_NAME_INDEX_TABLE: &[u32] = &[
        0x80000000, // Small ''.
    0x80004C41, // Small 'abs'.
    0x80000C81, // Small 'adc'.
    0x80098C81, // Small 'adcs'.
    0x80001081, // Small 'add'.
    0x80039081, // Small 'addg'.
    0x80099081, // Small 'adds'.
    0x80004881, // Small 'adr'.
    0x80084881, // Small 'adrp'.
    0x800011C1, // Small 'and'.
    0x800991C1, // Small 'ands'.
    0x80004A61, // Small 'asr'.
    0x800B4A61, // Small 'asrv'.
    0x80000281, // Small 'at'.
    0x801252A1, // Small 'autda'.
    0x83A252A1, // Small 'autdza'.
    0x802252A1, // Small 'autdb'.
    0x85A252A1, // Small 'autdzb'.
    0x8014D2A1, // Small 'autia'.
    0x00009000, // Large 'autia1716'.
    0x20BF5000, // Large 'autia|sp'.
    0xB414D2A1, // Small 'autiaz'.
    0x8024D2A1, // Small 'autib'.
    0x40055009, // Large 'autib|1716'.
    0x20BF5009, // Large 'autib|sp'.
    0xB424D2A1, // Small 'autibz'.
    0x83A4D2A1, // Small 'autiza'.
    0x85A4D2A1, // Small 'autizb'.
    0x8E161B01, // Small 'axflag'.
    0x80000002, // Small 'b'.
    0x80000062, // Small 'bc'.
    0x80000CC2, // Small 'bfc'.
    0x800024C2, // Small 'bfi'.
    0x800034C2, // Small 'bfm'.
    0x80C4E0C2, // Small 'bfxil'.
    0x80000D22, // Small 'bic'.
    0x80098D22, // Small 'bics'.
    0x80000182, // Small 'bl'.
    0x80004982, // Small 'blr'.
    0x80000242, // Small 'br'.
    0x80002E42, // Small 'brk'.
    0x80002682, // Small 'bti'.
    0x80004C23, // Small 'cas'.
    0x8000CC23, // Small 'casa'.
    0x8020CC23, // Small 'casab'.
    0x8080CC23, // Small 'casah'.
    0x80C0CC23, // Small 'casal'.
    0x84C0CC23, // Small 'casalb'.
    0x90C0CC23, // Small 'casalh'.
    0x80014C23, // Small 'casb'.
    0x80044C23, // Small 'cash'.
    0x80064C23, // Small 'casl'.
    0x80264C23, // Small 'caslb'.
    0x80864C23, // Small 'caslh'.
    0x80084C23, // Small 'casp'.
    0x80184C23, // Small 'caspa'.
    0x98184C23, // Small 'caspal'.
    0x80C84C23, // Small 'caspl'.
    0x800D3843, // Small 'cbnz'.
    0x80006843, // Small 'cbz'.
    0x80073463, // Small 'ccmn'.
    0x80083463, // Small 'ccmp'.
    0x816724C3, // Small 'cfinv'.
    0x100260C1, // Large 'chkfea|t'.
    0x8001B923, // Small 'cinc'.
    0x800B3923, // Small 'cinv'.
    0x84814983, // Small 'clrbhb'.
    0x8182C983, // Small 'clrex'.
    0x80004D83, // Small 'cls'.
    0x80006983, // Small 'clz'.
    0x800039A3, // Small 'cmn'.
    0x800041A3, // Small 'cmp'.
    0x800841A3, // Small 'cmpp'.
    0x800395C3, // Small 'cneg'.
    0x800051C3, // Small 'cnt'.
    0x85DF0E43, // Small 'crc32b'.
    0x100D60C7, // Large 'crc32c|b'.
    0x101660C7, // Large 'crc32c|h'.
    0x104860C7, // Large 'crc32c|w'.
    0x101360C7, // Large 'crc32c|x'.
    0x91DF0E43, // Small 'crc32h'.
    0xAFDF0E43, // Small 'crc32w'.
    0xB1DF0E43, // Small 'crc32x'.
    0x80011263, // Small 'csdb'.
    0x80061663, // Small 'csel'.
    0x800A1663, // Small 'cset'.
    0x80DA1663, // Small 'csetm'.
    0x80372663, // Small 'csinc'.
    0x81672663, // Small 'csinv'.
    0x8072BA63, // Small 'csneg'.
    0x80006A83, // Small 'ctz'.
    0x80000064, // Small 'dc'.
    0x81C9C064, // Small 'dcps1'.
    0x81D9C064, // Small 'dcps2'.
    0x81E9C064, // Small 'dcps3'.
    0x800020E4, // Small 'dgh'.
    0x800009A4, // Small 'dmb'.
    0x8009C244, // Small 'drps'.
    0x80000A64, // Small 'dsb'.
    0x800039E5, // Small 'eon'.
    0x800049E5, // Small 'eor'.
    0x80000A65, // Small 'esb'.
    0x80095305, // Small 'extr'.
    0x800A1645, // Small 'eret'.
    0x800025A7, // Small 'gmi'.
    0x800A3928, // Small 'hint'.
    0x80005188, // Small 'hlt'.
    0x80000EC8, // Small 'hvc'.
    0x80000069, // Small 'ic'.
    0x80000A69, // Small 'isb'.
    0x8042048C, // Small 'ldadd'.
    0x8242048C, // Small 'ldadda'.
    0x100D6051, // Large 'ldadda|b'.
    0x10166051, // Large 'ldadda|h'.
    0x00007051, // Large 'ldaddal'.
    0x100D7051, // Large 'ldaddal|b'.
    0x10167051, // Large 'ldaddal|h'.
    0x8442048C, // Small 'ldaddb'.
    0x9042048C, // Small 'ldaddh'.
    0x9842048C, // Small 'ldaddl'.
    0x206D5051, // Large 'ldadd|lb'.
    0x20155051, // Large 'ldadd|lh'.
    0x8009048C, // Small 'ldar'.
    0x8029048C, // Small 'ldarb'.
    0x8089048C, // Small 'ldarh'.
    0x810C048C, // Small 'ldaxp'.
    0x812C048C, // Small 'ldaxr'.
    0x852C048C, // Small 'ldaxrb'.
    0x912C048C, // Small 'ldaxrh'.
    0x81260C8C, // Small 'ldclr'.
    0x83260C8C, // Small 'ldclra'.
    0x100D6058, // Large 'ldclra|b'.
    0x10166058, // Large 'ldclra|h'.
    0x00007058, // Large 'ldclral'.
    0x100D7058, // Large 'ldclral|b'.
    0x10167058, // Large 'ldclral|h'.
    0x85260C8C, // Small 'ldclrb'.
    0x91260C8C, // Small 'ldclrh'.
    0x99260C8C, // Small 'ldclrl'.
    0x206D5058, // Large 'ldclr|lb'.
    0x20155058, // Large 'ldclr|lh'.
    0x8127948C, // Small 'ldeor'.
    0x8327948C, // Small 'ldeora'.
    0x100D605F, // Large 'ldeora|b'.
    0x1016605F, // Large 'ldeora|h'.
    0x0000705F, // Large 'ldeoral'.
    0x100D705F, // Large 'ldeoral|b'.
    0x1016705F, // Large 'ldeoral|h'.
    0x8527948C, // Small 'ldeorb'.
    0x9127948C, // Small 'ldeorh'.
    0x9927948C, // Small 'ldeorl'.
    0x206D505F, // Large 'ldeor|lb'.
    0x2015505F, // Large 'ldeor|lh'.
    0x80001C8C, // Small 'ldg'.
    0x80069C8C, // Small 'ldgm'.
    0x8120B08C, // Small 'ldlar'.
    0x8520B08C, // Small 'ldlarb'.
    0x9120B08C, // Small 'ldlarh'.
    0x8008388C, // Small 'ldnp'.
    0x8000408C, // Small 'ldp'.
    0x8179C08C, // Small 'ldpsw'.
    0x8000488C, // Small 'ldr'.
    0x8010C88C, // Small 'ldraa'.
    0x8020C88C, // Small 'ldrab'.
    0x8001488C, // Small 'ldrb'.
    0x8004488C, // Small 'ldrh'.
    0x8029C88C, // Small 'ldrsb'.
    0x8089C88C, // Small 'ldrsh'.
    0x8179C88C, // Small 'ldrsw'.
    0x8142CC8C, // Small 'ldset'.
    0x8342CC8C, // Small 'ldseta'.
    0x100D6066, // Large 'ldseta|b'.
    0x10166066, // Large 'ldseta|h'.
    0x00007066, // Large 'ldsetal'.
    0x100D7066, // Large 'ldsetal|b'.
    0x10167066, // Large 'ldsetal|h'.
    0x8542CC8C, // Small 'ldsetb'.
    0x9142CC8C, // Small 'ldseth'.
    0x9942CC8C, // Small 'ldsetl'.
    0x206D5066, // Large 'ldset|lb'.
    0x20155066, // Large 'ldset|lh'.
    0xB016CC8C, // Small 'ldsmax'.
    0x0000700E, // Large 'ldsmaxa'.
    0x100D700E, // Large 'ldsmaxa|b'.
    0x1016700E, // Large 'ldsmaxa|h'.
    0x0000800E, // Large 'ldsmaxal'.
    0x100D800E, // Large 'ldsmaxal|b'.
    0x1016800E, // Large 'ldsmaxal|h'.
    0x100D600E, // Large 'ldsmax|b'.
    0x1016600E, // Large 'ldsmax|h'.
    0x100E600E, // Large 'ldsmax|l'.
    0x206D600E, // Large 'ldsmax|lb'.
    0x2015600E, // Large 'ldsmax|lh'.
    0x9C96CC8C, // Small 'ldsmin'.
    0x00007017, // Large 'ldsmina'.
    0x100D7017, // Large 'ldsmina|b'.
    0x10167017, // Large 'ldsmina|h'.
    0x00008017, // Large 'ldsminal'.
    0x100D8017, // Large 'ldsminal|b'.
    0x10168017, // Large 'ldsminal|h'.
    0x100D6017, // Large 'ldsmin|b'.
    0x10166017, // Large 'ldsmin|h'.
    0x100E6017, // Large 'ldsmin|l'.
    0x206D6017, // Large 'ldsmin|lb'.
    0x20156017, // Large 'ldsmin|lh'.
    0x8009508C, // Small 'ldtr'.
    0x8029508C, // Small 'ldtrb'.
    0x8089508C, // Small 'ldtrh'.
    0x8539508C, // Small 'ldtrsb'.
    0x9139508C, // Small 'ldtrsh'.
    0xAF39508C, // Small 'ldtrsw'.
    0xB016D48C, // Small 'ldumax'.
    0x0000701F, // Large 'ldumaxa'.
    0x100D701F, // Large 'ldumaxa|b'.
    0x1016701F, // Large 'ldumaxa|h'.
    0x0000801F, // Large 'ldumaxal'.
    0x100D801F, // Large 'ldumaxal|b'.
    0x1016801F, // Large 'ldumaxal|h'.
    0x100D601F, // Large 'ldumax|b'.
    0x1016601F, // Large 'ldumax|h'.
    0x100E601F, // Large 'ldumax|l'.
    0x206D601F, // Large 'ldumax|lb'.
    0x2015601F, // Large 'ldumax|lh'.
    0x9C96D48C, // Small 'ldumin'.
    0x00007027, // Large 'ldumina'.
    0x100D7027, // Large 'ldumina|b'.
    0x10167027, // Large 'ldumina|h'.
    0x00008027, // Large 'lduminal'.
    0x100D8027, // Large 'lduminal|b'.
    0x10168027, // Large 'lduminal|h'.
    0x100D6027, // Large 'ldumin|b'.
    0x10166027, // Large 'ldumin|h'.
    0x100E6027, // Large 'ldumin|l'.
    0x206D6027, // Large 'ldumin|lb'.
    0x20156027, // Large 'ldumin|lh'.
    0x8009548C, // Small 'ldur'.
    0x8029548C, // Small 'ldurb'.
    0x8089548C, // Small 'ldurh'.
    0x8539548C, // Small 'ldursb'.
    0x9139548C, // Small 'ldursh'.
    0xAF39548C, // Small 'ldursw'.
    0x8008608C, // Small 'ldxp'.
    0x8009608C, // Small 'ldxr'.
    0x8029608C, // Small 'ldxrb'.
    0x8089608C, // Small 'ldxrh'.
    0x8000326C, // Small 'lsl'.
    0x800B326C, // Small 'lslv'.
    0x80004A6C, // Small 'lsr'.
    0x800B4A6C, // Small 'lsrv'.
    0x8002102D, // Small 'madd'.
    0x800395CD, // Small 'mneg'.
    0x800059ED, // Small 'mov'.
    0x8005D9ED, // Small 'movk'.
    0x800759ED, // Small 'movn'.
    0x800D59ED, // Small 'movz'.
    0x80004E4D, // Small 'mrs'.
    0x80004A6D, // Small 'msr'.
    0x8001566D, // Small 'msub'.
    0x800032AD, // Small 'mul'.
    0x80003ACD, // Small 'mvn'.
    0x80001CAE, // Small 'neg'.
    0x80099CAE, // Small 'negs'.
    0x80000CEE, // Small 'ngc'.
    0x80098CEE, // Small 'ngcs'.
    0x800041EE, // Small 'nop'.
    0x80003A4F, // Small 'orn'.
    0x80004A4F, // Small 'orr'.
    0x80120C30, // Small 'pacda'.
    0x80220C30, // Small 'pacdb'.
    0x83A20C30, // Small 'pacdza'.
    0x85A20C30, // Small 'pacdzb'.
    0x80138C30, // Small 'pacga'.
    0x80069A50, // Small 'prfm'.
    0x80214E70, // Small 'pssbb'.
    0x800A2452, // Small 'rbit'.
    0x800050B2, // Small 'ret'.
    0x800058B2, // Small 'rev'.
    0x2007313E, // Large 'rev|16'.
    0x81DF58B2, // Small 'rev32'.
    0x208F313E, // Large 'rev|64'.
    0x800049F2, // Small 'ror'.
    0x800B49F2, // Small 'rorv'.
    0x80000C53, // Small 'sbc'.
    0x80098C53, // Small 'sbcs'.
    0x81A49853, // Small 'sbfiz'.
    0x80069853, // Small 'sbfm'.
    0x800C1853, // Small 'sbfx'.
    0x800B2493, // Small 'sdiv'.
    0x1141413A, // Large 'setf|8'.
    0x2007413A, // Large 'setf|16'.
    0x800058B3, // Small 'sev'.
    0x800658B3, // Small 'sevl'.
    0x984205B3, // Small 'smaddl'.
    0x800C05B3, // Small 'smax'.
    0x80000DB3, // Small 'smc'.
    0x800725B3, // Small 'smin'.
    0x9872B9B3, // Small 'smnegl'.
    0x982ACDB3, // Small 'smsubl'.
    0x808655B3, // Small 'smulh'.
    0x80C655B3, // Small 'smull'.
    0x80010A73, // Small 'ssbb'.
    0x8003F693, // Small 'st2g'.
    0x80420693, // Small 'stadd'.
    0x98420693, // Small 'staddl'.
    0x84420693, // Small 'staddb'.
    0x206D50CD, // Large 'stadd|lb'.
    0x90420693, // Small 'staddh'.
    0x201550CD, // Large 'stadd|lh'.
    0x81260E93, // Small 'stclr'.
    0x99260E93, // Small 'stclrl'.
    0x85260E93, // Small 'stclrb'.
    0x206D50D2, // Large 'stclr|lb'.
    0x91260E93, // Small 'stclrh'.
    0x201550D2, // Large 'stclr|lh'.
    0x81279693, // Small 'steor'.
    0x99279693, // Small 'steorl'.
    0x85279693, // Small 'steorb'.
    0x206D50D7, // Large 'steor|lb'.
    0x91279693, // Small 'steorh'.
    0x201550D7, // Large 'steor|lh'.
    0x80001E93, // Small 'stg'.
    0x80069E93, // Small 'stgm'.
    0x80081E93, // Small 'stgp'.
    0x81263293, // Small 'stllr'.
    0x85263293, // Small 'stllrb'.
    0x91263293, // Small 'stllrh'.
    0x80093293, // Small 'stlr'.
    0x80293293, // Small 'stlrb'.
    0x80893293, // Small 'stlrh'.
    0x810C3293, // Small 'stlxp'.
    0x812C3293, // Small 'stlxr'.
    0x852C3293, // Small 'stlxrb'.
    0x912C3293, // Small 'stlxrh'.
    0x80083A93, // Small 'stnp'.
    0x80004293, // Small 'stp'.
    0x80004A93, // Small 'str'.
    0x80014A93, // Small 'strb'.
    0x80044A93, // Small 'strh'.
    0x8142CE93, // Small 'stset'.
    0x9942CE93, // Small 'stsetl'.
    0x8542CE93, // Small 'stsetb'.
    0x206D50DC, // Large 'stset|lb'.
    0x9142CE93, // Small 'stseth'.
    0x201550DC, // Large 'stset|lh'.
    0xB016CE93, // Small 'stsmax'.
    0x100E606F, // Large 'stsmax|l'.
    0x100D606F, // Large 'stsmax|b'.
    0x206D606F, // Large 'stsmax|lb'.
    0x1016606F, // Large 'stsmax|h'.
    0x2015606F, // Large 'stsmax|lh'.
    0x9C96CE93, // Small 'stsmin'.
    0x100E6075, // Large 'stsmin|l'.
    0x100D6075, // Large 'stsmin|b'.
    0x206D6075, // Large 'stsmin|lb'.
    0x10166075, // Large 'stsmin|h'.
    0x20156075, // Large 'stsmin|lh'.
    0x80095293, // Small 'sttr'.
    0x80295293, // Small 'sttrb'.
    0x80895293, // Small 'sttrh'.
    0xB016D693, // Small 'stumax'.
    0x100E607B, // Large 'stumax|l'.
    0x100D607B, // Large 'stumax|b'.
    0x206D607B, // Large 'stumax|lb'.
    0x1016607B, // Large 'stumax|h'.
    0x2015607B, // Large 'stumax|lh'.
    0x9C96D693, // Small 'stumin'.
    0x100E6081, // Large 'stumin|l'.
    0x100D6081, // Large 'stumin|b'.
    0x206D6081, // Large 'stumin|lb'.
    0x10166081, // Large 'stumin|h'.
    0x20156081, // Large 'stumin|lh'.
    0x80095693, // Small 'stur'.
    0x80295693, // Small 'sturb'.
    0x80895693, // Small 'sturh'.
    0x80086293, // Small 'stxp'.
    0x80096293, // Small 'stxr'.
    0x80296293, // Small 'stxrb'.
    0x80896293, // Small 'stxrh'.
    0x807EEA93, // Small 'stz2g'.
    0x8003EA93, // Small 'stzg'.
    0x80D3EA93, // Small 'stzgm'.
    0x80000AB3, // Small 'sub'.
    0x80038AB3, // Small 'subg'.
    0x80080AB3, // Small 'subp'.
    0x81380AB3, // Small 'subps'.
    0x80098AB3, // Small 'subs'.
    0x80000ED3, // Small 'svc'.
    0x800042F3, // Small 'swp'.
    0x8000C2F3, // Small 'swpa'.
    0x8020C2F3, // Small 'swpab'.
    0x8080C2F3, // Small 'swpah'.
    0x80C0C2F3, // Small 'swpal'.
    0x84C0C2F3, // Small 'swpalb'.
    0x90C0C2F3, // Small 'swpalh'.
    0x800142F3, // Small 'swpb'.
    0x800442F3, // Small 'swph'.
    0x800642F3, // Small 'swpl'.
    0x802642F3, // Small 'swplb'.
    0x808642F3, // Small 'swplh'.
    0x80015313, // Small 'sxtb'.
    0x80045313, // Small 'sxth'.
    0x800BD313, // Small 'sxtw'.
    0x80004F33, // Small 'sys'.
    0x80048994, // Small 'tlbi'.
    0x80005274, // Small 'tst'.
    0x800D3854, // Small 'tbnz'.
    0x80006854, // Small 'tbz'.
    0x81A49855, // Small 'ubfiz'.
    0x80069855, // Small 'ubfm'.
    0x800C1855, // Small 'ubfx'.
    0x80001895, // Small 'udf'.
    0x800B2495, // Small 'udiv'.
    0x984205B5, // Small 'umaddl'.
    0x800C05B5, // Small 'umax'.
    0x800725B5, // Small 'umin'.
    0x9872B9B5, // Small 'umnegl'.
    0x80C655B5, // Small 'umull'.
    0x808655B5, // Small 'umulh'.
    0x982ACDB5, // Small 'umsubl'.
    0x80015315, // Small 'uxtb'.
    0x80045315, // Small 'uxth'.
    0x800014D7, // Small 'wfe'.
    0x800024D7, // Small 'wfi'.
    0x8E161838, // Small 'xaflag'.
    0x80418618, // Small 'xpacd'.
    0x80918618, // Small 'xpaci'.
    0x208850E1, // Large 'xpacl|ri'.
    0x80461539, // Small 'yield'.
    0x80004C41, // Small 'abs'.
    0x80001081, // Small 'add'.
    0x80E41081, // Small 'addhn'.
    0xBAE41081, // Small 'addhn2'.
    0x80081081, // Small 'addp'.
    0x800B1081, // Small 'addv'.
    0x80024CA1, // Small 'aesd'.
    0x8002CCA1, // Small 'aese'.
    0x86D4CCA1, // Small 'aesimc'.
    0x8036CCA1, // Small 'aesmc'.
    0x800011C1, // Small 'and'.
    0x800C0462, // Small 'bcax'.
    0x814B0CC2, // Small 'bfcvt'.
    0x9D4B0CC2, // Small 'bfcvtn'.
    0x20B150E6, // Large 'bfcvt|n2'.
    0x814790C2, // Small 'bfdot'.
    0x206D50EB, // Large 'bfmla|lb'.
    0x20F050EB, // Large 'bfmla|lt'.
    0x82C6B4C2, // Small 'bfmmla'.
    0x80000D22, // Small 'bic'.
    0x80001922, // Small 'bif'.
    0x80005122, // Small 'bit'.
    0x80003262, // Small 'bsl'.
    0x80004D83, // Small 'cls'.
    0x80006983, // Small 'clz'.
    0x800895A3, // Small 'cmeq'.
    0x80029DA3, // Small 'cmge'.
    0x800A1DA3, // Small 'cmgt'.
    0x8004A1A3, // Small 'cmhi'.
    0x8009A1A3, // Small 'cmhs'.
    0x8002B1A3, // Small 'cmle'.
    0x800A31A3, // Small 'cmlt'.
    0x8149D1A3, // Small 'cmtst'.
    0x800051C3, // Small 'cnt'.
    0x800042A4, // Small 'dup'.
    0x800049E5, // Small 'eor'.
    0x800F49E5, // Small 'eor3'.
    0x80005305, // Small 'ext'.
    0x80020826, // Small 'fabd'.
    0x80098826, // Small 'fabs'.
    0x80538C26, // Small 'facge'.
    0x81438C26, // Small 'facgt'.
    0x80021026, // Small 'fadd'.
    0x81021026, // Small 'faddp'.
    0x80420466, // Small 'fcadd'.
    0x81068C66, // Small 'fccmp'.
    0x8B068C66, // Small 'fccmpe'.
    0x8112B466, // Small 'fcmeq'.
    0x8053B466, // Small 'fcmge'.
    0x8143B466, // Small 'fcmgt'.
    0x80163466, // Small 'fcmla'.
    0x80563466, // Small 'fcmle'.
    0x81463466, // Small 'fcmlt'.
    0x80083466, // Small 'fcmp'.
    0x80583466, // Small 'fcmpe'.
    0x80C2CC66, // Small 'fcsel'.
    0x800A5866, // Small 'fcvt'.
    0xA61A5866, // Small 'fcvtas'.
    0xAA1A5866, // Small 'fcvtau'.
    0x80CA5866, // Small 'fcvtl'.
    0xBACA5866, // Small 'fcvtl2'.
    0xA6DA5866, // Small 'fcvtms'.
    0xAADA5866, // Small 'fcvtmu'.
    0x80EA5866, // Small 'fcvtn'.
    0xBAEA5866, // Small 'fcvtn2'.
    0xA6EA5866, // Small 'fcvtns'.
    0xAAEA5866, // Small 'fcvtnu'.
    0xA70A5866, // Small 'fcvtps'.
    0xAB0A5866, // Small 'fcvtpu'.
    0x9D8A5866, // Small 'fcvtxn'.
    0x20B150F2, // Large 'fcvtx|n2'.
    0xA7AA5866, // Small 'fcvtzs'.
    0xABAA5866, // Small 'fcvtzu'.
    0x800B2486, // Small 'fdiv'.
    0x101060F7, // Large 'fjcvtz|s'.
    0x804205A6, // Small 'fmadd'.
    0x800C05A6, // Small 'fmax'.
    0x9AEC05A6, // Small 'fmaxnm'.
    0x104460FD, // Large 'fmaxnm|p'.
    0x10E960FD, // Large 'fmaxnm|v'.
    0x810C05A6, // Small 'fmaxp'.
    0x816C05A6, // Small 'fmaxv'.
    0x800725A6, // Small 'fmin'.
    0x9AE725A6, // Small 'fminnm'.
    0x10446103, // Large 'fminnm|p'.
    0x10E96103, // Large 'fminnm|v'.
    0x810725A6, // Small 'fminp'.
    0x816725A6, // Small 'fminv'.
    0x8000B1A6, // Small 'fmla'.
    0x80C0B1A6, // Small 'fmlal'.
    0xBAC0B1A6, // Small 'fmlal2'.
    0x8009B1A6, // Small 'fmls'.
    0x80C9B1A6, // Small 'fmlsl'.
    0xBAC9B1A6, // Small 'fmlsl2'.
    0x800B3DA6, // Small 'fmov'.
    0x802ACDA6, // Small 'fmsub'.
    0x800655A6, // Small 'fmul'.
    0x818655A6, // Small 'fmulx'.
    0x800395C6, // Small 'fneg'.
    0x8840B5C6, // Small 'fnmadd'.
    0x8559B5C6, // Small 'fnmsub'.
    0x80CAB5C6, // Small 'fnmul'.
    0x8B019646, // Small 'frecpe'.
    0xA7019646, // Small 'frecps'.
    0xB1019646, // Small 'frecpx'.
    0x10137087, // Large 'frint32|x'.
    0x108E7087, // Large 'frint32|z'.
    0x308F5087, // Large 'frint|64x'.
    0x30925087, // Large 'frint|64z'.
    0x83472646, // Small 'frinta'.
    0x93472646, // Small 'frinti'.
    0x9B472646, // Small 'frintm'.
    0x9D472646, // Small 'frintn'.
    0xA1472646, // Small 'frintp'.
    0xB1472646, // Small 'frintx'.
    0xB5472646, // Small 'frintz'.
    0x20D85109, // Large 'frsqr|te'.
    0x20705109, // Large 'frsqr|ts'.
    0x81494666, // Small 'fsqrt'.
    0x80015666, // Small 'fsub'.
    0x80004DC9, // Small 'ins'.
    0x8000708C, // Small 'ld1'.
    0x8009708C, // Small 'ld1r'.
    0x8000748C, // Small 'ld2'.
    0x8009748C, // Small 'ld2r'.
    0x8000788C, // Small 'ld3'.
    0x8009788C, // Small 'ld3r'.
    0x80007C8C, // Small 'ld4'.
    0x80097C8C, // Small 'ld4r'.
    0x8008388C, // Small 'ldnp'.
    0x8000408C, // Small 'ldp'.
    0x8000488C, // Small 'ldr'.
    0x8009548C, // Small 'ldur'.
    0x8000058D, // Small 'mla'.
    0x80004D8D, // Small 'mls'.
    0x800059ED, // Small 'mov'.
    0x8004D9ED, // Small 'movi'.
    0x800032AD, // Small 'mul'.
    0x80003ACD, // Small 'mvn'.
    0x8004BACD, // Small 'mvni'.
    0x80001CAE, // Small 'neg'.
    0x800051EE, // Small 'not'.
    0x80003A4F, // Small 'orn'.
    0x80004A4F, // Small 'orr'.
    0x800655B0, // Small 'pmul'.
    0x80C655B0, // Small 'pmull'.
    0xBAC655B0, // Small 'pmull2'.
    0x9C821032, // Small 'raddhn'.
    0x30B0410E, // Large 'radd|hn2'.
    0x800E6032, // Small 'rax1'.
    0x800A2452, // Small 'rbit'.
    0x2007313E, // Large 'rev|16'.
    0x81DF58B2, // Small 'rev32'.
    0x208F313E, // Large 'rev|64'.
    0x80E92272, // Small 'rshrn'.
    0xBAE92272, // Small 'rshrn2'.
    0x9C815672, // Small 'rsubhn'.
    0x30B04112, // Large 'rsub|hn2'.
    0x80008833, // Small 'saba'.
    0x80C08833, // Small 'sabal'.
    0xBAC08833, // Small 'sabal2'.
    0x80020833, // Small 'sabd'.
    0x80C20833, // Small 'sabdl'.
    0xBAC20833, // Small 'sabdl2'.
    0xA0C09033, // Small 'sadalp'.
    0x80C21033, // Small 'saddl'.
    0xBAC21033, // Small 'saddl2'.
    0xA0C21033, // Small 'saddlp'.
    0xACC21033, // Small 'saddlv'.
    0x81721033, // Small 'saddw'.
    0xBB721033, // Small 'saddw2'.
    0x806A5873, // Small 'scvtf'.
    0x800A3C93, // Small 'sdot'.
    0x803E0513, // Small 'sha1c'.
    0x808E0513, // Small 'sha1h'.
    0x80DE0513, // Small 'sha1m'.
    0x810E0513, // Small 'sha1p'.
    0x30354116, // Large 'sha1|su0'.
    0x303E4116, // Large 'sha1|su1'.
    0x1016602F, // Large 'sha256|h'.
    0x2095602F, // Large 'sha256|h2'.
    0x0000902F, // Large 'sha256su0'.
    0x1005802F, // Large 'sha256su|1'.
    0x10166038, // Large 'sha512|h'.
    0x20956038, // Large 'sha512|h2'.
    0x30356038, // Large 'sha512|su0'.
    0x303E6038, // Large 'sha512|su1'.
    0x80420513, // Small 'shadd'.
    0x80003113, // Small 'shl'.
    0x80063113, // Small 'shll'.
    0x81D63113, // Small 'shll2'.
    0x80074913, // Small 'shrn'.
    0x81D74913, // Small 'shrn2'.
    0x802ACD13, // Small 'shsub'.
    0x80002593, // Small 'sli'.
    0x10058041, // Large 'sm3partw|1'.
    0x10328041, // Large 'sm3partw|2'.
    0xB939F9B3, // Small 'sm3ss1'.
    0x1000611A, // Large 'sm3tt1|a'.
    0x100D611A, // Large 'sm3tt1|b'.
    0x2120511A, // Large 'sm3tt|2a'.
    0x2122511A, // Large 'sm3tt|2b'.
    0x8002FDB3, // Small 'sm4e'.
    0x00007124, // Large 'sm4ekey'.
    0x800C05B3, // Small 'smax'.
    0x810C05B3, // Small 'smaxp'.
    0x816C05B3, // Small 'smaxv'.
    0x800725B3, // Small 'smin'.
    0x810725B3, // Small 'sminp'.
    0x816725B3, // Small 'sminv'.
    0x80C0B1B3, // Small 'smlal'.
    0xBAC0B1B3, // Small 'smlal2'.
    0x80C9B1B3, // Small 'smlsl'.
    0xBAC9B1B3, // Small 'smlsl2'.
    0x801635B3, // Small 'smmla'.
    0x800B3DB3, // Small 'smov'.
    0x80C655B3, // Small 'smull'.
    0xBAC655B3, // Small 'smull2'.
    0x81310633, // Small 'sqabs'.
    0x80420633, // Small 'sqadd'.
    0x00007097, // Large 'sqdmlal'.
    0x10327097, // Large 'sqdmlal|2'.
    0x209E5097, // Large 'sqdml|sl'.
    0x309E5097, // Large 'sqdml|sl2'.
    0x101660A1, // Large 'sqdmul|h'.
    0x100E60A1, // Large 'sqdmul|l'.
    0x209F60A1, // Large 'sqdmul|l2'.
    0x8072BA33, // Small 'sqneg'.
    0x101670A7, // Large 'sqrdmla|h'.
    0x202F60A7, // Large 'sqrdml|sh'.
    0x30AE50A7, // Large 'sqrdm|ulh'.
    0x9889CA33, // Small 'sqrshl'.
    0x101C6049, // Large 'sqrshr|n'.
    0x20B16049, // Large 'sqrshr|n2'.
    0x00008049, // Large 'sqrshrun'.
    0x10328049, // Large 'sqrshrun|2'.
    0x80C44E33, // Small 'sqshl'.
    0xAAC44E33, // Small 'sqshlu'.
    0x9D244E33, // Small 'sqshrn'.
    0x20B150B3, // Large 'sqshr|n2'.
    0x101C60B3, // Large 'sqshru|n'.
    0x20B160B3, // Large 'sqshru|n2'.
    0x802ACE33, // Small 'sqsub'.
    0x80EA6233, // Small 'sqxtn'.
    0xBAEA6233, // Small 'sqxtn2'.
    0x9D5A6233, // Small 'sqxtun'.
    0x20B1512B, // Large 'sqxtu|n2'.
    0x8840A253, // Small 'srhadd'.
    0x80002653, // Small 'sri'.
    0x80C44E53, // Small 'srshl'.
    0x81244E53, // Small 'srshr'.
    0x80194E53, // Small 'srsra'.
    0x80062273, // Small 'sshl'.
    0x80C62273, // Small 'sshll'.
    0xBAC62273, // Small 'sshll2'.
    0x80092273, // Small 'sshr'.
    0x8000CA73, // Small 'ssra'.
    0x80C15673, // Small 'ssubl'.
    0xBAC15673, // Small 'ssubl2'.
    0x81715673, // Small 'ssubw'.
    0xBB715673, // Small 'ssubw2'.
    0x80007293, // Small 'st1'.
    0x80007693, // Small 'st2'.
    0x80007A93, // Small 'st3'.
    0x80007E93, // Small 'st4'.
    0x80083A93, // Small 'stnp'.
    0x80004293, // Small 'stp'.
    0x80004A93, // Small 'str'.
    0x80095693, // Small 'stur'.
    0x80000AB3, // Small 'sub'.
    0x80E40AB3, // Small 'subhn'.
    0xBAE40AB3, // Small 'subhn2'.
    0x814792B3, // Small 'sudot'.
    0x8840C6B3, // Small 'suqadd'.
    0x80065313, // Small 'sxtl'.
    0x81D65313, // Small 'sxtl2'.
    0x80003054, // Small 'tbl'.
    0x80006054, // Small 'tbx'.
    0x800E3A54, // Small 'trn1'.
    0x800EBA54, // Small 'trn2'.
    0x80008835, // Small 'uaba'.
    0x80C08835, // Small 'uabal'.
    0xBAC08835, // Small 'uabal2'.
    0x80020835, // Small 'uabd'.
    0x80C20835, // Small 'uabdl'.
    0xBAC20835, // Small 'uabdl2'.
    0xA0C09035, // Small 'uadalp'.
    0x80C21035, // Small 'uaddl'.
    0xBAC21035, // Small 'uaddl2'.
    0xA0C21035, // Small 'uaddlp'.
    0xACC21035, // Small 'uaddlv'.
    0x81721035, // Small 'uaddw'.
    0xBB721035, // Small 'uaddw2'.
    0x806A5875, // Small 'ucvtf'.
    0x800A3C95, // Small 'udot'.
    0x80420515, // Small 'uhadd'.
    0x802ACD15, // Small 'uhsub'.
    0x800C05B5, // Small 'umax'.
    0x810C05B5, // Small 'umaxp'.
    0x816C05B5, // Small 'umaxv'.
    0x800725B5, // Small 'umin'.
    0x810725B5, // Small 'uminp'.
    0x816725B5, // Small 'uminv'.
    0x80C0B1B5, // Small 'umlal'.
    0xBAC0B1B5, // Small 'umlal2'.
    0x80C9B1B5, // Small 'umlsl'.
    0xBAC9B1B5, // Small 'umlsl2'.
    0x801635B5, // Small 'ummla'.
    0x800B3DB5, // Small 'umov'.
    0x80C655B5, // Small 'umull'.
    0xBAC655B5, // Small 'umull2'.
    0x80420635, // Small 'uqadd'.
    0x9889CA35, // Small 'uqrshl'.
    0x101C60B9, // Large 'uqrshr|n'.
    0x20B160B9, // Large 'uqrshr|n2'.
    0x80C44E35, // Small 'uqshl'.
    0x9D244E35, // Small 'uqshrn'.
    0x20B15130, // Large 'uqshr|n2'.
    0x802ACE35, // Small 'uqsub'.
    0x80EA6235, // Small 'uqxtn'.
    0xBAEA6235, // Small 'uqxtn2'.
    0x8B019655, // Small 'urecpe'.
    0x8840A255, // Small 'urhadd'.
    0x80C44E55, // Small 'urshl'.
    0x81244E55, // Small 'urshr'.
    0x20D85135, // Large 'ursqr|te'.
    0x80194E55, // Small 'ursra'.
    0x81479275, // Small 'usdot'.
    0x80062275, // Small 'ushl'.
    0x80C62275, // Small 'ushll'.
    0xBAC62275, // Small 'ushll2'.
    0x80092275, // Small 'ushr'.
    0x82C6B675, // Small 'usmmla'.
    0x8840C675, // Small 'usqadd'.
    0x8000CA75, // Small 'usra'.
    0x80C15675, // Small 'usubl'.
    0xBAC15675, // Small 'usubl2'.
    0x81715675, // Small 'usubw'.
    0xBB715675, // Small 'usubw2'.
    0x80065315, // Small 'uxtl'.
    0x81D65315, // Small 'uxtl2'.
    0x800E4355, // Small 'uzp1'.
    0x800EC355, // Small 'uzp2'.
    0x80004838, // Small 'xar'.
    0x80003A98, // Small 'xtn'.
    0x800EBA98, // Small 'xtn2'.
    0x800E413A, // Small 'zip1'.
    0x800EC13A  // Small 'zip2'.
];

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum InstId {
    None = 0,                         // Instruction ''.
    Abs,                              // Instruction 'abs'.
    Adc,                              // Instruction 'adc'.
    Adcs,                             // Instruction 'adcs'.
    Add,                              // Instruction 'add'.
    Addg,                             // Instruction 'addg'.
    Adds,                             // Instruction 'adds'.
    Adr,                              // Instruction 'adr'.
    Adrp,                             // Instruction 'adrp'.
    And,                              // Instruction 'and'.
    Ands,                             // Instruction 'ands'.
    Asr,                              // Instruction 'asr'.
    Asrv,                             // Instruction 'asrv'.
    At,                               // Instruction 'at'.
    Autda,                            // Instruction 'autda'.
    Autdza,                           // Instruction 'autdza'.
    Autdb,                            // Instruction 'autdb'.
    Autdzb,                           // Instruction 'autdzb'.
    Autia,                            // Instruction 'autia'.
    Autia1716,                        // Instruction 'autia1716'.
    Autiasp,                          // Instruction 'autiasp'.
    Autiaz,                           // Instruction 'autiaz'.
    Autib,                            // Instruction 'autib'.
    Autib1716,                        // Instruction 'autib1716'.
    Autibsp,                          // Instruction 'autibsp'.
    Autibz,                           // Instruction 'autibz'.
    Autiza,                           // Instruction 'autiza'.
    Autizb,                           // Instruction 'autizb'.
    Axflag,                           // Instruction 'axflag'.
    B,                                // Instruction 'b'.
    Bc,                               // Instruction 'bc'.
    Bfc,                              // Instruction 'bfc'.
    Bfi,                              // Instruction 'bfi'.
    Bfm,                              // Instruction 'bfm'.
    Bfxil,                            // Instruction 'bfxil'.
    Bic,                              // Instruction 'bic'.
    Bics,                             // Instruction 'bics'.
    Bl,                               // Instruction 'bl'.
    Blr,                              // Instruction 'blr'.
    Br,                               // Instruction 'br'.
    Brk,                              // Instruction 'brk'.
    Bti,                              // Instruction 'bti'.
    Cas,                              // Instruction 'cas'.
    Casa,                             // Instruction 'casa'.
    Casab,                            // Instruction 'casab'.
    Casah,                            // Instruction 'casah'.
    Casal,                            // Instruction 'casal'.
    Casalb,                           // Instruction 'casalb'.
    Casalh,                           // Instruction 'casalh'.
    Casb,                             // Instruction 'casb'.
    Cash,                             // Instruction 'cash'.
    Casl,                             // Instruction 'casl'.
    Caslb,                            // Instruction 'caslb'.
    Caslh,                            // Instruction 'caslh'.
    Casp,                             // Instruction 'casp'.
    Caspa,                            // Instruction 'caspa'.
    Caspal,                           // Instruction 'caspal'.
    Caspl,                            // Instruction 'caspl'.
    Cbnz,                             // Instruction 'cbnz'.
    Cbz,                              // Instruction 'cbz'.
    Ccmn,                             // Instruction 'ccmn'.
    Ccmp,                             // Instruction 'ccmp'.
    Cfinv,                            // Instruction 'cfinv'.
    Chkfeat,                          // Instruction 'chkfeat'.
    Cinc,                             // Instruction 'cinc'.
    Cinv,                             // Instruction 'cinv'.
    Clrbhb,                           // Instruction 'clrbhb'.
    Clrex,                            // Instruction 'clrex'.
    Cls,                              // Instruction 'cls'.
    Clz,                              // Instruction 'clz'.
    Cmn,                              // Instruction 'cmn'.
    Cmp,                              // Instruction 'cmp'.
    Cmpp,                             // Instruction 'cmpp'.
    Cneg,                             // Instruction 'cneg'.
    Cnt,                              // Instruction 'cnt'.
    Crc32b,                           // Instruction 'crc32b'.
    Crc32cb,                          // Instruction 'crc32cb'.
    Crc32ch,                          // Instruction 'crc32ch'.
    Crc32cw,                          // Instruction 'crc32cw'.
    Crc32cx,                          // Instruction 'crc32cx'.
    Crc32h,                           // Instruction 'crc32h'.
    Crc32w,                           // Instruction 'crc32w'.
    Crc32x,                           // Instruction 'crc32x'.
    Csdb,                             // Instruction 'csdb'.
    Csel,                             // Instruction 'csel'.
    Cset,                             // Instruction 'cset'.
    Csetm,                            // Instruction 'csetm'.
    Csinc,                            // Instruction 'csinc'.
    Csinv,                            // Instruction 'csinv'.
    Csneg,                            // Instruction 'csneg'.
    Ctz,                              // Instruction 'ctz'.
    Dc,                               // Instruction 'dc'.
    Dcps1,                            // Instruction 'dcps1'.
    Dcps2,                            // Instruction 'dcps2'.
    Dcps3,                            // Instruction 'dcps3'.
    Dgh,                              // Instruction 'dgh'.
    Dmb,                              // Instruction 'dmb'.
    Drps,                             // Instruction 'drps'.
    Dsb,                              // Instruction 'dsb'.
    Eon,                              // Instruction 'eon'.
    Eor,                              // Instruction 'eor'.
    Esb,                              // Instruction 'esb'.
    Extr,                             // Instruction 'extr'.
    Eret,                             // Instruction 'eret'.
    Gmi,                              // Instruction 'gmi'.
    Hint,                             // Instruction 'hint'.
    Hlt,                              // Instruction 'hlt'.
    Hvc,                              // Instruction 'hvc'.
    Ic,                               // Instruction 'ic'.
    Isb,                              // Instruction 'isb'.
    Ldadd,                            // Instruction 'ldadd'.
    Ldadda,                           // Instruction 'ldadda'.
    Ldaddab,                          // Instruction 'ldaddab'.
    Ldaddah,                          // Instruction 'ldaddah'.
    Ldaddal,                          // Instruction 'ldaddal'.
    Ldaddalb,                         // Instruction 'ldaddalb'.
    Ldaddalh,                         // Instruction 'ldaddalh'.
    Ldaddb,                           // Instruction 'ldaddb'.
    Ldaddh,                           // Instruction 'ldaddh'.
    Ldaddl,                           // Instruction 'ldaddl'.
    Ldaddlb,                          // Instruction 'ldaddlb'.
    Ldaddlh,                          // Instruction 'ldaddlh'.
    Ldar,                             // Instruction 'ldar'.
    Ldarb,                            // Instruction 'ldarb'.
    Ldarh,                            // Instruction 'ldarh'.
    Ldaxp,                            // Instruction 'ldaxp'.
    Ldaxr,                            // Instruction 'ldaxr'.
    Ldaxrb,                           // Instruction 'ldaxrb'.
    Ldaxrh,                           // Instruction 'ldaxrh'.
    Ldclr,                            // Instruction 'ldclr'.
    Ldclra,                           // Instruction 'ldclra'.
    Ldclrab,                          // Instruction 'ldclrab'.
    Ldclrah,                          // Instruction 'ldclrah'.
    Ldclral,                          // Instruction 'ldclral'.
    Ldclralb,                         // Instruction 'ldclralb'.
    Ldclralh,                         // Instruction 'ldclralh'.
    Ldclrb,                           // Instruction 'ldclrb'.
    Ldclrh,                           // Instruction 'ldclrh'.
    Ldclrl,                           // Instruction 'ldclrl'.
    Ldclrlb,                          // Instruction 'ldclrlb'.
    Ldclrlh,                          // Instruction 'ldclrlh'.
    Ldeor,                            // Instruction 'ldeor'.
    Ldeora,                           // Instruction 'ldeora'.
    Ldeorab,                          // Instruction 'ldeorab'.
    Ldeorah,                          // Instruction 'ldeorah'.
    Ldeoral,                          // Instruction 'ldeoral'.
    Ldeoralb,                         // Instruction 'ldeoralb'.
    Ldeoralh,                         // Instruction 'ldeoralh'.
    Ldeorb,                           // Instruction 'ldeorb'.
    Ldeorh,                           // Instruction 'ldeorh'.
    Ldeorl,                           // Instruction 'ldeorl'.
    Ldeorlb,                          // Instruction 'ldeorlb'.
    Ldeorlh,                          // Instruction 'ldeorlh'.
    Ldg,                              // Instruction 'ldg'.
    Ldgm,                             // Instruction 'ldgm'.
    Ldlar,                            // Instruction 'ldlar'.
    Ldlarb,                           // Instruction 'ldlarb'.
    Ldlarh,                           // Instruction 'ldlarh'.
    Ldnp,                             // Instruction 'ldnp'.
    Ldp,                              // Instruction 'ldp'.
    Ldpsw,                            // Instruction 'ldpsw'.
    Ldr,                              // Instruction 'ldr'.
    Ldraa,                            // Instruction 'ldraa'.
    Ldrab,                            // Instruction 'ldrab'.
    Ldrb,                             // Instruction 'ldrb'.
    Ldrh,                             // Instruction 'ldrh'.
    Ldrsb,                            // Instruction 'ldrsb'.
    Ldrsh,                            // Instruction 'ldrsh'.
    Ldrsw,                            // Instruction 'ldrsw'.
    Ldset,                            // Instruction 'ldset'.
    Ldseta,                           // Instruction 'ldseta'.
    Ldsetab,                          // Instruction 'ldsetab'.
    Ldsetah,                          // Instruction 'ldsetah'.
    Ldsetal,                          // Instruction 'ldsetal'.
    Ldsetalb,                         // Instruction 'ldsetalb'.
    Ldsetalh,                         // Instruction 'ldsetalh'.
    Ldsetb,                           // Instruction 'ldsetb'.
    Ldseth,                           // Instruction 'ldseth'.
    Ldsetl,                           // Instruction 'ldsetl'.
    Ldsetlb,                          // Instruction 'ldsetlb'.
    Ldsetlh,                          // Instruction 'ldsetlh'.
    Ldsmax,                           // Instruction 'ldsmax'.
    Ldsmaxa,                          // Instruction 'ldsmaxa'.
    Ldsmaxab,                         // Instruction 'ldsmaxab'.
    Ldsmaxah,                         // Instruction 'ldsmaxah'.
    Ldsmaxal,                         // Instruction 'ldsmaxal'.
    Ldsmaxalb,                        // Instruction 'ldsmaxalb'.
    Ldsmaxalh,                        // Instruction 'ldsmaxalh'.
    Ldsmaxb,                          // Instruction 'ldsmaxb'.
    Ldsmaxh,                          // Instruction 'ldsmaxh'.
    Ldsmaxl,                          // Instruction 'ldsmaxl'.
    Ldsmaxlb,                         // Instruction 'ldsmaxlb'.
    Ldsmaxlh,                         // Instruction 'ldsmaxlh'.
    Ldsmin,                           // Instruction 'ldsmin'.
    Ldsmina,                          // Instruction 'ldsmina'.
    Ldsminab,                         // Instruction 'ldsminab'.
    Ldsminah,                         // Instruction 'ldsminah'.
    Ldsminal,                         // Instruction 'ldsminal'.
    Ldsminalb,                        // Instruction 'ldsminalb'.
    Ldsminalh,                        // Instruction 'ldsminalh'.
    Ldsminb,                          // Instruction 'ldsminb'.
    Ldsminh,                          // Instruction 'ldsminh'.
    Ldsminl,                          // Instruction 'ldsminl'.
    Ldsminlb,                         // Instruction 'ldsminlb'.
    Ldsminlh,                         // Instruction 'ldsminlh'.
    Ldtr,                             // Instruction 'ldtr'.
    Ldtrb,                            // Instruction 'ldtrb'.
    Ldtrh,                            // Instruction 'ldtrh'.
    Ldtrsb,                           // Instruction 'ldtrsb'.
    Ldtrsh,                           // Instruction 'ldtrsh'.
    Ldtrsw,                           // Instruction 'ldtrsw'.
    Ldumax,                           // Instruction 'ldumax'.
    Ldumaxa,                          // Instruction 'ldumaxa'.
    Ldumaxab,                         // Instruction 'ldumaxab'.
    Ldumaxah,                         // Instruction 'ldumaxah'.
    Ldumaxal,                         // Instruction 'ldumaxal'.
    Ldumaxalb,                        // Instruction 'ldumaxalb'.
    Ldumaxalh,                        // Instruction 'ldumaxalh'.
    Ldumaxb,                          // Instruction 'ldumaxb'.
    Ldumaxh,                          // Instruction 'ldumaxh'.
    Ldumaxl,                          // Instruction 'ldumaxl'.
    Ldumaxlb,                         // Instruction 'ldumaxlb'.
    Ldumaxlh,                         // Instruction 'ldumaxlh'.
    Ldumin,                           // Instruction 'ldumin'.
    Ldumina,                          // Instruction 'ldumina'.
    Lduminab,                         // Instruction 'lduminab'.
    Lduminah,                         // Instruction 'lduminah'.
    Lduminal,                         // Instruction 'lduminal'.
    Lduminalb,                        // Instruction 'lduminalb'.
    Lduminalh,                        // Instruction 'lduminalh'.
    Lduminb,                          // Instruction 'lduminb'.
    Lduminh,                          // Instruction 'lduminh'.
    Lduminl,                          // Instruction 'lduminl'.
    Lduminlb,                         // Instruction 'lduminlb'.
    Lduminlh,                         // Instruction 'lduminlh'.
    Ldur,                             // Instruction 'ldur'.
    Ldurb,                            // Instruction 'ldurb'.
    Ldurh,                            // Instruction 'ldurh'.
    Ldursb,                           // Instruction 'ldursb'.
    Ldursh,                           // Instruction 'ldursh'.
    Ldursw,                           // Instruction 'ldursw'.
    Ldxp,                             // Instruction 'ldxp'.
    Ldxr,                             // Instruction 'ldxr'.
    Ldxrb,                            // Instruction 'ldxrb'.
    Ldxrh,                            // Instruction 'ldxrh'.
    Lsl,                              // Instruction 'lsl'.
    Lslv,                             // Instruction 'lslv'.
    Lsr,                              // Instruction 'lsr'.
    Lsrv,                             // Instruction 'lsrv'.
    Madd,                             // Instruction 'madd'.
    Mneg,                             // Instruction 'mneg'.
    Mov,                              // Instruction 'mov'.
    Movk,                             // Instruction 'movk'.
    Movn,                             // Instruction 'movn'.
    Movz,                             // Instruction 'movz'.
    Mrs,                              // Instruction 'mrs'.
    Msr,                              // Instruction 'msr'.
    Msub,                             // Instruction 'msub'.
    Mul,                              // Instruction 'mul'.
    Mvn,                              // Instruction 'mvn'.
    Neg,                              // Instruction 'neg'.
    Negs,                             // Instruction 'negs'.
    Ngc,                              // Instruction 'ngc'.
    Ngcs,                             // Instruction 'ngcs'.
    Nop,                              // Instruction 'nop'.
    Orn,                              // Instruction 'orn'.
    Orr,                              // Instruction 'orr'.
    Pacda,                            // Instruction 'pacda'.
    Pacdb,                            // Instruction 'pacdb'.
    Pacdza,                           // Instruction 'pacdza'.
    Pacdzb,                           // Instruction 'pacdzb'.
    Pacga,                            // Instruction 'pacga'.
    Prfm,                             // Instruction 'prfm'.
    Pssbb,                            // Instruction 'pssbb'.
    Rbit,                             // Instruction 'rbit'.
    Ret,                              // Instruction 'ret'.
    Rev,                              // Instruction 'rev'.
    Rev16,                            // Instruction 'rev16'.
    Rev32,                            // Instruction 'rev32'.
    Rev64,                            // Instruction 'rev64'.
    Ror,                              // Instruction 'ror'.
    Rorv,                             // Instruction 'rorv'.
    Sbc,                              // Instruction 'sbc'.
    Sbcs,                             // Instruction 'sbcs'.
    Sbfiz,                            // Instruction 'sbfiz'.
    Sbfm,                             // Instruction 'sbfm'.
    Sbfx,                             // Instruction 'sbfx'.
    Sdiv,                             // Instruction 'sdiv'.
    Setf8,                            // Instruction 'setf8'.
    Setf16,                           // Instruction 'setf16'.
    Sev,                              // Instruction 'sev'.
    Sevl,                             // Instruction 'sevl'.
    Smaddl,                           // Instruction 'smaddl'.
    Smax,                             // Instruction 'smax'.
    Smc,                              // Instruction 'smc'.
    Smin,                             // Instruction 'smin'.
    Smnegl,                           // Instruction 'smnegl'.
    Smsubl,                           // Instruction 'smsubl'.
    Smulh,                            // Instruction 'smulh'.
    Smull,                            // Instruction 'smull'.
    Ssbb,                             // Instruction 'ssbb'.
    St2g,                             // Instruction 'st2g'.
    Stadd,                            // Instruction 'stadd'.
    Staddl,                           // Instruction 'staddl'.
    Staddb,                           // Instruction 'staddb'.
    Staddlb,                          // Instruction 'staddlb'.
    Staddh,                           // Instruction 'staddh'.
    Staddlh,                          // Instruction 'staddlh'.
    Stclr,                            // Instruction 'stclr'.
    Stclrl,                           // Instruction 'stclrl'.
    Stclrb,                           // Instruction 'stclrb'.
    Stclrlb,                          // Instruction 'stclrlb'.
    Stclrh,                           // Instruction 'stclrh'.
    Stclrlh,                          // Instruction 'stclrlh'.
    Steor,                            // Instruction 'steor'.
    Steorl,                           // Instruction 'steorl'.
    Steorb,                           // Instruction 'steorb'.
    Steorlb,                          // Instruction 'steorlb'.
    Steorh,                           // Instruction 'steorh'.
    Steorlh,                          // Instruction 'steorlh'.
    Stg,                              // Instruction 'stg'.
    Stgm,                             // Instruction 'stgm'.
    Stgp,                             // Instruction 'stgp'.
    Stllr,                            // Instruction 'stllr'.
    Stllrb,                           // Instruction 'stllrb'.
    Stllrh,                           // Instruction 'stllrh'.
    Stlr,                             // Instruction 'stlr'.
    Stlrb,                            // Instruction 'stlrb'.
    Stlrh,                            // Instruction 'stlrh'.
    Stlxp,                            // Instruction 'stlxp'.
    Stlxr,                            // Instruction 'stlxr'.
    Stlxrb,                           // Instruction 'stlxrb'.
    Stlxrh,                           // Instruction 'stlxrh'.
    Stnp,                             // Instruction 'stnp'.
    Stp,                              // Instruction 'stp'.
    Str,                              // Instruction 'str'.
    Strb,                             // Instruction 'strb'.
    Strh,                             // Instruction 'strh'.
    Stset,                            // Instruction 'stset'.
    Stsetl,                           // Instruction 'stsetl'.
    Stsetb,                           // Instruction 'stsetb'.
    Stsetlb,                          // Instruction 'stsetlb'.
    Stseth,                           // Instruction 'stseth'.
    Stsetlh,                          // Instruction 'stsetlh'.
    Stsmax,                           // Instruction 'stsmax'.
    Stsmaxl,                          // Instruction 'stsmaxl'.
    Stsmaxb,                          // Instruction 'stsmaxb'.
    Stsmaxlb,                         // Instruction 'stsmaxlb'.
    Stsmaxh,                          // Instruction 'stsmaxh'.
    Stsmaxlh,                         // Instruction 'stsmaxlh'.
    Stsmin,                           // Instruction 'stsmin'.
    Stsminl,                          // Instruction 'stsminl'.
    Stsminb,                          // Instruction 'stsminb'.
    Stsminlb,                         // Instruction 'stsminlb'.
    Stsminh,                          // Instruction 'stsminh'.
    Stsminlh,                         // Instruction 'stsminlh'.
    Sttr,                             // Instruction 'sttr'.
    Sttrb,                            // Instruction 'sttrb'.
    Sttrh,                            // Instruction 'sttrh'.
    Stumax,                           // Instruction 'stumax'.
    Stumaxl,                          // Instruction 'stumaxl'.
    Stumaxb,                          // Instruction 'stumaxb'.
    Stumaxlb,                         // Instruction 'stumaxlb'.
    Stumaxh,                          // Instruction 'stumaxh'.
    Stumaxlh,                         // Instruction 'stumaxlh'.
    Stumin,                           // Instruction 'stumin'.
    Stuminl,                          // Instruction 'stuminl'.
    Stuminb,                          // Instruction 'stuminb'.
    Stuminlb,                         // Instruction 'stuminlb'.
    Stuminh,                          // Instruction 'stuminh'.
    Stuminlh,                         // Instruction 'stuminlh'.
    Stur,                             // Instruction 'stur'.
    Sturb,                            // Instruction 'sturb'.
    Sturh,                            // Instruction 'sturh'.
    Stxp,                             // Instruction 'stxp'.
    Stxr,                             // Instruction 'stxr'.
    Stxrb,                            // Instruction 'stxrb'.
    Stxrh,                            // Instruction 'stxrh'.
    Stz2g,                            // Instruction 'stz2g'.
    Stzg,                             // Instruction 'stzg'.
    Stzgm,                            // Instruction 'stzgm'.
    Sub,                              // Instruction 'sub'.
    Subg,                             // Instruction 'subg'.
    Subp,                             // Instruction 'subp'.
    Subps,                            // Instruction 'subps'.
    Subs,                             // Instruction 'subs'.
    Svc,                              // Instruction 'svc'.
    Swp,                              // Instruction 'swp'.
    Swpa,                             // Instruction 'swpa'.
    Swpab,                            // Instruction 'swpab'.
    Swpah,                            // Instruction 'swpah'.
    Swpal,                            // Instruction 'swpal'.
    Swpalb,                           // Instruction 'swpalb'.
    Swpalh,                           // Instruction 'swpalh'.
    Swpb,                             // Instruction 'swpb'.
    Swph,                             // Instruction 'swph'.
    Swpl,                             // Instruction 'swpl'.
    Swplb,                            // Instruction 'swplb'.
    Swplh,                            // Instruction 'swplh'.
    Sxtb,                             // Instruction 'sxtb'.
    Sxth,                             // Instruction 'sxth'.
    Sxtw,                             // Instruction 'sxtw'.
    Sys,                              // Instruction 'sys'.
    Tlbi,                             // Instruction 'tlbi'.
    Tst,                              // Instruction 'tst'.
    Tbnz,                             // Instruction 'tbnz'.
    Tbz,                              // Instruction 'tbz'.
    Ubfiz,                            // Instruction 'ubfiz'.
    Ubfm,                             // Instruction 'ubfm'.
    Ubfx,                             // Instruction 'ubfx'.
    Udf,                              // Instruction 'udf'.
    Udiv,                             // Instruction 'udiv'.
    Umaddl,                           // Instruction 'umaddl'.
    Umax,                             // Instruction 'umax'.
    Umin,                             // Instruction 'umin'.
    Umnegl,                           // Instruction 'umnegl'.
    Umull,                            // Instruction 'umull'.
    Umulh,                            // Instruction 'umulh'.
    Umsubl,                           // Instruction 'umsubl'.
    Uxtb,                             // Instruction 'uxtb'.
    Uxth,                             // Instruction 'uxth'.
    Wfe,                              // Instruction 'wfe'.
    Wfi,                              // Instruction 'wfi'.
    Xaflag,                           // Instruction 'xaflag'.
    Xpacd,                            // Instruction 'xpacd'.
    Xpaci,                            // Instruction 'xpaci'.
    Xpaclri,                          // Instruction 'xpaclri'.
    Yield,                            // Instruction 'yield'.
    Abs_v,                            // Instruction 'abs' {ASIMD}.
    Add_v,                            // Instruction 'add' {ASIMD}.
    Addhn_v,                          // Instruction 'addhn' {ASIMD}.
    Addhn2_v,                         // Instruction 'addhn2' {ASIMD}.
    Addp_v,                           // Instruction 'addp' {ASIMD}.
    Addv_v,                           // Instruction 'addv' {ASIMD}.
    Aesd_v,                           // Instruction 'aesd' {ASIMD}.
    Aese_v,                           // Instruction 'aese' {ASIMD}.
    Aesimc_v,                         // Instruction 'aesimc' {ASIMD}.
    Aesmc_v,                          // Instruction 'aesmc' {ASIMD}.
    And_v,                            // Instruction 'and' {ASIMD}.
    Bcax_v,                           // Instruction 'bcax' {ASIMD}.
    Bfcvt_v,                          // Instruction 'bfcvt' {ASIMD}.
    Bfcvtn_v,                         // Instruction 'bfcvtn' {ASIMD}.
    Bfcvtn2_v,                        // Instruction 'bfcvtn2' {ASIMD}.
    Bfdot_v,                          // Instruction 'bfdot' {ASIMD}.
    Bfmlalb_v,                        // Instruction 'bfmlalb' {ASIMD}.
    Bfmlalt_v,                        // Instruction 'bfmlalt' {ASIMD}.
    Bfmmla_v,                         // Instruction 'bfmmla' {ASIMD}.
    Bic_v,                            // Instruction 'bic' {ASIMD}.
    Bif_v,                            // Instruction 'bif' {ASIMD}.
    Bit_v,                            // Instruction 'bit' {ASIMD}.
    Bsl_v,                            // Instruction 'bsl' {ASIMD}.
    Cls_v,                            // Instruction 'cls' {ASIMD}.
    Clz_v,                            // Instruction 'clz' {ASIMD}.
    Cmeq_v,                           // Instruction 'cmeq' {ASIMD}.
    Cmge_v,                           // Instruction 'cmge' {ASIMD}.
    Cmgt_v,                           // Instruction 'cmgt' {ASIMD}.
    Cmhi_v,                           // Instruction 'cmhi' {ASIMD}.
    Cmhs_v,                           // Instruction 'cmhs' {ASIMD}.
    Cmle_v,                           // Instruction 'cmle' {ASIMD}.
    Cmlt_v,                           // Instruction 'cmlt' {ASIMD}.
    Cmtst_v,                          // Instruction 'cmtst' {ASIMD}.
    Cnt_v,                            // Instruction 'cnt' {ASIMD}.
    Dup_v,                            // Instruction 'dup' {ASIMD}.
    Eor_v,                            // Instruction 'eor' {ASIMD}.
    Eor3_v,                           // Instruction 'eor3' {ASIMD}.
    Ext_v,                            // Instruction 'ext' {ASIMD}.
    Fabd_v,                           // Instruction 'fabd' {ASIMD}.
    Fabs_v,                           // Instruction 'fabs' {ASIMD}.
    Facge_v,                          // Instruction 'facge' {ASIMD}.
    Facgt_v,                          // Instruction 'facgt' {ASIMD}.
    Fadd_v,                           // Instruction 'fadd' {ASIMD}.
    Faddp_v,                          // Instruction 'faddp' {ASIMD}.
    Fcadd_v,                          // Instruction 'fcadd' {ASIMD}.
    Fccmp_v,                          // Instruction 'fccmp' {ASIMD}.
    Fccmpe_v,                         // Instruction 'fccmpe' {ASIMD}.
    Fcmeq_v,                          // Instruction 'fcmeq' {ASIMD}.
    Fcmge_v,                          // Instruction 'fcmge' {ASIMD}.
    Fcmgt_v,                          // Instruction 'fcmgt' {ASIMD}.
    Fcmla_v,                          // Instruction 'fcmla' {ASIMD}.
    Fcmle_v,                          // Instruction 'fcmle' {ASIMD}.
    Fcmlt_v,                          // Instruction 'fcmlt' {ASIMD}.
    Fcmp_v,                           // Instruction 'fcmp' {ASIMD}.
    Fcmpe_v,                          // Instruction 'fcmpe' {ASIMD}.
    Fcsel_v,                          // Instruction 'fcsel' {ASIMD}.
    Fcvt_v,                           // Instruction 'fcvt' {ASIMD}.
    Fcvtas_v,                         // Instruction 'fcvtas' {ASIMD}.
    Fcvtau_v,                         // Instruction 'fcvtau' {ASIMD}.
    Fcvtl_v,                          // Instruction 'fcvtl' {ASIMD}.
    Fcvtl2_v,                         // Instruction 'fcvtl2' {ASIMD}.
    Fcvtms_v,                         // Instruction 'fcvtms' {ASIMD}.
    Fcvtmu_v,                         // Instruction 'fcvtmu' {ASIMD}.
    Fcvtn_v,                          // Instruction 'fcvtn' {ASIMD}.
    Fcvtn2_v,                         // Instruction 'fcvtn2' {ASIMD}.
    Fcvtns_v,                         // Instruction 'fcvtns' {ASIMD}.
    Fcvtnu_v,                         // Instruction 'fcvtnu' {ASIMD}.
    Fcvtps_v,                         // Instruction 'fcvtps' {ASIMD}.
    Fcvtpu_v,                         // Instruction 'fcvtpu' {ASIMD}.
    Fcvtxn_v,                         // Instruction 'fcvtxn' {ASIMD}.
    Fcvtxn2_v,                        // Instruction 'fcvtxn2' {ASIMD}.
    Fcvtzs_v,                         // Instruction 'fcvtzs' {ASIMD}.
    Fcvtzu_v,                         // Instruction 'fcvtzu' {ASIMD}.
    Fdiv_v,                           // Instruction 'fdiv' {ASIMD}.
    Fjcvtzs_v,                        // Instruction 'fjcvtzs' {ASIMD}.
    Fmadd_v,                          // Instruction 'fmadd' {ASIMD}.
    Fmax_v,                           // Instruction 'fmax' {ASIMD}.
    Fmaxnm_v,                         // Instruction 'fmaxnm' {ASIMD}.
    Fmaxnmp_v,                        // Instruction 'fmaxnmp' {ASIMD}.
    Fmaxnmv_v,                        // Instruction 'fmaxnmv' {ASIMD}.
    Fmaxp_v,                          // Instruction 'fmaxp' {ASIMD}.
    Fmaxv_v,                          // Instruction 'fmaxv' {ASIMD}.
    Fmin_v,                           // Instruction 'fmin' {ASIMD}.
    Fminnm_v,                         // Instruction 'fminnm' {ASIMD}.
    Fminnmp_v,                        // Instruction 'fminnmp' {ASIMD}.
    Fminnmv_v,                        // Instruction 'fminnmv' {ASIMD}.
    Fminp_v,                          // Instruction 'fminp' {ASIMD}.
    Fminv_v,                          // Instruction 'fminv' {ASIMD}.
    Fmla_v,                           // Instruction 'fmla' {ASIMD}.
    Fmlal_v,                          // Instruction 'fmlal' {ASIMD}.
    Fmlal2_v,                         // Instruction 'fmlal2' {ASIMD}.
    Fmls_v,                           // Instruction 'fmls' {ASIMD}.
    Fmlsl_v,                          // Instruction 'fmlsl' {ASIMD}.
    Fmlsl2_v,                         // Instruction 'fmlsl2' {ASIMD}.
    Fmov_v,                           // Instruction 'fmov' {ASIMD}.
    Fmsub_v,                          // Instruction 'fmsub' {ASIMD}.
    Fmul_v,                           // Instruction 'fmul' {ASIMD}.
    Fmulx_v,                          // Instruction 'fmulx' {ASIMD}.
    Fneg_v,                           // Instruction 'fneg' {ASIMD}.
    Fnmadd_v,                         // Instruction 'fnmadd' {ASIMD}.
    Fnmsub_v,                         // Instruction 'fnmsub' {ASIMD}.
    Fnmul_v,                          // Instruction 'fnmul' {ASIMD}.
    Frecpe_v,                         // Instruction 'frecpe' {ASIMD}.
    Frecps_v,                         // Instruction 'frecps' {ASIMD}.
    Frecpx_v,                         // Instruction 'frecpx' {ASIMD}.
    Frint32x_v,                       // Instruction 'frint32x' {ASIMD}.
    Frint32z_v,                       // Instruction 'frint32z' {ASIMD}.
    Frint64x_v,                       // Instruction 'frint64x' {ASIMD}.
    Frint64z_v,                       // Instruction 'frint64z' {ASIMD}.
    Frinta_v,                         // Instruction 'frinta' {ASIMD}.
    Frinti_v,                         // Instruction 'frinti' {ASIMD}.
    Frintm_v,                         // Instruction 'frintm' {ASIMD}.
    Frintn_v,                         // Instruction 'frintn' {ASIMD}.
    Frintp_v,                         // Instruction 'frintp' {ASIMD}.
    Frintx_v,                         // Instruction 'frintx' {ASIMD}.
    Frintz_v,                         // Instruction 'frintz' {ASIMD}.
    Frsqrte_v,                        // Instruction 'frsqrte' {ASIMD}.
    Frsqrts_v,                        // Instruction 'frsqrts' {ASIMD}.
    Fsqrt_v,                          // Instruction 'fsqrt' {ASIMD}.
    Fsub_v,                           // Instruction 'fsub' {ASIMD}.
    Ins_v,                            // Instruction 'ins' {ASIMD}.
    Ld1_v,                            // Instruction 'ld1' {ASIMD}.
    Ld1r_v,                           // Instruction 'ld1r' {ASIMD}.
    Ld2_v,                            // Instruction 'ld2' {ASIMD}.
    Ld2r_v,                           // Instruction 'ld2r' {ASIMD}.
    Ld3_v,                            // Instruction 'ld3' {ASIMD}.
    Ld3r_v,                           // Instruction 'ld3r' {ASIMD}.
    Ld4_v,                            // Instruction 'ld4' {ASIMD}.
    Ld4r_v,                           // Instruction 'ld4r' {ASIMD}.
    Ldnp_v,                           // Instruction 'ldnp' {ASIMD}.
    Ldp_v,                            // Instruction 'ldp' {ASIMD}.
    Ldr_v,                            // Instruction 'ldr' {ASIMD}.
    Ldur_v,                           // Instruction 'ldur' {ASIMD}.
    Mla_v,                            // Instruction 'mla' {ASIMD}.
    Mls_v,                            // Instruction 'mls' {ASIMD}.
    Mov_v,                            // Instruction 'mov' {ASIMD}.
    Movi_v,                           // Instruction 'movi' {ASIMD}.
    Mul_v,                            // Instruction 'mul' {ASIMD}.
    Mvn_v,                            // Instruction 'mvn' {ASIMD}.
    Mvni_v,                           // Instruction 'mvni' {ASIMD}.
    Neg_v,                            // Instruction 'neg' {ASIMD}.
    Not_v,                            // Instruction 'not' {ASIMD}.
    Orn_v,                            // Instruction 'orn' {ASIMD}.
    Orr_v,                            // Instruction 'orr' {ASIMD}.
    Pmul_v,                           // Instruction 'pmul' {ASIMD}.
    Pmull_v,                          // Instruction 'pmull' {ASIMD}.
    Pmull2_v,                         // Instruction 'pmull2' {ASIMD}.
    Raddhn_v,                         // Instruction 'raddhn' {ASIMD}.
    Raddhn2_v,                        // Instruction 'raddhn2' {ASIMD}.
    Rax1_v,                           // Instruction 'rax1' {ASIMD}.
    Rbit_v,                           // Instruction 'rbit' {ASIMD}.
    Rev16_v,                          // Instruction 'rev16' {ASIMD}.
    Rev32_v,                          // Instruction 'rev32' {ASIMD}.
    Rev64_v,                          // Instruction 'rev64' {ASIMD}.
    Rshrn_v,                          // Instruction 'rshrn' {ASIMD}.
    Rshrn2_v,                         // Instruction 'rshrn2' {ASIMD}.
    Rsubhn_v,                         // Instruction 'rsubhn' {ASIMD}.
    Rsubhn2_v,                        // Instruction 'rsubhn2' {ASIMD}.
    Saba_v,                           // Instruction 'saba' {ASIMD}.
    Sabal_v,                          // Instruction 'sabal' {ASIMD}.
    Sabal2_v,                         // Instruction 'sabal2' {ASIMD}.
    Sabd_v,                           // Instruction 'sabd' {ASIMD}.
    Sabdl_v,                          // Instruction 'sabdl' {ASIMD}.
    Sabdl2_v,                         // Instruction 'sabdl2' {ASIMD}.
    Sadalp_v,                         // Instruction 'sadalp' {ASIMD}.
    Saddl_v,                          // Instruction 'saddl' {ASIMD}.
    Saddl2_v,                         // Instruction 'saddl2' {ASIMD}.
    Saddlp_v,                         // Instruction 'saddlp' {ASIMD}.
    Saddlv_v,                         // Instruction 'saddlv' {ASIMD}.
    Saddw_v,                          // Instruction 'saddw' {ASIMD}.
    Saddw2_v,                         // Instruction 'saddw2' {ASIMD}.
    Scvtf_v,                          // Instruction 'scvtf' {ASIMD}.
    Sdot_v,                           // Instruction 'sdot' {ASIMD}.
    Sha1c_v,                          // Instruction 'sha1c' {ASIMD}.
    Sha1h_v,                          // Instruction 'sha1h' {ASIMD}.
    Sha1m_v,                          // Instruction 'sha1m' {ASIMD}.
    Sha1p_v,                          // Instruction 'sha1p' {ASIMD}.
    Sha1su0_v,                        // Instruction 'sha1su0' {ASIMD}.
    Sha1su1_v,                        // Instruction 'sha1su1' {ASIMD}.
    Sha256h_v,                        // Instruction 'sha256h' {ASIMD}.
    Sha256h2_v,                       // Instruction 'sha256h2' {ASIMD}.
    Sha256su0_v,                      // Instruction 'sha256su0' {ASIMD}.
    Sha256su1_v,                      // Instruction 'sha256su1' {ASIMD}.
    Sha512h_v,                        // Instruction 'sha512h' {ASIMD}.
    Sha512h2_v,                       // Instruction 'sha512h2' {ASIMD}.
    Sha512su0_v,                      // Instruction 'sha512su0' {ASIMD}.
    Sha512su1_v,                      // Instruction 'sha512su1' {ASIMD}.
    Shadd_v,                          // Instruction 'shadd' {ASIMD}.
    Shl_v,                            // Instruction 'shl' {ASIMD}.
    Shll_v,                           // Instruction 'shll' {ASIMD}.
    Shll2_v,                          // Instruction 'shll2' {ASIMD}.
    Shrn_v,                           // Instruction 'shrn' {ASIMD}.
    Shrn2_v,                          // Instruction 'shrn2' {ASIMD}.
    Shsub_v,                          // Instruction 'shsub' {ASIMD}.
    Sli_v,                            // Instruction 'sli' {ASIMD}.
    Sm3partw1_v,                      // Instruction 'sm3partw1' {ASIMD}.
    Sm3partw2_v,                      // Instruction 'sm3partw2' {ASIMD}.
    Sm3ss1_v,                         // Instruction 'sm3ss1' {ASIMD}.
    Sm3tt1a_v,                        // Instruction 'sm3tt1a' {ASIMD}.
    Sm3tt1b_v,                        // Instruction 'sm3tt1b' {ASIMD}.
    Sm3tt2a_v,                        // Instruction 'sm3tt2a' {ASIMD}.
    Sm3tt2b_v,                        // Instruction 'sm3tt2b' {ASIMD}.
    Sm4e_v,                           // Instruction 'sm4e' {ASIMD}.
    Sm4ekey_v,                        // Instruction 'sm4ekey' {ASIMD}.
    Smax_v,                           // Instruction 'smax' {ASIMD}.
    Smaxp_v,                          // Instruction 'smaxp' {ASIMD}.
    Smaxv_v,                          // Instruction 'smaxv' {ASIMD}.
    Smin_v,                           // Instruction 'smin' {ASIMD}.
    Sminp_v,                          // Instruction 'sminp' {ASIMD}.
    Sminv_v,                          // Instruction 'sminv' {ASIMD}.
    Smlal_v,                          // Instruction 'smlal' {ASIMD}.
    Smlal2_v,                         // Instruction 'smlal2' {ASIMD}.
    Smlsl_v,                          // Instruction 'smlsl' {ASIMD}.
    Smlsl2_v,                         // Instruction 'smlsl2' {ASIMD}.
    Smmla_v,                          // Instruction 'smmla' {ASIMD}.
    Smov_v,                           // Instruction 'smov' {ASIMD}.
    Smull_v,                          // Instruction 'smull' {ASIMD}.
    Smull2_v,                         // Instruction 'smull2' {ASIMD}.
    Sqabs_v,                          // Instruction 'sqabs' {ASIMD}.
    Sqadd_v,                          // Instruction 'sqadd' {ASIMD}.
    Sqdmlal_v,                        // Instruction 'sqdmlal' {ASIMD}.
    Sqdmlal2_v,                       // Instruction 'sqdmlal2' {ASIMD}.
    Sqdmlsl_v,                        // Instruction 'sqdmlsl' {ASIMD}.
    Sqdmlsl2_v,                       // Instruction 'sqdmlsl2' {ASIMD}.
    Sqdmulh_v,                        // Instruction 'sqdmulh' {ASIMD}.
    Sqdmull_v,                        // Instruction 'sqdmull' {ASIMD}.
    Sqdmull2_v,                       // Instruction 'sqdmull2' {ASIMD}.
    Sqneg_v,                          // Instruction 'sqneg' {ASIMD}.
    Sqrdmlah_v,                       // Instruction 'sqrdmlah' {ASIMD}.
    Sqrdmlsh_v,                       // Instruction 'sqrdmlsh' {ASIMD}.
    Sqrdmulh_v,                       // Instruction 'sqrdmulh' {ASIMD}.
    Sqrshl_v,                         // Instruction 'sqrshl' {ASIMD}.
    Sqrshrn_v,                        // Instruction 'sqrshrn' {ASIMD}.
    Sqrshrn2_v,                       // Instruction 'sqrshrn2' {ASIMD}.
    Sqrshrun_v,                       // Instruction 'sqrshrun' {ASIMD}.
    Sqrshrun2_v,                      // Instruction 'sqrshrun2' {ASIMD}.
    Sqshl_v,                          // Instruction 'sqshl' {ASIMD}.
    Sqshlu_v,                         // Instruction 'sqshlu' {ASIMD}.
    Sqshrn_v,                         // Instruction 'sqshrn' {ASIMD}.
    Sqshrn2_v,                        // Instruction 'sqshrn2' {ASIMD}.
    Sqshrun_v,                        // Instruction 'sqshrun' {ASIMD}.
    Sqshrun2_v,                       // Instruction 'sqshrun2' {ASIMD}.
    Sqsub_v,                          // Instruction 'sqsub' {ASIMD}.
    Sqxtn_v,                          // Instruction 'sqxtn' {ASIMD}.
    Sqxtn2_v,                         // Instruction 'sqxtn2' {ASIMD}.
    Sqxtun_v,                         // Instruction 'sqxtun' {ASIMD}.
    Sqxtun2_v,                        // Instruction 'sqxtun2' {ASIMD}.
    Srhadd_v,                         // Instruction 'srhadd' {ASIMD}.
    Sri_v,                            // Instruction 'sri' {ASIMD}.
    Srshl_v,                          // Instruction 'srshl' {ASIMD}.
    Srshr_v,                          // Instruction 'srshr' {ASIMD}.
    Srsra_v,                          // Instruction 'srsra' {ASIMD}.
    Sshl_v,                           // Instruction 'sshl' {ASIMD}.
    Sshll_v,                          // Instruction 'sshll' {ASIMD}.
    Sshll2_v,                         // Instruction 'sshll2' {ASIMD}.
    Sshr_v,                           // Instruction 'sshr' {ASIMD}.
    Ssra_v,                           // Instruction 'ssra' {ASIMD}.
    Ssubl_v,                          // Instruction 'ssubl' {ASIMD}.
    Ssubl2_v,                         // Instruction 'ssubl2' {ASIMD}.
    Ssubw_v,                          // Instruction 'ssubw' {ASIMD}.
    Ssubw2_v,                         // Instruction 'ssubw2' {ASIMD}.
    St1_v,                            // Instruction 'st1' {ASIMD}.
    St2_v,                            // Instruction 'st2' {ASIMD}.
    St3_v,                            // Instruction 'st3' {ASIMD}.
    St4_v,                            // Instruction 'st4' {ASIMD}.
    Stnp_v,                           // Instruction 'stnp' {ASIMD}.
    Stp_v,                            // Instruction 'stp' {ASIMD}.
    Str_v,                            // Instruction 'str' {ASIMD}.
    Stur_v,                           // Instruction 'stur' {ASIMD}.
    Sub_v,                            // Instruction 'sub' {ASIMD}.
    Subhn_v,                          // Instruction 'subhn' {ASIMD}.
    Subhn2_v,                         // Instruction 'subhn2' {ASIMD}.
    Sudot_v,                          // Instruction 'sudot' {ASIMD}.
    Suqadd_v,                         // Instruction 'suqadd' {ASIMD}.
    Sxtl_v,                           // Instruction 'sxtl' {ASIMD}.
    Sxtl2_v,                          // Instruction 'sxtl2' {ASIMD}.
    Tbl_v,                            // Instruction 'tbl' {ASIMD}.
    Tbx_v,                            // Instruction 'tbx' {ASIMD}.
    Trn1_v,                           // Instruction 'trn1' {ASIMD}.
    Trn2_v,                           // Instruction 'trn2' {ASIMD}.
    Uaba_v,                           // Instruction 'uaba' {ASIMD}.
    Uabal_v,                          // Instruction 'uabal' {ASIMD}.
    Uabal2_v,                         // Instruction 'uabal2' {ASIMD}.
    Uabd_v,                           // Instruction 'uabd' {ASIMD}.
    Uabdl_v,                          // Instruction 'uabdl' {ASIMD}.
    Uabdl2_v,                         // Instruction 'uabdl2' {ASIMD}.
    Uadalp_v,                         // Instruction 'uadalp' {ASIMD}.
    Uaddl_v,                          // Instruction 'uaddl' {ASIMD}.
    Uaddl2_v,                         // Instruction 'uaddl2' {ASIMD}.
    Uaddlp_v,                         // Instruction 'uaddlp' {ASIMD}.
    Uaddlv_v,                         // Instruction 'uaddlv' {ASIMD}.
    Uaddw_v,                          // Instruction 'uaddw' {ASIMD}.
    Uaddw2_v,                         // Instruction 'uaddw2' {ASIMD}.
    Ucvtf_v,                          // Instruction 'ucvtf' {ASIMD}.
    Udot_v,                           // Instruction 'udot' {ASIMD}.
    Uhadd_v,                          // Instruction 'uhadd' {ASIMD}.
    Uhsub_v,                          // Instruction 'uhsub' {ASIMD}.
    Umax_v,                           // Instruction 'umax' {ASIMD}.
    Umaxp_v,                          // Instruction 'umaxp' {ASIMD}.
    Umaxv_v,                          // Instruction 'umaxv' {ASIMD}.
    Umin_v,                           // Instruction 'umin' {ASIMD}.
    Uminp_v,                          // Instruction 'uminp' {ASIMD}.
    Uminv_v,                          // Instruction 'uminv' {ASIMD}.
    Umlal_v,                          // Instruction 'umlal' {ASIMD}.
    Umlal2_v,                         // Instruction 'umlal2' {ASIMD}.
    Umlsl_v,                          // Instruction 'umlsl' {ASIMD}.
    Umlsl2_v,                         // Instruction 'umlsl2' {ASIMD}.
    Ummla_v,                          // Instruction 'ummla' {ASIMD}.
    Umov_v,                           // Instruction 'umov' {ASIMD}.
    Umull_v,                          // Instruction 'umull' {ASIMD}.
    Umull2_v,                         // Instruction 'umull2' {ASIMD}.
    Uqadd_v,                          // Instruction 'uqadd' {ASIMD}.
    Uqrshl_v,                         // Instruction 'uqrshl' {ASIMD}.
    Uqrshrn_v,                        // Instruction 'uqrshrn' {ASIMD}.
    Uqrshrn2_v,                       // Instruction 'uqrshrn2' {ASIMD}.
    Uqshl_v,                          // Instruction 'uqshl' {ASIMD}.
    Uqshrn_v,                         // Instruction 'uqshrn' {ASIMD}.
    Uqshrn2_v,                        // Instruction 'uqshrn2' {ASIMD}.
    Uqsub_v,                          // Instruction 'uqsub' {ASIMD}.
    Uqxtn_v,                          // Instruction 'uqxtn' {ASIMD}.
    Uqxtn2_v,                         // Instruction 'uqxtn2' {ASIMD}.
    Urecpe_v,                         // Instruction 'urecpe' {ASIMD}.
    Urhadd_v,                         // Instruction 'urhadd' {ASIMD}.
    Urshl_v,                          // Instruction 'urshl' {ASIMD}.
    Urshr_v,                          // Instruction 'urshr' {ASIMD}.
    Ursqrte_v,                        // Instruction 'ursqrte' {ASIMD}.
    Ursra_v,                          // Instruction 'ursra' {ASIMD}.
    Usdot_v,                          // Instruction 'usdot' {ASIMD}.
    Ushl_v,                           // Instruction 'ushl' {ASIMD}.
    Ushll_v,                          // Instruction 'ushll' {ASIMD}.
    Ushll2_v,                         // Instruction 'ushll2' {ASIMD}.
    Ushr_v,                           // Instruction 'ushr' {ASIMD}.
    Usmmla_v,                         // Instruction 'usmmla' {ASIMD}.
    Usqadd_v,                         // Instruction 'usqadd' {ASIMD}.
    Usra_v,                           // Instruction 'usra' {ASIMD}.
    Usubl_v,                          // Instruction 'usubl' {ASIMD}.
    Usubl2_v,                         // Instruction 'usubl2' {ASIMD}.
    Usubw_v,                          // Instruction 'usubw' {ASIMD}.
    Usubw2_v,                         // Instruction 'usubw2' {ASIMD}.
    Uxtl_v,                           // Instruction 'uxtl' {ASIMD}.
    Uxtl2_v,                          // Instruction 'uxtl2' {ASIMD}.
    Uzp1_v,                           // Instruction 'uzp1' {ASIMD}.
    Uzp2_v,                           // Instruction 'uzp2' {ASIMD}.
    Xar_v,                            // Instruction 'xar' {ASIMD}.
    Xtn_v,                            // Instruction 'xtn' {ASIMD}.
    Xtn2_v,                           // Instruction 'xtn2' {ASIMD}.
    Zip1_v,                           // Instruction 'zip1' {ASIMD}.
    Zip2_v,                           // Instruction 'zip2' {ASIMD}.
    _Count
}
