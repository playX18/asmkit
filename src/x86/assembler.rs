#![allow(dead_code)]
use super::emit::{self, PendingPrefixes};
use super::emitter::{CallEmitter, JmpEmitter, MovEmitter};
use super::operands::*;
use crate::{
    X86Error,
    core::{
        arch_traits::Arch,
        buffer::{CodeBuffer, CodeOffset, ConstantData, LabelUse},
        globals::InstOptions,
        operand::*,
        patch::{PatchableBlock, PatchableSite},
        target::Environment,
    },
};
use super::instdb::InstId;

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
/// bits 33..=35 mask id, bit 36 zeroing mask).
const OPC_LOCK: u64 = 0x2000000000;
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

    #[cfg(test)]
    fn last_error(&self) -> Option<X86Error> {
        match self.buffer.error() {
            Some(crate::AsmError::X86(error)) => Some(error.clone()),
            _ => None,
        }
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

    /// Reserve a nop-filled island for later custom rewriting.
    pub fn reserve_patch_block(
        &mut self,
        size: CodeOffset,
        align: CodeOffset,
    ) -> Result<PatchableBlock, crate::AsmError> {
        self.buffer.reserve_patch_block(size, align)
    }

    pub fn patchable_jmp(&mut self, label: Label) -> PatchableSite {
        self.long();
        self.jmp(label);
        let offset = self
            .buffer
            .cur_offset()
            .saturating_sub(LabelUse::X86JmpRel32.patch_size() as u32);
        let _ = self
            .buffer
            .record_label_patch_site(offset, label, LabelUse::X86JmpRel32);
        // SAFETY: long jmp/call emits a rel32 displacement at `offset`.
        unsafe { PatchableSite::new(offset, LabelUse::X86JmpRel32, 0) }
    }

    pub fn patchable_call(&mut self, label: Label) -> PatchableSite {
        self.long();
        self.call(label);
        let offset = self
            .buffer
            .cur_offset()
            .saturating_sub(LabelUse::X86JmpRel32.patch_size() as u32);
        let _ = self
            .buffer
            .record_label_patch_site(offset, label, LabelUse::X86JmpRel32);
        // SAFETY: long jmp/call emits a rel32 displacement at `offset`.
        unsafe { PatchableSite::new(offset, LabelUse::X86JmpRel32, 0) }
    }

    /// Patchable conditional jump (forced rel32).
    pub fn patchable_jcc(&mut self, cc: CondCode, label: Label) -> PatchableSite {
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
        self.long();
        self.emit_n(JCC[cc.code() as usize] as u32, &[label.as_operand()]);
        let offset = self
            .buffer
            .cur_offset()
            .saturating_sub(LabelUse::X86JmpRel32.patch_size() as u32);
        let _ = self
            .buffer
            .record_label_patch_site(offset, label, LabelUse::X86JmpRel32);
        // SAFETY: long jcc emits a rel32 displacement at `offset`.
        unsafe { PatchableSite::new(offset, LabelUse::X86JmpRel32, 0) }
    }

    pub fn patchable_mov<A, B>(&mut self, dst: A, src: B) -> PatchableBlock
    where
        A: OperandCast + Copy,
        B: OperandCast + Copy,
        Self: MovEmitter<A, B>,
    {
        let arch = self.buffer.env().arch();
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
            // SAFETY: poisoned handle; apply will fail bounds checks.
            return unsafe { PatchableBlock::new(u32::MAX, 1, arch) };
        };

        if !src_op.is_imm() {
            self.buffer
                .record_error(crate::AsmError::X86(X86Error::InvalidOperand {
                    operand_index: 1,
                    reason: "patchable_mov requires an immediate source",
                }));
            return unsafe { PatchableBlock::new(u32::MAX, size, arch) };
        }

        let offset = self.buffer.cur_offset();
        let previous_error = self.buffer.error().cloned();
        self.long();
        MovEmitter::mov(self, dst, src);
        if self.buffer.error().cloned() != previous_error
            || self.buffer.cur_offset() < offset + size
        {
            return unsafe { PatchableBlock::new(u32::MAX, size, arch) };
        }

        let offset = self.buffer.cur_offset() - size;
        let _ = self.buffer.record_patch_block(offset, size, 1);
        // SAFETY: long mov-imm places a `size`-byte immediate at `offset`.
        unsafe { PatchableBlock::new(offset, size, arch) }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::builder::Builder;
    use crate::core::inst::Inst;
    use crate::x86::instdb::InstId;
    use crate::x86::operands::regs::*;

    /// Assembles via `emit_n`, asserting no error, and finalizes label fixups.
    fn asm(f: impl FnOnce(&mut Assembler)) -> std::vec::Vec<u8> {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        {
            let mut a = Assembler::new(&mut buf);
            f(&mut a);
            assert!(a.last_error().is_none(), "{:?}", a.last_error());
        }
        buf.finish().unwrap().data().to_vec()
    }

    #[test]
    fn emit_n_integer_forms() {
        // mov rax, 1 — sign-extended C7 /0 form (AsmJit default, no long_form).
        assert_eq!(
            asm(|a| a.emit_n(InstId::Mov as u32, &[RAX.as_operand(), imm(1).as_operand()])),
            [0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]
        );
        // mov rax, rbx.
        assert_eq!(
            asm(|a| a.emit_n(InstId::Mov as u32, &[RAX.as_operand(), RBX.as_operand()])),
            [0x48, 0x89, 0xD8]
        );
        // add rax, rbx.
        assert_eq!(
            asm(|a| a.emit_n(InstId::Add as u32, &[RAX.as_operand(), RBX.as_operand()])),
            [0x48, 0x01, 0xD8]
        );
        // push rax / pop rax.
        assert_eq!(
            asm(|a| a.emit_n(InstId::Push as u32, &[RAX.as_operand()])),
            [0x50]
        );
        assert_eq!(
            asm(|a| a.emit_n(InstId::Pop as u32, &[RAX.as_operand()])),
            [0x58]
        );
        // cmovz rax, rbx.
        assert_eq!(
            asm(|a| a.emit_n(InstId::Cmovz as u32, &[RAX.as_operand(), RBX.as_operand()])),
            [0x48, 0x0F, 0x44, 0xC3]
        );
        // ret / syscall.
        assert_eq!(asm(|a| a.emit_n(InstId::Ret as u32, &[])), [0xC3]);
        assert_eq!(asm(|a| a.emit_n(InstId::Syscall as u32, &[])), [0x0F, 0x05]);
        // mov rax, 0x123456789 — 64-bit immediate uses the B8 (movabs) form.
        assert_eq!(
            asm(|a| a.emit_n(
                InstId::Mov as u32,
                &[RAX.as_operand(), imm(0x1_2345_6789i64).as_operand()]
            )),
            [0x48, 0xB8, 0x89, 0x67, 0x45, 0x23, 0x01, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn emit_n_invalid_sets_last_error() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let mut a = Assembler::new(&mut buf);
        // add rax, xmm0 — no signature matches; nothing is emitted.
        a.emit_n(InstId::Add as u32, &[RAX.as_operand(), XMM0.as_operand()]);
        assert!(matches!(
            a.last_error(),
            Some(X86Error::InvalidInstruction { .. })
        ));
        assert!(a.buffer.data().is_empty());
        // Unknown instruction id.
        a.buffer.clear();
        a.emit_n(u32::MAX, &[]);
        assert!(a.last_error().is_some());
    }

    #[test]
    fn raw_invalid_symbol_id_is_rejected() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let mut a = Assembler::new(&mut buf);
        let mut operand = Operand::new();
        operand.set_signature(OperandSignature::from(0x001A_0012));

        assert_eq!(
            a.try_emit_n(727u32, &[&operand]),
            Err(crate::AsmError::X86(X86Error::InvalidOperand {
                operand_index: 0,
                reason: "symbol is not declared in this buffer",
            }))
        );
        assert!(a.buffer.data().is_empty());

        a.emit_n(727u32, &[&operand]);
        assert_eq!(
            a.buffer.error(),
            Some(&crate::AsmError::X86(X86Error::InvalidOperand {
                operand_index: 0,
                reason: "symbol is not declared in this buffer",
            }))
        );
        assert!(a.buffer.data().is_empty());
    }

    #[test]
    fn emit_n_rejects_more_than_six_operands() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let mut a = Assembler::new(&mut buf);
        let ops = [RAX.as_operand(); 7];

        a.emit_n(InstId::Ret as u32, &ops);

        assert!(a.last_error().is_some());
        assert!(a.buffer.data().is_empty());
    }

    #[test]
    fn patchable_mov_emits_once_and_rejects_unsupported_operands() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        {
            let mut a = Assembler::new(&mut buf);
            a.patchable_mov(EAX, imm(42));
            assert_eq!(a.buffer.data(), &[0xB8, 42, 0, 0, 0]);
            assert!(a.last_error().is_none());

            a.patchable_mov(RAX, imm(42));
            assert_eq!(
                a.buffer.data(),
                &[0xB8, 42, 0, 0, 0, 0x48, 0xB8, 42, 0, 0, 0, 0, 0, 0, 0]
            );

            a.patchable_mov(RAX, RBX);
            assert!(matches!(
                a.last_error(),
                Some(X86Error::InvalidOperand {
                    operand_index: 1,
                    ..
                })
            ));
            assert_eq!(
                a.buffer.data(),
                &[0xB8, 42, 0, 0, 0, 0x48, 0xB8, 42, 0, 0, 0, 0, 0, 0, 0]
            );
        }

        assert!(matches!(
            buf.finish_patched(),
            Err(crate::AsmError::X86(X86Error::InvalidOperand {
                operand_index: 1,
                ..
            }))
        ));
    }

    #[test]
    fn emit_n_memory_forms() {
        // mov rax, [rip + 0x1234].
        assert_eq!(
            asm(|a| a.emit_n(
                InstId::Mov as u32,
                &[RAX.as_operand(), qword_ptr_rip(0x1234).as_operand()]
            )),
            [0x48, 0x8B, 0x05, 0x34, 0x12, 0x00, 0x00]
        );
        // mov [rip + label], eax — label bound right after the instruction.
        assert_eq!(
            asm(|a| {
                let label = a.get_label();
                a.emit_n(
                    InstId::Mov as u32,
                    &[dword_ptr_label(label, 0).as_operand(), EAX.as_operand()],
                );
                a.bind_label(label);
                a.emit_n(InstId::Ret as u32, &[]);
            }),
            [0x89, 0x05, 0x00, 0x00, 0x00, 0x00, 0xC3]
        );
        // fs prefix from the prefix setter applies to the mem operand.
        assert_eq!(
            asm(|a| {
                a.fs();
                a.emit_n(
                    InstId::Mov as u32,
                    &[RAX.as_operand(), qword_ptr_u64(0x40).as_operand()],
                );
            }),
            [0x64, 0x48, 0x8B, 0x04, 0x25, 0x40, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn emit_n_branch_forms() {
        // Backward jmp to a bound label uses rel8 by default.
        assert_eq!(
            asm(|a| {
                let label = a.get_label();
                a.bind_label(label);
                a.emit_n(
                    InstId::Jmp as u32,
                    &[Label::from_id(label.id()).as_operand()],
                );
            }),
            [0xEB, 0xFE]
        );
        // Forward jmp to an unbound label is relaxed at finalization.
        assert_eq!(
            asm(|a| {
                let label = a.get_label();
                a.emit_n(
                    InstId::Jmp as u32,
                    &[Label::from_id(label.id()).as_operand()],
                );
                a.bind_label(label);
            }),
            [0xEB, 0x00]
        );
        // call rel32 to an unbound label, bound right after.
        assert_eq!(
            asm(|a| {
                let label = a.get_label();
                a.emit_n(
                    InstId::Call as u32,
                    &[Label::from_id(label.id()).as_operand()],
                );
                a.bind_label(label);
            }),
            [0xE8, 0x00, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn patchable_branches_keep_the_near_form() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let target = buf.get_label();
        let site = {
            let mut asm = Assembler::new(&mut buf);
            asm.patchable_jmp(target)
        };
        buf.bind_label(target);

        let code = buf.finish_patched().unwrap();
        assert_eq!(code.data(), &[0xE9, 0, 0, 0, 0]);
        assert_eq!(site.offset(), 1);
        let catalog_site = code
            .patch_catalog()
            .sites()
            .iter()
            .find(|s| s.offset == site.offset())
            .unwrap();
        assert_eq!(catalog_site.current_target, 5);
    }

    #[test]
    fn patchable_jcc_and_mov_can_be_rewritten_offline() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let (jcc, imm_block, alt) = {
            let mut asm = Assembler::new(&mut buf);
            let target = asm.get_label();
            let alt = asm.get_label();
            let imm_block = asm.patchable_mov(EAX, imm(1i32));
            let jcc = asm.patchable_jcc(CondCode::Z, target);
            asm.bind_label(target);
            asm.emit_n(InstId::Ret as u32, &[]);
            let alt_off = {
                asm.bind_label(alt);
                asm.emit_n(InstId::Ret as u32, &[]);
                asm.label_offset(alt)
            };
            (jcc, imm_block, alt_off)
        };

        let code = buf.finish_patched().unwrap();
        let mut bytes = code.data().to_vec();
        unsafe {
            imm_block.repatch_u32(&mut bytes, 0x99).unwrap();
            jcc.retarget(&mut bytes, alt).unwrap();
        }
        assert_eq!(&bytes[imm_block.offset() as usize..][..4], &0x99u32.to_le_bytes());
    }

    #[test]
    fn emit_n_vex_evex_forms() {
        // vaddps ymm1, ymm2, ymm3 — AsmJit golden "C5EC58CB".
        assert_eq!(
            asm(|a| a.emit_n(
                InstId::Vaddps as u32,
                &[YMM1.as_operand(), YMM2.as_operand(), YMM3.as_operand()]
            )),
            [0xC5, 0xEC, 0x58, 0xCB]
        );
        // vmovdqu ymm0, ymm1.
        assert_eq!(
            asm(|a| a.emit_n(
                InstId::Vmovdqu as u32,
                &[YMM0.as_operand(), YMM1.as_operand()]
            )),
            [0xC5, 0xFE, 0x6F, 0xC1]
        );
        // vaddpd zmm1, zmm2, zmm3 (unmasked EVEX).
        assert_eq!(
            asm(|a| a.emit_n(
                InstId::Vaddpd as u32,
                &[ZMM1.as_operand(), ZMM2.as_operand(), ZMM3.as_operand()]
            )),
            [0x62, 0xF1, 0xED, 0x48, 0x58, 0xCB]
        );
        // {k1} mask from the prefix setter: EVEX P2 aaa=001.
        assert_eq!(
            asm(|a| {
                a.k(K1);
                a.emit_n(
                    InstId::Vaddpd as u32,
                    &[ZMM1.as_operand(), ZMM2.as_operand(), ZMM3.as_operand()],
                );
            }),
            [0x62, 0xF1, 0xED, 0x49, 0x58, 0xCB]
        );
    }

    #[test]
    fn emitter_traits_match_emit_n() {
        // The generated emitter traits (src/x86/emitter.rs) must produce the
        // same bytes as the direct `emit_n` calls above.
        use crate::x86::emitter::{AddEmitter, JmpEmitter, MovEmitter, VaddpsEmitter};

        // mov rax, 1.
        assert_eq!(
            asm(|a| MovEmitter::mov(a, RAX, imm(1))),
            [0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]
        );
        // mov rax, rbx.
        assert_eq!(asm(|a| MovEmitter::mov(a, RAX, RBX)), [0x48, 0x89, 0xD8]);
        // add rax, rbx.
        assert_eq!(asm(|a| AddEmitter::add(a, RAX, RBX)), [0x48, 0x01, 0xD8]);
        // vaddps ymm1, ymm2, ymm3.
        assert_eq!(
            asm(|a| VaddpsEmitter::vaddps(a, YMM1, YMM2, YMM3)),
            [0xC5, 0xEC, 0x58, 0xCB]
        );
        // Backward jmp to a bound label through the trait.
        assert_eq!(
            asm(|a| {
                let label = a.get_label();
                a.bind_label(label);
                JmpEmitter::jmp(a, label);
            }),
            [0xEB, 0xFE]
        );
    }

    #[test]
    fn emitter_typed_call_sites() {
        // Sized register constants and integer literals must work directly,
        // producing the same bytes as the abstract forms above.
        use crate::x86::emitter::{AddEmitter, MovEmitter, PaddwEmitter, VaddpsEmitter};

        // mov rax, 42 (integer literal via Into<Imm>).
        assert_eq!(
            asm(|a| MovEmitter::mov(a, RAX, 42)),
            [0x48, 0xC7, 0xC0, 0x2A, 0x00, 0x00, 0x00]
        );
        // mov eax, 42 (no REX prefix).
        assert_eq!(
            asm(|a| MovEmitter::mov(a, EAX, 42)),
            [0xB8, 0x2A, 0x00, 0x00, 0x00]
        );
        // add rax, rbx.
        assert_eq!(asm(|a| AddEmitter::add(a, RAX, RBX)), [0x48, 0x01, 0xD8]);
        // paddw xmm0, xmm1 (legacy SSE form).
        assert_eq!(
            asm(|a| PaddwEmitter::paddw(a, XMM0, XMM1)),
            [0x66, 0x0F, 0xFD, 0xC1]
        );
        // vaddps ymm1, ymm2, ymm3 (VEX form).
        assert_eq!(
            asm(|a| VaddpsEmitter::vaddps(a, YMM1, YMM2, YMM3)),
            [0xC5, 0xEC, 0x58, 0xCB]
        );
    }

    #[test]
    fn emit_n_prefix_forms() {
        // rep movsq (explicit implicit-mem form): F3 48 A5.
        assert_eq!(
            asm(|a| {
                a.rep();
                a.emit_n(
                    InstId::Movs as u32,
                    &[
                        qword_ptr(RDI, 0).as_operand(),
                        qword_ptr(RSI, 0).as_operand(),
                    ],
                );
            }),
            [0xF3, 0x48, 0xA5]
        );
        // lock add dword ptr [rbx], eax: F0 01 03.
        assert_eq!(
            asm(|a| {
                a.lock();
                a.emit_n(
                    InstId::Add as u32,
                    &[dword_ptr(RBX, 0).as_operand(), EAX.as_operand()],
                );
            }),
            [0xF0, 0x01, 0x03]
        );
    }

    #[test]
    fn conflicting_prefix_setters_poison_without_emitting() {
        let cases: &[fn(&mut Assembler<'_>)] = &[
            |a| {
                a.fs().gs();
            },
            |a| {
                a.k(K1).k(K2);
            },
            |a| {
                a.rd_sae().ru_sae();
            },
            |a| {
                a.seg(sreg(7));
            },
            |a| {
                a.k(k(8));
            },
            |a| {
                a.k(K0);
            },
        ];

        for set_prefixes in cases {
            let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
            let mut assembler = Assembler::new(&mut buffer);
            set_prefixes(&mut assembler);
            assembler.emit_n(InstId::Ret as u32, &[]);
            assert!(assembler.buffer.error().is_some());
            assert!(assembler.buffer.data().is_empty());
        }
    }

    #[test]
    fn emit_n_builder_replay_matches_direct() {
        fn direct() -> std::vec::Vec<u8> {
            asm(|a| {
                let done = a.get_label();
                a.emit_n(InstId::Mov as u32, &[RAX.as_operand(), imm(1).as_operand()]);
                a.emit_n(InstId::Add as u32, &[RAX.as_operand(), RBX.as_operand()]);
                a.emit_n(
                    InstId::Jmp as u32,
                    &[Label::from_id(done.id()).as_operand()],
                );
                a.emit_n(InstId::Ret as u32, &[]);
                a.bind_label(done);
                a.emit_n(InstId::Ret as u32, &[]);
            })
        }

        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let mut builder = Builder::new();
        let done = buf.get_label();
        builder
            .push_inst(Inst::with_operands(
                InstId::Mov as u32,
                &[*RAX.as_operand(), *imm(1).as_operand()],
            ))
            .unwrap();
        builder
            .push_inst(Inst::with_operands(
                InstId::Add as u32,
                &[*RAX.as_operand(), *RBX.as_operand()],
            ))
            .unwrap();
        builder
            .push_inst(Inst::with_operands(
                InstId::Jmp as u32,
                &[*Label::from_id(done.id()).as_operand()],
            ))
            .unwrap();
        builder
            .push_inst(Inst::with_operands(InstId::Ret as u32, &[]))
            .unwrap();
        builder.push_label(done);
        builder
            .push_inst(Inst::with_operands(InstId::Ret as u32, &[]))
            .unwrap();
        {
            let mut a = Assembler::new(&mut buf);
            builder.emit_into(&mut a).unwrap();
            assert!(a.last_error().is_none(), "{:?}", a.last_error());
        }
        assert_eq!(buf.finish().unwrap().data().to_vec(), direct());
    }

    #[test]
    fn builder_replays_options_and_mask_register() {
        let direct = asm(|a| {
            a.k(K1).z();
            a.emit_n(
                InstId::Vaddpd as u32,
                &[ZMM1.as_operand(), ZMM2.as_operand(), ZMM3.as_operand()],
            );
        });

        let mut inst = Inst::with_arch_operands(
            Arch::X64,
            InstId::Vaddpd as u32,
            &[*ZMM1.as_operand(), *ZMM2.as_operand(), *ZMM3.as_operand()],
        )
        .unwrap();
        inst.set_options(InstOptions::X86_ZMASK);
        inst.set_extra_reg(*K1.as_operand());
        let mut builder = Builder::for_arch(Arch::X64);
        builder.push_inst(inst).unwrap();

        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        builder.emit_into(&mut Assembler::new(&mut buf)).unwrap();
        assert_eq!(buf.finish().unwrap().data(), direct);
    }

    /// Assembles in 32-bit mode via `emit_n`, asserting no error.
    fn asm32(f: impl FnOnce(&mut Assembler)) -> std::vec::Vec<u8> {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X86));
        {
            let mut a = Assembler::new(&mut buf);
            f(&mut a);
            assert!(a.last_error().is_none(), "{:?}", a.last_error());
        }
        buf.finish().unwrap().data().to_vec()
    }

    /// Assembles in 32-bit mode, expecting an error.
    fn asm32_err(f: impl FnOnce(&mut Assembler)) -> X86Error {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X86));
        let mut a = Assembler::new(&mut buf);
        f(&mut a);
        a.last_error().expect("expected an emit error")
    }

    /// Assembles in 64-bit mode, expecting an error.
    fn asm64_err(f: impl FnOnce(&mut Assembler)) -> X86Error {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let mut a = Assembler::new(&mut buf);
        f(&mut a);
        a.last_error().expect("expected an emit error")
    }

    #[test]
    fn emit_n_32bit_gp_forms() {
        use crate::x86::emitter::{AddEmitter, MovEmitter};

        // mov eax, ebx — no REX in 32-bit mode.
        assert_eq!(asm32(|a| MovEmitter::mov(a, EAX, EBX)), [0x89, 0xD8]);
        // mov ax, bx / mov al, bl.
        assert_eq!(asm32(|a| MovEmitter::mov(a, AX, BX)), [0x66, 0x89, 0xD8]);
        assert_eq!(asm32(|a| MovEmitter::mov(a, AL, BL)), [0x88, 0xD8]);
        // add ecx, 1 (imm8 form); add eax, 0x12345678 (accumulator short form).
        assert_eq!(
            asm32(|a| a.emit_n(InstId::Add as u32, &[ECX.as_operand(), imm(1).as_operand()])),
            [0x83, 0xC1, 0x01]
        );
        assert_eq!(
            asm32(|a| AddEmitter::add(a, EAX, 0x1234_5678i32)),
            [0x05, 0x78, 0x56, 0x34, 0x12]
        );
        // mov eax, 0x12345678 / mov ax, 0x1234 / mov cl, 0x12.
        assert_eq!(
            asm32(|a| MovEmitter::mov(a, EAX, 0x1234_5678i32)),
            [0xB8, 0x78, 0x56, 0x34, 0x12]
        );
        assert_eq!(
            asm32(|a| MovEmitter::mov(a, AX, 0x1234)),
            [0x66, 0xB8, 0x34, 0x12]
        );
        assert_eq!(asm32(|a| MovEmitter::mov(a, CL, 0x12)), [0xB1, 0x12]);
    }

    #[test]
    fn emit_n_32bit_inc_dec_short_forms() {
        use crate::x86::emitter::{DecEmitter, IncEmitter};

        // INC/DEC r16|r32 short forms exist only in 32-bit mode.
        assert_eq!(asm32(|a| IncEmitter::inc(a, EAX)), [0x40]);
        assert_eq!(asm32(|a| IncEmitter::inc(a, ECX)), [0x41]);
        assert_eq!(asm32(|a| IncEmitter::inc(a, AX)), [0x66, 0x40]);
        assert_eq!(asm32(|a| DecEmitter::dec(a, EDX)), [0x4A]);
        assert_eq!(asm32(|a| DecEmitter::dec(a, DX)), [0x66, 0x4A]);
        // 8-bit and memory forms are shared with 64-bit.
        assert_eq!(asm32(|a| IncEmitter::inc(a, AL)), [0xFE, 0xC0]);
        assert_eq!(
            asm32(|a| IncEmitter::inc(a, dword_ptr(ECX, 0))),
            [0xFF, 0x01]
        );
        // In 64-bit mode the same instructions use the FF /0|/1 forms.
        assert_eq!(
            asm(|a| a.emit_n(InstId::Inc as u32, &[EAX.as_operand()])),
            [0xFF, 0xC0]
        );
    }

    #[test]
    fn emit_n_32bit_push_pop() {
        use crate::x86::emitter::{PopEmitter, PushEmitter};

        assert_eq!(asm32(|a| PushEmitter::push(a, EAX)), [0x50]);
        assert_eq!(asm32(|a| PushEmitter::push(a, AX)), [0x66, 0x50]);
        assert_eq!(asm32(|a| PopEmitter::pop(a, ECX)), [0x59]);
        assert_eq!(
            asm32(|a| PushEmitter::push(a, 0x1234_5678i32)),
            [0x68, 0x78, 0x56, 0x34, 0x12]
        );
        assert_eq!(
            asm32(|a| PushEmitter::push(a, dword_ptr(ECX, 0))),
            [0xFF, 0x31]
        );
        assert_eq!(
            asm32(|a| PushEmitter::push(a, word_ptr(ECX, 0))),
            [0x66, 0xFF, 0x31]
        );
        // push/pop m64 is not encodable in 32-bit mode.
        asm32_err(|a| PushEmitter::push(a, qword_ptr(ECX, 0)));
        asm32_err(|a| PopEmitter::pop(a, qword_ptr(ECX, 0)));
    }

    #[test]
    fn emit_n_32bit_far_pointer_forms() {
        // lcall/ljmp imm16, imm32 — 32-bit only.
        assert_eq!(
            asm32(|a| a.emit_n(
                InstId::Lcall as u32,
                &[imm(0x1234).as_operand(), imm(0x1234_5678).as_operand()]
            )),
            [0x9A, 0x78, 0x56, 0x34, 0x12, 0x34, 0x12]
        );
        assert_eq!(
            asm32(|a| a.emit_n(
                InstId::Ljmp as u32,
                &[imm(0x10).as_operand(), imm(0x20).as_operand()]
            )),
            [0xEA, 0x20, 0x00, 0x00, 0x00, 0x10, 0x00]
        );
        // Selector above 0xFFFF is rejected.
        asm32_err(|a| {
            a.emit_n(
                InstId::Lcall as u32,
                &[imm(0x1_0000).as_operand(), imm(0).as_operand()],
            )
        });
        // The imm,imm form is rejected in 64-bit mode.
        asm64_err(|a| {
            a.emit_n(
                InstId::Lcall as u32,
                &[imm(0x1234).as_operand(), imm(0x1234_5678).as_operand()],
            )
        });
        // lcall fword [ecx] — m16:32.
        assert_eq!(
            asm32(|a| a.emit_n(InstId::Lcall as u32, &[fword_ptr(ECX, 0).as_operand()])),
            [0xFF, 0x19]
        );
    }

    #[test]
    fn emit_n_32bit_movabs_and_xchg() {
        use crate::x86::emitter::{MovEmitter, XchgEmitter};

        // mov eax, [abs] uses the moffs A1 form in 32-bit mode.
        assert_eq!(
            asm32(|a| MovEmitter::mov(a, EAX, dword_ptr_u64(0x1234_5678))),
            [0xA1, 0x78, 0x56, 0x34, 0x12]
        );
        // mov [abs], eax — moffs A3 form.
        assert_eq!(
            asm32(|a| MovEmitter::mov(a, dword_ptr_u64(0x1234_5678), EAX)),
            [0xA3, 0x78, 0x56, 0x34, 0x12]
        );
        // xchg eax, eax is encoded as 90 in 32-bit mode (generic path in 64-bit).
        assert_eq!(asm32(|a| XchgEmitter::xchg(a, EAX, EAX)), [0x90]);
        assert_eq!(
            asm(|a| a.emit_n(InstId::Xchg as u32, &[EAX.as_operand(), EAX.as_operand()])),
            [0x87, 0xC0]
        );
    }

    #[test]
    fn emit_n_32bit_creg_lock_extension() {
        // mov eax, cr8 / mov cr8, eax use the LOCK prefix in 32-bit mode (AMD ext).
        assert_eq!(
            asm32(|a| a.emit_n(InstId::Mov as u32, &[EAX.as_operand(), CR8.as_operand()])),
            [0xF0, 0x0F, 0x20, 0xC0]
        );
        assert_eq!(
            asm32(|a| a.emit_n(InstId::Mov as u32, &[CR8.as_operand(), EAX.as_operand()])),
            [0xF0, 0x0F, 0x22, 0xC0]
        );
        // cr0 needs no LOCK.
        assert_eq!(
            asm32(|a| a.emit_n(InstId::Mov as u32, &[EAX.as_operand(), CR0.as_operand()])),
            [0x0F, 0x20, 0xC0]
        );
    }

    #[test]
    fn emit_n_32bit_mode_gating() {
        // 64-bit registers are not available in 32-bit mode.
        asm32_err(|a| a.emit_n(InstId::Mov as u32, &[RAX.as_operand(), RBX.as_operand()]));
        asm32_err(|a| a.emit_n(InstId::Push as u32, &[RAX.as_operand()]));
        // Register ids above 7 are not available in 32-bit mode.
        asm32_err(|a| a.emit_n(InstId::Mov as u32, &[EAX.as_operand(), R8D.as_operand()]));
        asm32_err(|a| {
            a.emit_n(
                InstId::Paddw as u32,
                &[XMM0.as_operand(), XMM8.as_operand()],
            )
        });
        // X64-only instructions are rejected in 32-bit mode.
        asm32_err(|a| a.emit_n(InstId::Syscall as u32, &[]));
        asm32_err(|a| a.emit_n(InstId::Movsxd as u32, &[EAX.as_operand(), EBX.as_operand()]));
        // 64-bit addressing registers are rejected in 32-bit mode.
        asm32_err(|a| {
            a.emit_n(
                InstId::Mov as u32,
                &[EAX.as_operand(), dword_ptr(RBX, 0).as_operand()],
            )
        });
        // A 64-bit absolute address is not encodable in 32-bit mode.
        asm32_err(|a| {
            a.emit_n(
                InstId::Mov as u32,
                &[EAX.as_operand(), dword_ptr_u64(0x1_0000_0000).as_operand()],
            )
        });
    }

    #[test]
    fn emit_n_32bit_string_ops() {
        // movsd with implicit [edi]/[esi] operands: no 67h in 32-bit mode.
        assert_eq!(
            asm32(|a| a.emit_n(
                InstId::Movs as u32,
                &[
                    dword_ptr(EDI, 0).as_operand(),
                    dword_ptr(ESI, 0).as_operand()
                ]
            )),
            [0xA5]
        );
        // jecxz with an explicit cx: 67h in 32-bit mode (immediate raw-disp8 form).
        assert_eq!(
            asm32(|a| a.emit_n(
                InstId::Jecxz as u32,
                &[CX.as_operand(), imm(-2).as_operand()],
            )),
            [0x67, 0xE3, 0xFE]
        );
    }
}
