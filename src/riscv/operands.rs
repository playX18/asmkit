
//! RISC-V Operands definition. 
use crate::{
    core::{
        arch_traits::{Arch, ArchTraits},
        operand::*,
        types::TypeId,
    },
    define_abstract_reg, define_final_reg, define_operand_cast, define_reg_traits,
};

use derive_more::derive::{Deref, DerefMut};

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Reg(pub BaseReg);

define_abstract_reg!(Reg, BaseReg);

define_reg_traits!(RISCV32Gp, RegGroup::Gp, 32, TypeId::Int32);
define_reg_traits!(RISCV64Gp, RegGroup::Gp, 64, TypeId::Int64);
define_reg_traits!(RISCVPC, RegGroup::PC, 64, TypeId::Int64);
define_reg_traits!(RISCVVec, RegGroup::Vec, 128, TypeId::Int32x4);
define_reg_traits!(RISCVFp, RegGroup::Vec, 64, TypeId::Float64);

impl Reg {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::RISCV64).reg_type_to_signature(typ)
    }

    pub fn is_gp(&self) -> bool {
        self.is_reg_group_of(RegGroup::Gp)
    }

    pub fn is_fp(&self) -> bool {
        self.has_base_signature(RISCVFp::SIGNATURE)
    }

    pub fn is_vec(&self) -> bool {
        self.has_base_signature(RISCVFp::SIGNATURE)
    }

    pub fn set_reg_t<T: RegTraits>(&mut self, rid: u32) {
        self.set_signature(T::SIGNATURE.into());
        self.set_id(rid);
    }

    pub fn set_type_and_id(&mut self, typ: RegType, id: u32) {
        self.set_signature(Self::signature_of(typ));
        self.set_id(id);
    }
    pub const SIGNATURE: u32 = BaseReg::SIGNATURE;
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Gp(pub Reg);

impl Gp {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::RISCV64).reg_type_to_signature(typ)
    }

    pub const X0: u32 = 0;
    pub const X1: u32 = 1;
    pub const X2: u32 = 2;
    pub const X3: u32 = 3;
    pub const X4: u32 = 4;
    pub const X5: u32 = 5;
    pub const X6: u32 = 6;
    pub const X7: u32 = 7;
    pub const X8: u32 = 8;
    pub const X9: u32 = 9;
    pub const X10: u32 = 10;
    pub const X11: u32 = 11;
    pub const X12: u32 = 12;
    pub const X13: u32 = 13;
    pub const X14: u32 = 14;
    pub const X15: u32 = 15;
    pub const X16: u32 = 16;
    pub const X17: u32 = 17;
    pub const X18: u32 = 18;
    pub const X19: u32 = 19;
    pub const X20: u32 = 20;
    pub const X21: u32 = 21;
    pub const X22: u32 = 22;
    pub const X23: u32 = 23;
    pub const X24: u32 = 24;
    pub const X25: u32 = 25;
    pub const X26: u32 = 26;
    pub const X27: u32 = 27;
    pub const X28: u32 = 28;
    pub const X29: u32 = 29;
    pub const X30: u32 = 30;
    pub const X31: u32 = 31;
}

define_final_reg!(Gp, Reg, RISCV64Gp);

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Fp(pub Reg);

define_final_reg!(Fp, Reg, RISCVFp);

impl Fp {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::RISCV64).reg_type_to_signature(typ)
    }

    pub const F0: u32 = 0;
    pub const F1: u32 = 1;
    pub const F2: u32 = 2;
    pub const F3: u32 = 3;
    pub const F4: u32 = 4;
    pub const F5: u32 = 5;
    pub const F6: u32 = 6;
    pub const F7: u32 = 7;
    pub const F8: u32 = 8;
    pub const F9: u32 = 9;
    pub const F10: u32 = 10;
    pub const F11: u32 = 11;
    pub const F12: u32 = 12;
    pub const F13: u32 = 13;
    pub const F14: u32 = 14;
    pub const F15: u32 = 15;
    pub const F16: u32 = 16;
    pub const F17: u32 = 17;
    pub const F18: u32 = 18;
    pub const F19: u32 = 19;
    pub const F20: u32 = 20;
    pub const F21: u32 = 21;
    pub const F22: u32 = 22;
    pub const F23: u32 = 23;
    pub const F24: u32 = 24;
    pub const F25: u32 = 25;
    pub const F26: u32 = 26;
    pub const F27: u32 = 27;
    pub const F28: u32 = 28;
    pub const F29: u32 = 29;
    pub const F30: u32 = 30;
    pub const F31: u32 = 31;
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Vp(pub Reg);

define_final_reg!(Vp, Reg, RISCVVec);

impl Vp {
    pub const fn signature_of(typ: RegType) -> OperandSignature {
        ArchTraits::by_arch(Arch::RISCV64).reg_type_to_signature(typ)
    }

    pub const V0: u32 = 0;
    pub const V1: u32 = 1;
    pub const V2: u32 = 2;
    pub const V3: u32 = 3;
    pub const V4: u32 = 4;
    pub const V5: u32 = 5;
    pub const V6: u32 = 6;
    pub const V7: u32 = 7;
    pub const V8: u32 = 8;
    pub const V9: u32 = 9;
    pub const V10: u32 = 10;
    pub const V11: u32 = 11;
    pub const V12: u32 = 12;
    pub const V13: u32 = 13;
    pub const V14: u32 = 14;
    pub const V15: u32 = 15;
    pub const V16: u32 = 16;
    pub const V17: u32 = 17;
    pub const V18: u32 = 18;
    pub const V19: u32 = 19;
    pub const V20: u32 = 20;
    pub const V21: u32 = 21;
    pub const V22: u32 = 22;
    pub const V23: u32 = 23;
    pub const V24: u32 = 24;
    pub const V25: u32 = 25;
    pub const V26: u32 = 26;
    pub const V27: u32 = 27;
    pub const V28: u32 = 28;
    pub const V29: u32 = 29;
    pub const V30: u32 = 30;
    pub const V31: u32 = 31;
}

