use crate::AsmError;

pub enum Reg {
    AX = 0x100,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    IP = 0x120,
    AH = 0x204,
    CH,
    DH,
    BH,
    ES = 0x300,
    CS,
    SS,
    DS,
    FS,
    GS,
    St0 = 0x400,
    St1,
    St2,
    St3,
    St4,
    St5,
    St6,
    St7,
    Xmm0 = 0x600,
    Xmm1,
    Xmm2,
    Xmm3,
    Xmm4,
    Xmm5,
    Xmm6,
    Xmm7,
    Xmm8,
    Xmm9,
    Xmm10,
    Xmm11,
    Xmm12,
    Xmm13,
    Xmm14,
    Xmm15,
    Xmm16,
    Xmm17,
    Xmm18,
    Xmm19,
    Xmm20,
    Xmm21,
    Xmm22,
    Xmm23,
    Xmm24,
    Xmm25,
    Xmm26,
    Xmm27,
    Xmm28,
    Xmm29,
    Xmm30,
    Xmm31,
    K0 = 0x700,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    Tmm0 = 0x800,
    Tmm1,
    Tmm2,
    Tmm3,
    Tmm4,
    Tmm5,
    Tmm6,
    Tmm7,

    Max,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Op(pub i64);

impl Op {
    pub const fn none() -> Self {
        Self(0)
    }
    pub const fn reg(r: Reg) -> Self {
        Self(r as i64)
    }
    /// Construct a memory operand. Unused parts can be set to 0 and will be
    /// ignored. IP can be used as base register, in which case the offset is
    /// interpreted as the offset from the /current/ position -- the size of the
    /// encoded instruction will be subtracted during encoding. scale must be 1, 2,
    /// 4, or 8; but is ignored if idx == Op(0). **/
    pub const fn mem(base: Op, sc: u8, idx: Op, off: i32) -> Self {
        Self(
            i64::MIN
                | ((base.0 & 0xfff) << 32)
                | ((idx.0 & 0xfff) << 44)
                | ((sc as i64 & 0xf) << 56)
                | (off as i64 & 0xffffffff),
        )
    }

