use super::opcodes::Opcode;
use crate::AsmError;
use crate::core::arch_traits::Arch;
use crate::core::buffer::{CodeBuffer, CodeOffset, ConstantData, LabelUse, Reloc, RelocTarget};
use crate::core::operand::*;
use crate::core::operand::{Imm, Sym};
use crate::core::patch::{DataEncoding, DataLabel, PatchableJump, PatchableRegion};
use crate::core::target::Environment;
use crate::riscv::instdb::{ANY, OPCODE_FEATURE_CONTEXT, OPCODE_FEATURE_MASKS, SIGNATURE_TABLE};
use crate::riscv::opcodes::Inst;
use crate::riscv::opcodes::{ALL_OPCODES, Encoding, OPCODE_XLEN, SHORT_OPCODE};
use crate::riscv::{Gp, RA, ZERO};
pub struct Assembler<'a> {
    pub(crate) buffer: &'a mut CodeBuffer,
    /// Scratch error set by the generated emitter during one checked attempt.
    last_error: Option<AsmError>,
}

fn validate_raw_operand(op: &Operand) -> bool {
    let Some(op_type) = op.signature.try_op_type() else {
        return false;
    };

    match op_type {
        OperandType::None => op.signature.bits() == 0,
        OperandType::Reg => {
            let Some(reg_type) = op.signature.try_reg_type() else {
                return false;
            };
            let Some(_) = op.signature.try_reg_group() else {
                return false;
            };
            let expected = crate::riscv::Reg::signature_of(reg_type);
            let mask = OperandSignature::OP_TYPE_MASK
                | OperandSignature::REG_TYPE_MASK
                | OperandSignature::REG_GROUP_MASK
                | OperandSignature::SIZE_MASK;
            expected.bits() != 0
                && op.signature.subset(mask) == expected.subset(mask)
                && op.id() <= 31
        }
        OperandType::Mem => {
            op.signature.try_mem_base_type().is_some()
                && op.signature.try_mem_index_type().is_some()
        }
        OperandType::Imm | OperandType::Label | OperandType::Sym => true,
        OperandType::RegList => false,
    }
}

impl<'a> Assembler<'a> {
    pub fn new(buffer: &'a mut CodeBuffer) -> Self {
        if !matches!(buffer.env().arch(), Arch::RISCV32 | Arch::RISCV64) {
            return Self::poisoned(buffer, AsmError::InvalidArch);
        }
        Self::unchecked(buffer)
    }

    pub fn try_new(buffer: &'a mut CodeBuffer) -> Result<Self, AsmError> {
        if !matches!(buffer.env().arch(), Arch::RISCV32 | Arch::RISCV64) {
            return Err(AsmError::InvalidArch);
        }
        Ok(Self::unchecked(buffer))
    }

    fn unchecked(buffer: &'a mut CodeBuffer) -> Self {
        Self {
            buffer,
            last_error: None,
        }
    }

    fn poisoned(buffer: &'a mut CodeBuffer, error: AsmError) -> Self {
        buffer.record_error(error);
        Self::unchecked(buffer)
    }

    /// Returns the environment (arch/mode) this assembler targets.
    pub fn environment(&self) -> &Environment {
        self.buffer.env()
    }

    /// Tests whether the assembler targets rv32 mode.
    pub fn is_32bit(&self) -> bool {
        self.buffer.env().is_32bit()
    }

    /// Tests whether the assembler targets rv64 mode.
    pub fn is_64bit(&self) -> bool {
        self.buffer.env().is_64bit()
    }

    pub fn get_label(&mut self) -> Label {
        self.buffer.get_label()
    }

    pub fn bind_label(&mut self, label: Label) {
        if let Err(error) = self.try_bind_label(label) {
            self.buffer.record_error(error);
        }
    }

    pub fn try_bind_label(&mut self, label: Label) -> Result<(), AsmError> {
        self.buffer.try_bind_label(label)
    }

    pub fn add_constant(&mut self, c: impl Into<ConstantData>) -> Label {
        let c = self.buffer.add_constant(c);
        self.buffer.get_label_for_constant(c)
    }

