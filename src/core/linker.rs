use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::fmt;

use smallvec::SmallVec;

use crate::AsmError;
use crate::core::buffer::{
    AsmReloc, CodeBufferFinalized, CodeOffset, ExternalName, Reloc, RelocTarget, SymData,
    relocation_patch_size,
};
use crate::core::operand::{Label, Sym};
use crate::core::patch::{PatchBlock, PatchCatalog, PatchSite};
use crate::core::section::FinalizedSection;

/// Links finalized sections and buffers into one in-memory image.
///
/// Sections are concatenated in insertion order, each starting at a multiple of
/// its alignment. Symbols exported with
/// [`CodeBuffer::bind_symbol`](crate::core::buffer::CodeBuffer::bind_symbol) are
/// resolved against the final layout: a relocation in any module that references
/// a defined name is rebound to the definition, so a symbol defined in module A
/// can be called from module B. Remaining undefined symbols stay external and are
/// resolved at load time, e.g. by name with
/// [`CodeBufferFinalized::allocate_resolved`](crate::core::buffer::CodeBufferFinalized::allocate_resolved).
///
/// This is deliberately not an ELF, COFF, or Mach-O linker: it emits no file
/// headers, section table, or loader metadata. Section names are diagnostic
/// only, and the linked result is one flat allocation; permissions are not
/// preserved per section. The result is a regular [`CodeBufferFinalized`], so
/// the usual loading machinery (`allocate`, `allocate_relocated`,
/// `allocate_resolved`) applies.
pub struct Linker {
    sections: Vec<FinalizedSection>,
}

/// Context for an in-memory image-link failure.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LinkError {
    IncompatibleArch {
        first_section: Cow<'static, str>,
        section: Cow<'static, str>,
    },
    DuplicateSymbol {
        name: Cow<'static, str>,
        first_section: Cow<'static, str>,
        section: Cow<'static, str>,
    },
    UnboundSymbol {
        name: Cow<'static, str>,
        section: Cow<'static, str>,
    },
    InvalidRelocation {
        section: Cow<'static, str>,
        offset: CodeOffset,
        kind: Reloc,
        target: &'static str,
        id: u32,
        reason: &'static str,
    },
}

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompatibleArch {
                first_section,
                section,
            } => write!(
                f,
                "section {section:?} has an incompatible target (first section: {first_section:?})"
            ),
            Self::DuplicateSymbol {
                name,
                first_section,
                section,
            } => write!(
                f,
                "symbol {name:?} is defined by both {first_section:?} and {section:?}"
            ),
            Self::UnboundSymbol { name, section } => {
                write!(
                    f,
                    "symbol {name:?} in section {section:?} is bound to no label"
                )
            }
            Self::InvalidRelocation {
                section,
                offset,
                kind,
                target,
                id,
                reason,
            } => write!(
                f,
                "relocation {kind:?} at {offset} in section {section:?} has invalid {target} target {id}: {reason}"
            ),
        }
    }
}

