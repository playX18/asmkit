//! Post-finalization code patching.
//!
//! # Usage
//!
//! 1. While emitting, the arch `patchable_*` helpers and
//!    [`CodeBuffer::reserve_patch_region`](crate::CodeBuffer::reserve_patch_region)
//!    return marks: [`PatchableJump`], [`DataLabel`] and [`PatchableRegion`].
//! 2. After [`CodeBuffer::finish`](crate::CodeBuffer::finish), turn a mark into a
//!    location in the image with [`CodeBufferFinalized::location_of`] (or
//!    [`CodeBufferFinalized::location_in`] for a linked image).
//! 3. Patch the image with [`repatch_jump`], [`repatch_value`] and
//!    [`rewrite_region`], or loaded code with their `_span` variants.

use smallvec::SmallVec;

use crate::{
    AsmError,
    core::{
        arch_traits::Arch,
        buffer::{CodeBufferFinalized, CodeOffset, LabelUse},
        relax::OffsetMap,
    },
};

#[cfg(feature = "jit")]
use crate::core::jit_allocator::{JitAllocator, Span};

/// A jump or call emitted with a fixed-size displacement whose target can be
/// changed after finalization.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PatchableJump {
    offset: CodeOffset,
    kind: LabelUse,
}

impl PatchableJump {
    pub(crate) const fn new(offset: CodeOffset, kind: LabelUse) -> Self {
        Self { offset, kind }
    }

    /// The mark returned when emission fails. Its location is out of bounds
    /// in every image, so patching through it fails.
    pub(crate) const fn invalid(kind: LabelUse) -> Self {
        Self::new(u32::MAX, kind)
    }
}

/// A patchable immediate: a fixed-size `mov` immediate on x86, a
/// `movz`/`movk` sequence on AArch64, or a literal on RISC-V.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DataLabel {
    offset: CodeOffset,
    size: u8,
    encoding: DataEncoding,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum DataEncoding {
    /// `size` little-endian bytes.
    #[cfg_attr(not(any(feature = "x86", feature = "riscv")), allow(dead_code))]
    Raw,
    /// `size / 4` AArch64 instructions: `movz` then `movk` for each further
    /// 16-bit chunk.
    #[cfg_attr(not(feature = "aarch64"), allow(dead_code))]
    A64MovWide,
}

impl DataLabel {
    pub(crate) const fn new(offset: CodeOffset, size: u8, encoding: DataEncoding) -> Self {
        Self {
            offset,
            size,
            encoding,
        }
    }

    pub(crate) const fn invalid(size: u8, encoding: DataEncoding) -> Self {
        Self::new(u32::MAX, size, encoding)
    }

    /// Size in bytes of the patched field.
    pub const fn size(self) -> u8 {
        self.size
    }
}

/// A nop-filled code region reserved for later rewriting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PatchableRegion {
    offset: CodeOffset,
    size: CodeOffset,
    arch: Arch,
}

impl PatchableRegion {
    pub(crate) const fn new(offset: CodeOffset, size: CodeOffset, arch: Arch) -> Self {
        Self { offset, size, arch }
    }

    pub const fn size(self) -> CodeOffset {
        self.size
    }
}

/// Where a [`PatchableJump`] is in a finished image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CodeLocationJump {
    offset: CodeOffset,
    kind: LabelUse,
}

impl CodeLocationJump {
    /// Offset of the displacement field.
    pub const fn offset(self) -> CodeOffset {
        self.offset
    }

    pub const fn kind(self) -> LabelUse {
        self.kind
    }
}

/// Where a [`DataLabel`] is in a finished image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CodeLocationData {
    offset: CodeOffset,
    size: u8,
    encoding: DataEncoding,
}

impl CodeLocationData {
    /// Offset of the first byte of the field.
    pub const fn offset(self) -> CodeOffset {
        self.offset
    }

    pub const fn size(self) -> u8 {
        self.size
    }
}

/// Where a [`PatchableRegion`] is in a finished image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CodeLocationRegion {
    offset: CodeOffset,
    size: CodeOffset,
    arch: Arch,
}

impl CodeLocationRegion {
    pub const fn offset(self) -> CodeOffset {
        self.offset
    }

    pub const fn size(self) -> CodeOffset {
        self.size
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::PatchableJump {}
    impl Sealed for super::DataLabel {}
    impl Sealed for super::PatchableRegion {}
}

/// A mark that [`CodeBufferFinalized::location_of`] turns into a location.
pub trait PatchMark: Copy + sealed::Sealed {
    type Location: Copy;

