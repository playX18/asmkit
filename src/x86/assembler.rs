#![allow(dead_code)]
use super::emit::{self, PendingPrefixes};
use super::emitter::{CallEmitter, JmpEmitter, MovEmitter};
use super::instdb::InstId;
use super::operands::*;
use crate::{
    X86Error,
    core::{
        arch_traits::Arch,
        buffer::{CodeBuffer, CodeOffset, ConstantData, LabelUse},
        globals::InstOptions,
        operand::*,
        patch::{DataEncoding, DataLabel, PatchableJump, PatchableRegion},
        target::Environment,
    },
};

/// X86/X64 Assembler implementation.
pub struct Assembler<'a> {
    pub(crate) buffer: &'a mut CodeBuffer,
    flags: u64,
    extra_reg: Reg,
}

const RC_RN: u64 = 0x0000000;
const RC_RD: u64 = 0x0800000;
const RC_RU: u64 = 0x1000000;
const RC_RZ: u64 = 0x1800000;
const RC_MASK: u64 = RC_RD | RC_RU;
const RC_ENABLED: u64 = 0x4000000;
const SEG_MASK: u64 = 0xe0000000;
const LONG: u64 = 0x100000000;

/// Bit 37: LOCK prefix (bits 23/24 are the rounding-mode RC bits, bits 20/21
/// REP/REPNE, bit 26 SAE/ER enable, bits 29..=31 segment, bit 32 long-form,
/// bits 33..=35 mask id, bit 36 zeroing mask, bit 38 short-form).
const OPC_LOCK: u64 = 0x2000000000;
/// Bit 38: short (rel8) branch form, also for labels not yet bound.
const SHORT: u64 = 0x4000000000;
/// Bit 36: AVX-512 zeroing mask `{z}` (bits 33..=35 carry the mask register id).
const OPC_Z: u64 = 0x1000000000;

impl crate::core::builder::InstSink for Assembler<'_> {
    fn arch(&self) -> Arch {
        self.environment().arch()
    }

    fn emit_inst(&mut self, inst: &crate::core::inst::Inst) -> Result<(), crate::AsmError> {
        let ops = inst.operands();
        let mut refs: smallvec::SmallVec<[&Operand; 6]> = smallvec::SmallVec::new();
        refs.extend(ops.iter());

        let extra_reg = inst.extra_reg();
        let mask_id = match extra_reg.signature.try_op_type() {
            Some(OperandType::None) if extra_reg.signature.bits() == 0 => 0,
            Some(OperandType::Reg) if extra_reg.signature.try_reg_type() == Some(RegType::Mask) => {
                extra_reg.id()
            }
            _ => {
                return Err(X86Error::InvalidMasking {
                    mask_reg: extra_reg.id(),
                    reason: "x86 extra register must be a mask register",
                }
                .into());
            }
        };
        self.try_emit_n_with_prefixes(
            inst.id(),
            &refs,
            PendingPrefixes {
                options: inst.options(),
                segment_id: 0,
                mask_id,
            },
        )
    }

    fn bind_label(&mut self, label: Label) -> Result<(), crate::AsmError> {
        self.try_bind_label(label)
    }
}

impl<'a> Assembler<'a> {
    /// Collects the pending prefix flags set by the prefix setters (`rep`, `lock`,
    /// `seg`, `k`, sae/rounding, `long`) and resets them, mapping them onto the
    /// asmjit-style [`InstOptions`]/extra-reg model consumed by [`Assembler::emit_n`].
    fn take_pending_prefixes(&mut self) -> PendingPrefixes {
        let flags = self.flags;
        self.flags = 0;

        let mut prefixes = PendingPrefixes::default();
        if flags & OPC_LOCK != 0 {
            prefixes.options |= InstOptions::X86_LOCK;
        }
        if flags & 0x200000 != 0 {
            prefixes.options |= InstOptions::X86_REP;
        }
        if flags & 0x100000 != 0 {
            prefixes.options |= InstOptions::X86_REPNE;
        }
        if flags & LONG != 0 {
            prefixes.options |= InstOptions::LONG_FORM;
        }
        if flags & SHORT != 0 {
            prefixes.options |= InstOptions::SHORT_FORM;
        }
        if flags & OPC_Z != 0 {
            prefixes.options |= InstOptions::X86_ZMASK;
        }
        if flags & RC_ENABLED != 0 {
            let rc = flags & (RC_RD | RC_RU);
            prefixes.options |= if rc == RC_RD {
                InstOptions::X86_ER | InstOptions::X86_RD_SAE
            } else if rc == RC_RU {
                InstOptions::X86_ER | InstOptions::X86_RU_SAE
            } else if rc == RC_RZ {
                InstOptions::X86_ER | InstOptions::X86_RZ_SAE
            } else {
                // `sae()` and `rn_sae()` share the same flag bits (RC_RN is zero);
                // both encode identically (EVEX.b with LL=00).
                InstOptions::X86_SAE
            };
        }
        prefixes.segment_id = ((flags & SEG_MASK) >> 29) as u32;
        prefixes.mask_id = ((flags >> 33) & 0x7) as u32;
        prefixes
    }

