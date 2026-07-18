//! X86 encoder lookup tables (port of the constant tables in AsmJit's `x86assembler.cpp`).
//!
//! Tables indexed by register type are REGENERATED from the template logic using asmkit's
//! [`RegType`] numbering — AsmJit's `RegType` discriminants differ (asmkit inserted
//! `SymTag`/`PC` ahead of the GP/Vec range), so transliterating raw table bytes would be
//! wrong.
//!
//! Derived from AsmJit (Zlib license) — this file is an altered version; see LICENSE notices.

use crate::core::operand::RegType;

use super::opcode::Opcode;

// X86 bytes used to encode important prefixes.
// --------------------------------------------

/// 1-byte REX prefix mask.
pub const X86_BYTE_REX: u8 = 0x40;
/// 1-byte REX.W component.
pub const X86_BYTE_REX_W: u8 = 0x08;
pub const X86_BYTE_INVALID_REX: u8 = 0x80;
/// 2-byte VEX prefix lead byte (0xC5).
pub const X86_BYTE_VEX2: u8 = 0xC5;
/// 3-byte VEX prefix lead byte (0xC4).
pub const X86_BYTE_VEX3: u8 = 0xC4;
/// 3-byte XOP prefix lead byte (0x8F).
pub const X86_BYTE_XOP3: u8 = 0x8F;
/// 4-byte EVEX prefix lead byte (0x62).
pub const X86_BYTE_EVEX: u8 = 0x62;

/// AsmJit specific (used to encode the VVVVV field in XOP/VEX/EVEX).
pub const VEX_VVVVV_SHIFT: u32 = 7;
pub const VEX_VVVVV_MASK: u32 = 0x1F << VEX_VVVVV_SHIFT;

/// Mandatory prefixes used to encode legacy [66, F3, F2] or [9B] byte.
pub static OPCODE_PP_TABLE: [u8; 8] = [0x00, 0x66, 0xF3, 0xF2, 0x00, 0x00, 0x00, 0x9B];

/// Instruction 2-byte/3-byte opcode prefix definition.
#[derive(Clone, Copy, Debug, Default)]
pub struct X86OpcodeMm {
    pub size: u8,
    pub data: [u8; 3],
}

impl X86OpcodeMm {
    pub const fn new(size: u8, data: [u8; 3]) -> Self {
        Self { size, data }
    }
}

/// Instruction 2-byte/3-byte opcode prefix data, indexed by the MM field (bits [11:8]).
#[rustfmt::skip]
pub static OPCODE_MM_TABLE: [X86OpcodeMm; 16] = [
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #00
    X86OpcodeMm::new(1, [0x0F, 0x00, 0]), // #01
    X86OpcodeMm::new(2, [0x0F, 0x38, 0]), // #02
    X86OpcodeMm::new(2, [0x0F, 0x3A, 0]), // #03
    X86OpcodeMm::new(2, [0x0F, 0x01, 0]), // #04
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #05
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #06
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #07
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #08
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #09
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #0A
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #0B
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #0C
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #0D
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #0E
    X86OpcodeMm::new(0, [0x00, 0x00, 0]), // #0F
];

/// Segment override prefix bytes indexed by segment register id (0 = none, 1..6 = ES..GS).
#[rustfmt::skip]
pub static SEGMENT_PREFIX_TABLE: [u8; 8] = [
    0x00, // None.
    0x26, // ES.
    0x2E, // CS.
    0x36, // SS.
    0x3E, // DS.
    0x64, // FS.
    0x65, // GS.
    0x00, // (unused)
];

/// PUSH of a segment register, indexed by segment register id.
#[rustfmt::skip]
pub static OPCODE_PUSH_SREG_TABLE: [u32; 8] = [
    Opcode::K000000 | 0x00, // None.
    Opcode::K000000 | 0x06, // Push ES.
    Opcode::K000000 | 0x0E, // Push CS.
    Opcode::K000000 | 0x16, // Push SS.
    Opcode::K000000 | 0x1E, // Push DS.
    Opcode::K000F00 | 0xA0, // Push FS.
    Opcode::K000F00 | 0xA8, // Push GS.
    0,
];

