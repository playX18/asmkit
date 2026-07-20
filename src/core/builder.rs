//! Deferred instruction builder.
//!
//! A [`Builder`] records a sequence of nodes (instructions and label-bind points) so passes
//! can inspect and mutate them — most importantly a future register-allocation pass — before
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

/// Sink that consumes replayed nodes — implemented by each architecture's `Assembler`.
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

#[cfg(test)]
mod builder_tests {
    use super::*;

    struct TestSink {
        arch: Arch,
        calls: usize,
        fail: bool,
    }

    impl InstSink for TestSink {
        fn arch(&self) -> Arch {
            self.arch
        }

        fn emit_inst(&mut self, _: &Inst) -> Result<(), AsmError> {
            self.calls += 1;
            if self.fail {
                Err(AsmError::InvalidOperand)
            } else {
                Ok(())
            }
        }

        fn bind_label(&mut self, _: Label) -> Result<(), AsmError> {
            self.calls += 1;
            Ok(())
        }
    }

    #[test]
    fn wrong_architecture_stops_before_replay() {
        let mut builder = Builder::for_arch(Arch::AArch64);
        builder
            .push_inst(Inst::with_arch_operands(Arch::AArch64, 1, &[]).unwrap())
            .unwrap();
        let mut sink = TestSink {
            arch: Arch::X64,
            calls: 0,
            fail: false,
        };

        assert_eq!(builder.emit_into(&mut sink), Err(AsmError::InvalidArch));
        assert_eq!(sink.calls, 0);
    }

    #[test]
    fn replay_stops_at_the_first_failure() {
        let mut builder = Builder::for_arch(Arch::X64);
        let inst = Inst::with_arch_operands(Arch::X64, 1, &[]).unwrap();
        builder.push_inst(inst).unwrap();
        builder.push_label(Label::from_id(0));
        builder.push_inst(inst).unwrap();
        let mut sink = TestSink {
            arch: Arch::X64,
            calls: 0,
            fail: true,
        };

        assert_eq!(builder.emit_into(&mut sink), Err(AsmError::InvalidOperand));
        assert_eq!(sink.calls, 1);
    }

    #[test]
    fn replacement_checks_the_instruction_architecture() {
        let mut builder = Builder::for_arch(Arch::RISCV64);
        builder
            .push_inst(Inst::with_arch_operands(Arch::RISCV64, 1, &[]).unwrap())
            .unwrap();
        assert_eq!(
            builder.replace_inst(0, Inst::with_arch_operands(Arch::AArch64, 1, &[]).unwrap()),
            Err(AsmError::InvalidArch)
        );
    }
}

#[cfg(all(test, feature = "aarch64"))]
mod tests {
    use super::*;
    use crate::aarch64::instdb::InstId;
    use crate::aarch64::*;
    use crate::core::buffer::CodeBuffer;
    use crate::core::operand::OperandCast;
    use crate::core::target::Environment;
    use std::vec::Vec;

    fn a64_inst(id: u32, operands: &[crate::core::operand::Operand]) -> Inst {
        Inst::with_arch_operands(crate::core::arch_traits::Arch::AArch64, id, operands).unwrap()
    }

    fn build_direct() -> Vec<u8> {
        let mut buf = CodeBuffer::new(Environment::new(crate::core::arch_traits::Arch::AArch64));
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

        buf.finish().unwrap().data().to_vec()
    }

    fn build_deferred() -> Vec<u8> {
        let mut buf = CodeBuffer::new(Environment::new(crate::core::arch_traits::Arch::AArch64));
        let mut builder = Builder::for_arch(crate::core::arch_traits::Arch::AArch64);
        let done = buf.get_label();

        builder
            .push_inst(a64_inst(
                InstId::Stp as u32,
                &[
                    *x29.as_operand(),
                    *x30.as_operand(),
                    *ptr(sp, 0).pre_offset(-32).as_operand(),
                ],
            ))
            .unwrap();
        builder
            .push_inst(a64_inst(
                InstId::Mov as u32,
                &[*x29.as_operand(), *sp.as_operand()],
            ))
            .unwrap();
        builder
            .push_inst(a64_inst(
                InstId::Add as u32,
                &[*x0.as_operand(), *x1.as_operand(), *x2.as_operand()],
            ))
            .unwrap();
        builder
            .push_inst(a64_inst(
                InstId::Cmp as u32,
                &[*x0.as_operand(), *imm(0).as_operand()],
            ))
            .unwrap();
        builder
            .push_inst(a64_inst(
                InstId::B.with_cc(crate::core::globals::CondCode::EQ),
                &[*done.as_operand()],
            ))
            .unwrap();
        builder
            .push_inst(a64_inst(
                InstId::Sub as u32,
                &[*x0.as_operand(), *x0.as_operand(), *imm(1).as_operand()],
            ))
            .unwrap();
        builder.push_label(done);
        builder
            .push_inst(a64_inst(
                InstId::Ldp as u32,
                &[
                    *x29.as_operand(),
                    *x30.as_operand(),
                    *ptr(sp, 0).post_offset(32).as_operand(),
                ],
            ))
            .unwrap();
        builder
            .push_inst(a64_inst(InstId::Ret as u32, &[*lr.as_operand()]))
            .unwrap();

        {
            let mut asm = Assembler::new(&mut buf);
            builder.emit_into(&mut asm).unwrap();
        }
        buf.finish().unwrap().data().to_vec()
    }

    #[test]
    fn replay_matches_direct_assembly() {
        assert_eq!(build_direct(), build_deferred());
    }
}
