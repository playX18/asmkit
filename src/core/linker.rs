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
use crate::core::section::FinalizedSection;

/// Links finalized sections and buffers into one in-memory image.
///
/// Sections are concatenated in insertion order, each starting at a multiple of
/// its alignment. Symbols exported with
/// [`CodeBuffer::bind_symbol`](crate::core::buffer::CodeBuffer::bind_symbol) are
/// resolved against the final layout: a relocation in any module that references
/// a defined [`ExternalName`] is rebound to the definition, so a symbol defined
/// in module A can be called from module B. Remaining undefined symbols stay
/// external and are resolved at load time with
/// [`CodeBufferFinalized::load`](crate::core::buffer::CodeBufferFinalized::load).
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
        name: ExternalName,
        first_section: Cow<'static, str>,
        section: Cow<'static, str>,
    },
    UnboundSymbol {
        name: ExternalName,
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
                "symbol {name} is defined by both {first_section:?} and {section:?}"
            ),
            Self::UnboundSymbol { name, section } => {
                write!(
                    f,
                    "symbol {name} in section {section:?} is bound to no label"
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

impl core::error::Error for LinkError {}

impl Linker {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }

    /// Adds a finalized section to the link. Returns the index that
    /// [`CodeBufferFinalized::location_in`] takes for marks from this
    /// section's buffer.
    pub fn add_section(&mut self, section: FinalizedSection) -> usize {
        let index = self.layout_count();
        self.sections.push(section);
        index
    }

    /// Adds a finalized buffer as a `.text` section aligned to the buffer's
    /// own alignment. Returns the index that
    /// [`CodeBufferFinalized::location_in`] takes for marks from `code`.
    pub fn add_buffer(&mut self, code: CodeBufferFinalized) -> usize {
        self.add_section(FinalizedSection {
            name: Cow::Borrowed(".text"),
            align: code.alignment,
            code,
        })
    }

    /// Number of sections added so far. A linked image added again brings one
    /// per section it was linked from.
    fn layout_count(&self) -> usize {
        self.sections
            .iter()
            .map(|section| section.code.section_bases.len())
            .sum()
    }

    /// Links all added sections into one image.
    ///
    /// Fails with [`AsmError::NoCodeGenerated`] when no sections were added or
    /// [`AsmError::Link`] with the relevant section, symbol, or relocation.
    pub fn link(self) -> Result<CodeBufferFinalized, AsmError> {
        if self.sections.is_empty() {
            return Err(AsmError::NoCodeGenerated);
        }
        let arch = self.sections[0].code.arch;
        if let Some(section) = self
            .sections
            .iter()
            .find(|section| section.code.arch != arch)
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
        let mut defined: Vec<(ExternalName, CodeOffset, Cow<'static, str>)> = Vec::new();
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
                        let defined_index = defined
                            .iter()
                            .position(|(defined_name, _, _)| defined_name == name);
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

        // 6. Record where each input section starts, so marks from its buffer
        // can still be located.
        let mut section_bases: SmallVec<[CodeOffset; 1]> = SmallVec::new();
        let mut layout_maps = SmallVec::new();
        for (section, &base) in self.sections.iter().zip(&bases) {
            for &inner in &section.code.section_bases {
                section_bases.push(base.checked_add(inner).ok_or(AsmError::TooLarge)?);
            }
            layout_maps.extend(section.code.layout_maps.iter().cloned());
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
            arch,
            section_bases,
            layout_maps,
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
