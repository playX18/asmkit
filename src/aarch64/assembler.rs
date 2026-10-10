use crate::AsmError;
use crate::aarch64::operands::*;
use crate::aarch64::{Gp, Reg, instdb::*};
use crate::core::arch_traits::Arch;
use crate::core::buffer::CodeBuffer;
use crate::core::buffer::{CodeOffset, Constant, LabelUse, Reloc, RelocDistance, RelocTarget};
use crate::core::globals::CondCode;
use crate::core::operand::*;
use crate::core::patch::{DataEncoding, DataLabel, PatchableJump, PatchableRegion};
use crate::core::target::Environment;

pub struct Assembler<'a> {
    pub(crate) buffer: &'a mut CodeBuffer,
    /// Scratch error set by the generated emitter during one checked attempt.
    pub(crate) last_error: Option<AsmError>,
}

fn validate_raw_operand(buffer: &CodeBuffer, op: &Operand) -> bool {
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
            let expected = Reg::signature_of(reg_type);
            let mask = OperandSignature::OP_TYPE_MASK
                | OperandSignature::REG_TYPE_MASK
                | OperandSignature::REG_GROUP_MASK
                | OperandSignature::SIZE_MASK;
            expected.bits() != 0
                && op.signature.subset(mask) == expected.subset(mask)
                && match reg_type {
                    RegType::Gp32 | RegType::Gp64 => op.id() <= 31 || op.id() == Gp::ID_ZR,
                    RegType::Vec8
                    | RegType::Vec16
                    | RegType::Vec32
                    | RegType::Vec64
                    | RegType::Vec128 => op.id() <= 31,
                    _ => false,
                }
        }
        OperandType::Mem => {
            let Some(base_type) = op.signature.try_mem_base_type() else {
                return false;
            };
            let Some(index_type) = op.signature.try_mem_index_type() else {
                return false;
            };
            let mem = op.as_::<BaseMem>();
            let offset_mode = op
                .signature
                .get_field::<{ Mem::SIGNATURE_MEM_OFFSET_MODE_MASK }>();
            let shift_op = op
                .signature
                .get_field::<{ Mem::SIGNATURE_MEM_SHIFT_OP_MASK }>();
            let valid_base = match base_type {
                RegType::None | RegType::LabelTag | RegType::SymTag => true,
                RegType::Gp32 | RegType::Gp64 => mem.base_id() <= 31,
                _ => false,
            };
            let valid_index = match index_type {
                RegType::None => mem.index_id() == 0,
                RegType::Gp32 | RegType::Gp64 => mem.index_id() <= 31,
                _ => false,
            };
            valid_base
                && valid_index
                && offset_mode <= OffsetMode::PostIndex as u32
                && shift_op <= 13
                && (!mem.has_base_sym()
                    || buffer.symbol_name(Sym::from_id(mem.base_id())).is_some())
        }
        OperandType::Sym => buffer.symbol_name(Sym::from_id(op.id())).is_some(),
        OperandType::Imm | OperandType::Label => true,
        OperandType::RegList => false,
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
        self.try_emit_n(inst.id(), &refs)
    }

    fn bind_label(&mut self, label: Label) -> Result<(), AsmError> {
        self.try_bind_label(label)
    }
}

pub trait LoadConstantEmitter<DST, SRC> {
    fn load_constant(&mut self, dst: DST, src: SRC);
}

impl LoadConstantEmitter<Gp, Constant> for Assembler<'_> {
    fn load_constant(&mut self, dst: Gp, src: Constant) {
        let label = self.buffer.get_label_for_constant(src);
        self.load_constant(dst, label);
    }
}

impl LoadConstantEmitter<Gp, Label> for Assembler<'_> {
    fn load_constant(&mut self, dst: Gp, src: Label) {
        self.adrp(dst, src);
        self.buffer
            .use_label_at_offset(self.buffer.cur_offset(), src, LabelUse::A64AddAbsLo12);
        self.add(dst, dst, imm(0));
    }
}