impl Linker {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }

    /// Adds a finalized section to the link.
    pub fn add_section(&mut self, section: FinalizedSection) {
        self.sections.push(section);
    }

    /// Adds a finalized buffer as a `.text` section aligned to the buffer's
    /// own alignment.
    pub fn add_buffer(&mut self, code: CodeBufferFinalized) {
        self.sections.push(FinalizedSection {
            name: Cow::Borrowed(".text"),
            align: code.alignment,
            code,
        });
    }

    /// Links all added sections into one image.
    ///
    /// Fails with [`AsmError::NoCodeGenerated`] when no sections were added or
    /// [`AsmError::Link`] with the relevant section, symbol, or relocation.
    pub fn link(self) -> Result<CodeBufferFinalized, AsmError> {
        if self.sections.is_empty() {
            return Err(AsmError::NoCodeGenerated);
        }
        let arch = self.sections[0].code.patch_catalog.arch();
        if let Some(section) = self
            .sections
            .iter()
            .find(|section| section.code.patch_catalog.arch() != arch)
        {
            return Err(AsmError::Link(LinkError::IncompatibleArch {
                first_section: self.sections[0].name.clone(),
                section: section.name.clone(),
            }));
        }

        // 1. Layout: assign each section a base offset.
        let mut bases = Vec::with_capacity(self.sections.len());
        let mut offset: CodeOffset = 0;
        let mut alignment = 1u32;
        for section in &self.sections {
            offset = align_up(offset, section.align)?;
            bases.push(offset);
            let section_size =
                CodeOffset::try_from(section.code.data.len()).map_err(|_| AsmError::TooLarge)?;
            offset = offset.checked_add(section_size).ok_or(AsmError::TooLarge)?;
            alignment = alignment.max(section.align).max(section.code.alignment);
        }
        let total_size = offset;

        // 2. Collect defined symbols (name -> global offset) in link order.
        let mut defined: Vec<(Cow<'static, str>, CodeOffset, Cow<'static, str>)> = Vec::new();
        for (section, &base) in self.sections.iter().zip(&bases) {
            for (name, local_offset) in &section.code.defined_symbols {
                let local_offset = *local_offset;
                if local_offset == u32::MAX {
                    return Err(AsmError::Link(LinkError::UnboundSymbol {
                        name: name.clone(),
                        section: section.name.clone(),
                    }));
                }
                if let Some((_, _, first_section)) = defined
                    .iter()
                    .find(|(defined_name, _, _)| defined_name == name)
                {
                    return Err(AsmError::Link(LinkError::DuplicateSymbol {
                        name: name.clone(),
                        first_section: first_section.clone(),
                        section: section.name.clone(),
                    }));
                }
                defined.push((
                    name.clone(),
                    base.checked_add(local_offset).ok_or(AsmError::TooLarge)?,
                    section.name.clone(),
                ));
            }
        }

        // 3. Merge label spaces: each section's labels rebased, then one
        // synthetic label per defined symbol. Relocations against defined
        // symbols are rewritten to target these labels, which the loading
        // path resolves internally.
        let mut label_offsets: SmallVec<[CodeOffset; 16]> = SmallVec::new();
        for (section, &base) in self.sections.iter().zip(&bases) {
            for &local_offset in &section.code.label_offsets {
                label_offsets.push(if local_offset == u32::MAX {
                    u32::MAX
                } else {
                    base.checked_add(local_offset).ok_or(AsmError::TooLarge)?
                });
            }
        }
        let defined_label_base =
            u32::try_from(label_offsets.len()).map_err(|_| AsmError::TooLarge)?;
        for (_, global_offset, _) in &defined {
            label_offsets.push(*global_offset);
        }

        // 4. Merge external symbol tables, deduplicating by name so GOT slots
        // are shared across modules.
        let mut symbols: SmallVec<[SymData; 16]> = SmallVec::new();
        let mut sym_maps: Vec<SmallVec<[u32; 16]>> = Vec::with_capacity(self.sections.len());
        for section in &self.sections {
            let mut map: SmallVec<[u32; 16]> = SmallVec::new();
            for sym in &section.code.symbols {
                let id = match symbols.iter().position(|merged| merged.name == sym.name) {
                    Some(index) => index as u32,
                    None => {
                        symbols.push(sym.clone());
                        (symbols.len() - 1) as u32
                    }
                };
                map.push(id);
            }
            sym_maps.push(map);
        }

        // 5. Concatenate data and rebase relocations.
        let mut data: SmallVec<[u8; 1024]> = SmallVec::new();
        data.resize(total_size as usize, 0);
        let mut relocs: SmallVec<[AsmReloc; 16]> = SmallVec::new();
        let mut label_base: u32 = 0;
        for (section_index, (section, &base)) in self.sections.iter().zip(&bases).enumerate() {
            let start = base as usize;
            let end = start
                .checked_add(section.code.data.len())
                .ok_or(AsmError::TooLarge)?;
            data.get_mut(start..end)
                .ok_or(AsmError::InvalidState)?
                .copy_from_slice(&section.code.data);

            for reloc in &section.code.relocs {
                let (target, target_id) = match &reloc.target {
                    RelocTarget::Label(label) => ("label", label.id()),
                    RelocTarget::Sym(sym) => ("symbol", sym.id()),
                };
                let invalid_reloc = |reason| {
                    AsmError::Link(LinkError::InvalidRelocation {
                        section: section.name.clone(),
                        offset: reloc.offset,
                        kind: reloc.kind,
                        target,
                        id: target_id,
                        reason,
                    })
                };
                let patch_size = relocation_patch_size(reloc.kind)
                    .map_err(|_| invalid_reloc("unsupported relocation kind"))?;
                let patch_end = (reloc.offset as usize)
                    .checked_add(patch_size)
                    .ok_or_else(|| invalid_reloc("patch range overflows"))?;
                if patch_end > section.code.data.len() {
                    return Err(invalid_reloc("patch range is outside the section"));
                }
                let target = match &reloc.target {
                    RelocTarget::Label(label) => {
                        if section
                            .code
                            .label_offsets
                            .get(label.id() as usize)
                            .is_none()
                        {
                            return Err(invalid_reloc("label id is outside the section"));
                        }
                        RelocTarget::Label(Label::from_id(
                            label_base
                                .checked_add(label.id())
                                .ok_or(AsmError::TooLarge)?,
                        ))
                    }
                    RelocTarget::Sym(sym) => {
                        let name = &section
                            .code
                            .symbols
                            .get(sym.id() as usize)
                            .ok_or_else(|| invalid_reloc("symbol id is outside the section"))?
                            .name;
                        let defined_index = match name {
                            ExternalName::Symbol(symbol_name) => defined
                                .iter()
                                .position(|(defined_name, _, _)| defined_name == symbol_name),
                            ExternalName::UserName(_) => None,
                        };
                        match defined_index {
                            // Defined in this link: bind to the synthetic label.
                            Some(index) => RelocTarget::Label(Label::from_id(
                                defined_label_base + index as u32,
                            )),
                            // Still external: remap to the merged symbol table.
                            None => RelocTarget::Sym(Sym::from_id(
                                *sym_maps[section_index].get(sym.id() as usize).ok_or_else(
                                    || invalid_reloc("symbol id is outside the section"),
                                )?,
                            )),
                        }
                    }
                };
                relocs.push(AsmReloc {
                    offset: base.checked_add(reloc.offset).ok_or(AsmError::TooLarge)?,
                    kind: reloc.kind,
                    addend: reloc.addend,
                    target,
                });
            }

            label_base = label_base
                .checked_add(
                    u32::try_from(section.code.label_offsets.len())
                        .map_err(|_| AsmError::TooLarge)?,
                )
                .ok_or(AsmError::TooLarge)?;
        }

        // 6. Merge patch catalogs, rebasing offsets.
        let mut blocks: SmallVec<[PatchBlock; 4]> = SmallVec::new();
        let mut sites: SmallVec<[PatchSite; 8]> = SmallVec::new();
        for (section, &base) in self.sections.iter().zip(&bases) {
            let catalog = &section.code.patch_catalog;
            for block in catalog.blocks() {
                blocks.push(PatchBlock {
                    offset: base.checked_add(block.offset).ok_or(AsmError::TooLarge)?,
                    ..*block
                });
            }
            for site in catalog.sites() {
                sites.push(PatchSite {
                    offset: base.checked_add(site.offset).ok_or(AsmError::TooLarge)?,
                    current_target: base
                        .checked_add(site.current_target)
                        .ok_or(AsmError::TooLarge)?,
                    ..*site
                });
            }
        }

        Ok(CodeBufferFinalized {
            data,
            relocs,
            symbols,
            label_offsets,
            defined_symbols: defined
                .into_iter()
                .map(|(name, offset, _)| (name, offset))
                .collect(),
            alignment,
            patch_catalog: PatchCatalog::with_parts(arch, blocks, sites),
        })
    }
}