/// POP of a segment register, indexed by segment register id.
#[rustfmt::skip]
pub static OPCODE_POP_SREG_TABLE: [u32; 8] = [
    Opcode::K000000 | 0x00, // None.
    Opcode::K000000 | 0x07, // Pop ES.
    Opcode::K000000 | 0x00, // Pop CS.
    Opcode::K000000 | 0x17, // Pop SS.
    Opcode::K000000 | 0x1F, // Pop DS.
    Opcode::K000F00 | 0xA1, // Pop FS.
    Opcode::K000F00 | 0xA9, // Pop GS.
    0,
];

// Memory operand info bits (X86MemInfo)
// -------------------------------------

/// Has BASE reg, REX.B can be 1, compatible with REX.B byte.
pub const MEM_INFO_BASE_GP: u8 = 0x01;
/// Has INDEX reg, REX.X can be 1, compatible with REX.X byte.
pub const MEM_INFO_INDEX: u8 = 0x02;
/// Base is a Label.
pub const MEM_INFO_BASE_LABEL: u8 = 0x10;
/// Base is RIP.
pub const MEM_INFO_BASE_RIP: u8 = 0x20;
/// Address-size override in 32-bit mode.
pub const MEM_INFO_67H_X86: u8 = 0x40;
/// Address-size override in 64-bit mode.
pub const MEM_INFO_67H_X64: u8 = 0x80;
/// Contains all address-size override bits.
pub const MEM_INFO_67H_MASK: u8 = 0xC0;

/// Computes [`MEM_INFO_TABLE`] entries (port of AsmJit's `X86MemInfo_T`).
///
/// `x` packs BASE register type in bits [4:0] and INDEX register type in bits [9:5].
const fn mem_info(x: u32) -> u8 {
    let b = x & 0x1F;
    let i = (x >> 5) & 0x1F;

    let (gp16, gp32, gp64, vec128, vec256, vec512) = (
        RegType::Gp16 as u32,
        RegType::Gp32 as u32,
        RegType::Gp64 as u32,
        RegType::Vec128 as u32,
        RegType::Vec256 as u32,
        RegType::Vec512 as u32,
    );

    let base = if b >= gp16 && b <= gp64 {
        MEM_INFO_BASE_GP
    } else if b == RegType::PC as u32 {
        MEM_INFO_BASE_RIP
    } else if b == RegType::LabelTag as u32 {
        MEM_INFO_BASE_LABEL
    } else {
        0
    };

    let index = if (i >= gp16 && i <= gp64) || (i >= vec128 && i <= vec512) {
        MEM_INFO_INDEX
    } else {
        0
    };

    let h67 = if (b == gp16 && i == RegType::None as u32)
        || (b == RegType::None as u32 && i == gp16)
        || (b == gp16 && i == gp16)
        || (b == gp16 && i == vec128)
        || (b == gp16 && i == vec256)
        || (b == gp16 && i == vec512)
        || (b == RegType::LabelTag as u32 && i == gp16)
    {
        MEM_INFO_67H_X86
    } else if (b == gp32 && i == RegType::None as u32)
        || (b == RegType::None as u32 && i == gp32)
        || (b == gp32 && i == gp32)
        || (b == gp32 && i == vec128)
        || (b == gp32 && i == vec256)
        || (b == gp32 && i == vec512)
        || (b == RegType::LabelTag as u32 && i == gp32)
    {
        MEM_INFO_67H_X64
    } else {
        0
    };

    base | index | h67 | 0x04 | 0x08
}

/// Lookup table with memory operand info based on BASE and INDEX register types
/// (see [`mem_info`]).
pub static MEM_INFO_TABLE: [u8; 1024] = {
    let mut table = [0u8; 1024];
    let mut x = 0usize;
    while x < 1024 {
        table[x] = mem_info(x as u32);
        x += 1;
    }
    table
};

/// VEX3 or XOP xor bits applied to the opcode before emission, indexed by the 'mmmmm'
/// value. Used only by 3-byte VEX and XOP prefixes; the idea is to minimize the difference
/// between VEX3 vs XOP encoding.
pub static VEX_PREFIX_TABLE: [u32; 16] = {
    let mut table = [0u32; 16];
    let mut x = 0usize;
    while x < 16 {
        table[x] = (if x & 0x08 != 0 {
            X86_BYTE_XOP3 as u32
        } else {
            X86_BYTE_VEX3 as u32
        }) | (0xF << 19)
            | (0x7 << 13);
        x += 1;
    }
    table
};

