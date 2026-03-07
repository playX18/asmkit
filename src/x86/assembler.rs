#![allow(dead_code, clippy::all)]
use super::opcodes::ALT_TAB;
use super::{features::*, operands::*};
use crate::{
    core::{
        buffer::{
            CodeBuffer, CodeOffset, ConstantData, LabelUse, Reloc, RelocDistance, RelocTarget,
        },
        emitter::Emitter,
        operand::*,
        patch::{PatchBlockId, PatchSiteId},
    },
    X86Error,
};

/// X86/X64 Assembler implementation.
pub struct Assembler<'a> {
    pub buffer: &'a mut CodeBuffer,
    flags: u64,
    extra_reg: Reg,
    last_error: Option<X86Error>,
}

const RC_RN: u64 = 0x0000000;
const RC_RD: u64 = 0x0800000;
const RC_RU: u64 = 0x1000000;
const RC_RZ: u64 = 0x1800000;
const SEG_MASK: u64 = 0xe0000000;
const ADDR32: u64 = 0x10000000;
const LONG: u64 = 0x100000000;

const OPC_66: u64 = 0x80000;
const OPC_F2: u64 = 0x100000;
const OPC_F3: u64 = 0x200000;
const OPC_REXW: u64 = 0x400000;
const OPC_LOCK: u64 = 0x800000;
const OPC_VEXL0: u64 = 0x1000000;
const OPC_VEXL1: u64 = 0x1800000;
const OPC_EVEXL0: u64 = 0x2000000;
const OPC_EVEXL1: u64 = 0x2800000;
const OPC_EVEXL2: u64 = 0x3000000;
const OPC_EVEXL3: u64 = 0x3800000;
const OPC_EVEXB: u64 = 0x4000000;
const OPC_VSIB: u64 = 0x8000000;
const OPC_67: u64 = ADDR32;
const OPC_SEG_MSK: u64 = 0xe0000000;
const OPC_JMPL: u64 = 0x100000000; // Placeholder for FE_JMPL, define as needed
const OPC_MASK_MSK: u64 = 0xe00000000;
const OPC_EVEXZ: u64 = 0x1000000000;
const OPC_USER_MSK: u64 = OPC_67 | OPC_SEG_MSK | OPC_MASK_MSK;
const OPC_FORCE_SIB: u64 = 0x2000000000;
const OPC_DOWNGRADE_VEX: u64 = 0x4000000000;
const OPC_DOWNGRADE_VEX_FLIPW: u64 = 0x40000000000;
const OPC_EVEX_DISP8SCALE: u64 = 0x38000000000;
const OPC_GPH_OP0: u64 = 0x200000000000;
const OPC_GPH_OP1: u64 = 0x400000000000;
const EPFX_REX_MSK: u64 = 0x43f;
const EPFX_REX: u64 = 0x20;
const EPFX_EVEX: u64 = 0x40;
const EPFX_REXR: u64 = 0x10;
const EPFX_REXX: u64 = 0x08;
const EPFX_REXB: u64 = 0x04;
const EPFX_REXR4: u64 = 0x02;
const EPFX_REXB4: u64 = 0x01;
const EPFX_REXX4: u64 = 0x400;
const EPFX_VVVV_IDX: u64 = 11;

fn op_imm_n(imm: Operand, immsz: usize) -> bool {
    if !imm.is_imm() {
        return false;
    }
    let imm = imm.as_::<Imm>();
    if immsz == 0 && imm.value() == 0 {
        return true;
    }

    if immsz == 1 && imm.is_int8() {
        return true;
    }
    if immsz == 2 && imm.is_int16() {
        return true;
    }
    if immsz == 3 && (imm.value() & 0xffffff) == imm.value() {
        return true;
    }
    if immsz == 4 && imm.is_int32() {
        return true;
    }
    if immsz == 8 {
        return true;
    }

    false
}

