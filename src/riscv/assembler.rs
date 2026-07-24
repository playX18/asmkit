use super::opcodes::Opcode;
use crate::AsmError;
use crate::core::arch_traits::Arch;
use crate::core::buffer::{CodeBuffer, CodeOffset, ConstantData, LabelUse, Reloc, RelocTarget};
use crate::core::operand::*;
use crate::core::operand::{Imm, Sym};
use crate::core::patch::{PatchableBlock, PatchableSite};
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

    #[cfg(test)]
    fn last_error(&self) -> Option<AsmError> {
        self.buffer.error().cloned()
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

    /// Reserve a nop-filled island for later custom rewriting.
    pub fn reserve_patch_block(
        &mut self,
        size: CodeOffset,
        align: CodeOffset,
    ) -> Result<PatchableBlock, AsmError> {
        self.buffer.reserve_patch_block(size, align)
    }

    pub fn patchable_j(&mut self, label: Label) -> PatchableSite {
        if self.buffer.error().is_some() {
            return unsafe { PatchableSite::new(u32::MAX, LabelUse::RVJal20, 0) };
        }
        let checkpoint = self.buffer.checkpoint();
        let offset = self.buffer.cur_offset();
        self.j(label);
        let _ = self
            .buffer
            .record_label_patch_site(offset, label, LabelUse::RVJal20);
        if self.buffer.error().is_some() {
            self.buffer.rollback(checkpoint);
            return unsafe { PatchableSite::new(u32::MAX, LabelUse::RVJal20, 0) };
        }
        // SAFETY: `j` emits a JAL-style instruction at `offset`.
        unsafe { PatchableSite::new(offset, LabelUse::RVJal20, 0) }
    }

    pub fn patchable_call(&mut self, label: Label) -> PatchableSite {
        if self.buffer.error().is_some() {
            return unsafe { PatchableSite::new(u32::MAX, LabelUse::RVJal20, 0) };
        }
        let checkpoint = self.buffer.checkpoint();
        let offset = self.buffer.cur_offset();
        self.jal(RA, label);
        let _ = self
            .buffer
            .record_label_patch_site(offset, label, LabelUse::RVJal20);
        if self.buffer.error().is_some() {
            self.buffer.rollback(checkpoint);
            return unsafe { PatchableSite::new(u32::MAX, LabelUse::RVJal20, 0) };
        }
        // SAFETY: `jal` emits a JAL instruction at `offset`.
        unsafe { PatchableSite::new(offset, LabelUse::RVJal20, 0) }
    }

    /// Materialize `imm` into `rd` with a fixed-size sequence and a patchable literal.
    ///
    /// Layout (RV64): `auipc; ld; jal; .dword` — the returned block covers the 8-byte literal.
    /// Layout (RV32): `auipc; lw; jal; .word` — the returned block covers the 4-byte literal.
    ///
    /// Rewrite with [`PatchableBlock::repatch_u64`] / [`PatchableBlock::repatch_u32`].
    pub fn patchable_li(&mut self, rd: Gp, imm: impl Into<i64>) -> PatchableBlock {
        let arch = self.buffer.env().arch();
        let value = imm.into();
        if self.buffer.error().is_some() {
            return unsafe { PatchableBlock::new(u32::MAX, 4, arch) };
        }
        let checkpoint = self.buffer.checkpoint();

        if self.is_32bit() {
            self.auipc(rd, crate::core::operand::imm(0));
            self.lw(rd, rd, crate::core::operand::imm(8));
            self.jal(ZERO, crate::core::operand::imm(8));
            let lit = self.buffer.cur_offset();
            self.buffer.write_u32(value as u32);
            let _ = self.buffer.record_patch_block(lit, 4, 4);
            if self.buffer.error().is_some() {
                self.buffer.rollback(checkpoint);
                return unsafe { PatchableBlock::new(u32::MAX, 4, arch) };
            }
            // SAFETY: literal word recorded as a patch block at `lit`.
            unsafe { PatchableBlock::new(lit, 4, arch) }
        } else {
            self.auipc(rd, crate::core::operand::imm(0));
            self.ld(rd, rd, crate::core::operand::imm(8));
            self.jal(ZERO, crate::core::operand::imm(12));
            let lit = self.buffer.cur_offset();
            self.buffer.write_u64(value as u64);
            let _ = self.buffer.record_patch_block(lit, 8, 4);
            if self.buffer.error().is_some() {
                self.buffer.rollback(checkpoint);
                return unsafe { PatchableBlock::new(u32::MAX, 8, arch) };
            }
            // SAFETY: literal dword recorded as a patch block at `lit`.
            unsafe { PatchableBlock::new(lit, 8, arch) }
        }
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
                    let rs1 = ops[1].id();
                    inst = inst.set_rd(rd).set_rs1(rs1);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::target::Environment;
    use crate::riscv::opcodes::{
        MATCH_C_LW, MATCH_C_NOT, MATCH_FADD_S, MATCH_FMADD_S, MATCH_FMV_W_X, MATCH_FMV_X_W,
        MATCH_LR_W, MATCH_SSAMOSWAP_W, MATCH_SSRDP, MATCH_VAESKF1_VI, MATCH_VFADD_VF, MATCH_VLE8_V,
    };
    use crate::riscv::operands::regs::*;
    use std::vec::Vec;

    fn rv32(buf: &mut CodeBuffer) -> Assembler<'_> {
        Assembler::new(buf)
    }

    fn data(buf: &mut CodeBuffer) -> Vec<u8> {
        buf.finish().unwrap().data().to_vec()
    }

    #[test]
    fn default_environment_is_rv64() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let asm = Assembler::new(&mut buf);
        assert!(!asm.is_32bit());
        assert_eq!(asm.environment().arch(), Arch::RISCV64);
    }

    #[test]
    fn fixed_fences_have_no_artificial_operands() {
        for arch in [Arch::RISCV32, Arch::RISCV64] {
            let mut buf = CodeBuffer::new(Environment::new(arch));
            {
                let mut asm = Assembler::new(&mut buf);
                asm.fence_i();
                asm.fence_tso();
                assert_eq!(asm.last_error(), None);
            }
            assert_eq!(
                data(&mut buf),
                [0x0000_100fu32.to_le_bytes(), 0x8330_000fu32.to_le_bytes()].concat()
            );
        }
    }

    #[test]
    fn baseline_rejects_optional_extensions_before_writing() {
        let mut buf = CodeBuffer::new(Environment::baseline(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        let vm = imm(1);

        let error = asm
            .try_emit_n(
                Opcode::VADDVV as i64,
                &[
                    V0.as_operand(),
                    V1.as_operand(),
                    V2.as_operand(),
                    vm.as_operand(),
                ],
            )
            .unwrap_err();

        assert!(
            matches!(error, AsmError::MissingCpuFeature { feature } if feature.contains("vadd.vv") && feature.contains("rv_v"))
        );
        assert!(asm.buffer.data().is_empty());
    }

    #[test]
    fn optional_extensions_are_enabled_by_default() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        let vm = imm(1);

        asm.try_emit_n(
            Opcode::VADDVV as i64,
            &[
                V0.as_operand(),
                V1.as_operand(),
                V2.as_operand(),
                vm.as_operand(),
            ],
        )
        .unwrap();

        assert_eq!(asm.buffer.data().len(), 4);
    }

    #[test]
    fn invalid_raw_opcode_is_rejected_without_writing() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);

        asm.emit_n(-1, &[]);
        assert_eq!(asm.last_error(), Some(AsmError::InvalidInstruction));
        assert!(asm.buffer.data().is_empty());

        asm.buffer.clear();
        asm.emit_n(i64::MAX, &[]);
        assert_eq!(asm.last_error(), Some(AsmError::InvalidInstruction));
        assert!(asm.buffer.data().is_empty());
    }

    #[test]
    fn raw_emission_rejects_malformed_or_extra_operands_without_mutation() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        let malformed = Operand {
            signature: OperandSignature::from(7),
            base_id: 0,
            data: [0; 2],
        };
        let invalid_register = Operand {
            signature: OperandSignature::from(
                OperandType::Reg as u32 | (31 << OperandSignature::REG_TYPE_SHIFT),
            ),
            base_id: 0,
            data: [0; 2],
        };
        let none = Operand::new();

        assert_eq!(
            asm.try_emit_n(Opcode::ADD as i64, &[&malformed]),
            Err(AsmError::InvalidOperand)
        );
        assert_eq!(
            asm.try_emit_n(Opcode::ADD as i64, &[&invalid_register]),
            Err(AsmError::InvalidOperand)
        );
        assert_eq!(
            asm.try_emit_n(Opcode::ADD as i64, &[RA.as_operand()]),
            Err(AsmError::InvalidOperand)
        );
        assert_eq!(
            asm.try_emit_n(
                Opcode::ADD as i64,
                &[&none, &none, &none, &none, &none, &none],
            ),
            Err(AsmError::InvalidOperand)
        );
        assert!(asm.buffer.error().is_none());
        assert!(asm.buffer.data().is_empty());
    }

    #[test]
    fn macro_helpers_reject_wrong_operand_kinds_without_panicking() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);

        asm.la(RA, imm(0));
        assert_eq!(asm.buffer.error(), Some(&AsmError::InvalidOperand));
        assert!(asm.buffer.data().is_empty());

        asm.buffer.clear();
        asm.call(Operand::new());
        assert_eq!(asm.buffer.error(), Some(&AsmError::InvalidOperand));
        assert!(asm.buffer.data().is_empty());
    }

    #[test]
    fn raw_label_registration_error_rolls_back_emission() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        let invalid_label = Label::from_id(0);

        assert_eq!(
            asm.try_emit_n(
                Opcode::JAL as i64,
                &[RA.as_operand(), invalid_label.as_operand()]
            ),
            Err(AsmError::InvalidArgument)
        );
        assert_eq!(asm.buffer.error(), Some(&AsmError::InvalidArgument));
        assert!(asm.buffer.data().is_empty());
    }

    #[test]
    fn failed_call_rolls_back_the_whole_sequence() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);

        asm.call(Label::from_id(0));

        assert_eq!(asm.buffer.error(), Some(&AsmError::InvalidArgument));
        assert!(asm.buffer.data().is_empty());
    }

    #[test]
    fn unimplemented_encodings_return_typed_errors() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        let one = imm(1);
        let sixteen = imm(16);
        let zero = imm(0);

        asm.emit_n(
            Opcode::CMPOP as i64,
            &[one.as_operand(), sixteen.as_operand()],
        );
        assert!(matches!(
            asm.last_error(),
            Some(AsmError::UnsupportedInstruction { .. })
        ));
        assert!(asm.buffer.data().is_empty());

        asm.buffer.clear();
        asm.emit_n(Opcode::CMMVA01S as i64, &[A0.as_operand(), A1.as_operand()]);
        assert!(matches!(
            asm.last_error(),
            Some(AsmError::UnsupportedInstruction { .. })
        ));
        assert!(asm.buffer.data().is_empty());

        asm.buffer.clear();
        asm.emit_n(
            Opcode::MOPRN as i64,
            &[
                zero.as_operand(),
                zero.as_operand(),
                zero.as_operand(),
                A0.as_operand(),
                A1.as_operand(),
            ],
        );
        assert!(matches!(
            asm.last_error(),
            Some(AsmError::UnsupportedInstruction { .. })
        ));
        assert!(asm.buffer.data().is_empty());

        asm.buffer.clear();
        asm.emit_n(
            Opcode::MOPRRN as i64,
            &[
                zero.as_operand(),
                zero.as_operand(),
                A0.as_operand(),
                A1.as_operand(),
                A2.as_operand(),
            ],
        );
        assert!(matches!(
            asm.last_error(),
            Some(AsmError::UnsupportedInstruction { .. })
        ));
        assert!(asm.buffer.data().is_empty());

        asm.buffer.clear();
        asm.emit_n(Opcode::CSEXTW as i64, &[A0.as_operand()]);
        assert!(matches!(
            asm.last_error(),
            Some(AsmError::UnsupportedInstruction { .. })
        ));
        assert!(asm.buffer.data().is_empty());
    }

    #[test]
    fn rv32_encodes_base_and_rv32_only_instructions() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV32));
        {
            let mut asm = rv32(&mut buf);
            asm.add(A0, A1, A2);
            // rv32-only variant of slli (shamt restricted to 5 bits).
            asm.slli_rv32(A0, A1, 5);
            assert_eq!(asm.last_error(), None);
        }
        let expected = [0x00C58533u32.to_le_bytes(), 0x00559513u32.to_le_bytes()].concat();
        assert_eq!(data(&mut buf), expected);
    }

    #[test]
    fn rv32_rejects_rv64_only_instructions() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV32));
        {
            let mut asm = rv32(&mut buf);
            asm.ld(A0, A1, imm(0));
            asm.addw(A0, A1, A2);
            assert_eq!(asm.last_error(), Some(AsmError::InvalidInstruction));
        }
        assert_eq!(buf.finish().err(), Some(AsmError::InvalidInstruction));
    }

    #[test]
    fn rv64_rejects_rv32_only_instructions() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        {
            let mut asm = Assembler::new(&mut buf);
            asm.slli_rv32(A0, A1, imm(5));
            assert_eq!(asm.last_error(), Some(AsmError::InvalidInstruction));
        }
        assert_eq!(buf.finish().err(), Some(AsmError::InvalidInstruction));
    }

    #[test]
    fn rv64_encodes_rv64_only_instructions() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        {
            let mut asm = Assembler::new(&mut buf);
            asm.ld(A0, A1, imm(0));
            asm.addw(A0, A1, A2);
            assert_eq!(asm.last_error(), None);
        }
        let expected = [0x0005B503u32.to_le_bytes(), 0x00C5853Bu32.to_le_bytes()].concat();
        assert_eq!(data(&mut buf), expected);
    }

    #[test]
    fn lpad_encodes_on_both_xlen() {
        // lpad is auipc with rd=x0; U-type immediates are passed pre-shifted
        // (bits [31:12] of the operand, as for lui/auipc).
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV32));
        {
            let mut asm = rv32(&mut buf);
            asm.lpad(imm(0x12345 << 12));
            assert_eq!(asm.last_error(), None);
        }
        assert_eq!(data(&mut buf), 0x12345017u32.to_le_bytes());

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        {
            let mut asm = Assembler::new(&mut buf);
            asm.lpad(imm(0x12345 << 12));
            assert_eq!(asm.last_error(), None);
        }
        assert_eq!(data(&mut buf), 0x12345017u32.to_le_bytes());
    }

    #[test]
    fn zicfiss_shadow_stack_instructions_encode() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV32));
        {
            let mut asm = rv32(&mut buf);
            asm.sspush_x1();
            asm.ssrdp(A0);
            asm.ssamoswap_w(A0, A1, A2, imm(0), imm(0));
            assert_eq!(asm.last_error(), None);
        }
        let sspush_x1 = 0xCE104073u32; // architecturally fixed encoding
        let ssrdp_a0 = MATCH_SSRDP | (10 << 7);
        let ssamoswap_w = MATCH_SSAMOSWAP_W | (10 << 7) | (11 << 15) | (12 << 20);
        let expected = [
            sspush_x1.to_le_bytes(),
            ssrdp_a0.to_le_bytes(),
            ssamoswap_w.to_le_bytes(),
        ]
        .concat();
        assert_eq!(data(&mut buf), expected);
    }

    #[test]
    fn rv32_accepts_zclsd_compressed_load_store_pair() {
        // Zclsd makes the c.ld/c.sd encodings available on rv32 (they reuse the
        // rv32 Zcf encoding space).
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV32));
        {
            let mut asm = rv32(&mut buf);
            asm.c_ld(A0, A1, imm(0));
            // Stores take (base, source, imm), like sw.
            asm.c_sd(A1, A0, imm(0));
            assert_eq!(asm.last_error(), None);
        }
        // c.ld a0', 0(a1') = 0x6188, c.sd a0', 0(a1') = 0xE188.
        assert_eq!(data(&mut buf), [0x88, 0x61, 0x88, 0xE1]);
    }

    #[test]
    fn compressed_loads_use_prime_register_fields() {
        // c.lw encodes rd'/rs1' as 3-bit fields (registers x8..x15).
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        {
            let mut asm = Assembler::new(&mut buf);
            asm.c_lw(S0, A5, imm(0));
            assert_eq!(asm.last_error(), None);
        }
        assert_eq!(
            data(&mut buf),
            ((MATCH_C_LW | (7 << 7)) as u16).to_le_bytes()
        );
    }

    #[test]
    fn typed_vector_fields_encode_and_validate() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        {
            let mut asm = Assembler::new(&mut buf);
            asm.vle8_v(V1, A0, imm(1), imm(3));
            assert_eq!(asm.last_error(), None);
        }
        let expected = MATCH_VLE8_V | (1 << 7) | (10 << 15) | (1 << 25) | (3 << 29);
        assert_eq!(data(&mut buf), expected.to_le_bytes());

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        asm.vle8_v(V1, A0, imm(2), imm(0));
        assert_eq!(asm.last_error(), Some(AsmError::InvalidOperand));
        asm.buffer.clear();
        asm.vle8_v(V1, A0, imm(1), imm(8));
        assert_eq!(asm.last_error(), Some(AsmError::InvalidOperand));

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        asm.vaeskf1_vi(V1, V2, 3);
        assert_eq!(asm.last_error(), None);
        let expected = MATCH_VAESKF1_VI | (1 << 7) | (3 << 15) | (2 << 20);
        assert_eq!(asm.buffer.data(), expected.to_le_bytes());

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        asm.vfadd_vf(V1, V2, F0, 1);
        asm.fmv_w_x(F1, A0);
        asm.fmv_x_w(A1, F2);
        assert_eq!(asm.last_error(), None);
        let expected = [
            (MATCH_VFADD_VF | (1 << 7) | (2 << 20) | (1 << 25)).to_le_bytes(),
            (MATCH_FMV_W_X | (1 << 7) | (10 << 15)).to_le_bytes(),
            (MATCH_FMV_X_W | (11 << 7) | (2 << 15)).to_le_bytes(),
        ]
        .concat();
        assert_eq!(asm.buffer.data(), expected);
    }

    #[test]
    fn pc_relative_label_register_coupling_is_transactional() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        let label = asm.get_label();
        asm.emit_n(
            Opcode::JALR as i64,
            &[A0.as_operand(), A1.as_operand(), label.as_operand()],
        );
        assert_eq!(asm.last_error(), Some(AsmError::InvalidOperand));
        assert!(asm.buffer.data().is_empty());

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        let label = asm.get_label();
        asm.emit_n(
            Opcode::LW as i64,
            &[A0.as_operand(), A1.as_operand(), label.as_operand()],
        );
        assert_eq!(asm.last_error(), Some(AsmError::InvalidOperand));
        assert!(asm.buffer.data().is_empty());

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        let label = asm.get_label();
        asm.emit_n(
            Opcode::JALR as i64,
            &[A0.as_operand(), A0.as_operand(), label.as_operand()],
        );
        assert_eq!(asm.last_error(), None);
        assert_eq!(asm.buffer.data().len(), 8);

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        let label = asm.get_label();
        asm.emit_n(
            Opcode::LW as i64,
            &[A0.as_operand(), A0.as_operand(), label.as_operand()],
        );
        assert_eq!(asm.last_error(), None);
        assert_eq!(asm.buffer.data().len(), 8);
    }

    #[test]
    fn typed_atomic_and_rounding_fields_validate() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        {
            let mut asm = Assembler::new(&mut buf);
            asm.lr_w(A0, A1, 0, 0);
            asm.lr_w(A0, A1, 0, 1);
            asm.lr_w(A0, A1, 1, 0);
            asm.lr_w(A0, A1, 1, 1);
            assert_eq!(asm.last_error(), None);
        }
        let base = MATCH_LR_W | (10 << 7) | (11 << 15);
        let expected = [
            base.to_le_bytes(),
            (base | (1 << 25)).to_le_bytes(),
            (base | (1 << 26)).to_le_bytes(),
            (base | (3 << 25)).to_le_bytes(),
        ]
        .concat();
        assert_eq!(data(&mut buf), expected);

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        asm.fadd_s(F0, F1, F2, imm(5));
        assert_eq!(asm.last_error(), Some(AsmError::InvalidOperand));
        asm.buffer.clear();
        asm.fadd_s(F0, F1, F2, imm(7));
        assert_eq!(asm.last_error(), None);
        let expected = MATCH_FADD_S | (1 << 15) | (2 << 20) | (7 << 12);
        assert_eq!(asm.buffer.data(), expected.to_le_bytes());

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        asm.fmadd_s(F0, F1, F2, F3, 0);
        assert_eq!(asm.last_error(), None);
        let expected = MATCH_FMADD_S | (1 << 15) | (2 << 20) | (3 << 27);
        assert_eq!(asm.buffer.data(), expected.to_le_bytes());
    }

    #[test]
    fn compressed_prime_registers_reject_non_prime_ids() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        asm.c_lw(T2, A1, imm(0));
        assert_eq!(asm.last_error(), Some(AsmError::InvalidOperand));
        assert!(asm.buffer.data().is_empty());

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        {
            let mut asm = Assembler::new(&mut buf);
            asm.c_not(S0);
            assert_eq!(asm.last_error(), None);
        }
        assert_eq!(
            data(&mut buf),
            ((MATCH_C_NOT & !(0x7 << 7)) as u16).to_le_bytes()
        );

        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let mut asm = Assembler::new(&mut buf);
        asm.c_not(T2);
        assert_eq!(asm.last_error(), Some(AsmError::InvalidOperand));
        assert!(asm.buffer.data().is_empty());

        asm.buffer.clear();
        asm.c_lw(A0, T2, imm(0));
        assert_eq!(asm.last_error(), Some(AsmError::InvalidOperand));
        assert!(asm.buffer.data().is_empty());

        asm.buffer.clear();
        asm.c_lw(A6, A1, imm(0));
        assert_eq!(asm.last_error(), Some(AsmError::InvalidOperand));
        assert!(asm.buffer.data().is_empty());
    }

    #[test]
    fn register_label_encodings_use_the_label_operand() {
        for arch in [Arch::RISCV32, Arch::RISCV64] {
            let mut buf = CodeBuffer::new(Environment::new(arch));
            let target = buf.get_label();
            {
                let mut asm = Assembler::new(&mut buf);
                asm.auipc(A0, target);
                asm.jal(A1, target);
                asm.bind_label(target);
                assert_eq!(asm.last_error(), None);
            }
            assert_eq!(buf.label_offset(target), 8);
            assert_eq!(
                data(&mut buf),
                [0x0000_0517u32.to_le_bytes(), 0x0040_05EFu32.to_le_bytes()].concat()
            );

            let mut buf = CodeBuffer::new(Environment::new(arch));
            let target = buf.get_label();
            {
                let mut asm = Assembler::new(&mut buf);
                asm.bind_label(target);
                asm.auipc(A0, target);
                asm.jal(A1, target);
                assert_eq!(asm.last_error(), None);
            }
            assert_eq!(buf.label_offset(target), 0);
            assert_eq!(
                data(&mut buf),
                [0x0000_0517u32.to_le_bytes(), 0xFFDFF5EFu32.to_le_bytes()].concat()
            );
        }
    }

    #[test]
    fn patchable_li_literal_can_be_rewritten() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let block = {
            let mut asm = Assembler::new(&mut buf);
            asm.patchable_li(A0, 0x1122_3344_5566_7788u64 as i64)
        };
        let code = buf.finish_patched().unwrap();
        assert_eq!(block.size(), 8);
        assert_eq!(
            &code.data()[block.offset() as usize..][..8],
            &0x1122_3344_5566_7788u64.to_le_bytes()
        );

        let mut bytes = code.data().to_vec();
        unsafe {
            block.repatch_u64(&mut bytes, 0xAABB_CCDD_EEFF_0011).unwrap();
        }
        assert_eq!(
            &bytes[block.offset() as usize..][..8],
            &0xAABB_CCDD_EEFF_0011u64.to_le_bytes()
        );
    }

    #[test]
    fn patchable_j_can_be_retargeted_offline() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let (site, alt) = {
            let mut asm = Assembler::new(&mut buf);
            let target = asm.get_label();
            let alt = asm.get_label();
            let site = asm.patchable_j(target);
            asm.bind_label(target);
            asm.addi(A0, A0, imm(1));
            asm.bind_label(alt);
            asm.addi(A0, A0, imm(2));
            (site, asm.label_offset(alt))
        };
        let code = buf.finish_patched().unwrap();
        let mut bytes = code.data().to_vec();
        unsafe {
            site.retarget(&mut bytes, alt).unwrap();
        }
        // Catalog still describes the original target; the handle rewrote the bytes.
        assert_ne!(bytes, code.data());
    }
}
