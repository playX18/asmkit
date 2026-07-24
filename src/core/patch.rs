//! Post-emit code patching (JSC MacroAssembler–style).
//!
//! # Workflow
//!
//! 1. During emission, call arch `patchable_*` helpers (or
//!    [`CodeBuffer::reserve_patch_block`](crate::CodeBuffer::reserve_patch_block)).
//!    These return self-describing [`PatchableSite`] / [`PatchableBlock`] handles.
//! 2. Finalize with [`CodeBuffer::finish_patched`](crate::CodeBuffer::finish_patched)
//!    so the [`PatchCatalog`] validates reachability (and the linker can rebase it).
//! 3. Apply patches with **`unsafe`** methods on a JIT [`Span`] or `&mut [u8]`.
//!    Patching does **not** go through [`CodeBufferFinalized`].
//!
//! # JSC correspondence
//!
//! | JSC | asmkit |
//! |---|---|
//! | `PatchableJump` / near call | [`PatchableSite`] + [`PatchableSite::retarget`] |
//! | `DataLabel32` / `DataLabelPtr` | [`PatchableBlock`] + [`PatchableBlock::repatch_u32`] / [`repatch_u64`](PatchableBlock::repatch_u64) |
//! | `padBeforePatch` + custom stub | [`reserve_patch_block`](crate::CodeBuffer::reserve_patch_block) → [`PatchableBlock::rewrite`] |
//! | `repatchJump` on a code pointer | `unsafe` apply on [`Span`] / `&mut [u8]` |
//!
//! # Site vs block vs custom region
//!
//! - **Site** — a fixed-width displacement field (`LabelUse`) for jumps/calls; retarget by
//!   code offset within the same image.
//! - **Block** — a reserved byte range (immediate field or whole insn sequence); rewrite or
//!   repatch integer payloads; shorter rewrites are nop-padded.
//! - **Custom** — [`reserve_patch_block`](crate::CodeBuffer::reserve_patch_block) plants a
//!   nop island you fill later with [`PatchableBlock::rewrite`].
//!
//! # Safety
//!
//! Handles from `patchable_*` describe locations in the buffer that produced them. Applying a
//! handle is still `unsafe`: the bytes/`Span` must be that image (after finalize/link with
//! stable offsets), and the caller must synchronize concurrent execution of the patched region.
//! [`PatchableSite::new`] / [`PatchableBlock::new`] are `unsafe` for the same reason when
//! constructing handles by hand.
//!
//! [`PatchCatalog`] remains for `finish_patched` validation and linker rebasing. After rebase,
//! convert entries with [`PatchSite::to_patchable`] / [`PatchBlock::to_patchable`].

use smallvec::SmallVec;

use crate::{
    AsmError,
    core::{
        arch_traits::Arch,
        buffer::{CodeBufferFinalized, CodeOffset, LabelUse},
    },
};

#[cfg(feature = "jit")]
use crate::core::jit_allocator::{JitAllocator, Span};

/// Catalog index for a patch block (linker / introspection).
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

/// Catalog index for a patch site (linker / introspection).
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

/// Catalog entry for a rewritable byte range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatchBlock {
    pub offset: CodeOffset,
    pub size: CodeOffset,
    pub align: CodeOffset,
}

impl PatchBlock {
    /// Build a patch handle from a (possibly rebased) catalog entry.
    pub const fn to_patchable(self, arch: Arch) -> PatchableBlock {
        // SAFETY: catalog entries describe blocks recorded during emission.
        unsafe { PatchableBlock::new(self.offset, self.size, arch) }
    }
}

/// Catalog entry for a displacement field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatchSite {
    pub offset: CodeOffset,
    pub kind: LabelUse,
    pub current_target: CodeOffset,
    pub addend: i64,
}

impl PatchSite {
    /// Build a patch handle from a (possibly rebased) catalog entry.
    pub const fn to_patchable(self) -> PatchableSite {
        // SAFETY: catalog entries describe sites recorded during emission.
        unsafe { PatchableSite::new(self.offset, self.kind, self.addend) }
    }
}

