#[derive(Copy, Clone)]
pub struct Field {
    pub pos: (u32, u32),
    pub name: &'static str,
}

pub const OPCODE: Field = Field {
    pos: (6, 7),
    name: "opcode",
};
pub const RD: Field = Field {
    pos: (11, 5),
    name: "rd",
};
pub const FUNCT3: Field = Field {
    pos: (14, 3),
    name: "funct3",
};
pub const RS1: Field = Field {
    pos: (19, 5),
    name: "rs1",
};
pub const RS2: Field = Field {
    pos: (24, 5),
    name: "rs2",
};

pub const R_FUNCT5: Field = Field {
    pos: (31, 5),
    name: "funct5",
};
pub const R_FUNCT7: Field = Field {
    pos: (31, 7),
    name: "funct7",
};
pub const R_AQ: Field = Field {
    pos: (26, 1),
    name: "aq",
};
pub const R_RL: Field = Field {
    pos: (25, 1),
    name: "rl",
};
pub const R_FP_FMT: Field = Field {
    pos: (26, 2),
    name: "fmt",
};
pub const R_FP_RM: Field = Field {
    pos: (14, 3),
    name: "rm",
};

pub const I_IMM_11_0: Field = Field {
    pos: (31, 12),
    name: "imm[11:0]",
};
pub const I_SHTYP_11_7: Field = Field {
    pos: (31, 5),
    name: "shtyp[11:7]",
};
pub const I_SHTYP_11_6: Field = Field {
    pos: (31, 6),
    name: "shtyp[11:6]",
};
pub const I_SHTYP_11_5: Field = Field {
    pos: (31, 7),
    name: "shtyp[11:5]",
};
pub const I_SHTYP: Field = Field {
    pos: (30, 1),
    name: "shtyp",
};
pub const I_SHAMT_6: Field = Field {
    pos: (26, 1),
    name: "shamt[6]",
};
pub const I_SHAMT_6_0: Field = Field {
    pos: (26, 7),
    name: "shamt[6:0]",
};
pub const I_SHAMT_5: Field = Field {
    pos: (25, 1),
    name: "shamt[5]",
};
pub const I_SHAMT_5_0: Field = Field {
    pos: (25, 6),
    name: "shamt[5:0]",
};
pub const I_SHAMT: Field = Field {
    pos: (24, 5),
    name: "shamt[4:0]",
};

pub const I_FUNCT12: Field = Field {
    pos: (31, 12),
    name: "funct12",
};
pub const I_CSR: Field = Field {
    pos: (31, 12),
    name: "csr",
};
pub const I_IMM_4_0: Field = Field {
    pos: (19, 5),
    name: "imm[4:0]",
};

pub const I_FM: Field = Field {
    pos: (31, 4),
    name: "fm",
};
pub const I_PRED: Field = Field {
    pos: (27, 4),
    name: "pred",
};
pub const I_SUCC: Field = Field {
    pos: (23, 4),
    name: "succ",
};

pub const S_IMM_4_0: Field = Field {
    pos: (11, 5),
    name: "imm[4:0]",
};
pub const S_IMM_11_5: Field = Field {
    pos: (31, 7),
    name: "imm[11:5]",
};

pub const B_IMM_4_1: Field = Field {
    pos: (11, 4),
    name: "imm[4:1]",
};
pub const B_IMM_11: Field = Field {
    pos: (7, 1),
    name: "imm[11]",
};
pub const B_IMM_10_5: Field = Field {
    pos: (30, 6),
    name: "imm[10:5]",
};
pub const B_IMM_12: Field = Field {
    pos: (31, 1),
    name: "imm[12]",
};

pub const U_IMM_31_12: Field = Field {
    pos: (31, 20),
    name: "imm[31:12]",
};

pub const J_IMM_20: Field = Field {
    pos: (31, 1),
    name: "imm[20]",
};
pub const J_IMM_10_1: Field = Field {
    pos: (30, 10),
    name: "imm[10:1]",
};
pub const J_IMM_11: Field = Field {
    pos: (20, 1),
    name: "imm[11]",
};
pub const J_IMM_19_12: Field = Field {
    pos: (19, 8),
    name: "imm[19:12]",
};

// ISA_C
pub const C_OPCODE: Field = Field {
    pos: (1, 2),
    name: "opcode",
};
pub const C_FUNCT6: Field = Field {
    pos: (15, 6),
    name: "funct6",
};
pub const C_FUNCT4: Field = Field {
    pos: (15, 4),
    name: "funct4",
};
pub const C_FUNCT3: Field = Field {
    pos: (15, 3),
    name: "funct3",
};
pub const C_FUNCT2: Field = Field {
    pos: (6, 2),
    name: "funct2",
};
pub const C_FUNCT2_CB: Field = Field {
    pos: (11, 2),
    name: "funct2",
};

pub const C_RD: Field = Field {
    pos: (11, 5),
    name: "rd",
};
pub const C_RS1: Field = Field {
    pos: (11, 5),
    name: "rs1",
};
pub const C_RD_RS1: Field = Field {
    pos: (11, 5),
    name: "rd/rs1",
};
pub const C_RS2: Field = Field {
    pos: (6, 5),
    name: "rs2",
};
pub const C_RD_PRIME: Field = Field {
    pos: (4, 3),
    name: "rd'",
};
pub const C_RS2_PRIME: Field = Field {
    pos: (4, 3),
    name: "rs2'",
};
pub const C_RS1_PRIME: Field = Field {
    pos: (9, 3),
    name: "rs1'",
};
pub const C_RD_RS1_PRIME: Field = Field {
    pos: (9, 3),
    name: "rd'/rs1'",
};

// ISA_C immediates
pub const C_IMM_CI_0: Field = Field {
    pos: (12, 1),
    name: "imm",
};
pub const C_IMM_CI_1: Field = Field {
    pos: (6, 5),
    name: "imm",
};
pub const C_IMM_CSS: Field = Field {
    pos: (12, 6),
    name: "imm",
};
pub const C_IMM_CIW: Field = Field {
    pos: (12, 8),
    name: "imm",
};
pub const C_IMM_CL_0: Field = Field {
    pos: (12, 3),
    name: "imm",
};
pub const C_IMM_CL_1: Field = Field {
    pos: (6, 2),
    name: "imm",
};
pub const C_IMM_CS_0: Field = Field {
    pos: (12, 3),
    name: "imm",
};
pub const C_IMM_CS_1: Field = Field {
    pos: (6, 2),
    name: "imm",
};
pub const C_IMM_CB_0: Field = Field {
    pos: (12, 3),
    name: "imm",
};
pub const C_IMM_CB_1: Field = Field {
    pos: (6, 5),
    name: "imm",
};
pub const C_IMM_CJ: Field = Field {
    pos: (12, 11),
    name: "imm",
};
pub const C_SHAMT_0: Field = Field {
    pos: (12, 1),
    name: "shamt",
};
pub const C_SHAMT_1: Field = Field {
    pos: (6, 5),
    name: "shamt",
};