pub mod regs {
    pub use super::*;

    pub const fn x(id: u32) -> Gp {
        Gp::from_id(id)
    }

    pub const fn f(id: u32) -> Fp {
        Fp::from_id(id)
    }

    pub const fn v(id: u32) -> Vp {
        Vp::from_id(id)
    }

    pub const X0: Gp = x(0);
    pub const X1: Gp = x(1);
    pub const X2: Gp = x(2);
    pub const X3: Gp = x(3);
    pub const X4: Gp = x(4);
    pub const X5: Gp = x(5);
    pub const X6: Gp = x(6);
    pub const X7: Gp = x(7);
    pub const X8: Gp = x(8);
    pub const X9: Gp = x(9);
    pub const X10: Gp = x(10);
    pub const X11: Gp = x(11);
    pub const X12: Gp = x(12);
    pub const X13: Gp = x(13);
    pub const X14: Gp = x(14);
    pub const X15: Gp = x(15);
    pub const X16: Gp = x(16);
    pub const X17: Gp = x(17);
    pub const X18: Gp = x(18);
    pub const X19: Gp = x(19);
    pub const X20: Gp = x(20);
    pub const X21: Gp = x(21);
    pub const X22: Gp = x(22);
    pub const X23: Gp = x(23);
    pub const X24: Gp = x(24);
    pub const X25: Gp = x(25);
    pub const X26: Gp = x(26);
    pub const X27: Gp = x(27);
    pub const X28: Gp = x(28);
    pub const X29: Gp = x(29);
    pub const X30: Gp = x(30);
    pub const X31: Gp = x(31);

    pub const ZERO: Gp = X0;
    pub const RA: Gp = X1;
    pub const SP: Gp = X2;
    pub const GP: Gp = X3;
    pub const TP: Gp = X4;
    pub const T0: Gp = X5;
    pub const T1: Gp = X6;
    pub const T2: Gp = X7;
    pub const S0: Gp = X8;
    pub const S1: Gp = X9;
    pub const A0: Gp = X10;
    pub const A1: Gp = X11;
    pub const A2: Gp = X12;
    pub const A3: Gp = X13;
    pub const A4: Gp = X14;
    pub const A5: Gp = X15;
    pub const A6: Gp = X16;
    pub const A7: Gp = X17;
    pub const S2: Gp = X18;
    pub const S3: Gp = X19;
    pub const S4: Gp = X20;
    pub const S5: Gp = X21;
    pub const S6: Gp = X22;
    pub const S7: Gp = X23;
    pub const S8: Gp = X24;
    pub const S9: Gp = X25;
    pub const S10: Gp = X26;
    pub const S11: Gp = X27;
    pub const T3: Gp = X28;
    pub const T4: Gp = X29;
    pub const T5: Gp = X30;
    pub const T6: Gp = X31;

    pub const F0: Fp = f(0);
    pub const F1: Fp = f(1);
    pub const F2: Fp = f(2);
    pub const F3: Fp = f(3);
    pub const F4: Fp = f(4);
    pub const F5: Fp = f(5);
    pub const F6: Fp = f(6);
    pub const F7: Fp = f(7);
    pub const F8: Fp = f(8);
    pub const F9: Fp = f(9);
    pub const F10: Fp = f(10);
    pub const F11: Fp = f(11);
    pub const F12: Fp = f(12);
    pub const F13: Fp = f(13);
    pub const F14: Fp = f(14);
    pub const F15: Fp = f(15);
    pub const F16: Fp = f(16);
    pub const F17: Fp = f(17);
    pub const F18: Fp = f(18);
    pub const F19: Fp = f(19);
    pub const F20: Fp = f(20);
    pub const F21: Fp = f(21);
    pub const F22: Fp = f(22);
    pub const F23: Fp = f(23);
    pub const F24: Fp = f(24);
    pub const F25: Fp = f(25);
    pub const F26: Fp = f(26);
    pub const F27: Fp = f(27);
    pub const F28: Fp = f(28);
    pub const F29: Fp = f(29);
    pub const F30: Fp = f(30);
    pub const F31: Fp = f(31);

    pub const V0: Vp = v(0);
    pub const V1: Vp = v(1);
    pub const V2: Vp = v(2);
    pub const V3: Vp = v(3);
    pub const V4: Vp = v(4);
    pub const V5: Vp = v(5);
    pub const V6: Vp = v(6);
    pub const V7: Vp = v(7);
    pub const V8: Vp = v(8);
    pub const V9: Vp = v(9);
    pub const V10: Vp = v(10);
    pub const V11: Vp = v(11);
    pub const V12: Vp = v(12);
    pub const V13: Vp = v(13);
    pub const V14: Vp = v(14);
    pub const V15: Vp = v(15);
    pub const V16: Vp = v(16);
    pub const V17: Vp = v(17);
    pub const V18: Vp = v(18);
    pub const V19: Vp = v(19);
    pub const V20: Vp = v(20);
    pub const V21: Vp = v(21);
    pub const V22: Vp = v(22);
    pub const V23: Vp = v(23);
    pub const V24: Vp = v(24);
    pub const V25: Vp = v(25);
    pub const V26: Vp = v(26);
    pub const V27: Vp = v(27);
    pub const V28: Vp = v(28);
    pub const V29: Vp = v(29);
    pub const V30: Vp = v(30);
    pub const V31: Vp = v(31);
}
