use core::fmt::{self, Write};

use super::{
    decode::{Decoder, Instruction},
    opcodes::Encoding,
    opcodes::OPCODE_STR,
    Opcode,
};

pub struct Formatter {
    abi_names: bool,
}

impl Formatter {
    pub const fn new() -> Self {
        Self { abi_names: !true }
    }

    fn write_reg<W: Write>(&self, out: &mut W, f: bool, vec: bool, reg: u32) -> fmt::Result {
        if vec && self.abi_names {
            match reg {
                0..=7 => write!(out, "v{}", reg),
                8..=15 => write!(out, "vs{}", reg - 8),
                16..=31 => write!(out, "v{}", reg),
                _ => write!(out, "<invalid>"),
            }?;
        } else if vec {
            write!(out, "v{}", reg)?;
        }

        if f && self.abi_names {
            match reg {
                0 => write!(out, "ft0"),
                1 => write!(out, "ft1"),
                2 => write!(out, "ft2"),
                3 => write!(out, "ft3"),
                4 => write!(out, "ft4"),
                5 => write!(out, "ft5"),
                6 => write!(out, "ft6"),
                7 => write!(out, "ft7"),
                8 => write!(out, "fs0"),
                9 => write!(out, "fs1"),
                10 => write!(out, "fa0"),
                11 => write!(out, "fa1"),
                12 => write!(out, "fa2"),
                13 => write!(out, "fa3"),
                14 => write!(out, "fa4"),
                15 => write!(out, "fa5"),
                16 => write!(out, "fa6"),
                17 => write!(out, "fa7"),
                18 => write!(out, "fs2"),
                19 => write!(out, "fs3"),
                20 => write!(out, "fs4"),
                21 => write!(out, "fs5"),
                22 => write!(out, "fs6"),
                23 => write!(out, "fs7"),
                24 => write!(out, "fs8"),
                25 => write!(out, "fs9"),
                26 => write!(out, "fs10"),
                27 => write!(out, "fs11"),
                28 => write!(out, "ft8"),
                29 => write!(out, "ft9"),
                30 => write!(out, "ft10"),
                31 => write!(out, "ft11"),
                _ => write!(out, "<invalid>"),
            }
        } else if f {
            write!(out, "f{}", reg)
        } else {
            if self.abi_names {
                match reg {
                    0 => write!(out, "zero"),
                    1 => write!(out, "ra"),
                    2 => write!(out, "sp"),
                    3 => write!(out, "gp"),
                    4 => write!(out, "tp"),
                    5 => write!(out, "t0"),
                    6 => write!(out, "t1"),
                    7 => write!(out, "t2"),
                    8 => write!(out, "s0"),
                    9 => write!(out, "s1"),
                    10 => write!(out, "a0"),
                    11 => write!(out, "a1"),
                    12 => write!(out, "a2"),
                    13 => write!(out, "a3"),
                    14 => write!(out, "a4"),
                    15 => write!(out, "a5"),
                    16 => write!(out, "a6"),
                    17 => write!(out, "a7"),
                    18 => write!(out, "s2"),
                    19 => write!(out, "s3"),
                    20 => write!(out, "s4"),
                    21 => write!(out, "s5"),
                    22 => write!(out, "s6"),
                    23 => write!(out, "s7"),
                    24 => write!(out, "s8"),
                    25 => write!(out, "s9"),
                    26 => write!(out, "s10"),
                    27 => write!(out, "s11"),
                    28 => write!(out, "t3"),
                    29 => write!(out, "t4"),
                    30 => write!(out, "t5"),
                    31 => write!(out, "t6"),
                    _ => write!(out, "<invalid>"),
                }
            } else {
                write!(out, "x{}", reg)
            }
        }
    }

    fn write_rlist<W: Write>(&self, out: &mut W, rlist: u32) -> fmt::Result {
        for i in [27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 9, 8, 1] {
            if rlist & (1 << i) != 0 {
                self.write_reg(out, false, false, i)?;
                write!(out, " ")?;
            }
        }

        Ok(())
    }

