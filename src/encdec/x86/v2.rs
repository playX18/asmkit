#![allow(unused_mut)]
//! X86/X64 Encoder V2
//!
//! This encoder implements all X86 encodings as a separate function for each opcode and its operand variants. It's faster
//! than `v1` but has a downside of large code-size and takes much longer to compile.
use crate::AsmError;

pub const LONG: u32 = 0x8;
pub const ADDR32: u32 = 0x10;
pub const SEG_MASK: u32 = 0x7;
pub const RC_MASK: u32 = 0x60;
pub const RC_RN: u32 = 0x00;
pub const RC_RD: u32 = 0x20;
pub const RC_RU: u32 = 0x40;
pub const RC_RZ: u32 = 0x60;
pub const MEM_LONG: u8 = 0x8;

pub mod regs {

    /// General purpose X86/X64 register
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct Gp(pub u8);

    impl From<u8> for Gp {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl Into<u8> for Gp {
        fn into(self) -> u8 {
            self.0
        }
    }

    pub const AX: Gp = Gp(0);
    pub const CX: Gp = Gp(1);
    pub const DX: Gp = Gp(2);
    pub const BX: Gp = Gp(3);
    pub const SP: Gp = Gp(4);
    pub const BP: Gp = Gp(5);
    pub const SI: Gp = Gp(6);
    pub const DI: Gp = Gp(7);
    pub const R8: Gp = Gp(8);
    pub const R9: Gp = Gp(9);
    pub const R10: Gp = Gp(10);
    pub const R11: Gp = Gp(11);
    pub const R12: Gp = Gp(12);
    pub const R13: Gp = Gp(13);
    pub const R14: Gp = Gp(14);
    pub const R15: Gp = Gp(15);
    pub const IP: Gp = Gp(0x20);
    pub const NOREG: Gp = Gp(0x80);

    /// Segment register
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct SReg(pub u8);

    impl From<u8> for SReg {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl Into<u8> for SReg {
        fn into(self) -> u8 {
            self.0
        }
    }

    pub const ES: SReg = SReg(0);
    pub const CS: SReg = SReg(1);
    pub const SS: SReg = SReg(2);
    pub const DS: SReg = SReg(3);
    pub const FS: SReg = SReg(4);
    pub const GS: SReg = SReg(5);

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct GpH(pub u8);

    impl From<u8> for GpH {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl Into<u8> for GpH {
        fn into(self) -> u8 {
            self.0
        }
    }

    pub const AH: GpH = GpH(4);
    pub const CH: GpH = GpH(5);
    pub const DH: GpH = GpH(6);
    pub const BH: GpH = GpH(7);

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct St(pub u8);

    impl From<u8> for St {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl Into<u8> for St {
        fn into(self) -> u8 {
            self.0
        }
    }

    pub const ST0: St = St(0);
    pub const ST1: St = St(1);
    pub const ST2: St = St(2);
    pub const ST3: St = St(3);
    pub const ST4: St = St(4);
    pub const ST5: St = St(5);
    pub const ST6: St = St(6);
    pub const ST7: St = St(7);

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Mm(pub u8);

    impl From<u8> for Mm {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl Into<u8> for Mm {
        fn into(self) -> u8 {
            self.0
        }
    }

    pub const MM0: Mm = Mm(0);
    pub const MM1: Mm = Mm(1);
    pub const MM2: Mm = Mm(2);
    pub const MM3: Mm = Mm(3);
    pub const MM4: Mm = Mm(4);
    pub const MM5: Mm = Mm(5);
    pub const MM6: Mm = Mm(6);
    pub const MM7: Mm = Mm(7);

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    pub struct Xmm(pub u8);

    impl From<u8> for Xmm {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl Into<u8> for Xmm {
        fn into(self) -> u8 {
            self.0
        }
    }