/// LL opcode field addressed by register size / 16 (propagates L.256/L.512).
pub static LL_BY_SIZE_DIV_16_TABLE: [u32; 16] = {
    let mut table = [0u32; 16];
    let mut x = 0usize;
    while x < 16 {
        table[x] = if x & (64 >> 4) != 0 {
            2 << Opcode::LL_SHIFT
        } else if x & (32 >> 4) != 0 {
            1 << Opcode::LL_SHIFT
        } else {
            0
        };
        x += 1;
    }
    table
};

/// LL opcode field addressed by register type (propagates L.256/L.512).
pub static LL_BY_REG_TYPE_TABLE: [u32; 16] = {
    let mut table = [0u32; 16];
    let mut x = 0usize;
    while x < 16 {
        table[x] = if x == RegType::Vec512 as usize {
            2 << Opcode::LL_SHIFT
        } else if x == RegType::Vec256 as usize {
            1 << Opcode::LL_SHIFT
        } else {
            0
        };
        x += 1;
    }
    table
};

/// Compressed-displacement scale (shift left) based on the 'TTWLL' field and the
/// instruction's tuple-type (TT) field (port of AsmJit's `X86CDisp8SHL_T`).
pub static CDISP8_SHL_TABLE: [u32; 32] = {
    let mut table = [0u32; 32];
    let mut x = 0usize;
    while x < 32 {
        let tt = (x >> 3) as u32;
        let ll = (x & 0x3) as u32;
        let w = ((x >> 2) & 0x1) as u32;
        let shift = match tt {
            0 => 0, // CDTT_None
            1 => {
                if ll == 0 {
                    0
                } else if ll == 1 {
                    1
                } else {
                    2
                }
            } // CDTT_ByLL
            2 => {
                if ll == 0 {
                    w
                } else if ll == 1 {
                    1 + w
                } else {
                    2 + w
                }
            } // CDTT_T1W
            3 => {
                if ll == 0 {
                    0
                } else if ll == 1 {
                    2
                } else {
                    3
                }
            } // CDTT_DUP
            _ => unreachable!(),
        };
        table[x] = shift << Opcode::CDSHL_SHIFT;
        x += 1;
    }
    table
};

/// MOD byte of a 16-bit `[BASE + disp]` address (0xFF = invalid).
#[rustfmt::skip]
pub static MOD16_BASE_TABLE: [u8; 8] = [
    0xFF, // AX -> N/A.
    0xFF, // CX -> N/A.
    0xFF, // DX -> N/A.
    0x07, // BX -> 111.
    0xFF, // SP -> N/A.
    0x06, // BP -> 110.
    0x04, // SI -> 100.
    0x05, // DI -> 101.
];

/// MOD byte of a 16-bit `[BASE + INDEX + disp]` combination (0xFF = invalid).
pub static MOD16_BASE_INDEX_TABLE: [u8; 64] = {
    const BX: u32 = 3;
    const BP: u32 = 5;
    const SI: u32 = 6;
    const DI: u32 = 7;
    let mut table = [0xFFu8; 64];
    let mut x = 0usize;
    while x < 64 {
        let b = (x >> 3) as u32;
        let i = (x & 0x7) as u32;
        table[x] = if (b == BX && i == SI) || (b == SI && i == BX) {
            0x00
        } else if (b == BX && i == DI) || (b == DI && i == BX) {
            0x01
        } else if (b == BP && i == SI) || (b == SI && i == BP) {
            0x02
        } else if (b == BP && i == DI) || (b == DI && i == BP) {
            0x03
        } else {
            0xFF
        };
        x += 1;
    }
    table
};

/// Multi-byte NOP table for code alignment (Intel SDM Vol. 2B, NOP).
///
/// `NOP_TABLE[n - 1]` is the optimal n-byte NOP slide, n in 1..=9.
#[rustfmt::skip]
pub static NOP_TABLE: [[u8; 9]; 9] = [
    [0x90, 0, 0, 0, 0, 0, 0, 0, 0],
    [0x66, 0x90, 0, 0, 0, 0, 0, 0, 0],
    [0x0F, 0x1F, 0x00, 0, 0, 0, 0, 0, 0],
    [0x0F, 0x1F, 0x40, 0x00, 0, 0, 0, 0, 0],
    [0x0F, 0x1F, 0x44, 0x00, 0x00, 0, 0, 0, 0],
    [0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00, 0, 0, 0],
    [0x0F, 0x1F, 0x80, 0x00, 0x00, 0x00, 0x00, 0, 0],
    [0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00, 0],
    [0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00],
];

