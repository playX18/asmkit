use super::decode_tab::*;

pub struct Decoder<'a> {
    buf: &'a [u8],
    cursor: usize,
    address: u64,
    mode: DecodeMode,
    table_root_idx: u16,
    error: DecoderError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Reg {
    #[default]
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

    IP = 0x10,
    None = 0x3f,
}

impl TryFrom<u8> for Reg {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Reg::R0),
            1 => Ok(Reg::R1),
            2 => Ok(Reg::R2),
            3 => Ok(Reg::R3),
            4 => Ok(Reg::R4),
            5 => Ok(Reg::R5),
            6 => Ok(Reg::R6),
            7 => Ok(Reg::R7),
            8 => Ok(Reg::R8),
            9 => Ok(Reg::R9),
            10 => Ok(Reg::R10),
            11 => Ok(Reg::R11),
            12 => Ok(Reg::R12),
            13 => Ok(Reg::R13),
            14 => Ok(Reg::R14),
            15 => Ok(Reg::R15),
            _ if value == 0x10 => Ok(Reg::IP),
            _ if value == 0x3f => Ok(Reg::None),
            _ => Err(()),
        }
    }
}

impl Reg {
    pub const AL: Self = Self::R0;
    pub const CL: Self = Self::R1;
    pub const DL: Self = Self::R2;
    pub const BL: Self = Self::R3;
    pub const AH: Self = Self::R4;
    pub const CH: Self = Self::R5;
    pub const DH: Self = Self::R6;
    pub const BH: Self = Self::R7;

    pub const AX: Self = Self::R0;
    pub const CX: Self = Self::R1;
    pub const DX: Self = Self::R2;
    pub const BX: Self = Self::R3;
    pub const SP: Self = Self::R4;
    pub const BP: Self = Self::R5;
    pub const SI: Self = Self::R6;
    pub const DI: Self = Self::R7;

    pub const ES: Self = Self::R0;
    pub const CS: Self = Self::R1;
    pub const SS: Self = Self::R2;
    pub const DS: Self = Self::R3;
    pub const FS: Self = Self::R4;
    pub const GS: Self = Self::R5;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum OperandType {
    #[default]
    None,
    Reg,
    Imm,
    Mem,
    Off,
    MemBCST,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegType {
    /// Vector (SSE/AVX) register XMMn/YMMn/ZMMn
    Vec = 0,
    /// Low general purpose register
    Gpl = 1,
    /// High-byte general purpose register
    Gph = 2,
    /// Segment register
    Seg = 3,
    /// FPU register ST(n)
    Fpu = 4,
    /// MMX register MMn
    Mmx = 5,
    /// TMM register TMMn
    Tmm = 6,
    /// Vector mask (AVX-512) register Kn
    Mask = 7,
    /// Bound register BNDn
    Bnd = 8,
    /// Control Register CRn
    Cr = 9,
    /// Debug Register DRn
    Dr = 10,
    /// Must be a memory operand
    Mem = 15,
}

impl TryFrom<u8> for RegType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RegType::Vec),
            1 => Ok(RegType::Gpl),
            2 => Ok(RegType::Gph),
            3 => Ok(RegType::Seg),
            4 => Ok(RegType::Fpu),
            5 => Ok(RegType::Mmx),
            6 => Ok(RegType::Tmm),
            7 => Ok(RegType::Mask),
            8 => Ok(RegType::Bnd),
            9 => Ok(RegType::Cr),
            10 => Ok(RegType::Dr),
            15 => Ok(RegType::Mem),
            _ => Err(()),
        }
    }
}

/// Do not depend on the actual enum values.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundControl {
    /// Round to nearest (even)
    Rn = 1,
    /// Round down
    Rd = 3,
    /// Round up
    Ru = 5,
    /// Round to zero (truncate)
    Rz = 7,
    /// Rounding mode as specified in MXCSR
    Mxcsr = 0,
    /// Rounding mode irrelevant, but SAE
    Sae = 6,
}

impl TryFrom<u8> for RoundControl {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RoundControl::Rn),
            3 => Ok(RoundControl::Rd),
            5 => Ok(RoundControl::Ru),
            7 => Ok(RoundControl::Rz),
            0 => Ok(RoundControl::Mxcsr),
            6 => Ok(RoundControl::Sae),
            _ => Err(()),
        }
    }
}

pub struct InstDesc {
    typ: Opcode,
    operand_indices: u16,
    operand_sizes: u16,
    reg_types: u16,
}

impl InstDesc {
    pub(crate) const fn new(
        typ: Opcode,
        operand_indices: u16,
        operand_sizes: u16,
        reg_types: u16,
    ) -> Self {
        Self {
            typ,
            operand_indices,
            operand_sizes,
            reg_types,
        }
    }

    pub(crate) const INVALID: Self = Self::new(Opcode::_3DNOW, 0, 0, 0);

    pub fn has_modrm(&self) -> bool {
        (self.operand_indices & (3 << 0)) != 0
    }

    pub fn modrm_idx(&self) -> u8 {
        (((self.operand_indices >> 0) & 3) ^ 3) as u8
    }