    pub const fn imm(x: i64) -> Self {
        Self(x)
    }
}

pub const NOREG: Op = Op(0);

pub const AX: Op = Op::reg(Reg::AX);
pub const CX: Op = Op::reg(Reg::CX);
pub const DX: Op = Op::reg(Reg::DX);
pub const BX: Op = Op::reg(Reg::BX);
pub const SP: Op = Op::reg(Reg::SP);
pub const BP: Op = Op::reg(Reg::BP);
pub const SI: Op = Op::reg(Reg::SI);
pub const DI: Op = Op::reg(Reg::DI);
pub const R8: Op = Op::reg(Reg::R8);
pub const R9: Op = Op::reg(Reg::R9);
pub const R10: Op = Op::reg(Reg::R10);
pub const R11: Op = Op::reg(Reg::R11);
pub const R12: Op = Op::reg(Reg::R12);
pub const R13: Op = Op::reg(Reg::R13);
pub const R14: Op = Op::reg(Reg::R14);
pub const R15: Op = Op::reg(Reg::R15);
pub const IP: Op = Op::reg(Reg::IP);

pub const AH: Op = Op::reg(Reg::AH);
pub const CH: Op = Op::reg(Reg::CH);
pub const DH: Op = Op::reg(Reg::DH);
pub const BH: Op = Op::reg(Reg::BH);

pub const ES: Op = Op::reg(Reg::ES);
pub const CS: Op = Op::reg(Reg::CS);
pub const SS: Op = Op::reg(Reg::SS);
pub const DS: Op = Op::reg(Reg::DS);
pub const FS: Op = Op::reg(Reg::FS);
pub const GS: Op = Op::reg(Reg::GS);

pub const ST0: Op = Op::reg(Reg::St0);
pub const ST1: Op = Op::reg(Reg::St1);
pub const ST2: Op = Op::reg(Reg::St2);
pub const ST3: Op = Op::reg(Reg::St3);
pub const ST4: Op = Op::reg(Reg::St4);
pub const ST5: Op = Op::reg(Reg::St5);
pub const ST6: Op = Op::reg(Reg::St6);
pub const ST7: Op = Op::reg(Reg::St7);

pub const XMM0: Op = Op::reg(Reg::Xmm0);
pub const XMM1: Op = Op::reg(Reg::Xmm1);
pub const XMM2: Op = Op::reg(Reg::Xmm2);
pub const XMM3: Op = Op::reg(Reg::Xmm3);
pub const XMM4: Op = Op::reg(Reg::Xmm4);
pub const XMM5: Op = Op::reg(Reg::Xmm5);
pub const XMM6: Op = Op::reg(Reg::Xmm6);
pub const XMM7: Op = Op::reg(Reg::Xmm7);
pub const XMM8: Op = Op::reg(Reg::Xmm8);
pub const XMM9: Op = Op::reg(Reg::Xmm9);
pub const XMM10: Op = Op::reg(Reg::Xmm10);
pub const XMM11: Op = Op::reg(Reg::Xmm11);
pub const XMM12: Op = Op::reg(Reg::Xmm12);
pub const XMM13: Op = Op::reg(Reg::Xmm13);
pub const XMM14: Op = Op::reg(Reg::Xmm14);
pub const XMM15: Op = Op::reg(Reg::Xmm15);
pub const XMM16: Op = Op::reg(Reg::Xmm16);
pub const XMM17: Op = Op::reg(Reg::Xmm17);
pub const XMM18: Op = Op::reg(Reg::Xmm18);
pub const XMM19: Op = Op::reg(Reg::Xmm19);
pub const XMM20: Op = Op::reg(Reg::Xmm20);
pub const XMM21: Op = Op::reg(Reg::Xmm21);
pub const XMM22: Op = Op::reg(Reg::Xmm22);
pub const XMM23: Op = Op::reg(Reg::Xmm23);
pub const XMM24: Op = Op::reg(Reg::Xmm24);
pub const XMM25: Op = Op::reg(Reg::Xmm25);
pub const XMM26: Op = Op::reg(Reg::Xmm26);
pub const XMM27: Op = Op::reg(Reg::Xmm27);
pub const XMM28: Op = Op::reg(Reg::Xmm28);
pub const XMM29: Op = Op::reg(Reg::Xmm29);
pub const XMM30: Op = Op::reg(Reg::Xmm30);
pub const XMM31: Op = Op::reg(Reg::Xmm31);

pub const K0: Op = Op::reg(Reg::K0);
pub const K1: Op = Op::reg(Reg::K1);
pub const K2: Op = Op::reg(Reg::K2);
pub const K3: Op = Op::reg(Reg::K3);
pub const K4: Op = Op::reg(Reg::K4);
pub const K5: Op = Op::reg(Reg::K5);
pub const K6: Op = Op::reg(Reg::K6);
pub const K7: Op = Op::reg(Reg::K7);

pub const TMM0: Op = Op::reg(Reg::Tmm0);
pub const TMM1: Op = Op::reg(Reg::Tmm1);
pub const TMM2: Op = Op::reg(Reg::Tmm2);
pub const TMM3: Op = Op::reg(Reg::Tmm3);
pub const TMM4: Op = Op::reg(Reg::Tmm4);
pub const TMM5: Op = Op::reg(Reg::Tmm5);
pub const TMM6: Op = Op::reg(Reg::Tmm6);
pub const TMM7: Op = Op::reg(Reg::Tmm7);

pub const LONG: i64 = 0x10000000;
pub const ADDR32: i64 = 0x10000000;

pub const OPC_66: i64 = 0x80000;
pub const OPC_F2: i64 = 0x100000;
pub const OPC_F3: i64 = 0x200000;
pub const OPC_REXW: i64 = 0x400000;
pub const OPC_LOCK: i64 = 0x800000;
pub const OPC_VEXL0: i64 = 0x1000000;
pub const OPC_VEXL1: i64 = 0x1800000;
pub const OPC_EVEXL0: i64 = 0x2000000;
pub const OPC_EVEXL1: i64 = 0x2800000;
pub const OPC_EVEXL2: i64 = 0x3000000;
pub const OPC_EVEXL3: i64 = 0x3800000;
pub const OPC_EVEXB: i64 = 0x4000000;
pub const OPC_VSIB: i64 = 0x8000000;
pub const OPC_67: i64 = ADDR32; // Assuming FE_ADDR32 is defined elsewhere
pub const OPC_SEG_MSK: i64 = 0xe0000000;
pub const OPC_LONG: i64 = LONG; // Assuming FE_JMPL is defined elsewhere
pub const OPC_MASK_MSK: i64 = 0xe00000000;
pub const OPC_EVEXZ: i64 = 0x1000000000;
pub const OPC_USER_MSK: i64 = OPC_67 | OPC_SEG_MSK | OPC_MASK_MSK;
pub const OPC_FORCE_SIB: i64 = 0x2000000000;
pub const OPC_DOWNGRADE_VEX: i64 = 0x4000000000;
pub const OPC_DOWNGRADE_VEX_FLIPW: i64 = 0x40000000000;
pub const OPC_EVEX_DISP8SCALE: i64 = 0x38000000000;
pub const OPC_GPH_OP0: i64 = 0x200000000000;
pub const OPC_GPH_OP1: i64 = 0x400000000000;

pub const EPFX_REX_MSK: u16 = 0x43f;
pub const EPFX_REX: u16 = 0x20;
pub const EPFX_EVEX: u16 = 0x40;
pub const EPFX_REXR: u16 = 0x10;
pub const EPFX_REXX: u16 = 0x08;
pub const EPFX_REXB: u16 = 0x04;
pub const EPFX_REXR4: u16 = 0x02;
pub const EPFX_REXB4: u16 = 0x01;
pub const EPFX_REXX4: u16 = 0x400;
pub const EPFX_VVVV_IDX: u16 = 11;

impl Op {
    pub const fn is_mem(&self) -> bool {
        self.0 < 0
    }

