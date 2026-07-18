#![allow(dead_code, clippy::all)]
use super::emit::{self, PendingPrefixes};
use super::emitter::{CallEmitter, JmpEmitter, MovEmitter};
use super::operands::*;
use crate::{
    X86Error,
    core::{
        buffer::{CodeBuffer, CodeOffset, ConstantData, LabelUse},
        globals::InstOptions,
        operand::*,
        patch::{PatchBlockId, PatchSiteId},
    },
};

/// X86/X64 Assembler implementation.
pub struct Assembler<'a> {
    pub buffer: &'a mut CodeBuffer,
    flags: u64,
    extra_reg: Reg,
    last_error: Option<X86Error>,
}

const RC_RN: u64 = 0x0000000;
const RC_RD: u64 = 0x0800000;
const RC_RU: u64 = 0x1000000;
const RC_RZ: u64 = 0x1800000;
const SEG_MASK: u64 = 0xe0000000;
const LONG: u64 = 0x100000000;

/// Bit 37: LOCK prefix (bits 23/24 are the rounding-mode RC bits, bits 20/21
/// REP/REPNE, bit 26 SAE/ER enable, bits 29..=31 segment, bit 32 long-form,
/// bits 33..=35 mask id, bit 36 zeroing mask).
const OPC_LOCK: u64 = 0x2000000000;
/// Bit 36: AVX-512 zeroing mask `{z}` (bits 33..=35 carry the mask register id).
const OPC_Z: u64 = 0x1000000000;

