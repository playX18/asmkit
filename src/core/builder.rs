//! Deferred instruction builder.
//!
//! A [`Builder`] records a sequence of nodes (instructions and label-bind points) so passes
//! can inspect and mutate them — most importantly a future register-allocation pass — before
//! machine code is produced. Replaying a builder into an [`InstSink`] (implemented by each
//! architecture's `Assembler`) emits the exact same bytes as direct assembly: labels and
//! relocations are recorded at emit time and resolved by `CodeBuffer::finish()` as usual.

use smallvec::SmallVec;

use crate::AsmError;

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

/// Sink that consumes replayed nodes — implemented by each architecture's `Assembler`.
pub trait InstSink {
    /// Emits one recorded instruction.
    fn emit_inst(&mut self, inst: &Inst);

    /// Binds a label at the current position.
    fn bind_label(&mut self, label: Label);
}

/// Records instructions and label-bind points for deferred emission.
#[derive(Clone, Debug, Default)]
pub struct Builder {
    nodes: SmallVec<[Node; 32]>,
}

impl Builder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
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
    pub fn push_inst(&mut self, inst: Inst) {
        self.nodes.push(Node::Inst(inst));
    }

    /// Records a label-bind point.
    pub fn push_label(&mut self, label: Label) {
        self.nodes.push(Node::Label(label));
    }

    /// Returns the recorded nodes.
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Returns the recorded nodes mutably (for mutation passes).
    pub fn nodes_mut(&mut self) -> &mut [Node] {
        &mut self.nodes
    }

    /// Replays all recorded nodes into `sink`, in order.
    pub fn emit_into<S: InstSink + ?Sized>(&self, sink: &mut S) -> Result<(), AsmError> {
        for node in self.nodes.iter() {
            match node {
                Node::Inst(inst) => sink.emit_inst(inst),
                Node::Label(label) => sink.bind_label(*label),
            }
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "aarch64"))]
mod tests {
    use super::*;
    use crate::aarch64::instdb::InstId;
    use crate::aarch64::*;
    use crate::core::buffer::CodeBuffer;
    use crate::core::operand::OperandCast;
    use std::vec::Vec;

    fn build_direct() -> Vec<u8> {
        let mut buf = CodeBuffer::new();
        let mut asm = Assembler::new(&mut buf);
        let done = asm.buffer.get_label();

        asm.stp(x29, x30, ptr(sp, 0).pre_offset(-32));
        asm.mov(x29, sp);
        asm.add(x0, x1, x2);
        asm.cmp(x0, imm(0));
        asm.b_eq(done);
        asm.sub(x0, x0, imm(1));
        asm.buffer.bind_label(done);
        asm.ldp(x29, x30, ptr(sp, 0).post_offset(32));
        asm.ret(lr);

        buf.finish().data().to_vec()
    }

    fn build_deferred() -> Vec<u8> {
        let mut buf = CodeBuffer::new();
        let mut builder = Builder::new();
        let done = buf.get_label();

        builder.push_inst(Inst::with_operands(
            InstId::Stp as u32,
            &[
                *x29.as_operand(),
                *x30.as_operand(),
                *ptr(sp, 0).pre_offset(-32).as_operand(),
            ],
        ));
        builder.push_inst(Inst::with_operands(
            InstId::Mov as u32,
            &[*x29.as_operand(), *sp.as_operand()],
        ));
        builder.push_inst(Inst::with_operands(
            InstId::Add as u32,
            &[*x0.as_operand(), *x1.as_operand(), *x2.as_operand()],
        ));
        builder.push_inst(Inst::with_operands(
            InstId::Cmp as u32,
            &[*x0.as_operand(), *imm(0).as_operand()],
        ));
        builder.push_inst(Inst::with_operands(
            InstId::B.with_cc(crate::core::globals::CondCode::EQ),
            &[*done.as_operand()],
        ));
        builder.push_inst(Inst::with_operands(
            InstId::Sub as u32,
            &[*x0.as_operand(), *x0.as_operand(), *imm(1).as_operand()],
        ));
        builder.push_label(done);
        builder.push_inst(Inst::with_operands(
            InstId::Ldp as u32,
            &[
                *x29.as_operand(),
                *x30.as_operand(),
                *ptr(sp, 0).post_offset(32).as_operand(),
            ],
        ));
        builder.push_inst(Inst::with_operands(InstId::Ret as u32, &[*lr.as_operand()]));

        {
            let mut asm = Assembler::new(&mut buf);
            builder.emit_into(&mut asm).unwrap();
        }
        buf.finish().data().to_vec()
    }

    #[test]
    fn replay_matches_direct_assembly() {
        assert_eq!(build_direct(), build_deferred());
    }
}