    pub const fn is_reg(&self) -> bool {
        self.0 >= 0 && self.0 < Reg::Max as i64
    }

    pub const fn is_label(&self) -> bool {
        self.0 >= Reg::Max as i64
    }

    pub const fn is_gpl(&self) -> bool {
        self.0 & !0x1f == 0x100
    }

    pub const fn is_gph(&self) -> bool {
        self.0 & !0x3 == 0x204
    }

    pub const fn is_xmm(&self) -> bool {
        self.0 & !0x1f == 0x600
    }

    pub const fn mem_offset(&self) -> i32 {
        self.0 as i32
    }

    pub const fn mem_base(&self) -> Op {
        Self((self.0 >> 32) & 0xfff)
    }

    pub const fn mem_idx(&self) -> Op {
        Self((self.0 >> 44) & 0xfff)
    }

    pub const fn mem_scale(&self) -> u8 {
        ((self.0 >> 56) & 0xf) as u8
    }

    pub const fn reg_idx(&self) -> u8 {
        (self.0 & 0xff) as u8
    }
}

fn op_imm_n(imm: i64, immsz: usize) -> bool {
    match immsz {
        0 => imm == 0,
        1 => (imm as i8) as i64 == imm,
        2 => (imm as i16) as i64 == imm,
        3 => (imm & 0xffffff) as i64 == imm,
        4 => (imm as i32) as i64 == imm,
        8 => (imm as i64) as i64 == imm,
        _ => false,
    }
}

const fn opc_size(opc: i64, epfx: u16) -> u32 {
    let mut res = 1;

    if opc & OPC_EVEXL0 != 0 {
        res += 4;
    } else if opc & OPC_VEXL0 != 0 {
        if opc & (OPC_REXW | 0x20000) != 0 || epfx & (EPFX_REXX | EPFX_REXB) != 0 {
            res += 3;
        } else {
            res += 2;
        }
    } else {
        if opc & OPC_LOCK != 0 {
            res += 1;
        }
        if opc & OPC_66 != 0 {
            res += 1;
        }
        if opc & (OPC_F2 | OPC_F3) != 0 {
            res += 1;
        }
        if opc & OPC_REXW != 0 || epfx & EPFX_REX_MSK != 0 {
            res += 1;
        }
        if opc & 0x30000 != 0 {
            res += 1;
        }
        if opc & 0x20000 != 0 {
            res += 1;
        }
    }

    if opc & OPC_SEG_MSK != 0 {
        res += 1;
    }
    if opc & OPC_67 != 0 {
        res += 1;
    }
    if opc & 0x8000 != 0 {
        res += 1;
    }

    res
}

pub struct BufWriter<'a> {
    buf: &'a mut [u8],
    cursor: usize,
}

