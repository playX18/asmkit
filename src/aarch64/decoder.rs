#![allow(unused_parens)]
use super::opcodes::*;
use derive_more::TryFrom;

#[derive(Copy, Clone, PartialEq, Eq, Debug, TryFrom,)]
#[repr(u8)]
#[try_from(repr)]
pub enum Cond {
    EQ = 0x0,
    NE = 0x1,

    HS = 0x2,

    LO = 0x3,
    MI = 0x4,
    PL = 0x5,
    VS = 0x6,
    VC = 0x7,
    HI = 0x8,
    LS = 0x9,
    GE = 0xa,
    LT = 0xb,
    GT = 0xc,
    LE = 0xd,
    AL = 0xe,
    NV = 0xf,
}

impl Cond {
    pub const CS: Cond = Cond::HS;
    pub const CC: Cond = Cond::LO;
}

#[derive(Copy, Clone, PartialEq, Eq, TryFrom, Debug)]
#[repr(u8)]
#[try_from(repr)]
pub enum Ext {
    Uxtb = 0,
    Uxth = 1,
    Uxtw = 2,
    Uxtx = 3,
    Sxtb = 4,
    Sxth = 5,
    Sxtw = 6,
    Sxtx = 7,
    Lsl = 8,
    Lsr = 9,
    Asr = 10,
    Ror = 11,
}