impl LoadConstantEmitter<Gp, Sym> for Assembler<'_> {
    fn load_constant(&mut self, dst: Gp, src: Sym) {
        let Some(distance) = self.buffer.symbol_distance(src) else {
            self.buffer.record_error(AsmError::InvalidArgument);
            return;
        };

        if self.buffer.env().pic() {
            // When PIC is enabled, all syms are referenced through the GOT.
            self.buffer
                .add_reloc(Reloc::Aarch64AdrGotPage21, RelocTarget::Sym(src), 0);
            self.adrp(dst, imm(0));
            self.buffer
                .add_reloc(Reloc::Aarch64Ld64GotLo12Nc, RelocTarget::Sym(src), 0);
            self.ldr(dst, ptr(dst, 0));
            return;
        }

        match distance {
            RelocDistance::Near => {
                self.buffer
                    .add_reloc(Reloc::Aarch64AdrPrelPgHi21, RelocTarget::Sym(src), 0);
                self.adrp(dst, imm(0));
                self.buffer
                    .add_reloc(Reloc::Aarch64AddAbsLo12Nc, RelocTarget::Sym(src), 0);
                self.add(dst, dst, imm(0));
            }

            RelocDistance::Far => {
                // With absolute offsets we set up a load from a preallocated space, and then jump
                // over it.
                //
                // Emit the following code:
                //   ldr     rd, #8
                //   b       #0x10
                //   <8 byte space>
                let constant_start = self.buffer.get_label();
                let constant_end = self.buffer.get_label();
                self.ldr(dst, label_ptr(constant_start, 0));
                self.b(constant_end);
                self.buffer.bind_label(constant_start);
                self.buffer.add_reloc(Reloc::Abs8, RelocTarget::Sym(src), 0);
                self.buffer.write_u64(0);
                self.buffer.bind_label(constant_end);
            }
        }
    }
}

impl<'a> Assembler<'a> {
    pub fn buffer_mut(&mut self) -> &mut CodeBuffer {
        self.buffer
    }

    pub fn new(buffer: &'a mut CodeBuffer) -> Self {
        if buffer.env().arch() != Arch::AArch64 {
            return Self::poisoned(buffer, AsmError::InvalidArch);
        }
        Self::unchecked(buffer)
    }