    /// Emits one instruction by id with explicit operands, using the asmjit-style
    /// InstInfo → signature match → emit-handler pipeline (the new primary emit
    /// path). Pending prefixes set by the prefix setters apply.
    pub fn emit_n(&mut self, id: impl Into<u32>, ops: &[&Operand]) {
        if let Err(error) = self.try_emit_n(id, ops) {
            self.buffer.record_error(error);
        }
    }

    pub fn try_emit_n(
        &mut self,
        id: impl Into<u32>,
        ops: &[&Operand],
    ) -> Result<(), crate::AsmError> {
        if let Some(error) = self.buffer.error().cloned() {
            return Err(error);
        }
        let prefixes = self.take_pending_prefixes();
        self.try_emit_n_with_prefixes(id, ops, prefixes)
    }

    fn try_emit_n_with_prefixes(
        &mut self,
        id: impl Into<u32>,
        ops: &[&Operand],
        prefixes: PendingPrefixes,
    ) -> Result<(), crate::AsmError> {
        if let Some(error) = self.buffer.error().cloned() {
            return Err(error);
        }
        let checkpoint = self.buffer.checkpoint();
        if let Err(error) = emit::emit_n(self.buffer, id.into(), ops, prefixes, self.is_32bit()) {
            self.buffer.rollback(checkpoint);
            return Err(error);
        }
        Ok(())
    }

    pub fn new(buf: &'a mut CodeBuffer) -> Self {
        if !matches!(buf.env().arch(), Arch::X86 | Arch::X64) {
            return Self::poisoned(buf, crate::AsmError::InvalidArch);
        }
        Self::unchecked(buf)
    }

    pub fn try_new(buf: &'a mut CodeBuffer) -> Result<Self, crate::AsmError> {
        if !matches!(buf.env().arch(), Arch::X86 | Arch::X64) {
            return Err(crate::AsmError::InvalidArch);
        }
        Ok(Self::unchecked(buf))
    }

    fn unchecked(buf: &'a mut CodeBuffer) -> Self {
        Self {
            buffer: buf,
            extra_reg: Reg::new(),
            flags: 0,
        }
    }

    fn poisoned(buf: &'a mut CodeBuffer, error: crate::AsmError) -> Self {
        buf.record_error(error);
        Self {
            buffer: buf,
            extra_reg: Reg::new(),
            flags: 0,
        }
    }

    /// Returns the environment (arch/mode) this assembler targets.
    pub fn environment(&self) -> &Environment {
        self.buffer.env()
    }

    /// Tests whether the assembler targets 32-bit X86 mode.
    pub fn is_32bit(&self) -> bool {
        self.buffer.env().is_32bit()
    }

    /// Tests whether the assembler targets 64-bit X64 mode.
    pub fn is_64bit(&self) -> bool {
        self.buffer.env().is_64bit()
    }

    pub fn sae(&mut self) -> &mut Self {
        self.set_rounding(RC_RN)
    }

    pub fn rn_sae(&mut self) -> &mut Self {
        self.set_rounding(RC_RN)
    }

    pub fn rd_sae(&mut self) -> &mut Self {
        self.set_rounding(RC_RD)
    }
    pub fn ru_sae(&mut self) -> &mut Self {
        self.set_rounding(RC_RU)
    }

    pub fn rz_sae(&mut self) -> &mut Self {
        self.set_rounding(RC_RZ)
    }

    fn set_rounding(&mut self, rounding: u64) -> &mut Self {
        let mask = RC_ENABLED | RC_MASK;
        let pending = self.flags & mask;
        let requested = RC_ENABLED | rounding;
        if pending != 0 && pending != requested {
            self.buffer
                .record_error(crate::AsmError::X86(X86Error::InvalidRoundingControl {
                    rc: requested,
                    reason: "conflicting pending rounding modes",
                }));
            return self;
        }
        self.flags = (self.flags & !mask) | requested;
        self
    }