    pub const XMM0: Xmm = Xmm(0);
    pub const XMM1: Xmm = Xmm(1);
    pub const XMM2: Xmm = Xmm(2);
    pub const XMM3: Xmm = Xmm(3);
    pub const XMM4: Xmm = Xmm(4);
    pub const XMM5: Xmm = Xmm(5);
    pub const XMM6: Xmm = Xmm(6);
    pub const XMM7: Xmm = Xmm(7);
    pub const XMM8: Xmm = Xmm(8);
    pub const XMM9: Xmm = Xmm(9);
    pub const XMM10: Xmm = Xmm(10);
    pub const XMM11: Xmm = Xmm(11);
    pub const XMM12: Xmm = Xmm(12);
    pub const XMM13: Xmm = Xmm(13);
    pub const XMM14: Xmm = Xmm(14);
    pub const XMM15: Xmm = Xmm(15);
    pub const XMM16: Xmm = Xmm(16);
    pub const XMM17: Xmm = Xmm(17);
    pub const XMM18: Xmm = Xmm(18);
    pub const XMM19: Xmm = Xmm(19);
    pub const XMM20: Xmm = Xmm(20);
    pub const XMM21: Xmm = Xmm(21);
    pub const XMM22: Xmm = Xmm(22);
    pub const XMM23: Xmm = Xmm(23);
    pub const XMM24: Xmm = Xmm(24);
    pub const XMM25: Xmm = Xmm(25);
    pub const XMM26: Xmm = Xmm(26);
    pub const XMM27: Xmm = Xmm(27);
    pub const XMM28: Xmm = Xmm(28);
    pub const XMM29: Xmm = Xmm(29);
    pub const XMM30: Xmm = Xmm(30);
    pub const XMM31: Xmm = Xmm(31);

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Mask(pub u8);

    impl From<u8> for Mask {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl Into<u8> for Mask {
        fn into(self) -> u8 {
            self.0
        }
    }

    pub const K0: Mask = Mask(0);
    pub const K1: Mask = Mask(1);
    pub const K2: Mask = Mask(2);
    pub const K3: Mask = Mask(3);
    pub const K4: Mask = Mask(4);
    pub const K5: Mask = Mask(5);
    pub const K6: Mask = Mask(6);
    pub const K7: Mask = Mask(7);

    pub const TMM0: Tmm = Tmm(0);
    pub const TMM1: Tmm = Tmm(1);
    pub const TMM2: Tmm = Tmm(2);
    pub const TMM3: Tmm = Tmm(3);
    pub const TMM4: Tmm = Tmm(4);
    pub const TMM5: Tmm = Tmm(5);
    pub const TMM6: Tmm = Tmm(6);
    pub const TMM7: Tmm = Tmm(7);

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Tmm(pub u8);

    impl From<u8> for Tmm {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl Into<u8> for Tmm {
        fn into(self) -> u8 {
            self.0
        }
    }

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct GpLH(pub u8);

    impl From<u8> for GpLH {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl Into<u8> for GpLH {
        fn into(self) -> u8 {
            self.0
        }
    }

    impl From<Gp> for GpLH {
        fn from(value: Gp) -> Self {
            Self(value.0)
        }
    }

    impl From<GpH> for GpLH {
        fn from(value: GpH) -> Self {
            Self(value.0 + 0x20)
        }
    }

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Dr(pub u8);

    impl From<u8> for Dr {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl Into<u8> for Dr {
        fn into(self) -> u8 {
            self.0
        }
    }

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Cr(pub u8);