    pub fn label_offset(&self, label: Label) -> CodeOffset {
        self.buffer.label_offset(label)
    }

    pub fn data(&self) -> &[u8] {
        self.buffer.data()
    }

    pub fn error(&self) -> Option<&AsmError> {
        self.buffer.error()
    }

    /// Reserve a nop-filled region for later rewriting.
    pub fn reserve_patch_region(
        &mut self,
        size: CodeOffset,
        align: CodeOffset,
    ) -> Result<PatchableRegion, AsmError> {
        self.buffer.reserve_patch_region(size, align)
    }

    /// Emits a `jal` through `emit`, returning it as a patchable jump.
    fn patchable_jal(&mut self, emit: impl FnOnce(&mut Self)) -> PatchableJump {
        if self.buffer.error().is_some() {
            return PatchableJump::invalid(LabelUse::RVJal20);
        }
        let checkpoint = self.buffer.checkpoint();
        let offset = self.buffer.cur_offset();
        emit(self);
        if self.buffer.error().is_some() {
            self.buffer.rollback(checkpoint);
            return PatchableJump::invalid(LabelUse::RVJal20);
        }
        PatchableJump::new(offset, LabelUse::RVJal20)
    }

    /// `j` to `label` that can be retargeted.
    pub fn patchable_j(&mut self, label: Label) -> PatchableJump {
        self.patchable_jal(|asm| asm.j(label))
    }

    /// `jal ra` to `label` that can be retargeted.
    pub fn patchable_call(&mut self, label: Label) -> PatchableJump {
        self.patchable_jal(|asm| asm.jal(RA, label))
    }

    /// Materialize `imm` into `rd` with a fixed-size sequence and a patchable literal.
    ///
    /// Layout (RV64): `auipc; ld; jal; .dword`: the returned label covers the 8-byte literal.
    /// Layout (RV32): `auipc; lw; jal; .word`: the returned label covers the 4-byte literal.
    pub fn patchable_li(&mut self, rd: Gp, imm: impl Into<i64>) -> DataLabel {
        let value = imm.into();
        let size = if self.is_32bit() { 4 } else { 8 };
        if self.buffer.error().is_some() {
            return DataLabel::invalid(size, DataEncoding::Raw);
        }
        let checkpoint = self.buffer.checkpoint();

        if self.is_32bit() {
            self.auipc(rd, crate::core::operand::imm(0));
            self.lw(rd, rd, crate::core::operand::imm(12));
            self.jal(ZERO, crate::core::operand::imm(8));
        } else {
            self.auipc(rd, crate::core::operand::imm(0));
            self.ld(rd, rd, crate::core::operand::imm(12));
            self.jal(ZERO, crate::core::operand::imm(12));
        }
        let lit = self.buffer.cur_offset();
        if self.is_32bit() {
            self.buffer.write_u32(value as u32);
        } else {
            self.buffer.write_u64(value as u64);
        }
        if self.buffer.error().is_some() {
            self.buffer.rollback(checkpoint);
            return DataLabel::invalid(size, DataEncoding::Raw);
        }
        DataLabel::new(lit, size, DataEncoding::Raw)
    }

