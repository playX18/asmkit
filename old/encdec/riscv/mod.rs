//! # RISC-V instruction encoder
//!
//! All functions in this module are auto-generated. It's possible to use this module for both decoding and encoding.
//!
//! Main goal of the encoder is to be as fast as possible and `pub const fn` if possible.
//!
//! # Notes
//! - Arguments to opcodes are NOT verified
//! - Short opcodes are supported, they're prefixed with `c_` and return `u16` on success
//! - Immediates are NOT verified. If immediate you pass is too large to be encoded it might end up as 0 or
//! any other value depending on immediate encoding.
//!
//! # WARNING
//! Auto-generated from riscv-opcodes repo tables.
#![allow(unused_mut)]
pub mod decode;
pub mod formatter;

mod token;

#[derive(Clone, Copy)]
pub struct Immediate {
    position_in_opcode: (u32, u32),
    position_in_immediate: (u32, u32),
}

pub const UIMM_LO: Immediate = Immediate {
    position_in_opcode: (31, 12),
    position_in_immediate: (31, 12),
};

pub const S_TYPE_HI: Immediate = Immediate {
    position_in_immediate: (11, 5),
    position_in_opcode: (31, 25),
};

pub const S_TYPE_LO: Immediate = Immediate {
    position_in_immediate: (4, 0),
    position_in_opcode: (11, 7),
};

// imm[20]: Highest bit of the 21-bit immediate
pub const UJ_TYPE_IMM_20: Immediate = Immediate {
    position_in_immediate: (20, 20),
    position_in_opcode: (31, 31),
};

// imm[10:1]: Lower 10 bits of the immediate value
pub const UJ_TYPE_IMM_10_1: Immediate = Immediate {
    position_in_immediate: (10, 1),
    position_in_opcode: (30, 21),
};

// imm[11]: The 11th bit of the immediate
pub const UJ_TYPE_IMM_11: Immediate = Immediate {
    position_in_immediate: (11, 11),
    position_in_opcode: (20, 20),
};

// imm[19:12]: Upper 8 bits (bits 19 to 12)
pub const UJ_TYPE_IMM_19_12: Immediate = Immediate {
    position_in_immediate: (19, 12),
    position_in_opcode: (19, 12),
};

pub const I_TYPE_11_0: Immediate = Immediate {
    position_in_immediate: (11, 0),
    position_in_opcode: (31, 20),
};

// imm[12]: Highest bit of the immediate
pub const B_TYPE_IMM_12: Immediate = Immediate {
    position_in_immediate: (12, 12),
    position_in_opcode: (31, 31),
};

// imm[10:5]: Middle bits of the immediate
pub const B_TYPE_IMM_10_5: Immediate = Immediate {
    position_in_immediate: (10, 5),
    position_in_opcode: (30, 25),
};

// imm[4:1]: Lower bits of the immediate
pub const B_TYPE_IMM_4_1: Immediate = Immediate {
    position_in_immediate: (4, 1),
    position_in_opcode: (11, 8),
};

// imm[11]: 11th bit of the immediate
pub const B_TYPE_IMM_11: Immediate = Immediate {
    position_in_immediate: (11, 11),
    position_in_opcode: (7, 7),
};

// imm[11]: The highest bit of the immediate
pub const CJ_TYPE_IMM_11: Immediate = Immediate {
    position_in_immediate: (11, 11),
    position_in_opcode: (12, 12), // Bit 12 in the compressed instruction
};

// imm[4]: The 4th bit of the immediate
pub const CJ_TYPE_IMM_4: Immediate = Immediate {
    position_in_immediate: (4, 4),
    position_in_opcode: (11, 11), // Bit 11 in the compressed instruction
};

// imm[9:8]: Bits 9 and 8 of the immediate
pub const CJ_TYPE_IMM_9_8: Immediate = Immediate {
    position_in_immediate: (9, 8),
    position_in_opcode: (10, 9), // Bits 10 and 9
};

