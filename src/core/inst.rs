//! Generic instruction representation.
//!
//! [`Inst`] is the asmkit equivalent of AsmJit's `BaseInst` + trailing operand array: an
//! architecture-namespaced instruction id, options, an optional extra register, and up to
//! [`MAX_OP_COUNT`] operands stored inline. It is the node type recorded by a Builder
//! (see `core::builder`) and consumed by `query_rw_info`-style APIs, while the per-mnemonic
//! emitter methods remain the ergonomic way to construct one.

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
    /// Instruction id with modifiers (architecture-namespaced).
    pub id: u32,
    /// Instruction options.
    pub options: InstOptions,
    /// Extra register used by the instruction (REP register or AVX-512 mask selector).
    pub extra_reg: Operand,
    /// Number of valid entries in `operands`.
    op_count: u8,
    /// Operands; only the first `op_count` entries are meaningful.
    pub operands: [Operand; MAX_OP_COUNT],
}

impl Default for Inst {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Inst {
    /// Creates a new instruction with the given `id` and no operands.
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            options: InstOptions::NONE,
            extra_reg: Operand::new(),
            op_count: 0,
            operands: [Operand::new(); MAX_OP_COUNT],
        }
    }

    /// Creates a new instruction with the given `id` and operands.
    ///
    /// Debug-asserts that `ops` fits into the inline operand array.
    pub fn with_operands(id: u32, ops: &[Operand]) -> Self {
        debug_assert!(ops.len() <= MAX_OP_COUNT);
        let mut inst = Self::new(id);
        inst.operands[..ops.len().min(MAX_OP_COUNT)].copy_from_slice(ops);
        inst.op_count = ops.len().min(MAX_OP_COUNT) as u8;
        inst
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
    pub fn set_operand(&mut self, index: usize, op: Operand) {
        debug_assert!(index < self.op_count());
        if index < self.op_count() {
            self.operands[index] = op;
        }
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
        self.op_count = ops.len() as u8;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_and_mutate() {
        let mut inst = Inst::new(42);
        assert_eq!(inst.id, 42);
        assert_eq!(inst.op_count(), 0);
        assert!(inst.operands().is_empty());

        inst.add_operand(Operand::new()).unwrap();
        inst.add_operand(Operand::new()).unwrap();
        assert_eq!(inst.op_count(), 2);

        inst.set_operands(&[Operand::new(); 3]).unwrap();
        assert_eq!(inst.op_count(), 3);

        for _ in 0..3 {
            inst.add_operand(Operand::new()).unwrap();
        }
        assert_eq!(inst.op_count(), MAX_OP_COUNT);
        assert!(inst.add_operand(Operand::new()).is_err());
    }
}