fn opc_size(opc: u64, epfx: u64) -> usize {
    let mut res = 1;

    if (opc & OPC_EVEXL0) != 0 {
        res += 4;
    } else if (opc & OPC_VEXL0) != 0 {
        if (opc & (OPC_REXW | 0x20000)) != 0 || (epfx & (EPFX_REXX | EPFX_REXB)) != 0 {
            res += 3;
        } else {
            res += 2;
        }
    } else {
        if (opc & OPC_LOCK) != 0 {
            res += 1;
        }
        if (opc & OPC_66) != 0 {
            res += 1;
        }
        if (opc & (OPC_F2 | OPC_F3)) != 0 {
            res += 1;
        }
        if (opc & OPC_REXW) != 0 || (epfx & EPFX_REX_MSK) != 0 {
            res += 1;
        }
        if (opc & 0x30000) != 0 {
            res += 1;
        }
        if (opc & 0x20000) != 0 {
            res += 1;
        }
    }
    if (opc & OPC_SEG_MSK) != 0 {
        res += 1;
    }
    if (opc & OPC_67) != 0 {
        res += 1;
    }
    if (opc & 0x8000) != 0 {
        res += 1;
    }

    res
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Encoding {
    NP,
    M,
    R,
    M1,
    MC,
    MR,
    RM,
    RMA,
    MRC,
    AM,
    MA,
    I,
    O,
    OA,
    S,
    A,
    D,
    FD,
    TD,
    IM,
    RVM,
    RVMR,
    RMV,
    VM,
    MVR,
    MRV,
    MAX,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
    pub const fn new() -> Self {
        Self {
            modreg: 0,
            modrm: 0,
            vexreg: 0,
            immctl: 0,
            immidx: 0,
            zregidx: 0,
            zregval: 0,
        }
    }
}

const ENCODING_INFOS: [EncodingInfo; Encoding::MAX as usize] = [
    EncodingInfo::new(),
    EncodingInfo {
        modrm: 0x0 ^ 3,
        immidx: 1,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modreg: 0x0 ^ 3,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x0 ^ 3,
        immctl: 1,
        immidx: 1,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x0 ^ 3,
        zregidx: 0x1 ^ 3,
        zregval: 1,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x0 ^ 3,
        modreg: 0x1 ^ 3,
        immidx: 2,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x1 ^ 3,
        modreg: 0x0 ^ 3,
        immidx: 2,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x1 ^ 3,
        modreg: 0x0 ^ 3,
        zregidx: 0x2 ^ 3,
        zregval: 0,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x0 ^ 3,
        modreg: 0x1 ^ 3,
        zregidx: 0x2 ^ 3,
        zregval: 1,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x1 ^ 3,
        zregidx: 0x0 ^ 3,
        zregval: 0,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x0 ^ 3,
        zregidx: 0x1 ^ 3,
        zregval: 0,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        immidx: 0,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modreg: 0x0 ^ 3,
        immidx: 1,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modreg: 0x0 ^ 3,
        zregidx: 0x1 ^ 3,
        zregval: 0,
        ..EncodingInfo::new()
    },
    EncodingInfo::new(),
    EncodingInfo {
        zregidx: 0x0 ^ 3,
        zregval: 0,
        immidx: 1,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        immidx: 0,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        zregidx: 0x0 ^ 3,
        zregval: 0,
        immctl: 2,
        immidx: 1,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        zregidx: 0x1 ^ 3,
        zregval: 0,
        immctl: 2,
        immidx: 0,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x1 ^ 3,
        immidx: 0,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x2 ^ 3,
        modreg: 0x0 ^ 3,
        vexreg: 0x1 ^ 3,
        immidx: 3,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x2 ^ 3,
        modreg: 0x0 ^ 3,
        vexreg: 0x1 ^ 3,
        immctl: 3,
        immidx: 3,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x1 ^ 3,
        modreg: 0x0 ^ 3,
        vexreg: 0x2 ^ 3,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x1 ^ 3,
        vexreg: 0x0 ^ 3,
        immidx: 2,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x0 ^ 3,
        modreg: 0x2 ^ 3,
        vexreg: 0x1 ^ 3,
        ..EncodingInfo::new()
    },
    EncodingInfo {
        modrm: 0x0 ^ 3,
        modreg: 0x1 ^ 3,
        vexreg: 0x2 ^ 3,
        ..EncodingInfo::new()
    },
];

impl<'a> Assembler<'a> {
    pub fn new(buf: &'a mut CodeBuffer) -> Self {
        Self {
            buffer: buf,
            extra_reg: Reg::new(),
            flags: 0,
            last_error: None,
        }
    }

    /// Get the last error that occurred during assembly.
    pub fn last_error(&self) -> Option<X86Error> {
        self.last_error.clone()
    }

    /// Clear the last error.
    pub fn clear_error(&mut self) {
        self.last_error = None;
    }

    pub fn sae(&mut self) -> &mut Self {
        self.flags |= 0x4000000;
        self
    }

    pub fn rn_sae(&mut self) -> &mut Self {
        self.flags |= 0x4000000 | RC_RN;
        self
    }

    pub fn rd_sae(&mut self) -> &mut Self {
        self.flags |= 0x4000000 | RC_RD;
        self
    }
    pub fn ru_sae(&mut self) -> &mut Self {
        self.flags |= 0x4000000 | RC_RU;
        self
    }

    pub fn rz_sae(&mut self) -> &mut Self {
        self.flags |= 0x4000000 | RC_RZ;
        self
    }

    pub fn seg(&mut self, sreg: SReg) -> &mut Self {
        self.flags |= ((sreg.id() & 0x7) as u64) << 29;
        self
    }

    pub fn fs(&mut self) -> &mut Self {
        self.seg(FS)
    }

    pub fn gs(&mut self) -> &mut Self {
        self.seg(GS)
    }

    pub fn k(&mut self, k: KReg) -> &mut Self {
        self.flags |= ((k.id() & 0x7) as u64) << 33;

        self
    }

    pub fn rep(&mut self) -> &mut Self {
        self.flags |= 0x200000;
        self
    }

    pub fn repnz(&mut self) -> &mut Self {
        self.flags |= 0x100000;
        self
    }

    pub fn repz(&mut self) -> &mut Self {
        self.rep()
    }

    pub fn lock(&mut self) -> &mut Self {
        self.flags |= 0x800000;
        self
    }

    pub fn long(&mut self) -> &mut Self {
        self.flags |= LONG;
        self
    }

    pub fn get_label(&mut self) -> Label {
        self.buffer.get_label()
    }

    pub fn bind_label(&mut self, label: Label) {
        self.buffer.bind_label(label);
    }

    pub fn add_constant(&mut self, c: impl Into<ConstantData>) -> Label {
        let c = self.buffer.add_constant(c);
        self.buffer.get_label_for_constant(c)
    }

    pub fn label_offset(&self, label: Label) -> CodeOffset {
        self.buffer.label_offset(label)
    }

    pub fn patchable_jmp(&mut self, label: Label) -> Result<PatchSiteId, crate::AsmError> {
        self.jmp(label);
        let offset = self
            .buffer
            .cur_offset()
            .saturating_sub(LabelUse::X86JmpRel32.patch_size() as u32);
        self.buffer
            .record_label_patch_site(offset, label, LabelUse::X86JmpRel32)
    }

    pub fn patchable_call(&mut self, label: Label) -> Result<PatchSiteId, crate::AsmError> {
        self.call(label);
        let offset = self
            .buffer
            .cur_offset()
            .saturating_sub(LabelUse::X86JmpRel32.patch_size() as u32);
        self.buffer
            .record_label_patch_site(offset, label, LabelUse::X86JmpRel32)
    }

    pub fn patchable_mov(
        &mut self,
        dst: impl OperandCast,
        src: impl OperandCast,
    ) -> Result<PatchBlockId, crate::AsmError> {
        let dst = *dst.as_operand();
        let src = *src.as_operand();

        if !src.is_imm() {
            return Err(crate::AsmError::InvalidOperand);
        }

        if dst.is_reg_type_of(RegType::Gp64) {
            self.mov64ri(dst, src);
            let offset = self.buffer.cur_offset().saturating_sub(8);
            self.buffer.record_patch_block(offset, 8, 1)
        } else if dst.is_reg_type_of(RegType::Gp32) {
            self.mov32ri(dst, src);
            let offset = self.buffer.cur_offset().saturating_sub(4);
            self.buffer.record_patch_block(offset, 4, 1)
        } else {
            Err(crate::AsmError::InvalidOperand)
        }
    }

    fn enc_opc(&mut self, opc: u64, epfx: u64) -> bool {
        if opc & OPC_SEG_MSK != 0 {
            self.buffer
                .put1(((0x65643e362e2600u64 >> (8 * ((opc >> 29) & 7))) & 0xff) as u8);
        }

        if opc & OPC_67 != 0 {
            self.buffer.put1(0x67);
        }

        if opc & OPC_EVEXL0 != 0 {
            self.buffer.put1(0x62);
            let mut b1 = (opc >> 16 & 7) as u8;
            if (epfx & EPFX_REXR) == 0 {
                b1 |= 0x80;
            }
            if (epfx & EPFX_REXX) == 0 {
                b1 |= 0x40;
            }
            if (epfx & EPFX_REXB) == 0 {
                b1 |= 0x20;
            }
            if (epfx & EPFX_REXR4) == 0 {
                b1 |= 0x10;
            }
            if (epfx & EPFX_REXB4) != 0 {
                b1 |= 0x08;
            }
            self.buffer.put1(b1);

            let mut b2 = (opc >> 20 & 3) as u8;
            if (epfx & EPFX_REXX4) == 0 {
                b2 |= 0x04;
            }
            b2 |= ((!(epfx >> EPFX_VVVV_IDX) & 0xf) << 3) as u8;
            if opc & OPC_REXW != 0 {
                b2 |= 0x80;
            }
            self.buffer.put1(b2);

            let mut b3 = (opc >> 33 & 7) as u8;
            b3 |= ((!(epfx >> EPFX_VVVV_IDX) & 0x10) >> 1) as u8;
            if opc & OPC_EVEXB != 0 {
                b3 |= 0x10;
            }
            b3 |= ((opc >> 23 & 3) << 5) as u8;
            if opc & OPC_EVEXZ != 0 {
                b3 |= 0x80;
            }
            self.buffer.put1(b3);
        } else if opc & OPC_VEXL0 != 0 {
            if (epfx & (EPFX_REXR4 | EPFX_REXX4 | EPFX_REXB4 | (0x10 << EPFX_VVVV_IDX))) != 0 {
                self.last_error = Some(X86Error::InvalidVEX {
                    field: "prefix",
                    reason: "VEX prefix has invalid extended register bits",
                });
                return true;
            }

            let vex3 = (opc & (OPC_REXW | 0x20000)) != 0 || (epfx & (EPFX_REXX | EPFX_REXB)) != 0;
            let pp = (opc >> 20 & 3) as u8;
            self.buffer.put1(0xc4 | !vex3 as u8);

            let mut b2 = pp | if (opc & 0x800000) != 0 { 0x4 } else { 0 };

            if vex3 {
                let mut b1 = (opc >> 16 & 3) as u8;
                if (epfx & EPFX_REXR) == 0 {
                    b1 |= 0x80;
                }
                if (epfx & EPFX_REXX) == 0 {
                    b1 |= 0x40;
                }
                if (epfx & EPFX_REXB) == 0 {
                    b1 |= 0x20;
                }
                self.buffer.put1(b1);

                if (opc & OPC_REXW) != 0 {
                    b2 |= 0x80;
                }
            } else {
                if (epfx & EPFX_REXR) == 0 {
                    b2 |= 0x80;
                }
            }

            b2 |= ((!(epfx >> EPFX_VVVV_IDX) & 0xf) << 3) as u8;
            self.buffer.put1(b2);
        } else {
            if opc & OPC_LOCK != 0 {
                self.buffer.put1(0xF0);
            }
            if opc & OPC_66 != 0 {
                self.buffer.put1(0x66);
            }
            if opc & OPC_F2 != 0 {
                self.buffer.put1(0xF2);
            }
            if opc & OPC_F3 != 0 {
                self.buffer.put1(0xF3);
            }
            if opc & OPC_REXW != 0 || epfx & EPFX_REX_MSK != 0 {
                let mut rex = 0x40;
                if opc & OPC_REXW != 0 {
                    rex |= 0x08;
                }
                if epfx & EPFX_REXR != 0 {
                    rex |= 0x04;
                }
                if epfx & EPFX_REXX != 0 {
                    rex |= 0x02;
                }
                if epfx & EPFX_REXB != 0 {
                    rex |= 0x01;
                }
                self.buffer.put1(rex);
            }
            if opc & 0x30000 != 0 {
                self.buffer.put1(0x0F);
            }
            if opc & 0x30000 == 0x20000 {
                self.buffer.put1(0x38);
            }
            if opc & 0x30000 == 0x30000 {
                self.buffer.put1(0x3A);
            }
        }

        self.buffer.put1((opc & 0xff) as u8);
        if (opc & 0x8000) != 0 {
            self.buffer.put1(((opc >> 8) & 0xff) as u8);
        }
        false
    }

    fn encode_imm(&mut self, imm: Operand, immsz: usize) -> bool {
        if !op_imm_n(imm, immsz) {
            self.last_error = Some(X86Error::InvalidImmediate {
                value: imm.as_::<Imm>().value() as i64,
                size: immsz,
                reason: "immediate value does not fit in specified size",
            });
            return true;
        }
        let imm = imm.as_::<Imm>().value() as u64;
        for i in 0..immsz {
            self.buffer.put1((imm >> 8 * i) as u8);
        }

        false
    }

    fn enc_o(&mut self, opc: u64, mut epfx: u64, op0: Operand) -> bool {
        if op0.id() & 0x8 != 0 {
            epfx |= EPFX_REXB;
        }

        if self.enc_opc(opc, epfx) {
            return true;
        }

        let ix = self.buffer.cur_offset() as usize - 1;
        let byte = self.buffer.data()[ix];

        self.buffer.data_mut()[ix] = (byte & 0xf8) | (op0.id() & 0x7) as u8;
        false
    }

    fn enc_mr(
        &mut self,
        mut opc: u64,
        mut epfx: u64,
        op0: Operand,
        op1: Operand,
        immsz: usize,
    ) -> bool {
        if op0.is_reg() && op0.id() & 0x8 != 0 {
            epfx |= EPFX_REXB;
        }

        if op0.is_reg() && op0.id() & 0x10 != 0 {
            epfx |= EPFX_REXX | EPFX_EVEX;
        }

        if op0.is_mem() && op0.as_::<Mem>().base_id() & 0x8 != 0 {
            epfx |= EPFX_REXB;
        }

        if op0.is_mem() && op0.as_::<Mem>().base_id() & 0x10 != 0 {
            epfx |= EPFX_REXB4;
        }

        if op0.is_mem() && op0.as_::<Mem>().index_id() & 0x8 != 0 {
            epfx |= EPFX_REXX;
        }

        if op0.is_mem() && op0.as_::<Mem>().index_id() & 0x10 != 0 {
            epfx |= if opc & OPC_VSIB != 0 {
                0x10 << EPFX_VVVV_IDX
            } else {
                EPFX_REXX4
            };
        }
        if op1.is_reg() && op1.id() & 0x8 != 0 {
            epfx |= EPFX_REXR;
        }

        if op1.is_reg() && op1.id() & 0x10 != 0 {
            epfx |= EPFX_REXR4;
        }

        let has_rex =
            (opc & (OPC_REXW | OPC_VEXL0 | OPC_EVEXL0) != 0) || (epfx & EPFX_REX_MSK) != 0;

        if has_rex && (op0.is_reg_type_of(RegType::Gp8Hi) || op1.is_reg_type_of(RegType::Gp8Hi)) {
            self.last_error = Some(X86Error::InvalidOperand {
                operand_index: if op0.is_reg_type_of(RegType::Gp8Hi) {
                    0
                } else {
                    1
                },
                reason: "high-byte registers cannot be used with REX prefix",
            });
            return true;
        }

        if epfx & (EPFX_EVEX | EPFX_REXB4 | EPFX_REXX4 | EPFX_REXR4 | (0x10 << EPFX_VVVV_IDX)) != 0
        {
            if (opc & OPC_EVEXL0) == 0 {
                self.last_error = Some(X86Error::InvalidEVEX {
                    field: "prefix",
                    reason: "EVEX-specific bits set but EVEX prefix not present",
                });
                return true;
            }
        } else if opc & OPC_DOWNGRADE_VEX != 0 {
            opc = (opc & !(OPC_EVEXL0 | OPC_EVEX_DISP8SCALE)) | OPC_VEXL0;
            if opc & OPC_DOWNGRADE_VEX_FLIPW != 0 {
                opc ^= OPC_REXW;
            }
        }

        if op0.is_reg() {
            if self.enc_opc(opc, epfx) {
                return true;
            }

            self.buffer
                .put1(0xc0 | ((op1.id() & 7) << 3) as u8 | (op0.id() & 7) as u8);
            return false;
        }

        let opcsz = opc_size(opc, epfx);

        let mut mod_ = 0;
        let reg = op1.id() & 7;
        let mut rm;
        let mut scale = 0;
        let mut idx = 4;
        let mut base = 0;
        let mut off = op0.as_::<Mem>().offset();
        let mut withsib = opc & OPC_FORCE_SIB != 0;
        let mem = op0.as_::<Mem>();

        if mem.has_index() {
            if opc & OPC_VSIB != 0 {
                if mem.index_type() != RegType::X86Xmm {
                    self.last_error = Some(X86Error::InvalidVSIB {
                        index_reg: mem.index_id(),
                        reason: "VSIB requires XMM register as index",
                    });
                    return true;
                }

                if opc & OPC_EVEXL0 != 0 && opc & OPC_MASK_MSK == 0 {
                    self.last_error = Some(X86Error::InvalidMasking {
                        mask_reg: 0,
                        reason: "VSIB with EVEX requires masking",
                    });
                    return true;
                }
            } else {
                if !matches!(mem.index_type(), RegType::Gp32 | RegType::Gp64) {
                    self.last_error = Some(X86Error::InvalidMemoryOperand {
                        base: Some(mem.base_id()),
                        index: Some(mem.index_id()),
                        scale: mem.shift() as _,
                        offset: mem.offset(),
                        reason: "index register must be Gp32 or Gp64",
                    });
                    return true;
                }

                if mem.index_id() == 4 {
                    self.last_error = Some(X86Error::InvalidSIB {
                        sib: 0,
                        reason: "index register cannot be RSP/R12",
                    });
                    return true;
                }
            }

            idx = mem.index_id() & 7;
            let scalabs = mem.shift();
            if scalabs & (scalabs.wrapping_sub(1)) != 0 {
                self.last_error = Some(X86Error::InvalidSIB {
                    sib: 0,
                    reason: "scale must be 1, 2, 4, or 8",
                });
                return true;
            }

            scale = if scalabs & 0xA != 0 { 1 } else { 0 } | if scalabs & 0xC != 0 { 2 } else { 0 };
            withsib = true;
        }

        let mut dispsz = 0;
        let mut label_use = None;
        let mut reloc = None;

        if !mem.has_base() {
            base = 5;
            rm = 4;
            dispsz = 4;
        } else if mem.has_base_reg() && mem.base_reg().is_rip() {
            rm = 5;
            dispsz = 4;
            off -= (opcsz + 5 + immsz) as i64;
            if withsib {
                self.last_error = Some(X86Error::InvalidRIPRelative {
                    offset: off,
                    reason: "RIP-relative addressing cannot be used with SIB",
                });
                return true;
            }
        } else if mem.has_base_label() {
            rm = 5;
            if withsib {
                self.last_error = Some(X86Error::InvalidLabel {
                    label_id: mem.base_id(),
                    reason: "label base cannot be used with SIB",
                });
                return true;
            }
            dispsz = 4;
            label_use = Some((mem.base_id(), LabelUse::X86JmpRel32));
        } else if mem.has_base_sym() {
            rm = 5;
            if withsib {
                self.last_error = Some(X86Error::InvalidSymbol {
                    symbol_id: mem.base_id(),
                    reason: "symbol base cannot be used with SIB",
                });
                return true;
            }
            dispsz = 4;
            let sym = Sym::from_id(mem.base_id());
            let distance = self.buffer.symbol_distance(sym);

            if distance == RelocDistance::Near {
                reloc = Some((sym, Reloc::X86PCRel4));
            } else {
                reloc = Some((sym, Reloc::X86GOTPCRel4))
            }
        } else {
            if !matches!(mem.base_type(), RegType::Gp32 | RegType::Gp64) {
                self.last_error = Some(X86Error::InvalidMemoryOperand {
                    base: Some(mem.base_id()),
                    index: Some(mem.index_id()),
                    scale: mem.shift() as _,
                    offset: mem.offset(),
                    reason: "base register must be Gp32 or Gp64",
                });
                return true;
            }

            rm = mem.base_id() & 0x7;

            if withsib || rm == 4 {
                base = rm;
                rm = 4;
            }

            if off != 0 {
                let disp8scale = (opc & OPC_EVEX_DISP8SCALE) >> 39;

                if (off & ((1 << (disp8scale)) - 1)) == 0
                    && op_imm_n(*imm(off >> disp8scale).as_operand(), 1)
                    && opc & LONG == 0
                {
                    mod_ = 0x40;
                    dispsz = 1;
                    off >>= disp8scale;
                } else {
                    mod_ = 0x80;
                    dispsz = 1;
                }
            } else if rm == 5 {
                mod_ = 0x40;
                dispsz = 1;
            }
        }

        if opcsz + 1 + (rm == 4) as usize + dispsz + immsz > 15 {
            self.last_error = Some(X86Error::TooLongInstruction {
                length: opcsz + 1 + (rm == 4) as usize + dispsz + immsz,
                max_length: 15,
            });
            return true;
        }

        if self.enc_opc(opc, epfx) {
            return true;
        }

        self.buffer.put1(mod_ as u8 | (reg << 3) as u8 | rm as u8);
        if rm == 4 {
            self.buffer
                .put1((scale << 6) as u8 | (idx << 3) as u8 | base as u8);
        }
        let offset = self.buffer.cur_offset();
        if let Some((label, label_use)) = label_use {
            self.buffer
                .use_label_at_offset(offset, Label::from_id(label), label_use);
        }

        if let Some((sym, reloc)) = reloc {
            self.buffer
                .add_reloc_at_offset(offset, reloc, RelocTarget::Sym(sym), -4);
        }
        self.encode_imm(*imm(off).as_operand(), dispsz)
    }
}

impl<'a> Emitter for Assembler<'a> {
    fn emit(&mut self, opcode: i64, op0: &Operand, op1: &Operand, op2: &Operand, op3: &Operand) {
        let mut opc = opcode as u64;
        opc |= self.flags;
        self.flags = 0;
        let ops = &[*op0, *op1, *op2, *op3];

        let mut epfx = 0;

        if opc & OPC_GPH_OP0 != 0 && op0.is_reg() && op0.id() >= Gp::SP {
            epfx |= EPFX_REX;
        } else if opc & OPC_GPH_OP0 == 0 && op0.is_reg_type_of(RegType::Gp8Hi) {
            self.last_error = Some(X86Error::InvalidRegister {
                reg_id: op0.id(),
                reg_type: "Gp8Hi",
                reason: "high-byte register not allowed in this context",
            });
            return;
        }

        if opc & OPC_GPH_OP1 != 0 && op1.is_reg() && op1.id() >= Gp::SP {
            epfx |= EPFX_REX;
        } else if opc & OPC_GPH_OP1 == 0 && op1.is_reg_type_of(RegType::Gp8Hi) {
            self.last_error = Some(X86Error::InvalidRegister {
                reg_id: op1.id(),
                reg_type: "Gp8Hi",
                reason: "high-byte register not allowed in this context",
            });
            return;
        }
        let mut label_use = None;
        let mut reloc = None;

        loop {
            macro_rules! next {
                () => {
                    let alt = opc >> 56;
                    if alt != 0 {
                        opc = ALT_TAB[alt as usize] as u64 | (opc & OPC_USER_MSK);
                        continue;
                    }
                };
            }

            let enc = (opc >> 51) & 0x1f;
            let ei = &ENCODING_INFOS[enc as usize];
            let mut imm = 0xcci64;
            let mut immsz = (opc >> 47) & 0xf;

            if ei.zregidx != 0 && ops[ei.zregidx as usize ^ 3].id() != 0 {
                next!();
            }

            if enc == Encoding::S as u64 {
                if ((op0.id() as u64) << 3 & 0x20) != (opc & 0x20) {
                    next!();
                }

                opc |= (op0.id() << 3) as u64;
            }

            if immsz != 0 {
                imm = ops[ei.immidx as usize].as_::<Imm>().value() as i64;

                if ei.immctl != 0 {
                    if ei.immctl == 2 {
                        immsz = if opc & OPC_67 != 0 { 4 } else { 8 };
                        if immsz == 4 {
                            imm = imm as i32 as i64; // addresses are zero-extended
                        }
                    } else if ei.immctl == 3 {
                        if !ops[ei.immidx as usize].is_reg_type_of(RegType::X86Xmm) {
                            self.last_error = Some(X86Error::InvalidRegister {
                                reg_id: ops[ei.immidx as usize].id(),
                                reg_type: "X86Xmm",
                                reason: "expected XMM register for immediate encoding",
                            });
                            return;
                        }
                        imm = (ops[ei.immidx as usize].id() << 4) as i64;
                        if !op_imm_n(*crate::core::operand::imm(imm).as_operand(), 1) {
                            self.last_error = Some(X86Error::InvalidImmediate {
                                value: imm,
                                size: 1,
                                reason: "XMM register index does not fit in 1 byte",
                            });
                            return;
                        }
                    } else if ei.immctl == 1 {
                        if imm != 1 {
                            next!();
                        }

                        immsz = 0;
                    }
                } else if enc == Encoding::D as u64 {
                    let has_alt = opc >> 56 != 0;
                    //let skip_to_alt = has_alt && opc & OPC_JMPL != 0;
                    let imm_op = ops[ei.immidx as usize];
                    if imm_op.is_label() {
                        if immsz == 1 && has_alt {
                            immsz = 4;
                            if opc & 0x80 != 0 {
                                opc -= 2;
                            } else {
                                opc += 0x10010;
                            }
                        } else if immsz == 1 {
                            self.last_error = Some(X86Error::InvalidLabel {
                                label_id: imm_op.id(),
                                reason: "label requires 32-bit displacement, 8-bit not supported",
                            });
                            return;
                        }
                        label_use = Some((imm_op.id(), LabelUse::X86JmpRel32));
                    } else if imm_op.is_sym() {
                        let sym = imm_op.as_::<Sym>();
                        if immsz == 1 && has_alt {
                            immsz = 4;
                            if opc & 0x80 != 0 {
                                opc -= 2;
                            } else {
                                opc += 0x10010;
                            }
                        } else if immsz == 1 {
                            self.last_error = Some(X86Error::InvalidSymbol {
                                symbol_id: sym.id(),
                                reason: "symbol requires 32-bit displacement, 8-bit not supported",
                            });
                            return;
                        }

                        let distance = self.buffer.symbol_distance(sym);
                        reloc = Some((
                            sym,
                            if distance == RelocDistance::Near {
                                Reloc::X86PCRel4
                            } else {
                                Reloc::X86GOTPCRel4
                            },
                        ));
                    }
                } else {
                    if !op_imm_n(*crate::core::operand::imm(imm).as_operand(), immsz as _) {
                        next!();
                    }
                }
            }

            if (opc & 0xfffffff) == 0x90 && ops[0].id() == 0 {
                next!();
            }

            if enc == Encoding::R as u64 {
                if self.enc_mr(opc, epfx, Operand::new(), ops[0], immsz as _) {
                    return;
                }
            } else if ei.modrm != 0 {
                let modreg = if ei.modreg != 0 {
                    ops[ei.modreg as usize ^ 3]
                } else {
                    *crate::x86::operands::Reg::from_type_and_id(
                        RegType::Gp64,
                        ((opc & 0xff00) >> 8) as u32,
                    )
                    .as_operand()
                };

                if ei.vexreg != 0 {
                    epfx |= (ops[ei.vexreg as usize ^ 3].id() as u64) << EPFX_VVVV_IDX;
                }

                if self.enc_mr(opc, epfx, ops[(ei.modrm ^ 3) as usize], modreg, immsz as _) {
                    next!();
                }
            } else if ei.modreg != 0 {
                if self.enc_o(opc, epfx, ops[(ei.modreg ^ 3) as usize]) {
                    return;
                }
            } else {
                if self.enc_opc(opc, epfx) {
                    return;
                }
            }

            if immsz != 0 {
                let offset = self.buffer.cur_offset();
                if let Some((sym, reloc)) = reloc {
                    self.buffer
                        .add_reloc_at_offset(offset, reloc, RelocTarget::Sym(sym), -4);
                }
                if let Some((label_id, label_use)) = label_use {
                    self.buffer
                        .use_label_at_offset(offset, Label::from_id(label_id), label_use);
                }

                if self.encode_imm(*crate::core::operand::imm(imm).as_operand(), immsz as _) {
                    return;
                }
            }

            self.flags = 0;

            break;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondCode {
    O = 0x0,
    NO = 0x1,
    C = 0x2,
    NC = 0x3,
    Z = 0x4,
    NZ = 0x5,
    BE = 0x6,
    A = 0x7,
    S = 0x8,
    NS = 0x9,
    P = 0xa,

    NP = 0xb,
    L = 0xc,
    GE = 0xd,
    LE = 0xe,
    G = 0xf,
}

impl CondCode {
    pub const B: Self = Self::C;
    pub const NAE: Self = Self::C;
    pub const AE: Self = Self::NC;
    pub const NB: Self = Self::NC;
    pub const E: Self = Self::Z;
    pub const NE: Self = Self::NZ;
    pub const NA: Self = Self::BE;
    pub const NBE: Self = Self::A;
    pub const PO: Self = Self::NP;
    pub const NGE: Self = Self::L;
    pub const NL: Self = Self::GE;
    pub const NG: Self = Self::LE;
    pub const NLE: Self = Self::G;
    pub const PE: Self = Self::P;

    pub const fn code(self) -> u8 {
        self as u8
    }

    pub fn invert(self) -> Self {
        match self {
            Self::O => Self::NO,
            Self::NO => Self::O,
            Self::C => Self::NC,
            Self::NC => Self::C,
            Self::Z => Self::NZ,
            Self::NZ => Self::Z,
            Self::BE => Self::A,
            Self::A => Self::BE,
            Self::S => Self::NS,
            Self::NS => Self::S,
            Self::P => Self::NP,
            Self::NP => Self::P,
            Self::L => Self::GE,
            Self::GE => Self::L,
            Self::LE => Self::G,
            Self::G => Self::LE,
        }
    }
}