impl<'a> BufWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, cursor: 0 }
    }

    pub fn write(&mut self, byte: u8) -> Result<(), AsmError> {
        if self.cursor < self.buf.len() {
            self.buf[self.cursor] = byte;
            self.cursor += 1;
            Ok(())
        } else {
            Err(AsmError::InvalidInstruction)
        }
    }

    pub fn write_slice(&mut self, slice: &[u8]) -> Result<(), AsmError> {
        if self.cursor + slice.len() <= self.buf.len() {
            self.buf[self.cursor..self.cursor + slice.len()].copy_from_slice(slice);
            self.cursor += slice.len();
            Ok(())
        } else {
            Err(AsmError::InvalidInstruction)
        }
    }

    pub fn len(&self) -> usize {
        self.cursor
    }
}

fn enc_opc(buf: &mut BufWriter, opc: i64, epfx: u16) -> Result<(), AsmError> {
    if opc & OPC_SEG_MSK != 0 {
        buf.write(((0x65643e362e2600u64 >> (8 * ((opc >> 29) & 7))) & 0xff) as u8)?;
    }
    if opc & OPC_67 != 0 {
        buf.write(0x67)?;
    }
    if opc & OPC_EVEXL0 != 0 {
        buf.write(0x62)?;
        let mut b1 = (opc >> 16 & 7) as u8;
        if epfx & EPFX_REXR == 0 {
            b1 |= 0x80;
        }
        if epfx & EPFX_REXX == 0 {
            b1 |= 0x40;
        }
        if epfx & EPFX_REXB == 0 {
            b1 |= 0x20;
        }
        if epfx & EPFX_REXR4 == 0 {
            b1 |= 0x10;
        }
        if epfx & EPFX_REXB4 != 0 {
            b1 |= 0x08;
        }
        buf.write(b1)?;
        let mut b2 = (opc >> 20 & 3) as u8;
        if epfx & EPFX_REXX4 == 0 {
            b2 |= 0x04;
        }
        b2 |= (!((epfx >> EPFX_VVVV_IDX) as u8) & 0xf) << 3;
        if opc & OPC_REXW != 0 {
            b2 |= 0x80;
        }
        buf.write(b2)?;
        let mut b3 = (opc >> 33 & 7) as u8;
        b3 |= (!((epfx >> EPFX_VVVV_IDX) as u8) & 0x10) >> 1;
        if opc & OPC_EVEXB != 0 {
            b3 |= 0x10;
        }
        b3 |= ((opc >> 23 & 3) << 5) as u8;
        if opc & OPC_EVEXZ != 0 {
            b3 |= 0x80;
        }
        buf.write(b3)?;
    } else if opc & OPC_VEXL0 != 0 {
        if epfx & (EPFX_REXR4 | EPFX_REXX4 | EPFX_REXB4 | (0x10 << EPFX_VVVV_IDX)) != 0 {
            return Err(AsmError::InvalidOperand);
        }
        let vex3 = opc & (OPC_REXW | 0x20000) != 0 || epfx & (EPFX_REXX | EPFX_REXB) != 0;
        let pp = (opc >> 20 & 3) as u8;
        buf.write(0xc4 | (!vex3 as u8))?;
        let mut b2 = pp | if opc & 0x800000 != 0 { 0x4 } else { 0 };
        if vex3 {
            let mut b1 = (opc >> 16 & 3) as u8;
            if epfx & EPFX_REXR == 0 {
                b1 |= 0x80;
            }
            if epfx & EPFX_REXX == 0 {
                b1 |= 0x40;
            }
            if epfx & EPFX_REXB == 0 {
                b1 |= 0x20;
            }
            buf.write(b1)?;
            if opc & OPC_REXW != 0 {
                b2 |= 0x80;
            }
        } else {
            if epfx & EPFX_REXR == 0 {
                b2 |= 0x80;
            }
        }
        b2 |= (!((epfx >> EPFX_VVVV_IDX) as u8) & 0xf) << 3;
        buf.write(b2)?;
    } else {
        if opc & OPC_LOCK != 0 {
            buf.write(0xF0)?;
        }
        if opc & OPC_66 != 0 {
            buf.write(0x66)?;
        }
        if opc & OPC_F2 != 0 {
            buf.write(0xF2)?;
        }
        if opc & OPC_F3 != 0 {
            buf.write(0xF3)?;
        }
        if opc & OPC_REXW != 0 || epfx & (EPFX_REX_MSK) != 0 {
            let mut rex = 0x40;
            if opc & OPC_REXW != 0 {
                rex |= 8;
            }
            if epfx & EPFX_REXR != 0 {
                rex |= 4;
            }
            if epfx & EPFX_REXX != 0 {
                rex |= 2;
            }
            if epfx & EPFX_REXB != 0 {
                rex |= 1;
            }
            buf.write(rex)?;
        }
        if opc & 0x30000 != 0 {
            buf.write(0x0F)?;
        }
        if (opc & 0x30000) == 0x20000 {
            buf.write(0x38)?;
        }
        if (opc & 0x30000) == 0x30000 {
            buf.write(0x3A)?;
        }
    }
    buf.write((opc & 0xff) as u8)?;
    if opc & 0x8000 != 0 {
        buf.write(((opc >> 8) & 0xff) as u8)?;
    }
    Ok(())
}

