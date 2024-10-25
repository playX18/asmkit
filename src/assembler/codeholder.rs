use alloc::{borrow::Cow, boxed::Box};

use smallvec::SmallVec;

use super::binemit::{CodeOffset, Label, LabelUse, Reloc};

/// A buffer of output to be produced, fixed up, and then emitted to a CodeSink
/// in bulk.
///
/// This struct uses `SmallVec`s to support small-ish function bodies without
/// any heap allocation. As such, it will be several kilobytes large. This is
/// likely fine as long as it is stack-allocated for function emission then
/// thrown away; but beware if many buffer objects are retained persistently.
pub struct CodeBuffer {
    data: SmallVec<[u8; 1024]>,
    relocs: SmallVec<[AsmReloc; 16]>,
    label_offsets: SmallVec<[CodeOffset; 16]>,
    pending_fixup_records: SmallVec<[AsmFixup; 16]>,
}

impl CodeBuffer {
    pub fn new() -> Box<Self> {
        let this = Box::new(CodeBuffer {
            data: SmallVec::new(),
            relocs: SmallVec::new(),
            label_offsets: SmallVec::new(),
            pending_fixup_records: SmallVec::new(),
        });

        this
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

    pub fn use_label_at_offset(&mut self, offset: CodeOffset, label: Label, kind: LabelUse) {
        let fixup = AsmFixup {
            kind,
            label,
            offset,
        };

        self.pending_fixup_records.push(fixup);
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

    pub fn handle_fixups(&mut self) {
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

        kind.patch(buffer, start, label_offset);
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
    pub fn finish(mut self) -> CodeBufferFinalized {
        self.handle_fixups();

        CodeBufferFinalized {
            data: self.data,
            relocs: self.relocs,
        }
    }
}
/// A `CodeBuffer` once emission is completed: holds generated code and records,
/// without fixups. This allows the type to be independent of the backend.
pub struct CodeBufferFinalized {
    pub(crate) data: SmallVec<[u8; 1024]>,
    pub(crate) relocs: SmallVec<[AsmReloc; 16]>,
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

            _ => todo!(),
        }
    }
}
