//! Deferred instruction builder.
//!
//! A [`Builder`] records a sequence of nodes (instructions and label-bind points) so passes
//! can inspect and mutate them: most importantly a future register-allocation pass: before
//! machine code is produced. Replaying a builder into an [`InstSink`] (implemented by each
//! architecture's `Assembler`) emits the exact same bytes as direct assembly: labels and
//! relocations are recorded at emit time and resolved by `CodeBuffer::finish()` as usual.

use smallvec::SmallVec;

use crate::AsmError;

use super::arch_traits::Arch;
use super::inst::Inst;
use super::operand::Label;

/// A node recorded by a [`Builder`].
#[derive(Clone, Copy, Debug)]
pub enum Node {
    /// An instruction with its operands.
    Inst(Inst),
    /// Binds a label at this position when replayed.
    Label(Label),
}

/// Sink that consumes replayed nodes: implemented by each architecture's `Assembler`.
pub trait InstSink {
    /// Target architecture accepted by this sink.
    fn arch(&self) -> Arch;

    /// Emits one recorded instruction.
    fn emit_inst(&mut self, inst: &Inst) -> Result<(), AsmError>;

    /// Binds a label at the current position.
    fn bind_label(&mut self, label: Label) -> Result<(), AsmError>;
}

/// Records instructions and label-bind points for deferred emission.
#[derive(Clone, Debug)]
pub struct Builder {
    arch: Arch,
    nodes: SmallVec<[Node; 32]>,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::for_arch(Arch::HOST)
    }

    /// Creates a builder that accepts instructions for `arch` only.
    pub fn for_arch(arch: Arch) -> Self {
        Self {
            arch,
            nodes: SmallVec::new(),
        }
    }

    /// Architecture accepted by this builder.
    pub const fn arch(&self) -> Arch {
        self.arch
    }

    /// Returns the number of recorded nodes.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Tests whether the builder is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Removes all recorded nodes.
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    /// Records an instruction.
    pub fn push_inst(&mut self, inst: Inst) -> Result<(), AsmError> {
        if inst.arch() != self.arch {
            return Err(AsmError::InvalidArch);
        }
        self.nodes.push(Node::Inst(inst));
        Ok(())
    }

    /// Records a label-bind point.
    pub fn push_label(&mut self, label: Label) {
        self.nodes.push(Node::Label(label));
    }

    /// Returns the recorded nodes.
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Replaces an instruction node after checking that it belongs to this builder.
    pub fn replace_inst(&mut self, index: usize, inst: Inst) -> Result<(), AsmError> {
        if inst.arch() != self.arch {
            return Err(AsmError::InvalidArch);
        }
        let Some(node) = self.nodes.get_mut(index) else {
            return Err(AsmError::InvalidArgument);
        };
        if !matches!(node, Node::Inst(_)) {
            return Err(AsmError::InvalidState);
        }
        *node = Node::Inst(inst);
        Ok(())
    }

    /// Replays all recorded nodes into `sink`, in order.
    pub fn emit_into<S: InstSink + ?Sized>(&self, sink: &mut S) -> Result<(), AsmError> {
        if sink.arch() != self.arch {
            return Err(AsmError::InvalidArch);
        }
        for node in self.nodes.iter() {
            match node {
                Node::Inst(inst) => sink.emit_inst(inst)?,
                Node::Label(label) => sink.bind_label(*label)?,
            }
        }
        Ok(())
    }
}