    pub fn format<W: Write>(&self, out: &mut W, inst: &Instruction) -> fmt::Result {
        let encoding = inst.code.encoding();
        let mnem = OPCODE_STR[inst.code as usize];

        write!(out, "{}", mnem)?;

        match encoding {
            Encoding::Bimm12HiRs1Bimm12lo => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                let addr =
                    inst.address as isize as isize + inst.value.bimm12lohi() as isize;
                write!(out, ", {:x}", addr)?;
            }

            Encoding::Bimm12HiRs1Rs2Bimm12lo => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs2())?;
                let addr =
                    inst.address as isize  + inst.value.bimm12lohi() as isize;
                write!(out, ", {:x}", addr)?;
            }

            Encoding::Bimm12HiRs2Bimm12lo => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rs2())?;
                let addr =
                    inst.address as isize + inst.len as isize + inst.value.bimm12lohi() as isize;
                write!(out, ", {:x}", addr)?;
            }

            Encoding::Bimm12HiRs2Rs1Bimm12lo => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rs2())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                let addr =
                    inst.address as isize + inst.len as isize + inst.value.bimm12lohi() as isize;
                write!(out, ", {:x}", addr)?;
            }

            Encoding::CImm12 => {
                write!(out, " ")?;
                let addr = inst.address as i64  + inst.value.c_imm12() as i64;
                write!(out, "{:x}", addr)?;
            }

            Encoding::CIndex => {
                write!(out, " ")?;
                write!(out, "{}", inst.value.c_index())?;
            }

            Encoding::CMopT => {
                write!(out, " ")?;
                write!(out, "{}", inst.value.c_mop_t())?;
            }

            Encoding::CNzimm10hiCNzimm10lo => {
                write!(out, " ")?;
                write!(out, "{}", inst.value.c_nzimm10lohi())?;
            }

            Encoding::CNzimm6hiCNzimm6lo => {
                write!(out, " ")?;
                write!(out, "{}", inst.value.c_nzimm6lohi())?;
            }

            Encoding::CRlistCSpimm => {
                write!(out, " ")?;
                let rlist = inst.value.c_rlist();
                self.write_rlist(out, rlist)?;
                write!(out, " {}", inst.value.c_spimm())?;
            }

            Encoding::CRs1N0 => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.c_rs1_n0())?;
            }

            Encoding::CRs2CUimm8spS => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.c_rs2())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_uimm8sp_s())?;
            }

            Encoding::CRs2CUimm9spS => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.c_rs2())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_uimm9sp_s())?;
            }

            Encoding::CSreg1CSreg2 => {
                // TODO: Broken, implement sreg decoding/encoding
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.c_sreg1())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.c_sreg2())?;
            }

            Encoding::CsrZimm => {
                let zimm = inst.value.zimm();
                write!(out, " ")?;
                write!(out, "csr{}, {}", inst.value.csr(), zimm)?;
            }

            Encoding::Empty => {}
            Encoding::FmPredSuccRs1Rd => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.pred())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.succ())?;
            }

            // <op> rs2, offset(rs1)
            Encoding::Imm12HiRs1Rs2Imm12lo => {
                write!(out, " ")?;
                let f = matches!(
                    inst.code,
                    Opcode::FSD | Opcode::FSH | Opcode::FSQ | Opcode::FSW
                );
                self.write_reg(out, f, false, inst.value.rs2())?;

                write!(out, ", {}(", inst.value.imm12lohi())?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ")")?;
            }

            Encoding::Imm12Rs1Rd => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.imm12())?;
            }

            Encoding::Jimm20 => {
                write!(out, " ")?;
                let addr = inst.address as i64 + inst.value.jimm20() as i64;
                write!(out, "{:x}", addr)?;
            }
            Encoding::RdJimm20 => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                let addr = inst.address as i64 + inst.value.jimm20() as i64;
                write!(out, "{:x}", addr)?;
            }
            Encoding::MopRT30MopRT2726MopRT2120RdRs1 => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
            }

            Encoding::MopRrT30MopRrT2726RdRs1Rs2 => {
                todo!()
            }

            Encoding::NfVmRs1Vd => {
                let vd = inst.value.vd();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();

                write!(out, " ")?;
                self.write_reg(out, false, true, vd)?;
                write!(out, ", (")?;
                self.write_reg(out, false, false, rs1)?;
                write!(out, ")")?;
                if vm == 0 {
                    write!(out, ", ")?;
                    write!(out, "v{}.t", vm)?;
                }
            }

            Encoding::NfVmRs1Vs3 => {
                let vs3 = inst.value.vs3();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();

                write!(out, " ")?;
                self.write_reg(out, false, true, vs3)?;
                write!(out, ", (")?;
                self.write_reg(out, false, false, rs1)?;
                write!(out, ")")?;
                if vm == 0 {
                    write!(out, ", ")?;
                    write!(out, "v{}.t", vm)?;
                }
            }

            // <op> vd, (rs1), rs2, vm
            Encoding::NfVmRs2Rs1Vd => {
                let vd = inst.value.vd();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();
                let rs2 = inst.value.rs2();

                write!(out, " ")?;
                self.write_reg(out, false, true, vd)?;
                write!(out, ", (")?;
                self.write_reg(out, false, false, rs1)?;
                write!(out, "), ")?;
                self.write_reg(out, false, false, rs2)?;
                if vm == 0 {
                    write!(out, ", ")?;
                    write!(out, "v{}.t", vm)?;
                }
            }

            // <op> v3, (rs1), rs2, vm
            Encoding::NfVmRs2Rs1Vs3 => {
                let vs3 = inst.value.vs3();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();
                let rs2 = inst.value.rs2();

                write!(out, " ")?;
                self.write_reg(out, false, true, vs3)?;
                write!(out, ", (")?;
                self.write_reg(out, false, false, rs1)?;
                write!(out, "), ")?;
                self.write_reg(out, false, false, rs2)?;
                if vm == 0 {
                    write!(out, ", ")?;
                    write!(out, "v{}.t", vm)?;
                }
            }

            // vd, (rs1), vs2
            Encoding::NfVmVs2Rs1Vd => {
                let vd = inst.value.vd();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();
                let vs2 = inst.value.vs2();

                write!(out, " ")?;
                self.write_reg(out, false, true, vd)?;
                write!(out, ", (")?;
                self.write_reg(out, false, false, rs1)?;
                write!(out, "), ")?;
                self.write_reg(out, false, true, vs2)?;
                if vm == 0 {
                    write!(out, ", ")?;
                    write!(out, "v{}.t", vm)?;
                }
            }

            Encoding::NfVmVs2Rs1Vs3 => {
                let vs3 = inst.value.vs3();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();
                let vs2 = inst.value.vs2();

                write!(out, " ")?;
                self.write_reg(out, false, true, vs3)?;
                write!(out, ", (")?;
                self.write_reg(out, false, false, rs1)?;
                write!(out, "), ")?;
                self.write_reg(out, false, true, vs2)?;
                if vm == 0 {
                    write!(out, ", ")?;
                    write!(out, "v{}.t", vm)?;
                }
            }

            Encoding::Rd => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
            }

            Encoding::RdCUimm9sphiCUimm9splo => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_uimm9splohi())?;
            }

            Encoding::RdCsrZimm => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                write!(out, "csr{}, {}", inst.value.csr(), inst.value.zimm())?;
            }

            Encoding::RdImm20 => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.imm20())?;
            }

            Encoding::RdN0CImm6loCImm6hi => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_imm6lohi())?;
            }

            Encoding::RdN0CRs2N0 => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs2())?;
            }

            Encoding::RdN0CUimm8sphiCUimm8splo => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_uimm8splohi())?;
            }

            Encoding::RdN0CUimm9sphiCUimm9splo => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_uimm9splohi())?;
            }

            Encoding::RdN2CNzimm18hiCNzimm18lo => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_nzimm18lohi())?;
            }

            Encoding::RdPCNzuimm10 => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_nzuimm10())?;
            }

            Encoding::RdPRs1PCUimm1 => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_uimm1())?;
            }

            Encoding::RdPRs1PCUimm2 => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_uimm2())?;
            }

            Encoding::RdPRs1PCUimm7loCUimm7hi => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", {}", inst.value.c_uimm7lohi())?;
            }

            Encoding::RdPRs1PCUimm8loCUimm8hi => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd_p() + 8)?;
                write!(out, ", ")?;

                self.write_reg(
                    out,
                    matches!(inst.code, Opcode::CFLD),
                    false,
                    inst.value.rs1_p() + 8,
                )?;
                write!(out, ", {}", inst.value.c_uimm8lohi())?;
            }

            Encoding::RdRs1 => match inst.code {
                Opcode::FLID | Opcode::FLIQ | Opcode::FLIH | Opcode::FLIS => {
                    self.format_fli(out, inst)?;
                }

                Opcode::FMVDX | Opcode::FMVHX | Opcode::FMVPQX | Opcode::FMVSX => {
                    write!(out, " ")?;
                    self.write_reg(out, true, false, inst.value.rd())?;
                    write!(out, ", ")?;
                    self.write_reg(out, false, false, inst.value.rs1())?;
                }

                Opcode::FMVXD | Opcode::FMVXS | Opcode::FMVXH | Opcode::FMVXW => {
                    write!(out, " ")?;
                    self.write_reg(out, false, false, inst.value.rd())?;
                    write!(out, ", ")?;
                    self.write_reg(out, true, false, inst.value.rs1())?;
                }

                Opcode::FCLASSD | Opcode::FCLASSH | Opcode::FCLASSQ | Opcode::FCLASSS => {
                    write!(out, " ")?;
                    self.write_reg(out, false, false, inst.value.rd())?;
                    write!(out, ", ")?;
                    self.write_reg(out, true, false, inst.value.rs1())?;
                }

                _ => {
                    write!(out, " ")?;
                    self.write_reg(out, false, false, inst.value.rd())?;
                    write!(out, ", ")?;
                    self.write_reg(out, false, false, inst.value.rs1())?;
                }
            },

            Encoding::RdRs1AqRl => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
            }
            // rd, csr, rs1
            Encoding::RdRs1Csr => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                write!(out, "csr{}, ", inst.value.csr())?;
                self.write_reg(out, false, false, inst.value.rs1())?;
            }

            // rd, offset(rs1)
            Encoding::RdRs1Imm12 => {
                let is_float = matches!(
                    inst.code,
                    Opcode::FLD | Opcode::FLH | Opcode::FLQ | Opcode::FLW
                );

                write!(out, " ")?;
                self.write_reg(out, is_float, false, inst.value.rd())?;
                write!(out, ", ")?;
                /*self.write_reg(out, is_float, false, inst.value.rs1())?;
                write!(out, ", {}", inst.value.imm12())?;
                */
                match inst.code {
                    Opcode::FLD
                    | Opcode::FLH
                    | Opcode::FLQ
                    | Opcode::FLW
                    | Opcode::LB
                    | Opcode::LBU
                    | Opcode::LD 
                    | Opcode::LH
                    | Opcode::LHU
                    | Opcode::LW
                    | Opcode::LWU => {
                        write!(out, "{}(", inst.value.imm12())?;
                        self.write_reg(out, false, false, inst.value.rs1())?;
                        write!(out, ")")?;
                    }

                    _ => {
                        self.write_reg(out, false, false, inst.value.rs1())?;
                        write!(out, ", {}", inst.value.imm12())?;
                    }
                }
            }

            // rd, rs1, imm6lohi
            Encoding::RdRs1N0CImm6loCImm6hi => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_imm6lohi())?;
            }

            // rd, rs1, nzimm6lohi
            Encoding::RdRs1N0CNzimm6loCNzimm6hi => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_nzimm6lohi())?;
            }

            // rd, rs1, nzuimm6lohi
            Encoding::RdRs1N0CNzuimm6hiCNzuimm6lo => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_nzuimm6lohi())?;
            }

            Encoding::RdRs1N0CRs2N0 => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.c_rs2())?;
            }

            Encoding::RdRs1P => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1_p())?;
            }

            Encoding::RdRs1PCImm6hiCImm6lo => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", ")?;
                write!(out, "{}", inst.value.c_imm6lohi())?;
            }

            Encoding::RdRs1PRs2P => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1_p())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs2_p())?;
            }

            Encoding::RdRs1Rm => {
                let (rd_is_f, rs1_is_f) = match inst.code {
                    Opcode::FCVTDH => (true, true),
                    Opcode::FCVTDL => (true, false),
                    Opcode::FCVTDLU => (true, false),
                    Opcode::FCVTDQ => (true, true),
                    Opcode::FCVTDS => (true, true),
                    Opcode::FCVTDW => (true, false),
                    Opcode::FCVTDWU => (true, false),
                    Opcode::FCVTHD => (true, true),
                    Opcode::FCVTHL => (true, false),
                    Opcode::FCVTHLU => (true, false),
                    Opcode::FCVTHQ => (true, true),
                    Opcode::FCVTHS => (true, true),
                    Opcode::FCVTHW => (true, false),
                    Opcode::FCVTHWU => (true, false),
                    Opcode::FCVTLD => (false, true),
                    Opcode::FCVTLUD => (false, true),
                    Opcode::FCVTLUH => (false, true),

                    Opcode::FCVTLH => (false, true),
                    Opcode::FCVTLQ => (false, true),
                    Opcode::FCVTLS => (false, true),
                    Opcode::FCVTLUQ => (false, true),
                    Opcode::FCVTLUS => (false, true),
                    Opcode::FCVTQD => (true, true),
                    Opcode::FCVTQH => (true, true),
                    Opcode::FCVTQL => (true, false),
                    Opcode::FCVTQLU => (true, false),
                    Opcode::FCVTQS => (true, true),
                    Opcode::FCVTQW => (true, false),
                    Opcode::FCVTQWU => (true, false),
                    Opcode::FCVTSD => (true, true),
                    Opcode::FCVTSH => (true, true),
                    Opcode::FCVTSL => (true, false),
                    Opcode::FCVTSLU => (true, false),
                    Opcode::FCVTSQ => (true, true),
                    Opcode::FCVTSW => (true, false),
                    Opcode::FCVTSWU => (true, false),
                    Opcode::FCVTWD => (false, true),
                    Opcode::FCVTWH => (false, true),
                    Opcode::FCVTWQ => (false, true),
                    Opcode::FCVTWS => (false, true),
                    Opcode::FCVTWUD => (false, true),
                    Opcode::FCVTWUH => (false, true),
                    Opcode::FCVTWUQ => (false, true),
                    Opcode::FCVTWUS => (false, true),
                    _ => (true, true),
                };

                write!(out, " ")?;
                self.write_reg(out, rd_is_f, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, rs1_is_f, false, inst.value.rs1())?;
            }
            Encoding::RdRs1Rs2 => {
                write!(out, " ")?;
                let is_f = mnem.starts_with("f");

                self.write_reg(out, is_f, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, is_f, false, inst.value.rs1())?;
                write!(out, ", ")?;
                self.write_reg(out, is_f, false, inst.value.rs2())?;
            }
            Encoding::RdRs1Shamtd => {
                write!(out, " ")?;
                self.write_reg(out, false, false, inst.value.rd())?;
                write!(out, ", ")?;
                self.write_reg(out, false, false, inst.value.rs1())?;
                write!(out, ", {}", inst.value.shamtd())?;
            }
            _ => todo!("{:?}", encoding),
        }

        Ok(())
    }

    fn format_fli<W: Write>(&self, out: &mut W, inst: &Instruction) -> fmt::Result {
        let rs1 = inst.value.rs1();
        let c = match rs1 {
            0 => "-1.0",
            1 => "0.0",
            2 => "1.0*2^-16",
            3 => "1.0*2^-15",
            4 => "1.0*2^-8",
            5 => "1.0*2^-7",
            6 => "0.0625",
            7 => "0.125",
            8 => "0.25",
            9 => "0.3125",
            10 => "0.375",
            11 => "0.4375",
            12 => "0.5",
            13 => "0.625",
            14 => "0.75",
            15 => "0.875",
            16 => "1.0",
            17 => "1.25",
            18 => "1.5",
            19 => "1.75",
            20 => "2.0",
            21 => "2.5",
            22 => "3",
            23 => "4",
            24 => "8",
            25 => "16",
            26 => "128",
            27 => "256",
            28 => "2^15",
            29 => "2^16",
            30 => "+inf",
            31 => "NaN",
            _ => "<invalid>",
        };

        write!(out, " ")?;
        self.write_reg(out, true, false, inst.value.rd())?;
        write!(out, ", {}", c)
    }
}

pub fn pretty_disassembler<W: Write>(
    out: &mut W,
    bitness: usize,
    data: &[u8],
    address: u64,
) -> fmt::Result {
    let mut decoder = Decoder::new(bitness, data, address);
    let mut fmt = Formatter::new();
    fmt.abi_names = true;

    let mut inst = Instruction::default();

    while decoder.can_decode() {
        decoder.decode_out(&mut inst);

        write!(out, "{:016x}: {:08x}  ", inst.address, inst.value.value)?;

        fmt.format(out, &inst)?;
        write!(out, "\n")?;
    }
    Ok(())
}
