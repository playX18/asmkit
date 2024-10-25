//! # RISC-V instruction encoder
//!
//! All functions in this module are auto-generated. It's possible to use this module for both decoding and encoding.
//!
//! Main goal of the encoder is to be as fast as possible and `pub const fn` if possible.
//!
//! # Notes
//! - Arguments to opcodes are verified primitively: we just check their mask
//! - Short opcodes are supported, they're prefixed with `c_` and return `u16` on success
//! - Immediate arguments are not combined into one and then encoded, it's a job of the user
//! of this library to do it manually e.g to have `BImmediate` type that decomposes to `bimm12hi` and `bimm12lo`.
//!
//! # WARNING
//! Auto-generated from riscv-opcodes repo tables.

mod fields;
mod token;
use crate::AsmError;
//include!(concat!(env!("OUT_DIR"), "/inst_rv.rs"));

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

pub const B_TYPE: &[Immediate] = &[
    B_TYPE_IMM_12,
    B_TYPE_IMM_11,
    B_TYPE_IMM_10_5,
    B_TYPE_IMM_4_1,
];
pub const S_TYPE: &[Immediate] = &[S_TYPE_HI, S_TYPE_LO];
pub const U_TYPE: &[Immediate] = &[UIMM_LO];
pub const J_TYPE: &[Immediate] = &[
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

pub const ZIMM6: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (26, 26),
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (19, 15),
    },
];
pub const ZIMM11: Immediate = Immediate {
    position_in_immediate: (10, 0),
    position_in_opcode: (30, 20),
};

pub const ZIMM10: Immediate = Immediate {
    position_in_immediate: (9, 0),
    position_in_opcode: (29, 20),
};

pub const ZIMM5: Immediate = Immediate {
    position_in_immediate: (4, 0),
    position_in_opcode: (19, 15),
};

pub const C_NZUIMM10: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 4),
        position_in_opcode: (12, 11),
        
    },
    Immediate {
        position_in_immediate: (9, 6),
        position_in_opcode: (10, 7)
    },

    Immediate {
        position_in_immediate: (2, 2),
        position_in_opcode: (6, 6)
    },
    Immediate {
        position_in_immediate: (3, 3),
        position_in_opcode: (5, 5)
    }
];

// imm[17] | ... | imm[16:12]
pub const C_NZUIMM18: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (12, 12)
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (6, 2)
    }
];

pub const C_NZUIMM6: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (13, 13),
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (7, 2),
    },
];

pub const C_UIMM8: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 3),
        position_in_opcode: (12, 10),
    },
    Immediate {
        position_in_immediate: (7, 6),
        position_in_opcode: (7, 6),
    },
];

pub const C_UIMM9SP: &[Immediate] = &[
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

pub const C_NZIMM6: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (12, 12)
    },
    Immediate {
        position_in_immediate: (4, 0),
        position_in_opcode: (6, 2)
    },  
];

pub const C_BIMM9: &[Immediate] = &[
    Immediate {
        position_in_immediate: (8, 8),
        position_in_opcode: (12, 12)
    },
    Immediate {
        position_in_immediate: (4, 3),
        position_in_opcode: (11, 10)
    },
    Immediate {
        position_in_immediate: (7, 6),
        position_in_opcode: (6, 5)
    },
    Immediate {
        position_in_immediate: (2, 1),
        position_in_opcode: (4, 3)
    },
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (2, 2)
    }
];

pub const C_UIMM8SP: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (12, 12)
    },
    Immediate {
        position_in_immediate: (4, 2),
        position_in_opcode: (6, 4),
    },
    Immediate {
        position_in_immediate: (7, 6),
        position_in_opcode: (3, 2)
    }
];

pub const C_UIMM8SP_S: &[Immediate] = &[
    Immediate {
        position_in_immediate: (5, 2),
        position_in_opcode: (12, 9)
    },
    Immediate {
        position_in_immediate: (7, 6),
        position_in_opcode: (8, 7),
    },
];


pub const C_IMM12: &[Immediate] = &[
    Immediate {
        position_in_immediate: (11, 11),
        position_in_opcode: (12, 12)
    },
    Immediate {
        position_in_immediate: (4, 4),
        position_in_opcode: (11, 11)
    },
    Immediate {
        position_in_immediate: (9, 8),
        position_in_opcode: (10, 9)
    },
    Immediate {
        position_in_immediate: (10, 10),
        position_in_opcode: (8, 8)
    },
    Immediate {
        position_in_immediate: (6, 6),
        position_in_opcode: (7, 7)
    },
    Immediate {
        position_in_immediate: (7, 7),
        position_in_opcode: (6, 6)
    },
    Immediate {
        position_in_immediate: (3, 1),
        position_in_opcode: (5, 3)
    },
    Immediate {
        position_in_immediate: (5, 5),
        position_in_opcode: (2, 2)
    }
];

pub const C_I_TYPE: &[Immediate] = &[
    Immediate {
        position_in_opcode: (12, 12),
        position_in_immediate: (5, 5),
    },
    Immediate {
        position_in_opcode: (6, 2),
        position_in_immediate: (4, 0),
    },
];
// nzimm[5]: The highest bit of the non-zero immediate
pub const CI_TYPE_NZIMM_5: Immediate = Immediate {
    position_in_immediate: (5, 5),
    position_in_opcode: (12, 12), // Bit 12
};

// nzimm[4:0]: The lower bits of the non-zero immediate
pub const CI_TYPE_NZIMM_4_0: Immediate = Immediate {
    position_in_immediate: (4, 0),
    position_in_opcode: (6, 2), // Bits 6 to 2
};

pub const CI_TYPE_NZIMM: &[Immediate] = &[CI_TYPE_NZIMM_5, CI_TYPE_NZIMM_4_0];

impl Immediate {
    pub const fn encode(&self, imm: i32) -> u32 {
        let imm = imm as u32;
        let bit_count = self.position_in_immediate.0 - self.position_in_immediate.1 + 1;
        let mask = (1u32 << bit_count) - 1;

        (((imm >> self.position_in_immediate.1) as u32 & mask) << self.position_in_opcode.1) as u32
    }
}