/// Finalized patch metadata for validation and linker rebasing.
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

/// Self-describing handle for a patchable displacement (jump/call).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PatchableSite {
    offset: CodeOffset,
    kind: LabelUse,
    addend: i64,
}

impl PatchableSite {
    /// Construct a site handle manually.
    ///
    /// # Safety
    ///
    /// `offset` in any image you later patch must be a valid displacement field of `kind`
    /// with the same layout as when the site was emitted or recorded.
    pub const unsafe fn new(offset: CodeOffset, kind: LabelUse, addend: i64) -> Self {
        Self {
            offset,
            kind,
            addend,
        }
    }

    pub const fn offset(self) -> CodeOffset {
        self.offset
    }

    pub const fn kind(self) -> LabelUse {
        self.kind
    }

    pub const fn addend(self) -> i64 {
        self.addend
    }

    /// Retarget this site in a mutable code image.
    ///
    /// # Safety
    ///
    /// `bytes` must be the code image this site was recorded against (same layout). The caller
    /// synchronizes concurrent execution of the patched region.
    pub unsafe fn retarget(
        self,
        bytes: &mut [u8],
        target_offset: CodeOffset,
    ) -> Result<(), AsmError> {
        if !self.kind.can_reach(self.offset, target_offset) {
            return Err(AsmError::TooLarge);
        }
        let patch_size = self.kind.patch_size();
        let patch_end = (self.offset as usize)
            .checked_add(patch_size)
            .ok_or(AsmError::InvalidState)?;
        if patch_end > bytes.len() {
            return Err(AsmError::InvalidState);
        }
        let patch_slice = &mut bytes[self.offset as usize..patch_end];
        self.kind
            .patch_with_addend(patch_slice, self.offset, target_offset, self.addend);
        Ok(())
    }

    /// Retarget this site in executable memory.
    ///
    /// # Safety
    ///
    /// `span` must be the loaded image this site was recorded against. The caller synchronizes
    /// concurrent execution of the patched region.
    #[cfg(feature = "jit")]
    pub unsafe fn retarget_span(
        self,
        jit_allocator: &mut JitAllocator,
        span: &mut Span,
        target_offset: CodeOffset,
    ) -> Result<(), AsmError> {
        if !self.kind.can_reach(self.offset, target_offset) {
            return Err(AsmError::TooLarge);
        }
        let patch_size = self.kind.patch_size();
        let patch_end = (self.offset as usize)
            .checked_add(patch_size)
            .ok_or(AsmError::InvalidState)?;
        if patch_end > span.size() {
            return Err(AsmError::InvalidState);
        }

        unsafe {
            jit_allocator.write(span, |span| {
                let patch_ptr = span.rw().add(self.offset as usize);
                let patch_slice = core::slice::from_raw_parts_mut(patch_ptr, patch_size);
                self.kind.patch_with_addend(
                    patch_slice,
                    self.offset,
                    target_offset,
                    self.addend,
                );
            })?;
        }
        Ok(())
    }
}

/// Self-describing handle for a rewritable code region (immediate or custom block).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PatchableBlock {
    offset: CodeOffset,
    size: CodeOffset,
    arch: Arch,
}

impl PatchableBlock {
    /// Construct a block handle manually.
    ///
    /// # Safety
    ///
    /// `offset..offset+size` in any image you later patch must be a reserved patch region for
    /// `arch` (instruction alignment and nop-fill rules apply).
    pub const unsafe fn new(offset: CodeOffset, size: CodeOffset, arch: Arch) -> Self {
        Self {
            offset,
            size,
            arch,
        }
    }

    pub const fn offset(self) -> CodeOffset {
        self.offset
    }

    pub const fn size(self) -> CodeOffset {
        self.size
    }

    pub const fn arch(self) -> Arch {
        self.arch
    }

