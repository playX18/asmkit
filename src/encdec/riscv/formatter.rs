use crate::encdec::FormatterOutput;

use super::{decode::{Decoder, Instruction}, Encoding, Opcode, OPCODE_STR};

pub struct Formatter {
    abi_names: bool,
}

impl Formatter {
    pub const fn new() -> Self {
        Self { abi_names: true }
    }

    fn write_reg<W: FormatterOutput>(&self, out: &mut W, f: bool, vec: bool, reg: u32) {
        if vec && self.abi_names {
            match reg {
                0..=7 => out.write_fmt(format_args!("v{}", reg)),
                8..=15 => out.write_fmt(format_args!("vs{}", reg - 8)),
                16..=31 => out.write_fmt(format_args!("v{}", reg)),
                _ => out.write_str("<invalid>"),
            }
            return;
        } else if vec {
            out.write_fmt(format_args!("v{}", reg));
            return;
        }

        if f && self.abi_names {
            match reg {
                0 => out.write_str("ft0"),
                1 => out.write_str("ft1"),
                2 => out.write_str("ft2"),
                3 => out.write_str("ft3"),
                4 => out.write_str("ft4"),
                5 => out.write_str("ft5"),
                6 => out.write_str("ft6"),
                7 => out.write_str("ft7"),
                8 => out.write_str("fs0"),
                9 => out.write_str("fs1"),
                10 => out.write_str("fa0"),
                11 => out.write_str("fa1"),
                12 => out.write_str("fa2"),
                13 => out.write_str("fa3"),
                14 => out.write_str("fa4"),
                15 => out.write_str("fa5"),
                16 => out.write_str("fa6"),
                17 => out.write_str("fa7"),
                18 => out.write_str("fs2"),
                19 => out.write_str("fs3"),
                20 => out.write_str("fs4"),
                21 => out.write_str("fs5"),
                22 => out.write_str("fs6"),
                23 => out.write_str("fs7"),
                24 => out.write_str("fs8"),
                25 => out.write_str("fs9"),
                26 => out.write_str("fs10"),
                27 => out.write_str("fs11"),
                28 => out.write_str("ft8"),
                29 => out.write_str("ft9"),
                30 => out.write_str("ft10"),
                31 => out.write_str("ft11"),
                _ => out.write_str("<invalid>"),
            }
        } else if f {
            out.write_str("f");
            out.write_fmt(format_args!("{}", reg));
        } else {
            if self.abi_names {
                match reg {
                    0 => out.write_str("zero"),
                    1 => out.write_str("ra"),
                    2 => out.write_str("sp"),
                    3 => out.write_str("gp"),
                    4 => out.write_str("tp"),
                    5 => out.write_str("t0"),
                    6 => out.write_str("t1"),
                    7 => out.write_str("t2"),
                    8 => out.write_str("s0"),
                    9 => out.write_str("s1"),
                    10 => out.write_str("a0"),
                    11 => out.write_str("a1"),
                    12 => out.write_str("a2"),
                    13 => out.write_str("a3"),
                    14 => out.write_str("a4"),
                    15 => out.write_str("a5"),
                    16 => out.write_str("a6"),
                    17 => out.write_str("a7"),
                    18 => out.write_str("s2"),
                    19 => out.write_str("s3"),
                    20 => out.write_str("s4"),
                    21 => out.write_str("s5"),
                    22 => out.write_str("s6"),
                    23 => out.write_str("s7"),
                    24 => out.write_str("s8"),
                    25 => out.write_str("s9"),
                    26 => out.write_str("s10"),
                    27 => out.write_str("s11"),
                    28 => out.write_str("t3"),
                    29 => out.write_str("t4"),
                    30 => out.write_str("t5"),
                    31 => out.write_str("t6"),
                    _ => out.write_str("<invalid>"),
                }
            } else {
                out.write_str("x");
                out.write_fmt(format_args!("{}", reg));
            }
        }
    }

    fn write_rlist<W: FormatterOutput>(&self, out: &mut W, rlist: u32) {
        for i in [27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 9, 8, 1] {
            if rlist & (1 << i) != 0 {
                self.write_reg(out, false, false, i);
                out.write_str(" ");
            }
        }
    }

