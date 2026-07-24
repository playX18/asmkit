use alloc::{borrow::Cow, collections::BinaryHeap, vec::Vec};
use core::fmt;

use smallvec::SmallVec;

use crate::AsmError;
use crate::core::arch_traits::Arch;
use crate::core::patch::{
    PatchBlock, PatchBlockId, PatchCatalog, PatchSite, PatchSiteId, PatchableBlock,
    fill_with_nops, minimum_patch_alignment,
};
#[cfg(feature = "riscv")]
use crate::riscv;

#[cfg(feature = "jit")]
use crate::core::jit_allocator::{JitAllocator, Span};

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
pub struct CodeBuffer {
    env: Environment,
    data: SmallVec<[u8; 1024]>,
    relocs: SmallVec<[AsmReloc; 16]>,
    symbols: SmallVec<[SymData; 16]>,
    defined_symbols: SmallVec<[(ExternalName, Label); 4]>,
    label_offsets: SmallVec<[CodeOffset; 16]>,
    pending_fixup_records: SmallVec<[AsmFixup; 16]>,
    pending_fixup_deadline: u32,
    pending_constants: SmallVec<[Constant; 16]>,
    pending_constants_size: CodeOffset,
    used_constants: SmallVec<[(Constant, CodeOffset); 4]>,
    constants: SmallVec<[(ConstantData, AsmConstant); 4]>,
    fixup_records: BinaryHeap<AsmFixup>,
    patch_blocks: SmallVec<[PendingPatchBlock; 4]>,
    patch_sites: SmallVec<[PendingPatchSite; 8]>,
    #[cfg(feature = "x86")]
    x86_branch_relaxations: SmallVec<[X86BranchRelaxation; 8]>,
    #[cfg(feature = "x86")]
    alignment_constraints: SmallVec<[(CodeOffset, CodeOffset); 4]>,
    error: Option<AsmError>,
}

/// Private rollback point for one raw instruction emission.
///
/// Raw backend emission may append bytes and metadata, and x86 may rewrite
/// bytes appended by the same attempt. It must not mutate metadata or bytes
/// that predate this checkpoint; island emission is a finalization operation.
#[derive(Clone, Copy)]
#[cfg(any(feature = "x86", feature = "aarch64", feature = "riscv"))]
pub(crate) struct EmissionCheckpoint {
    data_len: usize,
    relocs_len: usize,
    symbols_len: usize,
    defined_symbols_len: usize,
    label_offsets_len: usize,
    pending_fixup_records_len: usize,
    pending_fixup_deadline: u32,
    pending_constants_len: usize,
    pending_constants_size: CodeOffset,
    used_constants_len: usize,
    constants_len: usize,
    patch_blocks_len: usize,
    patch_sites_len: usize,
    #[cfg(feature = "x86")]
    x86_branch_relaxations_len: usize,
    #[cfg(feature = "x86")]
    alignment_constraints_len: usize,
}

#[cfg(feature = "x86")]
#[derive(Clone, Copy)]
struct X86BranchRelaxation {
    opcode_offset: CodeOffset,
    label: Label,
    short_opcode: u8,
    near_size: u8,
}

#[derive(Clone, Copy)]
struct PendingPatchBlock {
    offset: CodeOffset,
    size: CodeOffset,
    align: CodeOffset,
}

#[derive(Clone, Copy)]
enum PendingPatchTarget {
    Offset(CodeOffset),
    Label(Label),
}

#[derive(Clone, Copy)]
struct PendingPatchSite {
    offset: CodeOffset,
    kind: LabelUse,
    target: PendingPatchTarget,
    addend: i64,
}

/// An external name in a user-defined symbol table.
///
/// Cranelift-style opaque key: asmkit does not interpret `namespace` or `index`.
/// Hosts commonly use separate namespaces for functions vs data, but that
/// convention is not enforced here.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct UserExternalName {
    pub namespace: u32,
    pub index: u32,
}

impl UserExternalName {
    /// Creates a new user external name.
    pub const fn new(namespace: u32, index: u32) -> Self {
        Self { namespace, index }
    }
}

impl fmt::Display for UserExternalName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "u{}:{}", self.namespace, self.index)
    }
}

/// Name of an external (or exported) symbol.
///
/// Either a string [`Symbol`](ExternalName::Symbol) for human-readable /
/// ELF-like names, or a [`User`](ExternalName::User) namespace+index key for
/// backends that do not use string symbols.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExternalName {
    Symbol(Cow<'static, str>),
    User(UserExternalName),
}

impl ExternalName {
    /// Creates a user-defined external name from `namespace` and `index`.
    pub const fn user(namespace: u32, index: u32) -> Self {
        Self::User(UserExternalName::new(namespace, index))
    }
}

impl fmt::Display for ExternalName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbol(name) => f.write_str(name),
            Self::User(name) => fmt::Display::fmt(name, f),
        }
    }
}

impl From<UserExternalName> for ExternalName {
    fn from(name: UserExternalName) -> Self {
        Self::User(name)
    }
}

impl From<&'static str> for ExternalName {
    fn from(name: &'static str) -> Self {
        Self::Symbol(Cow::Borrowed(name))
    }
}

impl From<alloc::string::String> for ExternalName {
    fn from(name: alloc::string::String) -> Self {
        Self::Symbol(Cow::Owned(name))
    }
}