    pub fn seg(&mut self, sreg: SReg) -> &mut Self {
        let segment_id = sreg.id();
        if !(SReg::ES..=SReg::GS).contains(&segment_id) {
            self.buffer
                .record_error(crate::AsmError::X86(X86Error::InvalidPrefix {
                    prefix: segment_id as u64,
                    reason: "invalid segment override",
                }));
            return self;
        }
        let pending = (self.flags & SEG_MASK) >> 29;
        if pending != 0 && pending != segment_id as u64 {
            self.buffer
                .record_error(crate::AsmError::X86(X86Error::InvalidPrefix {
                    prefix: segment_id as u64,
                    reason: "conflicting pending segment overrides",
                }));
            return self;
        }
        self.flags = (self.flags & !SEG_MASK) | (segment_id as u64) << 29;
        self
    }

    pub fn fs(&mut self) -> &mut Self {
        self.seg(FS)
    }

    pub fn gs(&mut self) -> &mut Self {
        self.seg(GS)
    }

    pub fn k(&mut self, k: KReg) -> &mut Self {
        let mask_id = k.id();
        if !(1..=7).contains(&mask_id) {
            self.buffer
                .record_error(crate::AsmError::X86(X86Error::InvalidMasking {
                    mask_reg: mask_id,
                    reason: "mask register must be k1..k7",
                }));
            return self;
        }
        let pending = (self.flags >> 33) & 0x7;
        if pending != 0 && pending != mask_id as u64 {
            self.buffer
                .record_error(crate::AsmError::X86(X86Error::InvalidMasking {
                    mask_reg: mask_id,
                    reason: "conflicting pending mask registers",
                }));
            return self;
        }
        self.flags = (self.flags & !(0x7 << 33)) | (mask_id as u64) << 33;

        self
    }

    /// AVX-512 zeroing mask `{z}` for the next instruction (requires `k()`).
    pub fn z(&mut self) -> &mut Self {
        self.flags |= OPC_Z;
        self
    }

    pub fn rep(&mut self) -> &mut Self {
        self.flags |= 0x200000;
        self
    }

    pub fn repnz(&mut self) -> &mut Self {
        self.flags |= 0x100000;
        self
    }

    pub fn repz(&mut self) -> &mut Self {
        self.rep()
    }

    pub fn lock(&mut self) -> &mut Self {
        self.flags |= OPC_LOCK;
        self
    }

    pub fn long(&mut self) -> &mut Self {
        self.flags |= LONG;
        self
    }

    pub fn short(&mut self) -> &mut Self {
        self.flags |= SHORT;
        self
    }

    pub fn get_label(&mut self) -> Label {
        self.buffer.get_label()
    }

    pub fn bind_label(&mut self, label: Label) {
        if let Err(error) = self.try_bind_label(label) {
            self.buffer.record_error(error);
        }
    }