    pub fn format<W: FormatterOutput>(&self, out: &mut W, inst: &Instruction) {
        let encoding = inst.code.encoding();
        let mnem = OPCODE_STR[inst.code as usize];

        out.write_str(mnem);

        match encoding {
            Encoding::Bimm12HiRs1Bimm12lo => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rs1());
                let addr =
                    inst.address as isize + inst.value.bimm12lohi() as isize;
                out.write_fmt(format_args!("{:x}", addr));
            }

            Encoding::Bimm12HiRs1Rs2Bimm12lo => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rs1());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs2());
                let addr =
                    inst.address as isize + inst.value.bimm12lohi() as isize;
                out.write_fmt(format_args!("{:x}", addr));
            }

            Encoding::Bimm12HiRs2Bimm12lo => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rs2());
                let addr =
                    inst.address as isize + inst.value.bimm12lohi() as isize;
                out.write_fmt(format_args!("{:x}", addr));
            }

            Encoding::Bimm12HiRs2Rs1Bimm12lo => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rs2());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
                let addr =
                    inst.address as isize + inst.value.bimm12lohi() as isize;
                out.write_fmt(format_args!("{:x}", addr));
            }

            Encoding::CImm12 => {
                out.write_str(" ");
                let addr =
                    inst.address as isize + inst.value.c_imm12() as isize;
                out.write_fmt(format_args!("{:x}", addr));
            }

            Encoding::CIndex => {
                out.write_str(" ");
                out.write_fmt(format_args!("{}", inst.value.c_index()));
            }

            Encoding::CMopT => {
                out.write_str(" ");
                out.write_fmt(format_args!("{}", inst.value.c_mop_t()));
            }

            Encoding::CNzimm10hiCNzimm10lo => {
                out.write_str(" ");
                out.write_fmt(format_args!("{}", inst.value.c_nzimm10lohi()));
            }

            Encoding::CNzimm6hiCNzimm6lo => {
                out.write_str(" ");
                out.write_fmt(format_args!("{}", inst.value.c_nzimm6lohi()))
            }

            Encoding::CRlistCSpimm => {
                out.write_str(" ");
                let rlist = inst.value.c_rlist();
                self.write_rlist(out, rlist);
                out.write_fmt(format_args!(" {}", inst.value.c_spimm()));
            }

            Encoding::CRs1N0 => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.c_rs1_n0());
            }

            Encoding::CRs2CUimm8spS => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.c_rs2());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_uimm8sp_s()));
            }

            Encoding::CRs2CUimm9spS => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.c_rs2());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_uimm9sp_s()));
            }

            Encoding::CSreg1CSreg2 => {
                // TODO: Broken, implement sreg decoding/encoding
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.c_sreg1());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.c_sreg2());
            }

            Encoding::CsrZimm => {
                let zimm = inst.value.zimm();
                out.write_str(" ");
                out.write_fmt(format_args!("csr{}, {}", inst.value.csr(), zimm));
            }

            Encoding::Empty => {}
            Encoding::FmPredSuccRs1Rd => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.pred()));
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.succ()));
            }

            // <op> rs2, offset(rs1)
            Encoding::Imm12HiRs1Rs2Imm12lo => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rs2());
                self.write_reg(out, false, false, inst.value.rs2());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.imm12lohi()));
            }

            Encoding::Imm12Rs1Rd => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.imm12()));
            }

            Encoding::Jimm20 => {
                out.write_str(" ");
                let addr = inst.address as i64 + inst.value.jimm20() as i64;
                out.write_fmt(format_args!("{:x}", addr));
            }
            Encoding::RdJimm20 => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                let addr = inst.address as i64 + inst.value.jimm20() as i64;
                out.write_fmt(format_args!("{:x}", addr));
            }
            Encoding::MopRT30MopRT2726MopRT2120RdRs1 => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
            }

            Encoding::MopRrT30MopRrT2726RdRs1Rs2 => {
                todo!()
            }

            Encoding::NfVmRs1Vd => {
                let vd = inst.value.vd();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();

                out.write_str(" ");
                self.write_reg(out, false, true, vd);
                out.write_str(", (");
                self.write_reg(out, false, false, rs1);
                out.write_str(")");
                if vm == 0 {
                    out.write_str(", ");
                    out.write_fmt(format_args!("v{}.t", vm));
                }
            }

            Encoding::NfVmRs1Vs3 => {
                let vs3 = inst.value.vs3();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();

                out.write_str(" ");
                self.write_reg(out, false, true, vs3);
                out.write_str(", (");
                self.write_reg(out, false, false, rs1);
                out.write_str(")");
                if vm == 0 {
                    out.write_str(", ");
                    out.write_fmt(format_args!("v{}.t", vm));
                }
            }

            // <op> vd, (rs1), rs2, vm
            Encoding::NfVmRs2Rs1Vd => {
                let vd = inst.value.vd();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();
                let rs2 = inst.value.rs2();

                out.write_str(" ");
                self.write_reg(out, false, true, vd);
                out.write_str(", (");
                self.write_reg(out, false, false, rs1);
                out.write_str("), ");
                self.write_reg(out, false, false, rs2);
                if vm == 0 {
                    out.write_str(", ");
                    out.write_fmt(format_args!("v{}.t", vm));
                }
            }

            // <op> v3, (rs1), rs2, vm
            Encoding::NfVmRs2Rs1Vs3 => {
                let vs3 = inst.value.vs3();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();
                let rs2 = inst.value.rs2();

                out.write_str(" ");
                self.write_reg(out, false, true, vs3);
                out.write_str(", (");
                self.write_reg(out, false, false, rs1);
                out.write_str("), ");
                self.write_reg(out, false, false, rs2);
                if vm == 0 {
                    out.write_str(", ");
                    out.write_fmt(format_args!("v{}.t", vm));
                }
            }

            // vd, (rs1), vs2
            Encoding::NfVmVs2Rs1Vd => {
                let vd = inst.value.vd();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();
                let vs2 = inst.value.vs2();

                out.write_str(" ");
                self.write_reg(out, false, true, vd);
                out.write_str(", (");
                self.write_reg(out, false, false, rs1);
                out.write_str("), ");
                self.write_reg(out, false, true, vs2);
                if vm == 0 {
                    out.write_str(", ");
                    out.write_fmt(format_args!("v{}.t", vm));
                }
            }

            Encoding::NfVmVs2Rs1Vs3 => {
                let vs3 = inst.value.vs3();
                let vm = inst.value.vm();
                let rs1 = inst.value.rs1();
                let vs2 = inst.value.vs2();

                out.write_str(" ");
                self.write_reg(out, false, true, vs3);
                out.write_str(", (");
                self.write_reg(out, false, false, rs1);
                out.write_str("), ");
                self.write_reg(out, false, true, vs2);
                if vm == 0 {
                    out.write_str(", ");
                    out.write_fmt(format_args!("v{}.t", vm));
                }
            }

            Encoding::Rd => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
            }

            Encoding::RdCUimm9sphiCUimm9splo => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_uimm9splohi()));
            }

            Encoding::RdCsrZimm => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                out.write_fmt(format_args!(
                    "csr{}, {}",
                    inst.value.csr(),
                    inst.value.zimm()
                ));
            }

            Encoding::RdImm20 => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.imm20()));
            }

            Encoding::RdN0CImm6loCImm6hi => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_imm6lohi()));
            }

            Encoding::RdN0CRs2N0 => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs2());
            }

            Encoding::RdN0CUimm8sphiCUimm8splo => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_uimm8splohi()));
            }

            Encoding::RdN0CUimm9sphiCUimm9splo => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_uimm9splohi()));
            }

            Encoding::RdN2CNzimm18hiCNzimm18lo => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_nzimm18lohi()));
            }

            Encoding::RdPCNzuimm10 => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_nzuimm10()));
            }

            Encoding::RdPRs1PCUimm1 => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_uimm1()));
            }

            Encoding::RdPRs1PCUimm2 => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_uimm2()));
            }

            Encoding::RdPRs1PCUimm7loCUimm7hi => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
                out.write_fmt(format_args!(", {}", inst.value.c_uimm7lohi()));
            }

            Encoding::RdPRs1PCUimm8loCUimm8hi => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd_p() + 8);
                out.write_str(", ");

                self.write_reg(
                    out,
                    matches!(inst.code, Opcode::CFLD),
                    false,
                    inst.value.rs1_p() + 8,
                );
                out.write_fmt(format_args!(", {}", inst.value.c_uimm8lohi()));
            }

            Encoding::RdRs1 => match inst.code {
                Opcode::FLID | Opcode::FLIQ | Opcode::FLIH | Opcode::FLIS => {
                    self.format_fli(out, inst);
                }

                Opcode::FMVDX | Opcode::FMVHX | Opcode::FMVPQX | Opcode::FMVSX => {
                    out.write_str(" ");
                    self.write_reg(out, true, false, inst.value.rd());
                    out.write_str(", ");
                    self.write_reg(out, false, false, inst.value.rs1());
                }

                Opcode::FMVXD | Opcode::FMVXS | Opcode::FMVXH | Opcode::FMVXW => {
                    out.write_str(" ");
                    self.write_reg(out, false, false, inst.value.rd());
                    out.write_str(", ");
                    self.write_reg(out, true, false, inst.value.rs1());
                }

                Opcode::FCLASSD | Opcode::FCLASSH | Opcode::FCLASSQ | Opcode::FCLASSS => {
                    out.write_str(" ");
                    self.write_reg(out, false, false, inst.value.rd());
                    out.write_str(", ");
                    self.write_reg(out, true, false, inst.value.rs1());
                }

                _ => {
                    out.write_str(" ");
                    self.write_reg(out, false, false, inst.value.rd());
                    out.write_str(", ");
                    self.write_reg(out, false, false, inst.value.rs1());
                }
            },

            Encoding::RdRs1AqRl => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
            }
            // rd, csr, rs1
            Encoding::RdRs1Csr => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                out.write_fmt(format_args!("csr{}, ", inst.value.csr()));
                self.write_reg(out, false, false, inst.value.rs1());
            }

            // rd, offset(rs1)
            Encoding::RdRs1Imm12 => {
                let is_float = matches!(
                    inst.code,
                    Opcode::FLD | Opcode::FLH | Opcode::FLQ | Opcode::FLW
                );

                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, is_float, false, inst.value.rs1());
                out.write_fmt(format_args!(", {}", inst.value.imm12()));
            }

            // rd, rs1, imm6lohi
            Encoding::RdRs1N0CImm6loCImm6hi => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_imm6lohi()));
            }

            // rd, rs1, nzimm6lohi
            Encoding::RdRs1N0CNzimm6loCNzimm6hi => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_nzimm6lohi()));
            }

            // rd, rs1, nzuimm6lohi
            Encoding::RdRs1N0CNzuimm6hiCNzuimm6lo => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_nzuimm6lohi()));
            }

            Encoding::RdRs1N0CRs2N0 => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.c_rs2());
            }

            Encoding::RdRs1P => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1_p());
            }

            Encoding::RdRs1PCImm6hiCImm6lo => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1());
                out.write_str(", ");
                out.write_fmt(format_args!("{}", inst.value.c_imm6lohi()));
            }

            Encoding::RdRs1PRs2P => {
                out.write_str(" ");
                self.write_reg(out, false, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs1_p());
                out.write_str(", ");
                self.write_reg(out, false, false, inst.value.rs2_p());
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

                out.write_str(" ");
                self.write_reg(out, rd_is_f, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, rs1_is_f, false, inst.value.rs1());
            }
            Encoding::RdRs1Rs2 => {
                out.write_str(" ");
                let is_f = mnem.starts_with("f");

                self.write_reg(out, is_f, false, inst.value.rd());
                out.write_str(", ");
                self.write_reg(out, is_f, false, inst.value.rs1());
                out.write_str(", ");
                self.write_reg(out, is_f, false, inst.value.rs2());
            }
            _ => todo!("{:?}", encoding),
        }
    }

    fn format_fli<W: FormatterOutput>(&self, out: &mut W, inst: &Instruction) {
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

        out.write_str(" ");
        self.write_reg(out, true, false, inst.value.rd());
        out.write_fmt(format_args!(", {}", c));
    }
}


pub fn pretty_disassembler<W: FormatterOutput>(out: &mut W, bitness: usize, data: &[u8], address: u64) {
    let mut decoder = Decoder::new(bitness, data, address);
    let fmt = Formatter::new();

    let mut inst = Instruction::default();


    while decoder.can_decode() {
        decoder.decode_out(&mut inst);

        out.write_fmt(format_args!("{:016x}: {:08x}  ", inst.address, inst.value.value));

        fmt.format(out, &inst);
    }
}