impl crate::core::builder::InstSink for Assembler<'_> {
    fn emit_inst(&mut self, inst: &crate::core::inst::Inst) {
        let ops = inst.operands();
        let mut refs: smallvec::SmallVec<[&Operand; 6]> = smallvec::SmallVec::new();
        refs.extend(ops.iter());
        self.emit_n(inst.id, &refs);
    }

    fn bind_label(&mut self, label: Label) {
        Assembler::bind_label(self, label);
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
        if flags & 0x4000000 != 0 {
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
    /// path). Pending prefixes set by the prefix setters apply. On failure,
    /// `last_error` is set and nothing more is emitted.
    pub fn emit_n(&mut self, id: impl Into<u32>, ops: &[&Operand]) {
        let prefixes = self.take_pending_prefixes();
        if let Err(err) = emit::emit_n(self.buffer, id.into(), ops, prefixes) {
            self.last_error = Some(err);
        }
    }

    pub fn new(buf: &'a mut CodeBuffer) -> Self {
        Self {
            buffer: buf,
            extra_reg: Reg::new(),
            flags: 0,
            last_error: None,
        }
    }

    /// Get the last error that occurred during assembly.
    pub fn last_error(&self) -> Option<X86Error> {
        self.last_error.clone()
    }

    /// Clear the last error.
    pub fn clear_error(&mut self) {
        self.last_error = None;
    }

    pub fn sae(&mut self) -> &mut Self {
        self.flags |= 0x4000000;
        self
    }

    pub fn rn_sae(&mut self) -> &mut Self {
        self.flags |= 0x4000000 | RC_RN;
        self
    }

    pub fn rd_sae(&mut self) -> &mut Self {
        self.flags |= 0x4000000 | RC_RD;
        self
    }
    pub fn ru_sae(&mut self) -> &mut Self {
        self.flags |= 0x4000000 | RC_RU;
        self
    }

    pub fn rz_sae(&mut self) -> &mut Self {
        self.flags |= 0x4000000 | RC_RZ;
        self
    }

    pub fn seg(&mut self, sreg: SReg) -> &mut Self {
        self.flags |= ((sreg.id() & 0x7) as u64) << 29;
        self
    }

    pub fn fs(&mut self) -> &mut Self {
        self.seg(FS)
    }

    pub fn gs(&mut self) -> &mut Self {
        self.seg(GS)
    }

    pub fn k(&mut self, k: KReg) -> &mut Self {
        self.flags |= ((k.id() & 0x7) as u64) << 33;

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
        self.buffer.bind_label(label);
    }

    pub fn add_constant(&mut self, c: impl Into<ConstantData>) -> Label {
        let c = self.buffer.add_constant(c);
        self.buffer.get_label_for_constant(c)
    }

    pub fn label_offset(&self, label: Label) -> CodeOffset {
        self.buffer.label_offset(label)
    }

    pub fn patchable_jmp(&mut self, label: Label) -> PatchSiteId {
        self.jmp(label);
        let offset = self
            .buffer
            .cur_offset()
            .saturating_sub(LabelUse::X86JmpRel32.patch_size() as u32);
        self.buffer
            .record_label_patch_site(offset, label, LabelUse::X86JmpRel32)
    }

    pub fn patchable_call(&mut self, label: Label) -> PatchSiteId {
        self.call(label);
        let offset = self
            .buffer
            .cur_offset()
            .saturating_sub(LabelUse::X86JmpRel32.patch_size() as u32);
        self.buffer
            .record_label_patch_site(offset, label, LabelUse::X86JmpRel32)
    }

    pub fn patchable_mov<A, B>(&mut self, dst: A, src: B) -> PatchBlockId
    where
        A: OperandCast + Copy,
        B: OperandCast + Copy,
        Self: MovEmitter<A, B>,
    {
        let dst_op = *dst.as_operand();
        let src_op = *src.as_operand();
        if !src_op.is_imm() {
            unreachable!("patchable_mov currently only supports immediate sources");
        }

        MovEmitter::mov(self, dst, src);

        if dst_op.is_reg_type_of(RegType::Gp64) {
            let offset = self.buffer.cur_offset().saturating_sub(8);
            self.buffer.record_patch_block(offset, 8, 1)
        } else if dst_op.is_reg_type_of(RegType::Gp32) {
            MovEmitter::mov(self, dst, src);
            let offset = self.buffer.cur_offset().saturating_sub(4);
            self.buffer.record_patch_block(offset, 4, 1)
        } else {
            unreachable!("patchable_mov currently only supports Gp64 and Gp32 destinations");
        }
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
        let mut buf = CodeBuffer::new();
        {
            let mut a = Assembler::new(&mut buf);
            f(&mut a);
            assert!(a.last_error().is_none(), "{:?}", a.last_error());
        }
        buf.finish().data().to_vec()
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
        let mut buf = CodeBuffer::new();
        let mut a = Assembler::new(&mut buf);
        // add rax, xmm0 — no signature matches; nothing is emitted.
        a.emit_n(InstId::Add as u32, &[RAX.as_operand(), XMM0.as_operand()]);
        assert!(matches!(
            a.last_error(),
            Some(X86Error::InvalidInstruction { .. })
        ));
        assert!(a.buffer.data().is_empty());
        // Unknown instruction id.
        a.clear_error();
        a.emit_n(u32::MAX, &[]);
        assert!(a.last_error().is_some());
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
        // Backward jmp to a bound label: rel32 = 0 - 0 - 5 = -5.
        assert_eq!(
            asm(|a| {
                let label = a.get_label();
                a.bind_label(label);
                a.emit_n(
                    InstId::Jmp as u32,
                    &[Label::from_id(label.id()).as_operand()],
                );
            }),
            [0xE9, 0xFB, 0xFF, 0xFF, 0xFF]
        );
        // Forward jmp to an unbound label, bound right after: rel32 = 0.
        assert_eq!(
            asm(|a| {
                let label = a.get_label();
                a.emit_n(
                    InstId::Jmp as u32,
                    &[Label::from_id(label.id()).as_operand()],
                );
                a.bind_label(label);
            }),
            [0xE9, 0x00, 0x00, 0x00, 0x00]
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
            [0xE9, 0xFB, 0xFF, 0xFF, 0xFF]
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

        let mut buf = CodeBuffer::new();
        let mut builder = Builder::new();
        let done = buf.get_label();
        builder.push_inst(Inst::with_operands(
            InstId::Mov as u32,
            &[*RAX.as_operand(), *imm(1).as_operand()],
        ));
        builder.push_inst(Inst::with_operands(
            InstId::Add as u32,
            &[*RAX.as_operand(), *RBX.as_operand()],
        ));
        builder.push_inst(Inst::with_operands(
            InstId::Jmp as u32,
            &[*Label::from_id(done.id()).as_operand()],
        ));
        builder.push_inst(Inst::with_operands(InstId::Ret as u32, &[]));
        builder.push_label(done);
        builder.push_inst(Inst::with_operands(InstId::Ret as u32, &[]));
        {
            let mut a = Assembler::new(&mut buf);
            builder.emit_into(&mut a).unwrap();
            assert!(a.last_error().is_none(), "{:?}", a.last_error());
        }
        assert_eq!(buf.finish().data().to_vec(), direct());
    }
}
