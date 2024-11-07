use std::collections::BinaryHeap;

use alloc::borrow::Cow;

use smallvec::SmallVec;

use crate::assembler::riscv;

use super::binemit::{CodeOffset, Constant, ConstantData, Label, LabelUse, Reloc};

/// A buffer of output to be produced, fixed up, and then emitted to a CodeSink
/// in bulk.
///
/// This struct uses `SmallVec`s to support small-ish function bodies without
/// any heap allocation. As such, it will be several kilobytes large. This is
/// likely fine as long as it is stack-allocated for function emission then
/// thrown away; but beware if many buffer objects are retained persistently.
#[derive(Default)]
pub struct CodeBuffer {
    data: SmallVec<[u8; 1024]>,
    relocs: SmallVec<[AsmReloc; 16]>,
    label_offsets: SmallVec<[CodeOffset; 16]>,
    pending_fixup_records: SmallVec<[AsmFixup; 16]>,
    pending_fixup_deadline: u32,
    pending_constants: SmallVec<[Constant; 16]>,
    pending_constants_size: CodeOffset,
    used_constants: SmallVec<[(Constant, CodeOffset); 4]>,
    constants: Vec<(ConstantData, AsmConstant)>,
    fixup_records: BinaryHeap<AsmFixup>,
}

impl CodeBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.relocs.clear();
        self.label_offsets.clear();
        self.pending_fixup_records.clear();
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

    pub fn get_label(&mut self) -> Label {
        let l = self.label_offsets.len();
        self.label_offsets.push(u32::MAX);
        Label(l as _)
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

    pub fn cur_offset(&self) -> CodeOffset {
        self.data.len() as _
    }

    pub fn bind_label(&mut self, label: Label) {
        self.label_offsets[label.0 as usize] = self.cur_offset();
    }

    pub fn label_offset(&self, label: Label) -> u32 {
        self.label_offsets[label.0 as usize]
    }

    pub fn finish_fixups(&mut self) {
        for fixup in core::mem::take(&mut self.pending_fixup_records) {
            self.handle_fixup(fixup);
        }
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

        let buffer = &mut self.data[start as usize..end];

        let label_offset = self.label_offsets[label.0 as usize];
        if label_offset != u32::MAX {
            let veneer_required = if label_offset >= offset {
                false
            } else {
                (offset - label_offset) > kind.max_neg_range()
            };

            if veneer_required {
                self.emit_veneer(label, offset, kind);
            } else {
                kind.patch(buffer, start, label_offset);
            }
        } else {
            // If the offset of this label is not known at this time then
            // that means that a veneer is required because after this
            // island the target can't be in range of the original target.
            self.emit_veneer(label, offset, kind);
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExternalName {
    Symbol(Cow<'static, str>),
    UserName(u32),
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum RelocTarget {
    ExternalName(ExternalName),
}

/// A relocation resulting from emitting assembly.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct AsmReloc {
    pub offset: CodeOffset,
    pub kind: Reloc,
    pub addend: i64,
    pub target: RelocTarget,
}
/// A fixup to perform on the buffer once code is emitted.
/// Fixups always refer to labels and patch the code based on label offsets.
/// Hence, they are like relocations, but internal to one buffer.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct AsmFixup {
    pub label: Label,
    pub offset: CodeOffset,
    pub kind: LabelUse,
}

impl CodeBuffer {
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

    /// Emit all pending constants and required pending veneers.
    pub fn emit_island(&mut self, distance: CodeOffset) {
        let forced_threshold = self.worst_case_end_of_island(distance);

        for constant in std::mem::take(&mut self.pending_constants) {
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
        for fixup in std::mem::take(&mut self.pending_fixup_records) {
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
            alignment,
        }
    }
}

/// Metadata about a constant.
#[derive(Debug, Clone, Copy)]
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

    pub fn relocs(&self) -> &[AsmReloc] {
        &self.relocs[..]
    }

    pub fn alignment(&self) -> u32 {
        self.alignment
    }
}

/// A generic implementation to resolve relocations. Should work for most of the need.
pub fn perform_relocations(
    code: *mut u8,
    relocs: &[AsmReloc],
    get_address: impl Fn(&RelocTarget) -> *const u8,
    _get_got_entry: impl Fn(&RelocTarget) -> *const u8,
    _get_plt_entry: impl Fn(&RelocTarget) -> *const u8,
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
                let pcrel = i32::try_from((what as isize) - (at as isize)).unwrap();

                unsafe {
                    write_unaligned(at as *mut i32, pcrel);
                }
            }

            Reloc::RiscvAbs8 => unsafe {
               
                let buffer = ::core::slice::from_raw_parts_mut(at, 8 * 4);
                riscv::Assembler::patch_li(buffer, None, get_address(target) as _).unwrap();
            },

            _ => todo!(),
        }
    }
}