fn enc_imm(buf: &mut BufWriter, imm: i64, immsz: usize) -> Result<(), AsmError> {
    if !op_imm_n(imm, immsz) {
        return Err(AsmError::InvalidOperand);
    }
    for i in 0..immsz {
        buf.write(((imm >> (8 * i)) & 0xff) as u8)?;
    }
    Ok(())
}

fn enc_o(buf: &mut BufWriter, opc: i64, mut epfx: u16, op0: Op) -> Result<(), AsmError> {
    if op0.reg_idx() & 0x8 != 0 {
        epfx |= EPFX_REXB;
    }

    let has_rex = opc & OPC_REXW != 0 || epfx & EPFX_REX_MSK != 0;
    if has_rex && op0.is_gph() {
        return Err(AsmError::InvalidOperand);
    }

    enc_opc(buf, opc, epfx)?;
    let last_byte = buf.buf[buf.len() - 1];
    buf.buf[buf.len() - 1] = (last_byte & 0xf8) | (op0.reg_idx() & 0x7);
    Ok(())
}

fn enc_mr(
    buf: &mut BufWriter,
    mut opc: i64,
    mut epfx: u16,
    op0: Op,
    op1: Op,
    immsz: usize,
) -> Result<(), AsmError> {
    if op0.is_reg() {
        if op0.reg_idx() & 0x8 != 0 {
            epfx |= EPFX_REXB;
        }
        if op0.reg_idx() & 0x10 != 0 {
            epfx |= if opc & OPC_EVEXL0 == 0 {
                EPFX_REXX | EPFX_EVEX
            } else {
                EPFX_REXB4
            };
        }
    }
    if op0.is_mem() {
        if op0.mem_base().reg_idx() & 0x8 != 0 {
            epfx |= EPFX_REXB;
        }
        if op0.mem_base().reg_idx() & 0x10 != 0 {
            epfx |= EPFX_REXB4;
        }
        if op0.mem_idx().reg_idx() & 0x8 != 0 {
            epfx |= EPFX_REXX;
        }
        if op0.mem_idx().reg_idx() & 0x10 != 0 {
            epfx |= if opc & OPC_VSIB != 0 {
                0x10 << EPFX_VVVV_IDX
            } else {
                EPFX_REXX4
            };
        }
    }
    if op1.is_reg() {
        if op1.reg_idx() & 0x8 != 0 {
            epfx |= EPFX_REXR;
        }
        if op1.reg_idx() & 0x10 != 0 {
            epfx |= EPFX_REXR4;
        }
    }

    let has_rex = opc & (OPC_REXW | OPC_VEXL0 | OPC_EVEXL0) != 0 || epfx & EPFX_REX_MSK != 0;
    if has_rex && (op0.is_gph() || op1.is_gph()) {
        return Err(AsmError::InvalidOperand);
    }

    if epfx & (EPFX_EVEX | EPFX_REXB4 | EPFX_REXX4 | EPFX_REXR4 | (0x10 << EPFX_VVVV_IDX)) != 0 {
        if opc & OPC_EVEXL0 == 0 {
            return Err(AsmError::InvalidOperand);
        }
    } else if opc & OPC_DOWNGRADE_VEX != 0 {
        opc = (opc & !(OPC_EVEXL0 | OPC_EVEX_DISP8SCALE)) | OPC_VEXL0;
        if opc & OPC_DOWNGRADE_VEX_FLIPW != 0 {
            opc ^= OPC_REXW;
        }
    }

    if op0.is_reg() {
        enc_opc(buf, opc, epfx)?;
        buf.write(0xc0 | ((op1.reg_idx() & 7) << 3) | (op0.reg_idx() & 7))?;
        return Ok(());
    }

    let opcsz = opc_size(opc, epfx);

    let mut mod_rm = 0;
    let reg = op1.reg_idx() & 7;
    let mut rm;
    let mut scale = 0;
    let mut idx = 4;
    let mut base = 0;
    let mut off = op0.mem_offset();
    let mut withsib = opc & OPC_FORCE_SIB != 0;

    if op0.mem_idx().0 != 0 && op0.mem_scale() == 0 || op0.mem_idx().0 == 0 && op0.mem_scale() != 0
    {
        return Err(AsmError::InvalidOperand);
    }
    if op0.mem_idx().0 == 0 && opc & OPC_VSIB != 0 {
        return Err(AsmError::InvalidOperand);
    }
    if op0.mem_idx().0 != 0 {
        if opc & OPC_VSIB != 0 {
            if !op0.mem_idx().is_xmm() {
                return Err(AsmError::InvalidOperand);
            }
            if opc & OPC_EVEXL0 != 0 && opc & OPC_MASK_MSK == 0 {
                return Err(AsmError::InvalidOperand);
            }
        } else {
            if !op0.mem_idx().is_gpl() {
                return Err(AsmError::InvalidOperand);
            }
            if op0.mem_idx().reg_idx() == 4 {
                return Err(AsmError::InvalidOperand);
            }
        }
        idx = op0.mem_idx().reg_idx() & 7;
        let scalabs = op0.mem_scale();
        if scalabs & (scalabs - 1) != 0 {
            return Err(AsmError::InvalidOperand);
        }
        scale = (if scalabs & 0xA != 0 { 1 } else { 0 }) | (if scalabs & 0xC != 0 { 2 } else { 0 });
        withsib = true;
    }

    let mut dispsz = 0;
    if op0.mem_base().0 == 0 {
        base = 5;
        rm = 4;
        dispsz = 4;
    } else if op0.mem_base() == IP {
        rm = 5;
        dispsz = 4;
        off -= opcsz as i32 + 5 + immsz as i32;
        if withsib {
            return Err(AsmError::InvalidOperand);
        }
    } else {
        if !op0.mem_base().is_gpl() {
            return Err(AsmError::InvalidOperand);
        }
        rm = op0.mem_base().reg_idx() & 7;
        if withsib || rm == 4 {
            base = rm;
            rm = 4;
        }
        if off != 0 {
            let disp8scale = (opc & OPC_EVEX_DISP8SCALE) >> 39;
            if off & ((1 << disp8scale) - 1) == 0 && op_imm_n(off as i64 >> disp8scale, 1) {
                mod_rm = 0x40;
                dispsz = 1;
                off >>= disp8scale;
            } else {
                mod_rm = 0x80;
                dispsz = 4;
            }
        } else if rm == 5 {
            mod_rm = 0x40;
            dispsz = 1;
        }
    }

    if opcsz + 1 + (if rm == 4 { 1 } else { 0 }) + dispsz as u32 + immsz as u32 > 15 {
        return Err(AsmError::InvalidInstruction);
    }

    enc_opc(buf, opc, epfx)?;
    buf.write(mod_rm | (reg << 3) | rm)?;
    if rm == 4 {
        buf.write((scale << 6) | (idx << 3) | base)?;
    }
    enc_imm(buf, off as i64, dispsz as usize)
}