    pub fn try_bind_label(&mut self, label: Label) -> Result<(), crate::AsmError> {
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

    pub fn error(&self) -> Option<&crate::AsmError> {
        self.buffer.error()
    }

    /// Reserve a nop-filled region for later rewriting.
    pub fn reserve_patch_region(
        &mut self,
        size: CodeOffset,
        align: CodeOffset,
    ) -> Result<PatchableRegion, crate::AsmError> {
        self.buffer.reserve_patch_region(size, align)
    }

    /// Emits a rel32 jump through `emit`, returning its displacement as a
    /// patchable jump.
    fn patchable_rel32(&mut self, emit: impl FnOnce(&mut Self)) -> PatchableJump {
        let start = self.buffer.cur_offset();
        let previous_error = self.buffer.error().cloned();
        self.long();
        emit(self);
        let end = self.buffer.cur_offset();
        if self.buffer.error().cloned() != previous_error || end < start + 5 {
            return PatchableJump::invalid(LabelUse::X86JmpRel32);
        }
        // The displacement ends the instruction.
        PatchableJump::new(end - 4, LabelUse::X86JmpRel32)
    }

    /// `jmp` to `label` with a rel32 displacement that can be retargeted.
    pub fn patchable_jmp(&mut self, label: Label) -> PatchableJump {
        self.patchable_rel32(|asm| asm.jmp(label))
    }

    /// `call` to `label` with a rel32 displacement that can be retargeted.
    pub fn patchable_call(&mut self, label: Label) -> PatchableJump {
        self.patchable_rel32(|asm| asm.call(label))
    }

    /// Conditional jump to `label` with a rel32 displacement that can be
    /// retargeted.
    pub fn patchable_jcc(&mut self, cc: CondCode, label: Label) -> PatchableJump {
        const JCC: [InstId; 16] = [
            InstId::Jo,
            InstId::Jno,
            InstId::Jb,
            InstId::Jnb,
            InstId::Jz,
            InstId::Jnz,
            InstId::Jbe,
            InstId::Jnbe,
            InstId::Js,
            InstId::Jns,
            InstId::Jp,
            InstId::Jnp,
            InstId::Jl,
            InstId::Jnl,
            InstId::Jle,
            InstId::Jnle,
        ];
        self.patchable_rel32(|asm| {
            asm.emit_n(JCC[cc.code() as usize] as u32, &[label.as_operand()])
        })
    }

    /// `mov` of an immediate into a `Gp32`/`Gp64` register, with the
    /// immediate at a fixed size (4 or 8 bytes) so it can be rewritten.
    pub fn patchable_mov<A, B>(&mut self, dst: A, src: B) -> DataLabel
    where
        A: OperandCast + Copy,
        B: OperandCast + Copy,
        Self: MovEmitter<A, B>,
    {
        let dst_op = *dst.as_operand();
        let src_op = *src.as_operand();
        let size = if dst_op.is_reg_type_of(RegType::Gp64) {
            8
        } else if dst_op.is_reg_type_of(RegType::Gp32) {
            4
        } else {
            self.buffer
                .record_error(crate::AsmError::X86(X86Error::InvalidOperand {
                    operand_index: 0,
                    reason: "patchable_mov requires a Gp32 or Gp64 destination",
                }));
            return DataLabel::invalid(8, DataEncoding::Raw);
        };

        if !src_op.is_imm() {
            self.buffer
                .record_error(crate::AsmError::X86(X86Error::InvalidOperand {
                    operand_index: 1,
                    reason: "patchable_mov requires an immediate source",
                }));
            return DataLabel::invalid(size, DataEncoding::Raw);
        }

        let offset = self.buffer.cur_offset();
        let previous_error = self.buffer.error().cloned();
        self.long();
        MovEmitter::mov(self, dst, src);
        if self.buffer.error().cloned() != previous_error
            || self.buffer.cur_offset() < offset + CodeOffset::from(size)
        {
            return DataLabel::invalid(size, DataEncoding::Raw);
        }

        // Long mov-imm ends with the `size`-byte immediate.
        let offset = self.buffer.cur_offset() - CodeOffset::from(size);
        DataLabel::new(offset, size, DataEncoding::Raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondCode {
    O = 0x0,
    NO = 0x1,
    C = 0x2,
    NC = 0x3,
    Z = 0x4,
    NZ = 0x5,
    BE = 0x6,
    A = 0x7,
    S = 0x8,
    NS = 0x9,
    P = 0xa,

    NP = 0xb,
    L = 0xc,
    GE = 0xd,
    LE = 0xe,
    G = 0xf,
}

impl CondCode {
    pub const B: Self = Self::C;
    pub const NAE: Self = Self::C;
    pub const AE: Self = Self::NC;
    pub const NB: Self = Self::NC;
    pub const E: Self = Self::Z;
    pub const NE: Self = Self::NZ;
    pub const NA: Self = Self::BE;
    pub const NBE: Self = Self::A;
    pub const PO: Self = Self::NP;
    pub const NGE: Self = Self::L;
    pub const NL: Self = Self::GE;
    pub const NG: Self = Self::LE;
    pub const NLE: Self = Self::G;
    pub const PE: Self = Self::P;

    pub const fn code(self) -> u8 {
        self as u8
    }

    pub fn invert(self) -> Self {
        match self {
            Self::O => Self::NO,
            Self::NO => Self::O,
            Self::C => Self::NC,
            Self::NC => Self::C,
            Self::Z => Self::NZ,
            Self::NZ => Self::Z,
            Self::BE => Self::A,
            Self::A => Self::BE,
            Self::S => Self::NS,
            Self::NS => Self::S,
            Self::P => Self::NP,
            Self::NP => Self::P,
            Self::L => Self::GE,
            Self::GE => Self::L,
            Self::LE => Self::G,
            Self::G => Self::LE,
        }
    }
}
