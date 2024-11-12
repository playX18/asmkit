use alloc::{borrow::Cow, collections::BinaryHeap, vec::Vec};

use smallvec::SmallVec;

use crate::riscv::{self};

use super::{
    operand::{Label, Sym},
    target::Environment,
};

/// A buffer of output to be produced, fixed up, and then emitted to a CodeSink
/// in bulk.
///
/// This struct uses `SmallVec`s to support small-ish function bodies without
/// any heap allocation. As such, it will be several kilobytes large. This is
/// likely fine as long as it is stack-allocated for function emission then
/// thrown away; but beware if many buffer objects are retained persistently.
#[derive(Default)]
pub struct CodeBuffer {
    env: Environment,
    data: SmallVec<[u8; 1024]>,
    relocs: SmallVec<[AsmReloc; 16]>,
    symbols: SmallVec<[SymData; 16]>,
    label_offsets: SmallVec<[CodeOffset; 16]>,
    pending_fixup_records: SmallVec<[AsmFixup; 16]>,
    pending_fixup_deadline: u32,
    pending_constants: SmallVec<[Constant; 16]>,
    pending_constants_size: CodeOffset,
    used_constants: SmallVec<[(Constant, CodeOffset); 4]>,
    constants: SmallVec<[(ConstantData, AsmConstant); 4]>,
    fixup_records: BinaryHeap<AsmFixup>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ExternalName {
    Symbol(Cow<'static, str>),
    UserName(u32),
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum RelocTarget {
    Sym(Sym),
    Label(Label),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RelocDistance {
    Near,
    Far,
}

pub(crate) struct SymData {
    name: ExternalName,
    distance: RelocDistance,
}

/// A relocation resulting from emitting assembly.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AsmReloc {
    pub offset: CodeOffset,
    pub kind: Reloc,
    pub addend: i64,
    pub target: RelocTarget,
}
/// A fixup to perform on the buffer once code is emitted.
/// Fixups always refer to labels and patch the code based on label offsets.
/// Hence, they are like relocations, but internal to one buffer.
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct AsmFixup {
    pub label: Label,
    pub offset: CodeOffset,
    pub kind: LabelUse,
}

/// Metadata about a constant.
#[derive(Clone, Copy)]
struct AsmConstant {
    /// A label which has not yet been bound which can be used for this
    /// constant.
    ///
    /// This is lazily created when a label is requested for a constant and is
    /// cleared when a constant is emitted.
    upcoming_label: Option<Label>,
    /// Required alignment.
    align: CodeOffset,
    /// The byte size of this constant.
    size: usize,
}

/// A `CodeBuffer` once emission is completed: holds generated code and records,
/// without fixups. This allows the type to be independent of the backend.
pub struct CodeBufferFinalized {
    pub(crate) data: SmallVec<[u8; 1024]>,
    pub(crate) relocs: SmallVec<[AsmReloc; 16]>,
    pub(crate) symbols: SmallVec<[SymData; 16]>,
    pub(crate) alignment: u32,
}

impl CodeBufferFinalized {
    pub fn total_size(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data[..]
    }

    pub fn symbol_name(&self, sym: Sym) -> &ExternalName {
        &self.symbols[sym.id() as usize].name
    }

    pub fn symbol_distance(&self, sym: Sym) -> RelocDistance {
        self.symbols[sym.id() as usize].distance
    }

    pub fn relocs(&self) -> &[AsmReloc] {
        &self.relocs[..]
    }

    pub fn alignment(&self) -> u32 {
        self.alignment
    }
}

impl CodeBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_env(env: Environment) -> Self {
        Self {
            env,
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.relocs.clear();
        self.label_offsets.clear();
        self.pending_fixup_records.clear();
    }
    pub fn env(&self) -> &Environment {
        &self.env
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn relocs(&self) -> &[AsmReloc] {
        &self.relocs
    }

    pub fn put1(&mut self, value: u8) {
        self.data.push(value);
    }

    pub fn put2(&mut self, value: u16) {
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn put4(&mut self, value: u32) {
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn put8(&mut self, value: u64) {
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn write_u8(&mut self, value: u8) {
        self.data.push(value);
    }

    pub fn write_u16(&mut self, value: u16) {
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn write_u32(&mut self, value: u32) {
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn write_u64(&mut self, value: u64) {
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn add_symbol(&mut self, name: impl Into<ExternalName>, distance: RelocDistance) -> Sym {
        let ix = self.symbols.len();
        self.symbols.push(SymData {
            distance,
            name: name.into(),
        });

        Sym::from_id(ix as u32)
    }

    pub fn symbol_distance(&self, sym: Sym) -> RelocDistance {
        self.symbols[sym.id() as usize].distance
    }

    pub fn symbol_name(&self, sym: Sym) -> &ExternalName {
        &self.symbols[sym.id() as usize].name
    }

    pub fn get_label(&mut self) -> Label {
        let l = self.label_offsets.len();
        self.label_offsets.push(u32::MAX);
        Label::from_id(l as _)
    }

    pub fn get_label_for_constant(&mut self, constant: Constant) -> Label {
        let (
            _,
            AsmConstant {
                upcoming_label,
                align: _,
                size,
            },
        ) = self.constants[constant.0 as usize];
        if let Some(label) = upcoming_label {
            return label;
        }

        let label = self.get_label();
        self.pending_constants.push(constant);
        self.pending_constants_size += size as u32;
        self.constants[constant.0 as usize].1.upcoming_label = Some(label);
        label
    }

    pub fn add_constant(&mut self, constant: impl Into<ConstantData>) -> Constant {
        let c = self.constants.len() as u32;
        let data = constant.into();
        let x = AsmConstant {
            upcoming_label: None,
            align: data.alignment() as _,
            size: data.as_slice().len(),
        };
        self.constants.push((data, x));
        Constant(c)
    }

    pub fn use_label_at_offset(&mut self, offset: CodeOffset, label: Label, kind: LabelUse) {
        let fixup = AsmFixup {
            kind,
            label,
            offset,
        };

        self.pending_fixup_records.push(fixup);
    }

    /// Align up to the given alignment.
    pub fn align_to(&mut self, align_to: CodeOffset) {
        assert!(
            align_to.is_power_of_two(),
            "{align_to} is not a power of two"
        );
        while self.cur_offset() & (align_to - 1) != 0 {
            self.write_u8(0);
        }

        // Post-invariant: as for `put1()`.
    }

    pub fn cur_offset(&self) -> CodeOffset {
        self.data.len() as _
    }

    pub fn bind_label(&mut self, label: Label) {
        self.label_offsets[label.id() as usize] = self.cur_offset();
    }

    pub fn label_offset(&self, label: Label) -> u32 {
        self.label_offsets[label.id() as usize]
    }

    pub fn add_reloc(&mut self, kind: Reloc, target: RelocTarget, addend: i64) {
        let offset = self.cur_offset();
        self.add_reloc_at_offset(offset, kind, target, addend);
    }

    pub fn add_reloc_at_offset(
        &mut self,
        offset: CodeOffset,
        kind: Reloc,
        target: RelocTarget,
        addend: i64,
    ) {
        self.relocs.push(AsmReloc {
            addend,
            kind,
            offset,
            target,
        })
    }

    fn handle_fixup(&mut self, fixup: AsmFixup) {
        let AsmFixup {
            kind,
            label,
            offset,
        } = fixup;
        let start = offset as u32;
        let end = offset as usize + kind.patch_size();

        let label_offset = self.label_offsets[label.id() as usize];
        if label_offset != u32::MAX {
            let veneer_required = if label_offset >= offset {
                false
            } else {
                (offset - label_offset) > kind.max_neg_range()
            };

            if veneer_required {
                self.emit_veneer(label, offset, kind);
            } else {
                let slice = &mut self.data[start as usize..end as usize];

                kind.patch(slice, start, label_offset);
            }
        } else {
            // If the offset of this label is not known at this time then
            // that means that a veneer is required because after this
            // island the target can't be in range of the original target.
            self.emit_veneer(label, offset, kind);
        }
    }

    /// Emits a "veneer" the `kind` code at `offset` to jump to `label`.
    ///
    /// This will generate extra machine code, using `kind`, to get a
    /// larger-jump-kind than `kind` allows. The code at `offset` is then
    /// patched to jump to our new code, and then the new code is enqueued for
    /// a fixup to get processed at some later time.
    pub fn emit_veneer(&mut self, label: Label, offset: CodeOffset, kind: LabelUse) {
        // If this `kind` doesn't support a veneer then that's a bug in the
        // backend because we need to implement support for such a veneer.
        assert!(
            kind.supports_veneer(),
            "jump beyond the range of {kind:?} but a veneer isn't supported",
        );

        self.align_to(kind.align() as _);
        let veneer_offset = self.cur_offset();
        let start = offset as usize;
        let end = (offset + kind.patch_size() as u32) as usize;
        let slice = &mut self.data[start..end];

        kind.patch(slice, offset, veneer_offset);
        let veneer_slice = self.get_appended_space(kind.veneer_size() as usize);
        let (veneer_fixup_off, veneer_label_use) =
            kind.generate_veneer(veneer_slice, veneer_offset);

        // Register a new use of `label` with our new veneer fixup and
        // offset. This'll recalculate deadlines accordingly and
        // enqueue this fixup to get processed at some later
        // time.
        self.use_label_at_offset(veneer_fixup_off, label, veneer_label_use);
    }

    /// Reserve appended space and return a mutable slice referring to it.
    pub fn get_appended_space(&mut self, len: usize) -> &mut [u8] {
        let off = self.data.len();
        let new_len = self.data.len() + len;
        self.data.resize(new_len, 0);
        &mut self.data[off..]

        // Post-invariant: as for `put1()`.
    }

    /// Returns the maximal offset that islands can reach if `distance` more
    /// bytes are appended.
    ///
    /// This is used to determine if veneers need insertions since jumps that
    /// can't reach past this point must get a veneer of some form.
    fn worst_case_end_of_island(&self, distance: CodeOffset) -> CodeOffset {
        // Assume that all fixups will require veneers and that the veneers are
        // the worst-case size for each platform. This is an over-generalization
        // to avoid iterating over the `fixup_records` list or maintaining
        // information about it as we go along.
        let island_worst_case_size =
            ((self.fixup_records.len() + self.pending_fixup_records.len()) as u32) * 20
                + self.pending_constants_size;
        self.cur_offset()
            .saturating_add(distance)
            .saturating_add(island_worst_case_size)
    }

    fn should_apply_fixup(&self, fixup: &AsmFixup, forced_threshold: CodeOffset) -> bool {
        let label_offset = self.label_offset(fixup.label);
        label_offset != u32::MAX
            || fixup.offset.saturating_add(fixup.kind.max_pos_range()) < forced_threshold
    }
    /// Is an island needed within the next N bytes?
    pub fn island_needed(&mut self, distance: CodeOffset) -> bool {
        let deadline = match self.fixup_records.peek() {
            Some(fixup) => fixup
                .offset
                .saturating_add(fixup.kind.max_pos_range())
                .min(self.pending_fixup_deadline),
            None => self.pending_fixup_deadline,
        };

        deadline < u32::MAX && self.worst_case_end_of_island(distance) > deadline
    }

    /// Emit all pending constants and required pending veneers.
    pub fn emit_island(&mut self, distance: CodeOffset) {
        let forced_threshold = self.worst_case_end_of_island(distance);

        for constant in core::mem::take(&mut self.pending_constants) {
            let (_, AsmConstant { align, size, .. }) = self.constants[constant.0 as usize];
            let label = self.constants[constant.0 as usize]
                .1
                .upcoming_label
                .take()
                .unwrap();
            self.align_to(align as _);
            self.bind_label(label);
            self.used_constants.push((constant, self.cur_offset()));
            self.get_appended_space(size);
        }
        // Either handle all pending fixups because they're ready or move them
        // onto the `BinaryHeap` tracking all pending fixups if they aren't
        // ready.
        for fixup in core::mem::take(&mut self.pending_fixup_records) {
            if self.should_apply_fixup(&fixup, forced_threshold) {
                self.handle_fixup(fixup);
            } else {
                self.fixup_records.push(fixup);
            }
        }

        self.pending_fixup_deadline = u32::MAX;

        while let Some(fixup) = self.fixup_records.peek() {
            // If this fixup shouldn't be applied, that means its label isn't
            // defined yet and there'll be remaining space to apply a veneer if
            // necessary in the future after this island. In that situation
            // because `fixup_records` is sorted by deadline this loop can
            // exit.
            if !self.should_apply_fixup(fixup, forced_threshold) {
                break;
            }
            let fixup = self.fixup_records.pop().unwrap();
            self.handle_fixup(fixup);
        }
    }

    fn finish_emission_maybe_forcing_veneers(&mut self) {
        while !self.pending_constants.is_empty()
            || !self.pending_fixup_records.is_empty()
            || !self.fixup_records.is_empty()
        {
            // `emit_island()` will emit any pending veneers and constants, and
            // as a side-effect, will also take care of any fixups with resolved
            // labels eagerly.
            self.emit_island(u32::MAX);
        }
    }

    fn finish_constants(&mut self) -> u32 {
        let mut alignment = 32;

        for (constant, offset) in core::mem::take(&mut self.used_constants) {
            let constant = &self.constants[constant.0 as usize].0;
            let data = constant.as_slice();
            self.data[offset as usize..][..data.len()].copy_from_slice(data);
            alignment = constant.alignment().max(alignment);
        }

        alignment as _
    }

    pub fn finish(mut self) -> CodeBufferFinalized {
        self.finish_emission_maybe_forcing_veneers();
        let alignment = self.finish_constants();
        CodeBufferFinalized {
            data: self.data,
            relocs: self.relocs,
            symbols: self.symbols,
            alignment,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum ConstantData {
    WellKnown(&'static [u8]),
    U64([u8; 8]),
    Bytes(Vec<u8>),
}

impl ConstantData {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            ConstantData::WellKnown(data) => data,
            ConstantData::U64(data) => data.as_ref(),
            ConstantData::Bytes(data) => data,
        }
    }

    pub fn alignment(&self) -> usize {
        if self.as_slice().len() <= 8 {
            8
        } else {
            16
        }
    }
}

/// A use of a constant by one or mroe assembly instructions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Constant(pub(crate) u32);

impl From<&'static str> for ConstantData {
    fn from(value: &'static str) -> Self {
        Self::WellKnown(value.as_bytes())
    }
}

impl From<[u8; 8]> for ConstantData {
    fn from(value: [u8; 8]) -> Self {
        Self::U64(value)
    }
}

impl From<Vec<u8>> for ConstantData {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl From<&'static [u8]> for ConstantData {
    fn from(value: &'static [u8]) -> Self {
        Self::WellKnown(value)
    }
}

impl From<u64> for ConstantData {
    fn from(value: u64) -> Self {
        Self::U64(value.to_ne_bytes())
    }
}

/// Offset in bytes from the beginning of the function.
///
/// Cranelift can be used as a cross compiler, so we don't want to use a type like `usize` which
/// depends on the *host* platform, not the *target* platform.
pub type CodeOffset = u32;

/// Addend to add to the symbol value.
pub type Addend = i64;

/// Relocation kinds for every ISA
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Reloc {
    /// absolute 4-byte
    Abs4,
    /// absolute 8-byte
    Abs8,
    /// x86 PC-relative 4-byte
    X86PCRel4,
    /// x86 call to PC-relative 4-byte
    X86CallPCRel4,
    /// x86 call to PLT-relative 4-byte
    X86CallPLTRel4,
    /// x86 GOT PC-relative 4-byte
    X86GOTPCRel4,
    /// The 32-bit offset of the target from the beginning of its section.
    /// Equivalent to `IMAGE_REL_AMD64_SECREL`.
    /// See: [PE Format](https://docs.microsoft.com/en-us/windows/win32/debug/pe-format)
    X86SecRel,
    /// Arm32 call target
    Arm32Call,
    /// Arm64 call target. Encoded as bottom 26 bits of instruction. This
    /// value is sign-extended, multiplied by 4, and added to the PC of
    /// the call instruction to form the destination address.
    Arm64Call,

    /// Elf x86_64 32 bit signed PC relative offset to two GOT entries for GD symbol.
    ElfX86_64TlsGd,

    /// Mach-O x86_64 32 bit signed PC relative offset to a `__thread_vars` entry.
    MachOX86_64Tlv,

    /// Mach-O Aarch64 TLS
    /// PC-relative distance to the page of the TLVP slot.
    MachOAarch64TlsAdrPage21,

    /// Mach-O Aarch64 TLS
    /// Offset within page of TLVP slot.
    MachOAarch64TlsAdrPageOff12,

    /// Aarch64 TLSDESC Adr Page21
    /// This is equivalent to `R_AARCH64_TLSDESC_ADR_PAGE21` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescAdrPage21,

    /// Aarch64 TLSDESC Ld64 Lo12
    /// This is equivalent to `R_AARCH64_TLSDESC_LD64_LO12` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescLd64Lo12,

    /// Aarch64 TLSDESC Add Lo12
    /// This is equivalent to `R_AARCH64_TLSGD_ADD_LO12` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescAddLo12,

    /// Aarch64 TLSDESC Call
    /// This is equivalent to `R_AARCH64_TLSDESC_CALL` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescCall,

    /// AArch64 GOT Page
    /// Set the immediate value of an ADRP to bits 32:12 of X; check that –2^32 <= X < 2^32
    /// This is equivalent to `R_AARCH64_ADR_GOT_PAGE` (311) in the  [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#static-aarch64-relocations)
    Aarch64AdrGotPage21,

    /// AArch64 GOT Low bits

    /// Set the LD/ST immediate field to bits 11:3 of X. No overflow check; check that X&7 = 0
    /// This is equivalent to `R_AARCH64_LD64_GOT_LO12_NC` (312) in the  [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#static-aarch64-relocations)
    Aarch64Ld64GotLo12Nc,

    /// RISC-V Absolute address: 64-bit address.
    RiscvAbs8,

    /// RISC-V Call PLT: 32-bit PC-relative function call, macros call, tail (PIC)
    ///
    /// Despite having PLT in the name, this relocation is also used for normal calls.
    /// The non-PLT version of this relocation has been deprecated.
    ///
    /// This is the `R_RISCV_CALL_PLT` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#procedure-calls>
    RiscvCallPlt,

    /// RISC-V TLS GD: High 20 bits of 32-bit PC-relative TLS GD GOT reference,
    ///
    /// This is the `R_RISCV_TLS_GD_HI20` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#global-dynamic>
    RiscvTlsGdHi20,

    /// Low 12 bits of a 32-bit PC-relative relocation (I-Type instruction)
    ///
    /// This is the `R_RISCV_PCREL_LO12_I` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#pc-relative-symbol-addresses>
    RiscvPCRelLo12I,

    /// High 20 bits of a 32-bit PC-relative GOT offset relocation
    ///
    /// This is the `R_RISCV_GOT_HI20` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#pc-relative-symbol-addresses>
    RiscvGotHi20,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LabelUse {
    X86JmpRel32,
    /// 20-bit branch offset (unconditional branches). PC-rel, offset is
    /// imm << 1. Immediate is 20 signed bits. Use in Jal instructions.
    RVJal20,
    /// The unconditional jump instructions all use PC-relative
    /// addressing to help support position independent code. The JALR
    /// instruction was defined to enable a two-instruction sequence to
    /// jump anywhere in a 32-bit absolute address range. A LUI
    /// instruction can first load rs1 with the upper 20 bits of a
    /// target address, then JALR can add in the lower bits. Similarly,
    /// AUIPC then JALR can jump anywhere in a 32-bit pc-relative
    /// address range.
    RVPCRel32,

    /// All branch instructions use the B-type instruction format. The
    /// 12-bit B-immediate encodes signed offsets in multiples of 2, and
    /// is added to the current pc to give the target address. The
    /// conditional branch range is ±4 KiB.
    RVB12,

    /// Equivalent to the `R_RISCV_PCREL_HI20` relocation, Allows setting
    /// the immediate field of an `auipc` instruction.
    RVPCRelHi20,

    /// Similar to the `R_RISCV_PCREL_LO12_I` relocation but pointing to
    /// the final address, instead of the `PCREL_HI20` label. Allows setting
    /// the immediate field of I Type instructions such as `addi` or `lw`.
    ///
    /// Since we currently don't support offsets in labels, this relocation has
    /// an implicit offset of 4.
    RVPCRelLo12I,

    /// 11-bit PC-relative jump offset. Equivalent to the `RVC_JUMP` relocation
    RVCJump,
    /// 9-bit PC-relative branch offset.
    RVCB9,
    /// 14-bit branch offset (conditional branches). PC-rel, offset is imm <<
    /// 2. Immediate is 14 signed bits, in bits 18:5. Used by tbz and tbnz.
    A64Branch14,
    /// 19-bit branch offset (conditional branches). PC-rel, offset is imm << 2. Immediate is 19
    /// signed bits, in bits 23:5. Used by cbz, cbnz, b.cond.
    A64Branch19,
    /// 26-bit branch offset (unconditional branches). PC-rel, offset is imm << 2. Immediate is 26
    /// signed bits, in bits 25:0. Used by b, bl.
    A64Branch26,
    A64Ldr19,
    A64Adr21,
}

impl LabelUse {
    /// Maximum PC-relative range (positive), inclusive.
    pub const fn max_pos_range(self) -> CodeOffset {
        match self {
            LabelUse::RVJal20 => ((1 << 19) - 1) * 2,
            LabelUse::RVPCRelLo12I | LabelUse::RVPCRelHi20 | LabelUse::RVPCRel32 => {
                let imm20_max: i64 = ((1 << 19) - 1) << 12;
                let imm12_max = (1 << 11) - 1;
                (imm20_max + imm12_max) as _
            }
            LabelUse::RVB12 => ((1 << 11) - 1) * 2,
            LabelUse::RVCB9 => ((1 << 8) - 1) * 2,
            LabelUse::RVCJump => ((1 << 10) - 1) * 2,
            LabelUse::X86JmpRel32 => i32::MAX as _,
            _ => u32::MAX,
        }
    }

    pub const fn max_neg_range(self) -> CodeOffset {
        match self {
            LabelUse::RVPCRel32 => {
                let imm20_max: i64 = (1 << 19) << 12;
                let imm12_max = 1 << 11;
                (-imm20_max - imm12_max) as CodeOffset
            }
            _ => self.max_pos_range() + 2,
        }
    }

    pub const fn patch_size(&self) -> usize {
        match self {
            Self::X86JmpRel32 => 4,
            Self::RVCJump | Self::RVCB9 => 2,
            Self::RVJal20 | Self::RVB12 | Self::RVPCRelHi20 | Self::RVPCRelLo12I => 4,
            Self::RVPCRel32 => 8,
            _ => 4,
        }
    }

    pub const fn align(&self) -> usize {
        match self {
            Self::X86JmpRel32 => 1,
            Self::RVCJump => 4,
            Self::RVJal20 | Self::RVB12 | Self::RVCB9 | Self::RVPCRelHi20 | Self::RVPCRelLo12I => 4,
            Self::RVPCRel32 => 4,
            _ => 4,
        }
    }

    pub const fn supports_veneer(&self) -> bool {
        match self {
            Self::RVB12 | Self::RVJal20 | Self::RVCJump => true,
            _ => false,
        }
    }

    pub const fn veneer_size(&self) -> usize {
        match self {
            Self::RVB12 | Self::RVJal20 | Self::RVCJump => 8,
            _ => unreachable!(),
        }
    }

    pub fn generate_veneer(
        &self,
        buffer: &mut [u8],
        veneer_offset: CodeOffset,
    ) -> (CodeOffset, Self) {
        if matches!(
            self,
            Self::RVB12
                | Self::RVCJump
                | Self::RVJal20
                | Self::RVPCRelHi20
                | Self::RVPCRelLo12I
                | Self::RVPCRel32
        ) {
            let base = riscv::X31;

            {
                let x = riscv::Inst::new(riscv::Opcode::AUIPC)
                    .encode()
                    .set_rd(base.id())
                    .value
                    .to_le_bytes();
                buffer[0] = x[0];
                buffer[1] = x[1];
                buffer[2] = x[2];
                buffer[3] = x[3];
            }

            {
                let x = riscv::Inst::new(riscv::Opcode::JALR)
                    .encode()
                    .set_rd(riscv::ZERO.id())
                    .set_rs1(base.id())
                    .value
                    .to_le_bytes();
                buffer[4] = x[0];
                buffer[5] = x[1];
                buffer[6] = x[2];
                buffer[7] = x[3];
            }

            (veneer_offset, LabelUse::RVPCRel32)
        } else {
            todo!()
        }
    }
    pub fn patch(&self, buffer: &mut [u8], use_offset: CodeOffset, label_offset: CodeOffset) {
        let pc_reli = (label_offset as i64) - (use_offset as i64);

        let pc_rel = pc_reli as u32;

        match self {
            Self::X86JmpRel32 => {
                let addend = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);

                let value = pc_rel.wrapping_add(addend).wrapping_sub(4);

                buffer.copy_from_slice(&value.to_le_bytes());
            }

            Self::RVJal20 => {
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let offset = pc_rel as u32;
                let v = ((offset >> 12 & 0b1111_1111) << 12)
                    | ((offset >> 11 & 0b1) << 20)
                    | ((offset >> 1 & 0b11_1111_1111) << 21)
                    | ((offset >> 20 & 0b1) << 31);
                buffer[0..4].clone_from_slice(&u32::to_le_bytes(insn | v));
            }

            Self::RVPCRel32 => {
                let (imm20, imm12) = generate_imm(pc_rel as u64);
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let insn2 = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);

                let auipc = riscv::Inst::new(riscv::Opcode::AUIPC).encode().set_imm20(0);
                let jalr = riscv::Inst::new(riscv::Opcode::JALR)
                    .encode()
                    .set_rd(0)
                    .set_rs1(0)
                    .set_imm12(0);

                buffer[0..4].copy_from_slice(&(insn | auipc.value | imm20).to_le_bytes());
                buffer[4..8].copy_from_slice(&(insn2 | jalr.value | imm12).to_le_bytes());
            }

            Self::RVB12 => {
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let offset = pc_rel as u32;
                let v = ((offset >> 11 & 0b1) << 7)
                    | ((offset >> 1 & 0b1111) << 8)
                    | ((offset >> 5 & 0b11_1111) << 25)
                    | ((offset >> 12 & 0b1) << 31);
                buffer[0..4].clone_from_slice(&u32::to_le_bytes(insn | v));
            }

            Self::RVPCRelHi20 => {
                // See https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#pc-relative-symbol-addresses
                //
                // We need to add 0x800 to ensure that we land at the next page as soon as it goes out of range for the
                // Lo12 relocation. That relocation is signed and has a maximum range of -2048..2047. So when we get an
                // offset of 2048, we need to land at the next page and subtract instead.
                let offset = pc_reli as u32;
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let hi20 = offset.wrapping_add(0x800) >> 12;
                let insn = (insn & 0xfff) | (hi20 << 12);
                buffer[0..4].copy_from_slice(&insn.to_le_bytes());
            }

            Self::RVPCRelLo12I => {
                // `offset` is the offset from the current instruction to the target address.
                //
                // However we are trying to compute the offset to the target address from the previous instruction.
                // The previous instruction should be the one that contains the PCRelHi20 relocation and
                // stores/references the program counter (`auipc` usually).
                //
                // Since we are trying to compute the offset from the previous instruction, we can
                // represent it as offset = target_address - (current_instruction_address - 4)
                // which is equivalent to offset = target_address - current_instruction_address + 4.
                //
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);

                let lo12 = (pc_reli + 4) as u32 & 0xfff;
                let insn = (insn & 0xFFFFF) | (lo12 << 20);
                buffer[0..4].copy_from_slice(&insn.to_le_bytes());
            }

            Self::RVCJump => {
                debug_assert!(pc_rel & 1 == 0);

                let insn = riscv::Inst::new(riscv::Opcode::CJ)
                    .encode()
                    .set_c_imm12(pc_rel as _);
                buffer[0..2].clone_from_slice(&(insn.value as u16).to_le_bytes());
            }
            _ => todo!(),
        }
    }
}

pub const fn is_imm12(val: i64) -> bool {
    val >= -2048 && val <= 2047
}

pub(crate) fn generate_imm(value: u64) -> (u32, u32) {
    if is_imm12(value as _) {
        return (
            0,
            riscv::InstructionValue::new(0)
                .set_imm12(value as i64 as i32)
                .value,
        );
    }

    let value = value as i64;

    let mod_num = 4096i64;
    let (imm20, imm12) = if value > 0 {
        let mut imm20 = value / mod_num;
        let mut imm12 = value % mod_num;

        if imm12 >= 2048 {
            imm12 -= mod_num;
            imm20 += 1;
        }

        (imm20, imm12)
    } else {
        let value_abs = value.abs();
        let imm20 = value_abs / mod_num;
        let imm12 = value_abs % mod_num;
        let mut imm20 = -imm20;
        let mut imm12 = -imm12;
        if imm12 < -2048 {
            imm12 += mod_num;
            imm20 -= 1;
        }
        (imm20, imm12)
    };
    (
        riscv::InstructionValue::new(0).set_imm20(imm20 as _).value,
        riscv::InstructionValue::new(0).set_imm12(imm12 as _).value,
    )
}

/// A generic implementation of relocation resolving. 
/// 
/// # NOTE
/// 
/// Very simple and incomplete. At the moment only Abs4, Abs8, X86 and RISC-V GOT relocations are supported.
pub fn perform_relocations(
    code: *mut u8,
    code_rx: *const u8,
    relocs: &[AsmReloc],
    get_address: impl Fn(&RelocTarget) -> *const u8,
    get_got_entry: impl Fn(&RelocTarget) -> *const u8,
    get_plt_entry: impl Fn(&RelocTarget) -> *const u8,
) {
    use core::ptr::write_unaligned;

    for &AsmReloc {
        addend,
        kind,
        offset,
        ref target,
    } in relocs
    {
        let at = unsafe { code.offset(isize::try_from(offset).unwrap()) };
        let atrx = unsafe { code_rx.offset(isize::try_from(offset).unwrap()) };
        match kind {
            Reloc::Abs4 => {
                let base = get_address(target);
                let what = unsafe { base.offset(isize::try_from(addend).unwrap()) };
                unsafe {
                    write_unaligned(at as *mut u32, u32::try_from(what as usize).unwrap());
                }
            }

            Reloc::Abs8 => {
                let base = get_address(target);
                let what = unsafe { base.offset(isize::try_from(addend).unwrap()) };
                unsafe {
                    write_unaligned(at as *mut u64, u64::try_from(what as usize).unwrap());
                }
            }

            Reloc::X86PCRel4 | Reloc::X86CallPCRel4 => {
                let base = get_address(target);
                let what = unsafe { base.offset(isize::try_from(addend).unwrap()) };
                let pcrel = i32::try_from((what as isize) - (atrx as isize)).unwrap();

                unsafe {
                    write_unaligned(at as *mut i32, pcrel);
                }
            }

            Reloc::X86GOTPCRel4 => {
                let base = get_got_entry(target);
                let what = unsafe { base.offset(isize::try_from(addend).unwrap()) };
                let pcrel = i32::try_from((what as isize) - (atrx as isize)).unwrap();

                unsafe {
                    write_unaligned(at as *mut i32, pcrel);
                }
            }

            Reloc::X86CallPLTRel4 => {
                let base = get_plt_entry(target);
                let what = unsafe { base.offset(isize::try_from(addend).unwrap()) };
                let pcrel = i32::try_from((what as isize) - (atrx as isize)).unwrap();
                unsafe { write_unaligned(at as *mut i32, pcrel) };
            }

            Reloc::RiscvGotHi20 => {
                let base = get_got_entry(target);
                let what = unsafe { base.offset(isize::try_from(addend).unwrap()) };
                let pc_rel = i32::try_from((what as isize) - (atrx as isize)).unwrap();
                unsafe {
                    let buffer = core::slice::from_raw_parts_mut(at as *mut u8, 4);
                    let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                    let hi20 = (pc_rel as u32).wrapping_add(0x800) >> 12;
                    let insn = (insn & 0xfff) | (hi20 << 12);
                    buffer.copy_from_slice(&insn.to_le_bytes());
                }
            }

            Reloc::RiscvPCRelLo12I => {
                let base = get_got_entry(target);
                let what = unsafe { base.offset(isize::try_from(addend).unwrap()) };
                let pc_rel = i32::try_from((what as isize) - (atrx as isize)).unwrap();

                unsafe {
                    let buffer = core::slice::from_raw_parts_mut(at as *mut u8, 4);
                    let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                    let lo12 = (pc_rel + 4) as u32 & 0xfff;
                    let insn = (insn & 0xFFFFF) | (lo12 << 20);
                    buffer.copy_from_slice(&insn.to_le_bytes());
                }
            }

            _ => todo!(),
        }
    }
}