impl Default for Linker {
    fn default() -> Self {
        Self::new()
    }
}

fn align_up(offset: CodeOffset, align: u32) -> Result<CodeOffset, AsmError> {
    if !align.is_power_of_two() {
        return Err(AsmError::InvalidArgument);
    }
    offset
        .checked_add(align - 1)
        .map(|offset| offset & !(align - 1))
        .ok_or(AsmError::TooLarge)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::arch_traits::Arch;
    use crate::core::buffer::{CodeBuffer, Reloc, RelocDistance};
    use crate::core::section::Section;
    use crate::core::target::Environment;

    fn defined_label(buf: &mut CodeBuffer, name: &'static str) -> Label {
        let label = buf.get_label();
        buf.bind_label(label);
        buf.bind_symbol(name, label);
        label
    }

    #[test]
    fn section_layout_respects_alignment() {
        let mut text = Section::new(".text", 16).unwrap();
        let text_entry = defined_label(text.buffer_mut(), "entry");
        text.buffer_mut().write_u8(0xC3);
        assert_eq!(text.buffer().label_offset(text_entry), 0);

        let mut data = Section::new(".data", 16).unwrap();
        let data_sym = defined_label(data.buffer_mut(), "data_sym");
        data.buffer_mut().write_u8(0xAA);
        assert_eq!(data.buffer().label_offset(data_sym), 0);

        let mut linker = Linker::new();
        linker.add_section(text.finish().unwrap());
        linker.add_section(data.finish().unwrap());

        let image = linker.link().unwrap();
        // .text occupies [0, 1), .data is aligned up to 16.
        assert_eq!(image.total_size(), 17);
        // The image alignment also covers each buffer's constant alignment
        // (32 by default).
        assert_eq!(image.alignment(), 32);
        assert_eq!(image.defined_symbol_offset("entry"), Some(0));
        assert_eq!(image.defined_symbol_offset("data_sym"), Some(16));
        assert_eq!(image.data()[0], 0xC3);
        assert_eq!(image.data()[16], 0xAA);
    }

    #[test]
    fn cross_module_symbol_resolution() {
        // Module A defines "callee".
        let mut a = CodeBuffer::new(Environment::new(Arch::X64));
        defined_label(&mut a, "callee");
        a.write_u8(0xC3); // ret

        // Module B references "callee" as an undefined external with an
        // absolute 8-byte relocation, and keeps "missing" unresolved.
        let mut b = CodeBuffer::new(Environment::new(Arch::X64));
        let callee = b.extern_sym("callee", RelocDistance::Far);
        let missing = b.extern_sym("missing", RelocDistance::Far);
        b.add_reloc(Reloc::Abs8, RelocTarget::Sym(callee), 0);
        b.write_u64(0);
        b.add_reloc(Reloc::Abs8, RelocTarget::Sym(missing), 0);
        b.write_u64(0);

        let mut linker = Linker::new();
        linker.add_buffer(a.finish().unwrap());
        linker.add_buffer(b.finish().unwrap());

        let image = linker.link().unwrap();
        assert_eq!(image.defined_symbol_offset("callee"), Some(0));
        assert_eq!(image.relocs().len(), 2);

        // The resolved reference targets a label at the definition offset...
        match &image.relocs()[0].target {
            RelocTarget::Label(label) => {
                assert_eq!(image.label_offsets[label.id() as usize], 0);
            }
            target => panic!("expected label target, got {target:?}"),
        }
        // ...while the undefined symbol stays external and both references to
        // the same name share one symbol entry.
        match &image.relocs()[1].target {
            RelocTarget::Sym(sym) => {
                assert_eq!(
                    image.symbol_name(*sym),
                    Some(&crate::core::buffer::ExternalName::Symbol("missing".into()))
                );
            }
            target => panic!("expected symbol target, got {target:?}"),
        }
    }

    #[test]
    fn label_relocs_are_rebased_to_the_section_base() {
        // Second module has an absolute reference to its own label; after
        // linking the reference must point into the merged image.
        let mut a = CodeBuffer::new(Environment::new(Arch::X64));
        a.write_u64(0);

        let mut b = CodeBuffer::new(Environment::new(Arch::X64));
        let target = b.get_label();
        b.add_reloc(Reloc::Abs8, RelocTarget::Label(target), 0);
        b.write_u64(0);
        b.bind_label(target);

        let mut linker = Linker::new();
        linker.add_buffer(a.finish().unwrap());
        linker.add_buffer(b.finish().unwrap());

        let image = linker.link().unwrap();
        match &image.relocs()[0].target {
            RelocTarget::Label(label) => {
                // `add_buffer` aligns each module to its buffer alignment (32
                // by default), so b starts at offset 32 and its label sits 8
                // bytes into b.
                assert_eq!(image.label_offsets[label.id() as usize], 32 + 8);
            }
            target => panic!("expected label target, got {target:?}"),
        }
    }

    #[test]
    fn duplicate_definition_is_an_error() {
        let mut a = CodeBuffer::new(Environment::new(Arch::X64));
        defined_label(&mut a, "dup");
        let mut b = CodeBuffer::new(Environment::new(Arch::X64));
        defined_label(&mut b, "dup");

        let mut linker = Linker::new();
        linker.add_buffer(a.finish().unwrap());
        linker.add_buffer(b.finish().unwrap());

        assert_eq!(
            linker.link().err(),
            Some(AsmError::Link(LinkError::DuplicateSymbol {
                name: "dup".into(),
                first_section: ".text".into(),
                section: ".text".into(),
            }))
        );
    }

    #[test]
    fn empty_link_is_an_error() {
        assert_eq!(Linker::new().link().err(), Some(AsmError::NoCodeGenerated));
    }

    #[test]
    fn mixed_target_sections_are_rejected() {
        let x64 = Section::with_env(".x64", 1, Environment::new(Arch::X64))
            .unwrap()
            .finish()
            .unwrap();
        let aarch64 = Section::with_env(".a64", 4, Environment::new(Arch::AArch64))
            .unwrap()
            .finish()
            .unwrap();
        let mut linker = Linker::new();
        linker.add_section(x64);
        linker.add_section(aarch64);

        assert_eq!(
            linker.link().err(),
            Some(AsmError::Link(LinkError::IncompatibleArch {
                first_section: ".x64".into(),
                section: ".a64".into(),
            }))
        );
    }

    #[test]
    fn defined_symbol_bound_to_unbound_label_is_an_error() {
        let mut a = CodeBuffer::new(Environment::new(Arch::X64));
        let label = a.get_label();
        a.bind_symbol("dangling", label);
        a.write_u8(0xC3);

        assert_eq!(a.finish().err(), Some(AsmError::UnboundLabel));
    }

    #[test]
    fn unbound_symbol_reports_its_name_and_section() {
        let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
        buffer.write_u8(0xC3);
        let mut code = buffer.finish().unwrap();
        code.defined_symbols.push(("dangling".into(), u32::MAX));

        let mut linker = Linker::new();
        linker.add_section(FinalizedSection {
            name: Cow::Borrowed(".text"),
            align: 1,
            code,
        });

        assert_eq!(
            linker.link().err(),
            Some(AsmError::Link(LinkError::UnboundSymbol {
                name: "dangling".into(),
                section: ".text".into(),
            }))
        );
    }

    #[test]
    fn invalid_relocation_reports_its_section_and_target() {
        let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
        buffer.write_u32(0);
        let mut code = buffer.finish().unwrap();
        code.relocs.push(AsmReloc {
            offset: 0,
            kind: Reloc::Abs4,
            addend: 0,
            target: RelocTarget::Label(Label::from_id(7)),
        });

        let mut linker = Linker::new();
        linker.add_section(FinalizedSection {
            name: Cow::Borrowed(".bad"),
            align: 1,
            code,
        });

        assert_eq!(
            linker.link().err(),
            Some(AsmError::Link(LinkError::InvalidRelocation {
                section: ".bad".into(),
                offset: 0,
                kind: Reloc::Abs4,
                target: "label",
                id: 7,
                reason: "label id is outside the section",
            }))
        );
    }

    #[test]
    fn out_of_section_relocation_is_rejected_before_linking() {
        let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
        let label = buffer.get_label();
        buffer.bind_label(label);
        buffer.write_u32(0);
        let mut code = buffer.finish().unwrap();
        code.relocs.push(AsmReloc {
            offset: 1,
            kind: Reloc::Abs4,
            addend: 0,
            target: RelocTarget::Label(label),
        });

        let mut linker = Linker::new();
        linker.add_section(FinalizedSection {
            name: Cow::Borrowed(".bad"),
            align: 1,
            code,
        });

        assert_eq!(
            linker.link().err(),
            Some(AsmError::Link(LinkError::InvalidRelocation {
                section: ".bad".into(),
                offset: 1,
                kind: Reloc::Abs4,
                target: "label",
                id: label.id(),
                reason: "patch range is outside the section",
            }))
        );
    }

    #[cfg(all(feature = "jit", target_arch = "x86_64"))]
    #[test]
    fn got_based_extern_call_across_modules() {
        use crate::core::jit_allocator::JitAllocator;

        // Module A: `mov eax, 42; ret` exported as "callee".
        let mut a = CodeBuffer::new(Environment::new(Arch::X64));
        defined_label(&mut a, "callee");
        a.write_u8(0xB8);
        a.write_u32(42);
        a.write_u8(0xC3);

        // Module B: `call qword ptr [rip + GOT("callee")]; ret` exported as
        // "main", referencing "callee" as an undefined external.
        let mut b = CodeBuffer::new(Environment::new(Arch::X64));
        defined_label(&mut b, "main");
        let callee = b.extern_sym("callee", RelocDistance::Far);
        b.write_u8(0xFF);
        b.write_u8(0x15);
        b.add_reloc(Reloc::X86GOTPCRel4, RelocTarget::Sym(callee), -4);
        b.write_u32(0);
        b.write_u8(0xC3);

        let mut linker = Linker::new();
        linker.add_buffer(a.finish().unwrap());
        linker.add_buffer(b.finish().unwrap());
        let image = linker.link().unwrap();

        let entry = image.defined_symbol_offset("main").unwrap();
        let mut jit = JitAllocator::new(Default::default());
        let loaded = image
            .allocate_resolved(&mut jit, |name| {
                panic!("unexpected undefined symbol: {name}")
            })
            .unwrap();

        // One GOT slot for "callee", pointing at its definition inside the image.
        assert_eq!(loaded.got_targets().len(), 1);
        unsafe {
            let got_entry = core::ptr::read_unaligned(loaded.got_rx() as *const usize);
            assert_eq!(got_entry, loaded.rx() as usize);

            let main: extern "C" fn() -> u32 =
                core::mem::transmute(loaded.rx().add(entry as usize));
            assert_eq!(main(), 42);
        }
    }
}
