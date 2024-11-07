//! Custom operands for X86 backend.

use super::assembler::RawX86Assembler;
use crate::encdec::x86::v2::{Gp, Xmm, MEM_LONG, NOREG};
use crate::{assembler::binemit::Label, assembler::codeholder::ExternalName};

type UMem = crate::encdec::x86::v2::Mem;
type UMemV = crate::encdec::x86::v2::MemV;
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum MemBase {
    Gp(Gp),
    Label(Label),
    Ext(ExternalName),
}

impl MemBase {
    pub fn to_gp(&self) -> Gp {
        match self {
            Self::Gp(gp) => *gp,
            _ => crate::encdec::x86::v2::IP,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Mem {
    pub base: MemBase,
    pub index: Gp,
    pub scale: u8,
    pub disp: i32,
}

impl Mem {
    pub fn into_umem(&self, asm: &mut RawX86Assembler) -> UMem {
        let mut flags = 0;
        let disp = self.disp;
        if matches!(self.base, MemBase::Label(_) | MemBase::Ext(_)) || asm.is_long() {
            // label in memory address is unbound:
            // 1) Emit "long" version of instruction
            // 2) force DISP to be emitted as 4-byte by marking MEM_LONG in flags.
            asm.long();
            flags |= MEM_LONG;
        }
        UMem {
            base: self.base.to_gp(),
            idx: self.index,
            scale: self.scale,
            off: disp,
            flags,
        }
    }

    pub fn is_label(&self) -> bool {
        matches!(self.base, MemBase::Label(_))
    }
}

pub const fn ptr(base: Gp, disp: i32) -> Mem {
    Mem {
        base: MemBase::Gp(base),
        index: NOREG,
        scale: 1,
        disp,
    }
}

pub const fn label_ptr(label: Label, disp: i32) -> Mem {
    Mem {
        base: MemBase::Label(label),
        index: NOREG,
        scale: 1,
        disp,
    }
}

pub const fn ptr_index(base: Gp, index: Gp, scale: u8, disp: i32) -> Mem {
    Mem {
        base: MemBase::Gp(base),
        index,
        scale,
        disp,
    }
}

pub const fn label_ptr_index(label: Label, index: Gp, scale: u8, disp: i32) -> Mem {
    Mem {
        base: MemBase::Label(label),
        index,
        scale,
        disp,
    }
}

pub const fn ptr_disp(disp: i32) -> Mem {
    Mem {
        base: MemBase::Gp(NOREG),
        index: NOREG,
        scale: 1,
        disp,
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct MemV {
    pub base: MemBase,
    pub index: Xmm,
    pub scale: u8,
    pub disp: i32,
}

impl MemV {
    pub fn into_umem(&self, asm: &mut RawX86Assembler) -> UMemV {
        let mut flags = 0;
        let disp = self.disp;
        if matches!(self.base, MemBase::Label(_) | MemBase::Ext(_)) || asm.is_long() {
            // label in memory address is unbound:
            // 1) Emit "long" version of instruction
            // 2) force DISP to be emitted as 4-byte by marking MEM_LONG in flags.
            asm.long();
            flags |= MEM_LONG;
        }
        UMemV {
            base: self.base.to_gp(),
            idx: self.index,
            scale: self.scale,
            off: disp,
            flags,
        }
    }

    pub fn is_label(&self) -> bool {
        matches!(self.base, MemBase::Label(_))
    }
}