#[allow(dead_code)]
enum Encoding {
    NP,
    M,
    R,
    M1,
    MI,
    MC,
    MR,
    RM,
    RMA,
    MRI,
    RMI,
    MRC,
    AM,
    MA,
    I,
    IA,
    O,
    OI,
    OA,
    S,
    A,
    D,
    FD,
    TD,
    RVM,
    RVMI,
    RVMR,
    RMV,
    VM,
    VMI,
    MVR,
    MRV,
    MAX,
}

#[derive(Copy, Clone)]
struct EncodingInfo {
    modrm: u8,
    modreg: u8,
    vexreg: u8,
    immidx: u8,
    immctl: u8,
    zregidx: u8,
    zregval: u8,
}

impl EncodingInfo {
    const fn new(
        modrm: u8,
        modreg: u8,
        vexreg: u8,
        immidx: u8,
        immctl: u8,
        zregidx: u8,
        zregval: u8,
    ) -> Self {
        Self {
            modrm: modrm & 0b11,
            modreg: modreg & 0b11,
            vexreg: vexreg & 0b11,
            immidx: immidx & 0b11,
            immctl: immctl & 0b111,
            zregidx: zregidx & 0b11,
            zregval: zregval & 0b1,
        }
    }
}

const ENCODING_INFOS: [EncodingInfo; Encoding::MAX as usize] = [
    EncodingInfo::new(0, 0, 0, 0, 0, 0, 0), // NP
    EncodingInfo::new(0, 0, 0, 0, 0, 0, 0), // M
    EncodingInfo::new(0, 0, 0, 0, 0, 0, 0), // R
    EncodingInfo::new(0, 0, 0, 1, 1, 0, 0), // M1
    EncodingInfo::new(0, 0, 0, 1, 4, 0, 0), // MI
    EncodingInfo::new(0, 0, 0, 0, 0, 1, 1), // MC
    EncodingInfo::new(0, 1, 0, 0, 0, 0, 0), // MR
    EncodingInfo::new(1, 0, 0, 0, 0, 0, 0), // RM
    EncodingInfo::new(1, 0, 0, 0, 0, 2, 0), // RMA
    EncodingInfo::new(0, 1, 0, 2, 4, 0, 0), // MRI
    EncodingInfo::new(1, 0, 0, 2, 4, 0, 0), // RMI
    EncodingInfo::new(0, 1, 0, 0, 0, 2, 1), // MRC
    EncodingInfo::new(1, 0, 0, 0, 0, 0, 0), // AM
    EncodingInfo::new(0, 0, 0, 0, 0, 1, 0), // MA
    EncodingInfo::new(0, 0, 0, 0, 4, 0, 0), // I
    EncodingInfo::new(0, 0, 0, 1, 4, 0, 0), // IA
    EncodingInfo::new(0, 0, 0, 0, 0, 0, 0), // O
    EncodingInfo::new(0, 0, 0, 1, 4, 0, 0), // OI
    EncodingInfo::new(0, 0, 0, 0, 0, 1, 0), // OA
    EncodingInfo::new(0, 0, 0, 0, 0, 0, 0), // S
    EncodingInfo::new(0, 0, 0, 0, 0, 0, 0), // A
    EncodingInfo::new(0, 0, 0, 0, 6, 0, 0), // D
    EncodingInfo::new(0, 0, 0, 1, 2, 0, 0), // FD
    EncodingInfo::new(0, 0, 0, 0, 2, 1, 0), // TD
    EncodingInfo::new(2, 0, 1, 0, 0, 0, 0), // RVM
    EncodingInfo::new(2, 0, 1, 3, 4, 0, 0), // RVMI
    EncodingInfo::new(2, 0, 1, 3, 3, 0, 0), // RVMR
    EncodingInfo::new(1, 0, 2, 0, 0, 0, 0), // RMV
    EncodingInfo::new(1, 0, 0, 0, 0, 0, 0), // VM
    EncodingInfo::new(1, 0, 0, 2, 4, 0, 0), // VMI
    EncodingInfo::new(0, 2, 1, 0, 0, 0, 0), // MVR
    EncodingInfo::new(0, 1, 2, 0, 0, 0, 0), // MRV
];

