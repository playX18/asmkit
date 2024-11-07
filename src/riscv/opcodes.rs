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

/* Automatically generated by parse_opcodes */

pub const MATCH_ADD: u32 = 0x33;
pub const MASK_ADD: u32 = 0xfe00707f;
pub const MATCH_ADD_UW: u32 = 0x800003b;
pub const MASK_ADD_UW: u32 = 0xfe00707f;
pub const MATCH_ADDI: u32 = 0x13;
pub const MASK_ADDI: u32 = 0x707f;
pub const MATCH_ADDIW: u32 = 0x1b;
pub const MASK_ADDIW: u32 = 0x707f;
pub const MATCH_ADDW: u32 = 0x3b;
pub const MASK_ADDW: u32 = 0xfe00707f;
pub const MATCH_AES32DSI: u32 = 0x2a000033;
pub const MASK_AES32DSI: u32 = 0x3e00707f;
pub const MATCH_AES32DSMI: u32 = 0x2e000033;
pub const MASK_AES32DSMI: u32 = 0x3e00707f;
pub const MATCH_AES32ESI: u32 = 0x22000033;
pub const MASK_AES32ESI: u32 = 0x3e00707f;
pub const MATCH_AES32ESMI: u32 = 0x26000033;
pub const MASK_AES32ESMI: u32 = 0x3e00707f;
pub const MATCH_AES64DS: u32 = 0x3a000033;
pub const MASK_AES64DS: u32 = 0xfe00707f;
pub const MATCH_AES64DSM: u32 = 0x3e000033;
pub const MASK_AES64DSM: u32 = 0xfe00707f;
pub const MATCH_AES64ES: u32 = 0x32000033;
pub const MASK_AES64ES: u32 = 0xfe00707f;
pub const MATCH_AES64ESM: u32 = 0x36000033;
pub const MASK_AES64ESM: u32 = 0xfe00707f;
pub const MATCH_AES64IM: u32 = 0x30001013;
pub const MASK_AES64IM: u32 = 0xfff0707f;
pub const MATCH_AES64KS1I: u32 = 0x31001013;
pub const MASK_AES64KS1I: u32 = 0xff00707f;
pub const MATCH_AES64KS2: u32 = 0x7e000033;
pub const MASK_AES64KS2: u32 = 0xfe00707f;
pub const MATCH_AMOADD_B: u32 = 0x2f;
pub const MASK_AMOADD_B: u32 = 0xf800707f;
pub const MATCH_AMOADD_D: u32 = 0x302f;
pub const MASK_AMOADD_D: u32 = 0xf800707f;
pub const MATCH_AMOADD_H: u32 = 0x102f;
pub const MASK_AMOADD_H: u32 = 0xf800707f;
pub const MATCH_AMOADD_W: u32 = 0x202f;
pub const MASK_AMOADD_W: u32 = 0xf800707f;
pub const MATCH_AMOAND_B: u32 = 0x6000002f;
pub const MASK_AMOAND_B: u32 = 0xf800707f;
pub const MATCH_AMOAND_D: u32 = 0x6000302f;
pub const MASK_AMOAND_D: u32 = 0xf800707f;
pub const MATCH_AMOAND_H: u32 = 0x6000102f;
pub const MASK_AMOAND_H: u32 = 0xf800707f;
pub const MATCH_AMOAND_W: u32 = 0x6000202f;
pub const MASK_AMOAND_W: u32 = 0xf800707f;
pub const MATCH_AMOCAS_B: u32 = 0x2800002f;
pub const MASK_AMOCAS_B: u32 = 0xf800707f;
pub const MATCH_AMOCAS_D: u32 = 0x2800302f;
pub const MASK_AMOCAS_D: u32 = 0xf800707f;
pub const MATCH_AMOCAS_H: u32 = 0x2800102f;
pub const MASK_AMOCAS_H: u32 = 0xf800707f;
pub const MATCH_AMOCAS_Q: u32 = 0x2800402f;
pub const MASK_AMOCAS_Q: u32 = 0xf800707f;
pub const MATCH_AMOCAS_W: u32 = 0x2800202f;
pub const MASK_AMOCAS_W: u32 = 0xf800707f;
pub const MATCH_AMOMAX_B: u32 = 0xa000002f;
pub const MASK_AMOMAX_B: u32 = 0xf800707f;
pub const MATCH_AMOMAX_D: u32 = 0xa000302f;
pub const MASK_AMOMAX_D: u32 = 0xf800707f;
pub const MATCH_AMOMAX_H: u32 = 0xa000102f;
pub const MASK_AMOMAX_H: u32 = 0xf800707f;
pub const MATCH_AMOMAX_W: u32 = 0xa000202f;
pub const MASK_AMOMAX_W: u32 = 0xf800707f;
pub const MATCH_AMOMAXU_B: u32 = 0xe000002f;
pub const MASK_AMOMAXU_B: u32 = 0xf800707f;
pub const MATCH_AMOMAXU_D: u32 = 0xe000302f;
pub const MASK_AMOMAXU_D: u32 = 0xf800707f;
pub const MATCH_AMOMAXU_H: u32 = 0xe000102f;
pub const MASK_AMOMAXU_H: u32 = 0xf800707f;
pub const MATCH_AMOMAXU_W: u32 = 0xe000202f;
pub const MASK_AMOMAXU_W: u32 = 0xf800707f;
pub const MATCH_AMOMIN_B: u32 = 0x8000002f;
pub const MASK_AMOMIN_B: u32 = 0xf800707f;
pub const MATCH_AMOMIN_D: u32 = 0x8000302f;
pub const MASK_AMOMIN_D: u32 = 0xf800707f;
pub const MATCH_AMOMIN_H: u32 = 0x8000102f;
pub const MASK_AMOMIN_H: u32 = 0xf800707f;
pub const MATCH_AMOMIN_W: u32 = 0x8000202f;
pub const MASK_AMOMIN_W: u32 = 0xf800707f;
pub const MATCH_AMOMINU_B: u32 = 0xc000002f;
pub const MASK_AMOMINU_B: u32 = 0xf800707f;
pub const MATCH_AMOMINU_D: u32 = 0xc000302f;
pub const MASK_AMOMINU_D: u32 = 0xf800707f;
pub const MATCH_AMOMINU_H: u32 = 0xc000102f;
pub const MASK_AMOMINU_H: u32 = 0xf800707f;
pub const MATCH_AMOMINU_W: u32 = 0xc000202f;
pub const MASK_AMOMINU_W: u32 = 0xf800707f;
pub const MATCH_AMOOR_B: u32 = 0x4000002f;
pub const MASK_AMOOR_B: u32 = 0xf800707f;
pub const MATCH_AMOOR_D: u32 = 0x4000302f;
pub const MASK_AMOOR_D: u32 = 0xf800707f;
pub const MATCH_AMOOR_H: u32 = 0x4000102f;
pub const MASK_AMOOR_H: u32 = 0xf800707f;
pub const MATCH_AMOOR_W: u32 = 0x4000202f;
pub const MASK_AMOOR_W: u32 = 0xf800707f;
pub const MATCH_AMOSWAP_B: u32 = 0x800002f;
pub const MASK_AMOSWAP_B: u32 = 0xf800707f;
pub const MATCH_AMOSWAP_D: u32 = 0x800302f;
pub const MASK_AMOSWAP_D: u32 = 0xf800707f;
pub const MATCH_AMOSWAP_H: u32 = 0x800102f;
pub const MASK_AMOSWAP_H: u32 = 0xf800707f;
pub const MATCH_AMOSWAP_W: u32 = 0x800202f;
pub const MASK_AMOSWAP_W: u32 = 0xf800707f;
pub const MATCH_AMOXOR_B: u32 = 0x2000002f;
pub const MASK_AMOXOR_B: u32 = 0xf800707f;
pub const MATCH_AMOXOR_D: u32 = 0x2000302f;
pub const MASK_AMOXOR_D: u32 = 0xf800707f;
pub const MATCH_AMOXOR_H: u32 = 0x2000102f;
pub const MASK_AMOXOR_H: u32 = 0xf800707f;
pub const MATCH_AMOXOR_W: u32 = 0x2000202f;
pub const MASK_AMOXOR_W: u32 = 0xf800707f;
pub const MATCH_AND: u32 = 0x7033;
pub const MASK_AND: u32 = 0xfe00707f;
pub const MATCH_ANDI: u32 = 0x7013;
pub const MASK_ANDI: u32 = 0x707f;
pub const MATCH_ANDN: u32 = 0x40007033;
pub const MASK_ANDN: u32 = 0xfe00707f;
pub const MATCH_AUIPC: u32 = 0x17;
pub const MASK_AUIPC: u32 = 0x7f;
pub const MATCH_BCLR: u32 = 0x48001033;
pub const MASK_BCLR: u32 = 0xfe00707f;
pub const MATCH_BCLRI: u32 = 0x48001013;
pub const MASK_BCLRI: u32 = 0xfc00707f;
pub const MATCH_BCLRI_RV32: u32 = 0x48001013;
pub const MASK_BCLRI_RV32: u32 = 0xfe00707f;
pub const MATCH_BEQ: u32 = 0x63;
pub const MASK_BEQ: u32 = 0x707f;
pub const MATCH_BEQZ: u32 = 0x63;
pub const MASK_BEQZ: u32 = 0x1f0707f;
pub const MATCH_BEXT: u32 = 0x48005033;
pub const MASK_BEXT: u32 = 0xfe00707f;
pub const MATCH_BEXTI: u32 = 0x48005013;
pub const MASK_BEXTI: u32 = 0xfc00707f;
pub const MATCH_BEXTI_RV32: u32 = 0x48005013;
pub const MASK_BEXTI_RV32: u32 = 0xfe00707f;
pub const MATCH_BGE: u32 = 0x5063;
pub const MASK_BGE: u32 = 0x707f;
pub const MATCH_BGEU: u32 = 0x7063;
pub const MASK_BGEU: u32 = 0x707f;
pub const MATCH_BGEZ: u32 = 0x5063;
pub const MASK_BGEZ: u32 = 0x1f0707f;
pub const MATCH_BGT: u32 = 0x4063;
pub const MASK_BGT: u32 = 0x707f;
pub const MATCH_BGTU: u32 = 0x6063;
pub const MASK_BGTU: u32 = 0x707f;
pub const MATCH_BGTZ: u32 = 0x4063;
pub const MASK_BGTZ: u32 = 0xff07f;
pub const MATCH_BINV: u32 = 0x68001033;
pub const MASK_BINV: u32 = 0xfe00707f;
pub const MATCH_BINVI: u32 = 0x68001013;
pub const MASK_BINVI: u32 = 0xfc00707f;
pub const MATCH_BINVI_RV32: u32 = 0x68001013;
pub const MASK_BINVI_RV32: u32 = 0xfe00707f;
pub const MATCH_BLE: u32 = 0x5063;
pub const MASK_BLE: u32 = 0x707f;
pub const MATCH_BLEU: u32 = 0x7063;
pub const MASK_BLEU: u32 = 0x707f;
pub const MATCH_BLEZ: u32 = 0x5063;
pub const MASK_BLEZ: u32 = 0xff07f;
pub const MATCH_BLT: u32 = 0x4063;
pub const MASK_BLT: u32 = 0x707f;
pub const MATCH_BLTU: u32 = 0x6063;
pub const MASK_BLTU: u32 = 0x707f;
pub const MATCH_BLTZ: u32 = 0x4063;
pub const MASK_BLTZ: u32 = 0x1f0707f;
pub const MATCH_BNE: u32 = 0x1063;
pub const MASK_BNE: u32 = 0x707f;
pub const MATCH_BNEZ: u32 = 0x1063;
pub const MASK_BNEZ: u32 = 0x1f0707f;
pub const MATCH_BREV8: u32 = 0x68705013;
pub const MASK_BREV8: u32 = 0xfff0707f;
pub const MATCH_BSET: u32 = 0x28001033;
pub const MASK_BSET: u32 = 0xfe00707f;
pub const MATCH_BSETI: u32 = 0x28001013;
pub const MASK_BSETI: u32 = 0xfc00707f;
pub const MATCH_BSETI_RV32: u32 = 0x28001013;
pub const MASK_BSETI_RV32: u32 = 0xfe00707f;
pub const MATCH_C_ADD: u32 = 0x9002;
pub const MASK_C_ADD: u32 = 0xf003;
pub const MATCH_C_ADDI: u32 = 0x1;
pub const MASK_C_ADDI: u32 = 0xe003;
pub const MATCH_C_ADDI16SP: u32 = 0x6101;
pub const MASK_C_ADDI16SP: u32 = 0xef83;
pub const MATCH_C_ADDI4SPN: u32 = 0x0;
pub const MASK_C_ADDI4SPN: u32 = 0xe003;
pub const MATCH_C_ADDIW: u32 = 0x2001;
pub const MASK_C_ADDIW: u32 = 0xe003;
pub const MATCH_C_ADDW: u32 = 0x9c21;
pub const MASK_C_ADDW: u32 = 0xfc63;
pub const MATCH_C_AND: u32 = 0x8c61;
pub const MASK_C_AND: u32 = 0xfc63;
pub const MATCH_C_ANDI: u32 = 0x8801;
pub const MASK_C_ANDI: u32 = 0xec03;
pub const MATCH_C_BEQZ: u32 = 0xc001;
pub const MASK_C_BEQZ: u32 = 0xe003;
pub const MATCH_C_BNEZ: u32 = 0xe001;
pub const MASK_C_BNEZ: u32 = 0xe003;
pub const MATCH_C_EBREAK: u32 = 0x9002;
pub const MASK_C_EBREAK: u32 = 0xffff;
pub const MATCH_C_FLD: u32 = 0x2000;
pub const MASK_C_FLD: u32 = 0xe003;
pub const MATCH_C_FLDSP: u32 = 0x2002;
pub const MASK_C_FLDSP: u32 = 0xe003;
pub const MATCH_C_FLW: u32 = 0x6000;
pub const MASK_C_FLW: u32 = 0xe003;
pub const MATCH_C_FLWSP: u32 = 0x6002;
pub const MASK_C_FLWSP: u32 = 0xe003;
pub const MATCH_C_FSD: u32 = 0xa000;
pub const MASK_C_FSD: u32 = 0xe003;
pub const MATCH_C_FSDSP: u32 = 0xa002;
pub const MASK_C_FSDSP: u32 = 0xe003;
pub const MATCH_C_FSW: u32 = 0xe000;
pub const MASK_C_FSW: u32 = 0xe003;
pub const MATCH_C_FSWSP: u32 = 0xe002;
pub const MASK_C_FSWSP: u32 = 0xe003;
pub const MATCH_C_J: u32 = 0xa001;
pub const MASK_C_J: u32 = 0xe003;
pub const MATCH_C_JAL: u32 = 0x2001;
pub const MASK_C_JAL: u32 = 0xe003;
pub const MATCH_C_JALR: u32 = 0x9002;
pub const MASK_C_JALR: u32 = 0xf07f;
pub const MATCH_C_JR: u32 = 0x8002;
pub const MASK_C_JR: u32 = 0xf07f;
pub const MATCH_C_LBU: u32 = 0x8000;
pub const MASK_C_LBU: u32 = 0xfc03;
pub const MATCH_C_LD: u32 = 0x6000;
pub const MASK_C_LD: u32 = 0xe003;
pub const MATCH_C_LDSP: u32 = 0x6002;
pub const MASK_C_LDSP: u32 = 0xe003;
pub const MATCH_C_LH: u32 = 0x8440;
pub const MASK_C_LH: u32 = 0xfc43;
pub const MATCH_C_LHU: u32 = 0x8400;
pub const MASK_C_LHU: u32 = 0xfc43;
pub const MATCH_C_LI: u32 = 0x4001;
pub const MASK_C_LI: u32 = 0xe003;
pub const MATCH_C_LUI: u32 = 0x6001;
pub const MASK_C_LUI: u32 = 0xe003;
pub const MATCH_C_LW: u32 = 0x4000;
pub const MASK_C_LW: u32 = 0xe003;
pub const MATCH_C_LWSP: u32 = 0x4002;
pub const MASK_C_LWSP: u32 = 0xe003;
pub const MATCH_C_MOP_1: u32 = 0x6081;
pub const MASK_C_MOP_1: u32 = 0xffff;
pub const MATCH_C_MOP_11: u32 = 0x6581;
pub const MASK_C_MOP_11: u32 = 0xffff;
pub const MATCH_C_MOP_13: u32 = 0x6681;
pub const MASK_C_MOP_13: u32 = 0xffff;
pub const MATCH_C_MOP_15: u32 = 0x6781;
pub const MASK_C_MOP_15: u32 = 0xffff;
pub const MATCH_C_MOP_3: u32 = 0x6181;
pub const MASK_C_MOP_3: u32 = 0xffff;
pub const MATCH_C_MOP_5: u32 = 0x6281;
pub const MASK_C_MOP_5: u32 = 0xffff;
pub const MATCH_C_MOP_7: u32 = 0x6381;
pub const MASK_C_MOP_7: u32 = 0xffff;
pub const MATCH_C_MOP_9: u32 = 0x6481;
pub const MASK_C_MOP_9: u32 = 0xffff;
pub const MATCH_C_MOP_N: u32 = 0x6081;
pub const MASK_C_MOP_N: u32 = 0xf8ff;
pub const MATCH_C_MUL: u32 = 0x9c41;
pub const MASK_C_MUL: u32 = 0xfc63;
pub const MATCH_C_MV: u32 = 0x8002;
pub const MASK_C_MV: u32 = 0xf003;
pub const MATCH_C_NOP: u32 = 0x1;
pub const MASK_C_NOP: u32 = 0xef83;
pub const MATCH_C_NOT: u32 = 0x9c75;
pub const MASK_C_NOT: u32 = 0xfc7f;
pub const MATCH_C_NTL_ALL: u32 = 0x9016;
pub const MASK_C_NTL_ALL: u32 = 0xffff;
pub const MATCH_C_NTL_P1: u32 = 0x900a;
pub const MASK_C_NTL_P1: u32 = 0xffff;
pub const MATCH_C_NTL_PALL: u32 = 0x900e;
pub const MASK_C_NTL_PALL: u32 = 0xffff;
pub const MATCH_C_NTL_S1: u32 = 0x9012;
pub const MASK_C_NTL_S1: u32 = 0xffff;
pub const MATCH_C_OR: u32 = 0x8c41;
pub const MASK_C_OR: u32 = 0xfc63;
pub const MATCH_C_SB: u32 = 0x8800;
pub const MASK_C_SB: u32 = 0xfc03;
pub const MATCH_C_SD: u32 = 0xe000;
pub const MASK_C_SD: u32 = 0xe003;
pub const MATCH_C_SDSP: u32 = 0xe002;
pub const MASK_C_SDSP: u32 = 0xe003;
pub const MATCH_C_SEXT_B: u32 = 0x9c65;
pub const MASK_C_SEXT_B: u32 = 0xfc7f;
pub const MATCH_C_SEXT_H: u32 = 0x9c6d;
pub const MASK_C_SEXT_H: u32 = 0xfc7f;
pub const MATCH_C_SEXT_W: u32 = 0x2001;
pub const MASK_C_SEXT_W: u32 = 0xf07f;
pub const MATCH_C_SH: u32 = 0x8c00;
pub const MASK_C_SH: u32 = 0xfc43;
pub const MATCH_C_SLLI: u32 = 0x2;
pub const MASK_C_SLLI: u32 = 0xe003;
pub const MATCH_C_SLLI_RV32: u32 = 0x2;
pub const MASK_C_SLLI_RV32: u32 = 0xf003;
pub const MATCH_C_SRAI: u32 = 0x8401;
pub const MASK_C_SRAI: u32 = 0xec03;
pub const MATCH_C_SRAI_RV32: u32 = 0x8401;
pub const MASK_C_SRAI_RV32: u32 = 0xfc03;
pub const MATCH_C_SRLI: u32 = 0x8001;
pub const MASK_C_SRLI: u32 = 0xec03;
pub const MATCH_C_SRLI_RV32: u32 = 0x8001;
pub const MASK_C_SRLI_RV32: u32 = 0xfc03;
pub const MATCH_C_SUB: u32 = 0x8c01;
pub const MASK_C_SUB: u32 = 0xfc63;
pub const MATCH_C_SUBW: u32 = 0x9c01;
pub const MASK_C_SUBW: u32 = 0xfc63;
pub const MATCH_C_SW: u32 = 0xc000;
pub const MASK_C_SW: u32 = 0xe003;
pub const MATCH_C_SWSP: u32 = 0xc002;
pub const MASK_C_SWSP: u32 = 0xe003;
pub const MATCH_C_XOR: u32 = 0x8c21;
pub const MASK_C_XOR: u32 = 0xfc63;
pub const MATCH_C_ZEXT_B: u32 = 0x9c61;
pub const MASK_C_ZEXT_B: u32 = 0xfc7f;
pub const MATCH_C_ZEXT_H: u32 = 0x9c69;
pub const MASK_C_ZEXT_H: u32 = 0xfc7f;
pub const MATCH_C_ZEXT_W: u32 = 0x9c71;
pub const MASK_C_ZEXT_W: u32 = 0xfc7f;
pub const MATCH_CBO_CLEAN: u32 = 0x10200f;
pub const MASK_CBO_CLEAN: u32 = 0xfff07fff;
pub const MATCH_CBO_FLUSH: u32 = 0x20200f;
pub const MASK_CBO_FLUSH: u32 = 0xfff07fff;
pub const MATCH_CBO_INVAL: u32 = 0x200f;
pub const MASK_CBO_INVAL: u32 = 0xfff07fff;
pub const MATCH_CBO_ZERO: u32 = 0x40200f;
pub const MASK_CBO_ZERO: u32 = 0xfff07fff;
pub const MATCH_CLMUL: u32 = 0xa001033;
pub const MASK_CLMUL: u32 = 0xfe00707f;
pub const MATCH_CLMULH: u32 = 0xa003033;
pub const MASK_CLMULH: u32 = 0xfe00707f;
pub const MATCH_CLMULR: u32 = 0xa002033;
pub const MASK_CLMULR: u32 = 0xfe00707f;
pub const MATCH_CLZ: u32 = 0x60001013;
pub const MASK_CLZ: u32 = 0xfff0707f;
pub const MATCH_CLZW: u32 = 0x6000101b;
pub const MASK_CLZW: u32 = 0xfff0707f;
pub const MATCH_CM_JALT: u32 = 0xa002;
pub const MASK_CM_JALT: u32 = 0xfc03;
pub const MATCH_CM_MVA01S: u32 = 0xac62;
pub const MASK_CM_MVA01S: u32 = 0xfc63;
pub const MATCH_CM_MVSA01: u32 = 0xac22;
pub const MASK_CM_MVSA01: u32 = 0xfc63;
pub const MATCH_CM_POP: u32 = 0xba02;
pub const MASK_CM_POP: u32 = 0xff03;
pub const MATCH_CM_POPRET: u32 = 0xbe02;
pub const MASK_CM_POPRET: u32 = 0xff03;
pub const MATCH_CM_POPRETZ: u32 = 0xbc02;
pub const MASK_CM_POPRETZ: u32 = 0xff03;
pub const MATCH_CM_PUSH: u32 = 0xb802;
pub const MASK_CM_PUSH: u32 = 0xff03;
pub const MATCH_CPOP: u32 = 0x60201013;
pub const MASK_CPOP: u32 = 0xfff0707f;
pub const MATCH_CPOPW: u32 = 0x6020101b;
pub const MASK_CPOPW: u32 = 0xfff0707f;
pub const MATCH_CSRC: u32 = 0x3073;
pub const MASK_CSRC: u32 = 0x7fff;
pub const MATCH_CSRCI: u32 = 0x7073;
pub const MASK_CSRCI: u32 = 0x7fff;
pub const MATCH_CSRR: u32 = 0x2073;
pub const MASK_CSRR: u32 = 0xff07f;
pub const MATCH_CSRRC: u32 = 0x3073;
pub const MASK_CSRRC: u32 = 0x707f;
pub const MATCH_CSRRCI: u32 = 0x7073;
pub const MASK_CSRRCI: u32 = 0x707f;
pub const MATCH_CSRRS: u32 = 0x2073;
pub const MASK_CSRRS: u32 = 0x707f;
pub const MATCH_CSRRSI: u32 = 0x6073;
pub const MASK_CSRRSI: u32 = 0x707f;
pub const MATCH_CSRRW: u32 = 0x1073;
pub const MASK_CSRRW: u32 = 0x707f;
pub const MATCH_CSRRWI: u32 = 0x5073;
pub const MASK_CSRRWI: u32 = 0x707f;
pub const MATCH_CSRS: u32 = 0x2073;
pub const MASK_CSRS: u32 = 0x7fff;
pub const MATCH_CSRSI: u32 = 0x6073;
pub const MASK_CSRSI: u32 = 0x7fff;
pub const MATCH_CSRW: u32 = 0x1073;
pub const MASK_CSRW: u32 = 0x7fff;
pub const MATCH_CSRWI: u32 = 0x5073;
pub const MASK_CSRWI: u32 = 0x7fff;
pub const MATCH_CTZ: u32 = 0x60101013;
pub const MASK_CTZ: u32 = 0xfff0707f;
pub const MATCH_CTZW: u32 = 0x6010101b;
pub const MASK_CTZW: u32 = 0xfff0707f;
pub const MATCH_CZERO_EQZ: u32 = 0xe005033;
pub const MASK_CZERO_EQZ: u32 = 0xfe00707f;
pub const MATCH_CZERO_NEZ: u32 = 0xe007033;
pub const MASK_CZERO_NEZ: u32 = 0xfe00707f;
pub const MATCH_DIV: u32 = 0x2004033;
pub const MASK_DIV: u32 = 0xfe00707f;
pub const MATCH_DIVU: u32 = 0x2005033;
pub const MASK_DIVU: u32 = 0xfe00707f;
pub const MATCH_DIVUW: u32 = 0x200503b;
pub const MASK_DIVUW: u32 = 0xfe00707f;
pub const MATCH_DIVW: u32 = 0x200403b;
pub const MASK_DIVW: u32 = 0xfe00707f;
pub const MATCH_DRET: u32 = 0x7b200073;
pub const MASK_DRET: u32 = 0xffffffff;
pub const MATCH_EBREAK: u32 = 0x100073;
pub const MASK_EBREAK: u32 = 0xffffffff;
pub const MATCH_ECALL: u32 = 0x73;
pub const MASK_ECALL: u32 = 0xffffffff;
pub const MATCH_FABS_D: u32 = 0x22002053;
pub const MASK_FABS_D: u32 = 0xfe00707f;
pub const MATCH_FABS_H: u32 = 0x24002053;
pub const MASK_FABS_H: u32 = 0xfe00707f;
pub const MATCH_FABS_Q: u32 = 0x26002053;
pub const MASK_FABS_Q: u32 = 0xfe00707f;
pub const MATCH_FABS_S: u32 = 0x20002053;
pub const MASK_FABS_S: u32 = 0xfe00707f;
pub const MATCH_FADD_D: u32 = 0x2000053;
pub const MASK_FADD_D: u32 = 0xfe00007f;
pub const MATCH_FADD_H: u32 = 0x4000053;
pub const MASK_FADD_H: u32 = 0xfe00007f;
pub const MATCH_FADD_Q: u32 = 0x6000053;
pub const MASK_FADD_Q: u32 = 0xfe00007f;
pub const MATCH_FADD_S: u32 = 0x53;
pub const MASK_FADD_S: u32 = 0xfe00007f;
pub const MATCH_FCLASS_D: u32 = 0xe2001053;
pub const MASK_FCLASS_D: u32 = 0xfff0707f;
pub const MATCH_FCLASS_H: u32 = 0xe4001053;
pub const MASK_FCLASS_H: u32 = 0xfff0707f;
pub const MATCH_FCLASS_Q: u32 = 0xe6001053;
pub const MASK_FCLASS_Q: u32 = 0xfff0707f;
pub const MATCH_FCLASS_S: u32 = 0xe0001053;
pub const MASK_FCLASS_S: u32 = 0xfff0707f;
pub const MATCH_FCVT_D_H: u32 = 0x42200053;
pub const MASK_FCVT_D_H: u32 = 0xfff0007f;
pub const MATCH_FCVT_D_L: u32 = 0xd2200053;
pub const MASK_FCVT_D_L: u32 = 0xfff0007f;
pub const MATCH_FCVT_D_LU: u32 = 0xd2300053;
pub const MASK_FCVT_D_LU: u32 = 0xfff0007f;
pub const MATCH_FCVT_D_Q: u32 = 0x42300053;
pub const MASK_FCVT_D_Q: u32 = 0xfff0007f;
pub const MATCH_FCVT_D_S: u32 = 0x42000053;
pub const MASK_FCVT_D_S: u32 = 0xfff0007f;
pub const MATCH_FCVT_D_W: u32 = 0xd2000053;
pub const MASK_FCVT_D_W: u32 = 0xfff0007f;
pub const MATCH_FCVT_D_WU: u32 = 0xd2100053;
pub const MASK_FCVT_D_WU: u32 = 0xfff0007f;
pub const MATCH_FCVT_H_D: u32 = 0x44100053;
pub const MASK_FCVT_H_D: u32 = 0xfff0007f;
pub const MATCH_FCVT_H_L: u32 = 0xd4200053;
pub const MASK_FCVT_H_L: u32 = 0xfff0007f;
pub const MATCH_FCVT_H_LU: u32 = 0xd4300053;
pub const MASK_FCVT_H_LU: u32 = 0xfff0007f;
pub const MATCH_FCVT_H_Q: u32 = 0x44300053;
pub const MASK_FCVT_H_Q: u32 = 0xfff0007f;
pub const MATCH_FCVT_H_S: u32 = 0x44000053;
pub const MASK_FCVT_H_S: u32 = 0xfff0007f;
pub const MATCH_FCVT_H_W: u32 = 0xd4000053;
pub const MASK_FCVT_H_W: u32 = 0xfff0007f;
pub const MATCH_FCVT_H_WU: u32 = 0xd4100053;
pub const MASK_FCVT_H_WU: u32 = 0xfff0007f;
pub const MATCH_FCVT_L_D: u32 = 0xc2200053;
pub const MASK_FCVT_L_D: u32 = 0xfff0007f;
pub const MATCH_FCVT_L_H: u32 = 0xc4200053;
pub const MASK_FCVT_L_H: u32 = 0xfff0007f;
pub const MATCH_FCVT_L_Q: u32 = 0xc6200053;
pub const MASK_FCVT_L_Q: u32 = 0xfff0007f;
pub const MATCH_FCVT_L_S: u32 = 0xc0200053;
pub const MASK_FCVT_L_S: u32 = 0xfff0007f;
pub const MATCH_FCVT_LU_D: u32 = 0xc2300053;
pub const MASK_FCVT_LU_D: u32 = 0xfff0007f;
pub const MATCH_FCVT_LU_H: u32 = 0xc4300053;
pub const MASK_FCVT_LU_H: u32 = 0xfff0007f;
pub const MATCH_FCVT_LU_Q: u32 = 0xc6300053;
pub const MASK_FCVT_LU_Q: u32 = 0xfff0007f;
pub const MATCH_FCVT_LU_S: u32 = 0xc0300053;
pub const MASK_FCVT_LU_S: u32 = 0xfff0007f;
pub const MATCH_FCVT_Q_D: u32 = 0x46100053;
pub const MASK_FCVT_Q_D: u32 = 0xfff0007f;
pub const MATCH_FCVT_Q_H: u32 = 0x46200053;
pub const MASK_FCVT_Q_H: u32 = 0xfff0007f;
pub const MATCH_FCVT_Q_L: u32 = 0xd6200053;
pub const MASK_FCVT_Q_L: u32 = 0xfff0007f;
pub const MATCH_FCVT_Q_LU: u32 = 0xd6300053;
pub const MASK_FCVT_Q_LU: u32 = 0xfff0007f;
pub const MATCH_FCVT_Q_S: u32 = 0x46000053;
pub const MASK_FCVT_Q_S: u32 = 0xfff0007f;
pub const MATCH_FCVT_Q_W: u32 = 0xd6000053;
pub const MASK_FCVT_Q_W: u32 = 0xfff0007f;
pub const MATCH_FCVT_Q_WU: u32 = 0xd6100053;
pub const MASK_FCVT_Q_WU: u32 = 0xfff0007f;
pub const MATCH_FCVT_S_D: u32 = 0x40100053;
pub const MASK_FCVT_S_D: u32 = 0xfff0007f;
pub const MATCH_FCVT_S_H: u32 = 0x40200053;
pub const MASK_FCVT_S_H: u32 = 0xfff0007f;
pub const MATCH_FCVT_S_L: u32 = 0xd0200053;
pub const MASK_FCVT_S_L: u32 = 0xfff0007f;
pub const MATCH_FCVT_S_LU: u32 = 0xd0300053;
pub const MASK_FCVT_S_LU: u32 = 0xfff0007f;
pub const MATCH_FCVT_S_Q: u32 = 0x40300053;
pub const MASK_FCVT_S_Q: u32 = 0xfff0007f;
pub const MATCH_FCVT_S_W: u32 = 0xd0000053;
pub const MASK_FCVT_S_W: u32 = 0xfff0007f;
pub const MATCH_FCVT_S_WU: u32 = 0xd0100053;
pub const MASK_FCVT_S_WU: u32 = 0xfff0007f;
pub const MATCH_FCVT_W_D: u32 = 0xc2000053;
pub const MASK_FCVT_W_D: u32 = 0xfff0007f;
pub const MATCH_FCVT_W_H: u32 = 0xc4000053;
pub const MASK_FCVT_W_H: u32 = 0xfff0007f;
pub const MATCH_FCVT_W_Q: u32 = 0xc6000053;
pub const MASK_FCVT_W_Q: u32 = 0xfff0007f;
pub const MATCH_FCVT_W_S: u32 = 0xc0000053;
pub const MASK_FCVT_W_S: u32 = 0xfff0007f;
pub const MATCH_FCVT_WU_D: u32 = 0xc2100053;
pub const MASK_FCVT_WU_D: u32 = 0xfff0007f;
pub const MATCH_FCVT_WU_H: u32 = 0xc4100053;
pub const MASK_FCVT_WU_H: u32 = 0xfff0007f;
pub const MATCH_FCVT_WU_Q: u32 = 0xc6100053;
pub const MASK_FCVT_WU_Q: u32 = 0xfff0007f;
pub const MATCH_FCVT_WU_S: u32 = 0xc0100053;
pub const MASK_FCVT_WU_S: u32 = 0xfff0007f;
pub const MATCH_FCVTMOD_W_D: u32 = 0xc2801053;
pub const MASK_FCVTMOD_W_D: u32 = 0xfff0707f;
pub const MATCH_FDIV_D: u32 = 0x1a000053;
pub const MASK_FDIV_D: u32 = 0xfe00007f;
pub const MATCH_FDIV_H: u32 = 0x1c000053;
pub const MASK_FDIV_H: u32 = 0xfe00007f;
pub const MATCH_FDIV_Q: u32 = 0x1e000053;
pub const MASK_FDIV_Q: u32 = 0xfe00007f;
pub const MATCH_FDIV_S: u32 = 0x18000053;
pub const MASK_FDIV_S: u32 = 0xfe00007f;
pub const MATCH_FENCE: u32 = 0xf;
pub const MASK_FENCE: u32 = 0x707f;
pub const MATCH_FENCE_I: u32 = 0x100f;
pub const MASK_FENCE_I: u32 = 0x707f;
pub const MATCH_FENCE_TSO: u32 = 0x8330000f;
pub const MASK_FENCE_TSO: u32 = 0xfff0707f;
pub const MATCH_FEQ_D: u32 = 0xa2002053;
pub const MASK_FEQ_D: u32 = 0xfe00707f;
pub const MATCH_FEQ_H: u32 = 0xa4002053;
pub const MASK_FEQ_H: u32 = 0xfe00707f;
pub const MATCH_FEQ_Q: u32 = 0xa6002053;
pub const MASK_FEQ_Q: u32 = 0xfe00707f;
pub const MATCH_FEQ_S: u32 = 0xa0002053;
pub const MASK_FEQ_S: u32 = 0xfe00707f;
pub const MATCH_FLD: u32 = 0x3007;
pub const MASK_FLD: u32 = 0x707f;
pub const MATCH_FLE_D: u32 = 0xa2000053;
pub const MASK_FLE_D: u32 = 0xfe00707f;
pub const MATCH_FLE_H: u32 = 0xa4000053;
pub const MASK_FLE_H: u32 = 0xfe00707f;
pub const MATCH_FLE_Q: u32 = 0xa6000053;
pub const MASK_FLE_Q: u32 = 0xfe00707f;
pub const MATCH_FLE_S: u32 = 0xa0000053;
pub const MASK_FLE_S: u32 = 0xfe00707f;
pub const MATCH_FLEQ_D: u32 = 0xa2004053;
pub const MASK_FLEQ_D: u32 = 0xfe00707f;
pub const MATCH_FLEQ_H: u32 = 0xa4004053;
pub const MASK_FLEQ_H: u32 = 0xfe00707f;
pub const MATCH_FLEQ_Q: u32 = 0xa6004053;
pub const MASK_FLEQ_Q: u32 = 0xfe00707f;
pub const MATCH_FLEQ_S: u32 = 0xa0004053;
pub const MASK_FLEQ_S: u32 = 0xfe00707f;
pub const MATCH_FLH: u32 = 0x1007;
pub const MASK_FLH: u32 = 0x707f;
pub const MATCH_FLI_D: u32 = 0xf2100053;
pub const MASK_FLI_D: u32 = 0xfff0707f;
pub const MATCH_FLI_H: u32 = 0xf4100053;
pub const MASK_FLI_H: u32 = 0xfff0707f;
pub const MATCH_FLI_Q: u32 = 0xf6100053;
pub const MASK_FLI_Q: u32 = 0xfff0707f;
pub const MATCH_FLI_S: u32 = 0xf0100053;
pub const MASK_FLI_S: u32 = 0xfff0707f;
pub const MATCH_FLQ: u32 = 0x4007;
pub const MASK_FLQ: u32 = 0x707f;
pub const MATCH_FLT_D: u32 = 0xa2001053;
pub const MASK_FLT_D: u32 = 0xfe00707f;
pub const MATCH_FLT_H: u32 = 0xa4001053;
pub const MASK_FLT_H: u32 = 0xfe00707f;
pub const MATCH_FLT_Q: u32 = 0xa6001053;
pub const MASK_FLT_Q: u32 = 0xfe00707f;
pub const MATCH_FLT_S: u32 = 0xa0001053;
pub const MASK_FLT_S: u32 = 0xfe00707f;
pub const MATCH_FLTQ_D: u32 = 0xa2005053;
pub const MASK_FLTQ_D: u32 = 0xfe00707f;
pub const MATCH_FLTQ_H: u32 = 0xa4005053;
pub const MASK_FLTQ_H: u32 = 0xfe00707f;
pub const MATCH_FLTQ_Q: u32 = 0xa6005053;
pub const MASK_FLTQ_Q: u32 = 0xfe00707f;
pub const MATCH_FLTQ_S: u32 = 0xa0005053;
pub const MASK_FLTQ_S: u32 = 0xfe00707f;
pub const MATCH_FLW: u32 = 0x2007;
pub const MASK_FLW: u32 = 0x707f;
pub const MATCH_FMADD_D: u32 = 0x2000043;
pub const MASK_FMADD_D: u32 = 0x600007f;
pub const MATCH_FMADD_H: u32 = 0x4000043;
pub const MASK_FMADD_H: u32 = 0x600007f;
pub const MATCH_FMADD_Q: u32 = 0x6000043;
pub const MASK_FMADD_Q: u32 = 0x600007f;
pub const MATCH_FMADD_S: u32 = 0x43;
pub const MASK_FMADD_S: u32 = 0x600007f;
pub const MATCH_FMAX_D: u32 = 0x2a001053;
pub const MASK_FMAX_D: u32 = 0xfe00707f;
pub const MATCH_FMAX_H: u32 = 0x2c001053;
pub const MASK_FMAX_H: u32 = 0xfe00707f;
pub const MATCH_FMAX_Q: u32 = 0x2e001053;
pub const MASK_FMAX_Q: u32 = 0xfe00707f;
pub const MATCH_FMAX_S: u32 = 0x28001053;
pub const MASK_FMAX_S: u32 = 0xfe00707f;
pub const MATCH_FMAXM_D: u32 = 0x2a003053;
pub const MASK_FMAXM_D: u32 = 0xfe00707f;
pub const MATCH_FMAXM_H: u32 = 0x2c003053;
pub const MASK_FMAXM_H: u32 = 0xfe00707f;
pub const MATCH_FMAXM_Q: u32 = 0x2e003053;
pub const MASK_FMAXM_Q: u32 = 0xfe00707f;
pub const MATCH_FMAXM_S: u32 = 0x28003053;
pub const MASK_FMAXM_S: u32 = 0xfe00707f;
pub const MATCH_FMIN_D: u32 = 0x2a000053;
pub const MASK_FMIN_D: u32 = 0xfe00707f;
pub const MATCH_FMIN_H: u32 = 0x2c000053;
pub const MASK_FMIN_H: u32 = 0xfe00707f;
pub const MATCH_FMIN_Q: u32 = 0x2e000053;
pub const MASK_FMIN_Q: u32 = 0xfe00707f;
pub const MATCH_FMIN_S: u32 = 0x28000053;
pub const MASK_FMIN_S: u32 = 0xfe00707f;
pub const MATCH_FMINM_D: u32 = 0x2a002053;
pub const MASK_FMINM_D: u32 = 0xfe00707f;
pub const MATCH_FMINM_H: u32 = 0x2c002053;
pub const MASK_FMINM_H: u32 = 0xfe00707f;
pub const MATCH_FMINM_Q: u32 = 0x2e002053;
pub const MASK_FMINM_Q: u32 = 0xfe00707f;
pub const MATCH_FMINM_S: u32 = 0x28002053;
pub const MASK_FMINM_S: u32 = 0xfe00707f;
pub const MATCH_FMSUB_D: u32 = 0x2000047;
pub const MASK_FMSUB_D: u32 = 0x600007f;
pub const MATCH_FMSUB_H: u32 = 0x4000047;
pub const MASK_FMSUB_H: u32 = 0x600007f;
pub const MATCH_FMSUB_Q: u32 = 0x6000047;
pub const MASK_FMSUB_Q: u32 = 0x600007f;
pub const MATCH_FMSUB_S: u32 = 0x47;
pub const MASK_FMSUB_S: u32 = 0x600007f;
pub const MATCH_FMUL_D: u32 = 0x12000053;
pub const MASK_FMUL_D: u32 = 0xfe00007f;
pub const MATCH_FMUL_H: u32 = 0x14000053;
pub const MASK_FMUL_H: u32 = 0xfe00007f;
pub const MATCH_FMUL_Q: u32 = 0x16000053;
pub const MASK_FMUL_Q: u32 = 0xfe00007f;
pub const MATCH_FMUL_S: u32 = 0x10000053;
pub const MASK_FMUL_S: u32 = 0xfe00007f;
pub const MATCH_FMV_D: u32 = 0x22000053;
pub const MASK_FMV_D: u32 = 0xfe00707f;
pub const MATCH_FMV_D_X: u32 = 0xf2000053;
pub const MASK_FMV_D_X: u32 = 0xfff0707f;
pub const MATCH_FMV_H: u32 = 0x24000053;
pub const MASK_FMV_H: u32 = 0xfe00707f;
pub const MATCH_FMV_H_X: u32 = 0xf4000053;
pub const MASK_FMV_H_X: u32 = 0xfff0707f;
pub const MATCH_FMV_Q: u32 = 0x26000053;
pub const MASK_FMV_Q: u32 = 0xfe00707f;
pub const MATCH_FMV_S: u32 = 0x20000053;
pub const MASK_FMV_S: u32 = 0xfe00707f;
pub const MATCH_FMV_S_X: u32 = 0xf0000053;
pub const MASK_FMV_S_X: u32 = 0xfff0707f;
pub const MATCH_FMV_W_X: u32 = 0xf0000053;
pub const MASK_FMV_W_X: u32 = 0xfff0707f;
pub const MATCH_FMV_X_D: u32 = 0xe2000053;
pub const MASK_FMV_X_D: u32 = 0xfff0707f;
pub const MATCH_FMV_X_H: u32 = 0xe4000053;
pub const MASK_FMV_X_H: u32 = 0xfff0707f;
pub const MATCH_FMV_X_S: u32 = 0xe0000053;
pub const MASK_FMV_X_S: u32 = 0xfff0707f;
pub const MATCH_FMV_X_W: u32 = 0xe0000053;
pub const MASK_FMV_X_W: u32 = 0xfff0707f;
pub const MATCH_FMVH_X_D: u32 = 0xe2100053;
pub const MASK_FMVH_X_D: u32 = 0xfff0707f;
pub const MATCH_FMVH_X_Q: u32 = 0xe6100053;
pub const MASK_FMVH_X_Q: u32 = 0xfff0707f;
pub const MATCH_FMVP_D_X: u32 = 0xb2000053;
pub const MASK_FMVP_D_X: u32 = 0xfe00707f;
pub const MATCH_FMVP_Q_X: u32 = 0xb6000053;
pub const MASK_FMVP_Q_X: u32 = 0xfe00707f;
pub const MATCH_FNEG_D: u32 = 0x22001053;
pub const MASK_FNEG_D: u32 = 0xfe00707f;
pub const MATCH_FNEG_H: u32 = 0x24001053;
pub const MASK_FNEG_H: u32 = 0xfe00707f;
pub const MATCH_FNEG_Q: u32 = 0x26001053;
pub const MASK_FNEG_Q: u32 = 0xfe00707f;
pub const MATCH_FNEG_S: u32 = 0x20001053;
pub const MASK_FNEG_S: u32 = 0xfe00707f;
pub const MATCH_FNMADD_D: u32 = 0x200004f;
pub const MASK_FNMADD_D: u32 = 0x600007f;
pub const MATCH_FNMADD_H: u32 = 0x400004f;
pub const MASK_FNMADD_H: u32 = 0x600007f;
pub const MATCH_FNMADD_Q: u32 = 0x600004f;
pub const MASK_FNMADD_Q: u32 = 0x600007f;
pub const MATCH_FNMADD_S: u32 = 0x4f;
pub const MASK_FNMADD_S: u32 = 0x600007f;
pub const MATCH_FNMSUB_D: u32 = 0x200004b;
pub const MASK_FNMSUB_D: u32 = 0x600007f;
pub const MATCH_FNMSUB_H: u32 = 0x400004b;
pub const MASK_FNMSUB_H: u32 = 0x600007f;
pub const MATCH_FNMSUB_Q: u32 = 0x600004b;
pub const MASK_FNMSUB_Q: u32 = 0x600007f;
pub const MATCH_FNMSUB_S: u32 = 0x4b;
pub const MASK_FNMSUB_S: u32 = 0x600007f;
pub const MATCH_FRCSR: u32 = 0x302073;
pub const MASK_FRCSR: u32 = 0xfffff07f;
pub const MATCH_FRFLAGS: u32 = 0x102073;
pub const MASK_FRFLAGS: u32 = 0xfffff07f;
pub const MATCH_FROUND_D: u32 = 0x42400053;
pub const MASK_FROUND_D: u32 = 0xfff0007f;
pub const MATCH_FROUND_H: u32 = 0x44400053;
pub const MASK_FROUND_H: u32 = 0xfff0007f;
pub const MATCH_FROUND_Q: u32 = 0x46400053;
pub const MASK_FROUND_Q: u32 = 0xfff0007f;
pub const MATCH_FROUND_S: u32 = 0x40400053;
pub const MASK_FROUND_S: u32 = 0xfff0007f;
pub const MATCH_FROUNDNX_D: u32 = 0x42500053;
pub const MASK_FROUNDNX_D: u32 = 0xfff0007f;
pub const MATCH_FROUNDNX_H: u32 = 0x44500053;
pub const MASK_FROUNDNX_H: u32 = 0xfff0007f;
pub const MATCH_FROUNDNX_Q: u32 = 0x46500053;
pub const MASK_FROUNDNX_Q: u32 = 0xfff0007f;
pub const MATCH_FROUNDNX_S: u32 = 0x40500053;
pub const MASK_FROUNDNX_S: u32 = 0xfff0007f;
pub const MATCH_FRRM: u32 = 0x202073;
pub const MASK_FRRM: u32 = 0xfffff07f;
pub const MATCH_FSCSR: u32 = 0x301073;
pub const MASK_FSCSR: u32 = 0xfff0707f;
pub const MATCH_FSD: u32 = 0x3027;
pub const MASK_FSD: u32 = 0x707f;
pub const MATCH_FSFLAGS: u32 = 0x101073;
pub const MASK_FSFLAGS: u32 = 0xfff0707f;
pub const MATCH_FSFLAGSI: u32 = 0x105073;
pub const MASK_FSFLAGSI: u32 = 0xfff0707f;
pub const MATCH_FSGNJ_D: u32 = 0x22000053;
pub const MASK_FSGNJ_D: u32 = 0xfe00707f;
pub const MATCH_FSGNJ_H: u32 = 0x24000053;
pub const MASK_FSGNJ_H: u32 = 0xfe00707f;
pub const MATCH_FSGNJ_Q: u32 = 0x26000053;
pub const MASK_FSGNJ_Q: u32 = 0xfe00707f;
pub const MATCH_FSGNJ_S: u32 = 0x20000053;
pub const MASK_FSGNJ_S: u32 = 0xfe00707f;
pub const MATCH_FSGNJN_D: u32 = 0x22001053;
pub const MASK_FSGNJN_D: u32 = 0xfe00707f;
pub const MATCH_FSGNJN_H: u32 = 0x24001053;
pub const MASK_FSGNJN_H: u32 = 0xfe00707f;
pub const MATCH_FSGNJN_Q: u32 = 0x26001053;
pub const MASK_FSGNJN_Q: u32 = 0xfe00707f;
pub const MATCH_FSGNJN_S: u32 = 0x20001053;
pub const MASK_FSGNJN_S: u32 = 0xfe00707f;
pub const MATCH_FSGNJX_D: u32 = 0x22002053;
pub const MASK_FSGNJX_D: u32 = 0xfe00707f;
pub const MATCH_FSGNJX_H: u32 = 0x24002053;
pub const MASK_FSGNJX_H: u32 = 0xfe00707f;
pub const MATCH_FSGNJX_Q: u32 = 0x26002053;
pub const MASK_FSGNJX_Q: u32 = 0xfe00707f;
pub const MATCH_FSGNJX_S: u32 = 0x20002053;
pub const MASK_FSGNJX_S: u32 = 0xfe00707f;
pub const MATCH_FSH: u32 = 0x1027;
pub const MASK_FSH: u32 = 0x707f;
pub const MATCH_FSQ: u32 = 0x4027;
pub const MASK_FSQ: u32 = 0x707f;
pub const MATCH_FSQRT_D: u32 = 0x5a000053;
pub const MASK_FSQRT_D: u32 = 0xfff0007f;
pub const MATCH_FSQRT_H: u32 = 0x5c000053;
pub const MASK_FSQRT_H: u32 = 0xfff0007f;
pub const MATCH_FSQRT_Q: u32 = 0x5e000053;
pub const MASK_FSQRT_Q: u32 = 0xfff0007f;
pub const MATCH_FSQRT_S: u32 = 0x58000053;
pub const MASK_FSQRT_S: u32 = 0xfff0007f;
pub const MATCH_FSRM: u32 = 0x201073;
pub const MASK_FSRM: u32 = 0xfff0707f;
pub const MATCH_FSRMI: u32 = 0x205073;
pub const MASK_FSRMI: u32 = 0xfff0707f;
pub const MATCH_FSUB_D: u32 = 0xa000053;
pub const MASK_FSUB_D: u32 = 0xfe00007f;
pub const MATCH_FSUB_H: u32 = 0xc000053;
pub const MASK_FSUB_H: u32 = 0xfe00007f;
pub const MATCH_FSUB_Q: u32 = 0xe000053;
pub const MASK_FSUB_Q: u32 = 0xfe00007f;
pub const MATCH_FSUB_S: u32 = 0x8000053;
pub const MASK_FSUB_S: u32 = 0xfe00007f;
pub const MATCH_FSW: u32 = 0x2027;
pub const MASK_FSW: u32 = 0x707f;
pub const MATCH_HFENCE_GVMA: u32 = 0x62000073;
pub const MASK_HFENCE_GVMA: u32 = 0xfe007fff;
pub const MATCH_HFENCE_VVMA: u32 = 0x22000073;
pub const MASK_HFENCE_VVMA: u32 = 0xfe007fff;
pub const MATCH_HINVAL_GVMA: u32 = 0x66000073;
pub const MASK_HINVAL_GVMA: u32 = 0xfe007fff;
pub const MATCH_HINVAL_VVMA: u32 = 0x26000073;
pub const MASK_HINVAL_VVMA: u32 = 0xfe007fff;
pub const MATCH_HLV_B: u32 = 0x60004073;
pub const MASK_HLV_B: u32 = 0xfff0707f;
pub const MATCH_HLV_BU: u32 = 0x60104073;
pub const MASK_HLV_BU: u32 = 0xfff0707f;
pub const MATCH_HLV_D: u32 = 0x6c004073;
pub const MASK_HLV_D: u32 = 0xfff0707f;
pub const MATCH_HLV_H: u32 = 0x64004073;
pub const MASK_HLV_H: u32 = 0xfff0707f;
pub const MATCH_HLV_HU: u32 = 0x64104073;
pub const MASK_HLV_HU: u32 = 0xfff0707f;
pub const MATCH_HLV_W: u32 = 0x68004073;
pub const MASK_HLV_W: u32 = 0xfff0707f;
pub const MATCH_HLV_WU: u32 = 0x68104073;
pub const MASK_HLV_WU: u32 = 0xfff0707f;
pub const MATCH_HLVX_HU: u32 = 0x64304073;
pub const MASK_HLVX_HU: u32 = 0xfff0707f;
pub const MATCH_HLVX_WU: u32 = 0x68304073;
pub const MASK_HLVX_WU: u32 = 0xfff0707f;
pub const MATCH_HSV_B: u32 = 0x62004073;
pub const MASK_HSV_B: u32 = 0xfe007fff;
pub const MATCH_HSV_D: u32 = 0x6e004073;
pub const MASK_HSV_D: u32 = 0xfe007fff;
pub const MATCH_HSV_H: u32 = 0x66004073;
pub const MASK_HSV_H: u32 = 0xfe007fff;
pub const MATCH_HSV_W: u32 = 0x6a004073;
pub const MASK_HSV_W: u32 = 0xfe007fff;
pub const MATCH_J: u32 = 0x6f;
pub const MASK_J: u32 = 0xfff;
pub const MATCH_JAL: u32 = 0x6f;
pub const MASK_JAL: u32 = 0x7f;
pub const MATCH_JAL_PSEUDO: u32 = 0xef;
pub const MASK_JAL_PSEUDO: u32 = 0xfff;
pub const MATCH_JALR: u32 = 0x67;
pub const MASK_JALR: u32 = 0x707f;
pub const MATCH_JALR_PSEUDO: u32 = 0xe7;
pub const MASK_JALR_PSEUDO: u32 = 0xfff07fff;
pub const MATCH_JR: u32 = 0x67;
pub const MASK_JR: u32 = 0xfff07fff;
pub const MATCH_LB: u32 = 0x3;
pub const MASK_LB: u32 = 0x707f;
pub const MATCH_LBU: u32 = 0x4003;
pub const MASK_LBU: u32 = 0x707f;
pub const MATCH_LD: u32 = 0x3003;
pub const MASK_LD: u32 = 0x707f;
pub const MATCH_LH: u32 = 0x1003;
pub const MASK_LH: u32 = 0x707f;
pub const MATCH_LHU: u32 = 0x5003;
pub const MASK_LHU: u32 = 0x707f;
pub const MATCH_LR_D: u32 = 0x1000302f;
pub const MASK_LR_D: u32 = 0xf9f0707f;
pub const MATCH_LR_W: u32 = 0x1000202f;
pub const MASK_LR_W: u32 = 0xf9f0707f;
pub const MATCH_LUI: u32 = 0x37;
pub const MASK_LUI: u32 = 0x7f;
pub const MATCH_LW: u32 = 0x2003;
pub const MASK_LW: u32 = 0x707f;
pub const MATCH_LWU: u32 = 0x6003;
pub const MASK_LWU: u32 = 0x707f;
pub const MATCH_MAX: u32 = 0xa006033;
pub const MASK_MAX: u32 = 0xfe00707f;
pub const MATCH_MAXU: u32 = 0xa007033;
pub const MASK_MAXU: u32 = 0xfe00707f;
pub const MATCH_MIN: u32 = 0xa004033;
pub const MASK_MIN: u32 = 0xfe00707f;
pub const MATCH_MINU: u32 = 0xa005033;
pub const MASK_MINU: u32 = 0xfe00707f;
pub const MATCH_MOP_R_0: u32 = 0x81c04073;
pub const MASK_MOP_R_0: u32 = 0xfff0707f;
pub const MATCH_MOP_R_1: u32 = 0x81d04073;
pub const MASK_MOP_R_1: u32 = 0xfff0707f;
pub const MATCH_MOP_R_10: u32 = 0x89e04073;
pub const MASK_MOP_R_10: u32 = 0xfff0707f;
pub const MATCH_MOP_R_11: u32 = 0x89f04073;
pub const MASK_MOP_R_11: u32 = 0xfff0707f;
pub const MATCH_MOP_R_12: u32 = 0x8dc04073;
pub const MASK_MOP_R_12: u32 = 0xfff0707f;
pub const MATCH_MOP_R_13: u32 = 0x8dd04073;
pub const MASK_MOP_R_13: u32 = 0xfff0707f;
pub const MATCH_MOP_R_14: u32 = 0x8de04073;
pub const MASK_MOP_R_14: u32 = 0xfff0707f;
pub const MATCH_MOP_R_15: u32 = 0x8df04073;
pub const MASK_MOP_R_15: u32 = 0xfff0707f;
pub const MATCH_MOP_R_16: u32 = 0xc1c04073;
pub const MASK_MOP_R_16: u32 = 0xfff0707f;
pub const MATCH_MOP_R_17: u32 = 0xc1d04073;
pub const MASK_MOP_R_17: u32 = 0xfff0707f;
pub const MATCH_MOP_R_18: u32 = 0xc1e04073;
pub const MASK_MOP_R_18: u32 = 0xfff0707f;
pub const MATCH_MOP_R_19: u32 = 0xc1f04073;
pub const MASK_MOP_R_19: u32 = 0xfff0707f;
pub const MATCH_MOP_R_2: u32 = 0x81e04073;
pub const MASK_MOP_R_2: u32 = 0xfff0707f;
pub const MATCH_MOP_R_20: u32 = 0xc5c04073;
pub const MASK_MOP_R_20: u32 = 0xfff0707f;
pub const MATCH_MOP_R_21: u32 = 0xc5d04073;
pub const MASK_MOP_R_21: u32 = 0xfff0707f;
pub const MATCH_MOP_R_22: u32 = 0xc5e04073;
pub const MASK_MOP_R_22: u32 = 0xfff0707f;
pub const MATCH_MOP_R_23: u32 = 0xc5f04073;
pub const MASK_MOP_R_23: u32 = 0xfff0707f;
pub const MATCH_MOP_R_24: u32 = 0xc9c04073;
pub const MASK_MOP_R_24: u32 = 0xfff0707f;
pub const MATCH_MOP_R_25: u32 = 0xc9d04073;
pub const MASK_MOP_R_25: u32 = 0xfff0707f;
pub const MATCH_MOP_R_26: u32 = 0xc9e04073;
pub const MASK_MOP_R_26: u32 = 0xfff0707f;
pub const MATCH_MOP_R_27: u32 = 0xc9f04073;
pub const MASK_MOP_R_27: u32 = 0xfff0707f;
pub const MATCH_MOP_R_28: u32 = 0xcdc04073;
pub const MASK_MOP_R_28: u32 = 0xfff0707f;
pub const MATCH_MOP_R_29: u32 = 0xcdd04073;
pub const MASK_MOP_R_29: u32 = 0xfff0707f;
pub const MATCH_MOP_R_3: u32 = 0x81f04073;
pub const MASK_MOP_R_3: u32 = 0xfff0707f;
pub const MATCH_MOP_R_30: u32 = 0xcde04073;
pub const MASK_MOP_R_30: u32 = 0xfff0707f;
pub const MATCH_MOP_R_31: u32 = 0xcdf04073;
pub const MASK_MOP_R_31: u32 = 0xfff0707f;
pub const MATCH_MOP_R_4: u32 = 0x85c04073;
pub const MASK_MOP_R_4: u32 = 0xfff0707f;
pub const MATCH_MOP_R_5: u32 = 0x85d04073;
pub const MASK_MOP_R_5: u32 = 0xfff0707f;
pub const MATCH_MOP_R_6: u32 = 0x85e04073;
pub const MASK_MOP_R_6: u32 = 0xfff0707f;
pub const MATCH_MOP_R_7: u32 = 0x85f04073;
pub const MASK_MOP_R_7: u32 = 0xfff0707f;
pub const MATCH_MOP_R_8: u32 = 0x89c04073;
pub const MASK_MOP_R_8: u32 = 0xfff0707f;
pub const MATCH_MOP_R_9: u32 = 0x89d04073;
pub const MASK_MOP_R_9: u32 = 0xfff0707f;
pub const MATCH_MOP_R_N: u32 = 0x81c04073;
pub const MASK_MOP_R_N: u32 = 0xb3c0707f;
pub const MATCH_MOP_RR_0: u32 = 0x82004073;
pub const MASK_MOP_RR_0: u32 = 0xfe00707f;
pub const MATCH_MOP_RR_1: u32 = 0x86004073;
pub const MASK_MOP_RR_1: u32 = 0xfe00707f;
pub const MATCH_MOP_RR_2: u32 = 0x8a004073;
pub const MASK_MOP_RR_2: u32 = 0xfe00707f;
pub const MATCH_MOP_RR_3: u32 = 0x8e004073;
pub const MASK_MOP_RR_3: u32 = 0xfe00707f;
pub const MATCH_MOP_RR_4: u32 = 0xc2004073;
pub const MASK_MOP_RR_4: u32 = 0xfe00707f;
pub const MATCH_MOP_RR_5: u32 = 0xc6004073;
pub const MASK_MOP_RR_5: u32 = 0xfe00707f;
pub const MATCH_MOP_RR_6: u32 = 0xca004073;
pub const MASK_MOP_RR_6: u32 = 0xfe00707f;
pub const MATCH_MOP_RR_7: u32 = 0xce004073;
pub const MASK_MOP_RR_7: u32 = 0xfe00707f;
pub const MATCH_MOP_RR_N: u32 = 0x82004073;
pub const MASK_MOP_RR_N: u32 = 0xb200707f;
pub const MATCH_MRET: u32 = 0x30200073;
pub const MASK_MRET: u32 = 0xffffffff;
pub const MATCH_MUL: u32 = 0x2000033;
pub const MASK_MUL: u32 = 0xfe00707f;
pub const MATCH_MULH: u32 = 0x2001033;
pub const MASK_MULH: u32 = 0xfe00707f;
pub const MATCH_MULHSU: u32 = 0x2002033;
pub const MASK_MULHSU: u32 = 0xfe00707f;
pub const MATCH_MULHU: u32 = 0x2003033;
pub const MASK_MULHU: u32 = 0xfe00707f;
pub const MATCH_MULW: u32 = 0x200003b;
pub const MASK_MULW: u32 = 0xfe00707f;
pub const MATCH_MV: u32 = 0x13;
pub const MASK_MV: u32 = 0xfff0707f;
pub const MATCH_NEG: u32 = 0x40000033;
pub const MASK_NEG: u32 = 0xfff0707f;
pub const MATCH_NOP: u32 = 0x13;
pub const MASK_NOP: u32 = 0xffffffff;
pub const MATCH_NTL_ALL: u32 = 0x500033;
pub const MASK_NTL_ALL: u32 = 0xffffffff;
pub const MATCH_NTL_P1: u32 = 0x200033;
pub const MASK_NTL_P1: u32 = 0xffffffff;
pub const MATCH_NTL_PALL: u32 = 0x300033;
pub const MASK_NTL_PALL: u32 = 0xffffffff;
pub const MATCH_NTL_S1: u32 = 0x400033;
pub const MASK_NTL_S1: u32 = 0xffffffff;
pub const MATCH_OR: u32 = 0x6033;
pub const MASK_OR: u32 = 0xfe00707f;
pub const MATCH_ORC_B: u32 = 0x28705013;
pub const MASK_ORC_B: u32 = 0xfff0707f;
pub const MATCH_ORI: u32 = 0x6013;
pub const MASK_ORI: u32 = 0x707f;
pub const MATCH_ORN: u32 = 0x40006033;
pub const MASK_ORN: u32 = 0xfe00707f;
pub const MATCH_PACK: u32 = 0x8004033;
pub const MASK_PACK: u32 = 0xfe00707f;
pub const MATCH_PACKH: u32 = 0x8007033;
pub const MASK_PACKH: u32 = 0xfe00707f;
pub const MATCH_PACKW: u32 = 0x800403b;
pub const MASK_PACKW: u32 = 0xfe00707f;
pub const MATCH_PAUSE: u32 = 0x100000f;
pub const MASK_PAUSE: u32 = 0xffffffff;
pub const MATCH_PREFETCH_I: u32 = 0x6013;
pub const MASK_PREFETCH_I: u32 = 0x1f07fff;
pub const MATCH_PREFETCH_R: u32 = 0x106013;
pub const MASK_PREFETCH_R: u32 = 0x1f07fff;
pub const MATCH_PREFETCH_W: u32 = 0x306013;
pub const MASK_PREFETCH_W: u32 = 0x1f07fff;
pub const MATCH_RDCYCLE: u32 = 0xc0002073;
pub const MASK_RDCYCLE: u32 = 0xfffff07f;
pub const MATCH_RDCYCLEH: u32 = 0xc8002073;
pub const MASK_RDCYCLEH: u32 = 0xfffff07f;
pub const MATCH_RDINSTRET: u32 = 0xc0202073;
pub const MASK_RDINSTRET: u32 = 0xfffff07f;
pub const MATCH_RDINSTRETH: u32 = 0xc8202073;
pub const MASK_RDINSTRETH: u32 = 0xfffff07f;
pub const MATCH_RDTIME: u32 = 0xc0102073;
pub const MASK_RDTIME: u32 = 0xfffff07f;
pub const MATCH_RDTIMEH: u32 = 0xc8102073;
pub const MASK_RDTIMEH: u32 = 0xfffff07f;
pub const MATCH_REM: u32 = 0x2006033;
pub const MASK_REM: u32 = 0xfe00707f;
pub const MATCH_REMU: u32 = 0x2007033;
pub const MASK_REMU: u32 = 0xfe00707f;
pub const MATCH_REMUW: u32 = 0x200703b;
pub const MASK_REMUW: u32 = 0xfe00707f;
pub const MATCH_REMW: u32 = 0x200603b;
pub const MASK_REMW: u32 = 0xfe00707f;
pub const MATCH_RET: u32 = 0x8067;
pub const MASK_RET: u32 = 0xffffffff;
pub const MATCH_REV8: u32 = 0x6b805013;
pub const MASK_REV8: u32 = 0xfff0707f;
pub const MATCH_REV8_RV32: u32 = 0x69805013;
pub const MASK_REV8_RV32: u32 = 0xfff0707f;
pub const MATCH_ROL: u32 = 0x60001033;
pub const MASK_ROL: u32 = 0xfe00707f;
pub const MATCH_ROLW: u32 = 0x6000103b;
pub const MASK_ROLW: u32 = 0xfe00707f;
pub const MATCH_ROR: u32 = 0x60005033;
pub const MASK_ROR: u32 = 0xfe00707f;
pub const MATCH_RORI: u32 = 0x60005013;
pub const MASK_RORI: u32 = 0xfc00707f;
pub const MATCH_RORI_RV32: u32 = 0x60005013;
pub const MASK_RORI_RV32: u32 = 0xfe00707f;
pub const MATCH_RORIW: u32 = 0x6000501b;
pub const MASK_RORIW: u32 = 0xfe00707f;
pub const MATCH_RORW: u32 = 0x6000503b;
pub const MASK_RORW: u32 = 0xfe00707f;
pub const MATCH_SB: u32 = 0x23;
pub const MASK_SB: u32 = 0x707f;
pub const MATCH_SBREAK: u32 = 0x100073;
pub const MASK_SBREAK: u32 = 0xffffffff;
pub const MATCH_SC_D: u32 = 0x1800302f;
pub const MASK_SC_D: u32 = 0xf800707f;
pub const MATCH_SC_W: u32 = 0x1800202f;
pub const MASK_SC_W: u32 = 0xf800707f;
pub const MATCH_SCALL: u32 = 0x73;
pub const MASK_SCALL: u32 = 0xffffffff;
pub const MATCH_SD: u32 = 0x3023;
pub const MASK_SD: u32 = 0x707f;
pub const MATCH_SEQZ: u32 = 0x103013;
pub const MASK_SEQZ: u32 = 0xfff0707f;
pub const MATCH_SEXT_B: u32 = 0x60401013;
pub const MASK_SEXT_B: u32 = 0xfff0707f;
pub const MATCH_SEXT_H: u32 = 0x60501013;
pub const MASK_SEXT_H: u32 = 0xfff0707f;
pub const MATCH_SEXT_W: u32 = 0x1b;
pub const MASK_SEXT_W: u32 = 0xfff0707f;
pub const MATCH_SFENCE_INVAL_IR: u32 = 0x18100073;
pub const MASK_SFENCE_INVAL_IR: u32 = 0xffffffff;
pub const MATCH_SFENCE_VMA: u32 = 0x12000073;
pub const MASK_SFENCE_VMA: u32 = 0xfe007fff;
pub const MATCH_SFENCE_W_INVAL: u32 = 0x18000073;
pub const MASK_SFENCE_W_INVAL: u32 = 0xffffffff;
pub const MATCH_SGTZ: u32 = 0x2033;
pub const MASK_SGTZ: u32 = 0xfe0ff07f;
pub const MATCH_SH: u32 = 0x1023;
pub const MASK_SH: u32 = 0x707f;
pub const MATCH_SH1ADD: u32 = 0x20002033;
pub const MASK_SH1ADD: u32 = 0xfe00707f;
pub const MATCH_SH1ADD_UW: u32 = 0x2000203b;
pub const MASK_SH1ADD_UW: u32 = 0xfe00707f;
pub const MATCH_SH2ADD: u32 = 0x20004033;
pub const MASK_SH2ADD: u32 = 0xfe00707f;
pub const MATCH_SH2ADD_UW: u32 = 0x2000403b;
pub const MASK_SH2ADD_UW: u32 = 0xfe00707f;
pub const MATCH_SH3ADD: u32 = 0x20006033;
pub const MASK_SH3ADD: u32 = 0xfe00707f;
pub const MATCH_SH3ADD_UW: u32 = 0x2000603b;
pub const MASK_SH3ADD_UW: u32 = 0xfe00707f;
pub const MATCH_SHA256SIG0: u32 = 0x10201013;
pub const MASK_SHA256SIG0: u32 = 0xfff0707f;
pub const MATCH_SHA256SIG1: u32 = 0x10301013;
pub const MASK_SHA256SIG1: u32 = 0xfff0707f;
pub const MATCH_SHA256SUM0: u32 = 0x10001013;
pub const MASK_SHA256SUM0: u32 = 0xfff0707f;
pub const MATCH_SHA256SUM1: u32 = 0x10101013;
pub const MASK_SHA256SUM1: u32 = 0xfff0707f;
pub const MATCH_SHA512SIG0: u32 = 0x10601013;
pub const MASK_SHA512SIG0: u32 = 0xfff0707f;
pub const MATCH_SHA512SIG0H: u32 = 0x5c000033;
pub const MASK_SHA512SIG0H: u32 = 0xfe00707f;
pub const MATCH_SHA512SIG0L: u32 = 0x54000033;
pub const MASK_SHA512SIG0L: u32 = 0xfe00707f;
pub const MATCH_SHA512SIG1: u32 = 0x10701013;
pub const MASK_SHA512SIG1: u32 = 0xfff0707f;
pub const MATCH_SHA512SIG1H: u32 = 0x5e000033;
pub const MASK_SHA512SIG1H: u32 = 0xfe00707f;
pub const MATCH_SHA512SIG1L: u32 = 0x56000033;
pub const MASK_SHA512SIG1L: u32 = 0xfe00707f;
pub const MATCH_SHA512SUM0: u32 = 0x10401013;
pub const MASK_SHA512SUM0: u32 = 0xfff0707f;
pub const MATCH_SHA512SUM0R: u32 = 0x50000033;
pub const MASK_SHA512SUM0R: u32 = 0xfe00707f;
pub const MATCH_SHA512SUM1: u32 = 0x10501013;
pub const MASK_SHA512SUM1: u32 = 0xfff0707f;
pub const MATCH_SHA512SUM1R: u32 = 0x52000033;
pub const MASK_SHA512SUM1R: u32 = 0xfe00707f;
pub const MATCH_SINVAL_VMA: u32 = 0x16000073;
pub const MASK_SINVAL_VMA: u32 = 0xfe007fff;
pub const MATCH_SLL: u32 = 0x1033;
pub const MASK_SLL: u32 = 0xfe00707f;
pub const MATCH_SLLI: u32 = 0x1013;
pub const MASK_SLLI: u32 = 0xfc00707f;
pub const MATCH_SLLI_RV32: u32 = 0x1013;
pub const MASK_SLLI_RV32: u32 = 0xfe00707f;
pub const MATCH_SLLI_UW: u32 = 0x800101b;
pub const MASK_SLLI_UW: u32 = 0xfc00707f;
pub const MATCH_SLLIW: u32 = 0x101b;
pub const MASK_SLLIW: u32 = 0xfe00707f;
pub const MATCH_SLLW: u32 = 0x103b;
pub const MASK_SLLW: u32 = 0xfe00707f;
pub const MATCH_SLT: u32 = 0x2033;
pub const MASK_SLT: u32 = 0xfe00707f;
pub const MATCH_SLTI: u32 = 0x2013;
pub const MASK_SLTI: u32 = 0x707f;
pub const MATCH_SLTIU: u32 = 0x3013;
pub const MASK_SLTIU: u32 = 0x707f;
pub const MATCH_SLTU: u32 = 0x3033;
pub const MASK_SLTU: u32 = 0xfe00707f;
pub const MATCH_SLTZ: u32 = 0x2033;
pub const MASK_SLTZ: u32 = 0xfff0707f;
pub const MATCH_SM3P0: u32 = 0x10801013;
pub const MASK_SM3P0: u32 = 0xfff0707f;
pub const MATCH_SM3P1: u32 = 0x10901013;
pub const MASK_SM3P1: u32 = 0xfff0707f;
pub const MATCH_SM4ED: u32 = 0x30000033;
pub const MASK_SM4ED: u32 = 0x3e00707f;
pub const MATCH_SM4KS: u32 = 0x34000033;
pub const MASK_SM4KS: u32 = 0x3e00707f;
pub const MATCH_SNEZ: u32 = 0x3033;
pub const MASK_SNEZ: u32 = 0xfe0ff07f;
pub const MATCH_SRA: u32 = 0x40005033;
pub const MASK_SRA: u32 = 0xfe00707f;
pub const MATCH_SRAI: u32 = 0x40005013;
pub const MASK_SRAI: u32 = 0xfc00707f;
pub const MATCH_SRAI_RV32: u32 = 0x40005013;
pub const MASK_SRAI_RV32: u32 = 0xfe00707f;
pub const MATCH_SRAIW: u32 = 0x4000501b;
pub const MASK_SRAIW: u32 = 0xfe00707f;
pub const MATCH_SRAW: u32 = 0x4000503b;
pub const MASK_SRAW: u32 = 0xfe00707f;
pub const MATCH_SRET: u32 = 0x10200073;
pub const MASK_SRET: u32 = 0xffffffff;
pub const MATCH_SRL: u32 = 0x5033;
pub const MASK_SRL: u32 = 0xfe00707f;
pub const MATCH_SRLI: u32 = 0x5013;
pub const MASK_SRLI: u32 = 0xfc00707f;
pub const MATCH_SRLI_RV32: u32 = 0x5013;
pub const MASK_SRLI_RV32: u32 = 0xfe00707f;
pub const MATCH_SRLIW: u32 = 0x501b;
pub const MASK_SRLIW: u32 = 0xfe00707f;
pub const MATCH_SRLW: u32 = 0x503b;
pub const MASK_SRLW: u32 = 0xfe00707f;
pub const MATCH_SUB: u32 = 0x40000033;
pub const MASK_SUB: u32 = 0xfe00707f;
pub const MATCH_SUBW: u32 = 0x4000003b;
pub const MASK_SUBW: u32 = 0xfe00707f;
pub const MATCH_SW: u32 = 0x2023;
pub const MASK_SW: u32 = 0x707f;
pub const MATCH_UNZIP: u32 = 0x8f05013;
pub const MASK_UNZIP: u32 = 0xfff0707f;
pub const MATCH_VAADD_VV: u32 = 0x24002057;
pub const MASK_VAADD_VV: u32 = 0xfc00707f;
pub const MATCH_VAADD_VX: u32 = 0x24006057;
pub const MASK_VAADD_VX: u32 = 0xfc00707f;
pub const MATCH_VAADDU_VV: u32 = 0x20002057;
pub const MASK_VAADDU_VV: u32 = 0xfc00707f;
pub const MATCH_VAADDU_VX: u32 = 0x20006057;
pub const MASK_VAADDU_VX: u32 = 0xfc00707f;
pub const MATCH_VADC_VIM: u32 = 0x40003057;
pub const MASK_VADC_VIM: u32 = 0xfe00707f;
pub const MATCH_VADC_VVM: u32 = 0x40000057;
pub const MASK_VADC_VVM: u32 = 0xfe00707f;
pub const MATCH_VADC_VXM: u32 = 0x40004057;
pub const MASK_VADC_VXM: u32 = 0xfe00707f;
pub const MATCH_VADD_VI: u32 = 0x3057;
pub const MASK_VADD_VI: u32 = 0xfc00707f;
pub const MATCH_VADD_VV: u32 = 0x57;
pub const MASK_VADD_VV: u32 = 0xfc00707f;
pub const MATCH_VADD_VX: u32 = 0x4057;
pub const MASK_VADD_VX: u32 = 0xfc00707f;
pub const MATCH_VAESDF_VS: u32 = 0xa600a077;
pub const MASK_VAESDF_VS: u32 = 0xfe0ff07f;
pub const MATCH_VAESDF_VV: u32 = 0xa200a077;
pub const MASK_VAESDF_VV: u32 = 0xfe0ff07f;
pub const MATCH_VAESDM_VS: u32 = 0xa6002077;
pub const MASK_VAESDM_VS: u32 = 0xfe0ff07f;
pub const MATCH_VAESDM_VV: u32 = 0xa2002077;
pub const MASK_VAESDM_VV: u32 = 0xfe0ff07f;
pub const MATCH_VAESEF_VS: u32 = 0xa601a077;
pub const MASK_VAESEF_VS: u32 = 0xfe0ff07f;
pub const MATCH_VAESEF_VV: u32 = 0xa201a077;
pub const MASK_VAESEF_VV: u32 = 0xfe0ff07f;
pub const MATCH_VAESEM_VS: u32 = 0xa6012077;
pub const MASK_VAESEM_VS: u32 = 0xfe0ff07f;
pub const MATCH_VAESEM_VV: u32 = 0xa2012077;
pub const MASK_VAESEM_VV: u32 = 0xfe0ff07f;
pub const MATCH_VAESKF1_VI: u32 = 0x8a002077;
pub const MASK_VAESKF1_VI: u32 = 0xfe00707f;
pub const MATCH_VAESKF2_VI: u32 = 0xaa002077;
pub const MASK_VAESKF2_VI: u32 = 0xfe00707f;
pub const MATCH_VAESZ_VS: u32 = 0xa603a077;
pub const MASK_VAESZ_VS: u32 = 0xfe0ff07f;
pub const MATCH_VAND_VI: u32 = 0x24003057;
pub const MASK_VAND_VI: u32 = 0xfc00707f;
pub const MATCH_VAND_VV: u32 = 0x24000057;
pub const MASK_VAND_VV: u32 = 0xfc00707f;
pub const MATCH_VAND_VX: u32 = 0x24004057;
pub const MASK_VAND_VX: u32 = 0xfc00707f;
pub const MATCH_VANDN_VV: u32 = 0x4000057;
pub const MASK_VANDN_VV: u32 = 0xfc00707f;
pub const MATCH_VANDN_VX: u32 = 0x4004057;
pub const MASK_VANDN_VX: u32 = 0xfc00707f;
pub const MATCH_VASUB_VV: u32 = 0x2c002057;
pub const MASK_VASUB_VV: u32 = 0xfc00707f;
pub const MATCH_VASUB_VX: u32 = 0x2c006057;
pub const MASK_VASUB_VX: u32 = 0xfc00707f;
pub const MATCH_VASUBU_VV: u32 = 0x28002057;
pub const MASK_VASUBU_VV: u32 = 0xfc00707f;
pub const MATCH_VASUBU_VX: u32 = 0x28006057;
pub const MASK_VASUBU_VX: u32 = 0xfc00707f;
pub const MATCH_VBREV8_V: u32 = 0x48042057;
pub const MASK_VBREV8_V: u32 = 0xfc0ff07f;
pub const MATCH_VBREV_V: u32 = 0x48052057;
pub const MASK_VBREV_V: u32 = 0xfc0ff07f;
pub const MATCH_VCLMUL_VV: u32 = 0x30002057;
pub const MASK_VCLMUL_VV: u32 = 0xfc00707f;
pub const MATCH_VCLMUL_VX: u32 = 0x30006057;
pub const MASK_VCLMUL_VX: u32 = 0xfc00707f;
pub const MATCH_VCLMULH_VV: u32 = 0x34002057;
pub const MASK_VCLMULH_VV: u32 = 0xfc00707f;
pub const MATCH_VCLMULH_VX: u32 = 0x34006057;
pub const MASK_VCLMULH_VX: u32 = 0xfc00707f;
pub const MATCH_VCLZ_V: u32 = 0x48062057;
pub const MASK_VCLZ_V: u32 = 0xfc0ff07f;
pub const MATCH_VCOMPRESS_VM: u32 = 0x5e002057;
pub const MASK_VCOMPRESS_VM: u32 = 0xfe00707f;
pub const MATCH_VCPOP_M: u32 = 0x40082057;
pub const MASK_VCPOP_M: u32 = 0xfc0ff07f;
pub const MATCH_VCPOP_V: u32 = 0x48072057;
pub const MASK_VCPOP_V: u32 = 0xfc0ff07f;
pub const MATCH_VCTZ_V: u32 = 0x4806a057;
pub const MASK_VCTZ_V: u32 = 0xfc0ff07f;
pub const MATCH_VDIV_VV: u32 = 0x84002057;
pub const MASK_VDIV_VV: u32 = 0xfc00707f;
pub const MATCH_VDIV_VX: u32 = 0x84006057;
pub const MASK_VDIV_VX: u32 = 0xfc00707f;
pub const MATCH_VDIVU_VV: u32 = 0x80002057;
pub const MASK_VDIVU_VV: u32 = 0xfc00707f;
pub const MATCH_VDIVU_VX: u32 = 0x80006057;
pub const MASK_VDIVU_VX: u32 = 0xfc00707f;
pub const MATCH_VFADD_VF: u32 = 0x5057;
pub const MASK_VFADD_VF: u32 = 0xfc00707f;
pub const MATCH_VFADD_VV: u32 = 0x1057;
pub const MASK_VFADD_VV: u32 = 0xfc00707f;
pub const MATCH_VFCLASS_V: u32 = 0x4c081057;
pub const MASK_VFCLASS_V: u32 = 0xfc0ff07f;
pub const MATCH_VFCVT_F_X_V: u32 = 0x48019057;
pub const MASK_VFCVT_F_X_V: u32 = 0xfc0ff07f;
pub const MATCH_VFCVT_F_XU_V: u32 = 0x48011057;
pub const MASK_VFCVT_F_XU_V: u32 = 0xfc0ff07f;
pub const MATCH_VFCVT_RTZ_X_F_V: u32 = 0x48039057;
pub const MASK_VFCVT_RTZ_X_F_V: u32 = 0xfc0ff07f;
pub const MATCH_VFCVT_RTZ_XU_F_V: u32 = 0x48031057;
pub const MASK_VFCVT_RTZ_XU_F_V: u32 = 0xfc0ff07f;
pub const MATCH_VFCVT_X_F_V: u32 = 0x48009057;
pub const MASK_VFCVT_X_F_V: u32 = 0xfc0ff07f;
pub const MATCH_VFCVT_XU_F_V: u32 = 0x48001057;
pub const MASK_VFCVT_XU_F_V: u32 = 0xfc0ff07f;
pub const MATCH_VFDIV_VF: u32 = 0x80005057;
pub const MASK_VFDIV_VF: u32 = 0xfc00707f;
pub const MATCH_VFDIV_VV: u32 = 0x80001057;
pub const MASK_VFDIV_VV: u32 = 0xfc00707f;
pub const MATCH_VFIRST_M: u32 = 0x4008a057;
pub const MASK_VFIRST_M: u32 = 0xfc0ff07f;
pub const MATCH_VFMACC_VF: u32 = 0xb0005057;
pub const MASK_VFMACC_VF: u32 = 0xfc00707f;
pub const MATCH_VFMACC_VV: u32 = 0xb0001057;
pub const MASK_VFMACC_VV: u32 = 0xfc00707f;
pub const MATCH_VFMADD_VF: u32 = 0xa0005057;
pub const MASK_VFMADD_VF: u32 = 0xfc00707f;
pub const MATCH_VFMADD_VV: u32 = 0xa0001057;
pub const MASK_VFMADD_VV: u32 = 0xfc00707f;
pub const MATCH_VFMAX_VF: u32 = 0x18005057;
pub const MASK_VFMAX_VF: u32 = 0xfc00707f;
pub const MATCH_VFMAX_VV: u32 = 0x18001057;
pub const MASK_VFMAX_VV: u32 = 0xfc00707f;
pub const MATCH_VFMERGE_VFM: u32 = 0x5c005057;
pub const MASK_VFMERGE_VFM: u32 = 0xfe00707f;
pub const MATCH_VFMIN_VF: u32 = 0x10005057;
pub const MASK_VFMIN_VF: u32 = 0xfc00707f;
pub const MATCH_VFMIN_VV: u32 = 0x10001057;
pub const MASK_VFMIN_VV: u32 = 0xfc00707f;
pub const MATCH_VFMSAC_VF: u32 = 0xb8005057;
pub const MASK_VFMSAC_VF: u32 = 0xfc00707f;
pub const MATCH_VFMSAC_VV: u32 = 0xb8001057;
pub const MASK_VFMSAC_VV: u32 = 0xfc00707f;
pub const MATCH_VFMSUB_VF: u32 = 0xa8005057;
pub const MASK_VFMSUB_VF: u32 = 0xfc00707f;
pub const MATCH_VFMSUB_VV: u32 = 0xa8001057;
pub const MASK_VFMSUB_VV: u32 = 0xfc00707f;
pub const MATCH_VFMUL_VF: u32 = 0x90005057;
pub const MASK_VFMUL_VF: u32 = 0xfc00707f;
pub const MATCH_VFMUL_VV: u32 = 0x90001057;
pub const MASK_VFMUL_VV: u32 = 0xfc00707f;
pub const MATCH_VFMV_F_S: u32 = 0x42001057;
pub const MASK_VFMV_F_S: u32 = 0xfe0ff07f;
pub const MATCH_VFMV_S_F: u32 = 0x42005057;
pub const MASK_VFMV_S_F: u32 = 0xfff0707f;
pub const MATCH_VFMV_V_F: u32 = 0x5e005057;
pub const MASK_VFMV_V_F: u32 = 0xfff0707f;
pub const MATCH_VFNCVT_F_F_W: u32 = 0x480a1057;
pub const MASK_VFNCVT_F_F_W: u32 = 0xfc0ff07f;
pub const MATCH_VFNCVT_F_X_W: u32 = 0x48099057;
pub const MASK_VFNCVT_F_X_W: u32 = 0xfc0ff07f;
pub const MATCH_VFNCVT_F_XU_W: u32 = 0x48091057;
pub const MASK_VFNCVT_F_XU_W: u32 = 0xfc0ff07f;
pub const MATCH_VFNCVT_ROD_F_F_W: u32 = 0x480a9057;
pub const MASK_VFNCVT_ROD_F_F_W: u32 = 0xfc0ff07f;
pub const MATCH_VFNCVT_RTZ_X_F_W: u32 = 0x480b9057;
pub const MASK_VFNCVT_RTZ_X_F_W: u32 = 0xfc0ff07f;
pub const MATCH_VFNCVT_RTZ_XU_F_W: u32 = 0x480b1057;
pub const MASK_VFNCVT_RTZ_XU_F_W: u32 = 0xfc0ff07f;
pub const MATCH_VFNCVT_X_F_W: u32 = 0x48089057;
pub const MASK_VFNCVT_X_F_W: u32 = 0xfc0ff07f;
pub const MATCH_VFNCVT_XU_F_W: u32 = 0x48081057;
pub const MASK_VFNCVT_XU_F_W: u32 = 0xfc0ff07f;
pub const MATCH_VFNMACC_VF: u32 = 0xb4005057;
pub const MASK_VFNMACC_VF: u32 = 0xfc00707f;
pub const MATCH_VFNMACC_VV: u32 = 0xb4001057;
pub const MASK_VFNMACC_VV: u32 = 0xfc00707f;
pub const MATCH_VFNMADD_VF: u32 = 0xa4005057;
pub const MASK_VFNMADD_VF: u32 = 0xfc00707f;
pub const MATCH_VFNMADD_VV: u32 = 0xa4001057;
pub const MASK_VFNMADD_VV: u32 = 0xfc00707f;
pub const MATCH_VFNMSAC_VF: u32 = 0xbc005057;
pub const MASK_VFNMSAC_VF: u32 = 0xfc00707f;
pub const MATCH_VFNMSAC_VV: u32 = 0xbc001057;
pub const MASK_VFNMSAC_VV: u32 = 0xfc00707f;
pub const MATCH_VFNMSUB_VF: u32 = 0xac005057;
pub const MASK_VFNMSUB_VF: u32 = 0xfc00707f;
pub const MATCH_VFNMSUB_VV: u32 = 0xac001057;
pub const MASK_VFNMSUB_VV: u32 = 0xfc00707f;
pub const MATCH_VFRDIV_VF: u32 = 0x84005057;
pub const MASK_VFRDIV_VF: u32 = 0xfc00707f;
pub const MATCH_VFREC7_V: u32 = 0x4c029057;
pub const MASK_VFREC7_V: u32 = 0xfc0ff07f;
pub const MATCH_VFREDMAX_VS: u32 = 0x1c001057;
pub const MASK_VFREDMAX_VS: u32 = 0xfc00707f;
pub const MATCH_VFREDMIN_VS: u32 = 0x14001057;
pub const MASK_VFREDMIN_VS: u32 = 0xfc00707f;
pub const MATCH_VFREDOSUM_VS: u32 = 0xc001057;
pub const MASK_VFREDOSUM_VS: u32 = 0xfc00707f;
pub const MATCH_VFREDSUM_VS: u32 = 0x4001057;
pub const MASK_VFREDSUM_VS: u32 = 0xfc00707f;
pub const MATCH_VFREDUSUM_VS: u32 = 0x4001057;
pub const MASK_VFREDUSUM_VS: u32 = 0xfc00707f;
pub const MATCH_VFRSQRT7_V: u32 = 0x4c021057;
pub const MASK_VFRSQRT7_V: u32 = 0xfc0ff07f;
pub const MATCH_VFRSUB_VF: u32 = 0x9c005057;
pub const MASK_VFRSUB_VF: u32 = 0xfc00707f;
pub const MATCH_VFSGNJ_VF: u32 = 0x20005057;
pub const MASK_VFSGNJ_VF: u32 = 0xfc00707f;
pub const MATCH_VFSGNJ_VV: u32 = 0x20001057;
pub const MASK_VFSGNJ_VV: u32 = 0xfc00707f;
pub const MATCH_VFSGNJN_VF: u32 = 0x24005057;
pub const MASK_VFSGNJN_VF: u32 = 0xfc00707f;
pub const MATCH_VFSGNJN_VV: u32 = 0x24001057;
pub const MASK_VFSGNJN_VV: u32 = 0xfc00707f;
pub const MATCH_VFSGNJX_VF: u32 = 0x28005057;
pub const MASK_VFSGNJX_VF: u32 = 0xfc00707f;
pub const MATCH_VFSGNJX_VV: u32 = 0x28001057;
pub const MASK_VFSGNJX_VV: u32 = 0xfc00707f;
pub const MATCH_VFSLIDE1DOWN_VF: u32 = 0x3c005057;
pub const MASK_VFSLIDE1DOWN_VF: u32 = 0xfc00707f;
pub const MATCH_VFSLIDE1UP_VF: u32 = 0x38005057;
pub const MASK_VFSLIDE1UP_VF: u32 = 0xfc00707f;
pub const MATCH_VFSQRT_V: u32 = 0x4c001057;
pub const MASK_VFSQRT_V: u32 = 0xfc0ff07f;
pub const MATCH_VFSUB_VF: u32 = 0x8005057;
pub const MASK_VFSUB_VF: u32 = 0xfc00707f;
pub const MATCH_VFSUB_VV: u32 = 0x8001057;
pub const MASK_VFSUB_VV: u32 = 0xfc00707f;
pub const MATCH_VFWADD_VF: u32 = 0xc0005057;
pub const MASK_VFWADD_VF: u32 = 0xfc00707f;
pub const MATCH_VFWADD_VV: u32 = 0xc0001057;
pub const MASK_VFWADD_VV: u32 = 0xfc00707f;
pub const MATCH_VFWADD_WF: u32 = 0xd0005057;
pub const MASK_VFWADD_WF: u32 = 0xfc00707f;
pub const MATCH_VFWADD_WV: u32 = 0xd0001057;
pub const MASK_VFWADD_WV: u32 = 0xfc00707f;
pub const MATCH_VFWCVT_F_F_V: u32 = 0x48061057;
pub const MASK_VFWCVT_F_F_V: u32 = 0xfc0ff07f;
pub const MATCH_VFWCVT_F_X_V: u32 = 0x48059057;
pub const MASK_VFWCVT_F_X_V: u32 = 0xfc0ff07f;
pub const MATCH_VFWCVT_F_XU_V: u32 = 0x48051057;
pub const MASK_VFWCVT_F_XU_V: u32 = 0xfc0ff07f;
pub const MATCH_VFWCVT_RTZ_X_F_V: u32 = 0x48079057;
pub const MASK_VFWCVT_RTZ_X_F_V: u32 = 0xfc0ff07f;
pub const MATCH_VFWCVT_RTZ_XU_F_V: u32 = 0x48071057;
pub const MASK_VFWCVT_RTZ_XU_F_V: u32 = 0xfc0ff07f;
pub const MATCH_VFWCVT_X_F_V: u32 = 0x48049057;
pub const MASK_VFWCVT_X_F_V: u32 = 0xfc0ff07f;
pub const MATCH_VFWCVT_XU_F_V: u32 = 0x48041057;
pub const MASK_VFWCVT_XU_F_V: u32 = 0xfc0ff07f;
pub const MATCH_VFWMACC_VF: u32 = 0xf0005057;
pub const MASK_VFWMACC_VF: u32 = 0xfc00707f;
pub const MATCH_VFWMACC_VV: u32 = 0xf0001057;
pub const MASK_VFWMACC_VV: u32 = 0xfc00707f;
pub const MATCH_VFWMSAC_VF: u32 = 0xf8005057;
pub const MASK_VFWMSAC_VF: u32 = 0xfc00707f;
pub const MATCH_VFWMSAC_VV: u32 = 0xf8001057;
pub const MASK_VFWMSAC_VV: u32 = 0xfc00707f;
pub const MATCH_VFWMUL_VF: u32 = 0xe0005057;
pub const MASK_VFWMUL_VF: u32 = 0xfc00707f;
pub const MATCH_VFWMUL_VV: u32 = 0xe0001057;
pub const MASK_VFWMUL_VV: u32 = 0xfc00707f;
pub const MATCH_VFWNMACC_VF: u32 = 0xf4005057;
pub const MASK_VFWNMACC_VF: u32 = 0xfc00707f;
pub const MATCH_VFWNMACC_VV: u32 = 0xf4001057;
pub const MASK_VFWNMACC_VV: u32 = 0xfc00707f;
pub const MATCH_VFWNMSAC_VF: u32 = 0xfc005057;
pub const MASK_VFWNMSAC_VF: u32 = 0xfc00707f;
pub const MATCH_VFWNMSAC_VV: u32 = 0xfc001057;
pub const MASK_VFWNMSAC_VV: u32 = 0xfc00707f;
pub const MATCH_VFWREDOSUM_VS: u32 = 0xcc001057;
pub const MASK_VFWREDOSUM_VS: u32 = 0xfc00707f;
pub const MATCH_VFWREDSUM_VS: u32 = 0xc4001057;
pub const MASK_VFWREDSUM_VS: u32 = 0xfc00707f;
pub const MATCH_VFWREDUSUM_VS: u32 = 0xc4001057;
pub const MASK_VFWREDUSUM_VS: u32 = 0xfc00707f;
pub const MATCH_VFWSUB_VF: u32 = 0xc8005057;
pub const MASK_VFWSUB_VF: u32 = 0xfc00707f;
pub const MATCH_VFWSUB_VV: u32 = 0xc8001057;
pub const MASK_VFWSUB_VV: u32 = 0xfc00707f;
pub const MATCH_VFWSUB_WF: u32 = 0xd8005057;
pub const MASK_VFWSUB_WF: u32 = 0xfc00707f;
pub const MATCH_VFWSUB_WV: u32 = 0xd8001057;
pub const MASK_VFWSUB_WV: u32 = 0xfc00707f;
pub const MATCH_VGHSH_VV: u32 = 0xb2002077;
pub const MASK_VGHSH_VV: u32 = 0xfe00707f;
pub const MATCH_VGMUL_VV: u32 = 0xa208a077;
pub const MASK_VGMUL_VV: u32 = 0xfe0ff07f;
pub const MATCH_VID_V: u32 = 0x5008a057;
pub const MASK_VID_V: u32 = 0xfdfff07f;
pub const MATCH_VIOTA_M: u32 = 0x50082057;
pub const MASK_VIOTA_M: u32 = 0xfc0ff07f;
pub const MATCH_VL1R_V: u32 = 0x2800007;
pub const MASK_VL1R_V: u32 = 0xfff0707f;
pub const MATCH_VL1RE16_V: u32 = 0x2805007;
pub const MASK_VL1RE16_V: u32 = 0xfff0707f;
pub const MATCH_VL1RE32_V: u32 = 0x2806007;
pub const MASK_VL1RE32_V: u32 = 0xfff0707f;
pub const MATCH_VL1RE64_V: u32 = 0x2807007;
pub const MASK_VL1RE64_V: u32 = 0xfff0707f;
pub const MATCH_VL1RE8_V: u32 = 0x2800007;
pub const MASK_VL1RE8_V: u32 = 0xfff0707f;
pub const MATCH_VL2R_V: u32 = 0x22800007;
pub const MASK_VL2R_V: u32 = 0xfff0707f;
pub const MATCH_VL2RE16_V: u32 = 0x22805007;
pub const MASK_VL2RE16_V: u32 = 0xfff0707f;
pub const MATCH_VL2RE32_V: u32 = 0x22806007;
pub const MASK_VL2RE32_V: u32 = 0xfff0707f;
pub const MATCH_VL2RE64_V: u32 = 0x22807007;
pub const MASK_VL2RE64_V: u32 = 0xfff0707f;
pub const MATCH_VL2RE8_V: u32 = 0x22800007;
pub const MASK_VL2RE8_V: u32 = 0xfff0707f;
pub const MATCH_VL4R_V: u32 = 0x62800007;
pub const MASK_VL4R_V: u32 = 0xfff0707f;
pub const MATCH_VL4RE16_V: u32 = 0x62805007;
pub const MASK_VL4RE16_V: u32 = 0xfff0707f;
pub const MATCH_VL4RE32_V: u32 = 0x62806007;
pub const MASK_VL4RE32_V: u32 = 0xfff0707f;
pub const MATCH_VL4RE64_V: u32 = 0x62807007;
pub const MASK_VL4RE64_V: u32 = 0xfff0707f;
pub const MATCH_VL4RE8_V: u32 = 0x62800007;
pub const MASK_VL4RE8_V: u32 = 0xfff0707f;
pub const MATCH_VL8R_V: u32 = 0xe2800007;
pub const MASK_VL8R_V: u32 = 0xfff0707f;
pub const MATCH_VL8RE16_V: u32 = 0xe2805007;
pub const MASK_VL8RE16_V: u32 = 0xfff0707f;
pub const MATCH_VL8RE32_V: u32 = 0xe2806007;
pub const MASK_VL8RE32_V: u32 = 0xfff0707f;
pub const MATCH_VL8RE64_V: u32 = 0xe2807007;
pub const MASK_VL8RE64_V: u32 = 0xfff0707f;
pub const MATCH_VL8RE8_V: u32 = 0xe2800007;
pub const MASK_VL8RE8_V: u32 = 0xfff0707f;
pub const MATCH_VLE16_V: u32 = 0x5007;
pub const MASK_VLE16_V: u32 = 0x1df0707f;
pub const MATCH_VLE16FF_V: u32 = 0x1005007;
pub const MASK_VLE16FF_V: u32 = 0x1df0707f;
pub const MATCH_VLE1_V: u32 = 0x2b00007;
pub const MASK_VLE1_V: u32 = 0xfff0707f;
pub const MATCH_VLE32_V: u32 = 0x6007;
pub const MASK_VLE32_V: u32 = 0x1df0707f;
pub const MATCH_VLE32FF_V: u32 = 0x1006007;
pub const MASK_VLE32FF_V: u32 = 0x1df0707f;
pub const MATCH_VLE64_V: u32 = 0x7007;
pub const MASK_VLE64_V: u32 = 0x1df0707f;
pub const MATCH_VLE64FF_V: u32 = 0x1007007;
pub const MASK_VLE64FF_V: u32 = 0x1df0707f;
pub const MATCH_VLE8_V: u32 = 0x7;
pub const MASK_VLE8_V: u32 = 0x1df0707f;
pub const MATCH_VLE8FF_V: u32 = 0x1000007;
pub const MASK_VLE8FF_V: u32 = 0x1df0707f;
pub const MATCH_VLM_V: u32 = 0x2b00007;
pub const MASK_VLM_V: u32 = 0xfff0707f;
pub const MATCH_VLOXEI16_V: u32 = 0xc005007;
pub const MASK_VLOXEI16_V: u32 = 0x1c00707f;
pub const MATCH_VLOXEI32_V: u32 = 0xc006007;
pub const MASK_VLOXEI32_V: u32 = 0x1c00707f;
pub const MATCH_VLOXEI64_V: u32 = 0xc007007;
pub const MASK_VLOXEI64_V: u32 = 0x1c00707f;
pub const MATCH_VLOXEI8_V: u32 = 0xc000007;
pub const MASK_VLOXEI8_V: u32 = 0x1c00707f;
pub const MATCH_VLSE16_V: u32 = 0x8005007;
pub const MASK_VLSE16_V: u32 = 0x1c00707f;
pub const MATCH_VLSE32_V: u32 = 0x8006007;
pub const MASK_VLSE32_V: u32 = 0x1c00707f;
pub const MATCH_VLSE64_V: u32 = 0x8007007;
pub const MASK_VLSE64_V: u32 = 0x1c00707f;
pub const MATCH_VLSE8_V: u32 = 0x8000007;
pub const MASK_VLSE8_V: u32 = 0x1c00707f;
pub const MATCH_VLUXEI16_V: u32 = 0x4005007;
pub const MASK_VLUXEI16_V: u32 = 0x1c00707f;
pub const MATCH_VLUXEI32_V: u32 = 0x4006007;
pub const MASK_VLUXEI32_V: u32 = 0x1c00707f;
pub const MATCH_VLUXEI64_V: u32 = 0x4007007;
pub const MASK_VLUXEI64_V: u32 = 0x1c00707f;
pub const MATCH_VLUXEI8_V: u32 = 0x4000007;
pub const MASK_VLUXEI8_V: u32 = 0x1c00707f;
pub const MATCH_VMACC_VV: u32 = 0xb4002057;
pub const MASK_VMACC_VV: u32 = 0xfc00707f;
pub const MATCH_VMACC_VX: u32 = 0xb4006057;
pub const MASK_VMACC_VX: u32 = 0xfc00707f;
pub const MATCH_VMADC_VI: u32 = 0x46003057;
pub const MASK_VMADC_VI: u32 = 0xfe00707f;
pub const MATCH_VMADC_VIM: u32 = 0x44003057;
pub const MASK_VMADC_VIM: u32 = 0xfe00707f;
pub const MATCH_VMADC_VV: u32 = 0x46000057;
pub const MASK_VMADC_VV: u32 = 0xfe00707f;
pub const MATCH_VMADC_VVM: u32 = 0x44000057;
pub const MASK_VMADC_VVM: u32 = 0xfe00707f;
pub const MATCH_VMADC_VX: u32 = 0x46004057;
pub const MASK_VMADC_VX: u32 = 0xfe00707f;
pub const MATCH_VMADC_VXM: u32 = 0x44004057;
pub const MASK_VMADC_VXM: u32 = 0xfe00707f;
pub const MATCH_VMADD_VV: u32 = 0xa4002057;
pub const MASK_VMADD_VV: u32 = 0xfc00707f;
pub const MATCH_VMADD_VX: u32 = 0xa4006057;
pub const MASK_VMADD_VX: u32 = 0xfc00707f;
pub const MATCH_VMAND_MM: u32 = 0x66002057;
pub const MASK_VMAND_MM: u32 = 0xfe00707f;
pub const MATCH_VMANDN_MM: u32 = 0x62002057;
pub const MASK_VMANDN_MM: u32 = 0xfe00707f;
pub const MATCH_VMANDNOT_MM: u32 = 0x60002057;
pub const MASK_VMANDNOT_MM: u32 = 0xfc00707f;
pub const MATCH_VMAX_VV: u32 = 0x1c000057;
pub const MASK_VMAX_VV: u32 = 0xfc00707f;
pub const MATCH_VMAX_VX: u32 = 0x1c004057;
pub const MASK_VMAX_VX: u32 = 0xfc00707f;
pub const MATCH_VMAXU_VV: u32 = 0x18000057;
pub const MASK_VMAXU_VV: u32 = 0xfc00707f;
pub const MATCH_VMAXU_VX: u32 = 0x18004057;
pub const MASK_VMAXU_VX: u32 = 0xfc00707f;
pub const MATCH_VMERGE_VIM: u32 = 0x5c003057;
pub const MASK_VMERGE_VIM: u32 = 0xfe00707f;
pub const MATCH_VMERGE_VVM: u32 = 0x5c000057;
pub const MASK_VMERGE_VVM: u32 = 0xfe00707f;
pub const MATCH_VMERGE_VXM: u32 = 0x5c004057;
pub const MASK_VMERGE_VXM: u32 = 0xfe00707f;
pub const MATCH_VMFEQ_VF: u32 = 0x60005057;
pub const MASK_VMFEQ_VF: u32 = 0xfc00707f;
pub const MATCH_VMFEQ_VV: u32 = 0x60001057;
pub const MASK_VMFEQ_VV: u32 = 0xfc00707f;
pub const MATCH_VMFGE_VF: u32 = 0x7c005057;
pub const MASK_VMFGE_VF: u32 = 0xfc00707f;
pub const MATCH_VMFGT_VF: u32 = 0x74005057;
pub const MASK_VMFGT_VF: u32 = 0xfc00707f;
pub const MATCH_VMFLE_VF: u32 = 0x64005057;
pub const MASK_VMFLE_VF: u32 = 0xfc00707f;
pub const MATCH_VMFLE_VV: u32 = 0x64001057;
pub const MASK_VMFLE_VV: u32 = 0xfc00707f;
pub const MATCH_VMFLT_VF: u32 = 0x6c005057;
pub const MASK_VMFLT_VF: u32 = 0xfc00707f;
pub const MATCH_VMFLT_VV: u32 = 0x6c001057;
pub const MASK_VMFLT_VV: u32 = 0xfc00707f;
pub const MATCH_VMFNE_VF: u32 = 0x70005057;
pub const MASK_VMFNE_VF: u32 = 0xfc00707f;
pub const MATCH_VMFNE_VV: u32 = 0x70001057;
pub const MASK_VMFNE_VV: u32 = 0xfc00707f;
pub const MATCH_VMIN_VV: u32 = 0x14000057;
pub const MASK_VMIN_VV: u32 = 0xfc00707f;
pub const MATCH_VMIN_VX: u32 = 0x14004057;
pub const MASK_VMIN_VX: u32 = 0xfc00707f;
pub const MATCH_VMINU_VV: u32 = 0x10000057;
pub const MASK_VMINU_VV: u32 = 0xfc00707f;
pub const MATCH_VMINU_VX: u32 = 0x10004057;
pub const MASK_VMINU_VX: u32 = 0xfc00707f;
pub const MATCH_VMNAND_MM: u32 = 0x76002057;
pub const MASK_VMNAND_MM: u32 = 0xfe00707f;
pub const MATCH_VMNOR_MM: u32 = 0x7a002057;
pub const MASK_VMNOR_MM: u32 = 0xfe00707f;
pub const MATCH_VMOR_MM: u32 = 0x6a002057;
pub const MASK_VMOR_MM: u32 = 0xfe00707f;
pub const MATCH_VMORN_MM: u32 = 0x72002057;
pub const MASK_VMORN_MM: u32 = 0xfe00707f;
pub const MATCH_VMORNOT_MM: u32 = 0x70002057;
pub const MASK_VMORNOT_MM: u32 = 0xfc00707f;
pub const MATCH_VMSBC_VV: u32 = 0x4e000057;
pub const MASK_VMSBC_VV: u32 = 0xfe00707f;
pub const MATCH_VMSBC_VVM: u32 = 0x4c000057;
pub const MASK_VMSBC_VVM: u32 = 0xfe00707f;
pub const MATCH_VMSBC_VX: u32 = 0x4e004057;
pub const MASK_VMSBC_VX: u32 = 0xfe00707f;
pub const MATCH_VMSBC_VXM: u32 = 0x4c004057;
pub const MASK_VMSBC_VXM: u32 = 0xfe00707f;
pub const MATCH_VMSBF_M: u32 = 0x5000a057;
pub const MASK_VMSBF_M: u32 = 0xfc0ff07f;
pub const MATCH_VMSEQ_VI: u32 = 0x60003057;
pub const MASK_VMSEQ_VI: u32 = 0xfc00707f;
pub const MATCH_VMSEQ_VV: u32 = 0x60000057;
pub const MASK_VMSEQ_VV: u32 = 0xfc00707f;
pub const MATCH_VMSEQ_VX: u32 = 0x60004057;
pub const MASK_VMSEQ_VX: u32 = 0xfc00707f;
pub const MATCH_VMSGT_VI: u32 = 0x7c003057;
pub const MASK_VMSGT_VI: u32 = 0xfc00707f;
pub const MATCH_VMSGT_VX: u32 = 0x7c004057;
pub const MASK_VMSGT_VX: u32 = 0xfc00707f;
pub const MATCH_VMSGTU_VI: u32 = 0x78003057;
pub const MASK_VMSGTU_VI: u32 = 0xfc00707f;
pub const MATCH_VMSGTU_VX: u32 = 0x78004057;
pub const MASK_VMSGTU_VX: u32 = 0xfc00707f;
pub const MATCH_VMSIF_M: u32 = 0x5001a057;
pub const MASK_VMSIF_M: u32 = 0xfc0ff07f;
pub const MATCH_VMSLE_VI: u32 = 0x74003057;
pub const MASK_VMSLE_VI: u32 = 0xfc00707f;
pub const MATCH_VMSLE_VV: u32 = 0x74000057;
pub const MASK_VMSLE_VV: u32 = 0xfc00707f;
pub const MATCH_VMSLE_VX: u32 = 0x74004057;
pub const MASK_VMSLE_VX: u32 = 0xfc00707f;
pub const MATCH_VMSLEU_VI: u32 = 0x70003057;
pub const MASK_VMSLEU_VI: u32 = 0xfc00707f;
pub const MATCH_VMSLEU_VV: u32 = 0x70000057;
pub const MASK_VMSLEU_VV: u32 = 0xfc00707f;
pub const MATCH_VMSLEU_VX: u32 = 0x70004057;
pub const MASK_VMSLEU_VX: u32 = 0xfc00707f;
pub const MATCH_VMSLT_VV: u32 = 0x6c000057;
pub const MASK_VMSLT_VV: u32 = 0xfc00707f;
pub const MATCH_VMSLT_VX: u32 = 0x6c004057;
pub const MASK_VMSLT_VX: u32 = 0xfc00707f;
pub const MATCH_VMSLTU_VV: u32 = 0x68000057;
pub const MASK_VMSLTU_VV: u32 = 0xfc00707f;
pub const MATCH_VMSLTU_VX: u32 = 0x68004057;
pub const MASK_VMSLTU_VX: u32 = 0xfc00707f;
pub const MATCH_VMSNE_VI: u32 = 0x64003057;
pub const MASK_VMSNE_VI: u32 = 0xfc00707f;
pub const MATCH_VMSNE_VV: u32 = 0x64000057;
pub const MASK_VMSNE_VV: u32 = 0xfc00707f;
pub const MATCH_VMSNE_VX: u32 = 0x64004057;
pub const MASK_VMSNE_VX: u32 = 0xfc00707f;
pub const MATCH_VMSOF_M: u32 = 0x50012057;
pub const MASK_VMSOF_M: u32 = 0xfc0ff07f;
pub const MATCH_VMUL_VV: u32 = 0x94002057;
pub const MASK_VMUL_VV: u32 = 0xfc00707f;
pub const MATCH_VMUL_VX: u32 = 0x94006057;
pub const MASK_VMUL_VX: u32 = 0xfc00707f;
pub const MATCH_VMULH_VV: u32 = 0x9c002057;
pub const MASK_VMULH_VV: u32 = 0xfc00707f;
pub const MATCH_VMULH_VX: u32 = 0x9c006057;
pub const MASK_VMULH_VX: u32 = 0xfc00707f;
pub const MATCH_VMULHSU_VV: u32 = 0x98002057;
pub const MASK_VMULHSU_VV: u32 = 0xfc00707f;
pub const MATCH_VMULHSU_VX: u32 = 0x98006057;
pub const MASK_VMULHSU_VX: u32 = 0xfc00707f;
pub const MATCH_VMULHU_VV: u32 = 0x90002057;
pub const MASK_VMULHU_VV: u32 = 0xfc00707f;
pub const MATCH_VMULHU_VX: u32 = 0x90006057;
pub const MASK_VMULHU_VX: u32 = 0xfc00707f;
pub const MATCH_VMV1R_V: u32 = 0x9e003057;
pub const MASK_VMV1R_V: u32 = 0xfe0ff07f;
pub const MATCH_VMV2R_V: u32 = 0x9e00b057;
pub const MASK_VMV2R_V: u32 = 0xfe0ff07f;
pub const MATCH_VMV4R_V: u32 = 0x9e01b057;
pub const MASK_VMV4R_V: u32 = 0xfe0ff07f;
pub const MATCH_VMV8R_V: u32 = 0x9e03b057;
pub const MASK_VMV8R_V: u32 = 0xfe0ff07f;
pub const MATCH_VMV_S_X: u32 = 0x42006057;
pub const MASK_VMV_S_X: u32 = 0xfff0707f;
pub const MATCH_VMV_V_I: u32 = 0x5e003057;
pub const MASK_VMV_V_I: u32 = 0xfff0707f;
pub const MATCH_VMV_V_V: u32 = 0x5e000057;
pub const MASK_VMV_V_V: u32 = 0xfff0707f;
pub const MATCH_VMV_V_X: u32 = 0x5e004057;
pub const MASK_VMV_V_X: u32 = 0xfff0707f;
pub const MATCH_VMV_X_S: u32 = 0x42002057;
pub const MASK_VMV_X_S: u32 = 0xfe0ff07f;
pub const MATCH_VMXNOR_MM: u32 = 0x7e002057;
pub const MASK_VMXNOR_MM: u32 = 0xfe00707f;
pub const MATCH_VMXOR_MM: u32 = 0x6e002057;
pub const MASK_VMXOR_MM: u32 = 0xfe00707f;
pub const MATCH_VNCLIP_WI: u32 = 0xbc003057;
pub const MASK_VNCLIP_WI: u32 = 0xfc00707f;
pub const MATCH_VNCLIP_WV: u32 = 0xbc000057;
pub const MASK_VNCLIP_WV: u32 = 0xfc00707f;
pub const MATCH_VNCLIP_WX: u32 = 0xbc004057;
pub const MASK_VNCLIP_WX: u32 = 0xfc00707f;
pub const MATCH_VNCLIPU_WI: u32 = 0xb8003057;
pub const MASK_VNCLIPU_WI: u32 = 0xfc00707f;
pub const MATCH_VNCLIPU_WV: u32 = 0xb8000057;
pub const MASK_VNCLIPU_WV: u32 = 0xfc00707f;
pub const MATCH_VNCLIPU_WX: u32 = 0xb8004057;
pub const MASK_VNCLIPU_WX: u32 = 0xfc00707f;
pub const MATCH_VNMSAC_VV: u32 = 0xbc002057;
pub const MASK_VNMSAC_VV: u32 = 0xfc00707f;
pub const MATCH_VNMSAC_VX: u32 = 0xbc006057;
pub const MASK_VNMSAC_VX: u32 = 0xfc00707f;
pub const MATCH_VNMSUB_VV: u32 = 0xac002057;
pub const MASK_VNMSUB_VV: u32 = 0xfc00707f;
pub const MATCH_VNMSUB_VX: u32 = 0xac006057;
pub const MASK_VNMSUB_VX: u32 = 0xfc00707f;
pub const MATCH_VNSRA_WI: u32 = 0xb4003057;
pub const MASK_VNSRA_WI: u32 = 0xfc00707f;
pub const MATCH_VNSRA_WV: u32 = 0xb4000057;
pub const MASK_VNSRA_WV: u32 = 0xfc00707f;
pub const MATCH_VNSRA_WX: u32 = 0xb4004057;
pub const MASK_VNSRA_WX: u32 = 0xfc00707f;
pub const MATCH_VNSRL_WI: u32 = 0xb0003057;
pub const MASK_VNSRL_WI: u32 = 0xfc00707f;
pub const MATCH_VNSRL_WV: u32 = 0xb0000057;
pub const MASK_VNSRL_WV: u32 = 0xfc00707f;
pub const MATCH_VNSRL_WX: u32 = 0xb0004057;
pub const MASK_VNSRL_WX: u32 = 0xfc00707f;
pub const MATCH_VOR_VI: u32 = 0x28003057;
pub const MASK_VOR_VI: u32 = 0xfc00707f;
pub const MATCH_VOR_VV: u32 = 0x28000057;
pub const MASK_VOR_VV: u32 = 0xfc00707f;
pub const MATCH_VOR_VX: u32 = 0x28004057;
pub const MASK_VOR_VX: u32 = 0xfc00707f;
pub const MATCH_VPOPC_M: u32 = 0x40082057;
pub const MASK_VPOPC_M: u32 = 0xfc0ff07f;
pub const MATCH_VREDAND_VS: u32 = 0x4002057;
pub const MASK_VREDAND_VS: u32 = 0xfc00707f;
pub const MATCH_VREDMAX_VS: u32 = 0x1c002057;
pub const MASK_VREDMAX_VS: u32 = 0xfc00707f;
pub const MATCH_VREDMAXU_VS: u32 = 0x18002057;
pub const MASK_VREDMAXU_VS: u32 = 0xfc00707f;
pub const MATCH_VREDMIN_VS: u32 = 0x14002057;
pub const MASK_VREDMIN_VS: u32 = 0xfc00707f;
pub const MATCH_VREDMINU_VS: u32 = 0x10002057;
pub const MASK_VREDMINU_VS: u32 = 0xfc00707f;
pub const MATCH_VREDOR_VS: u32 = 0x8002057;
pub const MASK_VREDOR_VS: u32 = 0xfc00707f;
pub const MATCH_VREDSUM_VS: u32 = 0x2057;
pub const MASK_VREDSUM_VS: u32 = 0xfc00707f;
pub const MATCH_VREDXOR_VS: u32 = 0xc002057;
pub const MASK_VREDXOR_VS: u32 = 0xfc00707f;
pub const MATCH_VREM_VV: u32 = 0x8c002057;
pub const MASK_VREM_VV: u32 = 0xfc00707f;
pub const MATCH_VREM_VX: u32 = 0x8c006057;
pub const MASK_VREM_VX: u32 = 0xfc00707f;
pub const MATCH_VREMU_VV: u32 = 0x88002057;
pub const MASK_VREMU_VV: u32 = 0xfc00707f;
pub const MATCH_VREMU_VX: u32 = 0x88006057;
pub const MASK_VREMU_VX: u32 = 0xfc00707f;
pub const MATCH_VREV8_V: u32 = 0x4804a057;
pub const MASK_VREV8_V: u32 = 0xfc0ff07f;
pub const MATCH_VRGATHER_VI: u32 = 0x30003057;
pub const MASK_VRGATHER_VI: u32 = 0xfc00707f;
pub const MATCH_VRGATHER_VV: u32 = 0x30000057;
pub const MASK_VRGATHER_VV: u32 = 0xfc00707f;
pub const MATCH_VRGATHER_VX: u32 = 0x30004057;
pub const MASK_VRGATHER_VX: u32 = 0xfc00707f;
pub const MATCH_VRGATHEREI16_VV: u32 = 0x38000057;
pub const MASK_VRGATHEREI16_VV: u32 = 0xfc00707f;
pub const MATCH_VROL_VV: u32 = 0x54000057;
pub const MASK_VROL_VV: u32 = 0xfc00707f;
pub const MATCH_VROL_VX: u32 = 0x54004057;
pub const MASK_VROL_VX: u32 = 0xfc00707f;
pub const MATCH_VROR_VI: u32 = 0x50003057;
pub const MASK_VROR_VI: u32 = 0xf800707f;
pub const MATCH_VROR_VV: u32 = 0x50000057;
pub const MASK_VROR_VV: u32 = 0xfc00707f;
pub const MATCH_VROR_VX: u32 = 0x50004057;
pub const MASK_VROR_VX: u32 = 0xfc00707f;
pub const MATCH_VRSUB_VI: u32 = 0xc003057;
pub const MASK_VRSUB_VI: u32 = 0xfc00707f;
pub const MATCH_VRSUB_VX: u32 = 0xc004057;
pub const MASK_VRSUB_VX: u32 = 0xfc00707f;
pub const MATCH_VS1R_V: u32 = 0x2800027;
pub const MASK_VS1R_V: u32 = 0xfff0707f;
pub const MATCH_VS2R_V: u32 = 0x22800027;
pub const MASK_VS2R_V: u32 = 0xfff0707f;
pub const MATCH_VS4R_V: u32 = 0x62800027;
pub const MASK_VS4R_V: u32 = 0xfff0707f;
pub const MATCH_VS8R_V: u32 = 0xe2800027;
pub const MASK_VS8R_V: u32 = 0xfff0707f;
pub const MATCH_VSADD_VI: u32 = 0x84003057;
pub const MASK_VSADD_VI: u32 = 0xfc00707f;
pub const MATCH_VSADD_VV: u32 = 0x84000057;
pub const MASK_VSADD_VV: u32 = 0xfc00707f;
pub const MATCH_VSADD_VX: u32 = 0x84004057;
pub const MASK_VSADD_VX: u32 = 0xfc00707f;
pub const MATCH_VSADDU_VI: u32 = 0x80003057;
pub const MASK_VSADDU_VI: u32 = 0xfc00707f;
pub const MATCH_VSADDU_VV: u32 = 0x80000057;
pub const MASK_VSADDU_VV: u32 = 0xfc00707f;
pub const MATCH_VSADDU_VX: u32 = 0x80004057;
pub const MASK_VSADDU_VX: u32 = 0xfc00707f;
pub const MATCH_VSBC_VVM: u32 = 0x48000057;
pub const MASK_VSBC_VVM: u32 = 0xfe00707f;
pub const MATCH_VSBC_VXM: u32 = 0x48004057;
pub const MASK_VSBC_VXM: u32 = 0xfe00707f;
pub const MATCH_VSE16_V: u32 = 0x5027;
pub const MASK_VSE16_V: u32 = 0x1df0707f;
pub const MATCH_VSE1_V: u32 = 0x2b00027;
pub const MASK_VSE1_V: u32 = 0xfff0707f;
pub const MATCH_VSE32_V: u32 = 0x6027;
pub const MASK_VSE32_V: u32 = 0x1df0707f;
pub const MATCH_VSE64_V: u32 = 0x7027;
pub const MASK_VSE64_V: u32 = 0x1df0707f;
pub const MATCH_VSE8_V: u32 = 0x27;
pub const MASK_VSE8_V: u32 = 0x1df0707f;
pub const MATCH_VSETIVLI: u32 = 0xc0007057;
pub const MASK_VSETIVLI: u32 = 0xc000707f;
pub const MATCH_VSETVL: u32 = 0x80007057;
pub const MASK_VSETVL: u32 = 0xfe00707f;
pub const MATCH_VSETVLI: u32 = 0x7057;
pub const MASK_VSETVLI: u32 = 0x8000707f;
pub const MATCH_VSEXT_VF2: u32 = 0x4803a057;
pub const MASK_VSEXT_VF2: u32 = 0xfc0ff07f;
pub const MATCH_VSEXT_VF4: u32 = 0x4802a057;
pub const MASK_VSEXT_VF4: u32 = 0xfc0ff07f;
pub const MATCH_VSEXT_VF8: u32 = 0x4801a057;
pub const MASK_VSEXT_VF8: u32 = 0xfc0ff07f;
pub const MATCH_VSHA2CH_VV: u32 = 0xba002077;
pub const MASK_VSHA2CH_VV: u32 = 0xfe00707f;
pub const MATCH_VSHA2CL_VV: u32 = 0xbe002077;
pub const MASK_VSHA2CL_VV: u32 = 0xfe00707f;
pub const MATCH_VSHA2MS_VV: u32 = 0xb6002077;
pub const MASK_VSHA2MS_VV: u32 = 0xfe00707f;
pub const MATCH_VSLIDE1DOWN_VX: u32 = 0x3c006057;
pub const MASK_VSLIDE1DOWN_VX: u32 = 0xfc00707f;
pub const MATCH_VSLIDE1UP_VX: u32 = 0x38006057;
pub const MASK_VSLIDE1UP_VX: u32 = 0xfc00707f;
pub const MATCH_VSLIDEDOWN_VI: u32 = 0x3c003057;
pub const MASK_VSLIDEDOWN_VI: u32 = 0xfc00707f;
pub const MATCH_VSLIDEDOWN_VX: u32 = 0x3c004057;
pub const MASK_VSLIDEDOWN_VX: u32 = 0xfc00707f;
pub const MATCH_VSLIDEUP_VI: u32 = 0x38003057;
pub const MASK_VSLIDEUP_VI: u32 = 0xfc00707f;
pub const MATCH_VSLIDEUP_VX: u32 = 0x38004057;
pub const MASK_VSLIDEUP_VX: u32 = 0xfc00707f;
pub const MATCH_VSLL_VI: u32 = 0x94003057;
pub const MASK_VSLL_VI: u32 = 0xfc00707f;
pub const MATCH_VSLL_VV: u32 = 0x94000057;
pub const MASK_VSLL_VV: u32 = 0xfc00707f;
pub const MATCH_VSLL_VX: u32 = 0x94004057;
pub const MASK_VSLL_VX: u32 = 0xfc00707f;
pub const MATCH_VSM3C_VI: u32 = 0xae002077;
pub const MASK_VSM3C_VI: u32 = 0xfe00707f;
pub const MATCH_VSM3ME_VV: u32 = 0x82002077;
pub const MASK_VSM3ME_VV: u32 = 0xfe00707f;
pub const MATCH_VSM4K_VI: u32 = 0x86002077;
pub const MASK_VSM4K_VI: u32 = 0xfe00707f;
pub const MATCH_VSM4R_VS: u32 = 0xa6082077;
pub const MASK_VSM4R_VS: u32 = 0xfe0ff07f;
pub const MATCH_VSM4R_VV: u32 = 0xa2082077;
pub const MASK_VSM4R_VV: u32 = 0xfe0ff07f;
pub const MATCH_VSM_V: u32 = 0x2b00027;
pub const MASK_VSM_V: u32 = 0xfff0707f;
pub const MATCH_VSMUL_VV: u32 = 0x9c000057;
pub const MASK_VSMUL_VV: u32 = 0xfc00707f;
pub const MATCH_VSMUL_VX: u32 = 0x9c004057;
pub const MASK_VSMUL_VX: u32 = 0xfc00707f;
pub const MATCH_VSOXEI16_V: u32 = 0xc005027;
pub const MASK_VSOXEI16_V: u32 = 0x1c00707f;
pub const MATCH_VSOXEI32_V: u32 = 0xc006027;
pub const MASK_VSOXEI32_V: u32 = 0x1c00707f;
pub const MATCH_VSOXEI64_V: u32 = 0xc007027;
pub const MASK_VSOXEI64_V: u32 = 0x1c00707f;
pub const MATCH_VSOXEI8_V: u32 = 0xc000027;
pub const MASK_VSOXEI8_V: u32 = 0x1c00707f;
pub const MATCH_VSRA_VI: u32 = 0xa4003057;
pub const MASK_VSRA_VI: u32 = 0xfc00707f;
pub const MATCH_VSRA_VV: u32 = 0xa4000057;
pub const MASK_VSRA_VV: u32 = 0xfc00707f;
pub const MATCH_VSRA_VX: u32 = 0xa4004057;
pub const MASK_VSRA_VX: u32 = 0xfc00707f;
pub const MATCH_VSRL_VI: u32 = 0xa0003057;
pub const MASK_VSRL_VI: u32 = 0xfc00707f;
pub const MATCH_VSRL_VV: u32 = 0xa0000057;
pub const MASK_VSRL_VV: u32 = 0xfc00707f;
pub const MATCH_VSRL_VX: u32 = 0xa0004057;
pub const MASK_VSRL_VX: u32 = 0xfc00707f;
pub const MATCH_VSSE16_V: u32 = 0x8005027;
pub const MASK_VSSE16_V: u32 = 0x1c00707f;
pub const MATCH_VSSE32_V: u32 = 0x8006027;
pub const MASK_VSSE32_V: u32 = 0x1c00707f;
pub const MATCH_VSSE64_V: u32 = 0x8007027;
pub const MASK_VSSE64_V: u32 = 0x1c00707f;
pub const MATCH_VSSE8_V: u32 = 0x8000027;
pub const MASK_VSSE8_V: u32 = 0x1c00707f;
pub const MATCH_VSSRA_VI: u32 = 0xac003057;
pub const MASK_VSSRA_VI: u32 = 0xfc00707f;
pub const MATCH_VSSRA_VV: u32 = 0xac000057;
pub const MASK_VSSRA_VV: u32 = 0xfc00707f;
pub const MATCH_VSSRA_VX: u32 = 0xac004057;
pub const MASK_VSSRA_VX: u32 = 0xfc00707f;
pub const MATCH_VSSRL_VI: u32 = 0xa8003057;
pub const MASK_VSSRL_VI: u32 = 0xfc00707f;
pub const MATCH_VSSRL_VV: u32 = 0xa8000057;
pub const MASK_VSSRL_VV: u32 = 0xfc00707f;
pub const MATCH_VSSRL_VX: u32 = 0xa8004057;
pub const MASK_VSSRL_VX: u32 = 0xfc00707f;
pub const MATCH_VSSUB_VV: u32 = 0x8c000057;
pub const MASK_VSSUB_VV: u32 = 0xfc00707f;
pub const MATCH_VSSUB_VX: u32 = 0x8c004057;
pub const MASK_VSSUB_VX: u32 = 0xfc00707f;
pub const MATCH_VSSUBU_VV: u32 = 0x88000057;
pub const MASK_VSSUBU_VV: u32 = 0xfc00707f;
pub const MATCH_VSSUBU_VX: u32 = 0x88004057;
pub const MASK_VSSUBU_VX: u32 = 0xfc00707f;
pub const MATCH_VSUB_VV: u32 = 0x8000057;
pub const MASK_VSUB_VV: u32 = 0xfc00707f;
pub const MATCH_VSUB_VX: u32 = 0x8004057;
pub const MASK_VSUB_VX: u32 = 0xfc00707f;
pub const MATCH_VSUXEI16_V: u32 = 0x4005027;
pub const MASK_VSUXEI16_V: u32 = 0x1c00707f;
pub const MATCH_VSUXEI32_V: u32 = 0x4006027;
pub const MASK_VSUXEI32_V: u32 = 0x1c00707f;
pub const MATCH_VSUXEI64_V: u32 = 0x4007027;
pub const MASK_VSUXEI64_V: u32 = 0x1c00707f;
pub const MATCH_VSUXEI8_V: u32 = 0x4000027;
pub const MASK_VSUXEI8_V: u32 = 0x1c00707f;
pub const MATCH_VWADD_VV: u32 = 0xc4002057;
pub const MASK_VWADD_VV: u32 = 0xfc00707f;
pub const MATCH_VWADD_VX: u32 = 0xc4006057;
pub const MASK_VWADD_VX: u32 = 0xfc00707f;
pub const MATCH_VWADD_WV: u32 = 0xd4002057;
pub const MASK_VWADD_WV: u32 = 0xfc00707f;
pub const MATCH_VWADD_WX: u32 = 0xd4006057;
pub const MASK_VWADD_WX: u32 = 0xfc00707f;
pub const MATCH_VWADDU_VV: u32 = 0xc0002057;
pub const MASK_VWADDU_VV: u32 = 0xfc00707f;
pub const MATCH_VWADDU_VX: u32 = 0xc0006057;
pub const MASK_VWADDU_VX: u32 = 0xfc00707f;
pub const MATCH_VWADDU_WV: u32 = 0xd0002057;
pub const MASK_VWADDU_WV: u32 = 0xfc00707f;
pub const MATCH_VWADDU_WX: u32 = 0xd0006057;
pub const MASK_VWADDU_WX: u32 = 0xfc00707f;
pub const MATCH_VWMACC_VV: u32 = 0xf4002057;
pub const MASK_VWMACC_VV: u32 = 0xfc00707f;
pub const MATCH_VWMACC_VX: u32 = 0xf4006057;
pub const MASK_VWMACC_VX: u32 = 0xfc00707f;
pub const MATCH_VWMACCSU_VV: u32 = 0xfc002057;
pub const MASK_VWMACCSU_VV: u32 = 0xfc00707f;
pub const MATCH_VWMACCSU_VX: u32 = 0xfc006057;
pub const MASK_VWMACCSU_VX: u32 = 0xfc00707f;
pub const MATCH_VWMACCU_VV: u32 = 0xf0002057;
pub const MASK_VWMACCU_VV: u32 = 0xfc00707f;
pub const MATCH_VWMACCU_VX: u32 = 0xf0006057;
pub const MASK_VWMACCU_VX: u32 = 0xfc00707f;
pub const MATCH_VWMACCUS_VX: u32 = 0xf8006057;
pub const MASK_VWMACCUS_VX: u32 = 0xfc00707f;
pub const MATCH_VWMUL_VV: u32 = 0xec002057;
pub const MASK_VWMUL_VV: u32 = 0xfc00707f;
pub const MATCH_VWMUL_VX: u32 = 0xec006057;
pub const MASK_VWMUL_VX: u32 = 0xfc00707f;
pub const MATCH_VWMULSU_VV: u32 = 0xe8002057;
pub const MASK_VWMULSU_VV: u32 = 0xfc00707f;
pub const MATCH_VWMULSU_VX: u32 = 0xe8006057;
pub const MASK_VWMULSU_VX: u32 = 0xfc00707f;
pub const MATCH_VWMULU_VV: u32 = 0xe0002057;
pub const MASK_VWMULU_VV: u32 = 0xfc00707f;
pub const MATCH_VWMULU_VX: u32 = 0xe0006057;
pub const MASK_VWMULU_VX: u32 = 0xfc00707f;
pub const MATCH_VWREDSUM_VS: u32 = 0xc4000057;
pub const MASK_VWREDSUM_VS: u32 = 0xfc00707f;
pub const MATCH_VWREDSUMU_VS: u32 = 0xc0000057;
pub const MASK_VWREDSUMU_VS: u32 = 0xfc00707f;
pub const MATCH_VWSLL_VI: u32 = 0xd4003057;
pub const MASK_VWSLL_VI: u32 = 0xfc00707f;
pub const MATCH_VWSLL_VV: u32 = 0xd4000057;
pub const MASK_VWSLL_VV: u32 = 0xfc00707f;
pub const MATCH_VWSLL_VX: u32 = 0xd4004057;
pub const MASK_VWSLL_VX: u32 = 0xfc00707f;
pub const MATCH_VWSUB_VV: u32 = 0xcc002057;
pub const MASK_VWSUB_VV: u32 = 0xfc00707f;
pub const MATCH_VWSUB_VX: u32 = 0xcc006057;
pub const MASK_VWSUB_VX: u32 = 0xfc00707f;
pub const MATCH_VWSUB_WV: u32 = 0xdc002057;
pub const MASK_VWSUB_WV: u32 = 0xfc00707f;
pub const MATCH_VWSUB_WX: u32 = 0xdc006057;
pub const MASK_VWSUB_WX: u32 = 0xfc00707f;
pub const MATCH_VWSUBU_VV: u32 = 0xc8002057;
pub const MASK_VWSUBU_VV: u32 = 0xfc00707f;
pub const MATCH_VWSUBU_VX: u32 = 0xc8006057;
pub const MASK_VWSUBU_VX: u32 = 0xfc00707f;
pub const MATCH_VWSUBU_WV: u32 = 0xd8002057;
pub const MASK_VWSUBU_WV: u32 = 0xfc00707f;
pub const MATCH_VWSUBU_WX: u32 = 0xd8006057;
pub const MASK_VWSUBU_WX: u32 = 0xfc00707f;
pub const MATCH_VXOR_VI: u32 = 0x2c003057;
pub const MASK_VXOR_VI: u32 = 0xfc00707f;
pub const MATCH_VXOR_VV: u32 = 0x2c000057;
pub const MASK_VXOR_VV: u32 = 0xfc00707f;
pub const MATCH_VXOR_VX: u32 = 0x2c004057;
pub const MASK_VXOR_VX: u32 = 0xfc00707f;
pub const MATCH_VZEXT_VF2: u32 = 0x48032057;
pub const MASK_VZEXT_VF2: u32 = 0xfc0ff07f;
pub const MATCH_VZEXT_VF4: u32 = 0x48022057;
pub const MASK_VZEXT_VF4: u32 = 0xfc0ff07f;
pub const MATCH_VZEXT_VF8: u32 = 0x48012057;
pub const MASK_VZEXT_VF8: u32 = 0xfc0ff07f;
pub const MATCH_WFI: u32 = 0x10500073;
pub const MASK_WFI: u32 = 0xffffffff;
pub const MATCH_WRS_NTO: u32 = 0xd00073;
pub const MASK_WRS_NTO: u32 = 0xffffffff;
pub const MATCH_WRS_STO: u32 = 0x1d00073;
pub const MASK_WRS_STO: u32 = 0xffffffff;
pub const MATCH_XNOR: u32 = 0x40004033;
pub const MASK_XNOR: u32 = 0xfe00707f;
pub const MATCH_XOR: u32 = 0x4033;
pub const MASK_XOR: u32 = 0xfe00707f;
pub const MATCH_XORI: u32 = 0x4013;
pub const MASK_XORI: u32 = 0x707f;
pub const MATCH_XPERM4: u32 = 0x28002033;
pub const MASK_XPERM4: u32 = 0xfe00707f;
pub const MATCH_XPERM8: u32 = 0x28004033;
pub const MASK_XPERM8: u32 = 0xfe00707f;
pub const MATCH_ZEXT_B: u32 = 0x7013;
pub const MASK_ZEXT_B: u32 = 0xfff0707f;
pub const MATCH_ZEXT_H: u32 = 0x800403b;
pub const MASK_ZEXT_H: u32 = 0xfff0707f;
pub const MATCH_ZEXT_H_RV32: u32 = 0x8004033;
pub const MASK_ZEXT_H_RV32: u32 = 0xfff0707f;
pub const MATCH_ZEXT_W: u32 = 0x800003b;
pub const MASK_ZEXT_W: u32 = 0xfff0707f;
pub const MATCH_ZIP: u32 = 0x8f01013;
pub const MASK_ZIP: u32 = 0xfff0707f;
pub const CSR_FFLAGS: u16 = 0x1;
pub const CSR_FRM: u16 = 0x2;
pub const CSR_FCSR: u16 = 0x3;
pub const CSR_VSTART: u16 = 0x8;
pub const CSR_VXSAT: u16 = 0x9;
pub const CSR_VXRM: u16 = 0xa;
pub const CSR_VCSR: u16 = 0xf;
pub const CSR_SSP: u16 = 0x11;
pub const CSR_SEED: u16 = 0x15;
pub const CSR_JVT: u16 = 0x17;
pub const CSR_CYCLE: u16 = 0xc00;
pub const CSR_TIME: u16 = 0xc01;
pub const CSR_INSTRET: u16 = 0xc02;
pub const CSR_HPMCOUNTER3: u16 = 0xc03;
pub const CSR_HPMCOUNTER4: u16 = 0xc04;
pub const CSR_HPMCOUNTER5: u16 = 0xc05;
pub const CSR_HPMCOUNTER6: u16 = 0xc06;
pub const CSR_HPMCOUNTER7: u16 = 0xc07;
pub const CSR_HPMCOUNTER8: u16 = 0xc08;
pub const CSR_HPMCOUNTER9: u16 = 0xc09;
pub const CSR_HPMCOUNTER10: u16 = 0xc0a;
pub const CSR_HPMCOUNTER11: u16 = 0xc0b;
pub const CSR_HPMCOUNTER12: u16 = 0xc0c;
pub const CSR_HPMCOUNTER13: u16 = 0xc0d;
pub const CSR_HPMCOUNTER14: u16 = 0xc0e;
pub const CSR_HPMCOUNTER15: u16 = 0xc0f;
pub const CSR_HPMCOUNTER16: u16 = 0xc10;
pub const CSR_HPMCOUNTER17: u16 = 0xc11;
pub const CSR_HPMCOUNTER18: u16 = 0xc12;
pub const CSR_HPMCOUNTER19: u16 = 0xc13;
pub const CSR_HPMCOUNTER20: u16 = 0xc14;
pub const CSR_HPMCOUNTER21: u16 = 0xc15;
pub const CSR_HPMCOUNTER22: u16 = 0xc16;
pub const CSR_HPMCOUNTER23: u16 = 0xc17;
pub const CSR_HPMCOUNTER24: u16 = 0xc18;
pub const CSR_HPMCOUNTER25: u16 = 0xc19;
pub const CSR_HPMCOUNTER26: u16 = 0xc1a;
pub const CSR_HPMCOUNTER27: u16 = 0xc1b;
pub const CSR_HPMCOUNTER28: u16 = 0xc1c;
pub const CSR_HPMCOUNTER29: u16 = 0xc1d;
pub const CSR_HPMCOUNTER30: u16 = 0xc1e;
pub const CSR_HPMCOUNTER31: u16 = 0xc1f;
pub const CSR_VL: u16 = 0xc20;
pub const CSR_VTYPE: u16 = 0xc21;
pub const CSR_VLENB: u16 = 0xc22;
pub const CSR_SSTATUS: u16 = 0x100;
pub const CSR_SEDELEG: u16 = 0x102;
pub const CSR_SIDELEG: u16 = 0x103;
pub const CSR_SIE: u16 = 0x104;
pub const CSR_STVEC: u16 = 0x105;
pub const CSR_SCOUNTEREN: u16 = 0x106;
pub const CSR_SENVCFG: u16 = 0x10a;
pub const CSR_SSTATEEN0: u16 = 0x10c;
pub const CSR_SSTATEEN1: u16 = 0x10d;
pub const CSR_SSTATEEN2: u16 = 0x10e;
pub const CSR_SSTATEEN3: u16 = 0x10f;
pub const CSR_SCOUNTINHIBIT: u16 = 0x120;
pub const CSR_SSCRATCH: u16 = 0x140;
pub const CSR_SEPC: u16 = 0x141;
pub const CSR_SCAUSE: u16 = 0x142;
pub const CSR_STVAL: u16 = 0x143;
pub const CSR_SIP: u16 = 0x144;
pub const CSR_STIMECMP: u16 = 0x14d;
pub const CSR_SCTRCTL: u16 = 0x14e;
pub const CSR_SCTRSTATUS: u16 = 0x14f;
pub const CSR_SISELECT: u16 = 0x150;
pub const CSR_SIREG: u16 = 0x151;
pub const CSR_SIREG2: u16 = 0x152;
pub const CSR_SIREG3: u16 = 0x153;
pub const CSR_SIREG4: u16 = 0x155;
pub const CSR_SIREG5: u16 = 0x156;
pub const CSR_SIREG6: u16 = 0x157;
pub const CSR_STOPEI: u16 = 0x15c;
pub const CSR_SCTRDEPTH: u16 = 0x15f;
pub const CSR_SATP: u16 = 0x180;
pub const CSR_SRMCFG: u16 = 0x181;
pub const CSR_SCONTEXT: u16 = 0x5a8;
pub const CSR_VSSTATUS: u16 = 0x200;
pub const CSR_VSIE: u16 = 0x204;
pub const CSR_VSTVEC: u16 = 0x205;
pub const CSR_VSSCRATCH: u16 = 0x240;
pub const CSR_VSEPC: u16 = 0x241;
pub const CSR_VSCAUSE: u16 = 0x242;
pub const CSR_VSTVAL: u16 = 0x243;
pub const CSR_VSIP: u16 = 0x244;
pub const CSR_VSTIMECMP: u16 = 0x24d;
pub const CSR_VSCTRCTL: u16 = 0x24e;
pub const CSR_VSISELECT: u16 = 0x250;
pub const CSR_VSIREG: u16 = 0x251;
pub const CSR_VSIREG2: u16 = 0x252;
pub const CSR_VSIREG3: u16 = 0x253;
pub const CSR_VSIREG4: u16 = 0x255;
pub const CSR_VSIREG5: u16 = 0x256;
pub const CSR_VSIREG6: u16 = 0x257;
pub const CSR_VSTOPEI: u16 = 0x25c;
pub const CSR_VSATP: u16 = 0x280;
pub const CSR_HSTATUS: u16 = 0x600;
pub const CSR_HEDELEG: u16 = 0x602;
pub const CSR_HIDELEG: u16 = 0x603;
pub const CSR_HIE: u16 = 0x604;
pub const CSR_HTIMEDELTA: u16 = 0x605;
pub const CSR_HCOUNTEREN: u16 = 0x606;
pub const CSR_HGEIE: u16 = 0x607;
pub const CSR_HVIEN: u16 = 0x608;
pub const CSR_HVICTL: u16 = 0x609;
pub const CSR_HENVCFG: u16 = 0x60a;
pub const CSR_HSTATEEN0: u16 = 0x60c;
pub const CSR_HSTATEEN1: u16 = 0x60d;
pub const CSR_HSTATEEN2: u16 = 0x60e;
pub const CSR_HSTATEEN3: u16 = 0x60f;
pub const CSR_HTVAL: u16 = 0x643;
pub const CSR_HIP: u16 = 0x644;
pub const CSR_HVIP: u16 = 0x645;
pub const CSR_HVIPRIO1: u16 = 0x646;
pub const CSR_HVIPRIO2: u16 = 0x647;
pub const CSR_HTINST: u16 = 0x64a;
pub const CSR_HGATP: u16 = 0x680;
pub const CSR_HCONTEXT: u16 = 0x6a8;
pub const CSR_HGEIP: u16 = 0xe12;
pub const CSR_VSTOPI: u16 = 0xeb0;
pub const CSR_SCOUNTOVF: u16 = 0xda0;
pub const CSR_STOPI: u16 = 0xdb0;
pub const CSR_UTVT: u16 = 0x7;
pub const CSR_UNXTI: u16 = 0x45;
pub const CSR_UINTSTATUS: u16 = 0x46;
pub const CSR_USCRATCHCSW: u16 = 0x48;
pub const CSR_USCRATCHCSWL: u16 = 0x49;
pub const CSR_STVT: u16 = 0x107;
pub const CSR_SNXTI: u16 = 0x145;
pub const CSR_SINTSTATUS: u16 = 0x146;
pub const CSR_SSCRATCHCSW: u16 = 0x148;
pub const CSR_SSCRATCHCSWL: u16 = 0x149;
pub const CSR_MTVT: u16 = 0x307;
pub const CSR_MNXTI: u16 = 0x345;
pub const CSR_MINTSTATUS: u16 = 0x346;
pub const CSR_MSCRATCHCSW: u16 = 0x348;
pub const CSR_MSCRATCHCSWL: u16 = 0x349;
pub const CSR_MSTATUS: u16 = 0x300;
pub const CSR_MISA: u16 = 0x301;
pub const CSR_MEDELEG: u16 = 0x302;
pub const CSR_MIDELEG: u16 = 0x303;
pub const CSR_MIE: u16 = 0x304;
pub const CSR_MTVEC: u16 = 0x305;
pub const CSR_MCOUNTEREN: u16 = 0x306;
pub const CSR_MVIEN: u16 = 0x308;
pub const CSR_MVIP: u16 = 0x309;
pub const CSR_MENVCFG: u16 = 0x30a;
pub const CSR_MSTATEEN0: u16 = 0x30c;
pub const CSR_MSTATEEN1: u16 = 0x30d;
pub const CSR_MSTATEEN2: u16 = 0x30e;
pub const CSR_MSTATEEN3: u16 = 0x30f;
pub const CSR_MCOUNTINHIBIT: u16 = 0x320;
pub const CSR_MSCRATCH: u16 = 0x340;
pub const CSR_MEPC: u16 = 0x341;
pub const CSR_MCAUSE: u16 = 0x342;
pub const CSR_MTVAL: u16 = 0x343;
pub const CSR_MIP: u16 = 0x344;
pub const CSR_MTINST: u16 = 0x34a;
pub const CSR_MTVAL2: u16 = 0x34b;
pub const CSR_MCTRCTL: u16 = 0x34e;
pub const CSR_MISELECT: u16 = 0x350;
pub const CSR_MIREG: u16 = 0x351;
pub const CSR_MIREG2: u16 = 0x352;
pub const CSR_MIREG3: u16 = 0x353;
pub const CSR_MIREG4: u16 = 0x355;
pub const CSR_MIREG5: u16 = 0x356;
pub const CSR_MIREG6: u16 = 0x357;
pub const CSR_MTOPEI: u16 = 0x35c;
pub const CSR_PMPCFG0: u16 = 0x3a0;
pub const CSR_PMPCFG1: u16 = 0x3a1;
pub const CSR_PMPCFG2: u16 = 0x3a2;
pub const CSR_PMPCFG3: u16 = 0x3a3;
pub const CSR_PMPCFG4: u16 = 0x3a4;
pub const CSR_PMPCFG5: u16 = 0x3a5;
pub const CSR_PMPCFG6: u16 = 0x3a6;
pub const CSR_PMPCFG7: u16 = 0x3a7;
pub const CSR_PMPCFG8: u16 = 0x3a8;
pub const CSR_PMPCFG9: u16 = 0x3a9;
pub const CSR_PMPCFG10: u16 = 0x3aa;
pub const CSR_PMPCFG11: u16 = 0x3ab;
pub const CSR_PMPCFG12: u16 = 0x3ac;
pub const CSR_PMPCFG13: u16 = 0x3ad;
pub const CSR_PMPCFG14: u16 = 0x3ae;
pub const CSR_PMPCFG15: u16 = 0x3af;
pub const CSR_PMPADDR0: u16 = 0x3b0;
pub const CSR_PMPADDR1: u16 = 0x3b1;
pub const CSR_PMPADDR2: u16 = 0x3b2;
pub const CSR_PMPADDR3: u16 = 0x3b3;
pub const CSR_PMPADDR4: u16 = 0x3b4;
pub const CSR_PMPADDR5: u16 = 0x3b5;
pub const CSR_PMPADDR6: u16 = 0x3b6;
pub const CSR_PMPADDR7: u16 = 0x3b7;
pub const CSR_PMPADDR8: u16 = 0x3b8;
pub const CSR_PMPADDR9: u16 = 0x3b9;
pub const CSR_PMPADDR10: u16 = 0x3ba;
pub const CSR_PMPADDR11: u16 = 0x3bb;
pub const CSR_PMPADDR12: u16 = 0x3bc;
pub const CSR_PMPADDR13: u16 = 0x3bd;
pub const CSR_PMPADDR14: u16 = 0x3be;
pub const CSR_PMPADDR15: u16 = 0x3bf;
pub const CSR_PMPADDR16: u16 = 0x3c0;
pub const CSR_PMPADDR17: u16 = 0x3c1;
pub const CSR_PMPADDR18: u16 = 0x3c2;
pub const CSR_PMPADDR19: u16 = 0x3c3;
pub const CSR_PMPADDR20: u16 = 0x3c4;
pub const CSR_PMPADDR21: u16 = 0x3c5;
pub const CSR_PMPADDR22: u16 = 0x3c6;
pub const CSR_PMPADDR23: u16 = 0x3c7;
pub const CSR_PMPADDR24: u16 = 0x3c8;
pub const CSR_PMPADDR25: u16 = 0x3c9;
pub const CSR_PMPADDR26: u16 = 0x3ca;
pub const CSR_PMPADDR27: u16 = 0x3cb;
pub const CSR_PMPADDR28: u16 = 0x3cc;
pub const CSR_PMPADDR29: u16 = 0x3cd;
pub const CSR_PMPADDR30: u16 = 0x3ce;
pub const CSR_PMPADDR31: u16 = 0x3cf;
pub const CSR_PMPADDR32: u16 = 0x3d0;
pub const CSR_PMPADDR33: u16 = 0x3d1;
pub const CSR_PMPADDR34: u16 = 0x3d2;
pub const CSR_PMPADDR35: u16 = 0x3d3;
pub const CSR_PMPADDR36: u16 = 0x3d4;
pub const CSR_PMPADDR37: u16 = 0x3d5;
pub const CSR_PMPADDR38: u16 = 0x3d6;
pub const CSR_PMPADDR39: u16 = 0x3d7;
pub const CSR_PMPADDR40: u16 = 0x3d8;
pub const CSR_PMPADDR41: u16 = 0x3d9;
pub const CSR_PMPADDR42: u16 = 0x3da;
pub const CSR_PMPADDR43: u16 = 0x3db;
pub const CSR_PMPADDR44: u16 = 0x3dc;
pub const CSR_PMPADDR45: u16 = 0x3dd;
pub const CSR_PMPADDR46: u16 = 0x3de;
pub const CSR_PMPADDR47: u16 = 0x3df;
pub const CSR_PMPADDR48: u16 = 0x3e0;
pub const CSR_PMPADDR49: u16 = 0x3e1;
pub const CSR_PMPADDR50: u16 = 0x3e2;
pub const CSR_PMPADDR51: u16 = 0x3e3;
pub const CSR_PMPADDR52: u16 = 0x3e4;
pub const CSR_PMPADDR53: u16 = 0x3e5;
pub const CSR_PMPADDR54: u16 = 0x3e6;
pub const CSR_PMPADDR55: u16 = 0x3e7;
pub const CSR_PMPADDR56: u16 = 0x3e8;
pub const CSR_PMPADDR57: u16 = 0x3e9;
pub const CSR_PMPADDR58: u16 = 0x3ea;
pub const CSR_PMPADDR59: u16 = 0x3eb;
pub const CSR_PMPADDR60: u16 = 0x3ec;
pub const CSR_PMPADDR61: u16 = 0x3ed;
pub const CSR_PMPADDR62: u16 = 0x3ee;
pub const CSR_PMPADDR63: u16 = 0x3ef;
pub const CSR_MSECCFG: u16 = 0x747;
pub const CSR_TSELECT: u16 = 0x7a0;
pub const CSR_TDATA1: u16 = 0x7a1;
pub const CSR_TDATA2: u16 = 0x7a2;
pub const CSR_TDATA3: u16 = 0x7a3;
pub const CSR_TINFO: u16 = 0x7a4;
pub const CSR_TCONTROL: u16 = 0x7a5;
pub const CSR_MCONTEXT: u16 = 0x7a8;
pub const CSR_MSCONTEXT: u16 = 0x7aa;
pub const CSR_DCSR: u16 = 0x7b0;
pub const CSR_DPC: u16 = 0x7b1;
pub const CSR_DSCRATCH0: u16 = 0x7b2;
pub const CSR_DSCRATCH1: u16 = 0x7b3;
pub const CSR_MCYCLE: u16 = 0xb00;
pub const CSR_MINSTRET: u16 = 0xb02;
pub const CSR_MHPMCOUNTER3: u16 = 0xb03;
pub const CSR_MHPMCOUNTER4: u16 = 0xb04;
pub const CSR_MHPMCOUNTER5: u16 = 0xb05;
pub const CSR_MHPMCOUNTER6: u16 = 0xb06;
pub const CSR_MHPMCOUNTER7: u16 = 0xb07;
pub const CSR_MHPMCOUNTER8: u16 = 0xb08;
pub const CSR_MHPMCOUNTER9: u16 = 0xb09;
pub const CSR_MHPMCOUNTER10: u16 = 0xb0a;
pub const CSR_MHPMCOUNTER11: u16 = 0xb0b;
pub const CSR_MHPMCOUNTER12: u16 = 0xb0c;
pub const CSR_MHPMCOUNTER13: u16 = 0xb0d;
pub const CSR_MHPMCOUNTER14: u16 = 0xb0e;
pub const CSR_MHPMCOUNTER15: u16 = 0xb0f;
pub const CSR_MHPMCOUNTER16: u16 = 0xb10;
pub const CSR_MHPMCOUNTER17: u16 = 0xb11;
pub const CSR_MHPMCOUNTER18: u16 = 0xb12;
pub const CSR_MHPMCOUNTER19: u16 = 0xb13;
pub const CSR_MHPMCOUNTER20: u16 = 0xb14;
pub const CSR_MHPMCOUNTER21: u16 = 0xb15;
pub const CSR_MHPMCOUNTER22: u16 = 0xb16;
pub const CSR_MHPMCOUNTER23: u16 = 0xb17;
pub const CSR_MHPMCOUNTER24: u16 = 0xb18;
pub const CSR_MHPMCOUNTER25: u16 = 0xb19;
pub const CSR_MHPMCOUNTER26: u16 = 0xb1a;
pub const CSR_MHPMCOUNTER27: u16 = 0xb1b;
pub const CSR_MHPMCOUNTER28: u16 = 0xb1c;
pub const CSR_MHPMCOUNTER29: u16 = 0xb1d;
pub const CSR_MHPMCOUNTER30: u16 = 0xb1e;
pub const CSR_MHPMCOUNTER31: u16 = 0xb1f;
pub const CSR_MCYCLECFG: u16 = 0x321;
pub const CSR_MINSTRETCFG: u16 = 0x322;
pub const CSR_MHPMEVENT3: u16 = 0x323;
pub const CSR_MHPMEVENT4: u16 = 0x324;
pub const CSR_MHPMEVENT5: u16 = 0x325;
pub const CSR_MHPMEVENT6: u16 = 0x326;
pub const CSR_MHPMEVENT7: u16 = 0x327;
pub const CSR_MHPMEVENT8: u16 = 0x328;
pub const CSR_MHPMEVENT9: u16 = 0x329;
pub const CSR_MHPMEVENT10: u16 = 0x32a;
pub const CSR_MHPMEVENT11: u16 = 0x32b;
pub const CSR_MHPMEVENT12: u16 = 0x32c;
pub const CSR_MHPMEVENT13: u16 = 0x32d;
pub const CSR_MHPMEVENT14: u16 = 0x32e;
pub const CSR_MHPMEVENT15: u16 = 0x32f;
pub const CSR_MHPMEVENT16: u16 = 0x330;
pub const CSR_MHPMEVENT17: u16 = 0x331;
pub const CSR_MHPMEVENT18: u16 = 0x332;
pub const CSR_MHPMEVENT19: u16 = 0x333;
pub const CSR_MHPMEVENT20: u16 = 0x334;
pub const CSR_MHPMEVENT21: u16 = 0x335;
pub const CSR_MHPMEVENT22: u16 = 0x336;
pub const CSR_MHPMEVENT23: u16 = 0x337;
pub const CSR_MHPMEVENT24: u16 = 0x338;
pub const CSR_MHPMEVENT25: u16 = 0x339;
pub const CSR_MHPMEVENT26: u16 = 0x33a;
pub const CSR_MHPMEVENT27: u16 = 0x33b;
pub const CSR_MHPMEVENT28: u16 = 0x33c;
pub const CSR_MHPMEVENT29: u16 = 0x33d;
pub const CSR_MHPMEVENT30: u16 = 0x33e;
pub const CSR_MHPMEVENT31: u16 = 0x33f;
pub const CSR_MVENDORID: u16 = 0xf11;
pub const CSR_MARCHID: u16 = 0xf12;
pub const CSR_MIMPID: u16 = 0xf13;
pub const CSR_MHARTID: u16 = 0xf14;
pub const CSR_MCONFIGPTR: u16 = 0xf15;
pub const CSR_MTOPI: u16 = 0xfb0;
pub const CSR_SIEH: u16 = 0x114;
pub const CSR_SIPH: u16 = 0x154;
pub const CSR_STIMECMPH: u16 = 0x15d;
pub const CSR_VSIEH: u16 = 0x214;
pub const CSR_VSIPH: u16 = 0x254;
pub const CSR_VSTIMECMPH: u16 = 0x25d;
pub const CSR_HTIMEDELTAH: u16 = 0x615;
pub const CSR_HIDELEGH: u16 = 0x613;
pub const CSR_HVIENH: u16 = 0x618;
pub const CSR_HENVCFGH: u16 = 0x61a;
pub const CSR_HVIPH: u16 = 0x655;
pub const CSR_HVIPRIO1H: u16 = 0x656;
pub const CSR_HVIPRIO2H: u16 = 0x657;
pub const CSR_HSTATEEN0H: u16 = 0x61c;
pub const CSR_HSTATEEN1H: u16 = 0x61d;
pub const CSR_HSTATEEN2H: u16 = 0x61e;
pub const CSR_HSTATEEN3H: u16 = 0x61f;
pub const CSR_CYCLEH: u16 = 0xc80;
pub const CSR_TIMEH: u16 = 0xc81;
pub const CSR_INSTRETH: u16 = 0xc82;
pub const CSR_HPMCOUNTER3H: u16 = 0xc83;
pub const CSR_HPMCOUNTER4H: u16 = 0xc84;
pub const CSR_HPMCOUNTER5H: u16 = 0xc85;
pub const CSR_HPMCOUNTER6H: u16 = 0xc86;
pub const CSR_HPMCOUNTER7H: u16 = 0xc87;
pub const CSR_HPMCOUNTER8H: u16 = 0xc88;
pub const CSR_HPMCOUNTER9H: u16 = 0xc89;
pub const CSR_HPMCOUNTER10H: u16 = 0xc8a;
pub const CSR_HPMCOUNTER11H: u16 = 0xc8b;
pub const CSR_HPMCOUNTER12H: u16 = 0xc8c;
pub const CSR_HPMCOUNTER13H: u16 = 0xc8d;
pub const CSR_HPMCOUNTER14H: u16 = 0xc8e;
pub const CSR_HPMCOUNTER15H: u16 = 0xc8f;
pub const CSR_HPMCOUNTER16H: u16 = 0xc90;
pub const CSR_HPMCOUNTER17H: u16 = 0xc91;
pub const CSR_HPMCOUNTER18H: u16 = 0xc92;
pub const CSR_HPMCOUNTER19H: u16 = 0xc93;
pub const CSR_HPMCOUNTER20H: u16 = 0xc94;
pub const CSR_HPMCOUNTER21H: u16 = 0xc95;
pub const CSR_HPMCOUNTER22H: u16 = 0xc96;
pub const CSR_HPMCOUNTER23H: u16 = 0xc97;
pub const CSR_HPMCOUNTER24H: u16 = 0xc98;
pub const CSR_HPMCOUNTER25H: u16 = 0xc99;
pub const CSR_HPMCOUNTER26H: u16 = 0xc9a;
pub const CSR_HPMCOUNTER27H: u16 = 0xc9b;
pub const CSR_HPMCOUNTER28H: u16 = 0xc9c;
pub const CSR_HPMCOUNTER29H: u16 = 0xc9d;
pub const CSR_HPMCOUNTER30H: u16 = 0xc9e;
pub const CSR_HPMCOUNTER31H: u16 = 0xc9f;
pub const CSR_MSTATUSH: u16 = 0x310;
pub const CSR_MIDELEGH: u16 = 0x313;
pub const CSR_MIEH: u16 = 0x314;
pub const CSR_MVIENH: u16 = 0x318;
pub const CSR_MVIPH: u16 = 0x319;
pub const CSR_MENVCFGH: u16 = 0x31a;
pub const CSR_MSTATEEN0H: u16 = 0x31c;
pub const CSR_MSTATEEN1H: u16 = 0x31d;
pub const CSR_MSTATEEN2H: u16 = 0x31e;
pub const CSR_MSTATEEN3H: u16 = 0x31f;
pub const CSR_MIPH: u16 = 0x354;
pub const CSR_MCYCLECFGH: u16 = 0x721;
pub const CSR_MINSTRETCFGH: u16 = 0x722;
pub const CSR_MHPMEVENT3H: u16 = 0x723;
pub const CSR_MHPMEVENT4H: u16 = 0x724;
pub const CSR_MHPMEVENT5H: u16 = 0x725;
pub const CSR_MHPMEVENT6H: u16 = 0x726;
pub const CSR_MHPMEVENT7H: u16 = 0x727;
pub const CSR_MHPMEVENT8H: u16 = 0x728;
pub const CSR_MHPMEVENT9H: u16 = 0x729;
pub const CSR_MHPMEVENT10H: u16 = 0x72a;
pub const CSR_MHPMEVENT11H: u16 = 0x72b;
pub const CSR_MHPMEVENT12H: u16 = 0x72c;
pub const CSR_MHPMEVENT13H: u16 = 0x72d;
pub const CSR_MHPMEVENT14H: u16 = 0x72e;
pub const CSR_MHPMEVENT15H: u16 = 0x72f;
pub const CSR_MHPMEVENT16H: u16 = 0x730;
pub const CSR_MHPMEVENT17H: u16 = 0x731;
pub const CSR_MHPMEVENT18H: u16 = 0x732;
pub const CSR_MHPMEVENT19H: u16 = 0x733;
pub const CSR_MHPMEVENT20H: u16 = 0x734;
pub const CSR_MHPMEVENT21H: u16 = 0x735;
pub const CSR_MHPMEVENT22H: u16 = 0x736;
pub const CSR_MHPMEVENT23H: u16 = 0x737;
pub const CSR_MHPMEVENT24H: u16 = 0x738;
pub const CSR_MHPMEVENT25H: u16 = 0x739;
pub const CSR_MHPMEVENT26H: u16 = 0x73a;
pub const CSR_MHPMEVENT27H: u16 = 0x73b;
pub const CSR_MHPMEVENT28H: u16 = 0x73c;
pub const CSR_MHPMEVENT29H: u16 = 0x73d;
pub const CSR_MHPMEVENT30H: u16 = 0x73e;
pub const CSR_MHPMEVENT31H: u16 = 0x73f;
pub const CSR_MNSCRATCH: u16 = 0x740;
pub const CSR_MNEPC: u16 = 0x741;
pub const CSR_MNCAUSE: u16 = 0x742;
pub const CSR_MNSTATUS: u16 = 0x744;
pub const CSR_MSECCFGH: u16 = 0x757;
pub const CSR_MCYCLEH: u16 = 0xb80;
pub const CSR_MINSTRETH: u16 = 0xb82;
pub const CSR_MHPMCOUNTER3H: u16 = 0xb83;
pub const CSR_MHPMCOUNTER4H: u16 = 0xb84;
pub const CSR_MHPMCOUNTER5H: u16 = 0xb85;
pub const CSR_MHPMCOUNTER6H: u16 = 0xb86;
pub const CSR_MHPMCOUNTER7H: u16 = 0xb87;
pub const CSR_MHPMCOUNTER8H: u16 = 0xb88;
pub const CSR_MHPMCOUNTER9H: u16 = 0xb89;
pub const CSR_MHPMCOUNTER10H: u16 = 0xb8a;
pub const CSR_MHPMCOUNTER11H: u16 = 0xb8b;
pub const CSR_MHPMCOUNTER12H: u16 = 0xb8c;
pub const CSR_MHPMCOUNTER13H: u16 = 0xb8d;
pub const CSR_MHPMCOUNTER14H: u16 = 0xb8e;
pub const CSR_MHPMCOUNTER15H: u16 = 0xb8f;
pub const CSR_MHPMCOUNTER16H: u16 = 0xb90;
pub const CSR_MHPMCOUNTER17H: u16 = 0xb91;
pub const CSR_MHPMCOUNTER18H: u16 = 0xb92;
pub const CSR_MHPMCOUNTER19H: u16 = 0xb93;
pub const CSR_MHPMCOUNTER20H: u16 = 0xb94;
pub const CSR_MHPMCOUNTER21H: u16 = 0xb95;
pub const CSR_MHPMCOUNTER22H: u16 = 0xb96;
pub const CSR_MHPMCOUNTER23H: u16 = 0xb97;
pub const CSR_MHPMCOUNTER24H: u16 = 0xb98;
pub const CSR_MHPMCOUNTER25H: u16 = 0xb99;
pub const CSR_MHPMCOUNTER26H: u16 = 0xb9a;
pub const CSR_MHPMCOUNTER27H: u16 = 0xb9b;
pub const CSR_MHPMCOUNTER28H: u16 = 0xb9c;
pub const CSR_MHPMCOUNTER29H: u16 = 0xb9d;
pub const CSR_MHPMCOUNTER30H: u16 = 0xb9e;
pub const CSR_MHPMCOUNTER31H: u16 = 0xb9f;
pub const CAUSE_MISALIGNED_FETCH: u8 = 0x0;
pub const CAUSE_FETCH_ACCESS: u8 = 0x1;
pub const CAUSE_ILLEGAL_INSTRUCTION: u8 = 0x2;
pub const CAUSE_BREAKPOINT: u8 = 0x3;
pub const CAUSE_MISALIGNED_LOAD: u8 = 0x4;
pub const CAUSE_LOAD_ACCESS: u8 = 0x5;
pub const CAUSE_MISALIGNED_STORE: u8 = 0x6;
pub const CAUSE_STORE_ACCESS: u8 = 0x7;
pub const CAUSE_USER_ECALL: u8 = 0x8;
pub const CAUSE_SUPERVISOR_ECALL: u8 = 0x9;
pub const CAUSE_VIRTUAL_SUPERVISOR_ECALL: u8 = 0xa;
pub const CAUSE_MACHINE_ECALL: u8 = 0xb;
pub const CAUSE_FETCH_PAGE_FAULT: u8 = 0xc;
pub const CAUSE_LOAD_PAGE_FAULT: u8 = 0xd;
pub const CAUSE_STORE_PAGE_FAULT: u8 = 0xf;
pub const CAUSE_DOUBLE_TRAP: u8 = 0x10;
pub const CAUSE_SOFTWARE_CHECK_FAULT: u8 = 0x12;
pub const CAUSE_HARDWARE_ERROR_FAULT: u8 = 0x13;
pub const CAUSE_FETCH_GUEST_PAGE_FAULT: u8 = 0x14;
pub const CAUSE_LOAD_GUEST_PAGE_FAULT: u8 = 0x15;
pub const CAUSE_VIRTUAL_INSTRUCTION: u8 = 0x16;
pub const CAUSE_STORE_GUEST_PAGE_FAULT: u8 = 0x17;
/*pub static OPCODE32_MATCH: [u32; 1021] = [
0x33, /* add */
0xffff_ffff,/* add_uw */
0x13, /* addi */
0xffff_ffff,/* addiw */
0xffff_ffff,/* addw */
0x2a000033, /* aes32dsi */
0x2e000033, /* aes32dsmi */
0x22000033, /* aes32esi */
0x26000033, /* aes32esmi */
0xffff_ffff,/* aes64ds */
0xffff_ffff,/* aes64dsm */
0xffff_ffff,/* aes64es */
0xffff_ffff,/* aes64esm */
0xffff_ffff,/* aes64im */
0xffff_ffff,/* aes64ks1i */
0xffff_ffff,/* aes64ks2 */
0x2f, /* amoadd_b */
0xffff_ffff,/* amoadd_d */
0x102f, /* amoadd_h */
0x202f, /* amoadd_w */
0x6000002f, /* amoand_b */
0xffff_ffff,/* amoand_d */
0x6000102f, /* amoand_h */
0x6000202f, /* amoand_w */
0x2800002f, /* amocas_b */
0x2800302f, /* amocas_d */
0x2800102f, /* amocas_h */
0xffff_ffff,/* amocas_q */
0x2800202f, /* amocas_w */
0xa000002f, /* amomax_b */
0xffff_ffff,/* amomax_d */
0xa000102f, /* amomax_h */
0xa000202f, /* amomax_w */
0xe000002f, /* amomaxu_b */
0xffff_ffff,/* amomaxu_d */
0xe000102f, /* amomaxu_h */
0xe000202f, /* amomaxu_w */
0x8000002f, /* amomin_b */
0xffff_ffff,/* amomin_d */
0x8000102f, /* amomin_h */
0x8000202f, /* amomin_w */
0xc000002f, /* amominu_b */
0xffff_ffff,/* amominu_d */
0xc000102f, /* amominu_h */
0xc000202f, /* amominu_w */
0x4000002f, /* amoor_b */
0xffff_ffff,/* amoor_d */
0x4000102f, /* amoor_h */
0x4000202f, /* amoor_w */
0x800002f, /* amoswap_b */
0xffff_ffff,/* amoswap_d */
0x800102f, /* amoswap_h */
0x800202f, /* amoswap_w */
0x2000002f, /* amoxor_b */
0xffff_ffff,/* amoxor_d */
0x2000102f, /* amoxor_h */
0x2000202f, /* amoxor_w */
0x7033, /* and */
0x7013, /* andi */
0x40007033, /* andn */
0x17, /* auipc */
0x48001033, /* bclr */
0xffff_ffff,/* bclri */
0x48001013, /* bclri_rv32 */
0x63, /* beq */
0x63, /* beqz */
0x48005033, /* bext */
0xffff_ffff,/* bexti */
0x48005013, /* bexti_rv32 */
0x5063, /* bge */
0x7063, /* bgeu */
0x5063, /* bgez */
0x4063, /* bgt */
0x6063, /* bgtu */
0x4063, /* bgtz */
0x68001033, /* binv */
0xffff_ffff,/* binvi */
0x68001013, /* binvi_rv32 */
0x5063, /* ble */
0x7063, /* bleu */
0x5063, /* blez */
0x4063, /* blt */
0x6063, /* bltu */
0x4063, /* bltz */
0x1063, /* bne */
0x1063, /* bnez */
0x68705013, /* brev8 */
0x28001033, /* bset */
0xffff_ffff,/* bseti */
0x28001013, /* bseti_rv32 */
0x9002, /* c_add */
0x1, /* c_addi */
0x6101, /* c_addi16sp */
0x0, /* c_addi4spn */
0xffff_ffff,/* c_addiw */
0xffff_ffff,/* c_addw */
0x8c61, /* c_and */
0x8801, /* c_andi */
0xc001, /* c_beqz */
0xe001, /* c_bnez */
0x9002, /* c_ebreak */
0x2000, /* c_fld */
0x2002, /* c_fldsp */
0x6000, /* c_flw */
0x6002, /* c_flwsp */
0xa000, /* c_fsd */
0xa002, /* c_fsdsp */
0xe000, /* c_fsw */
0xe002, /* c_fswsp */
0xa001, /* c_j */
0x2001, /* c_jal */
0x9002, /* c_jalr */
0x8002, /* c_jr */
0x8000, /* c_lbu */
0xffff_ffff,/* c_ld */
0xffff_ffff,/* c_ldsp */
0x8440, /* c_lh */
0x8400, /* c_lhu */
0x4001, /* c_li */
0x6001, /* c_lui */
0x4000, /* c_lw */
0x4002, /* c_lwsp */
0x6081, /* c_mop_1 */
0x6581, /* c_mop_11 */
0x6681, /* c_mop_13 */
0x6781, /* c_mop_15 */
0x6181, /* c_mop_3 */
0x6281, /* c_mop_5 */
0x6381, /* c_mop_7 */
0x6481, /* c_mop_9 */
0x6081, /* c_mop_N */
0x9c41, /* c_mul */
0x8002, /* c_mv */
0x1, /* c_nop */
0x9c75, /* c_not */
0x9016, /* c_ntl_all */
0x900a, /* c_ntl_p1 */
0x900e, /* c_ntl_pall */
0x9012, /* c_ntl_s1 */
0x8c41, /* c_or */
0x8800, /* c_sb */
0xffff_ffff,/* c_sd */
0xffff_ffff,/* c_sdsp */
0x9c65, /* c_sext_b */
0x9c6d, /* c_sext_h */
0xffff_ffff,/* c_sext_w */
0x8c00, /* c_sh */
0x2, /* c_slli */
0x2, /* c_slli_rv32 */
0x8401, /* c_srai */
0x8401, /* c_srai_rv32 */
0x8001, /* c_srli */
0x8001, /* c_srli_rv32 */
0x8c01, /* c_sub */
0xffff_ffff,/* c_subw */
0xc000, /* c_sw */
0xc002, /* c_swsp */
0x8c21, /* c_xor */
0x9c61, /* c_zext_b */
0x9c69, /* c_zext_h */
0xffff_ffff,/* c_zext_w */
0x10200f, /* cbo_clean */
0x20200f, /* cbo_flush */
0x200f, /* cbo_inval */
0x40200f, /* cbo_zero */
0xa001033, /* clmul */
0xa003033, /* clmulh */
0xa002033, /* clmulr */
0x60001013, /* clz */
0xffff_ffff,/* clzw */
0xa002, /* cm_jalt */
0xac62, /* cm_mva01s */
0xac22, /* cm_mvsa01 */
0xba02, /* cm_pop */
0xbe02, /* cm_popret */
0xbc02, /* cm_popretz */
0xb802, /* cm_push */
0x60201013, /* cpop */
0xffff_ffff,/* cpopw */
0x3073, /* csrc */
0x7073, /* csrci */
0x2073, /* csrr */
0x3073, /* csrrc */
0x7073, /* csrrci */
0x2073, /* csrrs */
0x6073, /* csrrsi */
0x1073, /* csrrw */
0x5073, /* csrrwi */
0x2073, /* csrs */
0x6073, /* csrsi */
0x1073, /* csrw */
0x5073, /* csrwi */
0x60101013, /* ctz */
0xffff_ffff,/* ctzw */
0xe005033, /* czero_eqz */
0xe007033, /* czero_nez */
0x2004033, /* div */
0x2005033, /* divu */
0xffff_ffff,/* divuw */
0xffff_ffff,/* divw */
0x7b200073, /* dret */
0x100073, /* ebreak */
0x73, /* ecall */
0x22002053, /* fabs_d */
0x24002053, /* fabs_h */
0x26002053, /* fabs_q */
0x20002053, /* fabs_s */
0x2000053, /* fadd_d */
0x4000053, /* fadd_h */
0x6000053, /* fadd_q */
0x53, /* fadd_s */
0xe2001053, /* fclass_d */
0xe4001053, /* fclass_h */
0xe6001053, /* fclass_q */
0xe0001053, /* fclass_s */
0x42200053, /* fcvt_d_h */
0xffff_ffff,/* fcvt_d_l */
0xffff_ffff,/* fcvt_d_lu */
0x42300053, /* fcvt_d_q */
0x42000053, /* fcvt_d_s */
0xd2000053, /* fcvt_d_w */
0xd2100053, /* fcvt_d_wu */
0x44100053, /* fcvt_h_d */
0xffff_ffff,/* fcvt_h_l */
0xffff_ffff,/* fcvt_h_lu */
0x44300053, /* fcvt_h_q */
0x44000053, /* fcvt_h_s */
0xd4000053, /* fcvt_h_w */
0xd4100053, /* fcvt_h_wu */
0xffff_ffff,/* fcvt_l_d */
0xffff_ffff,/* fcvt_l_h */
0xffff_ffff,/* fcvt_l_q */
0xffff_ffff,/* fcvt_l_s */
0xffff_ffff,/* fcvt_lu_d */
0xffff_ffff,/* fcvt_lu_h */
0xffff_ffff,/* fcvt_lu_q */
0xffff_ffff,/* fcvt_lu_s */
0x46100053, /* fcvt_q_d */
0x46200053, /* fcvt_q_h */
0xffff_ffff,/* fcvt_q_l */
0xffff_ffff,/* fcvt_q_lu */
0x46000053, /* fcvt_q_s */
0xd6000053, /* fcvt_q_w */
0xd6100053, /* fcvt_q_wu */
0x40100053, /* fcvt_s_d */
0x40200053, /* fcvt_s_h */
0xffff_ffff,/* fcvt_s_l */
0xffff_ffff,/* fcvt_s_lu */
0x40300053, /* fcvt_s_q */
0xd0000053, /* fcvt_s_w */
0xd0100053, /* fcvt_s_wu */
0xc2000053, /* fcvt_w_d */
0xc4000053, /* fcvt_w_h */
0xc6000053, /* fcvt_w_q */
0xc0000053, /* fcvt_w_s */
0xc2100053, /* fcvt_wu_d */
0xc4100053, /* fcvt_wu_h */
0xc6100053, /* fcvt_wu_q */
0xc0100053, /* fcvt_wu_s */
0xc2801053, /* fcvtmod_w_d */
0x1a000053, /* fdiv_d */
0x1c000053, /* fdiv_h */
0x1e000053, /* fdiv_q */
0x18000053, /* fdiv_s */
0xf, /* fence */
0x100f, /* fence_i */
0x8330000f, /* fence_tso */
0xa2002053, /* feq_d */
0xa4002053, /* feq_h */
0xa6002053, /* feq_q */
0xa0002053, /* feq_s */
0x3007, /* fld */
0xa2000053, /* fle_d */
0xa4000053, /* fle_h */
0xa6000053, /* fle_q */
0xa0000053, /* fle_s */
0xa2004053, /* fleq_d */
0xa4004053, /* fleq_h */
0xa6004053, /* fleq_q */
0xa0004053, /* fleq_s */
0x1007, /* flh */
0xf2100053, /* fli_d */
0xf4100053, /* fli_h */
0xf6100053, /* fli_q */
0xf0100053, /* fli_s */
0x4007, /* flq */
0xa2001053, /* flt_d */
0xa4001053, /* flt_h */
0xa6001053, /* flt_q */
0xa0001053, /* flt_s */
0xa2005053, /* fltq_d */
0xa4005053, /* fltq_h */
0xa6005053, /* fltq_q */
0xa0005053, /* fltq_s */
0x2007, /* flw */
0x2000043, /* fmadd_d */
0x4000043, /* fmadd_h */
0x6000043, /* fmadd_q */
0x43, /* fmadd_s */
0x2a001053, /* fmax_d */
0x2c001053, /* fmax_h */
0x2e001053, /* fmax_q */
0x28001053, /* fmax_s */
0x2a003053, /* fmaxm_d */
0x2c003053, /* fmaxm_h */
0x2e003053, /* fmaxm_q */
0x28003053, /* fmaxm_s */
0x2a000053, /* fmin_d */
0x2c000053, /* fmin_h */
0x2e000053, /* fmin_q */
0x28000053, /* fmin_s */
0x2a002053, /* fminm_d */
0x2c002053, /* fminm_h */
0x2e002053, /* fminm_q */
0x28002053, /* fminm_s */
0x2000047, /* fmsub_d */
0x4000047, /* fmsub_h */
0x6000047, /* fmsub_q */
0x47, /* fmsub_s */
0x12000053, /* fmul_d */
0x14000053, /* fmul_h */
0x16000053, /* fmul_q */
0x10000053, /* fmul_s */
0x22000053, /* fmv_d */
0xffff_ffff,/* fmv_d_x */
0x24000053, /* fmv_h */
0xf4000053, /* fmv_h_x */
0x26000053, /* fmv_q */
0x20000053, /* fmv_s */
0xf0000053, /* fmv_s_x */
0xf0000053, /* fmv_w_x */
0xffff_ffff,/* fmv_x_d */
0xe4000053, /* fmv_x_h */
0xe0000053, /* fmv_x_s */
0xe0000053, /* fmv_x_w */
0xe2100053, /* fmvh_x_d */
0xffff_ffff,/* fmvh_x_q */
0xb2000053, /* fmvp_d_x */
0xffff_ffff,/* fmvp_q_x */
0x22001053, /* fneg_d */
0x24001053, /* fneg_h */
0x26001053, /* fneg_q */
0x20001053, /* fneg_s */
0x200004f, /* fnmadd_d */
0x400004f, /* fnmadd_h */
0x600004f, /* fnmadd_q */
0x4f, /* fnmadd_s */
0x200004b, /* fnmsub_d */
0x400004b, /* fnmsub_h */
0x600004b, /* fnmsub_q */
0x4b, /* fnmsub_s */
0x302073, /* frcsr */
0x102073, /* frflags */
0x42400053, /* fround_d */
0x44400053, /* fround_h */
0x46400053, /* fround_q */
0x40400053, /* fround_s */
0x42500053, /* froundnx_d */
0x44500053, /* froundnx_h */
0x46500053, /* froundnx_q */
0x40500053, /* froundnx_s */
0x202073, /* frrm */
0x301073, /* fscsr */
0x3027, /* fsd */
0x101073, /* fsflags */
0x105073, /* fsflagsi */
0x22000053, /* fsgnj_d */
0x24000053, /* fsgnj_h */
0x26000053, /* fsgnj_q */
0x20000053, /* fsgnj_s */
0x22001053, /* fsgnjn_d */
0x24001053, /* fsgnjn_h */
0x26001053, /* fsgnjn_q */
0x20001053, /* fsgnjn_s */
0x22002053, /* fsgnjx_d */
0x24002053, /* fsgnjx_h */
0x26002053, /* fsgnjx_q */
0x20002053, /* fsgnjx_s */
0x1027, /* fsh */
0x4027, /* fsq */
0x5a000053, /* fsqrt_d */
0x5c000053, /* fsqrt_h */
0x5e000053, /* fsqrt_q */
0x58000053, /* fsqrt_s */
0x201073, /* fsrm */
0x205073, /* fsrmi */
0xa000053, /* fsub_d */
0xc000053, /* fsub_h */
0xe000053, /* fsub_q */
0x8000053, /* fsub_s */
0x2027, /* fsw */
0x62000073, /* hfence_gvma */
0x22000073, /* hfence_vvma */
0x66000073, /* hinval_gvma */
0x26000073, /* hinval_vvma */
0x60004073, /* hlv_b */
0x60104073, /* hlv_bu */
0xffff_ffff,/* hlv_d */
0x64004073, /* hlv_h */
0x64104073, /* hlv_hu */
0x68004073, /* hlv_w */
0xffff_ffff,/* hlv_wu */
0x64304073, /* hlvx_hu */
0x68304073, /* hlvx_wu */
0x62004073, /* hsv_b */
0xffff_ffff,/* hsv_d */
0x66004073, /* hsv_h */
0x6a004073, /* hsv_w */
0x6f, /* j */
0x6f, /* jal */
0xef, /* jal_pseudo */
0x67, /* jalr */
0xe7, /* jalr_pseudo */
0x67, /* jr */
0x3, /* lb */
0x4003, /* lbu */
0xffff_ffff,/* ld */
0x1003, /* lh */
0x5003, /* lhu */
0xffff_ffff,/* lr_d */
0x1000202f, /* lr_w */
0x37, /* lui */
0x2003, /* lw */
0xffff_ffff,/* lwu */
0xa006033, /* max */
0xa007033, /* maxu */
0xa004033, /* min */
0xa005033, /* minu */
0x81c04073, /* mop_r_0 */
0x81d04073, /* mop_r_1 */
0x89e04073, /* mop_r_10 */
0x89f04073, /* mop_r_11 */
0x8dc04073, /* mop_r_12 */
0x8dd04073, /* mop_r_13 */
0x8de04073, /* mop_r_14 */
0x8df04073, /* mop_r_15 */
0xc1c04073, /* mop_r_16 */
0xc1d04073, /* mop_r_17 */
0xc1e04073, /* mop_r_18 */
0xc1f04073, /* mop_r_19 */
0x81e04073, /* mop_r_2 */
0xc5c04073, /* mop_r_20 */
0xc5d04073, /* mop_r_21 */
0xc5e04073, /* mop_r_22 */
0xc5f04073, /* mop_r_23 */
0xc9c04073, /* mop_r_24 */
0xc9d04073, /* mop_r_25 */
0xc9e04073, /* mop_r_26 */
0xc9f04073, /* mop_r_27 */
0xcdc04073, /* mop_r_28 */
0xcdd04073, /* mop_r_29 */
0x81f04073, /* mop_r_3 */
0xcde04073, /* mop_r_30 */
0xcdf04073, /* mop_r_31 */
0x85c04073, /* mop_r_4 */
0x85d04073, /* mop_r_5 */
0x85e04073, /* mop_r_6 */
0x85f04073, /* mop_r_7 */
0x89c04073, /* mop_r_8 */
0x89d04073, /* mop_r_9 */
0x81c04073, /* mop_r_N */
0x82004073, /* mop_rr_0 */
0x86004073, /* mop_rr_1 */
0x8a004073, /* mop_rr_2 */
0x8e004073, /* mop_rr_3 */
0xc2004073, /* mop_rr_4 */
0xc6004073, /* mop_rr_5 */
0xca004073, /* mop_rr_6 */
0xce004073, /* mop_rr_7 */
0x82004073, /* mop_rr_N */
0x30200073, /* mret */
0x2000033, /* mul */
0x2001033, /* mulh */
0x2002033, /* mulhsu */
0x2003033, /* mulhu */
0xffff_ffff,/* mulw */
0x13, /* mv */
0x40000033, /* neg */
0x13, /* nop */
0x500033, /* ntl_all */
0x200033, /* ntl_p1 */
0x300033, /* ntl_pall */
0x400033, /* ntl_s1 */
0x6033, /* or */
0x28705013, /* orc_b */
0x6013, /* ori */
0x40006033, /* orn */
0x8004033, /* pack */
0x8007033, /* packh */
0xffff_ffff,/* packw */
0x100000f, /* pause */
0x6013, /* prefetch_i */
0x106013, /* prefetch_r */
0x306013, /* prefetch_w */
0xc0002073, /* rdcycle */
0xc8002073, /* rdcycleh */
0xc0202073, /* rdinstret */
0xc8202073, /* rdinstreth */
0xc0102073, /* rdtime */
0xc8102073, /* rdtimeh */
0x2006033, /* rem */
0x2007033, /* remu */
0xffff_ffff,/* remuw */
0xffff_ffff,/* remw */
0x8067, /* ret */
0xffff_ffff,/* rev8 */
0x69805013, /* rev8_rv32 */
0x60001033, /* rol */
0xffff_ffff,/* rolw */
0x60005033, /* ror */
0xffff_ffff,/* rori */
0x60005013, /* rori_rv32 */
0xffff_ffff,/* roriw */
0xffff_ffff,/* rorw */
0x23, /* sb */
0x100073, /* sbreak */
0xffff_ffff,/* sc_d */
0x1800202f, /* sc_w */
0x73, /* scall */
0xffff_ffff,/* sd */
0x103013, /* seqz */
0x60401013, /* sext_b */
0x60501013, /* sext_h */
0xffff_ffff,/* sext_w */
0x18100073, /* sfence_inval_ir */
0x12000073, /* sfence_vma */
0x18000073, /* sfence_w_inval */
0x2033, /* sgtz */
0x1023, /* sh */
0x20002033, /* sh1add */
0xffff_ffff,/* sh1add_uw */
0x20004033, /* sh2add */
0xffff_ffff,/* sh2add_uw */
0x20006033, /* sh3add */
0xffff_ffff,/* sh3add_uw */
0x10201013, /* sha256sig0 */
0x10301013, /* sha256sig1 */
0x10001013, /* sha256sum0 */
0x10101013, /* sha256sum1 */
0xffff_ffff,/* sha512sig0 */
0x5c000033, /* sha512sig0h */
0x54000033, /* sha512sig0l */
0xffff_ffff,/* sha512sig1 */
0x5e000033, /* sha512sig1h */
0x56000033, /* sha512sig1l */
0xffff_ffff,/* sha512sum0 */
0x50000033, /* sha512sum0r */
0xffff_ffff,/* sha512sum1 */
0x52000033, /* sha512sum1r */
0x16000073, /* sinval_vma */
0x1033, /* sll */
0x1013, /* slli */
0x1013, /* slli_rv32 */
0xffff_ffff,/* slli_uw */
0xffff_ffff,/* slliw */
0xffff_ffff,/* sllw */
0x2033, /* slt */
0x2013, /* slti */
0x3013, /* sltiu */
0x3033, /* sltu */
0x2033, /* sltz */
0x10801013, /* sm3p0 */
0x10901013, /* sm3p1 */
0x30000033, /* sm4ed */
0x34000033, /* sm4ks */
0x3033, /* snez */
0x40005033, /* sra */
0x40005013, /* srai */
0x40005013, /* srai_rv32 */
0xffff_ffff,/* sraiw */
0xffff_ffff,/* sraw */
0x10200073, /* sret */
0x5033, /* srl */
0x5013, /* srli */
0x5013, /* srli_rv32 */
0xffff_ffff,/* srliw */
0xffff_ffff,/* srlw */
0x40000033, /* sub */
0xffff_ffff,/* subw */
0x2023, /* sw */
0x8f05013, /* unzip */
0x24002057, /* vaadd_vv */
0x24006057, /* vaadd_vx */
0x20002057, /* vaaddu_vv */
0x20006057, /* vaaddu_vx */
0x40003057, /* vadc_vim */
0x40000057, /* vadc_vvm */
0x40004057, /* vadc_vxm */
0x3057, /* vadd_vi */
0x57, /* vadd_vv */
0x4057, /* vadd_vx */
0xa600a077, /* vaesdf_vs */
0xa200a077, /* vaesdf_vv */
0xa6002077, /* vaesdm_vs */
0xa2002077, /* vaesdm_vv */
0xa601a077, /* vaesef_vs */
0xa201a077, /* vaesef_vv */
0xa6012077, /* vaesem_vs */
0xa2012077, /* vaesem_vv */
0x8a002077, /* vaeskf1_vi */
0xaa002077, /* vaeskf2_vi */
0xa603a077, /* vaesz_vs */
0x24003057, /* vand_vi */
0x24000057, /* vand_vv */
0x24004057, /* vand_vx */
0x4000057, /* vandn_vv */
0x4004057, /* vandn_vx */
0x2c002057, /* vasub_vv */
0x2c006057, /* vasub_vx */
0x28002057, /* vasubu_vv */
0x28006057, /* vasubu_vx */
0x48042057, /* vbrev8_v */
0x48052057, /* vbrev_v */
0x30002057, /* vclmul_vv */
0x30006057, /* vclmul_vx */
0x34002057, /* vclmulh_vv */
0x34006057, /* vclmulh_vx */
0x48062057, /* vclz_v */
0x5e002057, /* vcompress_vm */
0x40082057, /* vcpop_m */
0x48072057, /* vcpop_v */
0x4806a057, /* vctz_v */
0x84002057, /* vdiv_vv */
0x84006057, /* vdiv_vx */
0x80002057, /* vdivu_vv */
0x80006057, /* vdivu_vx */
0x5057, /* vfadd_vf */
0x1057, /* vfadd_vv */
0x4c081057, /* vfclass_v */
0x48019057, /* vfcvt_f_x_v */
0x48011057, /* vfcvt_f_xu_v */
0x48039057, /* vfcvt_rtz_x_f_v */
0x48031057, /* vfcvt_rtz_xu_f_v */
0x48009057, /* vfcvt_x_f_v */
0x48001057, /* vfcvt_xu_f_v */
0x80005057, /* vfdiv_vf */
0x80001057, /* vfdiv_vv */
0x4008a057, /* vfirst_m */
0xb0005057, /* vfmacc_vf */
0xb0001057, /* vfmacc_vv */
0xa0005057, /* vfmadd_vf */
0xa0001057, /* vfmadd_vv */
0x18005057, /* vfmax_vf */
0x18001057, /* vfmax_vv */
0x5c005057, /* vfmerge_vfm */
0x10005057, /* vfmin_vf */
0x10001057, /* vfmin_vv */
0xb8005057, /* vfmsac_vf */
0xb8001057, /* vfmsac_vv */
0xa8005057, /* vfmsub_vf */
0xa8001057, /* vfmsub_vv */
0x90005057, /* vfmul_vf */
0x90001057, /* vfmul_vv */
0x42001057, /* vfmv_f_s */
0x42005057, /* vfmv_s_f */
0x5e005057, /* vfmv_v_f */
0x480a1057, /* vfncvt_f_f_w */
0x48099057, /* vfncvt_f_x_w */
0x48091057, /* vfncvt_f_xu_w */
0x480a9057, /* vfncvt_rod_f_f_w */
0x480b9057, /* vfncvt_rtz_x_f_w */
0x480b1057, /* vfncvt_rtz_xu_f_w */
0x48089057, /* vfncvt_x_f_w */
0x48081057, /* vfncvt_xu_f_w */
0xb4005057, /* vfnmacc_vf */
0xb4001057, /* vfnmacc_vv */
0xa4005057, /* vfnmadd_vf */
0xa4001057, /* vfnmadd_vv */
0xbc005057, /* vfnmsac_vf */
0xbc001057, /* vfnmsac_vv */
0xac005057, /* vfnmsub_vf */
0xac001057, /* vfnmsub_vv */
0x84005057, /* vfrdiv_vf */
0x4c029057, /* vfrec7_v */
0x1c001057, /* vfredmax_vs */
0x14001057, /* vfredmin_vs */
0xc001057, /* vfredosum_vs */
0x4001057, /* vfredsum_vs */
0x4001057, /* vfredusum_vs */
0x4c021057, /* vfrsqrt7_v */
0x9c005057, /* vfrsub_vf */
0x20005057, /* vfsgnj_vf */
0x20001057, /* vfsgnj_vv */
0x24005057, /* vfsgnjn_vf */
0x24001057, /* vfsgnjn_vv */
0x28005057, /* vfsgnjx_vf */
0x28001057, /* vfsgnjx_vv */
0x3c005057, /* vfslide1down_vf */
0x38005057, /* vfslide1up_vf */
0x4c001057, /* vfsqrt_v */
0x8005057, /* vfsub_vf */
0x8001057, /* vfsub_vv */
0xc0005057, /* vfwadd_vf */
0xc0001057, /* vfwadd_vv */
0xd0005057, /* vfwadd_wf */
0xd0001057, /* vfwadd_wv */
0x48061057, /* vfwcvt_f_f_v */
0x48059057, /* vfwcvt_f_x_v */
0x48051057, /* vfwcvt_f_xu_v */
0x48079057, /* vfwcvt_rtz_x_f_v */
0x48071057, /* vfwcvt_rtz_xu_f_v */
0x48049057, /* vfwcvt_x_f_v */
0x48041057, /* vfwcvt_xu_f_v */
0xf0005057, /* vfwmacc_vf */
0xf0001057, /* vfwmacc_vv */
0xf8005057, /* vfwmsac_vf */
0xf8001057, /* vfwmsac_vv */
0xe0005057, /* vfwmul_vf */
0xe0001057, /* vfwmul_vv */
0xf4005057, /* vfwnmacc_vf */
0xf4001057, /* vfwnmacc_vv */
0xfc005057, /* vfwnmsac_vf */
0xfc001057, /* vfwnmsac_vv */
0xcc001057, /* vfwredosum_vs */
0xc4001057, /* vfwredsum_vs */
0xc4001057, /* vfwredusum_vs */
0xc8005057, /* vfwsub_vf */
0xc8001057, /* vfwsub_vv */
0xd8005057, /* vfwsub_wf */
0xd8001057, /* vfwsub_wv */
0xb2002077, /* vghsh_vv */
0xa208a077, /* vgmul_vv */
0x5008a057, /* vid_v */
0x50082057, /* viota_m */
0x2800007, /* vl1r_v */
0x2805007, /* vl1re16_v */
0x2806007, /* vl1re32_v */
0x2807007, /* vl1re64_v */
0x2800007, /* vl1re8_v */
0x22800007, /* vl2r_v */
0x22805007, /* vl2re16_v */
0x22806007, /* vl2re32_v */
0x22807007, /* vl2re64_v */
0x22800007, /* vl2re8_v */
0x62800007, /* vl4r_v */
0x62805007, /* vl4re16_v */
0x62806007, /* vl4re32_v */
0x62807007, /* vl4re64_v */
0x62800007, /* vl4re8_v */
0xe2800007, /* vl8r_v */
0xe2805007, /* vl8re16_v */
0xe2806007, /* vl8re32_v */
0xe2807007, /* vl8re64_v */
0xe2800007, /* vl8re8_v */
0x5007, /* vle16_v */
0x1005007, /* vle16ff_v */
0x2b00007, /* vle1_v */
0x6007, /* vle32_v */
0x1006007, /* vle32ff_v */
0x7007, /* vle64_v */
0x1007007, /* vle64ff_v */
0x7, /* vle8_v */
0x1000007, /* vle8ff_v */
0x2b00007, /* vlm_v */
0xc005007, /* vloxei16_v */
0xc006007, /* vloxei32_v */
0xc007007, /* vloxei64_v */
0xc000007, /* vloxei8_v */
0x8005007, /* vlse16_v */
0x8006007, /* vlse32_v */
0x8007007, /* vlse64_v */
0x8000007, /* vlse8_v */
0x4005007, /* vluxei16_v */
0x4006007, /* vluxei32_v */
0x4007007, /* vluxei64_v */
0x4000007, /* vluxei8_v */
0xb4002057, /* vmacc_vv */
0xb4006057, /* vmacc_vx */
0x46003057, /* vmadc_vi */
0x44003057, /* vmadc_vim */
0x46000057, /* vmadc_vv */
0x44000057, /* vmadc_vvm */
0x46004057, /* vmadc_vx */
0x44004057, /* vmadc_vxm */
0xa4002057, /* vmadd_vv */
0xa4006057, /* vmadd_vx */
0x66002057, /* vmand_mm */
0x62002057, /* vmandn_mm */
0x60002057, /* vmandnot_mm */
0x1c000057, /* vmax_vv */
0x1c004057, /* vmax_vx */
0x18000057, /* vmaxu_vv */
0x18004057, /* vmaxu_vx */
0x5c003057, /* vmerge_vim */
0x5c000057, /* vmerge_vvm */
0x5c004057, /* vmerge_vxm */
0x60005057, /* vmfeq_vf */
0x60001057, /* vmfeq_vv */
0x7c005057, /* vmfge_vf */
0x74005057, /* vmfgt_vf */
0x64005057, /* vmfle_vf */
0x64001057, /* vmfle_vv */
0x6c005057, /* vmflt_vf */
0x6c001057, /* vmflt_vv */
0x70005057, /* vmfne_vf */
0x70001057, /* vmfne_vv */
0x14000057, /* vmin_vv */
0x14004057, /* vmin_vx */
0x10000057, /* vminu_vv */
0x10004057, /* vminu_vx */
0x76002057, /* vmnand_mm */
0x7a002057, /* vmnor_mm */
0x6a002057, /* vmor_mm */
0x72002057, /* vmorn_mm */
0x70002057, /* vmornot_mm */
0x4e000057, /* vmsbc_vv */
0x4c000057, /* vmsbc_vvm */
0x4e004057, /* vmsbc_vx */
0x4c004057, /* vmsbc_vxm */
0x5000a057, /* vmsbf_m */
0x60003057, /* vmseq_vi */
0x60000057, /* vmseq_vv */
0x60004057, /* vmseq_vx */
0x7c003057, /* vmsgt_vi */
0x7c004057, /* vmsgt_vx */
0x78003057, /* vmsgtu_vi */
0x78004057, /* vmsgtu_vx */
0x5001a057, /* vmsif_m */
0x74003057, /* vmsle_vi */
0x74000057, /* vmsle_vv */
0x74004057, /* vmsle_vx */
0x70003057, /* vmsleu_vi */
0x70000057, /* vmsleu_vv */
0x70004057, /* vmsleu_vx */
0x6c000057, /* vmslt_vv */
0x6c004057, /* vmslt_vx */
0x68000057, /* vmsltu_vv */
0x68004057, /* vmsltu_vx */
0x64003057, /* vmsne_vi */
0x64000057, /* vmsne_vv */
0x64004057, /* vmsne_vx */
0x50012057, /* vmsof_m */
0x94002057, /* vmul_vv */
0x94006057, /* vmul_vx */
0x9c002057, /* vmulh_vv */
0x9c006057, /* vmulh_vx */
0x98002057, /* vmulhsu_vv */
0x98006057, /* vmulhsu_vx */
0x90002057, /* vmulhu_vv */
0x90006057, /* vmulhu_vx */
0x9e003057, /* vmv1r_v */
0x9e00b057, /* vmv2r_v */
0x9e01b057, /* vmv4r_v */
0x9e03b057, /* vmv8r_v */
0x42006057, /* vmv_s_x */
0x5e003057, /* vmv_v_i */
0x5e000057, /* vmv_v_v */
0x5e004057, /* vmv_v_x */
0x42002057, /* vmv_x_s */
0x7e002057, /* vmxnor_mm */
0x6e002057, /* vmxor_mm */
0xbc003057, /* vnclip_wi */
0xbc000057, /* vnclip_wv */
0xbc004057, /* vnclip_wx */
0xb8003057, /* vnclipu_wi */
0xb8000057, /* vnclipu_wv */
0xb8004057, /* vnclipu_wx */
0xbc002057, /* vnmsac_vv */
0xbc006057, /* vnmsac_vx */
0xac002057, /* vnmsub_vv */
0xac006057, /* vnmsub_vx */
0xb4003057, /* vnsra_wi */
0xb4000057, /* vnsra_wv */
0xb4004057, /* vnsra_wx */
0xb0003057, /* vnsrl_wi */
0xb0000057, /* vnsrl_wv */
0xb0004057, /* vnsrl_wx */
0x28003057, /* vor_vi */
0x28000057, /* vor_vv */
0x28004057, /* vor_vx */
0x40082057, /* vpopc_m */
0x4002057, /* vredand_vs */
0x1c002057, /* vredmax_vs */
0x18002057, /* vredmaxu_vs */
0x14002057, /* vredmin_vs */
0x10002057, /* vredminu_vs */
0x8002057, /* vredor_vs */
0x2057, /* vredsum_vs */
0xc002057, /* vredxor_vs */
0x8c002057, /* vrem_vv */
0x8c006057, /* vrem_vx */
0x88002057, /* vremu_vv */
0x88006057, /* vremu_vx */
0x4804a057, /* vrev8_v */
0x30003057, /* vrgather_vi */
0x30000057, /* vrgather_vv */
0x30004057, /* vrgather_vx */
0x38000057, /* vrgatherei16_vv */
0x54000057, /* vrol_vv */
0x54004057, /* vrol_vx */
0x50003057, /* vror_vi */
0x50000057, /* vror_vv */
0x50004057, /* vror_vx */
0xc003057, /* vrsub_vi */
0xc004057, /* vrsub_vx */
0x2800027, /* vs1r_v */
0x22800027, /* vs2r_v */
0x62800027, /* vs4r_v */
0xe2800027, /* vs8r_v */
0x84003057, /* vsadd_vi */
0x84000057, /* vsadd_vv */
0x84004057, /* vsadd_vx */
0x80003057, /* vsaddu_vi */
0x80000057, /* vsaddu_vv */
0x80004057, /* vsaddu_vx */
0x48000057, /* vsbc_vvm */
0x48004057, /* vsbc_vxm */
0x5027, /* vse16_v */
0x2b00027, /* vse1_v */
0x6027, /* vse32_v */
0x7027, /* vse64_v */
0x27, /* vse8_v */
0xc0007057, /* vsetivli */
0x80007057, /* vsetvl */
0x7057, /* vsetvli */
0x4803a057, /* vsext_vf2 */
0x4802a057, /* vsext_vf4 */
0x4801a057, /* vsext_vf8 */
0xba002077, /* vsha2ch_vv */
0xbe002077, /* vsha2cl_vv */
0xb6002077, /* vsha2ms_vv */
0x3c006057, /* vslide1down_vx */
0x38006057, /* vslide1up_vx */
0x3c003057, /* vslidedown_vi */
0x3c004057, /* vslidedown_vx */
0x38003057, /* vslideup_vi */
0x38004057, /* vslideup_vx */
0x94003057, /* vsll_vi */
0x94000057, /* vsll_vv */
0x94004057, /* vsll_vx */
0xae002077, /* vsm3c_vi */
0x82002077, /* vsm3me_vv */
0x86002077, /* vsm4k_vi */
0xa6082077, /* vsm4r_vs */
0xa2082077, /* vsm4r_vv */
0x2b00027, /* vsm_v */
0x9c000057, /* vsmul_vv */
0x9c004057, /* vsmul_vx */
0xc005027, /* vsoxei16_v */
0xc006027, /* vsoxei32_v */
0xc007027, /* vsoxei64_v */
0xc000027, /* vsoxei8_v */
0xa4003057, /* vsra_vi */
0xa4000057, /* vsra_vv */
0xa4004057, /* vsra_vx */
0xa0003057, /* vsrl_vi */
0xa0000057, /* vsrl_vv */
0xa0004057, /* vsrl_vx */
0x8005027, /* vsse16_v */
0x8006027, /* vsse32_v */
0x8007027, /* vsse64_v */
0x8000027, /* vsse8_v */
0xac003057, /* vssra_vi */
0xac000057, /* vssra_vv */
0xac004057, /* vssra_vx */
0xa8003057, /* vssrl_vi */
0xa8000057, /* vssrl_vv */
0xa8004057, /* vssrl_vx */
0x8c000057, /* vssub_vv */
0x8c004057, /* vssub_vx */
0x88000057, /* vssubu_vv */
0x88004057, /* vssubu_vx */
0x8000057, /* vsub_vv */
0x8004057, /* vsub_vx */
0x4005027, /* vsuxei16_v */
0x4006027, /* vsuxei32_v */
0x4007027, /* vsuxei64_v */
0x4000027, /* vsuxei8_v */
0xc4002057, /* vwadd_vv */
0xc4006057, /* vwadd_vx */
0xd4002057, /* vwadd_wv */
0xd4006057, /* vwadd_wx */
0xc0002057, /* vwaddu_vv */
0xc0006057, /* vwaddu_vx */
0xd0002057, /* vwaddu_wv */
0xd0006057, /* vwaddu_wx */
0xf4002057, /* vwmacc_vv */
0xf4006057, /* vwmacc_vx */
0xfc002057, /* vwmaccsu_vv */
0xfc006057, /* vwmaccsu_vx */
0xf0002057, /* vwmaccu_vv */
0xf0006057, /* vwmaccu_vx */
0xf8006057, /* vwmaccus_vx */
0xec002057, /* vwmul_vv */
0xec006057, /* vwmul_vx */
0xe8002057, /* vwmulsu_vv */
0xe8006057, /* vwmulsu_vx */
0xe0002057, /* vwmulu_vv */
0xe0006057, /* vwmulu_vx */
0xc4000057, /* vwredsum_vs */
0xc0000057, /* vwredsumu_vs */
0xd4003057, /* vwsll_vi */
0xd4000057, /* vwsll_vv */
0xd4004057, /* vwsll_vx */
0xcc002057, /* vwsub_vv */
0xcc006057, /* vwsub_vx */
0xdc002057, /* vwsub_wv */
0xdc006057, /* vwsub_wx */
0xc8002057, /* vwsubu_vv */
0xc8006057, /* vwsubu_vx */
0xd8002057, /* vwsubu_wv */
0xd8006057, /* vwsubu_wx */
0x2c003057, /* vxor_vi */
0x2c000057, /* vxor_vv */
0x2c004057, /* vxor_vx */
0x48032057, /* vzext_vf2 */
0x48022057, /* vzext_vf4 */
0x48012057, /* vzext_vf8 */
0x10500073, /* wfi */
0xd00073, /* wrs_nto */
0x1d00073, /* wrs_sto */
0x40004033, /* xnor */
0x4033, /* xor */
0x4013, /* xori */
0x28002033, /* xperm4 */
0x28004033, /* xperm8 */
0x7013, /* zext_b */
0xffff_ffff,/* zext_h */
0x8004033, /* zext_h_rv32 */
0xffff_ffff,/* zext_w */
0x8f01013, /* zip */
];
pub static OPCODE32_MASK: [u32; 1021] = [
0xfe00707f, /* add */
0xffff_ffff, /* add_uw */
0x707f, /* addi */
0xffff_ffff, /* addiw */
0xffff_ffff, /* addw */
0x3e00707f, /* aes32dsi */
0x3e00707f, /* aes32dsmi */
0x3e00707f, /* aes32esi */
0x3e00707f, /* aes32esmi */
0xffff_ffff, /* aes64ds */
0xffff_ffff, /* aes64dsm */
0xffff_ffff, /* aes64es */
0xffff_ffff, /* aes64esm */
0xffff_ffff, /* aes64im */
0xffff_ffff, /* aes64ks1i */
0xffff_ffff, /* aes64ks2 */
0xf800707f, /* amoadd_b */
0xffff_ffff, /* amoadd_d */
0xf800707f, /* amoadd_h */
0xf800707f, /* amoadd_w */
0xf800707f, /* amoand_b */
0xffff_ffff, /* amoand_d */
0xf800707f, /* amoand_h */
0xf800707f, /* amoand_w */
0xf800707f, /* amocas_b */
0xf800707f, /* amocas_d */
0xf800707f, /* amocas_h */
0xffff_ffff, /* amocas_q */
0xf800707f, /* amocas_w */
0xf800707f, /* amomax_b */
0xffff_ffff, /* amomax_d */
0xf800707f, /* amomax_h */
0xf800707f, /* amomax_w */
0xf800707f, /* amomaxu_b */
0xffff_ffff, /* amomaxu_d */
0xf800707f, /* amomaxu_h */
0xf800707f, /* amomaxu_w */
0xf800707f, /* amomin_b */
0xffff_ffff, /* amomin_d */
0xf800707f, /* amomin_h */
0xf800707f, /* amomin_w */
0xf800707f, /* amominu_b */
0xffff_ffff, /* amominu_d */
0xf800707f, /* amominu_h */
0xf800707f, /* amominu_w */
0xf800707f, /* amoor_b */
0xffff_ffff, /* amoor_d */
0xf800707f, /* amoor_h */
0xf800707f, /* amoor_w */
0xf800707f, /* amoswap_b */
0xffff_ffff, /* amoswap_d */
0xf800707f, /* amoswap_h */
0xf800707f, /* amoswap_w */
0xf800707f, /* amoxor_b */
0xffff_ffff, /* amoxor_d */
0xf800707f, /* amoxor_h */
0xf800707f, /* amoxor_w */
0xfe00707f, /* and */
0x707f, /* andi */
0xfe00707f, /* andn */
0x7f, /* auipc */
0xfe00707f, /* bclr */
0xffff_ffff, /* bclri */
0xfe00707f, /* bclri_rv32 */
0x707f, /* beq */
0x1f0707f, /* beqz */
0xfe00707f, /* bext */
0xffff_ffff, /* bexti */
0xfe00707f, /* bexti_rv32 */
0x707f, /* bge */
0x707f, /* bgeu */
0x1f0707f, /* bgez */
0x707f, /* bgt */
0x707f, /* bgtu */
0xff07f, /* bgtz */
0xfe00707f, /* binv */
0xffff_ffff, /* binvi */
0xfe00707f, /* binvi_rv32 */
0x707f, /* ble */
0x707f, /* bleu */
0xff07f, /* blez */
0x707f, /* blt */
0x707f, /* bltu */
0x1f0707f, /* bltz */
0x707f, /* bne */
0x1f0707f, /* bnez */
0xfff0707f, /* brev8 */
0xfe00707f, /* bset */
0xffff_ffff, /* bseti */
0xfe00707f, /* bseti_rv32 */
0xf003, /* c_add */
0xe003, /* c_addi */
0xef83, /* c_addi16sp */
0xe003, /* c_addi4spn */
0xffff_ffff, /* c_addiw */
0xffff_ffff, /* c_addw */
0xfc63, /* c_and */
0xec03, /* c_andi */
0xe003, /* c_beqz */
0xe003, /* c_bnez */
0xffff, /* c_ebreak */
0xe003, /* c_fld */
0xe003, /* c_fldsp */
0xe003, /* c_flw */
0xe003, /* c_flwsp */
0xe003, /* c_fsd */
0xe003, /* c_fsdsp */
0xe003, /* c_fsw */
0xe003, /* c_fswsp */
0xe003, /* c_j */
0xe003, /* c_jal */
0xf07f, /* c_jalr */
0xf07f, /* c_jr */
0xfc03, /* c_lbu */
0xffff_ffff, /* c_ld */
0xffff_ffff, /* c_ldsp */
0xfc43, /* c_lh */
0xfc43, /* c_lhu */
0xe003, /* c_li */
0xe003, /* c_lui */
0xe003, /* c_lw */
0xe003, /* c_lwsp */
0xffff, /* c_mop_1 */
0xffff, /* c_mop_11 */
0xffff, /* c_mop_13 */
0xffff, /* c_mop_15 */
0xffff, /* c_mop_3 */
0xffff, /* c_mop_5 */
0xffff, /* c_mop_7 */
0xffff, /* c_mop_9 */
0xf8ff, /* c_mop_N */
0xfc63, /* c_mul */
0xf003, /* c_mv */
0xef83, /* c_nop */
0xfc7f, /* c_not */
0xffff, /* c_ntl_all */
0xffff, /* c_ntl_p1 */
0xffff, /* c_ntl_pall */
0xffff, /* c_ntl_s1 */
0xfc63, /* c_or */
0xfc03, /* c_sb */
0xffff_ffff, /* c_sd */
0xffff_ffff, /* c_sdsp */
0xfc7f, /* c_sext_b */
0xfc7f, /* c_sext_h */
0xffff_ffff, /* c_sext_w */
0xfc43, /* c_sh */
0xe003, /* c_slli */
0xf003, /* c_slli_rv32 */
0xec03, /* c_srai */
0xfc03, /* c_srai_rv32 */
0xec03, /* c_srli */
0xfc03, /* c_srli_rv32 */
0xfc63, /* c_sub */
0xffff_ffff, /* c_subw */
0xe003, /* c_sw */
0xe003, /* c_swsp */
0xfc63, /* c_xor */
0xfc7f, /* c_zext_b */
0xfc7f, /* c_zext_h */
0xffff_ffff, /* c_zext_w */
0xfff07fff, /* cbo_clean */
0xfff07fff, /* cbo_flush */
0xfff07fff, /* cbo_inval */
0xfff07fff, /* cbo_zero */
0xfe00707f, /* clmul */
0xfe00707f, /* clmulh */
0xfe00707f, /* clmulr */
0xfff0707f, /* clz */
0xffff_ffff, /* clzw */
0xfc03, /* cm_jalt */
0xfc63, /* cm_mva01s */
0xfc63, /* cm_mvsa01 */
0xff03, /* cm_pop */
0xff03, /* cm_popret */
0xff03, /* cm_popretz */
0xff03, /* cm_push */
0xfff0707f, /* cpop */
0xffff_ffff, /* cpopw */
0x7fff, /* csrc */
0x7fff, /* csrci */
0xff07f, /* csrr */
0x707f, /* csrrc */
0x707f, /* csrrci */
0x707f, /* csrrs */
0x707f, /* csrrsi */
0x707f, /* csrrw */
0x707f, /* csrrwi */
0x7fff, /* csrs */
0x7fff, /* csrsi */
0x7fff, /* csrw */
0x7fff, /* csrwi */
0xfff0707f, /* ctz */
0xffff_ffff, /* ctzw */
0xfe00707f, /* czero_eqz */
0xfe00707f, /* czero_nez */
0xfe00707f, /* div */
0xfe00707f, /* divu */
0xffff_ffff, /* divuw */
0xffff_ffff, /* divw */
0xffffffff, /* dret */
0xffffffff, /* ebreak */
0xffffffff, /* ecall */
0xfe00707f, /* fabs_d */
0xfe00707f, /* fabs_h */
0xfe00707f, /* fabs_q */
0xfe00707f, /* fabs_s */
0xfe00007f, /* fadd_d */
0xfe00007f, /* fadd_h */
0xfe00007f, /* fadd_q */
0xfe00007f, /* fadd_s */
0xfff0707f, /* fclass_d */
0xfff0707f, /* fclass_h */
0xfff0707f, /* fclass_q */
0xfff0707f, /* fclass_s */
0xfff0007f, /* fcvt_d_h */
0xffff_ffff, /* fcvt_d_l */
0xffff_ffff, /* fcvt_d_lu */
0xfff0007f, /* fcvt_d_q */
0xfff0007f, /* fcvt_d_s */
0xfff0007f, /* fcvt_d_w */
0xfff0007f, /* fcvt_d_wu */
0xfff0007f, /* fcvt_h_d */
0xffff_ffff, /* fcvt_h_l */
0xffff_ffff, /* fcvt_h_lu */
0xfff0007f, /* fcvt_h_q */
0xfff0007f, /* fcvt_h_s */
0xfff0007f, /* fcvt_h_w */
0xfff0007f, /* fcvt_h_wu */
0xffff_ffff, /* fcvt_l_d */
0xffff_ffff, /* fcvt_l_h */
0xffff_ffff, /* fcvt_l_q */
0xffff_ffff, /* fcvt_l_s */
0xffff_ffff, /* fcvt_lu_d */
0xffff_ffff, /* fcvt_lu_h */
0xffff_ffff, /* fcvt_lu_q */
0xffff_ffff, /* fcvt_lu_s */
0xfff0007f, /* fcvt_q_d */
0xfff0007f, /* fcvt_q_h */
0xffff_ffff, /* fcvt_q_l */
0xffff_ffff, /* fcvt_q_lu */
0xfff0007f, /* fcvt_q_s */
0xfff0007f, /* fcvt_q_w */
0xfff0007f, /* fcvt_q_wu */
0xfff0007f, /* fcvt_s_d */
0xfff0007f, /* fcvt_s_h */
0xffff_ffff, /* fcvt_s_l */
0xffff_ffff, /* fcvt_s_lu */
0xfff0007f, /* fcvt_s_q */
0xfff0007f, /* fcvt_s_w */
0xfff0007f, /* fcvt_s_wu */
0xfff0007f, /* fcvt_w_d */
0xfff0007f, /* fcvt_w_h */
0xfff0007f, /* fcvt_w_q */
0xfff0007f, /* fcvt_w_s */
0xfff0007f, /* fcvt_wu_d */
0xfff0007f, /* fcvt_wu_h */
0xfff0007f, /* fcvt_wu_q */
0xfff0007f, /* fcvt_wu_s */
0xfff0707f, /* fcvtmod_w_d */
0xfe00007f, /* fdiv_d */
0xfe00007f, /* fdiv_h */
0xfe00007f, /* fdiv_q */
0xfe00007f, /* fdiv_s */
0x707f, /* fence */
0x707f, /* fence_i */
0xfff0707f, /* fence_tso */
0xfe00707f, /* feq_d */
0xfe00707f, /* feq_h */
0xfe00707f, /* feq_q */
0xfe00707f, /* feq_s */
0x707f, /* fld */
0xfe00707f, /* fle_d */
0xfe00707f, /* fle_h */
0xfe00707f, /* fle_q */
0xfe00707f, /* fle_s */
0xfe00707f, /* fleq_d */
0xfe00707f, /* fleq_h */
0xfe00707f, /* fleq_q */
0xfe00707f, /* fleq_s */
0x707f, /* flh */
0xfff0707f, /* fli_d */
0xfff0707f, /* fli_h */
0xfff0707f, /* fli_q */
0xfff0707f, /* fli_s */
0x707f, /* flq */
0xfe00707f, /* flt_d */
0xfe00707f, /* flt_h */
0xfe00707f, /* flt_q */
0xfe00707f, /* flt_s */
0xfe00707f, /* fltq_d */
0xfe00707f, /* fltq_h */
0xfe00707f, /* fltq_q */
0xfe00707f, /* fltq_s */
0x707f, /* flw */
0x600007f, /* fmadd_d */
0x600007f, /* fmadd_h */
0x600007f, /* fmadd_q */
0x600007f, /* fmadd_s */
0xfe00707f, /* fmax_d */
0xfe00707f, /* fmax_h */
0xfe00707f, /* fmax_q */
0xfe00707f, /* fmax_s */
0xfe00707f, /* fmaxm_d */
0xfe00707f, /* fmaxm_h */
0xfe00707f, /* fmaxm_q */
0xfe00707f, /* fmaxm_s */
0xfe00707f, /* fmin_d */
0xfe00707f, /* fmin_h */
0xfe00707f, /* fmin_q */
0xfe00707f, /* fmin_s */
0xfe00707f, /* fminm_d */
0xfe00707f, /* fminm_h */
0xfe00707f, /* fminm_q */
0xfe00707f, /* fminm_s */
0x600007f, /* fmsub_d */
0x600007f, /* fmsub_h */
0x600007f, /* fmsub_q */
0x600007f, /* fmsub_s */
0xfe00007f, /* fmul_d */
0xfe00007f, /* fmul_h */
0xfe00007f, /* fmul_q */
0xfe00007f, /* fmul_s */
0xfe00707f, /* fmv_d */
0xffff_ffff, /* fmv_d_x */
0xfe00707f, /* fmv_h */
0xfff0707f, /* fmv_h_x */
0xfe00707f, /* fmv_q */
0xfe00707f, /* fmv_s */
0xfff0707f, /* fmv_s_x */
0xfff0707f, /* fmv_w_x */
0xffff_ffff, /* fmv_x_d */
0xfff0707f, /* fmv_x_h */
0xfff0707f, /* fmv_x_s */
0xfff0707f, /* fmv_x_w */
0xfff0707f, /* fmvh_x_d */
0xffff_ffff, /* fmvh_x_q */
0xfe00707f, /* fmvp_d_x */
0xffff_ffff, /* fmvp_q_x */
0xfe00707f, /* fneg_d */
0xfe00707f, /* fneg_h */
0xfe00707f, /* fneg_q */
0xfe00707f, /* fneg_s */
0x600007f, /* fnmadd_d */
0x600007f, /* fnmadd_h */
0x600007f, /* fnmadd_q */
0x600007f, /* fnmadd_s */
0x600007f, /* fnmsub_d */
0x600007f, /* fnmsub_h */
0x600007f, /* fnmsub_q */
0x600007f, /* fnmsub_s */
0xfffff07f, /* frcsr */
0xfffff07f, /* frflags */
0xfff0007f, /* fround_d */
0xfff0007f, /* fround_h */
0xfff0007f, /* fround_q */
0xfff0007f, /* fround_s */
0xfff0007f, /* froundnx_d */
0xfff0007f, /* froundnx_h */
0xfff0007f, /* froundnx_q */
0xfff0007f, /* froundnx_s */
0xfffff07f, /* frrm */
0xfff0707f, /* fscsr */
0x707f, /* fsd */
0xfff0707f, /* fsflags */
0xfff0707f, /* fsflagsi */
0xfe00707f, /* fsgnj_d */
0xfe00707f, /* fsgnj_h */
0xfe00707f, /* fsgnj_q */
0xfe00707f, /* fsgnj_s */
0xfe00707f, /* fsgnjn_d */
0xfe00707f, /* fsgnjn_h */
0xfe00707f, /* fsgnjn_q */
0xfe00707f, /* fsgnjn_s */
0xfe00707f, /* fsgnjx_d */
0xfe00707f, /* fsgnjx_h */
0xfe00707f, /* fsgnjx_q */
0xfe00707f, /* fsgnjx_s */
0x707f, /* fsh */
0x707f, /* fsq */
0xfff0007f, /* fsqrt_d */
0xfff0007f, /* fsqrt_h */
0xfff0007f, /* fsqrt_q */
0xfff0007f, /* fsqrt_s */
0xfff0707f, /* fsrm */
0xfff0707f, /* fsrmi */
0xfe00007f, /* fsub_d */
0xfe00007f, /* fsub_h */
0xfe00007f, /* fsub_q */
0xfe00007f, /* fsub_s */
0x707f, /* fsw */
0xfe007fff, /* hfence_gvma */
0xfe007fff, /* hfence_vvma */
0xfe007fff, /* hinval_gvma */
0xfe007fff, /* hinval_vvma */
0xfff0707f, /* hlv_b */
0xfff0707f, /* hlv_bu */
0xffff_ffff, /* hlv_d */
0xfff0707f, /* hlv_h */
0xfff0707f, /* hlv_hu */
0xfff0707f, /* hlv_w */
0xffff_ffff, /* hlv_wu */
0xfff0707f, /* hlvx_hu */
0xfff0707f, /* hlvx_wu */
0xfe007fff, /* hsv_b */
0xffff_ffff, /* hsv_d */
0xfe007fff, /* hsv_h */
0xfe007fff, /* hsv_w */
0xfff, /* j */
0x7f, /* jal */
0xfff, /* jal_pseudo */
0x707f, /* jalr */
0xfff07fff, /* jalr_pseudo */
0xfff07fff, /* jr */
0x707f, /* lb */
0x707f, /* lbu */
0xffff_ffff, /* ld */
0x707f, /* lh */
0x707f, /* lhu */
0xffff_ffff, /* lr_d */
0xf9f0707f, /* lr_w */
0x7f, /* lui */
0x707f, /* lw */
0xffff_ffff, /* lwu */
0xfe00707f, /* max */
0xfe00707f, /* maxu */
0xfe00707f, /* min */
0xfe00707f, /* minu */
0xfff0707f, /* mop_r_0 */
0xfff0707f, /* mop_r_1 */
0xfff0707f, /* mop_r_10 */
0xfff0707f, /* mop_r_11 */
0xfff0707f, /* mop_r_12 */
0xfff0707f, /* mop_r_13 */
0xfff0707f, /* mop_r_14 */
0xfff0707f, /* mop_r_15 */
0xfff0707f, /* mop_r_16 */
0xfff0707f, /* mop_r_17 */
0xfff0707f, /* mop_r_18 */
0xfff0707f, /* mop_r_19 */
0xfff0707f, /* mop_r_2 */
0xfff0707f, /* mop_r_20 */
0xfff0707f, /* mop_r_21 */
0xfff0707f, /* mop_r_22 */
0xfff0707f, /* mop_r_23 */
0xfff0707f, /* mop_r_24 */
0xfff0707f, /* mop_r_25 */
0xfff0707f, /* mop_r_26 */
0xfff0707f, /* mop_r_27 */
0xfff0707f, /* mop_r_28 */
0xfff0707f, /* mop_r_29 */
0xfff0707f, /* mop_r_3 */
0xfff0707f, /* mop_r_30 */
0xfff0707f, /* mop_r_31 */
0xfff0707f, /* mop_r_4 */
0xfff0707f, /* mop_r_5 */
0xfff0707f, /* mop_r_6 */
0xfff0707f, /* mop_r_7 */
0xfff0707f, /* mop_r_8 */
0xfff0707f, /* mop_r_9 */
0xb3c0707f, /* mop_r_N */
0xfe00707f, /* mop_rr_0 */
0xfe00707f, /* mop_rr_1 */
0xfe00707f, /* mop_rr_2 */
0xfe00707f, /* mop_rr_3 */
0xfe00707f, /* mop_rr_4 */
0xfe00707f, /* mop_rr_5 */
0xfe00707f, /* mop_rr_6 */
0xfe00707f, /* mop_rr_7 */
0xb200707f, /* mop_rr_N */
0xffffffff, /* mret */
0xfe00707f, /* mul */
0xfe00707f, /* mulh */
0xfe00707f, /* mulhsu */
0xfe00707f, /* mulhu */
0xffff_ffff, /* mulw */
0xfff0707f, /* mv */
0xfff0707f, /* neg */
0xffffffff, /* nop */
0xffffffff, /* ntl_all */
0xffffffff, /* ntl_p1 */
0xffffffff, /* ntl_pall */
0xffffffff, /* ntl_s1 */
0xfe00707f, /* or */
0xfff0707f, /* orc_b */
0x707f, /* ori */
0xfe00707f, /* orn */
0xfe00707f, /* pack */
0xfe00707f, /* packh */
0xffff_ffff, /* packw */
0xffffffff, /* pause */
0x1f07fff, /* prefetch_i */
0x1f07fff, /* prefetch_r */
0x1f07fff, /* prefetch_w */
0xfffff07f, /* rdcycle */
0xfffff07f, /* rdcycleh */
0xfffff07f, /* rdinstret */
0xfffff07f, /* rdinstreth */
0xfffff07f, /* rdtime */
0xfffff07f, /* rdtimeh */
0xfe00707f, /* rem */
0xfe00707f, /* remu */
0xffff_ffff, /* remuw */
0xffff_ffff, /* remw */
0xffffffff, /* ret */
0xffff_ffff, /* rev8 */
0xfff0707f, /* rev8_rv32 */
0xfe00707f, /* rol */
0xffff_ffff, /* rolw */
0xfe00707f, /* ror */
0xffff_ffff, /* rori */
0xfe00707f, /* rori_rv32 */
0xffff_ffff, /* roriw */
0xffff_ffff, /* rorw */
0x707f, /* sb */
0xffffffff, /* sbreak */
0xffff_ffff, /* sc_d */
0xf800707f, /* sc_w */
0xffffffff, /* scall */
0xffff_ffff, /* sd */
0xfff0707f, /* seqz */
0xfff0707f, /* sext_b */
0xfff0707f, /* sext_h */
0xffff_ffff, /* sext_w */
0xffffffff, /* sfence_inval_ir */
0xfe007fff, /* sfence_vma */
0xffffffff, /* sfence_w_inval */
0xfe0ff07f, /* sgtz */
0x707f, /* sh */
0xfe00707f, /* sh1add */
0xffff_ffff, /* sh1add_uw */
0xfe00707f, /* sh2add */
0xffff_ffff, /* sh2add_uw */
0xfe00707f, /* sh3add */
0xffff_ffff, /* sh3add_uw */
0xfff0707f, /* sha256sig0 */
0xfff0707f, /* sha256sig1 */
0xfff0707f, /* sha256sum0 */
0xfff0707f, /* sha256sum1 */
0xffff_ffff, /* sha512sig0 */
0xfe00707f, /* sha512sig0h */
0xfe00707f, /* sha512sig0l */
0xffff_ffff, /* sha512sig1 */
0xfe00707f, /* sha512sig1h */
0xfe00707f, /* sha512sig1l */
0xffff_ffff, /* sha512sum0 */
0xfe00707f, /* sha512sum0r */
0xffff_ffff, /* sha512sum1 */
0xfe00707f, /* sha512sum1r */
0xfe007fff, /* sinval_vma */
0xfe00707f, /* sll */
0xfc00707f, /* slli */
0xfe00707f, /* slli_rv32 */
0xffff_ffff, /* slli_uw */
0xffff_ffff, /* slliw */
0xffff_ffff, /* sllw */
0xfe00707f, /* slt */
0x707f, /* slti */
0x707f, /* sltiu */
0xfe00707f, /* sltu */
0xfff0707f, /* sltz */
0xfff0707f, /* sm3p0 */
0xfff0707f, /* sm3p1 */
0x3e00707f, /* sm4ed */
0x3e00707f, /* sm4ks */
0xfe0ff07f, /* snez */
0xfe00707f, /* sra */
0xfc00707f, /* srai */
0xfe00707f, /* srai_rv32 */
0xffff_ffff, /* sraiw */
0xffff_ffff, /* sraw */
0xffffffff, /* sret */
0xfe00707f, /* srl */
0xfc00707f, /* srli */
0xfe00707f, /* srli_rv32 */
0xffff_ffff, /* srliw */
0xffff_ffff, /* srlw */
0xfe00707f, /* sub */
0xffff_ffff, /* subw */
0x707f, /* sw */
0xfff0707f, /* unzip */
0xfc00707f, /* vaadd_vv */
0xfc00707f, /* vaadd_vx */
0xfc00707f, /* vaaddu_vv */
0xfc00707f, /* vaaddu_vx */
0xfe00707f, /* vadc_vim */
0xfe00707f, /* vadc_vvm */
0xfe00707f, /* vadc_vxm */
0xfc00707f, /* vadd_vi */
0xfc00707f, /* vadd_vv */
0xfc00707f, /* vadd_vx */
0xfe0ff07f, /* vaesdf_vs */
0xfe0ff07f, /* vaesdf_vv */
0xfe0ff07f, /* vaesdm_vs */
0xfe0ff07f, /* vaesdm_vv */
0xfe0ff07f, /* vaesef_vs */
0xfe0ff07f, /* vaesef_vv */
0xfe0ff07f, /* vaesem_vs */
0xfe0ff07f, /* vaesem_vv */
0xfe00707f, /* vaeskf1_vi */
0xfe00707f, /* vaeskf2_vi */
0xfe0ff07f, /* vaesz_vs */
0xfc00707f, /* vand_vi */
0xfc00707f, /* vand_vv */
0xfc00707f, /* vand_vx */
0xfc00707f, /* vandn_vv */
0xfc00707f, /* vandn_vx */
0xfc00707f, /* vasub_vv */
0xfc00707f, /* vasub_vx */
0xfc00707f, /* vasubu_vv */
0xfc00707f, /* vasubu_vx */
0xfc0ff07f, /* vbrev8_v */
0xfc0ff07f, /* vbrev_v */
0xfc00707f, /* vclmul_vv */
0xfc00707f, /* vclmul_vx */
0xfc00707f, /* vclmulh_vv */
0xfc00707f, /* vclmulh_vx */
0xfc0ff07f, /* vclz_v */
0xfe00707f, /* vcompress_vm */
0xfc0ff07f, /* vcpop_m */
0xfc0ff07f, /* vcpop_v */
0xfc0ff07f, /* vctz_v */
0xfc00707f, /* vdiv_vv */
0xfc00707f, /* vdiv_vx */
0xfc00707f, /* vdivu_vv */
0xfc00707f, /* vdivu_vx */
0xfc00707f, /* vfadd_vf */
0xfc00707f, /* vfadd_vv */
0xfc0ff07f, /* vfclass_v */
0xfc0ff07f, /* vfcvt_f_x_v */
0xfc0ff07f, /* vfcvt_f_xu_v */
0xfc0ff07f, /* vfcvt_rtz_x_f_v */
0xfc0ff07f, /* vfcvt_rtz_xu_f_v */
0xfc0ff07f, /* vfcvt_x_f_v */
0xfc0ff07f, /* vfcvt_xu_f_v */
0xfc00707f, /* vfdiv_vf */
0xfc00707f, /* vfdiv_vv */
0xfc0ff07f, /* vfirst_m */
0xfc00707f, /* vfmacc_vf */
0xfc00707f, /* vfmacc_vv */
0xfc00707f, /* vfmadd_vf */
0xfc00707f, /* vfmadd_vv */
0xfc00707f, /* vfmax_vf */
0xfc00707f, /* vfmax_vv */
0xfe00707f, /* vfmerge_vfm */
0xfc00707f, /* vfmin_vf */
0xfc00707f, /* vfmin_vv */
0xfc00707f, /* vfmsac_vf */
0xfc00707f, /* vfmsac_vv */
0xfc00707f, /* vfmsub_vf */
0xfc00707f, /* vfmsub_vv */
0xfc00707f, /* vfmul_vf */
0xfc00707f, /* vfmul_vv */
0xfe0ff07f, /* vfmv_f_s */
0xfff0707f, /* vfmv_s_f */
0xfff0707f, /* vfmv_v_f */
0xfc0ff07f, /* vfncvt_f_f_w */
0xfc0ff07f, /* vfncvt_f_x_w */
0xfc0ff07f, /* vfncvt_f_xu_w */
0xfc0ff07f, /* vfncvt_rod_f_f_w */
0xfc0ff07f, /* vfncvt_rtz_x_f_w */
0xfc0ff07f, /* vfncvt_rtz_xu_f_w */
0xfc0ff07f, /* vfncvt_x_f_w */
0xfc0ff07f, /* vfncvt_xu_f_w */
0xfc00707f, /* vfnmacc_vf */
0xfc00707f, /* vfnmacc_vv */
0xfc00707f, /* vfnmadd_vf */
0xfc00707f, /* vfnmadd_vv */
0xfc00707f, /* vfnmsac_vf */
0xfc00707f, /* vfnmsac_vv */
0xfc00707f, /* vfnmsub_vf */
0xfc00707f, /* vfnmsub_vv */
0xfc00707f, /* vfrdiv_vf */
0xfc0ff07f, /* vfrec7_v */
0xfc00707f, /* vfredmax_vs */
0xfc00707f, /* vfredmin_vs */
0xfc00707f, /* vfredosum_vs */
0xfc00707f, /* vfredsum_vs */
0xfc00707f, /* vfredusum_vs */
0xfc0ff07f, /* vfrsqrt7_v */
0xfc00707f, /* vfrsub_vf */
0xfc00707f, /* vfsgnj_vf */
0xfc00707f, /* vfsgnj_vv */
0xfc00707f, /* vfsgnjn_vf */
0xfc00707f, /* vfsgnjn_vv */
0xfc00707f, /* vfsgnjx_vf */
0xfc00707f, /* vfsgnjx_vv */
0xfc00707f, /* vfslide1down_vf */
0xfc00707f, /* vfslide1up_vf */
0xfc0ff07f, /* vfsqrt_v */
0xfc00707f, /* vfsub_vf */
0xfc00707f, /* vfsub_vv */
0xfc00707f, /* vfwadd_vf */
0xfc00707f, /* vfwadd_vv */
0xfc00707f, /* vfwadd_wf */
0xfc00707f, /* vfwadd_wv */
0xfc0ff07f, /* vfwcvt_f_f_v */
0xfc0ff07f, /* vfwcvt_f_x_v */
0xfc0ff07f, /* vfwcvt_f_xu_v */
0xfc0ff07f, /* vfwcvt_rtz_x_f_v */
0xfc0ff07f, /* vfwcvt_rtz_xu_f_v */
0xfc0ff07f, /* vfwcvt_x_f_v */
0xfc0ff07f, /* vfwcvt_xu_f_v */
0xfc00707f, /* vfwmacc_vf */
0xfc00707f, /* vfwmacc_vv */
0xfc00707f, /* vfwmsac_vf */
0xfc00707f, /* vfwmsac_vv */
0xfc00707f, /* vfwmul_vf */
0xfc00707f, /* vfwmul_vv */
0xfc00707f, /* vfwnmacc_vf */
0xfc00707f, /* vfwnmacc_vv */
0xfc00707f, /* vfwnmsac_vf */
0xfc00707f, /* vfwnmsac_vv */
0xfc00707f, /* vfwredosum_vs */
0xfc00707f, /* vfwredsum_vs */
0xfc00707f, /* vfwredusum_vs */
0xfc00707f, /* vfwsub_vf */
0xfc00707f, /* vfwsub_vv */
0xfc00707f, /* vfwsub_wf */
0xfc00707f, /* vfwsub_wv */
0xfe00707f, /* vghsh_vv */
0xfe0ff07f, /* vgmul_vv */
0xfdfff07f, /* vid_v */
0xfc0ff07f, /* viota_m */
0xfff0707f, /* vl1r_v */
0xfff0707f, /* vl1re16_v */
0xfff0707f, /* vl1re32_v */
0xfff0707f, /* vl1re64_v */
0xfff0707f, /* vl1re8_v */
0xfff0707f, /* vl2r_v */
0xfff0707f, /* vl2re16_v */
0xfff0707f, /* vl2re32_v */
0xfff0707f, /* vl2re64_v */
0xfff0707f, /* vl2re8_v */
0xfff0707f, /* vl4r_v */
0xfff0707f, /* vl4re16_v */
0xfff0707f, /* vl4re32_v */
0xfff0707f, /* vl4re64_v */
0xfff0707f, /* vl4re8_v */
0xfff0707f, /* vl8r_v */
0xfff0707f, /* vl8re16_v */
0xfff0707f, /* vl8re32_v */
0xfff0707f, /* vl8re64_v */
0xfff0707f, /* vl8re8_v */
0x1df0707f, /* vle16_v */
0x1df0707f, /* vle16ff_v */
0xfff0707f, /* vle1_v */
0x1df0707f, /* vle32_v */
0x1df0707f, /* vle32ff_v */
0x1df0707f, /* vle64_v */
0x1df0707f, /* vle64ff_v */
0x1df0707f, /* vle8_v */
0x1df0707f, /* vle8ff_v */
0xfff0707f, /* vlm_v */
0x1c00707f, /* vloxei16_v */
0x1c00707f, /* vloxei32_v */
0x1c00707f, /* vloxei64_v */
0x1c00707f, /* vloxei8_v */
0x1c00707f, /* vlse16_v */
0x1c00707f, /* vlse32_v */
0x1c00707f, /* vlse64_v */
0x1c00707f, /* vlse8_v */
0x1c00707f, /* vluxei16_v */
0x1c00707f, /* vluxei32_v */
0x1c00707f, /* vluxei64_v */
0x1c00707f, /* vluxei8_v */
0xfc00707f, /* vmacc_vv */
0xfc00707f, /* vmacc_vx */
0xfe00707f, /* vmadc_vi */
0xfe00707f, /* vmadc_vim */
0xfe00707f, /* vmadc_vv */
0xfe00707f, /* vmadc_vvm */
0xfe00707f, /* vmadc_vx */
0xfe00707f, /* vmadc_vxm */
0xfc00707f, /* vmadd_vv */
0xfc00707f, /* vmadd_vx */
0xfe00707f, /* vmand_mm */
0xfe00707f, /* vmandn_mm */
0xfc00707f, /* vmandnot_mm */
0xfc00707f, /* vmax_vv */
0xfc00707f, /* vmax_vx */
0xfc00707f, /* vmaxu_vv */
0xfc00707f, /* vmaxu_vx */
0xfe00707f, /* vmerge_vim */
0xfe00707f, /* vmerge_vvm */
0xfe00707f, /* vmerge_vxm */
0xfc00707f, /* vmfeq_vf */
0xfc00707f, /* vmfeq_vv */
0xfc00707f, /* vmfge_vf */
0xfc00707f, /* vmfgt_vf */
0xfc00707f, /* vmfle_vf */
0xfc00707f, /* vmfle_vv */
0xfc00707f, /* vmflt_vf */
0xfc00707f, /* vmflt_vv */
0xfc00707f, /* vmfne_vf */
0xfc00707f, /* vmfne_vv */
0xfc00707f, /* vmin_vv */
0xfc00707f, /* vmin_vx */
0xfc00707f, /* vminu_vv */
0xfc00707f, /* vminu_vx */
0xfe00707f, /* vmnand_mm */
0xfe00707f, /* vmnor_mm */
0xfe00707f, /* vmor_mm */
0xfe00707f, /* vmorn_mm */
0xfc00707f, /* vmornot_mm */
0xfe00707f, /* vmsbc_vv */
0xfe00707f, /* vmsbc_vvm */
0xfe00707f, /* vmsbc_vx */
0xfe00707f, /* vmsbc_vxm */
0xfc0ff07f, /* vmsbf_m */
0xfc00707f, /* vmseq_vi */
0xfc00707f, /* vmseq_vv */
0xfc00707f, /* vmseq_vx */
0xfc00707f, /* vmsgt_vi */
0xfc00707f, /* vmsgt_vx */
0xfc00707f, /* vmsgtu_vi */
0xfc00707f, /* vmsgtu_vx */
0xfc0ff07f, /* vmsif_m */
0xfc00707f, /* vmsle_vi */
0xfc00707f, /* vmsle_vv */
0xfc00707f, /* vmsle_vx */
0xfc00707f, /* vmsleu_vi */
0xfc00707f, /* vmsleu_vv */
0xfc00707f, /* vmsleu_vx */
0xfc00707f, /* vmslt_vv */
0xfc00707f, /* vmslt_vx */
0xfc00707f, /* vmsltu_vv */
0xfc00707f, /* vmsltu_vx */
0xfc00707f, /* vmsne_vi */
0xfc00707f, /* vmsne_vv */
0xfc00707f, /* vmsne_vx */
0xfc0ff07f, /* vmsof_m */
0xfc00707f, /* vmul_vv */
0xfc00707f, /* vmul_vx */
0xfc00707f, /* vmulh_vv */
0xfc00707f, /* vmulh_vx */
0xfc00707f, /* vmulhsu_vv */
0xfc00707f, /* vmulhsu_vx */
0xfc00707f, /* vmulhu_vv */
0xfc00707f, /* vmulhu_vx */
0xfe0ff07f, /* vmv1r_v */
0xfe0ff07f, /* vmv2r_v */
0xfe0ff07f, /* vmv4r_v */
0xfe0ff07f, /* vmv8r_v */
0xfff0707f, /* vmv_s_x */
0xfff0707f, /* vmv_v_i */
0xfff0707f, /* vmv_v_v */
0xfff0707f, /* vmv_v_x */
0xfe0ff07f, /* vmv_x_s */
0xfe00707f, /* vmxnor_mm */
0xfe00707f, /* vmxor_mm */
0xfc00707f, /* vnclip_wi */
0xfc00707f, /* vnclip_wv */
0xfc00707f, /* vnclip_wx */
0xfc00707f, /* vnclipu_wi */
0xfc00707f, /* vnclipu_wv */
0xfc00707f, /* vnclipu_wx */
0xfc00707f, /* vnmsac_vv */
0xfc00707f, /* vnmsac_vx */
0xfc00707f, /* vnmsub_vv */
0xfc00707f, /* vnmsub_vx */
0xfc00707f, /* vnsra_wi */
0xfc00707f, /* vnsra_wv */
0xfc00707f, /* vnsra_wx */
0xfc00707f, /* vnsrl_wi */
0xfc00707f, /* vnsrl_wv */
0xfc00707f, /* vnsrl_wx */
0xfc00707f, /* vor_vi */
0xfc00707f, /* vor_vv */
0xfc00707f, /* vor_vx */
0xfc0ff07f, /* vpopc_m */
0xfc00707f, /* vredand_vs */
0xfc00707f, /* vredmax_vs */
0xfc00707f, /* vredmaxu_vs */
0xfc00707f, /* vredmin_vs */
0xfc00707f, /* vredminu_vs */
0xfc00707f, /* vredor_vs */
0xfc00707f, /* vredsum_vs */
0xfc00707f, /* vredxor_vs */
0xfc00707f, /* vrem_vv */
0xfc00707f, /* vrem_vx */
0xfc00707f, /* vremu_vv */
0xfc00707f, /* vremu_vx */
0xfc0ff07f, /* vrev8_v */
0xfc00707f, /* vrgather_vi */
0xfc00707f, /* vrgather_vv */
0xfc00707f, /* vrgather_vx */
0xfc00707f, /* vrgatherei16_vv */
0xfc00707f, /* vrol_vv */
0xfc00707f, /* vrol_vx */
0xf800707f, /* vror_vi */
0xfc00707f, /* vror_vv */
0xfc00707f, /* vror_vx */
0xfc00707f, /* vrsub_vi */
0xfc00707f, /* vrsub_vx */
0xfff0707f, /* vs1r_v */
0xfff0707f, /* vs2r_v */
0xfff0707f, /* vs4r_v */
0xfff0707f, /* vs8r_v */
0xfc00707f, /* vsadd_vi */
0xfc00707f, /* vsadd_vv */
0xfc00707f, /* vsadd_vx */
0xfc00707f, /* vsaddu_vi */
0xfc00707f, /* vsaddu_vv */
0xfc00707f, /* vsaddu_vx */
0xfe00707f, /* vsbc_vvm */
0xfe00707f, /* vsbc_vxm */
0x1df0707f, /* vse16_v */
0xfff0707f, /* vse1_v */
0x1df0707f, /* vse32_v */
0x1df0707f, /* vse64_v */
0x1df0707f, /* vse8_v */
0xc000707f, /* vsetivli */
0xfe00707f, /* vsetvl */
0x8000707f, /* vsetvli */
0xfc0ff07f, /* vsext_vf2 */
0xfc0ff07f, /* vsext_vf4 */
0xfc0ff07f, /* vsext_vf8 */
0xfe00707f, /* vsha2ch_vv */
0xfe00707f, /* vsha2cl_vv */
0xfe00707f, /* vsha2ms_vv */
0xfc00707f, /* vslide1down_vx */
0xfc00707f, /* vslide1up_vx */
0xfc00707f, /* vslidedown_vi */
0xfc00707f, /* vslidedown_vx */
0xfc00707f, /* vslideup_vi */
0xfc00707f, /* vslideup_vx */
0xfc00707f, /* vsll_vi */
0xfc00707f, /* vsll_vv */
0xfc00707f, /* vsll_vx */
0xfe00707f, /* vsm3c_vi */
0xfe00707f, /* vsm3me_vv */
0xfe00707f, /* vsm4k_vi */
0xfe0ff07f, /* vsm4r_vs */
0xfe0ff07f, /* vsm4r_vv */
0xfff0707f, /* vsm_v */
0xfc00707f, /* vsmul_vv */
0xfc00707f, /* vsmul_vx */
0x1c00707f, /* vsoxei16_v */
0x1c00707f, /* vsoxei32_v */
0x1c00707f, /* vsoxei64_v */
0x1c00707f, /* vsoxei8_v */
0xfc00707f, /* vsra_vi */
0xfc00707f, /* vsra_vv */
0xfc00707f, /* vsra_vx */
0xfc00707f, /* vsrl_vi */
0xfc00707f, /* vsrl_vv */
0xfc00707f, /* vsrl_vx */
0x1c00707f, /* vsse16_v */
0x1c00707f, /* vsse32_v */
0x1c00707f, /* vsse64_v */
0x1c00707f, /* vsse8_v */
0xfc00707f, /* vssra_vi */
0xfc00707f, /* vssra_vv */
0xfc00707f, /* vssra_vx */
0xfc00707f, /* vssrl_vi */
0xfc00707f, /* vssrl_vv */
0xfc00707f, /* vssrl_vx */
0xfc00707f, /* vssub_vv */
0xfc00707f, /* vssub_vx */
0xfc00707f, /* vssubu_vv */
0xfc00707f, /* vssubu_vx */
0xfc00707f, /* vsub_vv */
0xfc00707f, /* vsub_vx */
0x1c00707f, /* vsuxei16_v */
0x1c00707f, /* vsuxei32_v */
0x1c00707f, /* vsuxei64_v */
0x1c00707f, /* vsuxei8_v */
0xfc00707f, /* vwadd_vv */
0xfc00707f, /* vwadd_vx */
0xfc00707f, /* vwadd_wv */
0xfc00707f, /* vwadd_wx */
0xfc00707f, /* vwaddu_vv */
0xfc00707f, /* vwaddu_vx */
0xfc00707f, /* vwaddu_wv */
0xfc00707f, /* vwaddu_wx */
0xfc00707f, /* vwmacc_vv */
0xfc00707f, /* vwmacc_vx */
0xfc00707f, /* vwmaccsu_vv */
0xfc00707f, /* vwmaccsu_vx */
0xfc00707f, /* vwmaccu_vv */
0xfc00707f, /* vwmaccu_vx */
0xfc00707f, /* vwmaccus_vx */
0xfc00707f, /* vwmul_vv */
0xfc00707f, /* vwmul_vx */
0xfc00707f, /* vwmulsu_vv */
0xfc00707f, /* vwmulsu_vx */
0xfc00707f, /* vwmulu_vv */
0xfc00707f, /* vwmulu_vx */
0xfc00707f, /* vwredsum_vs */
0xfc00707f, /* vwredsumu_vs */
0xfc00707f, /* vwsll_vi */
0xfc00707f, /* vwsll_vv */
0xfc00707f, /* vwsll_vx */
0xfc00707f, /* vwsub_vv */
0xfc00707f, /* vwsub_vx */
0xfc00707f, /* vwsub_wv */
0xfc00707f, /* vwsub_wx */
0xfc00707f, /* vwsubu_vv */
0xfc00707f, /* vwsubu_vx */
0xfc00707f, /* vwsubu_wv */
0xfc00707f, /* vwsubu_wx */
0xfc00707f, /* vxor_vi */
0xfc00707f, /* vxor_vv */
0xfc00707f, /* vxor_vx */
0xfc0ff07f, /* vzext_vf2 */
0xfc0ff07f, /* vzext_vf4 */
0xfc0ff07f, /* vzext_vf8 */
0xffffffff, /* wfi */
0xffffffff, /* wrs_nto */
0xffffffff, /* wrs_sto */
0xfe00707f, /* xnor */
0xfe00707f, /* xor */
0x707f, /* xori */
0xfe00707f, /* xperm4 */
0xfe00707f, /* xperm8 */
0xfff0707f, /* zext_b */
0xffff_ffff, /* zext_h */
0xfff0707f, /* zext_h_rv32 */
0xffff_ffff, /* zext_w */
0xfff0707f, /* zip */
];
pub static OPCODE64_MATCH: [u32; 1021] = [
0x33, /* add */
0x800003b, /* add_uw */
0x13, /* addi */
0x1b, /* addiw */
0x3b, /* addw */
0xffff_ffff, /* aes32dsi */
0xffff_ffff, /* aes32dsmi */
0xffff_ffff, /* aes32esi */
0xffff_ffff, /* aes32esmi */
0x3a000033, /* aes64ds */
0x3e000033, /* aes64dsm */
0x32000033, /* aes64es */
0x36000033, /* aes64esm */
0x30001013, /* aes64im */
0x31001013, /* aes64ks1i */
0x7e000033, /* aes64ks2 */
0x2f, /* amoadd_b */
0x302f, /* amoadd_d */
0x102f, /* amoadd_h */
0x202f, /* amoadd_w */
0x6000002f, /* amoand_b */
0x6000302f, /* amoand_d */
0x6000102f, /* amoand_h */
0x6000202f, /* amoand_w */
0x2800002f, /* amocas_b */
0x2800302f, /* amocas_d */
0x2800102f, /* amocas_h */
0x2800402f, /* amocas_q */
0x2800202f, /* amocas_w */
0xa000002f, /* amomax_b */
0xa000302f, /* amomax_d */
0xa000102f, /* amomax_h */
0xa000202f, /* amomax_w */
0xe000002f, /* amomaxu_b */
0xe000302f, /* amomaxu_d */
0xe000102f, /* amomaxu_h */
0xe000202f, /* amomaxu_w */
0x8000002f, /* amomin_b */
0x8000302f, /* amomin_d */
0x8000102f, /* amomin_h */
0x8000202f, /* amomin_w */
0xc000002f, /* amominu_b */
0xc000302f, /* amominu_d */
0xc000102f, /* amominu_h */
0xc000202f, /* amominu_w */
0x4000002f, /* amoor_b */
0x4000302f, /* amoor_d */
0x4000102f, /* amoor_h */
0x4000202f, /* amoor_w */
0x800002f, /* amoswap_b */
0x800302f, /* amoswap_d */
0x800102f, /* amoswap_h */
0x800202f, /* amoswap_w */
0x2000002f, /* amoxor_b */
0x2000302f, /* amoxor_d */
0x2000102f, /* amoxor_h */
0x2000202f, /* amoxor_w */
0x7033, /* and */
0x7013, /* andi */
0x40007033, /* andn */
0x17, /* auipc */
0x48001033, /* bclr */
0x48001013, /* bclri */
0xffff_ffff, /* bclri_rv32 */
0x63, /* beq */
0x63, /* beqz */
0x48005033, /* bext */
0x48005013, /* bexti */
0xffff_ffff, /* bexti_rv32 */
0x5063, /* bge */
0x7063, /* bgeu */
0x5063, /* bgez */
0x4063, /* bgt */
0x6063, /* bgtu */
0x4063, /* bgtz */
0x68001033, /* binv */
0x68001013, /* binvi */
0xffff_ffff, /* binvi_rv32 */
0x5063, /* ble */
0x7063, /* bleu */
0x5063, /* blez */
0x4063, /* blt */
0x6063, /* bltu */
0x4063, /* bltz */
0x1063, /* bne */
0x1063, /* bnez */
0x68705013, /* brev8 */
0x28001033, /* bset */
0x28001013, /* bseti */
0xffff_ffff, /* bseti_rv32 */
0x9002, /* c_add */
0x1, /* c_addi */
0x6101, /* c_addi16sp */
0x0, /* c_addi4spn */
0x2001, /* c_addiw */
0x9c21, /* c_addw */
0x8c61, /* c_and */
0x8801, /* c_andi */
0xc001, /* c_beqz */
0xe001, /* c_bnez */
0x9002, /* c_ebreak */
0x2000, /* c_fld */
0x2002, /* c_fldsp */
0xffff_ffff, /* c_flw */
0xffff_ffff, /* c_flwsp */
0xa000, /* c_fsd */
0xa002, /* c_fsdsp */
0xffff_ffff, /* c_fsw */
0xffff_ffff, /* c_fswsp */
0xa001, /* c_j */
0xffff_ffff, /* c_jal */
0x9002, /* c_jalr */
0x8002, /* c_jr */
0x8000, /* c_lbu */
0x6000, /* c_ld */
0x6002, /* c_ldsp */
0x8440, /* c_lh */
0x8400, /* c_lhu */
0x4001, /* c_li */
0x6001, /* c_lui */
0x4000, /* c_lw */
0x4002, /* c_lwsp */
0x6081, /* c_mop_1 */
0x6581, /* c_mop_11 */
0x6681, /* c_mop_13 */
0x6781, /* c_mop_15 */
0x6181, /* c_mop_3 */
0x6281, /* c_mop_5 */
0x6381, /* c_mop_7 */
0x6481, /* c_mop_9 */
0x6081, /* c_mop_N */
0x9c41, /* c_mul */
0x8002, /* c_mv */
0x1, /* c_nop */
0x9c75, /* c_not */
0x9016, /* c_ntl_all */
0x900a, /* c_ntl_p1 */
0x900e, /* c_ntl_pall */
0x9012, /* c_ntl_s1 */
0x8c41, /* c_or */
0x8800, /* c_sb */
0xe000, /* c_sd */
0xe002, /* c_sdsp */
0x9c65, /* c_sext_b */
0x9c6d, /* c_sext_h */
0x2001, /* c_sext_w */
0x8c00, /* c_sh */
0x2, /* c_slli */
0xffff_ffff, /* c_slli_rv32 */
0x8401, /* c_srai */
0xffff_ffff, /* c_srai_rv32 */
0x8001, /* c_srli */
0xffff_ffff, /* c_srli_rv32 */
0x8c01, /* c_sub */
0x9c01, /* c_subw */
0xc000, /* c_sw */
0xc002, /* c_swsp */
0x8c21, /* c_xor */
0x9c61, /* c_zext_b */
0x9c69, /* c_zext_h */
0x9c71, /* c_zext_w */
0x10200f, /* cbo_clean */
0x20200f, /* cbo_flush */
0x200f, /* cbo_inval */
0x40200f, /* cbo_zero */
0xa001033, /* clmul */
0xa003033, /* clmulh */
0xa002033, /* clmulr */
0x60001013, /* clz */
0x6000101b, /* clzw */
0xa002, /* cm_jalt */
0xac62, /* cm_mva01s */
0xac22, /* cm_mvsa01 */
0xba02, /* cm_pop */
0xbe02, /* cm_popret */
0xbc02, /* cm_popretz */
0xb802, /* cm_push */
0x60201013, /* cpop */
0x6020101b, /* cpopw */
0x3073, /* csrc */
0x7073, /* csrci */
0x2073, /* csrr */
0x3073, /* csrrc */
0x7073, /* csrrci */
0x2073, /* csrrs */
0x6073, /* csrrsi */
0x1073, /* csrrw */
0x5073, /* csrrwi */
0x2073, /* csrs */
0x6073, /* csrsi */
0x1073, /* csrw */
0x5073, /* csrwi */
0x60101013, /* ctz */
0x6010101b, /* ctzw */
0xe005033, /* czero_eqz */
0xe007033, /* czero_nez */
0x2004033, /* div */
0x2005033, /* divu */
0x200503b, /* divuw */
0x200403b, /* divw */
0x7b200073, /* dret */
0x100073, /* ebreak */
0x73, /* ecall */
0x22002053, /* fabs_d */
0x24002053, /* fabs_h */
0x26002053, /* fabs_q */
0x20002053, /* fabs_s */
0x2000053, /* fadd_d */
0x4000053, /* fadd_h */
0x6000053, /* fadd_q */
0x53, /* fadd_s */
0xe2001053, /* fclass_d */
0xe4001053, /* fclass_h */
0xe6001053, /* fclass_q */
0xe0001053, /* fclass_s */
0x42200053, /* fcvt_d_h */
0xd2200053, /* fcvt_d_l */
0xd2300053, /* fcvt_d_lu */
0x42300053, /* fcvt_d_q */
0x42000053, /* fcvt_d_s */
0xd2000053, /* fcvt_d_w */
0xd2100053, /* fcvt_d_wu */
0x44100053, /* fcvt_h_d */
0xd4200053, /* fcvt_h_l */
0xd4300053, /* fcvt_h_lu */
0x44300053, /* fcvt_h_q */
0x44000053, /* fcvt_h_s */
0xd4000053, /* fcvt_h_w */
0xd4100053, /* fcvt_h_wu */
0xc2200053, /* fcvt_l_d */
0xc4200053, /* fcvt_l_h */
0xc6200053, /* fcvt_l_q */
0xc0200053, /* fcvt_l_s */
0xc2300053, /* fcvt_lu_d */
0xc4300053, /* fcvt_lu_h */
0xc6300053, /* fcvt_lu_q */
0xc0300053, /* fcvt_lu_s */
0x46100053, /* fcvt_q_d */
0x46200053, /* fcvt_q_h */
0xd6200053, /* fcvt_q_l */
0xd6300053, /* fcvt_q_lu */
0x46000053, /* fcvt_q_s */
0xd6000053, /* fcvt_q_w */
0xd6100053, /* fcvt_q_wu */
0x40100053, /* fcvt_s_d */
0x40200053, /* fcvt_s_h */
0xd0200053, /* fcvt_s_l */
0xd0300053, /* fcvt_s_lu */
0x40300053, /* fcvt_s_q */
0xd0000053, /* fcvt_s_w */
0xd0100053, /* fcvt_s_wu */
0xc2000053, /* fcvt_w_d */
0xc4000053, /* fcvt_w_h */
0xc6000053, /* fcvt_w_q */
0xc0000053, /* fcvt_w_s */
0xc2100053, /* fcvt_wu_d */
0xc4100053, /* fcvt_wu_h */
0xc6100053, /* fcvt_wu_q */
0xc0100053, /* fcvt_wu_s */
0xc2801053, /* fcvtmod_w_d */
0x1a000053, /* fdiv_d */
0x1c000053, /* fdiv_h */
0x1e000053, /* fdiv_q */
0x18000053, /* fdiv_s */
0xf, /* fence */
0x100f, /* fence_i */
0x8330000f, /* fence_tso */
0xa2002053, /* feq_d */
0xa4002053, /* feq_h */
0xa6002053, /* feq_q */
0xa0002053, /* feq_s */
0x3007, /* fld */
0xa2000053, /* fle_d */
0xa4000053, /* fle_h */
0xa6000053, /* fle_q */
0xa0000053, /* fle_s */
0xa2004053, /* fleq_d */
0xa4004053, /* fleq_h */
0xa6004053, /* fleq_q */
0xa0004053, /* fleq_s */
0x1007, /* flh */
0xf2100053, /* fli_d */
0xf4100053, /* fli_h */
0xf6100053, /* fli_q */
0xf0100053, /* fli_s */
0x4007, /* flq */
0xa2001053, /* flt_d */
0xa4001053, /* flt_h */
0xa6001053, /* flt_q */
0xa0001053, /* flt_s */
0xa2005053, /* fltq_d */
0xa4005053, /* fltq_h */
0xa6005053, /* fltq_q */
0xa0005053, /* fltq_s */
0x2007, /* flw */
0x2000043, /* fmadd_d */
0x4000043, /* fmadd_h */
0x6000043, /* fmadd_q */
0x43, /* fmadd_s */
0x2a001053, /* fmax_d */
0x2c001053, /* fmax_h */
0x2e001053, /* fmax_q */
0x28001053, /* fmax_s */
0x2a003053, /* fmaxm_d */
0x2c003053, /* fmaxm_h */
0x2e003053, /* fmaxm_q */
0x28003053, /* fmaxm_s */
0x2a000053, /* fmin_d */
0x2c000053, /* fmin_h */
0x2e000053, /* fmin_q */
0x28000053, /* fmin_s */
0x2a002053, /* fminm_d */
0x2c002053, /* fminm_h */
0x2e002053, /* fminm_q */
0x28002053, /* fminm_s */
0x2000047, /* fmsub_d */
0x4000047, /* fmsub_h */
0x6000047, /* fmsub_q */
0x47, /* fmsub_s */
0x12000053, /* fmul_d */
0x14000053, /* fmul_h */
0x16000053, /* fmul_q */
0x10000053, /* fmul_s */
0x22000053, /* fmv_d */
0xf2000053, /* fmv_d_x */
0x24000053, /* fmv_h */
0xf4000053, /* fmv_h_x */
0x26000053, /* fmv_q */
0x20000053, /* fmv_s */
0xf0000053, /* fmv_s_x */
0xf0000053, /* fmv_w_x */
0xe2000053, /* fmv_x_d */
0xe4000053, /* fmv_x_h */
0xe0000053, /* fmv_x_s */
0xe0000053, /* fmv_x_w */
0xffff_ffff, /* fmvh_x_d */
0xe6100053, /* fmvh_x_q */
0xffff_ffff, /* fmvp_d_x */
0xb6000053, /* fmvp_q_x */
0x22001053, /* fneg_d */
0x24001053, /* fneg_h */
0x26001053, /* fneg_q */
0x20001053, /* fneg_s */
0x200004f, /* fnmadd_d */
0x400004f, /* fnmadd_h */
0x600004f, /* fnmadd_q */
0x4f, /* fnmadd_s */
0x200004b, /* fnmsub_d */
0x400004b, /* fnmsub_h */
0x600004b, /* fnmsub_q */
0x4b, /* fnmsub_s */
0x302073, /* frcsr */
0x102073, /* frflags */
0x42400053, /* fround_d */
0x44400053, /* fround_h */
0x46400053, /* fround_q */
0x40400053, /* fround_s */
0x42500053, /* froundnx_d */
0x44500053, /* froundnx_h */
0x46500053, /* froundnx_q */
0x40500053, /* froundnx_s */
0x202073, /* frrm */
0x301073, /* fscsr */
0x3027, /* fsd */
0x101073, /* fsflags */
0x105073, /* fsflagsi */
0x22000053, /* fsgnj_d */
0x24000053, /* fsgnj_h */
0x26000053, /* fsgnj_q */
0x20000053, /* fsgnj_s */
0x22001053, /* fsgnjn_d */
0x24001053, /* fsgnjn_h */
0x26001053, /* fsgnjn_q */
0x20001053, /* fsgnjn_s */
0x22002053, /* fsgnjx_d */
0x24002053, /* fsgnjx_h */
0x26002053, /* fsgnjx_q */
0x20002053, /* fsgnjx_s */
0x1027, /* fsh */
0x4027, /* fsq */
0x5a000053, /* fsqrt_d */
0x5c000053, /* fsqrt_h */
0x5e000053, /* fsqrt_q */
0x58000053, /* fsqrt_s */
0x201073, /* fsrm */
0x205073, /* fsrmi */
0xa000053, /* fsub_d */
0xc000053, /* fsub_h */
0xe000053, /* fsub_q */
0x8000053, /* fsub_s */
0x2027, /* fsw */
0x62000073, /* hfence_gvma */
0x22000073, /* hfence_vvma */
0x66000073, /* hinval_gvma */
0x26000073, /* hinval_vvma */
0x60004073, /* hlv_b */
0x60104073, /* hlv_bu */
0x6c004073, /* hlv_d */
0x64004073, /* hlv_h */
0x64104073, /* hlv_hu */
0x68004073, /* hlv_w */
0x68104073, /* hlv_wu */
0x64304073, /* hlvx_hu */
0x68304073, /* hlvx_wu */
0x62004073, /* hsv_b */
0x6e004073, /* hsv_d */
0x66004073, /* hsv_h */
0x6a004073, /* hsv_w */
0x6f, /* j */
0x6f, /* jal */
0xef, /* jal_pseudo */
0x67, /* jalr */
0xe7, /* jalr_pseudo */
0x67, /* jr */
0x3, /* lb */
0x4003, /* lbu */
0x3003, /* ld */
0x1003, /* lh */
0x5003, /* lhu */
0x1000302f, /* lr_d */
0x1000202f, /* lr_w */
0x37, /* lui */
0x2003, /* lw */
0x6003, /* lwu */
0xa006033, /* max */
0xa007033, /* maxu */
0xa004033, /* min */
0xa005033, /* minu */
0x81c04073, /* mop_r_0 */
0x81d04073, /* mop_r_1 */
0x89e04073, /* mop_r_10 */
0x89f04073, /* mop_r_11 */
0x8dc04073, /* mop_r_12 */
0x8dd04073, /* mop_r_13 */
0x8de04073, /* mop_r_14 */
0x8df04073, /* mop_r_15 */
0xc1c04073, /* mop_r_16 */
0xc1d04073, /* mop_r_17 */
0xc1e04073, /* mop_r_18 */
0xc1f04073, /* mop_r_19 */
0x81e04073, /* mop_r_2 */
0xc5c04073, /* mop_r_20 */
0xc5d04073, /* mop_r_21 */
0xc5e04073, /* mop_r_22 */
0xc5f04073, /* mop_r_23 */
0xc9c04073, /* mop_r_24 */
0xc9d04073, /* mop_r_25 */
0xc9e04073, /* mop_r_26 */
0xc9f04073, /* mop_r_27 */
0xcdc04073, /* mop_r_28 */
0xcdd04073, /* mop_r_29 */
0x81f04073, /* mop_r_3 */
0xcde04073, /* mop_r_30 */
0xcdf04073, /* mop_r_31 */
0x85c04073, /* mop_r_4 */
0x85d04073, /* mop_r_5 */
0x85e04073, /* mop_r_6 */
0x85f04073, /* mop_r_7 */
0x89c04073, /* mop_r_8 */
0x89d04073, /* mop_r_9 */
0x81c04073, /* mop_r_N */
0x82004073, /* mop_rr_0 */
0x86004073, /* mop_rr_1 */
0x8a004073, /* mop_rr_2 */
0x8e004073, /* mop_rr_3 */
0xc2004073, /* mop_rr_4 */
0xc6004073, /* mop_rr_5 */
0xca004073, /* mop_rr_6 */
0xce004073, /* mop_rr_7 */
0x82004073, /* mop_rr_N */
0x30200073, /* mret */
0x2000033, /* mul */
0x2001033, /* mulh */
0x2002033, /* mulhsu */
0x2003033, /* mulhu */
0x200003b, /* mulw */
0x13, /* mv */
0x40000033, /* neg */
0x13, /* nop */
0x500033, /* ntl_all */
0x200033, /* ntl_p1 */
0x300033, /* ntl_pall */
0x400033, /* ntl_s1 */
0x6033, /* or */
0x28705013, /* orc_b */
0x6013, /* ori */
0x40006033, /* orn */
0x8004033, /* pack */
0x8007033, /* packh */
0x800403b, /* packw */
0x100000f, /* pause */
0x6013, /* prefetch_i */
0x106013, /* prefetch_r */
0x306013, /* prefetch_w */
0xc0002073, /* rdcycle */
0xffff_ffff, /* rdcycleh */
0xc0202073, /* rdinstret */
0xffff_ffff, /* rdinstreth */
0xc0102073, /* rdtime */
0xffff_ffff, /* rdtimeh */
0x2006033, /* rem */
0x2007033, /* remu */
0x200703b, /* remuw */
0x200603b, /* remw */
0x8067, /* ret */
0x6b805013, /* rev8 */
0xffff_ffff, /* rev8_rv32 */
0x60001033, /* rol */
0x6000103b, /* rolw */
0x60005033, /* ror */
0x60005013, /* rori */
0xffff_ffff, /* rori_rv32 */
0x6000501b, /* roriw */
0x6000503b, /* rorw */
0x23, /* sb */
0x100073, /* sbreak */
0x1800302f, /* sc_d */
0x1800202f, /* sc_w */
0x73, /* scall */
0x3023, /* sd */
0x103013, /* seqz */
0x60401013, /* sext_b */
0x60501013, /* sext_h */
0x1b, /* sext_w */
0x18100073, /* sfence_inval_ir */
0x12000073, /* sfence_vma */
0x18000073, /* sfence_w_inval */
0x2033, /* sgtz */
0x1023, /* sh */
0x20002033, /* sh1add */
0x2000203b, /* sh1add_uw */
0x20004033, /* sh2add */
0x2000403b, /* sh2add_uw */
0x20006033, /* sh3add */
0x2000603b, /* sh3add_uw */
0x10201013, /* sha256sig0 */
0x10301013, /* sha256sig1 */
0x10001013, /* sha256sum0 */
0x10101013, /* sha256sum1 */
0x10601013, /* sha512sig0 */
0xffff_ffff, /* sha512sig0h */
0xffff_ffff, /* sha512sig0l */
0x10701013, /* sha512sig1 */
0xffff_ffff, /* sha512sig1h */
0xffff_ffff, /* sha512sig1l */
0x10401013, /* sha512sum0 */
0xffff_ffff, /* sha512sum0r */
0x10501013, /* sha512sum1 */
0xffff_ffff, /* sha512sum1r */
0x16000073, /* sinval_vma */
0x1033, /* sll */
0x1013, /* slli */
0xffff_ffff, /* slli_rv32 */
0x800101b, /* slli_uw */
0x101b, /* slliw */
0x103b, /* sllw */
0x2033, /* slt */
0x2013, /* slti */
0x3013, /* sltiu */
0x3033, /* sltu */
0x2033, /* sltz */
0x10801013, /* sm3p0 */
0x10901013, /* sm3p1 */
0x30000033, /* sm4ed */
0x34000033, /* sm4ks */
0x3033, /* snez */
0x40005033, /* sra */
0x40005013, /* srai */
0xffff_ffff, /* srai_rv32 */
0x4000501b, /* sraiw */
0x4000503b, /* sraw */
0x10200073, /* sret */
0x5033, /* srl */
0x5013, /* srli */
0xffff_ffff, /* srli_rv32 */
0x501b, /* srliw */
0x503b, /* srlw */
0x40000033, /* sub */
0x4000003b, /* subw */
0x2023, /* sw */
0xffff_ffff, /* unzip */
0x24002057, /* vaadd_vv */
0x24006057, /* vaadd_vx */
0x20002057, /* vaaddu_vv */
0x20006057, /* vaaddu_vx */
0x40003057, /* vadc_vim */
0x40000057, /* vadc_vvm */
0x40004057, /* vadc_vxm */
0x3057, /* vadd_vi */
0x57, /* vadd_vv */
0x4057, /* vadd_vx */
0xa600a077, /* vaesdf_vs */
0xa200a077, /* vaesdf_vv */
0xa6002077, /* vaesdm_vs */
0xa2002077, /* vaesdm_vv */
0xa601a077, /* vaesef_vs */
0xa201a077, /* vaesef_vv */
0xa6012077, /* vaesem_vs */
0xa2012077, /* vaesem_vv */
0x8a002077, /* vaeskf1_vi */
0xaa002077, /* vaeskf2_vi */
0xa603a077, /* vaesz_vs */
0x24003057, /* vand_vi */
0x24000057, /* vand_vv */
0x24004057, /* vand_vx */
0x4000057, /* vandn_vv */
0x4004057, /* vandn_vx */
0x2c002057, /* vasub_vv */
0x2c006057, /* vasub_vx */
0x28002057, /* vasubu_vv */
0x28006057, /* vasubu_vx */
0x48042057, /* vbrev8_v */
0x48052057, /* vbrev_v */
0x30002057, /* vclmul_vv */
0x30006057, /* vclmul_vx */
0x34002057, /* vclmulh_vv */
0x34006057, /* vclmulh_vx */
0x48062057, /* vclz_v */
0x5e002057, /* vcompress_vm */
0x40082057, /* vcpop_m */
0x48072057, /* vcpop_v */
0x4806a057, /* vctz_v */
0x84002057, /* vdiv_vv */
0x84006057, /* vdiv_vx */
0x80002057, /* vdivu_vv */
0x80006057, /* vdivu_vx */
0x5057, /* vfadd_vf */
0x1057, /* vfadd_vv */
0x4c081057, /* vfclass_v */
0x48019057, /* vfcvt_f_x_v */
0x48011057, /* vfcvt_f_xu_v */
0x48039057, /* vfcvt_rtz_x_f_v */
0x48031057, /* vfcvt_rtz_xu_f_v */
0x48009057, /* vfcvt_x_f_v */
0x48001057, /* vfcvt_xu_f_v */
0x80005057, /* vfdiv_vf */
0x80001057, /* vfdiv_vv */
0x4008a057, /* vfirst_m */
0xb0005057, /* vfmacc_vf */
0xb0001057, /* vfmacc_vv */
0xa0005057, /* vfmadd_vf */
0xa0001057, /* vfmadd_vv */
0x18005057, /* vfmax_vf */
0x18001057, /* vfmax_vv */
0x5c005057, /* vfmerge_vfm */
0x10005057, /* vfmin_vf */
0x10001057, /* vfmin_vv */
0xb8005057, /* vfmsac_vf */
0xb8001057, /* vfmsac_vv */
0xa8005057, /* vfmsub_vf */
0xa8001057, /* vfmsub_vv */
0x90005057, /* vfmul_vf */
0x90001057, /* vfmul_vv */
0x42001057, /* vfmv_f_s */
0x42005057, /* vfmv_s_f */
0x5e005057, /* vfmv_v_f */
0x480a1057, /* vfncvt_f_f_w */
0x48099057, /* vfncvt_f_x_w */
0x48091057, /* vfncvt_f_xu_w */
0x480a9057, /* vfncvt_rod_f_f_w */
0x480b9057, /* vfncvt_rtz_x_f_w */
0x480b1057, /* vfncvt_rtz_xu_f_w */
0x48089057, /* vfncvt_x_f_w */
0x48081057, /* vfncvt_xu_f_w */
0xb4005057, /* vfnmacc_vf */
0xb4001057, /* vfnmacc_vv */
0xa4005057, /* vfnmadd_vf */
0xa4001057, /* vfnmadd_vv */
0xbc005057, /* vfnmsac_vf */
0xbc001057, /* vfnmsac_vv */
0xac005057, /* vfnmsub_vf */
0xac001057, /* vfnmsub_vv */
0x84005057, /* vfrdiv_vf */
0x4c029057, /* vfrec7_v */
0x1c001057, /* vfredmax_vs */
0x14001057, /* vfredmin_vs */
0xc001057, /* vfredosum_vs */
0x4001057, /* vfredsum_vs */
0x4001057, /* vfredusum_vs */
0x4c021057, /* vfrsqrt7_v */
0x9c005057, /* vfrsub_vf */
0x20005057, /* vfsgnj_vf */
0x20001057, /* vfsgnj_vv */
0x24005057, /* vfsgnjn_vf */
0x24001057, /* vfsgnjn_vv */
0x28005057, /* vfsgnjx_vf */
0x28001057, /* vfsgnjx_vv */
0x3c005057, /* vfslide1down_vf */
0x38005057, /* vfslide1up_vf */
0x4c001057, /* vfsqrt_v */
0x8005057, /* vfsub_vf */
0x8001057, /* vfsub_vv */
0xc0005057, /* vfwadd_vf */
0xc0001057, /* vfwadd_vv */
0xd0005057, /* vfwadd_wf */
0xd0001057, /* vfwadd_wv */
0x48061057, /* vfwcvt_f_f_v */
0x48059057, /* vfwcvt_f_x_v */
0x48051057, /* vfwcvt_f_xu_v */
0x48079057, /* vfwcvt_rtz_x_f_v */
0x48071057, /* vfwcvt_rtz_xu_f_v */
0x48049057, /* vfwcvt_x_f_v */
0x48041057, /* vfwcvt_xu_f_v */
0xf0005057, /* vfwmacc_vf */
0xf0001057, /* vfwmacc_vv */
0xf8005057, /* vfwmsac_vf */
0xf8001057, /* vfwmsac_vv */
0xe0005057, /* vfwmul_vf */
0xe0001057, /* vfwmul_vv */
0xf4005057, /* vfwnmacc_vf */
0xf4001057, /* vfwnmacc_vv */
0xfc005057, /* vfwnmsac_vf */
0xfc001057, /* vfwnmsac_vv */
0xcc001057, /* vfwredosum_vs */
0xc4001057, /* vfwredsum_vs */
0xc4001057, /* vfwredusum_vs */
0xc8005057, /* vfwsub_vf */
0xc8001057, /* vfwsub_vv */
0xd8005057, /* vfwsub_wf */
0xd8001057, /* vfwsub_wv */
0xb2002077, /* vghsh_vv */
0xa208a077, /* vgmul_vv */
0x5008a057, /* vid_v */
0x50082057, /* viota_m */
0x2800007, /* vl1r_v */
0x2805007, /* vl1re16_v */
0x2806007, /* vl1re32_v */
0x2807007, /* vl1re64_v */
0x2800007, /* vl1re8_v */
0x22800007, /* vl2r_v */
0x22805007, /* vl2re16_v */
0x22806007, /* vl2re32_v */
0x22807007, /* vl2re64_v */
0x22800007, /* vl2re8_v */
0x62800007, /* vl4r_v */
0x62805007, /* vl4re16_v */
0x62806007, /* vl4re32_v */
0x62807007, /* vl4re64_v */
0x62800007, /* vl4re8_v */
0xe2800007, /* vl8r_v */
0xe2805007, /* vl8re16_v */
0xe2806007, /* vl8re32_v */
0xe2807007, /* vl8re64_v */
0xe2800007, /* vl8re8_v */
0x5007, /* vle16_v */
0x1005007, /* vle16ff_v */
0x2b00007, /* vle1_v */
0x6007, /* vle32_v */
0x1006007, /* vle32ff_v */
0x7007, /* vle64_v */
0x1007007, /* vle64ff_v */
0x7, /* vle8_v */
0x1000007, /* vle8ff_v */
0x2b00007, /* vlm_v */
0xc005007, /* vloxei16_v */
0xc006007, /* vloxei32_v */
0xc007007, /* vloxei64_v */
0xc000007, /* vloxei8_v */
0x8005007, /* vlse16_v */
0x8006007, /* vlse32_v */
0x8007007, /* vlse64_v */
0x8000007, /* vlse8_v */
0x4005007, /* vluxei16_v */
0x4006007, /* vluxei32_v */
0x4007007, /* vluxei64_v */
0x4000007, /* vluxei8_v */
0xb4002057, /* vmacc_vv */
0xb4006057, /* vmacc_vx */
0x46003057, /* vmadc_vi */
0x44003057, /* vmadc_vim */
0x46000057, /* vmadc_vv */
0x44000057, /* vmadc_vvm */
0x46004057, /* vmadc_vx */
0x44004057, /* vmadc_vxm */
0xa4002057, /* vmadd_vv */
0xa4006057, /* vmadd_vx */
0x66002057, /* vmand_mm */
0x62002057, /* vmandn_mm */
0x60002057, /* vmandnot_mm */
0x1c000057, /* vmax_vv */
0x1c004057, /* vmax_vx */
0x18000057, /* vmaxu_vv */
0x18004057, /* vmaxu_vx */
0x5c003057, /* vmerge_vim */
0x5c000057, /* vmerge_vvm */
0x5c004057, /* vmerge_vxm */
0x60005057, /* vmfeq_vf */
0x60001057, /* vmfeq_vv */
0x7c005057, /* vmfge_vf */
0x74005057, /* vmfgt_vf */
0x64005057, /* vmfle_vf */
0x64001057, /* vmfle_vv */
0x6c005057, /* vmflt_vf */
0x6c001057, /* vmflt_vv */
0x70005057, /* vmfne_vf */
0x70001057, /* vmfne_vv */
0x14000057, /* vmin_vv */
0x14004057, /* vmin_vx */
0x10000057, /* vminu_vv */
0x10004057, /* vminu_vx */
0x76002057, /* vmnand_mm */
0x7a002057, /* vmnor_mm */
0x6a002057, /* vmor_mm */
0x72002057, /* vmorn_mm */
0x70002057, /* vmornot_mm */
0x4e000057, /* vmsbc_vv */
0x4c000057, /* vmsbc_vvm */
0x4e004057, /* vmsbc_vx */
0x4c004057, /* vmsbc_vxm */
0x5000a057, /* vmsbf_m */
0x60003057, /* vmseq_vi */
0x60000057, /* vmseq_vv */
0x60004057, /* vmseq_vx */
0x7c003057, /* vmsgt_vi */
0x7c004057, /* vmsgt_vx */
0x78003057, /* vmsgtu_vi */
0x78004057, /* vmsgtu_vx */
0x5001a057, /* vmsif_m */
0x74003057, /* vmsle_vi */
0x74000057, /* vmsle_vv */
0x74004057, /* vmsle_vx */
0x70003057, /* vmsleu_vi */
0x70000057, /* vmsleu_vv */
0x70004057, /* vmsleu_vx */
0x6c000057, /* vmslt_vv */
0x6c004057, /* vmslt_vx */
0x68000057, /* vmsltu_vv */
0x68004057, /* vmsltu_vx */
0x64003057, /* vmsne_vi */
0x64000057, /* vmsne_vv */
0x64004057, /* vmsne_vx */
0x50012057, /* vmsof_m */
0x94002057, /* vmul_vv */
0x94006057, /* vmul_vx */
0x9c002057, /* vmulh_vv */
0x9c006057, /* vmulh_vx */
0x98002057, /* vmulhsu_vv */
0x98006057, /* vmulhsu_vx */
0x90002057, /* vmulhu_vv */
0x90006057, /* vmulhu_vx */
0x9e003057, /* vmv1r_v */
0x9e00b057, /* vmv2r_v */
0x9e01b057, /* vmv4r_v */
0x9e03b057, /* vmv8r_v */
0x42006057, /* vmv_s_x */
0x5e003057, /* vmv_v_i */
0x5e000057, /* vmv_v_v */
0x5e004057, /* vmv_v_x */
0x42002057, /* vmv_x_s */
0x7e002057, /* vmxnor_mm */
0x6e002057, /* vmxor_mm */
0xbc003057, /* vnclip_wi */
0xbc000057, /* vnclip_wv */
0xbc004057, /* vnclip_wx */
0xb8003057, /* vnclipu_wi */
0xb8000057, /* vnclipu_wv */
0xb8004057, /* vnclipu_wx */
0xbc002057, /* vnmsac_vv */
0xbc006057, /* vnmsac_vx */
0xac002057, /* vnmsub_vv */
0xac006057, /* vnmsub_vx */
0xb4003057, /* vnsra_wi */
0xb4000057, /* vnsra_wv */
0xb4004057, /* vnsra_wx */
0xb0003057, /* vnsrl_wi */
0xb0000057, /* vnsrl_wv */
0xb0004057, /* vnsrl_wx */
0x28003057, /* vor_vi */
0x28000057, /* vor_vv */
0x28004057, /* vor_vx */
0x40082057, /* vpopc_m */
0x4002057, /* vredand_vs */
0x1c002057, /* vredmax_vs */
0x18002057, /* vredmaxu_vs */
0x14002057, /* vredmin_vs */
0x10002057, /* vredminu_vs */
0x8002057, /* vredor_vs */
0x2057, /* vredsum_vs */
0xc002057, /* vredxor_vs */
0x8c002057, /* vrem_vv */
0x8c006057, /* vrem_vx */
0x88002057, /* vremu_vv */
0x88006057, /* vremu_vx */
0x4804a057, /* vrev8_v */
0x30003057, /* vrgather_vi */
0x30000057, /* vrgather_vv */
0x30004057, /* vrgather_vx */
0x38000057, /* vrgatherei16_vv */
0x54000057, /* vrol_vv */
0x54004057, /* vrol_vx */
0x50003057, /* vror_vi */
0x50000057, /* vror_vv */
0x50004057, /* vror_vx */
0xc003057, /* vrsub_vi */
0xc004057, /* vrsub_vx */
0x2800027, /* vs1r_v */
0x22800027, /* vs2r_v */
0x62800027, /* vs4r_v */
0xe2800027, /* vs8r_v */
0x84003057, /* vsadd_vi */
0x84000057, /* vsadd_vv */
0x84004057, /* vsadd_vx */
0x80003057, /* vsaddu_vi */
0x80000057, /* vsaddu_vv */
0x80004057, /* vsaddu_vx */
0x48000057, /* vsbc_vvm */
0x48004057, /* vsbc_vxm */
0x5027, /* vse16_v */
0x2b00027, /* vse1_v */
0x6027, /* vse32_v */
0x7027, /* vse64_v */
0x27, /* vse8_v */
0xc0007057, /* vsetivli */
0x80007057, /* vsetvl */
0x7057, /* vsetvli */
0x4803a057, /* vsext_vf2 */
0x4802a057, /* vsext_vf4 */
0x4801a057, /* vsext_vf8 */
0xba002077, /* vsha2ch_vv */
0xbe002077, /* vsha2cl_vv */
0xb6002077, /* vsha2ms_vv */
0x3c006057, /* vslide1down_vx */
0x38006057, /* vslide1up_vx */
0x3c003057, /* vslidedown_vi */
0x3c004057, /* vslidedown_vx */
0x38003057, /* vslideup_vi */
0x38004057, /* vslideup_vx */
0x94003057, /* vsll_vi */
0x94000057, /* vsll_vv */
0x94004057, /* vsll_vx */
0xae002077, /* vsm3c_vi */
0x82002077, /* vsm3me_vv */
0x86002077, /* vsm4k_vi */
0xa6082077, /* vsm4r_vs */
0xa2082077, /* vsm4r_vv */
0x2b00027, /* vsm_v */
0x9c000057, /* vsmul_vv */
0x9c004057, /* vsmul_vx */
0xc005027, /* vsoxei16_v */
0xc006027, /* vsoxei32_v */
0xc007027, /* vsoxei64_v */
0xc000027, /* vsoxei8_v */
0xa4003057, /* vsra_vi */
0xa4000057, /* vsra_vv */
0xa4004057, /* vsra_vx */
0xa0003057, /* vsrl_vi */
0xa0000057, /* vsrl_vv */
0xa0004057, /* vsrl_vx */
0x8005027, /* vsse16_v */
0x8006027, /* vsse32_v */
0x8007027, /* vsse64_v */
0x8000027, /* vsse8_v */
0xac003057, /* vssra_vi */
0xac000057, /* vssra_vv */
0xac004057, /* vssra_vx */
0xa8003057, /* vssrl_vi */
0xa8000057, /* vssrl_vv */
0xa8004057, /* vssrl_vx */
0x8c000057, /* vssub_vv */
0x8c004057, /* vssub_vx */
0x88000057, /* vssubu_vv */
0x88004057, /* vssubu_vx */
0x8000057, /* vsub_vv */
0x8004057, /* vsub_vx */
0x4005027, /* vsuxei16_v */
0x4006027, /* vsuxei32_v */
0x4007027, /* vsuxei64_v */
0x4000027, /* vsuxei8_v */
0xc4002057, /* vwadd_vv */
0xc4006057, /* vwadd_vx */
0xd4002057, /* vwadd_wv */
0xd4006057, /* vwadd_wx */
0xc0002057, /* vwaddu_vv */
0xc0006057, /* vwaddu_vx */
0xd0002057, /* vwaddu_wv */
0xd0006057, /* vwaddu_wx */
0xf4002057, /* vwmacc_vv */
0xf4006057, /* vwmacc_vx */
0xfc002057, /* vwmaccsu_vv */
0xfc006057, /* vwmaccsu_vx */
0xf0002057, /* vwmaccu_vv */
0xf0006057, /* vwmaccu_vx */
0xf8006057, /* vwmaccus_vx */
0xec002057, /* vwmul_vv */
0xec006057, /* vwmul_vx */
0xe8002057, /* vwmulsu_vv */
0xe8006057, /* vwmulsu_vx */
0xe0002057, /* vwmulu_vv */
0xe0006057, /* vwmulu_vx */
0xc4000057, /* vwredsum_vs */
0xc0000057, /* vwredsumu_vs */
0xd4003057, /* vwsll_vi */
0xd4000057, /* vwsll_vv */
0xd4004057, /* vwsll_vx */
0xcc002057, /* vwsub_vv */
0xcc006057, /* vwsub_vx */
0xdc002057, /* vwsub_wv */
0xdc006057, /* vwsub_wx */
0xc8002057, /* vwsubu_vv */
0xc8006057, /* vwsubu_vx */
0xd8002057, /* vwsubu_wv */
0xd8006057, /* vwsubu_wx */
0x2c003057, /* vxor_vi */
0x2c000057, /* vxor_vv */
0x2c004057, /* vxor_vx */
0x48032057, /* vzext_vf2 */
0x48022057, /* vzext_vf4 */
0x48012057, /* vzext_vf8 */
0x10500073, /* wfi */
0xd00073, /* wrs_nto */
0x1d00073, /* wrs_sto */
0x40004033, /* xnor */
0x4033, /* xor */
0x4013, /* xori */
0x28002033, /* xperm4 */
0x28004033, /* xperm8 */
0x7013, /* zext_b */
0x800403b, /* zext_h */
0xffff_ffff, /* zext_h_rv32 */
0x800003b, /* zext_w */
0xffff_ffff, /* zip */
];
pub static OPCODE64_MASK: [u32; 1021] = [
0xfe00707f, /* add */
0xfe00707f, /* add_uw */
0x707f, /* addi */
0x707f, /* addiw */
0xfe00707f, /* addw */
0xffff_ffff, /* aes32dsi */
0xffff_ffff, /* aes32dsmi */
0xffff_ffff, /* aes32esi */
0xffff_ffff, /* aes32esmi */
0xfe00707f, /* aes64ds */
0xfe00707f, /* aes64dsm */
0xfe00707f, /* aes64es */
0xfe00707f, /* aes64esm */
0xfff0707f, /* aes64im */
0xff00707f, /* aes64ks1i */
0xfe00707f, /* aes64ks2 */
0xf800707f, /* amoadd_b */
0xf800707f, /* amoadd_d */
0xf800707f, /* amoadd_h */
0xf800707f, /* amoadd_w */
0xf800707f, /* amoand_b */
0xf800707f, /* amoand_d */
0xf800707f, /* amoand_h */
0xf800707f, /* amoand_w */
0xf800707f, /* amocas_b */
0xf800707f, /* amocas_d */
0xf800707f, /* amocas_h */
0xf800707f, /* amocas_q */
0xf800707f, /* amocas_w */
0xf800707f, /* amomax_b */
0xf800707f, /* amomax_d */
0xf800707f, /* amomax_h */
0xf800707f, /* amomax_w */
0xf800707f, /* amomaxu_b */
0xf800707f, /* amomaxu_d */
0xf800707f, /* amomaxu_h */
0xf800707f, /* amomaxu_w */
0xf800707f, /* amomin_b */
0xf800707f, /* amomin_d */
0xf800707f, /* amomin_h */
0xf800707f, /* amomin_w */
0xf800707f, /* amominu_b */
0xf800707f, /* amominu_d */
0xf800707f, /* amominu_h */
0xf800707f, /* amominu_w */
0xf800707f, /* amoor_b */
0xf800707f, /* amoor_d */
0xf800707f, /* amoor_h */
0xf800707f, /* amoor_w */
0xf800707f, /* amoswap_b */
0xf800707f, /* amoswap_d */
0xf800707f, /* amoswap_h */
0xf800707f, /* amoswap_w */
0xf800707f, /* amoxor_b */
0xf800707f, /* amoxor_d */
0xf800707f, /* amoxor_h */
0xf800707f, /* amoxor_w */
0xfe00707f, /* and */
0x707f, /* andi */
0xfe00707f, /* andn */
0x7f, /* auipc */
0xfe00707f, /* bclr */
0xfc00707f, /* bclri */
0xffff_ffff, /* bclri_rv32 */
0x707f, /* beq */
0x1f0707f, /* beqz */
0xfe00707f, /* bext */
0xfc00707f, /* bexti */
0xffff_ffff, /* bexti_rv32 */
0x707f, /* bge */
0x707f, /* bgeu */
0x1f0707f, /* bgez */
0x707f, /* bgt */
0x707f, /* bgtu */
0xff07f, /* bgtz */
0xfe00707f, /* binv */
0xfc00707f, /* binvi */
0xffff_ffff, /* binvi_rv32 */
0x707f, /* ble */
0x707f, /* bleu */
0xff07f, /* blez */
0x707f, /* blt */
0x707f, /* bltu */
0x1f0707f, /* bltz */
0x707f, /* bne */
0x1f0707f, /* bnez */
0xfff0707f, /* brev8 */
0xfe00707f, /* bset */
0xfc00707f, /* bseti */
0xffff_ffff, /* bseti_rv32 */
0xf003, /* c_add */
0xe003, /* c_addi */
0xef83, /* c_addi16sp */
0xe003, /* c_addi4spn */
0xe003, /* c_addiw */
0xfc63, /* c_addw */
0xfc63, /* c_and */
0xec03, /* c_andi */
0xe003, /* c_beqz */
0xe003, /* c_bnez */
0xffff, /* c_ebreak */
0xe003, /* c_fld */
0xe003, /* c_fldsp */
0xffff_ffff, /* c_flw */
0xffff_ffff, /* c_flwsp */
0xe003, /* c_fsd */
0xe003, /* c_fsdsp */
0xffff_ffff, /* c_fsw */
0xffff_ffff, /* c_fswsp */
0xe003, /* c_j */
0xffff_ffff, /* c_jal */
0xf07f, /* c_jalr */
0xf07f, /* c_jr */
0xfc03, /* c_lbu */
0xe003, /* c_ld */
0xe003, /* c_ldsp */
0xfc43, /* c_lh */
0xfc43, /* c_lhu */
0xe003, /* c_li */
0xe003, /* c_lui */
0xe003, /* c_lw */
0xe003, /* c_lwsp */
0xffff, /* c_mop_1 */
0xffff, /* c_mop_11 */
0xffff, /* c_mop_13 */
0xffff, /* c_mop_15 */
0xffff, /* c_mop_3 */
0xffff, /* c_mop_5 */
0xffff, /* c_mop_7 */
0xffff, /* c_mop_9 */
0xf8ff, /* c_mop_N */
0xfc63, /* c_mul */
0xf003, /* c_mv */
0xef83, /* c_nop */
0xfc7f, /* c_not */
0xffff, /* c_ntl_all */
0xffff, /* c_ntl_p1 */
0xffff, /* c_ntl_pall */
0xffff, /* c_ntl_s1 */
0xfc63, /* c_or */
0xfc03, /* c_sb */
0xe003, /* c_sd */
0xe003, /* c_sdsp */
0xfc7f, /* c_sext_b */
0xfc7f, /* c_sext_h */
0xf07f, /* c_sext_w */
0xfc43, /* c_sh */
0xe003, /* c_slli */
0xffff_ffff, /* c_slli_rv32 */
0xec03, /* c_srai */
0xffff_ffff, /* c_srai_rv32 */
0xec03, /* c_srli */
0xffff_ffff, /* c_srli_rv32 */
0xfc63, /* c_sub */
0xfc63, /* c_subw */
0xe003, /* c_sw */
0xe003, /* c_swsp */
0xfc63, /* c_xor */
0xfc7f, /* c_zext_b */
0xfc7f, /* c_zext_h */
0xfc7f, /* c_zext_w */
0xfff07fff, /* cbo_clean */
0xfff07fff, /* cbo_flush */
0xfff07fff, /* cbo_inval */
0xfff07fff, /* cbo_zero */
0xfe00707f, /* clmul */
0xfe00707f, /* clmulh */
0xfe00707f, /* clmulr */
0xfff0707f, /* clz */
0xfff0707f, /* clzw */
0xfc03, /* cm_jalt */
0xfc63, /* cm_mva01s */
0xfc63, /* cm_mvsa01 */
0xff03, /* cm_pop */
0xff03, /* cm_popret */
0xff03, /* cm_popretz */
0xff03, /* cm_push */
0xfff0707f, /* cpop */
0xfff0707f, /* cpopw */
0x7fff, /* csrc */
0x7fff, /* csrci */
0xff07f, /* csrr */
0x707f, /* csrrc */
0x707f, /* csrrci */
0x707f, /* csrrs */
0x707f, /* csrrsi */
0x707f, /* csrrw */
0x707f, /* csrrwi */
0x7fff, /* csrs */
0x7fff, /* csrsi */
0x7fff, /* csrw */
0x7fff, /* csrwi */
0xfff0707f, /* ctz */
0xfff0707f, /* ctzw */
0xfe00707f, /* czero_eqz */
0xfe00707f, /* czero_nez */
0xfe00707f, /* div */
0xfe00707f, /* divu */
0xfe00707f, /* divuw */
0xfe00707f, /* divw */
0xffffffff, /* dret */
0xffffffff, /* ebreak */
0xffffffff, /* ecall */
0xfe00707f, /* fabs_d */
0xfe00707f, /* fabs_h */
0xfe00707f, /* fabs_q */
0xfe00707f, /* fabs_s */
0xfe00007f, /* fadd_d */
0xfe00007f, /* fadd_h */
0xfe00007f, /* fadd_q */
0xfe00007f, /* fadd_s */
0xfff0707f, /* fclass_d */
0xfff0707f, /* fclass_h */
0xfff0707f, /* fclass_q */
0xfff0707f, /* fclass_s */
0xfff0007f, /* fcvt_d_h */
0xfff0007f, /* fcvt_d_l */
0xfff0007f, /* fcvt_d_lu */
0xfff0007f, /* fcvt_d_q */
0xfff0007f, /* fcvt_d_s */
0xfff0007f, /* fcvt_d_w */
0xfff0007f, /* fcvt_d_wu */
0xfff0007f, /* fcvt_h_d */
0xfff0007f, /* fcvt_h_l */
0xfff0007f, /* fcvt_h_lu */
0xfff0007f, /* fcvt_h_q */
0xfff0007f, /* fcvt_h_s */
0xfff0007f, /* fcvt_h_w */
0xfff0007f, /* fcvt_h_wu */
0xfff0007f, /* fcvt_l_d */
0xfff0007f, /* fcvt_l_h */
0xfff0007f, /* fcvt_l_q */
0xfff0007f, /* fcvt_l_s */
0xfff0007f, /* fcvt_lu_d */
0xfff0007f, /* fcvt_lu_h */
0xfff0007f, /* fcvt_lu_q */
0xfff0007f, /* fcvt_lu_s */
0xfff0007f, /* fcvt_q_d */
0xfff0007f, /* fcvt_q_h */
0xfff0007f, /* fcvt_q_l */
0xfff0007f, /* fcvt_q_lu */
0xfff0007f, /* fcvt_q_s */
0xfff0007f, /* fcvt_q_w */
0xfff0007f, /* fcvt_q_wu */
0xfff0007f, /* fcvt_s_d */
0xfff0007f, /* fcvt_s_h */
0xfff0007f, /* fcvt_s_l */
0xfff0007f, /* fcvt_s_lu */
0xfff0007f, /* fcvt_s_q */
0xfff0007f, /* fcvt_s_w */
0xfff0007f, /* fcvt_s_wu */
0xfff0007f, /* fcvt_w_d */
0xfff0007f, /* fcvt_w_h */
0xfff0007f, /* fcvt_w_q */
0xfff0007f, /* fcvt_w_s */
0xfff0007f, /* fcvt_wu_d */
0xfff0007f, /* fcvt_wu_h */
0xfff0007f, /* fcvt_wu_q */
0xfff0007f, /* fcvt_wu_s */
0xfff0707f, /* fcvtmod_w_d */
0xfe00007f, /* fdiv_d */
0xfe00007f, /* fdiv_h */
0xfe00007f, /* fdiv_q */
0xfe00007f, /* fdiv_s */
0x707f, /* fence */
0x707f, /* fence_i */
0xfff0707f, /* fence_tso */
0xfe00707f, /* feq_d */
0xfe00707f, /* feq_h */
0xfe00707f, /* feq_q */
0xfe00707f, /* feq_s */
0x707f, /* fld */
0xfe00707f, /* fle_d */
0xfe00707f, /* fle_h */
0xfe00707f, /* fle_q */
0xfe00707f, /* fle_s */
0xfe00707f, /* fleq_d */
0xfe00707f, /* fleq_h */
0xfe00707f, /* fleq_q */
0xfe00707f, /* fleq_s */
0x707f, /* flh */
0xfff0707f, /* fli_d */
0xfff0707f, /* fli_h */
0xfff0707f, /* fli_q */
0xfff0707f, /* fli_s */
0x707f, /* flq */
0xfe00707f, /* flt_d */
0xfe00707f, /* flt_h */
0xfe00707f, /* flt_q */
0xfe00707f, /* flt_s */
0xfe00707f, /* fltq_d */
0xfe00707f, /* fltq_h */
0xfe00707f, /* fltq_q */
0xfe00707f, /* fltq_s */
0x707f, /* flw */
0x600007f, /* fmadd_d */
0x600007f, /* fmadd_h */
0x600007f, /* fmadd_q */
0x600007f, /* fmadd_s */
0xfe00707f, /* fmax_d */
0xfe00707f, /* fmax_h */
0xfe00707f, /* fmax_q */
0xfe00707f, /* fmax_s */
0xfe00707f, /* fmaxm_d */
0xfe00707f, /* fmaxm_h */
0xfe00707f, /* fmaxm_q */
0xfe00707f, /* fmaxm_s */
0xfe00707f, /* fmin_d */
0xfe00707f, /* fmin_h */
0xfe00707f, /* fmin_q */
0xfe00707f, /* fmin_s */
0xfe00707f, /* fminm_d */
0xfe00707f, /* fminm_h */
0xfe00707f, /* fminm_q */
0xfe00707f, /* fminm_s */
0x600007f, /* fmsub_d */
0x600007f, /* fmsub_h */
0x600007f, /* fmsub_q */
0x600007f, /* fmsub_s */
0xfe00007f, /* fmul_d */
0xfe00007f, /* fmul_h */
0xfe00007f, /* fmul_q */
0xfe00007f, /* fmul_s */
0xfe00707f, /* fmv_d */
0xfff0707f, /* fmv_d_x */
0xfe00707f, /* fmv_h */
0xfff0707f, /* fmv_h_x */
0xfe00707f, /* fmv_q */
0xfe00707f, /* fmv_s */
0xfff0707f, /* fmv_s_x */
0xfff0707f, /* fmv_w_x */
0xfff0707f, /* fmv_x_d */
0xfff0707f, /* fmv_x_h */
0xfff0707f, /* fmv_x_s */
0xfff0707f, /* fmv_x_w */
0xffff_ffff, /* fmvh_x_d */
0xfff0707f, /* fmvh_x_q */
0xffff_ffff, /* fmvp_d_x */
0xfe00707f, /* fmvp_q_x */
0xfe00707f, /* fneg_d */
0xfe00707f, /* fneg_h */
0xfe00707f, /* fneg_q */
0xfe00707f, /* fneg_s */
0x600007f, /* fnmadd_d */
0x600007f, /* fnmadd_h */
0x600007f, /* fnmadd_q */
0x600007f, /* fnmadd_s */
0x600007f, /* fnmsub_d */
0x600007f, /* fnmsub_h */
0x600007f, /* fnmsub_q */
0x600007f, /* fnmsub_s */
0xfffff07f, /* frcsr */
0xfffff07f, /* frflags */
0xfff0007f, /* fround_d */
0xfff0007f, /* fround_h */
0xfff0007f, /* fround_q */
0xfff0007f, /* fround_s */
0xfff0007f, /* froundnx_d */
0xfff0007f, /* froundnx_h */
0xfff0007f, /* froundnx_q */
0xfff0007f, /* froundnx_s */
0xfffff07f, /* frrm */
0xfff0707f, /* fscsr */
0x707f, /* fsd */
0xfff0707f, /* fsflags */
0xfff0707f, /* fsflagsi */
0xfe00707f, /* fsgnj_d */
0xfe00707f, /* fsgnj_h */
0xfe00707f, /* fsgnj_q */
0xfe00707f, /* fsgnj_s */
0xfe00707f, /* fsgnjn_d */
0xfe00707f, /* fsgnjn_h */
0xfe00707f, /* fsgnjn_q */
0xfe00707f, /* fsgnjn_s */
0xfe00707f, /* fsgnjx_d */
0xfe00707f, /* fsgnjx_h */
0xfe00707f, /* fsgnjx_q */
0xfe00707f, /* fsgnjx_s */
0x707f, /* fsh */
0x707f, /* fsq */
0xfff0007f, /* fsqrt_d */
0xfff0007f, /* fsqrt_h */
0xfff0007f, /* fsqrt_q */
0xfff0007f, /* fsqrt_s */
0xfff0707f, /* fsrm */
0xfff0707f, /* fsrmi */
0xfe00007f, /* fsub_d */
0xfe00007f, /* fsub_h */
0xfe00007f, /* fsub_q */
0xfe00007f, /* fsub_s */
0x707f, /* fsw */
0xfe007fff, /* hfence_gvma */
0xfe007fff, /* hfence_vvma */
0xfe007fff, /* hinval_gvma */
0xfe007fff, /* hinval_vvma */
0xfff0707f, /* hlv_b */
0xfff0707f, /* hlv_bu */
0xfff0707f, /* hlv_d */
0xfff0707f, /* hlv_h */
0xfff0707f, /* hlv_hu */
0xfff0707f, /* hlv_w */
0xfff0707f, /* hlv_wu */
0xfff0707f, /* hlvx_hu */
0xfff0707f, /* hlvx_wu */
0xfe007fff, /* hsv_b */
0xfe007fff, /* hsv_d */
0xfe007fff, /* hsv_h */
0xfe007fff, /* hsv_w */
0xfff, /* j */
0x7f, /* jal */
0xfff, /* jal_pseudo */
0x707f, /* jalr */
0xfff07fff, /* jalr_pseudo */
0xfff07fff, /* jr */
0x707f, /* lb */
0x707f, /* lbu */
0x707f, /* ld */
0x707f, /* lh */
0x707f, /* lhu */
0xf9f0707f, /* lr_d */
0xf9f0707f, /* lr_w */
0x7f, /* lui */
0x707f, /* lw */
0x707f, /* lwu */
0xfe00707f, /* max */
0xfe00707f, /* maxu */
0xfe00707f, /* min */
0xfe00707f, /* minu */
0xfff0707f, /* mop_r_0 */
0xfff0707f, /* mop_r_1 */
0xfff0707f, /* mop_r_10 */
0xfff0707f, /* mop_r_11 */
0xfff0707f, /* mop_r_12 */
0xfff0707f, /* mop_r_13 */
0xfff0707f, /* mop_r_14 */
0xfff0707f, /* mop_r_15 */
0xfff0707f, /* mop_r_16 */
0xfff0707f, /* mop_r_17 */
0xfff0707f, /* mop_r_18 */
0xfff0707f, /* mop_r_19 */
0xfff0707f, /* mop_r_2 */
0xfff0707f, /* mop_r_20 */
0xfff0707f, /* mop_r_21 */
0xfff0707f, /* mop_r_22 */
0xfff0707f, /* mop_r_23 */
0xfff0707f, /* mop_r_24 */
0xfff0707f, /* mop_r_25 */
0xfff0707f, /* mop_r_26 */
0xfff0707f, /* mop_r_27 */
0xfff0707f, /* mop_r_28 */
0xfff0707f, /* mop_r_29 */
0xfff0707f, /* mop_r_3 */
0xfff0707f, /* mop_r_30 */
0xfff0707f, /* mop_r_31 */
0xfff0707f, /* mop_r_4 */
0xfff0707f, /* mop_r_5 */
0xfff0707f, /* mop_r_6 */
0xfff0707f, /* mop_r_7 */
0xfff0707f, /* mop_r_8 */
0xfff0707f, /* mop_r_9 */
0xb3c0707f, /* mop_r_N */
0xfe00707f, /* mop_rr_0 */
0xfe00707f, /* mop_rr_1 */
0xfe00707f, /* mop_rr_2 */
0xfe00707f, /* mop_rr_3 */
0xfe00707f, /* mop_rr_4 */
0xfe00707f, /* mop_rr_5 */
0xfe00707f, /* mop_rr_6 */
0xfe00707f, /* mop_rr_7 */
0xb200707f, /* mop_rr_N */
0xffffffff, /* mret */
0xfe00707f, /* mul */
0xfe00707f, /* mulh */
0xfe00707f, /* mulhsu */
0xfe00707f, /* mulhu */
0xfe00707f, /* mulw */
0xfff0707f, /* mv */
0xfff0707f, /* neg */
0xffffffff, /* nop */
0xffffffff, /* ntl_all */
0xffffffff, /* ntl_p1 */
0xffffffff, /* ntl_pall */
0xffffffff, /* ntl_s1 */
0xfe00707f, /* or */
0xfff0707f, /* orc_b */
0x707f, /* ori */
0xfe00707f, /* orn */
0xfe00707f, /* pack */
0xfe00707f, /* packh */
0xfe00707f, /* packw */
0xffffffff, /* pause */
0x1f07fff, /* prefetch_i */
0x1f07fff, /* prefetch_r */
0x1f07fff, /* prefetch_w */
0xfffff07f, /* rdcycle */
0xffff_ffff, /* rdcycleh */
0xfffff07f, /* rdinstret */
0xffff_ffff, /* rdinstreth */
0xfffff07f, /* rdtime */
0xffff_ffff, /* rdtimeh */
0xfe00707f, /* rem */
0xfe00707f, /* remu */
0xfe00707f, /* remuw */
0xfe00707f, /* remw */
0xffffffff, /* ret */
0xfff0707f, /* rev8 */
0xffff_ffff, /* rev8_rv32 */
0xfe00707f, /* rol */
0xfe00707f, /* rolw */
0xfe00707f, /* ror */
0xfc00707f, /* rori */
0xffff_ffff, /* rori_rv32 */
0xfe00707f, /* roriw */
0xfe00707f, /* rorw */
0x707f, /* sb */
0xffffffff, /* sbreak */
0xf800707f, /* sc_d */
0xf800707f, /* sc_w */
0xffffffff, /* scall */
0x707f, /* sd */
0xfff0707f, /* seqz */
0xfff0707f, /* sext_b */
0xfff0707f, /* sext_h */
0xfff0707f, /* sext_w */
0xffffffff, /* sfence_inval_ir */
0xfe007fff, /* sfence_vma */
0xffffffff, /* sfence_w_inval */
0xfe0ff07f, /* sgtz */
0x707f, /* sh */
0xfe00707f, /* sh1add */
0xfe00707f, /* sh1add_uw */
0xfe00707f, /* sh2add */
0xfe00707f, /* sh2add_uw */
0xfe00707f, /* sh3add */
0xfe00707f, /* sh3add_uw */
0xfff0707f, /* sha256sig0 */
0xfff0707f, /* sha256sig1 */
0xfff0707f, /* sha256sum0 */
0xfff0707f, /* sha256sum1 */
0xfff0707f, /* sha512sig0 */
0xffff_ffff, /* sha512sig0h */
0xffff_ffff, /* sha512sig0l */
0xfff0707f, /* sha512sig1 */
0xffff_ffff, /* sha512sig1h */
0xffff_ffff, /* sha512sig1l */
0xfff0707f, /* sha512sum0 */
0xffff_ffff, /* sha512sum0r */
0xfff0707f, /* sha512sum1 */
0xffff_ffff, /* sha512sum1r */
0xfe007fff, /* sinval_vma */
0xfe00707f, /* sll */
0xfc00707f, /* slli */
0xffff_ffff, /* slli_rv32 */
0xfc00707f, /* slli_uw */
0xfe00707f, /* slliw */
0xfe00707f, /* sllw */
0xfe00707f, /* slt */
0x707f, /* slti */
0x707f, /* sltiu */
0xfe00707f, /* sltu */
0xfff0707f, /* sltz */
0xfff0707f, /* sm3p0 */
0xfff0707f, /* sm3p1 */
0x3e00707f, /* sm4ed */
0x3e00707f, /* sm4ks */
0xfe0ff07f, /* snez */
0xfe00707f, /* sra */
0xfc00707f, /* srai */
0xffff_ffff, /* srai_rv32 */
0xfe00707f, /* sraiw */
0xfe00707f, /* sraw */
0xffffffff, /* sret */
0xfe00707f, /* srl */
0xfc00707f, /* srli */
0xffff_ffff, /* srli_rv32 */
0xfe00707f, /* srliw */
0xfe00707f, /* srlw */
0xfe00707f, /* sub */
0xfe00707f, /* subw */
0x707f, /* sw */
0xffff_ffff, /* unzip */
0xfc00707f, /* vaadd_vv */
0xfc00707f, /* vaadd_vx */
0xfc00707f, /* vaaddu_vv */
0xfc00707f, /* vaaddu_vx */
0xfe00707f, /* vadc_vim */
0xfe00707f, /* vadc_vvm */
0xfe00707f, /* vadc_vxm */
0xfc00707f, /* vadd_vi */
0xfc00707f, /* vadd_vv */
0xfc00707f, /* vadd_vx */
0xfe0ff07f, /* vaesdf_vs */
0xfe0ff07f, /* vaesdf_vv */
0xfe0ff07f, /* vaesdm_vs */
0xfe0ff07f, /* vaesdm_vv */
0xfe0ff07f, /* vaesef_vs */
0xfe0ff07f, /* vaesef_vv */
0xfe0ff07f, /* vaesem_vs */
0xfe0ff07f, /* vaesem_vv */
0xfe00707f, /* vaeskf1_vi */
0xfe00707f, /* vaeskf2_vi */
0xfe0ff07f, /* vaesz_vs */
0xfc00707f, /* vand_vi */
0xfc00707f, /* vand_vv */
0xfc00707f, /* vand_vx */
0xfc00707f, /* vandn_vv */
0xfc00707f, /* vandn_vx */
0xfc00707f, /* vasub_vv */
0xfc00707f, /* vasub_vx */
0xfc00707f, /* vasubu_vv */
0xfc00707f, /* vasubu_vx */
0xfc0ff07f, /* vbrev8_v */
0xfc0ff07f, /* vbrev_v */
0xfc00707f, /* vclmul_vv */
0xfc00707f, /* vclmul_vx */
0xfc00707f, /* vclmulh_vv */
0xfc00707f, /* vclmulh_vx */
0xfc0ff07f, /* vclz_v */
0xfe00707f, /* vcompress_vm */
0xfc0ff07f, /* vcpop_m */
0xfc0ff07f, /* vcpop_v */
0xfc0ff07f, /* vctz_v */
0xfc00707f, /* vdiv_vv */
0xfc00707f, /* vdiv_vx */
0xfc00707f, /* vdivu_vv */
0xfc00707f, /* vdivu_vx */
0xfc00707f, /* vfadd_vf */
0xfc00707f, /* vfadd_vv */
0xfc0ff07f, /* vfclass_v */
0xfc0ff07f, /* vfcvt_f_x_v */
0xfc0ff07f, /* vfcvt_f_xu_v */
0xfc0ff07f, /* vfcvt_rtz_x_f_v */
0xfc0ff07f, /* vfcvt_rtz_xu_f_v */
0xfc0ff07f, /* vfcvt_x_f_v */
0xfc0ff07f, /* vfcvt_xu_f_v */
0xfc00707f, /* vfdiv_vf */
0xfc00707f, /* vfdiv_vv */
0xfc0ff07f, /* vfirst_m */
0xfc00707f, /* vfmacc_vf */
0xfc00707f, /* vfmacc_vv */
0xfc00707f, /* vfmadd_vf */
0xfc00707f, /* vfmadd_vv */
0xfc00707f, /* vfmax_vf */
0xfc00707f, /* vfmax_vv */
0xfe00707f, /* vfmerge_vfm */
0xfc00707f, /* vfmin_vf */
0xfc00707f, /* vfmin_vv */
0xfc00707f, /* vfmsac_vf */
0xfc00707f, /* vfmsac_vv */
0xfc00707f, /* vfmsub_vf */
0xfc00707f, /* vfmsub_vv */
0xfc00707f, /* vfmul_vf */
0xfc00707f, /* vfmul_vv */
0xfe0ff07f, /* vfmv_f_s */
0xfff0707f, /* vfmv_s_f */
0xfff0707f, /* vfmv_v_f */
0xfc0ff07f, /* vfncvt_f_f_w */
0xfc0ff07f, /* vfncvt_f_x_w */
0xfc0ff07f, /* vfncvt_f_xu_w */
0xfc0ff07f, /* vfncvt_rod_f_f_w */
0xfc0ff07f, /* vfncvt_rtz_x_f_w */
0xfc0ff07f, /* vfncvt_rtz_xu_f_w */
0xfc0ff07f, /* vfncvt_x_f_w */
0xfc0ff07f, /* vfncvt_xu_f_w */
0xfc00707f, /* vfnmacc_vf */
0xfc00707f, /* vfnmacc_vv */
0xfc00707f, /* vfnmadd_vf */
0xfc00707f, /* vfnmadd_vv */
0xfc00707f, /* vfnmsac_vf */
0xfc00707f, /* vfnmsac_vv */
0xfc00707f, /* vfnmsub_vf */
0xfc00707f, /* vfnmsub_vv */
0xfc00707f, /* vfrdiv_vf */
0xfc0ff07f, /* vfrec7_v */
0xfc00707f, /* vfredmax_vs */
0xfc00707f, /* vfredmin_vs */
0xfc00707f, /* vfredosum_vs */
0xfc00707f, /* vfredsum_vs */
0xfc00707f, /* vfredusum_vs */
0xfc0ff07f, /* vfrsqrt7_v */
0xfc00707f, /* vfrsub_vf */
0xfc00707f, /* vfsgnj_vf */
0xfc00707f, /* vfsgnj_vv */
0xfc00707f, /* vfsgnjn_vf */
0xfc00707f, /* vfsgnjn_vv */
0xfc00707f, /* vfsgnjx_vf */
0xfc00707f, /* vfsgnjx_vv */
0xfc00707f, /* vfslide1down_vf */
0xfc00707f, /* vfslide1up_vf */
0xfc0ff07f, /* vfsqrt_v */
0xfc00707f, /* vfsub_vf */
0xfc00707f, /* vfsub_vv */
0xfc00707f, /* vfwadd_vf */
0xfc00707f, /* vfwadd_vv */
0xfc00707f, /* vfwadd_wf */
0xfc00707f, /* vfwadd_wv */
0xfc0ff07f, /* vfwcvt_f_f_v */
0xfc0ff07f, /* vfwcvt_f_x_v */
0xfc0ff07f, /* vfwcvt_f_xu_v */
0xfc0ff07f, /* vfwcvt_rtz_x_f_v */
0xfc0ff07f, /* vfwcvt_rtz_xu_f_v */
0xfc0ff07f, /* vfwcvt_x_f_v */
0xfc0ff07f, /* vfwcvt_xu_f_v */
0xfc00707f, /* vfwmacc_vf */
0xfc00707f, /* vfwmacc_vv */
0xfc00707f, /* vfwmsac_vf */
0xfc00707f, /* vfwmsac_vv */
0xfc00707f, /* vfwmul_vf */
0xfc00707f, /* vfwmul_vv */
0xfc00707f, /* vfwnmacc_vf */
0xfc00707f, /* vfwnmacc_vv */
0xfc00707f, /* vfwnmsac_vf */
0xfc00707f, /* vfwnmsac_vv */
0xfc00707f, /* vfwredosum_vs */
0xfc00707f, /* vfwredsum_vs */
0xfc00707f, /* vfwredusum_vs */
0xfc00707f, /* vfwsub_vf */
0xfc00707f, /* vfwsub_vv */
0xfc00707f, /* vfwsub_wf */
0xfc00707f, /* vfwsub_wv */
0xfe00707f, /* vghsh_vv */
0xfe0ff07f, /* vgmul_vv */
0xfdfff07f, /* vid_v */
0xfc0ff07f, /* viota_m */
0xfff0707f, /* vl1r_v */
0xfff0707f, /* vl1re16_v */
0xfff0707f, /* vl1re32_v */
0xfff0707f, /* vl1re64_v */
0xfff0707f, /* vl1re8_v */
0xfff0707f, /* vl2r_v */
0xfff0707f, /* vl2re16_v */
0xfff0707f, /* vl2re32_v */
0xfff0707f, /* vl2re64_v */
0xfff0707f, /* vl2re8_v */
0xfff0707f, /* vl4r_v */
0xfff0707f, /* vl4re16_v */
0xfff0707f, /* vl4re32_v */
0xfff0707f, /* vl4re64_v */
0xfff0707f, /* vl4re8_v */
0xfff0707f, /* vl8r_v */
0xfff0707f, /* vl8re16_v */
0xfff0707f, /* vl8re32_v */
0xfff0707f, /* vl8re64_v */
0xfff0707f, /* vl8re8_v */
0x1df0707f, /* vle16_v */
0x1df0707f, /* vle16ff_v */
0xfff0707f, /* vle1_v */
0x1df0707f, /* vle32_v */
0x1df0707f, /* vle32ff_v */
0x1df0707f, /* vle64_v */
0x1df0707f, /* vle64ff_v */
0x1df0707f, /* vle8_v */
0x1df0707f, /* vle8ff_v */
0xfff0707f, /* vlm_v */
0x1c00707f, /* vloxei16_v */
0x1c00707f, /* vloxei32_v */
0x1c00707f, /* vloxei64_v */
0x1c00707f, /* vloxei8_v */
0x1c00707f, /* vlse16_v */
0x1c00707f, /* vlse32_v */
0x1c00707f, /* vlse64_v */
0x1c00707f, /* vlse8_v */
0x1c00707f, /* vluxei16_v */
0x1c00707f, /* vluxei32_v */
0x1c00707f, /* vluxei64_v */
0x1c00707f, /* vluxei8_v */
0xfc00707f, /* vmacc_vv */
0xfc00707f, /* vmacc_vx */
0xfe00707f, /* vmadc_vi */
0xfe00707f, /* vmadc_vim */
0xfe00707f, /* vmadc_vv */
0xfe00707f, /* vmadc_vvm */
0xfe00707f, /* vmadc_vx */
0xfe00707f, /* vmadc_vxm */
0xfc00707f, /* vmadd_vv */
0xfc00707f, /* vmadd_vx */
0xfe00707f, /* vmand_mm */
0xfe00707f, /* vmandn_mm */
0xfc00707f, /* vmandnot_mm */
0xfc00707f, /* vmax_vv */
0xfc00707f, /* vmax_vx */
0xfc00707f, /* vmaxu_vv */
0xfc00707f, /* vmaxu_vx */
0xfe00707f, /* vmerge_vim */
0xfe00707f, /* vmerge_vvm */
0xfe00707f, /* vmerge_vxm */
0xfc00707f, /* vmfeq_vf */
0xfc00707f, /* vmfeq_vv */
0xfc00707f, /* vmfge_vf */
0xfc00707f, /* vmfgt_vf */
0xfc00707f, /* vmfle_vf */
0xfc00707f, /* vmfle_vv */
0xfc00707f, /* vmflt_vf */
0xfc00707f, /* vmflt_vv */
0xfc00707f, /* vmfne_vf */
0xfc00707f, /* vmfne_vv */
0xfc00707f, /* vmin_vv */
0xfc00707f, /* vmin_vx */
0xfc00707f, /* vminu_vv */
0xfc00707f, /* vminu_vx */
0xfe00707f, /* vmnand_mm */
0xfe00707f, /* vmnor_mm */
0xfe00707f, /* vmor_mm */
0xfe00707f, /* vmorn_mm */
0xfc00707f, /* vmornot_mm */
0xfe00707f, /* vmsbc_vv */
0xfe00707f, /* vmsbc_vvm */
0xfe00707f, /* vmsbc_vx */
0xfe00707f, /* vmsbc_vxm */
0xfc0ff07f, /* vmsbf_m */
0xfc00707f, /* vmseq_vi */
0xfc00707f, /* vmseq_vv */
0xfc00707f, /* vmseq_vx */
0xfc00707f, /* vmsgt_vi */
0xfc00707f, /* vmsgt_vx */
0xfc00707f, /* vmsgtu_vi */
0xfc00707f, /* vmsgtu_vx */
0xfc0ff07f, /* vmsif_m */
0xfc00707f, /* vmsle_vi */
0xfc00707f, /* vmsle_vv */
0xfc00707f, /* vmsle_vx */
0xfc00707f, /* vmsleu_vi */
0xfc00707f, /* vmsleu_vv */
0xfc00707f, /* vmsleu_vx */
0xfc00707f, /* vmslt_vv */
0xfc00707f, /* vmslt_vx */
0xfc00707f, /* vmsltu_vv */
0xfc00707f, /* vmsltu_vx */
0xfc00707f, /* vmsne_vi */
0xfc00707f, /* vmsne_vv */
0xfc00707f, /* vmsne_vx */
0xfc0ff07f, /* vmsof_m */
0xfc00707f, /* vmul_vv */
0xfc00707f, /* vmul_vx */
0xfc00707f, /* vmulh_vv */
0xfc00707f, /* vmulh_vx */
0xfc00707f, /* vmulhsu_vv */
0xfc00707f, /* vmulhsu_vx */
0xfc00707f, /* vmulhu_vv */
0xfc00707f, /* vmulhu_vx */
0xfe0ff07f, /* vmv1r_v */
0xfe0ff07f, /* vmv2r_v */
0xfe0ff07f, /* vmv4r_v */
0xfe0ff07f, /* vmv8r_v */
0xfff0707f, /* vmv_s_x */
0xfff0707f, /* vmv_v_i */
0xfff0707f, /* vmv_v_v */
0xfff0707f, /* vmv_v_x */
0xfe0ff07f, /* vmv_x_s */
0xfe00707f, /* vmxnor_mm */
0xfe00707f, /* vmxor_mm */
0xfc00707f, /* vnclip_wi */
0xfc00707f, /* vnclip_wv */
0xfc00707f, /* vnclip_wx */
0xfc00707f, /* vnclipu_wi */
0xfc00707f, /* vnclipu_wv */
0xfc00707f, /* vnclipu_wx */
0xfc00707f, /* vnmsac_vv */
0xfc00707f, /* vnmsac_vx */
0xfc00707f, /* vnmsub_vv */
0xfc00707f, /* vnmsub_vx */
0xfc00707f, /* vnsra_wi */
0xfc00707f, /* vnsra_wv */
0xfc00707f, /* vnsra_wx */
0xfc00707f, /* vnsrl_wi */
0xfc00707f, /* vnsrl_wv */
0xfc00707f, /* vnsrl_wx */
0xfc00707f, /* vor_vi */
0xfc00707f, /* vor_vv */
0xfc00707f, /* vor_vx */
0xfc0ff07f, /* vpopc_m */
0xfc00707f, /* vredand_vs */
0xfc00707f, /* vredmax_vs */
0xfc00707f, /* vredmaxu_vs */
0xfc00707f, /* vredmin_vs */
0xfc00707f, /* vredminu_vs */
0xfc00707f, /* vredor_vs */
0xfc00707f, /* vredsum_vs */
0xfc00707f, /* vredxor_vs */
0xfc00707f, /* vrem_vv */
0xfc00707f, /* vrem_vx */
0xfc00707f, /* vremu_vv */
0xfc00707f, /* vremu_vx */
0xfc0ff07f, /* vrev8_v */
0xfc00707f, /* vrgather_vi */
0xfc00707f, /* vrgather_vv */
0xfc00707f, /* vrgather_vx */
0xfc00707f, /* vrgatherei16_vv */
0xfc00707f, /* vrol_vv */
0xfc00707f, /* vrol_vx */
0xf800707f, /* vror_vi */
0xfc00707f, /* vror_vv */
0xfc00707f, /* vror_vx */
0xfc00707f, /* vrsub_vi */
0xfc00707f, /* vrsub_vx */
0xfff0707f, /* vs1r_v */
0xfff0707f, /* vs2r_v */
0xfff0707f, /* vs4r_v */
0xfff0707f, /* vs8r_v */
0xfc00707f, /* vsadd_vi */
0xfc00707f, /* vsadd_vv */
0xfc00707f, /* vsadd_vx */
0xfc00707f, /* vsaddu_vi */
0xfc00707f, /* vsaddu_vv */
0xfc00707f, /* vsaddu_vx */
0xfe00707f, /* vsbc_vvm */
0xfe00707f, /* vsbc_vxm */
0x1df0707f, /* vse16_v */
0xfff0707f, /* vse1_v */
0x1df0707f, /* vse32_v */
0x1df0707f, /* vse64_v */
0x1df0707f, /* vse8_v */
0xc000707f, /* vsetivli */
0xfe00707f, /* vsetvl */
0x8000707f, /* vsetvli */
0xfc0ff07f, /* vsext_vf2 */
0xfc0ff07f, /* vsext_vf4 */
0xfc0ff07f, /* vsext_vf8 */
0xfe00707f, /* vsha2ch_vv */
0xfe00707f, /* vsha2cl_vv */
0xfe00707f, /* vsha2ms_vv */
0xfc00707f, /* vslide1down_vx */
0xfc00707f, /* vslide1up_vx */
0xfc00707f, /* vslidedown_vi */
0xfc00707f, /* vslidedown_vx */
0xfc00707f, /* vslideup_vi */
0xfc00707f, /* vslideup_vx */
0xfc00707f, /* vsll_vi */
0xfc00707f, /* vsll_vv */
0xfc00707f, /* vsll_vx */
0xfe00707f, /* vsm3c_vi */
0xfe00707f, /* vsm3me_vv */
0xfe00707f, /* vsm4k_vi */
0xfe0ff07f, /* vsm4r_vs */
0xfe0ff07f, /* vsm4r_vv */
0xfff0707f, /* vsm_v */
0xfc00707f, /* vsmul_vv */
0xfc00707f, /* vsmul_vx */
0x1c00707f, /* vsoxei16_v */
0x1c00707f, /* vsoxei32_v */
0x1c00707f, /* vsoxei64_v */
0x1c00707f, /* vsoxei8_v */
0xfc00707f, /* vsra_vi */
0xfc00707f, /* vsra_vv */
0xfc00707f, /* vsra_vx */
0xfc00707f, /* vsrl_vi */
0xfc00707f, /* vsrl_vv */
0xfc00707f, /* vsrl_vx */
0x1c00707f, /* vsse16_v */
0x1c00707f, /* vsse32_v */
0x1c00707f, /* vsse64_v */
0x1c00707f, /* vsse8_v */
0xfc00707f, /* vssra_vi */
0xfc00707f, /* vssra_vv */
0xfc00707f, /* vssra_vx */
0xfc00707f, /* vssrl_vi */
0xfc00707f, /* vssrl_vv */
0xfc00707f, /* vssrl_vx */
0xfc00707f, /* vssub_vv */
0xfc00707f, /* vssub_vx */
0xfc00707f, /* vssubu_vv */
0xfc00707f, /* vssubu_vx */
0xfc00707f, /* vsub_vv */
0xfc00707f, /* vsub_vx */
0x1c00707f, /* vsuxei16_v */
0x1c00707f, /* vsuxei32_v */
0x1c00707f, /* vsuxei64_v */
0x1c00707f, /* vsuxei8_v */
0xfc00707f, /* vwadd_vv */
0xfc00707f, /* vwadd_vx */
0xfc00707f, /* vwadd_wv */
0xfc00707f, /* vwadd_wx */
0xfc00707f, /* vwaddu_vv */
0xfc00707f, /* vwaddu_vx */
0xfc00707f, /* vwaddu_wv */
0xfc00707f, /* vwaddu_wx */
0xfc00707f, /* vwmacc_vv */
0xfc00707f, /* vwmacc_vx */
0xfc00707f, /* vwmaccsu_vv */
0xfc00707f, /* vwmaccsu_vx */
0xfc00707f, /* vwmaccu_vv */
0xfc00707f, /* vwmaccu_vx */
0xfc00707f, /* vwmaccus_vx */
0xfc00707f, /* vwmul_vv */
0xfc00707f, /* vwmul_vx */
0xfc00707f, /* vwmulsu_vv */
0xfc00707f, /* vwmulsu_vx */
0xfc00707f, /* vwmulu_vv */
0xfc00707f, /* vwmulu_vx */
0xfc00707f, /* vwredsum_vs */
0xfc00707f, /* vwredsumu_vs */
0xfc00707f, /* vwsll_vi */
0xfc00707f, /* vwsll_vv */
0xfc00707f, /* vwsll_vx */
0xfc00707f, /* vwsub_vv */
0xfc00707f, /* vwsub_vx */
0xfc00707f, /* vwsub_wv */
0xfc00707f, /* vwsub_wx */
0xfc00707f, /* vwsubu_vv */
0xfc00707f, /* vwsubu_vx */
0xfc00707f, /* vwsubu_wv */
0xfc00707f, /* vwsubu_wx */
0xfc00707f, /* vxor_vi */
0xfc00707f, /* vxor_vv */
0xfc00707f, /* vxor_vx */
0xfc0ff07f, /* vzext_vf2 */
0xfc0ff07f, /* vzext_vf4 */
0xfc0ff07f, /* vzext_vf8 */
0xffffffff, /* wfi */
0xffffffff, /* wrs_nto */
0xffffffff, /* wrs_sto */
0xfe00707f, /* xnor */
0xfe00707f, /* xor */
0x707f, /* xori */
0xfe00707f, /* xperm4 */
0xfe00707f, /* xperm8 */
0xfff0707f, /* zext_b */
0xfff0707f, /* zext_h */
0xffff_ffff, /* zext_h_rv32 */
0xfff0707f, /* zext_w */
0xffff_ffff, /* zip */
];
pub static OPCODE_MATCH: [u32; 1021] = [
0x33,
0x800003b,
0x13,
0x1b,
0x3b,
0x2a000033,
0x2e000033,
0x22000033,
0x26000033,
0x3a000033,
0x3e000033,
0x32000033,
0x36000033,
0x30001013,
0x31001013,
0x7e000033,
0x2f,
0x302f,
0x102f,
0x202f,
0x6000002f,
0x6000302f,
0x6000102f,
0x6000202f,
0x2800002f,
0x2800302f,
0x2800102f,
0x2800402f,
0x2800202f,
0xa000002f,
0xa000302f,
0xa000102f,
0xa000202f,
0xe000002f,
0xe000302f,
0xe000102f,
0xe000202f,
0x8000002f,
0x8000302f,
0x8000102f,
0x8000202f,
0xc000002f,
0xc000302f,
0xc000102f,
0xc000202f,
0x4000002f,
0x4000302f,
0x4000102f,
0x4000202f,
0x800002f,
0x800302f,
0x800102f,
0x800202f,
0x2000002f,
0x2000302f,
0x2000102f,
0x2000202f,
0x7033,
0x7013,
0x40007033,
0x17,
0x48001033,
0x48001013,
0x48001013,
0x63,
0x63,
0x48005033,
0x48005013,
0x48005013,
0x5063,
0x7063,
0x5063,
0x4063,
0x6063,
0x4063,
0x68001033,
0x68001013,
0x68001013,
0x5063,
0x7063,
0x5063,
0x4063,
0x6063,
0x4063,
0x1063,
0x1063,
0x68705013,
0x28001033,
0x28001013,
0x28001013,
0x9002,
0x1,
0x6101,
0x0,
0x2001,
0x9c21,
0x8c61,
0x8801,
0xc001,
0xe001,
0x9002,
0x2000,
0x2002,
0x6000,
0x6002,
0xa000,
0xa002,
0xe000,
0xe002,
0xa001,
0x2001,
0x9002,
0x8002,
0x8000,
0x6000,
0x6002,
0x8440,
0x8400,
0x4001,
0x6001,
0x4000,
0x4002,
0x6081,
0x6581,
0x6681,
0x6781,
0x6181,
0x6281,
0x6381,
0x6481,
0x6081,
0x9c41,
0x8002,
0x1,
0x9c75,
0x9016,
0x900a,
0x900e,
0x9012,
0x8c41,
0x8800,
0xe000,
0xe002,
0x9c65,
0x9c6d,
0x2001,
0x8c00,
0x2,
0x2,
0x8401,
0x8401,
0x8001,
0x8001,
0x8c01,
0x9c01,
0xc000,
0xc002,
0x8c21,
0x9c61,
0x9c69,
0x9c71,
0x10200f,
0x20200f,
0x200f,
0x40200f,
0xa001033,
0xa003033,
0xa002033,
0x60001013,
0x6000101b,
0xa002,
0xac62,
0xac22,
0xba02,
0xbe02,
0xbc02,
0xb802,
0x60201013,
0x6020101b,
0x3073,
0x7073,
0x2073,
0x3073,
0x7073,
0x2073,
0x6073,
0x1073,
0x5073,
0x2073,
0x6073,
0x1073,
0x5073,
0x60101013,
0x6010101b,
0xe005033,
0xe007033,
0x2004033,
0x2005033,
0x200503b,
0x200403b,
0x7b200073,
0x100073,
0x73,
0x22002053,
0x24002053,
0x26002053,
0x20002053,
0x2000053,
0x4000053,
0x6000053,
0x53,
0xe2001053,
0xe4001053,
0xe6001053,
0xe0001053,
0x42200053,
0xd2200053,
0xd2300053,
0x42300053,
0x42000053,
0xd2000053,
0xd2100053,
0x44100053,
0xd4200053,
0xd4300053,
0x44300053,
0x44000053,
0xd4000053,
0xd4100053,
0xc2200053,
0xc4200053,
0xc6200053,
0xc0200053,
0xc2300053,
0xc4300053,
0xc6300053,
0xc0300053,
0x46100053,
0x46200053,
0xd6200053,
0xd6300053,
0x46000053,
0xd6000053,
0xd6100053,
0x40100053,
0x40200053,
0xd0200053,
0xd0300053,
0x40300053,
0xd0000053,
0xd0100053,
0xc2000053,
0xc4000053,
0xc6000053,
0xc0000053,
0xc2100053,
0xc4100053,
0xc6100053,
0xc0100053,
0xc2801053,
0x1a000053,
0x1c000053,
0x1e000053,
0x18000053,
0xf,
0x100f,
0x8330000f,
0xa2002053,
0xa4002053,
0xa6002053,
0xa0002053,
0x3007,
0xa2000053,
0xa4000053,
0xa6000053,
0xa0000053,
0xa2004053,
0xa4004053,
0xa6004053,
0xa0004053,
0x1007,
0xf2100053,
0xf4100053,
0xf6100053,
0xf0100053,
0x4007,
0xa2001053,
0xa4001053,
0xa6001053,
0xa0001053,
0xa2005053,
0xa4005053,
0xa6005053,
0xa0005053,
0x2007,
0x2000043,
0x4000043,
0x6000043,
0x43,
0x2a001053,
0x2c001053,
0x2e001053,
0x28001053,
0x2a003053,
0x2c003053,
0x2e003053,
0x28003053,
0x2a000053,
0x2c000053,
0x2e000053,
0x28000053,
0x2a002053,
0x2c002053,
0x2e002053,
0x28002053,
0x2000047,
0x4000047,
0x6000047,
0x47,
0x12000053,
0x14000053,
0x16000053,
0x10000053,
0x22000053,
0xf2000053,
0x24000053,
0xf4000053,
0x26000053,
0x20000053,
0xf0000053,
0xf0000053,
0xe2000053,
0xe4000053,
0xe0000053,
0xe0000053,
0xe2100053,
0xe6100053,
0xb2000053,
0xb6000053,
0x22001053,
0x24001053,
0x26001053,
0x20001053,
0x200004f,
0x400004f,
0x600004f,
0x4f,
0x200004b,
0x400004b,
0x600004b,
0x4b,
0x302073,
0x102073,
0x42400053,
0x44400053,
0x46400053,
0x40400053,
0x42500053,
0x44500053,
0x46500053,
0x40500053,
0x202073,
0x301073,
0x3027,
0x101073,
0x105073,
0x22000053,
0x24000053,
0x26000053,
0x20000053,
0x22001053,
0x24001053,
0x26001053,
0x20001053,
0x22002053,
0x24002053,
0x26002053,
0x20002053,
0x1027,
0x4027,
0x5a000053,
0x5c000053,
0x5e000053,
0x58000053,
0x201073,
0x205073,
0xa000053,
0xc000053,
0xe000053,
0x8000053,
0x2027,
0x62000073,
0x22000073,
0x66000073,
0x26000073,
0x60004073,
0x60104073,
0x6c004073,
0x64004073,
0x64104073,
0x68004073,
0x68104073,
0x64304073,
0x68304073,
0x62004073,
0x6e004073,
0x66004073,
0x6a004073,
0x6f,
0x6f,
0xef,
0x67,
0xe7,
0x67,
0x3,
0x4003,
0x3003,
0x1003,
0x5003,
0x1000302f,
0x1000202f,
0x37,
0x2003,
0x6003,
0xa006033,
0xa007033,
0xa004033,
0xa005033,
0x81c04073,
0x81d04073,
0x89e04073,
0x89f04073,
0x8dc04073,
0x8dd04073,
0x8de04073,
0x8df04073,
0xc1c04073,
0xc1d04073,
0xc1e04073,
0xc1f04073,
0x81e04073,
0xc5c04073,
0xc5d04073,
0xc5e04073,
0xc5f04073,
0xc9c04073,
0xc9d04073,
0xc9e04073,
0xc9f04073,
0xcdc04073,
0xcdd04073,
0x81f04073,
0xcde04073,
0xcdf04073,
0x85c04073,
0x85d04073,
0x85e04073,
0x85f04073,
0x89c04073,
0x89d04073,
0x81c04073,
0x82004073,
0x86004073,
0x8a004073,
0x8e004073,
0xc2004073,
0xc6004073,
0xca004073,
0xce004073,
0x82004073,
0x30200073,
0x2000033,
0x2001033,
0x2002033,
0x2003033,
0x200003b,
0x13,
0x40000033,
0x13,
0x500033,
0x200033,
0x300033,
0x400033,
0x6033,
0x28705013,
0x6013,
0x40006033,
0x8004033,
0x8007033,
0x800403b,
0x100000f,
0x6013,
0x106013,
0x306013,
0xc0002073,
0xc8002073,
0xc0202073,
0xc8202073,
0xc0102073,
0xc8102073,
0x2006033,
0x2007033,
0x200703b,
0x200603b,
0x8067,
0x6b805013,
0x69805013,
0x60001033,
0x6000103b,
0x60005033,
0x60005013,
0x60005013,
0x6000501b,
0x6000503b,
0x23,
0x100073,
0x1800302f,
0x1800202f,
0x73,
0x3023,
0x103013,
0x60401013,
0x60501013,
0x1b,
0x18100073,
0x12000073,
0x18000073,
0x2033,
0x1023,
0x20002033,
0x2000203b,
0x20004033,
0x2000403b,
0x20006033,
0x2000603b,
0x10201013,
0x10301013,
0x10001013,
0x10101013,
0x10601013,
0x5c000033,
0x54000033,
0x10701013,
0x5e000033,
0x56000033,
0x10401013,
0x50000033,
0x10501013,
0x52000033,
0x16000073,
0x1033,
0x1013,
0x1013,
0x800101b,
0x101b,
0x103b,
0x2033,
0x2013,
0x3013,
0x3033,
0x2033,
0x10801013,
0x10901013,
0x30000033,
0x34000033,
0x3033,
0x40005033,
0x40005013,
0x40005013,
0x4000501b,
0x4000503b,
0x10200073,
0x5033,
0x5013,
0x5013,
0x501b,
0x503b,
0x40000033,
0x4000003b,
0x2023,
0x8f05013,
0x24002057,
0x24006057,
0x20002057,
0x20006057,
0x40003057,
0x40000057,
0x40004057,
0x3057,
0x57,
0x4057,
0xa600a077,
0xa200a077,
0xa6002077,
0xa2002077,
0xa601a077,
0xa201a077,
0xa6012077,
0xa2012077,
0x8a002077,
0xaa002077,
0xa603a077,
0x24003057,
0x24000057,
0x24004057,
0x4000057,
0x4004057,
0x2c002057,
0x2c006057,
0x28002057,
0x28006057,
0x48042057,
0x48052057,
0x30002057,
0x30006057,
0x34002057,
0x34006057,
0x48062057,
0x5e002057,
0x40082057,
0x48072057,
0x4806a057,
0x84002057,
0x84006057,
0x80002057,
0x80006057,
0x5057,
0x1057,
0x4c081057,
0x48019057,
0x48011057,
0x48039057,
0x48031057,
0x48009057,
0x48001057,
0x80005057,
0x80001057,
0x4008a057,
0xb0005057,
0xb0001057,
0xa0005057,
0xa0001057,
0x18005057,
0x18001057,
0x5c005057,
0x10005057,
0x10001057,
0xb8005057,
0xb8001057,
0xa8005057,
0xa8001057,
0x90005057,
0x90001057,
0x42001057,
0x42005057,
0x5e005057,
0x480a1057,
0x48099057,
0x48091057,
0x480a9057,
0x480b9057,
0x480b1057,
0x48089057,
0x48081057,
0xb4005057,
0xb4001057,
0xa4005057,
0xa4001057,
0xbc005057,
0xbc001057,
0xac005057,
0xac001057,
0x84005057,
0x4c029057,
0x1c001057,
0x14001057,
0xc001057,
0x4001057,
0x4001057,
0x4c021057,
0x9c005057,
0x20005057,
0x20001057,
0x24005057,
0x24001057,
0x28005057,
0x28001057,
0x3c005057,
0x38005057,
0x4c001057,
0x8005057,
0x8001057,
0xc0005057,
0xc0001057,
0xd0005057,
0xd0001057,
0x48061057,
0x48059057,
0x48051057,
0x48079057,
0x48071057,
0x48049057,
0x48041057,
0xf0005057,
0xf0001057,
0xf8005057,
0xf8001057,
0xe0005057,
0xe0001057,
0xf4005057,
0xf4001057,
0xfc005057,
0xfc001057,
0xcc001057,
0xc4001057,
0xc4001057,
0xc8005057,
0xc8001057,
0xd8005057,
0xd8001057,
0xb2002077,
0xa208a077,
0x5008a057,
0x50082057,
0x2800007,
0x2805007,
0x2806007,
0x2807007,
0x2800007,
0x22800007,
0x22805007,
0x22806007,
0x22807007,
0x22800007,
0x62800007,
0x62805007,
0x62806007,
0x62807007,
0x62800007,
0xe2800007,
0xe2805007,
0xe2806007,
0xe2807007,
0xe2800007,
0x5007,
0x1005007,
0x2b00007,
0x6007,
0x1006007,
0x7007,
0x1007007,
0x7,
0x1000007,
0x2b00007,
0xc005007,
0xc006007,
0xc007007,
0xc000007,
0x8005007,
0x8006007,
0x8007007,
0x8000007,
0x4005007,
0x4006007,
0x4007007,
0x4000007,
0xb4002057,
0xb4006057,
0x46003057,
0x44003057,
0x46000057,
0x44000057,
0x46004057,
0x44004057,
0xa4002057,
0xa4006057,
0x66002057,
0x62002057,
0x60002057,
0x1c000057,
0x1c004057,
0x18000057,
0x18004057,
0x5c003057,
0x5c000057,
0x5c004057,
0x60005057,
0x60001057,
0x7c005057,
0x74005057,
0x64005057,
0x64001057,
0x6c005057,
0x6c001057,
0x70005057,
0x70001057,
0x14000057,
0x14004057,
0x10000057,
0x10004057,
0x76002057,
0x7a002057,
0x6a002057,
0x72002057,
0x70002057,
0x4e000057,
0x4c000057,
0x4e004057,
0x4c004057,
0x5000a057,
0x60003057,
0x60000057,
0x60004057,
0x7c003057,
0x7c004057,
0x78003057,
0x78004057,
0x5001a057,
0x74003057,
0x74000057,
0x74004057,
0x70003057,
0x70000057,
0x70004057,
0x6c000057,
0x6c004057,
0x68000057,
0x68004057,
0x64003057,
0x64000057,
0x64004057,
0x50012057,
0x94002057,
0x94006057,
0x9c002057,
0x9c006057,
0x98002057,
0x98006057,
0x90002057,
0x90006057,
0x9e003057,
0x9e00b057,
0x9e01b057,
0x9e03b057,
0x42006057,
0x5e003057,
0x5e000057,
0x5e004057,
0x42002057,
0x7e002057,
0x6e002057,
0xbc003057,
0xbc000057,
0xbc004057,
0xb8003057,
0xb8000057,
0xb8004057,
0xbc002057,
0xbc006057,
0xac002057,
0xac006057,
0xb4003057,
0xb4000057,
0xb4004057,
0xb0003057,
0xb0000057,
0xb0004057,
0x28003057,
0x28000057,
0x28004057,
0x40082057,
0x4002057,
0x1c002057,
0x18002057,
0x14002057,
0x10002057,
0x8002057,
0x2057,
0xc002057,
0x8c002057,
0x8c006057,
0x88002057,
0x88006057,
0x4804a057,
0x30003057,
0x30000057,
0x30004057,
0x38000057,
0x54000057,
0x54004057,
0x50003057,
0x50000057,
0x50004057,
0xc003057,
0xc004057,
0x2800027,
0x22800027,
0x62800027,
0xe2800027,
0x84003057,
0x84000057,
0x84004057,
0x80003057,
0x80000057,
0x80004057,
0x48000057,
0x48004057,
0x5027,
0x2b00027,
0x6027,
0x7027,
0x27,
0xc0007057,
0x80007057,
0x7057,
0x4803a057,
0x4802a057,
0x4801a057,
0xba002077,
0xbe002077,
0xb6002077,
0x3c006057,
0x38006057,
0x3c003057,
0x3c004057,
0x38003057,
0x38004057,
0x94003057,
0x94000057,
0x94004057,
0xae002077,
0x82002077,
0x86002077,
0xa6082077,
0xa2082077,
0x2b00027,
0x9c000057,
0x9c004057,
0xc005027,
0xc006027,
0xc007027,
0xc000027,
0xa4003057,
0xa4000057,
0xa4004057,
0xa0003057,
0xa0000057,
0xa0004057,
0x8005027,
0x8006027,
0x8007027,
0x8000027,
0xac003057,
0xac000057,
0xac004057,
0xa8003057,
0xa8000057,
0xa8004057,
0x8c000057,
0x8c004057,
0x88000057,
0x88004057,
0x8000057,
0x8004057,
0x4005027,
0x4006027,
0x4007027,
0x4000027,
0xc4002057,
0xc4006057,
0xd4002057,
0xd4006057,
0xc0002057,
0xc0006057,
0xd0002057,
0xd0006057,
0xf4002057,
0xf4006057,
0xfc002057,
0xfc006057,
0xf0002057,
0xf0006057,
0xf8006057,
0xec002057,
0xec006057,
0xe8002057,
0xe8006057,
0xe0002057,
0xe0006057,
0xc4000057,
0xc0000057,
0xd4003057,
0xd4000057,
0xd4004057,
0xcc002057,
0xcc006057,
0xdc002057,
0xdc006057,
0xc8002057,
0xc8006057,
0xd8002057,
0xd8006057,
0x2c003057,
0x2c000057,
0x2c004057,
0x48032057,
0x48022057,
0x48012057,
0x10500073,
0xd00073,
0x1d00073,
0x40004033,
0x4033,
0x4013,
0x28002033,
0x28004033,
0x7013,
0x800403b,
0x8004033,
0x800003b,
0x8f01013,
];
pub static OPCODE_MASK: [u32; 1021] = [
0xfe00707f,
0xfe00707f,
0x707f,
0x707f,
0xfe00707f,
0x3e00707f,
0x3e00707f,
0x3e00707f,
0x3e00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfff0707f,
0xff00707f,
0xfe00707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xf800707f,
0xfe00707f,
0x707f,
0xfe00707f,
0x7f,
0xfe00707f,
0xfc00707f,
0xfe00707f,
0x707f,
0x1f0707f,
0xfe00707f,
0xfc00707f,
0xfe00707f,
0x707f,
0x707f,
0x1f0707f,
0x707f,
0x707f,
0xff07f,
0xfe00707f,
0xfc00707f,
0xfe00707f,
0x707f,
0x707f,
0xff07f,
0x707f,
0x707f,
0x1f0707f,
0x707f,
0x1f0707f,
0xfff0707f,
0xfe00707f,
0xfc00707f,
0xfe00707f,
0xf003,
0xe003,
0xef83,
0xe003,
0xe003,
0xfc63,
0xfc63,
0xec03,
0xe003,
0xe003,
0xffff,
0xe003,
0xe003,
0xe003,
0xe003,
0xe003,
0xe003,
0xe003,
0xe003,
0xe003,
0xe003,
0xf07f,
0xf07f,
0xfc03,
0xe003,
0xe003,
0xfc43,
0xfc43,
0xe003,
0xe003,
0xe003,
0xe003,
0xffff,
0xffff,
0xffff,
0xffff,
0xffff,
0xffff,
0xffff,
0xffff,
0xf8ff,
0xfc63,
0xf003,
0xef83,
0xfc7f,
0xffff,
0xffff,
0xffff,
0xffff,
0xfc63,
0xfc03,
0xe003,
0xe003,
0xfc7f,
0xfc7f,
0xf07f,
0xfc43,
0xe003,
0xf003,
0xec03,
0xfc03,
0xec03,
0xfc03,
0xfc63,
0xfc63,
0xe003,
0xe003,
0xfc63,
0xfc7f,
0xfc7f,
0xfc7f,
0xfff07fff,
0xfff07fff,
0xfff07fff,
0xfff07fff,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfff0707f,
0xfff0707f,
0xfc03,
0xfc63,
0xfc63,
0xff03,
0xff03,
0xff03,
0xff03,
0xfff0707f,
0xfff0707f,
0x7fff,
0x7fff,
0xff07f,
0x707f,
0x707f,
0x707f,
0x707f,
0x707f,
0x707f,
0x7fff,
0x7fff,
0x7fff,
0x7fff,
0xfff0707f,
0xfff0707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xffffffff,
0xffffffff,
0xffffffff,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00007f,
0xfe00007f,
0xfe00007f,
0xfe00007f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0707f,
0xfe00007f,
0xfe00007f,
0xfe00007f,
0xfe00007f,
0x707f,
0x707f,
0xfff0707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0x707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0x707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0x707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0x707f,
0x600007f,
0x600007f,
0x600007f,
0x600007f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0x600007f,
0x600007f,
0x600007f,
0x600007f,
0xfe00007f,
0xfe00007f,
0xfe00007f,
0xfe00007f,
0xfe00707f,
0xfff0707f,
0xfe00707f,
0xfff0707f,
0xfe00707f,
0xfe00707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0x600007f,
0x600007f,
0x600007f,
0x600007f,
0x600007f,
0x600007f,
0x600007f,
0x600007f,
0xfffff07f,
0xfffff07f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfffff07f,
0xfff0707f,
0x707f,
0xfff0707f,
0xfff0707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0x707f,
0x707f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0007f,
0xfff0707f,
0xfff0707f,
0xfe00007f,
0xfe00007f,
0xfe00007f,
0xfe00007f,
0x707f,
0xfe007fff,
0xfe007fff,
0xfe007fff,
0xfe007fff,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfe007fff,
0xfe007fff,
0xfe007fff,
0xfe007fff,
0xfff,
0x7f,
0xfff,
0x707f,
0xfff07fff,
0xfff07fff,
0x707f,
0x707f,
0x707f,
0x707f,
0x707f,
0xf9f0707f,
0xf9f0707f,
0x7f,
0x707f,
0x707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xb3c0707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xb200707f,
0xffffffff,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfff0707f,
0xfff0707f,
0xffffffff,
0xffffffff,
0xffffffff,
0xffffffff,
0xffffffff,
0xfe00707f,
0xfff0707f,
0x707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xffffffff,
0x1f07fff,
0x1f07fff,
0x1f07fff,
0xfffff07f,
0xfffff07f,
0xfffff07f,
0xfffff07f,
0xfffff07f,
0xfffff07f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xffffffff,
0xfff0707f,
0xfff0707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0x707f,
0xffffffff,
0xf800707f,
0xf800707f,
0xffffffff,
0x707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xffffffff,
0xfe007fff,
0xffffffff,
0xfe0ff07f,
0x707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfe00707f,
0xfe00707f,
0xfff0707f,
0xfe00707f,
0xfe00707f,
0xfff0707f,
0xfe00707f,
0xfff0707f,
0xfe00707f,
0xfe007fff,
0xfe00707f,
0xfc00707f,
0xfe00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0x707f,
0x707f,
0xfe00707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0x3e00707f,
0x3e00707f,
0xfe0ff07f,
0xfe00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xffffffff,
0xfe00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0x707f,
0xfff0707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfe0ff07f,
0xfe0ff07f,
0xfe0ff07f,
0xfe0ff07f,
0xfe0ff07f,
0xfe0ff07f,
0xfe0ff07f,
0xfe0ff07f,
0xfe00707f,
0xfe00707f,
0xfe0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfe00707f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfe00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfe0ff07f,
0xfff0707f,
0xfff0707f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfe00707f,
0xfe0ff07f,
0xfdfff07f,
0xfc0ff07f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0x1df0707f,
0x1df0707f,
0xfff0707f,
0x1df0707f,
0x1df0707f,
0x1df0707f,
0x1df0707f,
0x1df0707f,
0x1df0707f,
0xfff0707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0xfc00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfc00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfe0ff07f,
0xfe0ff07f,
0xfe0ff07f,
0xfe0ff07f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfe0ff07f,
0xfe00707f,
0xfe00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xf800707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0x1df0707f,
0xfff0707f,
0x1df0707f,
0x1df0707f,
0x1df0707f,
0xc000707f,
0xfe00707f,
0x8000707f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfe00707f,
0xfe00707f,
0xfe00707f,
0xfe0ff07f,
0xfe0ff07f,
0xfff0707f,
0xfc00707f,
0xfc00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0x1c00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc00707f,
0xfc0ff07f,
0xfc0ff07f,
0xfc0ff07f,
0xffffffff,
0xffffffff,
0xffffffff,
0xfe00707f,
0xfe00707f,
0x707f,
0xfe00707f,
0xfe00707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
0xfff0707f,
];
pub static OPCODE_MASK_COMPRESSED: [u16; 1021] = [
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
61443,
57347,
61315,
57347,
57347,
64611,
64611,
60419,
57347,
57347,
65535,
57347,
57347,
57347,
57347,
57347,
57347,
57347,
57347,
57347,
57347,
61567,
61567,
64515,
57347,
57347,
64579,
64579,
57347,
57347,
57347,
57347,
65535,
65535,
65535,
65535,
65535,
65535,
65535,
65535,
63743,
64611,
61443,
61315,
64639,
65535,
65535,
65535,
65535,
64611,
64515,
57347,
57347,
64639,
64639,
61567,
64579,
57347,
61443,
60419,
64515,
60419,
64515,
64611,
64611,
57347,
57347,
64611,
64639,
64639,
64639,
32767,
32767,
32767,
32767,
28799,
28799,
28799,
28799,
28799,
64515,
64611,
64611,
65283,
65283,
65283,
65283,
28799,
28799,
32767,
32767,
61567,
28799,
28799,
28799,
28799,
28799,
28799,
32767,
32767,
32767,
32767,
28799,
28799,
28799,
28799,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
];

pub static OPCODE_MATCH_COMPRESSED: [u16; 1021] = [
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
36866,
1,
24833,
0,
8193,
39969,
35937,
34817,
49153,
57345,
36866,
8192,
8194,
24576,
24578,
40960,
40962,
57344,
57346,
40961,
8193,
36866,
32770,
32768,
24576,
24578,
33856,
33792,
16385,
24577,
16384,
16386,
24705,
25985,
26241,
26497,
24961,
25217,
25473,
25729,
24705,
40001,
32770,
1,
40053,
36886,
36874,
36878,
36882,
35905,
34816,
57344,
57346,
40037,
40045,
8193,
35840,
2,
2,
33793,
33793,
32769,
32769,
35841,
39937,
49152,
49154,
35873,
40033,
40041,
40049,
8207,
8207,
8207,
8207,
4147,
12339,
8243,
4115,
4123,
40962,
44130,
44066,
47618,
48642,
48130,
47106,
4115,
4123,
12403,
28787,
8307,
12403,
28787,
8307,
24691,
4211,
20595,
8307,
24691,
4211,
20595,
4115,
4123,
20531,
28723,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
0,
];

pub static ALL_OPCODES: [Opcode; 1021] = [
Opcode::ADD,
Opcode::ADDUW,
Opcode::ADDI,
Opcode::ADDIW,
Opcode::ADDW,
Opcode::AES32DSI,
Opcode::AES32DSMI,
Opcode::AES32ESI,
Opcode::AES32ESMI,
Opcode::AES64DS,
Opcode::AES64DSM,
Opcode::AES64ES,
Opcode::AES64ESM,
Opcode::AES64IM,
Opcode::AES64KS1I,
Opcode::AES64KS2,
Opcode::AMOADDB,
Opcode::AMOADDD,
Opcode::AMOADDH,
Opcode::AMOADDW,
Opcode::AMOANDB,
Opcode::AMOANDD,
Opcode::AMOANDH,
Opcode::AMOANDW,
Opcode::AMOCASB,
Opcode::AMOCASD,
Opcode::AMOCASH,
Opcode::AMOCASQ,
Opcode::AMOCASW,
Opcode::AMOMAXB,
Opcode::AMOMAXD,
Opcode::AMOMAXH,
Opcode::AMOMAXW,
Opcode::AMOMAXUB,
Opcode::AMOMAXUD,
Opcode::AMOMAXUH,
Opcode::AMOMAXUW,
Opcode::AMOMINB,
Opcode::AMOMIND,
Opcode::AMOMINH,
Opcode::AMOMINW,
Opcode::AMOMINUB,
Opcode::AMOMINUD,
Opcode::AMOMINUH,
Opcode::AMOMINUW,
Opcode::AMOORB,
Opcode::AMOORD,
Opcode::AMOORH,
Opcode::AMOORW,
Opcode::AMOSWAPB,
Opcode::AMOSWAPD,
Opcode::AMOSWAPH,
Opcode::AMOSWAPW,
Opcode::AMOXORB,
Opcode::AMOXORD,
Opcode::AMOXORH,
Opcode::AMOXORW,
Opcode::AND,
Opcode::ANDI,
Opcode::ANDN,
Opcode::AUIPC,
Opcode::BCLR,
Opcode::BCLRI,
Opcode::BCLRIRV32,
Opcode::BEQ,
Opcode::BEQZ,
Opcode::BEXT,
Opcode::BEXTI,
Opcode::BEXTIRV32,
Opcode::BGE,
Opcode::BGEU,
Opcode::BGEZ,
Opcode::BGT,
Opcode::BGTU,
Opcode::BGTZ,
Opcode::BINV,
Opcode::BINVI,
Opcode::BINVIRV32,
Opcode::BLE,
Opcode::BLEU,
Opcode::BLEZ,
Opcode::BLT,
Opcode::BLTU,
Opcode::BLTZ,
Opcode::BNE,
Opcode::BNEZ,
Opcode::BREV8,
Opcode::BSET,
Opcode::BSETI,
Opcode::BSETIRV32,
Opcode::CADD,
Opcode::CADDI,
Opcode::CADDI16SP,
Opcode::CADDI4SPN,
Opcode::CADDIW,
Opcode::CADDW,
Opcode::CAND,
Opcode::CANDI,
Opcode::CBEQZ,
Opcode::CBNEZ,
Opcode::CEBREAK,
Opcode::CFLD,
Opcode::CFLDSP,
Opcode::CFLW,
Opcode::CFLWSP,
Opcode::CFSD,
Opcode::CFSDSP,
Opcode::CFSW,
Opcode::CFSWSP,
Opcode::CJ,
Opcode::CJAL,
Opcode::CJALR,
Opcode::CJR,
Opcode::CLBU,
Opcode::CLD,
Opcode::CLDSP,
Opcode::CLH,
Opcode::CLHU,
Opcode::CLI,
Opcode::CLUI,
Opcode::CLW,
Opcode::CLWSP,
Opcode::CMOP1,
Opcode::CMOP11,
Opcode::CMOP13,
Opcode::CMOP15,
Opcode::CMOP3,
Opcode::CMOP5,
Opcode::CMOP7,
Opcode::CMOP9,
Opcode::CMOPN,
Opcode::CMUL,
Opcode::CMV,
Opcode::CNOP,
Opcode::CNOT,
Opcode::CNTLALL,
Opcode::CNTLP1,
Opcode::CNTLPALL,
Opcode::CNTLS1,
Opcode::COR,
Opcode::CSB,
Opcode::CSD,
Opcode::CSDSP,
Opcode::CSEXTB,
Opcode::CSEXTH,
Opcode::CSEXTW,
Opcode::CSH,
Opcode::CSLLI,
Opcode::CSLLIRV32,
Opcode::CSRAI,
Opcode::CSRAIRV32,
Opcode::CSRLI,
Opcode::CSRLIRV32,
Opcode::CSUB,
Opcode::CSUBW,
Opcode::CSW,
Opcode::CSWSP,
Opcode::CXOR,
Opcode::CZEXTB,
Opcode::CZEXTH,
Opcode::CZEXTW,
Opcode::CBOCLEAN,
Opcode::CBOFLUSH,
Opcode::CBOINVAL,
Opcode::CBOZERO,
Opcode::CLMUL,
Opcode::CLMULH,
Opcode::CLMULR,
Opcode::CLZ,
Opcode::CLZW,
Opcode::CMJALT,
Opcode::CMMVA01S,
Opcode::CMMVSA01,
Opcode::CMPOP,
Opcode::CMPOPRET,
Opcode::CMPOPRETZ,
Opcode::CMPUSH,
Opcode::CPOP,
Opcode::CPOPW,
Opcode::CSRC,
Opcode::CSRCI,
Opcode::CSRR,
Opcode::CSRRC,
Opcode::CSRRCI,
Opcode::CSRRS,
Opcode::CSRRSI,
Opcode::CSRRW,
Opcode::CSRRWI,
Opcode::CSRS,
Opcode::CSRSI,
Opcode::CSRW,
Opcode::CSRWI,
Opcode::CTZ,
Opcode::CTZW,
Opcode::CZEROEQZ,
Opcode::CZERONEZ,
Opcode::DIV,
Opcode::DIVU,
Opcode::DIVUW,
Opcode::DIVW,
Opcode::DRET,
Opcode::EBREAK,
Opcode::ECALL,
Opcode::FABSD,
Opcode::FABSH,
Opcode::FABSQ,
Opcode::FABSS,
Opcode::FADDD,
Opcode::FADDH,
Opcode::FADDQ,
Opcode::FADDS,
Opcode::FCLASSD,
Opcode::FCLASSH,
Opcode::FCLASSQ,
Opcode::FCLASSS,
Opcode::FCVTDH,
Opcode::FCVTDL,
Opcode::FCVTDLU,
Opcode::FCVTDQ,
Opcode::FCVTDS,
Opcode::FCVTDW,
Opcode::FCVTDWU,
Opcode::FCVTHD,
Opcode::FCVTHL,
Opcode::FCVTHLU,
Opcode::FCVTHQ,
Opcode::FCVTHS,
Opcode::FCVTHW,
Opcode::FCVTHWU,
Opcode::FCVTLD,
Opcode::FCVTLH,
Opcode::FCVTLQ,
Opcode::FCVTLS,
Opcode::FCVTLUD,
Opcode::FCVTLUH,
Opcode::FCVTLUQ,
Opcode::FCVTLUS,
Opcode::FCVTQD,
Opcode::FCVTQH,
Opcode::FCVTQL,
Opcode::FCVTQLU,
Opcode::FCVTQS,
Opcode::FCVTQW,
Opcode::FCVTQWU,
Opcode::FCVTSD,
Opcode::FCVTSH,
Opcode::FCVTSL,
Opcode::FCVTSLU,
Opcode::FCVTSQ,
Opcode::FCVTSW,
Opcode::FCVTSWU,
Opcode::FCVTWD,
Opcode::FCVTWH,
Opcode::FCVTWQ,
Opcode::FCVTWS,
Opcode::FCVTWUD,
Opcode::FCVTWUH,
Opcode::FCVTWUQ,
Opcode::FCVTWUS,
Opcode::FCVTMODWD,
Opcode::FDIVD,
Opcode::FDIVH,
Opcode::FDIVQ,
Opcode::FDIVS,
Opcode::FENCE,
Opcode::FENCEI,
Opcode::FENCETSO,
Opcode::FEQD,
Opcode::FEQH,
Opcode::FEQQ,
Opcode::FEQS,
Opcode::FLD,
Opcode::FLED,
Opcode::FLEH,
Opcode::FLEQ,
Opcode::FLES,
Opcode::FLEQD,
Opcode::FLEQH,
Opcode::FLEQQ,
Opcode::FLEQS,
Opcode::FLH,
Opcode::FLID,
Opcode::FLIH,
Opcode::FLIQ,
Opcode::FLIS,
Opcode::FLQ,
Opcode::FLTD,
Opcode::FLTH,
Opcode::FLTQ,
Opcode::FLTS,
Opcode::FLTQD,
Opcode::FLTQH,
Opcode::FLTQQ,
Opcode::FLTQS,
Opcode::FLW,
Opcode::FMADDD,
Opcode::FMADDH,
Opcode::FMADDQ,
Opcode::FMADDS,
Opcode::FMAXD,
Opcode::FMAXH,
Opcode::FMAXQ,
Opcode::FMAXS,
Opcode::FMAXMD,
Opcode::FMAXMH,
Opcode::FMAXMQ,
Opcode::FMAXMS,
Opcode::FMIND,
Opcode::FMINH,
Opcode::FMINQ,
Opcode::FMINS,
Opcode::FMINMD,
Opcode::FMINMH,
Opcode::FMINMQ,
Opcode::FMINMS,
Opcode::FMSUBD,
Opcode::FMSUBH,
Opcode::FMSUBQ,
Opcode::FMSUBS,
Opcode::FMULD,
Opcode::FMULH,
Opcode::FMULQ,
Opcode::FMULS,
Opcode::FMVD,
Opcode::FMVDX,
Opcode::FMVH,
Opcode::FMVHX,
Opcode::FMVQ,
Opcode::FMVS,
Opcode::FMVSX,
Opcode::FMVWX,
Opcode::FMVXD,
Opcode::FMVXH,
Opcode::FMVXS,
Opcode::FMVXW,
Opcode::FMVHXD,
Opcode::FMVHXQ,
Opcode::FMVPDX,
Opcode::FMVPQX,
Opcode::FNEGD,
Opcode::FNEGH,
Opcode::FNEGQ,
Opcode::FNEGS,
Opcode::FNMADDD,
Opcode::FNMADDH,
Opcode::FNMADDQ,
Opcode::FNMADDS,
Opcode::FNMSUBD,
Opcode::FNMSUBH,
Opcode::FNMSUBQ,
Opcode::FNMSUBS,
Opcode::FRCSR,
Opcode::FRFLAGS,
Opcode::FROUNDD,
Opcode::FROUNDH,
Opcode::FROUNDQ,
Opcode::FROUNDS,
Opcode::FROUNDNXD,
Opcode::FROUNDNXH,
Opcode::FROUNDNXQ,
Opcode::FROUNDNXS,
Opcode::FRRM,
Opcode::FSCSR,
Opcode::FSD,
Opcode::FSFLAGS,
Opcode::FSFLAGSI,
Opcode::FSGNJD,
Opcode::FSGNJH,
Opcode::FSGNJQ,
Opcode::FSGNJS,
Opcode::FSGNJND,
Opcode::FSGNJNH,
Opcode::FSGNJNQ,
Opcode::FSGNJNS,
Opcode::FSGNJXD,
Opcode::FSGNJXH,
Opcode::FSGNJXQ,
Opcode::FSGNJXS,
Opcode::FSH,
Opcode::FSQ,
Opcode::FSQRTD,
Opcode::FSQRTH,
Opcode::FSQRTQ,
Opcode::FSQRTS,
Opcode::FSRM,
Opcode::FSRMI,
Opcode::FSUBD,
Opcode::FSUBH,
Opcode::FSUBQ,
Opcode::FSUBS,
Opcode::FSW,
Opcode::HFENCEGVMA,
Opcode::HFENCEVVMA,
Opcode::HINVALGVMA,
Opcode::HINVALVVMA,
Opcode::HLVB,
Opcode::HLVBU,
Opcode::HLVD,
Opcode::HLVH,
Opcode::HLVHU,
Opcode::HLVW,
Opcode::HLVWU,
Opcode::HLVXHU,
Opcode::HLVXWU,
Opcode::HSVB,
Opcode::HSVD,
Opcode::HSVH,
Opcode::HSVW,
Opcode::J,
Opcode::JAL,
Opcode::JALPSEUDO,
Opcode::JALR,
Opcode::JALRPSEUDO,
Opcode::JR,
Opcode::LB,
Opcode::LBU,
Opcode::LD,
Opcode::LH,
Opcode::LHU,
Opcode::LRD,
Opcode::LRW,
Opcode::LUI,
Opcode::LW,
Opcode::LWU,
Opcode::MAX,
Opcode::MAXU,
Opcode::MIN,
Opcode::MINU,
Opcode::MOPR0,
Opcode::MOPR1,
Opcode::MOPR10,
Opcode::MOPR11,
Opcode::MOPR12,
Opcode::MOPR13,
Opcode::MOPR14,
Opcode::MOPR15,
Opcode::MOPR16,
Opcode::MOPR17,
Opcode::MOPR18,
Opcode::MOPR19,
Opcode::MOPR2,
Opcode::MOPR20,
Opcode::MOPR21,
Opcode::MOPR22,
Opcode::MOPR23,
Opcode::MOPR24,
Opcode::MOPR25,
Opcode::MOPR26,
Opcode::MOPR27,
Opcode::MOPR28,
Opcode::MOPR29,
Opcode::MOPR3,
Opcode::MOPR30,
Opcode::MOPR31,
Opcode::MOPR4,
Opcode::MOPR5,
Opcode::MOPR6,
Opcode::MOPR7,
Opcode::MOPR8,
Opcode::MOPR9,
Opcode::MOPRN,
Opcode::MOPRR0,
Opcode::MOPRR1,
Opcode::MOPRR2,
Opcode::MOPRR3,
Opcode::MOPRR4,
Opcode::MOPRR5,
Opcode::MOPRR6,
Opcode::MOPRR7,
Opcode::MOPRRN,
Opcode::MRET,
Opcode::MUL,
Opcode::MULH,
Opcode::MULHSU,
Opcode::MULHU,
Opcode::MULW,
Opcode::MV,
Opcode::NEG,
Opcode::NOP,
Opcode::NTLALL,
Opcode::NTLP1,
Opcode::NTLPALL,
Opcode::NTLS1,
Opcode::OR,
Opcode::ORCB,
Opcode::ORI,
Opcode::ORN,
Opcode::PACK,
Opcode::PACKH,
Opcode::PACKW,
Opcode::PAUSE,
Opcode::PREFETCHI,
Opcode::PREFETCHR,
Opcode::PREFETCHW,
Opcode::RDCYCLE,
Opcode::RDCYCLEH,
Opcode::RDINSTRET,
Opcode::RDINSTRETH,
Opcode::RDTIME,
Opcode::RDTIMEH,
Opcode::REM,
Opcode::REMU,
Opcode::REMUW,
Opcode::REMW,
Opcode::RET,
Opcode::REV8,
Opcode::REV8RV32,
Opcode::ROL,
Opcode::ROLW,
Opcode::ROR,
Opcode::RORI,
Opcode::RORIRV32,
Opcode::RORIW,
Opcode::RORW,
Opcode::SB,
Opcode::SBREAK,
Opcode::SCD,
Opcode::SCW,
Opcode::SCALL,
Opcode::SD,
Opcode::SEQZ,
Opcode::SEXTB,
Opcode::SEXTH,
Opcode::SEXTW,
Opcode::SFENCEINVALIR,
Opcode::SFENCEVMA,
Opcode::SFENCEWINVAL,
Opcode::SGTZ,
Opcode::SH,
Opcode::SH1ADD,
Opcode::SH1ADDUW,
Opcode::SH2ADD,
Opcode::SH2ADDUW,
Opcode::SH3ADD,
Opcode::SH3ADDUW,
Opcode::SHA256SIG0,
Opcode::SHA256SIG1,
Opcode::SHA256SUM0,
Opcode::SHA256SUM1,
Opcode::SHA512SIG0,
Opcode::SHA512SIG0H,
Opcode::SHA512SIG0L,
Opcode::SHA512SIG1,
Opcode::SHA512SIG1H,
Opcode::SHA512SIG1L,
Opcode::SHA512SUM0,
Opcode::SHA512SUM0R,
Opcode::SHA512SUM1,
Opcode::SHA512SUM1R,
Opcode::SINVALVMA,
Opcode::SLL,
Opcode::SLLI,
Opcode::SLLIRV32,
Opcode::SLLIUW,
Opcode::SLLIW,
Opcode::SLLW,
Opcode::SLT,
Opcode::SLTI,
Opcode::SLTIU,
Opcode::SLTU,
Opcode::SLTZ,
Opcode::SM3P0,
Opcode::SM3P1,
Opcode::SM4ED,
Opcode::SM4KS,
Opcode::SNEZ,
Opcode::SRA,
Opcode::SRAI,
Opcode::SRAIRV32,
Opcode::SRAIW,
Opcode::SRAW,
Opcode::SRET,
Opcode::SRL,
Opcode::SRLI,
Opcode::SRLIRV32,
Opcode::SRLIW,
Opcode::SRLW,
Opcode::SUB,
Opcode::SUBW,
Opcode::SW,
Opcode::UNZIP,
Opcode::VAADDVV,
Opcode::VAADDVX,
Opcode::VAADDUVV,
Opcode::VAADDUVX,
Opcode::VADCVIM,
Opcode::VADCVVM,
Opcode::VADCVXM,
Opcode::VADDVI,
Opcode::VADDVV,
Opcode::VADDVX,
Opcode::VAESDFVS,
Opcode::VAESDFVV,
Opcode::VAESDMVS,
Opcode::VAESDMVV,
Opcode::VAESEFVS,
Opcode::VAESEFVV,
Opcode::VAESEMVS,
Opcode::VAESEMVV,
Opcode::VAESKF1VI,
Opcode::VAESKF2VI,
Opcode::VAESZVS,
Opcode::VANDVI,
Opcode::VANDVV,
Opcode::VANDVX,
Opcode::VANDNVV,
Opcode::VANDNVX,
Opcode::VASUBVV,
Opcode::VASUBVX,
Opcode::VASUBUVV,
Opcode::VASUBUVX,
Opcode::VBREV8V,
Opcode::VBREVV,
Opcode::VCLMULVV,
Opcode::VCLMULVX,
Opcode::VCLMULHVV,
Opcode::VCLMULHVX,
Opcode::VCLZV,
Opcode::VCOMPRESSVM,
Opcode::VCPOPM,
Opcode::VCPOPV,
Opcode::VCTZV,
Opcode::VDIVVV,
Opcode::VDIVVX,
Opcode::VDIVUVV,
Opcode::VDIVUVX,
Opcode::VFADDVF,
Opcode::VFADDVV,
Opcode::VFCLASSV,
Opcode::VFCVTFXV,
Opcode::VFCVTFXUV,
Opcode::VFCVTRTZXFV,
Opcode::VFCVTRTZXUFV,
Opcode::VFCVTXFV,
Opcode::VFCVTXUFV,
Opcode::VFDIVVF,
Opcode::VFDIVVV,
Opcode::VFIRSTM,
Opcode::VFMACCVF,
Opcode::VFMACCVV,
Opcode::VFMADDVF,
Opcode::VFMADDVV,
Opcode::VFMAXVF,
Opcode::VFMAXVV,
Opcode::VFMERGEVFM,
Opcode::VFMINVF,
Opcode::VFMINVV,
Opcode::VFMSACVF,
Opcode::VFMSACVV,
Opcode::VFMSUBVF,
Opcode::VFMSUBVV,
Opcode::VFMULVF,
Opcode::VFMULVV,
Opcode::VFMVFS,
Opcode::VFMVSF,
Opcode::VFMVVF,
Opcode::VFNCVTFFW,
Opcode::VFNCVTFXW,
Opcode::VFNCVTFXUW,
Opcode::VFNCVTRODFFW,
Opcode::VFNCVTRTZXFW,
Opcode::VFNCVTRTZXUFW,
Opcode::VFNCVTXFW,
Opcode::VFNCVTXUFW,
Opcode::VFNMACCVF,
Opcode::VFNMACCVV,
Opcode::VFNMADDVF,
Opcode::VFNMADDVV,
Opcode::VFNMSACVF,
Opcode::VFNMSACVV,
Opcode::VFNMSUBVF,
Opcode::VFNMSUBVV,
Opcode::VFRDIVVF,
Opcode::VFREC7V,
Opcode::VFREDMAXVS,
Opcode::VFREDMINVS,
Opcode::VFREDOSUMVS,
Opcode::VFREDSUMVS,
Opcode::VFREDUSUMVS,
Opcode::VFRSQRT7V,
Opcode::VFRSUBVF,
Opcode::VFSGNJVF,
Opcode::VFSGNJVV,
Opcode::VFSGNJNVF,
Opcode::VFSGNJNVV,
Opcode::VFSGNJXVF,
Opcode::VFSGNJXVV,
Opcode::VFSLIDE1DOWNVF,
Opcode::VFSLIDE1UPVF,
Opcode::VFSQRTV,
Opcode::VFSUBVF,
Opcode::VFSUBVV,
Opcode::VFWADDVF,
Opcode::VFWADDVV,
Opcode::VFWADDWF,
Opcode::VFWADDWV,
Opcode::VFWCVTFFV,
Opcode::VFWCVTFXV,
Opcode::VFWCVTFXUV,
Opcode::VFWCVTRTZXFV,
Opcode::VFWCVTRTZXUFV,
Opcode::VFWCVTXFV,
Opcode::VFWCVTXUFV,
Opcode::VFWMACCVF,
Opcode::VFWMACCVV,
Opcode::VFWMSACVF,
Opcode::VFWMSACVV,
Opcode::VFWMULVF,
Opcode::VFWMULVV,
Opcode::VFWNMACCVF,
Opcode::VFWNMACCVV,
Opcode::VFWNMSACVF,
Opcode::VFWNMSACVV,
Opcode::VFWREDOSUMVS,
Opcode::VFWREDSUMVS,
Opcode::VFWREDUSUMVS,
Opcode::VFWSUBVF,
Opcode::VFWSUBVV,
Opcode::VFWSUBWF,
Opcode::VFWSUBWV,
Opcode::VGHSHVV,
Opcode::VGMULVV,
Opcode::VIDV,
Opcode::VIOTAM,
Opcode::VL1RV,
Opcode::VL1RE16V,
Opcode::VL1RE32V,
Opcode::VL1RE64V,
Opcode::VL1RE8V,
Opcode::VL2RV,
Opcode::VL2RE16V,
Opcode::VL2RE32V,
Opcode::VL2RE64V,
Opcode::VL2RE8V,
Opcode::VL4RV,
Opcode::VL4RE16V,
Opcode::VL4RE32V,
Opcode::VL4RE64V,
Opcode::VL4RE8V,
Opcode::VL8RV,
Opcode::VL8RE16V,
Opcode::VL8RE32V,
Opcode::VL8RE64V,
Opcode::VL8RE8V,
Opcode::VLE16V,
Opcode::VLE16FFV,
Opcode::VLE1V,
Opcode::VLE32V,
Opcode::VLE32FFV,
Opcode::VLE64V,
Opcode::VLE64FFV,
Opcode::VLE8V,
Opcode::VLE8FFV,
Opcode::VLMV,
Opcode::VLOXEI16V,
Opcode::VLOXEI32V,
Opcode::VLOXEI64V,
Opcode::VLOXEI8V,
Opcode::VLSE16V,
Opcode::VLSE32V,
Opcode::VLSE64V,
Opcode::VLSE8V,
Opcode::VLUXEI16V,
Opcode::VLUXEI32V,
Opcode::VLUXEI64V,
Opcode::VLUXEI8V,
Opcode::VMACCVV,
Opcode::VMACCVX,
Opcode::VMADCVI,
Opcode::VMADCVIM,
Opcode::VMADCVV,
Opcode::VMADCVVM,
Opcode::VMADCVX,
Opcode::VMADCVXM,
Opcode::VMADDVV,
Opcode::VMADDVX,
Opcode::VMANDMM,
Opcode::VMANDNMM,
Opcode::VMANDNOTMM,
Opcode::VMAXVV,
Opcode::VMAXVX,
Opcode::VMAXUVV,
Opcode::VMAXUVX,
Opcode::VMERGEVIM,
Opcode::VMERGEVVM,
Opcode::VMERGEVXM,
Opcode::VMFEQVF,
Opcode::VMFEQVV,
Opcode::VMFGEVF,
Opcode::VMFGTVF,
Opcode::VMFLEVF,
Opcode::VMFLEVV,
Opcode::VMFLTVF,
Opcode::VMFLTVV,
Opcode::VMFNEVF,
Opcode::VMFNEVV,
Opcode::VMINVV,
Opcode::VMINVX,
Opcode::VMINUVV,
Opcode::VMINUVX,
Opcode::VMNANDMM,
Opcode::VMNORMM,
Opcode::VMORMM,
Opcode::VMORNMM,
Opcode::VMORNOTMM,
Opcode::VMSBCVV,
Opcode::VMSBCVVM,
Opcode::VMSBCVX,
Opcode::VMSBCVXM,
Opcode::VMSBFM,
Opcode::VMSEQVI,
Opcode::VMSEQVV,
Opcode::VMSEQVX,
Opcode::VMSGTVI,
Opcode::VMSGTVX,
Opcode::VMSGTUVI,
Opcode::VMSGTUVX,
Opcode::VMSIFM,
Opcode::VMSLEVI,
Opcode::VMSLEVV,
Opcode::VMSLEVX,
Opcode::VMSLEUVI,
Opcode::VMSLEUVV,
Opcode::VMSLEUVX,
Opcode::VMSLTVV,
Opcode::VMSLTVX,
Opcode::VMSLTUVV,
Opcode::VMSLTUVX,
Opcode::VMSNEVI,
Opcode::VMSNEVV,
Opcode::VMSNEVX,
Opcode::VMSOFM,
Opcode::VMULVV,
Opcode::VMULVX,
Opcode::VMULHVV,
Opcode::VMULHVX,
Opcode::VMULHSUVV,
Opcode::VMULHSUVX,
Opcode::VMULHUVV,
Opcode::VMULHUVX,
Opcode::VMV1RV,
Opcode::VMV2RV,
Opcode::VMV4RV,
Opcode::VMV8RV,
Opcode::VMVSX,
Opcode::VMVVI,
Opcode::VMVVV,
Opcode::VMVVX,
Opcode::VMVXS,
Opcode::VMXNORMM,
Opcode::VMXORMM,
Opcode::VNCLIPWI,
Opcode::VNCLIPWV,
Opcode::VNCLIPWX,
Opcode::VNCLIPUWI,
Opcode::VNCLIPUWV,
Opcode::VNCLIPUWX,
Opcode::VNMSACVV,
Opcode::VNMSACVX,
Opcode::VNMSUBVV,
Opcode::VNMSUBVX,
Opcode::VNSRAWI,
Opcode::VNSRAWV,
Opcode::VNSRAWX,
Opcode::VNSRLWI,
Opcode::VNSRLWV,
Opcode::VNSRLWX,
Opcode::VORVI,
Opcode::VORVV,
Opcode::VORVX,
Opcode::VPOPCM,
Opcode::VREDANDVS,
Opcode::VREDMAXVS,
Opcode::VREDMAXUVS,
Opcode::VREDMINVS,
Opcode::VREDMINUVS,
Opcode::VREDORVS,
Opcode::VREDSUMVS,
Opcode::VREDXORVS,
Opcode::VREMVV,
Opcode::VREMVX,
Opcode::VREMUVV,
Opcode::VREMUVX,
Opcode::VREV8V,
Opcode::VRGATHERVI,
Opcode::VRGATHERVV,
Opcode::VRGATHERVX,
Opcode::VRGATHEREI16VV,
Opcode::VROLVV,
Opcode::VROLVX,
Opcode::VRORVI,
Opcode::VRORVV,
Opcode::VRORVX,
Opcode::VRSUBVI,
Opcode::VRSUBVX,
Opcode::VS1RV,
Opcode::VS2RV,
Opcode::VS4RV,
Opcode::VS8RV,
Opcode::VSADDVI,
Opcode::VSADDVV,
Opcode::VSADDVX,
Opcode::VSADDUVI,
Opcode::VSADDUVV,
Opcode::VSADDUVX,
Opcode::VSBCVVM,
Opcode::VSBCVXM,
Opcode::VSE16V,
Opcode::VSE1V,
Opcode::VSE32V,
Opcode::VSE64V,
Opcode::VSE8V,
Opcode::VSETIVLI,
Opcode::VSETVL,
Opcode::VSETVLI,
Opcode::VSEXTVF2,
Opcode::VSEXTVF4,
Opcode::VSEXTVF8,
Opcode::VSHA2CHVV,
Opcode::VSHA2CLVV,
Opcode::VSHA2MSVV,
Opcode::VSLIDE1DOWNVX,
Opcode::VSLIDE1UPVX,
Opcode::VSLIDEDOWNVI,
Opcode::VSLIDEDOWNVX,
Opcode::VSLIDEUPVI,
Opcode::VSLIDEUPVX,
Opcode::VSLLVI,
Opcode::VSLLVV,
Opcode::VSLLVX,
Opcode::VSM3CVI,
Opcode::VSM3MEVV,
Opcode::VSM4KVI,
Opcode::VSM4RVS,
Opcode::VSM4RVV,
Opcode::VSMV,
Opcode::VSMULVV,
Opcode::VSMULVX,
Opcode::VSOXEI16V,
Opcode::VSOXEI32V,
Opcode::VSOXEI64V,
Opcode::VSOXEI8V,
Opcode::VSRAVI,
Opcode::VSRAVV,
Opcode::VSRAVX,
Opcode::VSRLVI,
Opcode::VSRLVV,
Opcode::VSRLVX,
Opcode::VSSE16V,
Opcode::VSSE32V,
Opcode::VSSE64V,
Opcode::VSSE8V,
Opcode::VSSRAVI,
Opcode::VSSRAVV,
Opcode::VSSRAVX,
Opcode::VSSRLVI,
Opcode::VSSRLVV,
Opcode::VSSRLVX,
Opcode::VSSUBVV,
Opcode::VSSUBVX,
Opcode::VSSUBUVV,
Opcode::VSSUBUVX,
Opcode::VSUBVV,
Opcode::VSUBVX,
Opcode::VSUXEI16V,
Opcode::VSUXEI32V,
Opcode::VSUXEI64V,
Opcode::VSUXEI8V,
Opcode::VWADDVV,
Opcode::VWADDVX,
Opcode::VWADDWV,
Opcode::VWADDWX,
Opcode::VWADDUVV,
Opcode::VWADDUVX,
Opcode::VWADDUWV,
Opcode::VWADDUWX,
Opcode::VWMACCVV,
Opcode::VWMACCVX,
Opcode::VWMACCSUVV,
Opcode::VWMACCSUVX,
Opcode::VWMACCUVV,
Opcode::VWMACCUVX,
Opcode::VWMACCUSVX,
Opcode::VWMULVV,
Opcode::VWMULVX,
Opcode::VWMULSUVV,
Opcode::VWMULSUVX,
Opcode::VWMULUVV,
Opcode::VWMULUVX,
Opcode::VWREDSUMVS,
Opcode::VWREDSUMUVS,
Opcode::VWSLLVI,
Opcode::VWSLLVV,
Opcode::VWSLLVX,
Opcode::VWSUBVV,
Opcode::VWSUBVX,
Opcode::VWSUBWV,
Opcode::VWSUBWX,
Opcode::VWSUBUVV,
Opcode::VWSUBUVX,
Opcode::VWSUBUWV,
Opcode::VWSUBUWX,
Opcode::VXORVI,
Opcode::VXORVV,
Opcode::VXORVX,
Opcode::VZEXTVF2,
Opcode::VZEXTVF4,
Opcode::VZEXTVF8,
Opcode::WFI,
Opcode::WRSNTO,
Opcode::WRSSTO,
Opcode::XNOR,
Opcode::XOR,
Opcode::XORI,
Opcode::XPERM4,
Opcode::XPERM8,
Opcode::ZEXTB,
Opcode::ZEXTH,
Opcode::ZEXTHRV32,
Opcode::ZEXTW,
Opcode::ZIP,
];
pub static SHORT_OPCODE: [bool; 1021] = [
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
true,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
false,
];
pub const SHORT_OPCODES: [Opcode; 106] = [
Opcode::CADD,
Opcode::CADDI,
Opcode::CADDI16SP,
Opcode::CADDI4SPN,
Opcode::CADDIW,
Opcode::CADDW,
Opcode::CAND,
Opcode::CANDI,
Opcode::CBEQZ,
Opcode::CBNEZ,
Opcode::CEBREAK,
Opcode::CFLD,
Opcode::CFLDSP,
Opcode::CFLW,
Opcode::CFLWSP,
Opcode::CFSD,
Opcode::CFSDSP,
Opcode::CFSW,
Opcode::CFSWSP,
Opcode::CJ,
Opcode::CJAL,
Opcode::CJALR,
Opcode::CJR,
Opcode::CLBU,
Opcode::CLD,
Opcode::CLDSP,
Opcode::CLH,
Opcode::CLHU,
Opcode::CLI,
Opcode::CLUI,
Opcode::CLW,
Opcode::CLWSP,
Opcode::CMOP1,
Opcode::CMOP11,
Opcode::CMOP13,
Opcode::CMOP15,
Opcode::CMOP3,
Opcode::CMOP5,
Opcode::CMOP7,
Opcode::CMOP9,
Opcode::CMOPN,
Opcode::CMUL,
Opcode::CMV,
Opcode::CNOP,
Opcode::CNOT,
Opcode::CNTLALL,
Opcode::CNTLP1,
Opcode::CNTLPALL,
Opcode::CNTLS1,
Opcode::COR,
Opcode::CSB,
Opcode::CSD,
Opcode::CSDSP,
Opcode::CSEXTB,
Opcode::CSEXTH,
Opcode::CSEXTW,
Opcode::CSH,
Opcode::CSLLI,
Opcode::CSLLIRV32,
Opcode::CSRAI,
Opcode::CSRAIRV32,
Opcode::CSRLI,
Opcode::CSRLIRV32,
Opcode::CSUB,
Opcode::CSUBW,
Opcode::CSW,
Opcode::CSWSP,
Opcode::CXOR,
Opcode::CZEXTB,
Opcode::CZEXTH,
Opcode::CZEXTW,
Opcode::CBOCLEAN,
Opcode::CBOFLUSH,
Opcode::CBOINVAL,
Opcode::CBOZERO,
Opcode::CLMUL,
Opcode::CLMULH,
Opcode::CLMULR,
Opcode::CLZ,
Opcode::CLZW,
Opcode::CMJALT,
Opcode::CMMVA01S,
Opcode::CMMVSA01,
Opcode::CMPOP,
Opcode::CMPOPRET,
Opcode::CMPOPRETZ,
Opcode::CMPUSH,
Opcode::CPOP,
Opcode::CPOPW,
Opcode::CSRC,
Opcode::CSRCI,
Opcode::CSRR,
Opcode::CSRRC,
Opcode::CSRRCI,
Opcode::CSRRS,
Opcode::CSRRSI,
Opcode::CSRRW,
Opcode::CSRRWI,
Opcode::CSRS,
Opcode::CSRSI,
Opcode::CSRW,
Opcode::CSRWI,
Opcode::CTZ,
Opcode::CTZW,
Opcode::CZEROEQZ,
Opcode::CZERONEZ,
];
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(u32)]
pub enum Opcode {
    ADD,
    ADDUW,
    ADDI,
    ADDIW,
    ADDW,
    AES32DSI,
    AES32DSMI,
    AES32ESI,
    AES32ESMI,
    AES64DS,
    AES64DSM,
    AES64ES,
    AES64ESM,
    AES64IM,
    AES64KS1I,
    AES64KS2,
    AMOADDB,
    AMOADDD,
    AMOADDH,
    AMOADDW,
    AMOANDB,
    AMOANDD,
    AMOANDH,
    AMOANDW,
    AMOCASB,
    AMOCASD,
    AMOCASH,
    AMOCASQ,
    AMOCASW,
    AMOMAXB,
    AMOMAXD,
    AMOMAXH,
    AMOMAXW,
    AMOMAXUB,
    AMOMAXUD,
    AMOMAXUH,
    AMOMAXUW,
    AMOMINB,
    AMOMIND,
    AMOMINH,
    AMOMINW,
    AMOMINUB,
    AMOMINUD,
    AMOMINUH,
    AMOMINUW,
    AMOORB,
    AMOORD,
    AMOORH,
    AMOORW,
    AMOSWAPB,
    AMOSWAPD,
    AMOSWAPH,
    AMOSWAPW,
    AMOXORB,
    AMOXORD,
    AMOXORH,
    AMOXORW,
    AND,
    ANDI,
    ANDN,
    AUIPC,
    BCLR,
    BCLRI,
    BCLRIRV32,
    BEQ,
    BEQZ,
    BEXT,
    BEXTI,
    BEXTIRV32,
    BGE,
    BGEU,
    BGEZ,
    BGT,
    BGTU,
    BGTZ,
    BINV,
    BINVI,
    BINVIRV32,
    BLE,
    BLEU,
    BLEZ,
    BLT,
    BLTU,
    BLTZ,
    BNE,
    BNEZ,
    BREV8,
    BSET,
    BSETI,
    BSETIRV32,
    CADD,
    CADDI,
    CADDI16SP,
    CADDI4SPN,
    CADDIW,
    CADDW,
    CAND,
    CANDI,
    CBEQZ,
    CBNEZ,
    CEBREAK,
    CFLD,
    CFLDSP,
    CFLW,
    CFLWSP,
    CFSD,
    CFSDSP,
    CFSW,
    CFSWSP,
    CJ,
    CJAL,
    CJALR,
    CJR,
    CLBU,
    CLD,
    CLDSP,
    CLH,
    CLHU,
    CLI,
    CLUI,
    CLW,
    CLWSP,
    CMOP1,
    CMOP11,
    CMOP13,
    CMOP15,
    CMOP3,
    CMOP5,
    CMOP7,
    CMOP9,
    CMOPN,
    CMUL,
    CMV,
    CNOP,
    CNOT,
    CNTLALL,
    CNTLP1,
    CNTLPALL,
    CNTLS1,
    COR,
    CSB,
    CSD,
    CSDSP,
    CSEXTB,
    CSEXTH,
    CSEXTW,
    CSH,
    CSLLI,
    CSLLIRV32,
    CSRAI,
    CSRAIRV32,
    CSRLI,
    CSRLIRV32,
    CSUB,
    CSUBW,
    CSW,
    CSWSP,
    CXOR,
    CZEXTB,
    CZEXTH,
    CZEXTW,
    CBOCLEAN,
    CBOFLUSH,
    CBOINVAL,
    CBOZERO,
    CLMUL,
    CLMULH,
    CLMULR,
    CLZ,
    CLZW,
    CMJALT,
    CMMVA01S,
    CMMVSA01,
    CMPOP,
    CMPOPRET,
    CMPOPRETZ,
    CMPUSH,
    CPOP,
    CPOPW,
    CSRC,
    CSRCI,
    CSRR,
    CSRRC,
    CSRRCI,
    CSRRS,
    CSRRSI,
    CSRRW,
    CSRRWI,
    CSRS,
    CSRSI,
    CSRW,
    CSRWI,
    CTZ,
    CTZW,
    CZEROEQZ,
    CZERONEZ,
    DIV,
    DIVU,
    DIVUW,
    DIVW,
    DRET,
    EBREAK,
    ECALL,
    FABSD,
    FABSH,
    FABSQ,
    FABSS,
    FADDD,
    FADDH,
    FADDQ,
    FADDS,
    FCLASSD,
    FCLASSH,
    FCLASSQ,
    FCLASSS,
    FCVTDH,
    FCVTDL,
    FCVTDLU,
    FCVTDQ,
    FCVTDS,
    FCVTDW,
    FCVTDWU,
    FCVTHD,
    FCVTHL,
    FCVTHLU,
    FCVTHQ,
    FCVTHS,
    FCVTHW,
    FCVTHWU,
    FCVTLD,
    FCVTLH,
    FCVTLQ,
    FCVTLS,
    FCVTLUD,
    FCVTLUH,
    FCVTLUQ,
    FCVTLUS,
    FCVTQD,
    FCVTQH,
    FCVTQL,
    FCVTQLU,
    FCVTQS,
    FCVTQW,
    FCVTQWU,
    FCVTSD,
    FCVTSH,
    FCVTSL,
    FCVTSLU,
    FCVTSQ,
    FCVTSW,
    FCVTSWU,
    FCVTWD,
    FCVTWH,
    FCVTWQ,
    FCVTWS,
    FCVTWUD,
    FCVTWUH,
    FCVTWUQ,
    FCVTWUS,
    FCVTMODWD,
    FDIVD,
    FDIVH,
    FDIVQ,
    FDIVS,
    FENCE,
    FENCEI,
    FENCETSO,
    FEQD,
    FEQH,
    FEQQ,
    FEQS,
    FLD,
    FLED,
    FLEH,
    FLEQ,
    FLES,
    FLEQD,
    FLEQH,
    FLEQQ,
    FLEQS,
    FLH,
    FLID,
    FLIH,
    FLIQ,
    FLIS,
    FLQ,
    FLTD,
    FLTH,
    FLTQ,
    FLTS,
    FLTQD,
    FLTQH,
    FLTQQ,
    FLTQS,
    FLW,
    FMADDD,
    FMADDH,
    FMADDQ,
    FMADDS,
    FMAXD,
    FMAXH,
    FMAXQ,
    FMAXS,
    FMAXMD,
    FMAXMH,
    FMAXMQ,
    FMAXMS,
    FMIND,
    FMINH,
    FMINQ,
    FMINS,
    FMINMD,
    FMINMH,
    FMINMQ,
    FMINMS,
    FMSUBD,
    FMSUBH,
    FMSUBQ,
    FMSUBS,
    FMULD,
    FMULH,
    FMULQ,
    FMULS,
    FMVD,
    FMVDX,
    FMVH,
    FMVHX,
    FMVQ,
    FMVS,
    FMVSX,
    FMVWX,
    FMVXD,
    FMVXH,
    FMVXS,
    FMVXW,
    FMVHXD,
    FMVHXQ,
    FMVPDX,
    FMVPQX,
    FNEGD,
    FNEGH,
    FNEGQ,
    FNEGS,
    FNMADDD,
    FNMADDH,
    FNMADDQ,
    FNMADDS,
    FNMSUBD,
    FNMSUBH,
    FNMSUBQ,
    FNMSUBS,
    FRCSR,
    FRFLAGS,
    FROUNDD,
    FROUNDH,
    FROUNDQ,
    FROUNDS,
    FROUNDNXD,
    FROUNDNXH,
    FROUNDNXQ,
    FROUNDNXS,
    FRRM,
    FSCSR,
    FSD,
    FSFLAGS,
    FSFLAGSI,
    FSGNJD,
    FSGNJH,
    FSGNJQ,
    FSGNJS,
    FSGNJND,
    FSGNJNH,
    FSGNJNQ,
    FSGNJNS,
    FSGNJXD,
    FSGNJXH,
    FSGNJXQ,
    FSGNJXS,
    FSH,
    FSQ,
    FSQRTD,
    FSQRTH,
    FSQRTQ,
    FSQRTS,
    FSRM,
    FSRMI,
    FSUBD,
    FSUBH,
    FSUBQ,
    FSUBS,
    FSW,
    HFENCEGVMA,
    HFENCEVVMA,
    HINVALGVMA,
    HINVALVVMA,
    HLVB,
    HLVBU,
    HLVD,
    HLVH,
    HLVHU,
    HLVW,
    HLVWU,
    HLVXHU,
    HLVXWU,
    HSVB,
    HSVD,
    HSVH,
    HSVW,
    J,
    JAL,
    JALPSEUDO,
    JALR,
    JALRPSEUDO,
    JR,
    LB,
    LBU,
    LD,
    LH,
    LHU,
    LRD,
    LRW,
    LUI,
    LW,
    LWU,
    MAX,
    MAXU,
    MIN,
    MINU,
    MOPR0,
    MOPR1,
    MOPR10,
    MOPR11,
    MOPR12,
    MOPR13,
    MOPR14,
    MOPR15,
    MOPR16,
    MOPR17,
    MOPR18,
    MOPR19,
    MOPR2,
    MOPR20,
    MOPR21,
    MOPR22,
    MOPR23,
    MOPR24,
    MOPR25,
    MOPR26,
    MOPR27,
    MOPR28,
    MOPR29,
    MOPR3,
    MOPR30,
    MOPR31,
    MOPR4,
    MOPR5,
    MOPR6,
    MOPR7,
    MOPR8,
    MOPR9,
    MOPRN,
    MOPRR0,
    MOPRR1,
    MOPRR2,
    MOPRR3,
    MOPRR4,
    MOPRR5,
    MOPRR6,
    MOPRR7,
    MOPRRN,
    MRET,
    MUL,
    MULH,
    MULHSU,
    MULHU,
    MULW,
    MV,
    NEG,
    NOP,
    NTLALL,
    NTLP1,
    NTLPALL,
    NTLS1,
    OR,
    ORCB,
    ORI,
    ORN,
    PACK,
    PACKH,
    PACKW,
    PAUSE,
    PREFETCHI,
    PREFETCHR,
    PREFETCHW,
    RDCYCLE,
    RDCYCLEH,
    RDINSTRET,
    RDINSTRETH,
    RDTIME,
    RDTIMEH,
    REM,
    REMU,
    REMUW,
    REMW,
    RET,
    REV8,
    REV8RV32,
    ROL,
    ROLW,
    ROR,
    RORI,
    RORIRV32,
    RORIW,
    RORW,
    SB,
    SBREAK,
    SCD,
    SCW,
    SCALL,
    SD,
    SEQZ,
    SEXTB,
    SEXTH,
    SEXTW,
    SFENCEINVALIR,
    SFENCEVMA,
    SFENCEWINVAL,
    SGTZ,
    SH,
    SH1ADD,
    SH1ADDUW,
    SH2ADD,
    SH2ADDUW,
    SH3ADD,
    SH3ADDUW,
    SHA256SIG0,
    SHA256SIG1,
    SHA256SUM0,
    SHA256SUM1,
    SHA512SIG0,
    SHA512SIG0H,
    SHA512SIG0L,
    SHA512SIG1,
    SHA512SIG1H,
    SHA512SIG1L,
    SHA512SUM0,
    SHA512SUM0R,
    SHA512SUM1,
    SHA512SUM1R,
    SINVALVMA,
    SLL,
    SLLI,
    SLLIRV32,
    SLLIUW,
    SLLIW,
    SLLW,
    SLT,
    SLTI,
    SLTIU,
    SLTU,
    SLTZ,
    SM3P0,
    SM3P1,
    SM4ED,
    SM4KS,
    SNEZ,
    SRA,
    SRAI,
    SRAIRV32,
    SRAIW,
    SRAW,
    SRET,
    SRL,
    SRLI,
    SRLIRV32,
    SRLIW,
    SRLW,
    SUB,
    SUBW,
    SW,
    UNZIP,
    VAADDVV,
    VAADDVX,
    VAADDUVV,
    VAADDUVX,
    VADCVIM,
    VADCVVM,
    VADCVXM,
    VADDVI,
    VADDVV,
    VADDVX,
    VAESDFVS,
    VAESDFVV,
    VAESDMVS,
    VAESDMVV,
    VAESEFVS,
    VAESEFVV,
    VAESEMVS,
    VAESEMVV,
    VAESKF1VI,
    VAESKF2VI,
    VAESZVS,
    VANDVI,
    VANDVV,
    VANDVX,
    VANDNVV,
    VANDNVX,
    VASUBVV,
    VASUBVX,
    VASUBUVV,
    VASUBUVX,
    VBREV8V,
    VBREVV,
    VCLMULVV,
    VCLMULVX,
    VCLMULHVV,
    VCLMULHVX,
    VCLZV,
    VCOMPRESSVM,
    VCPOPM,
    VCPOPV,
    VCTZV,
    VDIVVV,
    VDIVVX,
    VDIVUVV,
    VDIVUVX,
    VFADDVF,
    VFADDVV,
    VFCLASSV,
    VFCVTFXV,
    VFCVTFXUV,
    VFCVTRTZXFV,
    VFCVTRTZXUFV,
    VFCVTXFV,
    VFCVTXUFV,
    VFDIVVF,
    VFDIVVV,
    VFIRSTM,
    VFMACCVF,
    VFMACCVV,
    VFMADDVF,
    VFMADDVV,
    VFMAXVF,
    VFMAXVV,
    VFMERGEVFM,
    VFMINVF,
    VFMINVV,
    VFMSACVF,
    VFMSACVV,
    VFMSUBVF,
    VFMSUBVV,
    VFMULVF,
    VFMULVV,
    VFMVFS,
    VFMVSF,
    VFMVVF,
    VFNCVTFFW,
    VFNCVTFXW,
    VFNCVTFXUW,
    VFNCVTRODFFW,
    VFNCVTRTZXFW,
    VFNCVTRTZXUFW,
    VFNCVTXFW,
    VFNCVTXUFW,
    VFNMACCVF,
    VFNMACCVV,
    VFNMADDVF,
    VFNMADDVV,
    VFNMSACVF,
    VFNMSACVV,
    VFNMSUBVF,
    VFNMSUBVV,
    VFRDIVVF,
    VFREC7V,
    VFREDMAXVS,
    VFREDMINVS,
    VFREDOSUMVS,
    VFREDSUMVS,
    VFREDUSUMVS,
    VFRSQRT7V,
    VFRSUBVF,
    VFSGNJVF,
    VFSGNJVV,
    VFSGNJNVF,
    VFSGNJNVV,
    VFSGNJXVF,
    VFSGNJXVV,
    VFSLIDE1DOWNVF,
    VFSLIDE1UPVF,
    VFSQRTV,
    VFSUBVF,
    VFSUBVV,
    VFWADDVF,
    VFWADDVV,
    VFWADDWF,
    VFWADDWV,
    VFWCVTFFV,
    VFWCVTFXV,
    VFWCVTFXUV,
    VFWCVTRTZXFV,
    VFWCVTRTZXUFV,
    VFWCVTXFV,
    VFWCVTXUFV,
    VFWMACCVF,
    VFWMACCVV,
    VFWMSACVF,
    VFWMSACVV,
    VFWMULVF,
    VFWMULVV,
    VFWNMACCVF,
    VFWNMACCVV,
    VFWNMSACVF,
    VFWNMSACVV,
    VFWREDOSUMVS,
    VFWREDSUMVS,
    VFWREDUSUMVS,
    VFWSUBVF,
    VFWSUBVV,
    VFWSUBWF,
    VFWSUBWV,
    VGHSHVV,
    VGMULVV,
    VIDV,
    VIOTAM,
    VL1RV,
    VL1RE16V,
    VL1RE32V,
    VL1RE64V,
    VL1RE8V,
    VL2RV,
    VL2RE16V,
    VL2RE32V,
    VL2RE64V,
    VL2RE8V,
    VL4RV,
    VL4RE16V,
    VL4RE32V,
    VL4RE64V,
    VL4RE8V,
    VL8RV,
    VL8RE16V,
    VL8RE32V,
    VL8RE64V,
    VL8RE8V,
    VLE16V,
    VLE16FFV,
    VLE1V,
    VLE32V,
    VLE32FFV,
    VLE64V,
    VLE64FFV,
    VLE8V,
    VLE8FFV,
    VLMV,
    VLOXEI16V,
    VLOXEI32V,
    VLOXEI64V,
    VLOXEI8V,
    VLSE16V,
    VLSE32V,
    VLSE64V,
    VLSE8V,
    VLUXEI16V,
    VLUXEI32V,
    VLUXEI64V,
    VLUXEI8V,
    VMACCVV,
    VMACCVX,
    VMADCVI,
    VMADCVIM,
    VMADCVV,
    VMADCVVM,
    VMADCVX,
    VMADCVXM,
    VMADDVV,
    VMADDVX,
    VMANDMM,
    VMANDNMM,
    VMANDNOTMM,
    VMAXVV,
    VMAXVX,
    VMAXUVV,
    VMAXUVX,
    VMERGEVIM,
    VMERGEVVM,
    VMERGEVXM,
    VMFEQVF,
    VMFEQVV,
    VMFGEVF,
    VMFGTVF,
    VMFLEVF,
    VMFLEVV,
    VMFLTVF,
    VMFLTVV,
    VMFNEVF,
    VMFNEVV,
    VMINVV,
    VMINVX,
    VMINUVV,
    VMINUVX,
    VMNANDMM,
    VMNORMM,
    VMORMM,
    VMORNMM,
    VMORNOTMM,
    VMSBCVV,
    VMSBCVVM,
    VMSBCVX,
    VMSBCVXM,
    VMSBFM,
    VMSEQVI,
    VMSEQVV,
    VMSEQVX,
    VMSGTVI,
    VMSGTVX,
    VMSGTUVI,
    VMSGTUVX,
    VMSIFM,
    VMSLEVI,
    VMSLEVV,
    VMSLEVX,
    VMSLEUVI,
    VMSLEUVV,
    VMSLEUVX,
    VMSLTVV,
    VMSLTVX,
    VMSLTUVV,
    VMSLTUVX,
    VMSNEVI,
    VMSNEVV,
    VMSNEVX,
    VMSOFM,
    VMULVV,
    VMULVX,
    VMULHVV,
    VMULHVX,
    VMULHSUVV,
    VMULHSUVX,
    VMULHUVV,
    VMULHUVX,
    VMV1RV,
    VMV2RV,
    VMV4RV,
    VMV8RV,
    VMVSX,
    VMVVI,
    VMVVV,
    VMVVX,
    VMVXS,
    VMXNORMM,
    VMXORMM,
    VNCLIPWI,
    VNCLIPWV,
    VNCLIPWX,
    VNCLIPUWI,
    VNCLIPUWV,
    VNCLIPUWX,
    VNMSACVV,
    VNMSACVX,
    VNMSUBVV,
    VNMSUBVX,
    VNSRAWI,
    VNSRAWV,
    VNSRAWX,
    VNSRLWI,
    VNSRLWV,
    VNSRLWX,
    VORVI,
    VORVV,
    VORVX,
    VPOPCM,
    VREDANDVS,
    VREDMAXVS,
    VREDMAXUVS,
    VREDMINVS,
    VREDMINUVS,
    VREDORVS,
    VREDSUMVS,
    VREDXORVS,
    VREMVV,
    VREMVX,
    VREMUVV,
    VREMUVX,
    VREV8V,
    VRGATHERVI,
    VRGATHERVV,
    VRGATHERVX,
    VRGATHEREI16VV,
    VROLVV,
    VROLVX,
    VRORVI,
    VRORVV,
    VRORVX,
    VRSUBVI,
    VRSUBVX,
    VS1RV,
    VS2RV,
    VS4RV,
    VS8RV,
    VSADDVI,
    VSADDVV,
    VSADDVX,
    VSADDUVI,
    VSADDUVV,
    VSADDUVX,
    VSBCVVM,
    VSBCVXM,
    VSE16V,
    VSE1V,
    VSE32V,
    VSE64V,
    VSE8V,
    VSETIVLI,
    VSETVL,
    VSETVLI,
    VSEXTVF2,
    VSEXTVF4,
    VSEXTVF8,
    VSHA2CHVV,
    VSHA2CLVV,
    VSHA2MSVV,
    VSLIDE1DOWNVX,
    VSLIDE1UPVX,
    VSLIDEDOWNVI,
    VSLIDEDOWNVX,
    VSLIDEUPVI,
    VSLIDEUPVX,
    VSLLVI,
    VSLLVV,
    VSLLVX,
    VSM3CVI,
    VSM3MEVV,
    VSM4KVI,
    VSM4RVS,
    VSM4RVV,
    VSMV,
    VSMULVV,
    VSMULVX,
    VSOXEI16V,
    VSOXEI32V,
    VSOXEI64V,
    VSOXEI8V,
    VSRAVI,
    VSRAVV,
    VSRAVX,
    VSRLVI,
    VSRLVV,
    VSRLVX,
    VSSE16V,
    VSSE32V,
    VSSE64V,
    VSSE8V,
    VSSRAVI,
    VSSRAVV,
    VSSRAVX,
    VSSRLVI,
    VSSRLVV,
    VSSRLVX,
    VSSUBVV,
    VSSUBVX,
    VSSUBUVV,
    VSSUBUVX,
    VSUBVV,
    VSUBVX,
    VSUXEI16V,
    VSUXEI32V,
    VSUXEI64V,
    VSUXEI8V,
    VWADDVV,
    VWADDVX,
    VWADDWV,
    VWADDWX,
    VWADDUVV,
    VWADDUVX,
    VWADDUWV,
    VWADDUWX,
    VWMACCVV,
    VWMACCVX,
    VWMACCSUVV,
    VWMACCSUVX,
    VWMACCUVV,
    VWMACCUVX,
    VWMACCUSVX,
    VWMULVV,
    VWMULVX,
    VWMULSUVV,
    VWMULSUVX,
    VWMULUVV,
    VWMULUVX,
    VWREDSUMVS,
    VWREDSUMUVS,
    VWSLLVI,
    VWSLLVV,
    VWSLLVX,
    VWSUBVV,
    VWSUBVX,
    VWSUBWV,
    VWSUBWX,
    VWSUBUVV,
    VWSUBUVX,
    VWSUBUWV,
    VWSUBUWX,
    VXORVI,
    VXORVV,
    VXORVX,
    VZEXTVF2,
    VZEXTVF4,
    VZEXTVF8,
    WFI,
    WRSNTO,
    WRSSTO,
    XNOR,
    XOR,
    XORI,
    XPERM4,
    XPERM8,
    ZEXTB,
    ZEXTH,
    ZEXTHRV32,
    ZEXTW,
    ZIP,
    Invalid,
}
/*
pub const OPCODE_STR: &[&str] = &[
    "add",
    "add.uw",
    "addi",
    "addiw",
    "addw",
    "aes32dsi",
    "aes32dsmi",
    "aes32esi",
    "aes32esmi",
    "aes64ds",
    "aes64dsm",
    "aes64es",
    "aes64esm",
    "aes64im",
    "aes64ks1i",
    "aes64ks2",
    "amoadd.b",
    "amoadd.d",
    "amoadd.h",
    "amoadd.w",
    "amoand.b",
    "amoand.d",
    "amoand.h",
    "amoand.w",
    "amocas.b",
    "amocas.d",
    "amocas.h",
    "amocas.q",
    "amocas.w",
    "amomax.b",
    "amomax.d",
    "amomax.h",
    "amomax.w",
    "amomaxu.b",
    "amomaxu.d",
    "amomaxu.h",
    "amomaxu.w",
    "amomin.b",
    "amomin.d",
    "amomin.h",
    "amomin.w",
    "amominu.b",
    "amominu.d",
    "amominu.h",
    "amominu.w",
    "amoor.b",
    "amoor.d",
    "amoor.h",
    "amoor.w",
    "amoswap.b",
    "amoswap.d",
    "amoswap.h",
    "amoswap.w",
    "amoxor.b",
    "amoxor.d",
    "amoxor.h",
    "amoxor.w",
    "and",
    "andi",
    "andn",
    "auipc",
    "bclr",
    "bclri",
    "bclri.rv32",
    "beq",
    "beqz",
    "bext",
    "bexti",
    "bexti.rv32",
    "bge",
    "bgeu",
    "bgez",
    "bgt",
    "bgtu",
    "bgtz",
    "binv",
    "binvi",
    "binvi.rv32",
    "ble",
    "bleu",
    "blez",
    "blt",
    "bltu",
    "bltz",
    "bne",
    "bnez",
    "brev8",
    "bset",
    "bseti",
    "bseti.rv32",
    "c.add",
    "c.addi",
    "c.addi16sp",
    "c.addi4spn",
    "c.addiw",
    "c.addw",
    "c.and",
    "c.andi",
    "c.beqz",
    "c.bnez",
    "c.ebreak",
    "c.fld",
    "c.fldsp",
    "c.flw",
    "c.flwsp",
    "c.fsd",
    "c.fsdsp",
    "c.fsw",
    "c.fswsp",
    "c.j",
    "c.jal",
    "c.jalr",
    "c.jr",
    "c.lbu",
    "c.ld",
    "c.ldsp",
    "c.lh",
    "c.lhu",
    "c.li",
    "c.lui",
    "c.lw",
    "c.lwsp",
    "c.mop.1",
    "c.mop.11",
    "c.mop.13",
    "c.mop.15",
    "c.mop.3",
    "c.mop.5",
    "c.mop.7",
    "c.mop.9",
    "c.mop.n",
    "c.mul",
    "c.mv",
    "c.nop",
    "c.not",
    "c.ntl.all",
    "c.ntl.p1",
    "c.ntl.pall",
    "c.ntl.s1",
    "c.or",
    "c.sb",
    "c.sd",
    "c.sdsp",
    "c.sext.b",
    "c.sext.h",
    "c.sext.w",
    "c.sh",
    "c.slli",
    "c.slli.rv32",
    "c.srai",
    "c.srai.rv32",
    "c.srli",
    "c.srli.rv32",
    "c.sub",
    "c.subw",
    "c.sw",
    "c.swsp",
    "c.xor",
    "c.zext.b",
    "c.zext.h",
    "c.zext.w",
    "cbo.clean",
    "cbo.flush",
    "cbo.inval",
    "cbo.zero",
    "clmul",
    "clmulh",
    "clmulr",
    "clz",
    "clzw",
    "cm.jalt",
    "cm.mva01s",
    "cm.mvsa01",
    "cm.pop",
    "cm.popret",
    "cm.popretz",
    "cm.push",
    "cpop",
    "cpopw",
    "csrc",
    "csrci",
    "csrr",
    "csrrc",
    "csrrci",
    "csrrs",
    "csrrsi",
    "csrrw",
    "csrrwi",
    "csrs",
    "csrsi",
    "csrw",
    "csrwi",
    "ctz",
    "ctzw",
    "czero.eqz",
    "czero.nez",
    "div",
    "divu",
    "divuw",
    "divw",
    "dret",
    "ebreak",
    "ecall",
    "fabs.d",
    "fabs.h",
    "fabs.q",
    "fabs.s",
    "fadd.d",
    "fadd.h",
    "fadd.q",
    "fadd.s",
    "fclass.d",
    "fclass.h",
    "fclass.q",
    "fclass.s",
    "fcvt.d.h",
    "fcvt.d.l",
    "fcvt.d.lu",
    "fcvt.d.q",
    "fcvt.d.s",
    "fcvt.d.w",
    "fcvt.d.wu",
    "fcvt.h.d",
    "fcvt.h.l",
    "fcvt.h.lu",
    "fcvt.h.q",
    "fcvt.h.s",
    "fcvt.h.w",
    "fcvt.h.wu",
    "fcvt.l.d",
    "fcvt.l.h",
    "fcvt.l.q",
    "fcvt.l.s",
    "fcvt.lu.d",
    "fcvt.lu.h",
    "fcvt.lu.q",
    "fcvt.lu.s",
    "fcvt.q.d",
    "fcvt.q.h",
    "fcvt.q.l",
    "fcvt.q.lu",
    "fcvt.q.s",
    "fcvt.q.w",
    "fcvt.q.wu",
    "fcvt.s.d",
    "fcvt.s.h",
    "fcvt.s.l",
    "fcvt.s.lu",
    "fcvt.s.q",
    "fcvt.s.w",
    "fcvt.s.wu",
    "fcvt.w.d",
    "fcvt.w.h",
    "fcvt.w.q",
    "fcvt.w.s",
    "fcvt.wu.d",
    "fcvt.wu.h",
    "fcvt.wu.q",
    "fcvt.wu.s",
    "fcvtmod.w.d",
    "fdiv.d",
    "fdiv.h",
    "fdiv.q",
    "fdiv.s",
    "fence",
    "fence.i",
    "fence.tso",
    "feq.d",
    "feq.h",
    "feq.q",
    "feq.s",
    "fld",
    "fle.d",
    "fle.h",
    "fle.q",
    "fle.s",
    "fleq.d",
    "fleq.h",
    "fleq.q",
    "fleq.s",
    "flh",
    "fli.d",
    "fli.h",
    "fli.q",
    "fli.s",
    "flq",
    "flt.d",
    "flt.h",
    "flt.q",
    "flt.s",
    "fltq.d",
    "fltq.h",
    "fltq.q",
    "fltq.s",
    "flw",
    "fmadd.d",
    "fmadd.h",
    "fmadd.q",
    "fmadd.s",
    "fmax.d",
    "fmax.h",
    "fmax.q",
    "fmax.s",
    "fmaxm.d",
    "fmaxm.h",
    "fmaxm.q",
    "fmaxm.s",
    "fmin.d",
    "fmin.h",
    "fmin.q",
    "fmin.s",
    "fminm.d",
    "fminm.h",
    "fminm.q",
    "fminm.s",
    "fmsub.d",
    "fmsub.h",
    "fmsub.q",
    "fmsub.s",
    "fmul.d",
    "fmul.h",
    "fmul.q",
    "fmul.s",
    "fmv.d",
    "fmv.d.x",
    "fmv.h",
    "fmv.h.x",
    "fmv.q",
    "fmv.s",
    "fmv.s.x",
    "fmv.w.x",
    "fmv.x.d",
    "fmv.x.h",
    "fmv.x.s",
    "fmv.x.w",
    "fmvh.x.d",
    "fmvh.x.q",
    "fmvp.d.x",
    "fmvp.q.x",
    "fneg.d",
    "fneg.h",
    "fneg.q",
    "fneg.s",
    "fnmadd.d",
    "fnmadd.h",
    "fnmadd.q",
    "fnmadd.s",
    "fnmsub.d",
    "fnmsub.h",
    "fnmsub.q",
    "fnmsub.s",
    "frcsr",
    "frflags",
    "fround.d",
    "fround.h",
    "fround.q",
    "fround.s",
    "froundnx.d",
    "froundnx.h",
    "froundnx.q",
    "froundnx.s",
    "frrm",
    "fscsr",
    "fsd",
    "fsflags",
    "fsflagsi",
    "fsgnj.d",
    "fsgnj.h",
    "fsgnj.q",
    "fsgnj.s",
    "fsgnjn.d",
    "fsgnjn.h",
    "fsgnjn.q",
    "fsgnjn.s",
    "fsgnjx.d",
    "fsgnjx.h",
    "fsgnjx.q",
    "fsgnjx.s",
    "fsh",
    "fsq",
    "fsqrt.d",
    "fsqrt.h",
    "fsqrt.q",
    "fsqrt.s",
    "fsrm",
    "fsrmi",
    "fsub.d",
    "fsub.h",
    "fsub.q",
    "fsub.s",
    "fsw",
    "hfence.gvma",
    "hfence.vvma",
    "hinval.gvma",
    "hinval.vvma",
    "hlv.b",
    "hlv.bu",
    "hlv.d",
    "hlv.h",
    "hlv.hu",
    "hlv.w",
    "hlv.wu",
    "hlvx.hu",
    "hlvx.wu",
    "hsv.b",
    "hsv.d",
    "hsv.h",
    "hsv.w",
    "j",
    "jal",
    "jal.pseudo",
    "jalr",
    "jalr.pseudo",
    "jr",
    "lb",
    "lbu",
    "ld",
    "lh",
    "lhu",
    "lr.d",
    "lr.w",
    "lui",
    "lw",
    "lwu",
    "max",
    "maxu",
    "min",
    "minu",
    "mop.r.0",
    "mop.r.1",
    "mop.r.10",
    "mop.r.11",
    "mop.r.12",
    "mop.r.13",
    "mop.r.14",
    "mop.r.15",
    "mop.r.16",
    "mop.r.17",
    "mop.r.18",
    "mop.r.19",
    "mop.r.2",
    "mop.r.20",
    "mop.r.21",
    "mop.r.22",
    "mop.r.23",
    "mop.r.24",
    "mop.r.25",
    "mop.r.26",
    "mop.r.27",
    "mop.r.28",
    "mop.r.29",
    "mop.r.3",
    "mop.r.30",
    "mop.r.31",
    "mop.r.4",
    "mop.r.5",
    "mop.r.6",
    "mop.r.7",
    "mop.r.8",
    "mop.r.9",
    "mop.r.n",
    "mop.rr.0",
    "mop.rr.1",
    "mop.rr.2",
    "mop.rr.3",
    "mop.rr.4",
    "mop.rr.5",
    "mop.rr.6",
    "mop.rr.7",
    "mop.rr.n",
    "mret",
    "mul",
    "mulh",
    "mulhsu",
    "mulhu",
    "mulw",
    "mv",
    "neg",
    "nop",
    "ntl.all",
    "ntl.p1",
    "ntl.pall",
    "ntl.s1",
    "or",
    "orc.b",
    "ori",
    "orn",
    "pack",
    "packh",
    "packw",
    "pause",
    "prefetch.i",
    "prefetch.r",
    "prefetch.w",
    "rdcycle",
    "rdcycleh",
    "rdinstret",
    "rdinstreth",
    "rdtime",
    "rdtimeh",
    "rem",
    "remu",
    "remuw",
    "remw",
    "ret",
    "rev8",
    "rev8.rv32",
    "rol",
    "rolw",
    "ror",
    "rori",
    "rori.rv32",
    "roriw",
    "rorw",
    "sb",
    "sbreak",
    "sc.d",
    "sc.w",
    "scall",
    "sd",
    "seqz",
    "sext.b",
    "sext.h",
    "sext.w",
    "sfence.inval.ir",
    "sfence.vma",
    "sfence.w.inval",
    "sgtz",
    "sh",
    "sh1add",
    "sh1add.uw",
    "sh2add",
    "sh2add.uw",
    "sh3add",
    "sh3add.uw",
    "sha256sig0",
    "sha256sig1",
    "sha256sum0",
    "sha256sum1",
    "sha512sig0",
    "sha512sig0h",
    "sha512sig0l",
    "sha512sig1",
    "sha512sig1h",
    "sha512sig1l",
    "sha512sum0",
    "sha512sum0r",
    "sha512sum1",
    "sha512sum1r",
    "sinval.vma",
    "sll",
    "slli",
    "slli.rv32",
    "slli.uw",
    "slliw",
    "sllw",
    "slt",
    "slti",
    "sltiu",
    "sltu",
    "sltz",
    "sm3p0",
    "sm3p1",
    "sm4ed",
    "sm4ks",
    "snez",
    "sra",
    "srai",
    "srai.rv32",
    "sraiw",
    "sraw",
    "sret",
    "srl",
    "srli",
    "srli.rv32",
    "srliw",
    "srlw",
    "sub",
    "subw",
    "sw",
    "unzip",
    "vaadd.vv",
    "vaadd.vx",
    "vaaddu.vv",
    "vaaddu.vx",
    "vadc.vim",
    "vadc.vvm",
    "vadc.vxm",
    "vadd.vi",
    "vadd.vv",
    "vadd.vx",
    "vaesdf.vs",
    "vaesdf.vv",
    "vaesdm.vs",
    "vaesdm.vv",
    "vaesef.vs",
    "vaesef.vv",
    "vaesem.vs",
    "vaesem.vv",
    "vaeskf1.vi",
    "vaeskf2.vi",
    "vaesz.vs",
    "vand.vi",
    "vand.vv",
    "vand.vx",
    "vandn.vv",
    "vandn.vx",
    "vasub.vv",
    "vasub.vx",
    "vasubu.vv",
    "vasubu.vx",
    "vbrev8.v",
    "vbrev.v",
    "vclmul.vv",
    "vclmul.vx",
    "vclmulh.vv",
    "vclmulh.vx",
    "vclz.v",
    "vcompress.vm",
    "vcpop.m",
    "vcpop.v",
    "vctz.v",
    "vdiv.vv",
    "vdiv.vx",
    "vdivu.vv",
    "vdivu.vx",
    "vfadd.vf",
    "vfadd.vv",
    "vfclass.v",
    "vfcvt.f.x.v",
    "vfcvt.f.xu.v",
    "vfcvt.rtz.x.f.v",
    "vfcvt.rtz.xu.f.v",
    "vfcvt.x.f.v",
    "vfcvt.xu.f.v",
    "vfdiv.vf",
    "vfdiv.vv",
    "vfirst.m",
    "vfmacc.vf",
    "vfmacc.vv",
    "vfmadd.vf",
    "vfmadd.vv",
    "vfmax.vf",
    "vfmax.vv",
    "vfmerge.vfm",
    "vfmin.vf",
    "vfmin.vv",
    "vfmsac.vf",
    "vfmsac.vv",
    "vfmsub.vf",
    "vfmsub.vv",
    "vfmul.vf",
    "vfmul.vv",
    "vfmv.f.s",
    "vfmv.s.f",
    "vfmv.v.f",
    "vfncvt.f.f.w",
    "vfncvt.f.x.w",
    "vfncvt.f.xu.w",
    "vfncvt.rod.f.f.w",
    "vfncvt.rtz.x.f.w",
    "vfncvt.rtz.xu.f.w",
    "vfncvt.x.f.w",
    "vfncvt.xu.f.w",
    "vfnmacc.vf",
    "vfnmacc.vv",
    "vfnmadd.vf",
    "vfnmadd.vv",
    "vfnmsac.vf",
    "vfnmsac.vv",
    "vfnmsub.vf",
    "vfnmsub.vv",
    "vfrdiv.vf",
    "vfrec7.v",
    "vfredmax.vs",
    "vfredmin.vs",
    "vfredosum.vs",
    "vfredsum.vs",
    "vfredusum.vs",
    "vfrsqrt7.v",
    "vfrsub.vf",
    "vfsgnj.vf",
    "vfsgnj.vv",
    "vfsgnjn.vf",
    "vfsgnjn.vv",
    "vfsgnjx.vf",
    "vfsgnjx.vv",
    "vfslide1down.vf",
    "vfslide1up.vf",
    "vfsqrt.v",
    "vfsub.vf",
    "vfsub.vv",
    "vfwadd.vf",
    "vfwadd.vv",
    "vfwadd.wf",
    "vfwadd.wv",
    "vfwcvt.f.f.v",
    "vfwcvt.f.x.v",
    "vfwcvt.f.xu.v",
    "vfwcvt.rtz.x.f.v",
    "vfwcvt.rtz.xu.f.v",
    "vfwcvt.x.f.v",
    "vfwcvt.xu.f.v",
    "vfwmacc.vf",
    "vfwmacc.vv",
    "vfwmsac.vf",
    "vfwmsac.vv",
    "vfwmul.vf",
    "vfwmul.vv",
    "vfwnmacc.vf",
    "vfwnmacc.vv",
    "vfwnmsac.vf",
    "vfwnmsac.vv",
    "vfwredosum.vs",
    "vfwredsum.vs",
    "vfwredusum.vs",
    "vfwsub.vf",
    "vfwsub.vv",
    "vfwsub.wf",
    "vfwsub.wv",
    "vghsh.vv",
    "vgmul.vv",
    "vid.v",
    "viota.m",
    "vl1r.v",
    "vl1re16.v",
    "vl1re32.v",
    "vl1re64.v",
    "vl1re8.v",
    "vl2r.v",
    "vl2re16.v",
    "vl2re32.v",
    "vl2re64.v",
    "vl2re8.v",
    "vl4r.v",
    "vl4re16.v",
    "vl4re32.v",
    "vl4re64.v",
    "vl4re8.v",
    "vl8r.v",
    "vl8re16.v",
    "vl8re32.v",
    "vl8re64.v",
    "vl8re8.v",
    "vle16.v",
    "vle16ff.v",
    "vle1.v",
    "vle32.v",
    "vle32ff.v",
    "vle64.v",
    "vle64ff.v",
    "vle8.v",
    "vle8ff.v",
    "vlm.v",
    "vloxei16.v",
    "vloxei32.v",
    "vloxei64.v",
    "vloxei8.v",
    "vlse16.v",
    "vlse32.v",
    "vlse64.v",
    "vlse8.v",
    "vluxei16.v",
    "vluxei32.v",
    "vluxei64.v",
    "vluxei8.v",
    "vmacc.vv",
    "vmacc.vx",
    "vmadc.vi",
    "vmadc.vim",
    "vmadc.vv",
    "vmadc.vvm",
    "vmadc.vx",
    "vmadc.vxm",
    "vmadd.vv",
    "vmadd.vx",
    "vmand.mm",
    "vmandn.mm",
    "vmandnot.mm",
    "vmax.vv",
    "vmax.vx",
    "vmaxu.vv",
    "vmaxu.vx",
    "vmerge.vim",
    "vmerge.vvm",
    "vmerge.vxm",
    "vmfeq.vf",
    "vmfeq.vv",
    "vmfge.vf",
    "vmfgt.vf",
    "vmfle.vf",
    "vmfle.vv",
    "vmflt.vf",
    "vmflt.vv",
    "vmfne.vf",
    "vmfne.vv",
    "vmin.vv",
    "vmin.vx",
    "vminu.vv",
    "vminu.vx",
    "vmnand.mm",
    "vmnor.mm",
    "vmor.mm",
    "vmorn.mm",
    "vmornot.mm",
    "vmsbc.vv",
    "vmsbc.vvm",
    "vmsbc.vx",
    "vmsbc.vxm",
    "vmsbf.m",
    "vmseq.vi",
    "vmseq.vv",
    "vmseq.vx",
    "vmsgt.vi",
    "vmsgt.vx",
    "vmsgtu.vi",
    "vmsgtu.vx",
    "vmsif.m",
    "vmsle.vi",
    "vmsle.vv",
    "vmsle.vx",
    "vmsleu.vi",
    "vmsleu.vv",
    "vmsleu.vx",
    "vmslt.vv",
    "vmslt.vx",
    "vmsltu.vv",
    "vmsltu.vx",
    "vmsne.vi",
    "vmsne.vv",
    "vmsne.vx",
    "vmsof.m",
    "vmul.vv",
    "vmul.vx",
    "vmulh.vv",
    "vmulh.vx",
    "vmulhsu.vv",
    "vmulhsu.vx",
    "vmulhu.vv",
    "vmulhu.vx",
    "vmv1r.v",
    "vmv2r.v",
    "vmv4r.v",
    "vmv8r.v",
    "vmv.s.x",
    "vmv.v.i",
    "vmv.v.v",
    "vmv.v.x",
    "vmv.x.s",
    "vmxnor.mm",
    "vmxor.mm",
    "vnclip.wi",
    "vnclip.wv",
    "vnclip.wx",
    "vnclipu.wi",
    "vnclipu.wv",
    "vnclipu.wx",
    "vnmsac.vv",
    "vnmsac.vx",
    "vnmsub.vv",
    "vnmsub.vx",
    "vnsra.wi",
    "vnsra.wv",
    "vnsra.wx",
    "vnsrl.wi",
    "vnsrl.wv",
    "vnsrl.wx",
    "vor.vi",
    "vor.vv",
    "vor.vx",
    "vpopc.m",
    "vredand.vs",
    "vredmax.vs",
    "vredmaxu.vs",
    "vredmin.vs",
    "vredminu.vs",
    "vredor.vs",
    "vredsum.vs",
    "vredxor.vs",
    "vrem.vv",
    "vrem.vx",
    "vremu.vv",
    "vremu.vx",
    "vrev8.v",
    "vrgather.vi",
    "vrgather.vv",
    "vrgather.vx",
    "vrgatherei16.vv",
    "vrol.vv",
    "vrol.vx",
    "vror.vi",
    "vror.vv",
    "vror.vx",
    "vrsub.vi",
    "vrsub.vx",
    "vs1r.v",
    "vs2r.v",
    "vs4r.v",
    "vs8r.v",
    "vsadd.vi",
    "vsadd.vv",
    "vsadd.vx",
    "vsaddu.vi",
    "vsaddu.vv",
    "vsaddu.vx",
    "vsbc.vvm",
    "vsbc.vxm",
    "vse16.v",
    "vse1.v",
    "vse32.v",
    "vse64.v",
    "vse8.v",
    "vsetivli",
    "vsetvl",
    "vsetvli",
    "vsext.vf2",
    "vsext.vf4",
    "vsext.vf8",
    "vsha2ch.vv",
    "vsha2cl.vv",
    "vsha2ms.vv",
    "vslide1down.vx",
    "vslide1up.vx",
    "vslidedown.vi",
    "vslidedown.vx",
    "vslideup.vi",
    "vslideup.vx",
    "vsll.vi",
    "vsll.vv",
    "vsll.vx",
    "vsm3c.vi",
    "vsm3me.vv",
    "vsm4k.vi",
    "vsm4r.vs",
    "vsm4r.vv",
    "vsm.v",
    "vsmul.vv",
    "vsmul.vx",
    "vsoxei16.v",
    "vsoxei32.v",
    "vsoxei64.v",
    "vsoxei8.v",
    "vsra.vi",
    "vsra.vv",
    "vsra.vx",
    "vsrl.vi",
    "vsrl.vv",
    "vsrl.vx",
    "vsse16.v",
    "vsse32.v",
    "vsse64.v",
    "vsse8.v",
    "vssra.vi",
    "vssra.vv",
    "vssra.vx",
    "vssrl.vi",
    "vssrl.vv",
    "vssrl.vx",
    "vssub.vv",
    "vssub.vx",
    "vssubu.vv",
    "vssubu.vx",
    "vsub.vv",
    "vsub.vx",
    "vsuxei16.v",
    "vsuxei32.v",
    "vsuxei64.v",
    "vsuxei8.v",
    "vwadd.vv",
    "vwadd.vx",
    "vwadd.wv",
    "vwadd.wx",
    "vwaddu.vv",
    "vwaddu.vx",
    "vwaddu.wv",
    "vwaddu.wx",
    "vwmacc.vv",
    "vwmacc.vx",
    "vwmaccsu.vv",
    "vwmaccsu.vx",
    "vwmaccu.vv",
    "vwmaccu.vx",
    "vwmaccus.vx",
    "vwmul.vv",
    "vwmul.vx",
    "vwmulsu.vv",
    "vwmulsu.vx",
    "vwmulu.vv",
    "vwmulu.vx",
    "vwredsum.vs",
    "vwredsumu.vs",
    "vwsll.vi",
    "vwsll.vv",
    "vwsll.vx",
    "vwsub.vv",
    "vwsub.vx",
    "vwsub.wv",
    "vwsub.wx",
    "vwsubu.vv",
    "vwsubu.vx",
    "vwsubu.wv",
    "vwsubu.wx",
    "vxor.vi",
    "vxor.vv",
    "vxor.vx",
    "vzext.vf2",
    "vzext.vf4",
    "vzext.vf8",
    "wfi",
    "wrs.nto",
    "wrs.sto",
    "xnor",
    "xor",
    "xori",
    "xperm4",
    "xperm8",
    "zext.b",
    "zext.h",
    "zext.h.rv32",
    "zext.w",
    "zip",
    "<invalid>"
];*/

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Inst {
    pub opcode: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub csr: i64,
    pub funct7: u32,
}

impl Inst {
    pub const fn encode(&self) -> InstructionValue {
        InstructionValue::new(
            0 | (self.funct7 << 25)
                | (self.rs2 << 20)
                | (self.rs1 << 15)
                | (self.funct3 << 12)
                | self.opcode,
        )
    }

    pub const fn new(op: Opcode) -> Self {
        match op {
            Opcode::Invalid => unreachable!(),
            Opcode::ADD => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::ADDUW => Inst {
                opcode: 0x3b,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::ADDI => Inst {
                opcode: 0x13,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::ADDIW => Inst {
                opcode: 0x1b,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::ADDW => Inst {
                opcode: 0x3b,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::AES32DSI => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2a0,
                funct7: 0x15,
            },
            Opcode::AES32DSMI => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2e0,
                funct7: 0x17,
            },
            Opcode::AES32ESI => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x220,
                funct7: 0x11,
            },
            Opcode::AES32ESMI => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x260,
                funct7: 0x13,
            },
            Opcode::AES64DS => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x3a0,
                funct7: 0x1d,
            },
            Opcode::AES64DSM => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x3e0,
                funct7: 0x1f,
            },
            Opcode::AES64ES => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x320,
                funct7: 0x19,
            },
            Opcode::AES64ESM => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x360,
                funct7: 0x1b,
            },
            Opcode::AES64IM => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x300,
                funct7: 0x18,
            },
            Opcode::AES64KS1I => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x10,
                csr: 0x310,
                funct7: 0x18,
            },
            Opcode::AES64KS2 => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x7e0,
                funct7: 0x3f,
            },
            Opcode::AMOADDB => Inst {
                opcode: 0x2f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::AMOADDD => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::AMOADDH => Inst {
                opcode: 0x2f,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::AMOADDW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::AMOANDB => Inst {
                opcode: 0x2f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::AMOANDD => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::AMOANDH => Inst {
                opcode: 0x2f,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::AMOANDW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::AMOCASB => Inst {
                opcode: 0x2f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::AMOCASD => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::AMOCASH => Inst {
                opcode: 0x2f,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::AMOCASQ => Inst {
                opcode: 0x2f,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::AMOCASW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::AMOMAXB => Inst {
                opcode: 0x2f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::AMOMAXD => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::AMOMAXH => Inst {
                opcode: 0x2f,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::AMOMAXW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::AMOMAXUB => Inst {
                opcode: 0x2f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe00,
                funct7: 0x70,
            },
            Opcode::AMOMAXUD => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe00,
                funct7: 0x70,
            },
            Opcode::AMOMAXUH => Inst {
                opcode: 0x2f,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe00,
                funct7: 0x70,
            },
            Opcode::AMOMAXUW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe00,
                funct7: 0x70,
            },
            Opcode::AMOMINB => Inst {
                opcode: 0x2f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::AMOMIND => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::AMOMINH => Inst {
                opcode: 0x2f,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::AMOMINW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::AMOMINUB => Inst {
                opcode: 0x2f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::AMOMINUD => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::AMOMINUH => Inst {
                opcode: 0x2f,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::AMOMINUW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::AMOORB => Inst {
                opcode: 0x2f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::AMOORD => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::AMOORH => Inst {
                opcode: 0x2f,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::AMOORW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::AMOSWAPB => Inst {
                opcode: 0x2f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::AMOSWAPD => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::AMOSWAPH => Inst {
                opcode: 0x2f,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::AMOSWAPW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::AMOXORB => Inst {
                opcode: 0x2f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::AMOXORD => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::AMOXORH => Inst {
                opcode: 0x2f,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::AMOXORW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::AND => Inst {
                opcode: 0x33,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::ANDI => Inst {
                opcode: 0x13,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::ANDN => Inst {
                opcode: 0x33,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::AUIPC => Inst {
                opcode: 0x17,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BCLR => Inst {
                opcode: 0x33,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::BCLRI => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::BCLRIRV32 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::BEQ => Inst {
                opcode: 0x63,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BEQZ => Inst {
                opcode: 0x63,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BEXT => Inst {
                opcode: 0x33,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::BEXTI => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::BEXTIRV32 => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::BGE => Inst {
                opcode: 0x63,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BGEU => Inst {
                opcode: 0x63,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BGEZ => Inst {
                opcode: 0x63,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BGT => Inst {
                opcode: 0x63,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BGTU => Inst {
                opcode: 0x63,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BGTZ => Inst {
                opcode: 0x63,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BINV => Inst {
                opcode: 0x33,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x680,
                funct7: 0x34,
            },
            Opcode::BINVI => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x680,
                funct7: 0x34,
            },
            Opcode::BINVIRV32 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x680,
                funct7: 0x34,
            },
            Opcode::BLE => Inst {
                opcode: 0x63,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BLEU => Inst {
                opcode: 0x63,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BLEZ => Inst {
                opcode: 0x63,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BLT => Inst {
                opcode: 0x63,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BLTU => Inst {
                opcode: 0x63,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BLTZ => Inst {
                opcode: 0x63,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BNE => Inst {
                opcode: 0x63,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BNEZ => Inst {
                opcode: 0x63,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::BREV8 => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x7,
                csr: 0x687,
                funct7: 0x34,
            },
            Opcode::BSET => Inst {
                opcode: 0x33,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::BSETI => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::BSETIRV32 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::CADD => Inst {
                opcode: 0x2,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CADDI => Inst {
                opcode: 0x1,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CADDI16SP => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CADDI4SPN => Inst {
                opcode: 0x0,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CADDIW => Inst {
                opcode: 0x1,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CADDW => Inst {
                opcode: 0x21,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CAND => Inst {
                opcode: 0x61,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CANDI => Inst {
                opcode: 0x1,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CBEQZ => Inst {
                opcode: 0x1,
                funct3: 0x4,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CBNEZ => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CEBREAK => Inst {
                opcode: 0x2,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CFLD => Inst {
                opcode: 0x0,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CFLDSP => Inst {
                opcode: 0x2,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CFLW => Inst {
                opcode: 0x0,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CFLWSP => Inst {
                opcode: 0x2,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CFSD => Inst {
                opcode: 0x0,
                funct3: 0x2,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CFSDSP => Inst {
                opcode: 0x2,
                funct3: 0x2,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CFSW => Inst {
                opcode: 0x0,
                funct3: 0x6,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CFSWSP => Inst {
                opcode: 0x2,
                funct3: 0x6,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CJ => Inst {
                opcode: 0x1,
                funct3: 0x2,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CJAL => Inst {
                opcode: 0x1,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CJALR => Inst {
                opcode: 0x2,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CJR => Inst {
                opcode: 0x2,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CLBU => Inst {
                opcode: 0x0,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CLD => Inst {
                opcode: 0x0,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CLDSP => Inst {
                opcode: 0x2,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CLH => Inst {
                opcode: 0x40,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CLHU => Inst {
                opcode: 0x0,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CLI => Inst {
                opcode: 0x1,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CLUI => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CLW => Inst {
                opcode: 0x0,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CLWSP => Inst {
                opcode: 0x2,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMOP1 => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMOP11 => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMOP13 => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMOP15 => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMOP3 => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMOP5 => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMOP7 => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMOP9 => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMOPN => Inst {
                opcode: 0x1,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMUL => Inst {
                opcode: 0x41,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMV => Inst {
                opcode: 0x2,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CNOP => Inst {
                opcode: 0x1,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CNOT => Inst {
                opcode: 0x75,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CNTLALL => Inst {
                opcode: 0x16,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CNTLP1 => Inst {
                opcode: 0xa,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CNTLPALL => Inst {
                opcode: 0xe,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CNTLS1 => Inst {
                opcode: 0x12,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::COR => Inst {
                opcode: 0x41,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSB => Inst {
                opcode: 0x0,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSD => Inst {
                opcode: 0x0,
                funct3: 0x6,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSDSP => Inst {
                opcode: 0x2,
                funct3: 0x6,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSEXTB => Inst {
                opcode: 0x65,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSEXTH => Inst {
                opcode: 0x6d,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSEXTW => Inst {
                opcode: 0x1,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSH => Inst {
                opcode: 0x0,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSLLI => Inst {
                opcode: 0x2,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSLLIRV32 => Inst {
                opcode: 0x2,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRAI => Inst {
                opcode: 0x1,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRAIRV32 => Inst {
                opcode: 0x1,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRLI => Inst {
                opcode: 0x1,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRLIRV32 => Inst {
                opcode: 0x1,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSUB => Inst {
                opcode: 0x1,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSUBW => Inst {
                opcode: 0x1,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSW => Inst {
                opcode: 0x0,
                funct3: 0x4,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSWSP => Inst {
                opcode: 0x2,
                funct3: 0x4,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CXOR => Inst {
                opcode: 0x21,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CZEXTB => Inst {
                opcode: 0x61,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CZEXTH => Inst {
                opcode: 0x69,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CZEXTW => Inst {
                opcode: 0x71,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CBOCLEAN => Inst {
                opcode: 0xf,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x1,
                funct7: 0x0,
            },
            Opcode::CBOFLUSH => Inst {
                opcode: 0xf,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x2,
                funct7: 0x0,
            },
            Opcode::CBOINVAL => Inst {
                opcode: 0xf,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CBOZERO => Inst {
                opcode: 0xf,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x4,
                csr: 0x4,
                funct7: 0x0,
            },
            Opcode::CLMUL => Inst {
                opcode: 0x33,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa0,
                funct7: 0x5,
            },
            Opcode::CLMULH => Inst {
                opcode: 0x33,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa0,
                funct7: 0x5,
            },
            Opcode::CLMULR => Inst {
                opcode: 0x33,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa0,
                funct7: 0x5,
            },
            Opcode::CLZ => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::CLZW => Inst {
                opcode: 0x1b,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::CMJALT => Inst {
                opcode: 0x2,
                funct3: 0x2,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMMVA01S => Inst {
                opcode: 0x62,
                funct3: 0x2,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMMVSA01 => Inst {
                opcode: 0x22,
                funct3: 0x2,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMPOP => Inst {
                opcode: 0x2,
                funct3: 0x3,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMPOPRET => Inst {
                opcode: 0x2,
                funct3: 0x3,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMPOPRETZ => Inst {
                opcode: 0x2,
                funct3: 0x3,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CMPUSH => Inst {
                opcode: 0x2,
                funct3: 0x3,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CPOP => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x602,
                funct7: 0x30,
            },
            Opcode::CPOPW => Inst {
                opcode: 0x1b,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x602,
                funct7: 0x30,
            },
            Opcode::CSRC => Inst {
                opcode: 0x73,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRCI => Inst {
                opcode: 0x73,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRR => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRRC => Inst {
                opcode: 0x73,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRRCI => Inst {
                opcode: 0x73,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRRS => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRRSI => Inst {
                opcode: 0x73,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRRW => Inst {
                opcode: 0x73,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRRWI => Inst {
                opcode: 0x73,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRS => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRSI => Inst {
                opcode: 0x73,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRW => Inst {
                opcode: 0x73,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CSRWI => Inst {
                opcode: 0x73,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::CTZ => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x601,
                funct7: 0x30,
            },
            Opcode::CTZW => Inst {
                opcode: 0x1b,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x601,
                funct7: 0x30,
            },
            Opcode::CZEROEQZ => Inst {
                opcode: 0x33,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe0,
                funct7: 0x7,
            },
            Opcode::CZERONEZ => Inst {
                opcode: 0x33,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe0,
                funct7: 0x7,
            },
            Opcode::DIV => Inst {
                opcode: 0x33,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::DIVU => Inst {
                opcode: 0x33,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::DIVUW => Inst {
                opcode: 0x3b,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::DIVW => Inst {
                opcode: 0x3b,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::DRET => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x12,
                csr: 0x7b2,
                funct7: 0x3d,
            },
            Opcode::EBREAK => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x1,
                funct7: 0x0,
            },
            Opcode::ECALL => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FABSD => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x220,
                funct7: 0x11,
            },
            Opcode::FABSH => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::FABSQ => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x260,
                funct7: 0x13,
            },
            Opcode::FABSS => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::FADDD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::FADDH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::FADDQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x60,
                funct7: 0x3,
            },
            Opcode::FADDS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FCLASSD => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe20,
                funct7: 0x71,
            },
            Opcode::FCLASSH => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe40,
                funct7: 0x72,
            },
            Opcode::FCLASSQ => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe60,
                funct7: 0x73,
            },
            Opcode::FCLASSS => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe00,
                funct7: 0x70,
            },
            Opcode::FCVTDH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x422,
                funct7: 0x21,
            },
            Opcode::FCVTDL => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0xd22,
                funct7: 0x69,
            },
            Opcode::FCVTDLU => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0xd23,
                funct7: 0x69,
            },
            Opcode::FCVTDQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0x423,
                funct7: 0x21,
            },
            Opcode::FCVTDS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x420,
                funct7: 0x21,
            },
            Opcode::FCVTDW => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd20,
                funct7: 0x69,
            },
            Opcode::FCVTDWU => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xd21,
                funct7: 0x69,
            },
            Opcode::FCVTHD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x441,
                funct7: 0x22,
            },
            Opcode::FCVTHL => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0xd42,
                funct7: 0x6a,
            },
            Opcode::FCVTHLU => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0xd43,
                funct7: 0x6a,
            },
            Opcode::FCVTHQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0x443,
                funct7: 0x22,
            },
            Opcode::FCVTHS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x440,
                funct7: 0x22,
            },
            Opcode::FCVTHW => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd40,
                funct7: 0x6a,
            },
            Opcode::FCVTHWU => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xd41,
                funct7: 0x6a,
            },
            Opcode::FCVTLD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0xc22,
                funct7: 0x61,
            },
            Opcode::FCVTLH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0xc42,
                funct7: 0x62,
            },
            Opcode::FCVTLQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0xc62,
                funct7: 0x63,
            },
            Opcode::FCVTLS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0xc02,
                funct7: 0x60,
            },
            Opcode::FCVTLUD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0xc23,
                funct7: 0x61,
            },
            Opcode::FCVTLUH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0xc43,
                funct7: 0x62,
            },
            Opcode::FCVTLUQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0xc63,
                funct7: 0x63,
            },
            Opcode::FCVTLUS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0xc03,
                funct7: 0x60,
            },
            Opcode::FCVTQD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x461,
                funct7: 0x23,
            },
            Opcode::FCVTQH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x462,
                funct7: 0x23,
            },
            Opcode::FCVTQL => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0xd62,
                funct7: 0x6b,
            },
            Opcode::FCVTQLU => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0xd63,
                funct7: 0x6b,
            },
            Opcode::FCVTQS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x460,
                funct7: 0x23,
            },
            Opcode::FCVTQW => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd60,
                funct7: 0x6b,
            },
            Opcode::FCVTQWU => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xd61,
                funct7: 0x6b,
            },
            Opcode::FCVTSD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x401,
                funct7: 0x20,
            },
            Opcode::FCVTSH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x402,
                funct7: 0x20,
            },
            Opcode::FCVTSL => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0xd02,
                funct7: 0x68,
            },
            Opcode::FCVTSLU => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0xd03,
                funct7: 0x68,
            },
            Opcode::FCVTSQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0x403,
                funct7: 0x20,
            },
            Opcode::FCVTSW => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd00,
                funct7: 0x68,
            },
            Opcode::FCVTSWU => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xd01,
                funct7: 0x68,
            },
            Opcode::FCVTWD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc20,
                funct7: 0x61,
            },
            Opcode::FCVTWH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc40,
                funct7: 0x62,
            },
            Opcode::FCVTWQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc60,
                funct7: 0x63,
            },
            Opcode::FCVTWS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::FCVTWUD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xc21,
                funct7: 0x61,
            },
            Opcode::FCVTWUH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xc41,
                funct7: 0x62,
            },
            Opcode::FCVTWUQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xc61,
                funct7: 0x63,
            },
            Opcode::FCVTWUS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xc01,
                funct7: 0x60,
            },
            Opcode::FCVTMODWD => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0xc28,
                funct7: 0x61,
            },
            Opcode::FDIVD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x1a0,
                funct7: 0xd,
            },
            Opcode::FDIVH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x1c0,
                funct7: 0xe,
            },
            Opcode::FDIVQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x1e0,
                funct7: 0xf,
            },
            Opcode::FDIVS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x180,
                funct7: 0xc,
            },
            Opcode::FENCE => Inst {
                opcode: 0xf,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FENCEI => Inst {
                opcode: 0xf,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FENCETSO => Inst {
                opcode: 0xf,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x13,
                csr: 0x833,
                funct7: 0x41,
            },
            Opcode::FEQD => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa20,
                funct7: 0x51,
            },
            Opcode::FEQH => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::FEQQ => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa60,
                funct7: 0x53,
            },
            Opcode::FEQS => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::FLD => Inst {
                opcode: 0x7,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FLED => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa20,
                funct7: 0x51,
            },
            Opcode::FLEH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::FLEQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa60,
                funct7: 0x53,
            },
            Opcode::FLES => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::FLEQD => Inst {
                opcode: 0x53,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa20,
                funct7: 0x51,
            },
            Opcode::FLEQH => Inst {
                opcode: 0x53,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::FLEQQ => Inst {
                opcode: 0x53,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa60,
                funct7: 0x53,
            },
            Opcode::FLEQS => Inst {
                opcode: 0x53,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::FLH => Inst {
                opcode: 0x7,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FLID => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xf21,
                funct7: 0x79,
            },
            Opcode::FLIH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xf41,
                funct7: 0x7a,
            },
            Opcode::FLIQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xf61,
                funct7: 0x7b,
            },
            Opcode::FLIS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xf01,
                funct7: 0x78,
            },
            Opcode::FLQ => Inst {
                opcode: 0x7,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FLTD => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa20,
                funct7: 0x51,
            },
            Opcode::FLTH => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::FLTQ => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa60,
                funct7: 0x53,
            },
            Opcode::FLTS => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::FLTQD => Inst {
                opcode: 0x53,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa20,
                funct7: 0x51,
            },
            Opcode::FLTQH => Inst {
                opcode: 0x53,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::FLTQQ => Inst {
                opcode: 0x53,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa60,
                funct7: 0x53,
            },
            Opcode::FLTQS => Inst {
                opcode: 0x53,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::FLW => Inst {
                opcode: 0x7,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FMADDD => Inst {
                opcode: 0x43,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::FMADDH => Inst {
                opcode: 0x43,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::FMADDQ => Inst {
                opcode: 0x43,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x60,
                funct7: 0x3,
            },
            Opcode::FMADDS => Inst {
                opcode: 0x43,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FMAXD => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2a0,
                funct7: 0x15,
            },
            Opcode::FMAXH => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2c0,
                funct7: 0x16,
            },
            Opcode::FMAXQ => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2e0,
                funct7: 0x17,
            },
            Opcode::FMAXS => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::FMAXMD => Inst {
                opcode: 0x53,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2a0,
                funct7: 0x15,
            },
            Opcode::FMAXMH => Inst {
                opcode: 0x53,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2c0,
                funct7: 0x16,
            },
            Opcode::FMAXMQ => Inst {
                opcode: 0x53,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2e0,
                funct7: 0x17,
            },
            Opcode::FMAXMS => Inst {
                opcode: 0x53,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::FMIND => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2a0,
                funct7: 0x15,
            },
            Opcode::FMINH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2c0,
                funct7: 0x16,
            },
            Opcode::FMINQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2e0,
                funct7: 0x17,
            },
            Opcode::FMINS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::FMINMD => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2a0,
                funct7: 0x15,
            },
            Opcode::FMINMH => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2c0,
                funct7: 0x16,
            },
            Opcode::FMINMQ => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2e0,
                funct7: 0x17,
            },
            Opcode::FMINMS => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::FMSUBD => Inst {
                opcode: 0x47,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::FMSUBH => Inst {
                opcode: 0x47,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::FMSUBQ => Inst {
                opcode: 0x47,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x60,
                funct7: 0x3,
            },
            Opcode::FMSUBS => Inst {
                opcode: 0x47,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FMULD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x120,
                funct7: 0x9,
            },
            Opcode::FMULH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x140,
                funct7: 0xa,
            },
            Opcode::FMULQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x160,
                funct7: 0xb,
            },
            Opcode::FMULS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x100,
                funct7: 0x8,
            },
            Opcode::FMVD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x220,
                funct7: 0x11,
            },
            Opcode::FMVDX => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf20,
                funct7: 0x79,
            },
            Opcode::FMVH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::FMVHX => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf40,
                funct7: 0x7a,
            },
            Opcode::FMVQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x260,
                funct7: 0x13,
            },
            Opcode::FMVS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::FMVSX => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf00,
                funct7: 0x78,
            },
            Opcode::FMVWX => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf00,
                funct7: 0x78,
            },
            Opcode::FMVXD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe20,
                funct7: 0x71,
            },
            Opcode::FMVXH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe40,
                funct7: 0x72,
            },
            Opcode::FMVXS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe00,
                funct7: 0x70,
            },
            Opcode::FMVXW => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe00,
                funct7: 0x70,
            },
            Opcode::FMVHXD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xe21,
                funct7: 0x71,
            },
            Opcode::FMVHXQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xe61,
                funct7: 0x73,
            },
            Opcode::FMVPDX => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb20,
                funct7: 0x59,
            },
            Opcode::FMVPQX => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb60,
                funct7: 0x5b,
            },
            Opcode::FNEGD => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x220,
                funct7: 0x11,
            },
            Opcode::FNEGH => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::FNEGQ => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x260,
                funct7: 0x13,
            },
            Opcode::FNEGS => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::FNMADDD => Inst {
                opcode: 0x4f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::FNMADDH => Inst {
                opcode: 0x4f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::FNMADDQ => Inst {
                opcode: 0x4f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x60,
                funct7: 0x3,
            },
            Opcode::FNMADDS => Inst {
                opcode: 0x4f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FNMSUBD => Inst {
                opcode: 0x4b,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::FNMSUBH => Inst {
                opcode: 0x4b,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::FNMSUBQ => Inst {
                opcode: 0x4b,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x60,
                funct7: 0x3,
            },
            Opcode::FNMSUBS => Inst {
                opcode: 0x4b,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FRCSR => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0x3,
                funct7: 0x0,
            },
            Opcode::FRFLAGS => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x1,
                funct7: 0x0,
            },
            Opcode::FROUNDD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x4,
                csr: 0x424,
                funct7: 0x21,
            },
            Opcode::FROUNDH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x4,
                csr: 0x444,
                funct7: 0x22,
            },
            Opcode::FROUNDQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x4,
                csr: 0x464,
                funct7: 0x23,
            },
            Opcode::FROUNDS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x4,
                csr: 0x404,
                funct7: 0x20,
            },
            Opcode::FROUNDNXD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x5,
                csr: 0x425,
                funct7: 0x21,
            },
            Opcode::FROUNDNXH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x5,
                csr: 0x445,
                funct7: 0x22,
            },
            Opcode::FROUNDNXQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x5,
                csr: 0x465,
                funct7: 0x23,
            },
            Opcode::FROUNDNXS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x5,
                csr: 0x405,
                funct7: 0x20,
            },
            Opcode::FRRM => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x2,
                funct7: 0x0,
            },
            Opcode::FSCSR => Inst {
                opcode: 0x73,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0x3,
                funct7: 0x0,
            },
            Opcode::FSD => Inst {
                opcode: 0x27,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FSFLAGS => Inst {
                opcode: 0x73,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x1,
                funct7: 0x0,
            },
            Opcode::FSFLAGSI => Inst {
                opcode: 0x73,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x1,
                funct7: 0x0,
            },
            Opcode::FSGNJD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x220,
                funct7: 0x11,
            },
            Opcode::FSGNJH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::FSGNJQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x260,
                funct7: 0x13,
            },
            Opcode::FSGNJS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::FSGNJND => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x220,
                funct7: 0x11,
            },
            Opcode::FSGNJNH => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::FSGNJNQ => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x260,
                funct7: 0x13,
            },
            Opcode::FSGNJNS => Inst {
                opcode: 0x53,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::FSGNJXD => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x220,
                funct7: 0x11,
            },
            Opcode::FSGNJXH => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::FSGNJXQ => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x260,
                funct7: 0x13,
            },
            Opcode::FSGNJXS => Inst {
                opcode: 0x53,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::FSH => Inst {
                opcode: 0x27,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FSQ => Inst {
                opcode: 0x27,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::FSQRTD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5a0,
                funct7: 0x2d,
            },
            Opcode::FSQRTH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5c0,
                funct7: 0x2e,
            },
            Opcode::FSQRTQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5e0,
                funct7: 0x2f,
            },
            Opcode::FSQRTS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x580,
                funct7: 0x2c,
            },
            Opcode::FSRM => Inst {
                opcode: 0x73,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x2,
                funct7: 0x0,
            },
            Opcode::FSRMI => Inst {
                opcode: 0x73,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x2,
                funct7: 0x0,
            },
            Opcode::FSUBD => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa0,
                funct7: 0x5,
            },
            Opcode::FSUBH => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::FSUBQ => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe0,
                funct7: 0x7,
            },
            Opcode::FSUBS => Inst {
                opcode: 0x53,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::FSW => Inst {
                opcode: 0x27,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::HFENCEGVMA => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x620,
                funct7: 0x31,
            },
            Opcode::HFENCEVVMA => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x220,
                funct7: 0x11,
            },
            Opcode::HINVALGVMA => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x660,
                funct7: 0x33,
            },
            Opcode::HINVALVVMA => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x260,
                funct7: 0x13,
            },
            Opcode::HLVB => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::HLVBU => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x601,
                funct7: 0x30,
            },
            Opcode::HLVD => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x6c0,
                funct7: 0x36,
            },
            Opcode::HLVH => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x640,
                funct7: 0x32,
            },
            Opcode::HLVHU => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x641,
                funct7: 0x32,
            },
            Opcode::HLVW => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x680,
                funct7: 0x34,
            },
            Opcode::HLVWU => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x681,
                funct7: 0x34,
            },
            Opcode::HLVXHU => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0x643,
                funct7: 0x32,
            },
            Opcode::HLVXWU => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0x683,
                funct7: 0x34,
            },
            Opcode::HSVB => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x620,
                funct7: 0x31,
            },
            Opcode::HSVD => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x6e0,
                funct7: 0x37,
            },
            Opcode::HSVH => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x660,
                funct7: 0x33,
            },
            Opcode::HSVW => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x6a0,
                funct7: 0x35,
            },
            Opcode::J => Inst {
                opcode: 0x6f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::JAL => Inst {
                opcode: 0x6f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::JALPSEUDO => Inst {
                opcode: 0x6f,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::JALR => Inst {
                opcode: 0x67,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::JALRPSEUDO => Inst {
                opcode: 0x67,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::JR => Inst {
                opcode: 0x67,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::LB => Inst {
                opcode: 0x3,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::LBU => Inst {
                opcode: 0x3,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::LD => Inst {
                opcode: 0x3,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::LH => Inst {
                opcode: 0x3,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::LHU => Inst {
                opcode: 0x3,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::LRD => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x100,
                funct7: 0x8,
            },
            Opcode::LRW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x100,
                funct7: 0x8,
            },
            Opcode::LUI => Inst {
                opcode: 0x37,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::LW => Inst {
                opcode: 0x3,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::LWU => Inst {
                opcode: 0x3,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::MAX => Inst {
                opcode: 0x33,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa0,
                funct7: 0x5,
            },
            Opcode::MAXU => Inst {
                opcode: 0x33,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa0,
                funct7: 0x5,
            },
            Opcode::MIN => Inst {
                opcode: 0x33,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa0,
                funct7: 0x5,
            },
            Opcode::MINU => Inst {
                opcode: 0x33,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa0,
                funct7: 0x5,
            },
            Opcode::MOPR0 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1c,
                csr: 0x81c,
                funct7: 0x40,
            },
            Opcode::MOPR1 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1d,
                csr: 0x81d,
                funct7: 0x40,
            },
            Opcode::MOPR10 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1e,
                csr: 0x89e,
                funct7: 0x44,
            },
            Opcode::MOPR11 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1f,
                csr: 0x89f,
                funct7: 0x44,
            },
            Opcode::MOPR12 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1c,
                csr: 0x8dc,
                funct7: 0x46,
            },
            Opcode::MOPR13 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1d,
                csr: 0x8dd,
                funct7: 0x46,
            },
            Opcode::MOPR14 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1e,
                csr: 0x8de,
                funct7: 0x46,
            },
            Opcode::MOPR15 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1f,
                csr: 0x8df,
                funct7: 0x46,
            },
            Opcode::MOPR16 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1c,
                csr: 0xc1c,
                funct7: 0x60,
            },
            Opcode::MOPR17 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1d,
                csr: 0xc1d,
                funct7: 0x60,
            },
            Opcode::MOPR18 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1e,
                csr: 0xc1e,
                funct7: 0x60,
            },
            Opcode::MOPR19 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1f,
                csr: 0xc1f,
                funct7: 0x60,
            },
            Opcode::MOPR2 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1e,
                csr: 0x81e,
                funct7: 0x40,
            },
            Opcode::MOPR20 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1c,
                csr: 0xc5c,
                funct7: 0x62,
            },
            Opcode::MOPR21 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1d,
                csr: 0xc5d,
                funct7: 0x62,
            },
            Opcode::MOPR22 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1e,
                csr: 0xc5e,
                funct7: 0x62,
            },
            Opcode::MOPR23 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1f,
                csr: 0xc5f,
                funct7: 0x62,
            },
            Opcode::MOPR24 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1c,
                csr: 0xc9c,
                funct7: 0x64,
            },
            Opcode::MOPR25 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1d,
                csr: 0xc9d,
                funct7: 0x64,
            },
            Opcode::MOPR26 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1e,
                csr: 0xc9e,
                funct7: 0x64,
            },
            Opcode::MOPR27 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1f,
                csr: 0xc9f,
                funct7: 0x64,
            },
            Opcode::MOPR28 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1c,
                csr: 0xcdc,
                funct7: 0x66,
            },
            Opcode::MOPR29 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1d,
                csr: 0xcdd,
                funct7: 0x66,
            },
            Opcode::MOPR3 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1f,
                csr: 0x81f,
                funct7: 0x40,
            },
            Opcode::MOPR30 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1e,
                csr: 0xcde,
                funct7: 0x66,
            },
            Opcode::MOPR31 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1f,
                csr: 0xcdf,
                funct7: 0x66,
            },
            Opcode::MOPR4 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1c,
                csr: 0x85c,
                funct7: 0x42,
            },
            Opcode::MOPR5 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1d,
                csr: 0x85d,
                funct7: 0x42,
            },
            Opcode::MOPR6 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1e,
                csr: 0x85e,
                funct7: 0x42,
            },
            Opcode::MOPR7 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1f,
                csr: 0x85f,
                funct7: 0x42,
            },
            Opcode::MOPR8 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1c,
                csr: 0x89c,
                funct7: 0x44,
            },
            Opcode::MOPR9 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1d,
                csr: 0x89d,
                funct7: 0x44,
            },
            Opcode::MOPRN => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x1c,
                csr: 0x81c,
                funct7: 0x40,
            },
            Opcode::MOPRR0 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x820,
                funct7: 0x41,
            },
            Opcode::MOPRR1 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x860,
                funct7: 0x43,
            },
            Opcode::MOPRR2 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x8a0,
                funct7: 0x45,
            },
            Opcode::MOPRR3 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x8e0,
                funct7: 0x47,
            },
            Opcode::MOPRR4 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc20,
                funct7: 0x61,
            },
            Opcode::MOPRR5 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc60,
                funct7: 0x63,
            },
            Opcode::MOPRR6 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xca0,
                funct7: 0x65,
            },
            Opcode::MOPRR7 => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xce0,
                funct7: 0x67,
            },
            Opcode::MOPRRN => Inst {
                opcode: 0x73,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x820,
                funct7: 0x41,
            },
            Opcode::MRET => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x302,
                funct7: 0x18,
            },
            Opcode::MUL => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::MULH => Inst {
                opcode: 0x33,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::MULHSU => Inst {
                opcode: 0x33,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::MULHU => Inst {
                opcode: 0x33,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::MULW => Inst {
                opcode: 0x3b,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::MV => Inst {
                opcode: 0x13,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::NEG => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::NOP => Inst {
                opcode: 0x13,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::NTLALL => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x5,
                csr: 0x5,
                funct7: 0x0,
            },
            Opcode::NTLP1 => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x2,
                funct7: 0x0,
            },
            Opcode::NTLPALL => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0x3,
                funct7: 0x0,
            },
            Opcode::NTLS1 => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x4,
                csr: 0x4,
                funct7: 0x0,
            },
            Opcode::OR => Inst {
                opcode: 0x33,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::ORCB => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x7,
                csr: 0x287,
                funct7: 0x14,
            },
            Opcode::ORI => Inst {
                opcode: 0x13,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::ORN => Inst {
                opcode: 0x33,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::PACK => Inst {
                opcode: 0x33,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::PACKH => Inst {
                opcode: 0x33,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::PACKW => Inst {
                opcode: 0x3b,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::PAUSE => Inst {
                opcode: 0xf,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x10,
                csr: 0x10,
                funct7: 0x0,
            },
            Opcode::PREFETCHI => Inst {
                opcode: 0x13,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::PREFETCHR => Inst {
                opcode: 0x13,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x1,
                funct7: 0x0,
            },
            Opcode::PREFETCHW => Inst {
                opcode: 0x13,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0x3,
                funct7: 0x0,
            },
            Opcode::RDCYCLE => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::RDCYCLEH => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc80,
                funct7: 0x64,
            },
            Opcode::RDINSTRET => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0xc02,
                funct7: 0x60,
            },
            Opcode::RDINSTRETH => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0xc82,
                funct7: 0x64,
            },
            Opcode::RDTIME => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xc01,
                funct7: 0x60,
            },
            Opcode::RDTIMEH => Inst {
                opcode: 0x73,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0xc81,
                funct7: 0x64,
            },
            Opcode::REM => Inst {
                opcode: 0x33,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::REMU => Inst {
                opcode: 0x33,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::REMUW => Inst {
                opcode: 0x3b,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::REMW => Inst {
                opcode: 0x3b,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x20,
                funct7: 0x1,
            },
            Opcode::RET => Inst {
                opcode: 0x67,
                funct3: 0x0,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::REV8 => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x18,
                csr: 0x6b8,
                funct7: 0x35,
            },
            Opcode::REV8RV32 => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x18,
                csr: 0x698,
                funct7: 0x34,
            },
            Opcode::ROL => Inst {
                opcode: 0x33,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::ROLW => Inst {
                opcode: 0x3b,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::ROR => Inst {
                opcode: 0x33,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::RORI => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::RORIRV32 => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::RORIW => Inst {
                opcode: 0x1b,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::RORW => Inst {
                opcode: 0x3b,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::SB => Inst {
                opcode: 0x23,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SBREAK => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x1,
                funct7: 0x0,
            },
            Opcode::SCD => Inst {
                opcode: 0x2f,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x180,
                funct7: 0xc,
            },
            Opcode::SCW => Inst {
                opcode: 0x2f,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x180,
                funct7: 0xc,
            },
            Opcode::SCALL => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SD => Inst {
                opcode: 0x23,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SEQZ => Inst {
                opcode: 0x13,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x1,
                funct7: 0x0,
            },
            Opcode::SEXTB => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x4,
                csr: 0x604,
                funct7: 0x30,
            },
            Opcode::SEXTH => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x5,
                csr: 0x605,
                funct7: 0x30,
            },
            Opcode::SEXTW => Inst {
                opcode: 0x1b,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SFENCEINVALIR => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x181,
                funct7: 0xc,
            },
            Opcode::SFENCEVMA => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x120,
                funct7: 0x9,
            },
            Opcode::SFENCEWINVAL => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x180,
                funct7: 0xc,
            },
            Opcode::SGTZ => Inst {
                opcode: 0x33,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SH => Inst {
                opcode: 0x23,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SH1ADD => Inst {
                opcode: 0x33,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::SH1ADDUW => Inst {
                opcode: 0x3b,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::SH2ADD => Inst {
                opcode: 0x33,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::SH2ADDUW => Inst {
                opcode: 0x3b,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::SH3ADD => Inst {
                opcode: 0x33,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::SH3ADDUW => Inst {
                opcode: 0x3b,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::SHA256SIG0 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x102,
                funct7: 0x8,
            },
            Opcode::SHA256SIG1 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x3,
                csr: 0x103,
                funct7: 0x8,
            },
            Opcode::SHA256SUM0 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x100,
                funct7: 0x8,
            },
            Opcode::SHA256SUM1 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x1,
                csr: 0x101,
                funct7: 0x8,
            },
            Opcode::SHA512SIG0 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x6,
                csr: 0x106,
                funct7: 0x8,
            },
            Opcode::SHA512SIG0H => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5c0,
                funct7: 0x2e,
            },
            Opcode::SHA512SIG0L => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x540,
                funct7: 0x2a,
            },
            Opcode::SHA512SIG1 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x7,
                csr: 0x107,
                funct7: 0x8,
            },
            Opcode::SHA512SIG1H => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5e0,
                funct7: 0x2f,
            },
            Opcode::SHA512SIG1L => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x560,
                funct7: 0x2b,
            },
            Opcode::SHA512SUM0 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x4,
                csr: 0x104,
                funct7: 0x8,
            },
            Opcode::SHA512SUM0R => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x500,
                funct7: 0x28,
            },
            Opcode::SHA512SUM1 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x5,
                csr: 0x105,
                funct7: 0x8,
            },
            Opcode::SHA512SUM1R => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x520,
                funct7: 0x29,
            },
            Opcode::SINVALVMA => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x160,
                funct7: 0xb,
            },
            Opcode::SLL => Inst {
                opcode: 0x33,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SLLI => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SLLIRV32 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SLLIUW => Inst {
                opcode: 0x1b,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::SLLIW => Inst {
                opcode: 0x1b,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SLLW => Inst {
                opcode: 0x3b,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SLT => Inst {
                opcode: 0x33,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SLTI => Inst {
                opcode: 0x13,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SLTIU => Inst {
                opcode: 0x13,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SLTU => Inst {
                opcode: 0x33,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SLTZ => Inst {
                opcode: 0x33,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SM3P0 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x108,
                funct7: 0x8,
            },
            Opcode::SM3P1 => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x9,
                csr: 0x109,
                funct7: 0x8,
            },
            Opcode::SM4ED => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x300,
                funct7: 0x18,
            },
            Opcode::SM4KS => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x340,
                funct7: 0x1a,
            },
            Opcode::SNEZ => Inst {
                opcode: 0x33,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SRA => Inst {
                opcode: 0x33,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::SRAI => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::SRAIRV32 => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::SRAIW => Inst {
                opcode: 0x1b,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::SRAW => Inst {
                opcode: 0x3b,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::SRET => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x2,
                csr: 0x102,
                funct7: 0x8,
            },
            Opcode::SRL => Inst {
                opcode: 0x33,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SRLI => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SRLIRV32 => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SRLIW => Inst {
                opcode: 0x1b,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SRLW => Inst {
                opcode: 0x3b,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::SUB => Inst {
                opcode: 0x33,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::SUBW => Inst {
                opcode: 0x3b,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::SW => Inst {
                opcode: 0x23,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::UNZIP => Inst {
                opcode: 0x13,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0xf,
                csr: 0x8f,
                funct7: 0x4,
            },
            Opcode::VAADDVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::VAADDVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::VAADDUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::VAADDUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::VADCVIM => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::VADCVVM => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::VADCVXM => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::VADDVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VADDVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VADDVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VAESDFVS => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0xa60,
                funct7: 0x53,
            },
            Opcode::VAESDFVV => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0xa20,
                funct7: 0x51,
            },
            Opcode::VAESDMVS => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa60,
                funct7: 0x53,
            },
            Opcode::VAESDMVV => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa20,
                funct7: 0x51,
            },
            Opcode::VAESEFVS => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x3,
                rs2: 0x0,
                csr: 0xa60,
                funct7: 0x53,
            },
            Opcode::VAESEFVV => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x3,
                rs2: 0x0,
                csr: 0xa20,
                funct7: 0x51,
            },
            Opcode::VAESEMVS => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x2,
                rs2: 0x0,
                csr: 0xa60,
                funct7: 0x53,
            },
            Opcode::VAESEMVV => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x2,
                rs2: 0x0,
                csr: 0xa20,
                funct7: 0x51,
            },
            Opcode::VAESKF1VI => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x8a0,
                funct7: 0x45,
            },
            Opcode::VAESKF2VI => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xaa0,
                funct7: 0x55,
            },
            Opcode::VAESZVS => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x7,
                rs2: 0x0,
                csr: 0xa60,
                funct7: 0x53,
            },
            Opcode::VANDVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::VANDVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::VANDVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::VANDNVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VANDNVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VASUBVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2c0,
                funct7: 0x16,
            },
            Opcode::VASUBVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2c0,
                funct7: 0x16,
            },
            Opcode::VASUBUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::VASUBUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::VBREV8V => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x8,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VBREVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0xa,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VCLMULVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x300,
                funct7: 0x18,
            },
            Opcode::VCLMULVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x300,
                funct7: 0x18,
            },
            Opcode::VCLMULHVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x340,
                funct7: 0x1a,
            },
            Opcode::VCLMULHVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x340,
                funct7: 0x1a,
            },
            Opcode::VCLZV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0xc,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VCOMPRESSVM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5e0,
                funct7: 0x2f,
            },
            Opcode::VCPOPM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x10,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::VCPOPV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0xe,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VCTZV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0xd,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VDIVVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x840,
                funct7: 0x42,
            },
            Opcode::VDIVVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x840,
                funct7: 0x42,
            },
            Opcode::VDIVUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::VDIVUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::VFADDVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VFADDVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VFCLASSV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x10,
                rs2: 0x0,
                csr: 0x4c0,
                funct7: 0x26,
            },
            Opcode::VFCVTFXV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x3,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFCVTFXUV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x2,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFCVTRTZXFV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x7,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFCVTRTZXUFV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x6,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFCVTXFV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFCVTXUFV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFDIVVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::VFDIVVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::VFIRSTM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x11,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::VFMACCVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb00,
                funct7: 0x58,
            },
            Opcode::VFMACCVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb00,
                funct7: 0x58,
            },
            Opcode::VFMADDVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::VFMADDVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::VFMAXVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x180,
                funct7: 0xc,
            },
            Opcode::VFMAXVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x180,
                funct7: 0xc,
            },
            Opcode::VFMERGEVFM => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5c0,
                funct7: 0x2e,
            },
            Opcode::VFMINVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x100,
                funct7: 0x8,
            },
            Opcode::VFMINVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x100,
                funct7: 0x8,
            },
            Opcode::VFMSACVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb80,
                funct7: 0x5c,
            },
            Opcode::VFMSACVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb80,
                funct7: 0x5c,
            },
            Opcode::VFMSUBVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa80,
                funct7: 0x54,
            },
            Opcode::VFMSUBVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa80,
                funct7: 0x54,
            },
            Opcode::VFMULVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x900,
                funct7: 0x48,
            },
            Opcode::VFMULVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x900,
                funct7: 0x48,
            },
            Opcode::VFMVFS => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x420,
                funct7: 0x21,
            },
            Opcode::VFMVSF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x420,
                funct7: 0x21,
            },
            Opcode::VFMVVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5e0,
                funct7: 0x2f,
            },
            Opcode::VFNCVTFFW => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x14,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFNCVTFXW => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x13,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFNCVTFXUW => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x12,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFNCVTRODFFW => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x15,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFNCVTRTZXFW => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x17,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFNCVTRTZXUFW => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x16,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFNCVTXFW => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x11,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFNCVTXUFW => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x10,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFNMACCVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb40,
                funct7: 0x5a,
            },
            Opcode::VFNMACCVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb40,
                funct7: 0x5a,
            },
            Opcode::VFNMADDVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::VFNMADDVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::VFNMSACVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xbc0,
                funct7: 0x5e,
            },
            Opcode::VFNMSACVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xbc0,
                funct7: 0x5e,
            },
            Opcode::VFNMSUBVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xac0,
                funct7: 0x56,
            },
            Opcode::VFNMSUBVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xac0,
                funct7: 0x56,
            },
            Opcode::VFRDIVVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x840,
                funct7: 0x42,
            },
            Opcode::VFREC7V => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x5,
                rs2: 0x0,
                csr: 0x4c0,
                funct7: 0x26,
            },
            Opcode::VFREDMAXVS => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x1c0,
                funct7: 0xe,
            },
            Opcode::VFREDMINVS => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x140,
                funct7: 0xa,
            },
            Opcode::VFREDOSUMVS => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VFREDSUMVS => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VFREDUSUMVS => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VFRSQRT7V => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x4,
                rs2: 0x0,
                csr: 0x4c0,
                funct7: 0x26,
            },
            Opcode::VFRSUBVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x9c0,
                funct7: 0x4e,
            },
            Opcode::VFSGNJVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::VFSGNJVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x200,
                funct7: 0x10,
            },
            Opcode::VFSGNJNVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::VFSGNJNVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x240,
                funct7: 0x12,
            },
            Opcode::VFSGNJXVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::VFSGNJXVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::VFSLIDE1DOWNVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x3c0,
                funct7: 0x1e,
            },
            Opcode::VFSLIDE1UPVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x380,
                funct7: 0x1c,
            },
            Opcode::VFSQRTV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x4c0,
                funct7: 0x26,
            },
            Opcode::VFSUBVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VFSUBVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VFWADDVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::VFWADDVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::VFWADDWF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd00,
                funct7: 0x68,
            },
            Opcode::VFWADDWV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd00,
                funct7: 0x68,
            },
            Opcode::VFWCVTFFV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0xc,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFWCVTFXV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0xb,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFWCVTFXUV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0xa,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFWCVTRTZXFV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0xf,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFWCVTRTZXUFV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0xe,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFWCVTXFV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x9,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFWCVTXUFV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x8,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VFWMACCVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf00,
                funct7: 0x78,
            },
            Opcode::VFWMACCVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf00,
                funct7: 0x78,
            },
            Opcode::VFWMSACVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf80,
                funct7: 0x7c,
            },
            Opcode::VFWMSACVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf80,
                funct7: 0x7c,
            },
            Opcode::VFWMULVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe00,
                funct7: 0x70,
            },
            Opcode::VFWMULVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe00,
                funct7: 0x70,
            },
            Opcode::VFWNMACCVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf40,
                funct7: 0x7a,
            },
            Opcode::VFWNMACCVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf40,
                funct7: 0x7a,
            },
            Opcode::VFWNMSACVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xfc0,
                funct7: 0x7e,
            },
            Opcode::VFWNMSACVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xfc0,
                funct7: 0x7e,
            },
            Opcode::VFWREDOSUMVS => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xcc0,
                funct7: 0x66,
            },
            Opcode::VFWREDSUMVS => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc40,
                funct7: 0x62,
            },
            Opcode::VFWREDUSUMVS => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc40,
                funct7: 0x62,
            },
            Opcode::VFWSUBVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc80,
                funct7: 0x64,
            },
            Opcode::VFWSUBVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc80,
                funct7: 0x64,
            },
            Opcode::VFWSUBWF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd80,
                funct7: 0x6c,
            },
            Opcode::VFWSUBWV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd80,
                funct7: 0x6c,
            },
            Opcode::VGHSHVV => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb20,
                funct7: 0x59,
            },
            Opcode::VGMULVV => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x11,
                rs2: 0x0,
                csr: 0xa20,
                funct7: 0x51,
            },
            Opcode::VIDV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x11,
                rs2: 0x0,
                csr: 0x500,
                funct7: 0x28,
            },
            Opcode::VIOTAM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x10,
                rs2: 0x0,
                csr: 0x500,
                funct7: 0x28,
            },
            Opcode::VL1RV => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x28,
                funct7: 0x1,
            },
            Opcode::VL1RE16V => Inst {
                opcode: 0x7,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x28,
                funct7: 0x1,
            },
            Opcode::VL1RE32V => Inst {
                opcode: 0x7,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x28,
                funct7: 0x1,
            },
            Opcode::VL1RE64V => Inst {
                opcode: 0x7,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x28,
                funct7: 0x1,
            },
            Opcode::VL1RE8V => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x28,
                funct7: 0x1,
            },
            Opcode::VL2RV => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x228,
                funct7: 0x11,
            },
            Opcode::VL2RE16V => Inst {
                opcode: 0x7,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x228,
                funct7: 0x11,
            },
            Opcode::VL2RE32V => Inst {
                opcode: 0x7,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x228,
                funct7: 0x11,
            },
            Opcode::VL2RE64V => Inst {
                opcode: 0x7,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x228,
                funct7: 0x11,
            },
            Opcode::VL2RE8V => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x228,
                funct7: 0x11,
            },
            Opcode::VL4RV => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x628,
                funct7: 0x31,
            },
            Opcode::VL4RE16V => Inst {
                opcode: 0x7,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x628,
                funct7: 0x31,
            },
            Opcode::VL4RE32V => Inst {
                opcode: 0x7,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x628,
                funct7: 0x31,
            },
            Opcode::VL4RE64V => Inst {
                opcode: 0x7,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x628,
                funct7: 0x31,
            },
            Opcode::VL4RE8V => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x628,
                funct7: 0x31,
            },
            Opcode::VL8RV => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0xe28,
                funct7: 0x71,
            },
            Opcode::VL8RE16V => Inst {
                opcode: 0x7,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0xe28,
                funct7: 0x71,
            },
            Opcode::VL8RE32V => Inst {
                opcode: 0x7,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0xe28,
                funct7: 0x71,
            },
            Opcode::VL8RE64V => Inst {
                opcode: 0x7,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0xe28,
                funct7: 0x71,
            },
            Opcode::VL8RE8V => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0xe28,
                funct7: 0x71,
            },
            Opcode::VLE16V => Inst {
                opcode: 0x7,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VLE16FFV => Inst {
                opcode: 0x7,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x10,
                csr: 0x10,
                funct7: 0x0,
            },
            Opcode::VLE1V => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0xb,
                csr: 0x2b,
                funct7: 0x1,
            },
            Opcode::VLE32V => Inst {
                opcode: 0x7,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VLE32FFV => Inst {
                opcode: 0x7,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x10,
                csr: 0x10,
                funct7: 0x0,
            },
            Opcode::VLE64V => Inst {
                opcode: 0x7,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VLE64FFV => Inst {
                opcode: 0x7,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x10,
                csr: 0x10,
                funct7: 0x0,
            },
            Opcode::VLE8V => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VLE8FFV => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x10,
                csr: 0x10,
                funct7: 0x0,
            },
            Opcode::VLMV => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0xb,
                csr: 0x2b,
                funct7: 0x1,
            },
            Opcode::VLOXEI16V => Inst {
                opcode: 0x7,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VLOXEI32V => Inst {
                opcode: 0x7,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VLOXEI64V => Inst {
                opcode: 0x7,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VLOXEI8V => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VLSE16V => Inst {
                opcode: 0x7,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VLSE32V => Inst {
                opcode: 0x7,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VLSE64V => Inst {
                opcode: 0x7,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VLSE8V => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VLUXEI16V => Inst {
                opcode: 0x7,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VLUXEI32V => Inst {
                opcode: 0x7,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VLUXEI64V => Inst {
                opcode: 0x7,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VLUXEI8V => Inst {
                opcode: 0x7,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VMACCVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb40,
                funct7: 0x5a,
            },
            Opcode::VMACCVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb40,
                funct7: 0x5a,
            },
            Opcode::VMADCVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x460,
                funct7: 0x23,
            },
            Opcode::VMADCVIM => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x440,
                funct7: 0x22,
            },
            Opcode::VMADCVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x460,
                funct7: 0x23,
            },
            Opcode::VMADCVVM => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x440,
                funct7: 0x22,
            },
            Opcode::VMADCVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x460,
                funct7: 0x23,
            },
            Opcode::VMADCVXM => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x440,
                funct7: 0x22,
            },
            Opcode::VMADDVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::VMADDVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::VMANDMM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x660,
                funct7: 0x33,
            },
            Opcode::VMANDNMM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x620,
                funct7: 0x31,
            },
            Opcode::VMANDNOTMM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::VMAXVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x1c0,
                funct7: 0xe,
            },
            Opcode::VMAXVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x1c0,
                funct7: 0xe,
            },
            Opcode::VMAXUVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x180,
                funct7: 0xc,
            },
            Opcode::VMAXUVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x180,
                funct7: 0xc,
            },
            Opcode::VMERGEVIM => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5c0,
                funct7: 0x2e,
            },
            Opcode::VMERGEVVM => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5c0,
                funct7: 0x2e,
            },
            Opcode::VMERGEVXM => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5c0,
                funct7: 0x2e,
            },
            Opcode::VMFEQVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::VMFEQVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::VMFGEVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x7c0,
                funct7: 0x3e,
            },
            Opcode::VMFGTVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x740,
                funct7: 0x3a,
            },
            Opcode::VMFLEVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x640,
                funct7: 0x32,
            },
            Opcode::VMFLEVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x640,
                funct7: 0x32,
            },
            Opcode::VMFLTVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x6c0,
                funct7: 0x36,
            },
            Opcode::VMFLTVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x6c0,
                funct7: 0x36,
            },
            Opcode::VMFNEVF => Inst {
                opcode: 0x57,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x700,
                funct7: 0x38,
            },
            Opcode::VMFNEVV => Inst {
                opcode: 0x57,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x700,
                funct7: 0x38,
            },
            Opcode::VMINVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x140,
                funct7: 0xa,
            },
            Opcode::VMINVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x140,
                funct7: 0xa,
            },
            Opcode::VMINUVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x100,
                funct7: 0x8,
            },
            Opcode::VMINUVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x100,
                funct7: 0x8,
            },
            Opcode::VMNANDMM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x760,
                funct7: 0x3b,
            },
            Opcode::VMNORMM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x7a0,
                funct7: 0x3d,
            },
            Opcode::VMORMM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x6a0,
                funct7: 0x35,
            },
            Opcode::VMORNMM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x720,
                funct7: 0x39,
            },
            Opcode::VMORNOTMM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x700,
                funct7: 0x38,
            },
            Opcode::VMSBCVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x4e0,
                funct7: 0x27,
            },
            Opcode::VMSBCVVM => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x4c0,
                funct7: 0x26,
            },
            Opcode::VMSBCVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x4e0,
                funct7: 0x27,
            },
            Opcode::VMSBCVXM => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x4c0,
                funct7: 0x26,
            },
            Opcode::VMSBFM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x500,
                funct7: 0x28,
            },
            Opcode::VMSEQVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::VMSEQVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::VMSEQVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x600,
                funct7: 0x30,
            },
            Opcode::VMSGTVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x7c0,
                funct7: 0x3e,
            },
            Opcode::VMSGTVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x7c0,
                funct7: 0x3e,
            },
            Opcode::VMSGTUVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x780,
                funct7: 0x3c,
            },
            Opcode::VMSGTUVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x780,
                funct7: 0x3c,
            },
            Opcode::VMSIFM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x3,
                rs2: 0x0,
                csr: 0x500,
                funct7: 0x28,
            },
            Opcode::VMSLEVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x740,
                funct7: 0x3a,
            },
            Opcode::VMSLEVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x740,
                funct7: 0x3a,
            },
            Opcode::VMSLEVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x740,
                funct7: 0x3a,
            },
            Opcode::VMSLEUVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x700,
                funct7: 0x38,
            },
            Opcode::VMSLEUVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x700,
                funct7: 0x38,
            },
            Opcode::VMSLEUVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x700,
                funct7: 0x38,
            },
            Opcode::VMSLTVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x6c0,
                funct7: 0x36,
            },
            Opcode::VMSLTVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x6c0,
                funct7: 0x36,
            },
            Opcode::VMSLTUVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x680,
                funct7: 0x34,
            },
            Opcode::VMSLTUVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x680,
                funct7: 0x34,
            },
            Opcode::VMSNEVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x640,
                funct7: 0x32,
            },
            Opcode::VMSNEVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x640,
                funct7: 0x32,
            },
            Opcode::VMSNEVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x640,
                funct7: 0x32,
            },
            Opcode::VMSOFM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x2,
                rs2: 0x0,
                csr: 0x500,
                funct7: 0x28,
            },
            Opcode::VMULVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x940,
                funct7: 0x4a,
            },
            Opcode::VMULVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x940,
                funct7: 0x4a,
            },
            Opcode::VMULHVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x9c0,
                funct7: 0x4e,
            },
            Opcode::VMULHVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x9c0,
                funct7: 0x4e,
            },
            Opcode::VMULHSUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x980,
                funct7: 0x4c,
            },
            Opcode::VMULHSUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x980,
                funct7: 0x4c,
            },
            Opcode::VMULHUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x900,
                funct7: 0x48,
            },
            Opcode::VMULHUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x900,
                funct7: 0x48,
            },
            Opcode::VMV1RV => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x9e0,
                funct7: 0x4f,
            },
            Opcode::VMV2RV => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x1,
                rs2: 0x0,
                csr: 0x9e0,
                funct7: 0x4f,
            },
            Opcode::VMV4RV => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x3,
                rs2: 0x0,
                csr: 0x9e0,
                funct7: 0x4f,
            },
            Opcode::VMV8RV => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x7,
                rs2: 0x0,
                csr: 0x9e0,
                funct7: 0x4f,
            },
            Opcode::VMVSX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x420,
                funct7: 0x21,
            },
            Opcode::VMVVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5e0,
                funct7: 0x2f,
            },
            Opcode::VMVVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5e0,
                funct7: 0x2f,
            },
            Opcode::VMVVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x5e0,
                funct7: 0x2f,
            },
            Opcode::VMVXS => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x420,
                funct7: 0x21,
            },
            Opcode::VMXNORMM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x7e0,
                funct7: 0x3f,
            },
            Opcode::VMXORMM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x6e0,
                funct7: 0x37,
            },
            Opcode::VNCLIPWI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xbc0,
                funct7: 0x5e,
            },
            Opcode::VNCLIPWV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xbc0,
                funct7: 0x5e,
            },
            Opcode::VNCLIPWX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xbc0,
                funct7: 0x5e,
            },
            Opcode::VNCLIPUWI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb80,
                funct7: 0x5c,
            },
            Opcode::VNCLIPUWV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb80,
                funct7: 0x5c,
            },
            Opcode::VNCLIPUWX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb80,
                funct7: 0x5c,
            },
            Opcode::VNMSACVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xbc0,
                funct7: 0x5e,
            },
            Opcode::VNMSACVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xbc0,
                funct7: 0x5e,
            },
            Opcode::VNMSUBVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xac0,
                funct7: 0x56,
            },
            Opcode::VNMSUBVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xac0,
                funct7: 0x56,
            },
            Opcode::VNSRAWI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb40,
                funct7: 0x5a,
            },
            Opcode::VNSRAWV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb40,
                funct7: 0x5a,
            },
            Opcode::VNSRAWX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb40,
                funct7: 0x5a,
            },
            Opcode::VNSRLWI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb00,
                funct7: 0x58,
            },
            Opcode::VNSRLWV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb00,
                funct7: 0x58,
            },
            Opcode::VNSRLWX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb00,
                funct7: 0x58,
            },
            Opcode::VORVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::VORVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::VORVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::VPOPCM => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x10,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::VREDANDVS => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VREDMAXVS => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x1c0,
                funct7: 0xe,
            },
            Opcode::VREDMAXUVS => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x180,
                funct7: 0xc,
            },
            Opcode::VREDMINVS => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x140,
                funct7: 0xa,
            },
            Opcode::VREDMINUVS => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x100,
                funct7: 0x8,
            },
            Opcode::VREDORVS => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VREDSUMVS => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VREDXORVS => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VREMVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x8c0,
                funct7: 0x46,
            },
            Opcode::VREMVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x8c0,
                funct7: 0x46,
            },
            Opcode::VREMUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x880,
                funct7: 0x44,
            },
            Opcode::VREMUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x880,
                funct7: 0x44,
            },
            Opcode::VREV8V => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x9,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VRGATHERVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x300,
                funct7: 0x18,
            },
            Opcode::VRGATHERVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x300,
                funct7: 0x18,
            },
            Opcode::VRGATHERVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x300,
                funct7: 0x18,
            },
            Opcode::VRGATHEREI16VV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x380,
                funct7: 0x1c,
            },
            Opcode::VROLVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x540,
                funct7: 0x2a,
            },
            Opcode::VROLVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x540,
                funct7: 0x2a,
            },
            Opcode::VRORVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x500,
                funct7: 0x28,
            },
            Opcode::VRORVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x500,
                funct7: 0x28,
            },
            Opcode::VRORVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x500,
                funct7: 0x28,
            },
            Opcode::VRSUBVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VRSUBVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VS1RV => Inst {
                opcode: 0x27,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x28,
                funct7: 0x1,
            },
            Opcode::VS2RV => Inst {
                opcode: 0x27,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x228,
                funct7: 0x11,
            },
            Opcode::VS4RV => Inst {
                opcode: 0x27,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0x628,
                funct7: 0x31,
            },
            Opcode::VS8RV => Inst {
                opcode: 0x27,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x8,
                csr: 0xe28,
                funct7: 0x71,
            },
            Opcode::VSADDVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x840,
                funct7: 0x42,
            },
            Opcode::VSADDVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x840,
                funct7: 0x42,
            },
            Opcode::VSADDVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x840,
                funct7: 0x42,
            },
            Opcode::VSADDUVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::VSADDUVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::VSADDUVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::VSBCVVM => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VSBCVXM => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VSE16V => Inst {
                opcode: 0x27,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VSE1V => Inst {
                opcode: 0x27,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0xb,
                csr: 0x2b,
                funct7: 0x1,
            },
            Opcode::VSE32V => Inst {
                opcode: 0x27,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VSE64V => Inst {
                opcode: 0x27,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VSE8V => Inst {
                opcode: 0x27,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VSETIVLI => Inst {
                opcode: 0x57,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::VSETVL => Inst {
                opcode: 0x57,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x800,
                funct7: 0x40,
            },
            Opcode::VSETVLI => Inst {
                opcode: 0x57,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::VSEXTVF2 => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x7,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VSEXTVF4 => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x5,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VSEXTVF8 => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x3,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VSHA2CHVV => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xba0,
                funct7: 0x5d,
            },
            Opcode::VSHA2CLVV => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xbe0,
                funct7: 0x5f,
            },
            Opcode::VSHA2MSVV => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xb60,
                funct7: 0x5b,
            },
            Opcode::VSLIDE1DOWNVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x3c0,
                funct7: 0x1e,
            },
            Opcode::VSLIDE1UPVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x380,
                funct7: 0x1c,
            },
            Opcode::VSLIDEDOWNVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x3c0,
                funct7: 0x1e,
            },
            Opcode::VSLIDEDOWNVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x3c0,
                funct7: 0x1e,
            },
            Opcode::VSLIDEUPVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x380,
                funct7: 0x1c,
            },
            Opcode::VSLIDEUPVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x380,
                funct7: 0x1c,
            },
            Opcode::VSLLVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x940,
                funct7: 0x4a,
            },
            Opcode::VSLLVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x940,
                funct7: 0x4a,
            },
            Opcode::VSLLVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x940,
                funct7: 0x4a,
            },
            Opcode::VSM3CVI => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xae0,
                funct7: 0x57,
            },
            Opcode::VSM3MEVV => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x820,
                funct7: 0x41,
            },
            Opcode::VSM4KVI => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x860,
                funct7: 0x43,
            },
            Opcode::VSM4RVS => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x10,
                rs2: 0x0,
                csr: 0xa60,
                funct7: 0x53,
            },
            Opcode::VSM4RVV => Inst {
                opcode: 0x77,
                funct3: 0x2,
                rs1: 0x10,
                rs2: 0x0,
                csr: 0xa20,
                funct7: 0x51,
            },
            Opcode::VSMV => Inst {
                opcode: 0x27,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0xb,
                csr: 0x2b,
                funct7: 0x1,
            },
            Opcode::VSMULVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x9c0,
                funct7: 0x4e,
            },
            Opcode::VSMULVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x9c0,
                funct7: 0x4e,
            },
            Opcode::VSOXEI16V => Inst {
                opcode: 0x27,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VSOXEI32V => Inst {
                opcode: 0x27,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VSOXEI64V => Inst {
                opcode: 0x27,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VSOXEI8V => Inst {
                opcode: 0x27,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc0,
                funct7: 0x6,
            },
            Opcode::VSRAVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::VSRAVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::VSRAVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa40,
                funct7: 0x52,
            },
            Opcode::VSRLVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::VSRLVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::VSRLVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa00,
                funct7: 0x50,
            },
            Opcode::VSSE16V => Inst {
                opcode: 0x27,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VSSE32V => Inst {
                opcode: 0x27,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VSSE64V => Inst {
                opcode: 0x27,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VSSE8V => Inst {
                opcode: 0x27,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VSSRAVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xac0,
                funct7: 0x56,
            },
            Opcode::VSSRAVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xac0,
                funct7: 0x56,
            },
            Opcode::VSSRAVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xac0,
                funct7: 0x56,
            },
            Opcode::VSSRLVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa80,
                funct7: 0x54,
            },
            Opcode::VSSRLVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa80,
                funct7: 0x54,
            },
            Opcode::VSSRLVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xa80,
                funct7: 0x54,
            },
            Opcode::VSSUBVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x8c0,
                funct7: 0x46,
            },
            Opcode::VSSUBVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x8c0,
                funct7: 0x46,
            },
            Opcode::VSSUBUVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x880,
                funct7: 0x44,
            },
            Opcode::VSSUBUVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x880,
                funct7: 0x44,
            },
            Opcode::VSUBVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VSUBVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::VSUXEI16V => Inst {
                opcode: 0x27,
                funct3: 0x5,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VSUXEI32V => Inst {
                opcode: 0x27,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VSUXEI64V => Inst {
                opcode: 0x27,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VSUXEI8V => Inst {
                opcode: 0x27,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x40,
                funct7: 0x2,
            },
            Opcode::VWADDVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc40,
                funct7: 0x62,
            },
            Opcode::VWADDVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc40,
                funct7: 0x62,
            },
            Opcode::VWADDWV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd40,
                funct7: 0x6a,
            },
            Opcode::VWADDWX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd40,
                funct7: 0x6a,
            },
            Opcode::VWADDUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::VWADDUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::VWADDUWV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd00,
                funct7: 0x68,
            },
            Opcode::VWADDUWX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd00,
                funct7: 0x68,
            },
            Opcode::VWMACCVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf40,
                funct7: 0x7a,
            },
            Opcode::VWMACCVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf40,
                funct7: 0x7a,
            },
            Opcode::VWMACCSUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xfc0,
                funct7: 0x7e,
            },
            Opcode::VWMACCSUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xfc0,
                funct7: 0x7e,
            },
            Opcode::VWMACCUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf00,
                funct7: 0x78,
            },
            Opcode::VWMACCUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf00,
                funct7: 0x78,
            },
            Opcode::VWMACCUSVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xf80,
                funct7: 0x7c,
            },
            Opcode::VWMULVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xec0,
                funct7: 0x76,
            },
            Opcode::VWMULVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xec0,
                funct7: 0x76,
            },
            Opcode::VWMULSUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe80,
                funct7: 0x74,
            },
            Opcode::VWMULSUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe80,
                funct7: 0x74,
            },
            Opcode::VWMULUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe00,
                funct7: 0x70,
            },
            Opcode::VWMULUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xe00,
                funct7: 0x70,
            },
            Opcode::VWREDSUMVS => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc40,
                funct7: 0x62,
            },
            Opcode::VWREDSUMUVS => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc00,
                funct7: 0x60,
            },
            Opcode::VWSLLVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd40,
                funct7: 0x6a,
            },
            Opcode::VWSLLVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd40,
                funct7: 0x6a,
            },
            Opcode::VWSLLVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd40,
                funct7: 0x6a,
            },
            Opcode::VWSUBVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xcc0,
                funct7: 0x66,
            },
            Opcode::VWSUBVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xcc0,
                funct7: 0x66,
            },
            Opcode::VWSUBWV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xdc0,
                funct7: 0x6e,
            },
            Opcode::VWSUBWX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xdc0,
                funct7: 0x6e,
            },
            Opcode::VWSUBUVV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc80,
                funct7: 0x64,
            },
            Opcode::VWSUBUVX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xc80,
                funct7: 0x64,
            },
            Opcode::VWSUBUWV => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd80,
                funct7: 0x6c,
            },
            Opcode::VWSUBUWX => Inst {
                opcode: 0x57,
                funct3: 0x6,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0xd80,
                funct7: 0x6c,
            },
            Opcode::VXORVI => Inst {
                opcode: 0x57,
                funct3: 0x3,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2c0,
                funct7: 0x16,
            },
            Opcode::VXORVV => Inst {
                opcode: 0x57,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2c0,
                funct7: 0x16,
            },
            Opcode::VXORVX => Inst {
                opcode: 0x57,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x2c0,
                funct7: 0x16,
            },
            Opcode::VZEXTVF2 => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x6,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VZEXTVF4 => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x4,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::VZEXTVF8 => Inst {
                opcode: 0x57,
                funct3: 0x2,
                rs1: 0x2,
                rs2: 0x0,
                csr: 0x480,
                funct7: 0x24,
            },
            Opcode::WFI => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x5,
                csr: 0x105,
                funct7: 0x8,
            },
            Opcode::WRSNTO => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0xd,
                csr: 0xd,
                funct7: 0x0,
            },
            Opcode::WRSSTO => Inst {
                opcode: 0x73,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x1d,
                csr: 0x1d,
                funct7: 0x0,
            },
            Opcode::XNOR => Inst {
                opcode: 0x33,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x400,
                funct7: 0x20,
            },
            Opcode::XOR => Inst {
                opcode: 0x33,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::XORI => Inst {
                opcode: 0x13,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::XPERM4 => Inst {
                opcode: 0x33,
                funct3: 0x2,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::XPERM8 => Inst {
                opcode: 0x33,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x280,
                funct7: 0x14,
            },
            Opcode::ZEXTB => Inst {
                opcode: 0x13,
                funct3: 0x7,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x0,
                funct7: 0x0,
            },
            Opcode::ZEXTH => Inst {
                opcode: 0x3b,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::ZEXTHRV32 => Inst {
                opcode: 0x33,
                funct3: 0x4,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::ZEXTW => Inst {
                opcode: 0x3b,
                funct3: 0x0,
                rs1: 0x0,
                rs2: 0x0,
                csr: 0x80,
                funct7: 0x4,
            },
            Opcode::ZIP => Inst {
                opcode: 0x13,
                funct3: 0x1,
                rs1: 0x0,
                rs2: 0xf,
                csr: 0x8f,
                funct7: 0x4,
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Encoding {
    Bimm12HiRs1Bimm12lo,
    Bimm12HiRs1Rs2Bimm12lo,
    Bimm12HiRs2Bimm12lo,
    Bimm12HiRs2Rs1Bimm12lo,
    CImm12,
    CIndex,
    CMopT,
    CNzimm10hiCNzimm10lo,
    CNzimm6hiCNzimm6lo,
    CRlistCSpimm,
    CRs1N0,
    CRs2CUimm8spS,
    CRs2CUimm9spS,
    CSreg1CSreg2,
    CsrZimm,
    Empty,
    FmPredSuccRs1Rd,
    Imm12HiRs1Rs2Imm12lo,
    Imm12Rs1Rd,
    Jimm20,
    MopRT30MopRT2726MopRT2120RdRs1,
    MopRrT30MopRrT2726RdRs1Rs2,
    NfVmRs1Vd,
    NfVmRs1Vs3,
    NfVmRs2Rs1Vd,
    NfVmRs2Rs1Vs3,
    NfVmVs2Rs1Vd,
    NfVmVs2Rs1Vs3,
    Rd,
    RdCUimm8sphiCUimm8splo,
    RdCUimm9sphiCUimm9splo,
    RdCsr,
    RdCsrZimm,
    RdImm20,
    RdJimm20,
    RdN0CImm6loCImm6hi,
    RdN0CRs2N0,
    RdN0CUimm8sphiCUimm8splo,
    RdN0CUimm9sphiCUimm9splo,
    RdN2CNzimm18hiCNzimm18lo,
    RdPCNzuimm10,
    RdPRs1PCUimm1,
    RdPRs1PCUimm2,
    RdPRs1PCUimm7loCUimm7hi,
    RdPRs1PCUimm8loCUimm8hi,
    RdRs1,
    RdRs1AqRl,
    RdRs1Csr,
    RdRs1Imm12,
    RdRs1N0,
    RdRs1N0CImm6loCImm6hi,
    RdRs1N0CNzimm6loCNzimm6hi,
    RdRs1N0CNzuimm6hiCNzuimm6lo,
    RdRs1N0CNzuimm6lo,
    RdRs1N0CRs2N0,
    RdRs1P,
    RdRs1PCImm6hiCImm6lo,
    RdRs1PCNzuimm5,
    RdRs1PCNzuimm6loCNzuimm6hi,
    RdRs1PRs2P,
    RdRs1Rm,
    RdRs1Rnum,
    RdRs1Rs2,
    RdRs1Rs2AqRl,
    RdRs1Rs2Bs,
    RdRs1Rs2EqRs1,
    RdRs1Rs2Rm,
    RdRs1Rs2Rs3Rm,
    RdRs1Shamtd,
    RdRs1Shamtw,
    RdRs2,
    RdZimm,
    Rs1,
    Rs1Csr,
    Rs1Imm12hi,
    Rs1N0,
    Rs1PCBimm9loCBimm9hi,
    Rs1PRs2PCUimm7loCUimm7hi,
    Rs1PRs2PCUimm8hiCUimm8lo,
    Rs1PRs2PCUimm8loCUimm8hi,
    Rs1Rd,
    Rs1Rs2,
    Rs1Vd,
    Rs1Vs3,
    Rs2PRs1PCUimm1,
    Rs2PRs1PCUimm2,
    Rs2Rs1Rd,
    Simm5Vd,
    VmVd,
    VmVs2Rd,
    VmVs2Rs1Vd,
    VmVs2Simm5Vd,
    VmVs2Vd,
    VmVs2Vs1Vd,
    VmVs2Zimm5Vd,
    Vs1Vd,
    Vs2Rd,
    Vs2Rs1Vd,
    Vs2Simm5Vd,
    Vs2Vd,
    Vs2Vs1Vd,
    Vs2Zimm5Vd,
    Zimm10ZimmRd,
    Zimm11Rs1Rd,
    Zimm6HiVmVs2Zimm6loVd,
}

impl Opcode {
    pub fn encoding(self) -> Encoding {
        use Opcode::*;
        match self {
            Opcode::Invalid => unreachable!(),
            BEQZ | BGEZ | BLTZ | BNEZ => Encoding::Bimm12HiRs1Bimm12lo,
            BEQ | BGE | BGEU | BLT | BLTU | BNE => Encoding::Bimm12HiRs1Rs2Bimm12lo,
            BGTZ | BLEZ => Encoding::Bimm12HiRs2Bimm12lo,
            BGT | BGTU | BLE | BLEU => Encoding::Bimm12HiRs2Rs1Bimm12lo,
            CJ | CJAL => Encoding::CImm12,
            CMJALT => Encoding::CIndex,
            CMOPN => Encoding::CMopT,
            CADDI16SP => Encoding::CNzimm10hiCNzimm10lo,
            CNOP => Encoding::CNzimm6hiCNzimm6lo,
            CMPOP | CMPOPRET | CMPOPRETZ | CMPUSH => Encoding::CRlistCSpimm,
            CJALR => Encoding::CRs1N0,
            CFSWSP | CSWSP => Encoding::CRs2CUimm8spS,
            CFSDSP | CSDSP => Encoding::CRs2CUimm9spS,
            CMMVA01S | CMMVSA01 => Encoding::CSreg1CSreg2,
            CSRCI | CSRSI | CSRWI => Encoding::CsrZimm,
            CEBREAK | CMOP1 | CMOP11 | CMOP13 | CMOP15 | CMOP3 | CMOP5 | CMOP7 | CMOP9
            | CNTLALL | CNTLP1 | CNTLPALL | CNTLS1 | DRET | EBREAK | ECALL | MRET | NOP
            | NTLALL | NTLP1 | NTLPALL | NTLS1 | PAUSE | RET | SBREAK | SCALL | SFENCEINVALIR
            | SFENCEWINVAL | SRET | WFI | WRSNTO | WRSSTO => Encoding::Empty,
            FENCE => Encoding::FmPredSuccRs1Rd,
            FSD | FSH | FSQ | FSW | SB | SD | SH | SW => Encoding::Imm12HiRs1Rs2Imm12lo,
            FENCEI => Encoding::Imm12Rs1Rd,
            J | JALPSEUDO => Encoding::Jimm20,
            MOPRN => Encoding::MopRT30MopRT2726MopRT2120RdRs1,
            MOPRRN => Encoding::MopRrT30MopRrT2726RdRs1Rs2,
            VLE16V | VLE16FFV | VLE32V | VLE32FFV | VLE64V | VLE64FFV | VLE8V | VLE8FFV => {
                Encoding::NfVmRs1Vd
            }
            VSE16V | VSE32V | VSE64V | VSE8V => Encoding::NfVmRs1Vs3,
            VLSE16V | VLSE32V | VLSE64V | VLSE8V => Encoding::NfVmRs2Rs1Vd,
            VSSE16V | VSSE32V | VSSE64V | VSSE8V => Encoding::NfVmRs2Rs1Vs3,
            VLOXEI16V | VLOXEI32V | VLOXEI64V | VLOXEI8V | VLUXEI16V | VLUXEI32V | VLUXEI64V
            | VLUXEI8V => Encoding::NfVmVs2Rs1Vd,
            VSOXEI16V | VSOXEI32V | VSOXEI64V | VSOXEI8V | VSUXEI16V | VSUXEI32V | VSUXEI64V
            | VSUXEI8V => Encoding::NfVmVs2Rs1Vs3,
            FRCSR | FRFLAGS | FRRM | RDCYCLE | RDCYCLEH | RDINSTRET | RDINSTRETH | RDTIME
            | RDTIMEH => Encoding::Rd,
            CFLWSP => Encoding::RdCUimm8sphiCUimm8splo,
            CFLDSP => Encoding::RdCUimm9sphiCUimm9splo,
            CSRR => Encoding::RdCsr,
            CSRRCI | CSRRSI | CSRRWI => Encoding::RdCsrZimm,
            AUIPC | LUI => Encoding::RdImm20,
            JAL => Encoding::RdJimm20,
            CLI => Encoding::RdN0CImm6loCImm6hi,
            CMV => Encoding::RdN0CRs2N0,
            CLWSP => Encoding::RdN0CUimm8sphiCUimm8splo,
            CLDSP => Encoding::RdN0CUimm9sphiCUimm9splo,
            CLUI => Encoding::RdN2CNzimm18hiCNzimm18lo,
            CADDI4SPN => Encoding::RdPCNzuimm10,
            CLH | CLHU => Encoding::RdPRs1PCUimm1,
            CLBU => Encoding::RdPRs1PCUimm2,
            CFLW | CLW => Encoding::RdPRs1PCUimm7loCUimm7hi,
            CFLD | CLD => Encoding::RdPRs1PCUimm8loCUimm8hi,
            AES64IM | BREV8 | CLZ | CLZW | CPOP | CPOPW | CTZ | CTZW | FCLASSD | FCLASSH
            | FCLASSQ | FCLASSS | FCVTMODWD | FLID | FLIH | FLIQ | FLIS | FMVDX | FMVHX | FMVSX
            | FMVWX | FMVXD | FMVXH | FMVXS | FMVXW | FMVHXD | FMVHXQ | FSCSR | FSFLAGS | FSRM
            | HLVB | HLVBU | HLVD | HLVH | HLVHU | HLVW | HLVWU | HLVXHU | HLVXWU | MOPR0
            | MOPR1 | MOPR10 | MOPR11 | MOPR12 | MOPR13 | MOPR14 | MOPR15 | MOPR16 | MOPR17
            | MOPR18 | MOPR19 | MOPR2 | MOPR20 | MOPR21 | MOPR22 | MOPR23 | MOPR24 | MOPR25
            | MOPR26 | MOPR27 | MOPR28 | MOPR29 | MOPR3 | MOPR30 | MOPR31 | MOPR4 | MOPR5
            | MOPR6 | MOPR7 | MOPR8 | MOPR9 | MV | NEG | ORCB | REV8 | REV8RV32 | SEQZ | SEXTB
            | SEXTH | SEXTW | SHA256SIG0 | SHA256SIG1 | SHA256SUM0 | SHA256SUM1 | SHA512SIG0
            | SHA512SIG1 | SHA512SUM0 | SHA512SUM1 | SLTZ | SM3P0 | SM3P1 | UNZIP | ZEXTB
            | ZEXTH | ZEXTHRV32 | ZEXTW | ZIP => Encoding::RdRs1,
            LRD | LRW => Encoding::RdRs1AqRl,
            CSRRC | CSRRS | CSRRW => Encoding::RdRs1Csr,
            ADDI | ADDIW | ANDI | FLD | FLH | FLQ | FLW | JALR | LB | LBU | LD | LH | LHU | LW
            | LWU | ORI | SLTI | SLTIU | XORI => Encoding::RdRs1Imm12,
            CSEXTW => Encoding::RdRs1N0,
            CADDIW => Encoding::RdRs1N0CImm6loCImm6hi,
            CADDI => Encoding::RdRs1N0CNzimm6loCNzimm6hi,
            CSLLI => Encoding::RdRs1N0CNzuimm6hiCNzuimm6lo,
            CSLLIRV32 => Encoding::RdRs1N0CNzuimm6lo,
            CADD => Encoding::RdRs1N0CRs2N0,
            CNOT | CSEXTB | CSEXTH | CZEXTB | CZEXTH | CZEXTW => Encoding::RdRs1P,
            CANDI => Encoding::RdRs1PCImm6hiCImm6lo,
            CSRAIRV32 | CSRLIRV32 => Encoding::RdRs1PCNzuimm5,
            CSRAI | CSRLI => Encoding::RdRs1PCNzuimm6loCNzuimm6hi,
            CADDW | CAND | CMUL | COR | CSUB | CSUBW | CXOR => Encoding::RdRs1PRs2P,
            FCVTDH | FCVTDL | FCVTDLU | FCVTDQ | FCVTDS | FCVTDW | FCVTDWU | FCVTHD | FCVTHL
            | FCVTHLU | FCVTHQ | FCVTHS | FCVTHW | FCVTHWU | FCVTLD | FCVTLH | FCVTLQ | FCVTLS
            | FCVTLUD | FCVTLUH | FCVTLUQ | FCVTLUS | FCVTQD | FCVTQH | FCVTQL | FCVTQLU
            | FCVTQS | FCVTQW | FCVTQWU | FCVTSD | FCVTSH | FCVTSL | FCVTSLU | FCVTSQ | FCVTSW
            | FCVTSWU | FCVTWD | FCVTWH | FCVTWQ | FCVTWS | FCVTWUD | FCVTWUH | FCVTWUQ
            | FCVTWUS | FROUNDD | FROUNDH | FROUNDQ | FROUNDS | FROUNDNXD | FROUNDNXH
            | FROUNDNXQ | FROUNDNXS | FSQRTD | FSQRTH | FSQRTQ | FSQRTS => Encoding::RdRs1Rm,
            AES64KS1I => Encoding::RdRs1Rnum,
            ADD | ADDUW | ADDW | AES64DS | AES64DSM | AES64ES | AES64ESM | AES64KS2 | AND
            | ANDN | BCLR | BEXT | BINV | BSET | CLMUL | CLMULH | CLMULR | CZEROEQZ | CZERONEZ
            | DIV | DIVU | DIVUW | DIVW | FEQD | FEQH | FEQQ | FEQS | FLED | FLEH | FLEQ | FLES
            | FLEQD | FLEQH | FLEQQ | FLEQS | FLTD | FLTH | FLTQ | FLTS | FLTQD | FLTQH | FLTQQ
            | FLTQS | FMAXD | FMAXH | FMAXQ | FMAXS | FMAXMD | FMAXMH | FMAXMQ | FMAXMS | FMIND
            | FMINH | FMINQ | FMINS | FMINMD | FMINMH | FMINMQ | FMINMS | FMVPDX | FMVPQX
            | FSGNJD | FSGNJH | FSGNJQ | FSGNJS | FSGNJND | FSGNJNH | FSGNJNQ | FSGNJNS
            | FSGNJXD | FSGNJXH | FSGNJXQ | FSGNJXS | MAX | MAXU | MIN | MINU | MOPRR0 | MOPRR1
            | MOPRR2 | MOPRR3 | MOPRR4 | MOPRR5 | MOPRR6 | MOPRR7 | MUL | MULH | MULHSU | MULHU
            | MULW | OR | ORN | PACK | PACKH | PACKW | REM | REMU | REMUW | REMW | ROL | ROLW
            | ROR | RORW | SH1ADD | SH1ADDUW | SH2ADD | SH2ADDUW | SH3ADD | SH3ADDUW
            | SHA512SIG0H | SHA512SIG0L | SHA512SIG1H | SHA512SIG1L | SHA512SUM0R | SHA512SUM1R
            | SLL | SLLW | SLT | SLTU | SRA | SRAW | SRL | SRLW | SUB | SUBW | XNOR | XOR
            | XPERM4 | XPERM8 => Encoding::RdRs1Rs2,
            AMOADDB | AMOADDD | AMOADDH | AMOADDW | AMOANDB | AMOANDD | AMOANDH | AMOANDW
            | AMOCASB | AMOCASD | AMOCASH | AMOCASQ | AMOCASW | AMOMAXB | AMOMAXD | AMOMAXH
            | AMOMAXW | AMOMAXUB | AMOMAXUD | AMOMAXUH | AMOMAXUW | AMOMINB | AMOMIND | AMOMINH
            | AMOMINW | AMOMINUB | AMOMINUD | AMOMINUH | AMOMINUW | AMOORB | AMOORD | AMOORH
            | AMOORW | AMOSWAPB | AMOSWAPD | AMOSWAPH | AMOSWAPW | AMOXORB | AMOXORD | AMOXORH
            | AMOXORW | SCD | SCW => Encoding::RdRs1Rs2AqRl,
            AES32DSI | AES32DSMI | AES32ESI | AES32ESMI | SM4ED | SM4KS => Encoding::RdRs1Rs2Bs,
            FABSD | FABSH | FABSQ | FABSS | FMVD | FMVH | FMVQ | FMVS | FNEGD | FNEGH | FNEGQ
            | FNEGS => Encoding::RdRs1Rs2EqRs1,
            FADDD | FADDH | FADDQ | FADDS | FDIVD | FDIVH | FDIVQ | FDIVS | FMULD | FMULH
            | FMULQ | FMULS | FSUBD | FSUBH | FSUBQ | FSUBS => Encoding::RdRs1Rs2Rm,
            FMADDD | FMADDH | FMADDQ | FMADDS | FMSUBD | FMSUBH | FMSUBQ | FMSUBS | FNMADDD
            | FNMADDH | FNMADDQ | FNMADDS | FNMSUBD | FNMSUBH | FNMSUBQ | FNMSUBS => {
                Encoding::RdRs1Rs2Rs3Rm
            }
            BCLRI | BEXTI | BINVI | BSETI | RORI | SLLI | SLLIUW | SRAI | SRLI => {
                Encoding::RdRs1Shamtd
            }
            BCLRIRV32 | BEXTIRV32 | BINVIRV32 | BSETIRV32 | RORIRV32 | RORIW | SLLIRV32 | SLLIW
            | SRAIRV32 | SRAIW | SRLIRV32 | SRLIW => Encoding::RdRs1Shamtw,
            SGTZ | SNEZ => Encoding::RdRs2,
            FSFLAGSI | FSRMI => Encoding::RdZimm,
            CBOCLEAN | CBOFLUSH | CBOINVAL | CBOZERO | JALRPSEUDO | JR => Encoding::Rs1,
            CSRC | CSRS | CSRW => Encoding::Rs1Csr,
            PREFETCHI | PREFETCHR | PREFETCHW => Encoding::Rs1Imm12hi,
            CJR => Encoding::Rs1N0,
            CBEQZ | CBNEZ => Encoding::Rs1PCBimm9loCBimm9hi,
            CFSW | CSW => Encoding::Rs1PRs2PCUimm7loCUimm7hi,
            CSD => Encoding::Rs1PRs2PCUimm8hiCUimm8lo,
            CFSD => Encoding::Rs1PRs2PCUimm8loCUimm8hi,
            FENCETSO => Encoding::Rs1Rd,
            HFENCEGVMA | HFENCEVVMA | HINVALGVMA | HINVALVVMA | HSVB | HSVD | HSVH | HSVW
            | SFENCEVMA | SINVALVMA => Encoding::Rs1Rs2,
            VFMVSF | VFMVVF | VL1RV | VL1RE16V | VL1RE32V | VL1RE64V | VL1RE8V | VL2RV
            | VL2RE16V | VL2RE32V | VL2RE64V | VL2RE8V | VL4RV | VL4RE16V | VL4RE32V | VL4RE64V
            | VL4RE8V | VL8RV | VL8RE16V | VL8RE32V | VL8RE64V | VL8RE8V | VLE1V | VLMV | VMVSX
            | VMVVX => Encoding::Rs1Vd,
            VS1RV | VS2RV | VS4RV | VS8RV | VSE1V | VSMV => Encoding::Rs1Vs3,
            CSH => Encoding::Rs2PRs1PCUimm1,
            CSB => Encoding::Rs2PRs1PCUimm2,
            VSETVL => Encoding::Rs2Rs1Rd,
            VMVVI => Encoding::Simm5Vd,
            VIDV => Encoding::VmVd,
            VCPOPM | VFIRSTM | VPOPCM => Encoding::VmVs2Rd,
            VAADDVX | VAADDUVX | VADDVX | VANDVX | VANDNVX | VASUBVX | VASUBUVX | VCLMULVX
            | VCLMULHVX | VDIVVX | VDIVUVX | VFADDVF | VFDIVVF | VFMACCVF | VFMADDVF | VFMAXVF
            | VFMINVF | VFMSACVF | VFMSUBVF | VFMULVF | VFNMACCVF | VFNMADDVF | VFNMSACVF
            | VFNMSUBVF | VFRDIVVF | VFRSUBVF | VFSGNJVF | VFSGNJNVF | VFSGNJXVF
            | VFSLIDE1DOWNVF | VFSLIDE1UPVF | VFSUBVF | VFWADDVF | VFWADDWF | VFWMACCVF
            | VFWMSACVF | VFWMULVF | VFWNMACCVF | VFWNMSACVF | VFWSUBVF | VFWSUBWF | VMACCVX
            | VMADDVX | VMAXVX | VMAXUVX | VMFEQVF | VMFGEVF | VMFGTVF | VMFLEVF | VMFLTVF
            | VMFNEVF | VMINVX | VMINUVX | VMSEQVX | VMSGTVX | VMSGTUVX | VMSLEVX | VMSLEUVX
            | VMSLTVX | VMSLTUVX | VMSNEVX | VMULVX | VMULHVX | VMULHSUVX | VMULHUVX | VNCLIPWX
            | VNCLIPUWX | VNMSACVX | VNMSUBVX | VNSRAWX | VNSRLWX | VORVX | VREMVX | VREMUVX
            | VRGATHERVX | VROLVX | VRORVX | VRSUBVX | VSADDVX | VSADDUVX | VSLIDE1DOWNVX
            | VSLIDE1UPVX | VSLIDEDOWNVX | VSLIDEUPVX | VSLLVX | VSMULVX | VSRAVX | VSRLVX
            | VSSRAVX | VSSRLVX | VSSUBVX | VSSUBUVX | VSUBVX | VWADDVX | VWADDWX | VWADDUVX
            | VWADDUWX | VWMACCVX | VWMACCSUVX | VWMACCUVX | VWMACCUSVX | VWMULVX | VWMULSUVX
            | VWMULUVX | VWSLLVX | VWSUBVX | VWSUBWX | VWSUBUVX | VWSUBUWX | VXORVX => {
                Encoding::VmVs2Rs1Vd
            }
            VADDVI | VANDVI | VMSEQVI | VMSGTVI | VMSGTUVI | VMSLEVI | VMSLEUVI | VMSNEVI
            | VNCLIPWI | VNCLIPUWI | VNSRAWI | VNSRLWI | VORVI | VRGATHERVI | VRSUBVI | VSADDVI
            | VSADDUVI | VSLIDEDOWNVI | VSLIDEUPVI | VSLLVI | VSRAVI | VSRLVI | VSSRAVI
            | VSSRLVI | VXORVI => Encoding::VmVs2Simm5Vd,
            VBREV8V | VBREVV | VCLZV | VCPOPV | VCTZV | VFCLASSV | VFCVTFXV | VFCVTFXUV
            | VFCVTRTZXFV | VFCVTRTZXUFV | VFCVTXFV | VFCVTXUFV | VFNCVTFFW | VFNCVTFXW
            | VFNCVTFXUW | VFNCVTRODFFW | VFNCVTRTZXFW | VFNCVTRTZXUFW | VFNCVTXFW | VFNCVTXUFW
            | VFREC7V | VFRSQRT7V | VFSQRTV | VFWCVTFFV | VFWCVTFXV | VFWCVTFXUV | VFWCVTRTZXFV
            | VFWCVTRTZXUFV | VFWCVTXFV | VFWCVTXUFV | VIOTAM | VMSBFM | VMSIFM | VMSOFM
            | VREV8V | VSEXTVF2 | VSEXTVF4 | VSEXTVF8 | VZEXTVF2 | VZEXTVF4 | VZEXTVF8 => {
                Encoding::VmVs2Vd
            }
            VAADDVV | VAADDUVV | VADDVV | VANDVV | VANDNVV | VASUBVV | VASUBUVV | VCLMULVV
            | VCLMULHVV | VDIVVV | VDIVUVV | VFADDVV | VFDIVVV | VFMACCVV | VFMADDVV | VFMAXVV
            | VFMINVV | VFMSACVV | VFMSUBVV | VFMULVV | VFNMACCVV | VFNMADDVV | VFNMSACVV
            | VFNMSUBVV | VFREDMAXVS | VFREDMINVS | VFREDOSUMVS | VFREDSUMVS | VFREDUSUMVS
            | VFSGNJVV | VFSGNJNVV | VFSGNJXVV | VFSUBVV | VFWADDVV | VFWADDWV | VFWMACCVV
            | VFWMSACVV | VFWMULVV | VFWNMACCVV | VFWNMSACVV | VFWREDOSUMVS | VFWREDSUMVS
            | VFWREDUSUMVS | VFWSUBVV | VFWSUBWV | VMACCVV | VMADDVV | VMANDNOTMM | VMAXVV
            | VMAXUVV | VMFEQVV | VMFLEVV | VMFLTVV | VMFNEVV | VMINVV | VMINUVV | VMORNOTMM
            | VMSEQVV | VMSLEVV | VMSLEUVV | VMSLTVV | VMSLTUVV | VMSNEVV | VMULVV | VMULHVV
            | VMULHSUVV | VMULHUVV | VNCLIPWV | VNCLIPUWV | VNMSACVV | VNMSUBVV | VNSRAWV
            | VNSRLWV | VORVV | VREDANDVS | VREDMAXVS | VREDMAXUVS | VREDMINVS | VREDMINUVS
            | VREDORVS | VREDSUMVS | VREDXORVS | VREMVV | VREMUVV | VRGATHERVV | VRGATHEREI16VV
            | VROLVV | VRORVV | VSADDVV | VSADDUVV | VSLLVV | VSMULVV | VSRAVV | VSRLVV
            | VSSRAVV | VSSRLVV | VSSUBVV | VSSUBUVV | VSUBVV | VWADDVV | VWADDWV | VWADDUVV
            | VWADDUWV | VWMACCVV | VWMACCSUVV | VWMACCUVV | VWMULVV | VWMULSUVV | VWMULUVV
            | VWREDSUMVS | VWREDSUMUVS | VWSLLVV | VWSUBVV | VWSUBWV | VWSUBUVV | VWSUBUWV
            | VXORVV => Encoding::VmVs2Vs1Vd,
            VWSLLVI => Encoding::VmVs2Zimm5Vd,
            VMVVV => Encoding::Vs1Vd,
            VFMVFS | VMVXS => Encoding::Vs2Rd,
            VADCVXM | VFMERGEVFM | VMADCVX | VMADCVXM | VMERGEVXM | VMSBCVX | VMSBCVXM
            | VSBCVXM => Encoding::Vs2Rs1Vd,
            VADCVIM | VMADCVI | VMADCVIM | VMERGEVIM => Encoding::Vs2Simm5Vd,
            VAESDFVS | VAESDFVV | VAESDMVS | VAESDMVV | VAESEFVS | VAESEFVV | VAESEMVS
            | VAESEMVV | VAESZVS | VGMULVV | VMV1RV | VMV2RV | VMV4RV | VMV8RV | VSM4RVS
            | VSM4RVV => Encoding::Vs2Vd,
            VADCVVM | VCOMPRESSVM | VGHSHVV | VMADCVV | VMADCVVM | VMANDMM | VMANDNMM
            | VMERGEVVM | VMNANDMM | VMNORMM | VMORMM | VMORNMM | VMSBCVV | VMSBCVVM | VMXNORMM
            | VMXORMM | VSBCVVM | VSHA2CHVV | VSHA2CLVV | VSHA2MSVV | VSM3MEVV => {
                Encoding::Vs2Vs1Vd
            }
            VAESKF1VI | VAESKF2VI | VSM3CVI | VSM4KVI => Encoding::Vs2Zimm5Vd,
            VSETIVLI => Encoding::Zimm10ZimmRd,
            VSETVLI => Encoding::Zimm11Rs1Rd,
            VRORVI => Encoding::Zimm6HiVmVs2Zimm6loVd,
        }
    }
}
pub const INSN_FIELD_RD: u32 = 0xf80;
pub const INSN_FIELD_RD_START: u32 = 7;
pub const INSN_FIELD_RD_SIZE: u32 = 5;
pub const INSN_FIELD_RT: u32 = 0xf8000;
pub const INSN_FIELD_RT_START: u32 = 15;
pub const INSN_FIELD_RT_SIZE: u32 = 5;
pub const INSN_FIELD_RS1: u32 = 0xf8000;
pub const INSN_FIELD_RS1_START: u32 = 15;
pub const INSN_FIELD_RS1_SIZE: u32 = 5;
pub const INSN_FIELD_RS2: u32 = 0x1f00000;
pub const INSN_FIELD_RS2_START: u32 = 20;
pub const INSN_FIELD_RS2_SIZE: u32 = 5;
pub const INSN_FIELD_RS3: u32 = 0xf8000000;
pub const INSN_FIELD_RS3_START: u32 = 27;
pub const INSN_FIELD_RS3_SIZE: u32 = 5;
pub const INSN_FIELD_AQRL: u32 = 0x6000000;
pub const INSN_FIELD_AQRL_START: u32 = 25;
pub const INSN_FIELD_AQRL_SIZE: u32 = 2;
pub const INSN_FIELD_AQ: u32 = 0x4000000;
pub const INSN_FIELD_AQ_START: u32 = 26;
pub const INSN_FIELD_AQ_SIZE: u32 = 1;
pub const INSN_FIELD_RL: u32 = 0x2000000;
pub const INSN_FIELD_RL_START: u32 = 25;
pub const INSN_FIELD_RL_SIZE: u32 = 1;
pub const INSN_FIELD_FM: u32 = 0xf0000000;
pub const INSN_FIELD_FM_START: u32 = 28;
pub const INSN_FIELD_FM_SIZE: u32 = 4;
pub const INSN_FIELD_PRED: u32 = 0xf000000;
pub const INSN_FIELD_PRED_START: u32 = 24;
pub const INSN_FIELD_PRED_SIZE: u32 = 4;
pub const INSN_FIELD_SUCC: u32 = 0xf00000;
pub const INSN_FIELD_SUCC_START: u32 = 20;
pub const INSN_FIELD_SUCC_SIZE: u32 = 4;
pub const INSN_FIELD_RM: u32 = 0x7000;
pub const INSN_FIELD_RM_START: u32 = 12;
pub const INSN_FIELD_RM_SIZE: u32 = 3;
pub const INSN_FIELD_FUNCT3: u32 = 0x7000;
pub const INSN_FIELD_FUNCT3_START: u32 = 12;
pub const INSN_FIELD_FUNCT3_SIZE: u32 = 3;
pub const INSN_FIELD_FUNCT2: u32 = 0x6000000;
pub const INSN_FIELD_FUNCT2_START: u32 = 25;
pub const INSN_FIELD_FUNCT2_SIZE: u32 = 2;
pub const INSN_FIELD_IMM20: u32 = 0xfffff000;
pub const INSN_FIELD_IMM20_START: u32 = 12;
pub const INSN_FIELD_IMM20_SIZE: u32 = 20;
pub const INSN_FIELD_JIMM20: u32 = 0xfffff000;
pub const INSN_FIELD_JIMM20_START: u32 = 12;
pub const INSN_FIELD_JIMM20_SIZE: u32 = 20;
pub const INSN_FIELD_IMM12: u32 = 0xfff00000;
pub const INSN_FIELD_IMM12_START: u32 = 20;
pub const INSN_FIELD_IMM12_SIZE: u32 = 12;
pub const INSN_FIELD_CSR: u32 = 0xfff00000;
pub const INSN_FIELD_CSR_START: u32 = 20;
pub const INSN_FIELD_CSR_SIZE: u32 = 12;
pub const INSN_FIELD_IMM12HI: u32 = 0xfe000000;
pub const INSN_FIELD_IMM12HI_START: u32 = 25;
pub const INSN_FIELD_IMM12HI_SIZE: u32 = 7;
pub const INSN_FIELD_BIMM12HI: u32 = 0xfe000000;
pub const INSN_FIELD_BIMM12HI_START: u32 = 25;
pub const INSN_FIELD_BIMM12HI_SIZE: u32 = 7;
pub const INSN_FIELD_IMM12LO: u32 = 0xf80;
pub const INSN_FIELD_IMM12LO_START: u32 = 7;
pub const INSN_FIELD_IMM12LO_SIZE: u32 = 5;
pub const INSN_FIELD_BIMM12LO: u32 = 0xf80;
pub const INSN_FIELD_BIMM12LO_START: u32 = 7;
pub const INSN_FIELD_BIMM12LO_SIZE: u32 = 5;
pub const INSN_FIELD_SHAMTQ: u32 = 0x7f00000;
pub const INSN_FIELD_SHAMTQ_START: u32 = 20;
pub const INSN_FIELD_SHAMTQ_SIZE: u32 = 7;
pub const INSN_FIELD_SHAMTW: u32 = 0x1f00000;
pub const INSN_FIELD_SHAMTW_START: u32 = 20;
pub const INSN_FIELD_SHAMTW_SIZE: u32 = 5;
pub const INSN_FIELD_SHAMTW4: u32 = 0xf00000;
pub const INSN_FIELD_SHAMTW4_START: u32 = 20;
pub const INSN_FIELD_SHAMTW4_SIZE: u32 = 4;
pub const INSN_FIELD_SHAMTD: u32 = 0x3f00000;
pub const INSN_FIELD_SHAMTD_START: u32 = 20;
pub const INSN_FIELD_SHAMTD_SIZE: u32 = 6;
pub const INSN_FIELD_BS: u32 = 0xc0000000;
pub const INSN_FIELD_BS_START: u32 = 30;
pub const INSN_FIELD_BS_SIZE: u32 = 2;
pub const INSN_FIELD_RNUM: u32 = 0xf00000;
pub const INSN_FIELD_RNUM_START: u32 = 20;
pub const INSN_FIELD_RNUM_SIZE: u32 = 4;
pub const INSN_FIELD_RC: u32 = 0x3e000000;
pub const INSN_FIELD_RC_START: u32 = 25;
pub const INSN_FIELD_RC_SIZE: u32 = 5;
pub const INSN_FIELD_IMM2: u32 = 0x300000;
pub const INSN_FIELD_IMM2_START: u32 = 20;
pub const INSN_FIELD_IMM2_SIZE: u32 = 2;
pub const INSN_FIELD_IMM3: u32 = 0x700000;
pub const INSN_FIELD_IMM3_START: u32 = 20;
pub const INSN_FIELD_IMM3_SIZE: u32 = 3;
pub const INSN_FIELD_IMM4: u32 = 0xf00000;
pub const INSN_FIELD_IMM4_START: u32 = 20;
pub const INSN_FIELD_IMM4_SIZE: u32 = 4;
pub const INSN_FIELD_IMM5: u32 = 0x1f00000;
pub const INSN_FIELD_IMM5_START: u32 = 20;
pub const INSN_FIELD_IMM5_SIZE: u32 = 5;
pub const INSN_FIELD_IMM6: u32 = 0x3f00000;
pub const INSN_FIELD_IMM6_START: u32 = 20;
pub const INSN_FIELD_IMM6_SIZE: u32 = 6;
pub const INSN_FIELD_ZIMM: u32 = 0xf8000;
pub const INSN_FIELD_ZIMM_START: u32 = 15;
pub const INSN_FIELD_ZIMM_SIZE: u32 = 5;
pub const INSN_FIELD_OPCODE: u32 = 0x7f;
pub const INSN_FIELD_OPCODE_START: u32 = 0;
pub const INSN_FIELD_OPCODE_SIZE: u32 = 7;
pub const INSN_FIELD_FUNCT7: u32 = 0xfe000000;
pub const INSN_FIELD_FUNCT7_START: u32 = 25;
pub const INSN_FIELD_FUNCT7_SIZE: u32 = 7;
pub const INSN_FIELD_VD: u32 = 0xf80;
pub const INSN_FIELD_VD_START: u32 = 7;
pub const INSN_FIELD_VD_SIZE: u32 = 5;
pub const INSN_FIELD_VS3: u32 = 0xf80;
pub const INSN_FIELD_VS3_START: u32 = 7;
pub const INSN_FIELD_VS3_SIZE: u32 = 5;
pub const INSN_FIELD_VS1: u32 = 0xf8000;
pub const INSN_FIELD_VS1_START: u32 = 15;
pub const INSN_FIELD_VS1_SIZE: u32 = 5;
pub const INSN_FIELD_VS2: u32 = 0x1f00000;
pub const INSN_FIELD_VS2_START: u32 = 20;
pub const INSN_FIELD_VS2_SIZE: u32 = 5;
pub const INSN_FIELD_VM: u32 = 0x2000000;
pub const INSN_FIELD_VM_START: u32 = 25;
pub const INSN_FIELD_VM_SIZE: u32 = 1;
pub const INSN_FIELD_WD: u32 = 0x4000000;
pub const INSN_FIELD_WD_START: u32 = 26;
pub const INSN_FIELD_WD_SIZE: u32 = 1;
pub const INSN_FIELD_AMOOP: u32 = 0xf8000000;
pub const INSN_FIELD_AMOOP_START: u32 = 27;
pub const INSN_FIELD_AMOOP_SIZE: u32 = 5;
pub const INSN_FIELD_NF: u32 = 0xe0000000;
pub const INSN_FIELD_NF_START: u32 = 29;
pub const INSN_FIELD_NF_SIZE: u32 = 3;
pub const INSN_FIELD_SIMM5: u32 = 0xf8000;
pub const INSN_FIELD_SIMM5_START: u32 = 15;
pub const INSN_FIELD_SIMM5_SIZE: u32 = 5;
pub const INSN_FIELD_ZIMM5: u32 = 0xf8000;
pub const INSN_FIELD_ZIMM5_START: u32 = 15;
pub const INSN_FIELD_ZIMM5_SIZE: u32 = 5;
pub const INSN_FIELD_ZIMM10: u32 = 0x3ff00000;
pub const INSN_FIELD_ZIMM10_START: u32 = 20;
pub const INSN_FIELD_ZIMM10_SIZE: u32 = 10;
pub const INSN_FIELD_ZIMM11: u32 = 0x7ff00000;
pub const INSN_FIELD_ZIMM11_START: u32 = 20;
pub const INSN_FIELD_ZIMM11_SIZE: u32 = 11;
pub const INSN_FIELD_ZIMM6HI: u32 = 0x4000000;
pub const INSN_FIELD_ZIMM6HI_START: u32 = 26;
pub const INSN_FIELD_ZIMM6HI_SIZE: u32 = 1;
pub const INSN_FIELD_ZIMM6LO: u32 = 0xf8000;
pub const INSN_FIELD_ZIMM6LO_START: u32 = 15;
pub const INSN_FIELD_ZIMM6LO_SIZE: u32 = 5;
pub const INSN_FIELD_C_NZUIMM10: u32 = 0x1fe0;
pub const INSN_FIELD_C_NZUIMM10_START: u32 = 5;
pub const INSN_FIELD_C_NZUIMM10_SIZE: u32 = 8;
pub const INSN_FIELD_C_UIMM7LO: u32 = 0x60;
pub const INSN_FIELD_C_UIMM7LO_START: u32 = 5;
pub const INSN_FIELD_C_UIMM7LO_SIZE: u32 = 2;
pub const INSN_FIELD_C_UIMM7HI: u32 = 0x1c00;
pub const INSN_FIELD_C_UIMM7HI_START: u32 = 10;
pub const INSN_FIELD_C_UIMM7HI_SIZE: u32 = 3;
pub const INSN_FIELD_C_UIMM8LO: u32 = 0x60;
pub const INSN_FIELD_C_UIMM8LO_START: u32 = 5;
pub const INSN_FIELD_C_UIMM8LO_SIZE: u32 = 2;
pub const INSN_FIELD_C_UIMM8HI: u32 = 0x1c00;
pub const INSN_FIELD_C_UIMM8HI_START: u32 = 10;
pub const INSN_FIELD_C_UIMM8HI_SIZE: u32 = 3;
pub const INSN_FIELD_C_UIMM9LO: u32 = 0x60;
pub const INSN_FIELD_C_UIMM9LO_START: u32 = 5;
pub const INSN_FIELD_C_UIMM9LO_SIZE: u32 = 2;
pub const INSN_FIELD_C_UIMM9HI: u32 = 0x1c00;
pub const INSN_FIELD_C_UIMM9HI_START: u32 = 10;
pub const INSN_FIELD_C_UIMM9HI_SIZE: u32 = 3;
pub const INSN_FIELD_C_NZIMM6LO: u32 = 0x7c;
pub const INSN_FIELD_C_NZIMM6LO_START: u32 = 2;
pub const INSN_FIELD_C_NZIMM6LO_SIZE: u32 = 5;
pub const INSN_FIELD_C_NZIMM6HI: u32 = 0x1000;
pub const INSN_FIELD_C_NZIMM6HI_START: u32 = 12;
pub const INSN_FIELD_C_NZIMM6HI_SIZE: u32 = 1;
pub const INSN_FIELD_C_IMM6LO: u32 = 0x7c;
pub const INSN_FIELD_C_IMM6LO_START: u32 = 2;
pub const INSN_FIELD_C_IMM6LO_SIZE: u32 = 5;
pub const INSN_FIELD_C_IMM6HI: u32 = 0x1000;
pub const INSN_FIELD_C_IMM6HI_START: u32 = 12;
pub const INSN_FIELD_C_IMM6HI_SIZE: u32 = 1;
pub const INSN_FIELD_C_NZIMM10HI: u32 = 0x1000;
pub const INSN_FIELD_C_NZIMM10HI_START: u32 = 12;
pub const INSN_FIELD_C_NZIMM10HI_SIZE: u32 = 1;
pub const INSN_FIELD_C_NZIMM10LO: u32 = 0x7c;
pub const INSN_FIELD_C_NZIMM10LO_START: u32 = 2;
pub const INSN_FIELD_C_NZIMM10LO_SIZE: u32 = 5;
pub const INSN_FIELD_C_NZIMM18HI: u32 = 0x1000;
pub const INSN_FIELD_C_NZIMM18HI_START: u32 = 12;
pub const INSN_FIELD_C_NZIMM18HI_SIZE: u32 = 1;
pub const INSN_FIELD_C_NZIMM18LO: u32 = 0x7c;
pub const INSN_FIELD_C_NZIMM18LO_START: u32 = 2;
pub const INSN_FIELD_C_NZIMM18LO_SIZE: u32 = 5;
pub const INSN_FIELD_C_IMM12: u32 = 0x1ffc;
pub const INSN_FIELD_C_IMM12_START: u32 = 2;
pub const INSN_FIELD_C_IMM12_SIZE: u32 = 11;
pub const INSN_FIELD_C_BIMM9LO: u32 = 0x7c;
pub const INSN_FIELD_C_BIMM9LO_START: u32 = 2;
pub const INSN_FIELD_C_BIMM9LO_SIZE: u32 = 5;
pub const INSN_FIELD_C_BIMM9HI: u32 = 0x1c00;
pub const INSN_FIELD_C_BIMM9HI_START: u32 = 10;
pub const INSN_FIELD_C_BIMM9HI_SIZE: u32 = 3;
pub const INSN_FIELD_C_NZUIMM5: u32 = 0x7c;
pub const INSN_FIELD_C_NZUIMM5_START: u32 = 2;
pub const INSN_FIELD_C_NZUIMM5_SIZE: u32 = 5;
pub const INSN_FIELD_C_NZUIMM6LO: u32 = 0x7c;
pub const INSN_FIELD_C_NZUIMM6LO_START: u32 = 2;
pub const INSN_FIELD_C_NZUIMM6LO_SIZE: u32 = 5;
pub const INSN_FIELD_C_NZUIMM6HI: u32 = 0x1000;
pub const INSN_FIELD_C_NZUIMM6HI_START: u32 = 12;
pub const INSN_FIELD_C_NZUIMM6HI_SIZE: u32 = 1;
pub const INSN_FIELD_C_UIMM8SPLO: u32 = 0x7c;
pub const INSN_FIELD_C_UIMM8SPLO_START: u32 = 2;
pub const INSN_FIELD_C_UIMM8SPLO_SIZE: u32 = 5;
pub const INSN_FIELD_C_UIMM8SPHI: u32 = 0x1000;
pub const INSN_FIELD_C_UIMM8SPHI_START: u32 = 12;
pub const INSN_FIELD_C_UIMM8SPHI_SIZE: u32 = 1;
pub const INSN_FIELD_C_UIMM8SP_S: u32 = 0x1f80;
pub const INSN_FIELD_C_UIMM8SP_S_START: u32 = 7;
pub const INSN_FIELD_C_UIMM8SP_S_SIZE: u32 = 6;
pub const INSN_FIELD_C_UIMM10SPLO: u32 = 0x7c;
pub const INSN_FIELD_C_UIMM10SPLO_START: u32 = 2;
pub const INSN_FIELD_C_UIMM10SPLO_SIZE: u32 = 5;
pub const INSN_FIELD_C_UIMM10SPHI: u32 = 0x1000;
pub const INSN_FIELD_C_UIMM10SPHI_START: u32 = 12;
pub const INSN_FIELD_C_UIMM10SPHI_SIZE: u32 = 1;
pub const INSN_FIELD_C_UIMM9SPLO: u32 = 0x7c;
pub const INSN_FIELD_C_UIMM9SPLO_START: u32 = 2;
pub const INSN_FIELD_C_UIMM9SPLO_SIZE: u32 = 5;
pub const INSN_FIELD_C_UIMM9SPHI: u32 = 0x1000;
pub const INSN_FIELD_C_UIMM9SPHI_START: u32 = 12;
pub const INSN_FIELD_C_UIMM9SPHI_SIZE: u32 = 1;
pub const INSN_FIELD_C_UIMM10SP_S: u32 = 0x1f80;
pub const INSN_FIELD_C_UIMM10SP_S_START: u32 = 7;
pub const INSN_FIELD_C_UIMM10SP_S_SIZE: u32 = 6;
pub const INSN_FIELD_C_UIMM9SP_S: u32 = 0x1f80;
pub const INSN_FIELD_C_UIMM9SP_S_START: u32 = 7;
pub const INSN_FIELD_C_UIMM9SP_S_SIZE: u32 = 6;
pub const INSN_FIELD_C_UIMM2: u32 = 0x60;
pub const INSN_FIELD_C_UIMM2_START: u32 = 5;
pub const INSN_FIELD_C_UIMM2_SIZE: u32 = 2;
pub const INSN_FIELD_C_UIMM1: u32 = 0x20;
pub const INSN_FIELD_C_UIMM1_START: u32 = 5;
pub const INSN_FIELD_C_UIMM1_SIZE: u32 = 1;
pub const INSN_FIELD_C_RLIST: u32 = 0xf0;
pub const INSN_FIELD_C_RLIST_START: u32 = 4;
pub const INSN_FIELD_C_RLIST_SIZE: u32 = 4;
pub const INSN_FIELD_C_SPIMM: u32 = 0xc;
pub const INSN_FIELD_C_SPIMM_START: u32 = 2;
pub const INSN_FIELD_C_SPIMM_SIZE: u32 = 2;
pub const INSN_FIELD_C_INDEX: u32 = 0x3fc;
pub const INSN_FIELD_C_INDEX_START: u32 = 2;
pub const INSN_FIELD_C_INDEX_SIZE: u32 = 8;
pub const INSN_FIELD_RS1_P: u32 = 0x380;
pub const INSN_FIELD_RS1_P_START: u32 = 7;
pub const INSN_FIELD_RS1_P_SIZE: u32 = 3;
pub const INSN_FIELD_RS2_P: u32 = 0x1c;
pub const INSN_FIELD_RS2_P_START: u32 = 2;
pub const INSN_FIELD_RS2_P_SIZE: u32 = 3;
pub const INSN_FIELD_RD_P: u32 = 0x1c;
pub const INSN_FIELD_RD_P_START: u32 = 2;
pub const INSN_FIELD_RD_P_SIZE: u32 = 3;
pub const INSN_FIELD_RD_RS1_N0: u32 = 0xf80;
pub const INSN_FIELD_RD_RS1_N0_START: u32 = 7;
pub const INSN_FIELD_RD_RS1_N0_SIZE: u32 = 5;
pub const INSN_FIELD_RD_RS1_P: u32 = 0x380;
pub const INSN_FIELD_RD_RS1_P_START: u32 = 7;
pub const INSN_FIELD_RD_RS1_P_SIZE: u32 = 3;
pub const INSN_FIELD_RD_RS1: u32 = 0xf80;
pub const INSN_FIELD_RD_RS1_START: u32 = 7;
pub const INSN_FIELD_RD_RS1_SIZE: u32 = 5;
pub const INSN_FIELD_RD_N2: u32 = 0xf80;
pub const INSN_FIELD_RD_N2_START: u32 = 7;
pub const INSN_FIELD_RD_N2_SIZE: u32 = 5;
pub const INSN_FIELD_RD_N0: u32 = 0xf80;
pub const INSN_FIELD_RD_N0_START: u32 = 7;
pub const INSN_FIELD_RD_N0_SIZE: u32 = 5;
pub const INSN_FIELD_RS1_N0: u32 = 0xf80;
pub const INSN_FIELD_RS1_N0_START: u32 = 7;
pub const INSN_FIELD_RS1_N0_SIZE: u32 = 5;
pub const INSN_FIELD_C_RS2_N0: u32 = 0x7c;
pub const INSN_FIELD_C_RS2_N0_START: u32 = 2;
pub const INSN_FIELD_C_RS2_N0_SIZE: u32 = 5;
pub const INSN_FIELD_C_RS1_N0: u32 = 0xf80;
pub const INSN_FIELD_C_RS1_N0_START: u32 = 7;
pub const INSN_FIELD_C_RS1_N0_SIZE: u32 = 5;
pub const INSN_FIELD_C_RS2: u32 = 0x7c;
pub const INSN_FIELD_C_RS2_START: u32 = 2;
pub const INSN_FIELD_C_RS2_SIZE: u32 = 5;
pub const INSN_FIELD_C_SREG1: u32 = 0x380;
pub const INSN_FIELD_C_SREG1_START: u32 = 7;
pub const INSN_FIELD_C_SREG1_SIZE: u32 = 3;
pub const INSN_FIELD_C_SREG2: u32 = 0x1c;
pub const INSN_FIELD_C_SREG2_START: u32 = 2;
pub const INSN_FIELD_C_SREG2_SIZE: u32 = 3;
pub const INSN_FIELD_MOP_R_T_30: u32 = 0x40000000;
pub const INSN_FIELD_MOP_R_T_30_START: u32 = 30;
pub const INSN_FIELD_MOP_R_T_30_SIZE: u32 = 1;
pub const INSN_FIELD_MOP_R_T_27_26: u32 = 0xc000000;
pub const INSN_FIELD_MOP_R_T_27_26_START: u32 = 26;
pub const INSN_FIELD_MOP_R_T_27_26_SIZE: u32 = 2;
pub const INSN_FIELD_MOP_R_T_21_20: u32 = 0x300000;
pub const INSN_FIELD_MOP_R_T_21_20_START: u32 = 20;
pub const INSN_FIELD_MOP_R_T_21_20_SIZE: u32 = 2;
pub const INSN_FIELD_MOP_RR_T_30: u32 = 0x40000000;
pub const INSN_FIELD_MOP_RR_T_30_START: u32 = 30;
pub const INSN_FIELD_MOP_RR_T_30_SIZE: u32 = 1;
pub const INSN_FIELD_MOP_RR_T_27_26: u32 = 0xc000000;
pub const INSN_FIELD_MOP_RR_T_27_26_START: u32 = 26;
pub const INSN_FIELD_MOP_RR_T_27_26_SIZE: u32 = 2;
pub const INSN_FIELD_C_MOP_T: u32 = 0x700;
pub const INSN_FIELD_C_MOP_T_START: u32 = 8;
pub const INSN_FIELD_C_MOP_T_SIZE: u32 = 3;
pub const INSN_FIELD_RS2_EQ_RS1: u32 = 0x1f00000;
pub const INSN_FIELD_RS2_EQ_RS1_START: u32 = 20;
pub const INSN_FIELD_RS2_EQ_RS1_SIZE: u32 = 5;

/// InstructionValue contains the 32-bit instruction value and also provides access into the desired field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InstructionValue {
    pub value: u32,
}

impl InstructionValue {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    pub const fn field<const FIELD_START: usize, const FIELD_SIZE: usize>(self) -> u32 {
        (self.value >> FIELD_START) & ((1 << FIELD_SIZE) - 1)
    }

    pub const fn rd(self) -> u32 {
        (self.value >> INSN_FIELD_RD_START) & ((1 << INSN_FIELD_RD_SIZE) - 1)
    }

    pub const fn set_rd(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RD;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RD_SIZE) - 1)) << INSN_FIELD_RD_START;
        self
    }
    pub const fn rt(self) -> u32 {
        (self.value >> INSN_FIELD_RT_START) & ((1 << INSN_FIELD_RT_SIZE) - 1)
    }

    pub const fn set_rt(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RT;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RT_SIZE) - 1)) << INSN_FIELD_RT_START;
        self
    }
    pub const fn rs1(self) -> u32 {
        (self.value >> INSN_FIELD_RS1_START) & ((1 << INSN_FIELD_RS1_SIZE) - 1)
    }

    pub const fn set_rs1(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RS1;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RS1_SIZE) - 1)) << INSN_FIELD_RS1_START;
        self
    }
    pub const fn rs2(self) -> u32 {
        (self.value >> INSN_FIELD_RS2_START) & ((1 << INSN_FIELD_RS2_SIZE) - 1)
    }

    pub const fn set_rs2(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RS2;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RS2_SIZE) - 1)) << INSN_FIELD_RS2_START;
        self
    }
    pub const fn rs3(self) -> u32 {
        (self.value >> INSN_FIELD_RS3_START) & ((1 << INSN_FIELD_RS3_SIZE) - 1)
    }

    pub const fn set_rs3(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RS3;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RS3_SIZE) - 1)) << INSN_FIELD_RS3_START;
        self
    }
    pub const fn aqrl(self) -> u32 {
        (self.value >> INSN_FIELD_AQRL_START) & ((1 << INSN_FIELD_AQRL_SIZE) - 1)
    }

    pub const fn set_aqrl(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_AQRL;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_AQRL_SIZE) - 1)) << INSN_FIELD_AQRL_START;
        self
    }
    pub const fn aq(self) -> u32 {
        (self.value >> INSN_FIELD_AQ_START) & ((1 << INSN_FIELD_AQ_SIZE) - 1)
    }

    pub const fn set_aq(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_AQ;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_AQ_SIZE) - 1)) << INSN_FIELD_AQ_START;
        self
    }
    pub const fn rl(self) -> u32 {
        (self.value >> INSN_FIELD_RL_START) & ((1 << INSN_FIELD_RL_SIZE) - 1)
    }

    pub const fn set_rl(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RL;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RL_SIZE) - 1)) << INSN_FIELD_RL_START;
        self
    }
    pub const fn fm(self) -> u32 {
        (self.value >> INSN_FIELD_FM_START) & ((1 << INSN_FIELD_FM_SIZE) - 1)
    }

    pub const fn set_fm(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_FM;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_FM_SIZE) - 1)) << INSN_FIELD_FM_START;
        self
    }
    pub const fn pred(self) -> u32 {
        (self.value >> INSN_FIELD_PRED_START) & ((1 << INSN_FIELD_PRED_SIZE) - 1)
    }

    pub const fn set_pred(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_PRED;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_PRED_SIZE) - 1)) << INSN_FIELD_PRED_START;
        self
    }
    pub const fn succ(self) -> u32 {
        (self.value >> INSN_FIELD_SUCC_START) & ((1 << INSN_FIELD_SUCC_SIZE) - 1)
    }

    pub const fn set_succ(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_SUCC;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_SUCC_SIZE) - 1)) << INSN_FIELD_SUCC_START;
        self
    }
    pub const fn rm(self) -> u32 {
        (self.value >> INSN_FIELD_RM_START) & ((1 << INSN_FIELD_RM_SIZE) - 1)
    }

    pub const fn set_rm(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RM;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RM_SIZE) - 1)) << INSN_FIELD_RM_START;
        self
    }
    pub const fn funct3(self) -> u32 {
        (self.value >> INSN_FIELD_FUNCT3_START) & ((1 << INSN_FIELD_FUNCT3_SIZE) - 1)
    }

    pub const fn set_funct3(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_FUNCT3;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_FUNCT3_SIZE) - 1)) << INSN_FIELD_FUNCT3_START;
        self
    }
    pub const fn funct2(self) -> u32 {
        (self.value >> INSN_FIELD_FUNCT2_START) & ((1 << INSN_FIELD_FUNCT2_SIZE) - 1)
    }

    pub const fn set_funct2(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_FUNCT2;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_FUNCT2_SIZE) - 1)) << INSN_FIELD_FUNCT2_START;
        self
    }
    pub const fn imm20_raw(self) -> u32 {
        (self.value >> INSN_FIELD_IMM20_START) & ((1 << INSN_FIELD_IMM20_SIZE) - 1)
    }

    pub const fn set_imm20_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_IMM20;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_IMM20_SIZE) - 1)) << INSN_FIELD_IMM20_START;
        self
    }
    pub const fn jimm20_raw(self) -> u32 {
        (self.value >> INSN_FIELD_JIMM20_START) & ((1 << INSN_FIELD_JIMM20_SIZE) - 1)
    }

    pub const fn set_jimm20_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_JIMM20;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_JIMM20_SIZE) - 1)) << INSN_FIELD_JIMM20_START;
        self
    }
    pub const fn imm12_raw(self) -> u32 {
        (self.value >> INSN_FIELD_IMM12_START) & ((1 << INSN_FIELD_IMM12_SIZE) - 1)
    }

    pub const fn set_imm12_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_IMM12;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_IMM12_SIZE) - 1)) << INSN_FIELD_IMM12_START;
        self
    }
    pub const fn csr(self) -> u32 {
        (self.value >> INSN_FIELD_CSR_START) & ((1 << INSN_FIELD_CSR_SIZE) - 1)
    }

    pub const fn set_csr(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_CSR;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_CSR_SIZE) - 1)) << INSN_FIELD_CSR_START;
        self
    }
    pub const fn imm12hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_IMM12HI_START) & ((1 << INSN_FIELD_IMM12HI_SIZE) - 1)
    }

    pub const fn set_imm12hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_IMM12HI;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_IMM12HI_SIZE) - 1)) << INSN_FIELD_IMM12HI_START;
        self
    }
    pub const fn bimm12hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_BIMM12HI_START) & ((1 << INSN_FIELD_BIMM12HI_SIZE) - 1)
    }

    pub const fn set_bimm12hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_BIMM12HI;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_BIMM12HI_SIZE) - 1)) << INSN_FIELD_BIMM12HI_START;
        self
    }
    pub const fn imm12lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_IMM12LO_START) & ((1 << INSN_FIELD_IMM12LO_SIZE) - 1)
    }

    pub const fn set_imm12lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_IMM12LO;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_IMM12LO_SIZE) - 1)) << INSN_FIELD_IMM12LO_START;
        self
    }
    pub const fn bimm12lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_BIMM12LO_START) & ((1 << INSN_FIELD_BIMM12LO_SIZE) - 1)
    }

    pub const fn set_bimm12lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_BIMM12LO;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_BIMM12LO_SIZE) - 1)) << INSN_FIELD_BIMM12LO_START;
        self
    }
    pub const fn shamtq(self) -> u32 {
        (self.value >> INSN_FIELD_SHAMTQ_START) & ((1 << INSN_FIELD_SHAMTQ_SIZE) - 1)
    }

    pub const fn set_shamtq(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_SHAMTQ;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_SHAMTQ_SIZE) - 1)) << INSN_FIELD_SHAMTQ_START;
        self
    }
    pub const fn shamtw(self) -> u32 {
        (self.value >> INSN_FIELD_SHAMTW_START) & ((1 << INSN_FIELD_SHAMTW_SIZE) - 1)
    }

    pub const fn set_shamtw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_SHAMTW;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_SHAMTW_SIZE) - 1)) << INSN_FIELD_SHAMTW_START;
        self
    }
    pub const fn shamtw4(self) -> u32 {
        (self.value >> INSN_FIELD_SHAMTW4_START) & ((1 << INSN_FIELD_SHAMTW4_SIZE) - 1)
    }

    pub const fn set_shamtw4(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_SHAMTW4;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_SHAMTW4_SIZE) - 1)) << INSN_FIELD_SHAMTW4_START;
        self
    }
    pub const fn shamtd(self) -> u32 {
        (self.value >> INSN_FIELD_SHAMTD_START) & ((1 << INSN_FIELD_SHAMTD_SIZE) - 1)
    }

    pub const fn set_shamtd(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_SHAMTD;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_SHAMTD_SIZE) - 1)) << INSN_FIELD_SHAMTD_START;
        self
    }
    pub const fn bs(self) -> u32 {
        (self.value >> INSN_FIELD_BS_START) & ((1 << INSN_FIELD_BS_SIZE) - 1)
    }

    pub const fn set_bs(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_BS;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_BS_SIZE) - 1)) << INSN_FIELD_BS_START;
        self
    }
    pub const fn rnum(self) -> u32 {
        (self.value >> INSN_FIELD_RNUM_START) & ((1 << INSN_FIELD_RNUM_SIZE) - 1)
    }

    pub const fn set_rnum(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RNUM;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RNUM_SIZE) - 1)) << INSN_FIELD_RNUM_START;
        self
    }
    pub const fn rc(self) -> u32 {
        (self.value >> INSN_FIELD_RC_START) & ((1 << INSN_FIELD_RC_SIZE) - 1)
    }

    pub const fn set_rc(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RC;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RC_SIZE) - 1)) << INSN_FIELD_RC_START;
        self
    }
    pub const fn imm2_raw(self) -> u32 {
        (self.value >> INSN_FIELD_IMM2_START) & ((1 << INSN_FIELD_IMM2_SIZE) - 1)
    }

    pub const fn set_imm2_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_IMM2;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_IMM2_SIZE) - 1)) << INSN_FIELD_IMM2_START;
        self
    }
    pub const fn imm3_raw(self) -> u32 {
        (self.value >> INSN_FIELD_IMM3_START) & ((1 << INSN_FIELD_IMM3_SIZE) - 1)
    }

    pub const fn set_imm3_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_IMM3;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_IMM3_SIZE) - 1)) << INSN_FIELD_IMM3_START;
        self
    }
    pub const fn imm4_raw(self) -> u32 {
        (self.value >> INSN_FIELD_IMM4_START) & ((1 << INSN_FIELD_IMM4_SIZE) - 1)
    }

    pub const fn set_imm4_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_IMM4;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_IMM4_SIZE) - 1)) << INSN_FIELD_IMM4_START;
        self
    }
    pub const fn imm5_raw(self) -> u32 {
        (self.value >> INSN_FIELD_IMM5_START) & ((1 << INSN_FIELD_IMM5_SIZE) - 1)
    }

    pub const fn set_imm5_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_IMM5;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_IMM5_SIZE) - 1)) << INSN_FIELD_IMM5_START;
        self
    }
    pub const fn imm6_raw(self) -> u32 {
        (self.value >> INSN_FIELD_IMM6_START) & ((1 << INSN_FIELD_IMM6_SIZE) - 1)
    }

    pub const fn set_imm6_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_IMM6;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_IMM6_SIZE) - 1)) << INSN_FIELD_IMM6_START;
        self
    }
    pub const fn zimm_raw(self) -> u32 {
        (self.value >> INSN_FIELD_ZIMM_START) & ((1 << INSN_FIELD_ZIMM_SIZE) - 1)
    }

    pub const fn set_zimm_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_ZIMM;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_ZIMM_SIZE) - 1)) << INSN_FIELD_ZIMM_START;
        self
    }
    pub const fn opcode(self) -> u32 {
        (self.value >> INSN_FIELD_OPCODE_START) & ((1 << INSN_FIELD_OPCODE_SIZE) - 1)
    }

    pub const fn set_opcode(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_OPCODE;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_OPCODE_SIZE) - 1)) << INSN_FIELD_OPCODE_START;
        self
    }
    pub const fn funct7(self) -> u32 {
        (self.value >> INSN_FIELD_FUNCT7_START) & ((1 << INSN_FIELD_FUNCT7_SIZE) - 1)
    }

    pub const fn set_funct7(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_FUNCT7;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_FUNCT7_SIZE) - 1)) << INSN_FIELD_FUNCT7_START;
        self
    }
    pub const fn vd(self) -> u32 {
        (self.value >> INSN_FIELD_VD_START) & ((1 << INSN_FIELD_VD_SIZE) - 1)
    }

    pub const fn set_vd(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_VD;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_VD_SIZE) - 1)) << INSN_FIELD_VD_START;
        self
    }
    pub const fn vs3(self) -> u32 {
        (self.value >> INSN_FIELD_VS3_START) & ((1 << INSN_FIELD_VS3_SIZE) - 1)
    }

    pub const fn set_vs3(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_VS3;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_VS3_SIZE) - 1)) << INSN_FIELD_VS3_START;
        self
    }
    pub const fn vs1(self) -> u32 {
        (self.value >> INSN_FIELD_VS1_START) & ((1 << INSN_FIELD_VS1_SIZE) - 1)
    }

    pub const fn set_vs1(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_VS1;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_VS1_SIZE) - 1)) << INSN_FIELD_VS1_START;
        self
    }
    pub const fn vs2(self) -> u32 {
        (self.value >> INSN_FIELD_VS2_START) & ((1 << INSN_FIELD_VS2_SIZE) - 1)
    }

    pub const fn set_vs2(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_VS2;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_VS2_SIZE) - 1)) << INSN_FIELD_VS2_START;
        self
    }
    pub const fn vm(self) -> u32 {
        (self.value >> INSN_FIELD_VM_START) & ((1 << INSN_FIELD_VM_SIZE) - 1)
    }

    pub const fn set_vm(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_VM;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_VM_SIZE) - 1)) << INSN_FIELD_VM_START;
        self
    }
    pub const fn wd(self) -> u32 {
        (self.value >> INSN_FIELD_WD_START) & ((1 << INSN_FIELD_WD_SIZE) - 1)
    }

    pub const fn set_wd(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_WD;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_WD_SIZE) - 1)) << INSN_FIELD_WD_START;
        self
    }
    pub const fn amoop(self) -> u32 {
        (self.value >> INSN_FIELD_AMOOP_START) & ((1 << INSN_FIELD_AMOOP_SIZE) - 1)
    }

    pub const fn set_amoop(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_AMOOP;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_AMOOP_SIZE) - 1)) << INSN_FIELD_AMOOP_START;
        self
    }
    pub const fn nf(self) -> u32 {
        (self.value >> INSN_FIELD_NF_START) & ((1 << INSN_FIELD_NF_SIZE) - 1)
    }

    pub const fn set_nf(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_NF;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_NF_SIZE) - 1)) << INSN_FIELD_NF_START;
        self
    }
    pub const fn simm5_raw(self) -> u32 {
        (self.value >> INSN_FIELD_SIMM5_START) & ((1 << INSN_FIELD_SIMM5_SIZE) - 1)
    }

    pub const fn set_simm5_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_SIMM5;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_SIMM5_SIZE) - 1)) << INSN_FIELD_SIMM5_START;
        self
    }
    pub const fn zimm5_raw(self) -> u32 {
        (self.value >> INSN_FIELD_ZIMM5_START) & ((1 << INSN_FIELD_ZIMM5_SIZE) - 1)
    }

    pub const fn set_zimm5_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_ZIMM5;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_ZIMM5_SIZE) - 1)) << INSN_FIELD_ZIMM5_START;
        self
    }
    pub const fn zimm10_raw(self) -> u32 {
        (self.value >> INSN_FIELD_ZIMM10_START) & ((1 << INSN_FIELD_ZIMM10_SIZE) - 1)
    }

    pub const fn set_zimm10_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_ZIMM10;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_ZIMM10_SIZE) - 1)) << INSN_FIELD_ZIMM10_START;
        self
    }
    pub const fn zimm11_raw(self) -> u32 {
        (self.value >> INSN_FIELD_ZIMM11_START) & ((1 << INSN_FIELD_ZIMM11_SIZE) - 1)
    }

    pub const fn set_zimm11_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_ZIMM11;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_ZIMM11_SIZE) - 1)) << INSN_FIELD_ZIMM11_START;
        self
    }
    pub const fn zimm6hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_ZIMM6HI_START) & ((1 << INSN_FIELD_ZIMM6HI_SIZE) - 1)
    }

    pub const fn set_zimm6hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_ZIMM6HI;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_ZIMM6HI_SIZE) - 1)) << INSN_FIELD_ZIMM6HI_START;
        self
    }
    pub const fn zimm6lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_ZIMM6LO_START) & ((1 << INSN_FIELD_ZIMM6LO_SIZE) - 1)
    }

    pub const fn set_zimm6lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_ZIMM6LO;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_ZIMM6LO_SIZE) - 1)) << INSN_FIELD_ZIMM6LO_START;
        self
    }
    pub const fn c_nzuimm10_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_NZUIMM10_START) & ((1 << INSN_FIELD_C_NZUIMM10_SIZE) - 1)
    }

    pub const fn set_c_nzuimm10_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_NZUIMM10;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_NZUIMM10_SIZE) - 1)) << INSN_FIELD_C_NZUIMM10_START;
        self
    }
    pub const fn c_uimm7lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM7LO_START) & ((1 << INSN_FIELD_C_UIMM7LO_SIZE) - 1)
    }

    pub const fn set_c_uimm7lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM7LO;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM7LO_SIZE) - 1)) << INSN_FIELD_C_UIMM7LO_START;
        self
    }
    pub const fn c_uimm7hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM7HI_START) & ((1 << INSN_FIELD_C_UIMM7HI_SIZE) - 1)
    }

    pub const fn set_c_uimm7hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM7HI;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM7HI_SIZE) - 1)) << INSN_FIELD_C_UIMM7HI_START;
        self
    }
    pub const fn c_uimm8lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM8LO_START) & ((1 << INSN_FIELD_C_UIMM8LO_SIZE) - 1)
    }

    pub const fn set_c_uimm8lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM8LO;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM8LO_SIZE) - 1)) << INSN_FIELD_C_UIMM8LO_START;
        self
    }
    pub const fn c_uimm8hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM8HI_START) & ((1 << INSN_FIELD_C_UIMM8HI_SIZE) - 1)
    }

    pub const fn set_c_uimm8hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM8HI;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM8HI_SIZE) - 1)) << INSN_FIELD_C_UIMM8HI_START;
        self
    }
    pub const fn c_uimm9lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM9LO_START) & ((1 << INSN_FIELD_C_UIMM9LO_SIZE) - 1)
    }

    pub const fn set_c_uimm9lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM9LO;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM9LO_SIZE) - 1)) << INSN_FIELD_C_UIMM9LO_START;
        self
    }
    pub const fn c_uimm9hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM9HI_START) & ((1 << INSN_FIELD_C_UIMM9HI_SIZE) - 1)
    }

    pub const fn set_c_uimm9hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM9HI;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM9HI_SIZE) - 1)) << INSN_FIELD_C_UIMM9HI_START;
        self
    }
    pub const fn c_nzimm6lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_NZIMM6LO_START) & ((1 << INSN_FIELD_C_NZIMM6LO_SIZE) - 1)
    }

    pub const fn set_c_nzimm6lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_NZIMM6LO;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_NZIMM6LO_SIZE) - 1)) << INSN_FIELD_C_NZIMM6LO_START;
        self
    }
    pub const fn c_nzimm6hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_NZIMM6HI_START) & ((1 << INSN_FIELD_C_NZIMM6HI_SIZE) - 1)
    }

    pub const fn set_c_nzimm6hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_NZIMM6HI;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_NZIMM6HI_SIZE) - 1)) << INSN_FIELD_C_NZIMM6HI_START;
        self
    }
    pub const fn c_imm6lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_IMM6LO_START) & ((1 << INSN_FIELD_C_IMM6LO_SIZE) - 1)
    }

    pub const fn set_c_imm6lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_IMM6LO;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_IMM6LO_SIZE) - 1)) << INSN_FIELD_C_IMM6LO_START;
        self
    }
    pub const fn c_imm6hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_IMM6HI_START) & ((1 << INSN_FIELD_C_IMM6HI_SIZE) - 1)
    }

    pub const fn set_c_imm6hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_IMM6HI;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_IMM6HI_SIZE) - 1)) << INSN_FIELD_C_IMM6HI_START;
        self
    }
    pub const fn c_nzimm10hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_NZIMM10HI_START) & ((1 << INSN_FIELD_C_NZIMM10HI_SIZE) - 1)
    }

    pub const fn set_c_nzimm10hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_NZIMM10HI;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_NZIMM10HI_SIZE) - 1)) << INSN_FIELD_C_NZIMM10HI_START;
        self
    }
    pub const fn c_nzimm10lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_NZIMM10LO_START) & ((1 << INSN_FIELD_C_NZIMM10LO_SIZE) - 1)
    }

    pub const fn set_c_nzimm10lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_NZIMM10LO;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_NZIMM10LO_SIZE) - 1)) << INSN_FIELD_C_NZIMM10LO_START;
        self
    }
    pub const fn c_nzimm18hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_NZIMM18HI_START) & ((1 << INSN_FIELD_C_NZIMM18HI_SIZE) - 1)
    }

    pub const fn set_c_nzimm18hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_NZIMM18HI;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_NZIMM18HI_SIZE) - 1)) << INSN_FIELD_C_NZIMM18HI_START;
        self
    }
    pub const fn c_nzimm18lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_NZIMM18LO_START) & ((1 << INSN_FIELD_C_NZIMM18LO_SIZE) - 1)
    }

    pub const fn set_c_nzimm18lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_NZIMM18LO;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_NZIMM18LO_SIZE) - 1)) << INSN_FIELD_C_NZIMM18LO_START;
        self
    }
    pub const fn c_imm12_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_IMM12_START) & ((1 << INSN_FIELD_C_IMM12_SIZE) - 1)
    }

    pub const fn set_c_imm12_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_IMM12;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_IMM12_SIZE) - 1)) << INSN_FIELD_C_IMM12_START;
        self
    }
    pub const fn c_bimm9lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_BIMM9LO_START) & ((1 << INSN_FIELD_C_BIMM9LO_SIZE) - 1)
    }

    pub const fn set_c_bimm9lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_BIMM9LO;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_BIMM9LO_SIZE) - 1)) << INSN_FIELD_C_BIMM9LO_START;
        self
    }
    pub const fn c_bimm9hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_BIMM9HI_START) & ((1 << INSN_FIELD_C_BIMM9HI_SIZE) - 1)
    }

    pub const fn set_c_bimm9hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_BIMM9HI;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_BIMM9HI_SIZE) - 1)) << INSN_FIELD_C_BIMM9HI_START;
        self
    }
    pub const fn c_nzuimm5_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_NZUIMM5_START) & ((1 << INSN_FIELD_C_NZUIMM5_SIZE) - 1)
    }

    pub const fn set_c_nzuimm5_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_NZUIMM5;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_NZUIMM5_SIZE) - 1)) << INSN_FIELD_C_NZUIMM5_START;
        self
    }
    pub const fn c_nzuimm6lo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_NZUIMM6LO_START) & ((1 << INSN_FIELD_C_NZUIMM6LO_SIZE) - 1)
    }

    pub const fn set_c_nzuimm6lo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_NZUIMM6LO;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_NZUIMM6LO_SIZE) - 1)) << INSN_FIELD_C_NZUIMM6LO_START;
        self
    }
    pub const fn c_nzuimm6hi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_NZUIMM6HI_START) & ((1 << INSN_FIELD_C_NZUIMM6HI_SIZE) - 1)
    }

    pub const fn set_c_nzuimm6hi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_NZUIMM6HI;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_NZUIMM6HI_SIZE) - 1)) << INSN_FIELD_C_NZUIMM6HI_START;
        self
    }
    pub const fn c_uimm8splo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM8SPLO_START) & ((1 << INSN_FIELD_C_UIMM8SPLO_SIZE) - 1)
    }

    pub const fn set_c_uimm8splo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM8SPLO;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM8SPLO_SIZE) - 1)) << INSN_FIELD_C_UIMM8SPLO_START;
        self
    }
    pub const fn c_uimm8sphi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM8SPHI_START) & ((1 << INSN_FIELD_C_UIMM8SPHI_SIZE) - 1)
    }

    pub const fn set_c_uimm8sphi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM8SPHI;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM8SPHI_SIZE) - 1)) << INSN_FIELD_C_UIMM8SPHI_START;
        self
    }
    pub const fn c_uimm8sp_s_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM8SP_S_START) & ((1 << INSN_FIELD_C_UIMM8SP_S_SIZE) - 1)
    }

    pub const fn set_c_uimm8sp_s_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM8SP_S;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM8SP_S_SIZE) - 1)) << INSN_FIELD_C_UIMM8SP_S_START;
        self
    }
    pub const fn c_uimm10splo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM10SPLO_START) & ((1 << INSN_FIELD_C_UIMM10SPLO_SIZE) - 1)
    }

    pub const fn set_c_uimm10splo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM10SPLO;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM10SPLO_SIZE) - 1)) << INSN_FIELD_C_UIMM10SPLO_START;
        self
    }
    pub const fn c_uimm10sphi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM10SPHI_START) & ((1 << INSN_FIELD_C_UIMM10SPHI_SIZE) - 1)
    }

    pub const fn set_c_uimm10sphi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM10SPHI;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM10SPHI_SIZE) - 1)) << INSN_FIELD_C_UIMM10SPHI_START;
        self
    }
    pub const fn c_uimm9splo_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM9SPLO_START) & ((1 << INSN_FIELD_C_UIMM9SPLO_SIZE) - 1)
    }

    pub const fn set_c_uimm9splo_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM9SPLO;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM9SPLO_SIZE) - 1)) << INSN_FIELD_C_UIMM9SPLO_START;
        self
    }
    pub const fn c_uimm9sphi_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM9SPHI_START) & ((1 << INSN_FIELD_C_UIMM9SPHI_SIZE) - 1)
    }

    pub const fn set_c_uimm9sphi_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM9SPHI;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM9SPHI_SIZE) - 1)) << INSN_FIELD_C_UIMM9SPHI_START;
        self
    }
    pub const fn c_uimm10sp_s_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM10SP_S_START) & ((1 << INSN_FIELD_C_UIMM10SP_S_SIZE) - 1)
    }

    pub const fn set_c_uimm10sp_s_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM10SP_S;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM10SP_S_SIZE) - 1)) << INSN_FIELD_C_UIMM10SP_S_START;
        self
    }
    pub const fn c_uimm9sp_s_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM9SP_S_START) & ((1 << INSN_FIELD_C_UIMM9SP_S_SIZE) - 1)
    }

    pub const fn set_c_uimm9sp_s_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM9SP_S;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_C_UIMM9SP_S_SIZE) - 1)) << INSN_FIELD_C_UIMM9SP_S_START;
        self
    }
    pub const fn c_uimm2_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM2_START) & ((1 << INSN_FIELD_C_UIMM2_SIZE) - 1)
    }

    pub const fn set_c_uimm2_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM2;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_UIMM2_SIZE) - 1)) << INSN_FIELD_C_UIMM2_START;
        self
    }
    pub const fn c_uimm1_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_UIMM1_START) & ((1 << INSN_FIELD_C_UIMM1_SIZE) - 1)
    }

    pub const fn set_c_uimm1_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_UIMM1;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_UIMM1_SIZE) - 1)) << INSN_FIELD_C_UIMM1_START;
        self
    }
    pub const fn c_rlist(self) -> u32 {
        (self.value >> INSN_FIELD_C_RLIST_START) & ((1 << INSN_FIELD_C_RLIST_SIZE) - 1)
    }

    pub const fn set_c_rlist(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_RLIST;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_RLIST_SIZE) - 1)) << INSN_FIELD_C_RLIST_START;
        self
    }
    pub const fn c_spimm_raw(self) -> u32 {
        (self.value >> INSN_FIELD_C_SPIMM_START) & ((1 << INSN_FIELD_C_SPIMM_SIZE) - 1)
    }

    pub const fn set_c_spimm_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_SPIMM;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_SPIMM_SIZE) - 1)) << INSN_FIELD_C_SPIMM_START;
        self
    }
    pub const fn c_index(self) -> u32 {
        (self.value >> INSN_FIELD_C_INDEX_START) & ((1 << INSN_FIELD_C_INDEX_SIZE) - 1)
    }

    pub const fn set_c_index(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_INDEX;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_INDEX_SIZE) - 1)) << INSN_FIELD_C_INDEX_START;
        self
    }
    pub const fn rs1_p(self) -> u32 {
        (self.value >> INSN_FIELD_RS1_P_START) & ((1 << INSN_FIELD_RS1_P_SIZE) - 1)
    }

    pub const fn set_rs1_p(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RS1_P;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RS1_P_SIZE) - 1)) << INSN_FIELD_RS1_P_START;
        self
    }
    pub const fn rs2_p(self) -> u32 {
        (self.value >> INSN_FIELD_RS2_P_START) & ((1 << INSN_FIELD_RS2_P_SIZE) - 1)
    }

    pub const fn set_rs2_p(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RS2_P;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RS2_P_SIZE) - 1)) << INSN_FIELD_RS2_P_START;
        self
    }
    pub const fn rd_p(self) -> u32 {
        (self.value >> INSN_FIELD_RD_P_START) & ((1 << INSN_FIELD_RD_P_SIZE) - 1)
    }

    pub const fn set_rd_p(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RD_P;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RD_P_SIZE) - 1)) << INSN_FIELD_RD_P_START;
        self
    }
    pub const fn rd_rs1_n0(self) -> u32 {
        (self.value >> INSN_FIELD_RD_RS1_N0_START) & ((1 << INSN_FIELD_RD_RS1_N0_SIZE) - 1)
    }

    pub const fn set_rd_rs1_n0(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RD_RS1_N0;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_RD_RS1_N0_SIZE) - 1)) << INSN_FIELD_RD_RS1_N0_START;
        self
    }
    pub const fn rd_rs1_p(self) -> u32 {
        (self.value >> INSN_FIELD_RD_RS1_P_START) & ((1 << INSN_FIELD_RD_RS1_P_SIZE) - 1)
    }

    pub const fn set_rd_rs1_p(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RD_RS1_P;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RD_RS1_P_SIZE) - 1)) << INSN_FIELD_RD_RS1_P_START;
        self
    }
    pub const fn rd_rs1(self) -> u32 {
        (self.value >> INSN_FIELD_RD_RS1_START) & ((1 << INSN_FIELD_RD_RS1_SIZE) - 1)
    }

    pub const fn set_rd_rs1(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RD_RS1;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RD_RS1_SIZE) - 1)) << INSN_FIELD_RD_RS1_START;
        self
    }
    pub const fn rd_n2(self) -> u32 {
        (self.value >> INSN_FIELD_RD_N2_START) & ((1 << INSN_FIELD_RD_N2_SIZE) - 1)
    }

    pub const fn set_rd_n2(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RD_N2;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RD_N2_SIZE) - 1)) << INSN_FIELD_RD_N2_START;
        self
    }
    pub const fn rd_n0(self) -> u32 {
        (self.value >> INSN_FIELD_RD_N0_START) & ((1 << INSN_FIELD_RD_N0_SIZE) - 1)
    }

    pub const fn set_rd_n0(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RD_N0;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RD_N0_SIZE) - 1)) << INSN_FIELD_RD_N0_START;
        self
    }
    pub const fn rs1_n0(self) -> u32 {
        (self.value >> INSN_FIELD_RS1_N0_START) & ((1 << INSN_FIELD_RS1_N0_SIZE) - 1)
    }

    pub const fn set_rs1_n0(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RS1_N0;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RS1_N0_SIZE) - 1)) << INSN_FIELD_RS1_N0_START;
        self
    }
    pub const fn c_rs2_n0(self) -> u32 {
        (self.value >> INSN_FIELD_C_RS2_N0_START) & ((1 << INSN_FIELD_C_RS2_N0_SIZE) - 1)
    }

    pub const fn set_c_rs2_n0(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_RS2_N0;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_RS2_N0_SIZE) - 1)) << INSN_FIELD_C_RS2_N0_START;
        self
    }
    pub const fn c_rs1_n0(self) -> u32 {
        (self.value >> INSN_FIELD_C_RS1_N0_START) & ((1 << INSN_FIELD_C_RS1_N0_SIZE) - 1)
    }

    pub const fn set_c_rs1_n0(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_RS1_N0;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_RS1_N0_SIZE) - 1)) << INSN_FIELD_C_RS1_N0_START;
        self
    }
    pub const fn c_rs2(self) -> u32 {
        (self.value >> INSN_FIELD_C_RS2_START) & ((1 << INSN_FIELD_C_RS2_SIZE) - 1)
    }

    pub const fn set_c_rs2(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_RS2;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_RS2_SIZE) - 1)) << INSN_FIELD_C_RS2_START;
        self
    }
    pub const fn c_sreg1(self) -> u32 {
        (self.value >> INSN_FIELD_C_SREG1_START) & ((1 << INSN_FIELD_C_SREG1_SIZE) - 1)
    }

    pub const fn set_c_sreg1(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_SREG1;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_SREG1_SIZE) - 1)) << INSN_FIELD_C_SREG1_START;
        self
    }
    pub const fn c_sreg2(self) -> u32 {
        (self.value >> INSN_FIELD_C_SREG2_START) & ((1 << INSN_FIELD_C_SREG2_SIZE) - 1)
    }

    pub const fn set_c_sreg2(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_SREG2;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_SREG2_SIZE) - 1)) << INSN_FIELD_C_SREG2_START;
        self
    }
    pub const fn mop_r_t_30(self) -> u32 {
        (self.value >> INSN_FIELD_MOP_R_T_30_START) & ((1 << INSN_FIELD_MOP_R_T_30_SIZE) - 1)
    }

    pub const fn set_mop_r_t_30(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_MOP_R_T_30;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_MOP_R_T_30_SIZE) - 1)) << INSN_FIELD_MOP_R_T_30_START;
        self
    }
    pub const fn mop_r_t_27_26(self) -> u32 {
        (self.value >> INSN_FIELD_MOP_R_T_27_26_START) & ((1 << INSN_FIELD_MOP_R_T_27_26_SIZE) - 1)
    }

    pub const fn set_mop_r_t_27_26(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_MOP_R_T_27_26;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_MOP_R_T_27_26_SIZE) - 1)) << INSN_FIELD_MOP_R_T_27_26_START;
        self
    }
    pub const fn mop_r_t_21_20(self) -> u32 {
        (self.value >> INSN_FIELD_MOP_R_T_21_20_START) & ((1 << INSN_FIELD_MOP_R_T_21_20_SIZE) - 1)
    }

    pub const fn set_mop_r_t_21_20(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_MOP_R_T_21_20;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_MOP_R_T_21_20_SIZE) - 1)) << INSN_FIELD_MOP_R_T_21_20_START;
        self
    }
    pub const fn mop_rr_t_30(self) -> u32 {
        (self.value >> INSN_FIELD_MOP_RR_T_30_START) & ((1 << INSN_FIELD_MOP_RR_T_30_SIZE) - 1)
    }

    pub const fn set_mop_rr_t_30(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_MOP_RR_T_30;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_MOP_RR_T_30_SIZE) - 1)) << INSN_FIELD_MOP_RR_T_30_START;
        self
    }
    pub const fn mop_rr_t_27_26(self) -> u32 {
        (self.value >> INSN_FIELD_MOP_RR_T_27_26_START)
            & ((1 << INSN_FIELD_MOP_RR_T_27_26_SIZE) - 1)
    }

    pub const fn set_mop_rr_t_27_26(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_MOP_RR_T_27_26;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_MOP_RR_T_27_26_SIZE) - 1))
            << INSN_FIELD_MOP_RR_T_27_26_START;
        self
    }
    pub const fn c_mop_t(self) -> u32 {
        (self.value >> INSN_FIELD_C_MOP_T_START) & ((1 << INSN_FIELD_C_MOP_T_SIZE) - 1)
    }

    pub const fn set_c_mop_t(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_MOP_T;

        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_MOP_T_SIZE) - 1)) << INSN_FIELD_C_MOP_T_START;
        self
    }
    pub const fn rs2_eq_rs1(self) -> u32 {
        (self.value >> INSN_FIELD_RS2_EQ_RS1_START) & ((1 << INSN_FIELD_RS2_EQ_RS1_SIZE) - 1)
    }

    pub const fn set_rs2_eq_rs1(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RS2_EQ_RS1;

        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_RS2_EQ_RS1_SIZE) - 1)) << INSN_FIELD_RS2_EQ_RS1_START;
        self
    }

    /// imm20
    pub const fn imm20(self) -> i32 {
        decode_immediate(&IMM20, self.value as _) as _
    }

    pub const fn set_imm20(mut self, imm20: i32) -> Self {
        self.value |= encode_immediate(&IMM20, imm20 as _);
        self
    }

    /// jimm20
    pub const fn jimm20(self) -> i32 {
        decode_immediate(&JIMM20, self.value as _) as _
    }

    pub const fn set_jimm20(mut self, jimm20: i32) -> Self {
        self.value |= encode_immediate(&JIMM20, jimm20 as _);
        self
    }

    /// imm12
    pub const fn imm12(self) -> i32 {
        decode_immediate(&IMM12, self.value as _) as _
    }

    pub const fn set_imm12(mut self, imm12: i32) -> Self {
        self.value |= encode_immediate(&IMM12, imm12 as _);
        self
    }

    /// imm12lohi
    pub const fn imm12lohi(self) -> i32 {
        decode_immediate(&IMM12LOHI, self.value as _) as _
    }

    pub const fn set_imm12lohi(mut self, imm12lohi: i32) -> Self {
        self.value |= encode_immediate(&IMM12LOHI, imm12lohi as _);
        self
    }

    /// bimm12lohi
    pub const fn bimm12lohi(self) -> i32 {
        decode_immediate(&BIMM12LOHI, self.value as _) as _
    }

    pub const fn set_bimm12lohi(mut self, bimm12lohi: i32) -> Self {
        self.value |= encode_immediate(&BIMM12LOHI, bimm12lohi as _);
        self
    }

    /// imm2
    pub const fn imm2(self) -> i32 {
        decode_immediate(&IMM2, self.value as _) as _
    }

    pub const fn set_imm2(mut self, imm2: i32) -> Self {
        self.value |= encode_immediate(&IMM2, imm2 as _);
        self
    }

    /// imm3
    pub const fn imm3(self) -> i32 {
        decode_immediate(&IMM3, self.value as _) as _
    }

    pub const fn set_imm3(mut self, imm3: i32) -> Self {
        self.value |= encode_immediate(&IMM3, imm3 as _);
        self
    }

    /// imm4
    pub const fn imm4(self) -> i32 {
        decode_immediate(&IMM4, self.value as _) as _
    }

    pub const fn set_imm4(mut self, imm4: i32) -> Self {
        self.value |= encode_immediate(&IMM4, imm4 as _);
        self
    }

    /// imm5
    pub const fn imm5(self) -> i32 {
        decode_immediate(&IMM5, self.value as _) as _
    }

    pub const fn set_imm5(mut self, imm5: i32) -> Self {
        self.value |= encode_immediate(&IMM5, imm5 as _);
        self
    }

    /// imm6
    pub const fn imm6(self) -> i32 {
        decode_immediate(&IMM6, self.value as _) as _
    }

    pub const fn set_imm6(mut self, imm6: i32) -> Self {
        self.value |= encode_immediate(&IMM6, imm6 as _);
        self
    }

    /// zimm
    pub const fn zimm(self) -> i32 {
        decode_immediate(&ZIMM, self.value as _) as _
    }

    pub const fn set_zimm(mut self, zimm: i32) -> Self {
        self.value |= encode_immediate(&ZIMM, zimm as _);
        self
    }

    /// simm5
    pub const fn simm5(self) -> i32 {
        decode_immediate(&SIMM5, self.value as _) as _
    }

    pub const fn set_simm5(mut self, simm5: i32) -> Self {
        self.value |= encode_immediate(&SIMM5, simm5 as _);
        self
    }

    /// zimm5
    pub const fn zimm5(self) -> i32 {
        decode_immediate(&ZIMM5, self.value as _) as _
    }

    pub const fn set_zimm5(mut self, zimm5: i32) -> Self {
        self.value |= encode_immediate(&ZIMM5, zimm5 as _);
        self
    }

    /// zimm10
    pub const fn zimm10(self) -> i32 {
        decode_immediate(&ZIMM10, self.value as _) as _
    }

    pub const fn set_zimm10(mut self, zimm10: i32) -> Self {
        self.value |= encode_immediate(&ZIMM10, zimm10 as _);
        self
    }

    /// zimm11
    pub const fn zimm11(self) -> i32 {
        decode_immediate(&ZIMM11, self.value as _) as _
    }

    pub const fn set_zimm11(mut self, zimm11: i32) -> Self {
        self.value |= encode_immediate(&ZIMM11, zimm11 as _);
        self
    }

    /// zimm6lohi
    pub const fn zimm6lohi(self) -> i32 {
        decode_immediate(&ZIMM6LOHI, self.value as _) as _
    }

    pub const fn set_zimm6lohi(mut self, zimm6lohi: i32) -> Self {
        self.value |= encode_immediate(&ZIMM6LOHI, zimm6lohi as _);
        self
    }

    /// c_nzuimm10
    pub const fn c_nzuimm10(self) -> u32 {
        decode_immediate(&C_NZUIMM10, self.value as _) as _
    }

    pub const fn set_c_nzuimm10(mut self, c_nzuimm10: u32) -> Self {
        self.value |= encode_immediate(&C_NZUIMM10, c_nzuimm10 as _);
        self
    }

    /// c_uimm7lohi
    pub const fn c_uimm7lohi(self) -> u32 {
        decode_immediate(&C_UIMM7LOHI, self.value as _) as _
    }

    pub const fn set_c_uimm7lohi(mut self, c_uimm7lohi: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM7LOHI, c_uimm7lohi as _);
        self
    }

    /// c_uimm8lohi
    pub const fn c_uimm8lohi(self) -> u32 {
        decode_immediate(&C_UIMM8LOHI, self.value as _) as _
    }

    pub const fn set_c_uimm8lohi(mut self, c_uimm8lohi: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM8LOHI, c_uimm8lohi as _);
        self
    }

    /// c_uimm9lohi
    pub const fn c_uimm9lohi(self) -> u32 {
        decode_immediate(&C_UIMM9LOHI, self.value as _) as _
    }

    pub const fn set_c_uimm9lohi(mut self, c_uimm9lohi: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM9LOHI, c_uimm9lohi as _);
        self
    }

    /// c_nzimm6lohi
    pub const fn c_nzimm6lohi(self) -> i32 {
        decode_immediate(&C_NZIMM6LOHI, self.value as _) as _
    }

    pub const fn set_c_nzimm6lohi(mut self, c_nzimm6lohi: i32) -> Self {
        self.value |= encode_immediate(&C_NZIMM6LOHI, c_nzimm6lohi as _);
        self
    }

    /// c_imm6lohi
    pub const fn c_imm6lohi(self) -> i32 {
        decode_immediate(&C_IMM6LOHI, self.value as _) as _
    }

    pub const fn set_c_imm6lohi(mut self, c_imm6lohi: i32) -> Self {
        self.value |= encode_immediate(&C_IMM6LOHI, c_imm6lohi as _);
        self
    }

    /// c_nzimm10lohi
    pub const fn c_nzimm10lohi(self) -> i32 {
        decode_immediate(&C_NZIMM10LOHI, self.value as _) as _
    }

    pub const fn set_c_nzimm10lohi(mut self, c_nzimm10lohi: i32) -> Self {
        self.value |= encode_immediate(&C_NZIMM10LOHI, c_nzimm10lohi as _);
        self
    }

    /// c_nzimm18lohi
    pub const fn c_nzimm18lohi(self) -> i32 {
        decode_immediate(&C_NZIMM18LOHI, self.value as _) as _
    }

    pub const fn set_c_nzimm18lohi(mut self, c_nzimm18lohi: i32) -> Self {
        self.value |= encode_immediate(&C_NZIMM18LOHI, c_nzimm18lohi as _);
        self
    }

    /// c_imm12
    pub const fn c_imm12(self) -> i32 {
        decode_immediate(&C_IMM12, self.value as _) as _
    }

    pub const fn set_c_imm12(mut self, c_imm12: i32) -> Self {
        self.value |= encode_immediate(&C_IMM12, c_imm12 as _);
        self
    }

    /// c_bimm9lohi
    pub const fn c_bimm9lohi(self) -> i32 {
        decode_immediate(&C_BIMM9LOHI, self.value as _) as _
    }

    pub const fn set_c_bimm9lohi(mut self, c_bimm9lohi: i32) -> Self {
        self.value |= encode_immediate(&C_BIMM9LOHI, c_bimm9lohi as _);
        self
    }

    /// c_nzuimm5
    pub const fn c_nzuimm5(self) -> u32 {
        decode_immediate(&C_NZUIMM5, self.value as _) as _
    }

    pub const fn set_c_nzuimm5(mut self, c_nzuimm5: u32) -> Self {
        self.value |= encode_immediate(&C_NZUIMM5, c_nzuimm5 as _);
        self
    }

    /// c_nzuimm6lohi
    pub const fn c_nzuimm6lohi(self) -> u32 {
        decode_immediate(&C_NZUIMM6LOHI, self.value as _) as _
    }

    pub const fn set_c_nzuimm6lohi(mut self, c_nzuimm6lohi: u32) -> Self {
        self.value |= encode_immediate(&C_NZUIMM6LOHI, c_nzuimm6lohi as _);
        self
    }

    /// c_uimm8splohi
    pub const fn c_uimm8splohi(self) -> u32 {
        decode_immediate(&C_UIMM8SPLOHI, self.value as _) as _
    }

    pub const fn set_c_uimm8splohi(mut self, c_uimm8splohi: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM8SPLOHI, c_uimm8splohi as _);
        self
    }

    /// c_uimm8sp_s
    pub const fn c_uimm8sp_s(self) -> u32 {
        decode_immediate(&C_UIMM8SP_S, self.value as _) as _
    }

    pub const fn set_c_uimm8sp_s(mut self, c_uimm8sp_s: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM8SP_S, c_uimm8sp_s as _);
        self
    }

    /// c_uimm10splohi
    pub const fn c_uimm10splohi(self) -> u32 {
        decode_immediate(&C_UIMM10SPLOHI, self.value as _) as _
    }

    pub const fn set_c_uimm10splohi(mut self, c_uimm10splohi: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM10SPLOHI, c_uimm10splohi as _);
        self
    }

    /// c_uimm9splohi
    pub const fn c_uimm9splohi(self) -> u32 {
        decode_immediate(&C_UIMM9SPLOHI, self.value as _) as _
    }

    pub const fn set_c_uimm9splohi(mut self, c_uimm9splohi: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM9SPLOHI, c_uimm9splohi as _);
        self
    }

    /// c_uimm10sp_s
    pub const fn c_uimm10sp_s(self) -> u32 {
        decode_immediate(&C_UIMM10SP_S, self.value as _) as _
    }

    pub const fn set_c_uimm10sp_s(mut self, c_uimm10sp_s: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM10SP_S, c_uimm10sp_s as _);
        self
    }

    /// c_uimm9sp_s
    pub const fn c_uimm9sp_s(self) -> u32 {
        decode_immediate(&C_UIMM9SP_S, self.value as _) as _
    }

    pub const fn set_c_uimm9sp_s(mut self, c_uimm9sp_s: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM9SP_S, c_uimm9sp_s as _);
        self
    }

    /// c_uimm2
    pub const fn c_uimm2(self) -> u32 {
        decode_immediate(&C_UIMM2, self.value as _) as _
    }

    pub const fn set_c_uimm2(mut self, c_uimm2: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM2, c_uimm2 as _);
        self
    }

    /// c_uimm1
    pub const fn c_uimm1(self) -> u32 {
        decode_immediate(&C_UIMM1, self.value as _) as _
    }

    pub const fn set_c_uimm1(mut self, c_uimm1: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM1, c_uimm1 as _);
        self
    }

    /// c_spimm
    pub const fn c_spimm(self) -> i32 {
        decode_immediate(&C_SPIMM, self.value as _) as _
    }

    pub const fn set_c_spimm(mut self, c_spimm: i32) -> Self {
        self.value |= encode_immediate(&C_SPIMM, c_spimm as _);
        self
    }
}