    /// Overwrite this block; shorter payloads are padded with architecture nops.
    ///
    /// # Safety
    ///
    /// `bytes` must be the code image this block was recorded against. The caller synchronizes
    /// concurrent execution of the patched region.
    pub unsafe fn rewrite(self, bytes: &mut [u8], new_bytes: &[u8]) -> Result<(), AsmError> {
        if new_bytes.len() > self.size as usize {
            return Err(AsmError::TooLarge);
        }
        let instruction_alignment = minimum_patch_alignment(self.arch) as usize;
        if new_bytes.len() % instruction_alignment != 0 {
            return Err(AsmError::InvalidArgument);
        }
        let block_end = (self.offset as usize)
            .checked_add(self.size as usize)
            .ok_or(AsmError::InvalidState)?;
        if block_end > bytes.len() {
            return Err(AsmError::InvalidState);
        }

        let block = &mut bytes[self.offset as usize..block_end];
        block[..new_bytes.len()].copy_from_slice(new_bytes);
        fill_with_nops(self.arch, &mut block[new_bytes.len()..])?;
        Ok(())
    }

    /// Write a little-endian `u32` into a 4-byte block (x86 `patchable_mov` on `Gp32`, etc.).
    ///
    /// # Safety
    ///
    /// See [`rewrite`](Self::rewrite).
    pub unsafe fn repatch_u32(self, bytes: &mut [u8], value: u32) -> Result<(), AsmError> {
        if self.size != 4 {
            return Err(AsmError::InvalidArgument);
        }
        unsafe { self.rewrite(bytes, &value.to_le_bytes()) }
    }

    /// Write a little-endian `u64` into an 8-byte block (x86 `patchable_mov` on `Gp64`, etc.).
    ///
    /// # Safety
    ///
    /// See [`rewrite`](Self::rewrite).
    pub unsafe fn repatch_u64(self, bytes: &mut [u8], value: u64) -> Result<(), AsmError> {
        if self.size != 8 {
            return Err(AsmError::InvalidArgument);
        }
        unsafe { self.rewrite(bytes, &value.to_le_bytes()) }
    }

    /// Overwrite this block in executable memory.
    ///
    /// # Safety
    ///
    /// `span` must be the loaded image this block was recorded against.
    #[cfg(feature = "jit")]
    pub unsafe fn rewrite_span(
        self,
        jit_allocator: &mut JitAllocator,
        span: &mut Span,
        new_bytes: &[u8],
    ) -> Result<(), AsmError> {
        if new_bytes.len() > self.size as usize {
            return Err(AsmError::TooLarge);
        }
        let instruction_alignment = minimum_patch_alignment(self.arch) as usize;
        if new_bytes.len() % instruction_alignment != 0 {
            return Err(AsmError::InvalidArgument);
        }
        let block_end = (self.offset as usize)
            .checked_add(self.size as usize)
            .ok_or(AsmError::InvalidState)?;
        if block_end > span.size() {
            return Err(AsmError::InvalidState);
        }

        let mut fill_result = Ok(());
        unsafe {
            jit_allocator.write(span, |span| {
                let block_ptr = span.rw().add(self.offset as usize);
                block_ptr.copy_from_nonoverlapping(new_bytes.as_ptr(), new_bytes.len());
                let tail = core::slice::from_raw_parts_mut(
                    block_ptr.add(new_bytes.len()),
                    self.size as usize - new_bytes.len(),
                );
                fill_result = fill_with_nops(self.arch, tail);
            })?;
        }
        fill_result
    }

    /// Write a little-endian `u32` into a 4-byte block in executable memory.
    ///
    /// # Safety
    ///
    /// See [`rewrite_span`](Self::rewrite_span).
    #[cfg(feature = "jit")]
    pub unsafe fn repatch_u32_span(
        self,
        jit_allocator: &mut JitAllocator,
        span: &mut Span,
        value: u32,
    ) -> Result<(), AsmError> {
        if self.size != 4 {
            return Err(AsmError::InvalidArgument);
        }
        unsafe { self.rewrite_span(jit_allocator, span, &value.to_le_bytes()) }
    }