    pub fn la(&mut self, rd: Gp, target: impl OperandCast) {
        if self.buffer.error().is_some() {
            return;
        }
        let checkpoint = self.buffer.checkpoint();
        let target = target.as_operand();

        if target.is_label() {
            let off = self.buffer.cur_offset();
            self.buffer
                .use_label_at_offset(off, target.as_::<Label>(), LabelUse::RVPCRelHi20);
            self.auipc(rd, imm(0));
            let off = self.buffer.cur_offset();
            self.buffer
                .use_label_at_offset(off, target.as_::<Label>(), LabelUse::RVPCRelLo12I);
            self.addi(rd, rd, imm(0));
        } else if target.is_sym() {
            if self.buffer.env().pic() {
                // Load a PC-relative address into a register.
                // RISC-V does this slightly differently from other arches. We emit a relocation
                // with a label, instead of the symbol itself.
                //
                // See: https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#pc-relative-symbol-addresses
                //
                // Emit the following code:
                // label:
                //   auipc rd, 0              # R_RISCV_GOT_HI20 (symbol_name)
                //   ld    rd, rd, 0          # R_RISCV_PCREL_LO12_I (label)

                let sym = target.as_::<Sym>();

                // Create the label that is going to be published to the final binary object.
                let auipc_label = self.get_label();
                self.bind_label(auipc_label);
                self.buffer
                    .add_reloc(Reloc::RiscvGotHi20, RelocTarget::Sym(sym), 0);

                self.auipc(rd, imm(0));
                // The `ld`/`lw` here, points to the `auipc` label instead of directly to the symbol.
                self.buffer
                    .add_reloc(Reloc::RiscvPCRelLo12I, RelocTarget::Label(auipc_label), 0);
                if self.is_32bit() {
                    self.lw(rd, rd, imm(0));
                } else {
                    self.ld(rd, rd, imm(0));
                }
            } else {
                // In the non PIC sequence we relocate the absolute address into
                // a prealocatted space, load it into a register and jump over it.
                //
                // Emit the following code:
                //   ld rd, label_data        # (lw on rv32)
                //   j label_end
                // label_data:
                //   <word space>             # ABS8 (ABS4 on rv32)
                // label_end:
                let label_data = self.get_label();
                let label_end = self.get_label();

                if self.is_32bit() {
                    self.emit_n(
                        Opcode::LW as i64,
                        &[rd.as_operand(), rd.as_operand(), label_data.as_operand()],
                    );
                } else {
                    self.emit_n(
                        Opcode::LD as i64,
                        &[rd.as_operand(), rd.as_operand(), label_data.as_operand()],
                    );
                }
                self.j(label_end);
                self.bind_label(label_data);
                if self.is_32bit() {
                    self.buffer
                        .add_reloc(Reloc::Abs4, RelocTarget::Sym(target.as_::<Sym>()), 0);
                    self.buffer.put4(0);
                } else {
                    self.buffer
                        .add_reloc(Reloc::Abs8, RelocTarget::Sym(target.as_::<Sym>()), 0);
                    self.buffer.put8(0);
                }
                self.bind_label(label_end);
            }
        } else {
            self.buffer.record_error(AsmError::InvalidOperand);
        }
        if self.buffer.error().is_some() {
            self.buffer.rollback(checkpoint);
        }
    }