// imm[10]: The 10th bit of the immediate
pub const CJ_TYPE_IMM_10: Immediate = Immediate {
    position_in_immediate: (10, 10),
    position_in_opcode: (8, 8), // Bit 8
};

// imm[6]: The 6th bit of the immediate
pub const CJ_TYPE_IMM_6: Immediate = Immediate {
    position_in_immediate: (6, 6),
    position_in_opcode: (7, 7), // Bit 7
};

// imm[7]: The 7th bit of the immediate
pub const CJ_TYPE_IMM_7: Immediate = Immediate {
    position_in_immediate: (7, 7),
    position_in_opcode: (6, 6), // Bit 6
};

// imm[3:1]: Bits 3 to 1 of the immediate
pub const CJ_TYPE_IMM_3_1: Immediate = Immediate {
    position_in_immediate: (3, 1),
    position_in_opcode: (5, 3), // Bits 5 to 3
};

// imm[5]: The 5th bit of the immediate
pub const CJ_TYPE_IMM_5: Immediate = Immediate {
    position_in_immediate: (5, 5),
    position_in_opcode: (2, 2), // Bit 2
};

pub const BIMM12LOHI: &[Immediate] = &[
    B_TYPE_IMM_12,
    B_TYPE_IMM_11,
    B_TYPE_IMM_10_5,
    B_TYPE_IMM_4_1,
];
pub const S_TYPE: &[Immediate] = &[S_TYPE_HI, S_TYPE_LO];
pub const IMM20: &[Immediate] = &[UIMM_LO];
pub const JIMM20: &[Immediate] = &[
    UJ_TYPE_IMM_20,
    UJ_TYPE_IMM_19_12,
    UJ_TYPE_IMM_11,
    UJ_TYPE_IMM_10_1,
];
pub const I_TYPE: &[Immediate] = &[I_TYPE_11_0];
pub const SIMM5: &[Immediate] = &[Immediate {
    position_in_immediate: (4, 0),
    position_in_opcode: (19, 15),
}];

pub const IMM6: &[Immediate] = &[];
pub const IMM12LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (11, 5),
        position_in_opcode: (31, 25),
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (11, 7),
    },
];

pub const IMM12: &[Immediate] = &[Immediate {
    position_in_immediate: (11, 0),
    position_in_opcode: (31, 20),
}];

pub const ZIMM: &[Immediate] = &[Immediate {
    position_in_immediate: (4, 0),
    position_in_opcode: (19, 15),
}];

pub const ZIMM6LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (26, 26),
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (19, 15),
    },
];
pub const ZIMM11: &[Immediate] = &[Immediate {
    position_in_immediate: (10, 0),
    position_in_opcode: (30, 20),
}];

pub const ZIMM10: &[Immediate] = &[Immediate {
    position_in_immediate: (9, 0),
    position_in_opcode: (29, 20),
}];

pub const ZIMM5: &[Immediate] = &[Immediate {
    position_in_immediate: (4, 0),
    position_in_opcode: (19, 15),
}];

pub const C_SPIMM: &[Immediate] = &[Immediate {
    position_in_immediate: (5, 4),
    position_in_opcode: (3, 2),
}];

pub const C_UIMM8SP_S: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 2),
        position_in_opcode: (12, 9),
    },
    Immediate {
        position_in_immediate: (7, 6),
        position_in_opcode: (6, 5),
    },
];

pub const C_UIMM1: &[Immediate] = &[Immediate {
    position_in_immediate: (1, 1),
    position_in_opcode: (5, 5),
}];

pub const C_UIMM7LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 3),
        position_in_opcode: (12, 10),
    },
    Immediate {
        position_in_immediate: (2, 2),
        position_in_opcode: (6, 6),
    },
    Immediate {
        position_in_immediate: (6, 6),
        position_in_opcode: (5, 5),
    },
];