    /// Write a little-endian `u64` into an 8-byte block in executable memory.
    ///
    /// # Safety
    ///
    /// See [`rewrite_span`](Self::rewrite_span).
    #[cfg(feature = "jit")]
    pub unsafe fn repatch_u64_span(
        self,
        jit_allocator: &mut JitAllocator,
        span: &mut Span,
        value: u64,
    ) -> Result<(), AsmError> {
        if self.size != 8 {
            return Err(AsmError::InvalidArgument);
        }
        unsafe { self.rewrite_span(jit_allocator, span, &value.to_le_bytes()) }
    }
}

pub fn minimum_patch_alignment(arch: Arch) -> CodeOffset {
    match arch {
        Arch::AArch64 | Arch::RISCV32 | Arch::RISCV64 => 4,
        _ => 1,
    }
}

pub fn fill_with_nops(arch: Arch, buffer: &mut [u8]) -> Result<(), AsmError> {
    let pattern: &[u8] = match arch {
        Arch::X86 | Arch::X64 => &[0x90],
        Arch::AArch64 => &[0x1f, 0x20, 0x03, 0xd5],
        Arch::RISCV32 | Arch::RISCV64 => &[0x13, 0x00, 0x00, 0x00],
        _ => return Err(AsmError::InvalidArgument),
    };

    if pattern.len() > 1 && buffer.len() % pattern.len() != 0 {
        return Err(AsmError::InvalidArgument);
    }

    for chunk in buffer.chunks_mut(pattern.len()) {
        chunk.copy_from_slice(pattern);
    }

    Ok(())
}

impl CodeBufferFinalized {
    pub fn patch_catalog(&self) -> &PatchCatalog {
        &self.patch_catalog
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retarget_rejects_out_of_range_slice() {
        let site = unsafe { PatchableSite::new(62, LabelUse::X86JmpRel32, 0) };
        let mut bytes = [0u8; 64];
        assert_eq!(
            unsafe { site.retarget(&mut bytes, 0) }.unwrap_err(),
            AsmError::InvalidState
        );
    }

    #[test]
    fn rewrite_rejects_misaligned_payload_for_a64() {
        let block = unsafe { PatchableBlock::new(0, 4, Arch::AArch64) };
        let mut bytes = [0u8; 4];
        assert_eq!(
            unsafe { block.rewrite(&mut bytes, &[0]) }.unwrap_err(),
            AsmError::InvalidArgument
        );
    }

    #[test]
    fn repatch_u32_round_trips() {
        let block = unsafe { PatchableBlock::new(1, 4, Arch::X64) };
        let mut bytes = [0xB8, 0, 0, 0, 0];
        unsafe { block.repatch_u32(&mut bytes, 0x11223344).unwrap() };
        assert_eq!(&bytes[1..], &[0x44, 0x33, 0x22, 0x11]);
    }

    #[cfg(feature = "jit")]
    #[test]
    fn span_patch_rejects_ranges_outside_span() {
        use crate::core::jit_allocator::JitAllocatorOptions;

        let mut allocator = JitAllocator::new(JitAllocatorOptions::default());
        let mut span = allocator.alloc(64).unwrap();
        let span_size = span.size() as CodeOffset;

        let block = unsafe { PatchableBlock::new(span_size, 1, Arch::X64) };
        assert_eq!(
            unsafe { block.rewrite_span(&mut allocator, &mut span, &[0x90]) }.unwrap_err(),
            AsmError::InvalidState
        );

        let site = unsafe { PatchableSite::new(span_size - 3, LabelUse::X86JmpRel32, 0) };
        assert_eq!(
            unsafe { site.retarget_span(&mut allocator, &mut span, 0) }.unwrap_err(),
            AsmError::InvalidState
        );
    }
}