#[cfg(test)]
mod tests {
    use super::*;

    const fn rt(t: RegType) -> u32 {
        t as u32
    }

    #[test]
    fn mem_info_spot_checks() {
        let (gp16, gp32, gp64, vec256) = (
            rt(RegType::Gp16),
            rt(RegType::Gp32),
            rt(RegType::Gp64),
            rt(RegType::Vec256),
        );
        let at = |b: u32, i: u32| MEM_INFO_TABLE[(b | (i << 5)) as usize];

        // No base, no index: only the passthrough bits.
        assert_eq!(at(0, 0), 0x04 | 0x08);
        // 64-bit GP base: base flag, no address override.
        assert_eq!(at(gp64, 0), MEM_INFO_BASE_GP | 0x04 | 0x08);
        // 16-bit GP base alone: X86 address override.
        assert_eq!(at(gp16, 0), MEM_INFO_BASE_GP | MEM_INFO_67H_X86 | 0x0C);
        // 32-bit GP base + vector index: X64 address override + index flag.
        assert_eq!(
            at(gp32, vec256),
            MEM_INFO_BASE_GP | MEM_INFO_INDEX | MEM_INFO_67H_X64 | 0x0C
        );
        // RIP base.
        assert_eq!(at(rt(RegType::PC), 0), MEM_INFO_BASE_RIP | 0x0C);
        // Label base + 32-bit index.
        assert_eq!(
            at(rt(RegType::LabelTag), gp32),
            MEM_INFO_BASE_LABEL | MEM_INFO_INDEX | MEM_INFO_67H_X64 | 0x0C
        );
    }

    #[test]
    fn ll_tables() {
        assert_eq!(LL_BY_SIZE_DIV_16_TABLE[1], 0);
        assert_eq!(LL_BY_SIZE_DIV_16_TABLE[2], 1 << Opcode::LL_SHIFT);
        assert_eq!(LL_BY_SIZE_DIV_16_TABLE[4], 2 << Opcode::LL_SHIFT);
        assert_eq!(
            LL_BY_REG_TYPE_TABLE[RegType::Vec512 as usize],
            2 << Opcode::LL_SHIFT
        );
        assert_eq!(
            LL_BY_REG_TYPE_TABLE[RegType::Vec256 as usize],
            1 << Opcode::LL_SHIFT
        );
        assert_eq!(LL_BY_REG_TYPE_TABLE[RegType::Vec128 as usize], 0);
    }

    #[test]
    fn cdisp8_shl_spot_checks() {
        // CDTT_None never scales.
        assert_eq!(CDISP8_SHL_TABLE[0], 0);
        // CDTT_ByLL (tt=1): shift == min(LL, 2) → x = (1<<3)|ll.
        assert_eq!(CDISP8_SHL_TABLE[8], 0);
        assert_eq!(CDISP8_SHL_TABLE[9], 1 << Opcode::CDSHL_SHIFT);
        assert_eq!(CDISP8_SHL_TABLE[11], 2 << Opcode::CDSHL_SHIFT);
        // CDTT_DUP (tt=3): LL 0,1,2 -> 0,2,3.
        assert_eq!(CDISP8_SHL_TABLE[24], 0);
        assert_eq!(CDISP8_SHL_TABLE[25], 2 << Opcode::CDSHL_SHIFT);
        assert_eq!(CDISP8_SHL_TABLE[26], 3 << Opcode::CDSHL_SHIFT);
    }

    #[test]
    fn mod16_tables() {
        assert_eq!(MOD16_BASE_TABLE[3], 0x07); // BX
        assert_eq!(MOD16_BASE_TABLE[0], 0xFF); // AX invalid
        assert_eq!(MOD16_BASE_INDEX_TABLE[(3 << 3) | 6], 0x00); // BX+SI
        assert_eq!(MOD16_BASE_INDEX_TABLE[(7 << 3) | 5], 0x03); // DI+BP
        assert_eq!(MOD16_BASE_INDEX_TABLE[0], 0xFF);
    }
}
