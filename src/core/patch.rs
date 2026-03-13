use smallvec::SmallVec;

use crate::{
    AsmError,
    core::{
        arch_traits::Arch,
        buffer::{CodeBufferFinalized, CodeOffset, LabelUse},
    },
};

#[cfg(feature = "jit")]
use crate::jit_allocator::{JitAllocator, Span};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PatchBlockId(u32);

impl PatchBlockId {
    pub(crate) const fn from_index(index: usize) -> Self {
        Self(index as u32)
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PatchSiteId(u32);

impl PatchSiteId {
    pub(crate) const fn from_index(index: usize) -> Self {
        Self(index as u32)
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatchBlock {
    pub offset: CodeOffset,
    pub size: CodeOffset,
    pub align: CodeOffset,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatchSite {
    pub offset: CodeOffset,
    pub kind: LabelUse,
    pub current_target: CodeOffset,
    pub addend: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatchCatalog {
    arch: Arch,
    blocks: SmallVec<[PatchBlock; 4]>,
    sites: SmallVec<[PatchSite; 8]>,
}

impl PatchCatalog {
    pub(crate) fn with_parts(
        arch: Arch,
        blocks: SmallVec<[PatchBlock; 4]>,
        sites: SmallVec<[PatchSite; 8]>,
    ) -> Self {
        Self {
            arch,
            blocks,
            sites,
        }
    }

    pub fn arch(&self) -> Arch {
        self.arch
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty() && self.sites.is_empty()
    }

    pub fn blocks(&self) -> &[PatchBlock] {
        &self.blocks
    }

    pub fn sites(&self) -> &[PatchSite] {
        &self.sites
    }

    pub fn block(&self, id: PatchBlockId) -> Option<&PatchBlock> {
        self.blocks.get(id.index())
    }

    pub fn site(&self, id: PatchSiteId) -> Option<&PatchSite> {
        self.sites.get(id.index())
    }

    pub fn site_mut(&mut self, id: PatchSiteId) -> Option<&mut PatchSite> {
        self.sites.get_mut(id.index())
    }
}

pub fn minimum_patch_alignment(arch: Arch) -> CodeOffset {
    match arch {
        Arch::AArch64 | Arch::AArch64BE | Arch::RISCV32 | Arch::RISCV64 => 4,
        _ => 1,
    }
}

pub fn fill_with_nops(arch: Arch, buffer: &mut [u8]) -> Result<(), AsmError> {
    let pattern: &[u8] = match arch {
        Arch::X86 | Arch::X64 => &[0x90],
        Arch::AArch64 => &[0x1f, 0x20, 0x03, 0xd5],
        Arch::AArch64BE => &[0xd5, 0x03, 0x20, 0x1f],
        Arch::RISCV32 | Arch::RISCV64 => &[0x13, 0x00, 0x00, 0x00],
        _ => return Err(AsmError::InvalidArgument),
    };

    if pattern.len() > 1 && !buffer.len().is_multiple_of(pattern.len()) {
        return Err(AsmError::InvalidArgument);
    }

    for chunk in buffer.chunks_mut(pattern.len()) {
        chunk.copy_from_slice(pattern);
    }

    Ok(())
}

#[cfg(feature = "jit")]
pub struct LoadedPatchableCode {
    catalog: PatchCatalog,
    span: Span,
}

#[cfg(feature = "jit")]
impl LoadedPatchableCode {
    pub(crate) fn new(span: Span, catalog: PatchCatalog) -> Self {
        Self { catalog, span }
    }

    pub fn patch_catalog(&self) -> &PatchCatalog {
        &self.catalog
    }

    pub const fn rx(&self) -> *const u8 {
        self.span.rx()
    }

    pub const fn rw(&self) -> *mut u8 {
        self.span.rw()
    }

    pub const fn span(&self) -> &Span {
        &self.span
    }

    pub fn retarget_site(
        &mut self,
        jit_allocator: &mut JitAllocator,
        id: PatchSiteId,
        target_offset: CodeOffset,
    ) -> Result<(), AsmError> {
        let site = *self.catalog.site(id).ok_or(AsmError::InvalidArgument)?;
        if !site.kind.can_reach(site.offset, target_offset) {
            return Err(AsmError::TooLarge);
        }

        unsafe {
            jit_allocator.write(&mut self.span, |span| {
                let patch_size = site.kind.patch_size();
                let patch_ptr = span.rw().wrapping_add(site.offset as usize);
                let patch_slice = core::slice::from_raw_parts_mut(patch_ptr, patch_size);
                site.kind
                    .patch_with_addend(patch_slice, site.offset, target_offset, site.addend);
            })?;
        }

        self.catalog.site_mut(id).unwrap().current_target = target_offset;
        Ok(())
    }

    pub fn rewrite_block(
        &mut self,
        jit_allocator: &mut JitAllocator,
        id: PatchBlockId,
        bytes: &[u8],
    ) -> Result<(), AsmError> {
        let block = *self.catalog.block(id).ok_or(AsmError::InvalidArgument)?;
        if bytes.len() > block.size as usize {
            return Err(AsmError::TooLarge);
        }

        unsafe {
            jit_allocator.write(&mut self.span, |span| {
                let block_ptr = span.rw().wrapping_add(block.offset as usize);
                block_ptr.copy_from_nonoverlapping(bytes.as_ptr(), bytes.len());
                let tail = core::slice::from_raw_parts_mut(
                    block_ptr.wrapping_add(bytes.len()),
                    block.size as usize - bytes.len(),
                );
                fill_with_nops(self.catalog.arch(), tail).expect("validated patch block size");
            })?;
        }

        Ok(())
    }
}

impl CodeBufferFinalized {
    pub fn patch_catalog(&self) -> &PatchCatalog {
        &self.patch_catalog
    }

    #[cfg(feature = "jit")]
    pub fn allocate_patched(
        &self,
        jit_allocator: &mut JitAllocator,
    ) -> Result<LoadedPatchableCode, AsmError> {
        let span = self.allocate(jit_allocator)?;
        Ok(LoadedPatchableCode::new(span, self.patch_catalog.clone()))
    }
}