    impl From<u8> for Cr {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl Into<u8> for Cr {
        fn into(self) -> u8 {
            self.0
        }
    }
}

pub use regs::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Mem {
    pub flags: u8,
    pub base: Gp,
    pub scale: u8,
    pub idx: Gp,
    pub off: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct MemV {
    pub flags: u8,
    pub base: Gp,
    pub scale: u8,
    pub idx: Xmm,
    pub off: i32,
}

/// An X86/X64 instruction value. Internally stored as 16-byte
/// buffer where first 15-bytes are used for instruction
/// and 16th byte is used as a "cursor".
pub struct Inst {
    /// buffer for instruction encoding
    /// ```text
    /// [byte0..byte14] - x86/x64 instruction
    /// byte15 - cursor into buffer
    /// ```
    buf: [u8; 16],
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

// EVEX/VEX "Opcode" format:
//
// | EVEX byte 4 | P P M M M - - W | Opcode byte | VEX-D VEX-D-FLIPW
// 0             8                 16            24
const OPC_VEX_WPP_SHIFT: u32 = 8;
const OPC_VEX_WPP_MASK: u32 = 0x83 << OPC_VEX_WPP_SHIFT;
const OPC_VEX_MMM_SHIFT: u32 = 10;
const OPC_VEX_MMM_MASK: u32 = 0x1f << OPC_VEX_MMM_SHIFT;
const OPC_VEX_DOWNGRADE_VEX: u32 = 1 << 24;
const OPC_VEX_DOWNGRADE_VEX_FLIPW: u32 = 1 << 25;

impl Inst {
    pub fn emit_u8(&mut self, value: u8) {
        self.buf[self.buf[15] as usize] = value;
        self.buf[15] += 1;
    }

    pub fn emit_imm(&mut self, imm: u64, size: usize) {
        let mut cursor = self.buf[15] as usize;
        for i in 0..size {
            self.buf[cursor] = (imm >> 8 * i) as u8;
            cursor += 1;
        }

        self.buf[15] = cursor as u8;
    }

    pub fn emit_seg67(&mut self, flags: u32) -> usize {
        let mut idx = 0;
        if flags & SEG_MASK != 0 {
            let seg = (0x65643e362e2600u64 >> (8 * (flags & SEG_MASK))) & 0xff;
            self.buf[idx] = seg as u8;
            idx += 1;
        }

        if flags & ADDR32 != 0 {
            self.buf[idx] = 0x67;
            idx += 1;
        }
        self.buf[15] = idx as u8;
        idx
    }

    pub fn emit_mem_common(
        &mut self,

        _ripoff: usize,
        op0: Mem,
        op1: u32,
        sibidx: usize,
        disp8scale: usize,
    ) -> Result<usize, AsmError> {
        let mut mod_ = 0;
        let reg = op1 & 7;
        let rm;

        let mut sib = 0x20;
        let mut withsib = false;
        let mut dispsz = 0;
        let mut off = op0.off;
        if sibidx < 8 {
            let scalabs = op0.scale;
            if scalabs & (scalabs - 1) != 0 {
                return Err(AsmError::InvalidOperand);
            }

            let scale = (scalabs & 0xa) as usize | (if scalabs & 0xc != 0 { 2 } else { 0 });
            sib = scale << 6 | sibidx << 3;
            withsib = true;
        }

        if op0.base.0 >= 0x20 {
            if op0.base.0 >= NOREG.0 {
                self.emit_u8((reg << 3) as u8 | 4);
                self.emit_u8(sib as u8 | 5);
                self.emit_imm(off as u32 as u64, 4);
                return Ok(self.len());
            } else if op0.base.0 == IP.0 {
                if withsib {
                    return Err(AsmError::InvalidOperand);
                }

                self.emit_u8((reg << 3) as u8 | 5);

                self.emit_imm(off as i64 as u64, 4);
                return Ok(self.len());
            }
        }

        rm = op0.base.0 as u8 & 7;

        if off != 0 || (op0.flags & MEM_LONG != 0) {
            if disp8scale == 0 {
                mod_ = if off as i8 as i32 == off && (op0.flags & MEM_LONG == 0) {
                    0x40
                } else {
                    0x80 as u8
                };

                dispsz = if off as i8 as i32 == off && (op0.flags & MEM_LONG == 0) {
                    1
                } else {
                    4
                };
            } else {
                if (off & ((1 << disp8scale) - 1)) == 0
                    && op_imm_n(off as i64 >> disp8scale, 1)
                    && (op0.flags & MEM_LONG == 0)
                {
                    off >>= disp8scale;
                    mod_ = 0x40;
                    dispsz = 1;
                } else {
                    mod_ = 0x80;
                    dispsz = 4;
                }
            }
        } else if rm == 5 {
            dispsz = 1;
            mod_ = 0x40;
        }

        if withsib || rm == 4 {
            self.emit_u8(mod_ as u8 | (reg << 3) as u8 | 4);
            self.emit_u8(sib as u8 | rm as u8);
            self.emit_imm(off as u32 as u64, dispsz as usize);
            Ok(self.len())
        } else {
            self.emit_u8(mod_ as u8 | (reg << 3) as u8 | rm as u8);
            self.emit_imm(off as u32 as u64, dispsz as usize);
            Ok(self.len())
        }
    }

    pub fn emit_mem(
        &mut self,
        ripoff: usize,
        op0: Mem,
        op1: u32,
        forcesib: bool,
        disp8scale: usize,
    ) -> Result<usize, AsmError> {
        let mut sibidx = if forcesib { 4 } else { 8 };

        if op0.idx.0 < NOREG.0 {
            if op0.scale == 0 {
                return Err(AsmError::InvalidOperand);
            }

            if op0.idx.0 == 4 {
                return Err(AsmError::InvalidOperand);
            }

            sibidx = op0.idx.0 & 7;
        }

        self.emit_mem_common(ripoff, op0, op1, sibidx as _, disp8scale)
    }

    pub fn emit_mem_vsib(
        &mut self,
        ripoff: usize,
        op0: MemV,
        op1: u32,
        forcesib: bool,
        disp8scale: usize,
    ) -> Result<usize, AsmError> {
        let _ = forcesib;
        if op0.scale == 0 {
            return Err(AsmError::InvalidOperand);
        }

        let mem = Mem {
            base: op0.base,
            scale: op0.scale,
            flags: op0.flags,
            idx: NOREG,
            off: op0.off,
        };

        self.emit_mem_common(ripoff, mem, op1, op0.idx.0 as usize & 7, disp8scale)
    }

    pub fn emit_vex_common(
        &mut self,
        opcode: u32,
        base: u8,
        idx: u8,
        reg: u8,
        vvvv: u32,
    ) -> Result<usize, AsmError> {
        if (base as u32 | idx as u32 | reg as u32 | vvvv) & 0x10 != 0 {
            return Err(AsmError::InvalidOperand);
        }

        let mut vex3 = ((base | idx) & 0x08) != 0 || (opcode & 0xfc00) != 0x0400;
        if vex3 {
            self.emit_u8(0xc4);
            let mut b1 = (opcode & OPC_VEX_MMM_MASK) >> OPC_VEX_MMM_SHIFT;
            if (reg & 0x08) == 0 {
                b1 |= 0x80;
            }
            if (idx & 0x08) == 0 {
                b1 |= 0x40;
            }

            if (base & 0x08) == 0 {
                b1 |= 0x20;
            }

            self.emit_u8(b1 as u8);

            let mut b2 = (opcode & OPC_VEX_WPP_MASK) >> OPC_VEX_WPP_SHIFT;

            if (opcode & 0x20) != 0 {
                b2 |= 0x04;
            }

            b2 |= (vvvv ^ 0xf) << 3;
            self.emit_u8(b2 as u8);
        } else {
            self.emit_u8(0xc5);
            let mut b2 = opcode >> OPC_VEX_WPP_SHIFT & 3;
            if (opcode & 0x20) != 0 {
                b2 |= 0x04;
            }

            if (reg & 0x08) == 0 {
                b2 |= 0x80;
            }

            b2 |= (vvvv ^ 0xf) << 3;
            self.emit_u8(b2 as u8);
        }

        self.emit_u8(((opcode & 0xff0000) >> 16) as u8);
        Ok(3 + vex3 as usize)
    }

    pub fn emit_vex_reg(
        &mut self,
        opcode: u32,
        rm: u8,
        reg: u8,
        vvvv: u32,
    ) -> Result<usize, AsmError> {
        let off = self.emit_vex_common(opcode, rm as _, 0, reg as _, vvvv)?;
        self.emit_u8(0xc0 | (reg << 3) & 0x38 | (rm & 7));
        Ok(off + 1)
    }

    pub fn emit_vex_mem(
        &mut self,
        opcode: u32,
        rm: Mem,
        reg: u8,
        vvvv: u32,
        ripoff: usize,
        forcesib: bool,
        disp8scale: usize,
    ) -> Result<usize, AsmError> {
        let off = self.emit_vex_common(opcode, rm.base.0, rm.idx.0, reg, vvvv)?;
        let memoff = self.emit_mem(ripoff + off, rm, reg as _, forcesib, disp8scale)?;
        Ok(off + memoff)
    }

    pub fn emit_vex_vsib(
        &mut self,
        opcode: u32,
        rm: MemV,
        reg: u8,
        vvvv: u32,
        ripoff: usize,
        forcesib: bool,
        disp8scale: usize,
    ) -> Result<usize, AsmError> {
        let off = self.emit_vex_common(opcode, rm.base.0, rm.idx.0, reg, vvvv)?;
        let memoff = self.emit_mem_vsib(ripoff + off, rm, reg as _, forcesib, disp8scale)?;
        Ok(off + memoff)
    }

    pub fn emit_evex_common(
        &mut self,
        opcode: u32,
        base: u8,
        idx: u8,
        reg: u8,
        vvvv: u32,
    ) -> Result<usize, AsmError> {
        self.emit_u8(0x62);
        let evexr3 = reg & 0x08 != 0;
        let evexr4 = reg & 0x10 != 0;
        let evexb3 = base & 0x08 != 0;
        let evexb4 = base & 0x10 != 0;
        let evexx3 = idx & 0x08 != 0;
        let evexx4 = idx & 0x10 != 0;
        let evexv4 = vvvv & 0x10 != 0;

        let mut b1 = (opcode & OPC_VEX_MMM_MASK) >> OPC_VEX_MMM_SHIFT;
        if !evexr3 {
            b1 |= 0x80;
        }
        if !evexx3 {
            b1 |= 0x40;
        }

        if !evexb3 {
            b1 |= 0x20;
        }

        if !evexr4 {
            b1 |= 0x10;
        }

        if evexb4 {
            b1 |= 0x08;
        }

        self.emit_u8(b1 as _);
        let mut b2 = (opcode & OPC_VEX_WPP_MASK) >> OPC_VEX_WPP_SHIFT;
        if !evexx4 {
            b2 |= 0x04;
        }

        b2 |= (!vvvv & 0xf) << 3;
        self.emit_u8(b2 as _);
        let mut b3 = (opcode & 0xff) as u8;
        if !evexv4 {
            b3 |= 0x08;
        }

        self.emit_u8(b3);
        self.emit_u8(((opcode & 0xff0000) >> 16) as u8);

        Ok(5)
    }

    pub fn evex_to_vex(opcode: u32) -> u32 {
        if opcode & OPC_VEX_DOWNGRADE_VEX_FLIPW != 0 {
            opcode ^ 0x8000
        } else {
            opcode
        }
    }

    pub fn emit_evex_reg(
        &mut self,
        opcode: u32,
        rm: u32,
        reg: u32,
        vvvv: u32,
    ) -> Result<usize, AsmError> {
        let off = if ((rm | reg | vvvv) & 0x10) == 0 && (opcode & OPC_VEX_DOWNGRADE_VEX) != 0 {
            self.emit_vex_common(Self::evex_to_vex(opcode), rm as _, 0, reg as _, vvvv)?
        } else {
            self.emit_evex_common(opcode, rm as _, 0, reg as _, vvvv)?
        };

        self.emit_u8(0xc0 | (reg << 3 & 0x38) as u8 | (rm & 7) as u8);
        Ok(off + 1)
    }

    pub fn emit_evex_xmm(
        &mut self,
        opcode: u32,
        rm: u8,
        reg: u8,
        vvvv: u32,
    ) -> Result<usize, AsmError> {
        let off = if ((rm | reg | vvvv as u8) & 0x10) == 0 && (opcode & OPC_VEX_DOWNGRADE_VEX) != 0
        {
            self.emit_vex_common(Self::evex_to_vex(opcode), rm as _, 0, reg as _, vvvv)?
        } else {
            self.emit_evex_common(opcode, rm as u8 & 0x0f, rm >> 1, reg as _, vvvv)?
        };

        self.emit_u8(0xc0 | (reg << 3 & 0x38) | (rm & 7));

        Ok(off + 1)
    }

    pub fn emit_evex_mem(
        &mut self,
        opcode: u32,
        rm: Mem,
        reg: u8,
        vvvv: u32,
        ripoff: usize,
        forcesib: bool,
        mut disp8scale: usize,
    ) -> Result<usize, AsmError> {
        let off = if ((rm.base.0 as u32 | rm.idx.0 as u32 | reg as u32 | vvvv) & 0x10) == 0
            && (opcode & OPC_VEX_DOWNGRADE_VEX) != 0
        {
            disp8scale = 0; // only AVX-512 EVEX compresses displacement
            self.emit_vex_common(Self::evex_to_vex(opcode), rm.base.0, rm.idx.0, reg, vvvv)?
        } else {
            self.emit_evex_common(opcode, rm.base.0, rm.idx.0, reg, vvvv)?
        };
        let memoff = self.emit_mem(ripoff + off, rm, reg as _, forcesib, disp8scale)?;
        Ok(off + memoff)
    }

    pub fn emit_evex_vsib(
        &mut self,
        opcode: u32,
        rm: MemV,
        reg: u8,
        vvvv: u32,
        ripoff: usize,
        forcesib: bool,
        disp8scale: usize,
    ) -> Result<usize, AsmError> {
        let _ = vvvv;
        // EVEX VSIB requires non-zero mask operand
        if (opcode & 0x7) == 0 {
            return Err(AsmError::InvalidOperand);
        }

        // EVEX.X4 is encoded in EVEX.V4
        let idx = rm.idx.0;
        let off = self.emit_evex_common(opcode, rm.base.0, idx & 0x0f, reg, vvvv)?;
        let memoff = self.emit_mem_vsib(ripoff + off, rm, reg as _, forcesib, disp8scale)?;
        Ok(off + memoff)
    }

    pub const fn new() -> Self {
        Self { buf: [0; 16] }
    }

    pub const fn len(&self) -> usize {
        self.buf[15] as usize
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len()]
    }
}

impl core::ops::Deref for Inst {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl core::ops::DerefMut for Inst {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let l = self.len();
        &mut self.buf[..l]
    }
}

fn op_reg_gph<T: Into<u8>>(x: T) -> bool {
    let x = x.into();
    x & !0x3 == 0x24
}

use core::ops::{Add, Mul};

impl Add<i32> for Gp {
    type Output = Mem;

    fn add(self, rhs: i32) -> Self::Output {
        Mem {
            base: self,
            off: rhs,
            idx: NOREG,
            scale: 1,
            flags: 0,
        }
    }
}

impl Add<Gp> for Mem {
    type Output = Mem;

    fn add(self, rhs: Gp) -> Self::Output {
        let mut this = self;
        this.idx = rhs;
        this
    }
}

impl Mul<u8> for Mem {
    type Output = Mem;

    fn mul(self, rhs: u8) -> Self::Output {
        let mut this = self;
        this.scale = rhs;
        this
    }
}

include!("inst.rs");