    #[doc(hidden)]
    fn locate(self, place: impl Fn(CodeOffset) -> CodeOffset) -> Self::Location;
}

/// Moves a buffer offset through branch relaxation and to its section,
/// keeping the out-of-bounds offset of an invalid mark out of bounds.
fn place(base: CodeOffset, layout: &OffsetMap, offset: CodeOffset) -> CodeOffset {
    if offset == u32::MAX {
        return offset;
    }
    base.saturating_add(layout.map(offset))
}

impl PatchMark for PatchableJump {
    type Location = CodeLocationJump;

    fn locate(self, place: impl Fn(CodeOffset) -> CodeOffset) -> CodeLocationJump {
        CodeLocationJump {
            offset: place(self.offset),
            kind: self.kind,
        }
    }
}

impl PatchMark for DataLabel {
    type Location = CodeLocationData;

    fn locate(self, place: impl Fn(CodeOffset) -> CodeOffset) -> CodeLocationData {
        CodeLocationData {
            offset: place(self.offset),
            size: self.size,
            encoding: self.encoding,
        }
    }
}

impl PatchMark for PatchableRegion {
    type Location = CodeLocationRegion;

    fn locate(self, place: impl Fn(CodeOffset) -> CodeOffset) -> CodeLocationRegion {
        CodeLocationRegion {
            offset: place(self.offset),
            size: self.size,
            arch: self.arch,
        }
    }
}

impl CodeBufferFinalized {
    /// Returns where `mark` is in this image.
    ///
    /// `mark` must come from the buffer this image was finished from. For an
    /// image made by [`Linker::link`](crate::Linker::link), use
    /// [`Self::location_in`].
    pub fn location_of<M: PatchMark>(&self, mark: M) -> M::Location {
        debug_assert_eq!(
            self.section_bases.len(),
            1,
            "linked images need `location_in`"
        );
        mark.locate(|offset| place(self.section_bases[0], &self.layout_maps[0], offset))
    }