pub const C_UIMM9_SPLOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (12, 12),
    },
    Immediate {
        position_in_immediate: (4, 2),
        position_in_opcode: (6, 4),
    },
    Immediate {
        position_in_immediate: (7, 6),
        position_in_opcode: (3, 2),
    },
];

pub const C_UIMM9SP_S: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 3),
        position_in_opcode: (12, 9),
    },
    Immediate {
        position_in_immediate: (8, 6),
        position_in_opcode: (8, 6),
    },
];

pub const C_UIMM2: &[Immediate] = &[
    Immediate {
        position_in_immediate: (0, 0),
        position_in_opcode: (6, 6),
    },
    Immediate {
        position_in_immediate: (1, 1),
        position_in_opcode: (5, 5),
    },
];

pub const C_NZUIMM10: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 4),
        position_in_opcode: (12, 11),
    },
    Immediate {
        position_in_immediate: (9, 6),
        position_in_opcode: (10, 7),
    },
    Immediate {
        position_in_immediate: (2, 2),
        position_in_opcode: (6, 6),
    },
    Immediate {
        position_in_immediate: (3, 3),
        position_in_opcode: (5, 5),
    },
];

pub const C_NZIMM10LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (12, 12),
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (6, 2),
    },
];

/// UNUSED FOR NOW!!!
pub const C_UIMM10SP_S: &[Immediate] = &[];
/// UNUSED FOR NOW!!!
pub const C_UIMM10SPLOHI: &[Immediate] = &[];
/// UNUSED FOR NOW!!!
pub const C_UIMM9LOHI: &[Immediate] = &[];
/// UNUSED FOR NOW!!!
pub const IMM5: &[Immediate] = &[];
/// UNUSED FOR NOW!!!
pub const IMM4: &[Immediate] = &[];
/// UNUSED FOR NOW!!!
pub const IMM3: &[Immediate] = &[];
/// UNUSED FOR NOW!!!
pub const IMM2: &[Immediate] = &[];

pub const C_NZUIMM5: &[Immediate] = &C_NZUIMM6LOHI;

pub const C_NZIMM18LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (12, 12),
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (6, 2),
    },
];

// imm[17] | ... | imm[16:12]
pub const C_NZUIMM18LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (12, 12),
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (6, 2),
    },
];

pub const C_NZUIMM6LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (13, 13),
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (7, 2),
    },
];

pub const C_UIMM6LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (12, 12),
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (6, 2),
    },
];

pub const C_UIMM8LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 3),
        position_in_opcode: (12, 10),
    },
    Immediate {
        position_in_immediate: (7, 6),
        position_in_opcode: (7, 6),
    },
];

pub const C_UIMM9SPLOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (13, 13),
    },
    Immediate {
        position_in_immediate: (4, 3),
        position_in_opcode: (7, 5),
    },
    Immediate {
        position_in_immediate: (8, 6),
        position_in_opcode: (4, 3),
    },
];

pub const C_NZIMM6LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (12, 12),
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (6, 2),
    },
];

pub const C_BIMM9LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (8, 8),
        position_in_opcode: (12, 12),
    },
    Immediate {
        position_in_immediate: (4, 3),
        position_in_opcode: (11, 10),
    },
    Immediate {
        position_in_immediate: (7, 6),
        position_in_opcode: (6, 5),
    },
    Immediate {
        position_in_immediate: (2, 1),
        position_in_opcode: (4, 3),
    },
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (2, 2),
    },
];

pub const C_UIMM8SPLOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (12, 12),
    },
    Immediate {
        position_in_immediate: (4, 2),
        position_in_opcode: (6, 4),
    },
    Immediate {
        position_in_immediate: (7, 6),
        position_in_opcode: (3, 2),
    },
];

pub const C_UIMM8SP_SLOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 2),
        position_in_opcode: (12, 9),
    },
    Immediate {
        position_in_immediate: (7, 6),
        position_in_opcode: (8, 7),
    },
];