    pub fn call(&mut self, target: impl OperandCast) {
        if self.buffer.error().is_some() {
            return;
        }
        let checkpoint = self.buffer.checkpoint();
        let target = target.as_operand();

        if target.is_label() {
            let off = self.buffer.cur_offset();
            self.buffer
                .use_label_at_offset(off, target.as_::<Label>(), LabelUse::RVPCRelHi20);
            self.auipc(RA, imm(0));
            let off = self.buffer.cur_offset();
            self.buffer
                .use_label_at_offset(off, target.as_::<Label>(), LabelUse::RVPCRelLo12I);
            self.jalr(RA, RA, imm(0));
        } else if target.is_sym() {
            let sym = target.as_::<Sym>();

            let reloc = Reloc::RiscvCallPlt;

            self.buffer.add_reloc(reloc, RelocTarget::Sym(sym), 0);
            self.auipc(RA, imm(0));
            self.jalr(RA, RA, imm(0));
        } else if target.is_imm() {
            self.jalr(RA, RA, target.as_::<Imm>());
        } else if target.is_reg() {
            self.jalr(RA, target.as_::<Gp>(), imm(0));
        } else {
            self.buffer.record_error(AsmError::InvalidOperand);
        }
        if self.buffer.error().is_some() {
            self.buffer.rollback(checkpoint);
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

impl<'a> Assembler<'a> {
    pub fn emit_n(&mut self, opcode: i64, ops: &[&Operand]) {
        if let Err(error) = self.try_emit_n(opcode, ops) {
            self.buffer.record_error(error);
        }
    }

    pub fn try_emit_n(&mut self, opcode: i64, ops: &[&Operand]) -> Result<(), AsmError> {
        if let Some(error) = self.buffer.error().cloned() {
            return Err(error);
        }
        if ops.len() > 5 || ops.iter().any(|op| !validate_raw_operand(op)) {
            return Err(AsmError::InvalidOperand);
        }
        let checkpoint = self.buffer.checkpoint();
        self.last_error = None;
        self.emit_n_inner(opcode, ops);
        if let Some(error) = self.last_error.take() {
            self.buffer.rollback(checkpoint);
            return Err(error);
        }
        if let Some(error) = self.buffer.error().cloned() {
            self.buffer.rollback(checkpoint);
            return Err(error);
        }
        Ok(())
    }

    #[allow(unused_assignments)]
    fn emit_n_inner(&mut self, opcode: i64, ops: &[&crate::core::operand::Operand]) {
        let Ok(opcode) = usize::try_from(opcode) else {
            self.last_error = Some(AsmError::InvalidInstruction);
            return;
        };
        let Some(opcode) = ALL_OPCODES.get(opcode).copied() else {
            self.last_error = Some(AsmError::InvalidInstruction);
            return;
        };
        if !self
            .environment()
            .supports_any_riscv_feature(&OPCODE_FEATURE_MASKS[opcode as usize])
        {
            self.last_error = Some(AsmError::MissingCpuFeature {
                feature: OPCODE_FEATURE_CONTEXT[opcode as usize],
            });
            return;
        }
        let signature = &SIGNATURE_TABLE[opcode.inst_info().signature_index as usize];
        let expected_operands = signature
            .iter()
            .position(|operand_class| *operand_class == ANY)
            .unwrap_or(signature.len());
        if ops.len() != expected_operands {
            self.last_error = Some(AsmError::InvalidOperand);
            return;
        }

        // Reject instructions that have no encoding on the target XLEN (e.g.
        // `ld` on rv32, or the rv32-only `slli.rv32` variants on rv64).
        let xlen_bit = if self.is_32bit() { 1 } else { 2 };
        if OPCODE_XLEN[opcode as usize] & xlen_bit == 0 {
            self.last_error = Some(AsmError::InvalidInstruction);
            return;
        }

        let encoding = opcode.encoding();
        let is_prime_register = |id| (8..=15).contains(&id);

        let mut inst = Inst::new(opcode).encode();
        let mut label_use = None;

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
        let mut short = SHORT_OPCODE[opcode as usize];
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

                if imm == 0 || !(-1024..=1024).contains(&imm) {
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
                self.last_error = Some(AsmError::UnsupportedInstruction {
                    reason: "RISC-V compressed register-list instructions are not implemented",
                });
                return;
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
                if !(0..=256).contains(&imm) {
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
                if !(0..=511).contains(&imm) {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
                inst = inst.set_c_rs2(rs2).set_c_uimm9sp_s(imm as _);
            }

            Encoding::CSreg1CSreg2 => {
                self.last_error = Some(AsmError::UnsupportedInstruction {
                    reason: "RISC-V compressed saved-register moves are not implemented",
                });
                return;
            }

            Encoding::CsrZimm5 => {
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

                inst = inst.set_csr(csr_imm as _).set_zimm5(zimm as _);
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
                if opcode == Opcode::FENCEI {
                    // imm12, rs1, and rd are reserved and canonically zero.
                } else if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rs1 = ops[0].id();
                    let rd = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value() as i32;

                    inst = inst.set_rs1(rs1).set_rd(rd).set_imm12(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Imm20 => {
                if isign3 == enc_ops1!(Imm) {
                    let imm = ops[0].as_::<Imm>().value() as i32;
                    inst = inst.set_imm20(imm);
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
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::MopRT30MopRT2726MopRT2120RdRs1 => {
                self.last_error = Some(AsmError::UnsupportedInstruction {
                    reason: "RISC-V MOP.RN instructions are not implemented",
                });
                return;
            }
            Encoding::MopRrT30MopRrT2726RdRs1Rs2 => {
                self.last_error = Some(AsmError::UnsupportedInstruction {
                    reason: "RISC-V MOP.RR.N instructions are not implemented",
                });
                return;
            }

            Encoding::NfVmRs1Vd => {
                if isign4 == enc_ops4!(Reg, Reg, Imm, Imm) {
                    let vd = ops[0].id();
                    let rs1 = ops[1].id();
                    let vm = ops[2].as_::<Imm>().value();
                    let nf = ops[3].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) || !(0..=7).contains(&nf) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    inst = inst.set_vd(vd).set_rs1(rs1).set_vm(vm as _).set_nf(nf as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::NfVmRs1Vs3 => {
                if isign4 == enc_ops4!(Reg, Reg, Imm, Imm) {
                    let vs3 = ops[0].id();
                    let rs1 = ops[1].id();
                    let vm = ops[2].as_::<Imm>().value();
                    let nf = ops[3].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) || !(0..=7).contains(&nf) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    inst = inst
                        .set_vs3(vs3)
                        .set_rs1(rs1)
                        .set_vm(vm as _)
                        .set_nf(nf as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::NfVmRs2Rs1Vd => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm)
                    && ops.get(4).is_some_and(|op| op.is_imm())
                {
                    let vd = ops[0].id();
                    let rs1 = ops[1].id();
                    let vm = ops[3].as_::<Imm>().value();
                    let nf = ops[4].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) || !(0..=7).contains(&nf) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let rs2 = ops[2].id();
                    inst = inst
                        .set_rs1(rs1)
                        .set_rs2(rs2)
                        .set_vd(vd)
                        .set_vm(vm as _)
                        .set_nf(nf as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::NfVmRs2Rs1Vs3 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm)
                    && ops.get(4).is_some_and(|op| op.is_imm())
                {
                    let vs3 = ops[0].id();
                    let rs1 = ops[1].id();
                    let vm = ops[3].as_::<Imm>().value();
                    let nf = ops[4].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) || !(0..=7).contains(&nf) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let rs2 = ops[2].id();
                    inst = inst
                        .set_rs1(rs1)
                        .set_rs2(rs2)
                        .set_vs3(vs3)
                        .set_vm(vm as _)
                        .set_nf(nf as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::NfVmVs2Rs1Vd => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm)
                    && ops.get(4).is_some_and(|op| op.is_imm())
                {
                    let vd = ops[0].id();
                    let rs1 = ops[1].id();
                    let vs2 = ops[2].id();
                    let vm = ops[3].as_::<Imm>().value();
                    let nf = ops[4].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) || !(0..=7).contains(&nf) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    inst = inst
                        .set_rs1(rs1)
                        .set_vd(vd)
                        .set_vs2(vs2)
                        .set_vm(vm as _)
                        .set_nf(nf as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::NfVmVs2Rs1Vs3 => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm)
                    && ops.get(4).is_some_and(|op| op.is_imm())
                {
                    let vs3 = ops[0].id();
                    let rs1 = ops[1].id();
                    let vs2 = ops[2].id();
                    let vm = ops[3].as_::<Imm>().value();
                    let nf = ops[4].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) || !(0..=7).contains(&nf) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    inst = inst
                        .set_vs3(vs3)
                        .set_rs1(rs1)
                        .set_vs2(vs2)
                        .set_vm(vm as _)
                        .set_nf(nf as _);
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

            Encoding::RdCsrZimm5 => {
                if isign3 == enc_ops3!(Reg, Imm, Imm) {
                    let rd = ops[0].id();
                    let csr = ops[1].as_::<Imm>().value() as u32;
                    let zimm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_csr(csr).set_zimm5(zimm);
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
                    label_use = Some((ops[1], LabelUse::RVPCRelHi20));
                    inst = inst.set_rd(rd).set_imm20(0);
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
                    label_use = Some((ops[1], LabelUse::RVJal20));
                    inst = inst.set_rd(rd).set_jimm20(0);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                };
            }

            Encoding::RdN0 => {
                if isign3 == enc_ops1!(Reg) {
                    let rd = ops[0].id();
                    inst = inst.set_rd_n0(rd);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
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
                    if !is_prime_register(rd) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rd_p(rd).set_c_nzimm10lohi(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdPRs1PCUimm1 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    if !is_prime_register(rd) || !is_prime_register(rs1) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let imm = ops[2].as_::<Imm>().value() as i32;

                    inst = inst.set_rd_p(rd).set_rs1_p(rs1).set_c_uimm1(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdPRs1PCUimm2 => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    if !is_prime_register(rd) || !is_prime_register(rs1) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let imm = ops[2].as_::<Imm>().value() as i32;

                    inst = inst.set_rd_p(rd).set_rs1_p(rs1).set_c_uimm2(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdPRs1PCUimm7loCUimm7hi => {
                short = true;
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    if !is_prime_register(rd) || !is_prime_register(rs1) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd_p(rd).set_rs1_p(rs1).set_c_uimm7lohi(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdPRs1PCUimm8loCUimm8hi => {
                short = true;
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    if !is_prime_register(rd) || !is_prime_register(rs1) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd_p(rd).set_rs1_p(rs1).set_c_uimm8lohi(imm as _);
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
                if isign4 == enc_ops4!(Reg, Reg, Imm, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let aq = ops[2].as_::<Imm>().value();
                    let rl = ops[3].as_::<Imm>().value();
                    if !(0..=1).contains(&aq) || !(0..=1).contains(&rl) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    inst = inst.set_rd(rd).set_rs1(rs1).set_aq(aq as _).set_rl(rl as _);
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
                    if rd != rs1 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let off = self.buffer.cur_offset();
                    self.buffer
                        .use_label_at_offset(off, ops[2].as_(), LabelUse::RVPCRelHi20);
                    self.auipc(ops[0].as_::<Gp>(), imm(0));
                    label_use = Some((ops[2], LabelUse::RVPCRelLo12I));
                    inst = inst.set_rd(rd).set_rs1(rs1).set_imm12(0);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1N0 => {
                self.last_error = Some(AsmError::UnsupportedInstruction {
                    reason: "RISC-V RdRs1N0 instructions are not implemented",
                });
                return;
            }

            Encoding::RdRs1Rm => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let rm = ops[2].as_::<Imm>().value() as i32;
                    if !matches!(rm, 0..=4 | 7) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
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
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm)
                    && ops.get(4).is_some_and(|op| op.is_imm())
                {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let rs2 = ops[2].id();
                    let aq = ops[3].as_::<Imm>().value();
                    let rl = ops[4].as_::<Imm>().value();
                    if !(0..=1).contains(&aq) || !(0..=1).contains(&rl) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    inst = inst
                        .set_rd(rd)
                        .set_rs1(rs1)
                        .set_rs2(rs2)
                        .set_aq(aq as _)
                        .set_rl(rl as _);
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
                    if !matches!(rm, 0..=4 | 7) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
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
                    if !matches!(rm, 0..=4 | 7) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

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
                    let rs2 = ops[1].id();
                    inst = inst.set_rd(rd).set_rs2(rs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdZimm5 => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_zimm5(imm);
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
                let rs1 = ops[0].id();
                if !is_prime_register(rs1) {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
                if isign3 == enc_ops2!(Reg, Imm) {
                    let imm = ops[1].as_::<Imm>().value();

                    inst = inst.set_rs1_p(rs1).set_c_bimm9lohi(imm as _);
                } else if isign3 == enc_ops2!(Reg, Label) {
                    label_use = Some((ops[1], LabelUse::RVCB9));
                    inst = inst.set_rs1_p(rs1).set_c_bimm9lohi(0);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1PRs2PCUimm7loCUimm7hi => {
                short = true;
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rs1 = ops[0].id();
                    let rs2 = ops[1].id();
                    if !is_prime_register(rs1) || !is_prime_register(rs2) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rs1_p(rs1).set_rs2_p(rs2).set_c_uimm7lohi(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1PRs2PCUimm8loCUimm8hi => {
                short = true;
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rs1 = ops[0].id();
                    let rs2 = ops[1].id();
                    if !is_prime_register(rs1) || !is_prime_register(rs2) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rs1_p(rs1).set_rs2_p(rs2).set_c_uimm8lohi(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1PRs2PCUimm8hiCUimm8lo => {
                short = true;
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rs1 = ops[0].id();
                    let rs2 = ops[1].id();
                    if !is_prime_register(rs1) || !is_prime_register(rs2) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let imm = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rs1_p(rs1).set_rs2_p(rs2).set_c_uimm8lohi(imm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Rs1Rd => {
                if opcode == Opcode::FENCETSO {
                    // rs1 and rd are unused and canonically zero.
                } else if isign3 == enc_ops2!(Reg, Reg) {
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
                    if !is_prime_register(rs1) || !is_prime_register(rs2) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
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
                    if !is_prime_register(rs1) || !is_prime_register(rs2) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
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
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let vs2 = ops[1].id();
                    let vm = ops[2].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    inst = inst.set_rd(rd).set_vs2(vs2).set_vm(vm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVd => {
                if isign3 == enc_ops2!(Reg, Imm) {
                    let vd = ops[0].id();
                    let vm = ops[1].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    inst = inst.set_vd(vd).set_vm(vm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVs2Rs1Vd => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let rs1 = ops[2].id();
                    let vs2 = ops[1].id();
                    let vm = ops[3].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let vd = ops[0].id();
                    inst = inst.set_vd(vd).set_vm(vm as _).set_rs1(rs1).set_vs2(vs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVs2Simm5Vd => {
                if isign4 == enc_ops4!(Reg, Reg, Imm, Imm) {
                    let simm5 = ops[2].as_::<Imm>().value() as i32;
                    let vs2 = ops[1].id();
                    let vm = ops[3].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let vd = ops[0].id();
                    inst = inst
                        .set_vd(vd)
                        .set_vm(vm as _)
                        .set_simm5(simm5)
                        .set_vs2(vs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVs2Vd => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let vd = ops[0].id();
                    let vs2 = ops[1].id();
                    let vm = ops[2].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    inst = inst.set_vd(vd).set_vs2(vs2).set_vm(vm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVs2Vs1Vd => {
                if isign4 == enc_ops4!(Reg, Reg, Reg, Imm) {
                    let vd = ops[0].id();
                    let vs1 = ops[1].id();
                    let vs2 = ops[2].id();
                    let vm = ops[3].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    inst = inst.set_vd(vd).set_vs1(vs1).set_vs2(vs2).set_vm(vm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::VmVs2Zimm5Vd => {
                if isign4 == enc_ops4!(Reg, Reg, Imm, Imm) {
                    let vd = ops[0].id();
                    let vs2 = ops[1].id();
                    let vm = ops[3].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let zimm5 = ops[2].as_::<Imm>().value() as i8;
                    inst = inst
                        .set_vd(vd)
                        .set_vs2(vs2)
                        .set_vm(vm as _)
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
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let vd = ops[0].id();
                    let vs2 = ops[1].id();
                    let zimm5 = ops[2].as_::<Imm>().value() as i8;
                    inst = inst.set_vd(vd).set_vs2(vs2).set_zimm5(zimm5 as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::Zimm10Zimm5Rd => {
                if isign3 == enc_ops3!(Reg, Imm, Imm) {
                    let rd = ops[0].id();
                    let uimm = ops[1].as_::<Imm>().value() as i8;
                    let vtypei = ops[2].as_::<Imm>().value() as i8;
                    inst = inst.set_rd(rd).set_zimm10(vtypei as _).set_zimm5(uimm as _);
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
                if isign4 == enc_ops4!(Reg, Reg, Imm, Imm) {
                    let vd = ops[0].id();
                    let vs2 = ops[1].id();
                    let imm = ops[2].as_::<Imm>().value();
                    let vm = ops[3].as_::<Imm>().value();
                    if !(0..=1).contains(&vm) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    inst = inst
                        .set_vd(vd)
                        .set_vs2(vs2)
                        .set_zimm6lohi(imm as _)
                        .set_vm(vm as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }
            Encoding::RdRs1N0CNzimm6loCNzimm6hi => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    if imm == 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    } else {
                        inst = inst.set_rd_rs1_n0(rd).set_c_nzimm6lohi(imm);
                    }
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1N0CImm6loCImm6hi => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rd_rs1_n0(rd).set_c_imm6lohi(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1N0CNzuimm6hiCNzuimm6lo => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    if imm == 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    } else {
                        inst = inst.set_rd_rs1_n0(rd).set_c_nzuimm6lohi(imm as u32);
                    }
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1N0CNzuimm6lo => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    if imm == 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    } else {
                        inst = inst.set_rd_rs1_n0(rd).set_c_nzuimm6lo_raw(imm as u32);
                    }
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1N0CRs2N0 => {
                short = true;
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    inst = inst.set_rd_rs1_n0(rd).set_c_rs2_n0(rs1);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1P => {
                short = true;
                if isign3 == enc_ops1!(Reg) {
                    let rd = ops[0].id();
                    if !is_prime_register(rd) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }

                    inst = inst.set_rd_rs1_p(rd);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1PCImm6hiCImm6lo => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    if !is_prime_register(rd) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    inst = inst.set_rd_rs1_p(rd).set_c_imm6lohi(imm);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1PCNzuimm5 => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    if imm == 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    } else {
                        inst = inst.set_rd(rd).set_rs1(0).set_c_nzuimm5(imm as u32);
                    }
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1PCNzuimm6loCNzuimm6hi => {
                short = true;
                if isign3 == enc_ops2!(Reg, Imm) {
                    let rd = ops[0].id();
                    if !is_prime_register(rd) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    let imm = ops[1].as_::<Imm>().value() as i32;
                    if imm == 0 {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    } else {
                        inst = inst.set_rd_rs1_p(rd).set_c_nzuimm6lohi(imm as u32);
                    }
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1PRs2P => {
                short = true;
                if isign3 == enc_ops2!(Reg, Reg) {
                    let rd = ops[0].id();
                    let rs2 = ops[1].id();
                    if !is_prime_register(rd) || !is_prime_register(rs2) {
                        self.last_error = Some(AsmError::InvalidOperand);
                        return;
                    }
                    inst = inst.set_rd_rs1_p(rd).set_rs2_p(rs2);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }

            Encoding::RdRs1Shamtd => {
                if isign3 == enc_ops3!(Reg, Reg, Imm) {
                    let rd = ops[0].id();
                    let rs1 = ops[1].id();
                    let shamt = ops[2].as_::<Imm>().value() as i32;
                    inst = inst.set_rd(rd).set_rs1(rs1).set_shamtd(shamt as _);
                } else {
                    self.last_error = Some(AsmError::InvalidOperand);
                    return;
                }
            }
        }
        let offset = self.buffer.cur_offset();
        if let Some((label, kind)) = label_use {
            self.buffer
                .use_label_at_offset(offset, label.as_::<Label>(), kind);
        }

        if short {
            self.buffer.put2(inst.value as u16);
        } else {
            self.buffer.put4(inst.value);
        }
    }
}

impl crate::core::builder::InstSink for Assembler<'_> {
    fn arch(&self) -> Arch {
        self.environment().arch()
    }

    fn emit_inst(&mut self, inst: &crate::core::inst::Inst) -> Result<(), AsmError> {
        let ops = inst.operands();
        let mut refs: smallvec::SmallVec<[&Operand; 6]> = smallvec::SmallVec::new();
        refs.extend(ops.iter());
        self.try_emit_n(inst.id as i64, &refs)
    }

    fn bind_label(&mut self, label: Label) -> Result<(), AsmError> {
        self.try_bind_label(label)
    }
}
