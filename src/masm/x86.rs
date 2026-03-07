use crate::{
    core::{
        emitter::Emitter,
        operand::{Imm, Label, OperandCast},
        target::X86Feature,
    },
    masm::{DoubleCondition, ExtensionSize, ResultCondition},
    x86::{
        opcodes::{JCC, SETCC8R},
        *,
    },
};

pub struct MacroAssemblerX86<'a> {
    pub assembler: Assembler<'a>,
}

impl<'a> MacroAssemblerX86<'a> {
    pub fn new(assembler: Assembler<'a>) -> Self {
        Self { assembler }
    }

    pub const fn scratch_register() -> Gpq {
        SCRATCH_REGISTER
    }

    pub fn bind_label(&mut self, label: Label) {
        self.assembler.bind_label(label);
    }

    pub fn supports_avx(&self) -> bool {
        self.assembler.buffer.env().x86_feature(X86Feature::AVX)
    }

    pub fn supports_avx2(&self) -> bool {
        self.assembler.buffer.env().x86_feature(X86Feature::AVX2)
    }

    pub fn supports_avx512f(&self) -> bool {
        self.assembler.buffer.env().x86_feature(X86Feature::AVX512F)
    }

    pub fn add32(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            self.assembler
                .lea32(*dst, ptr64_index(&lhs.as_::<Gp>(), &rhs.as_::<Gp>(), 1, 0));
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.lea32(
                *dst,
                ptr64(&lhs.as_::<Gp>(), rhs.as_::<Imm>().value() as i32),
            );
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.add32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32(*dst, *lhs);
            self.assembler.add32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov32(*dst, *lhs);
            self.assembler.add32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                rhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            self.assembler.add32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                rhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            self.assembler.add32(*dst, *rhs);
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vaddss(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_addss(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vaddss(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_addss(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported add combination");
        }
    }

    pub fn add64(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            self.assembler
                .lea64(*dst, ptr64_index(&lhs.as_::<Gp>(), &rhs.as_::<Gp>(), 1, 0));
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.lea64(
                    *dst,
                    ptr64(&lhs.as_::<Gp>(), rhs.as_::<Imm>().value() as i32),
                );
            } else {
                if dst.id() != lhs.id() {
                    self.mov(*dst, *lhs);
                }

                self.assembler.add64(*dst, *rhs);
            }
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.add64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.add64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.add64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                rhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            self.assembler.add64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                rhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.add64(*dst, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.add64(*dst, Self::scratch_register());
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vaddsd(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_addsd(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vaddsd(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_addsd(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported add combination");
        }
    }

    pub fn ret(&mut self) {
        self.assembler.ret()
    }

    pub fn push(&mut self, op: impl OperandCast) {
        self.assembler.push(*op.as_operand())
    }

    pub fn pop(&mut self, op: impl OperandCast) {
        self.assembler.pop(*op.as_operand())
    }

    pub fn get_label(&mut self) -> Label {
        self.assembler.get_label()
    }

    pub fn sub32(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub32(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub32(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32(*dst, *lhs);
            self.assembler.sub32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov32(*dst, *lhs);
            self.assembler.sub32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub32(*dst, *rhs);
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vsubss(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_subss(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vsubss(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_subss(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported sub combination");
        }
    }

    pub fn sub64(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub64(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub64(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.sub64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.sub64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.sub64(*dst, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.sub64(*dst, Self::scratch_register());
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vsubsd(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_subsd(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vsubsd(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_subsd(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported sub combination");
        }
    }

    pub fn and32(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and32(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and32(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32(*dst, *lhs);
            self.assembler.and32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov32(*dst, *lhs);
            self.assembler.and32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            self.assembler.and32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            self.assembler.and32(*dst, *rhs);
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vandps128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_andps(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vandps128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_andps(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported and combination");
        }
    }

    pub fn and64(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and64(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and64(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.and64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.and64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            self.assembler.and64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.and64(*dst, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.and64(*dst, Self::scratch_register());
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vandpd128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_andpd(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vandpd128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_andpd(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported and combination");
        }
    }

    pub fn or32(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or32(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or32(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32(*dst, *lhs);
            self.assembler.or32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov32(*dst, *lhs);
            self.assembler.or32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            self.assembler.or32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            self.assembler.or32(*dst, *rhs);
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vorps128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_orps(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vorps128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_orps(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported or combination");
        }
    }

    pub fn or64(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or64(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or64(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.or64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.or64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            self.assembler.or64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.or64(*dst, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.or64(*dst, Self::scratch_register());
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vorpd128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_orpd(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vorpd128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_orpd(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported or combination");
        }
    }

    pub fn xor32(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor32(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor32(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32(*dst, *lhs);
            self.assembler.xor32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov32(*dst, *lhs);
            self.assembler.xor32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            self.assembler.xor32(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            self.assembler.xor32(*dst, *rhs);
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vxorps128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_xorps(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vxorps128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_xorps(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported xor combination");
        }
    }

    pub fn xor64(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor64(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor64(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.xor64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.xor64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            self.assembler.xor64(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.xor64(*dst, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.xor64(*dst, Self::scratch_register());
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vxorpd128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_xorpd(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vxorpd128(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_xorpd(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported xor combination");
        }
    }

    pub fn mul32(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.assembler.mov32(*dst, *lhs);
            }

            self.assembler.imul32_2(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.imul32_3(*dst, *lhs, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                if dst.id() != lhs.id() {
                    self.assembler.mov32(*dst, *lhs);
                }
                self.assembler.imul32_2(*dst, Self::scratch_register());
            }
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.assembler.mov32(*dst, *lhs);
            }

            self.assembler.imul32_2(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32(Self::scratch_register(), *lhs);
            self.assembler.imul32_2(Self::scratch_register(), *rhs);
            self.assembler.mov32(*dst, Self::scratch_register());
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler
                    .imul32_3(Self::scratch_register(), *lhs, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.imul32_2(Self::scratch_register(), *lhs);
            }
            self.assembler.mov32(*dst, Self::scratch_register());
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_mem() {
            self.assembler.mov32(Self::scratch_register(), *lhs);
            self.assembler.imul32_2(Self::scratch_register(), *rhs);
            self.assembler.mov32(*dst, Self::scratch_register());
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vmulss(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_mulss(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vmulss(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_mulss(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported mul combination");
        }
    }

    pub fn mul64(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.assembler.mov64(*dst, *lhs);
            }

            self.assembler.imul64_2(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.imul64_3(*dst, *lhs, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                if dst.id() != lhs.id() {
                    self.assembler.mov64(*dst, *lhs);
                }
                self.assembler.imul64_2(*dst, Self::scratch_register());
            }
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.assembler.mov64(*dst, *lhs);
            }

            self.assembler.imul64_2(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64(Self::scratch_register(), *lhs);
            self.assembler.imul64_2(Self::scratch_register(), *rhs);
            self.assembler.mov64(*dst, Self::scratch_register());
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler
                    .imul64_3(Self::scratch_register(), *lhs, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.imul64_2(Self::scratch_register(), *lhs);
            }
            self.assembler.mov64(*dst, Self::scratch_register());
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_mem() {
            self.assembler.mov64(Self::scratch_register(), *lhs);
            self.assembler.imul64_2(Self::scratch_register(), *rhs);
            self.assembler.mov64(*dst, Self::scratch_register());
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vmulsd(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_mulsd(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vmulsd(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_mulsd(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported mul combination");
        }
    }

    pub fn div32(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vdivss(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_divss(*dst, *rhs);
            }
            return;
        }

        if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vdivss(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_divss(*dst, *rhs);
            }
            return;
        }

        if !((dst.is_gp() || dst.is_mem()) && (lhs.is_gp() || lhs.is_mem())) {
            unimplemented!("Unsupported div combination");
        }

        let preserve_rax_via_dst = dst.is_gp()
            && dst.id() != RAX.id()
            && dst.id() != RDX.id()
            && !(rhs.is_gp() && rhs.id() == dst.id());

        let divisor = if rhs.is_imm() {
            self.assembler.mov64(Self::scratch_register(), *rhs);
            *Self::scratch_register().as_operand()
        } else if rhs.is_gp() {
            let rhs_gp = rhs.as_::<Gp>();
            if rhs_gp.id() == RAX.id() || rhs_gp.id() == RDX.id() {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                *Self::scratch_register().as_operand()
            } else {
                *rhs
            }
        } else if rhs.is_mem() {
            *rhs
        } else {
            unimplemented!("Unsupported div combination");
        };

        if preserve_rax_via_dst {
            self.assembler.xchg32(EAX, *dst);
        }

        if lhs.is_gp() {
            if !(preserve_rax_via_dst && lhs.id() == dst.id()) {
                self.assembler.mov32(RAX, *lhs);
            }
        } else {
            self.assembler.mov32(RAX, *lhs);
        }

        self.assembler.cdq();

        self.assembler.idiv32(divisor);

        if dst.is_gp() {
            if preserve_rax_via_dst {
                self.assembler.xchg32(EAX, *dst);
            } else {
                self.assembler.mov32(*dst, RAX);
            }
        } else {
            self.assembler.mov32(*dst, RAX);
        }
    }

    pub fn div64(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vdivsd(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_divsd(*dst, *rhs);
            }
            return;
        }

        if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vdivsd(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_divsd(*dst, *rhs);
            }
            return;
        }

        if !((dst.is_gp() || dst.is_mem()) && (lhs.is_gp() || lhs.is_mem())) {
            unimplemented!("Unsupported div combination");
        }

        let preserve_rax_via_dst = dst.is_gp()
            && dst.id() != RAX.id()
            && dst.id() != RDX.id()
            && !(rhs.is_gp() && rhs.id() == dst.id());

        let divisor = if rhs.is_imm() {
            self.assembler.mov64(Self::scratch_register(), *rhs);
            *Self::scratch_register().as_operand()
        } else if rhs.is_gp() {
            let rhs_gp = rhs.as_::<Gp>();
            if rhs_gp.id() == RAX.id() || rhs_gp.id() == RDX.id() {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                *Self::scratch_register().as_operand()
            } else {
                *rhs
            }
        } else if rhs.is_mem() {
            *rhs
        } else {
            unimplemented!("Unsupported div combination");
        };

        if preserve_rax_via_dst {
            self.assembler.xchg64(RAX, *dst);
        }

        if lhs.is_gp() {
            if !(preserve_rax_via_dst && lhs.id() == dst.id()) {
                self.assembler.mov64(RAX, *lhs);
            }
        } else {
            self.assembler.mov64(RAX, *lhs);
        }

        self.assembler.cqo();

        self.assembler.idiv64(divisor);

        if dst.is_gp() {
            if preserve_rax_via_dst {
                self.assembler.xchg64(RAX, *dst);
            } else {
                self.assembler.mov64(*dst, RAX);
            }
        } else {
            self.assembler.mov64(*dst, RAX);
        }
    }

    pub fn mov(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        if dst.is_gp() && src.is_gp() {
            if dst.id() != src.id() {
                self.assembler.mov64(*dst, *src);
            }
        } else if dst.is_gp() && src.is_imm() {
            self.assembler.mov64(*dst, *src);
        } else if dst.is_gp() && src.is_mem() {
            self.assembler.mov64(*dst, *src);
        } else if dst.is_mem() && src.is_gp() {
            self.assembler.mov64(*dst, *src);
        } else if dst.is_mem() && src.is_imm() {
            self.assembler.mov64(*dst, *src);
        } else if dst.is_vec() && src.is_vec() {
            if dst.id() != src.id() {
                if self.supports_avx() {
                    self.assembler.vmovaps128(*dst, *src);
                } else {
                    self.assembler.sse_movaps(*dst, *src);
                }
            }
        } else {
            unimplemented!("Unsupported mov combination");
        }
    }

    pub fn load8(
        &mut self,
        dst: impl OperandCast,
        src: impl OperandCast,
        sign_extend: Option<ExtensionSize>,
    ) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_gp());
        assert!(src.is_mem());

        match sign_extend {
            Some(ExtensionSize::I8ToI32) => self.assembler.movsxr32m8(*dst, *src),
            Some(ExtensionSize::I8ToI64) => self.assembler.movsxr64m8(*dst, *src),
            None => self.assembler.mov8(*dst, *src),
            _ => unimplemented!("Unsupported sign extend size for load8"),
        }
    }

    pub fn load16(
        &mut self,
        dst: impl OperandCast,
        src: impl OperandCast,
        sign_extend: Option<ExtensionSize>,
    ) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_gp());
        assert!(src.is_mem());

        match sign_extend {
            Some(ExtensionSize::I16ToI32) => self.assembler.movsxr32m16(*dst, *src),
            Some(ExtensionSize::I16ToI64) => self.assembler.movsxr64m16(*dst, *src),
            None => self.assembler.mov16(*dst, *src),
            _ => unimplemented!("Unsupported sign extend size for load16"),
        }
    }

    pub fn load32(
        &mut self,
        dst: impl OperandCast,
        src: impl OperandCast,
        sign_extend: Option<ExtensionSize>,
    ) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_reg());
        assert!(src.is_mem());
        let mem = src.as_::<Mem>();
        match sign_extend {
            Some(ExtensionSize::I32ToI64) => {
                assert!(!dst.is_vec(), "Cannot sign extend to a vector register");
                self.assembler.movsxr64m32(*dst, *src)
            }
            None => {
                if dst.is_vec() {
                    if mem.is_abs() {
                        let addr = mem.absolute_address();
                        self.assembler
                            .mov64(Self::scratch_register(), imm(addr as i64));
                        self.load32(*dst, ptr32(Self::scratch_register(), 0), None)
                    } else {
                        if self.supports_avx() {
                            self.assembler.vmovss(*dst, *src)
                        } else {
                            self.assembler.sse_movss(*dst, *src)
                        }
                    }
                } else {
                    self.assembler.mov32(*dst, *src)
                }
            }
            _ => unimplemented!("Unsupported sign extend size for load32"),
        }
    }

    pub fn load64(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_reg() || dst.is_vec());
        assert!(src.is_mem());

        if dst.is_vec() {
            let mem = src.as_::<Mem>();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64(Self::scratch_register(), imm(addr as i64));
                self.load64(*dst, ptr64(Self::scratch_register(), 0))
            } else {
                if self.supports_avx() {
                    self.assembler.vmovsd(*dst, *src)
                } else {
                    self.assembler.sse_movsd(*dst, *src)
                }
            }
        } else {
            self.assembler.mov64(*dst, *src)
        }
    }

    pub fn store8(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());

        self.assembler.mov8(*dst, *src)
    }

    pub fn store16(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());

        self.assembler.mov16(*dst, *src)
    }

    pub fn store32(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());

        if src.is_imm() {
            self.assembler.mov32(*dst, *src)
        } else if src.is_gp() {
            self.assembler.mov32(*dst, *src)
        } else if src.is_vec() {
            if self.supports_avx() {
                self.assembler.vmovss(*dst, *src)
            } else {
                self.assembler.sse_movss(*dst, *src)
            }
        } else {
            unimplemented!("Unsupported store32 combination");
        }
    }

    pub fn store64(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());

        if src.is_imm() {
            self.assembler.mov64(*dst, *src)
        } else if src.is_gp() {
            self.assembler.mov64(*dst, *src)
        } else if src.is_vec() {
            if self.supports_avx() {
                self.assembler.vmovsd(*dst, *src)
            } else {
                self.assembler.sse_movsd(*dst, *src)
            }
        } else {
            unimplemented!("Unsupported store64 combination");
        }
    }

    pub fn store128(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());
        assert!(src.is_vec());

        if self.supports_avx() {
            self.assembler.vmovaps128(*dst, *src)
        } else {
            self.assembler.sse_movaps(*dst, *src)
        }
    }

    pub fn store256(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());
        assert!(src.is_vec());

        if self.supports_avx2() {
            self.assembler.vmovaps256(*dst, *src)
        } else {
            unreachable!("store256 requires AVX2")
        }
    }

    pub fn store512(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());
        assert!(src.is_vec());

        if self.supports_avx512f() {
            self.assembler.vmovaps512(*dst, *src)
        } else {
            unreachable!("store512 requires AVX-512F")
        }
    }

    pub fn zero_extend(
        &mut self,
        dst: impl OperandCast,
        src: impl OperandCast,
        size: ExtensionSize,
    ) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_gp());
        assert!(src.is_gp());

        match size {
            ExtensionSize::I8ToI32 => self.assembler.movzxr32r8(*dst, *src),
            ExtensionSize::I8ToI64 => self.assembler.movzxr64r8(*dst, *src),
            ExtensionSize::I16ToI32 => self.assembler.movzxr32r16(*dst, *src),
            ExtensionSize::I16ToI64 => self.assembler.movzxr64r16(*dst, *src),
            ExtensionSize::I32ToI64 => self.assembler.mov32(*dst, *src),
        }
    }

    pub fn sign_extend(
        &mut self,
        dst: impl OperandCast,
        src: impl OperandCast,
        size: ExtensionSize,
    ) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_gp());
        assert!(src.is_gp());

        match size {
            ExtensionSize::I8ToI32 => self.assembler.movsxr32r8(*dst, *src),
            ExtensionSize::I8ToI64 => self.assembler.movsxr64r8(*dst, *src),
            ExtensionSize::I16ToI32 => self.assembler.movsxr32r16(*dst, *src),
            ExtensionSize::I16ToI64 => self.assembler.movsxr64r16(*dst, *src),
            ExtensionSize::I32ToI64 => self.assembler.movsxr64r32(*dst, *src),
        }
    }

    pub fn swap32(&mut self, a: impl OperandCast, b: impl OperandCast) {
        let a = a.as_operand();
        let b = b.as_operand();

        if a.is_gp() && b.is_gp() {
            self.assembler.xchg32(*a, *b)
        } else if a.is_mem() && b.is_gp() {
            self.assembler.xchg32(*a, *b)
        } else {
            unreachable!("Unsupported swap32 combination");
        }
    }

    pub fn swap64(&mut self, a: impl OperandCast, b: impl OperandCast) {
        let a = a.as_operand();
        let b = b.as_operand();

        if a.is_gp() && b.is_gp() {
            self.assembler.xchg64(*a, *b)
        } else if a.is_mem() && b.is_gp() {
            self.assembler.xchg64(*a, *b)
        } else {
            unreachable!("Unsupported swap64 combination");
        }
    }

    pub fn call(&mut self, target: impl OperandCast) {
        let target = target.as_operand();

        self.assembler.call(*target)
    }

    pub fn jump(&mut self, target: impl OperandCast) {
        let target = target.as_operand();

        self.assembler.jmp(*target)
    }

    pub fn branch(&mut self, cond: ResultCondition) -> Label {
        let label = self.assembler.get_label();
        let op = JCC | cond.x86_condition() as i64;

        self.assembler.emit_n(op, &[&label]);
        label
    }

    pub fn branch_test32(
        &mut self,
        value: impl OperandCast,
        mask: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let value = value.as_operand();
        let mask = mask.as_operand();

        if value.is_gp() && mask.is_gp() {
            self.assembler.test32(*value, *mask);
            self.branch(cond)
        } else if value.is_gp() && mask.is_imm() {
            self.assembler.test32(*value, *mask);
            self.branch(cond)
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.has_shift() || mem.has_index() {
                if mask == -1 {
                    self.assembler.cmp32(*value, imm(0));
                } else {
                    self.assembler.test32(*value, imm(mask));
                }
            } else if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64(Self::scratch_register(), imm(addr as i64));
                return self.branch_test32(ptr32(Self::scratch_register(), 0), imm(mask), cond);
            } else {
                self.generate_test32(mem, mask as i32);
            }
            self.branch(cond)
        } else {
            unimplemented!("Unsupported branch_test32 combination");
        }
    }

    pub fn branch_test8(
        &mut self,
        value: impl OperandCast,
        mask: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let value = value.as_operand();
        let mask = mask.as_operand();

        if value.is_gp() && mask.is_gp() {
            self.assembler.test8(*value, *mask);
            self.branch(cond)
        } else if value.is_gp() && mask.is_imm() {
            self.assembler.test8(*value, *mask);
            self.branch(cond)
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64(Self::scratch_register(), imm(addr as i64));
                return self.branch_test8(ptr8(Self::scratch_register(), 0), imm(mask), cond);
            } else {
                if mask == -1 {
                    self.assembler.cmp8(*value, imm(0));
                } else {
                    self.assembler.test8(*value, imm(mask));
                }
                self.branch(cond)
            }
        } else {
            unimplemented!("Unsupported branch_test8 combination");
        }
    }

    pub fn branch_test16(
        &mut self,
        value: impl OperandCast,
        mask: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let value = value.as_operand();
        let mask = mask.as_operand();

        if value.is_gp() && mask.is_gp() {
            self.assembler.test16(*value, *mask);
            self.branch(cond)
        } else if value.is_gp() && mask.is_imm() {
            self.assembler.test16(*value, *mask);
            self.branch(cond)
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64(Self::scratch_register(), imm(addr as i64));
                return self.branch_test16(ptr16(Self::scratch_register(), 0), imm(mask), cond);
            } else {
                if mask == -1 {
                    self.assembler.cmp16(*value, imm(0));
                } else {
                    self.assembler.test16(*value, imm(mask));
                }
                self.branch(cond)
            }
        } else {
            unimplemented!("Unsupported branch_test16 combination");
        }
    }

    fn generate_test32(&mut self, ptr: Mem, mask: i32) {
        if mask == -1 {
            self.assembler.cmp32(ptr, imm(mask));
        } else if (mask & !0xff) == 0 {
            self.assembler.test8(ptr, imm(mask));
        } else if (mask & !0xff00) == 0 {
            self.assembler.test8(ptr + 1, imm(mask >> 8));
        } else if (mask & !0xff0000) == 0 {
            self.assembler.test8(ptr + 2, imm(mask >> 16));
        } else if (mask & !(0xff000000u32 as i32)) == 0 {
            self.assembler.test8(ptr + 3, imm(mask >> 24));
        } else {
            self.assembler.test32(ptr, imm(mask));
        }
    }

    pub fn branch_test64(
        &mut self,
        value: impl OperandCast,
        mask: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let value = value.as_operand();
        let mask = mask.as_operand();

        if value.is_gp() && mask.is_gp() {
            self.assembler.test64(*value, *mask);
            self.branch(cond)
        } else if value.is_gp() && mask.is_imm() {
            let mask = mask.as_::<Imm>().value();
            if mask == -1 {
                self.assembler.cmp64(*value, imm(0));
            } else if mask & !0x7f == 0 {
                self.assembler.test8(*value, imm(mask));
            } else {
                self.assembler.test64(*value, imm(mask));
            }

            self.branch(cond)
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64(Self::scratch_register(), imm(addr as i64));
                return self.branch_test64(ptr64(Self::scratch_register(), 0), imm(mask), cond);
            } else {
                if mask == -1 {
                    self.assembler.cmp64(*value, imm(0));
                } else if mask & !0x7f == 0 {
                    self.assembler.test8(*value, imm(mask));
                } else {
                    self.assembler.test64(*value, imm(mask));
                }
                self.branch(cond)
            }
        } else {
            unimplemented!("Unsupported branch_test64 combination");
        }
    }

    pub fn branch_test_bit64(
        &mut self,
        test_value: impl OperandCast,
        bit: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let label = self.assembler.get_label();

        let test_value = test_value.as_operand();
        let bit = bit.as_operand();

        if test_value.is_gp() && bit.is_gp() {
            self.assembler.bt64(*test_value, *bit);
        } else if test_value.is_mem() && bit.is_imm() {
            self.assembler.bt64(*test_value, *bit);
        } else if test_value.is_gp() && bit.is_imm() {
            self.assembler.bt64(*test_value, *bit);
        } else {
            unimplemented!("Unsupported branch_test_bit64 combination");
        }

        if cond == ResultCondition::NonZero {
            self.assembler.jc(label);
        } else if cond == ResultCondition::Zero {
            self.assembler.jnc(label);
        } else {
            unimplemented!("Unsupported condition for branch_test_bit64");
        }

        label
    }

    pub fn branch_test_bit32(
        &mut self,
        test_value: impl OperandCast,
        bit: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let label = self.assembler.get_label();

        let test_value = test_value.as_operand();
        let bit = bit.as_operand();

        if test_value.is_gp() && bit.is_gp() {
            self.assembler.bt32(*test_value, *bit);
        } else if test_value.is_mem() && bit.is_imm() {
            self.assembler.bt32(*test_value, *bit);
        } else if test_value.is_gp() && bit.is_imm() {
            self.assembler.bt32(*test_value, *bit);
        } else {
            unimplemented!("Unsupported branch_test_bit32 combination");
        }

        if cond == ResultCondition::NonZero {
            self.assembler.jc(label);
        } else if cond == ResultCondition::Zero {
            self.assembler.jnc(label);
        } else {
            unimplemented!("Unsupported condition for branch_test_bit32");
        }

        label
    }

    pub fn test32(
        &mut self,
        dest: impl OperandCast,
        value: impl OperandCast,
        mask: impl OperandCast,
        cond: ResultCondition,
    ) {
        let value = value.as_operand();
        let mask = mask.as_operand();

        if value.is_gp() && mask.is_gp() {
            self.assembler.test32(*value, *mask);
        } else if value.is_gp() && mask.is_imm() {
            self.assembler.test32(*value, *mask);
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64(Self::scratch_register(), imm(addr as i64));
                return self.test32(dest, ptr32(Self::scratch_register(), 0), imm(mask), cond);
            } else {
                self.generate_test32(mem, mask as i32);
            }
        } else {
            unimplemented!("Unsupported test32 combination");
        }

        self.set32(dest, cond)
    }

    pub fn test64(
        &mut self,
        dest: impl OperandCast,
        value: impl OperandCast,
        mask: impl OperandCast,
        cond: ResultCondition,
    ) {
        let value = value.as_operand();
        let mask = mask.as_operand();

        if value.is_gp() && mask.is_gp() {
            self.assembler.test64(*value, *mask);
        } else if value.is_gp() && mask.is_imm() {
            let mask = mask.as_::<Imm>().value();
            if mask == -1 {
                self.assembler.cmp64(*value, imm(0));
            } else if mask & !0x7f == 0 {
                self.assembler.test8(*value, imm(mask));
            } else {
                self.assembler.test64(*value, imm(mask));
            }
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64(Self::scratch_register(), imm(addr as i64));
                return self.test64(dest, ptr64(Self::scratch_register(), 0), imm(mask), cond);
            } else {
                if mask == -1 {
                    self.assembler.cmp64(*value, imm(0));
                } else if mask & !0x7f == 0 {
                    self.assembler.test8(*value, imm(mask));
                } else {
                    self.assembler.test64(*value, imm(mask));
                }
            }
        } else {
            unimplemented!("Unsupported test64 combination");
        }

        self.set32(dest, cond)
    }

    pub fn set32(&mut self, dest: impl OperandCast, cond: ResultCondition) {
        let dest = dest.as_operand();

        assert!(dest.is_gp());

        let op = SETCC8R | cond.x86_condition() as i64;
        self.assembler.emit_n(op, &[&dest.as_operand()]);
        self.assembler.movzxr32r8(*dest, *dest)
    }

    fn floating_point_compare(
        &mut self,
        cond: DoubleCondition,
        left: Vec,
        right: Vec,
        dest: Gp,
        mut compare: impl FnMut(&mut Self, Vec, Vec),
    ) {
        let (special, invert, cc) = cond.x86_condition();
        if special {
            if cond == DoubleCondition::EqualAndOrdered {
                if left == right {
                    compare(self, left, right);
                    self.assembler
                        .emit_n(SETCC8R | CondCode::NP as i64, &[&dest.as_operand()]);
                    return;
                }

                self.mov(dest, imm(0));
                compare(self, right, left);
                let is_unordered = self.assembler.get_label();
                self.assembler.jp(is_unordered);
                self.assembler
                    .emit_n(SETCC8R | CondCode::E as i64, &[&dest.as_operand()]);
                self.assembler.bind_label(is_unordered);
                return;
            }

            if cond == DoubleCondition::NotEqualOrUnordered {
                if left == right {
                    compare(self, left, right);
                    self.assembler
                        .emit_n(SETCC8R | CondCode::P as i64, &[&dest.as_operand()]);
                    return;
                }

                self.mov(dest, imm(1));
                compare(self, right, left);
                let is_unordered = self.assembler.get_label();
                self.assembler.jp(is_unordered);
                self.assembler
                    .emit_n(SETCC8R | CondCode::NE as i64, &[&dest.as_operand()]);
                self.assembler.bind_label(is_unordered);
                return;
            }

            unreachable!();
        }

        if invert {
            compare(self, left, right);
        } else {
            compare(self, right, left);
        }

        self.assembler
            .emit_n(SETCC8R | cc as i64, &[&dest.as_operand()]);
    }

    fn jump_after_floating_point_compare(
        &mut self,
        cond: DoubleCondition,
        left: Vec,
        right: Vec,
    ) -> Label {
        if cond == DoubleCondition::EqualAndOrdered {
            if left.id() == right.id() {
                let label = self.assembler.get_label();
                self.assembler.jnp(label);
                return label;
            }
            let is_unordered = self.assembler.get_label();
            self.assembler.jp(is_unordered);
            let result = self.assembler.get_label();
            self.assembler.jz(result);
            self.assembler.bind_label(is_unordered);
            return result;
        }

        if cond == DoubleCondition::NotEqualOrUnordered {
            if left.id() == right.id() {
                let label = self.assembler.get_label();
                self.assembler.jp(label);
                return label;
            }
            let is_unordered = self.assembler.get_label();
            let is_equal = self.assembler.get_label();
            self.assembler.jp(is_unordered);
            self.assembler.jz(is_equal);
            self.assembler.bind_label(is_unordered);
            let result = self.assembler.get_label();
            self.assembler.jmp(result);
            self.assembler.bind_label(is_equal);
            return result;
        }
        let (_, _, cc) = cond.x86_condition();
        let label = self.assembler.get_label();
        self.assembler.emit_n(JCC | cc as i64, &[&label]);
        label
    }
}

pub const SCRATCH_REGISTER: Gpq = R11;
pub const FP_TEMP_REGISTER: Xmm = XMM15;
