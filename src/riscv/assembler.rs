use super::opcodes::Opcode;
use crate::core::buffer::{CodeBuffer, LabelUse, Reloc, RelocTarget};
use crate::core::operand::*;
use crate::core::operand::{Imm, Sym};
use crate::riscv::opcodes::Encoding;
use crate::AsmError;
use crate::{core::emitter::Emitter, riscv::opcodes::Inst};
pub struct Assembler<'a> {
    pub buffer: &'a mut CodeBuffer,
    last_error: Option<AsmError>,
}

impl<'a> Assembler<'a> {
    pub fn new(buffer: &'a mut CodeBuffer) -> Self {
        Assembler {
            buffer,
            last_error: None,
        }
    }
}
macro_rules! enc_ops1 {
    ($op0:ident) => {
        OperandType::$op0 as u32
    };
}

macro_rules! enc_ops2 {
    ($op0:ident, $op1:ident) => {
        (OperandType::$op0 as u32) | ((OperandType::$op1 as u32) << 3)
    };
}

macro_rules! enc_ops3 {
    ($op0:ident, $op1:ident, $op2:ident) => {
        (OperandType::$op0 as u32)
            | ((OperandType::$op1 as u32) << 3)
            | ((OperandType::$op2 as u32) << 6)
    };
}

macro_rules! enc_ops4 {
    ($op0:ident, $op1:ident, $op2:ident, $op3:ident) => {
        (OperandType::$op0 as u32)
            | ((OperandType::$op1 as u32) << 3)
            | ((OperandType::$op2 as u32) << 6)
            | ((OperandType::$op3 as u32) << 9)
    };
}

impl<'a> Emitter for Assembler<'a> {
    fn emit(
        &mut self,
        opcode: i64,
        op0: &crate::core::operand::Operand,
        op1: &crate::core::operand::Operand,
        op2: &crate::core::operand::Operand,
        op3: &crate::core::operand::Operand,
    ) {
        self.emit_n(opcode, &[op0, op1, op2, op3])
    }