#[derive(Copy, Clone, PartialEq, Eq, TryFrom, Debug)]
#[repr(u8)]
#[try_from(repr)]
pub enum VectorArrangement {
    Va8b = 0,
    Va16b = 1,
    Va4h = 2,
    Va8h = 3,
    Va2s = 4,
    Va4s = 5,
    Va1d = 6,
    Va2d = 7,
    Va2h = 8,
    Va1q = 9,
}
#[derive(Copy, Clone, PartialEq, Eq, TryFrom, Debug)]
#[repr(u8)]
#[try_from(repr)]
pub enum OpType {
    None = 0,
    RegGp,
    RegGpInc,
    RegGpExt,
    RegSp,
    RegFp,
    RegVec,
    RegVtbl,
    RegVidx,
    RegVtblIdx,
    MemUoff,
    MemSoff,
    MemSoffPre,
    MemSoffPost,
    MemReg,
    MemRegPost,
    MemInc,
    Cond,
    Prfop,
    Sysreg,
    ImmSmall,
    Simm,
    Uimm,
    UimmShift,
    ImmLarge,
    ImmFloat,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Op {
    op_type: OpType,
    value: OpValue,
    detail: OpDetail,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum OpValue {
    Reg(u8),
    Prfop(u8),
    ImmShift { mask: bool, shift: u8 },
}

impl Default for Op {
    fn default() -> Self {
        Self {
            op_type: OpType::None,
            value: OpValue::Reg(0),
            detail: OpDetail::Gp { sf: false },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum OpDetail {
    Gp {
        sf: bool,
    },
    GpPExt {
        sf: bool,
        ext: Ext,
        shift: u8,
    },
    Fp {
        size: u8,
    },
    Vec {
        va: VectorArrangement,
    },
    Vidx {
        // 0=b, 1=h, 2=s, 3=d, (4=q), 5=2b, 6=4b, 7=2h
        esize: u8,
        elem: u8,
    },
    Vtbl {
        va: VectorArrangement,
        cnt: u8,
    },
    VtblIdx {
        esize: u8,
        elem: u8,
        cnt: u8,
    },
    MemReg {
        sc: u32,
        ext: Ext,
        shift: u8,
        offreg: u8,
    },
    Sysreg(u16),
    Uimm16(u16),
    Simm16(i16),
    Cond(Cond),
}

pub struct Inst {
    pub mnem: InstKind,
    pub ops: [Op; 5],
    pub imm: Imm,
}

impl Default for Inst {
    fn default() -> Self {
        Self {
            mnem: InstKind::Unknown,
            ops: [Op::default(); 5],
            imm: Imm {
                imm64: 0
            }
        }
    }
}

pub union Imm {
    imm64: u64,
    float8: f64,
}

pub fn classify(inst: u32) -> InstKind {
    InstKind::try_from(classify_impl(inst) as u16).unwrap()
}

fn ctz(v: u32) -> u32 {
    v.trailing_zeros() as _
}

fn clz(v: u32, sz: usize) -> u32 {
    if v != 0 {
        v.leading_zeros() + sz as u32 - 32
    } else {
        sz as _
    }
}

fn sext(imm: i32, bits: usize) -> i32 {
    let sign = 1 << (bits - 1);
    if imm & sign != 0 {
        ((imm ^ sign) - sign) as _
    } else {
        imm as _
    }
}

fn immlogical(sf: u32, n: u32, immr: u32, imms: u32) -> u32 {
    if (n == 0) && (imms == 0x3f) {
        return 0;
    }

    let len = 31 - (imms.count_ones() as u32);
    let levels = (1 << len) - 1;
    let s = imms & levels;
    let r = immr & levels;
    let esize = 1 << len;
    let mut welem = ((1 << (s + 1)) - 1) as u64;

    if r != 0 {
        welem = (welem >> r) | (welem << (esize - r));
    }

    if esize < 64 {
        welem &= ((1 << esize) - 1) as u64;
    }

    let mut wmask = 0 as u64;
    for i in (0..(!sf as u32 * 32)).step_by(esize as usize) {
        wmask |= welem << i;
    }

    wmask as u32
}

fn opreggp(idx: u32, sf: u32) -> Op {
    Op {
        op_type: OpType::RegGp,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Gp { sf: sf != 0 },
    }
}

fn opreggpinc(idx: u32) -> Op {
    Op {
        op_type: OpType::RegGpInc,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Gp { sf: true },
    }
}

fn opreggpsp(idx: u32, sf: u32) -> Op {
    Op {
        op_type: if idx != 31 {
            OpType::RegGp
        } else {
            OpType::RegSp
        },
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Gp { sf: sf != 0 },
    }
}

fn opreggpmaysp(maysp: u32, idx: u32, sf: u32) -> Op {
    if idx < 31 || maysp == 0 {
        Op {
            op_type: OpType::RegGp,
            value: OpValue::Reg(idx as u8),
            detail: OpDetail::Gp { sf: sf != 0 },
        }
    } else {
        Op {
            op_type: OpType::RegSp,
            value: OpValue::Reg(idx as u8),
            detail: OpDetail::Gp { sf: sf != 0 },
        }
    }
}

fn opreggpprf(isprf: u32, idx: u32, sf: u32) -> Op {
    Op {
        op_type: if isprf!= 0 { OpType::Prfop } else { OpType::RegGp },
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Gp { sf: sf!= 0 },
    }
    
}

fn opreggpext(idx: u32, sf: u32, ext: u32, shift: u32) -> Op {
    Op {
        op_type: OpType::RegGpExt,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::GpPExt {
            sf: sf!= 0,
            ext: Ext::try_from(ext as u8).unwrap(),
            shift: shift as u8,
        },
    }
    
}

fn opregfp(idx: u32, size: u32) -> Op {
    Op {
        op_type: OpType::RegFp,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Fp { size: size as u8 },
    }
    
}

fn opregvec(idx: u32, esize: u32, q: u32) -> Op {
    Op {
        op_type: OpType::RegVec,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Vec {
            va: VectorArrangement::try_from(((esize as u8) << 1) + q as u8).unwrap(),
        },
    }
    
}

fn opregvidx(idx: u32, esize: u32, elem: u32) -> Op {
    Op {
        op_type: OpType::RegVidx,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Vidx {
            esize: esize as u8,
            elem: elem as u8,
        },
    }
    
}

fn opregvtbl(idx: u32, esize: u32, q: u32, cnt: u32) -> Op {
    Op {
        op_type: OpType::RegVtbl,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Vtbl {
            va: VectorArrangement::try_from(((esize as u8) << 1) + q as u8).unwrap(),
            cnt: cnt as u8,
        },
    }
    
}

fn opregvtblidx(idx: u32, esize: u32, elem: u32, cnt: u32) -> Op {
    Op {
        op_type: OpType::RegVtblIdx,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::VtblIdx {
            esize: esize as u8,
            elem: elem as u8,
            cnt: cnt as u8,
        },
    }
}

pub fn opmemuoff(idx: u32, off: u32) -> Op {
    Op {
        op_type: OpType::MemUoff,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Uimm16(off as _),
    }
}

pub fn opmemsoff(idx: u32, off: i32) -> Op {
    Op {
        op_type: OpType::MemSoff,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Simm16(off as _),
    }
}

pub fn opmemsoffpre(idx: u32, off: i32) -> Op {
    Op {
        op_type: OpType::MemSoffPre,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Simm16(off as _),
    }
}

pub fn opmemsoffpost(idx: u32, off: i32) -> Op {
    Op {
        op_type: OpType::MemSoffPost,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Simm16(off as _),
    }
}

pub fn opmemreg(idx: u32, offreg: u32, ext: u32, scale: u32, shift: u32) -> Op {
    Op {
        op_type: OpType::MemReg,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::MemReg {
            sc: scale,
            ext: Ext::try_from(ext as u8).unwrap(),
            shift: shift as u8,
            offreg: offreg as u8,
        },
    }
}

pub fn opmemregsimdpost(idx: u32, offreg: u32, constoff: u32) -> Op {
    if offreg == 31 {
        opmemsoffpost(idx, constoff as _)
    } else {
        Op {
            op_type: OpType::MemRegPost,
            value: OpValue::Reg(idx as u8),
            detail: OpDetail::MemReg {
                sc: 0,
                ext: Ext::Uxtx,
                shift: 0,
                offreg: offreg as u8,
            },
        }
    }
}

pub fn opmeminc(idx: u32) -> Op {
    Op {
        op_type: OpType::MemInc,
        value: OpValue::Reg(idx as u8),
        detail: OpDetail::Uimm16(0),
    }
}

pub fn opimmsmall(imm6: u32) -> Op {
    Op {
        op_type: OpType::ImmSmall,
        value: OpValue::ImmShift {
            mask: false,
            shift: 0,
        },
        detail: OpDetail::Uimm16(imm6 as u16),
    }
}

pub fn opsimm(imm: i32) -> Op {
    Op {
        op_type: OpType::Simm,
        value: OpValue::ImmShift {
            mask: false,
            shift: 0,
        },
        detail: OpDetail::Simm16(imm as _),
    }
}

pub fn opuimm(imm: u32) -> Op {
    Op {
        op_type: OpType::Uimm,
        value: OpValue::ImmShift {
            mask: false,
            shift: 0,
        },
        detail: OpDetail::Uimm16(imm as _),
    }
}

pub fn opuimmshift(imm: u32, msl: u32, shift: u32) -> Op {
    Op {
        op_type: OpType::UimmShift,
        value: OpValue::ImmShift {
            mask: msl != 0,
            shift: shift as u8,
        },
        detail: OpDetail::Uimm16(imm as _),
    }
}

pub fn opreladdr(ddi: &mut Inst, imm: i32) -> Op {
    ddi.imm = Imm {
        imm64: imm as i32 as i64 as u64,
    };
    Op {
        op_type: OpType::ImmLarge,
        value: OpValue::ImmShift {
            mask: false,
            shift: 0,
        },
        detail: OpDetail::Uimm16(0),
    }
}

pub fn opimmlogical(ddi: &mut Inst, sf: u32, n: u32, immr: u32, imms: u32) -> Op {
    ddi.imm = Imm {
        imm64: immlogical(sf, n, immr, imms) as u64,
    };
    Op {
        op_type: OpType::ImmLarge,
        value: OpValue::ImmShift {
            mask: false,
            shift: 0,
        },
        detail: OpDetail::Uimm16(sf as u16),
    }
}

pub fn opimmsimdmask(ddi: &mut Inst, imm8: u32) -> Op {
    let mut res = 0u64;
    for i in 0..8 {
        if (imm8 & (1 << i)) != 0 {
            res |= 0xff << (i * 8);
        }
    }
    ddi.imm = Imm { imm64: res };
    Op {
        op_type: OpType::ImmLarge,
        value: OpValue::ImmShift {
            mask: false,
            shift: 0,
        },
        detail: OpDetail::Uimm16(1),
    }
}

pub fn opimmfloatzero(ddi: &mut Inst) -> Op {
    ddi.imm = Imm { float8: 0.0 };
    Op {
        op_type: OpType::ImmFloat,
        value: OpValue::ImmShift {
            mask: false,
            shift: 0,
        },
        detail: OpDetail::Uimm16(0x100),
    }
}

pub fn opimmfloat(ddi: &mut Inst, imm8: u32) -> Op {
    let res = (imm8 as u32 & 0x80) << 24
        | if (imm8 & 0x40) != 0 {
            0x3e000000
        } else {
            0x40000000
        }
        | ((imm8 & 0x3f) as u32) << 19;
    ddi.imm = Imm {
        float8: f64::from_bits(res as u64),
    };
    Op {
        op_type: OpType::ImmFloat,
        value: OpValue::ImmShift {
            mask: false,
            shift: 0,
        },
        detail: OpDetail::Uimm16(imm8 as u16),
    }
}

pub fn opsysreg(reg: u32) -> Op {
    Op {
        op_type: OpType::Sysreg,
        value: OpValue::ImmShift {
            mask: false,
            shift: 0,
        },
        detail: OpDetail::Sysreg(reg as u16),
    }
}

pub fn opcond(cond: u32) -> Op {
    Op {
        op_type: OpType::Cond,
        value: OpValue::ImmShift {
            mask: false,
            shift: 0,
        },
        detail: OpDetail::Cond(Cond::try_from(cond as u8).unwrap()),
    }
}

include!("classifier.rs");