pub const C_IMM6LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (12, 12),
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (6, 2),
    },
];
pub const C_IMM12: &[Immediate] = &[
    Immediate {
        position_in_immediate: (11, 11),
        position_in_opcode: (12, 12),
    },
    Immediate {
        position_in_immediate: (4, 4),
        position_in_opcode: (11, 11),
    },
    Immediate {
        position_in_immediate: (9, 8),
        position_in_opcode: (10, 9),
    },
    Immediate {
        position_in_immediate: (10, 10),
        position_in_opcode: (8, 8),
    },
    Immediate {
        position_in_immediate: (6, 6),
        position_in_opcode: (7, 7),
    },
    Immediate {
        position_in_immediate: (7, 7),
        position_in_opcode: (6, 6),
    },
    Immediate {
        position_in_immediate: (3, 1),
        position_in_opcode: (5, 3),
    },
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (2, 2),
    },
];

pub const C_IMM12LOHI: &[Immediate] = &[
    Immediate {
        position_in_immediate: (11, 11),
        position_in_opcode: (12, 12),
    },
    Immediate {
        position_in_immediate: (4, 4),
        position_in_opcode: (11, 11),
    },
    Immediate {
        position_in_immediate: (9, 8),
        position_in_opcode: (10, 9),
    },
    Immediate {
        position_in_immediate: (10, 10),
        position_in_opcode: (8, 8),
    },
    Immediate {
        position_in_immediate: (6, 6),
        position_in_opcode: (7, 7),
    },
    Immediate {
        position_in_immediate: (7, 7),
        position_in_opcode: (6, 6),
    },
    Immediate {
        position_in_immediate: (3, 1),
        position_in_opcode: (5, 3),
    },
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (2, 2),
    },
];

pub const fn encode_immediate(immediate: &[Immediate], imm: i32) -> u32 {
    let mut res = 0;
    let mut i = 0;
    while i < immediate.len() {
        res |= immediate[i].encode(imm);
        i += 1;
    }
    res
}

pub const fn decode_immediate(immediate: &[Immediate], op: u32) -> i32 {
    let mut res = 0u32;
    let mut i = 0;
    while i < immediate.len() {
        res |= immediate[i].decode(op);
        i += 1;
    }
    res as _
}

pub fn is_immediate_valid(immediate: &[Immediate], imm: i32) -> bool {
    immediate.iter().all(|i| i.is_valid(imm))
}

impl Immediate {
    pub const fn encode(&self, imm: i32) -> u32 {
        let imm = imm as u32;
        let bit_count = self.position_in_immediate.0 - self.position_in_immediate.1 + 1;
        let mask = (1u32 << bit_count) - 1;

        (((imm >> self.position_in_immediate.1) as u32 & mask) << self.position_in_opcode.1) as u32
    }

    pub const fn decode(&self, op: u32) -> u32 {
        let bit_count = self.position_in_opcode.0 - self.position_in_opcode.1 + 1;
        let mask = (1u32 << bit_count) - 1;
        ((op as u32 >> self.position_in_opcode.1) as u32 & mask) << self.position_in_immediate.1
    }