include!(concat!(env!("OUT_DIR"), "/inst_x86_v1.rs"));

pub fn fe_enc64_impl(
    buf: &mut BufWriter,
    mut opc: i64,
    op0: Op,
    op1: Op,
    op2: Op,
    op3: Op,
) -> Result<(), AsmError> {
    let ops = [op0, op1, op2, op3];
    let mut epfx = 0u16;

    if opc & OPC_GPH_OP0 != 0 && op0.is_gpl() && op0.reg_idx() >= 4 {
        epfx |= EPFX_REX;
    } else if opc & OPC_GPH_OP0 == 0 && op0.is_gph() {
        return Err(AsmError::InvalidOperand);
    }
    if opc & OPC_GPH_OP1 != 0 && op1.is_gpl() && op1.reg_idx() >= 4 {
        epfx |= EPFX_REX;
    } else if opc & OPC_GPH_OP1 == 0 && op1.is_gph() {
        return Err(AsmError::InvalidOperand);
    }
    loop {
        loop {
            let enc = (opc >> 51) & 0x1f;
            let ei = &ENCODING_INFOS[enc as usize];

            let mut imm = 0xcc;
            let mut immsz = ((opc >> 47) & 0xf) as usize;

            if ei.zregidx != 0 && ops[ei.zregidx as usize ^ 3].reg_idx() != ei.zregval {
                break;
            }

            if enc == Encoding::S as i64 {
                if (op0.reg_idx() << 3 & 0x20) as i64 != (opc & 0x20) {
                    break;
                }
                opc |= (op0.reg_idx() as i64) << 3;
            }

            if ei.immctl > 0 {
                imm = ops[ei.immidx as usize].0;
                if ei.immctl == 2 {
                    immsz = if opc & OPC_67 != 0 { 4 } else { 8 };
                    if immsz == 4 {
                        imm = imm as i32 as i64;
                    }
                }
                if ei.immctl == 3 {
                    if !ops[ei.immidx as usize].is_xmm() {
                        return Err(AsmError::InvalidOperand);
                    }
                    imm = (ops[ei.immidx as usize].reg_idx() as i64) << 4;
                }
                if ei.immctl == 6 {
                    if opc & LONG != 0 && opc >> 56 != 0 {
                        break;
                    }
                    imm -= buf.len() as i64 + opc_size(opc, epfx) as i64 + immsz as i64;
                }
                if ei.immctl == 1 && imm != 1 {
                    break;
                }
                if ei.immctl >= 2 && !op_imm_n(imm, immsz) {
                    break;
                }
            }

            if (opc & 0xfffffff) == 0x90 && ops[0] == AX {
                break;
            }

            if enc == Encoding::R as i64 {
                enc_mr(buf, opc, epfx, NOREG, ops[0], immsz)?;
            } else if ei.modrm != 0 {
                let modreg = if ei.modreg != 0 {
                    ops[ei.modreg as usize ^ 3]
                } else {
                    Op(((opc & 0xff00) >> 8) as i64)
                };
                if ei.vexreg != 0 {
                    epfx |= (ops[ei.vexreg as usize ^ 3].reg_idx() as u16) << EPFX_VVVV_IDX;
                }
                enc_mr(buf, opc, epfx, ops[ei.modrm as usize ^ 3], modreg, immsz)?;
            } else if ei.modreg != 0 {
                enc_o(buf, opc, epfx, ops[ei.modreg as usize ^ 3])?;
            } else {
                enc_opc(buf, opc, epfx)?;
            }

            if ei.immctl >= 2 {
                enc_imm(buf, imm, immsz)?;
            }

            return Ok(());
        }
        let alt = opc >> 56;
        if alt != 0 {
            opc = ALT_TAB[alt as usize] | (opc & OPC_USER_MSK);
            continue;
        }
        break;
    }

    Err(AsmError::InvalidInstruction)
}