impl From<Cow<'static, str>> for ExternalName {
    fn from(name: Cow<'static, str>) -> Self {
        Self::Symbol(name)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum RelocTarget {
    Sym(Sym),
    Label(Label),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RelocDistance {
    Near,
    Far,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct SymData {
    pub(crate) name: ExternalName,
    pub(crate) distance: RelocDistance,
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
pub(crate) struct AsmFixup {
    pub label: Label,
    pub offset: CodeOffset,
    pub kind: LabelUse,
}

impl AsmFixup {
    fn deadline(&self) -> CodeOffset {
        self.offset.saturating_sub(self.kind.max_pos_range())
    }
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
    pub(crate) label_offsets: SmallVec<[CodeOffset; 16]>,
    pub(crate) defined_symbols: SmallVec<[(ExternalName, CodeOffset); 4]>,
    pub(crate) alignment: u32,
    pub(crate) patch_catalog: PatchCatalog,
}

/// Executable memory loaded from a finalized code buffer with relocations applied.
#[cfg(feature = "jit")]
pub struct LoadedRelocatedCode {
    span: Span,
    code_size: usize,
    got_targets: Vec<RelocTarget>,
}

#[cfg(feature = "jit")]
impl LoadedRelocatedCode {
    pub const fn rx(&self) -> *const u8 {
        self.span.rx()
    }

    pub const fn rw(&self) -> *mut u8 {
        self.span.rw()
    }

    pub const fn span(&self) -> &Span {
        &self.span
    }

    pub const fn code_size(&self) -> usize {
        self.code_size
    }

    pub fn got_targets(&self) -> &[RelocTarget] {
        &self.got_targets
    }

    pub fn got_size(&self) -> usize {
        self.got_targets.len() * core::mem::size_of::<usize>()
    }

    pub fn got_rx(&self) -> *const u8 {
        self.rx().wrapping_add(self.code_size)
    }

    pub fn got_rw(&self) -> *mut u8 {
        self.rw().wrapping_add(self.code_size)
    }
}

pub fn reloc_uses_got(kind: Reloc) -> bool {
    matches!(
        kind,
        Reloc::X86GOTPCRel4
            | Reloc::RiscvGotHi20
            | Reloc::RiscvPCRelLo12I
            | Reloc::Aarch64AdrGotPage21
            | Reloc::Aarch64Ld64GotLo12Nc
    )
}

#[cfg(feature = "jit")]
pub(crate) fn got_slot_index(got_targets: &[RelocTarget], target: &RelocTarget) -> Option<usize> {
    got_targets.iter().position(|item| item == target)
}

impl CodeBufferFinalized {
    pub fn total_size(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn symbol_name(&self, sym: Sym) -> Option<&ExternalName> {
        self.symbols
            .get(sym.id() as usize)
            .map(|symbol| &symbol.name)
    }

    pub fn symbol_distance(&self, sym: Sym) -> Option<RelocDistance> {
        self.symbols
            .get(sym.id() as usize)
            .map(|symbol| symbol.distance)
    }

    /// Offset of a symbol exported with [`CodeBuffer::bind_symbol`], if present.
    ///
    /// After loading, the runtime address of the symbol is `rx + offset`.
    pub fn defined_symbol_offset(&self, name: &ExternalName) -> Option<CodeOffset> {
        self.defined_symbols
            .iter()
            .find(|(defined, _)| defined == name)
            .map(|(_, offset)| *offset)
    }

    /// Like [`Self::defined_symbol_offset`], for string [`ExternalName::Symbol`] exports.
    pub fn defined_symbol_str(&self, name: &str) -> Option<CodeOffset> {
        self.defined_symbols
            .iter()
            .find(|(defined, _)| matches!(defined, ExternalName::Symbol(s) if s.as_ref() == name))
            .map(|(_, offset)| *offset)
    }

    pub fn relocs(&self) -> &[AsmReloc] {
        &self.relocs[..]
    }

    pub fn alignment(&self) -> u32 {
        self.alignment
    }

    /// Allocate this code buffer in executable memory and return a `Span` referring to it.
    /// This will also write the code into the allocated memory. To execute
    /// code you can simply use [`span.rx()`](Span::rx) to get a pointer to read+exec memory
    /// and transmute that to a function pointer of the appropriate type.
    #[cfg(feature = "jit")]
    pub fn allocate(&self, jit_allocator: &mut JitAllocator) -> Result<Span, AsmError> {
        let mut span = jit_allocator.alloc(self.data().len())?;

        unsafe {
            jit_allocator.write(&mut span, |span| {
                span.rw()
                    .copy_from_nonoverlapping(self.data().as_ptr(), self.data().len());
            })?;
        }

        Ok(span)
    }

    /// Allocate executable memory and apply relocations, including GOT setup in JIT mode.
    ///
    /// GOT entries are created automatically for relocations that require them and populated
    /// with values returned by `get_address`.
    ///
    /// Relocations targeting a [`Label`](RelocTarget::Label) are resolved internally against
    /// the label offsets recorded at finalize time; the callbacks only see external symbols.
    #[cfg(feature = "jit")]
    pub fn allocate_relocated(
        &self,
        jit_allocator: &mut JitAllocator,
        get_address: impl Fn(&RelocTarget) -> *const u8,
        get_plt_entry: impl Fn(&RelocTarget) -> *const u8,
    ) -> Result<LoadedRelocatedCode, AsmError> {
        let mut got_targets = Vec::new();

        for reloc in &self.relocs {
            if reloc_uses_got(reloc.kind) && !got_targets.iter().any(|item| item == &reloc.target) {
                got_targets.push(reloc.target.clone());
            }
        }

        let got_size = got_targets
            .len()
            .checked_mul(core::mem::size_of::<usize>())
            .ok_or(AsmError::TooLarge)?;
        let total_size = self
            .data()
            .len()
            .checked_add(got_size)
            .ok_or(AsmError::TooLarge)?;
        let mut span = jit_allocator.alloc(total_size)?;

        let mut relocation_result = Ok(());
        unsafe {
            jit_allocator.write(&mut span, |span| {
                relocation_result = (|| {
                    span.rw()
                        .copy_from_nonoverlapping(self.data().as_ptr(), self.data().len());

                    let rx = span.rx();
                    let resolve = |target: &RelocTarget,
                                   fallback: &dyn Fn(&RelocTarget) -> *const u8|
                     -> Result<*const u8, AsmError> {
                        if let RelocTarget::Label(label) = target {
                            let offset = self
                                .label_offsets
                                .get(label.id() as usize)
                                .copied()
                                .ok_or(AsmError::InvalidArgument)?;
                            if offset == u32::MAX || offset as usize > self.data().len() {
                                return Err(AsmError::UnboundLabel);
                            }
                            return Ok(rx.add(offset as usize));
                        }
                        let address = fallback(target);
                        if address.is_null() {
                            return Err(AsmError::InvalidArgument);
                        }
                        Ok(address)
                    };

                    let got_rw = span.rw().add(self.data().len()) as *mut usize;
                    for (index, target) in got_targets.iter().enumerate() {
                        let addr = resolve(target, &get_address)?;
                        got_rw.add(index).write_unaligned(addr.addr());
                    }

                    let got_rx = rx.add(self.data().len());
                    perform_relocations(
                        span.rw(),
                        rx,
                        self.data().len(),
                        &self.relocs,
                        |target| resolve(target, &get_address),
                        |target| {
                            let index = got_slot_index(&got_targets, target)
                                .ok_or(AsmError::InvalidState)?;
                            let offset = index
                                .checked_mul(core::mem::size_of::<usize>())
                                .ok_or(AsmError::TooLarge)?;
                            Ok(got_rx.add(offset))
                        },
                        |target| resolve(target, &get_plt_entry),
                    )
                })();
            })?;
        }
        relocation_result?;

        Ok(LoadedRelocatedCode {
            span,
            code_size: self.data().len(),
            got_targets,
        })
    }

    /// Allocate executable memory and apply relocations, resolving external
    /// symbols through `resolve`.
    ///
    /// This is the ergonomic counterpart of [`Self::allocate_relocated`]: every
    /// undefined external symbol (declared with [`CodeBuffer::extern_sym`],
    /// [`CodeBuffer::extern_user`], or surviving a
    /// [`Linker`](crate::core::linker::Linker) link) is passed to `resolve`,
    /// which must return its address. Symbols defined inside the image (label
    /// targets and linked-in definitions) are resolved internally.
    #[cfg(feature = "jit")]
    pub fn allocate_resolved(
        &self,
        jit_allocator: &mut JitAllocator,
        resolve: impl Fn(&ExternalName) -> *const u8,
    ) -> Result<LoadedRelocatedCode, AsmError> {
        let by_name = |target: &RelocTarget| match target {
            RelocTarget::Sym(sym) => match self.symbol_name(*sym) {
                Some(name) => resolve(name),
                None => core::ptr::null(),
            },
            // Label targets are resolved internally by `allocate_relocated`.
            RelocTarget::Label(_) => core::ptr::null(),
        };

        self.allocate_relocated(jit_allocator, by_name, by_name)
    }
}

impl CodeBuffer {
    /// Creates a buffer for `env`.
    pub fn new(env: Environment) -> Self {
        Self {
            env,
            data: SmallVec::new(),
            relocs: SmallVec::new(),
            symbols: SmallVec::new(),
            defined_symbols: SmallVec::new(),
            label_offsets: SmallVec::new(),
            pending_fixup_records: SmallVec::new(),
            pending_fixup_deadline: 0,
            pending_constants: SmallVec::new(),
            pending_constants_size: 0,
            used_constants: SmallVec::new(),
            constants: SmallVec::new(),
            fixup_records: BinaryHeap::new(),
            patch_blocks: SmallVec::new(),
            patch_sites: SmallVec::new(),
            #[cfg(feature = "x86")]
            x86_branch_relaxations: SmallVec::new(),
            #[cfg(feature = "x86")]
            alignment_constraints: SmallVec::new(),
            error: None,
        }
    }

    /// Creates a buffer for the host target.
    pub fn host() -> Self {
        Self::new(Environment::host())
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.relocs.clear();
        self.label_offsets.clear();
        self.pending_fixup_records.clear();
        self.constants.clear();
        self.fixup_records.clear();
        self.symbols.clear();
        self.defined_symbols.clear();
        self.used_constants.clear();
        self.pending_fixup_deadline = 0;
        self.pending_constants_size = 0;
        self.pending_constants.clear();
        self.patch_blocks.clear();
        self.patch_sites.clear();
        #[cfg(feature = "x86")]
        self.x86_branch_relaxations.clear();
        #[cfg(feature = "x86")]
        self.alignment_constraints.clear();
        self.error = None;
    }

    /// Returns the first error recorded by a void assembler operation.
    pub fn error(&self) -> Option<&AsmError> {
        self.error.as_ref()
    }

    pub(crate) fn record_error(&mut self, error: AsmError) {
        if self.error.is_none() {
            self.error = Some(error);
        }
    }

    #[cfg(any(feature = "x86", feature = "aarch64", feature = "riscv"))]
    pub(crate) fn checkpoint(&self) -> EmissionCheckpoint {
        EmissionCheckpoint {
            data_len: self.data.len(),
            relocs_len: self.relocs.len(),
            symbols_len: self.symbols.len(),
            defined_symbols_len: self.defined_symbols.len(),
            label_offsets_len: self.label_offsets.len(),
            pending_fixup_records_len: self.pending_fixup_records.len(),
            pending_fixup_deadline: self.pending_fixup_deadline,
            pending_constants_len: self.pending_constants.len(),
            pending_constants_size: self.pending_constants_size,
            used_constants_len: self.used_constants.len(),
            constants_len: self.constants.len(),
            patch_blocks_len: self.patch_blocks.len(),
            patch_sites_len: self.patch_sites.len(),
            #[cfg(feature = "x86")]
            x86_branch_relaxations_len: self.x86_branch_relaxations.len(),
            #[cfg(feature = "x86")]
            alignment_constraints_len: self.alignment_constraints.len(),
        }
    }

    #[cfg(any(feature = "x86", feature = "aarch64", feature = "riscv"))]
    pub(crate) fn rollback(&mut self, checkpoint: EmissionCheckpoint) {
        self.data.truncate(checkpoint.data_len);
        self.relocs.truncate(checkpoint.relocs_len);
        self.symbols.truncate(checkpoint.symbols_len);
        self.defined_symbols
            .truncate(checkpoint.defined_symbols_len);
        self.label_offsets.truncate(checkpoint.label_offsets_len);
        self.pending_fixup_records
            .truncate(checkpoint.pending_fixup_records_len);
        self.pending_fixup_deadline = checkpoint.pending_fixup_deadline;
        self.pending_constants
            .truncate(checkpoint.pending_constants_len);
        self.pending_constants_size = checkpoint.pending_constants_size;
        self.used_constants.truncate(checkpoint.used_constants_len);
        self.constants.truncate(checkpoint.constants_len);
        self.patch_blocks.truncate(checkpoint.patch_blocks_len);
        self.patch_sites.truncate(checkpoint.patch_sites_len);
        #[cfg(feature = "x86")]
        self.x86_branch_relaxations
            .truncate(checkpoint.x86_branch_relaxations_len);
        #[cfg(feature = "x86")]
        self.alignment_constraints
            .truncate(checkpoint.alignment_constraints_len);
    }
    pub fn env(&self) -> &Environment {
        &self.env
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the byte at `offset`.
    ///
    /// Used by encoder patch-up paths (e.g. X86 LEA/abs32 fixups).
    #[cfg(feature = "x86")]
    pub(crate) fn byte_at(&self, offset: CodeOffset) -> u8 {
        self.data[offset as usize]
    }

    /// Overwrites the byte at `offset`.
    #[cfg(feature = "x86")]
    pub(crate) fn set_byte_at(&mut self, offset: CodeOffset, value: u8) {
        self.data[offset as usize] = value;
    }

    /// Inserts a byte at `offset`, shifting all subsequent bytes.
    ///
    /// This is a rare encoder patch-up operation; prefer appending.
    #[cfg(feature = "x86")]
    pub(crate) fn insert_at(&mut self, offset: CodeOffset, value: u8) {
        self.data.insert(offset as usize, value);
    }

    /// Removes the byte at `offset`, shifting all subsequent bytes.
    ///
    /// This is a rare encoder patch-up operation; prefer appending.
    #[cfg(feature = "x86")]
    pub(crate) fn remove_at(&mut self, offset: CodeOffset) {
        self.data.remove(offset as usize);
    }

    #[cfg(feature = "x86")]
    pub(crate) fn record_x86_branch_relaxation(
        &mut self,
        opcode_offset: CodeOffset,
        label: Label,
        short_opcode: u8,
        near_size: u8,
    ) {
        debug_assert!(matches!(near_size, 5 | 6));
        debug_assert!((opcode_offset as usize) + near_size as usize <= self.data.len());
        debug_assert!(self.label_offsets.get(label.id() as usize).is_some());
        self.x86_branch_relaxations.push(X86BranchRelaxation {
            opcode_offset,
            label,
            short_opcode,
            near_size,
        });
    }

    pub fn relocs(&self) -> &[AsmReloc] {
        &self.relocs
    }

    pub fn put1(&mut self, value: u8) {
        if self.error.is_some() {
            return;
        }
        self.data.push(value);
    }

    pub fn put2(&mut self, value: u16) {
        if self.error.is_some() {
            return;
        }
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn put4(&mut self, value: u32) {
        if self.error.is_some() {
            return;
        }
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn put8(&mut self, value: u64) {
        if self.error.is_some() {
            return;
        }
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn write_u8(&mut self, value: u8) {
        if self.error.is_some() {
            return;
        }
        self.data.push(value);
    }

    pub fn write_u16(&mut self, value: u16) {
        if self.error.is_some() {
            return;
        }
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn write_u32(&mut self, value: u32) {
        if self.error.is_some() {
            return;
        }
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn write_u64(&mut self, value: u64) {
        if self.error.is_some() {
            return;
        }
        self.data.extend_from_slice(&value.to_ne_bytes());
    }

    pub fn add_symbol(&mut self, name: impl Into<ExternalName>, distance: RelocDistance) -> Sym {
        if self.error.is_some() {
            return Sym::new();
        }
        let ix = self.symbols.len();
        self.symbols.push(SymData {
            distance,
            name: name.into(),
        });

        Sym::from_id(ix as u32)
    }

    /// Declares an external symbol by name, deduplicating: repeated calls with
    /// the same name return the same `Sym` (the distance of the first
    /// declaration wins).
    ///
    /// The returned symbol can be passed to the architecture assemblers (e.g.
    /// `ptr64_sym` on x86) which then record the appropriate relocation based
    /// on the symbol's distance.
    pub fn extern_sym(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        distance: RelocDistance,
    ) -> Sym {
        if self.error.is_some() {
            return Sym::new();
        }
        let name = ExternalName::Symbol(name.into());
        if let Some(ix) = self.symbols.iter().position(|sym| sym.name == name) {
            return Sym::from_id(ix as u32);
        }

        self.add_symbol(name, distance)
    }

    /// Declares an external symbol by user namespace+index, deduplicating:
    /// repeated calls with the same key return the same `Sym` (the distance of
    /// the first declaration wins).
    pub fn extern_user(&mut self, namespace: u32, index: u32, distance: RelocDistance) -> Sym {
        if self.error.is_some() {
            return Sym::new();
        }
        let name = ExternalName::user(namespace, index);
        if let Some(ix) = self.symbols.iter().position(|sym| sym.name == name) {
            return Sym::from_id(ix as u32);
        }

        self.add_symbol(name, distance)
    }

    /// Exports `label` under `name`, making it a defined symbol that other
    /// modules can resolve at link time (see [`crate::core::linker::Linker`]).
    ///
    /// `name` may be a string ([`ExternalName::Symbol`]) or a user key
    /// ([`ExternalName::User`]).
    pub fn bind_symbol(&mut self, name: impl Into<ExternalName>, label: Label) {
        if self.error.is_some() {
            return;
        }
        if self.label_offsets.get(label.id() as usize).is_none() {
            self.record_error(AsmError::InvalidArgument);
            return;
        }
        self.defined_symbols.push((name.into(), label));
    }

    pub fn symbol_distance(&self, sym: Sym) -> Option<RelocDistance> {
        self.symbols
            .get(sym.id() as usize)
            .map(|symbol| symbol.distance)
    }

    pub fn symbol_name(&self, sym: Sym) -> Option<&ExternalName> {
        self.symbols
            .get(sym.id() as usize)
            .map(|symbol| &symbol.name)
    }

    pub fn get_label(&mut self) -> Label {
        if self.error.is_some() {
            return Label::new();
        }
        let l = self.label_offsets.len();
        self.label_offsets.push(u32::MAX);
        Label::from_id(l as _)
    }

    pub fn is_bound(&self, label: Label) -> bool {
        self.label_offsets
            .get(label.id() as usize)
            .is_some_and(|offset| *offset != u32::MAX)
    }

    /// Number of labels created so far; label ids below this count are valid.
    pub fn label_count(&self) -> u32 {
        self.label_offsets.len() as u32
    }

    pub fn get_label_for_constant(&mut self, constant: Constant) -> Label {
        if self.error.is_some() {
            return Label::new();
        }
        let Some((_, metadata)) = self.constants.get(constant.0 as usize) else {
            self.record_error(AsmError::InvalidArgument);
            return Label::new();
        };
        let metadata = *metadata;
        let AsmConstant {
            upcoming_label,
            size,
            ..
        } = metadata;
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
        if self.error.is_some() {
            return Constant(u32::MAX);
        }
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
        if self.error.is_some() {
            return;
        }
        if let Err(error) = kind.validate_for_arch(self.env.arch()) {
            self.record_error(error);
            return;
        }
        if (offset as usize).checked_add(kind.patch_size()).is_none() {
            self.record_error(AsmError::TooLarge);
            return;
        }
        if self.label_offsets.get(label.id() as usize).is_none() {
            self.record_error(AsmError::InvalidArgument);
            return;
        }
        let fixup = AsmFixup {
            kind,
            label,
            offset,
        };

        self.pending_fixup_records.push(fixup);
        self.pending_fixup_deadline = self.pending_fixup_deadline.min(fixup.deadline());
    }

    /// Align up to the given alignment.
    pub fn try_align_to(&mut self, align_to: CodeOffset) -> Result<(), AsmError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        if !align_to.is_power_of_two() {
            return Err(AsmError::InvalidArgument);
        }
        while self.cur_offset() & (align_to - 1) != 0 {
            self.write_u8(0);
        }
        #[cfg(feature = "x86")]
        if align_to > 1 {
            self.alignment_constraints
                .push((self.cur_offset(), align_to));
        }
        Ok(())
    }

    pub fn align_to(&mut self, align_to: CodeOffset) {
        if let Err(error) = self.try_align_to(align_to) {
            self.record_error(error);
        }
    }

    pub fn cur_offset(&self) -> CodeOffset {
        self.data.len() as _
    }

    pub fn try_bind_label(&mut self, label: Label) -> Result<(), AsmError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        let current_offset = self.cur_offset();
        let Some(offset) = self.label_offsets.get_mut(label.id() as usize) else {
            return Err(AsmError::InvalidArgument);
        };
        if *offset != u32::MAX {
            return Err(AsmError::InvalidState);
        }
        *offset = current_offset;
        Ok(())
    }

    pub fn bind_label(&mut self, label: Label) {
        if self.error.is_some() {
            return;
        }
        if let Err(error) = self.try_bind_label(label) {
            self.record_error(error);
        }
    }

    pub fn label_offset(&self, label: Label) -> u32 {
        self.label_offsets
            .get(label.id() as usize)
            .copied()
            .unwrap_or(u32::MAX)
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
        if self.error.is_some() {
            return;
        }
        if !kind.supports_arch(self.env.arch()) {
            self.record_error(AsmError::InvalidArch);
            return;
        }
        let valid_target = match target {
            RelocTarget::Sym(sym) => self.symbols.get(sym.id() as usize).is_some(),
            RelocTarget::Label(label) => self.label_offsets.get(label.id() as usize).is_some(),
        };
        if !valid_target {
            self.record_error(AsmError::InvalidArgument);
            return;
        }
        self.relocs.push(AsmReloc {
            addend,
            kind,
            offset,
            target,
        })
    }

    /// Reserve a nop-filled island for later custom rewriting (JSC `padBeforePatch`).
    ///
    /// Returns a [`PatchableBlock`] handle; the block is also recorded in the patch catalog
    /// for `finish_patched` / linker rebasing.
    pub fn reserve_patch_block(
        &mut self,
        size: CodeOffset,
        align: CodeOffset,
    ) -> Result<PatchableBlock, AsmError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        let min_align = minimum_patch_alignment(self.env.arch());
        let align = align.max(min_align);
        if size == 0 || !align.is_power_of_two() {
            return Err(AsmError::InvalidArgument);
        }
        let nop_size = match self.env.arch() {
            Arch::X86 | Arch::X64 => 1,
            Arch::AArch64 | Arch::RISCV32 | Arch::RISCV64 => 4,
            _ => return Err(AsmError::InvalidArch),
        };
        if size as usize % nop_size != 0 {
            return Err(AsmError::InvalidArgument);
        }

        self.try_align_to(align)?;
        let arch = self.env.arch();
        let offset = self.cur_offset();
        let block = self.get_appended_space(size as usize);
        fill_with_nops(arch, block)?;

        self.patch_blocks.push(PendingPatchBlock {
            offset,
            size,
            align,
        });
        // SAFETY: we just reserved and recorded this nop island.
        Ok(unsafe { PatchableBlock::new(offset, size, arch) })
    }

    pub fn record_patch_block(
        &mut self,
        offset: CodeOffset,
        size: CodeOffset,
        align: CodeOffset,
    ) -> PatchBlockId {
        match self.try_record_patch_block(offset, size, align) {
            Ok(id) => id,
            Err(error) => {
                self.record_error(error);
                PatchBlockId::from_index(usize::MAX)
            }
        }
    }

    pub fn try_record_patch_block(
        &mut self,
        offset: CodeOffset,
        size: CodeOffset,
        align: CodeOffset,
    ) -> Result<PatchBlockId, AsmError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        if size == 0 || align == 0 || !align.is_power_of_two() || offset & (align - 1) != 0 {
            return Err(AsmError::InvalidArgument);
        }
        let end = (offset as usize)
            .checked_add(size as usize)
            .ok_or(AsmError::TooLarge)?;
        if end > self.data.len() {
            return Err(AsmError::InvalidArgument);
        }
        let id = PatchBlockId::from_index(self.patch_blocks.len());
        self.patch_blocks.push(PendingPatchBlock {
            offset,
            size,
            align,
        });
        Ok(id)
    }

    pub fn record_patch_site(
        &mut self,
        offset: CodeOffset,
        kind: LabelUse,
        target_offset: CodeOffset,
    ) -> PatchSiteId {
        match self.try_record_patch_site(offset, kind, target_offset) {
            Ok(id) => id,
            Err(error) => {
                self.record_error(error);
                PatchSiteId::from_index(usize::MAX)
            }
        }
    }

    pub fn try_record_patch_site(
        &mut self,
        offset: CodeOffset,
        kind: LabelUse,
        target_offset: CodeOffset,
    ) -> Result<PatchSiteId, AsmError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        self.validate_patch_site_offset(offset, kind)?;
        let id = PatchSiteId::from_index(self.patch_sites.len());
        self.patch_sites.push(PendingPatchSite {
            offset,
            kind,
            target: PendingPatchTarget::Offset(target_offset),
            addend: 0,
        });
        Ok(id)
    }

    pub fn record_label_patch_site(
        &mut self,
        offset: CodeOffset,
        label: Label,
        kind: LabelUse,
    ) -> PatchSiteId {
        match self.try_record_label_patch_site(offset, label, kind) {
            Ok(id) => id,
            Err(error) => {
                self.record_error(error);
                PatchSiteId::from_index(usize::MAX)
            }
        }
    }

    pub fn try_record_label_patch_site(
        &mut self,
        offset: CodeOffset,
        label: Label,
        kind: LabelUse,
    ) -> Result<PatchSiteId, AsmError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        if self.label_offsets.get(label.id() as usize).is_none() {
            return Err(AsmError::InvalidArgument);
        }
        self.validate_patch_site_offset(offset, kind)?;
        let id = PatchSiteId::from_index(self.patch_sites.len());
        self.patch_sites.push(PendingPatchSite {
            offset,
            kind,
            target: PendingPatchTarget::Label(label),
            addend: 0,
        });
        Ok(id)
    }

    fn validate_patch_site_offset(
        &self,
        offset: CodeOffset,
        kind: LabelUse,
    ) -> Result<(), AsmError> {
        kind.validate_for_arch(self.env.arch())?;
        let end = (offset as usize)
            .checked_add(kind.patch_size())
            .ok_or(AsmError::TooLarge)?;
        if end > self.data.len() {
            return Err(AsmError::InvalidArgument);
        }
        Ok(())
    }

    fn handle_fixup(&mut self, fixup: AsmFixup) -> Result<(), AsmError> {
        let AsmFixup {
            kind,
            label,
            offset,
        } = fixup;
        let start = offset;
        let end = (offset as usize)
            .checked_add(kind.patch_size())
            .ok_or(AsmError::TooLarge)?;
        if end > self.data.len() {
            return Err(AsmError::InvalidArgument);
        }

        let label_offset = self.label_offset(label);
        if label_offset != u32::MAX {
            if !kind.can_reach(offset, label_offset) {
                if label_offset < offset {
                    self.emit_veneer(label, offset, kind)?;
                } else {
                    return Err(AsmError::TooLarge);
                }
            } else {
                let slice = &mut self.data[start as usize..end];

                kind.patch(slice, start, label_offset);
            }
        } else {
            // If the offset of this label is not known at this time then
            // that means that a veneer is required because after this
            // island the target can't be in range of the original target.
            self.emit_veneer(label, offset, kind)?;
        }
        Ok(())
    }

    /// Emits a "veneer" the `kind` code at `offset` to jump to `label`.
    ///
    /// This will generate extra machine code, using `kind`, to get a
    /// larger-jump-kind than `kind` allows. The code at `offset` is then
    /// patched to jump to our new code, and then the new code is enqueued for
    /// a fixup to get processed at some later time.
    pub fn emit_veneer(
        &mut self,
        label: Label,
        offset: CodeOffset,
        kind: LabelUse,
    ) -> Result<(), AsmError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        kind.validate_for_arch(self.env.arch())?;
        if !kind.supports_veneer() {
            return Err(AsmError::UnsupportedInstruction {
                reason: "branch range requires an unsupported veneer",
            });
        }
        if self.label_offsets.get(label.id() as usize).is_none() {
            return Err(AsmError::InvalidArgument);
        }
        let start = offset as usize;
        let end = start
            .checked_add(kind.patch_size())
            .ok_or(AsmError::TooLarge)?;
        if end > self.data.len() {
            return Err(AsmError::InvalidArgument);
        }
        #[cfg(not(feature = "riscv"))]
        {
            let _ = (label, offset, kind);
            Err(AsmError::UnsupportedInstruction {
                reason: "RISC-V veneer support is disabled",
            })
        }

        #[cfg(feature = "riscv")]
        {
            self.try_align_to(kind.align() as _)?;
            let veneer_offset = self.cur_offset();
            let slice = &mut self.data[start..end];

            kind.patch(slice, offset, veneer_offset);
            let veneer_slice = self.get_appended_space(kind.veneer_size());
            let (veneer_fixup_off, veneer_label_use) =
                kind.generate_veneer(veneer_slice, veneer_offset);

            // Register a new use of `label` with our new veneer fixup and
            // offset. This'll recalculate deadlines accordingly and
            // enqueue this fixup to get processed at some later
            // time.
            self.use_label_at_offset(veneer_fixup_off, label, veneer_label_use);
            if let Some(error) = self.error.clone() {
                return Err(error);
            }
            Ok(())
        }
    }

    /// Reserve appended space and return a mutable slice referring to it.
    pub(crate) fn get_appended_space(&mut self, len: usize) -> &mut [u8] {
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
    pub fn emit_island(&mut self, distance: CodeOffset) -> Result<(), AsmError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        let forced_threshold = self.worst_case_end_of_island(distance);

        for constant in core::mem::take(&mut self.pending_constants) {
            let (_, AsmConstant { align, size, .. }) = self.constants[constant.0 as usize];
            let label = self.constants[constant.0 as usize]
                .1
                .upcoming_label
                .take()
                .unwrap();
            self.try_align_to(align as _)?;
            self.try_bind_label(label)?;
            self.used_constants.push((constant, self.cur_offset()));
            self.get_appended_space(size);
        }
        // Either handle all pending fixups because they're ready or move them
        // onto the `BinaryHeap` tracking all pending fixups if they aren't
        // ready.
        for fixup in core::mem::take(&mut self.pending_fixup_records) {
            if self.should_apply_fixup(&fixup, forced_threshold) {
                self.handle_fixup(fixup)?;
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
            self.handle_fixup(fixup)?;
        }
        Ok(())
    }

    fn finish_emission_maybe_forcing_veneers(&mut self) -> Result<(), AsmError> {
        while !self.pending_constants.is_empty()
            || !self.pending_fixup_records.is_empty()
            || !self.fixup_records.is_empty()
        {
            // `emit_island()` will emit any pending veneers and constants, and
            // as a side-effect, will also take care of any fixups with resolved
            // labels eagerly.
            self.emit_island(u32::MAX)?;
        }
        Ok(())
    }

    #[cfg(feature = "x86")]
    fn relax_x86_branches(&mut self) -> Result<(), AsmError> {
        loop {
            let mut selected = None;

            for (index, &candidate) in self.x86_branch_relaxations.iter().enumerate() {
                let start = candidate.opcode_offset as usize;
                let near_size = candidate.near_size as usize;
                let end = start.checked_add(near_size).ok_or(AsmError::TooLarge)?;
                if end > self.data.len() {
                    return Err(AsmError::InvalidState);
                }
                let valid_encoding = match candidate.near_size {
                    5 => self.data[start] == 0xE9 && candidate.short_opcode == 0xEB,
                    6 => {
                        self.data[start] == 0x0F
                            && self.data[start + 1] == candidate.short_opcode.wrapping_add(0x10)
                    }
                    _ => false,
                };
                if !valid_encoding {
                    return Err(AsmError::InvalidState);
                }

                let target = self.label_offset(candidate.label);
                if target == u32::MAX {
                    return Err(AsmError::UnboundLabel);
                }
                let old_end = candidate
                    .opcode_offset
                    .checked_add(candidate.near_size as u32)
                    .ok_or(AsmError::TooLarge)?;
                if target > candidate.opcode_offset && target < old_end {
                    return Err(AsmError::InvalidState);
                }

                let removed = candidate.near_size as u32 - 2;
                let target_after = if target >= old_end {
                    target - removed
                } else {
                    target
                };
                let short_end = candidate
                    .opcode_offset
                    .checked_add(2)
                    .ok_or(AsmError::TooLarge)?;
                let displacement = i64::from(target_after) - i64::from(short_end);
                let Ok(displacement) = i8::try_from(displacement) else {
                    continue;
                };

                let cut_start = short_end;
                let cut_end = old_end;
                let preserves_alignment =
                    self.alignment_constraints.iter().all(|&(offset, align)| {
                        offset < cut_end || (offset - removed) & (align - 1) == 0
                    }) && self.patch_blocks.iter().all(|block| {
                        block.offset < cut_end || (block.offset - removed) & (block.align - 1) == 0
                    });
                if !preserves_alignment {
                    continue;
                }

                selected = Some((index, candidate, displacement, cut_start, cut_end));
                break;
            }

            let Some((index, candidate, displacement, cut_start, cut_end)) = selected else {
                return Ok(());
            };
            let removed = cut_end - cut_start;
            let fixup_offset = cut_end - LabelUse::X86JmpRel32.patch_size() as u32;
            let is_candidate_fixup = |fixup: &AsmFixup| {
                fixup.offset == fixup_offset
                    && fixup.label == candidate.label
                    && fixup.kind == LabelUse::X86JmpRel32
            };

            let overlaps_cut = |offset: CodeOffset, size: usize| -> Result<bool, AsmError> {
                let end = offset
                    .checked_add(u32::try_from(size).map_err(|_| AsmError::TooLarge)?)
                    .ok_or(AsmError::TooLarge)?;
                Ok(offset < cut_end && end > cut_start)
            };
            for reloc in &self.relocs {
                if overlaps_cut(reloc.offset, relocation_patch_size(reloc.kind)?)? {
                    return Err(AsmError::InvalidState);
                }
            }
            for fixup in self
                .pending_fixup_records
                .iter()
                .chain(self.fixup_records.iter())
            {
                if !is_candidate_fixup(fixup)
                    && overlaps_cut(fixup.offset, fixup.kind.patch_size())?
                {
                    return Err(AsmError::InvalidState);
                }
            }
            for block in &self.patch_blocks {
                if overlaps_cut(block.offset, block.size as usize)? {
                    return Err(AsmError::InvalidState);
                }
            }
            for site in &self.patch_sites {
                if overlaps_cut(site.offset, site.kind.patch_size())?
                    || matches!(site.target, PendingPatchTarget::Offset(offset) if (cut_start..cut_end).contains(&offset))
                {
                    return Err(AsmError::InvalidState);
                }
            }
            if self
                .label_offsets
                .iter()
                .any(|&offset| offset != u32::MAX && (cut_start..cut_end).contains(&offset))
                || self
                    .used_constants
                    .iter()
                    .any(|&(_, offset)| (cut_start..cut_end).contains(&offset))
                || self
                    .alignment_constraints
                    .iter()
                    .any(|&(offset, _)| (cut_start..cut_end).contains(&offset))
            {
                return Err(AsmError::InvalidState);
            }
            for (other_index, other) in self.x86_branch_relaxations.iter().enumerate() {
                if other_index != index
                    && overlaps_cut(other.opcode_offset, other.near_size as usize)?
                {
                    return Err(AsmError::InvalidState);
                }
            }

            let start = candidate.opcode_offset as usize;
            self.data[start] = candidate.short_opcode;
            self.data[start + 1] = displacement as u8;
            self.data.drain(cut_start as usize..cut_end as usize);
            self.x86_branch_relaxations.remove(index);

            self.pending_fixup_records
                .retain(|fixup| !is_candidate_fixup(fixup));
            let mut fixups = core::mem::take(&mut self.fixup_records).into_vec();
            fixups.retain(|fixup| !is_candidate_fixup(fixup));

            let rebase = |offset: &mut CodeOffset| -> Result<(), AsmError> {
                if *offset >= cut_end {
                    *offset -= removed;
                } else if *offset >= cut_start {
                    return Err(AsmError::InvalidState);
                }
                Ok(())
            };

            for reloc in &mut self.relocs {
                rebase(&mut reloc.offset)?;
            }
            for offset in &mut self.label_offsets {
                if *offset != u32::MAX {
                    rebase(offset)?;
                }
            }
            for fixup in &mut self.pending_fixup_records {
                rebase(&mut fixup.offset)?;
            }
            for fixup in &mut fixups {
                rebase(&mut fixup.offset)?;
            }
            self.fixup_records = BinaryHeap::from(fixups);
            self.pending_fixup_deadline = self
                .pending_fixup_records
                .iter()
                .map(AsmFixup::deadline)
                .min()
                .unwrap_or(u32::MAX);
            for (_, offset) in &mut self.used_constants {
                rebase(offset)?;
            }
            for block in &mut self.patch_blocks {
                rebase(&mut block.offset)?;
            }
            for site in &mut self.patch_sites {
                rebase(&mut site.offset)?;
                if let PendingPatchTarget::Offset(offset) = &mut site.target {
                    rebase(offset)?;
                }
            }
            for relaxation in &mut self.x86_branch_relaxations {
                rebase(&mut relaxation.opcode_offset)?;
            }
            for (offset, _) in &mut self.alignment_constraints {
                rebase(offset)?;
            }
        }
    }

    /// Reject finalization failures that can be determined before island
    /// emission mutates the buffer.
    fn preflight_finalization(&self) -> Result<(), AsmError> {
        for fixup in self
            .pending_fixup_records
            .iter()
            .chain(self.fixup_records.iter())
        {
            let end = (fixup.offset as usize)
                .checked_add(fixup.kind.patch_size())
                .ok_or(AsmError::TooLarge)?;
            if end > self.data.len() {
                return Err(AsmError::InvalidArgument);
            }
            let label_offset = self.label_offset(fixup.label);
            if label_offset == u32::MAX {
                return Err(AsmError::UnboundLabel);
            }
            if fixup.kind.can_reach(fixup.offset, label_offset) {
                continue;
            }
            if matches!(
                fixup.kind,
                LabelUse::A64Branch14 | LabelUse::A64Branch19 | LabelUse::A64Branch26
            ) && !fixup.kind.supports_veneer()
            {
                return Err(AsmError::UnsupportedInstruction {
                    reason: "AArch64 branch veneers are not implemented",
                });
            }
            if label_offset >= fixup.offset {
                return Err(AsmError::TooLarge);
            }
            if !fixup.kind.supports_veneer() {
                return Err(AsmError::UnsupportedInstruction {
                    reason: "branch range requires an unsupported veneer",
                });
            }
            #[cfg(not(feature = "riscv"))]
            return Err(AsmError::UnsupportedInstruction {
                reason: "RISC-V veneer support is disabled",
            });
        }
        Ok(())
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

    fn resolve_patch_catalog(&self, validate_ranges: bool) -> Result<PatchCatalog, AsmError> {
        let mut blocks = SmallVec::new();
        let mut sites = SmallVec::new();

        for block in &self.patch_blocks {
            blocks.push(PatchBlock {
                offset: block.offset,
                size: block.size,
                align: block.align,
            });
        }

        for site in &self.patch_sites {
            let target_offset = match site.target {
                PendingPatchTarget::Offset(offset) => offset,
                PendingPatchTarget::Label(label) => self.label_offset(label),
            };

            if target_offset == u32::MAX {
                return Err(AsmError::InvalidState);
            }

            if validate_ranges && !site.kind.can_reach(site.offset, target_offset) {
                return Err(AsmError::TooLarge);
            }

            sites.push(PatchSite {
                offset: site.offset,
                kind: site.kind,
                current_target: target_offset,
                addend: site.addend,
            });
        }

        Ok(PatchCatalog::with_parts(self.env.arch(), blocks, sites))
    }

    fn resolved_defined_symbols(&self) -> SmallVec<[(ExternalName, CodeOffset); 4]> {
        // Unbound labels keep the sentinel offset; the linker reports them as
        // an error.
        self.defined_symbols
            .iter()
            .map(|(name, label)| (name.clone(), self.label_offset(*label)))
            .collect()
    }

    pub fn finish_patched(mut self) -> Result<CodeBufferFinalized, AsmError> {
        if let Some(error) = self.error.take() {
            return Err(error);
        }
        if self.has_unbound_labels() {
            return Err(AsmError::UnboundLabel);
        }
        self.preflight_finalization()?;
        #[cfg(feature = "x86")]
        self.relax_x86_branches()?;
        #[cfg(feature = "x86")]
        self.preflight_finalization()?;
        self.finish_emission_maybe_forcing_veneers()?;
        let patch_catalog = self.resolve_patch_catalog(true)?;
        let alignment = self.finish_constants();
        let defined_symbols = self.resolved_defined_symbols();
        Ok(CodeBufferFinalized {
            data: self.data,
            relocs: self.relocs,
            symbols: self.symbols,
            label_offsets: self.label_offsets,
            defined_symbols,
            alignment,
            patch_catalog,
        })
    }

    pub fn finish(&mut self) -> Result<CodeBufferFinalized, AsmError> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        if self.has_unbound_labels() {
            return Err(AsmError::UnboundLabel);
        }
        self.preflight_finalization()?;
        #[cfg(feature = "x86")]
        self.relax_x86_branches()?;
        #[cfg(feature = "x86")]
        self.preflight_finalization()?;
        self.finish_emission_maybe_forcing_veneers()?;
        let patch_catalog = self.resolve_patch_catalog(false)?;
        let alignment = self.finish_constants();
        Ok(CodeBufferFinalized {
            data: self.data.clone(),
            relocs: self.relocs.clone(),
            symbols: self.symbols.clone(),
            label_offsets: self.label_offsets.clone(),
            defined_symbols: self.resolved_defined_symbols(),
            alignment,
            patch_catalog,
        })
    }

    fn has_unbound_labels(&self) -> bool {
        let is_unbound = |label: Label| {
            self.label_offsets
                .get(label.id() as usize)
                .is_none_or(|offset| *offset == u32::MAX)
        };
        self.pending_fixup_records
            .iter()
            .any(|fixup| is_unbound(fixup.label))
            || self
                .fixup_records
                .iter()
                .any(|fixup| is_unbound(fixup.label))
            || self
                .relocs
                .iter()
                .any(|reloc| matches!(reloc.target, RelocTarget::Label(label) if is_unbound(label)))
            || self
                .defined_symbols
                .iter()
                .any(|(_, label)| is_unbound(*label))
            || self.patch_sites.iter().any(
                |site| matches!(site.target, PendingPatchTarget::Label(label) if is_unbound(label)),
            )
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
        if self.as_slice().len() <= 8 { 8 } else { 16 }
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    /// Equivalent of `R_AARCH64_ADR_PREL_PG_HI21`.
    Aarch64AdrPrelPgHi21,
    /// Equivalent of `R_AARCH64_ADD_ABS_LO12_NC`.
    Aarch64AddAbsLo12Nc,

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

impl Reloc {
    const fn supports_arch(self, arch: Arch) -> bool {
        match self {
            Self::Abs4 | Self::Abs8 => true,
            Self::X86PCRel4
            | Self::X86CallPCRel4
            | Self::X86CallPLTRel4
            | Self::X86GOTPCRel4
            | Self::X86SecRel
            | Self::ElfX86_64TlsGd
            | Self::MachOX86_64Tlv => {
                cfg!(feature = "x86") && matches!(arch, Arch::X86 | Arch::X64)
            }
            Self::Arm32Call => false,
            Self::Arm64Call
            | Self::MachOAarch64TlsAdrPage21
            | Self::MachOAarch64TlsAdrPageOff12
            | Self::Aarch64TlsDescAdrPage21
            | Self::Aarch64TlsDescLd64Lo12
            | Self::Aarch64TlsDescAddLo12
            | Self::Aarch64TlsDescCall
            | Self::Aarch64AdrGotPage21
            | Self::Aarch64Ld64GotLo12Nc
            | Self::Aarch64AdrPrelPgHi21
            | Self::Aarch64AddAbsLo12Nc => {
                cfg!(feature = "aarch64") && matches!(arch, Arch::AArch64)
            }
            Self::RiscvAbs8
            | Self::RiscvCallPlt
            | Self::RiscvTlsGdHi20
            | Self::RiscvPCRelLo12I
            | Self::RiscvGotHi20 => {
                cfg!(feature = "riscv") && matches!(arch, Arch::RISCV32 | Arch::RISCV64)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
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
    /// 19-bit offset for LDR (load literal). PC-rel, offset is imm << 2. Immediate is 19 signed bits,
    /// in bits 23:5.
    A64Ldr19,
    /// 21-bit offset for ADR (get address of label). PC-rel, offset is not shifted. Immediate is
    /// 21 signed bits, with high 19 bits in bits 23:5 and low 2 bits in bits 30:29.
    A64Adr21,
    /// 21-bit offset for ADRP (get address of label). PC-rel, offset is shifted. Immediate is
    /// 21 signed bits, with high 19 bits in bits 23:5 and low 2 bits in bits 30:29.
    A64Adrp21,
    A64Ldr12,

    A64AddAbsLo12,
}

impl LabelUse {
    fn validate_for_arch(self, arch: Arch) -> Result<(), AsmError> {
        if self == Self::A64Ldr12 {
            return Err(AsmError::UnsupportedInstruction {
                reason: "AArch64 LDR12 label patching is not implemented",
            });
        }
        if !self.supports_arch(arch) {
            return Err(AsmError::InvalidArch);
        }
        Ok(())
    }

    const fn supports_arch(self, arch: Arch) -> bool {
        match self {
            Self::X86JmpRel32 => cfg!(feature = "x86") && matches!(arch, Arch::X86 | Arch::X64),
            Self::RVJal20
            | Self::RVPCRel32
            | Self::RVB12
            | Self::RVPCRelHi20
            | Self::RVPCRelLo12I
            | Self::RVCJump
            | Self::RVCB9 => {
                cfg!(feature = "riscv") && matches!(arch, Arch::RISCV32 | Arch::RISCV64)
            }
            Self::A64Branch14
            | Self::A64Branch19
            | Self::A64Branch26
            | Self::A64Ldr19
            | Self::A64Adr21
            | Self::A64Adrp21
            | Self::A64Ldr12
            | Self::A64AddAbsLo12 => cfg!(feature = "aarch64") && matches!(arch, Arch::AArch64),
        }
    }

    pub fn can_reach(&self, use_offset: CodeOffset, label_offset: CodeOffset) -> bool {
        let delta = (label_offset as i64) - (use_offset as i64);

        match self {
            Self::X86JmpRel32 => {
                let disp = delta - 4;
                i32::try_from(disp).is_ok()
            }
            Self::RVJal20 => delta % 2 == 0 && (-(1 << 20)..=((1 << 20) - 2)).contains(&delta),
            Self::RVB12 => delta % 2 == 0 && (-(1 << 12)..=((1 << 12) - 2)).contains(&delta),
            Self::RVCJump => delta % 2 == 0 && (-(1 << 11)..=((1 << 11) - 2)).contains(&delta),
            Self::RVCB9 => delta % 2 == 0 && (-(1 << 8)..=((1 << 8) - 2)).contains(&delta),
            Self::RVPCRelHi20 | Self::RVPCRelLo12I | Self::RVPCRel32 => {
                i32::try_from(delta).is_ok()
            }
            Self::A64Branch14 => delta % 4 == 0 && (-(1 << 15)..=((1 << 15) - 4)).contains(&delta),
            Self::A64Branch19 | Self::A64Ldr19 => {
                delta % 4 == 0 && (-(1 << 20)..=((1 << 20) - 4)).contains(&delta)
            }
            Self::A64Branch26 => delta % 4 == 0 && (-(1 << 27)..=((1 << 27) - 4)).contains(&delta),
            Self::A64Adr21 => (-(1 << 20)..=((1 << 20) - 1)).contains(&delta),
            Self::A64Adrp21 => {
                let page_delta = ((label_offset & !0xfff) as i64) - ((use_offset & !0xfff) as i64);
                page_delta % 4096 == 0 && (-(1 << 32)..=((1 << 32) - 4096)).contains(&page_delta)
            }

            Self::A64AddAbsLo12 => {
                delta % 4096 == delta && (-(1 << 12)..=(1 << 12) - 1).contains(&delta)
            }

            Self::A64Ldr12 => true,
        }
    }

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
        matches!(self, Self::RVB12 | Self::RVJal20 | Self::RVCJump)
    }

    #[cfg(feature = "riscv")]
    pub(crate) fn veneer_size(&self) -> usize {
        debug_assert!(self.supports_veneer());
        8
    }

    #[cfg(feature = "riscv")]
    pub(crate) fn generate_veneer(
        &self,
        buffer: &mut [u8],
        veneer_offset: CodeOffset,
    ) -> (CodeOffset, Self) {
        debug_assert!(self.supports_veneer());
        let base = riscv::X31;

        {
            let x = riscv::opcodes::Inst::new(riscv::Opcode::AUIPC)
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
            let x = riscv::opcodes::Inst::new(riscv::Opcode::JALR)
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
    }
    pub(crate) fn patch(
        &self,
        buffer: &mut [u8],
        use_offset: CodeOffset,
        label_offset: CodeOffset,
    ) {
        let addend = match self {
            Self::X86JmpRel32 => i64::from(u32::from_le_bytes([
                buffer[0], buffer[1], buffer[2], buffer[3],
            ])),
            _ => 0,
        };

        self.patch_with_addend(buffer, use_offset, label_offset, addend);
    }

    pub(crate) fn patch_with_addend(
        &self,
        buffer: &mut [u8],
        use_offset: CodeOffset,
        label_offset: CodeOffset,
        addend: i64,
    ) {
        let pc_reli = (label_offset as i64) - (use_offset as i64);

        let pc_rel = pc_reli as u32;

        match self {
            Self::X86JmpRel32 => {
                let value = pc_rel.wrapping_add(addend as u32).wrapping_sub(4);

                buffer.copy_from_slice(&value.to_le_bytes());
            }

            Self::RVJal20 => {
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let offset = pc_rel;
                let v = ((offset >> 12 & 0b1111_1111) << 12)
                    | ((offset >> 11 & 0b1) << 20)
                    | ((offset >> 1 & 0b11_1111_1111) << 21)
                    | ((offset >> 20 & 0b1) << 31);
                buffer[0..4].clone_from_slice(&u32::to_le_bytes(insn | v));
            }

            Self::RVPCRel32 => {
                #[cfg(feature = "riscv")]
                {
                    let (imm20, imm12) = generate_imm(pc_rel as u64);
                    let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                    let insn2 = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);

                    let auipc = riscv::opcodes::Inst::new(riscv::Opcode::AUIPC)
                        .encode()
                        .set_imm20(0);
                    let jalr = riscv::opcodes::Inst::new(riscv::Opcode::JALR)
                        .encode()
                        .set_rd(0)
                        .set_rs1(0)
                        .set_imm12(0);

                    buffer[0..4].copy_from_slice(&(insn | auipc.value | imm20).to_le_bytes());
                    buffer[4..8].copy_from_slice(&(insn2 | jalr.value | imm12).to_le_bytes());
                }
                #[cfg(not(feature = "riscv"))]
                {
                    panic!("RISC-V veneers aren't supported without the `riscv` feature");
                }
            }

            Self::RVB12 => {
                #[cfg(feature = "riscv")]
                {
                    let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                    let offset = pc_rel;
                    let v = ((offset >> 11 & 0b1) << 7)
                        | ((offset >> 1 & 0b1111) << 8)
                        | ((offset >> 5 & 0b11_1111) << 25)
                        | ((offset >> 12 & 0b1) << 31);
                    buffer[0..4].clone_from_slice(&u32::to_le_bytes(insn | v));
                }
                #[cfg(not(feature = "riscv"))]
                {
                    panic!("RISC-V veneers aren't supported without the `riscv` feature");
                }
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

                #[cfg(feature = "riscv")]
                {
                    let insn = riscv::opcodes::Inst::new(riscv::Opcode::CJ)
                        .encode()
                        .set_c_imm12(pc_rel as _);
                    buffer[0..2].clone_from_slice(&(insn.value as u16).to_le_bytes());
                }
                #[cfg(not(feature = "riscv"))]
                {
                    panic!("RISC-V jumps aren't supported without the `riscv` feature");
                }
            }

            Self::RVCB9 => {
                debug_assert!(pc_rel & 1 == 0);

                #[cfg(feature = "riscv")]
                {
                    let insn = riscv::opcodes::Inst::new(riscv::Opcode::BEQZ)
                        .encode()
                        .set_c_bimm9lohi(pc_rel as _);
                    buffer[0..2].clone_from_slice(&(insn.value as u16).to_le_bytes());
                }
                #[cfg(not(feature = "riscv"))]
                {
                    panic!("RISC-V veneers aren't supported without the `riscv` feature");
                }
            }

            Self::A64Branch14 => {
                debug_assert!(pc_reli & 0b11 == 0);

                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let imm14 = ((pc_reli >> 2) as i32 as u32) & 0x3fff;
                let insn = (insn & !0x0007ffe0) | (imm14 << 5);
                buffer[0..4].copy_from_slice(&insn.to_le_bytes());
            }

            Self::A64Branch19 | Self::A64Ldr19 => {
                debug_assert!(pc_reli & 0b11 == 0);

                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let imm19 = ((pc_reli >> 2) as i32 as u32) & 0x7ffff;
                let insn = (insn & !0x00ffffe0) | (imm19 << 5);
                buffer[0..4].copy_from_slice(&insn.to_le_bytes());
            }

            Self::A64Branch26 => {
                debug_assert!(pc_reli & 0b11 == 0);

                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let imm26 = ((pc_reli >> 2) as i32 as u32) & 0x03ff_ffff;
                let insn = (insn & !0x03ff_ffff) | imm26;
                buffer[0..4].copy_from_slice(&insn.to_le_bytes());
            }

            Self::A64Adr21 => {
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let imm21 = (pc_reli as i32 as u32) & 0x1f_ffff;
                let immlo = imm21 & 0x3;
                let immhi = (imm21 >> 2) & 0x7ffff;
                let insn = (insn & !0x60ff_ffe0) | (immlo << 29) | (immhi << 5);
                buffer[0..4].copy_from_slice(&insn.to_le_bytes());
            }

            Self::A64Adrp21 => {
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);

                // 1. Calculate the page-aligned PC and Target
                let pc_page = (use_offset as i64) & !0xFFF;
                let target_page = ((label_offset as i64) + addend) & !0xFFF;

                // 2. Calculate the offset in pages
                let page_offset = (target_page - pc_page) >> 12;

                // 3. Encode the 21-bit signed immediate
                let imm21 = (page_offset as u32) & 0x1F_FFFF;
                let immlo = imm21 & 0x3; // Lowest 2 bits
                let immhi = (imm21 >> 2) & 0x7FFFF; // Upper 19 bits

                // 4. Clear existing immediate bits and insert new ones
                // Bits 29..31 (immlo) and Bits 5..24 (immhi)
                let insn = (insn & !0x60FF_FFE0) | (immlo << 29) | (immhi << 5);

                buffer[0..4].copy_from_slice(&insn.to_le_bytes());
            }

            Self::A64AddAbsLo12 => {
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);

                let imm12 = ((pc_reli as i32 as u32) & 0xfff) << 10;
                let insn = insn | imm12;
                buffer[0..4].copy_from_slice(&insn.to_le_bytes());
            }

            _ => todo!(),
        }
    }
}

pub const fn is_imm12(val: i64) -> bool {
    val >= -2048 && val <= 2047
}

#[allow(dead_code)]
pub(crate) fn generate_imm(value: u64) -> (u32, u32) {
    #[cfg(not(feature = "riscv"))]
    {
        let _ = value;
        panic!("Can't generate RISC-V immediates without the `riscv` feature");
    }
    #[cfg(feature = "riscv")]
    {
        if is_imm12(value as _) {
            return (
                0,
                riscv::opcodes::InstructionValue::new(0)
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
            riscv::opcodes::InstructionValue::new(0)
                .set_imm20(imm20 as _)
                .value,
            riscv::opcodes::InstructionValue::new(0)
                .set_imm12(imm12 as _)
                .value,
        )
    }
}

pub(crate) fn relocation_patch_size(kind: Reloc) -> Result<usize, AsmError> {
    match kind {
        Reloc::Abs4
        | Reloc::X86PCRel4
        | Reloc::X86CallPCRel4
        | Reloc::X86CallPLTRel4
        | Reloc::X86GOTPCRel4
        | Reloc::RiscvGotHi20
        | Reloc::RiscvPCRelLo12I
        | Reloc::Aarch64AdrPrelPgHi21
        | Reloc::Aarch64AddAbsLo12Nc
        | Reloc::Aarch64AdrGotPage21
        | Reloc::Aarch64Ld64GotLo12Nc => Ok(4),
        Reloc::Abs8 | Reloc::RiscvAbs8 | Reloc::RiscvCallPlt => Ok(8),
        _ => Err(AsmError::InvalidArgument),
    }
}

fn checked_address(base: usize, addend: i64) -> Result<usize, AsmError> {
    if addend >= 0 {
        base.checked_add(usize::try_from(addend).map_err(|_| AsmError::TooLarge)?)
    } else {
        base.checked_sub(usize::try_from(addend.unsigned_abs()).map_err(|_| AsmError::TooLarge)?)
    }
    .ok_or(AsmError::TooLarge)
}

fn checked_pcrel(target: usize, instruction: usize) -> Result<i64, AsmError> {
    i64::try_from(target as i128 - instruction as i128).map_err(|_| AsmError::TooLarge)
}

/// Applies relocations to one writable code span.
///
/// Unsupported relocation kinds, invalid resolver results, out-of-range patch
/// spans, address overflow, and displacement overflow are returned as errors.
///
/// # Safety
///
/// `code` and `code_rx` must refer to writable and executable views of the same
/// `code_size`-byte allocation. Resolver pointers are treated as addresses and
/// are never dereferenced.
pub unsafe fn perform_relocations(
    code: *mut u8,
    code_rx: *const u8,
    code_size: usize,
    relocs: &[AsmReloc],
    get_address: impl Fn(&RelocTarget) -> Result<*const u8, AsmError>,
    get_got_entry: impl Fn(&RelocTarget) -> Result<*const u8, AsmError>,
    get_plt_entry: impl Fn(&RelocTarget) -> Result<*const u8, AsmError>,
) -> Result<(), AsmError> {
    use core::ptr::write_unaligned;

    if code.is_null() || code_rx.is_null() {
        return Err(AsmError::InvalidArgument);
    }

    for &AsmReloc {
        addend,
        kind,
        offset,
        ref target,
    } in relocs
    {
        let patch_size = relocation_patch_size(kind)?;
        let patch_start = offset as usize;
        let patch_end = patch_start
            .checked_add(patch_size)
            .ok_or(AsmError::TooLarge)?;
        if patch_end > code_size {
            return Err(AsmError::InvalidArgument);
        }
        let at = unsafe { code.add(patch_start) };
        let atrx = code_rx
            .addr()
            .checked_add(patch_start)
            .ok_or(AsmError::TooLarge)?;
        let resolve = |resolver: &dyn Fn(&RelocTarget) -> Result<*const u8, AsmError>| {
            let base = resolver(target)?;
            if base.is_null() {
                return Err(AsmError::InvalidArgument);
            }
            checked_address(base.addr(), addend)
        };

        match kind {
            Reloc::Abs4 => {
                let what = resolve(&get_address)?;
                let what = u32::try_from(what).map_err(|_| AsmError::TooLarge)?;
                unsafe {
                    write_unaligned(at as *mut u32, what);
                }
            }

            Reloc::Abs8 | Reloc::RiscvAbs8 => {
                let what = resolve(&get_address)?;
                let what = u64::try_from(what).map_err(|_| AsmError::TooLarge)?;
                unsafe {
                    write_unaligned(at as *mut u64, what);
                }
            }

            Reloc::X86PCRel4 | Reloc::X86CallPCRel4 => {
                let what = resolve(&get_address)?;
                let pcrel =
                    i32::try_from(checked_pcrel(what, atrx)?).map_err(|_| AsmError::TooLarge)?;

                unsafe {
                    write_unaligned(at as *mut i32, pcrel);
                }
            }

            Reloc::X86GOTPCRel4 => {
                let what = resolve(&get_got_entry)?;
                let pcrel =
                    i32::try_from(checked_pcrel(what, atrx)?).map_err(|_| AsmError::TooLarge)?;

                unsafe {
                    write_unaligned(at as *mut i32, pcrel);
                }
            }

            Reloc::X86CallPLTRel4 => {
                let what = resolve(&get_plt_entry)?;
                let pcrel =
                    i32::try_from(checked_pcrel(what, atrx)?).map_err(|_| AsmError::TooLarge)?;
                unsafe { write_unaligned(at as *mut i32, pcrel) };
            }

            Reloc::RiscvGotHi20 => {
                let what = resolve(&get_got_entry)?;
                let pc_rel =
                    i32::try_from(checked_pcrel(what, atrx)?).map_err(|_| AsmError::TooLarge)?;
                unsafe {
                    let buffer = core::slice::from_raw_parts_mut(at, 4);
                    let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                    let hi20 = (pc_rel as u32).wrapping_add(0x800) >> 12;
                    let insn = (insn & 0xfff) | (hi20 << 12);
                    buffer.copy_from_slice(&insn.to_le_bytes());
                }
            }

            Reloc::RiscvPCRelLo12I => {
                let what = resolve(&get_got_entry)?;
                let pc_rel = checked_pcrel(what, atrx)?;
                let pc_rel = i32::try_from(pc_rel).map_err(|_| AsmError::TooLarge)?;

                unsafe {
                    let buffer = core::slice::from_raw_parts_mut(at, 4);
                    let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                    let lo12 = ((pc_rel as i64 + 4) as u32) & 0xfff;
                    let insn = (insn & 0xFFFFF) | (lo12 << 20);
                    buffer.copy_from_slice(&insn.to_le_bytes());
                }
            }

            Reloc::RiscvCallPlt => {
                #[cfg(not(feature = "riscv"))]
                {
                    return Err(AsmError::InvalidArgument);
                }
                #[cfg(feature = "riscv")]
                {
                    // A R_RISCV_CALL_PLT relocation expects auipc+jalr instruction pair.
                    // It is the equivalent of two relocations:
                    // 1. R_RISCV_PCREL_HI20 on the `auipc`
                    // 2. R_RISCV_PCREL_LO12_I on the `jalr`

                    let what = resolve(&get_address)?;
                    let pcrel = i32::try_from(checked_pcrel(what, atrx)?)
                        .map_err(|_| AsmError::TooLarge)?;

                    // See https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#pc-relative-symbol-addresses
                    // for a better explanation of the following code.
                    //
                    // Unlike the regular symbol relocations, here both "sub-relocations" point to the same address.
                    //
                    // `pcrel` is a signed value (+/- 2GiB range), when splitting it into two parts, we need to
                    // ensure that `hi20` is close enough to `pcrel` to be able to add `lo12` to it and still
                    // get a valid address.
                    //
                    // `lo12` is also a signed offset (+/- 2KiB range) relative to the `hi20` value.
                    //
                    // `hi20` should also be shifted right to be the "true" value. But we also need it
                    // left shifted for the `lo12` calculation and it also matches the instruction encoding.
                    let hi20 = pcrel.wrapping_add(0x800) as u32 & 0xFFFFF000u32;
                    let lo12 = (pcrel as u32).wrapping_sub(hi20) & 0xFFF;

                    unsafe {
                        let auipc_addr = at as *mut u32;
                        let auipc = riscv::opcodes::Inst::new(riscv::Opcode::AUIPC)
                            .encode()
                            .set_imm20(hi20 as _)
                            .value;
                        write_unaligned(auipc_addr, auipc_addr.read_unaligned() | auipc);

                        let jalr_addr = at.add(4) as *mut u32;
                        let jalr = riscv::opcodes::Inst::new(riscv::Opcode::JALR)
                            .encode()
                            .set_imm12(lo12 as _)
                            .value;
                        write_unaligned(jalr_addr, jalr_addr.read_unaligned() | jalr);
                    }
                }
            }

            Reloc::Aarch64AdrPrelPgHi21 => {
                let what = resolve(&get_address)?;
                let pages = ((what & !0xfff) as i128 - (atrx & !0xfff) as i128) >> 12;
                if !(-(1i128 << 20)..(1i128 << 20)).contains(&pages) {
                    return Err(AsmError::TooLarge);
                }
                let iptr = at as *mut u32;
                let imm21 = pages as u32 & 0x1f_ffff;
                let lo = (imm21 & 0x3) << 29;
                let hi = ((imm21 >> 2) & 0x7ffff) << 5;
                unsafe {
                    let insn = iptr.read_unaligned();
                    write_unaligned(iptr, insn | lo | hi);
                }
            }

            Reloc::Aarch64AddAbsLo12Nc => {
                let what = resolve(&get_address)?;
                let iptr = at as *mut u32;
                let imm12 = (what as u32 & 0xfff) << 10;
                unsafe {
                    let insn = iptr.read_unaligned();
                    write_unaligned(iptr, insn | imm12);
                }
            }

            Reloc::Aarch64AdrGotPage21 => {
                let what = resolve(&get_got_entry)?;
                let pages = ((what & !0xfff) as i128 - (atrx & !0xfff) as i128) >> 12;
                if !(-(1i128 << 20)..(1i128 << 20)).contains(&pages) {
                    return Err(AsmError::TooLarge);
                }
                let iptr = at as *mut u32;
                let imm21 = pages as u32 & 0x1f_ffff;
                let lo = (imm21 & 0x3) << 29;
                let hi = ((imm21 >> 2) & 0x7ffff) << 5;
                unsafe {
                    let insn = iptr.read_unaligned();
                    write_unaligned(iptr, insn | lo | hi);
                }
            }

            Reloc::Aarch64Ld64GotLo12Nc => {
                let what = resolve(&get_got_entry)?;
                if what & 7 != 0 {
                    return Err(AsmError::InvalidArgument);
                }
                let iptr = at as *mut u32;
                let imm12 = ((what as u32 & 0xfff) >> 3) << 10;
                unsafe {
                    let insn = iptr.read_unaligned();
                    write_unaligned(iptr, insn | imm12);
                }
            }

            _ => return Err(AsmError::InvalidArgument),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolved(address: usize) -> impl Fn(&RelocTarget) -> Result<*const u8, AsmError> {
        move |_| Ok(address as *const u8)
    }

    fn unresolved(_: &RelocTarget) -> Result<*const u8, AsmError> {
        Ok(core::ptr::null())
    }

    #[test]
    fn relocation_rejects_patch_outside_code() {
        let mut code = [0u8; 4];
        let relocs = [AsmReloc {
            offset: 1,
            kind: Reloc::Abs4,
            addend: 0,
            target: RelocTarget::Label(Label::from_id(0)),
        }];

        let result = unsafe {
            perform_relocations(
                code.as_mut_ptr(),
                code.as_ptr(),
                code.len(),
                &relocs,
                resolved(1),
                resolved(1),
                resolved(1),
            )
        };

        assert_eq!(result, Err(AsmError::InvalidArgument));
        assert_eq!(code, [0; 4]);
    }

    #[test]
    fn relocation_rejects_null_and_overflowing_targets() {
        let mut code = [0u8; 8];
        let reloc = AsmReloc {
            offset: 0,
            kind: Reloc::Abs8,
            addend: 0,
            target: RelocTarget::Label(Label::from_id(0)),
        };

        let null_result = unsafe {
            perform_relocations(
                code.as_mut_ptr(),
                code.as_ptr(),
                code.len(),
                core::slice::from_ref(&reloc),
                unresolved,
                unresolved,
                unresolved,
            )
        };
        assert_eq!(null_result, Err(AsmError::InvalidArgument));

        let overflowing = AsmReloc {
            addend: -2,
            ..reloc
        };
        let overflow_result = unsafe {
            perform_relocations(
                code.as_mut_ptr(),
                code.as_ptr(),
                code.len(),
                core::slice::from_ref(&overflowing),
                resolved(1),
                resolved(1),
                resolved(1),
            )
        };
        assert_eq!(overflow_result, Err(AsmError::TooLarge));
    }

    #[test]
    fn poisoned_buffer_rejects_raw_mutation_and_patch_metadata() {
        let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
        buffer.write_u32(0);
        let bytes = buffer.data().to_vec();
        buffer.record_error(AsmError::InvalidOperand);

        buffer.write_u8(1);
        buffer.put8(2);
        assert!(
            !buffer
                .add_symbol(ExternalName::user(0, 1), RelocDistance::Far)
                .is_valid()
        );
        assert_eq!(buffer.add_constant(3u64), Constant(u32::MAX));
        assert!(!buffer.get_label().is_valid());
        buffer.add_reloc(Reloc::Abs4, RelocTarget::Label(Label::from_id(0)), 0);
        let patch = buffer.try_record_patch_site(0, LabelUse::X86JmpRel32, 0);

        assert_eq!(patch, Err(AsmError::InvalidOperand));
        assert_eq!(buffer.data(), bytes);
        assert!(buffer.relocs().is_empty());
        assert!(buffer.patch_sites.is_empty());
        assert!(matches!(buffer.finish(), Err(AsmError::InvalidOperand)));
    }

    #[test]
    fn target_rejects_foreign_relocations_and_patch_kinds() {
        let mut buffer = CodeBuffer::new(Environment::new(Arch::AArch64));
        let label = buffer.get_label();
        buffer.add_reloc(Reloc::X86PCRel4, RelocTarget::Label(label), 0);
        assert_eq!(buffer.error(), Some(&AsmError::InvalidArch));
        assert!(buffer.relocs().is_empty());

        let mut buffer = CodeBuffer::new(Environment::new(Arch::AArch64));
        buffer.write_u32(0);
        assert_eq!(
            buffer.try_record_patch_site(0, LabelUse::X86JmpRel32, 0),
            Err(AsmError::InvalidArch)
        );
        assert!(buffer.patch_sites.is_empty());

        let mut buffer = CodeBuffer::new(Environment::new(Arch::AArch64));
        let label = buffer.get_label();
        buffer.use_label_at_offset(0, label, LabelUse::RVJal20);
        assert_eq!(buffer.error(), Some(&AsmError::InvalidArch));
        assert!(buffer.pending_fixup_records.is_empty());

        let mut buffer = CodeBuffer::new(Environment::new(Arch::AArch64));
        let label = buffer.get_label();
        assert_eq!(
            buffer.emit_veneer(label, 0, LabelUse::RVJal20),
            Err(AsmError::InvalidArch)
        );
        assert!(buffer.data().is_empty());

        let mut buffer = CodeBuffer::new(Environment::new(Arch::AArch64));
        buffer.write_u32(0);
        assert_eq!(
            buffer.try_record_patch_site(0, LabelUse::A64Ldr12, 0),
            Err(AsmError::UnsupportedInstruction {
                reason: "AArch64 LDR12 label patching is not implemented",
            })
        );
    }

    #[cfg(not(feature = "x86"))]
    #[test]
    fn disabled_x86_label_use_is_rejected_without_a_fixup() {
        let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
        let label = buffer.get_label();
        buffer.write_u32(0);
        let bytes = buffer.data().to_vec();

        buffer.use_label_at_offset(0, label, LabelUse::X86JmpRel32);

        assert_eq!(buffer.error(), Some(&AsmError::InvalidArch));
        assert_eq!(buffer.data(), bytes);
        assert!(buffer.pending_fixup_records.is_empty());
        assert_eq!(buffer.finish().err(), Some(AsmError::InvalidArch));

        let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
        let label = buffer.get_label();
        buffer.add_reloc(Reloc::X86PCRel4, RelocTarget::Label(label), 0);
        assert_eq!(buffer.error(), Some(&AsmError::InvalidArch));
        assert!(buffer.relocs().is_empty());
    }

    #[cfg(not(feature = "riscv"))]
    #[test]
    fn disabled_riscv_label_use_is_rejected_without_a_fixup() {
        let mut buffer = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let label = buffer.get_label();
        buffer.write_u32(0);
        let bytes = buffer.data().to_vec();

        buffer.use_label_at_offset(0, label, LabelUse::RVPCRel32);

        assert_eq!(buffer.error(), Some(&AsmError::InvalidArch));
        assert_eq!(buffer.data(), bytes);
        assert!(buffer.pending_fixup_records.is_empty());
        assert_eq!(buffer.finish().err(), Some(AsmError::InvalidArch));

        let mut buffer = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let label = buffer.get_label();
        buffer.add_reloc(Reloc::RiscvCallPlt, RelocTarget::Label(label), 0);
        assert_eq!(buffer.error(), Some(&AsmError::InvalidArch));
        assert!(buffer.relocs().is_empty());
    }

    #[cfg(not(feature = "aarch64"))]
    #[test]
    fn disabled_aarch64_label_use_is_rejected_without_a_fixup() {
        let mut buffer = CodeBuffer::new(Environment::new(Arch::AArch64));
        let label = buffer.get_label();
        buffer.write_u32(0);
        let bytes = buffer.data().to_vec();

        buffer.use_label_at_offset(0, label, LabelUse::A64Branch26);

        assert_eq!(buffer.error(), Some(&AsmError::InvalidArch));
        assert_eq!(buffer.data(), bytes);
        assert!(buffer.pending_fixup_records.is_empty());
        assert_eq!(buffer.finish().err(), Some(AsmError::InvalidArch));

        let mut buffer = CodeBuffer::new(Environment::new(Arch::AArch64));
        let label = buffer.get_label();
        buffer.add_reloc(Reloc::Arm64Call, RelocTarget::Label(label), 0);
        assert_eq!(buffer.error(), Some(&AsmError::InvalidArch));
        assert!(buffer.relocs().is_empty());
    }

    #[test]
    fn poisoned_buffer_does_not_consume_pending_island_state() {
        let mut buffer = CodeBuffer::new(Environment::new(Arch::AArch64));
        buffer.add_constant(7u64);
        let pending_constants = buffer.pending_constants.len();
        buffer.record_error(AsmError::InvalidOperand);

        assert_eq!(buffer.emit_island(0), Err(AsmError::InvalidOperand));
        assert_eq!(buffer.pending_constants.len(), pending_constants);
        assert!(buffer.data().is_empty());
    }

    #[cfg(feature = "jit")]
    #[test]
    fn allocate_resolved_rejects_unresolved_symbols() {
        let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
        let symbol = buffer.add_symbol(ExternalName::user(0, 7), RelocDistance::Far);
        buffer.add_reloc(Reloc::Abs8, RelocTarget::Sym(symbol), 0);
        buffer.write_u64(0);
        let code = buffer.finish().unwrap();
        let mut allocator = JitAllocator::new(Default::default());

        let result = code.allocate_resolved(&mut allocator, |_| core::ptr::null());

        assert_eq!(result.err(), Some(AsmError::InvalidArgument));
    }

    #[cfg(feature = "jit")]
    #[test]
    fn allocate_resolved_resolves_user_external_names() {
        let mut buffer = CodeBuffer::new(Environment::new(Arch::X64));
        let symbol = buffer.extern_user(1, 2, RelocDistance::Far);
        buffer.add_reloc(Reloc::Abs8, RelocTarget::Sym(symbol), 0);
        buffer.write_u64(0);
        let code = buffer.finish().unwrap();
        let mut allocator = JitAllocator::new(Default::default());

        // Any non-null address is accepted; Abs8 just patches the pointer.
        let target = 0x1000usize as *const u8;
        let loaded = code
            .allocate_resolved(&mut allocator, |name| match name {
                ExternalName::User(u) if u.namespace == 1 && u.index == 2 => target,
                _ => core::ptr::null(),
            })
            .unwrap();

        unsafe {
            let patched = core::ptr::read_unaligned(loaded.rx() as *const usize);
            assert_eq!(patched, target as usize);
        }
    }

    #[test]
    fn extern_sym_deduplicates_by_name() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let first = buf.extern_sym("puts", RelocDistance::Far);
        let other = buf.extern_sym("printf", RelocDistance::Near);
        let again = buf.extern_sym("puts", RelocDistance::Near);

        assert_eq!(first, again);
        assert_ne!(first, other);
        // The first declaration's distance wins.
        assert_eq!(buf.symbol_distance(first), Some(RelocDistance::Far));
        assert_eq!(buf.symbol_distance(other), Some(RelocDistance::Near));
    }

    #[test]
    fn extern_user_deduplicates_by_namespace_and_index() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let first = buf.extern_user(0, 1, RelocDistance::Far);
        let other_ns = buf.extern_user(1, 1, RelocDistance::Near);
        let other_idx = buf.extern_user(0, 2, RelocDistance::Near);
        let again = buf.extern_user(0, 1, RelocDistance::Near);

        assert_eq!(first, again);
        assert_ne!(first, other_ns);
        assert_ne!(first, other_idx);
        assert_eq!(buf.symbol_distance(first), Some(RelocDistance::Far));
        assert_eq!(
            buf.symbol_name(first),
            Some(&ExternalName::user(0, 1))
        );
    }

    #[test]
    fn unknown_symbols_are_fallible() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let missing = Sym::from_id(u32::MAX);

        assert_eq!(buf.symbol_name(missing), None);
        assert_eq!(buf.symbol_distance(missing), None);

        let finalized = buf.finish().unwrap();
        assert_eq!(finalized.symbol_name(missing), None);
        assert_eq!(finalized.symbol_distance(missing), None);
    }

    #[test]
    fn defined_symbols_are_resolved_at_finish() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        buf.write_u8(0x90);
        let entry = buf.get_label();
        buf.bind_label(entry);
        buf.bind_symbol("entry", entry);
        buf.write_u8(0xC3);

        let result = buf.finish().unwrap();
        assert_eq!(result.defined_symbol_str("entry"), Some(1));
        assert_eq!(result.defined_symbol_str("missing"), None);
        assert_eq!(
            result.defined_symbol_offset(&ExternalName::from("entry")),
            Some(1)
        );
    }

    #[test]
    fn invalid_label_binding_is_reported_without_mutating_labels() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let invalid = Label::from_id(0);

        assert_eq!(buf.try_bind_label(invalid), Err(AsmError::InvalidArgument));
        assert_eq!(buf.label_count(), 0);

        buf.bind_label(invalid);
        assert_eq!(buf.error(), Some(&AsmError::InvalidArgument));
        assert!(matches!(buf.finish(), Err(AsmError::InvalidArgument)));
    }

    #[cfg(feature = "x86")]
    #[test]
    fn invalid_patch_registration_does_not_create_metadata() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));

        assert_eq!(
            buf.try_record_patch_site(0, LabelUse::X86JmpRel32, 0),
            Err(AsmError::InvalidArgument)
        );
        assert!(buf.patch_sites.is_empty());

        buf.record_patch_site(0, LabelUse::X86JmpRel32, 0);
        assert_eq!(buf.error(), Some(&AsmError::InvalidArgument));
        assert!(buf.patch_sites.is_empty());
    }

    #[cfg(feature = "x86")]
    #[test]
    fn finish_rejects_an_unbound_fixup() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let label = buf.get_label();
        buf.write_u32(0);
        buf.use_label_at_offset(0, label, LabelUse::X86JmpRel32);

        assert!(matches!(buf.finish(), Err(AsmError::UnboundLabel)));
    }

    #[cfg(feature = "x86")]
    #[test]
    fn unsupported_veneer_leaves_buffer_unchanged() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let label = buf.get_label();
        buf.write_u32(0);
        let original = buf.data().to_vec();

        assert_eq!(
            buf.emit_veneer(label, 0, LabelUse::X86JmpRel32),
            Err(AsmError::UnsupportedInstruction {
                reason: "branch range requires an unsupported veneer",
            })
        );
        assert_eq!(buf.data(), original);
    }

    #[test]
    fn branch_ranges_include_the_last_encodable_offset() {
        for (kind, range) in [
            (LabelUse::A64Branch14, 1 << 15),
            (LabelUse::A64Branch19, 1 << 20),
            (LabelUse::A64Branch26, 1 << 27),
        ] {
            assert!(kind.can_reach(0, range - 4));
            assert!(!kind.can_reach(0, range));
            assert!(kind.can_reach(range, 0));
            assert!(!kind.can_reach(range + 4, 0));
        }

        assert!(LabelUse::RVB12.can_reach(0, (1 << 12) - 2));
        assert!(!LabelUse::RVB12.can_reach(0, 1 << 12));
        assert!(LabelUse::RVB12.can_reach(1 << 12, 0));
        assert!(!LabelUse::RVB12.can_reach((1 << 12) + 2, 0));
    }

    #[test]
    fn supported_veneer_ranges_cover_both_exact_boundaries() {
        for (kind, positive, negative, step) in [
            (LabelUse::RVJal20, (1 << 20) - 2, 1 << 20, 2),
            (LabelUse::RVB12, (1 << 12) - 2, 1 << 12, 2),
            (LabelUse::RVCJump, (1 << 11) - 2, 1 << 11, 2),
        ] {
            assert!(kind.supports_veneer());
            assert!(kind.can_reach(0, positive));
            assert!(!kind.can_reach(0, positive + step));
            assert!(kind.can_reach(negative, 0));
            assert!(!kind.can_reach(negative + step, 0));
        }
    }

    #[cfg(feature = "riscv")]
    #[test]
    fn riscv_veneer_is_finalized_as_a_fixup() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
        let label = buf.get_label();
        buf.bind_label(label);
        buf.write_u32(0);

        buf.emit_veneer(label, 0, LabelUse::RVB12).unwrap();
        let code = buf.finish().unwrap();
        assert_eq!(code.data().len(), 12);
        // The source branch reaches the veneer inserted directly after it.
        assert_eq!(&code.data()[0..4], &[0x00, 0x02, 0x00, 0x00]);
    }

    #[cfg(feature = "riscv")]
    #[test]
    fn riscv_supported_veneers_are_inserted_by_islands() {
        for kind in [LabelUse::RVJal20, LabelUse::RVB12, LabelUse::RVCJump] {
            let mut buf = CodeBuffer::new(Environment::new(Arch::RISCV64));
            let label = buf.get_label();
            let patch_size = kind.patch_size();
            buf.get_appended_space(patch_size);
            buf.use_label_at_offset(0, label, kind);

            buf.emit_island(u32::MAX).unwrap();
            buf.bind_label(label);
            let code = buf.finish().unwrap();

            assert_eq!(code.data().len(), 12);
            assert!(kind.can_reach(0, 4));
        }
    }

    #[cfg(feature = "riscv")]
    #[test]
    fn riscv_large_images_hit_exact_branch_boundaries() {
        for (kind, positive, step) in [
            (LabelUse::RVJal20, (1 << 20) - 2, 2),
            (LabelUse::RVB12, (1 << 12) - 2, 2),
            (LabelUse::RVCJump, (1 << 11) - 2, 2),
        ] {
            let mut exact = CodeBuffer::new(Environment::new(Arch::RISCV64));
            let label = exact.get_label();
            exact.get_appended_space(kind.patch_size());
            exact.use_label_at_offset(0, label, kind);
            exact.get_appended_space(positive as usize - kind.patch_size());
            exact.bind_label(label);
            assert!(exact.finish().is_ok());

            let mut outside = CodeBuffer::new(Environment::new(Arch::RISCV64));
            let label = outside.get_label();
            outside.get_appended_space(kind.patch_size());
            outside.use_label_at_offset(0, label, kind);
            outside.get_appended_space((positive + step) as usize - kind.patch_size());
            outside.bind_label(label);
            assert_eq!(outside.finish().err(), Some(AsmError::TooLarge));
        }
    }

    #[cfg(feature = "aarch64")]
    #[test]
    fn out_of_range_aarch64_branch_reports_unsupported_veneer() {
        let mut buf = CodeBuffer::new(Environment::new(Arch::AArch64));
        let label = buf.get_label();
        buf.write_u32(0);
        buf.use_label_at_offset(0, label, LabelUse::A64Branch14);
        buf.get_appended_space((1 << 15) - 4);
        buf.bind_label(label);

        assert_eq!(
            buf.finish().err(),
            Some(AsmError::UnsupportedInstruction {
                reason: "AArch64 branch veneers are not implemented",
            })
        );
    }

    #[cfg(feature = "x86")]
    #[test]
    fn x86_branch_relaxation_honors_rel8_boundaries() {
        use crate::x86::{Assembler, JEmitter, JmpEmitter};

        let forward = |padding: usize, conditional: bool| {
            let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
            let target = buf.get_label();
            {
                let mut asm = Assembler::new(&mut buf);
                if conditional {
                    asm.jz(target);
                } else {
                    asm.jmp(target);
                }
            }
            for _ in 0..padding {
                buf.write_u8(0x90);
            }
            buf.bind_label(target);
            buf.finish().unwrap().data().to_vec()
        };
        for conditional in [false, true] {
            let short = forward(127, conditional);
            assert_eq!(short[0], if conditional { 0x74 } else { 0xEB });
            assert_eq!(short[1], 127);

            let near = forward(128, conditional);
            if conditional {
                assert_eq!(&near[..2], &[0x0F, 0x84]);
            } else {
                assert_eq!(near[0], 0xE9);
            }
        }

        let backward = |padding: usize, conditional: bool| {
            let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
            let target = buf.get_label();
            buf.bind_label(target);
            for _ in 0..padding {
                buf.write_u8(0x90);
            }
            {
                let mut asm = Assembler::new(&mut buf);
                if conditional {
                    asm.jz(target);
                } else {
                    asm.jmp(target);
                }
            }
            buf.finish().unwrap().data().to_vec()
        };
        for conditional in [false, true] {
            let short = backward(126, conditional);
            assert_eq!(
                &short[126..],
                &[if conditional { 0x74 } else { 0xEB }, 0x80]
            );

            let near = backward(127, conditional);
            assert_eq!(near[127], if conditional { 0x0F } else { 0xE9 });
        }
    }

    #[cfg(feature = "x86")]
    #[test]
    fn x86_branch_relaxation_reaches_a_bounded_fixed_point() {
        use crate::x86::{Assembler, JmpEmitter};

        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let target = buf.get_label();
        {
            let mut asm = Assembler::new(&mut buf);
            asm.jmp(target);
        }
        for _ in 0..120 {
            buf.write_u8(0x90);
        }
        {
            let mut asm = Assembler::new(&mut buf);
            asm.jmp(target);
        }
        for _ in 0..3 {
            buf.write_u8(0x90);
        }
        buf.bind_label(target);

        let code = buf.finish().unwrap();
        assert_eq!(&code.data()[..2], &[0xEB, 125]);
        assert_eq!(&code.data()[122..124], &[0xEB, 3]);
        assert_eq!(code.label_offsets[target.id() as usize], 127);
    }

    #[cfg(feature = "x86")]
    #[test]
    fn x86_branch_relaxation_rebases_metadata_and_is_deterministic() {
        use crate::x86::{Assembler, JmpEmitter};

        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let target = buf.get_label();
        {
            let mut asm = Assembler::new(&mut buf);
            asm.jmp(target);
        }
        buf.write_u8(0x90);
        buf.bind_label(target);
        buf.bind_symbol("target", target);

        buf.add_reloc(Reloc::Abs4, RelocTarget::Label(target), 0);
        buf.write_u32(0);
        let block = buf.reserve_patch_block(4, 1).unwrap();
        let block_offset = block.offset();
        let site = buf
            .try_record_patch_site(
                block_offset,
                LabelUse::X86JmpRel32,
                buf.label_offset(target),
            )
            .unwrap();

        let later = buf.get_label();
        {
            let mut asm = Assembler::new(&mut buf);
            asm.long().jmp(later);
        }
        buf.write_u8(0x90);
        buf.bind_label(later);

        let first = buf.finish().unwrap();
        assert_eq!(&first.data()[..3], &[0xEB, 1, 0x90]);
        assert_eq!(first.defined_symbol_str("target"), Some(3));
        assert_eq!(first.relocs()[0].offset, 3);
        assert_eq!(first.patch_catalog().block(PatchBlockId::from_index(0)).unwrap().offset, 7);
        let patch_site = first.patch_catalog().site(site).unwrap();
        assert_eq!(patch_site.offset, 7);
        assert_eq!(patch_site.current_target, 3);
        assert_eq!(&first.data()[11..], &[0xE9, 1, 0, 0, 0, 0x90]);

        let second = buf.finish().unwrap();
        assert_eq!(second.data(), first.data());
        assert_eq!(second.relocs(), first.relocs());
        assert_eq!(second.patch_catalog(), first.patch_catalog());
    }

    #[cfg(feature = "x86")]
    #[test]
    fn x86_branch_relaxation_preserves_recorded_alignment() {
        use crate::x86::{Assembler, JmpEmitter};

        let mut buf = CodeBuffer::new(Environment::new(Arch::X64));
        let target = buf.get_label();
        {
            let mut asm = Assembler::new(&mut buf);
            asm.jmp(target);
        }
        buf.try_align_to(16).unwrap();
        buf.bind_label(target);

        let code = buf.finish().unwrap();
        assert_eq!(code.data()[0], 0xE9);
        assert_eq!(code.label_offsets[target.id() as usize], 16);
    }
}