    pub const fn is_valid(&self, imm: i32) -> bool {
        self.decode(self.encode(imm)) as i32 == imm
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Reg(pub u32);

impl Reg {
    pub const fn encode(self) -> u32 {
        self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VReg(u32);

impl VReg {
    pub const fn encode(self) -> u32 {
        self.0
    }
}

include!("inst.rs");
pub mod regs {
    use super::*;
    pub const X0: Reg = Reg(0);
    pub const X1: Reg = Reg(1);
    pub const X2: Reg = Reg(2);
    pub const X3: Reg = Reg(3);
    pub const X4: Reg = Reg(4);
    pub const X5: Reg = Reg(5);
    pub const X6: Reg = Reg(6);
    pub const X7: Reg = Reg(7);
    pub const X8: Reg = Reg(8);
    pub const X9: Reg = Reg(9);
    pub const X10: Reg = Reg(10);
    pub const X11: Reg = Reg(11);
    pub const X12: Reg = Reg(12);
    pub const X13: Reg = Reg(13);
    pub const X14: Reg = Reg(14);
    pub const X15: Reg = Reg(15);
    pub const X16: Reg = Reg(16);
    pub const X17: Reg = Reg(17);
    pub const X18: Reg = Reg(18);
    pub const X19: Reg = Reg(19);
    pub const X20: Reg = Reg(20);
    pub const X21: Reg = Reg(21);
    pub const X22: Reg = Reg(22);
    pub const X23: Reg = Reg(23);
    pub const X24: Reg = Reg(24);
    pub const X25: Reg = Reg(25);
    pub const X26: Reg = Reg(26);
    pub const X27: Reg = Reg(27);
    pub const X28: Reg = Reg(28);
    pub const X29: Reg = Reg(29);
    pub const X30: Reg = Reg(30);
    pub const X31: Reg = Reg(31);

    pub const ZERO: Reg = X0;
    pub const RA: Reg = X1;
    pub const SP: Reg = X2;
    pub const GP: Reg = X3;
    pub const TP: Reg = X4;
    pub const T0: Reg = X5;
    pub const T1: Reg = X6;
    pub const T2: Reg = X7;
    pub const S0: Reg = X8;
    pub const FP: Reg = X8;
    pub const S1: Reg = X9;
    pub const A0: Reg = X10;
    pub const A1: Reg = X11;
    pub const A2: Reg = X12;
    pub const A3: Reg = X13;
    pub const A4: Reg = X14;
    pub const A5: Reg = X15;
    pub const A6: Reg = X16;
    pub const A7: Reg = X17;
    pub const S2: Reg = X18;
    pub const S3: Reg = X19;
    pub const S4: Reg = X20;
    pub const S5: Reg = X21;
    pub const S6: Reg = X22;
    pub const S7: Reg = X23;
    pub const S8: Reg = X24;
    pub const S9: Reg = X25;
    pub const S10: Reg = X26;
    pub const S11: Reg = X27;
    pub const T3: Reg = X28;
    pub const T4: Reg = X29;
    pub const T5: Reg = X30;
    pub const T6: Reg = X31;

    pub const F0: Reg = Reg(0);
    pub const F1: Reg = Reg(1);
    pub const F2: Reg = Reg(2);
    pub const F3: Reg = Reg(3);
    pub const F4: Reg = Reg(4);
    pub const F5: Reg = Reg(5);
    pub const F6: Reg = Reg(6);
    pub const F7: Reg = Reg(7);
    pub const F8: Reg = Reg(8);
    pub const F9: Reg = Reg(9);
    pub const F10: Reg = Reg(10);
    pub const F11: Reg = Reg(11);
    pub const F12: Reg = Reg(12);
    pub const F13: Reg = Reg(13);
    pub const F14: Reg = Reg(14);
    pub const F15: Reg = Reg(15);
    pub const F16: Reg = Reg(16);
    pub const F17: Reg = Reg(17);
    pub const F18: Reg = Reg(18);
    pub const F19: Reg = Reg(19);
    pub const F20: Reg = Reg(20);
    pub const F21: Reg = Reg(21);
    pub const F22: Reg = Reg(22);
    pub const F23: Reg = Reg(23);
    pub const F24: Reg = Reg(24);
    pub const F25: Reg = Reg(25);
    pub const F26: Reg = Reg(26);
    pub const F27: Reg = Reg(27);
    pub const F28: Reg = Reg(28);
    pub const F29: Reg = Reg(29);
    pub const F30: Reg = Reg(30);
    pub const F31: Reg = Reg(31);

    pub const FT0: Reg = F0;
    pub const FT1: Reg = F1;
    pub const FT2: Reg = F2;
    pub const FT3: Reg = F3;
    pub const FT4: Reg = F4;
    pub const FT5: Reg = F5;
    pub const FT6: Reg = F6;
    pub const FT7: Reg = F7;
    pub const FS0: Reg = F8;
    pub const FS1: Reg = F9;
    pub const FA0: Reg = F10;
    pub const FA1: Reg = F11;
    pub const FA2: Reg = F12;
    pub const FA3: Reg = F13;
    pub const FA4: Reg = F14;
    pub const FA5: Reg = F15;
    pub const FA6: Reg = F16;
    pub const FA7: Reg = F17;
    pub const FS2: Reg = F18;
    pub const FS3: Reg = F19;
    pub const FS4: Reg = F20;
    pub const FS5: Reg = F21;
    pub const FS6: Reg = F22;
    pub const FS7: Reg = F23;
    pub const FS8: Reg = F24;
    pub const FS9: Reg = F25;
    pub const FS10: Reg = F26;
    pub const FS11: Reg = F27;
    pub const FT8: Reg = F28;
    pub const FT9: Reg = F29;
    pub const FT10: Reg = F30;
    pub const FT11: Reg = F31;

    pub const V0: VReg = VReg(0);
    pub const V1: VReg = VReg(1);
    pub const V2: VReg = VReg(2);
    pub const V3: VReg = VReg(3);
    pub const V4: VReg = VReg(4);
    pub const V5: VReg = VReg(5);
    pub const V6: VReg = VReg(6);
    pub const V7: VReg = VReg(7);
    pub const V8: VReg = VReg(8);
    pub const V9: VReg = VReg(9);
    pub const V10: VReg = VReg(10);
    pub const V11: VReg = VReg(11);
    pub const V12: VReg = VReg(12);
    pub const V13: VReg = VReg(13);
    pub const V14: VReg = VReg(14);
    pub const V15: VReg = VReg(15);
    pub const V16: VReg = VReg(16);
    pub const V17: VReg = VReg(17);
    pub const V18: VReg = VReg(18);
    pub const V19: VReg = VReg(19);
    pub const V20: VReg = VReg(20);
    pub const V21: VReg = VReg(21);
    pub const V22: VReg = VReg(22);
    pub const V23: VReg = VReg(23);
    pub const V24: VReg = VReg(24);
    pub const V25: VReg = VReg(25);
    pub const V26: VReg = VReg(26);
    pub const V27: VReg = VReg(27);
    pub const V28: VReg = VReg(28);
    pub const V29: VReg = VReg(29);
    pub const V30: VReg = VReg(30);
    pub const V31: VReg = VReg(31);

    pub const VT0: VReg = V0;
    pub const VT1: VReg = V1;
    pub const VT2: VReg = V2;
    pub const VT3: VReg = V3;
    pub const VT4: VReg = V4;
    pub const VT5: VReg = V5;
    pub const VT6: VReg = V6;
    pub const VT7: VReg = V7;
    pub const VS0: VReg = V8;
    pub const VS1: VReg = V9;
    pub const VA0: VReg = V10;
    pub const VA1: VReg = V11;
    pub const VA2: VReg = V12;
    pub const VA3: VReg = V13;
    pub const VA4: VReg = V14;
    pub const VA5: VReg = V15;
    pub const VA6: VReg = V16;
    pub const VA7: VReg = V17;
    pub const VS2: VReg = V18;
    pub const VS3: VReg = V19;
    pub const VS4: VReg = V20;
    pub const VS5: VReg = V21;
    pub const VS6: VReg = V22;
    pub const VS7: VReg = V23;
    pub const VS8: VReg = V24;
    pub const VS9: VReg = V25;
    pub const VS10: VReg = V26;
    pub const VS11: VReg = V27;
    pub const VT8: VReg = V28;
    pub const VT9: VReg = V29;
    pub const VT10: VReg = V30;
    pub const VT11: VReg = V31;
}

pub use regs::*;