    pub fn try_new(buffer: &'a mut CodeBuffer) -> Result<Self, AsmError> {
        if buffer.env().arch() != Arch::AArch64 {
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

    /// Tests whether the assembler targets a 32-bit mode (always false for A64).
    pub fn is_32bit(&self) -> bool {
        self.buffer.env().is_32bit()
    }

    /// Tests whether the assembler targets a 64-bit mode (always true for A64).
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

    /// A helper to load a constant address into a register.
    ///
    /// Supported variants are:
    /// ```text
    /// +------------------+
    /// |  DST  |  SRC     |
    /// +------------------+
    /// |  Gp   | Label    |
    /// |  Gp   | Sym      |
    /// |  Gp   | Constant |
    /// +------------------+
    /// ```
    ///
    /// Note that `Sym` is loaded based on `self.buffer.pic()` and its distance. If PIC is enabled
    /// then GOT is always used. Otherwise, if symbol is near it uses `adrp` + `add` combination, and
    /// for far symbols Abs8 reloc is used and data is embedded right into code.
    pub fn load_constant<DST, SRC>(&mut self, dst: DST, src: SRC)
    where
        Self: LoadConstantEmitter<DST, SRC>,
    {
        if self.buffer.error().is_some() {
            return;
        }
        let checkpoint = self.buffer.checkpoint();
        <Self as LoadConstantEmitter<DST, SRC>>::load_constant(self, dst, src);
        if self.buffer.error().is_some() {
            self.buffer.rollback(checkpoint);
        }
    }

    pub fn emit_n(&mut self, id: impl Into<u32>, ops: &[&Operand]) {
        if let Err(error) = self.try_emit_n(id, ops) {
            self.buffer.record_error(error);
        }
    }

    pub fn try_emit_n(&mut self, id: impl Into<u32>, ops: &[&Operand]) -> Result<(), AsmError> {
        if let Some(error) = self.buffer.error().cloned() {
            return Err(error);
        }
        if ops.len() > 6 || ops.iter().any(|op| !validate_raw_operand(self.buffer, op)) {
            return Err(AsmError::InvalidOperand);
        }
        let id = id.into();
        let checkpoint = self.buffer.checkpoint();
        self.last_error = None;
        self._emit(id, ops);
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

    pub fn data(&self) -> &[u8] {
        self.buffer.data()
    }

    pub fn relocs(&self) -> &[crate::core::buffer::AsmReloc] {
        self.buffer.relocs()
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

    /// Emits a 26-bit branch through `emit`, returning it as a patchable jump.
    fn patchable_branch26(&mut self, emit: impl FnOnce(&mut Self)) -> PatchableJump {
        if self.buffer.error().is_some() {
            return PatchableJump::invalid(LabelUse::A64Branch26);
        }
        let checkpoint = self.buffer.checkpoint();
        let offset = self.buffer.cur_offset();
        debug_assert_eq!(offset % 4, 0, "patchable branch must be 4 byte aligned");
        emit(self);
        if self.buffer.error().is_some() {
            self.buffer.rollback(checkpoint);
            return PatchableJump::invalid(LabelUse::A64Branch26);
        }
        PatchableJump::new(offset, LabelUse::A64Branch26)
    }

    /// `b` to `label` that can be retargeted.
    pub fn patchable_b(&mut self, label: Label) -> PatchableJump {
        self.patchable_branch26(|asm| asm.b(label))
    }

    /// `bl` to `label` that can be retargeted.
    pub fn patchable_bl(&mut self, label: Label) -> PatchableJump {
        self.patchable_branch26(|asm| asm.bl(label))
    }

    /// Patchable immediate materialization via a fixed `movz`/`movk` sequence.
    ///
    /// 64-bit destinations use 16 bytes (4 insns); 32-bit destinations use 8
    /// bytes (2 insns). [`repatch_value`](crate::repatch_value) re-encodes the
    /// whole sequence.
    pub fn patchable_mov(&mut self, rd: Gp, imm: impl Into<u64>) -> DataLabel {
        let value = imm.into();
        let is_64 = rd.is_gp64();
        let size = if is_64 { 16 } else { 8 };
        if self.buffer.error().is_some() {
            return DataLabel::invalid(size, DataEncoding::A64MovWide);
        }
        let checkpoint = self.buffer.checkpoint();
        let encoded = encode_patchable_mov_imm(rd.id(), is_64, value);
        let offset = self.buffer.cur_offset();
        for chunk in encoded.chunks_exact(4) {
            let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            self.buffer.write_u32(word);
        }
        if self.buffer.error().is_some() {
            self.buffer.rollback(checkpoint);
            return DataLabel::invalid(size, DataEncoding::A64MovWide);
        }
        DataLabel::new(offset, size, DataEncoding::A64MovWide)
    }
}

/// Encode a fixed-width patchable mov-immediate sequence for AArch64.
///
/// Always emits 2 instructions (8 bytes) for W registers and 4 (16 bytes) for X registers so
/// rewrites never change layout.
pub fn encode_patchable_mov_imm(
    rd: u32,
    is_64bit: bool,
    value: u64,
) -> smallvec::SmallVec<[u8; 16]> {
    let rd = rd & 0x1f;
    let mut out = smallvec::SmallVec::new();
    if is_64bit {
        const MOVZ: u32 = 0b11010010100000000000000000000000;
        const MOVK: u32 = 0b11110010100000000000000000000000;
        let words = [
            MOVZ | (((value as u32) & 0xFFFF) << 5) | rd,
            MOVK | (1 << 21) | ((((value >> 16) as u32) & 0xFFFF) << 5) | rd,
            MOVK | (2 << 21) | ((((value >> 32) as u32) & 0xFFFF) << 5) | rd,
            MOVK | (3 << 21) | ((((value >> 48) as u32) & 0xFFFF) << 5) | rd,
        ];
        for w in words {
            out.extend_from_slice(&w.to_le_bytes());
        }
    } else {
        const MOVZ: u32 = 0b01010010100000000000000000000000;
        const MOVK: u32 = 0b01110010100000000000000000000000;
        let value = value as u32;
        let words = [
            MOVZ | ((value & 0xFFFF) << 5) | rd,
            MOVK | (1 << 21) | (((value >> 16) & 0xFFFF) << 5) | rd,
        ];
        for w in words {
            out.extend_from_slice(&w.to_le_bytes());
        }
    }
    out
}

impl InstId {
    pub const ARM_COND: u32 = 0x78000000;
    pub const REAL_ID: u32 = 65535;
    pub const fn with_cc(self, cond: CondCode) -> u32 {
        let x = self as u32;
        x | (cond as u32) << Self::ARM_COND.trailing_zeros()
    }

    pub const fn extract_cc(inst: u32) -> CondCode {
        // SAFETY: `CondCode` is `#[repr(u8)]` with contiguous discriminants
        // 0..=15, and the 4-bit ARM condition field masked here is always in
        // that range.
        unsafe {
            core::mem::transmute(((inst & Self::ARM_COND) >> Self::ARM_COND.trailing_zeros()) as u8)
        }
    }

    pub const fn extract_real_id(inst: u32) -> u32 {
        inst & Self::REAL_ID
    }
}

pub const BLE: u32 = InstId::B.with_cc(CondCode::GE);

impl From<InstId> for u32 {
    fn from(inst: InstId) -> Self {
        inst as u32
    }
}

// The generated instdb references these through `use super::assembler::*`.
pub(super) use crate::aarch64::encoder::OffsetType;
pub use crate::aarch64::encoder::{
    LogicalImm, count_zero_half_words_64, encode_fp64_to_imm8, encode_logical_imm, is_fp16_imm8,
    is_fp32_imm8, is_fp64_imm8,
};
