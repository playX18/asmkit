//! Generic instruction representation.
//!
//! [`Inst`] is the asmkit equivalent of AsmJit's `BaseInst` + trailing operand array: an
//! architecture-namespaced instruction id, options, an optional extra register, and up to
//! [`MAX_OP_COUNT`] operands stored inline. It is the node type recorded by a Builder
//! (see `core::builder`) and consumed by `query_rw_info`-style APIs, while the per-mnemonic
//! emitter methods remain the ergonomic way to construct one.

use super::arch_traits::Arch;
use super::globals::{InstOptions, MAX_OP_COUNT};
use super::operand::Operand;
use crate::AsmError;

/// A generic instruction: id + options + extra register + inline operand array.
///
/// The id is architecture-namespaced (`x86::InstId`, `aarch64::InstId`, `riscv::Opcode`,
/// all cast to `u32`); architectures may pack modifiers into high bits of the id (AArch64
/// packs a [`super::globals::CondCode`]). `extra_reg` carries the REP register or AVX-512
/// `{k}` selector where applicable and is [`Operand::new()`] (none) otherwise.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Inst {
    arch: Arch,
    /// Instruction id with modifiers (architecture-namespaced).
    pub(crate) id: u32,
    /// Instruction options.
    pub(crate) options: InstOptions,
    /// Extra register used by the instruction (REP register or AVX-512 mask selector).
    pub(crate) extra_reg: Operand,
    /// Number of valid entries in `operands`.
    op_count: u8,
    /// Operands; only the first `op_count` entries are meaningful.
    operands: [Operand; MAX_OP_COUNT],
}

impl Inst {
    /// Creates a host-tagged instruction with the given `id` and no operands.
    /// Creates an instruction for a supported assembler architecture.
    pub(crate) const fn new_for(arch: Arch, id: u32) -> Self {
        Self {
            arch,
            id,
            options: InstOptions::NONE,
            extra_reg: Operand::new(),
            op_count: 0,
            operands: [Operand::new(); MAX_OP_COUNT],
        }
    }

    /// Creates a host-tagged instruction with the given `id` and operands.
    /// Creates an architecture-tagged instruction without operands.
    pub fn for_arch(arch: Arch, id: u32) -> Result<Self, AsmError> {
        Self::with_arch_operands(arch, id, &[])
    }

    /// Creates an architecture-tagged instruction without dropping operands.
    pub fn with_arch_operands(arch: Arch, id: u32, ops: &[Operand]) -> Result<Self, AsmError> {
        if !matches!(
            arch,
            Arch::X86 | Arch::X64 | Arch::AArch64 | Arch::RISCV32 | Arch::RISCV64
        ) {
            return Err(AsmError::InvalidArch);
        }
        let mut inst = Self::new_for(arch, id);
        inst.set_operands(ops)?;
        Ok(inst)
    }

    /// Architecture this instruction belongs to.
    pub const fn arch(&self) -> Arch {
        self.arch
    }

    /// Instruction id with architecture-specific modifiers.
    pub const fn id(&self) -> u32 {
        self.id
    }

    /// Encoding options retained by deferred replay.
    pub const fn options(&self) -> InstOptions {
        self.options
    }

    /// Optional extra register retained by deferred replay.
    pub const fn extra_reg(&self) -> Operand {
        self.extra_reg
    }

    /// Sets encoding options retained by deferred replay.
    pub fn set_options(&mut self, options: InstOptions) {
        self.options = options;
    }

    /// Sets the optional extra register retained by deferred replay.
    pub fn set_extra_reg(&mut self, extra_reg: Operand) {
        self.extra_reg = extra_reg;
    }

    /// Returns the number of operands.
    pub const fn op_count(&self) -> usize {
        self.op_count as usize
    }

    /// Returns the operands as a slice.
    pub fn operands(&self) -> &[Operand] {
        self.operands.split_at(self.op_count as usize).0
    }

    /// Returns the operands as a mutable slice.
    pub fn operands_mut(&mut self) -> &mut [Operand] {
        self.operands.split_at_mut(self.op_count as usize).0
    }

    /// Returns the operand at `index`, or `None` if out of range.
    pub fn operand(&self, index: usize) -> Option<&Operand> {
        self.operands().get(index)
    }

    /// Sets the operand at `index`. `index` must be less than `op_count`.
    pub fn set_operand(&mut self, index: usize, op: Operand) -> Result<(), AsmError> {
        let Some(slot) = self.operands_mut().get_mut(index) else {
            return Err(AsmError::InvalidArgument);
        };
        *slot = op;
        Ok(())
    }

    /// Appends an operand, or returns [`AsmError::InvalidState`] if the operand array is full.
    pub fn add_operand(&mut self, op: Operand) -> Result<(), AsmError> {
        if self.op_count() >= MAX_OP_COUNT {
            return Err(AsmError::InvalidState);
        }
        self.operands[self.op_count()] = op;
        self.op_count += 1;
        Ok(())
    }

    /// Sets all operands from `ops`, replacing the current operand list.
    ///
    /// Returns [`AsmError::InvalidState`] if `ops` doesn't fit into the operand array.
    pub fn set_operands(&mut self, ops: &[Operand]) -> Result<(), AsmError> {
        if ops.len() > MAX_OP_COUNT {
            return Err(AsmError::InvalidState);
        }
        self.operands[..ops.len()].copy_from_slice(ops);
        self.operands[ops.len()..].fill(Operand::new());
        self.op_count = ops.len() as u8;
        Ok(())
    }
}