    pub fn has_modreg(&self) -> bool {
        (self.operand_indices & (3 << 2)) != 0
    }

    pub fn modreg_idx(&self) -> u8 {
        (((self.operand_indices >> 2) & 3) ^ 3) as u8
    }

    pub fn has_vexreg(&self) -> bool {
        (self.operand_indices & (3 << 4)) != 0
    }

    pub fn vexreg_idx(&self) -> u8 {
        (((self.operand_indices >> 4) & 3) ^ 3) as u8
    }

    pub fn imm_control(&self) -> u8 {
        ((self.operand_indices >> 12) & 0x7) as u8
    }

    pub fn imm_idx(&self) -> u8 {
        (((self.operand_indices >> 6) & 3) ^ 3) as u8
    }

    pub fn evex_bcst(&self) -> bool {
        ((self.operand_indices >> 8) & 1) != 0
    }

    pub fn evex_mask(&self) -> bool {
        ((self.operand_indices >> 9) & 1) != 0
    }

    pub fn zeroreg_val(&self) -> bool {
        ((self.operand_indices >> 10) & 1) != 0
    }

    pub fn lock(&self) -> bool {
        ((self.operand_indices >> 11) & 1) != 0
    }

    pub fn vsib(&self) -> bool {
        ((self.operand_indices >> 15) & 1) != 0
    }

    pub fn opsize(&self) -> u8 {
        ((self.reg_types >> 11) & 7) as u8
    }

    pub fn modrm_size(&self) -> u8 {
        ((self.operand_sizes >> 0) & 3) as u8
    }

    pub fn modreg_size(&self) -> u8 {
        ((self.operand_sizes >> 2) & 3) as u8
    }

    pub fn vexreg_size(&self) -> u8 {
        ((self.operand_sizes >> 4) & 3) as u8
    }

    pub fn imm_size(&self) -> u8 {
        ((self.operand_sizes >> 6) & 3) as u8
    }

    pub fn legacy(&self) -> bool {
        ((self.operand_sizes >> 8) & 1) != 0
    }

    pub fn size_fix1(&self) -> u8 {
        ((self.operand_sizes >> 10) & 7) as u8
    }

    pub fn size_fix2(&self) -> u8 {
        ((self.operand_sizes >> 13) & 3) as u8
    }

    pub fn instr_width(&self) -> bool {
        ((self.operand_sizes >> 15) & 1) != 0
    }

    pub fn modrm(&self) -> bool {
        ((self.reg_types >> 14) & 1) != 0
    }

    pub fn ign66(&self) -> bool {
        ((self.reg_types >> 15) & 1) != 0
    }

    pub fn evex_sae(&self) -> bool {
        ((self.reg_types >> 8) & 1) != 0
    }

    pub fn evex_er(&self) -> bool {
        ((self.reg_types >> 9) & 1) != 0
    }

    pub fn evex_bcst16(&self) -> bool {
        ((self.reg_types >> 10) & 1) != 0
    }

    pub fn regty_modrm(&self) -> u8 {
        ((self.reg_types >> 0) & 7) as u8
    }

    pub fn regty_modreg(&self) -> u8 {
        ((self.reg_types >> 3) & 7) as u8
    }

