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

pub const C_NZUIMM5: &[Immediate] = C_NZUIMM6LOHI;

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
    let mut res = 0i32;
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

        ((imm >> self.position_in_immediate.1) & mask) << self.position_in_opcode.1
    }

    pub const fn decode(&self, op: u32) -> i32 {
        let bit_count = self.position_in_opcode.0 - self.position_in_opcode.1 + 1;
        let mask = (1u32 << bit_count) - 1;
        (((op >> self.position_in_opcode.1) & mask) << self.position_in_immediate.1) as _
    }

    pub const fn is_valid(&self, imm: i32) -> bool {
        self.decode(self.encode(imm)) == imm
    }
}

/* Automatically generated by parse_opcodes */
/* Automatically generated by parse_opcodes (meta/riscv.py). Do not edit by hand.
 * Derived from riscv-opcodes (BSD-3-Clause) and riscv-unified-db
 * (BSD-3-Clause-Clear); see meta/README.md for the input pins. */

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
pub const MATCH_C_SSPOPCHK_X5: u32 = 0x6281;
pub const MASK_C_SSPOPCHK_X5: u32 = 0xffff;
pub const MATCH_C_SSPUSH_X1: u32 = 0x6081;
pub const MASK_C_SSPUSH_X1: u32 = 0xffff;
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
pub const MATCH_FCVT_BF16_S: u32 = 0x44800053;
pub const MASK_FCVT_BF16_S: u32 = 0xfff0007f;
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
pub const MATCH_FCVT_S_BF16: u32 = 0x40600053;
pub const MASK_FCVT_S_BF16: u32 = 0xfff0007f;
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
pub const MATCH_LPAD: u32 = 0x17;
pub const MASK_LPAD: u32 = 0xfff;
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
pub const MATCH_MNRET: u32 = 0x70200073;
pub const MASK_MNRET: u32 = 0xffffffff;
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
pub const MATCH_SCTRCLR: u32 = 0x10400073;
pub const MASK_SCTRCLR: u32 = 0xffffffff;
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
pub const MATCH_SSAMOSWAP_D: u32 = 0x4800302f;
pub const MASK_SSAMOSWAP_D: u32 = 0xf800707f;
pub const MATCH_SSAMOSWAP_W: u32 = 0x4800202f;
pub const MASK_SSAMOSWAP_W: u32 = 0xf800707f;
pub const MATCH_SSPOPCHK_X1: u32 = 0xcdc0c073;
pub const MASK_SSPOPCHK_X1: u32 = 0xffffffff;
pub const MATCH_SSPOPCHK_X5: u32 = 0xcdc2c073;
pub const MASK_SSPOPCHK_X5: u32 = 0xffffffff;
pub const MATCH_SSPUSH_X1: u32 = 0xce104073;
pub const MASK_SSPUSH_X1: u32 = 0xffffffff;
pub const MATCH_SSPUSH_X5: u32 = 0xce504073;
pub const MASK_SSPUSH_X5: u32 = 0xffffffff;
pub const MATCH_SSRDP: u32 = 0xcdc04073;
pub const MASK_SSRDP: u32 = 0xfffff07f;
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
pub const MATCH_VFNCVTBF16_F_F_W: u32 = 0x480e9057;
pub const MASK_VFNCVTBF16_F_F_W: u32 = 0xfc0ff07f;
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
pub const MATCH_VFWCVTBF16_F_F_V: u32 = 0x48069057;
pub const MASK_VFWCVTBF16_F_F_V: u32 = 0xfc0ff07f;
pub const MATCH_VFWMACC_VF: u32 = 0xf0005057;
pub const MASK_VFWMACC_VF: u32 = 0xfc00707f;
pub const MATCH_VFWMACC_VV: u32 = 0xf0001057;
pub const MASK_VFWMACC_VV: u32 = 0xfc00707f;
pub const MATCH_VFWMACCBF16_VF: u32 = 0xec005057;
pub const MASK_VFWMACCBF16_VF: u32 = 0xfc00707f;
pub const MATCH_VFWMACCBF16_VV: u32 = 0xec001057;
pub const MASK_VFWMACCBF16_VV: u32 = 0xfc00707f;
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
pub const MATCH_ZEXT_B: u32 = 0xff07013;
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
pub const CSR_SPMPEN: u16 = 0x183;
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
pub const CSR_MPMPDELEG: u16 = 0x316;
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
pub const CSR_SPMPENH: u16 = 0x193;
pub const CSR_VSIEH: u16 = 0x214;
pub const CSR_VSIPH: u16 = 0x254;
pub const CSR_VSTIMECMPH: u16 = 0x25d;
pub const CSR_MEDELEGH: u16 = 0x312;
pub const CSR_HEDELEGH: u16 = 0x612;
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
pub static OPCODE32_MATCH: [u32; 1039] = [
    0x33,        /* add */
    0xffff_ffff, /* add_uw */
    0x13,        /* addi */
    0xffff_ffff, /* addiw */
    0xffff_ffff, /* addw */
    0x2a000033,  /* aes32dsi */
    0x2e000033,  /* aes32dsmi */
    0x22000033,  /* aes32esi */
    0x26000033,  /* aes32esmi */
    0xffff_ffff, /* aes64ds */
    0xffff_ffff, /* aes64dsm */
    0xffff_ffff, /* aes64es */
    0xffff_ffff, /* aes64esm */
    0xffff_ffff, /* aes64im */
    0xffff_ffff, /* aes64ks1i */
    0xffff_ffff, /* aes64ks2 */
    0x2f,        /* amoadd_b */
    0xffff_ffff, /* amoadd_d */
    0x102f,      /* amoadd_h */
    0x202f,      /* amoadd_w */
    0x6000002f,  /* amoand_b */
    0xffff_ffff, /* amoand_d */
    0x6000102f,  /* amoand_h */
    0x6000202f,  /* amoand_w */
    0x2800002f,  /* amocas_b */
    0x2800302f,  /* amocas_d */
    0x2800102f,  /* amocas_h */
    0xffff_ffff, /* amocas_q */
    0x2800202f,  /* amocas_w */
    0xa000002f,  /* amomax_b */
    0xffff_ffff, /* amomax_d */
    0xa000102f,  /* amomax_h */
    0xa000202f,  /* amomax_w */
    0xe000002f,  /* amomaxu_b */
    0xffff_ffff, /* amomaxu_d */
    0xe000102f,  /* amomaxu_h */
    0xe000202f,  /* amomaxu_w */
    0x8000002f,  /* amomin_b */
    0xffff_ffff, /* amomin_d */
    0x8000102f,  /* amomin_h */
    0x8000202f,  /* amomin_w */
    0xc000002f,  /* amominu_b */
    0xffff_ffff, /* amominu_d */
    0xc000102f,  /* amominu_h */
    0xc000202f,  /* amominu_w */
    0x4000002f,  /* amoor_b */
    0xffff_ffff, /* amoor_d */
    0x4000102f,  /* amoor_h */
    0x4000202f,  /* amoor_w */
    0x800002f,   /* amoswap_b */
    0xffff_ffff, /* amoswap_d */
    0x800102f,   /* amoswap_h */
    0x800202f,   /* amoswap_w */
    0x2000002f,  /* amoxor_b */
    0xffff_ffff, /* amoxor_d */
    0x2000102f,  /* amoxor_h */
    0x2000202f,  /* amoxor_w */
    0x7033,      /* and */
    0x7013,      /* andi */
    0x40007033,  /* andn */
    0x17,        /* auipc */
    0x48001033,  /* bclr */
    0xffff_ffff, /* bclri */
    0x48001013,  /* bclri_rv32 */
    0x63,        /* beq */
    0x63,        /* beqz */
    0x48005033,  /* bext */
    0xffff_ffff, /* bexti */
    0x48005013,  /* bexti_rv32 */
    0x5063,      /* bge */
    0x7063,      /* bgeu */
    0x5063,      /* bgez */
    0x4063,      /* bgt */
    0x6063,      /* bgtu */
    0x4063,      /* bgtz */
    0x68001033,  /* binv */
    0xffff_ffff, /* binvi */
    0x68001013,  /* binvi_rv32 */
    0x5063,      /* ble */
    0x7063,      /* bleu */
    0x5063,      /* blez */
    0x4063,      /* blt */
    0x6063,      /* bltu */
    0x4063,      /* bltz */
    0x1063,      /* bne */
    0x1063,      /* bnez */
    0x68705013,  /* brev8 */
    0x28001033,  /* bset */
    0xffff_ffff, /* bseti */
    0x28001013,  /* bseti_rv32 */
    0x9002,      /* c_add */
    0x1,         /* c_addi */
    0x6101,      /* c_addi16sp */
    0x0,         /* c_addi4spn */
    0xffff_ffff, /* c_addiw */
    0xffff_ffff, /* c_addw */
    0x8c61,      /* c_and */
    0x8801,      /* c_andi */
    0xc001,      /* c_beqz */
    0xe001,      /* c_bnez */
    0x9002,      /* c_ebreak */
    0x2000,      /* c_fld */
    0x2002,      /* c_fldsp */
    0x6000,      /* c_flw */
    0x6002,      /* c_flwsp */
    0xa000,      /* c_fsd */
    0xa002,      /* c_fsdsp */
    0xe000,      /* c_fsw */
    0xe002,      /* c_fswsp */
    0xa001,      /* c_j */
    0x2001,      /* c_jal */
    0x9002,      /* c_jalr */
    0x8002,      /* c_jr */
    0x8000,      /* c_lbu */
    0x6000,      /* c_ld */
    0x6002,      /* c_ldsp */
    0x8440,      /* c_lh */
    0x8400,      /* c_lhu */
    0x4001,      /* c_li */
    0x6001,      /* c_lui */
    0x4000,      /* c_lw */
    0x4002,      /* c_lwsp */
    0x6081,      /* c_mop_1 */
    0x6581,      /* c_mop_11 */
    0x6681,      /* c_mop_13 */
    0x6781,      /* c_mop_15 */
    0x6181,      /* c_mop_3 */
    0x6281,      /* c_mop_5 */
    0x6381,      /* c_mop_7 */
    0x6481,      /* c_mop_9 */
    0x6081,      /* c_mop_N */
    0x9c41,      /* c_mul */
    0x8002,      /* c_mv */
    0x1,         /* c_nop */
    0x9c75,      /* c_not */
    0x9016,      /* c_ntl_all */
    0x900a,      /* c_ntl_p1 */
    0x900e,      /* c_ntl_pall */
    0x9012,      /* c_ntl_s1 */
    0x8c41,      /* c_or */
    0x8800,      /* c_sb */
    0xe000,      /* c_sd */
    0xe002,      /* c_sdsp */
    0x9c65,      /* c_sext_b */
    0x9c6d,      /* c_sext_h */
    0xffff_ffff, /* c_sext_w */
    0x8c00,      /* c_sh */
    0x2,         /* c_slli */
    0x2,         /* c_slli_rv32 */
    0x8401,      /* c_srai */
    0x8401,      /* c_srai_rv32 */
    0x8001,      /* c_srli */
    0x8001,      /* c_srli_rv32 */
    0x6281,      /* c_sspopchk_x5 */
    0x6081,      /* c_sspush_x1 */
    0x8c01,      /* c_sub */
    0xffff_ffff, /* c_subw */
    0xc000,      /* c_sw */
    0xc002,      /* c_swsp */
    0x8c21,      /* c_xor */
    0x9c61,      /* c_zext_b */
    0x9c69,      /* c_zext_h */
    0xffff_ffff, /* c_zext_w */
    0x10200f,    /* cbo_clean */
    0x20200f,    /* cbo_flush */
    0x200f,      /* cbo_inval */
    0x40200f,    /* cbo_zero */
    0xa001033,   /* clmul */
    0xa003033,   /* clmulh */
    0xa002033,   /* clmulr */
    0x60001013,  /* clz */
    0xffff_ffff, /* clzw */
    0xa002,      /* cm_jalt */
    0xac62,      /* cm_mva01s */
    0xac22,      /* cm_mvsa01 */
    0xba02,      /* cm_pop */
    0xbe02,      /* cm_popret */
    0xbc02,      /* cm_popretz */
    0xb802,      /* cm_push */
    0x60201013,  /* cpop */
    0xffff_ffff, /* cpopw */
    0x3073,      /* csrc */
    0x7073,      /* csrci */
    0x2073,      /* csrr */
    0x3073,      /* csrrc */
    0x7073,      /* csrrci */
    0x2073,      /* csrrs */
    0x6073,      /* csrrsi */
    0x1073,      /* csrrw */
    0x5073,      /* csrrwi */
    0x2073,      /* csrs */
    0x6073,      /* csrsi */
    0x1073,      /* csrw */
    0x5073,      /* csrwi */
    0x60101013,  /* ctz */
    0xffff_ffff, /* ctzw */
    0xe005033,   /* czero_eqz */
    0xe007033,   /* czero_nez */
    0x2004033,   /* div */
    0x2005033,   /* divu */
    0xffff_ffff, /* divuw */
    0xffff_ffff, /* divw */
    0x7b200073,  /* dret */
    0x100073,    /* ebreak */
    0x73,        /* ecall */
    0x22002053,  /* fabs_d */
    0x24002053,  /* fabs_h */
    0x26002053,  /* fabs_q */
    0x20002053,  /* fabs_s */
    0x2000053,   /* fadd_d */
    0x4000053,   /* fadd_h */
    0x6000053,   /* fadd_q */
    0x53,        /* fadd_s */
    0xe2001053,  /* fclass_d */
    0xe4001053,  /* fclass_h */
    0xe6001053,  /* fclass_q */
    0xe0001053,  /* fclass_s */
    0x44800053,  /* fcvt_bf16_s */
    0x42200053,  /* fcvt_d_h */
    0xffff_ffff, /* fcvt_d_l */
    0xffff_ffff, /* fcvt_d_lu */
    0x42300053,  /* fcvt_d_q */
    0x42000053,  /* fcvt_d_s */
    0xd2000053,  /* fcvt_d_w */
    0xd2100053,  /* fcvt_d_wu */
    0x44100053,  /* fcvt_h_d */
    0xffff_ffff, /* fcvt_h_l */
    0xffff_ffff, /* fcvt_h_lu */
    0x44300053,  /* fcvt_h_q */
    0x44000053,  /* fcvt_h_s */
    0xd4000053,  /* fcvt_h_w */
    0xd4100053,  /* fcvt_h_wu */
    0xffff_ffff, /* fcvt_l_d */
    0xffff_ffff, /* fcvt_l_h */
    0xffff_ffff, /* fcvt_l_q */
    0xffff_ffff, /* fcvt_l_s */
    0xffff_ffff, /* fcvt_lu_d */
    0xffff_ffff, /* fcvt_lu_h */
    0xffff_ffff, /* fcvt_lu_q */
    0xffff_ffff, /* fcvt_lu_s */
    0x46100053,  /* fcvt_q_d */
    0x46200053,  /* fcvt_q_h */
    0xffff_ffff, /* fcvt_q_l */
    0xffff_ffff, /* fcvt_q_lu */
    0x46000053,  /* fcvt_q_s */
    0xd6000053,  /* fcvt_q_w */
    0xd6100053,  /* fcvt_q_wu */
    0x40600053,  /* fcvt_s_bf16 */
    0x40100053,  /* fcvt_s_d */
    0x40200053,  /* fcvt_s_h */
    0xffff_ffff, /* fcvt_s_l */
    0xffff_ffff, /* fcvt_s_lu */
    0x40300053,  /* fcvt_s_q */
    0xd0000053,  /* fcvt_s_w */
    0xd0100053,  /* fcvt_s_wu */
    0xc2000053,  /* fcvt_w_d */
    0xc4000053,  /* fcvt_w_h */
    0xc6000053,  /* fcvt_w_q */
    0xc0000053,  /* fcvt_w_s */
    0xc2100053,  /* fcvt_wu_d */
    0xc4100053,  /* fcvt_wu_h */
    0xc6100053,  /* fcvt_wu_q */
    0xc0100053,  /* fcvt_wu_s */
    0xc2801053,  /* fcvtmod_w_d */
    0x1a000053,  /* fdiv_d */
    0x1c000053,  /* fdiv_h */
    0x1e000053,  /* fdiv_q */
    0x18000053,  /* fdiv_s */
    0xf,         /* fence */
    0x100f,      /* fence_i */
    0x8330000f,  /* fence_tso */
    0xa2002053,  /* feq_d */
    0xa4002053,  /* feq_h */
    0xa6002053,  /* feq_q */
    0xa0002053,  /* feq_s */
    0x3007,      /* fld */
    0xa2000053,  /* fle_d */
    0xa4000053,  /* fle_h */
    0xa6000053,  /* fle_q */
    0xa0000053,  /* fle_s */
    0xa2004053,  /* fleq_d */
    0xa4004053,  /* fleq_h */
    0xa6004053,  /* fleq_q */
    0xa0004053,  /* fleq_s */
    0x1007,      /* flh */
    0xf2100053,  /* fli_d */
    0xf4100053,  /* fli_h */
    0xf6100053,  /* fli_q */
    0xf0100053,  /* fli_s */
    0x4007,      /* flq */
    0xa2001053,  /* flt_d */
    0xa4001053,  /* flt_h */
    0xa6001053,  /* flt_q */
    0xa0001053,  /* flt_s */
    0xa2005053,  /* fltq_d */
    0xa4005053,  /* fltq_h */
    0xa6005053,  /* fltq_q */
    0xa0005053,  /* fltq_s */
    0x2007,      /* flw */
    0x2000043,   /* fmadd_d */
    0x4000043,   /* fmadd_h */
    0x6000043,   /* fmadd_q */
    0x43,        /* fmadd_s */
    0x2a001053,  /* fmax_d */
    0x2c001053,  /* fmax_h */
    0x2e001053,  /* fmax_q */
    0x28001053,  /* fmax_s */
    0x2a003053,  /* fmaxm_d */
    0x2c003053,  /* fmaxm_h */
    0x2e003053,  /* fmaxm_q */
    0x28003053,  /* fmaxm_s */
    0x2a000053,  /* fmin_d */
    0x2c000053,  /* fmin_h */
    0x2e000053,  /* fmin_q */
    0x28000053,  /* fmin_s */
    0x2a002053,  /* fminm_d */
    0x2c002053,  /* fminm_h */
    0x2e002053,  /* fminm_q */
    0x28002053,  /* fminm_s */
    0x2000047,   /* fmsub_d */
    0x4000047,   /* fmsub_h */
    0x6000047,   /* fmsub_q */
    0x47,        /* fmsub_s */
    0x12000053,  /* fmul_d */
    0x14000053,  /* fmul_h */
    0x16000053,  /* fmul_q */
    0x10000053,  /* fmul_s */
    0x22000053,  /* fmv_d */
    0xffff_ffff, /* fmv_d_x */
    0x24000053,  /* fmv_h */
    0xf4000053,  /* fmv_h_x */
    0x26000053,  /* fmv_q */
    0x20000053,  /* fmv_s */
    0xf0000053,  /* fmv_s_x */
    0xf0000053,  /* fmv_w_x */
    0xffff_ffff, /* fmv_x_d */
    0xe4000053,  /* fmv_x_h */
    0xe0000053,  /* fmv_x_s */
    0xe0000053,  /* fmv_x_w */
    0xe2100053,  /* fmvh_x_d */
    0xffff_ffff, /* fmvh_x_q */
    0xb2000053,  /* fmvp_d_x */
    0xffff_ffff, /* fmvp_q_x */
    0x22001053,  /* fneg_d */
    0x24001053,  /* fneg_h */
    0x26001053,  /* fneg_q */
    0x20001053,  /* fneg_s */
    0x200004f,   /* fnmadd_d */
    0x400004f,   /* fnmadd_h */
    0x600004f,   /* fnmadd_q */
    0x4f,        /* fnmadd_s */
    0x200004b,   /* fnmsub_d */
    0x400004b,   /* fnmsub_h */
    0x600004b,   /* fnmsub_q */
    0x4b,        /* fnmsub_s */
    0x302073,    /* frcsr */
    0x102073,    /* frflags */
    0x42400053,  /* fround_d */
    0x44400053,  /* fround_h */
    0x46400053,  /* fround_q */
    0x40400053,  /* fround_s */
    0x42500053,  /* froundnx_d */
    0x44500053,  /* froundnx_h */
    0x46500053,  /* froundnx_q */
    0x40500053,  /* froundnx_s */
    0x202073,    /* frrm */
    0x301073,    /* fscsr */
    0x3027,      /* fsd */
    0x101073,    /* fsflags */
    0x105073,    /* fsflagsi */
    0x22000053,  /* fsgnj_d */
    0x24000053,  /* fsgnj_h */
    0x26000053,  /* fsgnj_q */
    0x20000053,  /* fsgnj_s */
    0x22001053,  /* fsgnjn_d */
    0x24001053,  /* fsgnjn_h */
    0x26001053,  /* fsgnjn_q */
    0x20001053,  /* fsgnjn_s */
    0x22002053,  /* fsgnjx_d */
    0x24002053,  /* fsgnjx_h */
    0x26002053,  /* fsgnjx_q */
    0x20002053,  /* fsgnjx_s */
    0x1027,      /* fsh */
    0x4027,      /* fsq */
    0x5a000053,  /* fsqrt_d */
    0x5c000053,  /* fsqrt_h */
    0x5e000053,  /* fsqrt_q */
    0x58000053,  /* fsqrt_s */
    0x201073,    /* fsrm */
    0x205073,    /* fsrmi */
    0xa000053,   /* fsub_d */
    0xc000053,   /* fsub_h */
    0xe000053,   /* fsub_q */
    0x8000053,   /* fsub_s */
    0x2027,      /* fsw */
    0x62000073,  /* hfence_gvma */
    0x22000073,  /* hfence_vvma */
    0x66000073,  /* hinval_gvma */
    0x26000073,  /* hinval_vvma */
    0x60004073,  /* hlv_b */
    0x60104073,  /* hlv_bu */
    0xffff_ffff, /* hlv_d */
    0x64004073,  /* hlv_h */
    0x64104073,  /* hlv_hu */
    0x68004073,  /* hlv_w */
    0xffff_ffff, /* hlv_wu */
    0x64304073,  /* hlvx_hu */
    0x68304073,  /* hlvx_wu */
    0x62004073,  /* hsv_b */
    0xffff_ffff, /* hsv_d */
    0x66004073,  /* hsv_h */
    0x6a004073,  /* hsv_w */
    0x6f,        /* j */
    0x6f,        /* jal */
    0xef,        /* jal_pseudo */
    0x67,        /* jalr */
    0xe7,        /* jalr_pseudo */
    0x67,        /* jr */
    0x3,         /* lb */
    0x4003,      /* lbu */
    0xffff_ffff, /* ld */
    0x1003,      /* lh */
    0x5003,      /* lhu */
    0x17,        /* lpad */
    0xffff_ffff, /* lr_d */
    0x1000202f,  /* lr_w */
    0x37,        /* lui */
    0x2003,      /* lw */
    0xffff_ffff, /* lwu */
    0xa006033,   /* max */
    0xa007033,   /* maxu */
    0xa004033,   /* min */
    0xa005033,   /* minu */
    0x70200073,  /* mnret */
    0x81c04073,  /* mop_r_0 */
    0x81d04073,  /* mop_r_1 */
    0x89e04073,  /* mop_r_10 */
    0x89f04073,  /* mop_r_11 */
    0x8dc04073,  /* mop_r_12 */
    0x8dd04073,  /* mop_r_13 */
    0x8de04073,  /* mop_r_14 */
    0x8df04073,  /* mop_r_15 */
    0xc1c04073,  /* mop_r_16 */
    0xc1d04073,  /* mop_r_17 */
    0xc1e04073,  /* mop_r_18 */
    0xc1f04073,  /* mop_r_19 */
    0x81e04073,  /* mop_r_2 */
    0xc5c04073,  /* mop_r_20 */
    0xc5d04073,  /* mop_r_21 */
    0xc5e04073,  /* mop_r_22 */
    0xc5f04073,  /* mop_r_23 */
    0xc9c04073,  /* mop_r_24 */
    0xc9d04073,  /* mop_r_25 */
    0xc9e04073,  /* mop_r_26 */
    0xc9f04073,  /* mop_r_27 */
    0xcdc04073,  /* mop_r_28 */
    0xcdd04073,  /* mop_r_29 */
    0x81f04073,  /* mop_r_3 */
    0xcde04073,  /* mop_r_30 */
    0xcdf04073,  /* mop_r_31 */
    0x85c04073,  /* mop_r_4 */
    0x85d04073,  /* mop_r_5 */
    0x85e04073,  /* mop_r_6 */
    0x85f04073,  /* mop_r_7 */
    0x89c04073,  /* mop_r_8 */
    0x89d04073,  /* mop_r_9 */
    0x81c04073,  /* mop_r_N */
    0x82004073,  /* mop_rr_0 */
    0x86004073,  /* mop_rr_1 */
    0x8a004073,  /* mop_rr_2 */
    0x8e004073,  /* mop_rr_3 */
    0xc2004073,  /* mop_rr_4 */
    0xc6004073,  /* mop_rr_5 */
    0xca004073,  /* mop_rr_6 */
    0xce004073,  /* mop_rr_7 */
    0x82004073,  /* mop_rr_N */
    0x30200073,  /* mret */
    0x2000033,   /* mul */
    0x2001033,   /* mulh */
    0x2002033,   /* mulhsu */
    0x2003033,   /* mulhu */
    0xffff_ffff, /* mulw */
    0x13,        /* mv */
    0x40000033,  /* neg */
    0x13,        /* nop */
    0x500033,    /* ntl_all */
    0x200033,    /* ntl_p1 */
    0x300033,    /* ntl_pall */
    0x400033,    /* ntl_s1 */
    0x6033,      /* or */
    0x28705013,  /* orc_b */
    0x6013,      /* ori */
    0x40006033,  /* orn */
    0x8004033,   /* pack */
    0x8007033,   /* packh */
    0xffff_ffff, /* packw */
    0x100000f,   /* pause */
    0x6013,      /* prefetch_i */
    0x106013,    /* prefetch_r */
    0x306013,    /* prefetch_w */
    0xc0002073,  /* rdcycle */
    0xc8002073,  /* rdcycleh */
    0xc0202073,  /* rdinstret */
    0xc8202073,  /* rdinstreth */
    0xc0102073,  /* rdtime */
    0xc8102073,  /* rdtimeh */
    0x2006033,   /* rem */
    0x2007033,   /* remu */
    0xffff_ffff, /* remuw */
    0xffff_ffff, /* remw */
    0x8067,      /* ret */
    0xffff_ffff, /* rev8 */
    0x69805013,  /* rev8_rv32 */
    0x60001033,  /* rol */
    0xffff_ffff, /* rolw */
    0x60005033,  /* ror */
    0xffff_ffff, /* rori */
    0x60005013,  /* rori_rv32 */
    0xffff_ffff, /* roriw */
    0xffff_ffff, /* rorw */
    0x23,        /* sb */
    0x100073,    /* sbreak */
    0xffff_ffff, /* sc_d */
    0x1800202f,  /* sc_w */
    0x73,        /* scall */
    0x10400073,  /* sctrclr */
    0xffff_ffff, /* sd */
    0x103013,    /* seqz */
    0x60401013,  /* sext_b */
    0x60501013,  /* sext_h */
    0xffff_ffff, /* sext_w */
    0x18100073,  /* sfence_inval_ir */
    0x12000073,  /* sfence_vma */
    0x18000073,  /* sfence_w_inval */
    0x2033,      /* sgtz */
    0x1023,      /* sh */
    0x20002033,  /* sh1add */
    0xffff_ffff, /* sh1add_uw */
    0x20004033,  /* sh2add */
    0xffff_ffff, /* sh2add_uw */
    0x20006033,  /* sh3add */
    0xffff_ffff, /* sh3add_uw */
    0x10201013,  /* sha256sig0 */
    0x10301013,  /* sha256sig1 */
    0x10001013,  /* sha256sum0 */
    0x10101013,  /* sha256sum1 */
    0xffff_ffff, /* sha512sig0 */
    0x5c000033,  /* sha512sig0h */
    0x54000033,  /* sha512sig0l */
    0xffff_ffff, /* sha512sig1 */
    0x5e000033,  /* sha512sig1h */
    0x56000033,  /* sha512sig1l */
    0xffff_ffff, /* sha512sum0 */
    0x50000033,  /* sha512sum0r */
    0xffff_ffff, /* sha512sum1 */
    0x52000033,  /* sha512sum1r */
    0x16000073,  /* sinval_vma */
    0x1033,      /* sll */
    0x1013,      /* slli */
    0x1013,      /* slli_rv32 */
    0xffff_ffff, /* slli_uw */
    0xffff_ffff, /* slliw */
    0xffff_ffff, /* sllw */
    0x2033,      /* slt */
    0x2013,      /* slti */
    0x3013,      /* sltiu */
    0x3033,      /* sltu */
    0x2033,      /* sltz */
    0x10801013,  /* sm3p0 */
    0x10901013,  /* sm3p1 */
    0x30000033,  /* sm4ed */
    0x34000033,  /* sm4ks */
    0x3033,      /* snez */
    0x40005033,  /* sra */
    0x40005013,  /* srai */
    0x40005013,  /* srai_rv32 */
    0xffff_ffff, /* sraiw */
    0xffff_ffff, /* sraw */
    0x10200073,  /* sret */
    0x5033,      /* srl */
    0x5013,      /* srli */
    0x5013,      /* srli_rv32 */
    0xffff_ffff, /* srliw */
    0xffff_ffff, /* srlw */
    0xffff_ffff, /* ssamoswap_d */
    0x4800202f,  /* ssamoswap_w */
    0xcdc0c073,  /* sspopchk_x1 */
    0xcdc2c073,  /* sspopchk_x5 */
    0xce104073,  /* sspush_x1 */
    0xce504073,  /* sspush_x5 */
    0xcdc04073,  /* ssrdp */
    0x40000033,  /* sub */
    0xffff_ffff, /* subw */
    0x2023,      /* sw */
    0x8f05013,   /* unzip */
    0x24002057,  /* vaadd_vv */
    0x24006057,  /* vaadd_vx */
    0x20002057,  /* vaaddu_vv */
    0x20006057,  /* vaaddu_vx */
    0x40003057,  /* vadc_vim */
    0x40000057,  /* vadc_vvm */
    0x40004057,  /* vadc_vxm */
    0x3057,      /* vadd_vi */
    0x57,        /* vadd_vv */
    0x4057,      /* vadd_vx */
    0xa600a077,  /* vaesdf_vs */
    0xa200a077,  /* vaesdf_vv */
    0xa6002077,  /* vaesdm_vs */
    0xa2002077,  /* vaesdm_vv */
    0xa601a077,  /* vaesef_vs */
    0xa201a077,  /* vaesef_vv */
    0xa6012077,  /* vaesem_vs */
    0xa2012077,  /* vaesem_vv */
    0x8a002077,  /* vaeskf1_vi */
    0xaa002077,  /* vaeskf2_vi */
    0xa603a077,  /* vaesz_vs */
    0x24003057,  /* vand_vi */
    0x24000057,  /* vand_vv */
    0x24004057,  /* vand_vx */
    0x4000057,   /* vandn_vv */
    0x4004057,   /* vandn_vx */
    0x2c002057,  /* vasub_vv */
    0x2c006057,  /* vasub_vx */
    0x28002057,  /* vasubu_vv */
    0x28006057,  /* vasubu_vx */
    0x48042057,  /* vbrev8_v */
    0x48052057,  /* vbrev_v */
    0x30002057,  /* vclmul_vv */
    0x30006057,  /* vclmul_vx */
    0x34002057,  /* vclmulh_vv */
    0x34006057,  /* vclmulh_vx */
    0x48062057,  /* vclz_v */
    0x5e002057,  /* vcompress_vm */
    0x40082057,  /* vcpop_m */
    0x48072057,  /* vcpop_v */
    0x4806a057,  /* vctz_v */
    0x84002057,  /* vdiv_vv */
    0x84006057,  /* vdiv_vx */
    0x80002057,  /* vdivu_vv */
    0x80006057,  /* vdivu_vx */
    0x5057,      /* vfadd_vf */
    0x1057,      /* vfadd_vv */
    0x4c081057,  /* vfclass_v */
    0x48019057,  /* vfcvt_f_x_v */
    0x48011057,  /* vfcvt_f_xu_v */
    0x48039057,  /* vfcvt_rtz_x_f_v */
    0x48031057,  /* vfcvt_rtz_xu_f_v */
    0x48009057,  /* vfcvt_x_f_v */
    0x48001057,  /* vfcvt_xu_f_v */
    0x80005057,  /* vfdiv_vf */
    0x80001057,  /* vfdiv_vv */
    0x4008a057,  /* vfirst_m */
    0xb0005057,  /* vfmacc_vf */
    0xb0001057,  /* vfmacc_vv */
    0xa0005057,  /* vfmadd_vf */
    0xa0001057,  /* vfmadd_vv */
    0x18005057,  /* vfmax_vf */
    0x18001057,  /* vfmax_vv */
    0x5c005057,  /* vfmerge_vfm */
    0x10005057,  /* vfmin_vf */
    0x10001057,  /* vfmin_vv */
    0xb8005057,  /* vfmsac_vf */
    0xb8001057,  /* vfmsac_vv */
    0xa8005057,  /* vfmsub_vf */
    0xa8001057,  /* vfmsub_vv */
    0x90005057,  /* vfmul_vf */
    0x90001057,  /* vfmul_vv */
    0x42001057,  /* vfmv_f_s */
    0x42005057,  /* vfmv_s_f */
    0x5e005057,  /* vfmv_v_f */
    0x480a1057,  /* vfncvt_f_f_w */
    0x48099057,  /* vfncvt_f_x_w */
    0x48091057,  /* vfncvt_f_xu_w */
    0x480a9057,  /* vfncvt_rod_f_f_w */
    0x480b9057,  /* vfncvt_rtz_x_f_w */
    0x480b1057,  /* vfncvt_rtz_xu_f_w */
    0x48089057,  /* vfncvt_x_f_w */
    0x48081057,  /* vfncvt_xu_f_w */
    0x480e9057,  /* vfncvtbf16_f_f_w */
    0xb4005057,  /* vfnmacc_vf */
    0xb4001057,  /* vfnmacc_vv */
    0xa4005057,  /* vfnmadd_vf */
    0xa4001057,  /* vfnmadd_vv */
    0xbc005057,  /* vfnmsac_vf */
    0xbc001057,  /* vfnmsac_vv */
    0xac005057,  /* vfnmsub_vf */
    0xac001057,  /* vfnmsub_vv */
    0x84005057,  /* vfrdiv_vf */
    0x4c029057,  /* vfrec7_v */
    0x1c001057,  /* vfredmax_vs */
    0x14001057,  /* vfredmin_vs */
    0xc001057,   /* vfredosum_vs */
    0x4001057,   /* vfredsum_vs */
    0x4001057,   /* vfredusum_vs */
    0x4c021057,  /* vfrsqrt7_v */
    0x9c005057,  /* vfrsub_vf */
    0x20005057,  /* vfsgnj_vf */
    0x20001057,  /* vfsgnj_vv */
    0x24005057,  /* vfsgnjn_vf */
    0x24001057,  /* vfsgnjn_vv */
    0x28005057,  /* vfsgnjx_vf */
    0x28001057,  /* vfsgnjx_vv */
    0x3c005057,  /* vfslide1down_vf */
    0x38005057,  /* vfslide1up_vf */
    0x4c001057,  /* vfsqrt_v */
    0x8005057,   /* vfsub_vf */
    0x8001057,   /* vfsub_vv */
    0xc0005057,  /* vfwadd_vf */
    0xc0001057,  /* vfwadd_vv */
    0xd0005057,  /* vfwadd_wf */
    0xd0001057,  /* vfwadd_wv */
    0x48061057,  /* vfwcvt_f_f_v */
    0x48059057,  /* vfwcvt_f_x_v */
    0x48051057,  /* vfwcvt_f_xu_v */
    0x48079057,  /* vfwcvt_rtz_x_f_v */
    0x48071057,  /* vfwcvt_rtz_xu_f_v */
    0x48049057,  /* vfwcvt_x_f_v */
    0x48041057,  /* vfwcvt_xu_f_v */
    0x48069057,  /* vfwcvtbf16_f_f_v */
    0xf0005057,  /* vfwmacc_vf */
    0xf0001057,  /* vfwmacc_vv */
    0xec005057,  /* vfwmaccbf16_vf */
    0xec001057,  /* vfwmaccbf16_vv */
    0xf8005057,  /* vfwmsac_vf */
    0xf8001057,  /* vfwmsac_vv */
    0xe0005057,  /* vfwmul_vf */
    0xe0001057,  /* vfwmul_vv */
    0xf4005057,  /* vfwnmacc_vf */
    0xf4001057,  /* vfwnmacc_vv */
    0xfc005057,  /* vfwnmsac_vf */
    0xfc001057,  /* vfwnmsac_vv */
    0xcc001057,  /* vfwredosum_vs */
    0xc4001057,  /* vfwredsum_vs */
    0xc4001057,  /* vfwredusum_vs */
    0xc8005057,  /* vfwsub_vf */
    0xc8001057,  /* vfwsub_vv */
    0xd8005057,  /* vfwsub_wf */
    0xd8001057,  /* vfwsub_wv */
    0xb2002077,  /* vghsh_vv */
    0xa208a077,  /* vgmul_vv */
    0x5008a057,  /* vid_v */
    0x50082057,  /* viota_m */
    0x2800007,   /* vl1r_v */
    0x2805007,   /* vl1re16_v */
    0x2806007,   /* vl1re32_v */
    0x2807007,   /* vl1re64_v */
    0x2800007,   /* vl1re8_v */
    0x22800007,  /* vl2r_v */
    0x22805007,  /* vl2re16_v */
    0x22806007,  /* vl2re32_v */
    0x22807007,  /* vl2re64_v */
    0x22800007,  /* vl2re8_v */
    0x62800007,  /* vl4r_v */
    0x62805007,  /* vl4re16_v */
    0x62806007,  /* vl4re32_v */
    0x62807007,  /* vl4re64_v */
    0x62800007,  /* vl4re8_v */
    0xe2800007,  /* vl8r_v */
    0xe2805007,  /* vl8re16_v */
    0xe2806007,  /* vl8re32_v */
    0xe2807007,  /* vl8re64_v */
    0xe2800007,  /* vl8re8_v */
    0x5007,      /* vle16_v */
    0x1005007,   /* vle16ff_v */
    0x2b00007,   /* vle1_v */
    0x6007,      /* vle32_v */
    0x1006007,   /* vle32ff_v */
    0x7007,      /* vle64_v */
    0x1007007,   /* vle64ff_v */
    0x7,         /* vle8_v */
    0x1000007,   /* vle8ff_v */
    0x2b00007,   /* vlm_v */
    0xc005007,   /* vloxei16_v */
    0xc006007,   /* vloxei32_v */
    0xc007007,   /* vloxei64_v */
    0xc000007,   /* vloxei8_v */
    0x8005007,   /* vlse16_v */
    0x8006007,   /* vlse32_v */
    0x8007007,   /* vlse64_v */
    0x8000007,   /* vlse8_v */
    0x4005007,   /* vluxei16_v */
    0x4006007,   /* vluxei32_v */
    0x4007007,   /* vluxei64_v */
    0x4000007,   /* vluxei8_v */
    0xb4002057,  /* vmacc_vv */
    0xb4006057,  /* vmacc_vx */
    0x46003057,  /* vmadc_vi */
    0x44003057,  /* vmadc_vim */
    0x46000057,  /* vmadc_vv */
    0x44000057,  /* vmadc_vvm */
    0x46004057,  /* vmadc_vx */
    0x44004057,  /* vmadc_vxm */
    0xa4002057,  /* vmadd_vv */
    0xa4006057,  /* vmadd_vx */
    0x66002057,  /* vmand_mm */
    0x62002057,  /* vmandn_mm */
    0x60002057,  /* vmandnot_mm */
    0x1c000057,  /* vmax_vv */
    0x1c004057,  /* vmax_vx */
    0x18000057,  /* vmaxu_vv */
    0x18004057,  /* vmaxu_vx */
    0x5c003057,  /* vmerge_vim */
    0x5c000057,  /* vmerge_vvm */
    0x5c004057,  /* vmerge_vxm */
    0x60005057,  /* vmfeq_vf */
    0x60001057,  /* vmfeq_vv */
    0x7c005057,  /* vmfge_vf */
    0x74005057,  /* vmfgt_vf */
    0x64005057,  /* vmfle_vf */
    0x64001057,  /* vmfle_vv */
    0x6c005057,  /* vmflt_vf */
    0x6c001057,  /* vmflt_vv */
    0x70005057,  /* vmfne_vf */
    0x70001057,  /* vmfne_vv */
    0x14000057,  /* vmin_vv */
    0x14004057,  /* vmin_vx */
    0x10000057,  /* vminu_vv */
    0x10004057,  /* vminu_vx */
    0x76002057,  /* vmnand_mm */
    0x7a002057,  /* vmnor_mm */
    0x6a002057,  /* vmor_mm */
    0x72002057,  /* vmorn_mm */
    0x70002057,  /* vmornot_mm */
    0x4e000057,  /* vmsbc_vv */
    0x4c000057,  /* vmsbc_vvm */
    0x4e004057,  /* vmsbc_vx */
    0x4c004057,  /* vmsbc_vxm */
    0x5000a057,  /* vmsbf_m */
    0x60003057,  /* vmseq_vi */
    0x60000057,  /* vmseq_vv */
    0x60004057,  /* vmseq_vx */
    0x7c003057,  /* vmsgt_vi */
    0x7c004057,  /* vmsgt_vx */
    0x78003057,  /* vmsgtu_vi */
    0x78004057,  /* vmsgtu_vx */
    0x5001a057,  /* vmsif_m */
    0x74003057,  /* vmsle_vi */
    0x74000057,  /* vmsle_vv */
    0x74004057,  /* vmsle_vx */
    0x70003057,  /* vmsleu_vi */
    0x70000057,  /* vmsleu_vv */
    0x70004057,  /* vmsleu_vx */
    0x6c000057,  /* vmslt_vv */
    0x6c004057,  /* vmslt_vx */
    0x68000057,  /* vmsltu_vv */
    0x68004057,  /* vmsltu_vx */
    0x64003057,  /* vmsne_vi */
    0x64000057,  /* vmsne_vv */
    0x64004057,  /* vmsne_vx */
    0x50012057,  /* vmsof_m */
    0x94002057,  /* vmul_vv */
    0x94006057,  /* vmul_vx */
    0x9c002057,  /* vmulh_vv */
    0x9c006057,  /* vmulh_vx */
    0x98002057,  /* vmulhsu_vv */
    0x98006057,  /* vmulhsu_vx */
    0x90002057,  /* vmulhu_vv */
    0x90006057,  /* vmulhu_vx */
    0x9e003057,  /* vmv1r_v */
    0x9e00b057,  /* vmv2r_v */
    0x9e01b057,  /* vmv4r_v */
    0x9e03b057,  /* vmv8r_v */
    0x42006057,  /* vmv_s_x */
    0x5e003057,  /* vmv_v_i */
    0x5e000057,  /* vmv_v_v */
    0x5e004057,  /* vmv_v_x */
    0x42002057,  /* vmv_x_s */
    0x7e002057,  /* vmxnor_mm */
    0x6e002057,  /* vmxor_mm */
    0xbc003057,  /* vnclip_wi */
    0xbc000057,  /* vnclip_wv */
    0xbc004057,  /* vnclip_wx */
    0xb8003057,  /* vnclipu_wi */
    0xb8000057,  /* vnclipu_wv */
    0xb8004057,  /* vnclipu_wx */
    0xbc002057,  /* vnmsac_vv */
    0xbc006057,  /* vnmsac_vx */
    0xac002057,  /* vnmsub_vv */
    0xac006057,  /* vnmsub_vx */
    0xb4003057,  /* vnsra_wi */
    0xb4000057,  /* vnsra_wv */
    0xb4004057,  /* vnsra_wx */
    0xb0003057,  /* vnsrl_wi */
    0xb0000057,  /* vnsrl_wv */
    0xb0004057,  /* vnsrl_wx */
    0x28003057,  /* vor_vi */
    0x28000057,  /* vor_vv */
    0x28004057,  /* vor_vx */
    0x40082057,  /* vpopc_m */
    0x4002057,   /* vredand_vs */
    0x1c002057,  /* vredmax_vs */
    0x18002057,  /* vredmaxu_vs */
    0x14002057,  /* vredmin_vs */
    0x10002057,  /* vredminu_vs */
    0x8002057,   /* vredor_vs */
    0x2057,      /* vredsum_vs */
    0xc002057,   /* vredxor_vs */
    0x8c002057,  /* vrem_vv */
    0x8c006057,  /* vrem_vx */
    0x88002057,  /* vremu_vv */
    0x88006057,  /* vremu_vx */
    0x4804a057,  /* vrev8_v */
    0x30003057,  /* vrgather_vi */
    0x30000057,  /* vrgather_vv */
    0x30004057,  /* vrgather_vx */
    0x38000057,  /* vrgatherei16_vv */
    0x54000057,  /* vrol_vv */
    0x54004057,  /* vrol_vx */
    0x50003057,  /* vror_vi */
    0x50000057,  /* vror_vv */
    0x50004057,  /* vror_vx */
    0xc003057,   /* vrsub_vi */
    0xc004057,   /* vrsub_vx */
    0x2800027,   /* vs1r_v */
    0x22800027,  /* vs2r_v */
    0x62800027,  /* vs4r_v */
    0xe2800027,  /* vs8r_v */
    0x84003057,  /* vsadd_vi */
    0x84000057,  /* vsadd_vv */
    0x84004057,  /* vsadd_vx */
    0x80003057,  /* vsaddu_vi */
    0x80000057,  /* vsaddu_vv */
    0x80004057,  /* vsaddu_vx */
    0x48000057,  /* vsbc_vvm */
    0x48004057,  /* vsbc_vxm */
    0x5027,      /* vse16_v */
    0x2b00027,   /* vse1_v */
    0x6027,      /* vse32_v */
    0x7027,      /* vse64_v */
    0x27,        /* vse8_v */
    0xc0007057,  /* vsetivli */
    0x80007057,  /* vsetvl */
    0x7057,      /* vsetvli */
    0x4803a057,  /* vsext_vf2 */
    0x4802a057,  /* vsext_vf4 */
    0x4801a057,  /* vsext_vf8 */
    0xba002077,  /* vsha2ch_vv */
    0xbe002077,  /* vsha2cl_vv */
    0xb6002077,  /* vsha2ms_vv */
    0x3c006057,  /* vslide1down_vx */
    0x38006057,  /* vslide1up_vx */
    0x3c003057,  /* vslidedown_vi */
    0x3c004057,  /* vslidedown_vx */
    0x38003057,  /* vslideup_vi */
    0x38004057,  /* vslideup_vx */
    0x94003057,  /* vsll_vi */
    0x94000057,  /* vsll_vv */
    0x94004057,  /* vsll_vx */
    0xae002077,  /* vsm3c_vi */
    0x82002077,  /* vsm3me_vv */
    0x86002077,  /* vsm4k_vi */
    0xa6082077,  /* vsm4r_vs */
    0xa2082077,  /* vsm4r_vv */
    0x2b00027,   /* vsm_v */
    0x9c000057,  /* vsmul_vv */
    0x9c004057,  /* vsmul_vx */
    0xc005027,   /* vsoxei16_v */
    0xc006027,   /* vsoxei32_v */
    0xc007027,   /* vsoxei64_v */
    0xc000027,   /* vsoxei8_v */
    0xa4003057,  /* vsra_vi */
    0xa4000057,  /* vsra_vv */
    0xa4004057,  /* vsra_vx */
    0xa0003057,  /* vsrl_vi */
    0xa0000057,  /* vsrl_vv */
    0xa0004057,  /* vsrl_vx */
    0x8005027,   /* vsse16_v */
    0x8006027,   /* vsse32_v */
    0x8007027,   /* vsse64_v */
    0x8000027,   /* vsse8_v */
    0xac003057,  /* vssra_vi */
    0xac000057,  /* vssra_vv */
    0xac004057,  /* vssra_vx */
    0xa8003057,  /* vssrl_vi */
    0xa8000057,  /* vssrl_vv */
    0xa8004057,  /* vssrl_vx */
    0x8c000057,  /* vssub_vv */
    0x8c004057,  /* vssub_vx */
    0x88000057,  /* vssubu_vv */
    0x88004057,  /* vssubu_vx */
    0x8000057,   /* vsub_vv */
    0x8004057,   /* vsub_vx */
    0x4005027,   /* vsuxei16_v */
    0x4006027,   /* vsuxei32_v */
    0x4007027,   /* vsuxei64_v */
    0x4000027,   /* vsuxei8_v */
    0xc4002057,  /* vwadd_vv */
    0xc4006057,  /* vwadd_vx */
    0xd4002057,  /* vwadd_wv */
    0xd4006057,  /* vwadd_wx */
    0xc0002057,  /* vwaddu_vv */
    0xc0006057,  /* vwaddu_vx */
    0xd0002057,  /* vwaddu_wv */
    0xd0006057,  /* vwaddu_wx */
    0xf4002057,  /* vwmacc_vv */
    0xf4006057,  /* vwmacc_vx */
    0xfc002057,  /* vwmaccsu_vv */
    0xfc006057,  /* vwmaccsu_vx */
    0xf0002057,  /* vwmaccu_vv */
    0xf0006057,  /* vwmaccu_vx */
    0xf8006057,  /* vwmaccus_vx */
    0xec002057,  /* vwmul_vv */
    0xec006057,  /* vwmul_vx */
    0xe8002057,  /* vwmulsu_vv */
    0xe8006057,  /* vwmulsu_vx */
    0xe0002057,  /* vwmulu_vv */
    0xe0006057,  /* vwmulu_vx */
    0xc4000057,  /* vwredsum_vs */
    0xc0000057,  /* vwredsumu_vs */
    0xd4003057,  /* vwsll_vi */
    0xd4000057,  /* vwsll_vv */
    0xd4004057,  /* vwsll_vx */
    0xcc002057,  /* vwsub_vv */
    0xcc006057,  /* vwsub_vx */
    0xdc002057,  /* vwsub_wv */
    0xdc006057,  /* vwsub_wx */
    0xc8002057,  /* vwsubu_vv */
    0xc8006057,  /* vwsubu_vx */
    0xd8002057,  /* vwsubu_wv */
    0xd8006057,  /* vwsubu_wx */
    0x2c003057,  /* vxor_vi */
    0x2c000057,  /* vxor_vv */
    0x2c004057,  /* vxor_vx */
    0x48032057,  /* vzext_vf2 */
    0x48022057,  /* vzext_vf4 */
    0x48012057,  /* vzext_vf8 */
    0x10500073,  /* wfi */
    0xd00073,    /* wrs_nto */
    0x1d00073,   /* wrs_sto */
    0x40004033,  /* xnor */
    0x4033,      /* xor */
    0x4013,      /* xori */
    0x28002033,  /* xperm4 */
    0x28004033,  /* xperm8 */
    0xff07013,   /* zext_b */
    0xffff_ffff, /* zext_h */
    0x8004033,   /* zext_h_rv32 */
    0xffff_ffff, /* zext_w */
    0x8f01013,   /* zip */
];
pub static OPCODE32_MASK: [u32; 1039] = [
    0xfe00707f,  /* add */
    0xffff_ffff, /* add_uw */
    0x707f,      /* addi */
    0xffff_ffff, /* addiw */
    0xffff_ffff, /* addw */
    0x3e00707f,  /* aes32dsi */
    0x3e00707f,  /* aes32dsmi */
    0x3e00707f,  /* aes32esi */
    0x3e00707f,  /* aes32esmi */
    0xffff_ffff, /* aes64ds */
    0xffff_ffff, /* aes64dsm */
    0xffff_ffff, /* aes64es */
    0xffff_ffff, /* aes64esm */
    0xffff_ffff, /* aes64im */
    0xffff_ffff, /* aes64ks1i */
    0xffff_ffff, /* aes64ks2 */
    0xf800707f,  /* amoadd_b */
    0xffff_ffff, /* amoadd_d */
    0xf800707f,  /* amoadd_h */
    0xf800707f,  /* amoadd_w */
    0xf800707f,  /* amoand_b */
    0xffff_ffff, /* amoand_d */
    0xf800707f,  /* amoand_h */
    0xf800707f,  /* amoand_w */
    0xf800707f,  /* amocas_b */
    0xf800707f,  /* amocas_d */
    0xf800707f,  /* amocas_h */
    0xffff_ffff, /* amocas_q */
    0xf800707f,  /* amocas_w */
    0xf800707f,  /* amomax_b */
    0xffff_ffff, /* amomax_d */
    0xf800707f,  /* amomax_h */
    0xf800707f,  /* amomax_w */
    0xf800707f,  /* amomaxu_b */
    0xffff_ffff, /* amomaxu_d */
    0xf800707f,  /* amomaxu_h */
    0xf800707f,  /* amomaxu_w */
    0xf800707f,  /* amomin_b */
    0xffff_ffff, /* amomin_d */
    0xf800707f,  /* amomin_h */
    0xf800707f,  /* amomin_w */
    0xf800707f,  /* amominu_b */
    0xffff_ffff, /* amominu_d */
    0xf800707f,  /* amominu_h */
    0xf800707f,  /* amominu_w */
    0xf800707f,  /* amoor_b */
    0xffff_ffff, /* amoor_d */
    0xf800707f,  /* amoor_h */
    0xf800707f,  /* amoor_w */
    0xf800707f,  /* amoswap_b */
    0xffff_ffff, /* amoswap_d */
    0xf800707f,  /* amoswap_h */
    0xf800707f,  /* amoswap_w */
    0xf800707f,  /* amoxor_b */
    0xffff_ffff, /* amoxor_d */
    0xf800707f,  /* amoxor_h */
    0xf800707f,  /* amoxor_w */
    0xfe00707f,  /* and */
    0x707f,      /* andi */
    0xfe00707f,  /* andn */
    0x7f,        /* auipc */
    0xfe00707f,  /* bclr */
    0xffff_ffff, /* bclri */
    0xfe00707f,  /* bclri_rv32 */
    0x707f,      /* beq */
    0x1f0707f,   /* beqz */
    0xfe00707f,  /* bext */
    0xffff_ffff, /* bexti */
    0xfe00707f,  /* bexti_rv32 */
    0x707f,      /* bge */
    0x707f,      /* bgeu */
    0x1f0707f,   /* bgez */
    0x707f,      /* bgt */
    0x707f,      /* bgtu */
    0xff07f,     /* bgtz */
    0xfe00707f,  /* binv */
    0xffff_ffff, /* binvi */
    0xfe00707f,  /* binvi_rv32 */
    0x707f,      /* ble */
    0x707f,      /* bleu */
    0xff07f,     /* blez */
    0x707f,      /* blt */
    0x707f,      /* bltu */
    0x1f0707f,   /* bltz */
    0x707f,      /* bne */
    0x1f0707f,   /* bnez */
    0xfff0707f,  /* brev8 */
    0xfe00707f,  /* bset */
    0xffff_ffff, /* bseti */
    0xfe00707f,  /* bseti_rv32 */
    0xf003,      /* c_add */
    0xe003,      /* c_addi */
    0xef83,      /* c_addi16sp */
    0xe003,      /* c_addi4spn */
    0xffff_ffff, /* c_addiw */
    0xffff_ffff, /* c_addw */
    0xfc63,      /* c_and */
    0xec03,      /* c_andi */
    0xe003,      /* c_beqz */
    0xe003,      /* c_bnez */
    0xffff,      /* c_ebreak */
    0xe003,      /* c_fld */
    0xe003,      /* c_fldsp */
    0xe003,      /* c_flw */
    0xe003,      /* c_flwsp */
    0xe003,      /* c_fsd */
    0xe003,      /* c_fsdsp */
    0xe003,      /* c_fsw */
    0xe003,      /* c_fswsp */
    0xe003,      /* c_j */
    0xe003,      /* c_jal */
    0xf07f,      /* c_jalr */
    0xf07f,      /* c_jr */
    0xfc03,      /* c_lbu */
    0xe003,      /* c_ld */
    0xe003,      /* c_ldsp */
    0xfc43,      /* c_lh */
    0xfc43,      /* c_lhu */
    0xe003,      /* c_li */
    0xe003,      /* c_lui */
    0xe003,      /* c_lw */
    0xe003,      /* c_lwsp */
    0xffff,      /* c_mop_1 */
    0xffff,      /* c_mop_11 */
    0xffff,      /* c_mop_13 */
    0xffff,      /* c_mop_15 */
    0xffff,      /* c_mop_3 */
    0xffff,      /* c_mop_5 */
    0xffff,      /* c_mop_7 */
    0xffff,      /* c_mop_9 */
    0xf8ff,      /* c_mop_N */
    0xfc63,      /* c_mul */
    0xf003,      /* c_mv */
    0xef83,      /* c_nop */
    0xfc7f,      /* c_not */
    0xffff,      /* c_ntl_all */
    0xffff,      /* c_ntl_p1 */
    0xffff,      /* c_ntl_pall */
    0xffff,      /* c_ntl_s1 */
    0xfc63,      /* c_or */
    0xfc03,      /* c_sb */
    0xe003,      /* c_sd */
    0xe003,      /* c_sdsp */
    0xfc7f,      /* c_sext_b */
    0xfc7f,      /* c_sext_h */
    0xffff_ffff, /* c_sext_w */
    0xfc43,      /* c_sh */
    0xe003,      /* c_slli */
    0xf003,      /* c_slli_rv32 */
    0xec03,      /* c_srai */
    0xfc03,      /* c_srai_rv32 */
    0xec03,      /* c_srli */
    0xfc03,      /* c_srli_rv32 */
    0xffff,      /* c_sspopchk_x5 */
    0xffff,      /* c_sspush_x1 */
    0xfc63,      /* c_sub */
    0xffff_ffff, /* c_subw */
    0xe003,      /* c_sw */
    0xe003,      /* c_swsp */
    0xfc63,      /* c_xor */
    0xfc7f,      /* c_zext_b */
    0xfc7f,      /* c_zext_h */
    0xffff_ffff, /* c_zext_w */
    0xfff07fff,  /* cbo_clean */
    0xfff07fff,  /* cbo_flush */
    0xfff07fff,  /* cbo_inval */
    0xfff07fff,  /* cbo_zero */
    0xfe00707f,  /* clmul */
    0xfe00707f,  /* clmulh */
    0xfe00707f,  /* clmulr */
    0xfff0707f,  /* clz */
    0xffff_ffff, /* clzw */
    0xfc03,      /* cm_jalt */
    0xfc63,      /* cm_mva01s */
    0xfc63,      /* cm_mvsa01 */
    0xff03,      /* cm_pop */
    0xff03,      /* cm_popret */
    0xff03,      /* cm_popretz */
    0xff03,      /* cm_push */
    0xfff0707f,  /* cpop */
    0xffff_ffff, /* cpopw */
    0x7fff,      /* csrc */
    0x7fff,      /* csrci */
    0xff07f,     /* csrr */
    0x707f,      /* csrrc */
    0x707f,      /* csrrci */
    0x707f,      /* csrrs */
    0x707f,      /* csrrsi */
    0x707f,      /* csrrw */
    0x707f,      /* csrrwi */
    0x7fff,      /* csrs */
    0x7fff,      /* csrsi */
    0x7fff,      /* csrw */
    0x7fff,      /* csrwi */
    0xfff0707f,  /* ctz */
    0xffff_ffff, /* ctzw */
    0xfe00707f,  /* czero_eqz */
    0xfe00707f,  /* czero_nez */
    0xfe00707f,  /* div */
    0xfe00707f,  /* divu */
    0xffff_ffff, /* divuw */
    0xffff_ffff, /* divw */
    0xffffffff,  /* dret */
    0xffffffff,  /* ebreak */
    0xffffffff,  /* ecall */
    0xfe00707f,  /* fabs_d */
    0xfe00707f,  /* fabs_h */
    0xfe00707f,  /* fabs_q */
    0xfe00707f,  /* fabs_s */
    0xfe00007f,  /* fadd_d */
    0xfe00007f,  /* fadd_h */
    0xfe00007f,  /* fadd_q */
    0xfe00007f,  /* fadd_s */
    0xfff0707f,  /* fclass_d */
    0xfff0707f,  /* fclass_h */
    0xfff0707f,  /* fclass_q */
    0xfff0707f,  /* fclass_s */
    0xfff0007f,  /* fcvt_bf16_s */
    0xfff0007f,  /* fcvt_d_h */
    0xffff_ffff, /* fcvt_d_l */
    0xffff_ffff, /* fcvt_d_lu */
    0xfff0007f,  /* fcvt_d_q */
    0xfff0007f,  /* fcvt_d_s */
    0xfff0007f,  /* fcvt_d_w */
    0xfff0007f,  /* fcvt_d_wu */
    0xfff0007f,  /* fcvt_h_d */
    0xffff_ffff, /* fcvt_h_l */
    0xffff_ffff, /* fcvt_h_lu */
    0xfff0007f,  /* fcvt_h_q */
    0xfff0007f,  /* fcvt_h_s */
    0xfff0007f,  /* fcvt_h_w */
    0xfff0007f,  /* fcvt_h_wu */
    0xffff_ffff, /* fcvt_l_d */
    0xffff_ffff, /* fcvt_l_h */
    0xffff_ffff, /* fcvt_l_q */
    0xffff_ffff, /* fcvt_l_s */
    0xffff_ffff, /* fcvt_lu_d */
    0xffff_ffff, /* fcvt_lu_h */
    0xffff_ffff, /* fcvt_lu_q */
    0xffff_ffff, /* fcvt_lu_s */
    0xfff0007f,  /* fcvt_q_d */
    0xfff0007f,  /* fcvt_q_h */
    0xffff_ffff, /* fcvt_q_l */
    0xffff_ffff, /* fcvt_q_lu */
    0xfff0007f,  /* fcvt_q_s */
    0xfff0007f,  /* fcvt_q_w */
    0xfff0007f,  /* fcvt_q_wu */
    0xfff0007f,  /* fcvt_s_bf16 */
    0xfff0007f,  /* fcvt_s_d */
    0xfff0007f,  /* fcvt_s_h */
    0xffff_ffff, /* fcvt_s_l */
    0xffff_ffff, /* fcvt_s_lu */
    0xfff0007f,  /* fcvt_s_q */
    0xfff0007f,  /* fcvt_s_w */
    0xfff0007f,  /* fcvt_s_wu */
    0xfff0007f,  /* fcvt_w_d */
    0xfff0007f,  /* fcvt_w_h */
    0xfff0007f,  /* fcvt_w_q */
    0xfff0007f,  /* fcvt_w_s */
    0xfff0007f,  /* fcvt_wu_d */
    0xfff0007f,  /* fcvt_wu_h */
    0xfff0007f,  /* fcvt_wu_q */
    0xfff0007f,  /* fcvt_wu_s */
    0xfff0707f,  /* fcvtmod_w_d */
    0xfe00007f,  /* fdiv_d */
    0xfe00007f,  /* fdiv_h */
    0xfe00007f,  /* fdiv_q */
    0xfe00007f,  /* fdiv_s */
    0x707f,      /* fence */
    0x707f,      /* fence_i */
    0xfff0707f,  /* fence_tso */
    0xfe00707f,  /* feq_d */
    0xfe00707f,  /* feq_h */
    0xfe00707f,  /* feq_q */
    0xfe00707f,  /* feq_s */
    0x707f,      /* fld */
    0xfe00707f,  /* fle_d */
    0xfe00707f,  /* fle_h */
    0xfe00707f,  /* fle_q */
    0xfe00707f,  /* fle_s */
    0xfe00707f,  /* fleq_d */
    0xfe00707f,  /* fleq_h */
    0xfe00707f,  /* fleq_q */
    0xfe00707f,  /* fleq_s */
    0x707f,      /* flh */
    0xfff0707f,  /* fli_d */
    0xfff0707f,  /* fli_h */
    0xfff0707f,  /* fli_q */
    0xfff0707f,  /* fli_s */
    0x707f,      /* flq */
    0xfe00707f,  /* flt_d */
    0xfe00707f,  /* flt_h */
    0xfe00707f,  /* flt_q */
    0xfe00707f,  /* flt_s */
    0xfe00707f,  /* fltq_d */
    0xfe00707f,  /* fltq_h */
    0xfe00707f,  /* fltq_q */
    0xfe00707f,  /* fltq_s */
    0x707f,      /* flw */
    0x600007f,   /* fmadd_d */
    0x600007f,   /* fmadd_h */
    0x600007f,   /* fmadd_q */
    0x600007f,   /* fmadd_s */
    0xfe00707f,  /* fmax_d */
    0xfe00707f,  /* fmax_h */
    0xfe00707f,  /* fmax_q */
    0xfe00707f,  /* fmax_s */
    0xfe00707f,  /* fmaxm_d */
    0xfe00707f,  /* fmaxm_h */
    0xfe00707f,  /* fmaxm_q */
    0xfe00707f,  /* fmaxm_s */
    0xfe00707f,  /* fmin_d */
    0xfe00707f,  /* fmin_h */
    0xfe00707f,  /* fmin_q */
    0xfe00707f,  /* fmin_s */
    0xfe00707f,  /* fminm_d */
    0xfe00707f,  /* fminm_h */
    0xfe00707f,  /* fminm_q */
    0xfe00707f,  /* fminm_s */
    0x600007f,   /* fmsub_d */
    0x600007f,   /* fmsub_h */
    0x600007f,   /* fmsub_q */
    0x600007f,   /* fmsub_s */
    0xfe00007f,  /* fmul_d */
    0xfe00007f,  /* fmul_h */
    0xfe00007f,  /* fmul_q */
    0xfe00007f,  /* fmul_s */
    0xfe00707f,  /* fmv_d */
    0xffff_ffff, /* fmv_d_x */
    0xfe00707f,  /* fmv_h */
    0xfff0707f,  /* fmv_h_x */
    0xfe00707f,  /* fmv_q */
    0xfe00707f,  /* fmv_s */
    0xfff0707f,  /* fmv_s_x */
    0xfff0707f,  /* fmv_w_x */
    0xffff_ffff, /* fmv_x_d */
    0xfff0707f,  /* fmv_x_h */
    0xfff0707f,  /* fmv_x_s */
    0xfff0707f,  /* fmv_x_w */
    0xfff0707f,  /* fmvh_x_d */
    0xffff_ffff, /* fmvh_x_q */
    0xfe00707f,  /* fmvp_d_x */
    0xffff_ffff, /* fmvp_q_x */
    0xfe00707f,  /* fneg_d */
    0xfe00707f,  /* fneg_h */
    0xfe00707f,  /* fneg_q */
    0xfe00707f,  /* fneg_s */
    0x600007f,   /* fnmadd_d */
    0x600007f,   /* fnmadd_h */
    0x600007f,   /* fnmadd_q */
    0x600007f,   /* fnmadd_s */
    0x600007f,   /* fnmsub_d */
    0x600007f,   /* fnmsub_h */
    0x600007f,   /* fnmsub_q */
    0x600007f,   /* fnmsub_s */
    0xfffff07f,  /* frcsr */
    0xfffff07f,  /* frflags */
    0xfff0007f,  /* fround_d */
    0xfff0007f,  /* fround_h */
    0xfff0007f,  /* fround_q */
    0xfff0007f,  /* fround_s */
    0xfff0007f,  /* froundnx_d */
    0xfff0007f,  /* froundnx_h */
    0xfff0007f,  /* froundnx_q */
    0xfff0007f,  /* froundnx_s */
    0xfffff07f,  /* frrm */
    0xfff0707f,  /* fscsr */
    0x707f,      /* fsd */
    0xfff0707f,  /* fsflags */
    0xfff0707f,  /* fsflagsi */
    0xfe00707f,  /* fsgnj_d */
    0xfe00707f,  /* fsgnj_h */
    0xfe00707f,  /* fsgnj_q */
    0xfe00707f,  /* fsgnj_s */
    0xfe00707f,  /* fsgnjn_d */
    0xfe00707f,  /* fsgnjn_h */
    0xfe00707f,  /* fsgnjn_q */
    0xfe00707f,  /* fsgnjn_s */
    0xfe00707f,  /* fsgnjx_d */
    0xfe00707f,  /* fsgnjx_h */
    0xfe00707f,  /* fsgnjx_q */
    0xfe00707f,  /* fsgnjx_s */
    0x707f,      /* fsh */
    0x707f,      /* fsq */
    0xfff0007f,  /* fsqrt_d */
    0xfff0007f,  /* fsqrt_h */
    0xfff0007f,  /* fsqrt_q */
    0xfff0007f,  /* fsqrt_s */
    0xfff0707f,  /* fsrm */
    0xfff0707f,  /* fsrmi */
    0xfe00007f,  /* fsub_d */
    0xfe00007f,  /* fsub_h */
    0xfe00007f,  /* fsub_q */
    0xfe00007f,  /* fsub_s */
    0x707f,      /* fsw */
    0xfe007fff,  /* hfence_gvma */
    0xfe007fff,  /* hfence_vvma */
    0xfe007fff,  /* hinval_gvma */
    0xfe007fff,  /* hinval_vvma */
    0xfff0707f,  /* hlv_b */
    0xfff0707f,  /* hlv_bu */
    0xffff_ffff, /* hlv_d */
    0xfff0707f,  /* hlv_h */
    0xfff0707f,  /* hlv_hu */
    0xfff0707f,  /* hlv_w */
    0xffff_ffff, /* hlv_wu */
    0xfff0707f,  /* hlvx_hu */
    0xfff0707f,  /* hlvx_wu */
    0xfe007fff,  /* hsv_b */
    0xffff_ffff, /* hsv_d */
    0xfe007fff,  /* hsv_h */
    0xfe007fff,  /* hsv_w */
    0xfff,       /* j */
    0x7f,        /* jal */
    0xfff,       /* jal_pseudo */
    0x707f,      /* jalr */
    0xfff07fff,  /* jalr_pseudo */
    0xfff07fff,  /* jr */
    0x707f,      /* lb */
    0x707f,      /* lbu */
    0xffff_ffff, /* ld */
    0x707f,      /* lh */
    0x707f,      /* lhu */
    0xfff,       /* lpad */
    0xffff_ffff, /* lr_d */
    0xf9f0707f,  /* lr_w */
    0x7f,        /* lui */
    0x707f,      /* lw */
    0xffff_ffff, /* lwu */
    0xfe00707f,  /* max */
    0xfe00707f,  /* maxu */
    0xfe00707f,  /* min */
    0xfe00707f,  /* minu */
    0xffffffff,  /* mnret */
    0xfff0707f,  /* mop_r_0 */
    0xfff0707f,  /* mop_r_1 */
    0xfff0707f,  /* mop_r_10 */
    0xfff0707f,  /* mop_r_11 */
    0xfff0707f,  /* mop_r_12 */
    0xfff0707f,  /* mop_r_13 */
    0xfff0707f,  /* mop_r_14 */
    0xfff0707f,  /* mop_r_15 */
    0xfff0707f,  /* mop_r_16 */
    0xfff0707f,  /* mop_r_17 */
    0xfff0707f,  /* mop_r_18 */
    0xfff0707f,  /* mop_r_19 */
    0xfff0707f,  /* mop_r_2 */
    0xfff0707f,  /* mop_r_20 */
    0xfff0707f,  /* mop_r_21 */
    0xfff0707f,  /* mop_r_22 */
    0xfff0707f,  /* mop_r_23 */
    0xfff0707f,  /* mop_r_24 */
    0xfff0707f,  /* mop_r_25 */
    0xfff0707f,  /* mop_r_26 */
    0xfff0707f,  /* mop_r_27 */
    0xfff0707f,  /* mop_r_28 */
    0xfff0707f,  /* mop_r_29 */
    0xfff0707f,  /* mop_r_3 */
    0xfff0707f,  /* mop_r_30 */
    0xfff0707f,  /* mop_r_31 */
    0xfff0707f,  /* mop_r_4 */
    0xfff0707f,  /* mop_r_5 */
    0xfff0707f,  /* mop_r_6 */
    0xfff0707f,  /* mop_r_7 */
    0xfff0707f,  /* mop_r_8 */
    0xfff0707f,  /* mop_r_9 */
    0xb3c0707f,  /* mop_r_N */
    0xfe00707f,  /* mop_rr_0 */
    0xfe00707f,  /* mop_rr_1 */
    0xfe00707f,  /* mop_rr_2 */
    0xfe00707f,  /* mop_rr_3 */
    0xfe00707f,  /* mop_rr_4 */
    0xfe00707f,  /* mop_rr_5 */
    0xfe00707f,  /* mop_rr_6 */
    0xfe00707f,  /* mop_rr_7 */
    0xb200707f,  /* mop_rr_N */
    0xffffffff,  /* mret */
    0xfe00707f,  /* mul */
    0xfe00707f,  /* mulh */
    0xfe00707f,  /* mulhsu */
    0xfe00707f,  /* mulhu */
    0xffff_ffff, /* mulw */
    0xfff0707f,  /* mv */
    0xfff0707f,  /* neg */
    0xffffffff,  /* nop */
    0xffffffff,  /* ntl_all */
    0xffffffff,  /* ntl_p1 */
    0xffffffff,  /* ntl_pall */
    0xffffffff,  /* ntl_s1 */
    0xfe00707f,  /* or */
    0xfff0707f,  /* orc_b */
    0x707f,      /* ori */
    0xfe00707f,  /* orn */
    0xfe00707f,  /* pack */
    0xfe00707f,  /* packh */
    0xffff_ffff, /* packw */
    0xffffffff,  /* pause */
    0x1f07fff,   /* prefetch_i */
    0x1f07fff,   /* prefetch_r */
    0x1f07fff,   /* prefetch_w */
    0xfffff07f,  /* rdcycle */
    0xfffff07f,  /* rdcycleh */
    0xfffff07f,  /* rdinstret */
    0xfffff07f,  /* rdinstreth */
    0xfffff07f,  /* rdtime */
    0xfffff07f,  /* rdtimeh */
    0xfe00707f,  /* rem */
    0xfe00707f,  /* remu */
    0xffff_ffff, /* remuw */
    0xffff_ffff, /* remw */
    0xffffffff,  /* ret */
    0xffff_ffff, /* rev8 */
    0xfff0707f,  /* rev8_rv32 */
    0xfe00707f,  /* rol */
    0xffff_ffff, /* rolw */
    0xfe00707f,  /* ror */
    0xffff_ffff, /* rori */
    0xfe00707f,  /* rori_rv32 */
    0xffff_ffff, /* roriw */
    0xffff_ffff, /* rorw */
    0x707f,      /* sb */
    0xffffffff,  /* sbreak */
    0xffff_ffff, /* sc_d */
    0xf800707f,  /* sc_w */
    0xffffffff,  /* scall */
    0xffffffff,  /* sctrclr */
    0xffff_ffff, /* sd */
    0xfff0707f,  /* seqz */
    0xfff0707f,  /* sext_b */
    0xfff0707f,  /* sext_h */
    0xffff_ffff, /* sext_w */
    0xffffffff,  /* sfence_inval_ir */
    0xfe007fff,  /* sfence_vma */
    0xffffffff,  /* sfence_w_inval */
    0xfe0ff07f,  /* sgtz */
    0x707f,      /* sh */
    0xfe00707f,  /* sh1add */
    0xffff_ffff, /* sh1add_uw */
    0xfe00707f,  /* sh2add */
    0xffff_ffff, /* sh2add_uw */
    0xfe00707f,  /* sh3add */
    0xffff_ffff, /* sh3add_uw */
    0xfff0707f,  /* sha256sig0 */
    0xfff0707f,  /* sha256sig1 */
    0xfff0707f,  /* sha256sum0 */
    0xfff0707f,  /* sha256sum1 */
    0xffff_ffff, /* sha512sig0 */
    0xfe00707f,  /* sha512sig0h */
    0xfe00707f,  /* sha512sig0l */
    0xffff_ffff, /* sha512sig1 */
    0xfe00707f,  /* sha512sig1h */
    0xfe00707f,  /* sha512sig1l */
    0xffff_ffff, /* sha512sum0 */
    0xfe00707f,  /* sha512sum0r */
    0xffff_ffff, /* sha512sum1 */
    0xfe00707f,  /* sha512sum1r */
    0xfe007fff,  /* sinval_vma */
    0xfe00707f,  /* sll */
    0xfc00707f,  /* slli */
    0xfe00707f,  /* slli_rv32 */
    0xffff_ffff, /* slli_uw */
    0xffff_ffff, /* slliw */
    0xffff_ffff, /* sllw */
    0xfe00707f,  /* slt */
    0x707f,      /* slti */
    0x707f,      /* sltiu */
    0xfe00707f,  /* sltu */
    0xfff0707f,  /* sltz */
    0xfff0707f,  /* sm3p0 */
    0xfff0707f,  /* sm3p1 */
    0x3e00707f,  /* sm4ed */
    0x3e00707f,  /* sm4ks */
    0xfe0ff07f,  /* snez */
    0xfe00707f,  /* sra */
    0xfc00707f,  /* srai */
    0xfe00707f,  /* srai_rv32 */
    0xffff_ffff, /* sraiw */
    0xffff_ffff, /* sraw */
    0xffffffff,  /* sret */
    0xfe00707f,  /* srl */
    0xfc00707f,  /* srli */
    0xfe00707f,  /* srli_rv32 */
    0xffff_ffff, /* srliw */
    0xffff_ffff, /* srlw */
    0xffff_ffff, /* ssamoswap_d */
    0xf800707f,  /* ssamoswap_w */
    0xffffffff,  /* sspopchk_x1 */
    0xffffffff,  /* sspopchk_x5 */
    0xffffffff,  /* sspush_x1 */
    0xffffffff,  /* sspush_x5 */
    0xfffff07f,  /* ssrdp */
    0xfe00707f,  /* sub */
    0xffff_ffff, /* subw */
    0x707f,      /* sw */
    0xfff0707f,  /* unzip */
    0xfc00707f,  /* vaadd_vv */
    0xfc00707f,  /* vaadd_vx */
    0xfc00707f,  /* vaaddu_vv */
    0xfc00707f,  /* vaaddu_vx */
    0xfe00707f,  /* vadc_vim */
    0xfe00707f,  /* vadc_vvm */
    0xfe00707f,  /* vadc_vxm */
    0xfc00707f,  /* vadd_vi */
    0xfc00707f,  /* vadd_vv */
    0xfc00707f,  /* vadd_vx */
    0xfe0ff07f,  /* vaesdf_vs */
    0xfe0ff07f,  /* vaesdf_vv */
    0xfe0ff07f,  /* vaesdm_vs */
    0xfe0ff07f,  /* vaesdm_vv */
    0xfe0ff07f,  /* vaesef_vs */
    0xfe0ff07f,  /* vaesef_vv */
    0xfe0ff07f,  /* vaesem_vs */
    0xfe0ff07f,  /* vaesem_vv */
    0xfe00707f,  /* vaeskf1_vi */
    0xfe00707f,  /* vaeskf2_vi */
    0xfe0ff07f,  /* vaesz_vs */
    0xfc00707f,  /* vand_vi */
    0xfc00707f,  /* vand_vv */
    0xfc00707f,  /* vand_vx */
    0xfc00707f,  /* vandn_vv */
    0xfc00707f,  /* vandn_vx */
    0xfc00707f,  /* vasub_vv */
    0xfc00707f,  /* vasub_vx */
    0xfc00707f,  /* vasubu_vv */
    0xfc00707f,  /* vasubu_vx */
    0xfc0ff07f,  /* vbrev8_v */
    0xfc0ff07f,  /* vbrev_v */
    0xfc00707f,  /* vclmul_vv */
    0xfc00707f,  /* vclmul_vx */
    0xfc00707f,  /* vclmulh_vv */
    0xfc00707f,  /* vclmulh_vx */
    0xfc0ff07f,  /* vclz_v */
    0xfe00707f,  /* vcompress_vm */
    0xfc0ff07f,  /* vcpop_m */
    0xfc0ff07f,  /* vcpop_v */
    0xfc0ff07f,  /* vctz_v */
    0xfc00707f,  /* vdiv_vv */
    0xfc00707f,  /* vdiv_vx */
    0xfc00707f,  /* vdivu_vv */
    0xfc00707f,  /* vdivu_vx */
    0xfc00707f,  /* vfadd_vf */
    0xfc00707f,  /* vfadd_vv */
    0xfc0ff07f,  /* vfclass_v */
    0xfc0ff07f,  /* vfcvt_f_x_v */
    0xfc0ff07f,  /* vfcvt_f_xu_v */
    0xfc0ff07f,  /* vfcvt_rtz_x_f_v */
    0xfc0ff07f,  /* vfcvt_rtz_xu_f_v */
    0xfc0ff07f,  /* vfcvt_x_f_v */
    0xfc0ff07f,  /* vfcvt_xu_f_v */
    0xfc00707f,  /* vfdiv_vf */
    0xfc00707f,  /* vfdiv_vv */
    0xfc0ff07f,  /* vfirst_m */
    0xfc00707f,  /* vfmacc_vf */
    0xfc00707f,  /* vfmacc_vv */
    0xfc00707f,  /* vfmadd_vf */
    0xfc00707f,  /* vfmadd_vv */
    0xfc00707f,  /* vfmax_vf */
    0xfc00707f,  /* vfmax_vv */
    0xfe00707f,  /* vfmerge_vfm */
    0xfc00707f,  /* vfmin_vf */
    0xfc00707f,  /* vfmin_vv */
    0xfc00707f,  /* vfmsac_vf */
    0xfc00707f,  /* vfmsac_vv */
    0xfc00707f,  /* vfmsub_vf */
    0xfc00707f,  /* vfmsub_vv */
    0xfc00707f,  /* vfmul_vf */
    0xfc00707f,  /* vfmul_vv */
    0xfe0ff07f,  /* vfmv_f_s */
    0xfff0707f,  /* vfmv_s_f */
    0xfff0707f,  /* vfmv_v_f */
    0xfc0ff07f,  /* vfncvt_f_f_w */
    0xfc0ff07f,  /* vfncvt_f_x_w */
    0xfc0ff07f,  /* vfncvt_f_xu_w */
    0xfc0ff07f,  /* vfncvt_rod_f_f_w */
    0xfc0ff07f,  /* vfncvt_rtz_x_f_w */
    0xfc0ff07f,  /* vfncvt_rtz_xu_f_w */
    0xfc0ff07f,  /* vfncvt_x_f_w */
    0xfc0ff07f,  /* vfncvt_xu_f_w */
    0xfc0ff07f,  /* vfncvtbf16_f_f_w */
    0xfc00707f,  /* vfnmacc_vf */
    0xfc00707f,  /* vfnmacc_vv */
    0xfc00707f,  /* vfnmadd_vf */
    0xfc00707f,  /* vfnmadd_vv */
    0xfc00707f,  /* vfnmsac_vf */
    0xfc00707f,  /* vfnmsac_vv */
    0xfc00707f,  /* vfnmsub_vf */
    0xfc00707f,  /* vfnmsub_vv */
    0xfc00707f,  /* vfrdiv_vf */
    0xfc0ff07f,  /* vfrec7_v */
    0xfc00707f,  /* vfredmax_vs */
    0xfc00707f,  /* vfredmin_vs */
    0xfc00707f,  /* vfredosum_vs */
    0xfc00707f,  /* vfredsum_vs */
    0xfc00707f,  /* vfredusum_vs */
    0xfc0ff07f,  /* vfrsqrt7_v */
    0xfc00707f,  /* vfrsub_vf */
    0xfc00707f,  /* vfsgnj_vf */
    0xfc00707f,  /* vfsgnj_vv */
    0xfc00707f,  /* vfsgnjn_vf */
    0xfc00707f,  /* vfsgnjn_vv */
    0xfc00707f,  /* vfsgnjx_vf */
    0xfc00707f,  /* vfsgnjx_vv */
    0xfc00707f,  /* vfslide1down_vf */
    0xfc00707f,  /* vfslide1up_vf */
    0xfc0ff07f,  /* vfsqrt_v */
    0xfc00707f,  /* vfsub_vf */
    0xfc00707f,  /* vfsub_vv */
    0xfc00707f,  /* vfwadd_vf */
    0xfc00707f,  /* vfwadd_vv */
    0xfc00707f,  /* vfwadd_wf */
    0xfc00707f,  /* vfwadd_wv */
    0xfc0ff07f,  /* vfwcvt_f_f_v */
    0xfc0ff07f,  /* vfwcvt_f_x_v */
    0xfc0ff07f,  /* vfwcvt_f_xu_v */
    0xfc0ff07f,  /* vfwcvt_rtz_x_f_v */
    0xfc0ff07f,  /* vfwcvt_rtz_xu_f_v */
    0xfc0ff07f,  /* vfwcvt_x_f_v */
    0xfc0ff07f,  /* vfwcvt_xu_f_v */
    0xfc0ff07f,  /* vfwcvtbf16_f_f_v */
    0xfc00707f,  /* vfwmacc_vf */
    0xfc00707f,  /* vfwmacc_vv */
    0xfc00707f,  /* vfwmaccbf16_vf */
    0xfc00707f,  /* vfwmaccbf16_vv */
    0xfc00707f,  /* vfwmsac_vf */
    0xfc00707f,  /* vfwmsac_vv */
    0xfc00707f,  /* vfwmul_vf */
    0xfc00707f,  /* vfwmul_vv */
    0xfc00707f,  /* vfwnmacc_vf */
    0xfc00707f,  /* vfwnmacc_vv */
    0xfc00707f,  /* vfwnmsac_vf */
    0xfc00707f,  /* vfwnmsac_vv */
    0xfc00707f,  /* vfwredosum_vs */
    0xfc00707f,  /* vfwredsum_vs */
    0xfc00707f,  /* vfwredusum_vs */
    0xfc00707f,  /* vfwsub_vf */
    0xfc00707f,  /* vfwsub_vv */
    0xfc00707f,  /* vfwsub_wf */
    0xfc00707f,  /* vfwsub_wv */
    0xfe00707f,  /* vghsh_vv */
    0xfe0ff07f,  /* vgmul_vv */
    0xfdfff07f,  /* vid_v */
    0xfc0ff07f,  /* viota_m */
    0xfff0707f,  /* vl1r_v */
    0xfff0707f,  /* vl1re16_v */
    0xfff0707f,  /* vl1re32_v */
    0xfff0707f,  /* vl1re64_v */
    0xfff0707f,  /* vl1re8_v */
    0xfff0707f,  /* vl2r_v */
    0xfff0707f,  /* vl2re16_v */
    0xfff0707f,  /* vl2re32_v */
    0xfff0707f,  /* vl2re64_v */
    0xfff0707f,  /* vl2re8_v */
    0xfff0707f,  /* vl4r_v */
    0xfff0707f,  /* vl4re16_v */
    0xfff0707f,  /* vl4re32_v */
    0xfff0707f,  /* vl4re64_v */
    0xfff0707f,  /* vl4re8_v */
    0xfff0707f,  /* vl8r_v */
    0xfff0707f,  /* vl8re16_v */
    0xfff0707f,  /* vl8re32_v */
    0xfff0707f,  /* vl8re64_v */
    0xfff0707f,  /* vl8re8_v */
    0x1df0707f,  /* vle16_v */
    0x1df0707f,  /* vle16ff_v */
    0xfff0707f,  /* vle1_v */
    0x1df0707f,  /* vle32_v */
    0x1df0707f,  /* vle32ff_v */
    0x1df0707f,  /* vle64_v */
    0x1df0707f,  /* vle64ff_v */
    0x1df0707f,  /* vle8_v */
    0x1df0707f,  /* vle8ff_v */
    0xfff0707f,  /* vlm_v */
    0x1c00707f,  /* vloxei16_v */
    0x1c00707f,  /* vloxei32_v */
    0x1c00707f,  /* vloxei64_v */
    0x1c00707f,  /* vloxei8_v */
    0x1c00707f,  /* vlse16_v */
    0x1c00707f,  /* vlse32_v */
    0x1c00707f,  /* vlse64_v */
    0x1c00707f,  /* vlse8_v */
    0x1c00707f,  /* vluxei16_v */
    0x1c00707f,  /* vluxei32_v */
    0x1c00707f,  /* vluxei64_v */
    0x1c00707f,  /* vluxei8_v */
    0xfc00707f,  /* vmacc_vv */
    0xfc00707f,  /* vmacc_vx */
    0xfe00707f,  /* vmadc_vi */
    0xfe00707f,  /* vmadc_vim */
    0xfe00707f,  /* vmadc_vv */
    0xfe00707f,  /* vmadc_vvm */
    0xfe00707f,  /* vmadc_vx */
    0xfe00707f,  /* vmadc_vxm */
    0xfc00707f,  /* vmadd_vv */
    0xfc00707f,  /* vmadd_vx */
    0xfe00707f,  /* vmand_mm */
    0xfe00707f,  /* vmandn_mm */
    0xfc00707f,  /* vmandnot_mm */
    0xfc00707f,  /* vmax_vv */
    0xfc00707f,  /* vmax_vx */
    0xfc00707f,  /* vmaxu_vv */
    0xfc00707f,  /* vmaxu_vx */
    0xfe00707f,  /* vmerge_vim */
    0xfe00707f,  /* vmerge_vvm */
    0xfe00707f,  /* vmerge_vxm */
    0xfc00707f,  /* vmfeq_vf */
    0xfc00707f,  /* vmfeq_vv */
    0xfc00707f,  /* vmfge_vf */
    0xfc00707f,  /* vmfgt_vf */
    0xfc00707f,  /* vmfle_vf */
    0xfc00707f,  /* vmfle_vv */
    0xfc00707f,  /* vmflt_vf */
    0xfc00707f,  /* vmflt_vv */
    0xfc00707f,  /* vmfne_vf */
    0xfc00707f,  /* vmfne_vv */
    0xfc00707f,  /* vmin_vv */
    0xfc00707f,  /* vmin_vx */
    0xfc00707f,  /* vminu_vv */
    0xfc00707f,  /* vminu_vx */
    0xfe00707f,  /* vmnand_mm */
    0xfe00707f,  /* vmnor_mm */
    0xfe00707f,  /* vmor_mm */
    0xfe00707f,  /* vmorn_mm */
    0xfc00707f,  /* vmornot_mm */
    0xfe00707f,  /* vmsbc_vv */
    0xfe00707f,  /* vmsbc_vvm */
    0xfe00707f,  /* vmsbc_vx */
    0xfe00707f,  /* vmsbc_vxm */
    0xfc0ff07f,  /* vmsbf_m */
    0xfc00707f,  /* vmseq_vi */
    0xfc00707f,  /* vmseq_vv */
    0xfc00707f,  /* vmseq_vx */
    0xfc00707f,  /* vmsgt_vi */
    0xfc00707f,  /* vmsgt_vx */
    0xfc00707f,  /* vmsgtu_vi */
    0xfc00707f,  /* vmsgtu_vx */
    0xfc0ff07f,  /* vmsif_m */
    0xfc00707f,  /* vmsle_vi */
    0xfc00707f,  /* vmsle_vv */
    0xfc00707f,  /* vmsle_vx */
    0xfc00707f,  /* vmsleu_vi */
    0xfc00707f,  /* vmsleu_vv */
    0xfc00707f,  /* vmsleu_vx */
    0xfc00707f,  /* vmslt_vv */
    0xfc00707f,  /* vmslt_vx */
    0xfc00707f,  /* vmsltu_vv */
    0xfc00707f,  /* vmsltu_vx */
    0xfc00707f,  /* vmsne_vi */
    0xfc00707f,  /* vmsne_vv */
    0xfc00707f,  /* vmsne_vx */
    0xfc0ff07f,  /* vmsof_m */
    0xfc00707f,  /* vmul_vv */
    0xfc00707f,  /* vmul_vx */
    0xfc00707f,  /* vmulh_vv */
    0xfc00707f,  /* vmulh_vx */
    0xfc00707f,  /* vmulhsu_vv */
    0xfc00707f,  /* vmulhsu_vx */
    0xfc00707f,  /* vmulhu_vv */
    0xfc00707f,  /* vmulhu_vx */
    0xfe0ff07f,  /* vmv1r_v */
    0xfe0ff07f,  /* vmv2r_v */
    0xfe0ff07f,  /* vmv4r_v */
    0xfe0ff07f,  /* vmv8r_v */
    0xfff0707f,  /* vmv_s_x */
    0xfff0707f,  /* vmv_v_i */
    0xfff0707f,  /* vmv_v_v */
    0xfff0707f,  /* vmv_v_x */
    0xfe0ff07f,  /* vmv_x_s */
    0xfe00707f,  /* vmxnor_mm */
    0xfe00707f,  /* vmxor_mm */
    0xfc00707f,  /* vnclip_wi */
    0xfc00707f,  /* vnclip_wv */
    0xfc00707f,  /* vnclip_wx */
    0xfc00707f,  /* vnclipu_wi */
    0xfc00707f,  /* vnclipu_wv */
    0xfc00707f,  /* vnclipu_wx */
    0xfc00707f,  /* vnmsac_vv */
    0xfc00707f,  /* vnmsac_vx */
    0xfc00707f,  /* vnmsub_vv */
    0xfc00707f,  /* vnmsub_vx */
    0xfc00707f,  /* vnsra_wi */
    0xfc00707f,  /* vnsra_wv */
    0xfc00707f,  /* vnsra_wx */
    0xfc00707f,  /* vnsrl_wi */
    0xfc00707f,  /* vnsrl_wv */
    0xfc00707f,  /* vnsrl_wx */
    0xfc00707f,  /* vor_vi */
    0xfc00707f,  /* vor_vv */
    0xfc00707f,  /* vor_vx */
    0xfc0ff07f,  /* vpopc_m */
    0xfc00707f,  /* vredand_vs */
    0xfc00707f,  /* vredmax_vs */
    0xfc00707f,  /* vredmaxu_vs */
    0xfc00707f,  /* vredmin_vs */
    0xfc00707f,  /* vredminu_vs */
    0xfc00707f,  /* vredor_vs */
    0xfc00707f,  /* vredsum_vs */
    0xfc00707f,  /* vredxor_vs */
    0xfc00707f,  /* vrem_vv */
    0xfc00707f,  /* vrem_vx */
    0xfc00707f,  /* vremu_vv */
    0xfc00707f,  /* vremu_vx */
    0xfc0ff07f,  /* vrev8_v */
    0xfc00707f,  /* vrgather_vi */
    0xfc00707f,  /* vrgather_vv */
    0xfc00707f,  /* vrgather_vx */
    0xfc00707f,  /* vrgatherei16_vv */
    0xfc00707f,  /* vrol_vv */
    0xfc00707f,  /* vrol_vx */
    0xf800707f,  /* vror_vi */
    0xfc00707f,  /* vror_vv */
    0xfc00707f,  /* vror_vx */
    0xfc00707f,  /* vrsub_vi */
    0xfc00707f,  /* vrsub_vx */
    0xfff0707f,  /* vs1r_v */
    0xfff0707f,  /* vs2r_v */
    0xfff0707f,  /* vs4r_v */
    0xfff0707f,  /* vs8r_v */
    0xfc00707f,  /* vsadd_vi */
    0xfc00707f,  /* vsadd_vv */
    0xfc00707f,  /* vsadd_vx */
    0xfc00707f,  /* vsaddu_vi */
    0xfc00707f,  /* vsaddu_vv */
    0xfc00707f,  /* vsaddu_vx */
    0xfe00707f,  /* vsbc_vvm */
    0xfe00707f,  /* vsbc_vxm */
    0x1df0707f,  /* vse16_v */
    0xfff0707f,  /* vse1_v */
    0x1df0707f,  /* vse32_v */
    0x1df0707f,  /* vse64_v */
    0x1df0707f,  /* vse8_v */
    0xc000707f,  /* vsetivli */
    0xfe00707f,  /* vsetvl */
    0x8000707f,  /* vsetvli */
    0xfc0ff07f,  /* vsext_vf2 */
    0xfc0ff07f,  /* vsext_vf4 */
    0xfc0ff07f,  /* vsext_vf8 */
    0xfe00707f,  /* vsha2ch_vv */
    0xfe00707f,  /* vsha2cl_vv */
    0xfe00707f,  /* vsha2ms_vv */
    0xfc00707f,  /* vslide1down_vx */
    0xfc00707f,  /* vslide1up_vx */
    0xfc00707f,  /* vslidedown_vi */
    0xfc00707f,  /* vslidedown_vx */
    0xfc00707f,  /* vslideup_vi */
    0xfc00707f,  /* vslideup_vx */
    0xfc00707f,  /* vsll_vi */
    0xfc00707f,  /* vsll_vv */
    0xfc00707f,  /* vsll_vx */
    0xfe00707f,  /* vsm3c_vi */
    0xfe00707f,  /* vsm3me_vv */
    0xfe00707f,  /* vsm4k_vi */
    0xfe0ff07f,  /* vsm4r_vs */
    0xfe0ff07f,  /* vsm4r_vv */
    0xfff0707f,  /* vsm_v */
    0xfc00707f,  /* vsmul_vv */
    0xfc00707f,  /* vsmul_vx */
    0x1c00707f,  /* vsoxei16_v */
    0x1c00707f,  /* vsoxei32_v */
    0x1c00707f,  /* vsoxei64_v */
    0x1c00707f,  /* vsoxei8_v */
    0xfc00707f,  /* vsra_vi */
    0xfc00707f,  /* vsra_vv */
    0xfc00707f,  /* vsra_vx */
    0xfc00707f,  /* vsrl_vi */
    0xfc00707f,  /* vsrl_vv */
    0xfc00707f,  /* vsrl_vx */
    0x1c00707f,  /* vsse16_v */
    0x1c00707f,  /* vsse32_v */
    0x1c00707f,  /* vsse64_v */
    0x1c00707f,  /* vsse8_v */
    0xfc00707f,  /* vssra_vi */
    0xfc00707f,  /* vssra_vv */
    0xfc00707f,  /* vssra_vx */
    0xfc00707f,  /* vssrl_vi */
    0xfc00707f,  /* vssrl_vv */
    0xfc00707f,  /* vssrl_vx */
    0xfc00707f,  /* vssub_vv */
    0xfc00707f,  /* vssub_vx */
    0xfc00707f,  /* vssubu_vv */
    0xfc00707f,  /* vssubu_vx */
    0xfc00707f,  /* vsub_vv */
    0xfc00707f,  /* vsub_vx */
    0x1c00707f,  /* vsuxei16_v */
    0x1c00707f,  /* vsuxei32_v */
    0x1c00707f,  /* vsuxei64_v */
    0x1c00707f,  /* vsuxei8_v */
    0xfc00707f,  /* vwadd_vv */
    0xfc00707f,  /* vwadd_vx */
    0xfc00707f,  /* vwadd_wv */
    0xfc00707f,  /* vwadd_wx */
    0xfc00707f,  /* vwaddu_vv */
    0xfc00707f,  /* vwaddu_vx */
    0xfc00707f,  /* vwaddu_wv */
    0xfc00707f,  /* vwaddu_wx */
    0xfc00707f,  /* vwmacc_vv */
    0xfc00707f,  /* vwmacc_vx */
    0xfc00707f,  /* vwmaccsu_vv */
    0xfc00707f,  /* vwmaccsu_vx */
    0xfc00707f,  /* vwmaccu_vv */
    0xfc00707f,  /* vwmaccu_vx */
    0xfc00707f,  /* vwmaccus_vx */
    0xfc00707f,  /* vwmul_vv */
    0xfc00707f,  /* vwmul_vx */
    0xfc00707f,  /* vwmulsu_vv */
    0xfc00707f,  /* vwmulsu_vx */
    0xfc00707f,  /* vwmulu_vv */
    0xfc00707f,  /* vwmulu_vx */
    0xfc00707f,  /* vwredsum_vs */
    0xfc00707f,  /* vwredsumu_vs */
    0xfc00707f,  /* vwsll_vi */
    0xfc00707f,  /* vwsll_vv */
    0xfc00707f,  /* vwsll_vx */
    0xfc00707f,  /* vwsub_vv */
    0xfc00707f,  /* vwsub_vx */
    0xfc00707f,  /* vwsub_wv */
    0xfc00707f,  /* vwsub_wx */
    0xfc00707f,  /* vwsubu_vv */
    0xfc00707f,  /* vwsubu_vx */
    0xfc00707f,  /* vwsubu_wv */
    0xfc00707f,  /* vwsubu_wx */
    0xfc00707f,  /* vxor_vi */
    0xfc00707f,  /* vxor_vv */
    0xfc00707f,  /* vxor_vx */
    0xfc0ff07f,  /* vzext_vf2 */
    0xfc0ff07f,  /* vzext_vf4 */
    0xfc0ff07f,  /* vzext_vf8 */
    0xffffffff,  /* wfi */
    0xffffffff,  /* wrs_nto */
    0xffffffff,  /* wrs_sto */
    0xfe00707f,  /* xnor */
    0xfe00707f,  /* xor */
    0x707f,      /* xori */
    0xfe00707f,  /* xperm4 */
    0xfe00707f,  /* xperm8 */
    0xfff0707f,  /* zext_b */
    0xffff_ffff, /* zext_h */
    0xfff0707f,  /* zext_h_rv32 */
    0xffff_ffff, /* zext_w */
    0xfff0707f,  /* zip */
];
pub static OPCODE64_MATCH: [u32; 1039] = [
    0x33,        /* add */
    0x800003b,   /* add_uw */
    0x13,        /* addi */
    0x1b,        /* addiw */
    0x3b,        /* addw */
    0xffff_ffff, /* aes32dsi */
    0xffff_ffff, /* aes32dsmi */
    0xffff_ffff, /* aes32esi */
    0xffff_ffff, /* aes32esmi */
    0x3a000033,  /* aes64ds */
    0x3e000033,  /* aes64dsm */
    0x32000033,  /* aes64es */
    0x36000033,  /* aes64esm */
    0x30001013,  /* aes64im */
    0x31001013,  /* aes64ks1i */
    0x7e000033,  /* aes64ks2 */
    0x2f,        /* amoadd_b */
    0x302f,      /* amoadd_d */
    0x102f,      /* amoadd_h */
    0x202f,      /* amoadd_w */
    0x6000002f,  /* amoand_b */
    0x6000302f,  /* amoand_d */
    0x6000102f,  /* amoand_h */
    0x6000202f,  /* amoand_w */
    0x2800002f,  /* amocas_b */
    0x2800302f,  /* amocas_d */
    0x2800102f,  /* amocas_h */
    0x2800402f,  /* amocas_q */
    0x2800202f,  /* amocas_w */
    0xa000002f,  /* amomax_b */
    0xa000302f,  /* amomax_d */
    0xa000102f,  /* amomax_h */
    0xa000202f,  /* amomax_w */
    0xe000002f,  /* amomaxu_b */
    0xe000302f,  /* amomaxu_d */
    0xe000102f,  /* amomaxu_h */
    0xe000202f,  /* amomaxu_w */
    0x8000002f,  /* amomin_b */
    0x8000302f,  /* amomin_d */
    0x8000102f,  /* amomin_h */
    0x8000202f,  /* amomin_w */
    0xc000002f,  /* amominu_b */
    0xc000302f,  /* amominu_d */
    0xc000102f,  /* amominu_h */
    0xc000202f,  /* amominu_w */
    0x4000002f,  /* amoor_b */
    0x4000302f,  /* amoor_d */
    0x4000102f,  /* amoor_h */
    0x4000202f,  /* amoor_w */
    0x800002f,   /* amoswap_b */
    0x800302f,   /* amoswap_d */
    0x800102f,   /* amoswap_h */
    0x800202f,   /* amoswap_w */
    0x2000002f,  /* amoxor_b */
    0x2000302f,  /* amoxor_d */
    0x2000102f,  /* amoxor_h */
    0x2000202f,  /* amoxor_w */
    0x7033,      /* and */
    0x7013,      /* andi */
    0x40007033,  /* andn */
    0x17,        /* auipc */
    0x48001033,  /* bclr */
    0x48001013,  /* bclri */
    0xffff_ffff, /* bclri_rv32 */
    0x63,        /* beq */
    0x63,        /* beqz */
    0x48005033,  /* bext */
    0x48005013,  /* bexti */
    0xffff_ffff, /* bexti_rv32 */
    0x5063,      /* bge */
    0x7063,      /* bgeu */
    0x5063,      /* bgez */
    0x4063,      /* bgt */
    0x6063,      /* bgtu */
    0x4063,      /* bgtz */
    0x68001033,  /* binv */
    0x68001013,  /* binvi */
    0xffff_ffff, /* binvi_rv32 */
    0x5063,      /* ble */
    0x7063,      /* bleu */
    0x5063,      /* blez */
    0x4063,      /* blt */
    0x6063,      /* bltu */
    0x4063,      /* bltz */
    0x1063,      /* bne */
    0x1063,      /* bnez */
    0x68705013,  /* brev8 */
    0x28001033,  /* bset */
    0x28001013,  /* bseti */
    0xffff_ffff, /* bseti_rv32 */
    0x9002,      /* c_add */
    0x1,         /* c_addi */
    0x6101,      /* c_addi16sp */
    0x0,         /* c_addi4spn */
    0x2001,      /* c_addiw */
    0x9c21,      /* c_addw */
    0x8c61,      /* c_and */
    0x8801,      /* c_andi */
    0xc001,      /* c_beqz */
    0xe001,      /* c_bnez */
    0x9002,      /* c_ebreak */
    0x2000,      /* c_fld */
    0x2002,      /* c_fldsp */
    0xffff_ffff, /* c_flw */
    0xffff_ffff, /* c_flwsp */
    0xa000,      /* c_fsd */
    0xa002,      /* c_fsdsp */
    0xffff_ffff, /* c_fsw */
    0xffff_ffff, /* c_fswsp */
    0xa001,      /* c_j */
    0xffff_ffff, /* c_jal */
    0x9002,      /* c_jalr */
    0x8002,      /* c_jr */
    0x8000,      /* c_lbu */
    0x6000,      /* c_ld */
    0x6002,      /* c_ldsp */
    0x8440,      /* c_lh */
    0x8400,      /* c_lhu */
    0x4001,      /* c_li */
    0x6001,      /* c_lui */
    0x4000,      /* c_lw */
    0x4002,      /* c_lwsp */
    0x6081,      /* c_mop_1 */
    0x6581,      /* c_mop_11 */
    0x6681,      /* c_mop_13 */
    0x6781,      /* c_mop_15 */
    0x6181,      /* c_mop_3 */
    0x6281,      /* c_mop_5 */
    0x6381,      /* c_mop_7 */
    0x6481,      /* c_mop_9 */
    0x6081,      /* c_mop_N */
    0x9c41,      /* c_mul */
    0x8002,      /* c_mv */
    0x1,         /* c_nop */
    0x9c75,      /* c_not */
    0x9016,      /* c_ntl_all */
    0x900a,      /* c_ntl_p1 */
    0x900e,      /* c_ntl_pall */
    0x9012,      /* c_ntl_s1 */
    0x8c41,      /* c_or */
    0x8800,      /* c_sb */
    0xe000,      /* c_sd */
    0xe002,      /* c_sdsp */
    0x9c65,      /* c_sext_b */
    0x9c6d,      /* c_sext_h */
    0x2001,      /* c_sext_w */
    0x8c00,      /* c_sh */
    0x2,         /* c_slli */
    0xffff_ffff, /* c_slli_rv32 */
    0x8401,      /* c_srai */
    0xffff_ffff, /* c_srai_rv32 */
    0x8001,      /* c_srli */
    0xffff_ffff, /* c_srli_rv32 */
    0x6281,      /* c_sspopchk_x5 */
    0x6081,      /* c_sspush_x1 */
    0x8c01,      /* c_sub */
    0x9c01,      /* c_subw */
    0xc000,      /* c_sw */
    0xc002,      /* c_swsp */
    0x8c21,      /* c_xor */
    0x9c61,      /* c_zext_b */
    0x9c69,      /* c_zext_h */
    0x9c71,      /* c_zext_w */
    0x10200f,    /* cbo_clean */
    0x20200f,    /* cbo_flush */
    0x200f,      /* cbo_inval */
    0x40200f,    /* cbo_zero */
    0xa001033,   /* clmul */
    0xa003033,   /* clmulh */
    0xa002033,   /* clmulr */
    0x60001013,  /* clz */
    0x6000101b,  /* clzw */
    0xa002,      /* cm_jalt */
    0xac62,      /* cm_mva01s */
    0xac22,      /* cm_mvsa01 */
    0xba02,      /* cm_pop */
    0xbe02,      /* cm_popret */
    0xbc02,      /* cm_popretz */
    0xb802,      /* cm_push */
    0x60201013,  /* cpop */
    0x6020101b,  /* cpopw */
    0x3073,      /* csrc */
    0x7073,      /* csrci */
    0x2073,      /* csrr */
    0x3073,      /* csrrc */
    0x7073,      /* csrrci */
    0x2073,      /* csrrs */
    0x6073,      /* csrrsi */
    0x1073,      /* csrrw */
    0x5073,      /* csrrwi */
    0x2073,      /* csrs */
    0x6073,      /* csrsi */
    0x1073,      /* csrw */
    0x5073,      /* csrwi */
    0x60101013,  /* ctz */
    0x6010101b,  /* ctzw */
    0xe005033,   /* czero_eqz */
    0xe007033,   /* czero_nez */
    0x2004033,   /* div */
    0x2005033,   /* divu */
    0x200503b,   /* divuw */
    0x200403b,   /* divw */
    0x7b200073,  /* dret */
    0x100073,    /* ebreak */
    0x73,        /* ecall */
    0x22002053,  /* fabs_d */
    0x24002053,  /* fabs_h */
    0x26002053,  /* fabs_q */
    0x20002053,  /* fabs_s */
    0x2000053,   /* fadd_d */
    0x4000053,   /* fadd_h */
    0x6000053,   /* fadd_q */
    0x53,        /* fadd_s */
    0xe2001053,  /* fclass_d */
    0xe4001053,  /* fclass_h */
    0xe6001053,  /* fclass_q */
    0xe0001053,  /* fclass_s */
    0x44800053,  /* fcvt_bf16_s */
    0x42200053,  /* fcvt_d_h */
    0xd2200053,  /* fcvt_d_l */
    0xd2300053,  /* fcvt_d_lu */
    0x42300053,  /* fcvt_d_q */
    0x42000053,  /* fcvt_d_s */
    0xd2000053,  /* fcvt_d_w */
    0xd2100053,  /* fcvt_d_wu */
    0x44100053,  /* fcvt_h_d */
    0xd4200053,  /* fcvt_h_l */
    0xd4300053,  /* fcvt_h_lu */
    0x44300053,  /* fcvt_h_q */
    0x44000053,  /* fcvt_h_s */
    0xd4000053,  /* fcvt_h_w */
    0xd4100053,  /* fcvt_h_wu */
    0xc2200053,  /* fcvt_l_d */
    0xc4200053,  /* fcvt_l_h */
    0xc6200053,  /* fcvt_l_q */
    0xc0200053,  /* fcvt_l_s */
    0xc2300053,  /* fcvt_lu_d */
    0xc4300053,  /* fcvt_lu_h */
    0xc6300053,  /* fcvt_lu_q */
    0xc0300053,  /* fcvt_lu_s */
    0x46100053,  /* fcvt_q_d */
    0x46200053,  /* fcvt_q_h */
    0xd6200053,  /* fcvt_q_l */
    0xd6300053,  /* fcvt_q_lu */
    0x46000053,  /* fcvt_q_s */
    0xd6000053,  /* fcvt_q_w */
    0xd6100053,  /* fcvt_q_wu */
    0x40600053,  /* fcvt_s_bf16 */
    0x40100053,  /* fcvt_s_d */
    0x40200053,  /* fcvt_s_h */
    0xd0200053,  /* fcvt_s_l */
    0xd0300053,  /* fcvt_s_lu */
    0x40300053,  /* fcvt_s_q */
    0xd0000053,  /* fcvt_s_w */
    0xd0100053,  /* fcvt_s_wu */
    0xc2000053,  /* fcvt_w_d */
    0xc4000053,  /* fcvt_w_h */
    0xc6000053,  /* fcvt_w_q */
    0xc0000053,  /* fcvt_w_s */
    0xc2100053,  /* fcvt_wu_d */
    0xc4100053,  /* fcvt_wu_h */
    0xc6100053,  /* fcvt_wu_q */
    0xc0100053,  /* fcvt_wu_s */
    0xc2801053,  /* fcvtmod_w_d */
    0x1a000053,  /* fdiv_d */
    0x1c000053,  /* fdiv_h */
    0x1e000053,  /* fdiv_q */
    0x18000053,  /* fdiv_s */
    0xf,         /* fence */
    0x100f,      /* fence_i */
    0x8330000f,  /* fence_tso */
    0xa2002053,  /* feq_d */
    0xa4002053,  /* feq_h */
    0xa6002053,  /* feq_q */
    0xa0002053,  /* feq_s */
    0x3007,      /* fld */
    0xa2000053,  /* fle_d */
    0xa4000053,  /* fle_h */
    0xa6000053,  /* fle_q */
    0xa0000053,  /* fle_s */
    0xa2004053,  /* fleq_d */
    0xa4004053,  /* fleq_h */
    0xa6004053,  /* fleq_q */
    0xa0004053,  /* fleq_s */
    0x1007,      /* flh */
    0xf2100053,  /* fli_d */
    0xf4100053,  /* fli_h */
    0xf6100053,  /* fli_q */
    0xf0100053,  /* fli_s */
    0x4007,      /* flq */
    0xa2001053,  /* flt_d */
    0xa4001053,  /* flt_h */
    0xa6001053,  /* flt_q */
    0xa0001053,  /* flt_s */
    0xa2005053,  /* fltq_d */
    0xa4005053,  /* fltq_h */
    0xa6005053,  /* fltq_q */
    0xa0005053,  /* fltq_s */
    0x2007,      /* flw */
    0x2000043,   /* fmadd_d */
    0x4000043,   /* fmadd_h */
    0x6000043,   /* fmadd_q */
    0x43,        /* fmadd_s */
    0x2a001053,  /* fmax_d */
    0x2c001053,  /* fmax_h */
    0x2e001053,  /* fmax_q */
    0x28001053,  /* fmax_s */
    0x2a003053,  /* fmaxm_d */
    0x2c003053,  /* fmaxm_h */
    0x2e003053,  /* fmaxm_q */
    0x28003053,  /* fmaxm_s */
    0x2a000053,  /* fmin_d */
    0x2c000053,  /* fmin_h */
    0x2e000053,  /* fmin_q */
    0x28000053,  /* fmin_s */
    0x2a002053,  /* fminm_d */
    0x2c002053,  /* fminm_h */
    0x2e002053,  /* fminm_q */
    0x28002053,  /* fminm_s */
    0x2000047,   /* fmsub_d */
    0x4000047,   /* fmsub_h */
    0x6000047,   /* fmsub_q */
    0x47,        /* fmsub_s */
    0x12000053,  /* fmul_d */
    0x14000053,  /* fmul_h */
    0x16000053,  /* fmul_q */
    0x10000053,  /* fmul_s */
    0x22000053,  /* fmv_d */
    0xf2000053,  /* fmv_d_x */
    0x24000053,  /* fmv_h */
    0xf4000053,  /* fmv_h_x */
    0x26000053,  /* fmv_q */
    0x20000053,  /* fmv_s */
    0xf0000053,  /* fmv_s_x */
    0xf0000053,  /* fmv_w_x */
    0xe2000053,  /* fmv_x_d */
    0xe4000053,  /* fmv_x_h */
    0xe0000053,  /* fmv_x_s */
    0xe0000053,  /* fmv_x_w */
    0xffff_ffff, /* fmvh_x_d */
    0xe6100053,  /* fmvh_x_q */
    0xffff_ffff, /* fmvp_d_x */
    0xb6000053,  /* fmvp_q_x */
    0x22001053,  /* fneg_d */
    0x24001053,  /* fneg_h */
    0x26001053,  /* fneg_q */
    0x20001053,  /* fneg_s */
    0x200004f,   /* fnmadd_d */
    0x400004f,   /* fnmadd_h */
    0x600004f,   /* fnmadd_q */
    0x4f,        /* fnmadd_s */
    0x200004b,   /* fnmsub_d */
    0x400004b,   /* fnmsub_h */
    0x600004b,   /* fnmsub_q */
    0x4b,        /* fnmsub_s */
    0x302073,    /* frcsr */
    0x102073,    /* frflags */
    0x42400053,  /* fround_d */
    0x44400053,  /* fround_h */
    0x46400053,  /* fround_q */
    0x40400053,  /* fround_s */
    0x42500053,  /* froundnx_d */
    0x44500053,  /* froundnx_h */
    0x46500053,  /* froundnx_q */
    0x40500053,  /* froundnx_s */
    0x202073,    /* frrm */
    0x301073,    /* fscsr */
    0x3027,      /* fsd */
    0x101073,    /* fsflags */
    0x105073,    /* fsflagsi */
    0x22000053,  /* fsgnj_d */
    0x24000053,  /* fsgnj_h */
    0x26000053,  /* fsgnj_q */
    0x20000053,  /* fsgnj_s */
    0x22001053,  /* fsgnjn_d */
    0x24001053,  /* fsgnjn_h */
    0x26001053,  /* fsgnjn_q */
    0x20001053,  /* fsgnjn_s */
    0x22002053,  /* fsgnjx_d */
    0x24002053,  /* fsgnjx_h */
    0x26002053,  /* fsgnjx_q */
    0x20002053,  /* fsgnjx_s */
    0x1027,      /* fsh */
    0x4027,      /* fsq */
    0x5a000053,  /* fsqrt_d */
    0x5c000053,  /* fsqrt_h */
    0x5e000053,  /* fsqrt_q */
    0x58000053,  /* fsqrt_s */
    0x201073,    /* fsrm */
    0x205073,    /* fsrmi */
    0xa000053,   /* fsub_d */
    0xc000053,   /* fsub_h */
    0xe000053,   /* fsub_q */
    0x8000053,   /* fsub_s */
    0x2027,      /* fsw */
    0x62000073,  /* hfence_gvma */
    0x22000073,  /* hfence_vvma */
    0x66000073,  /* hinval_gvma */
    0x26000073,  /* hinval_vvma */
    0x60004073,  /* hlv_b */
    0x60104073,  /* hlv_bu */
    0x6c004073,  /* hlv_d */
    0x64004073,  /* hlv_h */
    0x64104073,  /* hlv_hu */
    0x68004073,  /* hlv_w */
    0x68104073,  /* hlv_wu */
    0x64304073,  /* hlvx_hu */
    0x68304073,  /* hlvx_wu */
    0x62004073,  /* hsv_b */
    0x6e004073,  /* hsv_d */
    0x66004073,  /* hsv_h */
    0x6a004073,  /* hsv_w */
    0x6f,        /* j */
    0x6f,        /* jal */
    0xef,        /* jal_pseudo */
    0x67,        /* jalr */
    0xe7,        /* jalr_pseudo */
    0x67,        /* jr */
    0x3,         /* lb */
    0x4003,      /* lbu */
    0x3003,      /* ld */
    0x1003,      /* lh */
    0x5003,      /* lhu */
    0x17,        /* lpad */
    0x1000302f,  /* lr_d */
    0x1000202f,  /* lr_w */
    0x37,        /* lui */
    0x2003,      /* lw */
    0x6003,      /* lwu */
    0xa006033,   /* max */
    0xa007033,   /* maxu */
    0xa004033,   /* min */
    0xa005033,   /* minu */
    0x70200073,  /* mnret */
    0x81c04073,  /* mop_r_0 */
    0x81d04073,  /* mop_r_1 */
    0x89e04073,  /* mop_r_10 */
    0x89f04073,  /* mop_r_11 */
    0x8dc04073,  /* mop_r_12 */
    0x8dd04073,  /* mop_r_13 */
    0x8de04073,  /* mop_r_14 */
    0x8df04073,  /* mop_r_15 */
    0xc1c04073,  /* mop_r_16 */
    0xc1d04073,  /* mop_r_17 */
    0xc1e04073,  /* mop_r_18 */
    0xc1f04073,  /* mop_r_19 */
    0x81e04073,  /* mop_r_2 */
    0xc5c04073,  /* mop_r_20 */
    0xc5d04073,  /* mop_r_21 */
    0xc5e04073,  /* mop_r_22 */
    0xc5f04073,  /* mop_r_23 */
    0xc9c04073,  /* mop_r_24 */
    0xc9d04073,  /* mop_r_25 */
    0xc9e04073,  /* mop_r_26 */
    0xc9f04073,  /* mop_r_27 */
    0xcdc04073,  /* mop_r_28 */
    0xcdd04073,  /* mop_r_29 */
    0x81f04073,  /* mop_r_3 */
    0xcde04073,  /* mop_r_30 */
    0xcdf04073,  /* mop_r_31 */
    0x85c04073,  /* mop_r_4 */
    0x85d04073,  /* mop_r_5 */
    0x85e04073,  /* mop_r_6 */
    0x85f04073,  /* mop_r_7 */
    0x89c04073,  /* mop_r_8 */
    0x89d04073,  /* mop_r_9 */
    0x81c04073,  /* mop_r_N */
    0x82004073,  /* mop_rr_0 */
    0x86004073,  /* mop_rr_1 */
    0x8a004073,  /* mop_rr_2 */
    0x8e004073,  /* mop_rr_3 */
    0xc2004073,  /* mop_rr_4 */
    0xc6004073,  /* mop_rr_5 */
    0xca004073,  /* mop_rr_6 */
    0xce004073,  /* mop_rr_7 */
    0x82004073,  /* mop_rr_N */
    0x30200073,  /* mret */
    0x2000033,   /* mul */
    0x2001033,   /* mulh */
    0x2002033,   /* mulhsu */
    0x2003033,   /* mulhu */
    0x200003b,   /* mulw */
    0x13,        /* mv */
    0x40000033,  /* neg */
    0x13,        /* nop */
    0x500033,    /* ntl_all */
    0x200033,    /* ntl_p1 */
    0x300033,    /* ntl_pall */
    0x400033,    /* ntl_s1 */
    0x6033,      /* or */
    0x28705013,  /* orc_b */
    0x6013,      /* ori */
    0x40006033,  /* orn */
    0x8004033,   /* pack */
    0x8007033,   /* packh */
    0x800403b,   /* packw */
    0x100000f,   /* pause */
    0x6013,      /* prefetch_i */
    0x106013,    /* prefetch_r */
    0x306013,    /* prefetch_w */
    0xc0002073,  /* rdcycle */
    0xffff_ffff, /* rdcycleh */
    0xc0202073,  /* rdinstret */
    0xffff_ffff, /* rdinstreth */
    0xc0102073,  /* rdtime */
    0xffff_ffff, /* rdtimeh */
    0x2006033,   /* rem */
    0x2007033,   /* remu */
    0x200703b,   /* remuw */
    0x200603b,   /* remw */
    0x8067,      /* ret */
    0x6b805013,  /* rev8 */
    0xffff_ffff, /* rev8_rv32 */
    0x60001033,  /* rol */
    0x6000103b,  /* rolw */
    0x60005033,  /* ror */
    0x60005013,  /* rori */
    0xffff_ffff, /* rori_rv32 */
    0x6000501b,  /* roriw */
    0x6000503b,  /* rorw */
    0x23,        /* sb */
    0x100073,    /* sbreak */
    0x1800302f,  /* sc_d */
    0x1800202f,  /* sc_w */
    0x73,        /* scall */
    0x10400073,  /* sctrclr */
    0x3023,      /* sd */
    0x103013,    /* seqz */
    0x60401013,  /* sext_b */
    0x60501013,  /* sext_h */
    0x1b,        /* sext_w */
    0x18100073,  /* sfence_inval_ir */
    0x12000073,  /* sfence_vma */
    0x18000073,  /* sfence_w_inval */
    0x2033,      /* sgtz */
    0x1023,      /* sh */
    0x20002033,  /* sh1add */
    0x2000203b,  /* sh1add_uw */
    0x20004033,  /* sh2add */
    0x2000403b,  /* sh2add_uw */
    0x20006033,  /* sh3add */
    0x2000603b,  /* sh3add_uw */
    0x10201013,  /* sha256sig0 */
    0x10301013,  /* sha256sig1 */
    0x10001013,  /* sha256sum0 */
    0x10101013,  /* sha256sum1 */
    0x10601013,  /* sha512sig0 */
    0xffff_ffff, /* sha512sig0h */
    0xffff_ffff, /* sha512sig0l */
    0x10701013,  /* sha512sig1 */
    0xffff_ffff, /* sha512sig1h */
    0xffff_ffff, /* sha512sig1l */
    0x10401013,  /* sha512sum0 */
    0xffff_ffff, /* sha512sum0r */
    0x10501013,  /* sha512sum1 */
    0xffff_ffff, /* sha512sum1r */
    0x16000073,  /* sinval_vma */
    0x1033,      /* sll */
    0x1013,      /* slli */
    0xffff_ffff, /* slli_rv32 */
    0x800101b,   /* slli_uw */
    0x101b,      /* slliw */
    0x103b,      /* sllw */
    0x2033,      /* slt */
    0x2013,      /* slti */
    0x3013,      /* sltiu */
    0x3033,      /* sltu */
    0x2033,      /* sltz */
    0x10801013,  /* sm3p0 */
    0x10901013,  /* sm3p1 */
    0x30000033,  /* sm4ed */
    0x34000033,  /* sm4ks */
    0x3033,      /* snez */
    0x40005033,  /* sra */
    0x40005013,  /* srai */
    0xffff_ffff, /* srai_rv32 */
    0x4000501b,  /* sraiw */
    0x4000503b,  /* sraw */
    0x10200073,  /* sret */
    0x5033,      /* srl */
    0x5013,      /* srli */
    0xffff_ffff, /* srli_rv32 */
    0x501b,      /* srliw */
    0x503b,      /* srlw */
    0x4800302f,  /* ssamoswap_d */
    0x4800202f,  /* ssamoswap_w */
    0xcdc0c073,  /* sspopchk_x1 */
    0xcdc2c073,  /* sspopchk_x5 */
    0xce104073,  /* sspush_x1 */
    0xce504073,  /* sspush_x5 */
    0xcdc04073,  /* ssrdp */
    0x40000033,  /* sub */
    0x4000003b,  /* subw */
    0x2023,      /* sw */
    0xffff_ffff, /* unzip */
    0x24002057,  /* vaadd_vv */
    0x24006057,  /* vaadd_vx */
    0x20002057,  /* vaaddu_vv */
    0x20006057,  /* vaaddu_vx */
    0x40003057,  /* vadc_vim */
    0x40000057,  /* vadc_vvm */
    0x40004057,  /* vadc_vxm */
    0x3057,      /* vadd_vi */
    0x57,        /* vadd_vv */
    0x4057,      /* vadd_vx */
    0xa600a077,  /* vaesdf_vs */
    0xa200a077,  /* vaesdf_vv */
    0xa6002077,  /* vaesdm_vs */
    0xa2002077,  /* vaesdm_vv */
    0xa601a077,  /* vaesef_vs */
    0xa201a077,  /* vaesef_vv */
    0xa6012077,  /* vaesem_vs */
    0xa2012077,  /* vaesem_vv */
    0x8a002077,  /* vaeskf1_vi */
    0xaa002077,  /* vaeskf2_vi */
    0xa603a077,  /* vaesz_vs */
    0x24003057,  /* vand_vi */
    0x24000057,  /* vand_vv */
    0x24004057,  /* vand_vx */
    0x4000057,   /* vandn_vv */
    0x4004057,   /* vandn_vx */
    0x2c002057,  /* vasub_vv */
    0x2c006057,  /* vasub_vx */
    0x28002057,  /* vasubu_vv */
    0x28006057,  /* vasubu_vx */
    0x48042057,  /* vbrev8_v */
    0x48052057,  /* vbrev_v */
    0x30002057,  /* vclmul_vv */
    0x30006057,  /* vclmul_vx */
    0x34002057,  /* vclmulh_vv */
    0x34006057,  /* vclmulh_vx */
    0x48062057,  /* vclz_v */
    0x5e002057,  /* vcompress_vm */
    0x40082057,  /* vcpop_m */
    0x48072057,  /* vcpop_v */
    0x4806a057,  /* vctz_v */
    0x84002057,  /* vdiv_vv */
    0x84006057,  /* vdiv_vx */
    0x80002057,  /* vdivu_vv */
    0x80006057,  /* vdivu_vx */
    0x5057,      /* vfadd_vf */
    0x1057,      /* vfadd_vv */
    0x4c081057,  /* vfclass_v */
    0x48019057,  /* vfcvt_f_x_v */
    0x48011057,  /* vfcvt_f_xu_v */
    0x48039057,  /* vfcvt_rtz_x_f_v */
    0x48031057,  /* vfcvt_rtz_xu_f_v */
    0x48009057,  /* vfcvt_x_f_v */
    0x48001057,  /* vfcvt_xu_f_v */
    0x80005057,  /* vfdiv_vf */
    0x80001057,  /* vfdiv_vv */
    0x4008a057,  /* vfirst_m */
    0xb0005057,  /* vfmacc_vf */
    0xb0001057,  /* vfmacc_vv */
    0xa0005057,  /* vfmadd_vf */
    0xa0001057,  /* vfmadd_vv */
    0x18005057,  /* vfmax_vf */
    0x18001057,  /* vfmax_vv */
    0x5c005057,  /* vfmerge_vfm */
    0x10005057,  /* vfmin_vf */
    0x10001057,  /* vfmin_vv */
    0xb8005057,  /* vfmsac_vf */
    0xb8001057,  /* vfmsac_vv */
    0xa8005057,  /* vfmsub_vf */
    0xa8001057,  /* vfmsub_vv */
    0x90005057,  /* vfmul_vf */
    0x90001057,  /* vfmul_vv */
    0x42001057,  /* vfmv_f_s */
    0x42005057,  /* vfmv_s_f */
    0x5e005057,  /* vfmv_v_f */
    0x480a1057,  /* vfncvt_f_f_w */
    0x48099057,  /* vfncvt_f_x_w */
    0x48091057,  /* vfncvt_f_xu_w */
    0x480a9057,  /* vfncvt_rod_f_f_w */
    0x480b9057,  /* vfncvt_rtz_x_f_w */
    0x480b1057,  /* vfncvt_rtz_xu_f_w */
    0x48089057,  /* vfncvt_x_f_w */
    0x48081057,  /* vfncvt_xu_f_w */
    0x480e9057,  /* vfncvtbf16_f_f_w */
    0xb4005057,  /* vfnmacc_vf */
    0xb4001057,  /* vfnmacc_vv */
    0xa4005057,  /* vfnmadd_vf */
    0xa4001057,  /* vfnmadd_vv */
    0xbc005057,  /* vfnmsac_vf */
    0xbc001057,  /* vfnmsac_vv */
    0xac005057,  /* vfnmsub_vf */
    0xac001057,  /* vfnmsub_vv */
    0x84005057,  /* vfrdiv_vf */
    0x4c029057,  /* vfrec7_v */
    0x1c001057,  /* vfredmax_vs */
    0x14001057,  /* vfredmin_vs */
    0xc001057,   /* vfredosum_vs */
    0x4001057,   /* vfredsum_vs */
    0x4001057,   /* vfredusum_vs */
    0x4c021057,  /* vfrsqrt7_v */
    0x9c005057,  /* vfrsub_vf */
    0x20005057,  /* vfsgnj_vf */
    0x20001057,  /* vfsgnj_vv */
    0x24005057,  /* vfsgnjn_vf */
    0x24001057,  /* vfsgnjn_vv */
    0x28005057,  /* vfsgnjx_vf */
    0x28001057,  /* vfsgnjx_vv */
    0x3c005057,  /* vfslide1down_vf */
    0x38005057,  /* vfslide1up_vf */
    0x4c001057,  /* vfsqrt_v */
    0x8005057,   /* vfsub_vf */
    0x8001057,   /* vfsub_vv */
    0xc0005057,  /* vfwadd_vf */
    0xc0001057,  /* vfwadd_vv */
    0xd0005057,  /* vfwadd_wf */
    0xd0001057,  /* vfwadd_wv */
    0x48061057,  /* vfwcvt_f_f_v */
    0x48059057,  /* vfwcvt_f_x_v */
    0x48051057,  /* vfwcvt_f_xu_v */
    0x48079057,  /* vfwcvt_rtz_x_f_v */
    0x48071057,  /* vfwcvt_rtz_xu_f_v */
    0x48049057,  /* vfwcvt_x_f_v */
    0x48041057,  /* vfwcvt_xu_f_v */
    0x48069057,  /* vfwcvtbf16_f_f_v */
    0xf0005057,  /* vfwmacc_vf */
    0xf0001057,  /* vfwmacc_vv */
    0xec005057,  /* vfwmaccbf16_vf */
    0xec001057,  /* vfwmaccbf16_vv */
    0xf8005057,  /* vfwmsac_vf */
    0xf8001057,  /* vfwmsac_vv */
    0xe0005057,  /* vfwmul_vf */
    0xe0001057,  /* vfwmul_vv */
    0xf4005057,  /* vfwnmacc_vf */
    0xf4001057,  /* vfwnmacc_vv */
    0xfc005057,  /* vfwnmsac_vf */
    0xfc001057,  /* vfwnmsac_vv */
    0xcc001057,  /* vfwredosum_vs */
    0xc4001057,  /* vfwredsum_vs */
    0xc4001057,  /* vfwredusum_vs */
    0xc8005057,  /* vfwsub_vf */
    0xc8001057,  /* vfwsub_vv */
    0xd8005057,  /* vfwsub_wf */
    0xd8001057,  /* vfwsub_wv */
    0xb2002077,  /* vghsh_vv */
    0xa208a077,  /* vgmul_vv */
    0x5008a057,  /* vid_v */
    0x50082057,  /* viota_m */
    0x2800007,   /* vl1r_v */
    0x2805007,   /* vl1re16_v */
    0x2806007,   /* vl1re32_v */
    0x2807007,   /* vl1re64_v */
    0x2800007,   /* vl1re8_v */
    0x22800007,  /* vl2r_v */
    0x22805007,  /* vl2re16_v */
    0x22806007,  /* vl2re32_v */
    0x22807007,  /* vl2re64_v */
    0x22800007,  /* vl2re8_v */
    0x62800007,  /* vl4r_v */
    0x62805007,  /* vl4re16_v */
    0x62806007,  /* vl4re32_v */
    0x62807007,  /* vl4re64_v */
    0x62800007,  /* vl4re8_v */
    0xe2800007,  /* vl8r_v */
    0xe2805007,  /* vl8re16_v */
    0xe2806007,  /* vl8re32_v */
    0xe2807007,  /* vl8re64_v */
    0xe2800007,  /* vl8re8_v */
    0x5007,      /* vle16_v */
    0x1005007,   /* vle16ff_v */
    0x2b00007,   /* vle1_v */
    0x6007,      /* vle32_v */
    0x1006007,   /* vle32ff_v */
    0x7007,      /* vle64_v */
    0x1007007,   /* vle64ff_v */
    0x7,         /* vle8_v */
    0x1000007,   /* vle8ff_v */
    0x2b00007,   /* vlm_v */
    0xc005007,   /* vloxei16_v */
    0xc006007,   /* vloxei32_v */
    0xc007007,   /* vloxei64_v */
    0xc000007,   /* vloxei8_v */
    0x8005007,   /* vlse16_v */
    0x8006007,   /* vlse32_v */
    0x8007007,   /* vlse64_v */
    0x8000007,   /* vlse8_v */
    0x4005007,   /* vluxei16_v */
    0x4006007,   /* vluxei32_v */
    0x4007007,   /* vluxei64_v */
    0x4000007,   /* vluxei8_v */
    0xb4002057,  /* vmacc_vv */
    0xb4006057,  /* vmacc_vx */
    0x46003057,  /* vmadc_vi */
    0x44003057,  /* vmadc_vim */
    0x46000057,  /* vmadc_vv */
    0x44000057,  /* vmadc_vvm */
    0x46004057,  /* vmadc_vx */
    0x44004057,  /* vmadc_vxm */
    0xa4002057,  /* vmadd_vv */
    0xa4006057,  /* vmadd_vx */
    0x66002057,  /* vmand_mm */
    0x62002057,  /* vmandn_mm */
    0x60002057,  /* vmandnot_mm */
    0x1c000057,  /* vmax_vv */
    0x1c004057,  /* vmax_vx */
    0x18000057,  /* vmaxu_vv */
    0x18004057,  /* vmaxu_vx */
    0x5c003057,  /* vmerge_vim */
    0x5c000057,  /* vmerge_vvm */
    0x5c004057,  /* vmerge_vxm */
    0x60005057,  /* vmfeq_vf */
    0x60001057,  /* vmfeq_vv */
    0x7c005057,  /* vmfge_vf */
    0x74005057,  /* vmfgt_vf */
    0x64005057,  /* vmfle_vf */
    0x64001057,  /* vmfle_vv */
    0x6c005057,  /* vmflt_vf */
    0x6c001057,  /* vmflt_vv */
    0x70005057,  /* vmfne_vf */
    0x70001057,  /* vmfne_vv */
    0x14000057,  /* vmin_vv */
    0x14004057,  /* vmin_vx */
    0x10000057,  /* vminu_vv */
    0x10004057,  /* vminu_vx */
    0x76002057,  /* vmnand_mm */
    0x7a002057,  /* vmnor_mm */
    0x6a002057,  /* vmor_mm */
    0x72002057,  /* vmorn_mm */
    0x70002057,  /* vmornot_mm */
    0x4e000057,  /* vmsbc_vv */
    0x4c000057,  /* vmsbc_vvm */
    0x4e004057,  /* vmsbc_vx */
    0x4c004057,  /* vmsbc_vxm */
    0x5000a057,  /* vmsbf_m */
    0x60003057,  /* vmseq_vi */
    0x60000057,  /* vmseq_vv */
    0x60004057,  /* vmseq_vx */
    0x7c003057,  /* vmsgt_vi */
    0x7c004057,  /* vmsgt_vx */
    0x78003057,  /* vmsgtu_vi */
    0x78004057,  /* vmsgtu_vx */
    0x5001a057,  /* vmsif_m */
    0x74003057,  /* vmsle_vi */
    0x74000057,  /* vmsle_vv */
    0x74004057,  /* vmsle_vx */
    0x70003057,  /* vmsleu_vi */
    0x70000057,  /* vmsleu_vv */
    0x70004057,  /* vmsleu_vx */
    0x6c000057,  /* vmslt_vv */
    0x6c004057,  /* vmslt_vx */
    0x68000057,  /* vmsltu_vv */
    0x68004057,  /* vmsltu_vx */
    0x64003057,  /* vmsne_vi */
    0x64000057,  /* vmsne_vv */
    0x64004057,  /* vmsne_vx */
    0x50012057,  /* vmsof_m */
    0x94002057,  /* vmul_vv */
    0x94006057,  /* vmul_vx */
    0x9c002057,  /* vmulh_vv */
    0x9c006057,  /* vmulh_vx */
    0x98002057,  /* vmulhsu_vv */
    0x98006057,  /* vmulhsu_vx */
    0x90002057,  /* vmulhu_vv */
    0x90006057,  /* vmulhu_vx */
    0x9e003057,  /* vmv1r_v */
    0x9e00b057,  /* vmv2r_v */
    0x9e01b057,  /* vmv4r_v */
    0x9e03b057,  /* vmv8r_v */
    0x42006057,  /* vmv_s_x */
    0x5e003057,  /* vmv_v_i */
    0x5e000057,  /* vmv_v_v */
    0x5e004057,  /* vmv_v_x */
    0x42002057,  /* vmv_x_s */
    0x7e002057,  /* vmxnor_mm */
    0x6e002057,  /* vmxor_mm */
    0xbc003057,  /* vnclip_wi */
    0xbc000057,  /* vnclip_wv */
    0xbc004057,  /* vnclip_wx */
    0xb8003057,  /* vnclipu_wi */
    0xb8000057,  /* vnclipu_wv */
    0xb8004057,  /* vnclipu_wx */
    0xbc002057,  /* vnmsac_vv */
    0xbc006057,  /* vnmsac_vx */
    0xac002057,  /* vnmsub_vv */
    0xac006057,  /* vnmsub_vx */
    0xb4003057,  /* vnsra_wi */
    0xb4000057,  /* vnsra_wv */
    0xb4004057,  /* vnsra_wx */
    0xb0003057,  /* vnsrl_wi */
    0xb0000057,  /* vnsrl_wv */
    0xb0004057,  /* vnsrl_wx */
    0x28003057,  /* vor_vi */
    0x28000057,  /* vor_vv */
    0x28004057,  /* vor_vx */
    0x40082057,  /* vpopc_m */
    0x4002057,   /* vredand_vs */
    0x1c002057,  /* vredmax_vs */
    0x18002057,  /* vredmaxu_vs */
    0x14002057,  /* vredmin_vs */
    0x10002057,  /* vredminu_vs */
    0x8002057,   /* vredor_vs */
    0x2057,      /* vredsum_vs */
    0xc002057,   /* vredxor_vs */
    0x8c002057,  /* vrem_vv */
    0x8c006057,  /* vrem_vx */
    0x88002057,  /* vremu_vv */
    0x88006057,  /* vremu_vx */
    0x4804a057,  /* vrev8_v */
    0x30003057,  /* vrgather_vi */
    0x30000057,  /* vrgather_vv */
    0x30004057,  /* vrgather_vx */
    0x38000057,  /* vrgatherei16_vv */
    0x54000057,  /* vrol_vv */
    0x54004057,  /* vrol_vx */
    0x50003057,  /* vror_vi */
    0x50000057,  /* vror_vv */
    0x50004057,  /* vror_vx */
    0xc003057,   /* vrsub_vi */
    0xc004057,   /* vrsub_vx */
    0x2800027,   /* vs1r_v */
    0x22800027,  /* vs2r_v */
    0x62800027,  /* vs4r_v */
    0xe2800027,  /* vs8r_v */
    0x84003057,  /* vsadd_vi */
    0x84000057,  /* vsadd_vv */
    0x84004057,  /* vsadd_vx */
    0x80003057,  /* vsaddu_vi */
    0x80000057,  /* vsaddu_vv */
    0x80004057,  /* vsaddu_vx */
    0x48000057,  /* vsbc_vvm */
    0x48004057,  /* vsbc_vxm */
    0x5027,      /* vse16_v */
    0x2b00027,   /* vse1_v */
    0x6027,      /* vse32_v */
    0x7027,      /* vse64_v */
    0x27,        /* vse8_v */
    0xc0007057,  /* vsetivli */
    0x80007057,  /* vsetvl */
    0x7057,      /* vsetvli */
    0x4803a057,  /* vsext_vf2 */
    0x4802a057,  /* vsext_vf4 */
    0x4801a057,  /* vsext_vf8 */
    0xba002077,  /* vsha2ch_vv */
    0xbe002077,  /* vsha2cl_vv */
    0xb6002077,  /* vsha2ms_vv */
    0x3c006057,  /* vslide1down_vx */
    0x38006057,  /* vslide1up_vx */
    0x3c003057,  /* vslidedown_vi */
    0x3c004057,  /* vslidedown_vx */
    0x38003057,  /* vslideup_vi */
    0x38004057,  /* vslideup_vx */
    0x94003057,  /* vsll_vi */
    0x94000057,  /* vsll_vv */
    0x94004057,  /* vsll_vx */
    0xae002077,  /* vsm3c_vi */
    0x82002077,  /* vsm3me_vv */
    0x86002077,  /* vsm4k_vi */
    0xa6082077,  /* vsm4r_vs */
    0xa2082077,  /* vsm4r_vv */
    0x2b00027,   /* vsm_v */
    0x9c000057,  /* vsmul_vv */
    0x9c004057,  /* vsmul_vx */
    0xc005027,   /* vsoxei16_v */
    0xc006027,   /* vsoxei32_v */
    0xc007027,   /* vsoxei64_v */
    0xc000027,   /* vsoxei8_v */
    0xa4003057,  /* vsra_vi */
    0xa4000057,  /* vsra_vv */
    0xa4004057,  /* vsra_vx */
    0xa0003057,  /* vsrl_vi */
    0xa0000057,  /* vsrl_vv */
    0xa0004057,  /* vsrl_vx */
    0x8005027,   /* vsse16_v */
    0x8006027,   /* vsse32_v */
    0x8007027,   /* vsse64_v */
    0x8000027,   /* vsse8_v */
    0xac003057,  /* vssra_vi */
    0xac000057,  /* vssra_vv */
    0xac004057,  /* vssra_vx */
    0xa8003057,  /* vssrl_vi */
    0xa8000057,  /* vssrl_vv */
    0xa8004057,  /* vssrl_vx */
    0x8c000057,  /* vssub_vv */
    0x8c004057,  /* vssub_vx */
    0x88000057,  /* vssubu_vv */
    0x88004057,  /* vssubu_vx */
    0x8000057,   /* vsub_vv */
    0x8004057,   /* vsub_vx */
    0x4005027,   /* vsuxei16_v */
    0x4006027,   /* vsuxei32_v */
    0x4007027,   /* vsuxei64_v */
    0x4000027,   /* vsuxei8_v */
    0xc4002057,  /* vwadd_vv */
    0xc4006057,  /* vwadd_vx */
    0xd4002057,  /* vwadd_wv */
    0xd4006057,  /* vwadd_wx */
    0xc0002057,  /* vwaddu_vv */
    0xc0006057,  /* vwaddu_vx */
    0xd0002057,  /* vwaddu_wv */
    0xd0006057,  /* vwaddu_wx */
    0xf4002057,  /* vwmacc_vv */
    0xf4006057,  /* vwmacc_vx */
    0xfc002057,  /* vwmaccsu_vv */
    0xfc006057,  /* vwmaccsu_vx */
    0xf0002057,  /* vwmaccu_vv */
    0xf0006057,  /* vwmaccu_vx */
    0xf8006057,  /* vwmaccus_vx */
    0xec002057,  /* vwmul_vv */
    0xec006057,  /* vwmul_vx */
    0xe8002057,  /* vwmulsu_vv */
    0xe8006057,  /* vwmulsu_vx */
    0xe0002057,  /* vwmulu_vv */
    0xe0006057,  /* vwmulu_vx */
    0xc4000057,  /* vwredsum_vs */
    0xc0000057,  /* vwredsumu_vs */
    0xd4003057,  /* vwsll_vi */
    0xd4000057,  /* vwsll_vv */
    0xd4004057,  /* vwsll_vx */
    0xcc002057,  /* vwsub_vv */
    0xcc006057,  /* vwsub_vx */
    0xdc002057,  /* vwsub_wv */
    0xdc006057,  /* vwsub_wx */
    0xc8002057,  /* vwsubu_vv */
    0xc8006057,  /* vwsubu_vx */
    0xd8002057,  /* vwsubu_wv */
    0xd8006057,  /* vwsubu_wx */
    0x2c003057,  /* vxor_vi */
    0x2c000057,  /* vxor_vv */
    0x2c004057,  /* vxor_vx */
    0x48032057,  /* vzext_vf2 */
    0x48022057,  /* vzext_vf4 */
    0x48012057,  /* vzext_vf8 */
    0x10500073,  /* wfi */
    0xd00073,    /* wrs_nto */
    0x1d00073,   /* wrs_sto */
    0x40004033,  /* xnor */
    0x4033,      /* xor */
    0x4013,      /* xori */
    0x28002033,  /* xperm4 */
    0x28004033,  /* xperm8 */
    0xff07013,   /* zext_b */
    0x800403b,   /* zext_h */
    0xffff_ffff, /* zext_h_rv32 */
    0x800003b,   /* zext_w */
    0xffff_ffff, /* zip */
];
pub static OPCODE64_MASK: [u32; 1039] = [
    0xfe00707f,  /* add */
    0xfe00707f,  /* add_uw */
    0x707f,      /* addi */
    0x707f,      /* addiw */
    0xfe00707f,  /* addw */
    0xffff_ffff, /* aes32dsi */
    0xffff_ffff, /* aes32dsmi */
    0xffff_ffff, /* aes32esi */
    0xffff_ffff, /* aes32esmi */
    0xfe00707f,  /* aes64ds */
    0xfe00707f,  /* aes64dsm */
    0xfe00707f,  /* aes64es */
    0xfe00707f,  /* aes64esm */
    0xfff0707f,  /* aes64im */
    0xff00707f,  /* aes64ks1i */
    0xfe00707f,  /* aes64ks2 */
    0xf800707f,  /* amoadd_b */
    0xf800707f,  /* amoadd_d */
    0xf800707f,  /* amoadd_h */
    0xf800707f,  /* amoadd_w */
    0xf800707f,  /* amoand_b */
    0xf800707f,  /* amoand_d */
    0xf800707f,  /* amoand_h */
    0xf800707f,  /* amoand_w */
    0xf800707f,  /* amocas_b */
    0xf800707f,  /* amocas_d */
    0xf800707f,  /* amocas_h */
    0xf800707f,  /* amocas_q */
    0xf800707f,  /* amocas_w */
    0xf800707f,  /* amomax_b */
    0xf800707f,  /* amomax_d */
    0xf800707f,  /* amomax_h */
    0xf800707f,  /* amomax_w */
    0xf800707f,  /* amomaxu_b */
    0xf800707f,  /* amomaxu_d */
    0xf800707f,  /* amomaxu_h */
    0xf800707f,  /* amomaxu_w */
    0xf800707f,  /* amomin_b */
    0xf800707f,  /* amomin_d */
    0xf800707f,  /* amomin_h */
    0xf800707f,  /* amomin_w */
    0xf800707f,  /* amominu_b */
    0xf800707f,  /* amominu_d */
    0xf800707f,  /* amominu_h */
    0xf800707f,  /* amominu_w */
    0xf800707f,  /* amoor_b */
    0xf800707f,  /* amoor_d */
    0xf800707f,  /* amoor_h */
    0xf800707f,  /* amoor_w */
    0xf800707f,  /* amoswap_b */
    0xf800707f,  /* amoswap_d */
    0xf800707f,  /* amoswap_h */
    0xf800707f,  /* amoswap_w */
    0xf800707f,  /* amoxor_b */
    0xf800707f,  /* amoxor_d */
    0xf800707f,  /* amoxor_h */
    0xf800707f,  /* amoxor_w */
    0xfe00707f,  /* and */
    0x707f,      /* andi */
    0xfe00707f,  /* andn */
    0x7f,        /* auipc */
    0xfe00707f,  /* bclr */
    0xfc00707f,  /* bclri */
    0xffff_ffff, /* bclri_rv32 */
    0x707f,      /* beq */
    0x1f0707f,   /* beqz */
    0xfe00707f,  /* bext */
    0xfc00707f,  /* bexti */
    0xffff_ffff, /* bexti_rv32 */
    0x707f,      /* bge */
    0x707f,      /* bgeu */
    0x1f0707f,   /* bgez */
    0x707f,      /* bgt */
    0x707f,      /* bgtu */
    0xff07f,     /* bgtz */
    0xfe00707f,  /* binv */
    0xfc00707f,  /* binvi */
    0xffff_ffff, /* binvi_rv32 */
    0x707f,      /* ble */
    0x707f,      /* bleu */
    0xff07f,     /* blez */
    0x707f,      /* blt */
    0x707f,      /* bltu */
    0x1f0707f,   /* bltz */
    0x707f,      /* bne */
    0x1f0707f,   /* bnez */
    0xfff0707f,  /* brev8 */
    0xfe00707f,  /* bset */
    0xfc00707f,  /* bseti */
    0xffff_ffff, /* bseti_rv32 */
    0xf003,      /* c_add */
    0xe003,      /* c_addi */
    0xef83,      /* c_addi16sp */
    0xe003,      /* c_addi4spn */
    0xe003,      /* c_addiw */
    0xfc63,      /* c_addw */
    0xfc63,      /* c_and */
    0xec03,      /* c_andi */
    0xe003,      /* c_beqz */
    0xe003,      /* c_bnez */
    0xffff,      /* c_ebreak */
    0xe003,      /* c_fld */
    0xe003,      /* c_fldsp */
    0xffff_ffff, /* c_flw */
    0xffff_ffff, /* c_flwsp */
    0xe003,      /* c_fsd */
    0xe003,      /* c_fsdsp */
    0xffff_ffff, /* c_fsw */
    0xffff_ffff, /* c_fswsp */
    0xe003,      /* c_j */
    0xffff_ffff, /* c_jal */
    0xf07f,      /* c_jalr */
    0xf07f,      /* c_jr */
    0xfc03,      /* c_lbu */
    0xe003,      /* c_ld */
    0xe003,      /* c_ldsp */
    0xfc43,      /* c_lh */
    0xfc43,      /* c_lhu */
    0xe003,      /* c_li */
    0xe003,      /* c_lui */
    0xe003,      /* c_lw */
    0xe003,      /* c_lwsp */
    0xffff,      /* c_mop_1 */
    0xffff,      /* c_mop_11 */
    0xffff,      /* c_mop_13 */
    0xffff,      /* c_mop_15 */
    0xffff,      /* c_mop_3 */
    0xffff,      /* c_mop_5 */
    0xffff,      /* c_mop_7 */
    0xffff,      /* c_mop_9 */
    0xf8ff,      /* c_mop_N */
    0xfc63,      /* c_mul */
    0xf003,      /* c_mv */
    0xef83,      /* c_nop */
    0xfc7f,      /* c_not */
    0xffff,      /* c_ntl_all */
    0xffff,      /* c_ntl_p1 */
    0xffff,      /* c_ntl_pall */
    0xffff,      /* c_ntl_s1 */
    0xfc63,      /* c_or */
    0xfc03,      /* c_sb */
    0xe003,      /* c_sd */
    0xe003,      /* c_sdsp */
    0xfc7f,      /* c_sext_b */
    0xfc7f,      /* c_sext_h */
    0xf07f,      /* c_sext_w */
    0xfc43,      /* c_sh */
    0xe003,      /* c_slli */
    0xffff_ffff, /* c_slli_rv32 */
    0xec03,      /* c_srai */
    0xffff_ffff, /* c_srai_rv32 */
    0xec03,      /* c_srli */
    0xffff_ffff, /* c_srli_rv32 */
    0xffff,      /* c_sspopchk_x5 */
    0xffff,      /* c_sspush_x1 */
    0xfc63,      /* c_sub */
    0xfc63,      /* c_subw */
    0xe003,      /* c_sw */
    0xe003,      /* c_swsp */
    0xfc63,      /* c_xor */
    0xfc7f,      /* c_zext_b */
    0xfc7f,      /* c_zext_h */
    0xfc7f,      /* c_zext_w */
    0xfff07fff,  /* cbo_clean */
    0xfff07fff,  /* cbo_flush */
    0xfff07fff,  /* cbo_inval */
    0xfff07fff,  /* cbo_zero */
    0xfe00707f,  /* clmul */
    0xfe00707f,  /* clmulh */
    0xfe00707f,  /* clmulr */
    0xfff0707f,  /* clz */
    0xfff0707f,  /* clzw */
    0xfc03,      /* cm_jalt */
    0xfc63,      /* cm_mva01s */
    0xfc63,      /* cm_mvsa01 */
    0xff03,      /* cm_pop */
    0xff03,      /* cm_popret */
    0xff03,      /* cm_popretz */
    0xff03,      /* cm_push */
    0xfff0707f,  /* cpop */
    0xfff0707f,  /* cpopw */
    0x7fff,      /* csrc */
    0x7fff,      /* csrci */
    0xff07f,     /* csrr */
    0x707f,      /* csrrc */
    0x707f,      /* csrrci */
    0x707f,      /* csrrs */
    0x707f,      /* csrrsi */
    0x707f,      /* csrrw */
    0x707f,      /* csrrwi */
    0x7fff,      /* csrs */
    0x7fff,      /* csrsi */
    0x7fff,      /* csrw */
    0x7fff,      /* csrwi */
    0xfff0707f,  /* ctz */
    0xfff0707f,  /* ctzw */
    0xfe00707f,  /* czero_eqz */
    0xfe00707f,  /* czero_nez */
    0xfe00707f,  /* div */
    0xfe00707f,  /* divu */
    0xfe00707f,  /* divuw */
    0xfe00707f,  /* divw */
    0xffffffff,  /* dret */
    0xffffffff,  /* ebreak */
    0xffffffff,  /* ecall */
    0xfe00707f,  /* fabs_d */
    0xfe00707f,  /* fabs_h */
    0xfe00707f,  /* fabs_q */
    0xfe00707f,  /* fabs_s */
    0xfe00007f,  /* fadd_d */
    0xfe00007f,  /* fadd_h */
    0xfe00007f,  /* fadd_q */
    0xfe00007f,  /* fadd_s */
    0xfff0707f,  /* fclass_d */
    0xfff0707f,  /* fclass_h */
    0xfff0707f,  /* fclass_q */
    0xfff0707f,  /* fclass_s */
    0xfff0007f,  /* fcvt_bf16_s */
    0xfff0007f,  /* fcvt_d_h */
    0xfff0007f,  /* fcvt_d_l */
    0xfff0007f,  /* fcvt_d_lu */
    0xfff0007f,  /* fcvt_d_q */
    0xfff0007f,  /* fcvt_d_s */
    0xfff0007f,  /* fcvt_d_w */
    0xfff0007f,  /* fcvt_d_wu */
    0xfff0007f,  /* fcvt_h_d */
    0xfff0007f,  /* fcvt_h_l */
    0xfff0007f,  /* fcvt_h_lu */
    0xfff0007f,  /* fcvt_h_q */
    0xfff0007f,  /* fcvt_h_s */
    0xfff0007f,  /* fcvt_h_w */
    0xfff0007f,  /* fcvt_h_wu */
    0xfff0007f,  /* fcvt_l_d */
    0xfff0007f,  /* fcvt_l_h */
    0xfff0007f,  /* fcvt_l_q */
    0xfff0007f,  /* fcvt_l_s */
    0xfff0007f,  /* fcvt_lu_d */
    0xfff0007f,  /* fcvt_lu_h */
    0xfff0007f,  /* fcvt_lu_q */
    0xfff0007f,  /* fcvt_lu_s */
    0xfff0007f,  /* fcvt_q_d */
    0xfff0007f,  /* fcvt_q_h */
    0xfff0007f,  /* fcvt_q_l */
    0xfff0007f,  /* fcvt_q_lu */
    0xfff0007f,  /* fcvt_q_s */
    0xfff0007f,  /* fcvt_q_w */
    0xfff0007f,  /* fcvt_q_wu */
    0xfff0007f,  /* fcvt_s_bf16 */
    0xfff0007f,  /* fcvt_s_d */
    0xfff0007f,  /* fcvt_s_h */
    0xfff0007f,  /* fcvt_s_l */
    0xfff0007f,  /* fcvt_s_lu */
    0xfff0007f,  /* fcvt_s_q */
    0xfff0007f,  /* fcvt_s_w */
    0xfff0007f,  /* fcvt_s_wu */
    0xfff0007f,  /* fcvt_w_d */
    0xfff0007f,  /* fcvt_w_h */
    0xfff0007f,  /* fcvt_w_q */
    0xfff0007f,  /* fcvt_w_s */
    0xfff0007f,  /* fcvt_wu_d */
    0xfff0007f,  /* fcvt_wu_h */
    0xfff0007f,  /* fcvt_wu_q */
    0xfff0007f,  /* fcvt_wu_s */
    0xfff0707f,  /* fcvtmod_w_d */
    0xfe00007f,  /* fdiv_d */
    0xfe00007f,  /* fdiv_h */
    0xfe00007f,  /* fdiv_q */
    0xfe00007f,  /* fdiv_s */
    0x707f,      /* fence */
    0x707f,      /* fence_i */
    0xfff0707f,  /* fence_tso */
    0xfe00707f,  /* feq_d */
    0xfe00707f,  /* feq_h */
    0xfe00707f,  /* feq_q */
    0xfe00707f,  /* feq_s */
    0x707f,      /* fld */
    0xfe00707f,  /* fle_d */
    0xfe00707f,  /* fle_h */
    0xfe00707f,  /* fle_q */
    0xfe00707f,  /* fle_s */
    0xfe00707f,  /* fleq_d */
    0xfe00707f,  /* fleq_h */
    0xfe00707f,  /* fleq_q */
    0xfe00707f,  /* fleq_s */
    0x707f,      /* flh */
    0xfff0707f,  /* fli_d */
    0xfff0707f,  /* fli_h */
    0xfff0707f,  /* fli_q */
    0xfff0707f,  /* fli_s */
    0x707f,      /* flq */
    0xfe00707f,  /* flt_d */
    0xfe00707f,  /* flt_h */
    0xfe00707f,  /* flt_q */
    0xfe00707f,  /* flt_s */
    0xfe00707f,  /* fltq_d */
    0xfe00707f,  /* fltq_h */
    0xfe00707f,  /* fltq_q */
    0xfe00707f,  /* fltq_s */
    0x707f,      /* flw */
    0x600007f,   /* fmadd_d */
    0x600007f,   /* fmadd_h */
    0x600007f,   /* fmadd_q */
    0x600007f,   /* fmadd_s */
    0xfe00707f,  /* fmax_d */
    0xfe00707f,  /* fmax_h */
    0xfe00707f,  /* fmax_q */
    0xfe00707f,  /* fmax_s */
    0xfe00707f,  /* fmaxm_d */
    0xfe00707f,  /* fmaxm_h */
    0xfe00707f,  /* fmaxm_q */
    0xfe00707f,  /* fmaxm_s */
    0xfe00707f,  /* fmin_d */
    0xfe00707f,  /* fmin_h */
    0xfe00707f,  /* fmin_q */
    0xfe00707f,  /* fmin_s */
    0xfe00707f,  /* fminm_d */
    0xfe00707f,  /* fminm_h */
    0xfe00707f,  /* fminm_q */
    0xfe00707f,  /* fminm_s */
    0x600007f,   /* fmsub_d */
    0x600007f,   /* fmsub_h */
    0x600007f,   /* fmsub_q */
    0x600007f,   /* fmsub_s */
    0xfe00007f,  /* fmul_d */
    0xfe00007f,  /* fmul_h */
    0xfe00007f,  /* fmul_q */
    0xfe00007f,  /* fmul_s */
    0xfe00707f,  /* fmv_d */
    0xfff0707f,  /* fmv_d_x */
    0xfe00707f,  /* fmv_h */
    0xfff0707f,  /* fmv_h_x */
    0xfe00707f,  /* fmv_q */
    0xfe00707f,  /* fmv_s */
    0xfff0707f,  /* fmv_s_x */
    0xfff0707f,  /* fmv_w_x */
    0xfff0707f,  /* fmv_x_d */
    0xfff0707f,  /* fmv_x_h */
    0xfff0707f,  /* fmv_x_s */
    0xfff0707f,  /* fmv_x_w */
    0xffff_ffff, /* fmvh_x_d */
    0xfff0707f,  /* fmvh_x_q */
    0xffff_ffff, /* fmvp_d_x */
    0xfe00707f,  /* fmvp_q_x */
    0xfe00707f,  /* fneg_d */
    0xfe00707f,  /* fneg_h */
    0xfe00707f,  /* fneg_q */
    0xfe00707f,  /* fneg_s */
    0x600007f,   /* fnmadd_d */
    0x600007f,   /* fnmadd_h */
    0x600007f,   /* fnmadd_q */
    0x600007f,   /* fnmadd_s */
    0x600007f,   /* fnmsub_d */
    0x600007f,   /* fnmsub_h */
    0x600007f,   /* fnmsub_q */
    0x600007f,   /* fnmsub_s */
    0xfffff07f,  /* frcsr */
    0xfffff07f,  /* frflags */
    0xfff0007f,  /* fround_d */
    0xfff0007f,  /* fround_h */
    0xfff0007f,  /* fround_q */
    0xfff0007f,  /* fround_s */
    0xfff0007f,  /* froundnx_d */
    0xfff0007f,  /* froundnx_h */
    0xfff0007f,  /* froundnx_q */
    0xfff0007f,  /* froundnx_s */
    0xfffff07f,  /* frrm */
    0xfff0707f,  /* fscsr */
    0x707f,      /* fsd */
    0xfff0707f,  /* fsflags */
    0xfff0707f,  /* fsflagsi */
    0xfe00707f,  /* fsgnj_d */
    0xfe00707f,  /* fsgnj_h */
    0xfe00707f,  /* fsgnj_q */
    0xfe00707f,  /* fsgnj_s */
    0xfe00707f,  /* fsgnjn_d */
    0xfe00707f,  /* fsgnjn_h */
    0xfe00707f,  /* fsgnjn_q */
    0xfe00707f,  /* fsgnjn_s */
    0xfe00707f,  /* fsgnjx_d */
    0xfe00707f,  /* fsgnjx_h */
    0xfe00707f,  /* fsgnjx_q */
    0xfe00707f,  /* fsgnjx_s */
    0x707f,      /* fsh */
    0x707f,      /* fsq */
    0xfff0007f,  /* fsqrt_d */
    0xfff0007f,  /* fsqrt_h */
    0xfff0007f,  /* fsqrt_q */
    0xfff0007f,  /* fsqrt_s */
    0xfff0707f,  /* fsrm */
    0xfff0707f,  /* fsrmi */
    0xfe00007f,  /* fsub_d */
    0xfe00007f,  /* fsub_h */
    0xfe00007f,  /* fsub_q */
    0xfe00007f,  /* fsub_s */
    0x707f,      /* fsw */
    0xfe007fff,  /* hfence_gvma */
    0xfe007fff,  /* hfence_vvma */
    0xfe007fff,  /* hinval_gvma */
    0xfe007fff,  /* hinval_vvma */
    0xfff0707f,  /* hlv_b */
    0xfff0707f,  /* hlv_bu */
    0xfff0707f,  /* hlv_d */
    0xfff0707f,  /* hlv_h */
    0xfff0707f,  /* hlv_hu */
    0xfff0707f,  /* hlv_w */
    0xfff0707f,  /* hlv_wu */
    0xfff0707f,  /* hlvx_hu */
    0xfff0707f,  /* hlvx_wu */
    0xfe007fff,  /* hsv_b */
    0xfe007fff,  /* hsv_d */
    0xfe007fff,  /* hsv_h */
    0xfe007fff,  /* hsv_w */
    0xfff,       /* j */
    0x7f,        /* jal */
    0xfff,       /* jal_pseudo */
    0x707f,      /* jalr */
    0xfff07fff,  /* jalr_pseudo */
    0xfff07fff,  /* jr */
    0x707f,      /* lb */
    0x707f,      /* lbu */
    0x707f,      /* ld */
    0x707f,      /* lh */
    0x707f,      /* lhu */
    0xfff,       /* lpad */
    0xf9f0707f,  /* lr_d */
    0xf9f0707f,  /* lr_w */
    0x7f,        /* lui */
    0x707f,      /* lw */
    0x707f,      /* lwu */
    0xfe00707f,  /* max */
    0xfe00707f,  /* maxu */
    0xfe00707f,  /* min */
    0xfe00707f,  /* minu */
    0xffffffff,  /* mnret */
    0xfff0707f,  /* mop_r_0 */
    0xfff0707f,  /* mop_r_1 */
    0xfff0707f,  /* mop_r_10 */
    0xfff0707f,  /* mop_r_11 */
    0xfff0707f,  /* mop_r_12 */
    0xfff0707f,  /* mop_r_13 */
    0xfff0707f,  /* mop_r_14 */
    0xfff0707f,  /* mop_r_15 */
    0xfff0707f,  /* mop_r_16 */
    0xfff0707f,  /* mop_r_17 */
    0xfff0707f,  /* mop_r_18 */
    0xfff0707f,  /* mop_r_19 */
    0xfff0707f,  /* mop_r_2 */
    0xfff0707f,  /* mop_r_20 */
    0xfff0707f,  /* mop_r_21 */
    0xfff0707f,  /* mop_r_22 */
    0xfff0707f,  /* mop_r_23 */
    0xfff0707f,  /* mop_r_24 */
    0xfff0707f,  /* mop_r_25 */
    0xfff0707f,  /* mop_r_26 */
    0xfff0707f,  /* mop_r_27 */
    0xfff0707f,  /* mop_r_28 */
    0xfff0707f,  /* mop_r_29 */
    0xfff0707f,  /* mop_r_3 */
    0xfff0707f,  /* mop_r_30 */
    0xfff0707f,  /* mop_r_31 */
    0xfff0707f,  /* mop_r_4 */
    0xfff0707f,  /* mop_r_5 */
    0xfff0707f,  /* mop_r_6 */
    0xfff0707f,  /* mop_r_7 */
    0xfff0707f,  /* mop_r_8 */
    0xfff0707f,  /* mop_r_9 */
    0xb3c0707f,  /* mop_r_N */
    0xfe00707f,  /* mop_rr_0 */
    0xfe00707f,  /* mop_rr_1 */
    0xfe00707f,  /* mop_rr_2 */
    0xfe00707f,  /* mop_rr_3 */
    0xfe00707f,  /* mop_rr_4 */
    0xfe00707f,  /* mop_rr_5 */
    0xfe00707f,  /* mop_rr_6 */
    0xfe00707f,  /* mop_rr_7 */
    0xb200707f,  /* mop_rr_N */
    0xffffffff,  /* mret */
    0xfe00707f,  /* mul */
    0xfe00707f,  /* mulh */
    0xfe00707f,  /* mulhsu */
    0xfe00707f,  /* mulhu */
    0xfe00707f,  /* mulw */
    0xfff0707f,  /* mv */
    0xfff0707f,  /* neg */
    0xffffffff,  /* nop */
    0xffffffff,  /* ntl_all */
    0xffffffff,  /* ntl_p1 */
    0xffffffff,  /* ntl_pall */
    0xffffffff,  /* ntl_s1 */
    0xfe00707f,  /* or */
    0xfff0707f,  /* orc_b */
    0x707f,      /* ori */
    0xfe00707f,  /* orn */
    0xfe00707f,  /* pack */
    0xfe00707f,  /* packh */
    0xfe00707f,  /* packw */
    0xffffffff,  /* pause */
    0x1f07fff,   /* prefetch_i */
    0x1f07fff,   /* prefetch_r */
    0x1f07fff,   /* prefetch_w */
    0xfffff07f,  /* rdcycle */
    0xffff_ffff, /* rdcycleh */
    0xfffff07f,  /* rdinstret */
    0xffff_ffff, /* rdinstreth */
    0xfffff07f,  /* rdtime */
    0xffff_ffff, /* rdtimeh */
    0xfe00707f,  /* rem */
    0xfe00707f,  /* remu */
    0xfe00707f,  /* remuw */
    0xfe00707f,  /* remw */
    0xffffffff,  /* ret */
    0xfff0707f,  /* rev8 */
    0xffff_ffff, /* rev8_rv32 */
    0xfe00707f,  /* rol */
    0xfe00707f,  /* rolw */
    0xfe00707f,  /* ror */
    0xfc00707f,  /* rori */
    0xffff_ffff, /* rori_rv32 */
    0xfe00707f,  /* roriw */
    0xfe00707f,  /* rorw */
    0x707f,      /* sb */
    0xffffffff,  /* sbreak */
    0xf800707f,  /* sc_d */
    0xf800707f,  /* sc_w */
    0xffffffff,  /* scall */
    0xffffffff,  /* sctrclr */
    0x707f,      /* sd */
    0xfff0707f,  /* seqz */
    0xfff0707f,  /* sext_b */
    0xfff0707f,  /* sext_h */
    0xfff0707f,  /* sext_w */
    0xffffffff,  /* sfence_inval_ir */
    0xfe007fff,  /* sfence_vma */
    0xffffffff,  /* sfence_w_inval */
    0xfe0ff07f,  /* sgtz */
    0x707f,      /* sh */
    0xfe00707f,  /* sh1add */
    0xfe00707f,  /* sh1add_uw */
    0xfe00707f,  /* sh2add */
    0xfe00707f,  /* sh2add_uw */
    0xfe00707f,  /* sh3add */
    0xfe00707f,  /* sh3add_uw */
    0xfff0707f,  /* sha256sig0 */
    0xfff0707f,  /* sha256sig1 */
    0xfff0707f,  /* sha256sum0 */
    0xfff0707f,  /* sha256sum1 */
    0xfff0707f,  /* sha512sig0 */
    0xffff_ffff, /* sha512sig0h */
    0xffff_ffff, /* sha512sig0l */
    0xfff0707f,  /* sha512sig1 */
    0xffff_ffff, /* sha512sig1h */
    0xffff_ffff, /* sha512sig1l */
    0xfff0707f,  /* sha512sum0 */
    0xffff_ffff, /* sha512sum0r */
    0xfff0707f,  /* sha512sum1 */
    0xffff_ffff, /* sha512sum1r */
    0xfe007fff,  /* sinval_vma */
    0xfe00707f,  /* sll */
    0xfc00707f,  /* slli */
    0xffff_ffff, /* slli_rv32 */
    0xfc00707f,  /* slli_uw */
    0xfe00707f,  /* slliw */
    0xfe00707f,  /* sllw */
    0xfe00707f,  /* slt */
    0x707f,      /* slti */
    0x707f,      /* sltiu */
    0xfe00707f,  /* sltu */
    0xfff0707f,  /* sltz */
    0xfff0707f,  /* sm3p0 */
    0xfff0707f,  /* sm3p1 */
    0x3e00707f,  /* sm4ed */
    0x3e00707f,  /* sm4ks */
    0xfe0ff07f,  /* snez */
    0xfe00707f,  /* sra */
    0xfc00707f,  /* srai */
    0xffff_ffff, /* srai_rv32 */
    0xfe00707f,  /* sraiw */
    0xfe00707f,  /* sraw */
    0xffffffff,  /* sret */
    0xfe00707f,  /* srl */
    0xfc00707f,  /* srli */
    0xffff_ffff, /* srli_rv32 */
    0xfe00707f,  /* srliw */
    0xfe00707f,  /* srlw */
    0xf800707f,  /* ssamoswap_d */
    0xf800707f,  /* ssamoswap_w */
    0xffffffff,  /* sspopchk_x1 */
    0xffffffff,  /* sspopchk_x5 */
    0xffffffff,  /* sspush_x1 */
    0xffffffff,  /* sspush_x5 */
    0xfffff07f,  /* ssrdp */
    0xfe00707f,  /* sub */
    0xfe00707f,  /* subw */
    0x707f,      /* sw */
    0xffff_ffff, /* unzip */
    0xfc00707f,  /* vaadd_vv */
    0xfc00707f,  /* vaadd_vx */
    0xfc00707f,  /* vaaddu_vv */
    0xfc00707f,  /* vaaddu_vx */
    0xfe00707f,  /* vadc_vim */
    0xfe00707f,  /* vadc_vvm */
    0xfe00707f,  /* vadc_vxm */
    0xfc00707f,  /* vadd_vi */
    0xfc00707f,  /* vadd_vv */
    0xfc00707f,  /* vadd_vx */
    0xfe0ff07f,  /* vaesdf_vs */
    0xfe0ff07f,  /* vaesdf_vv */
    0xfe0ff07f,  /* vaesdm_vs */
    0xfe0ff07f,  /* vaesdm_vv */
    0xfe0ff07f,  /* vaesef_vs */
    0xfe0ff07f,  /* vaesef_vv */
    0xfe0ff07f,  /* vaesem_vs */
    0xfe0ff07f,  /* vaesem_vv */
    0xfe00707f,  /* vaeskf1_vi */
    0xfe00707f,  /* vaeskf2_vi */
    0xfe0ff07f,  /* vaesz_vs */
    0xfc00707f,  /* vand_vi */
    0xfc00707f,  /* vand_vv */
    0xfc00707f,  /* vand_vx */
    0xfc00707f,  /* vandn_vv */
    0xfc00707f,  /* vandn_vx */
    0xfc00707f,  /* vasub_vv */
    0xfc00707f,  /* vasub_vx */
    0xfc00707f,  /* vasubu_vv */
    0xfc00707f,  /* vasubu_vx */
    0xfc0ff07f,  /* vbrev8_v */
    0xfc0ff07f,  /* vbrev_v */
    0xfc00707f,  /* vclmul_vv */
    0xfc00707f,  /* vclmul_vx */
    0xfc00707f,  /* vclmulh_vv */
    0xfc00707f,  /* vclmulh_vx */
    0xfc0ff07f,  /* vclz_v */
    0xfe00707f,  /* vcompress_vm */
    0xfc0ff07f,  /* vcpop_m */
    0xfc0ff07f,  /* vcpop_v */
    0xfc0ff07f,  /* vctz_v */
    0xfc00707f,  /* vdiv_vv */
    0xfc00707f,  /* vdiv_vx */
    0xfc00707f,  /* vdivu_vv */
    0xfc00707f,  /* vdivu_vx */
    0xfc00707f,  /* vfadd_vf */
    0xfc00707f,  /* vfadd_vv */
    0xfc0ff07f,  /* vfclass_v */
    0xfc0ff07f,  /* vfcvt_f_x_v */
    0xfc0ff07f,  /* vfcvt_f_xu_v */
    0xfc0ff07f,  /* vfcvt_rtz_x_f_v */
    0xfc0ff07f,  /* vfcvt_rtz_xu_f_v */
    0xfc0ff07f,  /* vfcvt_x_f_v */
    0xfc0ff07f,  /* vfcvt_xu_f_v */
    0xfc00707f,  /* vfdiv_vf */
    0xfc00707f,  /* vfdiv_vv */
    0xfc0ff07f,  /* vfirst_m */
    0xfc00707f,  /* vfmacc_vf */
    0xfc00707f,  /* vfmacc_vv */
    0xfc00707f,  /* vfmadd_vf */
    0xfc00707f,  /* vfmadd_vv */
    0xfc00707f,  /* vfmax_vf */
    0xfc00707f,  /* vfmax_vv */
    0xfe00707f,  /* vfmerge_vfm */
    0xfc00707f,  /* vfmin_vf */
    0xfc00707f,  /* vfmin_vv */
    0xfc00707f,  /* vfmsac_vf */
    0xfc00707f,  /* vfmsac_vv */
    0xfc00707f,  /* vfmsub_vf */
    0xfc00707f,  /* vfmsub_vv */
    0xfc00707f,  /* vfmul_vf */
    0xfc00707f,  /* vfmul_vv */
    0xfe0ff07f,  /* vfmv_f_s */
    0xfff0707f,  /* vfmv_s_f */
    0xfff0707f,  /* vfmv_v_f */
    0xfc0ff07f,  /* vfncvt_f_f_w */
    0xfc0ff07f,  /* vfncvt_f_x_w */
    0xfc0ff07f,  /* vfncvt_f_xu_w */
    0xfc0ff07f,  /* vfncvt_rod_f_f_w */
    0xfc0ff07f,  /* vfncvt_rtz_x_f_w */
    0xfc0ff07f,  /* vfncvt_rtz_xu_f_w */
    0xfc0ff07f,  /* vfncvt_x_f_w */
    0xfc0ff07f,  /* vfncvt_xu_f_w */
    0xfc0ff07f,  /* vfncvtbf16_f_f_w */
    0xfc00707f,  /* vfnmacc_vf */
    0xfc00707f,  /* vfnmacc_vv */
    0xfc00707f,  /* vfnmadd_vf */
    0xfc00707f,  /* vfnmadd_vv */
    0xfc00707f,  /* vfnmsac_vf */
    0xfc00707f,  /* vfnmsac_vv */
    0xfc00707f,  /* vfnmsub_vf */
    0xfc00707f,  /* vfnmsub_vv */
    0xfc00707f,  /* vfrdiv_vf */
    0xfc0ff07f,  /* vfrec7_v */
    0xfc00707f,  /* vfredmax_vs */
    0xfc00707f,  /* vfredmin_vs */
    0xfc00707f,  /* vfredosum_vs */
    0xfc00707f,  /* vfredsum_vs */
    0xfc00707f,  /* vfredusum_vs */
    0xfc0ff07f,  /* vfrsqrt7_v */
    0xfc00707f,  /* vfrsub_vf */
    0xfc00707f,  /* vfsgnj_vf */
    0xfc00707f,  /* vfsgnj_vv */
    0xfc00707f,  /* vfsgnjn_vf */
    0xfc00707f,  /* vfsgnjn_vv */
    0xfc00707f,  /* vfsgnjx_vf */
    0xfc00707f,  /* vfsgnjx_vv */
    0xfc00707f,  /* vfslide1down_vf */
    0xfc00707f,  /* vfslide1up_vf */
    0xfc0ff07f,  /* vfsqrt_v */
    0xfc00707f,  /* vfsub_vf */
    0xfc00707f,  /* vfsub_vv */
    0xfc00707f,  /* vfwadd_vf */
    0xfc00707f,  /* vfwadd_vv */
    0xfc00707f,  /* vfwadd_wf */
    0xfc00707f,  /* vfwadd_wv */
    0xfc0ff07f,  /* vfwcvt_f_f_v */
    0xfc0ff07f,  /* vfwcvt_f_x_v */
    0xfc0ff07f,  /* vfwcvt_f_xu_v */
    0xfc0ff07f,  /* vfwcvt_rtz_x_f_v */
    0xfc0ff07f,  /* vfwcvt_rtz_xu_f_v */
    0xfc0ff07f,  /* vfwcvt_x_f_v */
    0xfc0ff07f,  /* vfwcvt_xu_f_v */
    0xfc0ff07f,  /* vfwcvtbf16_f_f_v */
    0xfc00707f,  /* vfwmacc_vf */
    0xfc00707f,  /* vfwmacc_vv */
    0xfc00707f,  /* vfwmaccbf16_vf */
    0xfc00707f,  /* vfwmaccbf16_vv */
    0xfc00707f,  /* vfwmsac_vf */
    0xfc00707f,  /* vfwmsac_vv */
    0xfc00707f,  /* vfwmul_vf */
    0xfc00707f,  /* vfwmul_vv */
    0xfc00707f,  /* vfwnmacc_vf */
    0xfc00707f,  /* vfwnmacc_vv */
    0xfc00707f,  /* vfwnmsac_vf */
    0xfc00707f,  /* vfwnmsac_vv */
    0xfc00707f,  /* vfwredosum_vs */
    0xfc00707f,  /* vfwredsum_vs */
    0xfc00707f,  /* vfwredusum_vs */
    0xfc00707f,  /* vfwsub_vf */
    0xfc00707f,  /* vfwsub_vv */
    0xfc00707f,  /* vfwsub_wf */
    0xfc00707f,  /* vfwsub_wv */
    0xfe00707f,  /* vghsh_vv */
    0xfe0ff07f,  /* vgmul_vv */
    0xfdfff07f,  /* vid_v */
    0xfc0ff07f,  /* viota_m */
    0xfff0707f,  /* vl1r_v */
    0xfff0707f,  /* vl1re16_v */
    0xfff0707f,  /* vl1re32_v */
    0xfff0707f,  /* vl1re64_v */
    0xfff0707f,  /* vl1re8_v */
    0xfff0707f,  /* vl2r_v */
    0xfff0707f,  /* vl2re16_v */
    0xfff0707f,  /* vl2re32_v */
    0xfff0707f,  /* vl2re64_v */
    0xfff0707f,  /* vl2re8_v */
    0xfff0707f,  /* vl4r_v */
    0xfff0707f,  /* vl4re16_v */
    0xfff0707f,  /* vl4re32_v */
    0xfff0707f,  /* vl4re64_v */
    0xfff0707f,  /* vl4re8_v */
    0xfff0707f,  /* vl8r_v */
    0xfff0707f,  /* vl8re16_v */
    0xfff0707f,  /* vl8re32_v */
    0xfff0707f,  /* vl8re64_v */
    0xfff0707f,  /* vl8re8_v */
    0x1df0707f,  /* vle16_v */
    0x1df0707f,  /* vle16ff_v */
    0xfff0707f,  /* vle1_v */
    0x1df0707f,  /* vle32_v */
    0x1df0707f,  /* vle32ff_v */
    0x1df0707f,  /* vle64_v */
    0x1df0707f,  /* vle64ff_v */
    0x1df0707f,  /* vle8_v */
    0x1df0707f,  /* vle8ff_v */
    0xfff0707f,  /* vlm_v */
    0x1c00707f,  /* vloxei16_v */
    0x1c00707f,  /* vloxei32_v */
    0x1c00707f,  /* vloxei64_v */
    0x1c00707f,  /* vloxei8_v */
    0x1c00707f,  /* vlse16_v */
    0x1c00707f,  /* vlse32_v */
    0x1c00707f,  /* vlse64_v */
    0x1c00707f,  /* vlse8_v */
    0x1c00707f,  /* vluxei16_v */
    0x1c00707f,  /* vluxei32_v */
    0x1c00707f,  /* vluxei64_v */
    0x1c00707f,  /* vluxei8_v */
    0xfc00707f,  /* vmacc_vv */
    0xfc00707f,  /* vmacc_vx */
    0xfe00707f,  /* vmadc_vi */
    0xfe00707f,  /* vmadc_vim */
    0xfe00707f,  /* vmadc_vv */
    0xfe00707f,  /* vmadc_vvm */
    0xfe00707f,  /* vmadc_vx */
    0xfe00707f,  /* vmadc_vxm */
    0xfc00707f,  /* vmadd_vv */
    0xfc00707f,  /* vmadd_vx */
    0xfe00707f,  /* vmand_mm */
    0xfe00707f,  /* vmandn_mm */
    0xfc00707f,  /* vmandnot_mm */
    0xfc00707f,  /* vmax_vv */
    0xfc00707f,  /* vmax_vx */
    0xfc00707f,  /* vmaxu_vv */
    0xfc00707f,  /* vmaxu_vx */
    0xfe00707f,  /* vmerge_vim */
    0xfe00707f,  /* vmerge_vvm */
    0xfe00707f,  /* vmerge_vxm */
    0xfc00707f,  /* vmfeq_vf */
    0xfc00707f,  /* vmfeq_vv */
    0xfc00707f,  /* vmfge_vf */
    0xfc00707f,  /* vmfgt_vf */
    0xfc00707f,  /* vmfle_vf */
    0xfc00707f,  /* vmfle_vv */
    0xfc00707f,  /* vmflt_vf */
    0xfc00707f,  /* vmflt_vv */
    0xfc00707f,  /* vmfne_vf */
    0xfc00707f,  /* vmfne_vv */
    0xfc00707f,  /* vmin_vv */
    0xfc00707f,  /* vmin_vx */
    0xfc00707f,  /* vminu_vv */
    0xfc00707f,  /* vminu_vx */
    0xfe00707f,  /* vmnand_mm */
    0xfe00707f,  /* vmnor_mm */
    0xfe00707f,  /* vmor_mm */
    0xfe00707f,  /* vmorn_mm */
    0xfc00707f,  /* vmornot_mm */
    0xfe00707f,  /* vmsbc_vv */
    0xfe00707f,  /* vmsbc_vvm */
    0xfe00707f,  /* vmsbc_vx */
    0xfe00707f,  /* vmsbc_vxm */
    0xfc0ff07f,  /* vmsbf_m */
    0xfc00707f,  /* vmseq_vi */
    0xfc00707f,  /* vmseq_vv */
    0xfc00707f,  /* vmseq_vx */
    0xfc00707f,  /* vmsgt_vi */
    0xfc00707f,  /* vmsgt_vx */
    0xfc00707f,  /* vmsgtu_vi */
    0xfc00707f,  /* vmsgtu_vx */
    0xfc0ff07f,  /* vmsif_m */
    0xfc00707f,  /* vmsle_vi */
    0xfc00707f,  /* vmsle_vv */
    0xfc00707f,  /* vmsle_vx */
    0xfc00707f,  /* vmsleu_vi */
    0xfc00707f,  /* vmsleu_vv */
    0xfc00707f,  /* vmsleu_vx */
    0xfc00707f,  /* vmslt_vv */
    0xfc00707f,  /* vmslt_vx */
    0xfc00707f,  /* vmsltu_vv */
    0xfc00707f,  /* vmsltu_vx */
    0xfc00707f,  /* vmsne_vi */
    0xfc00707f,  /* vmsne_vv */
    0xfc00707f,  /* vmsne_vx */
    0xfc0ff07f,  /* vmsof_m */
    0xfc00707f,  /* vmul_vv */
    0xfc00707f,  /* vmul_vx */
    0xfc00707f,  /* vmulh_vv */
    0xfc00707f,  /* vmulh_vx */
    0xfc00707f,  /* vmulhsu_vv */
    0xfc00707f,  /* vmulhsu_vx */
    0xfc00707f,  /* vmulhu_vv */
    0xfc00707f,  /* vmulhu_vx */
    0xfe0ff07f,  /* vmv1r_v */
    0xfe0ff07f,  /* vmv2r_v */
    0xfe0ff07f,  /* vmv4r_v */
    0xfe0ff07f,  /* vmv8r_v */
    0xfff0707f,  /* vmv_s_x */
    0xfff0707f,  /* vmv_v_i */
    0xfff0707f,  /* vmv_v_v */
    0xfff0707f,  /* vmv_v_x */
    0xfe0ff07f,  /* vmv_x_s */
    0xfe00707f,  /* vmxnor_mm */
    0xfe00707f,  /* vmxor_mm */
    0xfc00707f,  /* vnclip_wi */
    0xfc00707f,  /* vnclip_wv */
    0xfc00707f,  /* vnclip_wx */
    0xfc00707f,  /* vnclipu_wi */
    0xfc00707f,  /* vnclipu_wv */
    0xfc00707f,  /* vnclipu_wx */
    0xfc00707f,  /* vnmsac_vv */
    0xfc00707f,  /* vnmsac_vx */
    0xfc00707f,  /* vnmsub_vv */
    0xfc00707f,  /* vnmsub_vx */
    0xfc00707f,  /* vnsra_wi */
    0xfc00707f,  /* vnsra_wv */
    0xfc00707f,  /* vnsra_wx */
    0xfc00707f,  /* vnsrl_wi */
    0xfc00707f,  /* vnsrl_wv */
    0xfc00707f,  /* vnsrl_wx */
    0xfc00707f,  /* vor_vi */
    0xfc00707f,  /* vor_vv */
    0xfc00707f,  /* vor_vx */
    0xfc0ff07f,  /* vpopc_m */
    0xfc00707f,  /* vredand_vs */
    0xfc00707f,  /* vredmax_vs */
    0xfc00707f,  /* vredmaxu_vs */
    0xfc00707f,  /* vredmin_vs */
    0xfc00707f,  /* vredminu_vs */
    0xfc00707f,  /* vredor_vs */
    0xfc00707f,  /* vredsum_vs */
    0xfc00707f,  /* vredxor_vs */
    0xfc00707f,  /* vrem_vv */
    0xfc00707f,  /* vrem_vx */
    0xfc00707f,  /* vremu_vv */
    0xfc00707f,  /* vremu_vx */
    0xfc0ff07f,  /* vrev8_v */
    0xfc00707f,  /* vrgather_vi */
    0xfc00707f,  /* vrgather_vv */
    0xfc00707f,  /* vrgather_vx */
    0xfc00707f,  /* vrgatherei16_vv */
    0xfc00707f,  /* vrol_vv */
    0xfc00707f,  /* vrol_vx */
    0xf800707f,  /* vror_vi */
    0xfc00707f,  /* vror_vv */
    0xfc00707f,  /* vror_vx */
    0xfc00707f,  /* vrsub_vi */
    0xfc00707f,  /* vrsub_vx */
    0xfff0707f,  /* vs1r_v */
    0xfff0707f,  /* vs2r_v */
    0xfff0707f,  /* vs4r_v */
    0xfff0707f,  /* vs8r_v */
    0xfc00707f,  /* vsadd_vi */
    0xfc00707f,  /* vsadd_vv */
    0xfc00707f,  /* vsadd_vx */
    0xfc00707f,  /* vsaddu_vi */
    0xfc00707f,  /* vsaddu_vv */
    0xfc00707f,  /* vsaddu_vx */
    0xfe00707f,  /* vsbc_vvm */
    0xfe00707f,  /* vsbc_vxm */
    0x1df0707f,  /* vse16_v */
    0xfff0707f,  /* vse1_v */
    0x1df0707f,  /* vse32_v */
    0x1df0707f,  /* vse64_v */
    0x1df0707f,  /* vse8_v */
    0xc000707f,  /* vsetivli */
    0xfe00707f,  /* vsetvl */
    0x8000707f,  /* vsetvli */
    0xfc0ff07f,  /* vsext_vf2 */
    0xfc0ff07f,  /* vsext_vf4 */
    0xfc0ff07f,  /* vsext_vf8 */
    0xfe00707f,  /* vsha2ch_vv */
    0xfe00707f,  /* vsha2cl_vv */
    0xfe00707f,  /* vsha2ms_vv */
    0xfc00707f,  /* vslide1down_vx */
    0xfc00707f,  /* vslide1up_vx */
    0xfc00707f,  /* vslidedown_vi */
    0xfc00707f,  /* vslidedown_vx */
    0xfc00707f,  /* vslideup_vi */
    0xfc00707f,  /* vslideup_vx */
    0xfc00707f,  /* vsll_vi */
    0xfc00707f,  /* vsll_vv */
    0xfc00707f,  /* vsll_vx */
    0xfe00707f,  /* vsm3c_vi */
    0xfe00707f,  /* vsm3me_vv */
    0xfe00707f,  /* vsm4k_vi */
    0xfe0ff07f,  /* vsm4r_vs */
    0xfe0ff07f,  /* vsm4r_vv */
    0xfff0707f,  /* vsm_v */
    0xfc00707f,  /* vsmul_vv */
    0xfc00707f,  /* vsmul_vx */
    0x1c00707f,  /* vsoxei16_v */
    0x1c00707f,  /* vsoxei32_v */
    0x1c00707f,  /* vsoxei64_v */
    0x1c00707f,  /* vsoxei8_v */
    0xfc00707f,  /* vsra_vi */
    0xfc00707f,  /* vsra_vv */
    0xfc00707f,  /* vsra_vx */
    0xfc00707f,  /* vsrl_vi */
    0xfc00707f,  /* vsrl_vv */
    0xfc00707f,  /* vsrl_vx */
    0x1c00707f,  /* vsse16_v */
    0x1c00707f,  /* vsse32_v */
    0x1c00707f,  /* vsse64_v */
    0x1c00707f,  /* vsse8_v */
    0xfc00707f,  /* vssra_vi */
    0xfc00707f,  /* vssra_vv */
    0xfc00707f,  /* vssra_vx */
    0xfc00707f,  /* vssrl_vi */
    0xfc00707f,  /* vssrl_vv */
    0xfc00707f,  /* vssrl_vx */
    0xfc00707f,  /* vssub_vv */
    0xfc00707f,  /* vssub_vx */
    0xfc00707f,  /* vssubu_vv */
    0xfc00707f,  /* vssubu_vx */
    0xfc00707f,  /* vsub_vv */
    0xfc00707f,  /* vsub_vx */
    0x1c00707f,  /* vsuxei16_v */
    0x1c00707f,  /* vsuxei32_v */
    0x1c00707f,  /* vsuxei64_v */
    0x1c00707f,  /* vsuxei8_v */
    0xfc00707f,  /* vwadd_vv */
    0xfc00707f,  /* vwadd_vx */
    0xfc00707f,  /* vwadd_wv */
    0xfc00707f,  /* vwadd_wx */
    0xfc00707f,  /* vwaddu_vv */
    0xfc00707f,  /* vwaddu_vx */
    0xfc00707f,  /* vwaddu_wv */
    0xfc00707f,  /* vwaddu_wx */
    0xfc00707f,  /* vwmacc_vv */
    0xfc00707f,  /* vwmacc_vx */
    0xfc00707f,  /* vwmaccsu_vv */
    0xfc00707f,  /* vwmaccsu_vx */
    0xfc00707f,  /* vwmaccu_vv */
    0xfc00707f,  /* vwmaccu_vx */
    0xfc00707f,  /* vwmaccus_vx */
    0xfc00707f,  /* vwmul_vv */
    0xfc00707f,  /* vwmul_vx */
    0xfc00707f,  /* vwmulsu_vv */
    0xfc00707f,  /* vwmulsu_vx */
    0xfc00707f,  /* vwmulu_vv */
    0xfc00707f,  /* vwmulu_vx */
    0xfc00707f,  /* vwredsum_vs */
    0xfc00707f,  /* vwredsumu_vs */
    0xfc00707f,  /* vwsll_vi */
    0xfc00707f,  /* vwsll_vv */
    0xfc00707f,  /* vwsll_vx */
    0xfc00707f,  /* vwsub_vv */
    0xfc00707f,  /* vwsub_vx */
    0xfc00707f,  /* vwsub_wv */
    0xfc00707f,  /* vwsub_wx */
    0xfc00707f,  /* vwsubu_vv */
    0xfc00707f,  /* vwsubu_vx */
    0xfc00707f,  /* vwsubu_wv */
    0xfc00707f,  /* vwsubu_wx */
    0xfc00707f,  /* vxor_vi */
    0xfc00707f,  /* vxor_vv */
    0xfc00707f,  /* vxor_vx */
    0xfc0ff07f,  /* vzext_vf2 */
    0xfc0ff07f,  /* vzext_vf4 */
    0xfc0ff07f,  /* vzext_vf8 */
    0xffffffff,  /* wfi */
    0xffffffff,  /* wrs_nto */
    0xffffffff,  /* wrs_sto */
    0xfe00707f,  /* xnor */
    0xfe00707f,  /* xor */
    0x707f,      /* xori */
    0xfe00707f,  /* xperm4 */
    0xfe00707f,  /* xperm8 */
    0xfff0707f,  /* zext_b */
    0xfff0707f,  /* zext_h */
    0xffff_ffff, /* zext_h_rv32 */
    0xfff0707f,  /* zext_w */
    0xffff_ffff, /* zip */
];
pub static OPCODE_MATCH: [u32; 1039] = [
    0x33,       /* add */
    0x800003b,  /* add_uw */
    0x13,       /* addi */
    0x1b,       /* addiw */
    0x3b,       /* addw */
    0x2a000033, /* aes32dsi */
    0x2e000033, /* aes32dsmi */
    0x22000033, /* aes32esi */
    0x26000033, /* aes32esmi */
    0x3a000033, /* aes64ds */
    0x3e000033, /* aes64dsm */
    0x32000033, /* aes64es */
    0x36000033, /* aes64esm */
    0x30001013, /* aes64im */
    0x31001013, /* aes64ks1i */
    0x7e000033, /* aes64ks2 */
    0x2f,       /* amoadd_b */
    0x302f,     /* amoadd_d */
    0x102f,     /* amoadd_h */
    0x202f,     /* amoadd_w */
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
    0x800002f,  /* amoswap_b */
    0x800302f,  /* amoswap_d */
    0x800102f,  /* amoswap_h */
    0x800202f,  /* amoswap_w */
    0x2000002f, /* amoxor_b */
    0x2000302f, /* amoxor_d */
    0x2000102f, /* amoxor_h */
    0x2000202f, /* amoxor_w */
    0x7033,     /* and */
    0x7013,     /* andi */
    0x40007033, /* andn */
    0x17,       /* auipc */
    0x48001033, /* bclr */
    0x48001013, /* bclri */
    0x48001013, /* bclri_rv32 */
    0x63,       /* beq */
    0x63,       /* beqz */
    0x48005033, /* bext */
    0x48005013, /* bexti */
    0x48005013, /* bexti_rv32 */
    0x5063,     /* bge */
    0x7063,     /* bgeu */
    0x5063,     /* bgez */
    0x4063,     /* bgt */
    0x6063,     /* bgtu */
    0x4063,     /* bgtz */
    0x68001033, /* binv */
    0x68001013, /* binvi */
    0x68001013, /* binvi_rv32 */
    0x5063,     /* ble */
    0x7063,     /* bleu */
    0x5063,     /* blez */
    0x4063,     /* blt */
    0x6063,     /* bltu */
    0x4063,     /* bltz */
    0x1063,     /* bne */
    0x1063,     /* bnez */
    0x68705013, /* brev8 */
    0x28001033, /* bset */
    0x28001013, /* bseti */
    0x28001013, /* bseti_rv32 */
    0x9002,     /* c_add */
    0x1,        /* c_addi */
    0x6101,     /* c_addi16sp */
    0x0,        /* c_addi4spn */
    0x2001,     /* c_addiw */
    0x9c21,     /* c_addw */
    0x8c61,     /* c_and */
    0x8801,     /* c_andi */
    0xc001,     /* c_beqz */
    0xe001,     /* c_bnez */
    0x9002,     /* c_ebreak */
    0x2000,     /* c_fld */
    0x2002,     /* c_fldsp */
    0x6000,     /* c_flw */
    0x6002,     /* c_flwsp */
    0xa000,     /* c_fsd */
    0xa002,     /* c_fsdsp */
    0xe000,     /* c_fsw */
    0xe002,     /* c_fswsp */
    0xa001,     /* c_j */
    0x2001,     /* c_jal */
    0x9002,     /* c_jalr */
    0x8002,     /* c_jr */
    0x8000,     /* c_lbu */
    0x6000,     /* c_ld */
    0x6002,     /* c_ldsp */
    0x8440,     /* c_lh */
    0x8400,     /* c_lhu */
    0x4001,     /* c_li */
    0x6001,     /* c_lui */
    0x4000,     /* c_lw */
    0x4002,     /* c_lwsp */
    0x6081,     /* c_mop_1 */
    0x6581,     /* c_mop_11 */
    0x6681,     /* c_mop_13 */
    0x6781,     /* c_mop_15 */
    0x6181,     /* c_mop_3 */
    0x6281,     /* c_mop_5 */
    0x6381,     /* c_mop_7 */
    0x6481,     /* c_mop_9 */
    0x6081,     /* c_mop_N */
    0x9c41,     /* c_mul */
    0x8002,     /* c_mv */
    0x1,        /* c_nop */
    0x9c75,     /* c_not */
    0x9016,     /* c_ntl_all */
    0x900a,     /* c_ntl_p1 */
    0x900e,     /* c_ntl_pall */
    0x9012,     /* c_ntl_s1 */
    0x8c41,     /* c_or */
    0x8800,     /* c_sb */
    0xe000,     /* c_sd */
    0xe002,     /* c_sdsp */
    0x9c65,     /* c_sext_b */
    0x9c6d,     /* c_sext_h */
    0x2001,     /* c_sext_w */
    0x8c00,     /* c_sh */
    0x2,        /* c_slli */
    0x2,        /* c_slli_rv32 */
    0x8401,     /* c_srai */
    0x8401,     /* c_srai_rv32 */
    0x8001,     /* c_srli */
    0x8001,     /* c_srli_rv32 */
    0x6281,     /* c_sspopchk_x5 */
    0x6081,     /* c_sspush_x1 */
    0x8c01,     /* c_sub */
    0x9c01,     /* c_subw */
    0xc000,     /* c_sw */
    0xc002,     /* c_swsp */
    0x8c21,     /* c_xor */
    0x9c61,     /* c_zext_b */
    0x9c69,     /* c_zext_h */
    0x9c71,     /* c_zext_w */
    0x10200f,   /* cbo_clean */
    0x20200f,   /* cbo_flush */
    0x200f,     /* cbo_inval */
    0x40200f,   /* cbo_zero */
    0xa001033,  /* clmul */
    0xa003033,  /* clmulh */
    0xa002033,  /* clmulr */
    0x60001013, /* clz */
    0x6000101b, /* clzw */
    0xa002,     /* cm_jalt */
    0xac62,     /* cm_mva01s */
    0xac22,     /* cm_mvsa01 */
    0xba02,     /* cm_pop */
    0xbe02,     /* cm_popret */
    0xbc02,     /* cm_popretz */
    0xb802,     /* cm_push */
    0x60201013, /* cpop */
    0x6020101b, /* cpopw */
    0x3073,     /* csrc */
    0x7073,     /* csrci */
    0x2073,     /* csrr */
    0x3073,     /* csrrc */
    0x7073,     /* csrrci */
    0x2073,     /* csrrs */
    0x6073,     /* csrrsi */
    0x1073,     /* csrrw */
    0x5073,     /* csrrwi */
    0x2073,     /* csrs */
    0x6073,     /* csrsi */
    0x1073,     /* csrw */
    0x5073,     /* csrwi */
    0x60101013, /* ctz */
    0x6010101b, /* ctzw */
    0xe005033,  /* czero_eqz */
    0xe007033,  /* czero_nez */
    0x2004033,  /* div */
    0x2005033,  /* divu */
    0x200503b,  /* divuw */
    0x200403b,  /* divw */
    0x7b200073, /* dret */
    0x100073,   /* ebreak */
    0x73,       /* ecall */
    0x22002053, /* fabs_d */
    0x24002053, /* fabs_h */
    0x26002053, /* fabs_q */
    0x20002053, /* fabs_s */
    0x2000053,  /* fadd_d */
    0x4000053,  /* fadd_h */
    0x6000053,  /* fadd_q */
    0x53,       /* fadd_s */
    0xe2001053, /* fclass_d */
    0xe4001053, /* fclass_h */
    0xe6001053, /* fclass_q */
    0xe0001053, /* fclass_s */
    0x44800053, /* fcvt_bf16_s */
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
    0x40600053, /* fcvt_s_bf16 */
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
    0xf,        /* fence */
    0x100f,     /* fence_i */
    0x8330000f, /* fence_tso */
    0xa2002053, /* feq_d */
    0xa4002053, /* feq_h */
    0xa6002053, /* feq_q */
    0xa0002053, /* feq_s */
    0x3007,     /* fld */
    0xa2000053, /* fle_d */
    0xa4000053, /* fle_h */
    0xa6000053, /* fle_q */
    0xa0000053, /* fle_s */
    0xa2004053, /* fleq_d */
    0xa4004053, /* fleq_h */
    0xa6004053, /* fleq_q */
    0xa0004053, /* fleq_s */
    0x1007,     /* flh */
    0xf2100053, /* fli_d */
    0xf4100053, /* fli_h */
    0xf6100053, /* fli_q */
    0xf0100053, /* fli_s */
    0x4007,     /* flq */
    0xa2001053, /* flt_d */
    0xa4001053, /* flt_h */
    0xa6001053, /* flt_q */
    0xa0001053, /* flt_s */
    0xa2005053, /* fltq_d */
    0xa4005053, /* fltq_h */
    0xa6005053, /* fltq_q */
    0xa0005053, /* fltq_s */
    0x2007,     /* flw */
    0x2000043,  /* fmadd_d */
    0x4000043,  /* fmadd_h */
    0x6000043,  /* fmadd_q */
    0x43,       /* fmadd_s */
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
    0x2000047,  /* fmsub_d */
    0x4000047,  /* fmsub_h */
    0x6000047,  /* fmsub_q */
    0x47,       /* fmsub_s */
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
    0xe2100053, /* fmvh_x_d */
    0xe6100053, /* fmvh_x_q */
    0xb2000053, /* fmvp_d_x */
    0xb6000053, /* fmvp_q_x */
    0x22001053, /* fneg_d */
    0x24001053, /* fneg_h */
    0x26001053, /* fneg_q */
    0x20001053, /* fneg_s */
    0x200004f,  /* fnmadd_d */
    0x400004f,  /* fnmadd_h */
    0x600004f,  /* fnmadd_q */
    0x4f,       /* fnmadd_s */
    0x200004b,  /* fnmsub_d */
    0x400004b,  /* fnmsub_h */
    0x600004b,  /* fnmsub_q */
    0x4b,       /* fnmsub_s */
    0x302073,   /* frcsr */
    0x102073,   /* frflags */
    0x42400053, /* fround_d */
    0x44400053, /* fround_h */
    0x46400053, /* fround_q */
    0x40400053, /* fround_s */
    0x42500053, /* froundnx_d */
    0x44500053, /* froundnx_h */
    0x46500053, /* froundnx_q */
    0x40500053, /* froundnx_s */
    0x202073,   /* frrm */
    0x301073,   /* fscsr */
    0x3027,     /* fsd */
    0x101073,   /* fsflags */
    0x105073,   /* fsflagsi */
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
    0x1027,     /* fsh */
    0x4027,     /* fsq */
    0x5a000053, /* fsqrt_d */
    0x5c000053, /* fsqrt_h */
    0x5e000053, /* fsqrt_q */
    0x58000053, /* fsqrt_s */
    0x201073,   /* fsrm */
    0x205073,   /* fsrmi */
    0xa000053,  /* fsub_d */
    0xc000053,  /* fsub_h */
    0xe000053,  /* fsub_q */
    0x8000053,  /* fsub_s */
    0x2027,     /* fsw */
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
    0x6f,       /* j */
    0x6f,       /* jal */
    0xef,       /* jal_pseudo */
    0x67,       /* jalr */
    0xe7,       /* jalr_pseudo */
    0x67,       /* jr */
    0x3,        /* lb */
    0x4003,     /* lbu */
    0x3003,     /* ld */
    0x1003,     /* lh */
    0x5003,     /* lhu */
    0x17,       /* lpad */
    0x1000302f, /* lr_d */
    0x1000202f, /* lr_w */
    0x37,       /* lui */
    0x2003,     /* lw */
    0x6003,     /* lwu */
    0xa006033,  /* max */
    0xa007033,  /* maxu */
    0xa004033,  /* min */
    0xa005033,  /* minu */
    0x70200073, /* mnret */
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
    0x2000033,  /* mul */
    0x2001033,  /* mulh */
    0x2002033,  /* mulhsu */
    0x2003033,  /* mulhu */
    0x200003b,  /* mulw */
    0x13,       /* mv */
    0x40000033, /* neg */
    0x13,       /* nop */
    0x500033,   /* ntl_all */
    0x200033,   /* ntl_p1 */
    0x300033,   /* ntl_pall */
    0x400033,   /* ntl_s1 */
    0x6033,     /* or */
    0x28705013, /* orc_b */
    0x6013,     /* ori */
    0x40006033, /* orn */
    0x8004033,  /* pack */
    0x8007033,  /* packh */
    0x800403b,  /* packw */
    0x100000f,  /* pause */
    0x6013,     /* prefetch_i */
    0x106013,   /* prefetch_r */
    0x306013,   /* prefetch_w */
    0xc0002073, /* rdcycle */
    0xc8002073, /* rdcycleh */
    0xc0202073, /* rdinstret */
    0xc8202073, /* rdinstreth */
    0xc0102073, /* rdtime */
    0xc8102073, /* rdtimeh */
    0x2006033,  /* rem */
    0x2007033,  /* remu */
    0x200703b,  /* remuw */
    0x200603b,  /* remw */
    0x8067,     /* ret */
    0x6b805013, /* rev8 */
    0x69805013, /* rev8_rv32 */
    0x60001033, /* rol */
    0x6000103b, /* rolw */
    0x60005033, /* ror */
    0x60005013, /* rori */
    0x60005013, /* rori_rv32 */
    0x6000501b, /* roriw */
    0x6000503b, /* rorw */
    0x23,       /* sb */
    0x100073,   /* sbreak */
    0x1800302f, /* sc_d */
    0x1800202f, /* sc_w */
    0x73,       /* scall */
    0x10400073, /* sctrclr */
    0x3023,     /* sd */
    0x103013,   /* seqz */
    0x60401013, /* sext_b */
    0x60501013, /* sext_h */
    0x1b,       /* sext_w */
    0x18100073, /* sfence_inval_ir */
    0x12000073, /* sfence_vma */
    0x18000073, /* sfence_w_inval */
    0x2033,     /* sgtz */
    0x1023,     /* sh */
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
    0x5c000033, /* sha512sig0h */
    0x54000033, /* sha512sig0l */
    0x10701013, /* sha512sig1 */
    0x5e000033, /* sha512sig1h */
    0x56000033, /* sha512sig1l */
    0x10401013, /* sha512sum0 */
    0x50000033, /* sha512sum0r */
    0x10501013, /* sha512sum1 */
    0x52000033, /* sha512sum1r */
    0x16000073, /* sinval_vma */
    0x1033,     /* sll */
    0x1013,     /* slli */
    0x1013,     /* slli_rv32 */
    0x800101b,  /* slli_uw */
    0x101b,     /* slliw */
    0x103b,     /* sllw */
    0x2033,     /* slt */
    0x2013,     /* slti */
    0x3013,     /* sltiu */
    0x3033,     /* sltu */
    0x2033,     /* sltz */
    0x10801013, /* sm3p0 */
    0x10901013, /* sm3p1 */
    0x30000033, /* sm4ed */
    0x34000033, /* sm4ks */
    0x3033,     /* snez */
    0x40005033, /* sra */
    0x40005013, /* srai */
    0x40005013, /* srai_rv32 */
    0x4000501b, /* sraiw */
    0x4000503b, /* sraw */
    0x10200073, /* sret */
    0x5033,     /* srl */
    0x5013,     /* srli */
    0x5013,     /* srli_rv32 */
    0x501b,     /* srliw */
    0x503b,     /* srlw */
    0x4800302f, /* ssamoswap_d */
    0x4800202f, /* ssamoswap_w */
    0xcdc0c073, /* sspopchk_x1 */
    0xcdc2c073, /* sspopchk_x5 */
    0xce104073, /* sspush_x1 */
    0xce504073, /* sspush_x5 */
    0xcdc04073, /* ssrdp */
    0x40000033, /* sub */
    0x4000003b, /* subw */
    0x2023,     /* sw */
    0x8f05013,  /* unzip */
    0x24002057, /* vaadd_vv */
    0x24006057, /* vaadd_vx */
    0x20002057, /* vaaddu_vv */
    0x20006057, /* vaaddu_vx */
    0x40003057, /* vadc_vim */
    0x40000057, /* vadc_vvm */
    0x40004057, /* vadc_vxm */
    0x3057,     /* vadd_vi */
    0x57,       /* vadd_vv */
    0x4057,     /* vadd_vx */
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
    0x4000057,  /* vandn_vv */
    0x4004057,  /* vandn_vx */
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
    0x5057,     /* vfadd_vf */
    0x1057,     /* vfadd_vv */
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
    0x480e9057, /* vfncvtbf16_f_f_w */
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
    0xc001057,  /* vfredosum_vs */
    0x4001057,  /* vfredsum_vs */
    0x4001057,  /* vfredusum_vs */
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
    0x8005057,  /* vfsub_vf */
    0x8001057,  /* vfsub_vv */
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
    0x48069057, /* vfwcvtbf16_f_f_v */
    0xf0005057, /* vfwmacc_vf */
    0xf0001057, /* vfwmacc_vv */
    0xec005057, /* vfwmaccbf16_vf */
    0xec001057, /* vfwmaccbf16_vv */
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
    0x2800007,  /* vl1r_v */
    0x2805007,  /* vl1re16_v */
    0x2806007,  /* vl1re32_v */
    0x2807007,  /* vl1re64_v */
    0x2800007,  /* vl1re8_v */
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
    0x5007,     /* vle16_v */
    0x1005007,  /* vle16ff_v */
    0x2b00007,  /* vle1_v */
    0x6007,     /* vle32_v */
    0x1006007,  /* vle32ff_v */
    0x7007,     /* vle64_v */
    0x1007007,  /* vle64ff_v */
    0x7,        /* vle8_v */
    0x1000007,  /* vle8ff_v */
    0x2b00007,  /* vlm_v */
    0xc005007,  /* vloxei16_v */
    0xc006007,  /* vloxei32_v */
    0xc007007,  /* vloxei64_v */
    0xc000007,  /* vloxei8_v */
    0x8005007,  /* vlse16_v */
    0x8006007,  /* vlse32_v */
    0x8007007,  /* vlse64_v */
    0x8000007,  /* vlse8_v */
    0x4005007,  /* vluxei16_v */
    0x4006007,  /* vluxei32_v */
    0x4007007,  /* vluxei64_v */
    0x4000007,  /* vluxei8_v */
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
    0x4002057,  /* vredand_vs */
    0x1c002057, /* vredmax_vs */
    0x18002057, /* vredmaxu_vs */
    0x14002057, /* vredmin_vs */
    0x10002057, /* vredminu_vs */
    0x8002057,  /* vredor_vs */
    0x2057,     /* vredsum_vs */
    0xc002057,  /* vredxor_vs */
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
    0xc003057,  /* vrsub_vi */
    0xc004057,  /* vrsub_vx */
    0x2800027,  /* vs1r_v */
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
    0x5027,     /* vse16_v */
    0x2b00027,  /* vse1_v */
    0x6027,     /* vse32_v */
    0x7027,     /* vse64_v */
    0x27,       /* vse8_v */
    0xc0007057, /* vsetivli */
    0x80007057, /* vsetvl */
    0x7057,     /* vsetvli */
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
    0x2b00027,  /* vsm_v */
    0x9c000057, /* vsmul_vv */
    0x9c004057, /* vsmul_vx */
    0xc005027,  /* vsoxei16_v */
    0xc006027,  /* vsoxei32_v */
    0xc007027,  /* vsoxei64_v */
    0xc000027,  /* vsoxei8_v */
    0xa4003057, /* vsra_vi */
    0xa4000057, /* vsra_vv */
    0xa4004057, /* vsra_vx */
    0xa0003057, /* vsrl_vi */
    0xa0000057, /* vsrl_vv */
    0xa0004057, /* vsrl_vx */
    0x8005027,  /* vsse16_v */
    0x8006027,  /* vsse32_v */
    0x8007027,  /* vsse64_v */
    0x8000027,  /* vsse8_v */
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
    0x8000057,  /* vsub_vv */
    0x8004057,  /* vsub_vx */
    0x4005027,  /* vsuxei16_v */
    0x4006027,  /* vsuxei32_v */
    0x4007027,  /* vsuxei64_v */
    0x4000027,  /* vsuxei8_v */
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
    0xd00073,   /* wrs_nto */
    0x1d00073,  /* wrs_sto */
    0x40004033, /* xnor */
    0x4033,     /* xor */
    0x4013,     /* xori */
    0x28002033, /* xperm4 */
    0x28004033, /* xperm8 */
    0xff07013,  /* zext_b */
    0x800403b,  /* zext_h */
    0x8004033,  /* zext_h_rv32 */
    0x800003b,  /* zext_w */
    0x8f01013,  /* zip */
];
pub static OPCODE_MASK: [u32; 1039] = [
    0xfe00707f, /* add */
    0xfe00707f, /* add_uw */
    0x707f,     /* addi */
    0x707f,     /* addiw */
    0xfe00707f, /* addw */
    0x3e00707f, /* aes32dsi */
    0x3e00707f, /* aes32dsmi */
    0x3e00707f, /* aes32esi */
    0x3e00707f, /* aes32esmi */
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
    0x707f,     /* andi */
    0xfe00707f, /* andn */
    0x7f,       /* auipc */
    0xfe00707f, /* bclr */
    0xfc00707f, /* bclri */
    0xfe00707f, /* bclri_rv32 */
    0x707f,     /* beq */
    0x1f0707f,  /* beqz */
    0xfe00707f, /* bext */
    0xfc00707f, /* bexti */
    0xfe00707f, /* bexti_rv32 */
    0x707f,     /* bge */
    0x707f,     /* bgeu */
    0x1f0707f,  /* bgez */
    0x707f,     /* bgt */
    0x707f,     /* bgtu */
    0xff07f,    /* bgtz */
    0xfe00707f, /* binv */
    0xfc00707f, /* binvi */
    0xfe00707f, /* binvi_rv32 */
    0x707f,     /* ble */
    0x707f,     /* bleu */
    0xff07f,    /* blez */
    0x707f,     /* blt */
    0x707f,     /* bltu */
    0x1f0707f,  /* bltz */
    0x707f,     /* bne */
    0x1f0707f,  /* bnez */
    0xfff0707f, /* brev8 */
    0xfe00707f, /* bset */
    0xfc00707f, /* bseti */
    0xfe00707f, /* bseti_rv32 */
    0xf003,     /* c_add */
    0xe003,     /* c_addi */
    0xef83,     /* c_addi16sp */
    0xe003,     /* c_addi4spn */
    0xe003,     /* c_addiw */
    0xfc63,     /* c_addw */
    0xfc63,     /* c_and */
    0xec03,     /* c_andi */
    0xe003,     /* c_beqz */
    0xe003,     /* c_bnez */
    0xffff,     /* c_ebreak */
    0xe003,     /* c_fld */
    0xe003,     /* c_fldsp */
    0xe003,     /* c_flw */
    0xe003,     /* c_flwsp */
    0xe003,     /* c_fsd */
    0xe003,     /* c_fsdsp */
    0xe003,     /* c_fsw */
    0xe003,     /* c_fswsp */
    0xe003,     /* c_j */
    0xe003,     /* c_jal */
    0xf07f,     /* c_jalr */
    0xf07f,     /* c_jr */
    0xfc03,     /* c_lbu */
    0xe003,     /* c_ld */
    0xe003,     /* c_ldsp */
    0xfc43,     /* c_lh */
    0xfc43,     /* c_lhu */
    0xe003,     /* c_li */
    0xe003,     /* c_lui */
    0xe003,     /* c_lw */
    0xe003,     /* c_lwsp */
    0xffff,     /* c_mop_1 */
    0xffff,     /* c_mop_11 */
    0xffff,     /* c_mop_13 */
    0xffff,     /* c_mop_15 */
    0xffff,     /* c_mop_3 */
    0xffff,     /* c_mop_5 */
    0xffff,     /* c_mop_7 */
    0xffff,     /* c_mop_9 */
    0xf8ff,     /* c_mop_N */
    0xfc63,     /* c_mul */
    0xf003,     /* c_mv */
    0xef83,     /* c_nop */
    0xfc7f,     /* c_not */
    0xffff,     /* c_ntl_all */
    0xffff,     /* c_ntl_p1 */
    0xffff,     /* c_ntl_pall */
    0xffff,     /* c_ntl_s1 */
    0xfc63,     /* c_or */
    0xfc03,     /* c_sb */
    0xe003,     /* c_sd */
    0xe003,     /* c_sdsp */
    0xfc7f,     /* c_sext_b */
    0xfc7f,     /* c_sext_h */
    0xf07f,     /* c_sext_w */
    0xfc43,     /* c_sh */
    0xe003,     /* c_slli */
    0xf003,     /* c_slli_rv32 */
    0xec03,     /* c_srai */
    0xfc03,     /* c_srai_rv32 */
    0xec03,     /* c_srli */
    0xfc03,     /* c_srli_rv32 */
    0xffff,     /* c_sspopchk_x5 */
    0xffff,     /* c_sspush_x1 */
    0xfc63,     /* c_sub */
    0xfc63,     /* c_subw */
    0xe003,     /* c_sw */
    0xe003,     /* c_swsp */
    0xfc63,     /* c_xor */
    0xfc7f,     /* c_zext_b */
    0xfc7f,     /* c_zext_h */
    0xfc7f,     /* c_zext_w */
    0xfff07fff, /* cbo_clean */
    0xfff07fff, /* cbo_flush */
    0xfff07fff, /* cbo_inval */
    0xfff07fff, /* cbo_zero */
    0xfe00707f, /* clmul */
    0xfe00707f, /* clmulh */
    0xfe00707f, /* clmulr */
    0xfff0707f, /* clz */
    0xfff0707f, /* clzw */
    0xfc03,     /* cm_jalt */
    0xfc63,     /* cm_mva01s */
    0xfc63,     /* cm_mvsa01 */
    0xff03,     /* cm_pop */
    0xff03,     /* cm_popret */
    0xff03,     /* cm_popretz */
    0xff03,     /* cm_push */
    0xfff0707f, /* cpop */
    0xfff0707f, /* cpopw */
    0x7fff,     /* csrc */
    0x7fff,     /* csrci */
    0xff07f,    /* csrr */
    0x707f,     /* csrrc */
    0x707f,     /* csrrci */
    0x707f,     /* csrrs */
    0x707f,     /* csrrsi */
    0x707f,     /* csrrw */
    0x707f,     /* csrrwi */
    0x7fff,     /* csrs */
    0x7fff,     /* csrsi */
    0x7fff,     /* csrw */
    0x7fff,     /* csrwi */
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
    0xfff0007f, /* fcvt_bf16_s */
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
    0xfff0007f, /* fcvt_s_bf16 */
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
    0x707f,     /* fence */
    0x707f,     /* fence_i */
    0xfff0707f, /* fence_tso */
    0xfe00707f, /* feq_d */
    0xfe00707f, /* feq_h */
    0xfe00707f, /* feq_q */
    0xfe00707f, /* feq_s */
    0x707f,     /* fld */
    0xfe00707f, /* fle_d */
    0xfe00707f, /* fle_h */
    0xfe00707f, /* fle_q */
    0xfe00707f, /* fle_s */
    0xfe00707f, /* fleq_d */
    0xfe00707f, /* fleq_h */
    0xfe00707f, /* fleq_q */
    0xfe00707f, /* fleq_s */
    0x707f,     /* flh */
    0xfff0707f, /* fli_d */
    0xfff0707f, /* fli_h */
    0xfff0707f, /* fli_q */
    0xfff0707f, /* fli_s */
    0x707f,     /* flq */
    0xfe00707f, /* flt_d */
    0xfe00707f, /* flt_h */
    0xfe00707f, /* flt_q */
    0xfe00707f, /* flt_s */
    0xfe00707f, /* fltq_d */
    0xfe00707f, /* fltq_h */
    0xfe00707f, /* fltq_q */
    0xfe00707f, /* fltq_s */
    0x707f,     /* flw */
    0x600007f,  /* fmadd_d */
    0x600007f,  /* fmadd_h */
    0x600007f,  /* fmadd_q */
    0x600007f,  /* fmadd_s */
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
    0x600007f,  /* fmsub_d */
    0x600007f,  /* fmsub_h */
    0x600007f,  /* fmsub_q */
    0x600007f,  /* fmsub_s */
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
    0xfff0707f, /* fmvh_x_d */
    0xfff0707f, /* fmvh_x_q */
    0xfe00707f, /* fmvp_d_x */
    0xfe00707f, /* fmvp_q_x */
    0xfe00707f, /* fneg_d */
    0xfe00707f, /* fneg_h */
    0xfe00707f, /* fneg_q */
    0xfe00707f, /* fneg_s */
    0x600007f,  /* fnmadd_d */
    0x600007f,  /* fnmadd_h */
    0x600007f,  /* fnmadd_q */
    0x600007f,  /* fnmadd_s */
    0x600007f,  /* fnmsub_d */
    0x600007f,  /* fnmsub_h */
    0x600007f,  /* fnmsub_q */
    0x600007f,  /* fnmsub_s */
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
    0x707f,     /* fsd */
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
    0x707f,     /* fsh */
    0x707f,     /* fsq */
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
    0x707f,     /* fsw */
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
    0xfff,      /* j */
    0x7f,       /* jal */
    0xfff,      /* jal_pseudo */
    0x707f,     /* jalr */
    0xfff07fff, /* jalr_pseudo */
    0xfff07fff, /* jr */
    0x707f,     /* lb */
    0x707f,     /* lbu */
    0x707f,     /* ld */
    0x707f,     /* lh */
    0x707f,     /* lhu */
    0xfff,      /* lpad */
    0xf9f0707f, /* lr_d */
    0xf9f0707f, /* lr_w */
    0x7f,       /* lui */
    0x707f,     /* lw */
    0x707f,     /* lwu */
    0xfe00707f, /* max */
    0xfe00707f, /* maxu */
    0xfe00707f, /* min */
    0xfe00707f, /* minu */
    0xffffffff, /* mnret */
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
    0x707f,     /* ori */
    0xfe00707f, /* orn */
    0xfe00707f, /* pack */
    0xfe00707f, /* packh */
    0xfe00707f, /* packw */
    0xffffffff, /* pause */
    0x1f07fff,  /* prefetch_i */
    0x1f07fff,  /* prefetch_r */
    0x1f07fff,  /* prefetch_w */
    0xfffff07f, /* rdcycle */
    0xfffff07f, /* rdcycleh */
    0xfffff07f, /* rdinstret */
    0xfffff07f, /* rdinstreth */
    0xfffff07f, /* rdtime */
    0xfffff07f, /* rdtimeh */
    0xfe00707f, /* rem */
    0xfe00707f, /* remu */
    0xfe00707f, /* remuw */
    0xfe00707f, /* remw */
    0xffffffff, /* ret */
    0xfff0707f, /* rev8 */
    0xfff0707f, /* rev8_rv32 */
    0xfe00707f, /* rol */
    0xfe00707f, /* rolw */
    0xfe00707f, /* ror */
    0xfc00707f, /* rori */
    0xfe00707f, /* rori_rv32 */
    0xfe00707f, /* roriw */
    0xfe00707f, /* rorw */
    0x707f,     /* sb */
    0xffffffff, /* sbreak */
    0xf800707f, /* sc_d */
    0xf800707f, /* sc_w */
    0xffffffff, /* scall */
    0xffffffff, /* sctrclr */
    0x707f,     /* sd */
    0xfff0707f, /* seqz */
    0xfff0707f, /* sext_b */
    0xfff0707f, /* sext_h */
    0xfff0707f, /* sext_w */
    0xffffffff, /* sfence_inval_ir */
    0xfe007fff, /* sfence_vma */
    0xffffffff, /* sfence_w_inval */
    0xfe0ff07f, /* sgtz */
    0x707f,     /* sh */
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
    0xfe00707f, /* sha512sig0h */
    0xfe00707f, /* sha512sig0l */
    0xfff0707f, /* sha512sig1 */
    0xfe00707f, /* sha512sig1h */
    0xfe00707f, /* sha512sig1l */
    0xfff0707f, /* sha512sum0 */
    0xfe00707f, /* sha512sum0r */
    0xfff0707f, /* sha512sum1 */
    0xfe00707f, /* sha512sum1r */
    0xfe007fff, /* sinval_vma */
    0xfe00707f, /* sll */
    0xfc00707f, /* slli */
    0xfe00707f, /* slli_rv32 */
    0xfc00707f, /* slli_uw */
    0xfe00707f, /* slliw */
    0xfe00707f, /* sllw */
    0xfe00707f, /* slt */
    0x707f,     /* slti */
    0x707f,     /* sltiu */
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
    0xfe00707f, /* sraiw */
    0xfe00707f, /* sraw */
    0xffffffff, /* sret */
    0xfe00707f, /* srl */
    0xfc00707f, /* srli */
    0xfe00707f, /* srli_rv32 */
    0xfe00707f, /* srliw */
    0xfe00707f, /* srlw */
    0xf800707f, /* ssamoswap_d */
    0xf800707f, /* ssamoswap_w */
    0xffffffff, /* sspopchk_x1 */
    0xffffffff, /* sspopchk_x5 */
    0xffffffff, /* sspush_x1 */
    0xffffffff, /* sspush_x5 */
    0xfffff07f, /* ssrdp */
    0xfe00707f, /* sub */
    0xfe00707f, /* subw */
    0x707f,     /* sw */
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
    0xfc0ff07f, /* vfncvtbf16_f_f_w */
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
    0xfc0ff07f, /* vfwcvtbf16_f_f_v */
    0xfc00707f, /* vfwmacc_vf */
    0xfc00707f, /* vfwmacc_vv */
    0xfc00707f, /* vfwmaccbf16_vf */
    0xfc00707f, /* vfwmaccbf16_vv */
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
    0x707f,     /* xori */
    0xfe00707f, /* xperm4 */
    0xfe00707f, /* xperm8 */
    0xfff0707f, /* zext_b */
    0xfff0707f, /* zext_h */
    0xfff0707f, /* zext_h_rv32 */
    0xfff0707f, /* zext_w */
    0xfff0707f, /* zip */
];
pub static OPCODE_MASK_COMPRESSED: [u16; 1039] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61443, 57347,
    61315, 57347, 57347, 64611, 64611, 60419, 57347, 57347, 65535, 57347, 57347, 57347, 57347,
    57347, 57347, 57347, 57347, 57347, 57347, 61567, 61567, 64515, 57347, 57347, 64579, 64579,
    57347, 57347, 57347, 57347, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 63743,
    64611, 61443, 61315, 64639, 65535, 65535, 65535, 65535, 64611, 64515, 57347, 57347, 64639,
    64639, 61567, 64579, 57347, 61443, 60419, 64515, 60419, 64515, 65535, 65535, 64611, 64611,
    57347, 57347, 64611, 64639, 64639, 64639, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64515, 64611, 64611,
    65283, 65283, 65283, 65283, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
];
pub static OPCODE_MATCH_COMPRESSED: [u16; 1039] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36866, 1, 24833,
    0, 8193, 39969, 35937, 34817, 49153, 57345, 36866, 8192, 8194, 24576, 24578, 40960, 40962,
    57344, 57346, 40961, 8193, 36866, 32770, 32768, 24576, 24578, 33856, 33792, 16385, 24577,
    16384, 16386, 24705, 25985, 26241, 26497, 24961, 25217, 25473, 25729, 24705, 40001, 32770, 1,
    40053, 36886, 36874, 36878, 36882, 35905, 34816, 57344, 57346, 40037, 40045, 8193, 35840, 2, 2,
    33793, 33793, 32769, 32769, 25217, 24705, 35841, 39937, 49152, 49154, 35873, 40033, 40041,
    40049, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40962, 44130, 44066, 47618, 48642, 48130, 47106, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
pub static OPCODE_XLEN: [u8; 1039] = [
    3, /* add */
    2, /* add_uw */
    3, /* addi */
    2, /* addiw */
    2, /* addw */
    1, /* aes32dsi */
    1, /* aes32dsmi */
    1, /* aes32esi */
    1, /* aes32esmi */
    2, /* aes64ds */
    2, /* aes64dsm */
    2, /* aes64es */
    2, /* aes64esm */
    2, /* aes64im */
    2, /* aes64ks1i */
    2, /* aes64ks2 */
    3, /* amoadd_b */
    2, /* amoadd_d */
    3, /* amoadd_h */
    3, /* amoadd_w */
    3, /* amoand_b */
    2, /* amoand_d */
    3, /* amoand_h */
    3, /* amoand_w */
    3, /* amocas_b */
    3, /* amocas_d */
    3, /* amocas_h */
    2, /* amocas_q */
    3, /* amocas_w */
    3, /* amomax_b */
    2, /* amomax_d */
    3, /* amomax_h */
    3, /* amomax_w */
    3, /* amomaxu_b */
    2, /* amomaxu_d */
    3, /* amomaxu_h */
    3, /* amomaxu_w */
    3, /* amomin_b */
    2, /* amomin_d */
    3, /* amomin_h */
    3, /* amomin_w */
    3, /* amominu_b */
    2, /* amominu_d */
    3, /* amominu_h */
    3, /* amominu_w */
    3, /* amoor_b */
    2, /* amoor_d */
    3, /* amoor_h */
    3, /* amoor_w */
    3, /* amoswap_b */
    2, /* amoswap_d */
    3, /* amoswap_h */
    3, /* amoswap_w */
    3, /* amoxor_b */
    2, /* amoxor_d */
    3, /* amoxor_h */
    3, /* amoxor_w */
    3, /* and */
    3, /* andi */
    3, /* andn */
    3, /* auipc */
    3, /* bclr */
    2, /* bclri */
    1, /* bclri_rv32 */
    3, /* beq */
    3, /* beqz */
    3, /* bext */
    2, /* bexti */
    1, /* bexti_rv32 */
    3, /* bge */
    3, /* bgeu */
    3, /* bgez */
    3, /* bgt */
    3, /* bgtu */
    3, /* bgtz */
    3, /* binv */
    2, /* binvi */
    1, /* binvi_rv32 */
    3, /* ble */
    3, /* bleu */
    3, /* blez */
    3, /* blt */
    3, /* bltu */
    3, /* bltz */
    3, /* bne */
    3, /* bnez */
    3, /* brev8 */
    3, /* bset */
    2, /* bseti */
    1, /* bseti_rv32 */
    3, /* c_add */
    3, /* c_addi */
    3, /* c_addi16sp */
    3, /* c_addi4spn */
    2, /* c_addiw */
    2, /* c_addw */
    3, /* c_and */
    3, /* c_andi */
    3, /* c_beqz */
    3, /* c_bnez */
    3, /* c_ebreak */
    3, /* c_fld */
    3, /* c_fldsp */
    1, /* c_flw */
    1, /* c_flwsp */
    3, /* c_fsd */
    3, /* c_fsdsp */
    1, /* c_fsw */
    1, /* c_fswsp */
    3, /* c_j */
    1, /* c_jal */
    3, /* c_jalr */
    3, /* c_jr */
    3, /* c_lbu */
    3, /* c_ld */
    3, /* c_ldsp */
    3, /* c_lh */
    3, /* c_lhu */
    3, /* c_li */
    3, /* c_lui */
    3, /* c_lw */
    3, /* c_lwsp */
    3, /* c_mop_1 */
    3, /* c_mop_11 */
    3, /* c_mop_13 */
    3, /* c_mop_15 */
    3, /* c_mop_3 */
    3, /* c_mop_5 */
    3, /* c_mop_7 */
    3, /* c_mop_9 */
    3, /* c_mop_N */
    3, /* c_mul */
    3, /* c_mv */
    3, /* c_nop */
    3, /* c_not */
    3, /* c_ntl_all */
    3, /* c_ntl_p1 */
    3, /* c_ntl_pall */
    3, /* c_ntl_s1 */
    3, /* c_or */
    3, /* c_sb */
    3, /* c_sd */
    3, /* c_sdsp */
    3, /* c_sext_b */
    3, /* c_sext_h */
    2, /* c_sext_w */
    3, /* c_sh */
    3, /* c_slli */
    1, /* c_slli_rv32 */
    3, /* c_srai */
    1, /* c_srai_rv32 */
    3, /* c_srli */
    1, /* c_srli_rv32 */
    3, /* c_sspopchk_x5 */
    3, /* c_sspush_x1 */
    3, /* c_sub */
    2, /* c_subw */
    3, /* c_sw */
    3, /* c_swsp */
    3, /* c_xor */
    3, /* c_zext_b */
    3, /* c_zext_h */
    2, /* c_zext_w */
    3, /* cbo_clean */
    3, /* cbo_flush */
    3, /* cbo_inval */
    3, /* cbo_zero */
    3, /* clmul */
    3, /* clmulh */
    3, /* clmulr */
    3, /* clz */
    2, /* clzw */
    3, /* cm_jalt */
    3, /* cm_mva01s */
    3, /* cm_mvsa01 */
    3, /* cm_pop */
    3, /* cm_popret */
    3, /* cm_popretz */
    3, /* cm_push */
    3, /* cpop */
    2, /* cpopw */
    3, /* csrc */
    3, /* csrci */
    3, /* csrr */
    3, /* csrrc */
    3, /* csrrci */
    3, /* csrrs */
    3, /* csrrsi */
    3, /* csrrw */
    3, /* csrrwi */
    3, /* csrs */
    3, /* csrsi */
    3, /* csrw */
    3, /* csrwi */
    3, /* ctz */
    2, /* ctzw */
    3, /* czero_eqz */
    3, /* czero_nez */
    3, /* div */
    3, /* divu */
    2, /* divuw */
    2, /* divw */
    3, /* dret */
    3, /* ebreak */
    3, /* ecall */
    3, /* fabs_d */
    3, /* fabs_h */
    3, /* fabs_q */
    3, /* fabs_s */
    3, /* fadd_d */
    3, /* fadd_h */
    3, /* fadd_q */
    3, /* fadd_s */
    3, /* fclass_d */
    3, /* fclass_h */
    3, /* fclass_q */
    3, /* fclass_s */
    3, /* fcvt_bf16_s */
    3, /* fcvt_d_h */
    2, /* fcvt_d_l */
    2, /* fcvt_d_lu */
    3, /* fcvt_d_q */
    3, /* fcvt_d_s */
    3, /* fcvt_d_w */
    3, /* fcvt_d_wu */
    3, /* fcvt_h_d */
    2, /* fcvt_h_l */
    2, /* fcvt_h_lu */
    3, /* fcvt_h_q */
    3, /* fcvt_h_s */
    3, /* fcvt_h_w */
    3, /* fcvt_h_wu */
    2, /* fcvt_l_d */
    2, /* fcvt_l_h */
    2, /* fcvt_l_q */
    2, /* fcvt_l_s */
    2, /* fcvt_lu_d */
    2, /* fcvt_lu_h */
    2, /* fcvt_lu_q */
    2, /* fcvt_lu_s */
    3, /* fcvt_q_d */
    3, /* fcvt_q_h */
    2, /* fcvt_q_l */
    2, /* fcvt_q_lu */
    3, /* fcvt_q_s */
    3, /* fcvt_q_w */
    3, /* fcvt_q_wu */
    3, /* fcvt_s_bf16 */
    3, /* fcvt_s_d */
    3, /* fcvt_s_h */
    2, /* fcvt_s_l */
    2, /* fcvt_s_lu */
    3, /* fcvt_s_q */
    3, /* fcvt_s_w */
    3, /* fcvt_s_wu */
    3, /* fcvt_w_d */
    3, /* fcvt_w_h */
    3, /* fcvt_w_q */
    3, /* fcvt_w_s */
    3, /* fcvt_wu_d */
    3, /* fcvt_wu_h */
    3, /* fcvt_wu_q */
    3, /* fcvt_wu_s */
    3, /* fcvtmod_w_d */
    3, /* fdiv_d */
    3, /* fdiv_h */
    3, /* fdiv_q */
    3, /* fdiv_s */
    3, /* fence */
    3, /* fence_i */
    3, /* fence_tso */
    3, /* feq_d */
    3, /* feq_h */
    3, /* feq_q */
    3, /* feq_s */
    3, /* fld */
    3, /* fle_d */
    3, /* fle_h */
    3, /* fle_q */
    3, /* fle_s */
    3, /* fleq_d */
    3, /* fleq_h */
    3, /* fleq_q */
    3, /* fleq_s */
    3, /* flh */
    3, /* fli_d */
    3, /* fli_h */
    3, /* fli_q */
    3, /* fli_s */
    3, /* flq */
    3, /* flt_d */
    3, /* flt_h */
    3, /* flt_q */
    3, /* flt_s */
    3, /* fltq_d */
    3, /* fltq_h */
    3, /* fltq_q */
    3, /* fltq_s */
    3, /* flw */
    3, /* fmadd_d */
    3, /* fmadd_h */
    3, /* fmadd_q */
    3, /* fmadd_s */
    3, /* fmax_d */
    3, /* fmax_h */
    3, /* fmax_q */
    3, /* fmax_s */
    3, /* fmaxm_d */
    3, /* fmaxm_h */
    3, /* fmaxm_q */
    3, /* fmaxm_s */
    3, /* fmin_d */
    3, /* fmin_h */
    3, /* fmin_q */
    3, /* fmin_s */
    3, /* fminm_d */
    3, /* fminm_h */
    3, /* fminm_q */
    3, /* fminm_s */
    3, /* fmsub_d */
    3, /* fmsub_h */
    3, /* fmsub_q */
    3, /* fmsub_s */
    3, /* fmul_d */
    3, /* fmul_h */
    3, /* fmul_q */
    3, /* fmul_s */
    3, /* fmv_d */
    2, /* fmv_d_x */
    3, /* fmv_h */
    3, /* fmv_h_x */
    3, /* fmv_q */
    3, /* fmv_s */
    3, /* fmv_s_x */
    3, /* fmv_w_x */
    2, /* fmv_x_d */
    3, /* fmv_x_h */
    3, /* fmv_x_s */
    3, /* fmv_x_w */
    1, /* fmvh_x_d */
    2, /* fmvh_x_q */
    1, /* fmvp_d_x */
    2, /* fmvp_q_x */
    3, /* fneg_d */
    3, /* fneg_h */
    3, /* fneg_q */
    3, /* fneg_s */
    3, /* fnmadd_d */
    3, /* fnmadd_h */
    3, /* fnmadd_q */
    3, /* fnmadd_s */
    3, /* fnmsub_d */
    3, /* fnmsub_h */
    3, /* fnmsub_q */
    3, /* fnmsub_s */
    3, /* frcsr */
    3, /* frflags */
    3, /* fround_d */
    3, /* fround_h */
    3, /* fround_q */
    3, /* fround_s */
    3, /* froundnx_d */
    3, /* froundnx_h */
    3, /* froundnx_q */
    3, /* froundnx_s */
    3, /* frrm */
    3, /* fscsr */
    3, /* fsd */
    3, /* fsflags */
    3, /* fsflagsi */
    3, /* fsgnj_d */
    3, /* fsgnj_h */
    3, /* fsgnj_q */
    3, /* fsgnj_s */
    3, /* fsgnjn_d */
    3, /* fsgnjn_h */
    3, /* fsgnjn_q */
    3, /* fsgnjn_s */
    3, /* fsgnjx_d */
    3, /* fsgnjx_h */
    3, /* fsgnjx_q */
    3, /* fsgnjx_s */
    3, /* fsh */
    3, /* fsq */
    3, /* fsqrt_d */
    3, /* fsqrt_h */
    3, /* fsqrt_q */
    3, /* fsqrt_s */
    3, /* fsrm */
    3, /* fsrmi */
    3, /* fsub_d */
    3, /* fsub_h */
    3, /* fsub_q */
    3, /* fsub_s */
    3, /* fsw */
    3, /* hfence_gvma */
    3, /* hfence_vvma */
    3, /* hinval_gvma */
    3, /* hinval_vvma */
    3, /* hlv_b */
    3, /* hlv_bu */
    2, /* hlv_d */
    3, /* hlv_h */
    3, /* hlv_hu */
    3, /* hlv_w */
    2, /* hlv_wu */
    3, /* hlvx_hu */
    3, /* hlvx_wu */
    3, /* hsv_b */
    2, /* hsv_d */
    3, /* hsv_h */
    3, /* hsv_w */
    3, /* j */
    3, /* jal */
    3, /* jal_pseudo */
    3, /* jalr */
    3, /* jalr_pseudo */
    3, /* jr */
    3, /* lb */
    3, /* lbu */
    2, /* ld */
    3, /* lh */
    3, /* lhu */
    3, /* lpad */
    2, /* lr_d */
    3, /* lr_w */
    3, /* lui */
    3, /* lw */
    2, /* lwu */
    3, /* max */
    3, /* maxu */
    3, /* min */
    3, /* minu */
    3, /* mnret */
    3, /* mop_r_0 */
    3, /* mop_r_1 */
    3, /* mop_r_10 */
    3, /* mop_r_11 */
    3, /* mop_r_12 */
    3, /* mop_r_13 */
    3, /* mop_r_14 */
    3, /* mop_r_15 */
    3, /* mop_r_16 */
    3, /* mop_r_17 */
    3, /* mop_r_18 */
    3, /* mop_r_19 */
    3, /* mop_r_2 */
    3, /* mop_r_20 */
    3, /* mop_r_21 */
    3, /* mop_r_22 */
    3, /* mop_r_23 */
    3, /* mop_r_24 */
    3, /* mop_r_25 */
    3, /* mop_r_26 */
    3, /* mop_r_27 */
    3, /* mop_r_28 */
    3, /* mop_r_29 */
    3, /* mop_r_3 */
    3, /* mop_r_30 */
    3, /* mop_r_31 */
    3, /* mop_r_4 */
    3, /* mop_r_5 */
    3, /* mop_r_6 */
    3, /* mop_r_7 */
    3, /* mop_r_8 */
    3, /* mop_r_9 */
    3, /* mop_r_N */
    3, /* mop_rr_0 */
    3, /* mop_rr_1 */
    3, /* mop_rr_2 */
    3, /* mop_rr_3 */
    3, /* mop_rr_4 */
    3, /* mop_rr_5 */
    3, /* mop_rr_6 */
    3, /* mop_rr_7 */
    3, /* mop_rr_N */
    3, /* mret */
    3, /* mul */
    3, /* mulh */
    3, /* mulhsu */
    3, /* mulhu */
    2, /* mulw */
    3, /* mv */
    3, /* neg */
    3, /* nop */
    3, /* ntl_all */
    3, /* ntl_p1 */
    3, /* ntl_pall */
    3, /* ntl_s1 */
    3, /* or */
    3, /* orc_b */
    3, /* ori */
    3, /* orn */
    3, /* pack */
    3, /* packh */
    2, /* packw */
    3, /* pause */
    3, /* prefetch_i */
    3, /* prefetch_r */
    3, /* prefetch_w */
    3, /* rdcycle */
    1, /* rdcycleh */
    3, /* rdinstret */
    1, /* rdinstreth */
    3, /* rdtime */
    1, /* rdtimeh */
    3, /* rem */
    3, /* remu */
    2, /* remuw */
    2, /* remw */
    3, /* ret */
    2, /* rev8 */
    1, /* rev8_rv32 */
    3, /* rol */
    2, /* rolw */
    3, /* ror */
    2, /* rori */
    1, /* rori_rv32 */
    2, /* roriw */
    2, /* rorw */
    3, /* sb */
    3, /* sbreak */
    2, /* sc_d */
    3, /* sc_w */
    3, /* scall */
    3, /* sctrclr */
    2, /* sd */
    3, /* seqz */
    3, /* sext_b */
    3, /* sext_h */
    2, /* sext_w */
    3, /* sfence_inval_ir */
    3, /* sfence_vma */
    3, /* sfence_w_inval */
    3, /* sgtz */
    3, /* sh */
    3, /* sh1add */
    2, /* sh1add_uw */
    3, /* sh2add */
    2, /* sh2add_uw */
    3, /* sh3add */
    2, /* sh3add_uw */
    3, /* sha256sig0 */
    3, /* sha256sig1 */
    3, /* sha256sum0 */
    3, /* sha256sum1 */
    2, /* sha512sig0 */
    1, /* sha512sig0h */
    1, /* sha512sig0l */
    2, /* sha512sig1 */
    1, /* sha512sig1h */
    1, /* sha512sig1l */
    2, /* sha512sum0 */
    1, /* sha512sum0r */
    2, /* sha512sum1 */
    1, /* sha512sum1r */
    3, /* sinval_vma */
    3, /* sll */
    3, /* slli */
    1, /* slli_rv32 */
    2, /* slli_uw */
    2, /* slliw */
    2, /* sllw */
    3, /* slt */
    3, /* slti */
    3, /* sltiu */
    3, /* sltu */
    3, /* sltz */
    3, /* sm3p0 */
    3, /* sm3p1 */
    3, /* sm4ed */
    3, /* sm4ks */
    3, /* snez */
    3, /* sra */
    3, /* srai */
    1, /* srai_rv32 */
    2, /* sraiw */
    2, /* sraw */
    3, /* sret */
    3, /* srl */
    3, /* srli */
    1, /* srli_rv32 */
    2, /* srliw */
    2, /* srlw */
    2, /* ssamoswap_d */
    3, /* ssamoswap_w */
    3, /* sspopchk_x1 */
    3, /* sspopchk_x5 */
    3, /* sspush_x1 */
    3, /* sspush_x5 */
    3, /* ssrdp */
    3, /* sub */
    2, /* subw */
    3, /* sw */
    1, /* unzip */
    3, /* vaadd_vv */
    3, /* vaadd_vx */
    3, /* vaaddu_vv */
    3, /* vaaddu_vx */
    3, /* vadc_vim */
    3, /* vadc_vvm */
    3, /* vadc_vxm */
    3, /* vadd_vi */
    3, /* vadd_vv */
    3, /* vadd_vx */
    3, /* vaesdf_vs */
    3, /* vaesdf_vv */
    3, /* vaesdm_vs */
    3, /* vaesdm_vv */
    3, /* vaesef_vs */
    3, /* vaesef_vv */
    3, /* vaesem_vs */
    3, /* vaesem_vv */
    3, /* vaeskf1_vi */
    3, /* vaeskf2_vi */
    3, /* vaesz_vs */
    3, /* vand_vi */
    3, /* vand_vv */
    3, /* vand_vx */
    3, /* vandn_vv */
    3, /* vandn_vx */
    3, /* vasub_vv */
    3, /* vasub_vx */
    3, /* vasubu_vv */
    3, /* vasubu_vx */
    3, /* vbrev8_v */
    3, /* vbrev_v */
    3, /* vclmul_vv */
    3, /* vclmul_vx */
    3, /* vclmulh_vv */
    3, /* vclmulh_vx */
    3, /* vclz_v */
    3, /* vcompress_vm */
    3, /* vcpop_m */
    3, /* vcpop_v */
    3, /* vctz_v */
    3, /* vdiv_vv */
    3, /* vdiv_vx */
    3, /* vdivu_vv */
    3, /* vdivu_vx */
    3, /* vfadd_vf */
    3, /* vfadd_vv */
    3, /* vfclass_v */
    3, /* vfcvt_f_x_v */
    3, /* vfcvt_f_xu_v */
    3, /* vfcvt_rtz_x_f_v */
    3, /* vfcvt_rtz_xu_f_v */
    3, /* vfcvt_x_f_v */
    3, /* vfcvt_xu_f_v */
    3, /* vfdiv_vf */
    3, /* vfdiv_vv */
    3, /* vfirst_m */
    3, /* vfmacc_vf */
    3, /* vfmacc_vv */
    3, /* vfmadd_vf */
    3, /* vfmadd_vv */
    3, /* vfmax_vf */
    3, /* vfmax_vv */
    3, /* vfmerge_vfm */
    3, /* vfmin_vf */
    3, /* vfmin_vv */
    3, /* vfmsac_vf */
    3, /* vfmsac_vv */
    3, /* vfmsub_vf */
    3, /* vfmsub_vv */
    3, /* vfmul_vf */
    3, /* vfmul_vv */
    3, /* vfmv_f_s */
    3, /* vfmv_s_f */
    3, /* vfmv_v_f */
    3, /* vfncvt_f_f_w */
    3, /* vfncvt_f_x_w */
    3, /* vfncvt_f_xu_w */
    3, /* vfncvt_rod_f_f_w */
    3, /* vfncvt_rtz_x_f_w */
    3, /* vfncvt_rtz_xu_f_w */
    3, /* vfncvt_x_f_w */
    3, /* vfncvt_xu_f_w */
    3, /* vfncvtbf16_f_f_w */
    3, /* vfnmacc_vf */
    3, /* vfnmacc_vv */
    3, /* vfnmadd_vf */
    3, /* vfnmadd_vv */
    3, /* vfnmsac_vf */
    3, /* vfnmsac_vv */
    3, /* vfnmsub_vf */
    3, /* vfnmsub_vv */
    3, /* vfrdiv_vf */
    3, /* vfrec7_v */
    3, /* vfredmax_vs */
    3, /* vfredmin_vs */
    3, /* vfredosum_vs */
    3, /* vfredsum_vs */
    3, /* vfredusum_vs */
    3, /* vfrsqrt7_v */
    3, /* vfrsub_vf */
    3, /* vfsgnj_vf */
    3, /* vfsgnj_vv */
    3, /* vfsgnjn_vf */
    3, /* vfsgnjn_vv */
    3, /* vfsgnjx_vf */
    3, /* vfsgnjx_vv */
    3, /* vfslide1down_vf */
    3, /* vfslide1up_vf */
    3, /* vfsqrt_v */
    3, /* vfsub_vf */
    3, /* vfsub_vv */
    3, /* vfwadd_vf */
    3, /* vfwadd_vv */
    3, /* vfwadd_wf */
    3, /* vfwadd_wv */
    3, /* vfwcvt_f_f_v */
    3, /* vfwcvt_f_x_v */
    3, /* vfwcvt_f_xu_v */
    3, /* vfwcvt_rtz_x_f_v */
    3, /* vfwcvt_rtz_xu_f_v */
    3, /* vfwcvt_x_f_v */
    3, /* vfwcvt_xu_f_v */
    3, /* vfwcvtbf16_f_f_v */
    3, /* vfwmacc_vf */
    3, /* vfwmacc_vv */
    3, /* vfwmaccbf16_vf */
    3, /* vfwmaccbf16_vv */
    3, /* vfwmsac_vf */
    3, /* vfwmsac_vv */
    3, /* vfwmul_vf */
    3, /* vfwmul_vv */
    3, /* vfwnmacc_vf */
    3, /* vfwnmacc_vv */
    3, /* vfwnmsac_vf */
    3, /* vfwnmsac_vv */
    3, /* vfwredosum_vs */
    3, /* vfwredsum_vs */
    3, /* vfwredusum_vs */
    3, /* vfwsub_vf */
    3, /* vfwsub_vv */
    3, /* vfwsub_wf */
    3, /* vfwsub_wv */
    3, /* vghsh_vv */
    3, /* vgmul_vv */
    3, /* vid_v */
    3, /* viota_m */
    3, /* vl1r_v */
    3, /* vl1re16_v */
    3, /* vl1re32_v */
    3, /* vl1re64_v */
    3, /* vl1re8_v */
    3, /* vl2r_v */
    3, /* vl2re16_v */
    3, /* vl2re32_v */
    3, /* vl2re64_v */
    3, /* vl2re8_v */
    3, /* vl4r_v */
    3, /* vl4re16_v */
    3, /* vl4re32_v */
    3, /* vl4re64_v */
    3, /* vl4re8_v */
    3, /* vl8r_v */
    3, /* vl8re16_v */
    3, /* vl8re32_v */
    3, /* vl8re64_v */
    3, /* vl8re8_v */
    3, /* vle16_v */
    3, /* vle16ff_v */
    3, /* vle1_v */
    3, /* vle32_v */
    3, /* vle32ff_v */
    3, /* vle64_v */
    3, /* vle64ff_v */
    3, /* vle8_v */
    3, /* vle8ff_v */
    3, /* vlm_v */
    3, /* vloxei16_v */
    3, /* vloxei32_v */
    3, /* vloxei64_v */
    3, /* vloxei8_v */
    3, /* vlse16_v */
    3, /* vlse32_v */
    3, /* vlse64_v */
    3, /* vlse8_v */
    3, /* vluxei16_v */
    3, /* vluxei32_v */
    3, /* vluxei64_v */
    3, /* vluxei8_v */
    3, /* vmacc_vv */
    3, /* vmacc_vx */
    3, /* vmadc_vi */
    3, /* vmadc_vim */
    3, /* vmadc_vv */
    3, /* vmadc_vvm */
    3, /* vmadc_vx */
    3, /* vmadc_vxm */
    3, /* vmadd_vv */
    3, /* vmadd_vx */
    3, /* vmand_mm */
    3, /* vmandn_mm */
    3, /* vmandnot_mm */
    3, /* vmax_vv */
    3, /* vmax_vx */
    3, /* vmaxu_vv */
    3, /* vmaxu_vx */
    3, /* vmerge_vim */
    3, /* vmerge_vvm */
    3, /* vmerge_vxm */
    3, /* vmfeq_vf */
    3, /* vmfeq_vv */
    3, /* vmfge_vf */
    3, /* vmfgt_vf */
    3, /* vmfle_vf */
    3, /* vmfle_vv */
    3, /* vmflt_vf */
    3, /* vmflt_vv */
    3, /* vmfne_vf */
    3, /* vmfne_vv */
    3, /* vmin_vv */
    3, /* vmin_vx */
    3, /* vminu_vv */
    3, /* vminu_vx */
    3, /* vmnand_mm */
    3, /* vmnor_mm */
    3, /* vmor_mm */
    3, /* vmorn_mm */
    3, /* vmornot_mm */
    3, /* vmsbc_vv */
    3, /* vmsbc_vvm */
    3, /* vmsbc_vx */
    3, /* vmsbc_vxm */
    3, /* vmsbf_m */
    3, /* vmseq_vi */
    3, /* vmseq_vv */
    3, /* vmseq_vx */
    3, /* vmsgt_vi */
    3, /* vmsgt_vx */
    3, /* vmsgtu_vi */
    3, /* vmsgtu_vx */
    3, /* vmsif_m */
    3, /* vmsle_vi */
    3, /* vmsle_vv */
    3, /* vmsle_vx */
    3, /* vmsleu_vi */
    3, /* vmsleu_vv */
    3, /* vmsleu_vx */
    3, /* vmslt_vv */
    3, /* vmslt_vx */
    3, /* vmsltu_vv */
    3, /* vmsltu_vx */
    3, /* vmsne_vi */
    3, /* vmsne_vv */
    3, /* vmsne_vx */
    3, /* vmsof_m */
    3, /* vmul_vv */
    3, /* vmul_vx */
    3, /* vmulh_vv */
    3, /* vmulh_vx */
    3, /* vmulhsu_vv */
    3, /* vmulhsu_vx */
    3, /* vmulhu_vv */
    3, /* vmulhu_vx */
    3, /* vmv1r_v */
    3, /* vmv2r_v */
    3, /* vmv4r_v */
    3, /* vmv8r_v */
    3, /* vmv_s_x */
    3, /* vmv_v_i */
    3, /* vmv_v_v */
    3, /* vmv_v_x */
    3, /* vmv_x_s */
    3, /* vmxnor_mm */
    3, /* vmxor_mm */
    3, /* vnclip_wi */
    3, /* vnclip_wv */
    3, /* vnclip_wx */
    3, /* vnclipu_wi */
    3, /* vnclipu_wv */
    3, /* vnclipu_wx */
    3, /* vnmsac_vv */
    3, /* vnmsac_vx */
    3, /* vnmsub_vv */
    3, /* vnmsub_vx */
    3, /* vnsra_wi */
    3, /* vnsra_wv */
    3, /* vnsra_wx */
    3, /* vnsrl_wi */
    3, /* vnsrl_wv */
    3, /* vnsrl_wx */
    3, /* vor_vi */
    3, /* vor_vv */
    3, /* vor_vx */
    3, /* vpopc_m */
    3, /* vredand_vs */
    3, /* vredmax_vs */
    3, /* vredmaxu_vs */
    3, /* vredmin_vs */
    3, /* vredminu_vs */
    3, /* vredor_vs */
    3, /* vredsum_vs */
    3, /* vredxor_vs */
    3, /* vrem_vv */
    3, /* vrem_vx */
    3, /* vremu_vv */
    3, /* vremu_vx */
    3, /* vrev8_v */
    3, /* vrgather_vi */
    3, /* vrgather_vv */
    3, /* vrgather_vx */
    3, /* vrgatherei16_vv */
    3, /* vrol_vv */
    3, /* vrol_vx */
    3, /* vror_vi */
    3, /* vror_vv */
    3, /* vror_vx */
    3, /* vrsub_vi */
    3, /* vrsub_vx */
    3, /* vs1r_v */
    3, /* vs2r_v */
    3, /* vs4r_v */
    3, /* vs8r_v */
    3, /* vsadd_vi */
    3, /* vsadd_vv */
    3, /* vsadd_vx */
    3, /* vsaddu_vi */
    3, /* vsaddu_vv */
    3, /* vsaddu_vx */
    3, /* vsbc_vvm */
    3, /* vsbc_vxm */
    3, /* vse16_v */
    3, /* vse1_v */
    3, /* vse32_v */
    3, /* vse64_v */
    3, /* vse8_v */
    3, /* vsetivli */
    3, /* vsetvl */
    3, /* vsetvli */
    3, /* vsext_vf2 */
    3, /* vsext_vf4 */
    3, /* vsext_vf8 */
    3, /* vsha2ch_vv */
    3, /* vsha2cl_vv */
    3, /* vsha2ms_vv */
    3, /* vslide1down_vx */
    3, /* vslide1up_vx */
    3, /* vslidedown_vi */
    3, /* vslidedown_vx */
    3, /* vslideup_vi */
    3, /* vslideup_vx */
    3, /* vsll_vi */
    3, /* vsll_vv */
    3, /* vsll_vx */
    3, /* vsm3c_vi */
    3, /* vsm3me_vv */
    3, /* vsm4k_vi */
    3, /* vsm4r_vs */
    3, /* vsm4r_vv */
    3, /* vsm_v */
    3, /* vsmul_vv */
    3, /* vsmul_vx */
    3, /* vsoxei16_v */
    3, /* vsoxei32_v */
    3, /* vsoxei64_v */
    3, /* vsoxei8_v */
    3, /* vsra_vi */
    3, /* vsra_vv */
    3, /* vsra_vx */
    3, /* vsrl_vi */
    3, /* vsrl_vv */
    3, /* vsrl_vx */
    3, /* vsse16_v */
    3, /* vsse32_v */
    3, /* vsse64_v */
    3, /* vsse8_v */
    3, /* vssra_vi */
    3, /* vssra_vv */
    3, /* vssra_vx */
    3, /* vssrl_vi */
    3, /* vssrl_vv */
    3, /* vssrl_vx */
    3, /* vssub_vv */
    3, /* vssub_vx */
    3, /* vssubu_vv */
    3, /* vssubu_vx */
    3, /* vsub_vv */
    3, /* vsub_vx */
    3, /* vsuxei16_v */
    3, /* vsuxei32_v */
    3, /* vsuxei64_v */
    3, /* vsuxei8_v */
    3, /* vwadd_vv */
    3, /* vwadd_vx */
    3, /* vwadd_wv */
    3, /* vwadd_wx */
    3, /* vwaddu_vv */
    3, /* vwaddu_vx */
    3, /* vwaddu_wv */
    3, /* vwaddu_wx */
    3, /* vwmacc_vv */
    3, /* vwmacc_vx */
    3, /* vwmaccsu_vv */
    3, /* vwmaccsu_vx */
    3, /* vwmaccu_vv */
    3, /* vwmaccu_vx */
    3, /* vwmaccus_vx */
    3, /* vwmul_vv */
    3, /* vwmul_vx */
    3, /* vwmulsu_vv */
    3, /* vwmulsu_vx */
    3, /* vwmulu_vv */
    3, /* vwmulu_vx */
    3, /* vwredsum_vs */
    3, /* vwredsumu_vs */
    3, /* vwsll_vi */
    3, /* vwsll_vv */
    3, /* vwsll_vx */
    3, /* vwsub_vv */
    3, /* vwsub_vx */
    3, /* vwsub_wv */
    3, /* vwsub_wx */
    3, /* vwsubu_vv */
    3, /* vwsubu_vx */
    3, /* vwsubu_wv */
    3, /* vwsubu_wx */
    3, /* vxor_vi */
    3, /* vxor_vv */
    3, /* vxor_vx */
    3, /* vzext_vf2 */
    3, /* vzext_vf4 */
    3, /* vzext_vf8 */
    3, /* wfi */
    3, /* wrs_nto */
    3, /* wrs_sto */
    3, /* xnor */
    3, /* xor */
    3, /* xori */
    3, /* xperm4 */
    3, /* xperm8 */
    3, /* zext_b */
    2, /* zext_h */
    1, /* zext_h_rv32 */
    2, /* zext_w */
    1, /* zip */
];

pub static ALL_OPCODES: [Opcode; 1039] = [
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
    Opcode::CSSPOPCHKX5,
    Opcode::CSSPUSHX1,
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
    Opcode::FCVTBF16S,
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
    Opcode::FCVTSBF16,
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
    Opcode::LPAD,
    Opcode::LRD,
    Opcode::LRW,
    Opcode::LUI,
    Opcode::LW,
    Opcode::LWU,
    Opcode::MAX,
    Opcode::MAXU,
    Opcode::MIN,
    Opcode::MINU,
    Opcode::MNRET,
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
    Opcode::SCTRCLR,
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
    Opcode::SSAMOSWAPD,
    Opcode::SSAMOSWAPW,
    Opcode::SSPOPCHKX1,
    Opcode::SSPOPCHKX5,
    Opcode::SSPUSHX1,
    Opcode::SSPUSHX5,
    Opcode::SSRDP,
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
    Opcode::VFNCVTBF16FFW,
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
    Opcode::VFWCVTBF16FFV,
    Opcode::VFWMACCVF,
    Opcode::VFWMACCVV,
    Opcode::VFWMACCBF16VF,
    Opcode::VFWMACCBF16VV,
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

pub static SHORT_OPCODE: [bool; 1039] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, false, false, false, false, false, false, false,
    false, false, true, true, true, true, true, true, true, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false,
];
pub const SHORT_OPCODES: [Opcode; 80] = [
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
    Opcode::CSSPOPCHKX5,
    Opcode::CSSPUSHX1,
    Opcode::CSUB,
    Opcode::CSUBW,
    Opcode::CSW,
    Opcode::CSWSP,
    Opcode::CXOR,
    Opcode::CZEXTB,
    Opcode::CZEXTH,
    Opcode::CZEXTW,
    Opcode::CMJALT,
    Opcode::CMMVA01S,
    Opcode::CMMVSA01,
    Opcode::CMPOP,
    Opcode::CMPOPRET,
    Opcode::CMPOPRETZ,
    Opcode::CMPUSH,
];
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(u32)]
pub enum Opcode {
    /// Integer add
    ///
    /// Add the value in rs1 to rs2, and store the result in rd.
    /// Any overflow is thrown away.
    ///
    /// # Forms
    /// Assembly: `add xd, xs1, xs2`
    ADD,
    /// Add unsigned word
    ///
    /// This instruction performs an XLEN-wide addition between rs2 and the
    /// zero-extended least-significant word of rs1.
    ///
    /// # Forms
    /// Assembly: `add.uw xd, xs1, xs2`
    ADDUW,
    /// Add immediate
    ///
    /// Add an immediate to the value in rs1, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `addi xd, xs1, imm`
    ADDI,
    /// Add immediate word
    ///
    /// Add an immediate to the 32-bit value in rs1, and store the sign extended result in rd
    ///
    /// # Forms
    /// Assembly: `addiw xd, xs1, imm`
    ADDIW,
    /// Add word
    ///
    /// Add the 32-bit values in rs1 to rs2, and store the sign-extended result in rd.
    /// Any overflow is thrown away.
    ///
    /// # Forms
    /// Assembly: `addw xd, xs1, xs2`
    ADDW,
    /// RISC-V `aes32dsi` instruction.
    ///
    /// # Forms
    /// Assembly: `aes32dsi xd, xs1, xs2, bs`
    AES32DSI,
    /// RISC-V `aes32dsmi` instruction.
    ///
    /// # Forms
    /// Assembly: `aes32dsmi xd, xs1, xs2, bs`
    AES32DSMI,
    /// RISC-V `aes32esi` instruction.
    ///
    /// # Forms
    /// Assembly: `aes32esi xd, xs1, xs2, bs`
    AES32ESI,
    /// RISC-V `aes32esmi` instruction.
    ///
    /// # Forms
    /// Assembly: `aes32esmi xd, xs1, xs2, bs`
    AES32ESMI,
    /// RISC-V `aes64ds` instruction.
    ///
    /// # Forms
    /// Assembly: `aes64ds xd, xs1, xs2`
    AES64DS,
    /// RISC-V `aes64dsm` instruction.
    ///
    /// # Forms
    /// Assembly: `aes64dsm xd, xs1, xs2`
    AES64DSM,
    /// RISC-V `aes64es` instruction.
    ///
    /// # Forms
    /// Assembly: `aes64es xd, xs1, xs2`
    AES64ES,
    /// RISC-V `aes64esm` instruction.
    ///
    /// # Forms
    /// Assembly: `aes64esm xd, xs1, xs2`
    AES64ESM,
    /// RISC-V `aes64im` instruction.
    ///
    /// # Forms
    /// Assembly: `aes64im xd, xs1`
    AES64IM,
    /// RISC-V `aes64ks1i` instruction.
    ///
    /// # Forms
    /// Assembly: `aes64ks1i xd, xs1, rnum`
    AES64KS1I,
    /// RISC-V `aes64ks2` instruction.
    ///
    /// # Forms
    /// Assembly: `aes64ks2 xd, xs1, xs2`
    AES64KS2,
    /// RISC-V `amoadd.b` instruction.
    ///
    /// # Forms
    /// Assembly: `amoadd.b xd, xs1, xs2, aq, rl`
    AMOADDB,
    /// Atomic fetch-and-add doubleword
    ///
    /// Atomically:
    ///
    ///   * Load the doubleword at address _rs1_
    ///   * Write the loaded value into _rd_
    ///   * Add the value of register _rs2_ to the loaded value
    ///   * Write the sum to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amoadd.d xd, xs2, (xs1)`
    AMOADDD,
    /// RISC-V `amoadd.h` instruction.
    ///
    /// # Forms
    /// Assembly: `amoadd.h xd, xs1, xs2, aq, rl`
    AMOADDH,
    /// Atomic fetch-and-add word
    ///
    /// Atomically:
    ///
    ///   * Load the word at address _rs1_
    ///   * Write the sign-extended value into _rd_
    ///   * Add the least-significant word of register _rs2_ to the loaded value
    ///   * Write the sum to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amoadd.w xd, xs2, (xrs1)`
    AMOADDW,
    /// RISC-V `amoand.b` instruction.
    ///
    /// # Forms
    /// Assembly: `amoand.b xd, xs1, xs2, aq, rl`
    AMOANDB,
    /// Atomic fetch-and-and doubleword
    ///
    /// Atomically:
    ///
    ///   * Load the doubleword at address _rs1_
    ///   * Write the loaded value into _rd_
    ///   * AND the value of register _rs2_ to the loaded value
    ///   * Write the result to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amoand.d xd, xs2, (xrs1)`
    AMOANDD,
    /// RISC-V `amoand.h` instruction.
    ///
    /// # Forms
    /// Assembly: `amoand.h xd, xs1, xs2, aq, rl`
    AMOANDH,
    /// Atomic fetch-and-and word
    ///
    /// Atomically:
    ///
    ///   * Load the word at address _rs1_
    ///   * Write the sign-extended value into _rd_
    ///   * AND the least-significant word of register _rs2_ to the loaded value
    ///   * Write the result to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amoand.w xd, xs2, (xrs1)`
    AMOANDW,
    /// RISC-V `amocas.b` instruction.
    ///
    /// # Forms
    /// Assembly: `amocas.b xd, xs1, xs2, aq, rl`
    AMOCASB,
    /// RISC-V `amocas.d` instruction.
    ///
    /// # Forms
    /// Assembly: `amocas.d xd, xs1, xs2, aq, rl`
    AMOCASD,
    /// RISC-V `amocas.h` instruction.
    ///
    /// # Forms
    /// Assembly: `amocas.h xd, xs1, xs2, aq, rl`
    AMOCASH,
    /// RISC-V `amocas.q` instruction.
    ///
    /// # Forms
    /// Assembly: `amocas.q xd, xs1, xs2, aq, rl`
    AMOCASQ,
    /// RISC-V `amocas.w` instruction.
    ///
    /// # Forms
    /// Assembly: `amocas.w xd, xs1, xs2, aq, rl`
    AMOCASW,
    /// RISC-V `amomax.b` instruction.
    ///
    /// # Forms
    /// Assembly: `amomax.b xd, xs1, xs2, aq, rl`
    AMOMAXB,
    /// Atomic MAX doubleword
    ///
    /// Atomically:
    ///
    ///   * Load the doubleword at address _rs1_
    ///   * Write the loaded value into _rd_
    ///   * Signed compare the value of register _rs2_ to the loaded value, and select the maximum value
    ///   * Write the maximum to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amomax.d xd, xs2, (xrs1)`
    AMOMAXD,
    /// RISC-V `amomax.h` instruction.
    ///
    /// # Forms
    /// Assembly: `amomax.h xd, xs1, xs2, aq, rl`
    AMOMAXH,
    /// Atomic MAX word
    ///
    /// Atomically:
    ///
    ///   * Load the word at address _rs1_
    ///   * Write the sign-extended value into _rd_
    ///   * Signed compare the least-significant word of register _rs2_ to the loaded value, and select the maximum value
    ///   * Write the maximum to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amomax.w xd, xs2, (xrs1)`
    AMOMAXW,
    /// RISC-V `amomaxu.b` instruction.
    ///
    /// # Forms
    /// Assembly: `amomaxu.b xd, xs1, xs2, aq, rl`
    AMOMAXUB,
    /// Atomic MAX unsigned doubleword
    ///
    /// Atomically:
    ///
    ///   * Load the doubleword at address _rs1_
    ///   * Write the loaded value into _rd_
    ///   * Unsigned compare the value of register _rs2_ to the loaded value, and select the maximum value
    ///   * Write the maximum to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amomaxu.d xd, xs2, (xrs1)`
    AMOMAXUD,
    /// RISC-V `amomaxu.h` instruction.
    ///
    /// # Forms
    /// Assembly: `amomaxu.h xd, xs1, xs2, aq, rl`
    AMOMAXUH,
    /// Atomic MAX unsigned word
    ///
    /// Atomically:
    ///
    ///   * Load the word at address _rs1_
    ///   * Write the sign-extended value into _rd_
    ///   * Unsigned compare the least-significant word of register _rs2_ to the loaded value, and select the maximum value
    ///   * Write the maximum to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amomaxu.w xd, xs2, (xrs1)`
    AMOMAXUW,
    /// RISC-V `amomin.b` instruction.
    ///
    /// # Forms
    /// Assembly: `amomin.b xd, xs1, xs2, aq, rl`
    AMOMINB,
    /// Atomic MIN doubleword
    ///
    /// Atomically:
    ///
    ///   * Load the doubleword at address _rs1_
    ///   * Write the loaded value into _rd_
    ///   * Signed compare the value of register _rs2_ to the loaded value, and select the minimum value
    ///   * Write the minimum to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amomin.d xd, xs2, (xrs1)`
    AMOMIND,
    /// RISC-V `amomin.h` instruction.
    ///
    /// # Forms
    /// Assembly: `amomin.h xd, xs1, xs2, aq, rl`
    AMOMINH,
    /// Atomic MIN word
    ///
    /// Atomically:
    ///
    ///   * Load the word at address _rs1_
    ///   * Write the sign-extended value into _rd_
    ///   * Signed compare the least-significant word of register _rs2_ to the loaded value, and select the minimum value
    ///   * Write the result to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amomin.w xd, xs2, (xrs1)`
    AMOMINW,
    /// RISC-V `amominu.b` instruction.
    ///
    /// # Forms
    /// Assembly: `amominu.b xd, xs1, xs2, aq, rl`
    AMOMINUB,
    /// Atomic MIN unsigned doubleword
    ///
    /// Atomically:
    ///
    ///   * Load the doubleword at address _rs1_
    ///   * Write the loaded value into _rd_
    ///   * Unsigned compare the value of register _rs2_ to the loaded value, and select the minimum value
    ///   * Write the minimum to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amominu.d xd, xs2, (xrs1)`
    AMOMINUD,
    /// RISC-V `amominu.h` instruction.
    ///
    /// # Forms
    /// Assembly: `amominu.h xd, xs1, xs2, aq, rl`
    AMOMINUH,
    /// Atomic MIN unsigned word
    ///
    /// Atomically:
    ///
    ///   * Load the word at address _rs1_
    ///   * Write the sign-extended value into _rd_
    ///   * Unsigned compare the least-significant word of register _rs2_ to the loaded word, and select the minimum value
    ///   * Write the result to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amominu.w xd, xs2, (xrs1)`
    AMOMINUW,
    /// RISC-V `amoor.b` instruction.
    ///
    /// # Forms
    /// Assembly: `amoor.b xd, xs1, xs2, aq, rl`
    AMOORB,
    /// Atomic fetch-and-or doubleword
    ///
    /// Atomically:
    ///
    ///   * Load the doubleword at address _rs1_
    ///   * Write the loaded value into _rd_
    ///   * OR the value of register _rs2_ to the loaded value
    ///   * Write the result to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amoor.d xd, xs2, (xrs1)`
    AMOORD,
    /// RISC-V `amoor.h` instruction.
    ///
    /// # Forms
    /// Assembly: `amoor.h xd, xs1, xs2, aq, rl`
    AMOORH,
    /// Atomic fetch-and-or word
    ///
    /// Atomically:
    ///
    ///   * Load the word at address _rs1_
    ///   * Write the sign-extended value into _rd_
    ///   * OR the least-significant word of register _rs2_ to the loaded value
    ///   * Write the result to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amoor.w xd, xs2, (xrs1)`
    AMOORW,
    /// RISC-V `amoswap.b` instruction.
    ///
    /// # Forms
    /// Assembly: `amoswap.b xd, xs1, xs2, aq, rl`
    AMOSWAPB,
    /// Atomic SWAP doubleword
    ///
    /// Atomically:
    ///
    ///   * Load the doubleword at address _rs1_
    ///   * Write the value into _rd_
    ///   * Store the value of register _rs2_ to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amoswap.d xd, xs2, (xrs1)`
    AMOSWAPD,
    /// RISC-V `amoswap.h` instruction.
    ///
    /// # Forms
    /// Assembly: `amoswap.h xd, xs1, xs2, aq, rl`
    AMOSWAPH,
    /// Atomic SWAP word
    ///
    /// Atomically:
    ///
    ///   * Load the word at address _rs1_
    ///   * Write the sign-extended value into _rd_
    ///   * Store the least-significant word of register _rs2_ to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amoswap.w xd, xs2, (xrs1)`
    AMOSWAPW,
    /// RISC-V `amoxor.b` instruction.
    ///
    /// # Forms
    /// Assembly: `amoxor.b xd, xs1, xs2, aq, rl`
    AMOXORB,
    /// Atomic fetch-and-xor doubleword
    ///
    /// Atomically:
    ///
    ///   * Load the doubleword at address _rs1_
    ///   * Write the loaded value into _rd_
    ///   * XOR the value of register _rs2_ to the loaded value
    ///   * Write the result to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amoxor.d xd, xs2, (xrs1)`
    AMOXORD,
    /// RISC-V `amoxor.h` instruction.
    ///
    /// # Forms
    /// Assembly: `amoxor.h xd, xs1, xs2, aq, rl`
    AMOXORH,
    /// Atomic fetch-and-xor word
    ///
    /// Atomically:
    ///
    ///   * Load the word at address _rs1_
    ///   * Write the sign-extended value into _rd_
    ///   * XOR the least-significant word of register _rs2_ to the loaded value
    ///   * Write the result to the address in _rs1_
    ///
    /// # Forms
    /// Assembly: `amoxor.w xd, xs2, (xrs1)`
    AMOXORW,
    /// And
    ///
    /// And rs1 with rs2, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `and xd, xs1, xs2`
    AND,
    /// And immediate
    ///
    /// And an immediate to the value in rs1, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `andi xd, xs1, imm`
    ANDI,
    /// AND with inverted operand
    ///
    /// This instruction performs the bitwise logical AND operation between `rs1` and the
    /// bitwise inversion of `rs2`.
    ///
    /// # Forms
    /// Assembly: `andn xd, xs1, xs2`
    ANDN,
    /// Add upper immediate to pc
    ///
    /// Add an immediate to the current PC.
    ///
    /// # Forms
    /// Assembly: `auipc xd, imm`
    AUIPC,
    /// Single-Bit clear (Register)
    ///
    /// This instruction returns rs1 with a single bit cleared at the index specified in rs2.
    /// The index is read from the lower log2(XLEN) bits of rs2.
    ///
    /// # Forms
    /// Assembly: `bclr xd, xs1, xs2`
    BCLR,
    /// Single-Bit clear (Immediate)
    ///
    /// This instruction returns rs1 with a single bit cleared at the index specified in shamt. The
    /// index is read from the lower log2(XLEN) bits of shamt. For RV32, the encodings corresponding
    /// to shamt\[5\]=1 are reserved.
    ///
    /// # Forms
    /// Assembly: `bclri xd, xs1, shamt`
    BCLRI,
    /// Single-Bit clear (Immediate)
    ///
    /// This instruction returns rs1 with a single bit cleared at the index specified in shamt. The
    /// index is read from the lower log2(XLEN) bits of shamt. For RV32, the encodings corresponding
    /// to shamt\[5\]=1 are reserved.
    ///
    /// # Forms
    /// Assembly: `bclri.rv32 xd, xs1, shamt`
    BCLRIRV32,
    /// Branch if equal
    ///
    /// Branch to PC + imm if
    /// the value in register rs1 is equal to the value in register rs2.
    ///
    /// Raise a `MisalignedAddress` exception if PC + imm is misaligned.
    ///
    /// # Forms
    /// Assembly: `beq xs1, xs2, imm`
    BEQ,
    /// RISC-V `beqz` instruction.
    ///
    /// # Forms
    /// Assembly: `beqz rs1 bimm12lohi`
    BEQZ,
    /// Single-Bit extract (Register)
    ///
    /// This instruction returns a single bit extracted from rs1 at the index specified in rs2.
    /// The index is read from the lower log2(XLEN) bits of rs2.
    ///
    /// # Forms
    /// Assembly: `bext xd, xs1, xs2`
    BEXT,
    /// Single-Bit extract (Immediate)
    ///
    /// This instruction returns a single bit extracted from rs1 at the index specified in rs2.
    /// The index is read from the lower log2(XLEN) bits of shamt. For RV32, the encodings
    /// corresponding to shamt\[5\]=1 are reserved.
    ///
    /// # Forms
    /// Assembly: `bexti xd, xs1, shamt`
    BEXTI,
    /// Single-Bit extract (Immediate)
    ///
    /// This instruction returns a single bit extracted from rs1 at the index specified in rs2.
    /// The index is read from the lower log2(XLEN) bits of shamt. For RV32, the encodings
    /// corresponding to shamt\[5\]=1 are reserved.
    ///
    /// # Forms
    /// Assembly: `bexti.rv32 xd, xs1, shamt`
    BEXTIRV32,
    /// Branch if greater than or equal
    ///
    /// Branch to PC + imm if
    /// the signed value in register rs1 is greater than or equal to the signed value in register rs2.
    ///
    /// Raise a `MisalignedAddress` exception if PC + imm is misaligned.
    ///
    /// # Forms
    /// Assembly: `bge xs1, xs2, imm`
    BGE,
    /// Branch if greater than or equal unsigned
    ///
    /// Branch to PC + imm if
    /// the unsigned value in register rs1 is greater than or equal to the unsigned value in register rs2.
    ///
    /// Raise a `MisalignedAddress` exception if PC + imm is misaligned.
    ///
    /// # Forms
    /// Assembly: `bgeu xs1, xs2, imm`
    BGEU,
    /// RISC-V `bgez` instruction.
    ///
    /// # Forms
    /// Assembly: `bgez rs1 bimm12lohi`
    BGEZ,
    /// RISC-V `bgt` instruction.
    ///
    /// # Forms
    /// Assembly: `bgt rs1 rs2 bimm12lohi`
    BGT,
    /// RISC-V `bgtu` instruction.
    ///
    /// # Forms
    /// Assembly: `bgtu rs1 rs2 bimm12lohi`
    BGTU,
    /// RISC-V `bgtz` instruction.
    ///
    /// # Forms
    /// Assembly: `bgtz rs2 bimm12lohi`
    BGTZ,
    /// Single-Bit invert (Register)
    ///
    /// This instruction returns rs1 with a single bit inverted at the index specified in rs2.
    /// The index is read from the lower log2(XLEN) bits of rs2.
    ///
    /// # Forms
    /// Assembly: `binv xd, xs1, xs2`
    BINV,
    /// Single-Bit invert (Immediate)
    ///
    /// This instruction returns rs1 with a single bit inverted at the index specified in shamt.
    /// The index is read from the lower log2(XLEN) bits of shamt.
    /// For RV32, the encodings corresponding to shamt\[5\]=1 are reserved.
    ///
    /// # Forms
    /// Assembly: `binvi xd, xs1, shamt`
    BINVI,
    /// Single-Bit invert (Immediate)
    ///
    /// This instruction returns rs1 with a single bit inverted at the index specified in shamt.
    /// The index is read from the lower log2(XLEN) bits of shamt.
    /// For RV32, the encodings corresponding to shamt\[5\]=1 are reserved.
    ///
    /// # Forms
    /// Assembly: `binvi.rv32 xd, xs1, shamt`
    BINVIRV32,
    /// RISC-V `ble` instruction.
    ///
    /// # Forms
    /// Assembly: `ble rs1 rs2 bimm12lohi`
    BLE,
    /// RISC-V `bleu` instruction.
    ///
    /// # Forms
    /// Assembly: `bleu rs1 rs2 bimm12lohi`
    BLEU,
    /// RISC-V `blez` instruction.
    ///
    /// # Forms
    /// Assembly: `blez rs2 bimm12lohi`
    BLEZ,
    /// Branch if less than
    ///
    /// Branch to PC + imm if
    /// the signed value in register rs1 is less than the signed value in register rs2.
    ///
    /// Raise a `MisalignedAddress` exception if PC + imm is misaligned.
    ///
    /// # Forms
    /// Assembly: `blt xs1, xs2, imm`
    BLT,
    /// Branch if less than unsigned
    ///
    /// Branch to PC + imm if
    /// the unsigned value in register rs1 is less than the unsigned value in register rs2.
    ///
    /// Raise a `MisalignedAddress` exception if PC + imm is misaligned.
    ///
    /// # Forms
    /// Assembly: `bltu xs1, xs2, imm`
    BLTU,
    /// RISC-V `bltz` instruction.
    ///
    /// # Forms
    /// Assembly: `bltz rs1 bimm12lohi`
    BLTZ,
    /// Branch if not equal
    ///
    /// Branch to PC + imm if
    /// the value in register rs1 is not equal to the value in register rs2.
    ///
    /// Raise a `MisalignedAddress` exception if PC + imm is misaligned.
    ///
    /// # Forms
    /// Assembly: `bne xs1, xs2, imm`
    BNE,
    /// RISC-V `bnez` instruction.
    ///
    /// # Forms
    /// Assembly: `bnez rs1 bimm12lohi`
    BNEZ,
    /// Reverse bits in bytes
    ///
    /// This instruction reverses the order of the bits in every byte of a register.
    ///
    /// # Forms
    /// Assembly: `brev8 xd, xs1`
    BREV8,
    /// Single-Bit set (Register)
    ///
    /// This instruction returns rs1 with a single bit set at the index specified in rs2.
    /// The index is read from the lower log2(XLEN) bits of rs2.
    ///
    /// # Forms
    /// Assembly: `bset xd, xs1, xs2`
    BSET,
    /// Single-Bit set (Immediate)
    ///
    /// This instruction returns rs1 with a single bit set at the index specified in shamt.
    /// The index is read from the lower log2(XLEN) bits of shamt.
    /// For RV32, the encodings corresponding to shamt\[5\]=1 are reserved.
    ///
    /// # Forms
    /// Assembly: `bseti xd, xs1, shamt`
    BSETI,
    /// Single-Bit set (Immediate)
    ///
    /// This instruction returns rs1 with a single bit set at the index specified in shamt.
    /// The index is read from the lower log2(XLEN) bits of shamt.
    /// For RV32, the encodings corresponding to shamt\[5\]=1 are reserved.
    ///
    /// # Forms
    /// Assembly: `bseti.rv32 xd, xs1, shamt`
    BSETIRV32,
    /// Add
    ///
    /// Add the value in rs2 to rd, and store the result in rd.
    /// C.ADD expands into `add rd, rd, rs2`.
    ///
    /// # Forms
    /// Assembly: `c.add xd, rs2`
    CADD,
    /// Add a sign-extended non-zero immediate
    ///
    /// C.ADDI adds the non-zero sign-extended 6-bit immediate to the value in register rd then writes the result to rd.
    /// C.ADDI expands into `addi rd, rd, imm`.
    /// C.ADDI is only valid when rd &ne; x0 and imm &ne; 0.
    /// The code points with rd=x0 encode the C.NOP instruction; the remaining code points with imm=0 encode HINTs.
    ///
    /// # Forms
    /// Assembly: `c.addi xd, imm`
    CADDI,
    /// Add a sign-extended non-zero immediate
    ///
    /// C.ADDI16SP adds the non-zero sign-extended 6-bit immediate to the value in the stack pointer (sp=x2), where the immediate is scaled to represent multiples of 16 in the range (-512,496).
    /// C.ADDI16SP is used to adjust the stack pointer in procedure prologues and epilogues.
    /// It expands into `addi x2, x2, nzimm\[9:4\]`.
    /// C.ADDI16SP is only valid when nzimm &ne; 0; the code point with nzimm=0 is reserved.
    ///
    /// # Forms
    /// Assembly: `c.addi16sp imm`
    CADDI16SP,
    /// Add a zero-extended non-zero immediate, scaled by 4, to the stack pointer
    ///
    /// Adds a zero-extended non-zero immediate, scaled by 4, to the stack pointer, x2, and writes the result to rd'.
    /// This instruction is used to generate pointers to stack-allocated variables.
    /// It expands to `addi rd', x2, nzuimm\[9:2\]`.
    /// C.ADDI4SPN is only valid when nzuimm &ne; 0; the code points with nzuimm=0 are reserved.
    ///
    /// # Forms
    /// Assembly: `c.addi4spn xd, imm`
    CADDI4SPN,
    /// Add a sign-extended non-zero immediate
    ///
    /// C.ADDIW is an RV64C/RV128C-only instruction that performs the same computation as C.ADDI but produces a 32-bit result, then sign-extends result to 64 bits.
    /// C.ADDIW expands into `addiw rd, rd, imm`.
    /// The immediate can be zero for C.ADDIW, where this corresponds to `sext.w rd`.
    /// C.ADDIW is only valid when rd &ne; x0; the code points with rd=x0 are reserved.
    ///
    /// # Forms
    /// Assembly: `c.addiw xd, imm`
    CADDIW,
    /// Add word
    ///
    /// Add the 32-bit values in rs2 from rd, and store the result in rd.
    /// The rd and rs2 register indexes should be used as rd+8 and rs2+8 (registers x8-x15).
    /// C.ADDW expands into `addw rd, rd, rs2`.
    ///
    /// # Forms
    /// Assembly: `c.addw xd, rs2`
    CADDW,
    /// And
    ///
    /// And rd with rs2, and store the result in rd
    /// The rd and rs2 register indexes should be used as rd+8 and rs2+8 (registers x8-x15).
    /// C.AND expands into `and rd, rd, rs2`.
    ///
    /// # Forms
    /// Assembly: `c.and xd, rs2`
    CAND,
    /// And immediate
    ///
    /// And an immediate to the value in rd, and store the result in rd.
    /// The rd register index should be used as rd+8 (registers x8-x15).
    /// C.ANDI expands into `andi rd, rd, imm`.
    ///
    /// # Forms
    /// Assembly: `c.andi xd, imm`
    CANDI,
    /// Branch if Equal Zero
    ///
    /// C.BEQZ performs conditional control transfers. The offset is sign-extended and added to the pc to form the branch target address. It can therefore target a &pm;256 B range. C.BEQZ takes the branch if the value in register rs1' is zero.
    /// It expands to `beq` `rs1, x0, offset`.
    ///
    /// # Forms
    /// Assembly: `c.beqz xs1, imm`
    CBEQZ,
    /// Branch if NOT Equal Zero
    ///
    /// C.BEQZ performs conditional control transfers. The offset is sign-extended and added to the pc to form the branch target address. It can therefore target a &pm;256 B range. C.BEQZ takes the branch if the value in register rs1' is NOT zero.
    /// It expands to `beq` `rs1, x0, offset`.
    ///
    /// # Forms
    /// Assembly: `c.bnez xs1, imm`
    CBNEZ,
    /// Breakpoint exception.
    ///
    /// The C.EBREAK instruction is used by debuggers to cause control to be transferred back to
    /// a debugging environment. Unless overridden by an external debug environment,
    /// C.EBREAK raises a breakpoint exception and performs no other operation.
    ///
    /// \[NOTE\]
    /// As described in the `C` Standard Extension for Compressed Instructions, the `c.ebreak`
    /// instruction performs the same operation as the EBREAK instruction.
    ///
    /// EBREAK causes the receiving privilege mode's epc register to be set to the address of
    /// the EBREAK instruction itself, not the address of the following instruction.
    /// As EBREAK causes a synchronous exception, it is not considered to retire,
    /// and should not increment the `minstret` CSR.
    ///
    /// # Forms
    /// Assembly: `c.ebreak " "`
    CEBREAK,
    /// Load double-precision
    ///
    /// Loads a double precision floating-point value from memory into register rd.
    /// It computes an effective address by adding the zero-extended offset, scaled by 8,
    /// to the base address in register rs1.
    /// It expands to `fld` `rd, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.fld xd, imm(xs1)`
    CFLD,
    /// Load doubleword into floating-point register from stack
    ///
    /// Loads a double-precision floating-point value from memory into floating-point register rd.
    /// It computes its effective address by adding the zero-extended offset, scaled by 8,
    /// to the stack pointer, x2.
    /// It expands to `fld` `rd, offset(x2)`.
    ///
    /// # Forms
    /// Assembly: `c.fldsp fd, imm(sp)`
    CFLDSP,
    /// Load single-precision
    ///
    /// Loads a single precision floating-point value from memory into register rd.
    /// It computes an effective address by adding the zero-extended offset, scaled by 4,
    /// to the base address in register rs1.
    /// It expands to `flw` `rd, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.flw xd, imm(xs1)`
    CFLW,
    /// Load word into floating-point register from stack
    ///
    /// Loads a single-precision floating-point value from memory into floating-point register rd.
    /// It computes its effective address by adding the zero-extended offset, scaled by 4,
    /// to the stack pointer, x2.
    /// It expands to `flw` `rd, offset(x2)`.
    ///
    /// # Forms
    /// Assembly: `c.flwsp fd, imm(sp)`
    CFLWSP,
    /// Store double-precision
    ///
    /// Stores a double precision floating-point value in register rs2 to memory.
    /// It computes an effective address by adding the zero-extended offset, scaled by 8,
    /// to the base address in register rs1.
    /// It expands to `fsd` `rs2, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.fsd xs2, imm(xs1)`
    CFSD,
    /// Store double-precision value to stack
    ///
    /// Stores a double-precision floating-point value in floating-point register rs2 to memory.
    /// It computes an effective address by adding the zero-extended offset, scaled by 8,
    /// to the stack pointer, x2.
    /// It expands to `fsd` `rs2, offset(x2)`.
    ///
    /// # Forms
    /// Assembly: `c.fsdsp fs2, imm(sp)`
    CFSDSP,
    /// Store single-precision
    ///
    /// Stores a single precision floating-point value in register rs2 to memory.
    /// It computes an effective address by adding the zero-extended offset, scaled by 4,
    /// to the base address in register rs1.
    /// It expands to `fsw` `rs2, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.fsw xs2, imm(xs1)`
    CFSW,
    /// Store single-precision value to stack
    ///
    /// Stores a single-precision floating-point value in floating-point register rs2 to memory.
    /// It computes an effective address by adding the zero-extended offset, scaled by 4,
    /// to the stack pointer, x2.
    /// It expands to `fsw` `rs2, offset(x2)`.
    ///
    /// # Forms
    /// Assembly: `c.fswsp fs2, imm(sp)`
    CFSWSP,
    /// Jump
    ///
    /// C.J performs an unconditional control transfer. The offset is sign-extended and added to the pc to form the jump target address. C.J can therefore target a &pm;2 KiB range.
    /// It expands to `jal` `x0, offset`.
    ///
    /// # Forms
    /// Assembly: `c.j imm`
    CJ,
    /// Jump and Link
    ///
    /// C.JAL is an RV32C-only instruction that performs the same operation as C.J, but additionally writes the address of the instruction following the jump (pc+2) to the link register, x1.
    /// It expands to `jal` `x1, offset`.
    ///
    /// # Forms
    /// Assembly: `c.jal imm`
    CJAL,
    /// Jump and Link Register.
    ///
    /// C.JALR (jump and link register) performs the same operation as C.JR, but additionally writes the address of the instruction following the jump (pc+2) to the link register, x1.
    /// C.JALR expands to jalr x1, 0(rs1).
    ///
    /// # Forms
    /// Assembly: `c.jalr xs1`
    CJALR,
    /// Jump Register
    ///
    /// C.JR (jump register) performs an unconditional control transfer to the address in register rs1.
    /// C.JR expands to jalr x0, 0(rs1).
    ///
    /// # Forms
    /// Assembly: `c.jr xs1`
    CJR,
    /// Load unsigned byte, 16-bit encoding
    ///
    /// Loads a 8-bit value from memory into register rd.
    /// It computes an effective address by adding the zero-extended offset, to the base address in register rs1.
    /// It expands to `lbu` `rd, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.lbu xd, imm(xs1)`
    CLBU,
    /// Load double
    ///
    /// Loads a 64-bit value from memory into register rd.
    /// It computes an effective address by adding the zero-extended offset, scaled by 8,
    /// to the base address in register rs1.
    /// It expands to `ld` `rd, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.ld xd, imm(xs1)`
    CLD,
    /// Load doubleword from stack pointer
    ///
    /// C.LDSP is an RV64C/RV128C-only instruction that loads a 64-bit value from memory
    /// into register rd.
    /// It computes its effective address by adding the zero-extended offset, scaled by 8,
    /// to the stack pointer, x2.
    /// It expands to `ld` `rd, offset(x2)`.
    /// C.LDSP is only valid when rd &ne; x0 the code points with rd=x0 are reserved.
    ///
    /// # Forms
    /// Assembly: `c.ldsp xd, imm(sp)`
    CLDSP,
    /// Load signed halfword, 16-bit encoding
    ///
    /// Loads a 16-bit value from memory into register rd.
    /// It computes an effective address by adding the zero-extended offset, to the base address in register rs1.
    /// It expands to `lh` `rd, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.lh xd, imm(xs1)`
    CLH,
    /// Load unsigned halfword, 16-bit encoding
    ///
    /// Loads a 16-bit value from memory into register rd.
    /// It computes an effective address by adding the zero-extended offset, to the base address in register rs1.
    /// It expands to `lhu` `rd, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.lhu xd, imm(xs1)`
    CLHU,
    /// Load the sign-extended 6-bit immediate
    ///
    /// C.LI loads the sign-extended 6-bit immediate, imm, into register rd.
    /// C.LI expands into `addi rd, x0, imm`.
    /// C.LI is only valid when rd &ne; x0; the code points with rd=x0 encode HINTs.
    ///
    /// # Forms
    /// Assembly: `c.li xd, imm`
    CLI,
    /// Load the non-zero 6-bit immediate field into bits 17-12 of the destination register
    ///
    /// C.LUI loads the non-zero 6-bit immediate field into bits 17-12 of the destination register, clears the bottom 12 bits, and sign-extends bit 17 into all higher bits of the destination.
    /// C.LUI expands into `lui rd, imm`.
    /// C.LUI is only valid when rd&ne;x0 and rd&ne;x2, and when the immediate is not equal to zero.
    /// The code points with imm=0 are reserved; the remaining code points with rd=x0 are HINTs; and the remaining code points with rd=x2 correspond to the C.ADDI16SP instruction
    ///
    /// # Forms
    /// Assembly: `c.lui xd, imm`
    CLUI,
    /// Load word
    ///
    /// Loads a 32-bit value from memory into register rd.
    /// It computes an effective address by adding the zero-extended offset, scaled by 4,
    /// to the base address in register rs1.
    /// It expands to `lw` `rd, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.lw xd, imm(xs1)`
    CLW,
    /// Load word from stack pointer
    ///
    /// Loads a 32-bit value from memory into register rd.
    /// It computes an effective address by adding the zero-extended offset, scaled by 4,
    /// to the stack pointer, x2.
    /// It expands to `lw` `rd, offset(x2)`.
    /// C.LWSP is only valid when rd &ne; x0. The code points with rd=x0 are reserved.
    ///
    /// # Forms
    /// Assembly: `c.lwsp xd, imm(sp)`
    CLWSP,
    /// RISC-V `c.mop.1` instruction.
    ///
    /// # Forms
    /// Assembly: `c.mop.1`
    CMOP1,
    /// RISC-V `c.mop.11` instruction.
    ///
    /// # Forms
    /// Assembly: `c.mop.11`
    CMOP11,
    /// RISC-V `c.mop.13` instruction.
    ///
    /// # Forms
    /// Assembly: `c.mop.13`
    CMOP13,
    /// RISC-V `c.mop.15` instruction.
    ///
    /// # Forms
    /// Assembly: `c.mop.15`
    CMOP15,
    /// RISC-V `c.mop.3` instruction.
    ///
    /// # Forms
    /// Assembly: `c.mop.3`
    CMOP3,
    /// RISC-V `c.mop.5` instruction.
    ///
    /// # Forms
    /// Assembly: `c.mop.5`
    CMOP5,
    /// RISC-V `c.mop.7` instruction.
    ///
    /// # Forms
    /// Assembly: `c.mop.7`
    CMOP7,
    /// RISC-V `c.mop.9` instruction.
    ///
    /// # Forms
    /// Assembly: `c.mop.9`
    CMOP9,
    /// RISC-V `c.mop.n` instruction.
    ///
    /// # Forms
    /// Assembly: `c.mop.n c_mop_t`
    CMOPN,
    /// Multiply, 16-bit encoding
    ///
    /// This instruction multiplies XLEN bits of the source operands from rsd' and rs2' and writes the lowest XLEN bits of the result to rsd'.
    ///
    /// # Forms
    /// Assembly: `c.mul xd, xs2`
    CMUL,
    /// Move Register
    ///
    /// C.MV (move register) performs copy of the data in register rs2 to register rd
    /// C.MV expands to addi rd, x0, rs2.
    ///
    /// # Forms
    /// Assembly: `c.mv xd, xs2`
    CMV,
    /// Non-operation
    ///
    /// C.NOP expands into `addi x0, x0, imm`.
    ///
    /// # Forms
    /// Assembly: `c.nop imm`
    CNOP,
    /// Bitwise not, 16-bit encoding
    ///
    /// This instruction takes a single source/destination operand.
    /// This instruction takes the one's complement of rd'/rs1' and writes the result to the same register.
    ///
    /// # Forms
    /// Assembly: `c.not xd`
    CNOT,
    /// RISC-V `c.ntl.all` instruction.
    ///
    /// # Forms
    /// Assembly: `c.ntl.all`
    CNTLALL,
    /// RISC-V `c.ntl.p1` instruction.
    ///
    /// # Forms
    /// Assembly: `c.ntl.p1`
    CNTLP1,
    /// RISC-V `c.ntl.pall` instruction.
    ///
    /// # Forms
    /// Assembly: `c.ntl.pall`
    CNTLPALL,
    /// RISC-V `c.ntl.s1` instruction.
    ///
    /// # Forms
    /// Assembly: `c.ntl.s1`
    CNTLS1,
    /// Or
    ///
    /// Or rd with rs2, and store the result in rd
    /// The rd and rs2 register indexes should be used as rd+8 and rs2+8 (registers x8-x15).
    /// C.OR expands into `or rd, rd, rs2`.
    ///
    /// # Forms
    /// Assembly: `c.or xd, rs2`
    COR,
    /// Store unsigned byte, 16-bit encoding
    ///
    /// Stores a 8-bit value from register rs2 into memory.
    /// It computes an effective address by adding the zero-extended offset, to the base address in register rs1.
    /// It expands to `sb` `rs2, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.sb xs2, imm(xs1)`
    CSB,
    /// Store double
    ///
    /// Stores a 64-bit value in register rs2 to memory.
    /// It computes an effective address by adding the zero-extended offset, scaled by 8,
    /// to the base address in register rs1.
    /// It expands to `sd` `rs2, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.sd xs2, imm(xs1)`
    CSD,
    /// Store doubleword to stack
    ///
    /// Stores a 64-bit value in register rs2 to memory.
    /// It computes an effective address by adding the zero-extended offset, scaled by 8,
    /// to the stack pointer, x2.
    /// It expands to `sd` `rs2, offset(x2)`.
    ///
    /// # Forms
    /// Assembly: `c.sdsp xs2, imm(sp)`
    CSDSP,
    /// Sign-extend byte, 16-bit encoding
    ///
    /// This instruction takes a single source/destination operand.
    /// This instruction sign-extends the least-significant byte of the source to XLEN by copying
    /// the most-significant bit in the byte (i.e., bit 7) to all of the more-significant bits.
    ///
    /// # Forms
    /// Assembly: `c.sext.b xd`
    CSEXTB,
    /// Sign-extend halfword, 16-bit encoding
    ///
    /// This instruction takes a single source/destination operand.
    /// This instruction sign-extends the least-significant halfword of the source to XLEN by copying
    /// the most-significant bit in the halfword (i.e., bit 15) to all of the more-significant bits.
    ///
    /// # Forms
    /// Assembly: `c.sext.h xd`
    CSEXTH,
    /// RISC-V `c.sext.w` instruction.
    ///
    /// # Forms
    /// Assembly: `c.sext.w rd_rs1_n0`
    CSEXTW,
    /// Store unsigned halfword, 16-bit encoding
    ///
    /// Stores a 16-bit value from register rs2 into memory.
    /// It computes an effective address by adding the zero-extended offset, to the base address in register rs1.
    /// It expands to `sh` `rs2, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.sh xs2, imm(xs1)`
    CSH,
    /// Shift left logical immediate
    ///
    /// Shift the value in rd left by shamt, and store the result back in rd.
    /// C.SLLI expands into `slli rd, rd, shamt`.
    ///
    /// # Forms
    /// Assembly: `c.slli xd, shamt`
    CSLLI,
    /// Shift left logical immediate
    ///
    /// Shift the value in rd left by shamt, and store the result back in rd.
    /// C.SLLI expands into `slli rd, rd, shamt`.
    ///
    /// # Forms
    /// Assembly: `c.slli.rv32 xd, shamt`
    CSLLIRV32,
    /// Shift right arithmetical immediate
    ///
    /// Arithmetic shift (the original sign bit is copied into the vacated upper bits) the value in rd right by shamt, and store the result in rd.
    /// The rd register index should be used as rd+8 (registers x8-x15).
    /// C.SRAI expands into `srai rd, rd, shamt`.
    ///
    /// # Forms
    /// Assembly: `c.srai xd, shamt`
    CSRAI,
    /// Shift right arithmetical immediate
    ///
    /// Arithmetic shift (the original sign bit is copied into the vacated upper bits) the value in rd right by shamt, and store the result in rd.
    /// The rd register index should be used as rd+8 (registers x8-x15).
    /// C.SRAI expands into `srai rd, rd, shamt`.
    ///
    /// # Forms
    /// Assembly: `c.srai.rv32 xd, shamt`
    CSRAIRV32,
    /// Shift right logical immediate
    ///
    /// Shift the value in rd right by shamt, and store the result back in rd.
    /// The rd register index should be used as rd+8 (registers x8-x15).
    /// C.SRLI expands into `srli rd, rd, shamt`.
    ///
    /// # Forms
    /// Assembly: `c.srli xd, shamt`
    CSRLI,
    /// Shift right logical immediate
    ///
    /// Shift the value in rd right by shamt, and store the result back in rd.
    /// The rd register index should be used as rd+8 (registers x8-x15).
    /// C.SRLI expands into `srli rd, rd, shamt`.
    ///
    /// # Forms
    /// Assembly: `c.srli.rv32 xd, shamt`
    CSRLIRV32,
    /// RISC-V `c.sspopchk.x5` instruction.
    ///
    /// # Forms
    /// Assembly: `c.sspopchk.x5`
    CSSPOPCHKX5,
    /// RISC-V `c.sspush.x1` instruction.
    ///
    /// # Forms
    /// Assembly: `c.sspush.x1`
    CSSPUSHX1,
    /// Subtract
    ///
    /// Subtract the value in rs2 from rd, and store the result in rd.
    /// The rd and rs2 register indexes should be used as rd+8 and rs2+8 (registers x8-x15).
    /// C.SUB expands into `sub rd, rd, rs2`.
    ///
    /// # Forms
    /// Assembly: `c.sub xd, rs2`
    CSUB,
    /// Subtract word
    ///
    /// Subtract the 32-bit values in rs2 from rd, and store the result in rd.
    /// The rd and rs2 register indexes should be used as rd+8 and rs2+8 (registers x8-x15).
    /// C.SUBW expands into `subw rd, rd, rs2`.
    ///
    /// # Forms
    /// Assembly: `c.subw xd, rs2`
    CSUBW,
    /// Store word
    ///
    /// Stores a 32-bit value in register rs2 to memory.
    /// It computes an effective address by adding the zero-extended offset, scaled by 4,
    /// to the base address in register rs1.
    /// It expands to `sw` `rs2, offset(rs1)`.
    ///
    /// # Forms
    /// Assembly: `c.sw xs2, imm(xs1)`
    CSW,
    /// Store word to stack
    ///
    /// Stores a 32-bit value in register rs2 to memory.
    /// It computes an effective address by adding the zero-extended offset, scaled by 4,
    /// to the stack pointer, x2.
    /// It expands to `sw` `rs2, offset(x2)`.
    ///
    /// # Forms
    /// Assembly: `c.swsp xs2, imm(sp)`
    CSWSP,
    /// Exclusive Or
    ///
    /// Exclusive or rd with rs2, and store the result in rd
    /// The rd and rs2 register indexes should be used as rd+8 and rs2+8 (registers x8-x15).
    /// C.XOR expands into `xor rd, rd, rs2`.
    ///
    /// # Forms
    /// Assembly: `c.xor xd, rs2`
    CXOR,
    /// Zero-extend byte, 16-bit encoding
    ///
    /// This instruction takes a single source/destination operand.
    /// This instruction zero-extends the least-significant byte of the source to XLEN by inserting
    /// 0's into all of the bits more significant than 7.
    ///
    /// # Forms
    /// Assembly: `c.zext.b xd`
    CZEXTB,
    /// Zero-extend halfword, 16-bit encoding
    ///
    /// This instruction takes a single source/destination operand.
    /// This instruction zero-extends the least-significant halfword of the source to XLEN by inserting
    /// 0's into all of the bits more significant than 15.
    ///
    /// # Forms
    /// Assembly: `c.zext.h xd`
    CZEXTH,
    /// Zero-extend word, 16-bit encoding
    ///
    /// This instruction takes a single source/destination operand.
    /// It zero-extends the least-significant word of the operand to XLEN bits by inserting zeros into all of the bits more significant than 31.
    ///
    /// # Forms
    /// Assembly: `c.zext.w xd`
    CZEXTW,
    /// Cache Block Clean
    ///
    /// Cleans an entire cache block globally throughout the system.
    ///
    /// Exactly what happens is coherence protocol-dependent, but in general it is expected that after this
    /// operation():
    ///
    ///   * The cache block will be in the clean (not dirty) state in any coherent cache holding a valid copy of the line.
    ///   * The data will be cleaned to a point such that an incoherent load can observe the cleaned data.
    ///
    /// `cbo.clean` is ordered by `FENCE` instructions but not `FENCE.I` or `SFENCE.VMA`.
    ///
    /// &lt;%- if CACHE_BLOCK_SIZE.bit_length &gt; \[PMP_GRANULARITY, PMA_GRANULARITY\].min -%&gt;
    /// Both PMP and PMA access control must be the same for all bytes in the block; otherwise, `cbo.clean` has UNSPECIFIED behavior.
    /// &lt;%- end -%&gt;
    ///
    /// Clean operations are treated as stores for page and access permissions. If permission checks fail,
    /// one of the following exceptions will occur:
    ///
    ///   &lt;%- if ext?(:H) -%&gt;
    ///   * `Store/AMO Guest-Page Fault` if virtual memory translation fails during G-stage translation.
    ///   &lt;%- end -%&gt;
    ///   * `Store/AMO Page Fault` if virtual memory translation fails &lt;% if ext?(:H) %&gt;when V=0 or during VS-stage translation&lt;% end %&gt;
    ///   * `Store/AMO Access Fault` if a PMP or PMA access check fails
    ///
    /// &lt;%- if CACHE_BLOCK_SIZE.bit_length &lt;= \[PMP_GRANULARITY, PMA_GRANULARITY\].min -%&gt;
    /// Because cache blocks are naturally aligned and always fit in a single PMP or PMA regions, the PMP
    /// and PMA access checks only need to check a single address in the line.
    /// &lt;%- end -%&gt;
    ///
    /// CBO operations never raise a misaligned address fault.
    ///
    /// # Forms
    /// Assembly: `cbo.clean "TODO"`
    CBOCLEAN,
    /// Cache Block Flush
    ///
    /// Flushes an entire cache block by cleaning it and then invalidating it in all caches.
    ///
    /// `cbo.flush` is ordered by `FENCE` instructions but not `FENCE.I` or `SFENCE.VMA`.
    ///
    /// &lt;%- if CACHE_BLOCK_SIZE.bit_length &gt; \[PMP_GRANULARITY, PMA_GRANULARITY\].min -%&gt;
    /// Both PMP and PMA access control must be the same for all bytes in the block; otherwise, `cbo.flush` has UNSPECIFIED behavior.
    /// &lt;%- end -%&gt;
    ///
    /// Flush operations are treated as stores for page and access permissions. If permission checks fail,
    /// one of the following exceptions will occur:
    ///
    ///   &lt;%- if ext?(:H) -%&gt;
    ///   * `Store/AMO Guest-Page Fault` if virtual memory translation fails during G-stage translation.
    ///   &lt;%- end -%&gt;
    ///   * `Store/AMO Page Fault` if virtual memory translation fails &lt;% if ext?(:H) %&gt;when V=0 or during VS-stage translation&lt;% end %&gt;
    ///   * `Store/AMO Access Fault` if a PMP or PMA access check fails.
    ///
    /// &lt;%- if CACHE_BLOCK_SIZE.bit_length &lt;= \[PMP_GRANULARITY, PMA_GRANULARITY\].min -%&gt;
    /// Because cache blocks are naturally aligned and always fit in a single PMP or PMA regions, the PMP
    /// and PMA access checks only need to check a single address in the line.
    /// &lt;%- end -%&gt;
    ///
    /// CBO operations never raise a misaligned address fault.
    ///
    /// # Forms
    /// Assembly: `cbo.flush "TODO"`
    CBOFLUSH,
    /// Cache Block Invalidate
    ///
    /// Either invalidates or flushes (clean + invalidate) a cache block, depending on the current mode and value of
    /// `menvcfg.CBIE`, `senvcfg.CBIE`, and/or `henvcfg.CBIE`.
    ///
    /// The instruction is an invalidate (without a clean) when:
    ///
    ///   * In M-mode
    ///   * In (H)S-mode and `menvcfg.CBIE` == 11
    ///   * In U-mode and `menvcfg.CBIE` == 11 and `senvcfg.CBIE` == 11
    ///   * In VS-mode and `menvcfg.CBIE` == 11 and `henvcfg.CBIE` == 11
    ///   * In VU-mode and `menvcfg.CBIE` == 11 and `henvcfg.CBIE` == 11 and `senvcfg.CBIE` == 11
    ///
    /// Otherwise, if the instruction does not trap (see Access section), the operation is a flush.
    /// The table below summarizes the options.
    ///
    /// \[%autowidth,cols="1,1,1,1,1,1,1,1",separator="!"\]
    /// !===
    /// .2+h!\[.rotate\]#`menvcfg.CBIE`# .2+h! \[.rotate\]#`senvcfg.CBIE`# .2+h! \[.rotate\]#`henvcfg.CBIE`#
    /// 5+^.&gt;h! `cbe.inval` Operation
    /// .^h! M-mode .^h! S-mode .^h! U-mode .^h! VS-mode .^h! VU-mode
    ///
    /// ! 00 ! - ! - ! Invalidate ! `Illegal Instruction` ! `Illegal Instruction` ! `Virtual Instruction` ! `Virtual Instruction`
    /// ! 01 ! 00 ! 00 ! Invalidate ! Flush  ! `Illegal Instruction` ! `Virtual Instruction` ! `Virtual Instruction`
    /// ! 01 ! 00 ! 01 ! Invalidate ! Flush  ! `Illegal Instruction` ! Flush ! `Virtual Instruction`
    /// ! 01 ! 00 ! 11 ! Invalidate ! Flush  ! `Illegal Instruction` ! Flush ! `Virtual Instruction`
    /// ! 01 ! 01 ! 00 ! Invalidate ! Flush  ! Flush ! `Virtual Instruction` ! `Virtual Instruction`
    /// ! 01 ! 01 ! 01 ! Invalidate ! Flush  ! Flush ! Flush ! Flush
    /// ! 01 ! 01 ! 11 ! Invalidate ! Flush  ! Flush ! Flush ! Flush
    /// ! 01 ! 11 ! 00 ! Invalidate ! Flush  ! Flush ! `Virtual Instruction` ! `Virtual Instruction`
    /// ! 01 ! 11 ! 01 ! Invalidate ! Flush  ! Flush ! Flush ! Flush
    /// ! 01 ! 11 ! 11 ! Invalidate ! Flush  ! Flush ! Flush ! Flush
    /// ! 11 ! 00 ! 00  ! Invalidate ! Invalidate  ! `Illegal Instruction` ! `Virtual Instruction` ! `Virtual Instruction`
    /// ! 11 ! 00 ! 01  ! Invalidate ! Invalidate  ! `Illegal Instruction` ! Flush ! `Virtual Instruction`
    /// ! 11 ! 00 ! 11  ! Invalidate ! Invalidate  ! `Illegal Instruction` ! Invalidate ! `Virtual Instruction`
    /// ! 11 ! 01 ! 00 ! Invalidate ! Invalidate  ! Flush ! `Virtual Instruction` ! `Virtual Instruction`
    /// ! 11 ! 01 ! 01 ! Invalidate ! Invalidate  ! Flush ! Flush ! Flush
    /// ! 11 ! 01 ! 11 ! Invalidate ! Invalidate  ! Flush ! Invalidate ! Flush
    /// ! 11 ! 11 ! 00 ! Invalidate ! Invalidate  ! Invalidate ! `Virtual Instruction` ! `Virtual Instruction`
    /// ! 11 ! 11 ! 01 ! Invalidate ! Invalidate  ! Invalidate ! Flush ! Flush
    /// ! 11 ! 11 ! 11 ! Invalidate ! Invalidate  ! Invalidate ! Invalidate ! Invalidate
    /// !===
    ///
    /// `cbo.inval` is ordered by `FENCE` instructions but not `FENCE.I` or `SFENCE.VMA`.
    ///
    /// &lt;%- if CACHE_BLOCK_SIZE.bit_length &gt; \[PMP_GRANULARITY, PMA_GRANULARITY\].min -%&gt;
    /// Both PMP and PMA access control must be the same for all bytes in the block; otherwise, `cbo.zero` has UNSPECIFIED behavior.
    /// &lt;%- end -%&gt;
    ///
    /// Invalidate operations are treated as stores for page and access permissions. If permission checks fail,
    /// one of the following exceptions will occur:
    ///
    ///   &lt;%- if ext?(:H) -%&gt;
    ///   * `Store/AMO Guest-Page Fault` if virtual memory translation fails during G-stage translation.
    ///   &lt;%- end -%&gt;
    ///   * `Store/AMO Page Fault` if virtual memory translation fails &lt;% if ext?(:H) %&gt;when V=0 or during VS-stage translation&lt;% end %&gt;
    ///   * `Store/AMO Access Fault` if a PMP or PMA access check fails.
    ///
    /// &lt;%- if CACHE_BLOCK_SIZE.bit_length &lt;= \[PMP_GRANULARITY, PMA_GRANULARITY\].min -%&gt;
    /// Because cache blocks are naturally aligned and always fit in a single PMP or PMA regions, the PMP
    /// and PMA access checks only need to check a single address in the line.
    /// &lt;%- end -%&gt;
    ///
    /// CBO operations never raise a misaligned address fault.
    ///
    /// # Forms
    /// Assembly: `cbo.inval "TODO"`
    CBOINVAL,
    /// Cache Block Zero
    ///
    /// Zeros an entire cache block
    ///
    /// The block zeroing does not need to be atomic.
    ///
    /// `cbo.zero` is ordered by `FENCE` instructions but not `FENCE.I` or `SFENCE.VMA`.
    ///
    /// &lt;%- if CACHE_BLOCK_SIZE.bit_length &gt; \[PMP_GRANULARITY, PMA_GRANULARITY\].min -%&gt;
    /// Both PMP and PMA access control must be the same for all bytes in the block; otherwise, `cbo.zero` has UNSPECIFIED behavior.
    /// &lt;%- end -%&gt;
    ///
    /// Clean operations are treated as stores for page and access permissions. If permission checks fail,
    /// one of the following exceptions will occur:
    ///
    ///   &lt;%- if ext?(:H) -%&gt;
    ///   * `Store/AMO Guest-Page Fault` if virtual memory translation fails during G-stage translation.
    ///   &lt;%- end -%&gt;
    ///   * `Store/AMO Page Fault` if virtual memory translation fails &lt;% if ext?(:H) %&gt;when V=0 or during VS-stage translation&lt;% end %&gt;
    ///   * `Store/AMO Access Fault` if a PMP or PMA access check fails.
    ///
    /// &lt;%- if CACHE_BLOCK_SIZE.bit_length &lt;= \[PMP_GRANULARITY, PMA_GRANULARITY\].min -%&gt;
    /// Because cache blocks are naturally aligned and always fit in a single PMP or PMA regions, the PMP
    /// and PMA access checks only need to check a single address in the line.
    /// &lt;%- end -%&gt;
    ///
    /// CBO operations never raise a misaligned address fault.
    ///
    /// # Forms
    /// Assembly: `cbo.zero "TODO"`
    CBOZERO,
    /// Carry-less multiply (low-part)
    ///
    /// `clmul` produces the lower half of the 2*XLEN carry-less product
    ///
    /// # Forms
    /// Assembly: `clmul xd, xs1, xs2`
    CLMUL,
    /// Carry-less multiply (high-part)
    ///
    /// `clmulh` produces the upper half of the 2*XLEN carry-less product
    ///
    /// # Forms
    /// Assembly: `clmulh xd, xs1, xs2`
    CLMULH,
    /// Carry-less multiply (reversed)
    ///
    /// `clmulr` produces bits 2*XLEN-2:XLEN-1 of the 2*XLEN carry-less product
    ///
    /// # Forms
    /// Assembly: `clmulr xd, xs1, xs2`
    CLMULR,
    /// Count leading zero bits
    ///
    /// This instruction counts the number of 0's before the first 1,
    /// starting at the most-significant bit (i.e., XLEN-1) and progressing to bit 0.
    /// Accordingly, if the input is 0, the output is XLEN, and if the most-significant
    /// bit of the input is a 1, the output is 0.
    ///
    /// # Forms
    /// Assembly: `clz xd, xs1`
    CLZ,
    /// Count leading zero bits in word
    ///
    /// This instruction counts the number of 0's before the first 1 starting at bit 31 and progressing to bit 0.
    /// Accordingly, if the least-significant word is 0, the output is 32, and if the most-significant
    /// bit of the word (_i.e._, bit 31) is a 1, the output is 0.
    ///
    /// # Forms
    /// Assembly: `clzw xd, xs1`
    CLZW,
    /// RISC-V `cm.jalt` instruction.
    ///
    /// # Forms
    /// Assembly: `cm.jalt c_index`
    CMJALT,
    /// Move two s0-s7 registers into a0-a1
    ///
    /// This instruction moves r1s' into a0 and r2s' into a1. The execution is atomic, so it is not possible to observe state where only one of a0 or a1 have been updated.
    /// The encoding uses sreg number specifiers instead of xreg number specifiers to save encoding space. The mapping between them is specified in the pseudo-code below.
    ///
    /// # Forms
    /// Assembly: `cm.mva01s r1s, r2s`
    CMMVA01S,
    /// Move a0-a1 into two registers of s0-s7
    ///
    /// This instruction moves a0 into r1s' and a1 into r2s'. r1s' and r2s' must be different.
    /// The execution is atomic, so it is not possible to observe state where only one of r1s' or r2s' has been updated.
    /// The encoding uses sreg number specifiers instead of xreg number specifiers to save encoding space.
    /// The mapping between them is specified in the pseudo-code below.
    ///
    /// # Forms
    /// Assembly: `cm.mvsa01 r1s, r2s`
    CMMVSA01,
    /// Destroy function call stack frame
    ///
    /// Destroy stack frame: load `ra` and 0 to 12 saved registers from the stack frame, deallocate the stack frame.
    /// This instruction pops (loads) the registers in `reg_list` from stack memory, and then adjusts the stack pointer by `stack_adj`.
    ///
    /// Restrictions on stack_adj:
    ///
    /// * it must be enough to store all of the listed registers
    /// * it must be a multiple of 16 (bytes):
    /// ** for RV32 the allowed values are: 16, 32, 48, 64, 80, 96, 112
    /// ** for RV64 the allowed values are: 16, 32, 48, 64, 80, 96, 112, 128, 144, 160
    ///
    /// # Forms
    /// Assembly: `cm.pop reg_list, stack_adj`
    CMPOP,
    /// Destroy function call stack frame and return to `ra`.
    ///
    /// Destroy stack frame: load `ra` and 0 to 12 saved registers from the stack frame, deallocate the stack frame, return to `ra`.
    /// This instruction pops (loads) the registers in `reg_list` from stack memory, and then adjusts the stack pointer by `stack_adj` and then return to `ra`.
    ///
    /// Restrictions on stack_adj:
    ///
    /// * it must be enough to store all of the listed registers
    /// * it must be a multiple of 16 (bytes):
    /// ** for RV32 the allowed values are: 16, 32, 48, 64, 80, 96, 112
    /// ** for RV64 the allowed values are: 16, 32, 48, 64, 80, 96, 112, 128, 144, 160
    ///
    /// # Forms
    /// Assembly: `cm.popret reg_list, stack_adj`
    CMPOPRET,
    /// Destroy function call stack frame, move zero to `a0` and return to `ra`.
    ///
    /// Destroy stack frame: load `ra` and 0 to 12 saved registers from the stack frame, deallocate the stack frame, move zero to `a0`, return to `ra`.
    /// This instruction pops (loads) the registers in `reg_list` from stack memory, and then adjusts the stack pointer by `stack_adj`, move zero to `a0` and then return to `ra`.
    ///
    /// Restrictions on stack_adj:
    ///
    /// * it must be enough to store all of the listed registers
    /// * it must be a multiple of 16 (bytes):
    /// ** for RV32 the allowed values are: 16, 32, 48, 64, 80, 96, 112
    /// ** for RV64 the allowed values are: 16, 32, 48, 64, 80, 96, 112, 128, 144, 160
    ///
    /// # Forms
    /// Assembly: `cm.popretz reg_list, stack_adj`
    CMPOPRETZ,
    /// Create function call stack frame
    ///
    /// Create stack frame: store `ra` and 0 to 12 saved registers to the stack frame, optionally allocate additional stack space.
    /// This instruction pushes (stores) the registers in `reg_list` to the memory below the stack pointer,
    /// and then creates the stack frame by decrementing the stack pointer by `stack_adj`.
    ///
    /// Restrictions on stack_adj:
    ///
    /// * it must be enough to store all of the listed registers
    /// * it must be a multiple of 16 (bytes):
    /// ** for RV32 the allowed values are: 16, 32, 48, 64, 80, 96, 112
    /// ** for RV64 the allowed values are: 16, 32, 48, 64, 80, 96, 112, 128, 144, 160
    ///
    /// # Forms
    /// Assembly: `cm.push reg_list, -stack_adj`
    CMPUSH,
    /// Count set bits
    ///
    /// This instructions counts the number of 1's (i.e., set bits) in the source register.
    ///
    /// .Software Hint
    /// \[NOTE\]
    /// ----
    /// This operations is known as population count, popcount, sideways sum,
    /// bit summation, or Hamming weight.
    ///
    /// The GCC builtin function `__builtin_popcount (unsigned int x)` is
    /// implemented by cpop on RV32 and by cpopw on RV64. The GCC builtin
    /// function `__builtin_popcountl (unsigned long x)` for LP64 is
    /// implemented by cpop on RV64.
    /// ----
    ///
    /// # Forms
    /// Assembly: `cpop xd, xs1`
    CPOP,
    /// Count set bits in word
    ///
    /// This instructions counts the number of 1's (i.e., set bits) in the least-significant word of the source register.
    ///
    /// .Software Hint
    /// \[NOTE\]
    /// ----
    /// This operations is known as population count, popcount, sideways sum,
    /// bit summation, or Hamming weight.
    ///
    /// The GCC builtin function `__builtin_popcount (unsigned int x)` is
    /// implemented by cpop on RV32 and by cpopw on RV64. The GCC builtin
    /// function `__builtin_popcountl (unsigned long x)` for LP64 is
    /// implemented by cpop on RV64.
    /// ----
    ///
    /// # Forms
    /// Assembly: `cpopw xd, xs1`
    CPOPW,
    /// RISC-V `csrc` instruction.
    ///
    /// # Forms
    /// Assembly: `csrc rs1 csr`
    CSRC,
    /// RISC-V `csrci` instruction.
    ///
    /// # Forms
    /// Assembly: `csrci csr zimm5`
    CSRCI,
    /// RISC-V `csrr` instruction.
    ///
    /// # Forms
    /// Assembly: `csrr rd csr`
    CSRR,
    /// RISC-V `csrrc` instruction.
    ///
    /// # Forms
    /// Assembly: `csrrc xd, xs1, csr`
    CSRRC,
    /// RISC-V `csrrci` instruction.
    ///
    /// # Forms
    /// Assembly: `csrrci xd, csr, imm`
    CSRRCI,
    /// Atomic Read and Set Bits in CSR
    ///
    /// Atomically read and set bits in a CSR.
    ///
    /// Reads the value of the CSR, zero-extends the value to `XLEN` bits,
    /// and writes it to integer register `rd`. The initial value in integer
    /// register `rs1` is treated as a bit mask that specifies bit positions
    /// to be set in the CSR. Any bit that is high in `rs1` will cause the
    /// corresponding bit to be set in the CSR, if that CSR bit is writable.
    /// Other bits in the CSR are not explicitly written.
    ///
    /// # Forms
    /// Assembly: `csrrs xd, xs1, csr`
    CSRRS,
    /// RISC-V `csrrsi` instruction.
    ///
    /// # Forms
    /// Assembly: `csrrsi xd, csr, imm`
    CSRRSI,
    /// Atomic Read/Write CSR
    ///
    /// Atomically swap values in the CSRs and integer registers.
    ///
    /// Read the old value of the CSR, zero-extends the value to `XLEN` bits,
    /// and then write it to integer register rd.
    /// The initial value in rs1 is written to the CSR.
    /// If `rd=x0`, then the instruction shall not read the CSR and shall not
    /// cause any of the side effects that might occur on a CSR read.
    ///
    /// # Forms
    /// Assembly: `csrrw xd, xs1, csr`
    CSRRW,
    /// Atomic Read/Write CSR Immediate
    ///
    /// Atomically write CSR using a 5-bit immediate, and load the previous value into 'rd'.
    ///
    /// Read the old value of the CSR, zero-extends the value to `XLEN` bits,
    /// and then write it to integer register rd.
    /// The 5-bit uimm field is zero-extended and written to the CSR.
    /// If `rd=x0`, then the instruction shall not read the CSR and shall not
    /// cause any of the side effects that might occur on a CSR read.
    ///
    /// # Forms
    /// Assembly: `csrrwi xd, zimm, csr`
    CSRRWI,
    /// RISC-V `csrs` instruction.
    ///
    /// # Forms
    /// Assembly: `csrs rs1 csr`
    CSRS,
    /// RISC-V `csrsi` instruction.
    ///
    /// # Forms
    /// Assembly: `csrsi csr zimm5`
    CSRSI,
    /// RISC-V `csrw` instruction.
    ///
    /// # Forms
    /// Assembly: `csrw rs1 csr`
    CSRW,
    /// RISC-V `csrwi` instruction.
    ///
    /// # Forms
    /// Assembly: `csrwi csr zimm5`
    CSRWI,
    /// Count trailing zero bits
    ///
    /// This instruction counts the number of 0's before the first 1,
    /// starting at the least-significant bit (i.e., 0) and progressing
    /// to the most-significant bit (i.e., XLEN-1). Accordingly, if the
    /// input is 0, the output is XLEN, and if the least-significant bit
    /// of the input is a 1, the output is 0.
    ///
    /// # Forms
    /// Assembly: `ctz xd, xs1`
    CTZ,
    /// Count trailing zero bits in word
    ///
    /// This instruction counts the number of 0's before the first 1,
    /// starting at the least-significant bit (i.e., 0) and progressing
    /// to the most-significant bit of the least-significant word (i.e., 31). Accordingly, if the
    /// least-significant word is 0, the output is 32, and if the least-significant bit
    /// of the input is a 1, the output is 0.
    ///
    /// # Forms
    /// Assembly: `ctzw xd, xs1`
    CTZW,
    /// RISC-V `czero.eqz` instruction.
    ///
    /// # Forms
    /// Assembly: `czero.eqz xd, xs1, xs2`
    CZEROEQZ,
    /// RISC-V `czero.nez` instruction.
    ///
    /// # Forms
    /// Assembly: `czero.nez xd, xs1, xs2`
    CZERONEZ,
    /// Signed division
    ///
    /// Divide rs1 by rs2, and store the result in rd. The remainder is discarded.
    ///
    /// Division by zero will put -1 into rd.
    ///
    /// Division resulting in signed overflow (when most negative number is divided by -1)
    /// will put the most negative number into rd;
    ///
    /// # Forms
    /// Assembly: `div xd, xs1, xs2`
    DIV,
    /// Unsigned division
    ///
    /// Divide unsigned values in rs1 by rs2, and store the result in rd.
    ///
    /// The remainder is discarded.
    ///
    /// If the value in rs2 is zero, rd gets the largest unsigned value.
    ///
    /// # Forms
    /// Assembly: `divu xd, xs1, xs2`
    DIVU,
    /// Unsigned 32-bit division
    ///
    /// Divide the unsigned 32-bit values in rs1 and rs2, and store the sign-extended result in rd.
    ///
    /// The remainder is discarded.
    ///
    /// If the value in rs2 is zero, rd is written with all 1s.
    ///
    /// # Forms
    /// Assembly: `divuw xd, xs1, xs2`
    DIVUW,
    /// Signed 32-bit division
    ///
    /// Divide the lower 32-bits of register rs1 by the lower 32-bits of register rs2,
    /// and store the sign-extended result in rd.
    ///
    /// The remainder is discarded.
    ///
    /// Division by zero will put -1 into rd.
    ///
    /// Division resulting in signed overflow (when most negative number is divided by -1)
    /// will put the most negative number into rd;
    ///
    /// # Forms
    /// Assembly: `divw xd, xs1, xs2`
    DIVW,
    /// RISC-V `dret` instruction.
    ///
    /// # Forms
    /// Assembly: `dret dret`
    DRET,
    /// Breakpoint exception
    ///
    /// The EBREAK instruction is used by debuggers to cause control to be transferred back to
    /// a debugging environment. Unless overridden by an external debug environment,
    /// EBREAK raises a breakpoint exception and performs no other operation.
    ///
    /// \[NOTE\]
    /// As described in the `C` Standard Extension for Compressed Instructions, the `c.ebreak`
    /// instruction performs the same operation as the EBREAK instruction.
    ///
    /// EBREAK causes the receiving privilege mode's epc register to be set to the address of
    /// the EBREAK instruction itself, not the address of the following instruction.
    /// As EBREAK causes a synchronous exception, it is not considered to retire,
    /// and should not increment the `minstret` CSR.
    ///
    /// # Forms
    /// Assembly: `ebreak ""`
    EBREAK,
    /// Environment call
    ///
    /// The ECALL instruction is used to make a request to the supporting execution environment.
    /// When executed in U-mode, S-mode, or M-mode, it generates an environment-call-from-U-mode
    /// exception, environment-call-from-S-mode exception, or environment-call-from-M-mode
    /// exception, respectively, and performs no other operation.
    ///
    /// \[NOTE\]
    /// ECALL generates a different exception for each originating privilege mode so that
    /// environment call exceptions can be selectively delegated.
    /// A typical use case for Unix-like operating systems is to delegate to S-mode
    /// the environment-call-from-U-mode exception but not the others.
    ///
    /// ECALL causes the receiving privilege mode's epc register to be set to the address of
    /// the ECALL instruction itself, not the address of the following instruction.
    /// As ECALL causes a synchronous exception, it is not considered to retire,
    /// and should not increment the `minstret` CSR.
    ///
    /// # Forms
    /// Assembly: `ecall ""`
    ECALL,
    /// RISC-V `fabs.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fabs.d rd rs1 rs2_eq_rs1`
    FABSD,
    /// RISC-V `fabs.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fabs.h rd rs1 rs2_eq_rs1`
    FABSH,
    /// RISC-V `fabs.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fabs.q rd rs1 rs2_eq_rs1`
    FABSQ,
    /// RISC-V `fabs.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fabs.s rd rs1 rs2_eq_rs1`
    FABSS,
    /// RISC-V `fadd.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fadd.d xd, xs1, xs2, rm`
    FADDD,
    /// RISC-V `fadd.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fadd.h xd, xs1, xs2, rm`
    FADDH,
    /// RISC-V `fadd.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fadd.q qd, qs1, qs2, rm`
    FADDQ,
    /// Single-precision floating-point addition
    ///
    /// Do the single-precision floating-point addition of fs1 and fs2 and store the result in fd.
    /// rm is the dynamic Rounding Mode.
    ///
    /// # Forms
    /// Assembly: `fadd.s fd, fs1, fs2, rm`
    FADDS,
    /// RISC-V `fclass.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fclass.d xd, xs1`
    FCLASSD,
    /// RISC-V `fclass.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fclass.h xd, xs1`
    FCLASSH,
    /// RISC-V `fclass.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fclass.q xd, qs1`
    FCLASSQ,
    /// Single-precision floating-point classify.
    ///
    /// The `fclass.s` instruction examines the value in floating-point register
    /// _fs1_ and writes to integer register _rd_ a 10-bit mask that indicates
    /// the class of the floating-point number.
    /// The format of the mask is described in the table below.
    /// The corresponding bit in _rd_ will be set if the property is true and
    /// clear otherwise.
    /// All other bits in _rd_ are cleared.
    /// Note that exactly one bit in rd will be set.
    /// `fclass.s` does not set the floating-point exception flags.
    ///
    /// .Format of result of `fclass` instruction.
    /// \[%autowidth,float="center",align="center",cols="^,&lt;",options="header",\]
    /// |===
    /// |_rd_ bit |Meaning
    /// |0 |_rs1_ is latexmath:\[$-\infty$\].
    /// |1 |_rs1_ is a negative normal number.
    /// |2 |_rs1_ is a negative subnormal number.
    /// |3 |_rs1_ is latexmath:\[$-0$\].
    /// |4 |_rs1_ is latexmath:\[$+0$\].
    /// |5 |_rs1_ is a positive subnormal number.
    /// |6 |_rs1_ is a positive normal number.
    /// |7 |_rs1_ is latexmath:\[$+\infty$\].
    /// |8 |_rs1_ is a signaling NaN.
    /// |9 |_rs1_ is a quiet NaN.
    /// |===
    ///
    /// # Forms
    /// Assembly: `fclass.s xd, fs1`
    FCLASSS,
    /// RISC-V `fcvt.bf16.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.bf16.s xd, xs1, rm`
    FCVTBF16S,
    /// RISC-V `fcvt.d.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.d.h xd, xs1, rm`
    FCVTDH,
    /// RISC-V `fcvt.d.l` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.d.l xd, xs1, rm`
    FCVTDL,
    /// RISC-V `fcvt.d.lu` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.d.lu xd, xs1, rm`
    FCVTDLU,
    /// RISC-V `fcvt.d.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.d.q xd, qs1, rm`
    FCVTDQ,
    /// RISC-V `fcvt.d.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.d.s xd, xs1, rm`
    FCVTDS,
    /// RISC-V `fcvt.d.w` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.d.w xd, xs1, rm`
    FCVTDW,
    /// RISC-V `fcvt.d.wu` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.d.wu xd, xs1, rm`
    FCVTDWU,
    /// RISC-V `fcvt.h.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.h.d xd, xs1, rm`
    FCVTHD,
    /// RISC-V `fcvt.h.l` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.h.l xd, xs1, rm`
    FCVTHL,
    /// RISC-V `fcvt.h.lu` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.h.lu xd, xs1, rm`
    FCVTHLU,
    /// RISC-V `fcvt.h.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.h.q xd, qs1, rm`
    FCVTHQ,
    /// Convert half-precision float to a single-precision float
    ///
    /// Converts a half-precision number in floating-point register _fs1_ into a single-precision floating-point number in
    /// floating-point register _fd_.
    ///
    /// `fcvt.h.s` rounds according to the _rm_ field.
    ///
    /// All floating-point conversion instructions set the Inexact exception flag if the rounded
    /// result differs from the operand value and the Invalid exception flag is not set.
    ///
    /// # Forms
    /// Assembly: `fcvt.h.s fd, xs1`
    FCVTHS,
    /// RISC-V `fcvt.h.w` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.h.w xd, xs1, rm`
    FCVTHW,
    /// RISC-V `fcvt.h.wu` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.h.wu xd, xs1, rm`
    FCVTHWU,
    /// RISC-V `fcvt.l.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.l.d xd, xs1, rm`
    FCVTLD,
    /// RISC-V `fcvt.l.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.l.h xd, xs1, rm`
    FCVTLH,
    /// RISC-V `fcvt.l.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.l.q xd, qs1, rm`
    FCVTLQ,
    /// RISC-V `fcvt.l.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.l.s xd, fs1, rm`
    FCVTLS,
    /// RISC-V `fcvt.lu.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.lu.d xd, xs1, rm`
    FCVTLUD,
    /// RISC-V `fcvt.lu.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.lu.h xd, xs1, rm`
    FCVTLUH,
    /// RISC-V `fcvt.lu.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.lu.q qd, hs1, rm`
    FCVTLUQ,
    /// RISC-V `fcvt.lu.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.lu.s xd, fs1, rm`
    FCVTLUS,
    /// RISC-V `fcvt.q.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.q.d dd, fs1, rm`
    FCVTQD,
    /// RISC-V `fcvt.q.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.q.h hd, qs1, rm`
    FCVTQH,
    /// RISC-V `fcvt.q.l` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.q.l qd, xs1, rm`
    FCVTQL,
    /// RISC-V `fcvt.q.lu` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.q.lu qd, xs1, rm`
    FCVTQLU,
    /// RISC-V `fcvt.q.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.q.s qd, fs1, rm`
    FCVTQS,
    /// RISC-V `fcvt.q.w` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.q.w fd, xs1, rm`
    FCVTQW,
    /// RISC-V `fcvt.q.wu` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.q.wu qd, xs1, rm`
    FCVTQWU,
    /// RISC-V `fcvt.s.bf16` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.s.bf16 xd, xs1, rm`
    FCVTSBF16,
    /// RISC-V `fcvt.s.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.s.d xd, xs1, rm`
    FCVTSD,
    /// Convert single-precision float to a half-precision float
    ///
    /// Converts a single-precision number in floating-point register _fs1_ into a half-precision floating-point number in
    /// floating-point register _fd_.
    ///
    /// `fcvt.s.h` will never round, and so the 'rm' field is effectively ignored.
    ///
    /// # Forms
    /// Assembly: `fcvt.s.h fd, xs1`
    FCVTSH,
    /// RISC-V `fcvt.s.l` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.s.l fd, xs1, rm`
    FCVTSL,
    /// RISC-V `fcvt.s.lu` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.s.lu fd, xs1, rm`
    FCVTSLU,
    /// RISC-V `fcvt.s.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.s.q fd, qs1, rm`
    FCVTSQ,
    /// Convert signed 32-bit integer to single-precision float
    ///
    /// Converts a 32-bit signed integer in integer register _rs1_ into a floating-point number in
    /// floating-point register _fd_.
    ///
    /// All floating-point to integer and integer to floating-point conversion instructions round
    /// according to the _rm_ field.
    /// A floating-point register can be initialized to floating-point positive zero using
    /// `fcvt.s.w rd, x0`, which will never set any exception flags.
    ///
    /// All floating-point conversion instructions set the Inexact exception flag if the rounded
    /// result differs from the operand value and the Invalid exception flag is not set.
    ///
    /// # Forms
    /// Assembly: `fcvt.s.w fd, xs1`
    FCVTSW,
    /// RISC-V `fcvt.s.wu` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.s.wu fd, xs1, rm`
    FCVTSWU,
    /// RISC-V `fcvt.w.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.w.d xd, xs1, rm`
    FCVTWD,
    /// RISC-V `fcvt.w.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.w.h xd, xs1, rm`
    FCVTWH,
    /// RISC-V `fcvt.w.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.w.q xd, qs1, rm`
    FCVTWQ,
    /// Convert single-precision float to integer word to signed 32-bit integer.
    ///
    /// Converts a floating-point number in floating-point register _fs1_ to a signed 32-bit integer indicates
    /// integer register _rd_.
    ///
    /// For XLEN &gt;32, `fcvt.w.s` sign-extends the 32-bit result to the destination register width.
    ///
    /// If the rounded result is not representable as a 32-bit signed integer, it is clipped to the
    /// nearest value and the invalid flag is set.
    ///
    /// The range of valid inputs and behavior for invalid inputs are:
    ///
    /// \[separator="!"\]
    /// !===
    /// ! ! Value
    ///
    /// h! Minimum valid input (after rounding) ! `-2^31`
    /// h! Maximum valid input (after rounding) ! `2^31 - 1`
    /// h! Output for out-of-range negative input ! `-2^31`
    /// h! Output for `-&infin;` ! `-2^31`
    /// h! Output for out-of-range positive input ! `2^31 - 1`
    /// h! Output for `+&infin;` for `NaN` ! `2^31 - 1`
    /// !===
    ///
    /// All floating-point to integer and integer to floating-point conversion instructions round
    /// according to the _rm_ field.
    /// A floating-point register can be initialized to floating-point positive zero using
    /// `fcvt.s.w rd, x0`, which will never set any exception flags.
    ///
    /// All floating-point conversion instructions set the Inexact exception flag if the rounded
    /// result differs from the operand value and the Invalid exception flag is not set.
    ///
    /// # Forms
    /// Assembly: `fcvt.w.s xd, fs1`
    FCVTWS,
    /// RISC-V `fcvt.wu.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.wu.d xd, xs1, rm`
    FCVTWUD,
    /// RISC-V `fcvt.wu.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.wu.h xd, xs1, rm`
    FCVTWUH,
    /// RISC-V `fcvt.wu.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.wu.q xd, xs1, rm`
    FCVTWUQ,
    /// RISC-V `fcvt.wu.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvt.wu.s xd, fs1, rm`
    FCVTWUS,
    /// RISC-V `fcvtmod.w.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fcvtmod.w.d xd, xs1`
    FCVTMODWD,
    /// RISC-V `fdiv.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fdiv.d xd, xs1, xs2, rm`
    FDIVD,
    /// RISC-V `fdiv.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fdiv.h xd, xs1, xs2, rm`
    FDIVH,
    /// RISC-V `fdiv.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fdiv.q qd, qs1, qs2, rm`
    FDIVQ,
    /// RISC-V `fdiv.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fdiv.s fd, fs1, fs2, rm`
    FDIVS,
    /// Memory ordering fence
    ///
    /// Orders memory operations.
    ///
    /// The `fence` instruction is used to order device I/O and memory accesses as
    /// viewed by other RISC-V harts and external devices or coprocessors. Any
    /// combination of device input (I), device output (O), memory reads \(R),
    /// and memory writes (W) may be ordered with respect to any combination of
    /// the same. Informally, no other RISC-V hart or external device can
    /// observe any operation in the _successor_ set following a `fence` before
    /// any operation in the _predecessor_ set preceding the `fence`.
    ///
    /// The predecessor and successor fields have the same format to specify operation types:
    ///
    /// \[%autowidth\]
    /// |===
    /// 4+| `pred` 4+| `succ`
    ///
    /// | 27 | 26 |25 | 24 | 23 | 22 | 21| 20
    /// | PI | PO |PR | PW | SI | SO |SR | SW
    /// |===
    ///
    /// \[%autowidth,align="center",cols="^1,^1,&lt;3",options="header"\]
    /// .Fence mode encoding
    /// |===
    /// |_fm_ field |Mnemonic |Meaning
    /// |0000 |_none_ |Normal Fence
    /// |1000 |TSO |With `FENCE RW,RW`: exclude write-to-read ordering; otherwise: _Reserved for future use._
    /// 2+|_other_ |_Reserved for future use._
    /// |===
    ///
    /// When the mode field _fm_ is `0001` and both the predecessor and successor sets are 'RW',
    /// then the instruction acts as a special-case `fence.tso`. `fence.tso` orders all load operations
    /// in its predecessor set before all memory operations in its successor set, and all store operations
    /// in its predecessor set before all store operations in its successor set. This leaves non-AMO store
    /// operations in the 'fence.tso's predecessor set unordered with non-AMO loads in its successor set.
    ///
    /// When mode field _fm_ is not `0001`, or when mode field _fm_ is `0001` but the _pred_ and
    /// _succ_ fields are not both 'RW' (0x3), then the fence acts as a baseline fence (_e.g._, _fm_ is
    /// effectively `0000`). This is unaffected by the FIOM bits, described below (implicit promotion does
    /// not change how `fence.tso` is decoded).
    ///
    /// The `rs1` and `rd` fields are unused and ignored.
    ///
    /// In modes other than M-mode, `fence` is further affected by `menvcfg.FIOM`,
    /// `senvcfg.FIOM`&lt;% if ext?(:H) %&gt;, and/or `henvcfg.FIOM`&lt;% end %&gt;
    /// as follows:
    ///
    /// .Effective PR/PW/SR/SW in (H)S-mode
    /// \[%autowidth,cols=",,,",options="header",separator="!"\]
    /// !===
    /// ! \[.rotate\]#`menvcfg.FIOM`# ! `pred.PI` +
    /// `pred.PO` +
    /// `succ.SI` +
    /// `succ.SO`
    /// ! -&gt; +
    /// -&gt; +
    /// -&gt; +
    /// -&gt;
    /// ! effective `PR` +
    /// effective `PW` +
    /// effective `SR` +
    /// effective `SW`
    ///
    /// ! 0 ! - ! ! from encoding
    /// ! 1 ! 0 ! ! from encoding
    /// ! 1 ! 1 ! ! 1
    /// !===
    ///
    /// .Effective PR/PW/SR/SW in U-mode
    /// \[%autowidth,options="header",separator="!",cols=",,,,"\]
    /// !===
    /// ! \[.rotate\]#`menvcfg.FIOM`# ! \[.rotate\]#`senvcfg.FIOM`# !  `pred.PI` +
    /// `pred.PO` +
    /// `succ.SI` +
    /// `succ.SO`
    /// ! -&gt; +
    /// -&gt; +
    /// -&gt; +
    /// -&gt;
    /// ! effective `PR` +
    /// effective `PW` +
    /// effective `SR` +
    /// effective `SW`
    ///
    /// ! 0 ! 0 ! - ! ! from encoding
    /// ! 0 ! 1 ! 0 ! ! from encoding
    /// ! 0 ! 1 ! 1 ! ! 1
    /// ! 1 ! - ! 0 ! ! from encoding
    /// ! 1 ! - ! 1 ! ! 1
    /// !===
    ///
    /// &lt;%- if ext?(:H) -%&gt;
    /// .Effective PR/PW/SR/SW in VS-mode and VU-mode
    /// \[%autowidth,options="header",separator="!",cols=",,,,"\]
    /// !===
    /// ! \[.rotate\]#`menvcfg.FIOM`# ! \[.rotate\]#`henvcfg.FIOM`# !  `pred.PI` +
    /// `pred.PO` +
    /// `succ.SI` +
    /// `succ.SO`
    /// ! -&gt; +
    /// -&gt; +
    /// -&gt; +
    /// -&gt;
    /// ! effective `PR` +
    /// effective `PW` +
    /// effective `SR` +
    /// effective `SW`
    ///
    /// ! 0 ! 0 ! - ! ! from encoding
    /// ! 0 ! 1 ! 0 ! ! from encoding
    /// ! 0 ! 1 ! 1 ! ! 1
    /// ! 1 ! - ! 0 ! ! from encoding
    /// ! 1 ! - ! 1 ! ! 1
    /// !===
    /// &lt;%- end -%&gt;
    ///
    /// # Forms
    /// Assembly: `fence "TODO"`
    FENCE,
    /// Instruction fence
    ///
    /// The FENCE.I instruction is used to synchronize the instruction and data
    /// streams. RISC-V does not guarantee that stores to instruction memory
    /// will be made visible to instruction fetches on a RISC-V hart until that
    /// hart executes a FENCE.I instruction. A FENCE.I instruction ensures that
    /// a subsequent instruction fetch on a RISC-V hart will see any previous
    /// data stores already visible to the same RISC-V hart. FENCE.I does _not_
    /// ensure that other RISC-V harts' instruction fetches will observe the
    /// local hart's stores in a multiprocessor system. To make a store to
    /// instruction memory visible to all RISC-V harts, the writing hart also
    /// has to execute a data FENCE before requesting that all remote RISC-V
    /// harts execute a FENCE.I.
    ///
    /// The unused fields in the FENCE.I instruction, _imm\[11:0\]_, _rs1_, and
    /// _rd_, are reserved for finer-grain fences in future extensions. For
    /// forward compatibility, base implementations shall ignore these fields,
    /// and standard software shall zero these fields.
    /// (((FENCE.I, finer-grained)))
    /// (((FENCE.I, forward compatibility)))
    ///
    /// \[NOTE\]
    /// ====
    /// Because FENCE.I only orders stores with a hart's own instruction
    /// fetches, application code should only rely upon FENCE.I if the
    /// application thread will not be migrated to a different hart. The EEI can
    /// provide mechanisms for efficient multiprocessor instruction-stream
    /// synchronization.
    /// ====
    ///
    /// # Forms
    /// Assembly: `fence.i ""`
    FENCEI,
    /// RISC-V `fence.tso` instruction.
    ///
    /// # Forms
    /// Assembly: `fence.tso`
    FENCETSO,
    /// RISC-V `feq.d` instruction.
    ///
    /// # Forms
    /// Assembly: `feq.d xd, xs1, xs2`
    FEQD,
    /// RISC-V `feq.h` instruction.
    ///
    /// # Forms
    /// Assembly: `feq.h xd, xs1, xs2`
    FEQH,
    /// RISC-V `feq.q` instruction.
    ///
    /// # Forms
    /// Assembly: `feq.q xd, qs1, qs2`
    FEQQ,
    /// Single-precision floating-point equal
    ///
    /// Writes 1 to _rd_ if _fs1_ and _fs2_ are equal, and 0 otherwise.
    ///
    /// If either operand is NaN, the result is 0 (not equal). If either operand is a signaling NaN, the invalid flag is set.
    ///
    /// Positive zero is considered equal to negative zero.
    ///
    /// # Forms
    /// Assembly: `feq.s xd, fs1, fs2`
    FEQS,
    /// RISC-V `fld` instruction.
    ///
    /// # Forms
    /// Assembly: `fld xd, xs1, imm`
    FLD,
    /// RISC-V `fle.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fle.d xd, xs1, xs2`
    FLED,
    /// RISC-V `fle.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fle.h xd, xs1, xs2`
    FLEH,
    /// RISC-V `fle.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fle.q xd, qs1, qs2`
    FLEQ,
    /// Single-precision floating-point less than or equal
    ///
    /// Writes 1 to _rd_ if _fs1_ is less than or equal to _fs2_, and 0 otherwise.
    ///
    /// If either operand is NaN, the result is 0 (not equal).
    /// If either operand is a NaN (signaling or quiet), the invalid flag is set.
    ///
    /// Positive zero and negative zero are considered equal.
    ///
    /// # Forms
    /// Assembly: `fle.s xd, fs1, fs2`
    FLES,
    /// RISC-V `fleq.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fleq.d xd, xs1, xs2`
    FLEQD,
    /// RISC-V `fleq.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fleq.h xd, xs1, xs2`
    FLEQH,
    /// RISC-V `fleq.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fleq.q xd, qs1, qs2`
    FLEQQ,
    /// RISC-V `fleq.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fleq.s xd, fs1, fs2`
    FLEQS,
    /// Half-precision floating-point load
    ///
    /// The `flh` instruction loads a single-precision floating-point value from memory at address _rs1_ + _imm_ into floating-point register _rd_.
    ///
    /// `flh` does not modify the bits being transferred; in particular, the payloads of non-canonical NaNs are preserved.
    ///
    /// `flh` is only guaranteed to execute atomically if the effective address is naturally aligned.
    ///
    /// # Forms
    /// Assembly: `flh fd, imm(xs1)`
    FLH,
    /// RISC-V `fli.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fli.d xd, xs1`
    FLID,
    /// RISC-V `fli.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fli.h xd, xs1`
    FLIH,
    /// RISC-V `fli.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fli.q fd, qs1`
    FLIQ,
    /// RISC-V `fli.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fli.s fd, fs1`
    FLIS,
    /// RISC-V `flq` instruction.
    ///
    /// # Forms
    /// Assembly: `flq qd, xs1, imm`
    FLQ,
    /// RISC-V `flt.d` instruction.
    ///
    /// # Forms
    /// Assembly: `flt.d xd, xs1, xs2`
    FLTD,
    /// RISC-V `flt.h` instruction.
    ///
    /// # Forms
    /// Assembly: `flt.h xd, xs1, xs2`
    FLTH,
    /// RISC-V `flt.q` instruction.
    ///
    /// # Forms
    /// Assembly: `flt.q xd, qs1, qs2`
    FLTQ,
    /// Single-precision floating-point less than
    ///
    /// Writes 1 to _rd_ if _fs1_ is less than _fs2_, and 0 otherwise.
    ///
    /// If either operand is NaN, the result is 0 (not equal).
    /// If either operand is a NaN (signaling or quiet), the invalid flag is set.
    ///
    /// # Forms
    /// Assembly: `flt.s xd, fs1, fs2`
    FLTS,
    /// RISC-V `fltq.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fltq.d xd, xs1, xs2`
    FLTQD,
    /// RISC-V `fltq.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fltq.h xd, xs1, xs2`
    FLTQH,
    /// RISC-V `fltq.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fltq.q qd, qs1, qs2`
    FLTQQ,
    /// RISC-V `fltq.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fltq.s xd, fs1, fs2`
    FLTQS,
    /// Single-precision floating-point load
    ///
    /// The `flw` instruction loads a single-precision floating-point value from memory at address _rs1_ + _imm_ into floating-point register _fd_.
    ///
    /// `flw` does not modify the bits being transferred; in particular, the payloads of non-canonical NaNs are preserved.
    ///
    /// # Forms
    /// Assembly: `flw fd, xs1, imm`
    FLW,
    /// RISC-V `fmadd.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fmadd.d xd, xs1, xs2, xs3, rm`
    FMADDD,
    /// RISC-V `fmadd.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fmadd.h xd, xs1, xs2, xs3, rm`
    FMADDH,
    /// RISC-V `fmadd.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fmadd.q qd, qs1, qs2, qs3, rm`
    FMADDQ,
    /// RISC-V `fmadd.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fmadd.s fd, fs1, fs2, fs3, rm`
    FMADDS,
    /// RISC-V `fmax.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fmax.d xd, xs1, xs2`
    FMAXD,
    /// RISC-V `fmax.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fmax.h xd, xs1, xs2`
    FMAXH,
    /// RISC-V `fmax.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fmax.q qd, qs1, qs2`
    FMAXQ,
    /// RISC-V `fmax.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fmax.s fd, fs1, fs2`
    FMAXS,
    /// RISC-V `fmaxm.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fmaxm.d xd, xs1, xs2`
    FMAXMD,
    /// RISC-V `fmaxm.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fmaxm.h xd, xs1, xs2`
    FMAXMH,
    /// RISC-V `fmaxm.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fmaxm.q qd, qs1, qs2`
    FMAXMQ,
    /// RISC-V `fmaxm.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fmaxm.s xd, xs1, xs2`
    FMAXMS,
    /// RISC-V `fmin.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fmin.d xd, xs1, xs2`
    FMIND,
    /// RISC-V `fmin.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fmin.h xd, xs1, xs2`
    FMINH,
    /// RISC-V `fmin.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fmin.q xd, xs1, xs2`
    FMINQ,
    /// RISC-V `fmin.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fmin.s xd, xs1, xs2`
    FMINS,
    /// RISC-V `fminm.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fminm.d xd, xs1, xs2`
    FMINMD,
    /// RISC-V `fminm.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fminm.h xd, xs1, xs2`
    FMINMH,
    /// RISC-V `fminm.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fminm.q qd, qs1, qs2`
    FMINMQ,
    /// RISC-V `fminm.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fminm.s fd, fs1, fs2`
    FMINMS,
    /// RISC-V `fmsub.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fmsub.d xd, xs1, xs2, xs3, rm`
    FMSUBD,
    /// RISC-V `fmsub.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fmsub.h xd, xs1, xs2, xs3, rm`
    FMSUBH,
    /// RISC-V `fmsub.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fmsub.q qd, qs1, qs2, qs3, rm`
    FMSUBQ,
    /// RISC-V `fmsub.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fmsub.s fd, fs1, fs2, fs3, rm`
    FMSUBS,
    /// RISC-V `fmul.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fmul.d xd, xs1, xs2, rm`
    FMULD,
    /// RISC-V `fmul.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fmul.h xd, xs1, xs2, rm`
    FMULH,
    /// RISC-V `fmul.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fmul.q qd, qs1, qs2, rm`
    FMULQ,
    /// RISC-V `fmul.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fmul.s fd, fs1, fs2, rm`
    FMULS,
    /// RISC-V `fmv.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fmv.d rd rs1 rs2_eq_rs1`
    FMVD,
    /// RISC-V `fmv.d.x` instruction.
    ///
    /// # Forms
    /// Assembly: `fmv.d.x xd, xs1`
    FMVDX,
    /// RISC-V `fmv.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fmv.h rd rs1 rs2_eq_rs1`
    FMVH,
    /// Half-precision floating-point move from integer
    ///
    /// Moves the half-precision value encoded in IEEE 754-2008 standard encoding
    /// from the lower 16 bits of integer register `rs1` to the floating-point
    /// register `fd`. The bits are not modified in the transfer, and in particular,
    /// the payloads of non-canonical NaNs are preserved.
    ///
    /// # Forms
    /// Assembly: `fmv.h.x fd, xs1`
    FMVHX,
    /// RISC-V `fmv.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fmv.q rd rs1 rs2_eq_rs1`
    FMVQ,
    /// RISC-V `fmv.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fmv.s rd rs1 rs2_eq_rs1`
    FMVS,
    /// RISC-V `fmv.s.x` instruction.
    ///
    /// # Forms
    /// Assembly: `fmv.s.x rd rs1`
    FMVSX,
    /// Single-precision floating-point move from integer
    ///
    /// Moves the single-precision value encoded in IEEE 754-2008 standard encoding
    /// from the lower 32 bits of integer register `rs1` to the floating-point
    /// register `fd`. The bits are not modified in the transfer, and in particular,
    /// the payloads of non-canonical NaNs are preserved.
    ///
    /// # Forms
    /// Assembly: `fmv.w.x fd, xs1`
    FMVWX,
    /// RISC-V `fmv.x.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fmv.x.d xd, xs1`
    FMVXD,
    /// Move half-precision value from floating-point to integer register
    ///
    /// Moves the half-precision value in floating-point register rs1 represented in IEEE 754-2008
    /// encoding to the lower 16 bits of integer register rd.
    ///
    /// The bits are not modified in the transfer, and in particular, the payloads of non-canonical
    /// NaNs are preserved.
    ///
    /// The highest XLEN-16 bits of the destination register are filled with copies of the
    /// floating-point number's sign bit.
    ///
    /// # Forms
    /// Assembly: `fmv.x.h rd, fs1`
    FMVXH,
    /// RISC-V `fmv.x.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fmv.x.s rd rs1`
    FMVXS,
    /// Move single-precision value from floating-point to integer register
    ///
    /// Moves the single-precision value in floating-point register rs1 represented in IEEE 754-2008
    /// encoding to the lower 32 bits of integer register rd.
    /// The bits are not modified in the transfer, and in particular, the payloads of non-canonical
    /// NaNs are preserved.
    /// For RV64, the higher 32 bits of the destination register are filled with copies of the
    /// floating-point number's sign bit.
    ///
    /// # Forms
    /// Assembly: `fmv.x.w xd, fs1`
    FMVXW,
    /// RISC-V `fmvh.x.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fmvh.x.d xd, xs1`
    FMVHXD,
    /// RISC-V `fmvh.x.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fmvh.x.q xd, qs1`
    FMVHXQ,
    /// RISC-V `fmvp.d.x` instruction.
    ///
    /// # Forms
    /// Assembly: `fmvp.d.x xd, xs1, xs2`
    FMVPDX,
    /// RISC-V `fmvp.q.x` instruction.
    ///
    /// # Forms
    /// Assembly: `fmvp.q.x qd, xs1, xs2`
    FMVPQX,
    /// RISC-V `fneg.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fneg.d rd rs1 rs2_eq_rs1`
    FNEGD,
    /// RISC-V `fneg.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fneg.h rd rs1 rs2_eq_rs1`
    FNEGH,
    /// RISC-V `fneg.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fneg.q rd rs1 rs2_eq_rs1`
    FNEGQ,
    /// RISC-V `fneg.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fneg.s rd rs1 rs2_eq_rs1`
    FNEGS,
    /// RISC-V `fnmadd.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fnmadd.d xd, xs1, xs2, xs3, rm`
    FNMADDD,
    /// RISC-V `fnmadd.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fnmadd.h xd, xs1, xs2, xs3, rm`
    FNMADDH,
    /// RISC-V `fnmadd.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fnmadd.q qd, qs1, qs2, qs3, rm`
    FNMADDQ,
    /// RISC-V `fnmadd.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fnmadd.s fd, fs1, fs2, fs3, rm`
    FNMADDS,
    /// RISC-V `fnmsub.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fnmsub.d xd, xs1, xs2, xs3, rm`
    FNMSUBD,
    /// RISC-V `fnmsub.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fnmsub.h xd, xs1, xs2, xs3, rm`
    FNMSUBH,
    /// RISC-V `fnmsub.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fnmsub.q qd, qs1, qs2, qs3, rm`
    FNMSUBQ,
    /// RISC-V `fnmsub.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fnmsub.s xd, xs1, xs2, xs3, rm`
    FNMSUBS,
    /// RISC-V `frcsr` instruction.
    ///
    /// # Forms
    /// Assembly: `frcsr rd`
    FRCSR,
    /// RISC-V `frflags` instruction.
    ///
    /// # Forms
    /// Assembly: `frflags rd`
    FRFLAGS,
    /// RISC-V `fround.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fround.d xd, xs1, rm`
    FROUNDD,
    /// RISC-V `fround.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fround.h xd, xs1, rm`
    FROUNDH,
    /// RISC-V `fround.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fround.q qd, qs1, rm`
    FROUNDQ,
    /// RISC-V `fround.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fround.s fd, xs1, rm`
    FROUNDS,
    /// RISC-V `froundnx.d` instruction.
    ///
    /// # Forms
    /// Assembly: `froundnx.d xd, xs1, rm`
    FROUNDNXD,
    /// RISC-V `froundnx.h` instruction.
    ///
    /// # Forms
    /// Assembly: `froundnx.h xd, xs1, rm`
    FROUNDNXH,
    /// RISC-V `froundnx.q` instruction.
    ///
    /// # Forms
    /// Assembly: `froundnx.q qd, qs1, rm`
    FROUNDNXQ,
    /// RISC-V `froundnx.s` instruction.
    ///
    /// # Forms
    /// Assembly: `froundnx.s fd, rs1, rm`
    FROUNDNXS,
    /// RISC-V `frrm` instruction.
    ///
    /// # Forms
    /// Assembly: `frrm rd`
    FRRM,
    /// RISC-V `fscsr` instruction.
    ///
    /// # Forms
    /// Assembly: `fscsr rd rs1`
    FSCSR,
    /// RISC-V `fsd` instruction.
    ///
    /// # Forms
    /// Assembly: `fsd xs1, xs2, imm`
    FSD,
    /// RISC-V `fsflags` instruction.
    ///
    /// # Forms
    /// Assembly: `fsflags rd rs1`
    FSFLAGS,
    /// RISC-V `fsflagsi` instruction.
    ///
    /// # Forms
    /// Assembly: `fsflagsi rd zimm5`
    FSFLAGSI,
    /// RISC-V `fsgnj.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fsgnj.d xd, xs1, xs2`
    FSGNJD,
    /// RISC-V `fsgnj.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fsgnj.h xd, xs1, xs2`
    FSGNJH,
    /// RISC-V `fsgnj.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fsgnj.q qd, qs1, qs2`
    FSGNJQ,
    /// Single-precision sign inject
    ///
    /// Writes _fd_ with sign bit of _fs2_ and the exponent and mantissa of _fs1_.
    ///
    /// Sign-injection instructions do not set floating-point exception flags, nor do they canonicalize NaNs.
    ///
    /// # Forms
    /// Assembly: `fsgnj.s fd, fs1, fs2`
    FSGNJS,
    /// RISC-V `fsgnjn.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fsgnjn.d xd, xs1, xs2`
    FSGNJND,
    /// RISC-V `fsgnjn.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fsgnjn.h xd, xs1, xs2`
    FSGNJNH,
    /// RISC-V `fsgnjn.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fsgnjn.q qd, qs1, qs2`
    FSGNJNQ,
    /// Single-precision sign inject negate
    ///
    /// Writes _fd_ with the opposite of the sign bit of _fs2_ and the exponent and mantissa of _fs1_.
    ///
    /// Sign-injection instructions do not set floating-point exception flags, nor do they canonicalize NaNs.
    ///
    /// # Forms
    /// Assembly: `fsgnjn.s fd, fs1, fs2`
    FSGNJNS,
    /// RISC-V `fsgnjx.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fsgnjx.d xd, xs1, xs2`
    FSGNJXD,
    /// RISC-V `fsgnjx.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fsgnjx.h xd, xs1, xs2`
    FSGNJXH,
    /// RISC-V `fsgnjx.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fsgnjx.q qd, qs1, qs2`
    FSGNJXQ,
    /// Single-precision sign inject exclusive or
    ///
    /// Writes _fd_ with the xor of the sign bits of _fs2_ and _fs1_ and the exponent and mantissa of _fs1_.
    ///
    /// Sign-injection instructions do not set floating-point exception flags, nor do they canonicalize NaNs.
    ///
    /// # Forms
    /// Assembly: `fsgnjx.s fd, fs1, fs2`
    FSGNJXS,
    /// Half-precision floating-point store
    ///
    /// The `fsh` instruction stores a half-precision floating-point value
    /// from register _rd_ to memory at address _rs1_ + _imm_.
    ///
    /// `fsh` does not modify the bits being transferred; in particular, the payloads of non-canonical NaNs are preserved.
    ///
    /// `fsh` ignores all but the lower 16 bits in _rs2_.
    ///
    /// `fsh` is only guaranteed to execute atomically if the effective address is naturally aligned.
    ///
    /// # Forms
    /// Assembly: `fsh fs2, imm(xs1)`
    FSH,
    /// RISC-V `fsq` instruction.
    ///
    /// # Forms
    /// Assembly: `fsq xs1, qs2, imm`
    FSQ,
    /// RISC-V `fsqrt.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fsqrt.d xd, xs1, rm`
    FSQRTD,
    /// RISC-V `fsqrt.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fsqrt.h xd, xs1, rm`
    FSQRTH,
    /// RISC-V `fsqrt.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fsqrt.q qd, qs1, rm`
    FSQRTQ,
    /// RISC-V `fsqrt.s` instruction.
    ///
    /// # Forms
    /// Assembly: `fsqrt.s fd, fs1, rm`
    FSQRTS,
    /// RISC-V `fsrm` instruction.
    ///
    /// # Forms
    /// Assembly: `fsrm rd rs1`
    FSRM,
    /// RISC-V `fsrmi` instruction.
    ///
    /// # Forms
    /// Assembly: `fsrmi rd zimm5`
    FSRMI,
    /// RISC-V `fsub.d` instruction.
    ///
    /// # Forms
    /// Assembly: `fsub.d xd, xs1, xs2, rm`
    FSUBD,
    /// RISC-V `fsub.h` instruction.
    ///
    /// # Forms
    /// Assembly: `fsub.h xd, xs1, xs2, rm`
    FSUBH,
    /// RISC-V `fsub.q` instruction.
    ///
    /// # Forms
    /// Assembly: `fsub.q qd, qs1, qs2, rm`
    FSUBQ,
    /// Single-precision floating-point subtraction
    ///
    /// Do the single-precision floating-point subtraction of fs2 from fs1 and store the result in fd.
    /// rm is the dynamic Rounding Mode.
    ///
    /// # Forms
    /// Assembly: `fsub.s fd, fs1, fs2, rm`
    FSUBS,
    /// Single-precision floating-point store
    ///
    /// The `fsw` instruction stores a single-precision floating-point value in _fs2_ to memory at address _rs1_ + _imm_.
    ///
    /// `fsw` does not modify the bits being transferred; in particular, the payloads of non-canonical NaNs are preserved.
    ///
    /// # Forms
    /// Assembly: `fsw fs2, xs1, imm`
    FSW,
    /// RISC-V `hfence.gvma` instruction.
    ///
    /// # Forms
    /// Assembly: `hfence.gvma xs1, xs2`
    HFENCEGVMA,
    /// RISC-V `hfence.vvma` instruction.
    ///
    /// # Forms
    /// Assembly: `hfence.vvma xs1, xs2`
    HFENCEVVMA,
    /// Invalidate cached address translations
    ///
    /// `hinval.gvma` has the same semantics as `sinval.vma` except that it combines with
    /// `sfence.w.inval` and `sfence.inval.ir` to replace `hfence.gvma` and uses VMID instead of ASID.
    ///
    /// # Forms
    /// Assembly: `hinval.gvma xs1, xs2`
    HINVALGVMA,
    /// Invalidate cached address translations
    ///
    /// `hinval.vvma` has the same semantics as `sinval.vma` except that it combines with
    /// `sfence.w.inval` and `sfence.inval.ir` to replace `hfence.vvma`.
    ///
    /// # Forms
    /// Assembly: `hinval.vvma xs1, xs2`
    HINVALVVMA,
    /// RISC-V `hlv.b` instruction.
    ///
    /// # Forms
    /// Assembly: `hlv.b xd, xs1`
    HLVB,
    /// RISC-V `hlv.bu` instruction.
    ///
    /// # Forms
    /// Assembly: `hlv.bu xd, xs1`
    HLVBU,
    /// RISC-V `hlv.d` instruction.
    ///
    /// # Forms
    /// Assembly: `hlv.d xd, xs1`
    HLVD,
    /// RISC-V `hlv.h` instruction.
    ///
    /// # Forms
    /// Assembly: `hlv.h xd, xs1`
    HLVH,
    /// RISC-V `hlv.hu` instruction.
    ///
    /// # Forms
    /// Assembly: `hlv.hu xd, xs1`
    HLVHU,
    /// RISC-V `hlv.w` instruction.
    ///
    /// # Forms
    /// Assembly: `hlv.w xd, xs1`
    HLVW,
    /// RISC-V `hlv.wu` instruction.
    ///
    /// # Forms
    /// Assembly: `hlv.wu xd, xs1`
    HLVWU,
    /// RISC-V `hlvx.hu` instruction.
    ///
    /// # Forms
    /// Assembly: `hlvx.hu xd, xs1`
    HLVXHU,
    /// RISC-V `hlvx.wu` instruction.
    ///
    /// # Forms
    /// Assembly: `hlvx.wu xd, xs1`
    HLVXWU,
    /// RISC-V `hsv.b` instruction.
    ///
    /// # Forms
    /// Assembly: `hsv.b xs1, xs2`
    HSVB,
    /// RISC-V `hsv.d` instruction.
    ///
    /// # Forms
    /// Assembly: `hsv.d xs1, xs2`
    HSVD,
    /// RISC-V `hsv.h` instruction.
    ///
    /// # Forms
    /// Assembly: `hsv.h xs1, xs2`
    HSVH,
    /// RISC-V `hsv.w` instruction.
    ///
    /// # Forms
    /// Assembly: `hsv.w xs1, xs2`
    HSVW,
    /// RISC-V `j` instruction.
    ///
    /// # Forms
    /// Assembly: `j jimm20`
    J,
    /// Jump and link
    ///
    /// Jump to a PC-relative offset and store the return
    /// address in rd.
    ///
    /// # Forms
    /// Assembly: `jal xd, imm`
    JAL,
    /// RISC-V `jal.pseudo` instruction.
    ///
    /// # Forms
    /// Assembly: `jal.pseudo jimm20`
    JALPSEUDO,
    /// Jump and link register
    ///
    /// Jump to an address formed by adding rs1
    /// to a signed offset then clearing the least
    /// significant bit, and store the return address
    /// in rd.
    ///
    /// # Forms
    /// Assembly: `jalr xd, imm(rs1)`
    JALR,
    /// RISC-V `jalr.pseudo` instruction.
    ///
    /// # Forms
    /// Assembly: `jalr.pseudo rs1`
    JALRPSEUDO,
    /// RISC-V `jr` instruction.
    ///
    /// # Forms
    /// Assembly: `jr rs1`
    JR,
    /// Load byte
    ///
    /// Load 8 bits of data into register `rd` from an
    /// address formed by adding `rs1` to a signed offset.
    /// Sign extend the result.
    ///
    /// # Forms
    /// Assembly: `lb xd, imm(rs1)`
    LB,
    /// Load byte unsigned
    ///
    /// Load 8 bits of data into register `rd` from an
    /// address formed by adding `rs1` to a signed offset.
    /// Zero extend the result.
    ///
    /// # Forms
    /// Assembly: `lbu xd, imm(rs1)`
    LBU,
    /// Load doubleword
    ///
    /// Load 64 bits of data into register `rd` from an
    /// address formed by adding `rs1` to a signed offset.
    ///
    /// # Forms
    /// Assembly: `ld xd, imm(rs1)`
    LD,
    /// Load halfword
    ///
    /// Load 16 bits of data into register `rd` from an
    /// address formed by adding `rs1` to a signed offset.
    /// Sign extend the result.
    ///
    /// # Forms
    /// Assembly: `lh xd, imm(rs1)`
    LH,
    /// Load halfword unsigned
    ///
    /// Load 16 bits of data into register `rd` from an
    /// address formed by adding `rs1` to a signed offset.
    /// Zero extend the result.
    ///
    /// # Forms
    /// Assembly: `lhu xd, imm(rs1)`
    LHU,
    /// RISC-V `lpad` instruction.
    ///
    /// # Forms
    /// Assembly: `lpad imm`
    LPAD,
    /// Load reserved doubleword
    ///
    /// Loads a word from the address in rs1, places the value in rd,
    /// and registers a _reservation set_  -- a set of bytes that subsumes the bytes in the
    /// addressed word.
    ///
    /// The address in rs1 must be 8-byte aligned.
    ///
    /// If the address is not naturally aligned, a `LoadAddressMisaligned` exception or an
    /// `LoadAccessFault` exception will be generated. The access-fault exception can be generated
    /// for a memory access that would otherwise be able to complete except for the misalignment,
    /// if the misaligned access should not be emulated.
    ///
    /// An implementation can register an arbitrarily large reservation set on each LR, provided the
    /// reservation set includes all bytes of the addressed data word or doubleword.
    /// An SC can only pair with the most recent LR in program order.
    /// An SC may succeed only if no store from another hart to the reservation set can be
    /// observed to have occurred between the LR and the SC, and if there is no other SC between the
    /// LR and itself in program order.
    /// An SC may succeed only if no write from a device other than a hart to the bytes accessed by
    /// the LR instruction can be observed to have occurred between the LR and SC. Note this LR
    /// might have had a different effective address and data size, but reserved the SC's
    /// address as part of the reservation set.
    ///
    /// \[NOTE\]
    /// ----
    /// Following this model, in systems with memory translation, an SC is allowed to succeed if the
    /// earlier LR reserved the same location using an alias with a different virtual address, but is
    /// also allowed to fail if the virtual address is different.
    ///
    /// To accommodate legacy devices and buses, writes from devices other than RISC-V harts are only
    /// required to invalidate reservations when they overlap the bytes accessed by the LR.
    /// These writes are not required to invalidate the reservation when they access other bytes in
    /// the reservation set.
    /// ----
    ///
    /// Software should not set the _rl_ bit on an LR instruction unless the _aq_ bit is also set.
    /// LR.rl and SC.aq instructions are not guaranteed to provide any stronger ordering than those
    /// with both bits clear, but may result in lower performance.
    ///
    /// # Forms
    /// Assembly: `lr.d xd, xs1`
    LRD,
    /// Load reserved word
    ///
    /// Loads a word from the address in rs1, places the sign-extended value in rd,
    /// and registers a _reservation set_  -- a set of bytes that subsumes the bytes in the
    /// addressed word.
    ///
    /// &lt;%- if XLEN == 64 -%&gt;
    /// The 32-bit load result is sign-extended to 64-bits.
    /// &lt;%- end -%&gt;
    ///
    /// The address in rs1 must be naturally aligned to the size of the operand
    /// (_i.e._, eight-byte aligned for doublewords and four-byte aligned for words).
    ///
    /// If the address is not naturally aligned, a `LoadAddressMisaligned` exception or an
    /// `LoadAccessFault` exception will be generated. The access-fault exception can be generated
    /// for a memory access that would otherwise be able to complete except for the misalignment,
    /// if the misaligned access should not be emulated.
    ///
    /// An implementation can register an arbitrarily large reservation set on each LR, provided the
    /// reservation set includes all bytes of the addressed data word or doubleword.
    /// An SC can only pair with the most recent LR in program order.
    /// An SC may succeed only if no store from another hart to the reservation set can be
    /// observed to have occurred between the LR and the SC, and if there is no other SC between the
    /// LR and itself in program order.
    /// An SC may succeed only if no write from a device other than a hart to the bytes accessed by
    /// the LR instruction can be observed to have occurred between the LR and SC. Note this LR
    /// might have had a different effective address and data size, but reserved the SC's
    /// address as part of the reservation set.
    ///
    /// \[NOTE\]
    /// ----
    /// Following this model, in systems with memory translation, an SC is allowed to succeed if the
    /// earlier LR reserved the same location using an alias with a different virtual address, but is
    /// also allowed to fail if the virtual address is different.
    ///
    /// To accommodate legacy devices and buses, writes from devices other than RISC-V harts are only
    /// required to invalidate reservations when they overlap the bytes accessed by the LR.
    /// These writes are not required to invalidate the reservation when they access other bytes in
    /// the reservation set.
    /// ----
    ///
    /// Software should not set the _rl_ bit on an LR instruction unless the _aq_ bit is also set.
    /// LR.rl and SC.aq instructions are not guaranteed to provide any stronger ordering than those
    /// with both bits clear, but may result in lower performance.
    ///
    /// # Forms
    /// Assembly: `lr.w xd, xs1`
    LRW,
    /// Load upper immediate
    ///
    /// Load the zero-extended imm into rd.
    ///
    /// # Forms
    /// Assembly: `lui xd, imm`
    LUI,
    /// Load word
    ///
    /// Load 32 bits of data into register `rd` from an
    /// address formed by adding `rs1` to a signed offset.
    /// Sign extend the result.
    ///
    /// # Forms
    /// Assembly: `lw xd, imm(rs1)`
    LW,
    /// Load word unsigned
    ///
    /// Load 64 bits of data into register `rd` from an
    /// address formed by adding `rs1` to a signed offset.
    /// Zero extend the result.
    ///
    /// # Forms
    /// Assembly: `lwu xd, imm(rs1)`
    LWU,
    /// Maximum
    ///
    /// This instruction returns the larger of two signed integers.
    ///
    /// .Software Hint
    /// \[NOTE\]
    /// Calculating the absolute value of a signed integer can be performed using the
    /// following sequence: `neg rD,rS` followed by `max rD,rS,rD. When using this
    /// common sequence, it is suggested that they are scheduled with no intervening
    /// instructions so that implementations that are so optimized can fuse them
    /// together.
    ///
    /// # Forms
    /// Assembly: `max xd, xs1, xs2`
    MAX,
    /// Unsigned maximum
    ///
    /// This instruction returns the larger of two unsigned integers.
    ///
    /// # Forms
    /// Assembly: `maxu xd, xs1, xs2`
    MAXU,
    /// Minimum
    ///
    /// This instruction returns the smaller of two signed integers.
    ///
    /// # Forms
    /// Assembly: `min xd, xs1, xs2`
    MIN,
    /// Unsigned minimum
    ///
    /// This instruction returns the smaller of two unsigned integers.
    ///
    /// # Forms
    /// Assembly: `minu xd, xs1, xs2`
    MINU,
    /// RISC-V `mnret` instruction.
    ///
    /// # Forms
    /// Assembly: `mnret mnret`
    MNRET,
    /// RISC-V `mop.r.0` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.0 rd rs1`
    MOPR0,
    /// RISC-V `mop.r.1` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.1 rd rs1`
    MOPR1,
    /// RISC-V `mop.r.10` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.10 rd rs1`
    MOPR10,
    /// RISC-V `mop.r.11` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.11 rd rs1`
    MOPR11,
    /// RISC-V `mop.r.12` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.12 rd rs1`
    MOPR12,
    /// RISC-V `mop.r.13` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.13 rd rs1`
    MOPR13,
    /// RISC-V `mop.r.14` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.14 rd rs1`
    MOPR14,
    /// RISC-V `mop.r.15` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.15 rd rs1`
    MOPR15,
    /// RISC-V `mop.r.16` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.16 rd rs1`
    MOPR16,
    /// RISC-V `mop.r.17` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.17 rd rs1`
    MOPR17,
    /// RISC-V `mop.r.18` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.18 rd rs1`
    MOPR18,
    /// RISC-V `mop.r.19` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.19 rd rs1`
    MOPR19,
    /// RISC-V `mop.r.2` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.2 rd rs1`
    MOPR2,
    /// RISC-V `mop.r.20` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.20 rd rs1`
    MOPR20,
    /// RISC-V `mop.r.21` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.21 rd rs1`
    MOPR21,
    /// RISC-V `mop.r.22` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.22 rd rs1`
    MOPR22,
    /// RISC-V `mop.r.23` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.23 rd rs1`
    MOPR23,
    /// RISC-V `mop.r.24` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.24 rd rs1`
    MOPR24,
    /// RISC-V `mop.r.25` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.25 rd rs1`
    MOPR25,
    /// RISC-V `mop.r.26` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.26 rd rs1`
    MOPR26,
    /// RISC-V `mop.r.27` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.27 rd rs1`
    MOPR27,
    /// RISC-V `mop.r.28` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.28 rd rs1`
    MOPR28,
    /// RISC-V `mop.r.29` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.29 rd rs1`
    MOPR29,
    /// RISC-V `mop.r.3` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.3 rd rs1`
    MOPR3,
    /// RISC-V `mop.r.30` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.30 rd rs1`
    MOPR30,
    /// RISC-V `mop.r.31` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.31 rd rs1`
    MOPR31,
    /// RISC-V `mop.r.4` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.4 rd rs1`
    MOPR4,
    /// RISC-V `mop.r.5` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.5 rd rs1`
    MOPR5,
    /// RISC-V `mop.r.6` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.6 rd rs1`
    MOPR6,
    /// RISC-V `mop.r.7` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.7 rd rs1`
    MOPR7,
    /// RISC-V `mop.r.8` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.8 rd rs1`
    MOPR8,
    /// RISC-V `mop.r.9` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.9 rd rs1`
    MOPR9,
    /// RISC-V `mop.r.n` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.r.n mop_r_t_30, mop_r_t_27_26, mop_r_t_21_20, xd, xs1`
    MOPRN,
    /// RISC-V `mop.rr.0` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.rr.0 rd rs1 rs2`
    MOPRR0,
    /// RISC-V `mop.rr.1` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.rr.1 rd rs1 rs2`
    MOPRR1,
    /// RISC-V `mop.rr.2` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.rr.2 rd rs1 rs2`
    MOPRR2,
    /// RISC-V `mop.rr.3` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.rr.3 rd rs1 rs2`
    MOPRR3,
    /// RISC-V `mop.rr.4` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.rr.4 rd rs1 rs2`
    MOPRR4,
    /// RISC-V `mop.rr.5` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.rr.5 rd rs1 rs2`
    MOPRR5,
    /// RISC-V `mop.rr.6` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.rr.6 rd rs1 rs2`
    MOPRR6,
    /// RISC-V `mop.rr.7` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.rr.7 rd rs1 rs2`
    MOPRR7,
    /// RISC-V `mop.rr.n` instruction.
    ///
    /// # Forms
    /// Assembly: `mop.rr.n mop_rr_t_30, mop_rr_t_27_26, xd, xs1, xs2`
    MOPRRN,
    /// Machine Exception Return
    ///
    /// Returns from an exception in M-mode.
    ///
    /// # Forms
    /// Assembly: `mret ""`
    MRET,
    /// Signed multiply
    ///
    /// MUL performs an XLEN-bitxXLEN-bit multiplication of `rs1` by `rs2` and places the lower
    /// XLEN bits in the destination register.
    /// Any overflow is thrown away.
    ///
    /// \[NOTE\]
    /// If both the high and low bits of the same product are required, then the recommended code
    /// sequence is:
    /// MULH\[\[S\]U\] rdh, rs1, rs2; MUL rdl, rs1, rs2
    /// (source register specifiers must be in same order and rdh cannot be the same as rs1 or rs2).
    /// Microarchitectures can then fuse these into a single multiply operation instead of
    /// performing two separate multiplies.
    ///
    /// # Forms
    /// Assembly: `mul xd, xs1, xs2`
    MUL,
    /// Signed multiply high
    ///
    /// Multiply the signed values in rs1 to rs2, and store the upper half of the result in rd.
    /// The lower half is thrown away.
    ///
    /// If both the upper and lower halves are needed, it suggested to use the sequence:
    ///
    /// ---
    ///   mulh rdh, rs1, rs2
    ///   mul  rdl, rs1, rs2
    /// ---
    ///
    /// Microarchitectures may look for that sequence and fuse the operations.
    ///
    /// # Forms
    /// Assembly: `mulh xd, xs1, xs2`
    MULH,
    /// Signed/unsigned multiply high
    ///
    /// Multiply the signed value in rs1 by the unsigned value in rs2, and store the upper half of the result in rd.
    /// The lower half is thrown away.
    ///
    /// If both the upper and lower halves are needed, it suggested to use the sequence:
    ///
    /// ---
    ///   mulhsu rdh, rs1, rs2
    ///   mul    rdl, rs1, rs2
    /// ---
    ///
    /// Microarchitectures may look for that sequence and fuse the operations.
    ///
    /// # Forms
    /// Assembly: `mulhsu xd, xs1, xs2`
    MULHSU,
    /// Unsigned multiply high
    ///
    /// Multiply the unsigned values in rs1 to rs2, and store the upper half of the result in rd.
    /// The lower half is thrown away.
    ///
    /// If both the upper and lower halves are needed, it suggested to use the sequence:
    ///
    /// ---
    ///   mulhu rdh, rs1, rs2
    ///   mul   rdl, rs1, rs2
    /// ---
    ///
    /// Microarchitectures may look for that sequence and fuse the operations.
    ///
    /// # Forms
    /// Assembly: `mulhu xd, xs1, xs2`
    MULHU,
    /// Signed 32-bit multiply
    ///
    /// Multiplies the lower 32 bits of the source registers, placing the sign-extension of the
    /// lower 32 bits of the result into the destination register.
    ///
    /// Any overflow is thrown away.
    ///
    /// \[NOTE\]
    /// In RV64, MUL can be used to obtain the upper 32 bits of the 64-bit product,
    /// but signed arguments must be proper 32-bit signed values, whereas unsigned arguments
    /// must have their upper 32 bits clear. If the arguments are not known to be sign- or zero-extended,
    /// an alternative is to shift both arguments left by 32 bits, then use MULH\[\[S\]U\].
    ///
    /// # Forms
    /// Assembly: `mulw xd, xs1, xs2`
    MULW,
    /// RISC-V `mv` instruction.
    ///
    /// # Forms
    /// Assembly: `mv rd rs1`
    MV,
    /// RISC-V `neg` instruction.
    ///
    /// # Forms
    /// Assembly: `neg rd rs1`
    NEG,
    /// RISC-V `nop` instruction.
    ///
    /// # Forms
    /// Assembly: `nop`
    NOP,
    /// RISC-V `ntl.all` instruction.
    ///
    /// # Forms
    /// Assembly: `ntl.all`
    NTLALL,
    /// RISC-V `ntl.p1` instruction.
    ///
    /// # Forms
    /// Assembly: `ntl.p1`
    NTLP1,
    /// RISC-V `ntl.pall` instruction.
    ///
    /// # Forms
    /// Assembly: `ntl.pall`
    NTLPALL,
    /// RISC-V `ntl.s1` instruction.
    ///
    /// # Forms
    /// Assembly: `ntl.s1`
    NTLS1,
    /// Or
    ///
    /// Or rs1 with rs2, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `or xd, xs1, xs2`
    OR,
    /// Bitware OR-combine, byte granule
    ///
    /// Combines the bits within each byte using bitwise logical OR. This sets the bits
    /// of each byte in the result rd to all zeros if no bit within the respective byte
    /// of rs is set, or to all ones if any bit within the respective byte of rs is set.
    ///
    /// # Forms
    /// Assembly: `orc.b xd, xs1, xs2`
    ORCB,
    /// Or immediate
    ///
    /// Or an immediate to the value in rs1, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `ori xd, xs1, imm`
    ORI,
    /// OR with inverted operand
    ///
    /// This instruction performs the bitwise logical OR operation between rs1 and the bitwise inversion of rs2.
    ///
    /// # Forms
    /// Assembly: `orn xd, xs1, xs2`
    ORN,
    /// RISC-V `pack` instruction.
    ///
    /// # Forms
    /// Assembly: `pack xd, xs1, xs2`
    PACK,
    /// RISC-V `packh` instruction.
    ///
    /// # Forms
    /// Assembly: `packh xd, xs1, xs2`
    PACKH,
    /// RISC-V `packw` instruction.
    ///
    /// # Forms
    /// Assembly: `packw xd, xs1, xs2`
    PACKW,
    /// RISC-V `pause` instruction.
    ///
    /// # Forms
    /// Assembly: `pause`
    PAUSE,
    /// RISC-V `prefetch.i` instruction.
    ///
    /// # Forms
    /// Assembly: `prefetch.i rs1 imm12lohi`
    PREFETCHI,
    /// RISC-V `prefetch.r` instruction.
    ///
    /// # Forms
    /// Assembly: `prefetch.r rs1 imm12lohi`
    PREFETCHR,
    /// RISC-V `prefetch.w` instruction.
    ///
    /// # Forms
    /// Assembly: `prefetch.w rs1 imm12lohi`
    PREFETCHW,
    /// RISC-V `rdcycle` instruction.
    ///
    /// # Forms
    /// Assembly: `rdcycle rd`
    RDCYCLE,
    /// RISC-V `rdcycleh` instruction.
    ///
    /// # Forms
    /// Assembly: `rdcycleh rd`
    RDCYCLEH,
    /// RISC-V `rdinstret` instruction.
    ///
    /// # Forms
    /// Assembly: `rdinstret rd`
    RDINSTRET,
    /// RISC-V `rdinstreth` instruction.
    ///
    /// # Forms
    /// Assembly: `rdinstreth rd`
    RDINSTRETH,
    /// RISC-V `rdtime` instruction.
    ///
    /// # Forms
    /// Assembly: `rdtime rd`
    RDTIME,
    /// RISC-V `rdtimeh` instruction.
    ///
    /// # Forms
    /// Assembly: `rdtimeh rd`
    RDTIMEH,
    /// Signed remainder
    ///
    /// Calculate the remainder of signed division of rs1 by rs2, and store the result in rd.
    ///
    /// If the value in register rs2 is zero, write the value in rs1 into rd;
    ///
    /// If the result of the division overflows, write zero into rd;
    ///
    /// # Forms
    /// Assembly: `rem xd, xs1, xs2`
    REM,
    /// Unsigned remainder
    ///
    /// Calculate the remainder of unsigned division of rs1 by rs2, and store the result in rd.
    ///
    /// # Forms
    /// Assembly: `remu xd, xs1, xs2`
    REMU,
    /// Unsigned 32-bit remainder
    ///
    /// Calculate the remainder of unsigned division of the 32-bit values in rs1 by rs2,
    /// and store the sign-extended result in rd.
    ///
    /// If the value in rs2 is zero, rd gets the sign-extended value in rs1.
    ///
    /// # Forms
    /// Assembly: `remuw xd, xs1, xs2`
    REMUW,
    /// Signed 32-bit remainder
    ///
    /// Calculate the remainder of signed division of the 32-bit values rs1 by rs2,
    /// and store the sign-extended result in rd.
    ///
    /// If the value in register rs2 is zero, write the sign-extended 32-bit value in rs1 into rd;
    ///
    /// If the result of the division overflows, write zero into rd;
    ///
    /// # Forms
    /// Assembly: `remw xd, xs1, xs2`
    REMW,
    /// RISC-V `ret` instruction.
    ///
    /// # Forms
    /// Assembly: `ret`
    RET,
    /// Byte-reverse register (RV64 encoding)
    ///
    /// This instruction reverses the order of the bytes in rs1.
    ///
    /// \[NOTE\]
    /// The rev8 mnemonic corresponds to different instruction encodings in RV32 and RV64.
    ///
    /// \[NOTE\]
    /// The byte-reverse operation is only available for the full register width. To emulate word-sized
    /// and halfword-sized byte-reversal, perform a `rev8 rd,rs` followed by a `srai rd,rd,K`, where K
    /// is XLEN-32 and XLEN-16, respectively.
    ///
    /// # Forms
    /// Assembly: `rev8 xd, xs1`
    REV8,
    /// Byte-reverse register (RV64 encoding)
    ///
    /// This instruction reverses the order of the bytes in rs1.
    ///
    /// \[NOTE\]
    /// The rev8 mnemonic corresponds to different instruction encodings in RV32 and RV64.
    ///
    /// \[NOTE\]
    /// The byte-reverse operation is only available for the full register width. To emulate word-sized
    /// and halfword-sized byte-reversal, perform a `rev8 rd,rs` followed by a `srai rd,rd,K`, where K
    /// is XLEN-32 and XLEN-16, respectively.
    ///
    /// # Forms
    /// Assembly: `rev8.rv32 xd, xs1`
    REV8RV32,
    /// Rotate left (Register)
    ///
    /// This instruction performs a rotate left of rs1 by the amount in least-significant `log2(XLEN)` bits of rs2.
    ///
    /// # Forms
    /// Assembly: `rol xd, xs1, xs2`
    ROL,
    /// Rotate left word (Register)
    ///
    /// This instruction performs a rotate left of the least-significant word of rs1 by the amount in least-significant 5 bits of rs2.
    /// The resulting word value is sign-extended by copying bit 31 to all of the more-significant bits.
    ///
    /// # Forms
    /// Assembly: `rolw xd, xs1, xs2`
    ROLW,
    /// Rotate right (Register)
    ///
    /// This instruction performs a rotate right of rs1 by the amount in least-significant `log2(XLEN)` bits of rs2.
    ///
    /// # Forms
    /// Assembly: `ror xd, xs1, xs2`
    ROR,
    /// Rotate right (Immediate)
    ///
    /// This instruction performs a rotate right of rs1 by the amount in the least-significant log2(XLEN) bits of shamt.
    /// For RV32, the encodings corresponding to shamt\[5\]=1 are reserved.
    ///
    /// # Forms
    /// Assembly: `rori xd, xs1, shamt`
    RORI,
    /// Rotate right (Immediate)
    ///
    /// This instruction performs a rotate right of rs1 by the amount in the least-significant log2(XLEN) bits of shamt.
    /// For RV32, the encodings corresponding to shamt\[5\]=1 are reserved.
    ///
    /// # Forms
    /// Assembly: `rori.rv32 xd, xs1, shamt`
    RORIRV32,
    /// Rotate right word (Immediate)
    ///
    /// This instruction performs a rotate right on the least-significant word of rs1 by the amount in
    /// the least-significant log2(XLEN) bits of shamt. The resulting word value is sign-extended by
    /// copying bit 31 to all of the more-significant bits.
    ///
    /// # Forms
    /// Assembly: `roriw xd, xs1, shamt`
    RORIW,
    /// Rotate right word (Register)
    ///
    /// This instruction performs a rotate right on the least-significant word of rs1 by the amount in
    /// least-significant 5 bits of rs2. The resultant word is sign-extended by copying bit 31 to all
    /// of the more-significant bits.
    ///
    /// # Forms
    /// Assembly: `rorw xd, xs1, xs2`
    RORW,
    /// Store byte
    ///
    /// Store 8 bits of data from register `rs2` to an
    /// address formed by adding `rs1` to a signed offset.
    ///
    /// # Forms
    /// Assembly: `sb xs2, imm(xs1)`
    SB,
    /// RISC-V `sbreak` instruction.
    ///
    /// # Forms
    /// Assembly: `sbreak`
    SBREAK,
    /// Store conditional doubleword
    ///
    /// `sc.d` conditionally writes a doubleword in _rs2_ to the address in _rs1_:
    /// the `sc.d` succeeds only if the reservation is still valid and the
    /// reservation set contains the bytes being written. If the `sc.d` succeeds,
    /// the instruction writes the doubleword in _rs2_ to memory, and it writes zero to _rd_.
    /// If the `sc.d` fails, the instruction does not write to memory, and it writes a
    /// nonzero value to _rd_. For the purposes of memory protection, a failed `sc.d`
    /// may be treated like a store. Regardless of success or failure, executing an
    /// `sc.d` instruction invalidates any reservation held by this hart.
    ///
    /// The failure code with value 1 encodes an unspecified failure.
    /// Other failure codes are reserved at this time.
    /// Portable software should only assume the failure code will be non-zero.
    ///
    /// The address held in _rs1_ must be naturally aligned to the size of the operand
    /// (_i.e._, eight-byte aligned).
    /// If the address is not naturally aligned, an address-misaligned exception or an
    /// access-fault exception will be generated.
    /// The access-fault exception can be generated for a memory access that would otherwise
    /// be able to complete except for the misalignment,
    /// if the misaligned access should not be emulated.
    ///
    /// \[NOTE\]
    /// --
    /// Emulating misaligned LR/SC sequences is impractical in most systems.
    ///
    /// Misaligned LR/SC sequences also raise the possibility of accessing multiple
    /// reservation sets at once, which present definitions do not provide for.
    /// --
    ///
    /// An implementation can register an arbitrarily large reservation set on each LR,
    /// provided the reservation set includes all bytes of the addressed data word or
    /// doubleword.
    /// An SC can only pair with the most recent LR in program order.
    /// An SC may succeed only if no store from another hart to the reservation set
    /// can be observed to have occurred between the LR and the SC,
    /// and if there is no other SC between the LR and itself in program order.
    /// An SC may succeed only if no write from a device other than a hart to the bytes
    /// accessed by the LR instruction can be observed to have occurred between the LR
    /// and SC.
    /// Note this LR might have had a different effective address and data size,
    /// but reserved the SC's address as part of the reservation set.
    ///
    /// \[NOTE\]
    /// ----
    /// Following this model, in systems with memory translation, an SC is allowed to succeed if the
    /// earlier LR reserved the same location using an alias with a different virtual address, but is
    /// also allowed to fail if the virtual address is different.
    ///
    /// To accommodate legacy devices and buses, writes from devices other than RISC-V harts are only
    /// required to invalidate reservations when they overlap the bytes accessed by the LR.
    /// These writes are not required to invalidate the reservation when they access other bytes in
    /// the reservation set.
    /// ----
    ///
    /// The SC must fail if the address is not within the reservation set of the most
    /// recent LR in program order.
    /// The SC must fail if a store to the reservation set from another hart can be
    /// observed to occur between the LR and SC.
    /// The SC must fail if a write from some other device to the bytes accessed by the
    /// LR can be observed to occur between the LR and SC.
    /// (If such a device writes the reservation set but does not write the bytes accessed
    /// by the LR, the SC may or may not fail.)
    /// An SC must fail if there is another SC (to any address) between the LR and the SC
    /// in program order.
    /// The precise statement of the atomicity requirements for successful LR/SC sequences
    /// is defined by the Atomicity Axiom of the memory model.
    ///
    /// \[NOTE\]
    /// --
    /// The platform should provide a means to determine the size and shape of the reservation set.
    ///
    /// A platform specification may constrain the size and shape of the reservation set.
    ///
    /// A store-conditional instruction to a scratch word of memory should be used to forcibly invalidate any existing load reservation:
    ///
    ///   * during a preemptive context switch, and
    ///   * if necessary when changing virtual to physical address mappings, such as when migrating pages that might contain an active reservation.
    ///
    /// The invalidation of a hart's reservation when it executes an LR or SC imply that a hart can only hold one reservation at a time, and that an SC can only pair with the most recent LR, and LR with the next following SC, in program order. This is a restriction to the Atomicity Axiom in Section 18.1 that ensures software runs correctly on expected common implementations that operate in this manner.
    /// --
    ///
    /// An SC instruction can never be observed by another RISC-V hart before the LR instruction that established the reservation.
    ///
    /// \[NOTE\]
    /// --
    /// The LR/SC sequence can be given acquire semantics by setting the aq bit on the LR instruction. The LR/SC sequence can be given release semantics by by setting the rl bit on the SC instruction. Assuming suitable mappings for other atomic operations, setting the aq bit on the LR instruction, and setting the rl bit on the SC instruction makes the LR/SC sequence sequentially consistent in the C++ memory_order_seq_cst sense. Such a sequence does not act as a fence for ordering ordinary load and store instructions before and after the sequence. Specific instruction mappings for other C++ atomic operations, or stronger notions of "sequential consistency", may require both bits to be set on either or both of the LR or SC instruction.
    ///
    /// If neither bit is set on either LR or SC, the LR/SC sequence can be observed to occur before or after surrounding memory operations from the same RISC-V hart. This can be appropriate when the LR/SC sequence is used to implement a parallel reduction operation.
    /// --
    ///
    /// Software should not set the _rl_ bit on an LR instruction unless the _aq_ bit is also set.
    /// LR.rl and SC.aq instructions are not guaranteed to provide any stronger ordering than those
    /// with both bits clear, but may result in lower performance.
    ///
    /// # Forms
    /// Assembly: `sc.d xd, xs2, xs1`
    SCD,
    /// Store conditional word
    ///
    /// `sc.w` conditionally writes a word in _rs2_ to the address in _rs1_:
    /// the `sc.w` succeeds only if the reservation is still valid and the
    /// reservation set contains the bytes being written. If the `sc.w` succeeds,
    /// the instruction writes the word in _rs2_ to memory, and it writes zero to _rd_.
    /// If the `sc.w` fails, the instruction does not write to memory, and it writes a
    /// nonzero value to _rd_. For the purposes of memory protection, a failed `sc.w`
    /// may be treated like a store. Regardless of success or failure, executing an
    /// `sc.w` instruction invalidates any reservation held by this hart.
    ///
    /// &lt;%- if XLEN == 64 -%&gt;
    /// \[NOTE\]
    /// If a value other than 0 or 1 is defined as a result for `sc.w`, the value will before
    /// sign-extended into _rd_.
    /// &lt;%- end -%&gt;
    ///
    /// The failure code with value 1 encodes an unspecified failure.
    /// Other failure codes are reserved at this time.
    /// Portable software should only assume the failure code will be non-zero.
    ///
    /// The address held in _rs1_ must be naturally aligned to the size of the operand
    /// (_i.e._, eight-byte aligned for doublewords and four-byte aligned for words).
    /// If the address is not naturally aligned, an address-misaligned exception or an
    /// access-fault exception will be generated.
    /// The access-fault exception can be generated for a memory access that would otherwise
    /// be able to complete except for the misalignment,
    /// if the misaligned access should not be emulated.
    ///
    /// \[NOTE\]
    /// --
    /// Emulating misaligned LR/SC sequences is impractical in most systems.
    ///
    /// Misaligned LR/SC sequences also raise the possibility of accessing multiple
    /// reservation sets at once, which present definitions do not provide for.
    /// --
    ///
    /// An implementation can register an arbitrarily large reservation set on each LR,
    /// provided the reservation set includes all bytes of the addressed data word or
    /// doubleword.
    /// An SC can only pair with the most recent LR in program order.
    /// An SC may succeed only if no store from another hart to the reservation set
    /// can be observed to have occurred between the LR and the SC,
    /// and if there is no other SC between the LR and itself in program order.
    /// An SC may succeed only if no write from a device other than a hart to the bytes
    /// accessed by the LR instruction can be observed to have occurred between the LR
    /// and SC.
    /// Note this LR might have had a different effective address and data size,
    /// but reserved the SC's address as part of the reservation set.
    ///
    /// \[NOTE\]
    /// ----
    /// Following this model, in systems with memory translation, an SC is allowed to succeed if the
    /// earlier LR reserved the same location using an alias with a different virtual address, but is
    /// also allowed to fail if the virtual address is different.
    ///
    /// To accommodate legacy devices and buses, writes from devices other than RISC-V harts are only
    /// required to invalidate reservations when they overlap the bytes accessed by the LR.
    /// These writes are not required to invalidate the reservation when they access other bytes in
    /// the reservation set.
    /// ----
    ///
    /// The SC must fail if the address is not within the reservation set of the most
    /// recent LR in program order.
    /// The SC must fail if a store to the reservation set from another hart can be
    /// observed to occur between the LR and SC.
    /// The SC must fail if a write from some other device to the bytes accessed by the
    /// LR can be observed to occur between the LR and SC.
    /// (If such a device writes the reservation set but does not write the bytes accessed
    /// by the LR, the SC may or may not fail.)
    /// An SC must fail if there is another SC (to any address) between the LR and the SC
    /// in program order.
    /// The precise statement of the atomicity requirements for successful LR/SC sequences
    /// is defined by the Atomicity Axiom of the memory model.
    ///
    /// \[NOTE\]
    /// --
    /// The platform should provide a means to determine the size and shape of the reservation set.
    ///
    /// A platform specification may constrain the size and shape of the reservation set.
    ///
    /// A store-conditional instruction to a scratch word of memory should be used to forcibly invalidate any existing load reservation:
    ///
    ///   * during a preemptive context switch, and
    ///   * if necessary when changing virtual to physical address mappings, such as when migrating pages that might contain an active reservation.
    ///
    /// The invalidation of a hart's reservation when it executes an LR or SC imply that a hart can only hold one reservation at a time, and that an SC can only pair with the most recent LR, and LR with the next following SC, in program order. This is a restriction to the Atomicity Axiom in Section 18.1 that ensures software runs correctly on expected common implementations that operate in this manner.
    /// --
    ///
    /// An SC instruction can never be observed by another RISC-V hart before the LR instruction that established the reservation.
    ///
    /// \[NOTE\]
    /// --
    /// The LR/SC sequence can be given acquire semantics by setting the aq bit on the LR instruction. The LR/SC sequence can be given release semantics by by setting the rl bit on the SC instruction. Assuming suitable mappings for other atomic operations, setting the aq bit on the LR instruction, and setting the rl bit on the SC instruction makes the LR/SC sequence sequentially consistent in the C++ memory_order_seq_cst sense. Such a sequence does not act as a fence for ordering ordinary load and store instructions before and after the sequence. Specific instruction mappings for other C++ atomic operations, or stronger notions of "sequential consistency", may require both bits to be set on either or both of the LR or SC instruction.
    ///
    /// If neither bit is set on either LR or SC, the LR/SC sequence can be observed to occur before or after surrounding memory operations from the same RISC-V hart. This can be appropriate when the LR/SC sequence is used to implement a parallel reduction operation.
    /// --
    ///
    /// Software should not set the _rl_ bit on an LR instruction unless the _aq_ bit is also set.
    /// LR.rl and SC.aq instructions are not guaranteed to provide any stronger ordering than those
    /// with both bits clear, but may result in lower performance.
    ///
    /// # Forms
    /// Assembly: `sc.w xd, xs2, xs1`
    SCW,
    /// RISC-V `scall` instruction.
    ///
    /// # Forms
    /// Assembly: `scall`
    SCALL,
    /// RISC-V `sctrclr` instruction.
    ///
    /// # Forms
    /// Assembly: `sctrclr sctrclr`
    SCTRCLR,
    /// Store doubleword
    ///
    /// Store 64 bits of data from register `rs2` to an
    /// address formed by adding `rs1` to a signed offset.
    ///
    /// # Forms
    /// Assembly: `sd xs2, imm(xs1)`
    SD,
    /// RISC-V `seqz` instruction.
    ///
    /// # Forms
    /// Assembly: `seqz rd rs1`
    SEQZ,
    /// Sign-extend byte
    ///
    /// This instruction sign-extends the least-significant byte in the source to XLEN by copying the
    /// most-significant bit in the byte (i.e., bit 7) to all of the more-significant bits.
    ///
    /// # Forms
    /// Assembly: `sext.b xd, xs1`
    SEXTB,
    /// Sign-extend halfword
    ///
    /// This instruction sign-extends the least-significant halfword in the source to XLEN by copying the
    /// most-significant bit in the halfword (i.e., bit 15) to all of the more-significant bits.
    ///
    /// # Forms
    /// Assembly: `sext.h xd, xs1`
    SEXTH,
    /// RISC-V `sext.w` instruction.
    ///
    /// # Forms
    /// Assembly: `sext.w rd rs1`
    SEXTW,
    /// Order implicit page table reads after invalidation
    ///
    /// The `sfence.inval.ir` instruction guarantees that any previous `sinval.vma`
    /// instructions executed by the current hart are ordered before subsequent implicit references by
    /// that hart to the memory-management data structures.
    ///
    /// # Forms
    /// Assembly: `sfence.inval.ir ""`
    SFENCEINVALIR,
    /// Supervisor memory-management fence
    ///
    /// The supervisor memory-management fence instruction `SFENCE.VMA` is used to
    /// synchronize updates to in-memory memory-management data structures with
    /// current execution. Instruction execution causes implicit reads and
    /// writes to these data structures; however, these implicit references are
    /// ordinarily not ordered with respect to explicit loads and stores.
    /// Executing an SFENCE.VMA instruction guarantees that any previous stores
    /// already visible to the current RISC-V hart are ordered before certain
    /// implicit references by subsequent instructions in that hart to the
    /// memory-management data structures. The specific set of operations
    /// ordered by SFENCE.VMA is determined by _rs1_ and _rs2_, as described
    /// below. SFENCE.VMA is also used to invalidate entries in the
    /// address-translation cache associated with a hart (see &lt;&lt;sv32algorithm&gt;&gt;). Further details on the behavior of this instruction are described in &lt;&lt;virt-control&gt;&gt; and &lt;&lt;pmp-vmem&gt;&gt;.
    ///
    /// \[NOTE\]
    /// ====
    /// The SFENCE.VMA is used to flush any local hardware caches related to
    /// address translation. It is specified as a fence rather than a TLB flush
    /// to provide cleaner semantics with respect to which instructions are
    /// affected by the flush operation and to support a wider variety of
    /// dynamic caching structures and memory-management schemes. SFENCE.VMA is
    /// also used by higher privilege levels to synchronize page table writes
    /// and the address translation hardware.
    /// ====
    ///
    /// SFENCE.VMA orders only the local hart's implicit references to the
    /// memory-management data structures.
    ///
    /// \[NOTE\]
    /// ====
    /// Consequently, other harts must be notified separately when the
    /// memory-management data structures have been modified. One approach is to
    /// use 1) a local data fence to ensure local writes are visible globally,
    /// then 2) an interprocessor interrupt to the other thread, then 3) a local
    /// SFENCE.VMA in the interrupt handler of the remote thread, and finally 4)
    /// signal back to originating thread that operation is complete. This is,
    /// of course, the RISC-V analog to a TLB shootdown.
    /// ====
    ///
    /// For the common case that the translation data structures have only been
    /// modified for a single address mapping (i.e., one page or superpage),
    /// _rs1_ can specify a virtual address within that mapping to effect a
    /// translation fence for that mapping only. Furthermore, for the common
    /// case that the translation data structures have only been modified for a
    /// single address-space identifier, _rs2_ can specify the address space.
    /// The behavior of SFENCE.VMA depends on _rs1_ and _rs2_ as follows:
    ///
    /// * If __rs1__=`x0` and __rs2__=`x0`, the fence orders all reads and writes
    /// made to any level of the page tables, for all address spaces. The fence
    /// also invalidates all address-translation cache entries, for all address
    /// spaces.
    /// * If __rs1__=`x0` and __rs2__&#8800;``x0``, the fence orders all
    /// reads and writes made to any level of the page tables, but only for the
    /// address space identified by integer register _rs2_. Accesses to _global_
    /// mappings (see &lt;&lt;translation&gt;&gt;) are not ordered. The
    /// fence also invalidates all address-translation cache entries matching
    /// the address space identified by integer register _rs2_, except for
    /// entries containing global mappings.
    /// * If __rs1__&#8800;``x0`` and __rs2__=`x0`, the fence orders only
    /// reads and writes made to leaf page table entries corresponding to the
    /// virtual address in __rs1__, for all address spaces. The fence also
    /// invalidates all address-translation cache entries that contain leaf page
    /// table entries corresponding to the virtual address in _rs1_, for all
    /// address spaces.
    /// * If __rs1__&#8800;``x0`` and __rs2__&#8800;``x0``, the
    /// fence orders only reads and writes made to leaf page table entries
    /// corresponding to the virtual address in _rs1_, for the address space
    /// identified by integer register _rs2_. Accesses to global mappings are
    /// not ordered. The fence also invalidates all address-translation cache
    /// entries that contain leaf page table entries corresponding to the
    /// virtual address in _rs1_ and that match the address space identified by
    /// integer register _rs2_, except for entries containing global mappings.
    ///
    /// If the value held in _rs1_ is not a valid virtual address, then the
    /// SFENCE.VMA instruction has no effect. No exception is raised in this
    /// case.
    ///
    /// When __rs2__&#8800;``x0``, bits SXLEN-1:ASIDMAX of the value held
    /// in _rs2_ are reserved for future standard use. Until their use is
    /// defined by a standard extension, they should be zeroed by software and
    /// ignored by current implementations. Furthermore, if
    /// ASIDLEN&lt;ASIDMAX, the implementation shall ignore bits
    /// ASIDMAX-1:ASIDLEN of the value held in _rs2_.
    ///
    /// \[NOTE\]
    /// ====
    /// It is always legal to over-fence, e.g., by fencing only based on a
    /// subset of the bits in _rs1_ and/or _rs2_, and/or by simply treating all
    /// SFENCE.VMA instructions as having _rs1_=`x0` and/or _rs2_=`x0`. For
    /// example, simpler implementations can ignore the virtual address in _rs1_
    /// and the ASID value in _rs2_ and always perform a global fence. The
    /// choice not to raise an exception when an invalid virtual address is held
    /// in _rs1_ facilitates this type of simplification.
    /// ====
    ///
    /// An implicit read of the memory-management data structures may return any
    /// translation for an address that was valid at any time since the most
    /// recent SFENCE.VMA that subsumes that address. The ordering implied by
    /// SFENCE.VMA does not place implicit reads and writes to the
    /// memory-management data structures into the global memory order in a way
    /// that interacts cleanly with the standard RVWMO ordering rules. In
    /// particular, even though an SFENCE.VMA orders prior explicit accesses
    /// before subsequent implicit accesses, and those implicit accesses are
    /// ordered before their associated explicit accesses, SFENCE.VMA does not
    /// necessarily place prior explicit accesses before subsequent explicit
    /// accesses in the global memory order. These implicit loads also need not
    /// otherwise obey normal program order semantics with respect to prior
    /// loads or stores to the same address.
    ///
    /// \[NOTE\]
    /// ====
    /// A consequence of this specification is that an implementation may use
    /// any translation for an address that was valid at any time since the most
    /// recent SFENCE.VMA that subsumes that address. In particular, if a leaf
    /// PTE is modified but a subsuming SFENCE.VMA is not executed, either the
    /// old translation or the new translation will be used, but the choice is
    /// unpredictable. The behavior is otherwise well-defined.
    ///
    /// In a conventional TLB design, it is possible for multiple entries to
    /// match a single address if, for example, a page is upgraded to a
    /// superpage without first clearing the original non-leaf PTE's valid bit
    /// and executing an SFENCE.VMA with __rs1__=`x0`. In this case, a similar
    /// remark applies: it is unpredictable whether the old non-leaf PTE or the
    /// new leaf PTE is used, but the behavior is otherwise well defined.
    ///
    /// Another consequence of this specification is that it is generally unsafe
    /// to update a PTE using a set of stores of a width less than the width of
    /// the PTE, as it is legal for the implementation to read the PTE at any
    /// time, including when only some of the partial stores have taken effect.
    ///
    /// ***
    ///
    /// This specification permits the caching of PTEs whose V (Valid) bit is
    /// clear. Operating systems must be written to cope with this possibility,
    /// but implementers are reminded that eagerly caching invalid PTEs will
    /// reduce performance by causing additional page faults.
    /// ====
    ///
    /// Implementations must only perform implicit reads of the translation data
    /// structures pointed to by the current contents of the `satp` register or
    /// a subsequent valid (V=1) translation data structure entry, and must only
    /// raise exceptions for implicit accesses that are generated as a result of
    /// instruction execution, not those that are performed speculatively.
    ///
    /// Changes to the `sstatus` fields SUM and MXR take effect immediately,
    /// without the need to execute an SFENCE.VMA instruction. Changing
    /// `satp`.MODE from Bare to other modes and vice versa also takes effect
    /// immediately, without the need to execute an SFENCE.VMA instruction.
    /// Likewise, changes to `satp`.ASID take effect immediately.
    ///
    /// \[TIP\]
    /// ====
    /// The following common situations typically require executing an
    /// SFENCE.VMA instruction:
    ///
    /// * When software recycles an ASID (i.e., reassociates it with a different
    /// page table), it should _first_ change `satp` to point to the new page
    /// table using the recycled ASID, _then_ execute SFENCE.VMA with __rs1__=`x0`
    /// and _rs2_ set to the recycled ASID. Alternatively, software can execute
    /// the same SFENCE.VMA instruction while a different ASID is loaded into
    /// `satp`, provided the next time `satp` is loaded with the recycled ASID,
    /// it is simultaneously loaded with the new page table.
    /// * If the implementation does not provide ASIDs, or software chooses to
    /// always use ASID 0, then after every `satp` write, software should
    /// execute SFENCE.VMA with __rs1__=`x0`. In the common case that no global
    /// translations have been modified, _rs2_ should be set to a register other
    /// than `x0` but which contains the value zero, so that global translations
    /// are not flushed.
    /// * If software modifies a non-leaf PTE, it should execute SFENCE.VMA with
    /// __rs1__=`x0`. If any PTE along the traversal path had its G bit set, _rs2_
    /// must be `x0`; otherwise, _rs2_ should be set to the ASID for which the
    /// translation is being modified.
    /// * If software modifies a leaf PTE, it should execute SFENCE.VMA with
    /// _rs1_ set to a virtual address within the page. If any PTE along the
    /// traversal path had its G bit set, _rs2_ must be `x0`; otherwise, _rs2_
    /// should be set to the ASID for which the translation is being modified.
    /// * For the special cases of increasing the permissions on a leaf PTE and
    /// changing an invalid PTE to a valid leaf, software may choose to execute
    /// the SFENCE.VMA lazily. After modifying the PTE but before executing
    /// SFENCE.VMA, either the new or old permissions will be used. In the
    /// latter case, a page-fault exception might occur, at which point software
    /// should execute SFENCE.VMA in accordance with the previous bullet point.
    /// ====
    ///
    /// If a hart employs an address-translation cache, that cache must appear
    /// to be private to that hart. In particular, the meaning of an ASID is
    /// local to a hart; software may choose to use the same ASID to refer to
    /// different address spaces on different harts.
    ///
    /// \[NOTE\]
    /// ====
    /// A future extension could redefine ASIDs to be global across the SEE,
    /// enabling such options as shared translation caches and hardware support
    /// for broadcast TLB shootdown. However, as OSes have evolved to
    /// significantly reduce the scope of TLB shootdowns using novel
    /// ASID-management techniques, we expect the local-ASID scheme to remain
    /// attractive for its simplicity and possibly better scalability.
    /// ====
    ///
    /// For implementations that make `satp`.MODE read-only zero (always Bare),
    /// attempts to execute an SFENCE.VMA instruction might raise an
    /// illegal-instruction exception.
    ///
    /// # Forms
    /// Assembly: `sfence.vma xs1, xs2`
    SFENCEVMA,
    /// Order writes before sfence
    ///
    /// The `sfence.w.inval` instruction guarantees that any previous stores already visible to the
    /// current RISC-V hart are ordered before subsequent `sinval.vma` instructions executed by the
    /// same hart.
    ///
    /// # Forms
    /// Assembly: `sfence.w.inval ""`
    SFENCEWINVAL,
    /// RISC-V `sgtz` instruction.
    ///
    /// # Forms
    /// Assembly: `sgtz rd rs2`
    SGTZ,
    /// Store halfword
    ///
    /// Store 16 bits of data from register `rs2` to an
    /// address formed by adding `rs1` to a signed offset.
    ///
    /// # Forms
    /// Assembly: `sh xs2, imm(xs1)`
    SH,
    /// Shift left by 1 and add
    ///
    /// This instruction shifts `rs1` to the left by 1 bit and adds it to `rs2`.
    ///
    /// # Forms
    /// Assembly: `sh1add xd, xs1, xs2`
    SH1ADD,
    /// Shift unsigned word left by 1 and add
    ///
    /// This instruction performs an XLEN-wide addition of two addends. The first addend is rs2.
    /// The second addend is the unsigned value formed by extracting the least-significant word of rs1
    /// and shifting it left by 1 place.
    ///
    /// # Forms
    /// Assembly: `sh1add.uw xd, xs1, xs2`
    SH1ADDUW,
    /// Shift left by 2 and add
    ///
    /// This instruction shifts `rs1` to the left by 2 places and adds it to `rs2`.
    ///
    /// # Forms
    /// Assembly: `sh2add xd, xs1, xs2`
    SH2ADD,
    /// Shift unsigned word left by 2 and add
    ///
    /// This instruction performs an XLEN-wide addition of two addends. The first addend is rs2.
    /// The second addend is the unsigned value formed by extracting the least-significant word of rs1
    /// and shifting it left by 2 places.
    ///
    /// # Forms
    /// Assembly: `sh2add.uw xd, xs1, xs2`
    SH2ADDUW,
    /// Shift left by 3 and add
    ///
    /// This instruction shifts `rs1` to the left by 3 places and adds it to `rs2`.
    ///
    /// # Forms
    /// Assembly: `sh3add xd, xs1, xs2`
    SH3ADD,
    /// Shift unsigned word left by 3 and add
    ///
    /// This instruction performs an XLEN-wide addition of two addends. The first addend is rs2.
    /// The second addend is the unsigned value formed by extracting the least-significant word of rs1
    /// and shifting it left by 3 places.
    ///
    /// # Forms
    /// Assembly: `sh3add.uw xd, xs1, xs2`
    SH3ADDUW,
    /// RISC-V `sha256sig0` instruction.
    ///
    /// # Forms
    /// Assembly: `sha256sig0 xd, xs1`
    SHA256SIG0,
    /// RISC-V `sha256sig1` instruction.
    ///
    /// # Forms
    /// Assembly: `sha256sig1 xd, xs1`
    SHA256SIG1,
    /// RISC-V `sha256sum0` instruction.
    ///
    /// # Forms
    /// Assembly: `sha256sum0 xd, xs1`
    SHA256SUM0,
    /// RISC-V `sha256sum1` instruction.
    ///
    /// # Forms
    /// Assembly: `sha256sum1 xd, xs1`
    SHA256SUM1,
    /// RISC-V `sha512sig0` instruction.
    ///
    /// # Forms
    /// Assembly: `sha512sig0 xd, xs1`
    SHA512SIG0,
    /// RISC-V `sha512sig0h` instruction.
    ///
    /// # Forms
    /// Assembly: `sha512sig0h xd, xs1, xs2`
    SHA512SIG0H,
    /// RISC-V `sha512sig0l` instruction.
    ///
    /// # Forms
    /// Assembly: `sha512sig0l xd, xs1, xs2`
    SHA512SIG0L,
    /// RISC-V `sha512sig1` instruction.
    ///
    /// # Forms
    /// Assembly: `sha512sig1 xd, xs1`
    SHA512SIG1,
    /// RISC-V `sha512sig1h` instruction.
    ///
    /// # Forms
    /// Assembly: `sha512sig1h xd, xs1, xs2`
    SHA512SIG1H,
    /// RISC-V `sha512sig1l` instruction.
    ///
    /// # Forms
    /// Assembly: `sha512sig1l xd, xs1, xs2`
    SHA512SIG1L,
    /// RISC-V `sha512sum0` instruction.
    ///
    /// # Forms
    /// Assembly: `sha512sum0 xd, xs1`
    SHA512SUM0,
    /// RISC-V `sha512sum0r` instruction.
    ///
    /// # Forms
    /// Assembly: `sha512sum0r xd, xs1, xs2`
    SHA512SUM0R,
    /// RISC-V `sha512sum1` instruction.
    ///
    /// # Forms
    /// Assembly: `sha512sum1 xd, xs1`
    SHA512SUM1,
    /// RISC-V `sha512sum1r` instruction.
    ///
    /// # Forms
    /// Assembly: `sha512sum1r xd, xs1, xs2`
    SHA512SUM1R,
    /// Invalidate cached address translations
    ///
    /// # Forms
    /// Assembly: `sinval.vma xs1, xs2`
    SINVALVMA,
    /// Shift left logical
    ///
    /// Shift the value in `rs1` left by the value in the lower 6 bits of `rs2`, and store the result in `rd`.
    ///
    /// # Forms
    /// Assembly: `sll xd, xs1, xs2`
    SLL,
    /// Shift left logical immediate
    ///
    /// Shift the value in rs1 left by shamt, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `slli xd, xs1, shamt`
    SLLI,
    /// Shift left logical immediate
    ///
    /// Shift the value in rs1 left by shamt, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `slli.rv32 xd, xs1, shamt`
    SLLIRV32,
    /// Shift left unsigned word (Immediate)
    ///
    /// This instruction takes the least-significant word of rs1, zero-extends it, and shifts it
    /// left by the immediate.
    ///
    /// \[NOTE\]
    /// This instruction is the same as `slli` with `zext.w` performed on rs1 before shifting.
    ///
    /// # Forms
    /// Assembly: `slli.uw xd, xs1, shamt`
    SLLIUW,
    /// Shift left logical immediate word
    ///
    /// Shift the 32-bit value in rs1 left by shamt, and store the sign-extended result in rd
    ///
    /// # Forms
    /// Assembly: `slliw xd, xs1, shamt`
    SLLIW,
    /// Shift left logical word
    ///
    /// Shift the 32-bit value in `rs1` left by the value in the lower 5 bits of `rs2`, and store the sign-extended result in `rd`.
    ///
    /// # Forms
    /// Assembly: `sllw xd, xs1, xs2`
    SLLW,
    /// Set on less than
    ///
    /// Places the value 1 in register `rd` if register `rs1` is less than the value in register `rs2`, where
    /// both sources are treated as signed numbers, else 0 is written to `rd`.
    ///
    /// # Forms
    /// Assembly: `slt xd, xs1, rs2`
    SLT,
    /// Set on less than immediate
    ///
    /// Places the value 1 in register `rd` if register `rs1` is less than the sign-extended immediate
    /// when both are treated as signed numbers, else 0 is written to `rd`.
    ///
    /// # Forms
    /// Assembly: `slti xd, xs1, imm`
    SLTI,
    /// Set on less than immediate unsigned
    ///
    /// Places the value 1 in register `rd` if register `rs1` is less than the sign-extended immediate
    /// when both are treated as unsigned numbers (_i.e._, the immediate is first sign-extended to
    /// XLEN bits then treated as an unsigned number), else 0 is written to `rd`.
    ///
    /// NOTE: `sltiu rd, rs1, 1` sets `rd` to 1 if `rs1` equals zero, otherwise sets `rd` to 0
    /// (assembler pseudoinstruction `SEQZ rd, rs`).
    ///
    /// # Forms
    /// Assembly: `sltiu xd, xs1, imm`
    SLTIU,
    /// Set on less than unsigned
    ///
    /// Places the value 1 in register `rd` if register `rs1` is less than the value in register `rs2`, where
    /// both sources are treated as unsigned numbers, else 0 is written to `rd`.
    ///
    /// # Forms
    /// Assembly: `sltu xd, xs1, xs2`
    SLTU,
    /// RISC-V `sltz` instruction.
    ///
    /// # Forms
    /// Assembly: `sltz rd rs1`
    SLTZ,
    /// RISC-V `sm3p0` instruction.
    ///
    /// # Forms
    /// Assembly: `sm3p0 xd, xs1`
    SM3P0,
    /// RISC-V `sm3p1` instruction.
    ///
    /// # Forms
    /// Assembly: `sm3p1 xd, xs1`
    SM3P1,
    /// RISC-V `sm4ed` instruction.
    ///
    /// # Forms
    /// Assembly: `sm4ed xd, xs1, xs2, bs`
    SM4ED,
    /// RISC-V `sm4ks` instruction.
    ///
    /// # Forms
    /// Assembly: `sm4ks xd, xs1, xs2, bs`
    SM4KS,
    /// RISC-V `snez` instruction.
    ///
    /// # Forms
    /// Assembly: `snez rd rs2`
    SNEZ,
    /// Shift right arithmetic
    ///
    /// Arithmetic shift the value in `rs1` right by the value in the lower 5 bits of `rs2`, and store the result in `rd`.
    ///
    /// # Forms
    /// Assembly: `sra xd, xs1, xs2`
    SRA,
    /// Shift right arithmetic immediate
    ///
    /// Arithmetic shift (the original sign bit is copied into the vacated upper bits) the
    /// value in rs1 right by shamt, and store the result in rd.
    ///
    /// # Forms
    /// Assembly: `srai xd, xs1, shamt`
    SRAI,
    /// Shift right arithmetic immediate
    ///
    /// Arithmetic shift (the original sign bit is copied into the vacated upper bits) the
    /// value in rs1 right by shamt, and store the result in rd.
    ///
    /// # Forms
    /// Assembly: `srai.rv32 xd, xs1, shamt`
    SRAIRV32,
    /// Shift right arithmetic immediate word
    ///
    /// Arithmetic shift (the original sign bit is copied into the vacated upper bits) the
    /// 32-bit value in rs1 right by shamt, and store the sign-extended result in rd.
    ///
    /// # Forms
    /// Assembly: `sraiw xd, xs1, shamt`
    SRAIW,
    /// Shift right arithmetic word
    ///
    /// Arithmetic shift the 32-bit value in `rs1` right by the value in the lower 5 bits of `rs2`, and store the sign-extended result in `rd`.
    ///
    /// # Forms
    /// Assembly: `sraw xd, xs1, xs2`
    SRAW,
    /// Supervisor Exception Return
    ///
    /// Returns from an exception.
    ///
    /// When `sret` is allowed to execute, its behavior depends on whether or not the current privilege
    /// mode is virtualized.
    ///
    /// *When the current privilege mode is (H)S-mode or M-mode*
    ///
    /// `sret` sets  `hstatus.HPV` = 0, `mstatus.SPP` = 0,
    /// `mstatus.SIE` = `mstatus.SPIE`, and `mstatus.SPIE` = 1,
    /// changes the privilege mode according to the table below,
    /// and then jumps to the address in `sepc`.
    ///
    /// .Next privilege mode following an `sret` in (H)S-mode or M-mode
    /// \[%autowidth\]
    /// |===
    /// | \[.rotate\]#`mstatus.SPP`# | \[.rotate\]#`hstatus.SPV`# .&gt;| Mode after `sret`
    ///
    /// | 0 | 0 | U-mode
    /// | 0 | 1 | VU-mode
    /// | 1 | 0 | (H)S-mode
    /// | 1 | 1 | VS-mode
    /// |===
    ///
    /// *When the current privilege mode is VS-mode*
    ///
    /// `sret` sets
    /// `vsstatus.SPP` = 0, `vsstatus.SIE` = `vstatus.SPIE`, and `vsstatus.SPIE` = 1,
    /// changes the privilege mode according to the table below,
    /// and then jumps to the address in `vsepc`.
    ///
    /// .Next privilege mode following an `sret` in (H)S-mode or M-mode
    /// \[%autowidth\]
    /// |===
    /// | \[.rotate\]#`vsstatus.SPP`# .&gt;| Mode after `sret`
    ///
    /// | 0 | VU-mode
    /// | 1 | VS-mode
    /// |===
    ///
    /// # Forms
    /// Assembly: `sret ""`
    SRET,
    /// Shift right logical
    ///
    /// Logical shift the value in `rs1` right by the value in the lower bits of `rs2`, and store the result in `rd`.
    ///
    /// # Forms
    /// Assembly: `srl xd, xs1, xs2`
    SRL,
    /// Shift right logical immediate
    ///
    /// Shift the value in rs1 right by shamt, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `srli xd, xs1, shamt`
    SRLI,
    /// Shift right logical immediate
    ///
    /// Shift the value in rs1 right by shamt, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `srli.rv32 xd, xs1, shamt`
    SRLIRV32,
    /// Shift right logical immediate word
    ///
    /// Shift the 32-bit value in rs1 right by shamt, and store the sign-extended result in rd
    ///
    /// # Forms
    /// Assembly: `srliw xd, xs1, shamt`
    SRLIW,
    /// Shift right logical word
    ///
    /// Logical shift the 32-bit value in `rs1` right by the value in the lower 5 bits of `rs2`, and store the sign-extended result in `rd`.
    ///
    /// # Forms
    /// Assembly: `srlw xd, xs1, xs2`
    SRLW,
    /// RISC-V `ssamoswap.d` instruction.
    ///
    /// # Forms
    /// Assembly: `ssamoswap.d xd, xs1, xs2, aq, rl`
    SSAMOSWAPD,
    /// RISC-V `ssamoswap.w` instruction.
    ///
    /// # Forms
    /// Assembly: `ssamoswap.w xd, xs1, xs2, aq, rl`
    SSAMOSWAPW,
    /// RISC-V `sspopchk.x1` instruction.
    ///
    /// # Forms
    /// Assembly: `sspopchk.x1 sspopchk_x1`
    SSPOPCHKX1,
    /// RISC-V `sspopchk.x5` instruction.
    ///
    /// # Forms
    /// Assembly: `sspopchk.x5 sspopchk_x5`
    SSPOPCHKX5,
    /// RISC-V `sspush.x1` instruction.
    ///
    /// # Forms
    /// Assembly: `sspush.x1 sspush_x1`
    SSPUSHX1,
    /// RISC-V `sspush.x5` instruction.
    ///
    /// # Forms
    /// Assembly: `sspush.x5 sspush_x5`
    SSPUSHX5,
    /// RISC-V `ssrdp` instruction.
    ///
    /// # Forms
    /// Assembly: `ssrdp xd`
    SSRDP,
    /// Subtract
    ///
    /// Subtract the value in rs2 from rs1, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `sub xd, xs1, xs2`
    SUB,
    /// Subtract word
    ///
    /// Subtract the 32-bit values in rs2 from rs1, and store the sign-extended result in rd
    ///
    /// # Forms
    /// Assembly: `subw xd, xs1, xs2`
    SUBW,
    /// Store word
    ///
    /// Store 32 bits of data from register `rs2` to an
    /// address formed by adding `rs1` to a signed offset.
    ///
    /// # Forms
    /// Assembly: `sw xs2, imm(xs1)`
    SW,
    /// Bit deinterleave
    ///
    /// This instruction gathers bits from the high and low halves of the source word into odd/even bit
    /// positions in the destination word. It is the inverse of the zip instruction. This instruction is
    /// available only on RV32.
    ///
    /// # Forms
    /// Assembly: `unzip xd, xs1`
    UNZIP,
    /// RISC-V `vaadd.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vaadd.vv vm, vs2, vs1, vd`
    VAADDVV,
    /// RISC-V `vaadd.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vaadd.vx vm, vs2, xs1, vd`
    VAADDVX,
    /// RISC-V `vaaddu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vaaddu.vv vm, vs2, vs1, vd`
    VAADDUVV,
    /// RISC-V `vaaddu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vaaddu.vx vm, vs2, xs1, vd`
    VAADDUVX,
    /// RISC-V `vadc.vim` instruction.
    ///
    /// # Forms
    /// Assembly: `vadc.vim vs2, vd, imm`
    VADCVIM,
    /// RISC-V `vadc.vvm` instruction.
    ///
    /// # Forms
    /// Assembly: `vadc.vvm vs2, vs1, vd`
    VADCVVM,
    /// RISC-V `vadc.vxm` instruction.
    ///
    /// # Forms
    /// Assembly: `vadc.vxm vs2, xs1, vd`
    VADCVXM,
    /// RISC-V `vadd.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vadd.vi vm, vs2, vd, imm`
    VADDVI,
    /// RISC-V `vadd.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vadd.vv vm, vs2, vs1, vd`
    VADDVV,
    /// RISC-V `vadd.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vadd.vx vm, vs2, xs1, vd`
    VADDVX,
    /// RISC-V `vaesdf.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vaesdf.vs vs2, vd`
    VAESDFVS,
    /// RISC-V `vaesdf.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vaesdf.vv vs2, vd`
    VAESDFVV,
    /// RISC-V `vaesdm.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vaesdm.vs vs2, vd`
    VAESDMVS,
    /// RISC-V `vaesdm.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vaesdm.vv vs2, vd`
    VAESDMVV,
    /// RISC-V `vaesef.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vaesef.vs vs2, vd`
    VAESEFVS,
    /// RISC-V `vaesef.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vaesef.vv vs2, vd`
    VAESEFVV,
    /// RISC-V `vaesem.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vaesem.vs vs2, vd`
    VAESEMVS,
    /// RISC-V `vaesem.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vaesem.vv vs2, vd`
    VAESEMVV,
    /// RISC-V `vaeskf1.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vaeskf1.vi vs2, vd, imm`
    VAESKF1VI,
    /// RISC-V `vaeskf2.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vaeskf2.vi vs2, vd, imm`
    VAESKF2VI,
    /// Vector AES round zero
    ///
    /// # Forms
    /// Assembly: `vaesz.vs vs2, vd`
    VAESZVS,
    /// RISC-V `vand.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vand.vi vm, vs2, vd, imm`
    VANDVI,
    /// RISC-V `vand.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vand.vv vm, vs2, vs1, vd`
    VANDVV,
    /// RISC-V `vand.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vand.vx vm, vs2, xs1, vd`
    VANDVX,
    /// RISC-V `vandn.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vandn.vv vm, vs2, vs1, vd`
    VANDNVV,
    /// RISC-V `vandn.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vandn.vx vm, vs2, xs1, vd`
    VANDNVX,
    /// RISC-V `vasub.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vasub.vv vm, vs2, vs1, vd`
    VASUBVV,
    /// RISC-V `vasub.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vasub.vx vm, vs2, xs1, vd`
    VASUBVX,
    /// RISC-V `vasubu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vasubu.vv vm, vs2, vs1, vd`
    VASUBUVV,
    /// RISC-V `vasubu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vasubu.vx vm, vs2, xs1, vd`
    VASUBUVX,
    /// RISC-V `vbrev8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vbrev8.v vm, vs2, vd`
    VBREV8V,
    /// RISC-V `vbrev.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vbrev.v vm, vs2, vd`
    VBREVV,
    /// RISC-V `vclmul.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vclmul.vv vm, vs2, vs1, vd`
    VCLMULVV,
    /// RISC-V `vclmul.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vclmul.vx vm, vs2, xs1, vd`
    VCLMULVX,
    /// RISC-V `vclmulh.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vclmulh.vv vm, vs2, vs1, vd`
    VCLMULHVV,
    /// RISC-V `vclmulh.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vclmulh.vx vm, vs2, xs1, vd`
    VCLMULHVX,
    /// RISC-V `vclz.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vclz.v vm, vs2, vd`
    VCLZV,
    /// RISC-V `vcompress.vm` instruction.
    ///
    /// # Forms
    /// Assembly: `vcompress.vm vs2, vs1, vd`
    VCOMPRESSVM,
    /// RISC-V `vcpop.m` instruction.
    ///
    /// # Forms
    /// Assembly: `vcpop.m vm, vs2, xd`
    VCPOPM,
    /// RISC-V `vcpop.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vcpop.v vm, vs2, vd`
    VCPOPV,
    /// RISC-V `vctz.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vctz.v vm, vs2, vd`
    VCTZV,
    /// RISC-V `vdiv.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vdiv.vv vm, vs2, vs1, vd`
    VDIVVV,
    /// RISC-V `vdiv.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vdiv.vx vm, vs2, xs1, vd`
    VDIVVX,
    /// RISC-V `vdivu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vdivu.vv vm, vs2, vs1, vd`
    VDIVUVV,
    /// RISC-V `vdivu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vdivu.vx vm, vs2, xs1, vd`
    VDIVUVX,
    /// RISC-V `vfadd.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfadd.vf vm, vs2, xs1, vd`
    VFADDVF,
    /// RISC-V `vfadd.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfadd.vv vm, vs2, vs1, vd`
    VFADDVV,
    /// RISC-V `vfclass.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfclass.v vm, vs2, vd`
    VFCLASSV,
    /// RISC-V `vfcvt.f.x.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfcvt.f.x.v vm, vs2, vd`
    VFCVTFXV,
    /// RISC-V `vfcvt.f.xu.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfcvt.f.xu.v vm, vs2, vd`
    VFCVTFXUV,
    /// RISC-V `vfcvt.rtz.x.f.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfcvt.rtz.x.f.v vm, vs2, vd`
    VFCVTRTZXFV,
    /// RISC-V `vfcvt.rtz.xu.f.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfcvt.rtz.xu.f.v vm, vs2, vd`
    VFCVTRTZXUFV,
    /// RISC-V `vfcvt.x.f.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfcvt.x.f.v vm, vs2, vd`
    VFCVTXFV,
    /// RISC-V `vfcvt.xu.f.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfcvt.xu.f.v vm, vs2, vd`
    VFCVTXUFV,
    /// RISC-V `vfdiv.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfdiv.vf vm, vs2, xs1, vd`
    VFDIVVF,
    /// RISC-V `vfdiv.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfdiv.vv vm, vs2, vs1, vd`
    VFDIVVV,
    /// RISC-V `vfirst.m` instruction.
    ///
    /// # Forms
    /// Assembly: `vfirst.m vm, vs2, xd`
    VFIRSTM,
    /// RISC-V `vfmacc.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmacc.vf vm, vs2, xs1, vd`
    VFMACCVF,
    /// RISC-V `vfmacc.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmacc.vv vm, vs2, vs1, vd`
    VFMACCVV,
    /// RISC-V `vfmadd.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmadd.vf vm, vs2, xs1, vd`
    VFMADDVF,
    /// RISC-V `vfmadd.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmadd.vv vm, vs2, vs1, vd`
    VFMADDVV,
    /// RISC-V `vfmax.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmax.vf vm, vs2, xs1, vd`
    VFMAXVF,
    /// RISC-V `vfmax.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmax.vv vm, vs2, vs1, vd`
    VFMAXVV,
    /// RISC-V `vfmerge.vfm` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmerge.vfm vs2, xs1, vd`
    VFMERGEVFM,
    /// RISC-V `vfmin.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmin.vf vm, vs2, xs1, vd`
    VFMINVF,
    /// RISC-V `vfmin.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmin.vv vm, vs2, vs1, vd`
    VFMINVV,
    /// RISC-V `vfmsac.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmsac.vf vm, vs2, xs1, vd`
    VFMSACVF,
    /// RISC-V `vfmsac.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmsac.vv vm, vs2, vs1, vd`
    VFMSACVV,
    /// RISC-V `vfmsub.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmsub.vf vm, vs2, xs1, vd`
    VFMSUBVF,
    /// RISC-V `vfmsub.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmsub.vv vm, vs2, vs1, vd`
    VFMSUBVV,
    /// RISC-V `vfmul.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmul.vf vm, vs2, xs1, vd`
    VFMULVF,
    /// RISC-V `vfmul.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmul.vv vm, vs2, vs1, vd`
    VFMULVV,
    /// RISC-V `vfmv.f.s` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmv.f.s vs2, xd`
    VFMVFS,
    /// RISC-V `vfmv.s.f` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmv.s.f xs1, vd`
    VFMVSF,
    /// RISC-V `vfmv.v.f` instruction.
    ///
    /// # Forms
    /// Assembly: `vfmv.v.f xs1, vd`
    VFMVVF,
    /// RISC-V `vfncvt.f.f.w` instruction.
    ///
    /// # Forms
    /// Assembly: `vfncvt.f.f.w vm, vs2, vd`
    VFNCVTFFW,
    /// RISC-V `vfncvt.f.x.w` instruction.
    ///
    /// # Forms
    /// Assembly: `vfncvt.f.x.w vm, vs2, vd`
    VFNCVTFXW,
    /// RISC-V `vfncvt.f.xu.w` instruction.
    ///
    /// # Forms
    /// Assembly: `vfncvt.f.xu.w vm, vs2, vd`
    VFNCVTFXUW,
    /// RISC-V `vfncvt.rod.f.f.w` instruction.
    ///
    /// # Forms
    /// Assembly: `vfncvt.rod.f.f.w vm, vs2, vd`
    VFNCVTRODFFW,
    /// RISC-V `vfncvt.rtz.x.f.w` instruction.
    ///
    /// # Forms
    /// Assembly: `vfncvt.rtz.x.f.w vm, vs2, vd`
    VFNCVTRTZXFW,
    /// RISC-V `vfncvt.rtz.xu.f.w` instruction.
    ///
    /// # Forms
    /// Assembly: `vfncvt.rtz.xu.f.w vm, vs2, vd`
    VFNCVTRTZXUFW,
    /// RISC-V `vfncvt.x.f.w` instruction.
    ///
    /// # Forms
    /// Assembly: `vfncvt.x.f.w vm, vs2, vd`
    VFNCVTXFW,
    /// RISC-V `vfncvt.xu.f.w` instruction.
    ///
    /// # Forms
    /// Assembly: `vfncvt.xu.f.w vm, vs2, vd`
    VFNCVTXUFW,
    /// RISC-V `vfncvtbf16.f.f.w` instruction.
    ///
    /// # Forms
    /// Assembly: `vfncvtbf16.f.f.w vm, vs2, vd`
    VFNCVTBF16FFW,
    /// RISC-V `vfnmacc.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfnmacc.vf vm, vs2, xs1, vd`
    VFNMACCVF,
    /// RISC-V `vfnmacc.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfnmacc.vv vm, vs2, vs1, vd`
    VFNMACCVV,
    /// RISC-V `vfnmadd.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfnmadd.vf vm, vs2, xs1, vd`
    VFNMADDVF,
    /// RISC-V `vfnmadd.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfnmadd.vv vm, vs2, vs1, vd`
    VFNMADDVV,
    /// RISC-V `vfnmsac.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfnmsac.vf vm, vs2, xs1, vd`
    VFNMSACVF,
    /// RISC-V `vfnmsac.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfnmsac.vv vm, vs2, vs1, vd`
    VFNMSACVV,
    /// RISC-V `vfnmsub.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfnmsub.vf vm, vs2, xs1, vd`
    VFNMSUBVF,
    /// RISC-V `vfnmsub.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfnmsub.vv vm, vs2, vs1, vd`
    VFNMSUBVV,
    /// RISC-V `vfrdiv.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfrdiv.vf vm, vs2, xs1, vd`
    VFRDIVVF,
    /// RISC-V `vfrec7.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfrec7.v vm, vs2, vd`
    VFREC7V,
    /// RISC-V `vfredmax.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vfredmax.vs vm, vs2, vs1, vd`
    VFREDMAXVS,
    /// RISC-V `vfredmin.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vfredmin.vs vm, vs2, vs1, vd`
    VFREDMINVS,
    /// RISC-V `vfredosum.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vfredosum.vs vm, vs2, vs1, vd`
    VFREDOSUMVS,
    /// RISC-V `vfredsum.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vfredsum.vs vd vs1 vs2 vm`
    VFREDSUMVS,
    /// RISC-V `vfredusum.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vfredusum.vs vm, vs2, vs1, vd`
    VFREDUSUMVS,
    /// RISC-V `vfrsqrt7.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfrsqrt7.v vm, vs2, vd`
    VFRSQRT7V,
    /// RISC-V `vfrsub.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfrsub.vf vm, vs2, xs1, vd`
    VFRSUBVF,
    /// RISC-V `vfsgnj.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfsgnj.vf vm, vs2, xs1, vd`
    VFSGNJVF,
    /// RISC-V `vfsgnj.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfsgnj.vv vm, vs2, vs1, vd`
    VFSGNJVV,
    /// RISC-V `vfsgnjn.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfsgnjn.vf vm, vs2, xs1, vd`
    VFSGNJNVF,
    /// RISC-V `vfsgnjn.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfsgnjn.vv vm, vs2, vs1, vd`
    VFSGNJNVV,
    /// RISC-V `vfsgnjx.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfsgnjx.vf vm, vs2, xs1, vd`
    VFSGNJXVF,
    /// RISC-V `vfsgnjx.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfsgnjx.vv vm, vs2, vs1, vd`
    VFSGNJXVV,
    /// RISC-V `vfslide1down.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfslide1down.vf vm, vs2, xs1, vd`
    VFSLIDE1DOWNVF,
    /// RISC-V `vfslide1up.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfslide1up.vf vm, vs2, xs1, vd`
    VFSLIDE1UPVF,
    /// RISC-V `vfsqrt.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfsqrt.v vm, vs2, vd`
    VFSQRTV,
    /// RISC-V `vfsub.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfsub.vf vm, vs2, xs1, vd`
    VFSUBVF,
    /// RISC-V `vfsub.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfsub.vv vm, vs2, vs1, vd`
    VFSUBVV,
    /// RISC-V `vfwadd.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwadd.vf vm, vs2, xs1, vd`
    VFWADDVF,
    /// RISC-V `vfwadd.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwadd.vv vm, vs2, vs1, vd`
    VFWADDVV,
    /// RISC-V `vfwadd.wf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwadd.wf vm, vs2, xs1, vd`
    VFWADDWF,
    /// RISC-V `vfwadd.wv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwadd.wv vm, vs2, vs1, vd`
    VFWADDWV,
    /// RISC-V `vfwcvt.f.f.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwcvt.f.f.v vm, vs2, vd`
    VFWCVTFFV,
    /// RISC-V `vfwcvt.f.x.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwcvt.f.x.v vm, vs2, vd`
    VFWCVTFXV,
    /// RISC-V `vfwcvt.f.xu.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwcvt.f.xu.v vm, vs2, vd`
    VFWCVTFXUV,
    /// RISC-V `vfwcvt.rtz.x.f.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwcvt.rtz.x.f.v vm, vs2, vd`
    VFWCVTRTZXFV,
    /// RISC-V `vfwcvt.rtz.xu.f.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwcvt.rtz.xu.f.v vm, vs2, vd`
    VFWCVTRTZXUFV,
    /// RISC-V `vfwcvt.x.f.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwcvt.x.f.v vm, vs2, vd`
    VFWCVTXFV,
    /// RISC-V `vfwcvt.xu.f.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwcvt.xu.f.v vm, vs2, vd`
    VFWCVTXUFV,
    /// RISC-V `vfwcvtbf16.f.f.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwcvtbf16.f.f.v vm, vs2, vd`
    VFWCVTBF16FFV,
    /// RISC-V `vfwmacc.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwmacc.vf vm, vs2, xs1, vd`
    VFWMACCVF,
    /// RISC-V `vfwmacc.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwmacc.vv vm, vs2, vs1, vd`
    VFWMACCVV,
    /// RISC-V `vfwmaccbf16.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwmaccbf16.vf vm, vs2, xs1, vd`
    VFWMACCBF16VF,
    /// RISC-V `vfwmaccbf16.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwmaccbf16.vv vm, vs2, vs1, vd`
    VFWMACCBF16VV,
    /// RISC-V `vfwmsac.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwmsac.vf vm, vs2, xs1, vd`
    VFWMSACVF,
    /// RISC-V `vfwmsac.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwmsac.vv vm, vs2, vs1, vd`
    VFWMSACVV,
    /// RISC-V `vfwmul.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwmul.vf vm, vs2, xs1, vd`
    VFWMULVF,
    /// RISC-V `vfwmul.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwmul.vv vm, vs2, vs1, vd`
    VFWMULVV,
    /// RISC-V `vfwnmacc.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwnmacc.vf vm, vs2, xs1, vd`
    VFWNMACCVF,
    /// RISC-V `vfwnmacc.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwnmacc.vv vm, vs2, vs1, vd`
    VFWNMACCVV,
    /// RISC-V `vfwnmsac.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwnmsac.vf vm, vs2, xs1, vd`
    VFWNMSACVF,
    /// RISC-V `vfwnmsac.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwnmsac.vv vm, vs2, vs1, vd`
    VFWNMSACVV,
    /// RISC-V `vfwredosum.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwredosum.vs vm, vs2, vs1, vd`
    VFWREDOSUMVS,
    /// RISC-V `vfwredsum.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwredsum.vs vd vs1 vs2 vm`
    VFWREDSUMVS,
    /// RISC-V `vfwredusum.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwredusum.vs vm, vs2, vs1, vd`
    VFWREDUSUMVS,
    /// RISC-V `vfwsub.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwsub.vf vm, vs2, xs1, vd`
    VFWSUBVF,
    /// RISC-V `vfwsub.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwsub.vv vm, vs2, vs1, vd`
    VFWSUBVV,
    /// RISC-V `vfwsub.wf` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwsub.wf vm, vs2, xs1, vd`
    VFWSUBWF,
    /// RISC-V `vfwsub.wv` instruction.
    ///
    /// # Forms
    /// Assembly: `vfwsub.wv vm, vs2, vs1, vd`
    VFWSUBWV,
    /// RISC-V `vghsh.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vghsh.vv vs2, vs1, vd`
    VGHSHVV,
    /// RISC-V `vgmul.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vgmul.vv vs2, vd`
    VGMULVV,
    /// RISC-V `vid.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vid.v vm, vd`
    VIDV,
    /// RISC-V `viota.m` instruction.
    ///
    /// # Forms
    /// Assembly: `viota.m vm, vs2, vd`
    VIOTAM,
    /// RISC-V `vl1r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl1r.v vd rs1`
    VL1RV,
    /// RISC-V `vl1re16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl1re16.v xs1, vd`
    VL1RE16V,
    /// RISC-V `vl1re32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl1re32.v xs1, vd`
    VL1RE32V,
    /// RISC-V `vl1re64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl1re64.v xs1, vd`
    VL1RE64V,
    /// RISC-V `vl1re8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl1re8.v xs1, vd`
    VL1RE8V,
    /// RISC-V `vl2r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl2r.v vd rs1`
    VL2RV,
    /// RISC-V `vl2re16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl2re16.v xs1, vd`
    VL2RE16V,
    /// RISC-V `vl2re32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl2re32.v xs1, vd`
    VL2RE32V,
    /// RISC-V `vl2re64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl2re64.v xs1, vd`
    VL2RE64V,
    /// RISC-V `vl2re8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl2re8.v xs1, vd`
    VL2RE8V,
    /// RISC-V `vl4r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl4r.v vd rs1`
    VL4RV,
    /// RISC-V `vl4re16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl4re16.v xs1, vd`
    VL4RE16V,
    /// RISC-V `vl4re32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl4re32.v xs1, vd`
    VL4RE32V,
    /// RISC-V `vl4re64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl4re64.v xs1, vd`
    VL4RE64V,
    /// RISC-V `vl4re8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl4re8.v xs1, vd`
    VL4RE8V,
    /// RISC-V `vl8r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl8r.v vd rs1`
    VL8RV,
    /// RISC-V `vl8re16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl8re16.v xs1, vd`
    VL8RE16V,
    /// RISC-V `vl8re32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl8re32.v xs1, vd`
    VL8RE32V,
    /// RISC-V `vl8re64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl8re64.v xs1, vd`
    VL8RE64V,
    /// RISC-V `vl8re8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vl8re8.v xs1, vd`
    VL8RE8V,
    /// RISC-V `vle16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vle16.v vm, xs1, vd`
    VLE16V,
    /// RISC-V `vle16ff.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vle16ff.v vm, xs1, vd`
    VLE16FFV,
    /// RISC-V `vle1.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vle1.v vd rs1`
    VLE1V,
    /// RISC-V `vle32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vle32.v vm, xs1, vd`
    VLE32V,
    /// RISC-V `vle32ff.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vle32ff.v vm, xs1, vd`
    VLE32FFV,
    /// RISC-V `vle64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vle64.v vm, xs1, vd`
    VLE64V,
    /// RISC-V `vle64ff.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vle64ff.v vm, xs1, vd`
    VLE64FFV,
    /// RISC-V `vle8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vle8.v vm, xs1, vd`
    VLE8V,
    /// RISC-V `vle8ff.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vle8ff.v vm, xs1, vd`
    VLE8FFV,
    /// RISC-V `vlm.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vlm.v xs1, vd`
    VLMV,
    /// RISC-V `vloxei16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vloxei16.v vm, vs2, xs1, vd`
    VLOXEI16V,
    /// RISC-V `vloxei32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vloxei32.v vm, vs2, xs1, vd`
    VLOXEI32V,
    /// RISC-V `vloxei64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vloxei64.v vm, vs2, xs1, vd`
    VLOXEI64V,
    /// RISC-V `vloxei8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vloxei8.v vm, vs2, xs1, vd`
    VLOXEI8V,
    /// RISC-V `vlse16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vlse16.v vm, xs2, xs1, vd`
    VLSE16V,
    /// RISC-V `vlse32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vlse32.v vm, xs2, xs1, vd`
    VLSE32V,
    /// RISC-V `vlse64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vlse64.v vm, xs2, xs1, vd`
    VLSE64V,
    /// RISC-V `vlse8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vlse8.v vm, xs2, xs1, vd`
    VLSE8V,
    /// RISC-V `vluxei16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vluxei16.v vm, vs2, xs1, vd`
    VLUXEI16V,
    /// RISC-V `vluxei32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vluxei32.v vm, vs2, xs1, vd`
    VLUXEI32V,
    /// RISC-V `vluxei64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vluxei64.v vm, vs2, xs1, vd`
    VLUXEI64V,
    /// RISC-V `vluxei8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vluxei8.v vm, vs2, xs1, vd`
    VLUXEI8V,
    /// RISC-V `vmacc.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmacc.vv vm, vs2, vs1, vd`
    VMACCVV,
    /// RISC-V `vmacc.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmacc.vx vm, vs2, xs1, vd`
    VMACCVX,
    /// RISC-V `vmadc.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vmadc.vi vs2, vd, imm`
    VMADCVI,
    /// RISC-V `vmadc.vim` instruction.
    ///
    /// # Forms
    /// Assembly: `vmadc.vim vs2, vd, imm`
    VMADCVIM,
    /// RISC-V `vmadc.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmadc.vv vs2, vs1, vd`
    VMADCVV,
    /// RISC-V `vmadc.vvm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmadc.vvm vs2, vs1, vd`
    VMADCVVM,
    /// RISC-V `vmadc.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmadc.vx vs2, xs1, vd`
    VMADCVX,
    /// RISC-V `vmadc.vxm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmadc.vxm vs2, xs1, vd`
    VMADCVXM,
    /// RISC-V `vmadd.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmadd.vv vm, vs2, vs1, vd`
    VMADDVV,
    /// RISC-V `vmadd.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmadd.vx vm, vs2, xs1, vd`
    VMADDVX,
    /// RISC-V `vmand.mm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmand.mm vs2, vs1, vd`
    VMANDMM,
    /// RISC-V `vmandn.mm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmandn.mm vs2, vs1, vd`
    VMANDNMM,
    /// RISC-V `vmandnot.mm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmandnot.mm vd vs1 vs2 vm`
    VMANDNOTMM,
    /// RISC-V `vmax.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmax.vv vm, vs2, vs1, vd`
    VMAXVV,
    /// RISC-V `vmax.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmax.vx vm, vs2, xs1, vd`
    VMAXVX,
    /// RISC-V `vmaxu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmaxu.vv vm, vs2, vs1, vd`
    VMAXUVV,
    /// RISC-V `vmaxu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmaxu.vx vm, vs2, xs1, vd`
    VMAXUVX,
    /// RISC-V `vmerge.vim` instruction.
    ///
    /// # Forms
    /// Assembly: `vmerge.vim vs2, vd, imm`
    VMERGEVIM,
    /// RISC-V `vmerge.vvm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmerge.vvm vs2, vs1, vd`
    VMERGEVVM,
    /// RISC-V `vmerge.vxm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmerge.vxm vs2, xs1, vd`
    VMERGEVXM,
    /// RISC-V `vmfeq.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vmfeq.vf vm, vs2, xs1, vd`
    VMFEQVF,
    /// RISC-V `vmfeq.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmfeq.vv vm, vs2, vs1, vd`
    VMFEQVV,
    /// RISC-V `vmfge.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vmfge.vf vm, vs2, xs1, vd`
    VMFGEVF,
    /// RISC-V `vmfgt.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vmfgt.vf vm, vs2, xs1, vd`
    VMFGTVF,
    /// RISC-V `vmfle.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vmfle.vf vm, vs2, xs1, vd`
    VMFLEVF,
    /// RISC-V `vmfle.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmfle.vv vm, vs2, vs1, vd`
    VMFLEVV,
    /// RISC-V `vmflt.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vmflt.vf vm, vs2, xs1, vd`
    VMFLTVF,
    /// RISC-V `vmflt.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmflt.vv vm, vs2, vs1, vd`
    VMFLTVV,
    /// RISC-V `vmfne.vf` instruction.
    ///
    /// # Forms
    /// Assembly: `vmfne.vf vm, vs2, xs1, vd`
    VMFNEVF,
    /// RISC-V `vmfne.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmfne.vv vm, vs2, vs1, vd`
    VMFNEVV,
    /// RISC-V `vmin.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmin.vv vm, vs2, vs1, vd`
    VMINVV,
    /// RISC-V `vmin.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmin.vx vm, vs2, xs1, vd`
    VMINVX,
    /// RISC-V `vminu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vminu.vv vm, vs2, vs1, vd`
    VMINUVV,
    /// RISC-V `vminu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vminu.vx vm, vs2, xs1, vd`
    VMINUVX,
    /// RISC-V `vmnand.mm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmnand.mm vs2, vs1, vd`
    VMNANDMM,
    /// RISC-V `vmnor.mm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmnor.mm vs2, vs1, vd`
    VMNORMM,
    /// RISC-V `vmor.mm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmor.mm vs2, vs1, vd`
    VMORMM,
    /// RISC-V `vmorn.mm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmorn.mm vs2, vs1, vd`
    VMORNMM,
    /// RISC-V `vmornot.mm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmornot.mm vd vs1 vs2 vm`
    VMORNOTMM,
    /// RISC-V `vmsbc.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsbc.vv vs2, vs1, vd`
    VMSBCVV,
    /// RISC-V `vmsbc.vvm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsbc.vvm vs2, vs1, vd`
    VMSBCVVM,
    /// RISC-V `vmsbc.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsbc.vx vs2, xs1, vd`
    VMSBCVX,
    /// RISC-V `vmsbc.vxm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsbc.vxm vs2, xs1, vd`
    VMSBCVXM,
    /// RISC-V `vmsbf.m` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsbf.m vm, vs2, vd`
    VMSBFM,
    /// RISC-V `vmseq.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vmseq.vi vm, vs2, vd, imm`
    VMSEQVI,
    /// RISC-V `vmseq.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmseq.vv vm, vs2, vs1, vd`
    VMSEQVV,
    /// RISC-V `vmseq.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmseq.vx vm, vs2, xs1, vd`
    VMSEQVX,
    /// RISC-V `vmsgt.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsgt.vi vm, vs2, vd, imm`
    VMSGTVI,
    /// RISC-V `vmsgt.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsgt.vx vm, vs2, xs1, vd`
    VMSGTVX,
    /// RISC-V `vmsgtu.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsgtu.vi vm, vs2, vd, imm`
    VMSGTUVI,
    /// RISC-V `vmsgtu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsgtu.vx vm, vs2, xs1, vd`
    VMSGTUVX,
    /// RISC-V `vmsif.m` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsif.m vm, vs2, vd`
    VMSIFM,
    /// RISC-V `vmsle.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsle.vi vm, vs2, vd, imm`
    VMSLEVI,
    /// RISC-V `vmsle.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsle.vv vm, vs2, vs1, vd`
    VMSLEVV,
    /// RISC-V `vmsle.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsle.vx vm, vs2, xs1, vd`
    VMSLEVX,
    /// RISC-V `vmsleu.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsleu.vi vm, vs2, vd, imm`
    VMSLEUVI,
    /// RISC-V `vmsleu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsleu.vv vm, vs2, vs1, vd`
    VMSLEUVV,
    /// RISC-V `vmsleu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsleu.vx vm, vs2, xs1, vd`
    VMSLEUVX,
    /// RISC-V `vmslt.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmslt.vv vm, vs2, vs1, vd`
    VMSLTVV,
    /// RISC-V `vmslt.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmslt.vx vm, vs2, xs1, vd`
    VMSLTVX,
    /// RISC-V `vmsltu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsltu.vv vm, vs2, vs1, vd`
    VMSLTUVV,
    /// RISC-V `vmsltu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsltu.vx vm, vs2, xs1, vd`
    VMSLTUVX,
    /// RISC-V `vmsne.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsne.vi vm, vs2, vd, imm`
    VMSNEVI,
    /// RISC-V `vmsne.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsne.vv vm, vs2, vs1, vd`
    VMSNEVV,
    /// RISC-V `vmsne.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsne.vx vm, vs2, xs1, vd`
    VMSNEVX,
    /// RISC-V `vmsof.m` instruction.
    ///
    /// # Forms
    /// Assembly: `vmsof.m vm, vs2, vd`
    VMSOFM,
    /// RISC-V `vmul.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmul.vv vm, vs2, vs1, vd`
    VMULVV,
    /// RISC-V `vmul.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmul.vx vm, vs2, xs1, vd`
    VMULVX,
    /// RISC-V `vmulh.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmulh.vv vm, vs2, vs1, vd`
    VMULHVV,
    /// RISC-V `vmulh.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmulh.vx vm, vs2, xs1, vd`
    VMULHVX,
    /// RISC-V `vmulhsu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmulhsu.vv vm, vs2, vs1, vd`
    VMULHSUVV,
    /// RISC-V `vmulhsu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmulhsu.vx vm, vs2, xs1, vd`
    VMULHSUVX,
    /// RISC-V `vmulhu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vmulhu.vv vm, vs2, vs1, vd`
    VMULHUVV,
    /// RISC-V `vmulhu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vmulhu.vx vm, vs2, xs1, vd`
    VMULHUVX,
    /// RISC-V `vmv1r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vmv1r.v vs2, vd`
    VMV1RV,
    /// RISC-V `vmv2r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vmv2r.v vs2, vd`
    VMV2RV,
    /// RISC-V `vmv4r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vmv4r.v vs2, vd`
    VMV4RV,
    /// RISC-V `vmv8r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vmv8r.v vs2, vd`
    VMV8RV,
    /// RISC-V `vmv.s.x` instruction.
    ///
    /// # Forms
    /// Assembly: `vmv.s.x xs1, vd`
    VMVSX,
    /// RISC-V `vmv.v.i` instruction.
    ///
    /// # Forms
    /// Assembly: `vmv.v.i vd, imm`
    VMVVI,
    /// RISC-V `vmv.v.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vmv.v.v vs1, vd`
    VMVVV,
    /// RISC-V `vmv.v.x` instruction.
    ///
    /// # Forms
    /// Assembly: `vmv.v.x xs1, vd`
    VMVVX,
    /// RISC-V `vmv.x.s` instruction.
    ///
    /// # Forms
    /// Assembly: `vmv.x.s vs2, xd`
    VMVXS,
    /// RISC-V `vmxnor.mm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmxnor.mm vs2, vs1, vd`
    VMXNORMM,
    /// RISC-V `vmxor.mm` instruction.
    ///
    /// # Forms
    /// Assembly: `vmxor.mm vs2, vs1, vd`
    VMXORMM,
    /// RISC-V `vnclip.wi` instruction.
    ///
    /// # Forms
    /// Assembly: `vnclip.wi vm, vs2, vd, imm`
    VNCLIPWI,
    /// RISC-V `vnclip.wv` instruction.
    ///
    /// # Forms
    /// Assembly: `vnclip.wv vm, vs2, vs1, vd`
    VNCLIPWV,
    /// RISC-V `vnclip.wx` instruction.
    ///
    /// # Forms
    /// Assembly: `vnclip.wx vm, vs2, xs1, vd`
    VNCLIPWX,
    /// RISC-V `vnclipu.wi` instruction.
    ///
    /// # Forms
    /// Assembly: `vnclipu.wi vm, vs2, vd, imm`
    VNCLIPUWI,
    /// RISC-V `vnclipu.wv` instruction.
    ///
    /// # Forms
    /// Assembly: `vnclipu.wv vm, vs2, vs1, vd`
    VNCLIPUWV,
    /// RISC-V `vnclipu.wx` instruction.
    ///
    /// # Forms
    /// Assembly: `vnclipu.wx vm, vs2, xs1, vd`
    VNCLIPUWX,
    /// RISC-V `vnmsac.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vnmsac.vv vm, vs2, vs1, vd`
    VNMSACVV,
    /// RISC-V `vnmsac.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vnmsac.vx vm, vs2, xs1, vd`
    VNMSACVX,
    /// RISC-V `vnmsub.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vnmsub.vv vm, vs2, vs1, vd`
    VNMSUBVV,
    /// RISC-V `vnmsub.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vnmsub.vx vm, vs2, xs1, vd`
    VNMSUBVX,
    /// RISC-V `vnsra.wi` instruction.
    ///
    /// # Forms
    /// Assembly: `vnsra.wi vm, vs2, vd, imm`
    VNSRAWI,
    /// RISC-V `vnsra.wv` instruction.
    ///
    /// # Forms
    /// Assembly: `vnsra.wv vm, vs2, vs1, vd`
    VNSRAWV,
    /// RISC-V `vnsra.wx` instruction.
    ///
    /// # Forms
    /// Assembly: `vnsra.wx vm, vs2, xs1, vd`
    VNSRAWX,
    /// RISC-V `vnsrl.wi` instruction.
    ///
    /// # Forms
    /// Assembly: `vnsrl.wi vm, vs2, vd, imm`
    VNSRLWI,
    /// RISC-V `vnsrl.wv` instruction.
    ///
    /// # Forms
    /// Assembly: `vnsrl.wv vm, vs2, vs1, vd`
    VNSRLWV,
    /// RISC-V `vnsrl.wx` instruction.
    ///
    /// # Forms
    /// Assembly: `vnsrl.wx vm, vs2, xs1, vd`
    VNSRLWX,
    /// RISC-V `vor.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vor.vi vm, vs2, vd, imm`
    VORVI,
    /// RISC-V `vor.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vor.vv vm, vs2, vs1, vd`
    VORVV,
    /// RISC-V `vor.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vor.vx vm, vs2, xs1, vd`
    VORVX,
    /// RISC-V `vpopc.m` instruction.
    ///
    /// # Forms
    /// Assembly: `vpopc.m rd vs2 vm`
    VPOPCM,
    /// RISC-V `vredand.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vredand.vs vm, vs2, vs1, vd`
    VREDANDVS,
    /// RISC-V `vredmax.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vredmax.vs vm, vs2, vs1, vd`
    VREDMAXVS,
    /// RISC-V `vredmaxu.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vredmaxu.vs vm, vs2, vs1, vd`
    VREDMAXUVS,
    /// RISC-V `vredmin.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vredmin.vs vm, vs2, vs1, vd`
    VREDMINVS,
    /// RISC-V `vredminu.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vredminu.vs vm, vs2, vs1, vd`
    VREDMINUVS,
    /// RISC-V `vredor.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vredor.vs vm, vs2, vs1, vd`
    VREDORVS,
    /// RISC-V `vredsum.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vredsum.vs vm, vs2, vs1, vd`
    VREDSUMVS,
    /// RISC-V `vredxor.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vredxor.vs vm, vs2, vs1, vd`
    VREDXORVS,
    /// RISC-V `vrem.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vrem.vv vm, vs2, vs1, vd`
    VREMVV,
    /// RISC-V `vrem.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vrem.vx vm, vs2, xs1, vd`
    VREMVX,
    /// RISC-V `vremu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vremu.vv vm, vs2, vs1, vd`
    VREMUVV,
    /// RISC-V `vremu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vremu.vx vm, vs2, xs1, vd`
    VREMUVX,
    /// RISC-V `vrev8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vrev8.v vm, vs2, vd`
    VREV8V,
    /// RISC-V `vrgather.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vrgather.vi vm, vs2, vd, imm`
    VRGATHERVI,
    /// RISC-V `vrgather.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vrgather.vv vm, vs2, vs1, vd`
    VRGATHERVV,
    /// RISC-V `vrgather.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vrgather.vx vm, vs2, xs1, vd`
    VRGATHERVX,
    /// RISC-V `vrgatherei16.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vrgatherei16.vv vm, vs2, vs1, vd`
    VRGATHEREI16VV,
    /// RISC-V `vrol.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vrol.vv vm, vs2, vs1, vd`
    VROLVV,
    /// RISC-V `vrol.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vrol.vx vm, vs2, xs1, vd`
    VROLVX,
    /// RISC-V `vror.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vror.vi vm, vs2, vd, imm`
    VRORVI,
    /// RISC-V `vror.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vror.vv vm, vs2, vs1, vd`
    VRORVV,
    /// RISC-V `vror.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vror.vx vm, vs2, xs1, vd`
    VRORVX,
    /// RISC-V `vrsub.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vrsub.vi vm, vs2, vd, imm`
    VRSUBVI,
    /// RISC-V `vrsub.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vrsub.vx vm, vs2, xs1, vd`
    VRSUBVX,
    /// RISC-V `vs1r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vs1r.v xs1, vs3`
    VS1RV,
    /// RISC-V `vs2r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vs2r.v xs1, vs3`
    VS2RV,
    /// RISC-V `vs4r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vs4r.v xs1, vs3`
    VS4RV,
    /// RISC-V `vs8r.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vs8r.v xs1, vs3`
    VS8RV,
    /// RISC-V `vsadd.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vsadd.vi vm, vs2, vd, imm`
    VSADDVI,
    /// RISC-V `vsadd.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsadd.vv vm, vs2, vs1, vd`
    VSADDVV,
    /// RISC-V `vsadd.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vsadd.vx vm, vs2, xs1, vd`
    VSADDVX,
    /// RISC-V `vsaddu.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vsaddu.vi vm, vs2, vd, imm`
    VSADDUVI,
    /// RISC-V `vsaddu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsaddu.vv vm, vs2, vs1, vd`
    VSADDUVV,
    /// RISC-V `vsaddu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vsaddu.vx vm, vs2, xs1, vd`
    VSADDUVX,
    /// RISC-V `vsbc.vvm` instruction.
    ///
    /// # Forms
    /// Assembly: `vsbc.vvm vs2, vs1, vd`
    VSBCVVM,
    /// RISC-V `vsbc.vxm` instruction.
    ///
    /// # Forms
    /// Assembly: `vsbc.vxm vs2, xs1, vd`
    VSBCVXM,
    /// RISC-V `vse16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vse16.v vm, xs1, vs3`
    VSE16V,
    /// RISC-V `vse1.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vse1.v vs3 rs1`
    VSE1V,
    /// RISC-V `vse32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vse32.v vm, xs1, vs3`
    VSE32V,
    /// RISC-V `vse64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vse64.v vm, xs1, vs3`
    VSE64V,
    /// RISC-V `vse8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vse8.v vm, xs1, vs3`
    VSE8V,
    /// RISC-V `vsetivli` instruction.
    ///
    /// # Forms
    /// Assembly: `vsetivli xd, imm`
    VSETIVLI,
    /// RISC-V `vsetvl` instruction.
    ///
    /// # Forms
    /// Assembly: `vsetvl xs2, xs1, xd`
    VSETVL,
    /// RISC-V `vsetvli` instruction.
    ///
    /// # Forms
    /// Assembly: `vsetvli xs1, xd, imm`
    VSETVLI,
    /// RISC-V `vsext.vf2` instruction.
    ///
    /// # Forms
    /// Assembly: `vsext.vf2 vm, vs2, vd`
    VSEXTVF2,
    /// RISC-V `vsext.vf4` instruction.
    ///
    /// # Forms
    /// Assembly: `vsext.vf4 vm, vs2, vd`
    VSEXTVF4,
    /// RISC-V `vsext.vf8` instruction.
    ///
    /// # Forms
    /// Assembly: `vsext.vf8 vm, vs2, vd`
    VSEXTVF8,
    /// RISC-V `vsha2ch.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsha2ch.vv vs2, vs1, vd`
    VSHA2CHVV,
    /// RISC-V `vsha2cl.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsha2cl.vv vs2, vs1, vd`
    VSHA2CLVV,
    /// RISC-V `vsha2ms.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsha2ms.vv vs2, vs1, vd`
    VSHA2MSVV,
    /// RISC-V `vslide1down.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vslide1down.vx vm, vs2, xs1, vd`
    VSLIDE1DOWNVX,
    /// RISC-V `vslide1up.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vslide1up.vx vm, vs2, xs1, vd`
    VSLIDE1UPVX,
    /// RISC-V `vslidedown.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vslidedown.vi vm, vs2, vd, imm`
    VSLIDEDOWNVI,
    /// RISC-V `vslidedown.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vslidedown.vx vm, vs2, xs1, vd`
    VSLIDEDOWNVX,
    /// RISC-V `vslideup.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vslideup.vi vm, vs2, vd, imm`
    VSLIDEUPVI,
    /// RISC-V `vslideup.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vslideup.vx vm, vs2, xs1, vd`
    VSLIDEUPVX,
    /// RISC-V `vsll.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vsll.vi vm, vs2, vd, imm`
    VSLLVI,
    /// RISC-V `vsll.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsll.vv vm, vs2, vs1, vd`
    VSLLVV,
    /// RISC-V `vsll.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vsll.vx vm, vs2, xs1, vd`
    VSLLVX,
    /// RISC-V `vsm3c.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vsm3c.vi vs2, vd, imm`
    VSM3CVI,
    /// RISC-V `vsm3me.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsm3me.vv vs2, vs1, vd`
    VSM3MEVV,
    /// RISC-V `vsm4k.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vsm4k.vi vs2, vd, imm`
    VSM4KVI,
    /// RISC-V `vsm4r.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vsm4r.vs vs2, vd`
    VSM4RVS,
    /// RISC-V `vsm4r.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsm4r.vv vs2, vd`
    VSM4RVV,
    /// RISC-V `vsm.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsm.v xs1, vs3`
    VSMV,
    /// RISC-V `vsmul.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsmul.vv vm, vs2, vs1, vd`
    VSMULVV,
    /// RISC-V `vsmul.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vsmul.vx vm, vs2, xs1, vd`
    VSMULVX,
    /// RISC-V `vsoxei16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsoxei16.v vm, vs2, xs1, vs3`
    VSOXEI16V,
    /// RISC-V `vsoxei32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsoxei32.v vm, vs2, xs1, vs3`
    VSOXEI32V,
    /// RISC-V `vsoxei64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsoxei64.v vm, vs2, xs1, vs3`
    VSOXEI64V,
    /// RISC-V `vsoxei8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsoxei8.v vm, vs2, xs1, vs3`
    VSOXEI8V,
    /// RISC-V `vsra.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vsra.vi vm, vs2, vd, imm`
    VSRAVI,
    /// RISC-V `vsra.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsra.vv vm, vs2, vs1, vd`
    VSRAVV,
    /// RISC-V `vsra.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vsra.vx vm, vs2, xs1, vd`
    VSRAVX,
    /// RISC-V `vsrl.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vsrl.vi vm, vs2, vd, imm`
    VSRLVI,
    /// RISC-V `vsrl.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsrl.vv vm, vs2, vs1, vd`
    VSRLVV,
    /// RISC-V `vsrl.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vsrl.vx vm, vs2, xs1, vd`
    VSRLVX,
    /// RISC-V `vsse16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsse16.v vm, xs2, xs1, vs3`
    VSSE16V,
    /// RISC-V `vsse32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsse32.v vm, xs2, xs1, vs3`
    VSSE32V,
    /// RISC-V `vsse64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsse64.v vm, xs2, xs1, vs3`
    VSSE64V,
    /// RISC-V `vsse8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsse8.v vm, xs2, xs1, vs3`
    VSSE8V,
    /// RISC-V `vssra.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vssra.vi vm, vs2, vd, imm`
    VSSRAVI,
    /// RISC-V `vssra.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vssra.vv vm, vs2, vs1, vd`
    VSSRAVV,
    /// RISC-V `vssra.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vssra.vx vm, vs2, xs1, vd`
    VSSRAVX,
    /// RISC-V `vssrl.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vssrl.vi vm, vs2, vd, imm`
    VSSRLVI,
    /// RISC-V `vssrl.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vssrl.vv vm, vs2, vs1, vd`
    VSSRLVV,
    /// RISC-V `vssrl.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vssrl.vx vm, vs2, xs1, vd`
    VSSRLVX,
    /// RISC-V `vssub.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vssub.vv vm, vs2, vs1, vd`
    VSSUBVV,
    /// RISC-V `vssub.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vssub.vx vm, vs2, xs1, vd`
    VSSUBVX,
    /// RISC-V `vssubu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vssubu.vv vm, vs2, vs1, vd`
    VSSUBUVV,
    /// RISC-V `vssubu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vssubu.vx vm, vs2, xs1, vd`
    VSSUBUVX,
    /// RISC-V `vsub.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vsub.vv vm, vs2, vs1, vd`
    VSUBVV,
    /// RISC-V `vsub.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vsub.vx vm, vs2, xs1, vd`
    VSUBVX,
    /// RISC-V `vsuxei16.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsuxei16.v vm, vs2, xs1, vs3`
    VSUXEI16V,
    /// RISC-V `vsuxei32.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsuxei32.v vm, vs2, xs1, vs3`
    VSUXEI32V,
    /// RISC-V `vsuxei64.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsuxei64.v vm, vs2, xs1, vs3`
    VSUXEI64V,
    /// RISC-V `vsuxei8.v` instruction.
    ///
    /// # Forms
    /// Assembly: `vsuxei8.v vm, vs2, xs1, vs3`
    VSUXEI8V,
    /// RISC-V `vwadd.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwadd.vv vm, vs2, vs1, vd`
    VWADDVV,
    /// RISC-V `vwadd.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwadd.vx vm, vs2, xs1, vd`
    VWADDVX,
    /// RISC-V `vwadd.wv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwadd.wv vm, vs2, vs1, vd`
    VWADDWV,
    /// RISC-V `vwadd.wx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwadd.wx vm, vs2, xs1, vd`
    VWADDWX,
    /// RISC-V `vwaddu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwaddu.vv vm, vs2, vs1, vd`
    VWADDUVV,
    /// RISC-V `vwaddu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwaddu.vx vm, vs2, xs1, vd`
    VWADDUVX,
    /// RISC-V `vwaddu.wv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwaddu.wv vm, vs2, vs1, vd`
    VWADDUWV,
    /// RISC-V `vwaddu.wx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwaddu.wx vm, vs2, xs1, vd`
    VWADDUWX,
    /// RISC-V `vwmacc.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmacc.vv vm, vs2, vs1, vd`
    VWMACCVV,
    /// RISC-V `vwmacc.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmacc.vx vm, vs2, xs1, vd`
    VWMACCVX,
    /// RISC-V `vwmaccsu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmaccsu.vv vm, vs2, vs1, vd`
    VWMACCSUVV,
    /// RISC-V `vwmaccsu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmaccsu.vx vm, vs2, xs1, vd`
    VWMACCSUVX,
    /// RISC-V `vwmaccu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmaccu.vv vm, vs2, vs1, vd`
    VWMACCUVV,
    /// RISC-V `vwmaccu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmaccu.vx vm, vs2, xs1, vd`
    VWMACCUVX,
    /// RISC-V `vwmaccus.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmaccus.vx vm, vs2, xs1, vd`
    VWMACCUSVX,
    /// RISC-V `vwmul.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmul.vv vm, vs2, vs1, vd`
    VWMULVV,
    /// RISC-V `vwmul.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmul.vx vm, vs2, xs1, vd`
    VWMULVX,
    /// RISC-V `vwmulsu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmulsu.vv vm, vs2, vs1, vd`
    VWMULSUVV,
    /// RISC-V `vwmulsu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmulsu.vx vm, vs2, xs1, vd`
    VWMULSUVX,
    /// RISC-V `vwmulu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmulu.vv vm, vs2, vs1, vd`
    VWMULUVV,
    /// RISC-V `vwmulu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwmulu.vx vm, vs2, xs1, vd`
    VWMULUVX,
    /// RISC-V `vwredsum.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vwredsum.vs vm, vs2, vs1, vd`
    VWREDSUMVS,
    /// RISC-V `vwredsumu.vs` instruction.
    ///
    /// # Forms
    /// Assembly: `vwredsumu.vs vm, vs2, vs1, vd`
    VWREDSUMUVS,
    /// RISC-V `vwsll.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vwsll.vi vm, vs2, vd, imm`
    VWSLLVI,
    /// RISC-V `vwsll.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwsll.vv vm, vs2, vs1, vd`
    VWSLLVV,
    /// RISC-V `vwsll.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwsll.vx vm, vs2, xs1, vd`
    VWSLLVX,
    /// RISC-V `vwsub.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwsub.vv vm, vs2, vs1, vd`
    VWSUBVV,
    /// RISC-V `vwsub.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwsub.vx vm, vs2, xs1, vd`
    VWSUBVX,
    /// RISC-V `vwsub.wv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwsub.wv vm, vs2, vs1, vd`
    VWSUBWV,
    /// RISC-V `vwsub.wx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwsub.wx vm, vs2, xs1, vd`
    VWSUBWX,
    /// RISC-V `vwsubu.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwsubu.vv vm, vs2, vs1, vd`
    VWSUBUVV,
    /// RISC-V `vwsubu.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwsubu.vx vm, vs2, xs1, vd`
    VWSUBUVX,
    /// RISC-V `vwsubu.wv` instruction.
    ///
    /// # Forms
    /// Assembly: `vwsubu.wv vm, vs2, vs1, vd`
    VWSUBUWV,
    /// RISC-V `vwsubu.wx` instruction.
    ///
    /// # Forms
    /// Assembly: `vwsubu.wx vm, vs2, xs1, vd`
    VWSUBUWX,
    /// RISC-V `vxor.vi` instruction.
    ///
    /// # Forms
    /// Assembly: `vxor.vi vm, vs2, vd, imm`
    VXORVI,
    /// RISC-V `vxor.vv` instruction.
    ///
    /// # Forms
    /// Assembly: `vxor.vv vm, vs2, vs1, vd`
    VXORVV,
    /// RISC-V `vxor.vx` instruction.
    ///
    /// # Forms
    /// Assembly: `vxor.vx vm, vs2, xs1, vd`
    VXORVX,
    /// RISC-V `vzext.vf2` instruction.
    ///
    /// # Forms
    /// Assembly: `vzext.vf2 vm, vs2, vd`
    VZEXTVF2,
    /// RISC-V `vzext.vf4` instruction.
    ///
    /// # Forms
    /// Assembly: `vzext.vf4 vm, vs2, vd`
    VZEXTVF4,
    /// RISC-V `vzext.vf8` instruction.
    ///
    /// # Forms
    /// Assembly: `vzext.vf8 vm, vs2, vd`
    VZEXTVF8,
    /// Wait for interrupt
    ///
    /// Can causes the processor to enter a low-power state until the next interrupt occurs.
    ///
    /// &lt;%- if ext?(:H) -%&gt;
    /// The behavior of `wfi` is affected by the `mstatus.TW`
    /// and `hstatus.VTW` bits, as summarized below.
    ///
    /// \[%autowidth,%footer\]
    /// |===
    /// .2+| \[.rotate\]#`mstatus.TW`# .2+| \[.rotate\]#`hstatus.VTW`# 4+^.&gt;| `wfi` behavior
    /// h| HS-mode h| U-mode h| VS-mode h| in VU-mode
    ///
    /// | 0 | 0 | Wait | Trap (I) | Wait | Trap (V)
    /// | 0 | 1 | Wait | Trap (I) | Trap (V) | Trap (V)
    /// | 1 | - | Trap (I) | Trap (I) | Trap (I) | Trap (I)
    ///
    /// 6+| Trap (I) - Trap with `Illegal Instruction` code +
    /// Trap (V) - Trap with `Virtual Instruction` code
    /// |===
    ///
    /// &lt;%- else -%&gt;
    /// The `wfi` instruction is also affected by `mstatus.TW`, as shown below:
    ///
    /// \[%autowidth,%footer\]
    /// |===
    /// .2+| \[.rotate\]#`mstatus.TW`# 2+^.&gt;| `wfi` behavior
    /// h| S-mode h| U-mode
    ///
    /// | 0 | Wait | Trap (I)
    /// | 1 | Trap (I) | Trap (I)
    ///
    /// 3+| Trap (I) - Trap with `Illegal Instruction` code
    /// |===
    ///
    /// &lt;%- end -%&gt;
    ///
    /// When `wfi` is marked as causing a trap above, the implementation is allowed to wait
    /// for an unspecified period of time to see if an interrupt occurs before raising the trap.
    /// That period of time can be zero (_i.e._, `wfi` always causes a trap in the cases identified
    /// above).
    ///
    /// # Forms
    /// Assembly: `wfi ""`
    WFI,
    /// RISC-V `wrs.nto` instruction.
    ///
    /// # Forms
    /// Assembly: `wrs.nto wrs_nto`
    WRSNTO,
    /// RISC-V `wrs.sto` instruction.
    ///
    /// # Forms
    /// Assembly: `wrs.sto wrs_sto`
    WRSSTO,
    /// Exclusive NOR
    ///
    /// This instruction performs the bit-wise exclusive-NOR operation on rs1 and rs2.
    ///
    /// # Forms
    /// Assembly: `xnor xd, xs1, xs2`
    XNOR,
    /// Exclusive Or
    ///
    /// Exclusive or rs1 with rs2, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `xor xd, xs1, xs2`
    XOR,
    /// Exclusive Or immediate
    ///
    /// Exclusive or an immediate to the value in rs1, and store the result in rd
    ///
    /// # Forms
    /// Assembly: `xori xd, xs1, imm`
    XORI,
    /// Crossbar permutation (nibbles)
    ///
    /// The xperm4 instruction operates on nibbles. The rs1 register contains a vector of XLEN/4 4-bit
    /// elements. The rs2 register contains a vector of XLEN/4 4-bit indexes. The result is each element in
    /// rs2 replaced by the indexed element in rs1, or zero if the index into rs2 is out of bounds.
    ///
    /// # Forms
    /// Assembly: `xperm4 xd, xs1, xs2`
    XPERM4,
    /// Crossbar permutation (bytes)
    ///
    /// The xperm8 instruction operates on bytes. The rs1 register contains a vector of XLEN/8 8-bit
    /// elements. The rs2 register contains a vector of XLEN/8 8-bit indexes. The result is each element in
    /// rs2 replaced by the indexed element in rs1, or zero if the index into rs2 is out of bounds.
    ///
    /// # Forms
    /// Assembly: `xperm8 xd, xs1, xs2`
    XPERM8,
    /// RISC-V `zext.b` instruction.
    ///
    /// # Forms
    /// Assembly: `zext.b rd rs1`
    ZEXTB,
    /// Zero-extend halfword
    ///
    /// This instruction zero-extends the least-significant halfword of the source to XLEN by inserting
    /// 0's into all of the bits more significant than 15.
    ///
    /// \[NOTE\]
    /// The *zext.h* instruction is a pseudo-op for `pack` when `Zbkb` is implemented and XLEN == 32.
    ///
    /// \[NOTE\]
    /// The *zext.h* instruction is a pseudo-op for `packw` when `Zbkb` is implemented and XLEN == 64.
    ///
    /// # Forms
    /// Assembly: `zext.h xd, xs1`
    ZEXTH,
    /// Zero-extend halfword
    ///
    /// This instruction zero-extends the least-significant halfword of the source to XLEN by inserting
    /// 0's into all of the bits more significant than 15.
    ///
    /// \[NOTE\]
    /// The *zext.h* instruction is a pseudo-op for `pack` when `Zbkb` is implemented and XLEN == 32.
    ///
    /// \[NOTE\]
    /// The *zext.h* instruction is a pseudo-op for `packw` when `Zbkb` is implemented and XLEN == 64.
    ///
    /// # Forms
    /// Assembly: `zext.h.rv32 xd, xs1`
    ZEXTHRV32,
    /// RISC-V `zext.w` instruction.
    ///
    /// # Forms
    /// Assembly: `zext.w rd rs1`
    ZEXTW,
    /// Bit interleave
    ///
    /// This instruction scatters all of the odd and even bits of a source word into the high and low halves
    /// of a destination word. It is the inverse of the unzip instruction. This instruction is available only on
    /// RV32.
    ///
    /// # Forms
    /// Assembly: `zip xd, xs1`
    ZIP,
    Invalid,
}

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
    "c.sspopchk.x5",
    "c.sspush.x1",
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
    "fcvt.bf16.s",
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
    "fcvt.s.bf16",
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
    "lpad",
    "lr.d",
    "lr.w",
    "lui",
    "lw",
    "lwu",
    "max",
    "maxu",
    "min",
    "minu",
    "mnret",
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
    "sctrclr",
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
    "ssamoswap.d",
    "ssamoswap.w",
    "sspopchk.x1",
    "sspopchk.x5",
    "sspush.x1",
    "sspush.x5",
    "ssrdp",
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
    "vfncvtbf16.f.f.w",
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
    "vfwcvtbf16.f.f.v",
    "vfwmacc.vf",
    "vfwmacc.vv",
    "vfwmaccbf16.vf",
    "vfwmaccbf16.vv",
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
    "<invalid>",
];

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Inst {
    value: u32,
}

impl Inst {
    pub const fn encode(&self) -> InstructionValue {
        InstructionValue::new(self.value)
    }

    pub const fn new(op: Opcode) -> Self {
        match op {
            Opcode::Invalid => unreachable!(),
            Opcode::ADD => Inst { value: 0x33 },
            Opcode::ADDUW => Inst { value: 0x800003b },
            Opcode::ADDI => Inst { value: 0x13 },
            Opcode::ADDIW => Inst { value: 0x1b },
            Opcode::ADDW => Inst { value: 0x3b },
            Opcode::AES32DSI => Inst { value: 0x2a000033 },
            Opcode::AES32DSMI => Inst { value: 0x2e000033 },
            Opcode::AES32ESI => Inst { value: 0x22000033 },
            Opcode::AES32ESMI => Inst { value: 0x26000033 },
            Opcode::AES64DS => Inst { value: 0x3a000033 },
            Opcode::AES64DSM => Inst { value: 0x3e000033 },
            Opcode::AES64ES => Inst { value: 0x32000033 },
            Opcode::AES64ESM => Inst { value: 0x36000033 },
            Opcode::AES64IM => Inst { value: 0x30001013 },
            Opcode::AES64KS1I => Inst { value: 0x31001013 },
            Opcode::AES64KS2 => Inst { value: 0x7e000033 },
            Opcode::AMOADDB => Inst { value: 0x2f },
            Opcode::AMOADDD => Inst { value: 0x302f },
            Opcode::AMOADDH => Inst { value: 0x102f },
            Opcode::AMOADDW => Inst { value: 0x202f },
            Opcode::AMOANDB => Inst { value: 0x6000002f },
            Opcode::AMOANDD => Inst { value: 0x6000302f },
            Opcode::AMOANDH => Inst { value: 0x6000102f },
            Opcode::AMOANDW => Inst { value: 0x6000202f },
            Opcode::AMOCASB => Inst { value: 0x2800002f },
            Opcode::AMOCASD => Inst { value: 0x2800302f },
            Opcode::AMOCASH => Inst { value: 0x2800102f },
            Opcode::AMOCASQ => Inst { value: 0x2800402f },
            Opcode::AMOCASW => Inst { value: 0x2800202f },
            Opcode::AMOMAXB => Inst { value: 0xa000002f },
            Opcode::AMOMAXD => Inst { value: 0xa000302f },
            Opcode::AMOMAXH => Inst { value: 0xa000102f },
            Opcode::AMOMAXW => Inst { value: 0xa000202f },
            Opcode::AMOMAXUB => Inst { value: 0xe000002f },
            Opcode::AMOMAXUD => Inst { value: 0xe000302f },
            Opcode::AMOMAXUH => Inst { value: 0xe000102f },
            Opcode::AMOMAXUW => Inst { value: 0xe000202f },
            Opcode::AMOMINB => Inst { value: 0x8000002f },
            Opcode::AMOMIND => Inst { value: 0x8000302f },
            Opcode::AMOMINH => Inst { value: 0x8000102f },
            Opcode::AMOMINW => Inst { value: 0x8000202f },
            Opcode::AMOMINUB => Inst { value: 0xc000002f },
            Opcode::AMOMINUD => Inst { value: 0xc000302f },
            Opcode::AMOMINUH => Inst { value: 0xc000102f },
            Opcode::AMOMINUW => Inst { value: 0xc000202f },
            Opcode::AMOORB => Inst { value: 0x4000002f },
            Opcode::AMOORD => Inst { value: 0x4000302f },
            Opcode::AMOORH => Inst { value: 0x4000102f },
            Opcode::AMOORW => Inst { value: 0x4000202f },
            Opcode::AMOSWAPB => Inst { value: 0x800002f },
            Opcode::AMOSWAPD => Inst { value: 0x800302f },
            Opcode::AMOSWAPH => Inst { value: 0x800102f },
            Opcode::AMOSWAPW => Inst { value: 0x800202f },
            Opcode::AMOXORB => Inst { value: 0x2000002f },
            Opcode::AMOXORD => Inst { value: 0x2000302f },
            Opcode::AMOXORH => Inst { value: 0x2000102f },
            Opcode::AMOXORW => Inst { value: 0x2000202f },
            Opcode::AND => Inst { value: 0x7033 },
            Opcode::ANDI => Inst { value: 0x7013 },
            Opcode::ANDN => Inst { value: 0x40007033 },
            Opcode::AUIPC => Inst { value: 0x17 },
            Opcode::BCLR => Inst { value: 0x48001033 },
            Opcode::BCLRI => Inst { value: 0x48001013 },
            Opcode::BCLRIRV32 => Inst { value: 0x48001013 },
            Opcode::BEQ => Inst { value: 0x63 },
            Opcode::BEQZ => Inst { value: 0x63 },
            Opcode::BEXT => Inst { value: 0x48005033 },
            Opcode::BEXTI => Inst { value: 0x48005013 },
            Opcode::BEXTIRV32 => Inst { value: 0x48005013 },
            Opcode::BGE => Inst { value: 0x5063 },
            Opcode::BGEU => Inst { value: 0x7063 },
            Opcode::BGEZ => Inst { value: 0x5063 },
            Opcode::BGT => Inst { value: 0x4063 },
            Opcode::BGTU => Inst { value: 0x6063 },
            Opcode::BGTZ => Inst { value: 0x4063 },
            Opcode::BINV => Inst { value: 0x68001033 },
            Opcode::BINVI => Inst { value: 0x68001013 },
            Opcode::BINVIRV32 => Inst { value: 0x68001013 },
            Opcode::BLE => Inst { value: 0x5063 },
            Opcode::BLEU => Inst { value: 0x7063 },
            Opcode::BLEZ => Inst { value: 0x5063 },
            Opcode::BLT => Inst { value: 0x4063 },
            Opcode::BLTU => Inst { value: 0x6063 },
            Opcode::BLTZ => Inst { value: 0x4063 },
            Opcode::BNE => Inst { value: 0x1063 },
            Opcode::BNEZ => Inst { value: 0x1063 },
            Opcode::BREV8 => Inst { value: 0x68705013 },
            Opcode::BSET => Inst { value: 0x28001033 },
            Opcode::BSETI => Inst { value: 0x28001013 },
            Opcode::BSETIRV32 => Inst { value: 0x28001013 },
            Opcode::CADD => Inst { value: 0x9002 },
            Opcode::CADDI => Inst { value: 0x1 },
            Opcode::CADDI16SP => Inst { value: 0x6101 },
            Opcode::CADDI4SPN => Inst { value: 0x0 },
            Opcode::CADDIW => Inst { value: 0x2001 },
            Opcode::CADDW => Inst { value: 0x9c21 },
            Opcode::CAND => Inst { value: 0x8c61 },
            Opcode::CANDI => Inst { value: 0x8801 },
            Opcode::CBEQZ => Inst { value: 0xc001 },
            Opcode::CBNEZ => Inst { value: 0xe001 },
            Opcode::CEBREAK => Inst { value: 0x9002 },
            Opcode::CFLD => Inst { value: 0x2000 },
            Opcode::CFLDSP => Inst { value: 0x2002 },
            Opcode::CFLW => Inst { value: 0x6000 },
            Opcode::CFLWSP => Inst { value: 0x6002 },
            Opcode::CFSD => Inst { value: 0xa000 },
            Opcode::CFSDSP => Inst { value: 0xa002 },
            Opcode::CFSW => Inst { value: 0xe000 },
            Opcode::CFSWSP => Inst { value: 0xe002 },
            Opcode::CJ => Inst { value: 0xa001 },
            Opcode::CJAL => Inst { value: 0x2001 },
            Opcode::CJALR => Inst { value: 0x9002 },
            Opcode::CJR => Inst { value: 0x8002 },
            Opcode::CLBU => Inst { value: 0x8000 },
            Opcode::CLD => Inst { value: 0x6000 },
            Opcode::CLDSP => Inst { value: 0x6002 },
            Opcode::CLH => Inst { value: 0x8440 },
            Opcode::CLHU => Inst { value: 0x8400 },
            Opcode::CLI => Inst { value: 0x4001 },
            Opcode::CLUI => Inst { value: 0x6001 },
            Opcode::CLW => Inst { value: 0x4000 },
            Opcode::CLWSP => Inst { value: 0x4002 },
            Opcode::CMOP1 => Inst { value: 0x6081 },
            Opcode::CMOP11 => Inst { value: 0x6581 },
            Opcode::CMOP13 => Inst { value: 0x6681 },
            Opcode::CMOP15 => Inst { value: 0x6781 },
            Opcode::CMOP3 => Inst { value: 0x6181 },
            Opcode::CMOP5 => Inst { value: 0x6281 },
            Opcode::CMOP7 => Inst { value: 0x6381 },
            Opcode::CMOP9 => Inst { value: 0x6481 },
            Opcode::CMOPN => Inst { value: 0x6081 },
            Opcode::CMUL => Inst { value: 0x9c41 },
            Opcode::CMV => Inst { value: 0x8002 },
            Opcode::CNOP => Inst { value: 0x1 },
            Opcode::CNOT => Inst { value: 0x9c75 },
            Opcode::CNTLALL => Inst { value: 0x9016 },
            Opcode::CNTLP1 => Inst { value: 0x900a },
            Opcode::CNTLPALL => Inst { value: 0x900e },
            Opcode::CNTLS1 => Inst { value: 0x9012 },
            Opcode::COR => Inst { value: 0x8c41 },
            Opcode::CSB => Inst { value: 0x8800 },
            Opcode::CSD => Inst { value: 0xe000 },
            Opcode::CSDSP => Inst { value: 0xe002 },
            Opcode::CSEXTB => Inst { value: 0x9c65 },
            Opcode::CSEXTH => Inst { value: 0x9c6d },
            Opcode::CSEXTW => Inst { value: 0x2001 },
            Opcode::CSH => Inst { value: 0x8c00 },
            Opcode::CSLLI => Inst { value: 0x2 },
            Opcode::CSLLIRV32 => Inst { value: 0x2 },
            Opcode::CSRAI => Inst { value: 0x8401 },
            Opcode::CSRAIRV32 => Inst { value: 0x8401 },
            Opcode::CSRLI => Inst { value: 0x8001 },
            Opcode::CSRLIRV32 => Inst { value: 0x8001 },
            Opcode::CSSPOPCHKX5 => Inst { value: 0x6281 },
            Opcode::CSSPUSHX1 => Inst { value: 0x6081 },
            Opcode::CSUB => Inst { value: 0x8c01 },
            Opcode::CSUBW => Inst { value: 0x9c01 },
            Opcode::CSW => Inst { value: 0xc000 },
            Opcode::CSWSP => Inst { value: 0xc002 },
            Opcode::CXOR => Inst { value: 0x8c21 },
            Opcode::CZEXTB => Inst { value: 0x9c61 },
            Opcode::CZEXTH => Inst { value: 0x9c69 },
            Opcode::CZEXTW => Inst { value: 0x9c71 },
            Opcode::CBOCLEAN => Inst { value: 0x10200f },
            Opcode::CBOFLUSH => Inst { value: 0x20200f },
            Opcode::CBOINVAL => Inst { value: 0x200f },
            Opcode::CBOZERO => Inst { value: 0x40200f },
            Opcode::CLMUL => Inst { value: 0xa001033 },
            Opcode::CLMULH => Inst { value: 0xa003033 },
            Opcode::CLMULR => Inst { value: 0xa002033 },
            Opcode::CLZ => Inst { value: 0x60001013 },
            Opcode::CLZW => Inst { value: 0x6000101b },
            Opcode::CMJALT => Inst { value: 0xa002 },
            Opcode::CMMVA01S => Inst { value: 0xac62 },
            Opcode::CMMVSA01 => Inst { value: 0xac22 },
            Opcode::CMPOP => Inst { value: 0xba02 },
            Opcode::CMPOPRET => Inst { value: 0xbe02 },
            Opcode::CMPOPRETZ => Inst { value: 0xbc02 },
            Opcode::CMPUSH => Inst { value: 0xb802 },
            Opcode::CPOP => Inst { value: 0x60201013 },
            Opcode::CPOPW => Inst { value: 0x6020101b },
            Opcode::CSRC => Inst { value: 0x3073 },
            Opcode::CSRCI => Inst { value: 0x7073 },
            Opcode::CSRR => Inst { value: 0x2073 },
            Opcode::CSRRC => Inst { value: 0x3073 },
            Opcode::CSRRCI => Inst { value: 0x7073 },
            Opcode::CSRRS => Inst { value: 0x2073 },
            Opcode::CSRRSI => Inst { value: 0x6073 },
            Opcode::CSRRW => Inst { value: 0x1073 },
            Opcode::CSRRWI => Inst { value: 0x5073 },
            Opcode::CSRS => Inst { value: 0x2073 },
            Opcode::CSRSI => Inst { value: 0x6073 },
            Opcode::CSRW => Inst { value: 0x1073 },
            Opcode::CSRWI => Inst { value: 0x5073 },
            Opcode::CTZ => Inst { value: 0x60101013 },
            Opcode::CTZW => Inst { value: 0x6010101b },
            Opcode::CZEROEQZ => Inst { value: 0xe005033 },
            Opcode::CZERONEZ => Inst { value: 0xe007033 },
            Opcode::DIV => Inst { value: 0x2004033 },
            Opcode::DIVU => Inst { value: 0x2005033 },
            Opcode::DIVUW => Inst { value: 0x200503b },
            Opcode::DIVW => Inst { value: 0x200403b },
            Opcode::DRET => Inst { value: 0x7b200073 },
            Opcode::EBREAK => Inst { value: 0x100073 },
            Opcode::ECALL => Inst { value: 0x73 },
            Opcode::FABSD => Inst { value: 0x22002053 },
            Opcode::FABSH => Inst { value: 0x24002053 },
            Opcode::FABSQ => Inst { value: 0x26002053 },
            Opcode::FABSS => Inst { value: 0x20002053 },
            Opcode::FADDD => Inst { value: 0x2000053 },
            Opcode::FADDH => Inst { value: 0x4000053 },
            Opcode::FADDQ => Inst { value: 0x6000053 },
            Opcode::FADDS => Inst { value: 0x53 },
            Opcode::FCLASSD => Inst { value: 0xe2001053 },
            Opcode::FCLASSH => Inst { value: 0xe4001053 },
            Opcode::FCLASSQ => Inst { value: 0xe6001053 },
            Opcode::FCLASSS => Inst { value: 0xe0001053 },
            Opcode::FCVTBF16S => Inst { value: 0x44800053 },
            Opcode::FCVTDH => Inst { value: 0x42200053 },
            Opcode::FCVTDL => Inst { value: 0xd2200053 },
            Opcode::FCVTDLU => Inst { value: 0xd2300053 },
            Opcode::FCVTDQ => Inst { value: 0x42300053 },
            Opcode::FCVTDS => Inst { value: 0x42000053 },
            Opcode::FCVTDW => Inst { value: 0xd2000053 },
            Opcode::FCVTDWU => Inst { value: 0xd2100053 },
            Opcode::FCVTHD => Inst { value: 0x44100053 },
            Opcode::FCVTHL => Inst { value: 0xd4200053 },
            Opcode::FCVTHLU => Inst { value: 0xd4300053 },
            Opcode::FCVTHQ => Inst { value: 0x44300053 },
            Opcode::FCVTHS => Inst { value: 0x44000053 },
            Opcode::FCVTHW => Inst { value: 0xd4000053 },
            Opcode::FCVTHWU => Inst { value: 0xd4100053 },
            Opcode::FCVTLD => Inst { value: 0xc2200053 },
            Opcode::FCVTLH => Inst { value: 0xc4200053 },
            Opcode::FCVTLQ => Inst { value: 0xc6200053 },
            Opcode::FCVTLS => Inst { value: 0xc0200053 },
            Opcode::FCVTLUD => Inst { value: 0xc2300053 },
            Opcode::FCVTLUH => Inst { value: 0xc4300053 },
            Opcode::FCVTLUQ => Inst { value: 0xc6300053 },
            Opcode::FCVTLUS => Inst { value: 0xc0300053 },
            Opcode::FCVTQD => Inst { value: 0x46100053 },
            Opcode::FCVTQH => Inst { value: 0x46200053 },
            Opcode::FCVTQL => Inst { value: 0xd6200053 },
            Opcode::FCVTQLU => Inst { value: 0xd6300053 },
            Opcode::FCVTQS => Inst { value: 0x46000053 },
            Opcode::FCVTQW => Inst { value: 0xd6000053 },
            Opcode::FCVTQWU => Inst { value: 0xd6100053 },
            Opcode::FCVTSBF16 => Inst { value: 0x40600053 },
            Opcode::FCVTSD => Inst { value: 0x40100053 },
            Opcode::FCVTSH => Inst { value: 0x40200053 },
            Opcode::FCVTSL => Inst { value: 0xd0200053 },
            Opcode::FCVTSLU => Inst { value: 0xd0300053 },
            Opcode::FCVTSQ => Inst { value: 0x40300053 },
            Opcode::FCVTSW => Inst { value: 0xd0000053 },
            Opcode::FCVTSWU => Inst { value: 0xd0100053 },
            Opcode::FCVTWD => Inst { value: 0xc2000053 },
            Opcode::FCVTWH => Inst { value: 0xc4000053 },
            Opcode::FCVTWQ => Inst { value: 0xc6000053 },
            Opcode::FCVTWS => Inst { value: 0xc0000053 },
            Opcode::FCVTWUD => Inst { value: 0xc2100053 },
            Opcode::FCVTWUH => Inst { value: 0xc4100053 },
            Opcode::FCVTWUQ => Inst { value: 0xc6100053 },
            Opcode::FCVTWUS => Inst { value: 0xc0100053 },
            Opcode::FCVTMODWD => Inst { value: 0xc2801053 },
            Opcode::FDIVD => Inst { value: 0x1a000053 },
            Opcode::FDIVH => Inst { value: 0x1c000053 },
            Opcode::FDIVQ => Inst { value: 0x1e000053 },
            Opcode::FDIVS => Inst { value: 0x18000053 },
            Opcode::FENCE => Inst { value: 0xf },
            Opcode::FENCEI => Inst { value: 0x100f },
            Opcode::FENCETSO => Inst { value: 0x8330000f },
            Opcode::FEQD => Inst { value: 0xa2002053 },
            Opcode::FEQH => Inst { value: 0xa4002053 },
            Opcode::FEQQ => Inst { value: 0xa6002053 },
            Opcode::FEQS => Inst { value: 0xa0002053 },
            Opcode::FLD => Inst { value: 0x3007 },
            Opcode::FLED => Inst { value: 0xa2000053 },
            Opcode::FLEH => Inst { value: 0xa4000053 },
            Opcode::FLEQ => Inst { value: 0xa6000053 },
            Opcode::FLES => Inst { value: 0xa0000053 },
            Opcode::FLEQD => Inst { value: 0xa2004053 },
            Opcode::FLEQH => Inst { value: 0xa4004053 },
            Opcode::FLEQQ => Inst { value: 0xa6004053 },
            Opcode::FLEQS => Inst { value: 0xa0004053 },
            Opcode::FLH => Inst { value: 0x1007 },
            Opcode::FLID => Inst { value: 0xf2100053 },
            Opcode::FLIH => Inst { value: 0xf4100053 },
            Opcode::FLIQ => Inst { value: 0xf6100053 },
            Opcode::FLIS => Inst { value: 0xf0100053 },
            Opcode::FLQ => Inst { value: 0x4007 },
            Opcode::FLTD => Inst { value: 0xa2001053 },
            Opcode::FLTH => Inst { value: 0xa4001053 },
            Opcode::FLTQ => Inst { value: 0xa6001053 },
            Opcode::FLTS => Inst { value: 0xa0001053 },
            Opcode::FLTQD => Inst { value: 0xa2005053 },
            Opcode::FLTQH => Inst { value: 0xa4005053 },
            Opcode::FLTQQ => Inst { value: 0xa6005053 },
            Opcode::FLTQS => Inst { value: 0xa0005053 },
            Opcode::FLW => Inst { value: 0x2007 },
            Opcode::FMADDD => Inst { value: 0x2000043 },
            Opcode::FMADDH => Inst { value: 0x4000043 },
            Opcode::FMADDQ => Inst { value: 0x6000043 },
            Opcode::FMADDS => Inst { value: 0x43 },
            Opcode::FMAXD => Inst { value: 0x2a001053 },
            Opcode::FMAXH => Inst { value: 0x2c001053 },
            Opcode::FMAXQ => Inst { value: 0x2e001053 },
            Opcode::FMAXS => Inst { value: 0x28001053 },
            Opcode::FMAXMD => Inst { value: 0x2a003053 },
            Opcode::FMAXMH => Inst { value: 0x2c003053 },
            Opcode::FMAXMQ => Inst { value: 0x2e003053 },
            Opcode::FMAXMS => Inst { value: 0x28003053 },
            Opcode::FMIND => Inst { value: 0x2a000053 },
            Opcode::FMINH => Inst { value: 0x2c000053 },
            Opcode::FMINQ => Inst { value: 0x2e000053 },
            Opcode::FMINS => Inst { value: 0x28000053 },
            Opcode::FMINMD => Inst { value: 0x2a002053 },
            Opcode::FMINMH => Inst { value: 0x2c002053 },
            Opcode::FMINMQ => Inst { value: 0x2e002053 },
            Opcode::FMINMS => Inst { value: 0x28002053 },
            Opcode::FMSUBD => Inst { value: 0x2000047 },
            Opcode::FMSUBH => Inst { value: 0x4000047 },
            Opcode::FMSUBQ => Inst { value: 0x6000047 },
            Opcode::FMSUBS => Inst { value: 0x47 },
            Opcode::FMULD => Inst { value: 0x12000053 },
            Opcode::FMULH => Inst { value: 0x14000053 },
            Opcode::FMULQ => Inst { value: 0x16000053 },
            Opcode::FMULS => Inst { value: 0x10000053 },
            Opcode::FMVD => Inst { value: 0x22000053 },
            Opcode::FMVDX => Inst { value: 0xf2000053 },
            Opcode::FMVH => Inst { value: 0x24000053 },
            Opcode::FMVHX => Inst { value: 0xf4000053 },
            Opcode::FMVQ => Inst { value: 0x26000053 },
            Opcode::FMVS => Inst { value: 0x20000053 },
            Opcode::FMVSX => Inst { value: 0xf0000053 },
            Opcode::FMVWX => Inst { value: 0xf0000053 },
            Opcode::FMVXD => Inst { value: 0xe2000053 },
            Opcode::FMVXH => Inst { value: 0xe4000053 },
            Opcode::FMVXS => Inst { value: 0xe0000053 },
            Opcode::FMVXW => Inst { value: 0xe0000053 },
            Opcode::FMVHXD => Inst { value: 0xe2100053 },
            Opcode::FMVHXQ => Inst { value: 0xe6100053 },
            Opcode::FMVPDX => Inst { value: 0xb2000053 },
            Opcode::FMVPQX => Inst { value: 0xb6000053 },
            Opcode::FNEGD => Inst { value: 0x22001053 },
            Opcode::FNEGH => Inst { value: 0x24001053 },
            Opcode::FNEGQ => Inst { value: 0x26001053 },
            Opcode::FNEGS => Inst { value: 0x20001053 },
            Opcode::FNMADDD => Inst { value: 0x200004f },
            Opcode::FNMADDH => Inst { value: 0x400004f },
            Opcode::FNMADDQ => Inst { value: 0x600004f },
            Opcode::FNMADDS => Inst { value: 0x4f },
            Opcode::FNMSUBD => Inst { value: 0x200004b },
            Opcode::FNMSUBH => Inst { value: 0x400004b },
            Opcode::FNMSUBQ => Inst { value: 0x600004b },
            Opcode::FNMSUBS => Inst { value: 0x4b },
            Opcode::FRCSR => Inst { value: 0x302073 },
            Opcode::FRFLAGS => Inst { value: 0x102073 },
            Opcode::FROUNDD => Inst { value: 0x42400053 },
            Opcode::FROUNDH => Inst { value: 0x44400053 },
            Opcode::FROUNDQ => Inst { value: 0x46400053 },
            Opcode::FROUNDS => Inst { value: 0x40400053 },
            Opcode::FROUNDNXD => Inst { value: 0x42500053 },
            Opcode::FROUNDNXH => Inst { value: 0x44500053 },
            Opcode::FROUNDNXQ => Inst { value: 0x46500053 },
            Opcode::FROUNDNXS => Inst { value: 0x40500053 },
            Opcode::FRRM => Inst { value: 0x202073 },
            Opcode::FSCSR => Inst { value: 0x301073 },
            Opcode::FSD => Inst { value: 0x3027 },
            Opcode::FSFLAGS => Inst { value: 0x101073 },
            Opcode::FSFLAGSI => Inst { value: 0x105073 },
            Opcode::FSGNJD => Inst { value: 0x22000053 },
            Opcode::FSGNJH => Inst { value: 0x24000053 },
            Opcode::FSGNJQ => Inst { value: 0x26000053 },
            Opcode::FSGNJS => Inst { value: 0x20000053 },
            Opcode::FSGNJND => Inst { value: 0x22001053 },
            Opcode::FSGNJNH => Inst { value: 0x24001053 },
            Opcode::FSGNJNQ => Inst { value: 0x26001053 },
            Opcode::FSGNJNS => Inst { value: 0x20001053 },
            Opcode::FSGNJXD => Inst { value: 0x22002053 },
            Opcode::FSGNJXH => Inst { value: 0x24002053 },
            Opcode::FSGNJXQ => Inst { value: 0x26002053 },
            Opcode::FSGNJXS => Inst { value: 0x20002053 },
            Opcode::FSH => Inst { value: 0x1027 },
            Opcode::FSQ => Inst { value: 0x4027 },
            Opcode::FSQRTD => Inst { value: 0x5a000053 },
            Opcode::FSQRTH => Inst { value: 0x5c000053 },
            Opcode::FSQRTQ => Inst { value: 0x5e000053 },
            Opcode::FSQRTS => Inst { value: 0x58000053 },
            Opcode::FSRM => Inst { value: 0x201073 },
            Opcode::FSRMI => Inst { value: 0x205073 },
            Opcode::FSUBD => Inst { value: 0xa000053 },
            Opcode::FSUBH => Inst { value: 0xc000053 },
            Opcode::FSUBQ => Inst { value: 0xe000053 },
            Opcode::FSUBS => Inst { value: 0x8000053 },
            Opcode::FSW => Inst { value: 0x2027 },
            Opcode::HFENCEGVMA => Inst { value: 0x62000073 },
            Opcode::HFENCEVVMA => Inst { value: 0x22000073 },
            Opcode::HINVALGVMA => Inst { value: 0x66000073 },
            Opcode::HINVALVVMA => Inst { value: 0x26000073 },
            Opcode::HLVB => Inst { value: 0x60004073 },
            Opcode::HLVBU => Inst { value: 0x60104073 },
            Opcode::HLVD => Inst { value: 0x6c004073 },
            Opcode::HLVH => Inst { value: 0x64004073 },
            Opcode::HLVHU => Inst { value: 0x64104073 },
            Opcode::HLVW => Inst { value: 0x68004073 },
            Opcode::HLVWU => Inst { value: 0x68104073 },
            Opcode::HLVXHU => Inst { value: 0x64304073 },
            Opcode::HLVXWU => Inst { value: 0x68304073 },
            Opcode::HSVB => Inst { value: 0x62004073 },
            Opcode::HSVD => Inst { value: 0x6e004073 },
            Opcode::HSVH => Inst { value: 0x66004073 },
            Opcode::HSVW => Inst { value: 0x6a004073 },
            Opcode::J => Inst { value: 0x6f },
            Opcode::JAL => Inst { value: 0x6f },
            Opcode::JALPSEUDO => Inst { value: 0xef },
            Opcode::JALR => Inst { value: 0x67 },
            Opcode::JALRPSEUDO => Inst { value: 0xe7 },
            Opcode::JR => Inst { value: 0x67 },
            Opcode::LB => Inst { value: 0x3 },
            Opcode::LBU => Inst { value: 0x4003 },
            Opcode::LD => Inst { value: 0x3003 },
            Opcode::LH => Inst { value: 0x1003 },
            Opcode::LHU => Inst { value: 0x5003 },
            Opcode::LPAD => Inst { value: 0x17 },
            Opcode::LRD => Inst { value: 0x1000302f },
            Opcode::LRW => Inst { value: 0x1000202f },
            Opcode::LUI => Inst { value: 0x37 },
            Opcode::LW => Inst { value: 0x2003 },
            Opcode::LWU => Inst { value: 0x6003 },
            Opcode::MAX => Inst { value: 0xa006033 },
            Opcode::MAXU => Inst { value: 0xa007033 },
            Opcode::MIN => Inst { value: 0xa004033 },
            Opcode::MINU => Inst { value: 0xa005033 },
            Opcode::MNRET => Inst { value: 0x70200073 },
            Opcode::MOPR0 => Inst { value: 0x81c04073 },
            Opcode::MOPR1 => Inst { value: 0x81d04073 },
            Opcode::MOPR10 => Inst { value: 0x89e04073 },
            Opcode::MOPR11 => Inst { value: 0x89f04073 },
            Opcode::MOPR12 => Inst { value: 0x8dc04073 },
            Opcode::MOPR13 => Inst { value: 0x8dd04073 },
            Opcode::MOPR14 => Inst { value: 0x8de04073 },
            Opcode::MOPR15 => Inst { value: 0x8df04073 },
            Opcode::MOPR16 => Inst { value: 0xc1c04073 },
            Opcode::MOPR17 => Inst { value: 0xc1d04073 },
            Opcode::MOPR18 => Inst { value: 0xc1e04073 },
            Opcode::MOPR19 => Inst { value: 0xc1f04073 },
            Opcode::MOPR2 => Inst { value: 0x81e04073 },
            Opcode::MOPR20 => Inst { value: 0xc5c04073 },
            Opcode::MOPR21 => Inst { value: 0xc5d04073 },
            Opcode::MOPR22 => Inst { value: 0xc5e04073 },
            Opcode::MOPR23 => Inst { value: 0xc5f04073 },
            Opcode::MOPR24 => Inst { value: 0xc9c04073 },
            Opcode::MOPR25 => Inst { value: 0xc9d04073 },
            Opcode::MOPR26 => Inst { value: 0xc9e04073 },
            Opcode::MOPR27 => Inst { value: 0xc9f04073 },
            Opcode::MOPR28 => Inst { value: 0xcdc04073 },
            Opcode::MOPR29 => Inst { value: 0xcdd04073 },
            Opcode::MOPR3 => Inst { value: 0x81f04073 },
            Opcode::MOPR30 => Inst { value: 0xcde04073 },
            Opcode::MOPR31 => Inst { value: 0xcdf04073 },
            Opcode::MOPR4 => Inst { value: 0x85c04073 },
            Opcode::MOPR5 => Inst { value: 0x85d04073 },
            Opcode::MOPR6 => Inst { value: 0x85e04073 },
            Opcode::MOPR7 => Inst { value: 0x85f04073 },
            Opcode::MOPR8 => Inst { value: 0x89c04073 },
            Opcode::MOPR9 => Inst { value: 0x89d04073 },
            Opcode::MOPRN => Inst { value: 0x81c04073 },
            Opcode::MOPRR0 => Inst { value: 0x82004073 },
            Opcode::MOPRR1 => Inst { value: 0x86004073 },
            Opcode::MOPRR2 => Inst { value: 0x8a004073 },
            Opcode::MOPRR3 => Inst { value: 0x8e004073 },
            Opcode::MOPRR4 => Inst { value: 0xc2004073 },
            Opcode::MOPRR5 => Inst { value: 0xc6004073 },
            Opcode::MOPRR6 => Inst { value: 0xca004073 },
            Opcode::MOPRR7 => Inst { value: 0xce004073 },
            Opcode::MOPRRN => Inst { value: 0x82004073 },
            Opcode::MRET => Inst { value: 0x30200073 },
            Opcode::MUL => Inst { value: 0x2000033 },
            Opcode::MULH => Inst { value: 0x2001033 },
            Opcode::MULHSU => Inst { value: 0x2002033 },
            Opcode::MULHU => Inst { value: 0x2003033 },
            Opcode::MULW => Inst { value: 0x200003b },
            Opcode::MV => Inst { value: 0x13 },
            Opcode::NEG => Inst { value: 0x40000033 },
            Opcode::NOP => Inst { value: 0x13 },
            Opcode::NTLALL => Inst { value: 0x500033 },
            Opcode::NTLP1 => Inst { value: 0x200033 },
            Opcode::NTLPALL => Inst { value: 0x300033 },
            Opcode::NTLS1 => Inst { value: 0x400033 },
            Opcode::OR => Inst { value: 0x6033 },
            Opcode::ORCB => Inst { value: 0x28705013 },
            Opcode::ORI => Inst { value: 0x6013 },
            Opcode::ORN => Inst { value: 0x40006033 },
            Opcode::PACK => Inst { value: 0x8004033 },
            Opcode::PACKH => Inst { value: 0x8007033 },
            Opcode::PACKW => Inst { value: 0x800403b },
            Opcode::PAUSE => Inst { value: 0x100000f },
            Opcode::PREFETCHI => Inst { value: 0x6013 },
            Opcode::PREFETCHR => Inst { value: 0x106013 },
            Opcode::PREFETCHW => Inst { value: 0x306013 },
            Opcode::RDCYCLE => Inst { value: 0xc0002073 },
            Opcode::RDCYCLEH => Inst { value: 0xc8002073 },
            Opcode::RDINSTRET => Inst { value: 0xc0202073 },
            Opcode::RDINSTRETH => Inst { value: 0xc8202073 },
            Opcode::RDTIME => Inst { value: 0xc0102073 },
            Opcode::RDTIMEH => Inst { value: 0xc8102073 },
            Opcode::REM => Inst { value: 0x2006033 },
            Opcode::REMU => Inst { value: 0x2007033 },
            Opcode::REMUW => Inst { value: 0x200703b },
            Opcode::REMW => Inst { value: 0x200603b },
            Opcode::RET => Inst { value: 0x8067 },
            Opcode::REV8 => Inst { value: 0x6b805013 },
            Opcode::REV8RV32 => Inst { value: 0x69805013 },
            Opcode::ROL => Inst { value: 0x60001033 },
            Opcode::ROLW => Inst { value: 0x6000103b },
            Opcode::ROR => Inst { value: 0x60005033 },
            Opcode::RORI => Inst { value: 0x60005013 },
            Opcode::RORIRV32 => Inst { value: 0x60005013 },
            Opcode::RORIW => Inst { value: 0x6000501b },
            Opcode::RORW => Inst { value: 0x6000503b },
            Opcode::SB => Inst { value: 0x23 },
            Opcode::SBREAK => Inst { value: 0x100073 },
            Opcode::SCD => Inst { value: 0x1800302f },
            Opcode::SCW => Inst { value: 0x1800202f },
            Opcode::SCALL => Inst { value: 0x73 },
            Opcode::SCTRCLR => Inst { value: 0x10400073 },
            Opcode::SD => Inst { value: 0x3023 },
            Opcode::SEQZ => Inst { value: 0x103013 },
            Opcode::SEXTB => Inst { value: 0x60401013 },
            Opcode::SEXTH => Inst { value: 0x60501013 },
            Opcode::SEXTW => Inst { value: 0x1b },
            Opcode::SFENCEINVALIR => Inst { value: 0x18100073 },
            Opcode::SFENCEVMA => Inst { value: 0x12000073 },
            Opcode::SFENCEWINVAL => Inst { value: 0x18000073 },
            Opcode::SGTZ => Inst { value: 0x2033 },
            Opcode::SH => Inst { value: 0x1023 },
            Opcode::SH1ADD => Inst { value: 0x20002033 },
            Opcode::SH1ADDUW => Inst { value: 0x2000203b },
            Opcode::SH2ADD => Inst { value: 0x20004033 },
            Opcode::SH2ADDUW => Inst { value: 0x2000403b },
            Opcode::SH3ADD => Inst { value: 0x20006033 },
            Opcode::SH3ADDUW => Inst { value: 0x2000603b },
            Opcode::SHA256SIG0 => Inst { value: 0x10201013 },
            Opcode::SHA256SIG1 => Inst { value: 0x10301013 },
            Opcode::SHA256SUM0 => Inst { value: 0x10001013 },
            Opcode::SHA256SUM1 => Inst { value: 0x10101013 },
            Opcode::SHA512SIG0 => Inst { value: 0x10601013 },
            Opcode::SHA512SIG0H => Inst { value: 0x5c000033 },
            Opcode::SHA512SIG0L => Inst { value: 0x54000033 },
            Opcode::SHA512SIG1 => Inst { value: 0x10701013 },
            Opcode::SHA512SIG1H => Inst { value: 0x5e000033 },
            Opcode::SHA512SIG1L => Inst { value: 0x56000033 },
            Opcode::SHA512SUM0 => Inst { value: 0x10401013 },
            Opcode::SHA512SUM0R => Inst { value: 0x50000033 },
            Opcode::SHA512SUM1 => Inst { value: 0x10501013 },
            Opcode::SHA512SUM1R => Inst { value: 0x52000033 },
            Opcode::SINVALVMA => Inst { value: 0x16000073 },
            Opcode::SLL => Inst { value: 0x1033 },
            Opcode::SLLI => Inst { value: 0x1013 },
            Opcode::SLLIRV32 => Inst { value: 0x1013 },
            Opcode::SLLIUW => Inst { value: 0x800101b },
            Opcode::SLLIW => Inst { value: 0x101b },
            Opcode::SLLW => Inst { value: 0x103b },
            Opcode::SLT => Inst { value: 0x2033 },
            Opcode::SLTI => Inst { value: 0x2013 },
            Opcode::SLTIU => Inst { value: 0x3013 },
            Opcode::SLTU => Inst { value: 0x3033 },
            Opcode::SLTZ => Inst { value: 0x2033 },
            Opcode::SM3P0 => Inst { value: 0x10801013 },
            Opcode::SM3P1 => Inst { value: 0x10901013 },
            Opcode::SM4ED => Inst { value: 0x30000033 },
            Opcode::SM4KS => Inst { value: 0x34000033 },
            Opcode::SNEZ => Inst { value: 0x3033 },
            Opcode::SRA => Inst { value: 0x40005033 },
            Opcode::SRAI => Inst { value: 0x40005013 },
            Opcode::SRAIRV32 => Inst { value: 0x40005013 },
            Opcode::SRAIW => Inst { value: 0x4000501b },
            Opcode::SRAW => Inst { value: 0x4000503b },
            Opcode::SRET => Inst { value: 0x10200073 },
            Opcode::SRL => Inst { value: 0x5033 },
            Opcode::SRLI => Inst { value: 0x5013 },
            Opcode::SRLIRV32 => Inst { value: 0x5013 },
            Opcode::SRLIW => Inst { value: 0x501b },
            Opcode::SRLW => Inst { value: 0x503b },
            Opcode::SSAMOSWAPD => Inst { value: 0x4800302f },
            Opcode::SSAMOSWAPW => Inst { value: 0x4800202f },
            Opcode::SSPOPCHKX1 => Inst { value: 0xcdc0c073 },
            Opcode::SSPOPCHKX5 => Inst { value: 0xcdc2c073 },
            Opcode::SSPUSHX1 => Inst { value: 0xce104073 },
            Opcode::SSPUSHX5 => Inst { value: 0xce504073 },
            Opcode::SSRDP => Inst { value: 0xcdc04073 },
            Opcode::SUB => Inst { value: 0x40000033 },
            Opcode::SUBW => Inst { value: 0x4000003b },
            Opcode::SW => Inst { value: 0x2023 },
            Opcode::UNZIP => Inst { value: 0x8f05013 },
            Opcode::VAADDVV => Inst { value: 0x24002057 },
            Opcode::VAADDVX => Inst { value: 0x24006057 },
            Opcode::VAADDUVV => Inst { value: 0x20002057 },
            Opcode::VAADDUVX => Inst { value: 0x20006057 },
            Opcode::VADCVIM => Inst { value: 0x40003057 },
            Opcode::VADCVVM => Inst { value: 0x40000057 },
            Opcode::VADCVXM => Inst { value: 0x40004057 },
            Opcode::VADDVI => Inst { value: 0x3057 },
            Opcode::VADDVV => Inst { value: 0x57 },
            Opcode::VADDVX => Inst { value: 0x4057 },
            Opcode::VAESDFVS => Inst { value: 0xa600a077 },
            Opcode::VAESDFVV => Inst { value: 0xa200a077 },
            Opcode::VAESDMVS => Inst { value: 0xa6002077 },
            Opcode::VAESDMVV => Inst { value: 0xa2002077 },
            Opcode::VAESEFVS => Inst { value: 0xa601a077 },
            Opcode::VAESEFVV => Inst { value: 0xa201a077 },
            Opcode::VAESEMVS => Inst { value: 0xa6012077 },
            Opcode::VAESEMVV => Inst { value: 0xa2012077 },
            Opcode::VAESKF1VI => Inst { value: 0x8a002077 },
            Opcode::VAESKF2VI => Inst { value: 0xaa002077 },
            Opcode::VAESZVS => Inst { value: 0xa603a077 },
            Opcode::VANDVI => Inst { value: 0x24003057 },
            Opcode::VANDVV => Inst { value: 0x24000057 },
            Opcode::VANDVX => Inst { value: 0x24004057 },
            Opcode::VANDNVV => Inst { value: 0x4000057 },
            Opcode::VANDNVX => Inst { value: 0x4004057 },
            Opcode::VASUBVV => Inst { value: 0x2c002057 },
            Opcode::VASUBVX => Inst { value: 0x2c006057 },
            Opcode::VASUBUVV => Inst { value: 0x28002057 },
            Opcode::VASUBUVX => Inst { value: 0x28006057 },
            Opcode::VBREV8V => Inst { value: 0x48042057 },
            Opcode::VBREVV => Inst { value: 0x48052057 },
            Opcode::VCLMULVV => Inst { value: 0x30002057 },
            Opcode::VCLMULVX => Inst { value: 0x30006057 },
            Opcode::VCLMULHVV => Inst { value: 0x34002057 },
            Opcode::VCLMULHVX => Inst { value: 0x34006057 },
            Opcode::VCLZV => Inst { value: 0x48062057 },
            Opcode::VCOMPRESSVM => Inst { value: 0x5e002057 },
            Opcode::VCPOPM => Inst { value: 0x40082057 },
            Opcode::VCPOPV => Inst { value: 0x48072057 },
            Opcode::VCTZV => Inst { value: 0x4806a057 },
            Opcode::VDIVVV => Inst { value: 0x84002057 },
            Opcode::VDIVVX => Inst { value: 0x84006057 },
            Opcode::VDIVUVV => Inst { value: 0x80002057 },
            Opcode::VDIVUVX => Inst { value: 0x80006057 },
            Opcode::VFADDVF => Inst { value: 0x5057 },
            Opcode::VFADDVV => Inst { value: 0x1057 },
            Opcode::VFCLASSV => Inst { value: 0x4c081057 },
            Opcode::VFCVTFXV => Inst { value: 0x48019057 },
            Opcode::VFCVTFXUV => Inst { value: 0x48011057 },
            Opcode::VFCVTRTZXFV => Inst { value: 0x48039057 },
            Opcode::VFCVTRTZXUFV => Inst { value: 0x48031057 },
            Opcode::VFCVTXFV => Inst { value: 0x48009057 },
            Opcode::VFCVTXUFV => Inst { value: 0x48001057 },
            Opcode::VFDIVVF => Inst { value: 0x80005057 },
            Opcode::VFDIVVV => Inst { value: 0x80001057 },
            Opcode::VFIRSTM => Inst { value: 0x4008a057 },
            Opcode::VFMACCVF => Inst { value: 0xb0005057 },
            Opcode::VFMACCVV => Inst { value: 0xb0001057 },
            Opcode::VFMADDVF => Inst { value: 0xa0005057 },
            Opcode::VFMADDVV => Inst { value: 0xa0001057 },
            Opcode::VFMAXVF => Inst { value: 0x18005057 },
            Opcode::VFMAXVV => Inst { value: 0x18001057 },
            Opcode::VFMERGEVFM => Inst { value: 0x5c005057 },
            Opcode::VFMINVF => Inst { value: 0x10005057 },
            Opcode::VFMINVV => Inst { value: 0x10001057 },
            Opcode::VFMSACVF => Inst { value: 0xb8005057 },
            Opcode::VFMSACVV => Inst { value: 0xb8001057 },
            Opcode::VFMSUBVF => Inst { value: 0xa8005057 },
            Opcode::VFMSUBVV => Inst { value: 0xa8001057 },
            Opcode::VFMULVF => Inst { value: 0x90005057 },
            Opcode::VFMULVV => Inst { value: 0x90001057 },
            Opcode::VFMVFS => Inst { value: 0x42001057 },
            Opcode::VFMVSF => Inst { value: 0x42005057 },
            Opcode::VFMVVF => Inst { value: 0x5e005057 },
            Opcode::VFNCVTFFW => Inst { value: 0x480a1057 },
            Opcode::VFNCVTFXW => Inst { value: 0x48099057 },
            Opcode::VFNCVTFXUW => Inst { value: 0x48091057 },
            Opcode::VFNCVTRODFFW => Inst { value: 0x480a9057 },
            Opcode::VFNCVTRTZXFW => Inst { value: 0x480b9057 },
            Opcode::VFNCVTRTZXUFW => Inst { value: 0x480b1057 },
            Opcode::VFNCVTXFW => Inst { value: 0x48089057 },
            Opcode::VFNCVTXUFW => Inst { value: 0x48081057 },
            Opcode::VFNCVTBF16FFW => Inst { value: 0x480e9057 },
            Opcode::VFNMACCVF => Inst { value: 0xb4005057 },
            Opcode::VFNMACCVV => Inst { value: 0xb4001057 },
            Opcode::VFNMADDVF => Inst { value: 0xa4005057 },
            Opcode::VFNMADDVV => Inst { value: 0xa4001057 },
            Opcode::VFNMSACVF => Inst { value: 0xbc005057 },
            Opcode::VFNMSACVV => Inst { value: 0xbc001057 },
            Opcode::VFNMSUBVF => Inst { value: 0xac005057 },
            Opcode::VFNMSUBVV => Inst { value: 0xac001057 },
            Opcode::VFRDIVVF => Inst { value: 0x84005057 },
            Opcode::VFREC7V => Inst { value: 0x4c029057 },
            Opcode::VFREDMAXVS => Inst { value: 0x1c001057 },
            Opcode::VFREDMINVS => Inst { value: 0x14001057 },
            Opcode::VFREDOSUMVS => Inst { value: 0xc001057 },
            Opcode::VFREDSUMVS => Inst { value: 0x4001057 },
            Opcode::VFREDUSUMVS => Inst { value: 0x4001057 },
            Opcode::VFRSQRT7V => Inst { value: 0x4c021057 },
            Opcode::VFRSUBVF => Inst { value: 0x9c005057 },
            Opcode::VFSGNJVF => Inst { value: 0x20005057 },
            Opcode::VFSGNJVV => Inst { value: 0x20001057 },
            Opcode::VFSGNJNVF => Inst { value: 0x24005057 },
            Opcode::VFSGNJNVV => Inst { value: 0x24001057 },
            Opcode::VFSGNJXVF => Inst { value: 0x28005057 },
            Opcode::VFSGNJXVV => Inst { value: 0x28001057 },
            Opcode::VFSLIDE1DOWNVF => Inst { value: 0x3c005057 },
            Opcode::VFSLIDE1UPVF => Inst { value: 0x38005057 },
            Opcode::VFSQRTV => Inst { value: 0x4c001057 },
            Opcode::VFSUBVF => Inst { value: 0x8005057 },
            Opcode::VFSUBVV => Inst { value: 0x8001057 },
            Opcode::VFWADDVF => Inst { value: 0xc0005057 },
            Opcode::VFWADDVV => Inst { value: 0xc0001057 },
            Opcode::VFWADDWF => Inst { value: 0xd0005057 },
            Opcode::VFWADDWV => Inst { value: 0xd0001057 },
            Opcode::VFWCVTFFV => Inst { value: 0x48061057 },
            Opcode::VFWCVTFXV => Inst { value: 0x48059057 },
            Opcode::VFWCVTFXUV => Inst { value: 0x48051057 },
            Opcode::VFWCVTRTZXFV => Inst { value: 0x48079057 },
            Opcode::VFWCVTRTZXUFV => Inst { value: 0x48071057 },
            Opcode::VFWCVTXFV => Inst { value: 0x48049057 },
            Opcode::VFWCVTXUFV => Inst { value: 0x48041057 },
            Opcode::VFWCVTBF16FFV => Inst { value: 0x48069057 },
            Opcode::VFWMACCVF => Inst { value: 0xf0005057 },
            Opcode::VFWMACCVV => Inst { value: 0xf0001057 },
            Opcode::VFWMACCBF16VF => Inst { value: 0xec005057 },
            Opcode::VFWMACCBF16VV => Inst { value: 0xec001057 },
            Opcode::VFWMSACVF => Inst { value: 0xf8005057 },
            Opcode::VFWMSACVV => Inst { value: 0xf8001057 },
            Opcode::VFWMULVF => Inst { value: 0xe0005057 },
            Opcode::VFWMULVV => Inst { value: 0xe0001057 },
            Opcode::VFWNMACCVF => Inst { value: 0xf4005057 },
            Opcode::VFWNMACCVV => Inst { value: 0xf4001057 },
            Opcode::VFWNMSACVF => Inst { value: 0xfc005057 },
            Opcode::VFWNMSACVV => Inst { value: 0xfc001057 },
            Opcode::VFWREDOSUMVS => Inst { value: 0xcc001057 },
            Opcode::VFWREDSUMVS => Inst { value: 0xc4001057 },
            Opcode::VFWREDUSUMVS => Inst { value: 0xc4001057 },
            Opcode::VFWSUBVF => Inst { value: 0xc8005057 },
            Opcode::VFWSUBVV => Inst { value: 0xc8001057 },
            Opcode::VFWSUBWF => Inst { value: 0xd8005057 },
            Opcode::VFWSUBWV => Inst { value: 0xd8001057 },
            Opcode::VGHSHVV => Inst { value: 0xb2002077 },
            Opcode::VGMULVV => Inst { value: 0xa208a077 },
            Opcode::VIDV => Inst { value: 0x5008a057 },
            Opcode::VIOTAM => Inst { value: 0x50082057 },
            Opcode::VL1RV => Inst { value: 0x2800007 },
            Opcode::VL1RE16V => Inst { value: 0x2805007 },
            Opcode::VL1RE32V => Inst { value: 0x2806007 },
            Opcode::VL1RE64V => Inst { value: 0x2807007 },
            Opcode::VL1RE8V => Inst { value: 0x2800007 },
            Opcode::VL2RV => Inst { value: 0x22800007 },
            Opcode::VL2RE16V => Inst { value: 0x22805007 },
            Opcode::VL2RE32V => Inst { value: 0x22806007 },
            Opcode::VL2RE64V => Inst { value: 0x22807007 },
            Opcode::VL2RE8V => Inst { value: 0x22800007 },
            Opcode::VL4RV => Inst { value: 0x62800007 },
            Opcode::VL4RE16V => Inst { value: 0x62805007 },
            Opcode::VL4RE32V => Inst { value: 0x62806007 },
            Opcode::VL4RE64V => Inst { value: 0x62807007 },
            Opcode::VL4RE8V => Inst { value: 0x62800007 },
            Opcode::VL8RV => Inst { value: 0xe2800007 },
            Opcode::VL8RE16V => Inst { value: 0xe2805007 },
            Opcode::VL8RE32V => Inst { value: 0xe2806007 },
            Opcode::VL8RE64V => Inst { value: 0xe2807007 },
            Opcode::VL8RE8V => Inst { value: 0xe2800007 },
            Opcode::VLE16V => Inst { value: 0x5007 },
            Opcode::VLE16FFV => Inst { value: 0x1005007 },
            Opcode::VLE1V => Inst { value: 0x2b00007 },
            Opcode::VLE32V => Inst { value: 0x6007 },
            Opcode::VLE32FFV => Inst { value: 0x1006007 },
            Opcode::VLE64V => Inst { value: 0x7007 },
            Opcode::VLE64FFV => Inst { value: 0x1007007 },
            Opcode::VLE8V => Inst { value: 0x7 },
            Opcode::VLE8FFV => Inst { value: 0x1000007 },
            Opcode::VLMV => Inst { value: 0x2b00007 },
            Opcode::VLOXEI16V => Inst { value: 0xc005007 },
            Opcode::VLOXEI32V => Inst { value: 0xc006007 },
            Opcode::VLOXEI64V => Inst { value: 0xc007007 },
            Opcode::VLOXEI8V => Inst { value: 0xc000007 },
            Opcode::VLSE16V => Inst { value: 0x8005007 },
            Opcode::VLSE32V => Inst { value: 0x8006007 },
            Opcode::VLSE64V => Inst { value: 0x8007007 },
            Opcode::VLSE8V => Inst { value: 0x8000007 },
            Opcode::VLUXEI16V => Inst { value: 0x4005007 },
            Opcode::VLUXEI32V => Inst { value: 0x4006007 },
            Opcode::VLUXEI64V => Inst { value: 0x4007007 },
            Opcode::VLUXEI8V => Inst { value: 0x4000007 },
            Opcode::VMACCVV => Inst { value: 0xb4002057 },
            Opcode::VMACCVX => Inst { value: 0xb4006057 },
            Opcode::VMADCVI => Inst { value: 0x46003057 },
            Opcode::VMADCVIM => Inst { value: 0x44003057 },
            Opcode::VMADCVV => Inst { value: 0x46000057 },
            Opcode::VMADCVVM => Inst { value: 0x44000057 },
            Opcode::VMADCVX => Inst { value: 0x46004057 },
            Opcode::VMADCVXM => Inst { value: 0x44004057 },
            Opcode::VMADDVV => Inst { value: 0xa4002057 },
            Opcode::VMADDVX => Inst { value: 0xa4006057 },
            Opcode::VMANDMM => Inst { value: 0x66002057 },
            Opcode::VMANDNMM => Inst { value: 0x62002057 },
            Opcode::VMANDNOTMM => Inst { value: 0x60002057 },
            Opcode::VMAXVV => Inst { value: 0x1c000057 },
            Opcode::VMAXVX => Inst { value: 0x1c004057 },
            Opcode::VMAXUVV => Inst { value: 0x18000057 },
            Opcode::VMAXUVX => Inst { value: 0x18004057 },
            Opcode::VMERGEVIM => Inst { value: 0x5c003057 },
            Opcode::VMERGEVVM => Inst { value: 0x5c000057 },
            Opcode::VMERGEVXM => Inst { value: 0x5c004057 },
            Opcode::VMFEQVF => Inst { value: 0x60005057 },
            Opcode::VMFEQVV => Inst { value: 0x60001057 },
            Opcode::VMFGEVF => Inst { value: 0x7c005057 },
            Opcode::VMFGTVF => Inst { value: 0x74005057 },
            Opcode::VMFLEVF => Inst { value: 0x64005057 },
            Opcode::VMFLEVV => Inst { value: 0x64001057 },
            Opcode::VMFLTVF => Inst { value: 0x6c005057 },
            Opcode::VMFLTVV => Inst { value: 0x6c001057 },
            Opcode::VMFNEVF => Inst { value: 0x70005057 },
            Opcode::VMFNEVV => Inst { value: 0x70001057 },
            Opcode::VMINVV => Inst { value: 0x14000057 },
            Opcode::VMINVX => Inst { value: 0x14004057 },
            Opcode::VMINUVV => Inst { value: 0x10000057 },
            Opcode::VMINUVX => Inst { value: 0x10004057 },
            Opcode::VMNANDMM => Inst { value: 0x76002057 },
            Opcode::VMNORMM => Inst { value: 0x7a002057 },
            Opcode::VMORMM => Inst { value: 0x6a002057 },
            Opcode::VMORNMM => Inst { value: 0x72002057 },
            Opcode::VMORNOTMM => Inst { value: 0x70002057 },
            Opcode::VMSBCVV => Inst { value: 0x4e000057 },
            Opcode::VMSBCVVM => Inst { value: 0x4c000057 },
            Opcode::VMSBCVX => Inst { value: 0x4e004057 },
            Opcode::VMSBCVXM => Inst { value: 0x4c004057 },
            Opcode::VMSBFM => Inst { value: 0x5000a057 },
            Opcode::VMSEQVI => Inst { value: 0x60003057 },
            Opcode::VMSEQVV => Inst { value: 0x60000057 },
            Opcode::VMSEQVX => Inst { value: 0x60004057 },
            Opcode::VMSGTVI => Inst { value: 0x7c003057 },
            Opcode::VMSGTVX => Inst { value: 0x7c004057 },
            Opcode::VMSGTUVI => Inst { value: 0x78003057 },
            Opcode::VMSGTUVX => Inst { value: 0x78004057 },
            Opcode::VMSIFM => Inst { value: 0x5001a057 },
            Opcode::VMSLEVI => Inst { value: 0x74003057 },
            Opcode::VMSLEVV => Inst { value: 0x74000057 },
            Opcode::VMSLEVX => Inst { value: 0x74004057 },
            Opcode::VMSLEUVI => Inst { value: 0x70003057 },
            Opcode::VMSLEUVV => Inst { value: 0x70000057 },
            Opcode::VMSLEUVX => Inst { value: 0x70004057 },
            Opcode::VMSLTVV => Inst { value: 0x6c000057 },
            Opcode::VMSLTVX => Inst { value: 0x6c004057 },
            Opcode::VMSLTUVV => Inst { value: 0x68000057 },
            Opcode::VMSLTUVX => Inst { value: 0x68004057 },
            Opcode::VMSNEVI => Inst { value: 0x64003057 },
            Opcode::VMSNEVV => Inst { value: 0x64000057 },
            Opcode::VMSNEVX => Inst { value: 0x64004057 },
            Opcode::VMSOFM => Inst { value: 0x50012057 },
            Opcode::VMULVV => Inst { value: 0x94002057 },
            Opcode::VMULVX => Inst { value: 0x94006057 },
            Opcode::VMULHVV => Inst { value: 0x9c002057 },
            Opcode::VMULHVX => Inst { value: 0x9c006057 },
            Opcode::VMULHSUVV => Inst { value: 0x98002057 },
            Opcode::VMULHSUVX => Inst { value: 0x98006057 },
            Opcode::VMULHUVV => Inst { value: 0x90002057 },
            Opcode::VMULHUVX => Inst { value: 0x90006057 },
            Opcode::VMV1RV => Inst { value: 0x9e003057 },
            Opcode::VMV2RV => Inst { value: 0x9e00b057 },
            Opcode::VMV4RV => Inst { value: 0x9e01b057 },
            Opcode::VMV8RV => Inst { value: 0x9e03b057 },
            Opcode::VMVSX => Inst { value: 0x42006057 },
            Opcode::VMVVI => Inst { value: 0x5e003057 },
            Opcode::VMVVV => Inst { value: 0x5e000057 },
            Opcode::VMVVX => Inst { value: 0x5e004057 },
            Opcode::VMVXS => Inst { value: 0x42002057 },
            Opcode::VMXNORMM => Inst { value: 0x7e002057 },
            Opcode::VMXORMM => Inst { value: 0x6e002057 },
            Opcode::VNCLIPWI => Inst { value: 0xbc003057 },
            Opcode::VNCLIPWV => Inst { value: 0xbc000057 },
            Opcode::VNCLIPWX => Inst { value: 0xbc004057 },
            Opcode::VNCLIPUWI => Inst { value: 0xb8003057 },
            Opcode::VNCLIPUWV => Inst { value: 0xb8000057 },
            Opcode::VNCLIPUWX => Inst { value: 0xb8004057 },
            Opcode::VNMSACVV => Inst { value: 0xbc002057 },
            Opcode::VNMSACVX => Inst { value: 0xbc006057 },
            Opcode::VNMSUBVV => Inst { value: 0xac002057 },
            Opcode::VNMSUBVX => Inst { value: 0xac006057 },
            Opcode::VNSRAWI => Inst { value: 0xb4003057 },
            Opcode::VNSRAWV => Inst { value: 0xb4000057 },
            Opcode::VNSRAWX => Inst { value: 0xb4004057 },
            Opcode::VNSRLWI => Inst { value: 0xb0003057 },
            Opcode::VNSRLWV => Inst { value: 0xb0000057 },
            Opcode::VNSRLWX => Inst { value: 0xb0004057 },
            Opcode::VORVI => Inst { value: 0x28003057 },
            Opcode::VORVV => Inst { value: 0x28000057 },
            Opcode::VORVX => Inst { value: 0x28004057 },
            Opcode::VPOPCM => Inst { value: 0x40082057 },
            Opcode::VREDANDVS => Inst { value: 0x4002057 },
            Opcode::VREDMAXVS => Inst { value: 0x1c002057 },
            Opcode::VREDMAXUVS => Inst { value: 0x18002057 },
            Opcode::VREDMINVS => Inst { value: 0x14002057 },
            Opcode::VREDMINUVS => Inst { value: 0x10002057 },
            Opcode::VREDORVS => Inst { value: 0x8002057 },
            Opcode::VREDSUMVS => Inst { value: 0x2057 },
            Opcode::VREDXORVS => Inst { value: 0xc002057 },
            Opcode::VREMVV => Inst { value: 0x8c002057 },
            Opcode::VREMVX => Inst { value: 0x8c006057 },
            Opcode::VREMUVV => Inst { value: 0x88002057 },
            Opcode::VREMUVX => Inst { value: 0x88006057 },
            Opcode::VREV8V => Inst { value: 0x4804a057 },
            Opcode::VRGATHERVI => Inst { value: 0x30003057 },
            Opcode::VRGATHERVV => Inst { value: 0x30000057 },
            Opcode::VRGATHERVX => Inst { value: 0x30004057 },
            Opcode::VRGATHEREI16VV => Inst { value: 0x38000057 },
            Opcode::VROLVV => Inst { value: 0x54000057 },
            Opcode::VROLVX => Inst { value: 0x54004057 },
            Opcode::VRORVI => Inst { value: 0x50003057 },
            Opcode::VRORVV => Inst { value: 0x50000057 },
            Opcode::VRORVX => Inst { value: 0x50004057 },
            Opcode::VRSUBVI => Inst { value: 0xc003057 },
            Opcode::VRSUBVX => Inst { value: 0xc004057 },
            Opcode::VS1RV => Inst { value: 0x2800027 },
            Opcode::VS2RV => Inst { value: 0x22800027 },
            Opcode::VS4RV => Inst { value: 0x62800027 },
            Opcode::VS8RV => Inst { value: 0xe2800027 },
            Opcode::VSADDVI => Inst { value: 0x84003057 },
            Opcode::VSADDVV => Inst { value: 0x84000057 },
            Opcode::VSADDVX => Inst { value: 0x84004057 },
            Opcode::VSADDUVI => Inst { value: 0x80003057 },
            Opcode::VSADDUVV => Inst { value: 0x80000057 },
            Opcode::VSADDUVX => Inst { value: 0x80004057 },
            Opcode::VSBCVVM => Inst { value: 0x48000057 },
            Opcode::VSBCVXM => Inst { value: 0x48004057 },
            Opcode::VSE16V => Inst { value: 0x5027 },
            Opcode::VSE1V => Inst { value: 0x2b00027 },
            Opcode::VSE32V => Inst { value: 0x6027 },
            Opcode::VSE64V => Inst { value: 0x7027 },
            Opcode::VSE8V => Inst { value: 0x27 },
            Opcode::VSETIVLI => Inst { value: 0xc0007057 },
            Opcode::VSETVL => Inst { value: 0x80007057 },
            Opcode::VSETVLI => Inst { value: 0x7057 },
            Opcode::VSEXTVF2 => Inst { value: 0x4803a057 },
            Opcode::VSEXTVF4 => Inst { value: 0x4802a057 },
            Opcode::VSEXTVF8 => Inst { value: 0x4801a057 },
            Opcode::VSHA2CHVV => Inst { value: 0xba002077 },
            Opcode::VSHA2CLVV => Inst { value: 0xbe002077 },
            Opcode::VSHA2MSVV => Inst { value: 0xb6002077 },
            Opcode::VSLIDE1DOWNVX => Inst { value: 0x3c006057 },
            Opcode::VSLIDE1UPVX => Inst { value: 0x38006057 },
            Opcode::VSLIDEDOWNVI => Inst { value: 0x3c003057 },
            Opcode::VSLIDEDOWNVX => Inst { value: 0x3c004057 },
            Opcode::VSLIDEUPVI => Inst { value: 0x38003057 },
            Opcode::VSLIDEUPVX => Inst { value: 0x38004057 },
            Opcode::VSLLVI => Inst { value: 0x94003057 },
            Opcode::VSLLVV => Inst { value: 0x94000057 },
            Opcode::VSLLVX => Inst { value: 0x94004057 },
            Opcode::VSM3CVI => Inst { value: 0xae002077 },
            Opcode::VSM3MEVV => Inst { value: 0x82002077 },
            Opcode::VSM4KVI => Inst { value: 0x86002077 },
            Opcode::VSM4RVS => Inst { value: 0xa6082077 },
            Opcode::VSM4RVV => Inst { value: 0xa2082077 },
            Opcode::VSMV => Inst { value: 0x2b00027 },
            Opcode::VSMULVV => Inst { value: 0x9c000057 },
            Opcode::VSMULVX => Inst { value: 0x9c004057 },
            Opcode::VSOXEI16V => Inst { value: 0xc005027 },
            Opcode::VSOXEI32V => Inst { value: 0xc006027 },
            Opcode::VSOXEI64V => Inst { value: 0xc007027 },
            Opcode::VSOXEI8V => Inst { value: 0xc000027 },
            Opcode::VSRAVI => Inst { value: 0xa4003057 },
            Opcode::VSRAVV => Inst { value: 0xa4000057 },
            Opcode::VSRAVX => Inst { value: 0xa4004057 },
            Opcode::VSRLVI => Inst { value: 0xa0003057 },
            Opcode::VSRLVV => Inst { value: 0xa0000057 },
            Opcode::VSRLVX => Inst { value: 0xa0004057 },
            Opcode::VSSE16V => Inst { value: 0x8005027 },
            Opcode::VSSE32V => Inst { value: 0x8006027 },
            Opcode::VSSE64V => Inst { value: 0x8007027 },
            Opcode::VSSE8V => Inst { value: 0x8000027 },
            Opcode::VSSRAVI => Inst { value: 0xac003057 },
            Opcode::VSSRAVV => Inst { value: 0xac000057 },
            Opcode::VSSRAVX => Inst { value: 0xac004057 },
            Opcode::VSSRLVI => Inst { value: 0xa8003057 },
            Opcode::VSSRLVV => Inst { value: 0xa8000057 },
            Opcode::VSSRLVX => Inst { value: 0xa8004057 },
            Opcode::VSSUBVV => Inst { value: 0x8c000057 },
            Opcode::VSSUBVX => Inst { value: 0x8c004057 },
            Opcode::VSSUBUVV => Inst { value: 0x88000057 },
            Opcode::VSSUBUVX => Inst { value: 0x88004057 },
            Opcode::VSUBVV => Inst { value: 0x8000057 },
            Opcode::VSUBVX => Inst { value: 0x8004057 },
            Opcode::VSUXEI16V => Inst { value: 0x4005027 },
            Opcode::VSUXEI32V => Inst { value: 0x4006027 },
            Opcode::VSUXEI64V => Inst { value: 0x4007027 },
            Opcode::VSUXEI8V => Inst { value: 0x4000027 },
            Opcode::VWADDVV => Inst { value: 0xc4002057 },
            Opcode::VWADDVX => Inst { value: 0xc4006057 },
            Opcode::VWADDWV => Inst { value: 0xd4002057 },
            Opcode::VWADDWX => Inst { value: 0xd4006057 },
            Opcode::VWADDUVV => Inst { value: 0xc0002057 },
            Opcode::VWADDUVX => Inst { value: 0xc0006057 },
            Opcode::VWADDUWV => Inst { value: 0xd0002057 },
            Opcode::VWADDUWX => Inst { value: 0xd0006057 },
            Opcode::VWMACCVV => Inst { value: 0xf4002057 },
            Opcode::VWMACCVX => Inst { value: 0xf4006057 },
            Opcode::VWMACCSUVV => Inst { value: 0xfc002057 },
            Opcode::VWMACCSUVX => Inst { value: 0xfc006057 },
            Opcode::VWMACCUVV => Inst { value: 0xf0002057 },
            Opcode::VWMACCUVX => Inst { value: 0xf0006057 },
            Opcode::VWMACCUSVX => Inst { value: 0xf8006057 },
            Opcode::VWMULVV => Inst { value: 0xec002057 },
            Opcode::VWMULVX => Inst { value: 0xec006057 },
            Opcode::VWMULSUVV => Inst { value: 0xe8002057 },
            Opcode::VWMULSUVX => Inst { value: 0xe8006057 },
            Opcode::VWMULUVV => Inst { value: 0xe0002057 },
            Opcode::VWMULUVX => Inst { value: 0xe0006057 },
            Opcode::VWREDSUMVS => Inst { value: 0xc4000057 },
            Opcode::VWREDSUMUVS => Inst { value: 0xc0000057 },
            Opcode::VWSLLVI => Inst { value: 0xd4003057 },
            Opcode::VWSLLVV => Inst { value: 0xd4000057 },
            Opcode::VWSLLVX => Inst { value: 0xd4004057 },
            Opcode::VWSUBVV => Inst { value: 0xcc002057 },
            Opcode::VWSUBVX => Inst { value: 0xcc006057 },
            Opcode::VWSUBWV => Inst { value: 0xdc002057 },
            Opcode::VWSUBWX => Inst { value: 0xdc006057 },
            Opcode::VWSUBUVV => Inst { value: 0xc8002057 },
            Opcode::VWSUBUVX => Inst { value: 0xc8006057 },
            Opcode::VWSUBUWV => Inst { value: 0xd8002057 },
            Opcode::VWSUBUWX => Inst { value: 0xd8006057 },
            Opcode::VXORVI => Inst { value: 0x2c003057 },
            Opcode::VXORVV => Inst { value: 0x2c000057 },
            Opcode::VXORVX => Inst { value: 0x2c004057 },
            Opcode::VZEXTVF2 => Inst { value: 0x48032057 },
            Opcode::VZEXTVF4 => Inst { value: 0x48022057 },
            Opcode::VZEXTVF8 => Inst { value: 0x48012057 },
            Opcode::WFI => Inst { value: 0x10500073 },
            Opcode::WRSNTO => Inst { value: 0xd00073 },
            Opcode::WRSSTO => Inst { value: 0x1d00073 },
            Opcode::XNOR => Inst { value: 0x40004033 },
            Opcode::XOR => Inst { value: 0x4033 },
            Opcode::XORI => Inst { value: 0x4013 },
            Opcode::XPERM4 => Inst { value: 0x28002033 },
            Opcode::XPERM8 => Inst { value: 0x28004033 },
            Opcode::ZEXTB => Inst { value: 0xff07013 },
            Opcode::ZEXTH => Inst { value: 0x800403b },
            Opcode::ZEXTHRV32 => Inst { value: 0x8004033 },
            Opcode::ZEXTW => Inst { value: 0x800003b },
            Opcode::ZIP => Inst { value: 0x8f01013 },
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
    CsrZimm5,
    Empty,
    FmPredSuccRs1Rd,
    Imm12HiRs1Rs2Imm12lo,
    Imm12Rs1Rd,
    Imm20,
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
    RdCsrZimm5,
    RdImm20,
    RdJimm20,
    RdN0,
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
    RdZimm5,
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
    Zimm10Zimm5Rd,
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
            CSRCI | CSRSI | CSRWI => Encoding::CsrZimm5,
            CEBREAK | CMOP1 | CMOP11 | CMOP13 | CMOP15 | CMOP3 | CMOP5 | CMOP7 | CMOP9
            | CNTLALL | CNTLP1 | CNTLPALL | CNTLS1 | CSSPOPCHKX5 | CSSPUSHX1 | DRET | EBREAK
            | ECALL | MNRET | MRET | NOP | NTLALL | NTLP1 | NTLPALL | NTLS1 | PAUSE | RET
            | SBREAK | SCALL | SCTRCLR | SFENCEINVALIR | SFENCEWINVAL | SRET | SSPOPCHKX1
            | SSPOPCHKX5 | SSPUSHX1 | SSPUSHX5 | WFI | WRSNTO | WRSSTO => Encoding::Empty,
            FENCE => Encoding::FmPredSuccRs1Rd,
            FSD | FSH | FSQ | FSW | SB | SD | SH | SW => Encoding::Imm12HiRs1Rs2Imm12lo,
            FENCEI => Encoding::Imm12Rs1Rd,
            LPAD => Encoding::Imm20,
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
            CSRRCI | CSRRSI | CSRRWI => Encoding::RdCsrZimm5,
            AUIPC | LUI => Encoding::RdImm20,
            JAL => Encoding::RdJimm20,
            SSRDP => Encoding::RdN0,
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
            FCVTBF16S | FCVTDH | FCVTDL | FCVTDLU | FCVTDQ | FCVTDS | FCVTDW | FCVTDWU | FCVTHD
            | FCVTHL | FCVTHLU | FCVTHQ | FCVTHS | FCVTHW | FCVTHWU | FCVTLD | FCVTLH | FCVTLQ
            | FCVTLS | FCVTLUD | FCVTLUH | FCVTLUQ | FCVTLUS | FCVTQD | FCVTQH | FCVTQL
            | FCVTQLU | FCVTQS | FCVTQW | FCVTQWU | FCVTSBF16 | FCVTSD | FCVTSH | FCVTSL
            | FCVTSLU | FCVTSQ | FCVTSW | FCVTSWU | FCVTWD | FCVTWH | FCVTWQ | FCVTWS | FCVTWUD
            | FCVTWUH | FCVTWUQ | FCVTWUS | FROUNDD | FROUNDH | FROUNDQ | FROUNDS | FROUNDNXD
            | FROUNDNXH | FROUNDNXQ | FROUNDNXS | FSQRTD | FSQRTH | FSQRTQ | FSQRTS => {
                Encoding::RdRs1Rm
            }
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
            | AMOXORW | SCD | SCW | SSAMOSWAPD | SSAMOSWAPW => Encoding::RdRs1Rs2AqRl,
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
            FSFLAGSI | FSRMI => Encoding::RdZimm5,
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
            | VFWMACCBF16VF | VFWMSACVF | VFWMULVF | VFWNMACCVF | VFWNMSACVF | VFWSUBVF
            | VFWSUBWF | VMACCVX | VMADDVX | VMAXVX | VMAXUVX | VMFEQVF | VMFGEVF | VMFGTVF
            | VMFLEVF | VMFLTVF | VMFNEVF | VMINVX | VMINUVX | VMSEQVX | VMSGTVX | VMSGTUVX
            | VMSLEVX | VMSLEUVX | VMSLTVX | VMSLTUVX | VMSNEVX | VMULVX | VMULHVX | VMULHSUVX
            | VMULHUVX | VNCLIPWX | VNCLIPUWX | VNMSACVX | VNMSUBVX | VNSRAWX | VNSRLWX | VORVX
            | VREMVX | VREMUVX | VRGATHERVX | VROLVX | VRORVX | VRSUBVX | VSADDVX | VSADDUVX
            | VSLIDE1DOWNVX | VSLIDE1UPVX | VSLIDEDOWNVX | VSLIDEUPVX | VSLLVX | VSMULVX
            | VSRAVX | VSRLVX | VSSRAVX | VSSRLVX | VSSUBVX | VSSUBUVX | VSUBVX | VWADDVX
            | VWADDWX | VWADDUVX | VWADDUWX | VWMACCVX | VWMACCSUVX | VWMACCUVX | VWMACCUSVX
            | VWMULVX | VWMULSUVX | VWMULUVX | VWSLLVX | VWSUBVX | VWSUBWX | VWSUBUVX
            | VWSUBUWX | VXORVX => Encoding::VmVs2Rs1Vd,
            VADDVI | VANDVI | VMSEQVI | VMSGTVI | VMSGTUVI | VMSLEVI | VMSLEUVI | VMSNEVI
            | VORVI | VRSUBVI | VSADDVI | VSADDUVI | VXORVI => Encoding::VmVs2Simm5Vd,
            VBREV8V | VBREVV | VCLZV | VCPOPV | VCTZV | VFCLASSV | VFCVTFXV | VFCVTFXUV
            | VFCVTRTZXFV | VFCVTRTZXUFV | VFCVTXFV | VFCVTXUFV | VFNCVTFFW | VFNCVTFXW
            | VFNCVTFXUW | VFNCVTRODFFW | VFNCVTRTZXFW | VFNCVTRTZXUFW | VFNCVTXFW | VFNCVTXUFW
            | VFNCVTBF16FFW | VFREC7V | VFRSQRT7V | VFSQRTV | VFWCVTFFV | VFWCVTFXV
            | VFWCVTFXUV | VFWCVTRTZXFV | VFWCVTRTZXUFV | VFWCVTXFV | VFWCVTXUFV
            | VFWCVTBF16FFV | VIOTAM | VMSBFM | VMSIFM | VMSOFM | VREV8V | VSEXTVF2 | VSEXTVF4
            | VSEXTVF8 | VZEXTVF2 | VZEXTVF4 | VZEXTVF8 => Encoding::VmVs2Vd,
            VAADDVV | VAADDUVV | VADDVV | VANDVV | VANDNVV | VASUBVV | VASUBUVV | VCLMULVV
            | VCLMULHVV | VDIVVV | VDIVUVV | VFADDVV | VFDIVVV | VFMACCVV | VFMADDVV | VFMAXVV
            | VFMINVV | VFMSACVV | VFMSUBVV | VFMULVV | VFNMACCVV | VFNMADDVV | VFNMSACVV
            | VFNMSUBVV | VFREDMAXVS | VFREDMINVS | VFREDOSUMVS | VFREDSUMVS | VFREDUSUMVS
            | VFSGNJVV | VFSGNJNVV | VFSGNJXVV | VFSUBVV | VFWADDVV | VFWADDWV | VFWMACCVV
            | VFWMACCBF16VV | VFWMSACVV | VFWMULVV | VFWNMACCVV | VFWNMSACVV | VFWREDOSUMVS
            | VFWREDSUMVS | VFWREDUSUMVS | VFWSUBVV | VFWSUBWV | VMACCVV | VMADDVV | VMANDNOTMM
            | VMAXVV | VMAXUVV | VMFEQVV | VMFLEVV | VMFLTVV | VMFNEVV | VMINVV | VMINUVV
            | VMORNOTMM | VMSEQVV | VMSLEVV | VMSLEUVV | VMSLTVV | VMSLTUVV | VMSNEVV | VMULVV
            | VMULHVV | VMULHSUVV | VMULHUVV | VNCLIPWV | VNCLIPUWV | VNMSACVV | VNMSUBVV
            | VNSRAWV | VNSRLWV | VORVV | VREDANDVS | VREDMAXVS | VREDMAXUVS | VREDMINVS
            | VREDMINUVS | VREDORVS | VREDSUMVS | VREDXORVS | VREMVV | VREMUVV | VRGATHERVV
            | VRGATHEREI16VV | VROLVV | VRORVV | VSADDVV | VSADDUVV | VSLLVV | VSMULVV | VSRAVV
            | VSRLVV | VSSRAVV | VSSRLVV | VSSUBVV | VSSUBUVV | VSUBVV | VWADDVV | VWADDWV
            | VWADDUVV | VWADDUWV | VWMACCVV | VWMACCSUVV | VWMACCUVV | VWMULVV | VWMULSUVV
            | VWMULUVV | VWREDSUMVS | VWREDSUMUVS | VWSLLVV | VWSUBVV | VWSUBWV | VWSUBUVV
            | VWSUBUWV | VXORVV => Encoding::VmVs2Vs1Vd,
            VNCLIPWI | VNCLIPUWI | VNSRAWI | VNSRLWI | VRGATHERVI | VSLIDEDOWNVI | VSLIDEUPVI
            | VSLLVI | VSRAVI | VSRLVI | VSSRAVI | VSSRLVI | VWSLLVI => Encoding::VmVs2Zimm5Vd,
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
            VSETIVLI => Encoding::Zimm10Zimm5Rd,
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
pub const INSN_FIELD_RD_P_E: u32 = 0x18;
pub const INSN_FIELD_RD_P_E_START: u32 = 3;
pub const INSN_FIELD_RD_P_E_SIZE: u32 = 2;
pub const INSN_FIELD_RS2_P_E: u32 = 0x18;
pub const INSN_FIELD_RS2_P_E_START: u32 = 3;
pub const INSN_FIELD_RS2_P_E_SIZE: u32 = 2;
pub const INSN_FIELD_RD_N0_E: u32 = 0xf00;
pub const INSN_FIELD_RD_N0_E_START: u32 = 8;
pub const INSN_FIELD_RD_N0_E_SIZE: u32 = 4;
pub const INSN_FIELD_C_RS2_E: u32 = 0x78;
pub const INSN_FIELD_C_RS2_E_START: u32 = 3;
pub const INSN_FIELD_C_RS2_E_SIZE: u32 = 4;
pub const INSN_FIELD_RD_E: u32 = 0xf00;
pub const INSN_FIELD_RD_E_START: u32 = 8;
pub const INSN_FIELD_RD_E_SIZE: u32 = 4;
pub const INSN_FIELD_RS2_E: u32 = 0x1e00000;
pub const INSN_FIELD_RS2_E_START: u32 = 21;
pub const INSN_FIELD_RS2_E_SIZE: u32 = 4;
pub const INSN_FIELD_P_IMM10: u32 = 0x1ff8000;
pub const INSN_FIELD_P_IMM10_START: u32 = 15;
pub const INSN_FIELD_P_IMM10_SIZE: u32 = 10;
pub const INSN_FIELD_P_IMM8: u32 = 0xff0000;
pub const INSN_FIELD_P_IMM8_START: u32 = 16;
pub const INSN_FIELD_P_IMM8_SIZE: u32 = 8;
pub const INSN_FIELD_P_W_UIMM3: u32 = 0x700000;
pub const INSN_FIELD_P_W_UIMM3_START: u32 = 20;
pub const INSN_FIELD_P_W_UIMM3_SIZE: u32 = 3;
pub const INSN_FIELD_P_W_UIMM4: u32 = 0xf00000;
pub const INSN_FIELD_P_W_UIMM4_START: u32 = 20;
pub const INSN_FIELD_P_W_UIMM4_SIZE: u32 = 4;
pub const INSN_FIELD_P_W_UIMM5: u32 = 0x1f00000;
pub const INSN_FIELD_P_W_UIMM5_START: u32 = 20;
pub const INSN_FIELD_P_W_UIMM5_SIZE: u32 = 5;
pub const INSN_FIELD_P_W_UIMM6: u32 = 0x3f00000;
pub const INSN_FIELD_P_W_UIMM6_START: u32 = 20;
pub const INSN_FIELD_P_W_UIMM6_SIZE: u32 = 6;
pub const INSN_FIELD_P_W: u32 = 0x6000000;
pub const INSN_FIELD_P_W_START: u32 = 25;
pub const INSN_FIELD_P_W_SIZE: u32 = 2;
pub const INSN_FIELD_P_RD_P: u32 = 0xf00;
pub const INSN_FIELD_P_RD_P_START: u32 = 8;
pub const INSN_FIELD_P_RD_P_SIZE: u32 = 4;
pub const INSN_FIELD_P_RS1_P: u32 = 0xf0000;
pub const INSN_FIELD_P_RS1_P_START: u32 = 16;
pub const INSN_FIELD_P_RS1_P_SIZE: u32 = 4;
pub const INSN_FIELD_P_RS2_P: u32 = 0x1e00000;
pub const INSN_FIELD_P_RS2_P_START: u32 = 21;
pub const INSN_FIELD_P_RS2_P_SIZE: u32 = 4;
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
    pub const fn rd_p_e(self) -> u32 {
        (self.value >> INSN_FIELD_RD_P_E_START) & ((1 << INSN_FIELD_RD_P_E_SIZE) - 1)
    }

    pub const fn set_rd_p_e(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RD_P_E;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RD_P_E_SIZE) - 1)) << INSN_FIELD_RD_P_E_START;
        self
    }
    pub const fn rs2_p_e(self) -> u32 {
        (self.value >> INSN_FIELD_RS2_P_E_START) & ((1 << INSN_FIELD_RS2_P_E_SIZE) - 1)
    }

    pub const fn set_rs2_p_e(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RS2_P_E;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RS2_P_E_SIZE) - 1)) << INSN_FIELD_RS2_P_E_START;
        self
    }
    pub const fn rd_n0_e(self) -> u32 {
        (self.value >> INSN_FIELD_RD_N0_E_START) & ((1 << INSN_FIELD_RD_N0_E_SIZE) - 1)
    }

    pub const fn set_rd_n0_e(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RD_N0_E;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RD_N0_E_SIZE) - 1)) << INSN_FIELD_RD_N0_E_START;
        self
    }
    pub const fn c_rs2_e(self) -> u32 {
        (self.value >> INSN_FIELD_C_RS2_E_START) & ((1 << INSN_FIELD_C_RS2_E_SIZE) - 1)
    }

    pub const fn set_c_rs2_e(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_C_RS2_E;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_C_RS2_E_SIZE) - 1)) << INSN_FIELD_C_RS2_E_START;
        self
    }
    pub const fn rd_e(self) -> u32 {
        (self.value >> INSN_FIELD_RD_E_START) & ((1 << INSN_FIELD_RD_E_SIZE) - 1)
    }

    pub const fn set_rd_e(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RD_E;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RD_E_SIZE) - 1)) << INSN_FIELD_RD_E_START;
        self
    }
    pub const fn rs2_e(self) -> u32 {
        (self.value >> INSN_FIELD_RS2_E_START) & ((1 << INSN_FIELD_RS2_E_SIZE) - 1)
    }

    pub const fn set_rs2_e(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_RS2_E;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_RS2_E_SIZE) - 1)) << INSN_FIELD_RS2_E_START;
        self
    }
    pub const fn p_imm10_raw(self) -> u32 {
        (self.value >> INSN_FIELD_P_IMM10_START) & ((1 << INSN_FIELD_P_IMM10_SIZE) - 1)
    }

    pub const fn set_p_imm10_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_P_IMM10;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_P_IMM10_SIZE) - 1)) << INSN_FIELD_P_IMM10_START;
        self
    }
    pub const fn p_imm8_raw(self) -> u32 {
        (self.value >> INSN_FIELD_P_IMM8_START) & ((1 << INSN_FIELD_P_IMM8_SIZE) - 1)
    }

    pub const fn set_p_imm8_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_P_IMM8;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_P_IMM8_SIZE) - 1)) << INSN_FIELD_P_IMM8_START;
        self
    }
    pub const fn p_w_uimm3_raw(self) -> u32 {
        (self.value >> INSN_FIELD_P_W_UIMM3_START) & ((1 << INSN_FIELD_P_W_UIMM3_SIZE) - 1)
    }

    pub const fn set_p_w_uimm3_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_P_W_UIMM3;
        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_P_W_UIMM3_SIZE) - 1)) << INSN_FIELD_P_W_UIMM3_START;
        self
    }
    pub const fn p_w_uimm4_raw(self) -> u32 {
        (self.value >> INSN_FIELD_P_W_UIMM4_START) & ((1 << INSN_FIELD_P_W_UIMM4_SIZE) - 1)
    }

    pub const fn set_p_w_uimm4_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_P_W_UIMM4;
        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_P_W_UIMM4_SIZE) - 1)) << INSN_FIELD_P_W_UIMM4_START;
        self
    }
    pub const fn p_w_uimm5_raw(self) -> u32 {
        (self.value >> INSN_FIELD_P_W_UIMM5_START) & ((1 << INSN_FIELD_P_W_UIMM5_SIZE) - 1)
    }

    pub const fn set_p_w_uimm5_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_P_W_UIMM5;
        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_P_W_UIMM5_SIZE) - 1)) << INSN_FIELD_P_W_UIMM5_START;
        self
    }
    pub const fn p_w_uimm6_raw(self) -> u32 {
        (self.value >> INSN_FIELD_P_W_UIMM6_START) & ((1 << INSN_FIELD_P_W_UIMM6_SIZE) - 1)
    }

    pub const fn set_p_w_uimm6_raw(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_P_W_UIMM6;
        self.value &= !mask;
        self.value |=
            (value & ((1 << INSN_FIELD_P_W_UIMM6_SIZE) - 1)) << INSN_FIELD_P_W_UIMM6_START;
        self
    }
    pub const fn p_w(self) -> u32 {
        (self.value >> INSN_FIELD_P_W_START) & ((1 << INSN_FIELD_P_W_SIZE) - 1)
    }

    pub const fn set_p_w(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_P_W;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_P_W_SIZE) - 1)) << INSN_FIELD_P_W_START;
        self
    }
    pub const fn p_rd_p(self) -> u32 {
        (self.value >> INSN_FIELD_P_RD_P_START) & ((1 << INSN_FIELD_P_RD_P_SIZE) - 1)
    }

    pub const fn set_p_rd_p(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_P_RD_P;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_P_RD_P_SIZE) - 1)) << INSN_FIELD_P_RD_P_START;
        self
    }
    pub const fn p_rs1_p(self) -> u32 {
        (self.value >> INSN_FIELD_P_RS1_P_START) & ((1 << INSN_FIELD_P_RS1_P_SIZE) - 1)
    }

    pub const fn set_p_rs1_p(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_P_RS1_P;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_P_RS1_P_SIZE) - 1)) << INSN_FIELD_P_RS1_P_START;
        self
    }
    pub const fn p_rs2_p(self) -> u32 {
        (self.value >> INSN_FIELD_P_RS2_P_START) & ((1 << INSN_FIELD_P_RS2_P_SIZE) - 1)
    }

    pub const fn set_p_rs2_p(mut self, value: u32) -> Self {
        let mask = INSN_FIELD_P_RS2_P;
        self.value &= !mask;
        self.value |= (value & ((1 << INSN_FIELD_P_RS2_P_SIZE) - 1)) << INSN_FIELD_P_RS2_P_START;
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

    /// c_uimm9splohi
    pub const fn c_uimm9splohi(self) -> u32 {
        decode_immediate(&C_UIMM9SPLOHI, self.value as _) as _
    }

    pub const fn set_c_uimm9splohi(mut self, c_uimm9splohi: u32) -> Self {
        self.value |= encode_immediate(&C_UIMM9SPLOHI, c_uimm9splohi as _);
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