    /// Returns where `mark`, from the buffer added as `section` to a
    /// [`Linker`](crate::Linker), is in this image. Returns `None` when there
    /// is no such section.
    pub fn location_in<M: PatchMark>(&self, section: usize, mark: M) -> Option<M::Location> {
        let base = *self.section_bases.get(section)?;
        let layout = self.layout_maps.get(section)?;
        Some(mark.locate(|offset| place(base, layout, offset)))
    }
}

fn field(len: usize, offset: CodeOffset, size: usize) -> Result<core::ops::Range<usize>, AsmError> {
    let start = offset as usize;
    let end = start.checked_add(size).ok_or(AsmError::InvalidState)?;
    if end > len {
        return Err(AsmError::InvalidState);
    }
    Ok(start..end)
}

fn relink(bytes: &mut [u8], kind: LabelUse, delta: i64) -> Result<(), AsmError> {
    if !kind.can_reach_delta(delta) {
        return Err(AsmError::TooLarge);
    }
    kind.patch_with_addend(bytes, 0, 0, delta);
    Ok(())
}

/// Points `jump` at `target`, an offset in the same image.
pub fn repatch_jump(
    code: &mut [u8],
    jump: CodeLocationJump,
    target: CodeOffset,
) -> Result<(), AsmError> {
    let range = field(code.len(), jump.offset, jump.kind.patch_size())?;
    let delta = i64::from(target) - i64::from(jump.offset);
    relink(&mut code[range], jump.kind, delta)
}

/// Points `jump` in loaded code at the absolute address `target`.
///
/// # Safety
///
/// `span` must hold the image `jump` was located in, and no thread may execute
/// the jump while it is rewritten.
#[cfg(feature = "jit")]
pub unsafe fn repatch_jump_span(
    jit_allocator: &mut JitAllocator,
    span: &mut Span,
    jump: CodeLocationJump,
    target: *const u8,
) -> Result<(), AsmError> {
    let range = field(span.size(), jump.offset, jump.kind.patch_size())?;
    let at = span.rx().addr() + range.start;
    let delta =
        i64::try_from(target.addr() as i128 - at as i128).map_err(|_| AsmError::TooLarge)?;
    if !jump.kind.can_reach_delta(delta) {
        return Err(AsmError::TooLarge);
    }
    // SAFETY: `range` is inside the span, and the caller guarantees the span
    // holds this jump and that nothing executes it concurrently.
    unsafe {
        jit_allocator.write(span, |span| {
            let bytes = core::slice::from_raw_parts_mut(span.rw().add(range.start), range.len());
            jump.kind.patch_with_addend(bytes, 0, 0, delta);
        })
    }
}

fn encode_value(
    current: &[u8],
    data: CodeLocationData,
    value: u64,
) -> Result<SmallVec<[u8; 16]>, AsmError> {
    let size = data.size as usize;
    match data.encoding {
        DataEncoding::Raw => {
            if size < 8 && value >> (size * 8) != 0 {
                return Err(AsmError::TooLarge);
            }
            Ok(SmallVec::from_slice(&value.to_le_bytes()[..size]))
        }
        DataEncoding::A64MovWide => {
            #[cfg(feature = "aarch64")]
            {
                // Re-encode for the register and width the sequence was
                // emitted with, read back from the `movz`.
                let movz = u32::from_le_bytes([current[0], current[1], current[2], current[3]]);
                let is_64bit = movz >> 31 == 1;
                if !is_64bit && value > u64::from(u32::MAX) {
                    return Err(AsmError::TooLarge);
                }
                Ok(crate::aarch64::encode_patchable_mov_imm(
                    movz & 0x1f,
                    is_64bit,
                    value,
                ))
            }
            #[cfg(not(feature = "aarch64"))]
            {
                let _ = (current, value);
                Err(AsmError::InvalidArch)
            }
        }
    }
}

fn decode_value(bytes: &[u8], data: CodeLocationData) -> u64 {
    match data.encoding {
        DataEncoding::Raw => {
            let mut value = [0u8; 8];
            value[..bytes.len()].copy_from_slice(bytes);
            u64::from_le_bytes(value)
        }
        DataEncoding::A64MovWide => bytes.chunks_exact(4).fold(0, |value, insn| {
            let insn = u32::from_le_bytes([insn[0], insn[1], insn[2], insn[3]]);
            let imm16 = u64::from((insn >> 5) & 0xffff);
            let hw = (insn >> 21) & 3;
            value | (imm16 << (16 * hw))
        }),
    }
}

/// Points `jump` in loaded code at the absolute address `target` while other
/// threads may execute it: one aligned 4-byte store plus a targeted icache
/// flush (based on `NativeCall::set_destination_mt_safe` in HotSpot). Only 4-byte
/// displacement fields are supported.
///
/// Fails with [`AsmError::TooLarge`] when `target` is out of range and with
/// [`AsmError::UnalignedPatch`] when the field is not 4 byte aligned.
///
/// # Safety
///
/// `span` must hold the image `jump` was located in. Patching must be
/// serialized by the caller (no two threads patch at once), executing
/// threads run free and observe either the old or the new target, never a
/// invalid displacement.
#[cfg(feature = "jit")]
pub unsafe fn repatch_jump_span_mt_safe(
    span: &Span,
    jump: CodeLocationJump,
    target: *const u8,
) -> Result<(), AsmError> {
    use core::sync::atomic::{AtomicU32, Ordering};

    use crate::util::virtual_memory::{flush_instruction_cache, with_jit_write_access};

    if jump.kind.patch_size() != 4 {
        return Err(AsmError::InvalidArgument);
    }
    let range = field(span.size(), jump.offset, 4)?;
    let at = span.rx().addr() + range.start;
    if at % 4 != 0 {
        return Err(AsmError::UnalignedPatch);
    }
    let delta =
        i64::try_from(target.addr() as i128 - at as i128).map_err(|_| AsmError::TooLarge)?;
    if !jump.kind.can_reach_delta(delta) {
        return Err(AsmError::TooLarge);
    }
    // Seed a scratch word with the current bytes (fixed-opcode targets like
    // AArch64 branches preserve their opcode bits across the immediate),
    // encode into it exactly as the image patcher would, then commit it
    // with one atomic store.
    // SAFETY: `range` is inside the live span. The
    // read races safely with executors (they never write), the caller
    // serializes patching threads.
    let mut word = [0u8; 4];
    unsafe { word.copy_from_slice(core::slice::from_raw_parts(span.rx().add(range.start), 4)) };
    jump.kind.patch_with_addend(&mut word, 0, 0, delta);
    let disp = u32::from_le_bytes(word);
    // SAFETY: as above
    unsafe {
        with_jit_write_access(|| {
            AtomicU32::from_ptr(span.rw().add(range.start) as *mut u32)
                .store(disp, Ordering::Release);
        });
        flush_instruction_cache(span.rx().add(range.start), 4)?;
    }
    Ok(())
}

/// Writes `value` into the immediate at `data`. Fails with [`AsmError::TooLarge`] when
/// `value` does not fit the field.
pub fn repatch_value(code: &mut [u8], data: CodeLocationData, value: u64) -> Result<(), AsmError> {
    let range = field(code.len(), data.offset, data.size as usize)?;
    let encoded = encode_value(&code[range.clone()], data, value)?;
    code[range].copy_from_slice(&encoded);
    Ok(())
}

/// Reads back the immediate at `data`.
pub fn read_value(code: &[u8], data: CodeLocationData) -> Result<u64, AsmError> {
    let range = field(code.len(), data.offset, data.size as usize)?;
    Ok(decode_value(&code[range], data))
}

/// Writes `value` into the immediate at `data` in loaded code.
///
/// # Safety
///
/// `span` must hold the image `data` was located in, and no thread may execute
/// the instruction while it is rewritten.
#[cfg(feature = "jit")]
pub unsafe fn repatch_value_span(
    jit_allocator: &mut JitAllocator,
    span: &mut Span,
    data: CodeLocationData,
    value: u64,
) -> Result<(), AsmError> {
    let range = field(span.size(), data.offset, data.size as usize)?;
    // SAFETY: `range` is inside the span, which the caller guarantees is live.
    let current = unsafe { core::slice::from_raw_parts(span.rx().add(range.start), range.len()) };
    let encoded = encode_value(current, data, value)?;
    // SAFETY: as above, and nothing executes the range while it is written.
    unsafe {
        jit_allocator.write(span, |span| {
            span.rw()
                .add(range.start)
                .copy_from_nonoverlapping(encoded.as_ptr(), encoded.len());
        })
    }
}

/// Reads back the immediate at `data` in loaded code.
///
/// # Safety
///
/// `span` must be a live allocation holding the image `data` was located in.
#[cfg(feature = "jit")]
pub unsafe fn read_value_span(span: &Span, data: CodeLocationData) -> Result<u64, AsmError> {
    let range = field(span.size(), data.offset, data.size as usize)?;
    // SAFETY: `range` is inside the span, which the caller guarantees is live.
    let bytes = unsafe { core::slice::from_raw_parts(span.rx().add(range.start), range.len()) };
    Ok(decode_value(bytes, data))
}

fn check_region_payload(region: CodeLocationRegion, new_bytes: &[u8]) -> Result<(), AsmError> {
    if new_bytes.len() > region.size as usize {
        return Err(AsmError::TooLarge);
    }
    if new_bytes.len() % minimum_patch_alignment(region.arch) as usize != 0 {
        return Err(AsmError::InvalidArgument);
    }
    Ok(())
}

/// Overwrites `region` with `new_bytes`, padding the rest with nops.
pub fn rewrite_region(
    code: &mut [u8],
    region: CodeLocationRegion,
    new_bytes: &[u8],
) -> Result<(), AsmError> {
    check_region_payload(region, new_bytes)?;
    let range = field(code.len(), region.offset, region.size as usize)?;
    let block = &mut code[range];
    block[..new_bytes.len()].copy_from_slice(new_bytes);
    fill_with_nops(region.arch, &mut block[new_bytes.len()..])
}

/// Overwrites `region` in loaded code with `new_bytes`, padding the rest with
/// nops.
///
/// # Safety
///
/// `span` must hold the image `region` was located in, and no thread may
/// execute the region while it is rewritten.
#[cfg(feature = "jit")]
pub unsafe fn rewrite_region_span(
    jit_allocator: &mut JitAllocator,
    span: &mut Span,
    region: CodeLocationRegion,
    new_bytes: &[u8],
) -> Result<(), AsmError> {
    check_region_payload(region, new_bytes)?;
    let range = field(span.size(), region.offset, region.size as usize)?;
    let mut result = Ok(());
    // SAFETY: `range` is inside the span, and the caller guarantees the span
    // holds this region and that nothing executes it concurrently.
    unsafe {
        jit_allocator.write(span, |span| {
            let block = core::slice::from_raw_parts_mut(span.rw().add(range.start), range.len());
            block[..new_bytes.len()].copy_from_slice(new_bytes);
            result = fill_with_nops(region.arch, &mut block[new_bytes.len()..]);
        })?;
    }
    result
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