    pub fn regty_vexreg(&self) -> u8 {
        ((self.reg_types >> 6) & 3) as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodeMode {
    Decode64,
    Decode32,
}

impl DecodeMode {
    fn is_64(&self) -> bool {
        matches!(self, Self::Decode64)
    }
}

fn table_lookup(cur_idx: u16, entry_idx: u8) -> u16 {
    DECODE_TABLE[cur_idx as usize + entry_idx as usize]
}

fn table_walk(table_entry: u16, entry_idx: u8) -> u16 {
    table_lookup(table_entry & !0x3, entry_idx)
}

fn load_le_1(buf: &[u8]) -> u64 {
    buf[0] as u64
}

fn load_le_2(buf: &[u8]) -> u64 {
    load_le_1(buf) | (load_le_1(&buf[1..]) << 8)
}

fn load_le_3(buf: &[u8]) -> u64 {
    load_le_2(buf) | (load_le_1(&buf[2..]) << 16)
}

fn load_le_4(buf: &[u8]) -> u64 {
    load_le_2(buf) | (load_le_2(&buf[2..]) << 16)
}

fn load_le_8(buf: &[u8]) -> u64 {
    load_le_4(buf) | (load_le_4(&buf[4..]) << 32)
}

const PREFIX_REXB: u8 = 0x01;
const PREFIX_REXX: u8 = 0x02;
const PREFIX_REXR: u8 = 0x04;
const PREFIX_REXW: u8 = 0x08;
const PREFIX_REX: u8 = 0x40;
const PREFIX_REXRR: u8 = 0x10;
const PREFIX_VEX: u8 = 0x20;

#[derive(Default, Copy, Clone, Debug)]
pub struct Op {
    pub typ: OperandType,
    pub size: u8,
    pub reg: u8,
    pub misc: u8,
}

#[derive(Default, Debug)]
pub struct Instruction {
    pub typ: Opcode,
    pub flags: u8,
    pub segment: u8,
    pub addrsz: u8,
    pub operandsz: u8,
    pub size: u8,
    pub evex: u8,
    pub operands: [Op; 4],
    pub disp: i64,
    pub imm: i64,
    pub address: u64,
}

impl Instruction {
    pub fn is_valid(&self) -> bool {
        self.typ != Opcode::INVALID
    }

    pub fn code(&self) -> Opcode {
        self.typ
    }

    pub fn address(&self) -> u64 {
        self.address
    }

    pub fn size(&self) -> usize {
        self.size as _
    }

    pub fn segment(&self) -> Option<Reg> {
        Reg::try_from(self.segment)
            .ok()
            .filter(|reg| *reg != Reg::None)
    }

    pub fn addrsize(&self) -> usize {
        1 << self.addrsz
    }

    pub fn addrsize_log(&self) -> usize {
        self.addrsz as _
    }

    pub fn opsize(&self) -> usize {
        1 << self.operandsz
    }

    pub fn opsize_log(&self) -> usize {
        self.operandsz as _
    }

    pub fn has_rep(&self) -> bool {
        self.flags & 4 != 0
    }

    pub fn has_repnz(&self) -> bool {
        self.flags & 2 != 0
    }

    pub fn has_lock(&self) -> bool {
        self.flags & 1 != 0
    }

    pub fn is_64(&self) -> bool {
        self.flags & 128 != 0
    }

    pub fn op_type(&self, idx: usize) -> OperandType {
        self.operands[idx].typ
    }

    pub fn op_size(&self, idx: usize) -> usize {
        1 << self.operands[idx].size >> 1
    }

    pub fn op_size_log(&self, idx: usize) -> usize {
        (self.operands[idx].size - 1) as usize
    }

    pub fn op_reg(&self, idx: usize) -> Option<Reg> {
        Reg::try_from(self.operands[idx].reg).ok()
    }

    pub fn op_reg_type(&self, idx: usize) -> Option<RegType> {
        RegType::try_from(self.operands[idx].misc).ok()
    }

    pub fn op_reg_high(&self, idx: usize) -> bool {
        self.op_reg_type(idx) == Some(RegType::Gph)
    }

    pub fn op_base(&self, idx: usize) -> Option<Reg> {
        self.op_reg(idx)
    }

    pub fn op_index(&self, idx: usize) -> Option<Reg> {
        Reg::try_from(self.operands[idx].misc & 0x3f).ok()
    }

    pub fn op_scale(&self, idx: usize) -> u8 {
        self.operands[idx].misc >> 6
    }

    pub fn op_disp(&self, idx: usize) -> i64 {
        let _ = idx;
        self.disp
    }

    pub fn op_bcstsz(&self, idx: usize) -> usize {
        1 << self.op_bcstsz_log(idx)
    }

    pub fn op_bcstsz_log(&self, idx: usize) -> usize {
        let _ = idx;
        self.segment as usize >> 6
    }

    pub fn op_imm(&self, idx: usize) -> i64 {
        let _ = idx;
        self.imm
    }

    pub fn maskreg(&self) -> Option<Reg> {
        if self.evex & 0x07 == 0 {
            return None;
        }
        Reg::try_from(self.evex & 0x07).ok()
    }

    pub fn maskzero(&self) -> bool {
        self.evex & 0x80 != 0
    }

    pub fn round_control(&self) -> Option<RoundControl> {
        RoundControl::try_from((self.evex & 0x70) >> 4).ok()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DecoderError {
    None,
    InvalidInstruction,
    Internal,
    NoMoreBytes,
}

impl<'a> Decoder<'a> {
    pub fn new(bitness: u32, bytes: &'a [u8], ip: u64) -> Self {
        Self {
            mode: if bitness < 64 {
                DecodeMode::Decode32
            } else {
                DecodeMode::Decode64
            },
            buf: bytes,
            cursor: 0,
            address: ip,
            table_root_idx: if bitness == 64 {
                DECODE_TABLE_OFFSET_64 as u16
            } else {
                DECODE_TABLE_OFFSET_32 as u16
            },
            error: DecoderError::None,
        }
    }

    pub fn decode_out(&mut self, inst: &mut Instruction) {
        inst.typ = Opcode::INVALID;

        let mut vex_operand = 0u8;
        let mut addr_size = if self.mode.is_64() { 3 } else { 2 };

        let mut prefix_rex = 0u8;
        let mut vexl = 0u8;
        let mut prefix_rep = 0u8;
        let mut prefix_evex = 0u32;

        inst.segment = Reg::None as _;
        let start = self.cursor;
        #[allow(dead_code)]
        const PF_SEG1: usize = 0xfff8 - 0xfff8;
        const PF_SEG2: usize = 0xfff9 - 0xfff8;
        const PF_66: usize = 0xfffa - 0xfff8;
        const PF_67: usize = 0xfffb - 0xfff8;
        const PF_LOCK: usize = 0xfffc - 0xfff8;
        const PF_REP: usize = 0xfffd - 0xfff8;
        const PF_REX: usize = 0xfffe - 0xfff8;

        let mut prefixes = [0u8; 8];
        let mut table_entry;

        loop {
            if self.cursor >= self.buf.len() {
                return self.partial();
            }

            let prefix = self.peek();

            table_entry = table_lookup(self.table_root_idx, prefix);
            if table_entry.wrapping_sub(0xfff8) >= 8 {
                break;
            }

            prefixes[PF_REX] = 0;
            prefixes[(table_entry.wrapping_sub(0xfff8)) as usize] = prefix;
            self.read_u8();
        }

        if self.cursor > start {
            if prefixes[PF_SEG2] != 0 {
                inst.segment = prefixes[PF_SEG2] >> 3 & 3;
            } else {
                inst.segment = prefixes[PF_SEG2] & 7;
            }

            if prefixes[PF_67] != 0 {
                addr_size -= 1;
            }

            prefix_rex = prefixes[PF_REX];
            prefix_rep = prefixes[PF_REP];
        }
        // table_entry kinds: INSTR(0), T16(1), ESCAPE_A(2), ESCAPE_B(3)

        'direct: loop {
            if table_entry & 2 == 0 {
                self.read_u8();
                if table_entry & 1 != 0 {
                    // Then, walk through ModR/M-encoded opcode extensions
                    if self.cursor >= self.buf.len() {
                        return self.partial();
                    }

                    let isreg = self.peek() >= 0xc0;
                    table_entry = table_walk(table_entry, ((self.peek() >> 2) & 0xe) | isreg as u8);
                    // table_entry kinds: INSTR(0), T8E(1)
                    if table_entry & 1 != 0 {
                        table_entry = table_walk(table_entry, self.peek() & 7);
                    }
                }
                // table_entry kinds: INSTR(0)
                break 'direct;
            }

            if self.cursor >= self.buf.len() {
                return self.partial();
            }

            let mut opcode_escape = 0;
            let mut mandatory_prefix = 0u8;

            if self.peek() == 0x0f {
                if self.cursor + 1 >= self.buf.len() {
                    return self.partial();
                }

                if self.peek1() == 0x38 {
                    opcode_escape = 2;
                } else if self.peek1() == 0x3a {
                    opcode_escape = 3;
                } else {
                    opcode_escape = 1;
                }

                self.cursor += if opcode_escape >= 2 { 2 } else { 1 };

                // If there is no REP/REPNZ prefix offer 66h as mandatory prefix. If
                // there is a REP prefix, then the 66h prefix is ignored here.
                mandatory_prefix = if prefix_rep != 0 {
                    prefix_rep ^ 0xf1
                } else {
                    (prefixes[PF_66] != 0) as u8
                };
            } else if self.peek().wrapping_sub(0xc4) < 2 || self.peek() == 0x62 {
                let vex_prefix = self.peek();

                if self.cursor + 1 >= self.buf.len() {
                    return self.partial();
                }

                if !self.mode.is_64() && self.peek1() < 0xc0 {
                    self.read_u8();
                    table_entry = table_walk(table_entry, 0);
                    // table_entry kinds: INSTR(0)
                    break 'direct;
                }

                if prefixes[PF_66] != 0 || prefixes[PF_REP] != 0 || prefix_rex != 0 {
                    return self.invalid();
                }

                let mut byte = self.peek1();
                if vex_prefix == 0xc5 {
                    opcode_escape = 1;
                    prefix_rex = if byte & 0x80 != 0 { 0 } else { PREFIX_REXR }
                } else
                // 3 byte vex or evex
                {
                    // SDM Vol 2A 2-15 (Dec. 2016): Ignored in 32-bit mode
                    if self.mode.is_64() {
                        prefix_rex = byte >> 5 ^ 0x7;
                    }

                    if vex_prefix == 0x62 {
                        // Bit 3 of opcode_escape must be clear.
                        if byte & 0x08 != 0 {
                            return self.invalid();
                        }

                        if self.mode.is_64() {
                            prefix_rex |= (byte & PREFIX_REXRR) ^ PREFIX_REXRR;
                        }
                    } else
                    // 3 byte VEX
                    {
                        // bits 4:3 of opcode_escape must be clear
                        if byte & 0x18 != 0 {
                            return self.invalid();
                        }
                    }

                    opcode_escape = byte & 0x07;

                    if opcode_escape == 0 {
                        let prefix_len = if vex_prefix == 0x62 { 4 } else { 3 };

                        if self.cursor + prefix_len > self.buf.len() {
                            return self.partial();
                        } else {
                            return self.invalid();
                        }
                    }

                    // load third byte of VEX prefix
                    if self.cursor + 2 >= self.buf.len() {
                        return self.partial();
                    }

                    byte = self.peek2();
                    prefix_rex |= if byte & 0x80 != 0 { PREFIX_REXW } else { 0 };
                }

                mandatory_prefix = byte & 3;
                vex_operand = ((byte & 0x78) >> 3) ^ 0xf;
                prefix_rex |= PREFIX_VEX;

                if vex_prefix == 0x62
                // EVEX
                {
                    // Bit 10 must be 1.
                    if byte & 0x04 == 0 {
                        return self.invalid();
                    }

                    if self.cursor + 3 >= self.buf.len() {
                        return self.partial();
                    }

                    byte = self.peek3();

                    vexl = (byte >> 5) & 3;
                    prefix_evex = byte as u32 | 0x100;
                    if self.mode.is_64() {
                        vex_operand |= if byte & 0x08 != 0 { 0 } else { 0x10 };
                    } else if byte & 0x08 == 0 {
                        return self.invalid();
                    }

                    self.cursor += 4;
                } else {
                    vexl = if byte & 0x04 != 0 { 1 } else { 0 };
                    self.cursor += 0xc7 - vex_prefix as usize; // 3 for c4, 2 for c5
                }
            }

            table_entry = table_walk(table_entry, opcode_escape);

            if table_entry == 0 {
                return self.invalid();
            }

            if self.cursor >= self.buf.len() {
                return self.partial();
            }

            table_entry = table_walk(table_entry, self.read_u8());

            // Handle mandatory prefixes (which behave like an opcode ext.).
            if table_entry & 3 == 3 {
                table_entry = table_walk(table_entry, mandatory_prefix);
            }

            if table_entry & 1 != 0 {
                if self.cursor >= self.buf.len() {
                    return self.partial();
                }

                let isreg = self.peek() >= 0xc0;

                table_entry = table_walk(table_entry, ((self.peek() >> 2) & 0xe) | isreg as u8);

                if table_entry & 1 != 0 {
                    table_entry = table_walk(table_entry, self.peek() & 7);
                }
            }

            // For VEX prefix, we have to distinguish between VEX.W and VEX.L which may
            // be part of the opcode.
            if table_entry & 2 != 0 {
                let mut index = 0;
                index |= if prefix_rex & PREFIX_REXW != 0 {
                    1 << 0
                } else {
                    0
                };
                index |= vexl << 1;
                table_entry = table_walk(table_entry, index);
            }

            break 'direct;
        }

        if table_entry == 0 {
            return self.invalid();
        }

        let desc = &TABLE_DESCS[table_entry as usize >> 2];

        inst.typ = desc.typ;
        inst.addrsz = addr_size;
        inst.flags = ((prefix_rep + 1) & 6) + if self.mode.is_64() { 128 } else { 0 };
        inst.address = self.address + start as u64;

        inst.operands = [Op::default(); 4];

        if desc.modrm() && self.cursor >= self.buf.len() {
            return self.partial();
        }

        if desc.modrm() {
            self.read_u8();
        }

        let op_byte = self.buf[self.cursor - 1] | (if desc.modrm() { 0 } else { 0xc0 });

        if prefix_evex != 0 {
            if desc.vsib() && (prefix_evex & 0x07 == 0 || prefix_evex & 0x80 != 0) {
                return self.invalid();
            }

            if !desc.evex_mask() && prefix_evex & 0x87 != 0 {
                return self.invalid();
            }

            if prefix_evex & 0x87 == 0x80 {
                return self.invalid();
            }

            if prefix_evex & 0x10 != 0 && op_byte & 0xc0 == 0xc0 {
                if !desc.evex_sae() {
                    return self.invalid();
                }

                vexl = 2;
                if desc.evex_er() {
                    inst.evex = prefix_evex as _;
                } else {
                    inst.evex = (prefix_evex & 0x87) as u8 | 0x60;
                }
            } else {
                if vexl == 3 {
                    return self.invalid();
                }

                inst.evex = (prefix_evex & 0x87) as u8;
            }

            if desc.vsib() {
                vex_operand &= 0xf;
            }
        } else {
            inst.evex = 0;
        }

        let op_size: u8;
        let mut op_size_alt = 0;

        if desc.opsize() & 4 == 0 {
            if self.mode.is_64() {
                op_size = if (prefix_rex & PREFIX_REXW != 0) || desc.opsize() == 3 {
                    4
                } else if prefixes[PF_66] != 0 && !desc.ign66() {
                    2
                } else if desc.opsize() != 0 {
                    4
                } else {
                    3
                };
            } else {
                op_size = if prefixes[PF_66] != 0 && !desc.ign66() {
                    2
                } else {
                    3
                };
            }
        } else {
            op_size = 5 + vexl as u8;
            op_size_alt = op_size - (desc.opsize() as u8 & 3);
        }

        let operand_sizes = [desc.size_fix1(), desc.size_fix2() + 2, op_size, op_size_alt];
        'skip_modrm: loop {
            if matches!(inst.typ, Opcode::MOV_CR | Opcode::MOV_DR) {
                let modreg = (op_byte >> 3) & 0x7;
                let modrm = op_byte & 0x7;

                let op_modreg = &mut inst.operands[desc.modreg_idx() as usize];
                op_modreg.typ = OperandType::Reg;
                op_modreg.size = op_size;
                op_modreg.reg = modreg | if prefix_rex & PREFIX_REXR != 0 { 8 } else { 0 };
                op_modreg.misc = if matches!(inst.typ, Opcode::MOV_CR) {
                    RegType::Cr as u8
                } else {
                    RegType::Dr as u8
                };

                if matches!(inst.typ, Opcode::MOV_CR) && (!0x011d >> op_modreg.reg) & 1 != 0 {
                    return self.invalid();
                } else if matches!(inst.typ, Opcode::MOV_DR) && prefix_rex & PREFIX_REXR != 0 {
                    return self.invalid();
                }

                let op_modrm = &mut inst.operands[desc.modrm_idx() as usize];
                op_modrm.typ = OperandType::Reg;
                op_modrm.size = op_size;
                op_modrm.reg = modrm | if prefix_rex & PREFIX_REXB != 0 { 8 } else { 0 };
                break 'skip_modrm;
            }

            if desc.has_modreg() {
                let op_modreg = &mut inst.operands[desc.modreg_idx() as usize];
                let mut reg_idx = (op_byte & 0x38) as usize >> 3;
                let reg_ty = desc.regty_modreg();

                op_modreg.misc = reg_ty;

                if reg_ty < 2 {
                    reg_idx += if prefix_rex & PREFIX_REXR != 0 { 8 } else { 0 };
                } else if reg_idx == 7 && (prefix_rex & PREFIX_REXR != 0 || prefix_evex & 0x80 != 0)
                {
                    return self.invalid();
                }
                if reg_ty == RegType::Vec as u8 {
                    reg_idx += if prefix_rex & PREFIX_REXRR != 0 {
                        16
                    } else {
                        0
                    };
                } else if prefix_rex & PREFIX_REXRR != 0 {
                    return self.invalid();
                }

                op_modreg.typ = OperandType::Reg;
                op_modreg.size = operand_sizes[desc.modreg_size() as usize];
                op_modreg.reg = reg_idx as _;
            }

            if desc.has_modrm() {
                let op_modrm = &mut inst.operands[desc.modrm_idx() as usize];
                op_modrm.size = operand_sizes[desc.modrm_size() as usize];

                let rm = op_byte & 0x07;
                'end_modrm: loop {
                    if op_byte >= 0xc0 {
                        let mut reg_idx = rm;
                        let reg_ty = desc.regty_modrm();
                        op_modrm.misc = reg_ty;
                        if reg_ty < 2 {
                            reg_idx += if prefix_rex & PREFIX_REXB != 0 { 8 } else { 0 };
                        }

                        if prefix_evex != 0 && reg_ty == 0 {
                            reg_idx += if prefix_rex & PREFIX_REXX != 0 { 16 } else { 0 };
                        }

                        op_modrm.typ = OperandType::Reg;
                        op_modrm.reg = reg_idx;
                    } else {
                        let mut dispscale = 0;
                        if prefix_evex != 0 {
                            if prefix_evex & 0x80 != 0 && desc.modrm_idx() == 0 {
                                return self.invalid();
                            }

                            if prefix_evex & 0x10 != 0 {
                                if !desc.evex_bcst() {
                                    return self.invalid();
                                }

                                if desc.evex_bcst16() {
                                    dispscale = 1;
                                } else {
                                    dispscale = if prefix_rex & PREFIX_REXW != 0 { 3 } else { 2 };
                                }

                                inst.segment |= dispscale << 6;
                                op_modrm.typ = OperandType::MemBCST;
                            } else {
                                dispscale = op_modrm.size - 1;
                                op_modrm.typ = OperandType::Mem;
                            }
                        } else {
                            op_modrm.typ = OperandType::Mem;
                        }

                        if addr_size == 1 {
                            assert!(!self.mode.is_64());

                            if desc.vsib() {
                                return self.invalid();
                            }

                            if rm < 6 {
                                op_modrm.misc = if rm & 1 != 0 {
                                    Reg::DI as u8
                                } else {
                                    Reg::SI as u8
                                };

                                if rm < 4 {
                                    op_modrm.reg = if rm & 2 != 0 {
                                        Reg::BP as u8
                                    } else {
                                        Reg::BX as u8
                                    };
                                } else if rm < 6 || op_byte & 0xc7 == 0x06 {
                                    op_modrm.reg = Reg::None as u8;
                                } else {
                                    op_modrm.reg = if rm == 6 {
                                        Reg::BP as u8
                                    } else {
                                        Reg::BX as u8
                                    };
                                }

                                let dispbase = &self.buf[self.cursor..];

                                if op_byte & 0x40 != 0 {
                                    if self.cursor + 1 >= self.buf.len() {
                                        return self.partial();
                                    }
                                    self.read_u8();

                                    inst.disp = ((load_le_1(dispbase) as i8) << dispscale) as i64;
                                } else if op_byte & 0x80 != 0 || op_byte & 0xc7 == 0x06 {
                                    if self.cursor + 2 >= self.buf.len() {
                                        return self.partial();
                                    }

                                    self.cursor += 2;
                                    inst.disp = load_le_2(dispbase) as i16 as i64;
                                } else {
                                    inst.disp = 0;
                                }
                                break 'end_modrm;
                            }

                            // SIB byte
                            let mut base = rm;
                            if rm == 4 {
                                if self.cursor >= self.buf.len() {
                                    return self.partial();
                                }

                                let sib = self.read_u8();
                                let scale = sib & 0xc0;
                                let mut idx = (sib & 0x38) >> 3;
                                idx += if prefix_rex & PREFIX_REXX != 0 { 8 } else { 0 };
                                base = sib & 0x07;
                                if idx == 4 {
                                    idx = Reg::None as u8;
                                }
                                op_modrm.misc = scale | idx;
                            } else {
                                op_modrm.misc = Reg::None as u8;
                            }

                            if desc.vsib() {
                                if rm != 4 {
                                    return self.invalid();
                                }

                                if op_modrm.misc & 0x3f == Reg::None as u8 {
                                    op_modrm.misc &= 0xc4;
                                }

                                if prefix_evex != 0 {
                                    op_modrm.misc |= if prefix_evex & 0x8 != 0 { 0 } else { 0x10 };
                                }
                            }

                            if op_byte < 0x40 && rm == 5 && self.mode.is_64() {
                                op_modrm.reg = Reg::IP as u8;
                            } else if op_byte < 0x40 && base == 5 {
                                op_modrm.reg = Reg::None as u8;
                            } else {
                                op_modrm.reg =
                                    base + if prefix_rex & PREFIX_REXB != 0 { 8 } else { 0 };
                            }

                            let dispbase = &self.buf[self.cursor..];

                            if op_byte & 0x40 != 0 {
                                if self.cursor + 1 > self.buf.len() {
                                    return self.partial();
                                }

                                self.read_u8();
                                inst.disp =
                                    ((load_le_1(dispbase) as i8 as i64) << dispscale) as i64;
                            } else if op_byte & 0x80 != 0 || (op_byte < 0x40 && base == 5) {
                                if self.cursor + 4 > self.buf.len() {
                                    return self.partial();
                                }
                                self.cursor += 4;
                                inst.disp = load_le_4(dispbase) as i32 as i64;
                            } else {
                                inst.disp = 0;
                            }
                        }
                        break;
                    }

                    break;
                }
            }

            if desc.has_vexreg() {
                let operand = &mut inst.operands[desc.vexreg_idx() as usize];

                if desc.zeroreg_val() {
                    operand.typ = OperandType::Reg;
                    operand.size = 1;
                    operand.reg = Reg::CL as u8;
                    operand.misc = RegType::Gpl as u8;
                } else {
                    operand.typ = OperandType::Reg;
                    operand.size = operand_sizes[desc.vexreg_size() as usize];
                    if !self.mode.is_64() {
                        vex_operand &= 0x7;
                    }

                    operand.reg = vex_operand as _;

                    let reg_ty = desc.regty_vexreg();

                    if prefix_rex & PREFIX_VEX != 0 {
                        if reg_ty == 2 && vex_operand >= 8 {
                            return self.invalid();
                        }

                        if reg_ty == 3 {
                            operand.reg &= 0x7;
                        }

                        operand.misc = ((3528u32 >> (3 * reg_ty as u32)) & 0x7) as u8;
                    } else {
                        operand.misc = ((2504u32 >> (3 * reg_ty as u32)) & 0x7) as u8
                    }
                }
            } else if vex_operand != 0 {
                return self.invalid();
            }

            let imm_control = desc.imm_control();

            if imm_control == 0 {
            } else if imm_control == 1 {
                let operand = &mut inst.operands[desc.imm_idx() as usize];
                operand.typ = OperandType::Imm;
                operand.size = 1;
                inst.imm = 1;
            } else if imm_control == 2 {
                // 2 = memory, address-sized, used for mov with moffs operand
                let operand = &mut inst.operands[desc.imm_idx() as usize];
                operand.typ = OperandType::Mem;
                operand.size = operand_sizes[desc.imm_size() as usize];
                operand.reg = Reg::None as u8;
                operand.misc = Reg::None as u8;

                let moffsz = 1usize << addr_size;
                if self.cursor + moffsz > self.buf.len() {
                    return self.partial();
                }

                if moffsz == 2 {
                    inst.disp = load_le_2(&self.buf[self.cursor..]) as _;
                }
                if moffsz == 4 {
                    inst.disp = load_le_4(&self.buf[self.cursor..]) as _;
                }

                if moffsz == 8 {
                    inst.disp = load_le_8(&self.buf[self.cursor..]) as _;
                }

                self.cursor += moffsz;
            } else if imm_control == 3 {
                let operand = &mut inst.operands[desc.imm_idx() as usize];
                operand.typ = OperandType::Reg;
                operand.size = op_size;

                operand.misc = RegType::Vec as u8;

                if self.cursor + 1 > self.buf.len() {
                    return self.partial();
                }

                let mut reg = load_le_1(&self.buf[self.cursor..]) as u8;
                self.cursor += 1;

                if !self.mode.is_64() {
                    reg &= 0x7f;
                }

                operand.reg = reg >> 4;
                inst.imm = (reg & 0x0f) as i64;
            } else if imm_control != 0 {
                // 4/5 = immediate, operand-sized/8 bit
                // 6/7 = offset, operand-sized/8 bit (used for jumps/calls)
                let imm_byte = imm_control & 1;
                let imm_offset = imm_control & 2;

                let operand = &mut inst.operands[desc.imm_idx() as usize];
                operand.typ = OperandType::Imm;

                if imm_byte != 0 {
                    if self.cursor + 1 > self.buf.len() {
                        return self.partial();
                    }

                    inst.imm = load_le_1(&self.buf[self.cursor..]) as i8 as i64;
                    self.cursor += 1;
                    operand.size = if desc.imm_size() & 1 != 0 { 1 } else { op_size };
                } else {
                    operand.size = operand_sizes[desc.imm_size() as usize];

                    let imm_size = if matches!(
                        inst.typ,
                        Opcode::RET | Opcode::RETF | Opcode::SSE_EXTRQ | Opcode::SSE_INSERTQ
                    ) {
                        2
                    } else if matches!(inst.typ, Opcode::JMPF | Opcode::CALLF) {
                        (1 << op_size >> 1) + 2
                    } else if matches!(inst.typ, Opcode::ENTER) {
                        3
                    } else {
                        if op_size == 2 {
                            2
                        } else {
                            4
                        }
                    };

                    if self.cursor + imm_size > self.buf.len() {
                        return self.partial();
                    }

                    if imm_size == 2 {
                        inst.imm = load_le_2(&self.buf[self.cursor..]) as i16 as i64;
                    } else if imm_size == 3 {
                        inst.imm = load_le_3(&self.buf[self.cursor..]) as i32 as i64;
                    } else if imm_size == 4 {
                        inst.imm = load_le_4(&self.buf[self.cursor..]) as i32 as i64;
                    } else if imm_size == 8 {
                        inst.imm = load_le_8(&self.buf[self.cursor..]) as i64;
                    }

                    self.cursor += imm_size;
                }

                if imm_offset != 0 {
                    if inst.address != 0 {
                        inst.imm += inst.address as i64 + self.cursor as i64;
                    } else {
                        operand.typ = OperandType::Off;
                    }
                }
            }
            break 'skip_modrm;
        }

        if prefixes[PF_LOCK] != 0 {
            if !desc.lock() || inst.operands[0].typ != OperandType::Mem {
                return self.invalid();
            }

            inst.flags |= 1;
        }

        if desc.legacy() {
            if prefix_rex & PREFIX_REX == 0 {
                for i in 0..2 {
                    let operand = &mut inst.operands[i];
                    if operand.typ == OperandType::None {
                        break;
                    }

                    if operand.typ == OperandType::Reg
                        && operand.misc == RegType::Gpl as u8
                        && operand.size == 1
                        && operand.reg >= 4
                    {
                        operand.misc = RegType::Gph as u8;
                    }
                }
            }

            if inst.typ == Opcode::XCHG_NOP {
                if inst.operands[0].reg == 0 && inst.operands[1].reg == 0 {
                    inst.operands[0].typ = OperandType::None;
                    inst.operands[1].typ = OperandType::None;
                    inst.typ = Opcode::NOP;
                } else {
                    inst.typ = Opcode::XCHG;
                }
            }

            if inst.typ == Opcode::_3DNOW {
                let opc3dn = inst.imm;

                if opc3dn & 0x40 != 0 {
                    return self.invalid();
                }

                let msk = if opc3dn & 0x80 != 0 {
                    0x88d144d144d14400u64 as i64
                } else {
                    0x30003000i64
                };

                if (msk >> (opc3dn & 0x3f) & 1) == 0 {
                    return self.invalid();
                }

                inst.operandsz = if desc.instr_width() { op_size - 1 } else { 0 };
            } else {
                inst.operandsz = 0;
            }
        }
        inst.typ = desc.typ;
        inst.size = (self.cursor - start as usize) as u8;
    }

    pub fn decode(&mut self) -> Instruction {
        let mut inst = Instruction::default();
        self.decode_out(&mut inst);
        inst
    }

    fn partial(&mut self) {
        self.error = DecoderError::NoMoreBytes;
    }

    fn invalid(&mut self) {
        self.error = DecoderError::InvalidInstruction;
    }

    pub fn can_decode(&self) -> bool {
        self.cursor < self.buf.len()
    }

    fn peek(&self) -> u8 {
        self.buf[self.cursor]
    }

    fn peek1(&self) -> u8 {
        self.buf[self.cursor + 1]
    }

    fn peek2(&self) -> u8 {
        self.buf[self.cursor + 2]
    }

    fn peek3(&self) -> u8 {
        self.buf[self.cursor + 3]
    }

    fn read_u8(&mut self) -> u8 {
        let ret = self.buf[self.cursor];
        self.cursor += 1;
        return ret;
    }
}