    #[allow(unused_assignments)]
    fn emit_n(&mut self, opcode: i64, ops: &[&crate::core::operand::Operand]) {
        assert!(opcode < Opcode::Invalid as i64 && opcode >= 0);
        let opcode: Opcode = unsafe { core::mem::transmute(opcode as u32) };
        let encoding = opcode.encoding();

        let mut inst = Inst::new(opcode).encode();
        let mut label_use = None;
        let mut reloc = None;

        let isign3 = match ops {
            [] => 0,
            [op0] => op0.op_type() as u32,
            [op0, op1] => op0.op_type() as u32 + ((op1.op_type() as u32) << 3),
            [op0, op1, op2, ..] => {
                op0.op_type() as u32 + ((op1.op_type() as u32) << 3) + ((op2.op_type() as u32) << 6)
            }
        };

        let isign4 = match ops {
            [] => 0,
            [op0] => op0.op_type() as u32,
            [op0, op1] => op0.op_type() as u32 + ((op1.op_type() as u32) << 3),
            [op0, op1, op2] => {
                op0.op_type() as u32 + ((op1.op_type() as u32) << 3) + ((op2.op_type() as u32) << 6)
            }
            [op0, op1, op2, op3, ..] => {
                op0.op_type() as u32
                    + ((op1.op_type() as u32) << 3)
                    + ((op2.op_type() as u32) << 6)
                    + ((op3.op_type() as u32) << 9)
            }
        };
        let mut short = false;
        match encoding {
            Encoding::Bimm12HiRs1Bimm12lo => {
                let rs1 = ops[0].id();
                let imm = if ops[1].is_imm() {
                    ops[1].as_::<Imm>().value() as i32
                } else if ops[1].is_label() {
                    label_use = Some((ops[1], LabelUse::RVB12));
                    0
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                inst = inst.set_rs1(rs1).set_bimm12lohi(imm);
            }

            Encoding::Bimm12HiRs1Rs2Bimm12lo => {
                let rs1 = ops[0].id();
                let rs2 = ops[1].id();

                let imm = if ops[2].is_imm() {
                    ops[2].as_::<Imm>().value() as i32
                } else if ops[2].is_label() {
                    label_use = Some((ops[2], LabelUse::RVB12));
                    0
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                inst = inst.set_rs1(rs1).set_rs2(rs2).set_bimm12lohi(imm);
            }

            Encoding::Bimm12HiRs2Rs1Bimm12lo => {
                let rs1 = ops[0].id();
                let rs2 = ops[1].id();
                let imm = if ops[2].is_imm() {
                    ops[2].as_::<Imm>().value() as i32
                } else if ops[2].is_label() {
                    label_use = Some((ops[2], LabelUse::RVB12));
                    0
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                inst = inst.set_rs2(rs2).set_rs1(rs1).set_bimm12lohi(imm);
            }

            Encoding::Bimm12HiRs2Bimm12lo => {
                let rs2 = ops[0].id();
                let imm = if ops[1].is_imm() {
                    ops[1].as_::<Imm>().value() as i32
                } else if ops[1].is_label() {
                    label_use = Some((ops[1], LabelUse::RVB12));
                    0
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                inst = inst.set_rs2(rs2).set_bimm12lohi(imm);
            }

            Encoding::CImm12 => {
                short = true;
                let imm = if ops[0].is_imm() {
                    ops[0].as_::<Imm>().value() as i32
                } else if ops[0].is_label() {
                    label_use = Some((ops[0], LabelUse::RVCJump));
                    0
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                inst = inst.set_c_imm12(imm)
            }

            Encoding::CIndex => {
                short = true;
                let imm = if ops[0].is_imm() {
                    ops[0].as_::<Imm>().value() as i32
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };
                inst = inst.set_c_index(imm as _);
            }

            Encoding::CMopT => {
                short = true;
                let imm = if ops[0].is_imm() {
                    ops[0].as_::<Imm>().value() as i32
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                inst = inst.set_c_mop_t(imm as _);
            }

            Encoding::CNzimm10hiCNzimm10lo => {
                short = true;
                let imm = if ops[0].is_imm() {
                    ops[0].as_::<Imm>().value() as i32
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                if imm == 0 || imm > 1024 || imm < -1024 {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                inst = inst.set_c_nzimm10lohi(imm);
            }

            Encoding::CNzimm6hiCNzimm6lo => {
                short = true;
                let imm = if ops[0].is_imm() {
                    ops[0].as_::<Imm>().value() as i32
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                if imm == 0 || imm > 64 {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }

                inst = inst.set_c_nzimm6lohi(imm)
            }

            Encoding::CRlistCSpimm => {
                short = true;
                todo!()
            }

            Encoding::CRs1N0 => {
                short = true;
                let rs1 = ops[0].id();
                inst = inst.set_rs1_n0(rs1);
            }

            Encoding::CRs2CUimm8spS => {
                short = true;
                let rs2 = ops[0].id();
                let imm = if ops[1].is_imm() {
                    ops[1].as_::<Imm>().value() as i32
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };
                if imm < 0 || imm > 256 {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
                inst = inst.set_c_uimm8lohi(imm as _).set_c_rs2(rs2);
            }

            Encoding::CRs2CUimm9spS => {
                short = true;
                let rs2 = ops[0].id();
                let imm = if ops[1].is_imm() {
                    ops[1].as_::<Imm>().value() as i32
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };
                if imm < 0 || imm > 511 {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
                inst = inst.set_c_rs2(rs2).set_c_uimm9sp_s(imm as _);
            }

            Encoding::CSreg1CSreg2 => {
                short = true;
                todo!()
            }

            Encoding::CsrZimm => {
                let csr_imm = if ops[0].is_imm() {
                    ops[0].as_::<Imm>().value() as i32
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                let zimm = if ops[1].is_imm() {
                    ops[1].as_::<Imm>().value()
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                inst = inst.set_csr(csr_imm as _).set_zimm(zimm as _);
            }

            Encoding::Empty => {}
            Encoding::FmPredSuccRs1Rd => {
                let fm = if ops[0].is_imm() {
                    ops[0].as_::<Imm>().value() as u8
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                let pred = if ops[1].is_imm() {
                    ops[1].as_::<Imm>().value() as u8
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                let succ = if ops[2].is_imm() {
                    ops[2].as_::<Imm>().value() as u8
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };

                let rs1 = ops[3].id();
                let rd = ops[4].id();

                inst = inst
                    .set_fm(fm as _)
                    .set_pred(pred as _)
                    .set_succ(succ as _)
                    .set_rs1(rs1)
                    .set_rd(rd);
            }

            Encoding::Imm12HiRs1Rs2Imm12lo => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rs1 = ops[0].id();
                    let rs2 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;

                    inst = inst.set_rs1(rs1).set_rs2(rs2).set_imm12lohi(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };
            }

            Encoding::Imm12Rs1Rd => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rs1 = ops[0].id();
                    let rd = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;

                    inst = inst.set_rs1(rs1).set_rd(rd).set_imm12(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Jimm20 => {
                if isign3 == enc_ops1!(Imm) {
                    let imm = ops[0].as_::<Imm>().value() as i32;
                    inst = inst.set_jimm20(imm);
                } else if isign3 == enc_ops1!(Label) {
                    label_use = Some((ops[0], LabelUse::RVJal20));
                    inst = inst.set_jimm20(0);
                } else if isign3 == enc_ops1!(Sym) {
                    let sym = ops[0].as_::<Sym>();

                    let _distance = self.buffer.symbol_distance(sym);
                    // TODO: Should this work only with NEAR symbols?
                    reloc = Some((sym, Reloc::RiscvGotHi20))
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::MopRT30MopRT2726MopRT2120RdRs1 => {
                todo!()
            }
            Encoding::MopRrT30MopRrT2726RdRs1Rs2 => {
                todo!()
            }

            Encoding::NfVmRs1Vd => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let vd = ops[0].id();
                    let rs1 = ops[1].id();
                    let vm = ops[2].id();

                    inst = inst.set_vd(vd).set_rs1(rs1).set_vm(vm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::NfVmRs1Vs3 => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let vs3 = ops[0].id();
                    let rs1 = ops[1].id();
                    let vm = ops[2].id();

                    inst = inst.set_vs3(vs3).set_rs1(rs1).set_vm(vm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::NfVmRs2Rs1Vd => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Reg) {
                    let vd = ops[0].id();
                    let rs1 = ops[1].id();
                    let vm = ops[3].id();
                    let rs2 = ops[2].id();
                    inst = inst.set_rs1(rs1).set_rs2(rs2).set_vd(vd).set_vm(vm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::NfVmRs2Rs1Vs3 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Reg) {
                    let vs3 = ops[0].id();
                    let rs1 = ops[1].id();
                    let vm = ops[3].id();
                    let rs2 = ops[2].id();
                    inst = inst.set_rs1(rs1).set_rs2(rs2).set_vs3(vs3).set_vm(vm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::NfVmVs2Rs1Vd => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Reg) {
                    let vd = ops[0].id();
                    let rs1 = ops[1].id();
                    let vs2 = ops[2].id();
                    let vm = ops[3].id();

                    inst = inst.set_rs1(rs1).set_vd(vd).set_vs2(vs2).set_vm(vm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::NfVmVs2Rs1Vs3 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Reg) {
                    let vs3 = ops[0].id();
                    let rs1 = ops[1].id();
                    let vs2 = ops[2].id();
                    let vm = ops[3].id();

                    inst = inst.set_vs3(vs3).set_rs1(rs1).set_vs2(vs2).set_vm(vm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rd => {
                if isign3 == enc_ops1!(Reg) {
                    let rd = ops[0].id();
                    inst = inst.set_rd(rd);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdCUimm8sphiCUimm8splo => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as u32;

                    inst = inst.set_rd(rd).set_c_uimm8splohi(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdCUimm9sphiCUimm9splo => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as u32;

                    inst = inst.set_rd(rd).set_c_uimm9splohi(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdCsr => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let csr = ops[1].as_::<Imm>().value() as u32;
                    inst = inst.set_rd(rd).set_csr(csr);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdCsrZimm => {
                if isign3 == enc_ops3!(Reg, Imm, Imm) {
                    let rd = ops[0].id();
                    let csr = ops[1].as_::<Imm>().value() as u32;
                    let zimm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_csr(csr).set_zimm(zimm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }
            Encoding::RdImm20 => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_imm20(imm);
                } else if isign3 == enc_ops2!(Reg, Label) {
                    let rd = ops[0].id();
                    label_use = Some((ops[0], LabelUse::RVPCRelHi20));
                    inst = inst.set_rd(rd).set_imm20(0);
                } else if isign3 == enc_ops2!(Reg, Sym) {
                    /*let rd = ops[0].id();
                    let sym = ops[1].as_::<Sym>();

                    reloc = Some((sym, Reloc::RiscvGotHi20));
                    inst = inst.set_rd(rd).set_imm20(0);*/
                    todo!()
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };
            }

            Encoding::RdJimm20 => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_jimm20(imm);
                } else if isign3 == enc_ops2!(Reg, Label) {
                    let rd = ops[0].id();
                    label_use = Some((ops[0], LabelUse::RVJal20));
                    inst = inst.set_rd(rd).set_jimm20(0);
                } else if isign3 == enc_ops2!(Reg, Sym) {
                    let rd = ops[0].id();
                    let sym = ops[1].as_::<Sym>();

                    reloc = Some((sym, Reloc::RiscvGotHi20));
                    inst = inst.set_rd(rd).set_jimm20(0);
                    todo!()
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };
            }

            Encoding::RdN0CImm6loCImm6hi => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rd_n0(rd).set_c_imm6lohi(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdN0CRs2N0 => {
                short = true;
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    inst = inst.set_rd_n0(rd).set_c_rs2(rs1);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdN0CUimm8sphiCUimm8splo => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rd_n0(rd).set_c_uimm8splohi(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdN0CUimm9sphiCUimm9splo => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rd_n0(rd).set_c_uimm9splohi(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdN2CNzimm18hiCNzimm18lo => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    if imm == 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    } else {
                        inst = inst.set_rd_n2(rd).set_c_nzimm18lohi(imm);
                    }
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdPCNzuimm10 => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rd_p(rd).set_c_nzimm10lohi(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdPRs1PCUimm1 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;

                    inst = inst.set_rd_p(rd).set_rs1_p(rs1).set_c_uimm1(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdPRs1PCUimm2 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;

                    inst = inst.set_rd_p(rd).set_rs1_p(rs1).set_c_uimm2(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdPRs1PCUimm7loCUimm7hi => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_c_uimm7lohi(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdPRs1PCUimm8loCUimm8hi => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_c_uimm8lohi(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    inst = inst.set_rd(rd).set_rs1(rs1);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1AqRl => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_aqrl(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1Csr => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_csr(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1Imm12 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_imm12(imm);
                } else if isign3 == enc_ops3!(Reg, Reg, Label) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    label_use = Some((ops[2], LabelUse::RVPCRelLo12I));
                    inst = inst.set_rd(rd).set_rs1(rs1).set_imm12(0);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1N0 => {
                short = true;
                if isign3 == enc_ops2!(Reg, Reg) {
                    let _rd = ops[0].id();
                    let _rs1 = ops[1].id();
                    todo!()
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1Rm => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let rm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_rm(rm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }
            Encoding::RdRs1Rnum => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let rm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_rnum(rm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1Rs2 => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let rs2 = ops[2].id();
                    inst = inst.set_rd(rd).set_rs1(rs1).set_rs2(rs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1Rs2AqRl => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let rs2 = ops[2].id();
                    let imm = ops[3].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_rs2(rs2).set_aqrl(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1Rs2Bs => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let rs2 = ops[2].id();
                    let imm = ops[3].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_rs2(rs2).set_bs(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1Rs2EqRs1 => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let rs2 = ops[2].id();
                    inst = inst.set_rd(rd).set_rs1(rs1).set_rs2_eq_rs1(rs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1Rs2Rm => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let rs2 = ops[2].id();
                    let rm = ops[3].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_rs2(rs2).set_rm(rm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1Rs2Rs3Rm => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Reg) && ops[4].op_type() == OperandType::Imm {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let rs2 = ops[2].id();
                    let rs3 = ops[3].id();
                    let rm = ops[4].as_::<Imm>().value() as i32;

                    inst = inst
                        .set_rd(rd)
                        .set_rs1(rs1)
                        .set_rs2(rs2)
                        .set_rs3(rs3)
                        .set_rm(rm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1Shamtw => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let shamt = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_shamtw(shamt as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }
            Encoding::RdRs2 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    inst = inst.set_rd(rd).set_rs1(rs1);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdZimm => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_zimm(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1 => {
                if isign3 == enc_ops1!(Reg) {
                    let rs1 = ops[0].id();
                    inst = inst.set_rs1(rs1);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1Csr => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rs1 = ops[0].id();
                    let csr = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rs1(rs1).set_csr(csr as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1Imm12hi => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rs1 = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rs1(rs1).set_imm12hi_raw(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1N0 => {
                short = true;
                if isign3 == enc_ops1!(Reg) {
                    let rs1 = ops[0].id();
                    inst = inst.set_rs1_n0(rs1);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1PCBimm9loCBimm9hi => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rs1 = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value();

                    inst = inst.set_rs1(rs1).set_c_bimm9lohi(imm as _);
                } else if isign3 == enc_ops2!(Reg, Label) {
                    let rs1 = ops[0].id();
                    label_use = Some((ops[1], LabelUse::RVCB9));
                    inst = inst.set_rs1(rs1).set_c_bimm9lohi(0);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1PRs2PCUimm7loCUimm7hi => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rs1 = ops[0].id();
                    let rs2 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rs1_p(rs1).set_rs2_p(rs2).set_c_uimm7lohi(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1PRs2PCUimm8loCUimm8hi => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rs1 = ops[0].id();
                    let rs2 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rs1_p(rs1).set_rs2_p(rs2).set_c_uimm8lohi(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1PRs2PCUimm8hiCUimm8lo => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rs1 = ops[0].id();
                    let rs2 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rs1_p(rs1).set_rs2_p(rs2).set_c_uimm8lohi(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1Rd => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();

                    inst = inst.set_rd(rd).set_rs1(rs1);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1Rs2 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rs1 = ops[0].id();
                    let rs2 = ops[1].id();
                    inst = inst.set_rs1(rs1).set_rs2(rs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1Vd => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rs1 = ops[1].id();
                    let vd = ops[0].id();
                    inst = inst.set_vd(vd).set_rs1(rs1);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }
            Encoding::Rs1Vs3 => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rs1 = ops[1].id();
                    let vs3 = ops[0].id();
                    inst = inst.set_rs1(rs1).set_vs3(vs3);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs2PRs1PCUimm1 => {
                short = true;
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rs1 = ops[0].id();
                    let rs2 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rs1_p(rs1).set_rs2_p(rs2).set_c_uimm1(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs2PRs1PCUimm2 => {
                short = true;
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rs1 = ops[0].id();
                    let rs2 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rs1_p(rs1).set_rs2_p(rs2).set_c_uimm2(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }
            Encoding::Rs2Rs1Rd => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let rs2 = ops[2].id();

                    inst = inst.set_rd(rd).set_rs1(rs1).set_rs2(rs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Simm5Vd => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let vd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i8;
                    inst = inst.set_vd(vd).set_simm5(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVs2Rd => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let rd = ops[0].id();
                    let vs2 = ops[1].id();
                    let vm = ops[2].id();
                    inst = inst.set_rd(rd).set_vs2(vs2).set_vm(vm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVd => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let vd = ops[0].id();
                    let vm = ops[1].id();
                    inst = inst.set_vd(vd).set_vm(vm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVs2Rs1Vd => {
                if isign3 == enc_ops4!(Reg, Reg, Reg, Reg) {
                    let rs1 = ops[2].id();
                    let vs2 = ops[1].id();
                    let vm = ops[3].id();
                    let vd = ops[0].id();
                    inst = inst.set_vd(vd).set_vm(vm).set_rs1(rs1).set_vs2(vs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVs2Simm5Vd => {
                if isign3 == enc_ops4!(Reg, Reg, Imm, Reg) {
                    let simm5 = ops[2].as_::<Imm>().value() as i32;
                    let vs2 = ops[1].id();
                    let vm = ops[3].id();
                    let vd = ops[0].id();
                    inst = inst.set_vd(vd).set_vm(vm).set_simm5(simm5).set_vs2(vs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVs2Vd => {
                if isign3 == enc_ops3!(Reg, Reg, Reg) {
                    let vd = ops[0].id();
                    let vs2 = ops[1].id();
                    let vm = ops[2].id();

                    inst = inst.set_vd(vd).set_vs2(vs2).set_vm(vm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVs2Vs1Vd => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Reg) {
                    let vd = ops[0].id();
                    let vs1 = ops[1].id();
                    let vs2 = ops[2].id();
                    let vm = ops[3].id();
                    inst = inst.set_vd(vd).set_vs1(vs1).set_vs2(vs2).set_vm(vm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVs2Zimm5Vd => {
                if isign4 == enc_ops4!(Reg, Reg, Imm, Reg) {
                    let vd = ops[0].id();
                    let vs2 = ops[1].id();
                    let vm = ops[3].id();
                    let zimm5 = ops[2].as_::<Imm>().value() as i8;
                    inst = inst
                        .set_vd(vd)
                        .set_vs2(vs2)
                        .set_vm(vm)
                        .set_zimm5(zimm5 as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Vs1Vd => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let vd = ops[0].id();
                    let vs1 = ops[1].id();
                    inst = inst.set_vd(vd).set_vs1(vs1);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Vs2Rd => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let vs2 = ops[1].id();
                    inst = inst.set_rd(rd).set_vs2(vs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Vs2Rs1Vd => {
                if isign4 == enc_ops3!(Reg, Reg, Reg) {
                    let rs1 = ops[1].id();
                    let vs2 = ops[2].id();
                    let vd = ops[0].id();
                    inst = inst.set_vd(vd).set_vs2(vs2).set_rs1(rs1);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Vs2Simm5Vd => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let vd = ops[0].id();
                    let vs2 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value();

                    inst = inst.set_vd(vd).set_vs2(vs2).set_simm5(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Vs2Vd => {
                if isign3 == enc_ops2!(Reg, Reg) {
                    let vd = ops[0].id();
                    let vs2 = ops[1].id();
                    inst = inst.set_vd(vd).set_vs2(vs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Vs2Vs1Vd => {
                if isign4 == enc_ops3!(Reg, Reg, Reg) {
                    let vd = ops[0].id();
                    let vs1 = ops[1].id();
                    let vs2 = ops[2].id();

                    inst = inst.set_vd(vd).set_vs1(vs1).set_vs2(vs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Vs2Zimm5Vd => {
                if isign3 == enc_ops3!(Reg, Imm, Reg) {
                    let vd = ops[0].id();
                    let vs2 = ops[1].id();
                    let zimm5 = ops[2].as_::<Imm>().value() as i8;
                    inst = inst.set_vd(vd).set_vs2(vs2).set_zimm5(zimm5 as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Zimm10ZimmRd => {
                if isign3 == enc_ops3!(Reg, Imm, Imm) {
                    let rd = ops[0].id();
                    let uimm = ops[1].as_::<Imm>().value() as i8;
                    let vtypei = ops[2].as_::<Imm>().value() as i8;
                    inst = inst.set_rd(rd).set_zimm10(vtypei as _).set_zimm(uimm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Zimm11Rs1Rd => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_zimm11(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Zimm6HiVmVs2Zimm6loVd => {
                if isign4 == enc_ops4!(Reg, Reg, Imm, Reg) {
                    let vd = ops[0].id();
                    let vs2 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value();
                    let vm = ops[3].id();

                    inst = inst
                        .set_vd(vd)
                        .set_vs2(vs2)
                        .set_zimm6lohi(imm as _)
                        .set_vm(vm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }
            _ => todo!(),
        }
        let offset = self.buffer.cur_offset();
        if let Some((label, kind)) = label_use {
            self.buffer
                .use_label_at_offset(offset, label.as_::<Label>(), kind);
        }

        if let Some((sym, kind)) = reloc {
            let target = RelocTarget::ExternalName(self.buffer.symbol_name(sym).clone());
            self.buffer.add_reloc_at_offset(offset, kind, target, 0);
        }

        if short {
            self.buffer.put2(inst.value as u16);
        } else {
            self.buffer.put4(inst.value);
        }
        
    }
}

impl super::emitter::EmitterExplicit for Assembler<'_> {}