#[macro_export]
macro_rules! emit_x86 {
    ($buf: expr, $mnem: ident, $op: expr) => {{
        let mut buf = $crate::x86::v1::BufWriter::new($buf);
        $crate::x86::v1::fe_enc64_impl(
            &mut buf,
            $crate::x86::v1::$mnem,
            $op[0],
            NOREG,
            NOREG,
            NOREG,
        )
        .map(|x| buf.len())
    }};

    ($buf: expr, $mnem: ident, $op0: expr, $op1: expr) => {{
        let mut buf = $crate::x86::v1::BufWriter::new($buf);
        $crate::x86::v1::fe_enc64_impl(&mut buf, $crate::x86::v1::$mnem, $op0, $op1, NOREG, NOREG)
            .map(|x| buf.len())
    }};
    ($buf: expr, $mnem: ident, $op0: expr, $op1: expr, $op2: expr) => {{
        let mut buf = $crate::x86::v1::BufWriter::new($buf);
        $crate::x86::v1::fe_enc64_impl(&mut buf, $crate::x86::v1::$mnem, $op0, $op1, $op2, NOREG)
            .map(|x| buf.len())
    }};
    ($buf: expr, $mnem: ident, $op0: expr, $op1: expr, $op2: expr, $op3: expr) => {{
        let mut buf = $crate::x86::v1::BufWriter::new($buf);
        $crate::x86::v1::fe_enc64_impl(&mut buf, $crate::x86::v1::$mnem, $op0, $op1, $op2, $op3)
            .map(|x| buf.len())
    }};
}
