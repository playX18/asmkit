use core::ops::Deref;

use crate::{
    core::{
        emitter::Emitter,
        operand::{Imm, Label, OperandCast},
        target::X86Feature,
    },
    masm::{DoubleCondition, ExtensionSize, JumpList, RelationalCondition, ResultCondition},
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

    pub fn add8(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.add8(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.add8(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.add8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov8(*dst, *lhs);
            self.assembler.add8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov8(*dst, *lhs);
            self.assembler.add8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            self.assembler.add8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            self.assembler.add8(*dst, *rhs);
        } else {
            unimplemented!("Unsupported add combination");
        }
    }

    pub fn add16(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.add16(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64(*dst, *lhs);
            self.assembler.add16(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.add16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov16(*dst, *lhs);
            self.assembler.add16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov16(*dst, *lhs);
            self.assembler.add16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            self.assembler.add16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            self.assembler.add16(*dst, *rhs);
        } else {
            unimplemented!("Unsupported add combination");
        }
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

    pub fn sub8(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub8(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub8(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov8(*dst, *lhs);
            self.assembler.sub8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov8(*dst, *lhs);
            self.assembler.sub8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub8(*dst, *rhs);
        } else {
            unimplemented!("Unsupported sub combination");
        }
    }

    pub fn sub16(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub16(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub16(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov16(*dst, *lhs);
            self.assembler.sub16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov16(*dst, *lhs);
            self.assembler.sub16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i8::MIN as i64 && imm <= i8::MAX as i64 {
                self.assembler.sub16(*dst, *rhs);
            } else {
                self.assembler.mov16(Self::scratch_register(), *rhs);
                self.assembler.sub16(*dst, Self::scratch_register());
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

    pub fn and8(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and8(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and8(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov8(*dst, *lhs);
            self.assembler.and8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov8(*dst, *lhs);
            self.assembler.and8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            self.assembler.and8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            self.assembler.and8(*dst, *rhs);
        } else {
            unimplemented!("Unsupported and combination");
        }
    }

    pub fn and16(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and16(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and16(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov16(*dst, *lhs);
            self.assembler.and16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov16(*dst, *lhs);
            self.assembler.and16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            self.assembler.and16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i8::MIN as i64 && imm <= i8::MAX as i64 {
                self.assembler.and16(*dst, *rhs);
            } else {
                self.assembler.mov16(Self::scratch_register(), *rhs);
                self.assembler.and16(*dst, Self::scratch_register());
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

    pub fn or8(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or8(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or8(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov8(*dst, *lhs);
            self.assembler.or8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov8(*dst, *lhs);
            self.assembler.or8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            self.assembler.or8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            self.assembler.or8(*dst, *rhs);
        } else {
            unimplemented!("Unsupported or combination");
        }
    }

    pub fn or16(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or16(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or16(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov16(*dst, *lhs);
            self.assembler.or16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov16(*dst, *lhs);
            self.assembler.or16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            self.assembler.or16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i8::MIN as i64 && imm <= i8::MAX as i64 {
                self.assembler.or16(*dst, *rhs);
            } else {
                self.assembler.mov16(Self::scratch_register(), *rhs);
                self.assembler.or16(*dst, Self::scratch_register());
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

    pub fn xor8(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor8(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor8(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov8(*dst, *lhs);
            self.assembler.xor8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov8(*dst, *lhs);
            self.assembler.xor8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            self.assembler.xor8(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            self.assembler.xor8(*dst, *rhs);
        } else {
            unimplemented!("Unsupported xor combination");
        }
    }

    pub fn xor16(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor16(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor16(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov16(*dst, *lhs);
            self.assembler.xor16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov16(*dst, *lhs);
            self.assembler.xor16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            self.assembler.xor16(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i8::MIN as i64 && imm <= i8::MAX as i64 {
                self.assembler.xor16(*dst, *rhs);
            } else {
                self.assembler.mov16(Self::scratch_register(), *rhs);
                self.assembler.xor16(*dst, Self::scratch_register());
            }
        } else {
            unimplemented!("Unsupported xor combination");
        }
    }

    pub fn lshift32(
        &mut self,
        dst: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
    ) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if rhs.is_imm() {
            if dst.is_gp() && lhs.is_gp() {
                self.mov32_if_needed(*dst, *lhs);
                self.assembler.shl32(*dst, *rhs);
            } else if dst.is_gp() && lhs.is_mem() {
                self.load32(*dst, *lhs, None);
                self.assembler.shl32(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_gp() {
                self.assembler.mov32(*dst, *lhs);
                self.assembler.shl32(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_mem() {
                assert_eq!(
                    dst.as_::<Mem>(),
                    lhs.as_::<Mem>(),
                    "Cannot lshift two memory operands"
                );
                self.assembler.shl32(*dst, *rhs);
            } else {
                unimplemented!("Unsupported lshift32 combination");
            }
            return;
        }

        if rhs.is_gp() {
            self.with_shift_count_in_rcx(*rhs, |this| {
                if dst.is_gp() {
                    if dst.id() == RCX.id() {
                        if lhs.is_gp() || lhs.is_mem() {
                            this.assembler.mov32(Self::scratch_register(), *lhs);
                        } else {
                            unimplemented!("Unsupported lshift32 combination");
                        }

                        this.assembler.shl32(Self::scratch_register(), RCX);
                        this.assembler.mov32(*dst, Self::scratch_register());
                    } else if lhs.is_gp() {
                        this.mov32_if_needed(*dst, *lhs);
                        this.assembler.shl32(*dst, RCX);
                    } else if lhs.is_mem() {
                        this.load32(*dst, *lhs, None);
                        this.assembler.shl32(*dst, RCX);
                    } else {
                        unimplemented!("Unsupported lshift32 combination");
                    }
                } else if dst.is_mem() {
                    if lhs.is_gp() {
                        this.assembler.mov32(*dst, *lhs);
                    } else if lhs.is_mem() {
                        assert_eq!(
                            dst.as_::<Mem>(),
                            lhs.as_::<Mem>(),
                            "Cannot lshift two memory operands"
                        );
                    } else {
                        unimplemented!("Unsupported lshift32 combination");
                    }

                    this.assembler.shl32(*dst, RCX);
                } else {
                    unimplemented!("Unsupported lshift32 combination");
                }
            });
            return;
        }

        unimplemented!("Unsupported lshift32 combination");
    }

    pub fn lshift64(
        &mut self,
        dst: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
    ) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if rhs.is_imm() {
            if dst.is_gp() && lhs.is_gp() {
                self.assembler.mov64(*dst, *lhs);
                self.assembler.shl64(*dst, *rhs);
            } else if dst.is_gp() && lhs.is_mem() {
                self.load64(*dst, *lhs);
                self.assembler.shl64(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_gp() {
                self.assembler.mov64(*dst, *lhs);
                self.assembler.shl64(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_mem() {
                assert_eq!(
                    dst.as_::<Mem>(),
                    lhs.as_::<Mem>(),
                    "Cannot lshift two memory operands"
                );
                self.assembler.shl64(*dst, *rhs);
            } else {
                unimplemented!("Unsupported lshift64 combination");
            }
            return;
        }

        if rhs.is_gp() {
            self.with_shift_count_in_rcx(*rhs, |this| {
                if dst.is_gp() {
                    if dst.id() == RCX.id() {
                        if lhs.is_gp() || lhs.is_mem() {
                            this.assembler.mov64(Self::scratch_register(), *lhs);
                        } else {
                            unimplemented!("Unsupported lshift64 combination");
                        }

                        this.assembler.shl64(Self::scratch_register(), RCX);
                        this.assembler.mov64(*dst, Self::scratch_register());
                    } else if lhs.is_gp() {
                        this.assembler.mov64(*dst, *lhs);
                        this.assembler.shl64(*dst, RCX);
                    } else if lhs.is_mem() {
                        this.load64(*dst, *lhs);
                        this.assembler.shl64(*dst, RCX);
                    } else {
                        unimplemented!("Unsupported lshift64 combination");
                    }
                } else if dst.is_mem() {
                    if lhs.is_gp() {
                        this.assembler.mov64(*dst, *lhs);
                    } else if lhs.is_mem() {
                        assert_eq!(
                            dst.as_::<Mem>(),
                            lhs.as_::<Mem>(),
                            "Cannot lshift two memory operands"
                        );
                    } else {
                        unimplemented!("Unsupported lshift64 combination");
                    }

                    this.assembler.shl64(*dst, RCX);
                } else {
                    unimplemented!("Unsupported lshift64 combination");
                }
            });
            return;
        }

        unimplemented!("Unsupported lshift64 combination");
    }

    pub fn rshift32(
        &mut self,
        dst: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
    ) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if rhs.is_imm() {
            if dst.is_gp() && lhs.is_gp() {
                self.mov32_if_needed(*dst, *lhs);
                self.assembler.sar32(*dst, *rhs);
            } else if dst.is_gp() && lhs.is_mem() {
                self.load32(*dst, *lhs, None);
                self.assembler.sar32(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_gp() {
                self.assembler.mov32(*dst, *lhs);
                self.assembler.sar32(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_mem() {
                assert_eq!(
                    dst.as_::<Mem>(),
                    lhs.as_::<Mem>(),
                    "Cannot rshift two memory operands"
                );
                self.assembler.sar32(*dst, *rhs);
            } else {
                unimplemented!("Unsupported rshift32 combination");
            }
            return;
        }

        if rhs.is_gp() {
            self.with_shift_count_in_rcx(*rhs, |this| {
                if dst.is_gp() {
                    if dst.id() == RCX.id() {
                        if lhs.is_gp() || lhs.is_mem() {
                            this.assembler.mov32(Self::scratch_register(), *lhs);
                        } else {
                            unimplemented!("Unsupported rshift32 combination");
                        }

                        this.assembler.sar32(Self::scratch_register(), RCX);
                        this.assembler.mov32(*dst, Self::scratch_register());
                    } else if lhs.is_gp() {
                        this.mov32_if_needed(*dst, *lhs);
                        this.assembler.sar32(*dst, RCX);
                    } else if lhs.is_mem() {
                        this.load32(*dst, *lhs, None);
                        this.assembler.sar32(*dst, RCX);
                    } else {
                        unimplemented!("Unsupported rshift32 combination");
                    }
                } else if dst.is_mem() {
                    if lhs.is_gp() {
                        this.assembler.mov32(*dst, *lhs);
                    } else if lhs.is_mem() {
                        assert_eq!(
                            dst.as_::<Mem>(),
                            lhs.as_::<Mem>(),
                            "Cannot rshift two memory operands"
                        );
                    } else {
                        unimplemented!("Unsupported rshift32 combination");
                    }

                    this.assembler.sar32(*dst, RCX);
                } else {
                    unimplemented!("Unsupported rshift32 combination");
                }
            });
            return;
        }

        unimplemented!("Unsupported rshift32 combination");
    }

    pub fn rshift64(
        &mut self,
        dst: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
    ) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if rhs.is_imm() {
            if dst.is_gp() && lhs.is_gp() {
                self.assembler.mov64(*dst, *lhs);
                self.assembler.sar64(*dst, *rhs);
            } else if dst.is_gp() && lhs.is_mem() {
                self.load64(*dst, *lhs);
                self.assembler.sar64(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_gp() {
                self.assembler.mov64(*dst, *lhs);
                self.assembler.sar64(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_mem() {
                assert_eq!(
                    dst.as_::<Mem>(),
                    lhs.as_::<Mem>(),
                    "Cannot rshift two memory operands"
                );
                self.assembler.sar64(*dst, *rhs);
            } else {
                unimplemented!("Unsupported rshift64 combination");
            }
            return;
        }

        if rhs.is_gp() {
            self.with_shift_count_in_rcx(*rhs, |this| {
                if dst.is_gp() {
                    if dst.id() == RCX.id() {
                        if lhs.is_gp() || lhs.is_mem() {
                            this.assembler.mov64(Self::scratch_register(), *lhs);
                        } else {
                            unimplemented!("Unsupported rshift64 combination");
                        }

                        this.assembler.sar64(Self::scratch_register(), RCX);
                        this.assembler.mov64(*dst, Self::scratch_register());
                    } else if lhs.is_gp() {
                        this.assembler.mov64(*dst, *lhs);
                        this.assembler.sar64(*dst, RCX);
                    } else if lhs.is_mem() {
                        this.load64(*dst, *lhs);
                        this.assembler.sar64(*dst, RCX);
                    } else {
                        unimplemented!("Unsupported rshift64 combination");
                    }
                } else if dst.is_mem() {
                    if lhs.is_gp() {
                        this.assembler.mov64(*dst, *lhs);
                    } else if lhs.is_mem() {
                        assert_eq!(
                            dst.as_::<Mem>(),
                            lhs.as_::<Mem>(),
                            "Cannot rshift two memory operands"
                        );
                    } else {
                        unimplemented!("Unsupported rshift64 combination");
                    }

                    this.assembler.sar64(*dst, RCX);
                } else {
                    unimplemented!("Unsupported rshift64 combination");
                }
            });
            return;
        }

        unimplemented!("Unsupported rshift64 combination");
    }

    pub fn urshift32(
        &mut self,
        dst: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
    ) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if rhs.is_imm() {
            if dst.is_gp() && lhs.is_gp() {
                self.mov32_if_needed(*dst, *lhs);
                self.assembler.shr32(*dst, *rhs);
            } else if dst.is_gp() && lhs.is_mem() {
                self.load32(*dst, *lhs, None);
                self.assembler.shr32(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_gp() {
                self.assembler.mov32(*dst, *lhs);
                self.assembler.shr32(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_mem() {
                assert_eq!(
                    dst.as_::<Mem>(),
                    lhs.as_::<Mem>(),
                    "Cannot urshift two memory operands"
                );
                self.assembler.shr32(*dst, *rhs);
            } else {
                unimplemented!("Unsupported urshift32 combination");
            }
            return;
        }

        if rhs.is_gp() {
            self.with_shift_count_in_rcx(*rhs, |this| {
                if dst.is_gp() {
                    if dst.id() == RCX.id() {
                        if lhs.is_gp() || lhs.is_mem() {
                            this.assembler.mov32(Self::scratch_register(), *lhs);
                        } else {
                            unimplemented!("Unsupported urshift32 combination");
                        }

                        this.assembler.shr32(Self::scratch_register(), RCX);
                        this.assembler.mov32(*dst, Self::scratch_register());
                    } else if lhs.is_gp() {
                        this.mov32_if_needed(*dst, *lhs);
                        this.assembler.shr32(*dst, RCX);
                    } else if lhs.is_mem() {
                        this.load32(*dst, *lhs, None);
                        this.assembler.shr32(*dst, RCX);
                    } else {
                        unimplemented!("Unsupported urshift32 combination");
                    }
                } else if dst.is_mem() {
                    if lhs.is_gp() {
                        this.assembler.mov32(*dst, *lhs);
                    } else if lhs.is_mem() {
                        assert_eq!(
                            dst.as_::<Mem>(),
                            lhs.as_::<Mem>(),
                            "Cannot urshift two memory operands"
                        );
                    } else {
                        unimplemented!("Unsupported urshift32 combination");
                    }

                    this.assembler.shr32(*dst, RCX);
                } else {
                    unimplemented!("Unsupported urshift32 combination");
                }
            });
            return;
        }

        unimplemented!("Unsupported urshift32 combination");
    }

    pub fn urshift64(
        &mut self,
        dst: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
    ) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if rhs.is_imm() {
            if dst.is_gp() && lhs.is_gp() {
                self.assembler.mov64(*dst, *lhs);
                self.assembler.shr64(*dst, *rhs);
            } else if dst.is_gp() && lhs.is_mem() {
                self.load64(*dst, *lhs);
                self.assembler.shr64(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_gp() {
                self.assembler.mov64(*dst, *lhs);
                self.assembler.shr64(*dst, *rhs);
            } else if dst.is_mem() && lhs.is_mem() {
                assert_eq!(
                    dst.as_::<Mem>(),
                    lhs.as_::<Mem>(),
                    "Cannot urshift two memory operands"
                );
                self.assembler.shr64(*dst, *rhs);
            } else {
                unimplemented!("Unsupported urshift64 combination");
            }
            return;
        }

        if rhs.is_gp() {
            self.with_shift_count_in_rcx(*rhs, |this| {
                if dst.is_gp() {
                    if dst.id() == RCX.id() {
                        if lhs.is_gp() || lhs.is_mem() {
                            this.assembler.mov64(Self::scratch_register(), *lhs);
                        } else {
                            unimplemented!("Unsupported urshift64 combination");
                        }

                        this.assembler.shr64(Self::scratch_register(), RCX);
                        this.assembler.mov64(*dst, Self::scratch_register());
                    } else if lhs.is_gp() {
                        this.assembler.mov64(*dst, *lhs);
                        this.assembler.shr64(*dst, RCX);
                    } else if lhs.is_mem() {
                        this.load64(*dst, *lhs);
                        this.assembler.shr64(*dst, RCX);
                    } else {
                        unimplemented!("Unsupported urshift64 combination");
                    }
                } else if dst.is_mem() {
                    if lhs.is_gp() {
                        this.assembler.mov64(*dst, *lhs);
                    } else if lhs.is_mem() {
                        assert_eq!(
                            dst.as_::<Mem>(),
                            lhs.as_::<Mem>(),
                            "Cannot urshift two memory operands"
                        );
                    } else {
                        unimplemented!("Unsupported urshift64 combination");
                    }

                    this.assembler.shr64(*dst, RCX);
                } else {
                    unimplemented!("Unsupported urshift64 combination");
                }
            });
            return;
        }

        unimplemented!("Unsupported urshift64 combination");
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

    pub fn mul16(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.assembler.mov16(*dst, *lhs);
            }

            self.assembler.imul16_2(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i16::MIN as i64 && imm <= i16::MAX as i64 {
                self.assembler.imul16_3(*dst, *lhs, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                if dst.id() != lhs.id() {
                    self.assembler.mov16(*dst, *lhs);
                }
                self.assembler.imul16_2(*dst, Self::scratch_register());
            }
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.assembler.mov16(*dst, *lhs);
            }

            self.assembler.imul16_2(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov16(Self::scratch_register(), *lhs);
            self.assembler.imul16_2(Self::scratch_register(), *rhs);
            self.assembler.mov16(*dst, Self::scratch_register());
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i16::MIN as i64 && imm <= i16::MAX as i64 {
                self.assembler
                    .imul16_3(Self::scratch_register(), *lhs, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.imul16_2(Self::scratch_register(), *lhs);
            }
            self.assembler.mov16(*dst, Self::scratch_register());
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_mem() {
            self.assembler.mov16(Self::scratch_register(), *lhs);
            self.assembler.imul16_2(Self::scratch_register(), *rhs);
            self.assembler.mov16(*dst, Self::scratch_register());
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

    pub fn compare_double(
        &mut self,
        left: impl Deref<Target = Vec>,
        right: impl Deref<Target = Vec>,
        dest: impl Deref<Target = Gp>,
        cond: DoubleCondition,
    ) {
        self.floating_point_compare(cond, *left, *right, *dest, |this, arg1, arg2| {
            if this.supports_avx() {
                this.assembler.vucomisd(arg1, arg2);
            } else {
                this.assembler.sse_ucomisd(arg1, arg2);
            }
        });
    }

    pub fn compare_float(
        &mut self,
        left: impl Deref<Target = Vec>,
        right: impl Deref<Target = Vec>,
        dest: impl Deref<Target = Gp>,
        cond: DoubleCondition,
    ) {
        self.floating_point_compare(cond, *left, *right, *dest, |this, arg1, arg2| {
            if this.supports_avx() {
                this.assembler.vucomiss(arg1, arg2);
            } else {
                this.assembler.sse_ucomiss(arg1, arg2);
            }
        });
    }

    pub fn branch_double(
        &mut self,
        left: impl Deref<Target = Vec>,
        right: impl Deref<Target = Vec>,
        cond: DoubleCondition,
    ) -> Label {
        let (_special, invert, _) = cond.x86_condition();
        if invert {
            if self.supports_avx() {
                self.assembler.vucomisd(*left, *right);
            } else {
                self.assembler.sse_ucomisd(*left, *right);
            }
        } else {
            if self.supports_avx() {
                self.assembler.vucomisd(*right, *left);
            } else {
                self.assembler.sse_ucomisd(*right, *left);
            }
        }

        self.jump_after_floating_point_compare(cond, *left, *right)
    }

    pub fn branch_float(
        &mut self,
        left: impl Deref<Target = Vec>,
        right: impl Deref<Target = Vec>,
        cond: DoubleCondition,
    ) -> Label {
        let (_special, invert, _) = cond.x86_condition();
        if invert {
            if self.supports_avx() {
                self.assembler.vucomiss(*left, *right);
            } else {
                self.assembler.sse_ucomiss(*left, *right);
            }
        } else {
            if self.supports_avx() {
                self.assembler.vucomiss(*right, *left);
            } else {
                self.assembler.sse_ucomiss(*right, *left);
            }
        }

        self.jump_after_floating_point_compare(cond, *left, *right)
    }

    /// Truncates 'src' to an integer, and places the resulting 'dest'.
    /// If the result is not representable as a 32 bit value, branch.
    /// May also branch for some values that are representable in 32 bits
    /// (specifically, in this case, [i32::MIN]).
    pub fn branch_truncate_double_to_int32(
        &mut self,
        dest: impl Deref<Target = Gp>,
        src: impl Deref<Target = Vec>,
        branch_on_fail: bool,
    ) -> Label {
        if self.supports_avx() {
            self.assembler.vcvttsd2si32(*dest, *src);
        } else {
            self.assembler.sse_cvttsd2si32(*dest, *src);
        }

        self.branch32(
            *dest,
            imm(i32::MIN as i64),
            branch_on_fail
                .then_some(RelationalCondition::NotEqual)
                .unwrap_or(RelationalCondition::Equal),
        )
    }

    pub fn truncate_double_to_int32(
        &mut self,
        dest: impl Deref<Target = Gp>,
        src: impl Deref<Target = Vec>,
    ) {
        if self.supports_avx() {
            self.assembler.vcvttsd2si32(*dest, *src);
        } else {
            self.assembler.sse_cvttsd2si32(*dest, *src);
        }
    }

    pub fn truncate_float_to_int32(
        &mut self,
        dest: impl Deref<Target = Gp>,
        src: impl Deref<Target = Vec>,
    ) {
        if self.supports_avx() {
            self.assembler.vcvttss2si32(*dest, *src);
        } else {
            self.assembler.sse_cvttss2si32(*dest, *src);
        }
    }

    /// Convert 'src' to an integer, and places the resulting 'dest'.
    /// If the result is not representable as a 32 bit value, branch.
    /// May also branch for some values that are representable in 32 bits
    /// (specifically, in this case, 0).
    pub fn branch_convert_double_to_int32(
        &mut self,
        dest: impl Deref<Target = Gp>,
        src: impl Deref<Target = Vec>,
        failure_cases: &mut JumpList,
        fp_temp: Xmm,
        neg_zero_check: bool,
    ) {
        if self.supports_avx() {
            self.assembler.vcvtsd2si32(*dest, *src);
        } else {
            self.assembler.sse_cvtsd2si32(*dest, *src);
        }

        if neg_zero_check {
            let value_is_non_zero = self.branch_test32(*dest, *dest, ResultCondition::NonZero);
            if self.supports_avx() {
                self.assembler.vmovmskpd128(Self::scratch_register(), *src);
            } else {
                self.assembler.sse_movmskpd(Self::scratch_register(), *src);
            }

            failure_cases.push(self.branch_test32(
                Self::scratch_register(),
                imm(1),
                ResultCondition::NonZero,
            ));
            self.assembler.bind_label(value_is_non_zero);
        }

        self.convert_int32_to_double(fp_temp, dest);
        if self.supports_avx() {
            self.assembler.vucomisd(fp_temp, *src);
        } else {
            self.assembler.sse_ucomisd(fp_temp, *src);
        }

        let jp = self.assembler.get_label();
        self.assembler.jp(jp);
        let jne = self.assembler.get_label();
        self.assembler.jnz(jne);
        failure_cases.push(jp);
        failure_cases.push(jne);
    }

    pub fn convert_double_to_int32(
        &mut self,
        dest: impl Deref<Target = Gp>,
        src: impl Deref<Target = Vec>,
    ) {
        if self.supports_avx() {
            self.assembler.vcvtsd2si32(*dest, *src);
        } else {
            self.assembler.sse_cvtsd2si32(*dest, *src);
        }
    }

    pub fn convert_int32_to_double(
        &mut self,
        dest: impl Deref<Target = Vec>,
        src: impl Deref<Target = Gp>,
    ) {
        if self.supports_avx() {
            self.assembler.vcvtsi2sd32(*dest, *dest, *src);
        } else {
            self.assembler.sse_cvtsi2sd32(*dest, *src);
        }
    }

    pub fn convert_int32_to_float(
        &mut self,
        dest: impl Deref<Target = Vec>,
        src: impl Deref<Target = Gp>,
    ) {
        if self.supports_avx() {
            self.assembler.vcvtsi2ss32(*dest, *dest, *src);
        } else {
            self.assembler.sse_cvtsi2ss32(*dest, *src);
        }
    }

    pub fn convert_uint32_to_double(
        &mut self,
        dest: impl Deref<Target = Vec>,
        src: impl Deref<Target = Gp>,
    ) {
        self.zero_extend(Self::scratch_register(), *src, ExtensionSize::I32ToI64);
        self.convert_int64_to_double(dest, Self::scratch_register());
    }

    pub fn convert_int64_to_double(
        &mut self,
        dest: impl Deref<Target = Vec>,
        src: impl Deref<Target = Gp>,
    ) {
        if self.supports_avx() {
            self.assembler.vcvtsi2sd64(*dest, *dest, *src);
        } else {
            self.assembler.sse_cvtsi2sd64(*dest, *src);
        }
    }

    pub fn convert_int64_to_float(
        &mut self,
        dest: impl Deref<Target = Vec>,
        src: impl Deref<Target = Gp>,
    ) {
        if self.supports_avx() {
            self.assembler.vcvtsi2ss64(*dest, *dest, *src);
        } else {
            self.assembler.sse_cvtsi2ss64(*dest, *src);
        }
    }

    pub fn convert_uint64_to_double(
        &mut self,
        dest: impl Deref<Target = Vec>,
        src: impl Deref<Target = Gp>,
        scratch: impl Deref<Target = Gp>,
    ) {
        let scratch2 = Self::scratch_register();
        let scratch = *scratch;

        self.assembler.test64(*src, *src);
        let sign_bit_set = self.assembler.get_label();
        self.assembler
            .emit_n(JCC | CondCode::S as i64, &[&sign_bit_set]);
        if self.supports_avx() {
            self.assembler.vcvtsi2sd64(*dest, *dest, *src);
        } else {
            self.assembler.sse_cvtsi2sd64(*dest, *src);
        }

        let done = self.assembler.get_label();
        self.assembler.jmp(done);
        self.assembler.bind_label(sign_bit_set);
        if scratch.id() != src.id() {
            self.assembler.mov64(*scratch, *src);
        }

        self.assembler.mov64(scratch2, *src);
        self.assembler.shr64(scratch, imm(1));
        self.assembler.and64(scratch2, imm(1));
        self.assembler.or64(scratch2, scratch);
        if self.supports_avx() {
            self.assembler.vcvtsi2sd64(*dest, *dest, scratch2);
            self.assembler.vaddsd(*dest, *dest, *dest);
        } else {
            self.assembler.sse_cvtsi2sd64(*dest, scratch2);
            self.assembler.sse_addsd(*dest, *dest);
        }

        self.assembler.bind_label(done);
    }

    pub fn convert_uint64_to_float(
        &mut self,
        dest: impl Deref<Target = Vec>,
        src: impl Deref<Target = Gp>,
        scratch: impl Deref<Target = Gp>,
    ) {
        let scratch2 = Self::scratch_register();
        let scratch = *scratch;

        self.assembler.test64(*src, *src);
        let sign_bit_set = self.assembler.get_label();
        self.assembler
            .emit_n(JCC | CondCode::S as i64, &[&sign_bit_set]);
        if self.supports_avx() {
            self.assembler.vcvtsi2ss64(*dest, *dest, *src);
        } else {
            self.assembler.sse_cvtsi2ss64(*dest, *src);
        }

        let done = self.assembler.get_label();
        self.assembler.jmp(done);
        self.assembler.bind_label(sign_bit_set);
        if scratch.id() != src.id() {
            self.assembler.mov64(*scratch, *src);
        }

        self.assembler.mov64(scratch2, *src);
        self.assembler.shr64(scratch, imm(1));
        self.assembler.and64(scratch2, imm(1));
        self.assembler.or64(scratch2, scratch);
        if self.supports_avx() {
            self.assembler.vcvtsi2ss64(*dest, *dest, scratch2);
            self.assembler.vaddss(*dest, *dest, *dest);
        } else {
            self.assembler.sse_cvtsi2ss64(*dest, scratch2);
            self.assembler.sse_addss(*dest, *dest);
        }

        self.assembler.bind_label(done);
    }

    pub fn branch8(
        &mut self,
        left: impl OperandCast,
        right: impl OperandCast,
        cond: RelationalCondition,
    ) -> Label {
        let left = left.as_operand();
        let right = right.as_operand();

        if !left.is_mem() {
            unreachable!("Left operand of branch8 must be memory");
        }

        let mem = left.as_::<Mem>();
        if mem.is_abs() {
            let abs = mem.absolute_address();
            self.assembler
                .mov64(Self::scratch_register(), imm(abs as i64));
            self.branch8(ptr8(Self::scratch_register(), 0), *right, cond)
        } else {
            self.assembler.cmp8(*left, *right);
            let op = JCC | cond.x86_condition() as i64;
            let label = self.assembler.get_label();
            self.assembler.emit_n(op, &[&label]);
            label
        }
    }

    pub fn branch16(
        &mut self,
        left: impl OperandCast,
        right: impl OperandCast,
        cond: RelationalCondition,
    ) -> Label {
        let left = left.as_operand();
        let right = right.as_operand();

        if !left.is_mem() {
            unreachable!("Left operand of branch16 must be memory");
        }

        let mem = left.as_::<Mem>();
        if mem.is_abs() {
            let abs = mem.absolute_address();
            self.assembler
                .mov64(Self::scratch_register(), imm(abs as i64));
            self.branch16(ptr16(Self::scratch_register(), 0), *right, cond)
        } else {
            self.assembler.cmp16(*left, *right);
            let op = JCC | cond.x86_condition() as i64;
            let label = self.assembler.get_label();
            self.assembler.emit_n(op, &[&label]);
            label
        }
    }

    pub fn branch32(
        &mut self,
        left: impl OperandCast,
        right: impl OperandCast,
        cond: RelationalCondition,
    ) -> Label {
        let left = left.as_operand();
        let right = right.as_operand();

        if left.is_gp() && right.is_gp() {
            self.assembler.cmp32(*left, *right);
        } else if left.is_gp() && right.is_imm() {
            let val = right.as_::<Imm>().value();
            if val == 0 {
                if let Some(cond) = compute_compare_to_zero_test(cond) {
                    return self.branch_test32(*left, *left, cond);
                }
            }
            self.assembler.cmp32(*left, *right);
        } else if left.is_mem() && right.is_gp() {
            let mem = left.as_::<Mem>();
            if mem.is_abs() {
                let abs = mem.absolute_address();
                self.assembler
                    .mov64(Self::scratch_register(), imm(abs as i64));
                return self.branch32(ptr32(Self::scratch_register(), 0), *right, cond);
            }
            self.assembler.cmp32(*left, *right);
        } else if left.is_mem() && right.is_imm() {
            let mem = left.as_::<Mem>();

            if mem.is_abs() {
                let abs = mem.absolute_address();
                self.assembler
                    .mov64(Self::scratch_register(), imm(abs as i64));
                return self.branch32(ptr32(Self::scratch_register(), 0), *right, cond);
            }

            self.assembler.cmp32(*left, *right);
        } else {
            unimplemented!("Unsupported branch32 combination");
        }

        let op = JCC | cond.x86_condition() as i64;
        let label = self.assembler.get_label();
        self.assembler.emit_n(op, &[&label]);
        label
    }

    pub fn branch64(
        &mut self,
        left: impl OperandCast,
        right: impl OperandCast,
        cond: RelationalCondition,
    ) -> Label {
        let left = left.as_operand();
        let right = right.as_operand();

        if left.is_gp() && right.is_gp() {
            self.assembler.cmp64(*left, *right);
        } else if left.is_gp() && right.is_imm() {
            let val = right.as_::<Imm>().value();
            if val == 0 {
                if let Some(cond) = compute_compare_to_zero_test(cond) {
                    return self.branch_test64(*left, *left, cond);
                }
            }
            self.assembler.cmp64(*left, *right);
        } else if left.is_mem() && right.is_gp() {
            let mem = left.as_::<Mem>();
            if mem.is_abs() {
                let abs = mem.absolute_address();
                self.assembler
                    .mov64(Self::scratch_register(), imm(abs as i64));
                return self.branch64(ptr64(Self::scratch_register(), 0), *right, cond);
            }
            self.assembler.cmp64(*left, *right);
        } else if left.is_mem() && right.is_imm() {
            let mem = left.as_::<Mem>();

            if mem.is_abs() {
                let abs = mem.absolute_address();
                self.assembler
                    .mov64(Self::scratch_register(), imm(abs as i64));
                return self.branch64(ptr64(Self::scratch_register(), 0), *right, cond);
            }

            let val = right.as_::<Imm>().value();
            if val < i32::MIN as i64 || val > i32::MAX as i64 {
                self.mov64(Self::scratch_register(), *right);
                self.assembler.cmp64(*left, Self::scratch_register());
            } else {
                self.assembler.cmp64(*left, *right);
            }
        } else {
            unimplemented!("Unsupported branch64 combination");
        }

        let op = JCC | cond.x86_condition() as i64;
        let label = self.assembler.get_label();
        self.assembler.emit_n(op, &[&label]);
        label
    }

    /// Execute `dest = lhs + rhs` and branch if the result satisfies `cond`.
    ///
    /// This is useful for checking overflows, counting loops, and similar patterns where you want to update a value and branch based on the new value.
    pub fn branch_add32(
        &mut self,
        dest: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let dest = dest.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dest.is_gp() && lhs.is_gp() && lhs.id() == dest.id() {
            self.assembler.add32(*dest, *rhs);
        } else if lhs.is_mem() && rhs.is_gp() && dest.is_gp() && rhs.id() == dest.id() {
            self.assembler.add32(*dest, *lhs);
        } else if lhs.is_mem()
            && lhs.as_::<Mem>().has_base()
            && rhs.is_gp()
            && dest.is_gp()
            && lhs.as_::<Mem>().base_id() == dest.id()
        {
            // dest = *dest;
            // dest += rhs;
            self.load32(*dest, *lhs, None);
            self.assembler.add32(*dest, *rhs);
        } else if lhs.is_mem() && rhs.is_gp() && dest.is_gp() {
            self.zero_extend(*dest, *rhs, ExtensionSize::I32ToI64);
            self.assembler.add32(*dest, *lhs);
        } else if lhs.is_gp() && rhs.is_mem() && dest.is_gp() {
            return self.branch_add32(*dest, *lhs, *rhs, cond);
        } else if lhs.is_gp() && rhs.is_imm() && dest.is_gp() {
            self.mov32_if_needed(*dest, *lhs);
            self.assembler.add32(*dest, *rhs);
        } else if lhs.is_mem() && rhs.is_imm() && dest.is_gp() {
            self.load32(*dest, *lhs, None);
            self.assembler.add32(*dest, *rhs);
        } else if lhs.is_gp() && rhs.is_imm() && dest.is_mem() {
            self.store32(*dest, *lhs);
            self.assembler.add32(*dest, *rhs);
        } else {
            unimplemented!("Unsupported branch_add32 combination");
        }

        let op = JCC | cond.x86_condition() as i64;
        let label = self.assembler.get_label();
        self.assembler.emit_n(op, &[&label]);
        label
    }

    /// Execute `dest = lhs + rhs` (64-bit) and branch if the result satisfies `cond`.
    pub fn branch_add64(
        &mut self,
        dest: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let dest = dest.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dest.is_gp() && lhs.is_gp() && lhs.id() == dest.id() {
            if rhs.is_imm() {
                let val = rhs.as_::<Imm>().value();
                if val < i32::MIN as i64 || val > i32::MAX as i64 {
                    self.mov64(Self::scratch_register(), *rhs);
                    self.assembler.add64(*dest, Self::scratch_register());
                } else {
                    self.assembler.add64(*dest, *rhs);
                }
            } else {
                self.assembler.add64(*dest, *rhs);
            }
        } else if lhs.is_mem() && rhs.is_gp() && dest.is_gp() && rhs.id() == dest.id() {
            self.assembler.add64(*dest, *lhs);
        } else if lhs.is_mem()
            && lhs.as_::<Mem>().has_base()
            && rhs.is_gp()
            && dest.is_gp()
            && lhs.as_::<Mem>().base_id() == dest.id()
        {
            self.load64(*dest, *lhs);
            self.assembler.add64(*dest, *rhs);
        } else if lhs.is_mem() && rhs.is_gp() && dest.is_gp() {
            self.assembler.mov64(*dest, *rhs);
            self.assembler.add64(*dest, *lhs);
        } else if lhs.is_gp() && rhs.is_mem() && dest.is_gp() {
            return self.branch_add64(*dest, *lhs, *rhs, cond);
        } else if lhs.is_gp() && rhs.is_imm() && dest.is_gp() {
            self.assembler.mov64(*dest, *lhs);
            let val = rhs.as_::<Imm>().value();
            if val < i32::MIN as i64 || val > i32::MAX as i64 {
                self.mov64(Self::scratch_register(), *rhs);
                self.assembler.add64(*dest, Self::scratch_register());
            } else {
                self.assembler.add64(*dest, *rhs);
            }
        } else if lhs.is_mem() && rhs.is_imm() && dest.is_gp() {
            self.load64(*dest, *lhs);
            let val = rhs.as_::<Imm>().value();
            if val < i32::MIN as i64 || val > i32::MAX as i64 {
                self.mov64(Self::scratch_register(), *rhs);
                self.assembler.add64(*dest, Self::scratch_register());
            } else {
                self.assembler.add64(*dest, *rhs);
            }
        } else {
            unimplemented!("Unsupported branch_add64 combination");
        }

        let op = JCC | cond.x86_condition() as i64;
        let label = self.assembler.get_label();
        self.assembler.emit_n(op, &[&label]);
        label
    }

    /// Execute `dest = lhs - rhs` and branch if the result satisfies `cond`.
    pub fn branch_sub32(
        &mut self,
        dest: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let dest = dest.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dest.is_gp() && lhs.is_gp() && rhs.is_gp() {
            self.mov32_if_needed(*dest, *lhs);
            self.assembler.sub32(*dest, *rhs);
        } else if dest.is_gp() && lhs.is_gp() && rhs.is_imm() {
            self.mov32_if_needed(*dest, *lhs);
            self.assembler.sub32(*dest, *rhs);
        } else if dest.is_gp() && lhs.is_gp() && rhs.is_mem() {
            self.mov32_if_needed(*dest, *lhs);
            self.assembler.sub32(*dest, *rhs);
        } else if dest.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32(*dest, *lhs);
            self.assembler.sub32(*dest, *rhs);
        } else if dest.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov32(*dest, *lhs);
            self.assembler.sub32(*dest, *rhs);
        } else if dest.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dest.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub32(*dest, *rhs);
        } else if dest.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dest.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub32(*dest, *rhs);
        } else {
            unimplemented!("Unsupported branch_sub32 combination");
        }

        let op = JCC | cond.x86_condition() as i64;
        let label = self.assembler.get_label();
        self.assembler.emit_n(op, &[&label]);
        label
    }

    /// Execute `dest = lhs - rhs` (64-bit) and branch if the result satisfies `cond`.
    pub fn branch_sub64(
        &mut self,
        dest: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let dest = dest.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dest.is_gp() && lhs.is_gp() && rhs.is_gp() {
            self.mov64(*dest, *lhs);
            self.assembler.sub64(*dest, *rhs);
        } else if dest.is_gp() && lhs.is_gp() && rhs.is_imm() {
            self.mov64(*dest, *lhs);
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.sub64(*dest, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.sub64(*dest, Self::scratch_register());
            }
        } else if dest.is_gp() && lhs.is_gp() && rhs.is_mem() {
            self.mov64(*dest, *lhs);
            self.assembler.sub64(*dest, *rhs);
        } else if dest.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64(*dest, *lhs);
            self.assembler.sub64(*dest, *rhs);
        } else if dest.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64(*dest, *lhs);
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.sub64(*dest, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.sub64(*dest, Self::scratch_register());
            }
        } else if dest.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dest.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub64(*dest, *rhs);
        } else if dest.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dest.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.sub64(*dest, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.sub64(*dest, Self::scratch_register());
            }
        } else {
            unimplemented!("Unsupported branch_sub64 combination");
        }

        let op = JCC | cond.x86_condition() as i64;
        let label = self.assembler.get_label();
        self.assembler.emit_n(op, &[&label]);
        label
    }

    /// Execute `dest = lhs * rhs` and branch if the result satisfies `cond`.
    pub fn branch_mul32(
        &mut self,
        dest: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let dest = dest.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dest.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dest.id() != lhs.id() {
                self.assembler.mov32(*dest, *lhs);
            }

            self.assembler.imul32_2(*dest, *rhs);
        } else if dest.is_gp() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.imul32_3(*dest, *lhs, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                if dest.id() != lhs.id() {
                    self.assembler.mov32(*dest, *lhs);
                }
                self.assembler.imul32_2(*dest, Self::scratch_register());
            }
        } else if dest.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dest.id() != lhs.id() {
                self.assembler.mov32(*dest, *lhs);
            }

            self.assembler.imul32_2(*dest, *rhs);
        } else if dest.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32(Self::scratch_register(), *lhs);
            self.assembler.imul32_2(Self::scratch_register(), *rhs);
            self.assembler.mov32(*dest, Self::scratch_register());
        } else if dest.is_mem() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler
                    .imul32_3(Self::scratch_register(), *lhs, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.imul32_2(Self::scratch_register(), *lhs);
            }
            self.assembler.mov32(*dest, Self::scratch_register());
        } else if dest.is_mem() && lhs.is_gp() && rhs.is_mem() {
            self.assembler.mov32(Self::scratch_register(), *lhs);
            self.assembler.imul32_2(Self::scratch_register(), *rhs);
            self.assembler.mov32(*dest, Self::scratch_register());
        } else {
            unimplemented!("Unsupported branch_mul32 combination");
        }

        let op = JCC | cond.x86_condition() as i64;
        let label = self.assembler.get_label();
        self.assembler.emit_n(op, &[&label]);
        label
    }

    /// Execute `dest = lhs * rhs` (64-bit) and branch if the result satisfies `cond`.
    pub fn branch_mul64(
        &mut self,
        dest: impl OperandCast,
        lhs: impl OperandCast,
        rhs: impl OperandCast,
        cond: ResultCondition,
    ) -> Label {
        let dest = dest.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dest.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dest.id() != lhs.id() {
                self.assembler.mov64(*dest, *lhs);
            }

            self.assembler.imul64_2(*dest, *rhs);
        } else if dest.is_gp() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.imul64_3(*dest, *lhs, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                if dest.id() != lhs.id() {
                    self.assembler.mov64(*dest, *lhs);
                }
                self.assembler.imul64_2(*dest, Self::scratch_register());
            }
        } else if dest.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dest.id() != lhs.id() {
                self.assembler.mov64(*dest, *lhs);
            }

            self.assembler.imul64_2(*dest, *rhs);
        } else if dest.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64(Self::scratch_register(), *lhs);
            self.assembler.imul64_2(Self::scratch_register(), *rhs);
            self.assembler.mov64(*dest, Self::scratch_register());
        } else if dest.is_mem() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler
                    .imul64_3(Self::scratch_register(), *lhs, *rhs);
            } else {
                self.assembler.mov64(Self::scratch_register(), *rhs);
                self.assembler.imul64_2(Self::scratch_register(), *lhs);
            }
            self.assembler.mov64(*dest, Self::scratch_register());
        } else if dest.is_mem() && lhs.is_gp() && rhs.is_mem() {
            self.assembler.mov64(Self::scratch_register(), *lhs);
            self.assembler.imul64_2(Self::scratch_register(), *rhs);
            self.assembler.mov64(*dest, Self::scratch_register());
        } else {
            unimplemented!("Unsupported branch_mul64 combination");
        }

        let op = JCC | cond.x86_condition() as i64;
        let label = self.assembler.get_label();
        self.assembler.emit_n(op, &[&label]);
        label
    }

    /// Force 64-bit move, even if the source is an immediate that could fit in 32 bits. This is useful for ensuring that the upper 32 bits of the destination register are zeroed out, which can be important for certain operations and calling conventions.
    pub fn mov64(&mut self, dest: impl OperandCast, src: impl OperandCast) {
        let dest = dest.as_operand();
        let src = src.as_operand();
        assert!(
            dest.is_gp(),
            "dest of mov64 must be a general-purpose register"
        );
        assert!(
            src.is_gp() || src.is_imm(),
            "src of mov64 must be a general-purpose register or immediate"
        );

        self.assembler.mov64(*dest, *src)
    }

    /// Move `src` to `dest`, but only if they are different registers.
    pub fn mov32_if_needed(&mut self, dest: impl OperandCast, src: impl OperandCast) {
        let dest = dest.as_operand();
        let src = src.as_operand();
        assert!(
            dest.is_gp(),
            "dest of mov32_if_needed must be a general-purpose register"
        );
        assert!(
            src.is_gp(),
            "src of mov32_if_needed must be a general-purpose register"
        );
        if dest.id() != src.id() {
            self.assembler.mov32(*dest, *src)
        }
    }

    fn with_shift_count_in_rcx(
        &mut self,
        rhs: impl OperandCast,
        mut emit_shift: impl FnMut(&mut Self),
    ) {
        let rhs = rhs.as_operand();

        assert!(
            rhs.is_gp(),
            "shift count must be a general-purpose register"
        );

        let needs_restore = rhs.id() != RCX.id();
        if needs_restore {
            self.assembler.push(RCX);
            self.assembler.mov64(RCX, *rhs);
        }

        emit_shift(self);

        if needs_restore {
            self.assembler.pop(RCX);
        }
    }
}

pub const SCRATCH_REGISTER: Gpq = R11;
pub const FP_TEMP_REGISTER: Xmm = XMM15;

fn compute_compare_to_zero_test(cond: RelationalCondition) -> Option<ResultCondition> {
    match cond {
        RelationalCondition::Equal => Some(ResultCondition::Zero),
        RelationalCondition::NotEqual => Some(ResultCondition::NonZero),
        RelationalCondition::LessThan => Some(ResultCondition::Signed),
        RelationalCondition::GreaterThanOrEqual => Some(ResultCondition::PositiveOrZero),
        _ => None,
    }
}

#[cfg(all(test, unix, target_arch = "x86_64", feature = "jit"))]
mod tests {
    use alloc::boxed::Box;

    use super::*;
    use crate::core::{
        buffer::CodeBuffer,
        jit_allocator::{JitAllocator, Span},
    };

    struct CompiledCode {
        jit: Box<JitAllocator>,
        span: Span,
    }

    impl CompiledCode {
        fn new(emit: impl FnOnce(&mut MacroAssemblerX86<'_>)) -> Self {
            let mut buffer = CodeBuffer::new();
            let assembler = Assembler::new(&mut buffer);
            let mut masm = MacroAssemblerX86::new(assembler);
            emit(&mut masm);

            let code = buffer.finish();
            let mut jit = JitAllocator::new(Default::default());
            let span = code
                .allocate(&mut jit)
                .expect("failed to allocate executable code");

            Self { jit, span }
        }

        fn entry(&self) -> *const u8 {
            self.span.rx()
        }
    }

    impl Drop for CompiledCode {
        fn drop(&mut self) {
            unsafe {
                self.jit
                    .release(self.span.rx())
                    .expect("failed to release executable code");
            }
        }
    }

    #[test]
    fn executes_recursive_factorial() {
        let code = CompiledCode::new(|asm| {
            let factorial = asm.get_label();

            asm.bind_label(factorial);
            asm.mov(RAX, imm(1));

            let recurse = asm.branch_test64(RDI, RDI, ResultCondition::NonZero);
            asm.ret();

            asm.bind_label(recurse);
            asm.push(RBX);
            asm.mov(RBX, RDI);
            asm.sub64(RDI, RDI, imm(1));
            asm.call(factorial);
            asm.mov(RDX, RAX);
            asm.mov(RAX, RBX);
            asm.mul64(RAX, RAX, RDX);
            asm.pop(RBX);
            asm.ret();
        });

        let factorial: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };

        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3_628_800);
    }

    #[test]
    fn branch_add64_detects_signed_overflow() {
        let code = CompiledCode::new(|asm| {
            let overflow = asm.branch_add64(RAX, RDI, imm(i64::MAX), ResultCondition::Overflow);

            asm.mov(RAX, imm(0));
            asm.ret();

            asm.bind_label(overflow);
            asm.mov(RAX, imm(1));
            asm.ret();
        });

        let did_overflow: extern "C" fn(i64) -> u64 = unsafe { std::mem::transmute(code.entry()) };

        assert_eq!(did_overflow(0), 0);
        assert_eq!(did_overflow(-1), 0);
        assert_eq!(did_overflow(1), 1);
    }

    #[test]
    fn converts_uint64_to_double_across_sign_bit() {
        let code = CompiledCode::new(|asm| {
            asm.convert_uint64_to_double(XMM0, RDI, RDI);
            asm.ret();
        });

        let to_double: extern "C" fn(u64) -> f64 = unsafe { std::mem::transmute(code.entry()) };

        for input in [0, 1, 42, 1_u64 << 63, u64::MAX] {
            assert_eq!(to_double(input).to_bits(), (input as f64).to_bits());
        }
    }

    #[test]
    fn converts_uint64_to_float_across_sign_bit() {
        let code = CompiledCode::new(|asm| {
            asm.convert_uint64_to_float(XMM0, RDI, RDI);
            asm.ret();
        });

        let to_float: extern "C" fn(u64) -> f32 = unsafe { std::mem::transmute(code.entry()) };

        for input in [0, 1, 42, 1_u64 << 63, u64::MAX] {
            assert_eq!(to_float(input).to_bits(), (input as f32).to_bits());
        }
    }

    // ==================== Arithmetic Opcodes ====================

    #[test]
    fn add8_gp_reg_immediate() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.add8(RAX, RAX, imm(10));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(5), 15);
        assert_eq!(func(250), 4);
    }

    #[test]
    fn add16_gp_reg_immediate() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.add16(RAX, RAX, imm(1000));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(500), 1500);
        assert_eq!(func(65000), 65000 + 1000 - 65536);
    }

    #[test]
    fn add32_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.add32(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u32, u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(100, 200), 300);
    }

    #[test]
    fn add64_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.add64(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(
            func(1_000_000_000_000, 2_000_000_000_000),
            3_000_000_000_000
        );
    }

    #[test]
    fn sub8_gp_reg_immediate() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.sub8(RAX, RAX, imm(5));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(10), 5);
        assert_eq!(func(3), 254);
    }

    #[test]
    fn sub16_gp_reg_immediate() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.sub16(RAX, RAX, imm(100));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(200), 100);
        assert_eq!(func(50), 65486);
    }

    #[test]
    fn sub32_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.sub32(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u32, u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(500, 200), 300);
    }

    #[test]
    fn sub64_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.sub64(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(1_000_000, 400_000), 600_000);
    }

    #[test]
    fn mul16_gp_reg_immediate() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.mul16(RAX, RAX, imm(3));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(100), 300);
        assert_eq!(func(1000), 3000);
    }

    #[test]
    fn mul32_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.mul32(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u32, u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(1000, 500), 500_000);
    }

    #[test]
    fn mul64_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.mul64(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(1_000_000, 1_000_000), 1_000_000_000_000);
    }

    #[test]
    fn div32_basic() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.div32(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u32, u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(100, 5), 20);
        assert_eq!(func(1000, 3), 333);
    }

    #[test]
    fn div64_basic() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.div64(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(1_000_000, 1000), 1000);
        assert_eq!(func(100, 3), 33);
    }

    // ==================== Logic Opcodes ====================

    #[test]
    fn and8_gp_reg_immediate() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.and8(RAX, RAX, imm(0x0F));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0xFF), 0x0F);
        assert_eq!(func(0xAB), 0x0B);
    }

    #[test]
    fn and16_gp_reg_immediate() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.and16(RAX, RAX, imm(0x00FF));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0xFFFF), 0x00FF);
        assert_eq!(func(0x1234), 0x0034);
    }

    #[test]
    fn and32_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.and32(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u32, u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0xFFFF0000, 0x0000FFFF), 0);
        assert_eq!(func(0xFFFFFFFF, 0x12345678), 0x12345678);
    }

    #[test]
    fn and64_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.and64(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(
            func(0xFFFFFFFFFFFFFFFF, 0x123456789ABCDEF0),
            0x123456789ABCDEF0
        );
    }

    #[test]
    #[ignore]
    fn or8_gp_reg_immediate() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.or8(RAX, RAX, imm(0xF0));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0x0F), 0xFF);
        assert_eq!(func(0x00), 0xF0);
    }

    #[test]
    #[ignore]
    fn or16_gp_reg_immediate() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.or16(RAX, RAX, imm(0xF000));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0x0FFF), 0xFFFF);
    }

    #[test]
    fn or32_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.or32(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u32, u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0xF0F0F0F0, 0x0F0F0F0F), 0xFFFFFFFF);
    }

    #[test]
    fn or64_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.or64(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(
            func(0x00000000FFFFFFFF, 0xFFFFFFFF00000000),
            0xFFFFFFFFFFFFFFFF
        );
    }

    #[test]
    #[ignore]
    fn xor8_gp_reg_immediate() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.xor8(RAX, RAX, imm(0xFF));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0x00), 0xFF);
        assert_eq!(func(0xFF), 0x00);
        assert_eq!(func(0xAA), 0x55);
    }

    #[test]
    #[ignore]
    fn xor16_gp_reg_immediate() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.xor16(RAX, RAX, imm(0xFFFF));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0x0000), 0xFFFF);
        assert_eq!(func(0xFFFF), 0x0000);
    }

    #[test]
    fn xor32_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.xor32(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u32, u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0xFFFFFFFF, 0xFFFFFFFF), 0);
        assert_eq!(func(0x12345678, 0x87654321), 0x95511559);
    }

    #[test]
    fn xor64_gp_reg_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.xor64(RAX, RAX, RSI);
            asm.ret();
        });

        let func: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0x0, 0xFFFFFFFFFFFFFFFF), 0xFFFFFFFFFFFFFFFF);
    }

    // ==================== Shift Opcodes ====================

    #[test]
    fn lshift32_imm() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.lshift32(RAX, RAX, imm(4));
            asm.ret();
        });

        let func: extern "C" fn(u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(1), 16);
        assert_eq!(func(0x10000000), 0);
    }

    #[test]
    fn lshift64_imm() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.lshift64(RAX, RAX, imm(8));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(1), 256);
        assert_eq!(func(0x00FF00FF00FF00FF), 0xFF00FF00FF00FF00);
    }

    #[test]
    fn rshift32_imm() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.rshift32(RAX, RAX, imm(4));
            asm.ret();
        });

        let func: extern "C" fn(u32) -> u32 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(16), 1);
        assert_eq!(
            func(0x80000000u32),
            (0x80000000u32 as i32).wrapping_shr(4) as u32
        );
    }

    #[test]
    fn rshift64_imm() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.rshift64(RAX, RAX, imm(8));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(256), 1);
        assert_eq!(
            func(0xFF00FF00FF00FF00),
            (0xFF00FF00FF00FF00u64 as i64).wrapping_shr(8) as u64
        );
    }

    #[test]
    fn urshift32_imm() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.urshift32(RAX, RAX, imm(4));
            asm.ret();
        });

        let func: extern "C" fn(u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(16), 1);
        assert_eq!(func(0x80000000u32), 0x08000000);
    }

    #[test]
    fn urshift64_imm() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.urshift64(RAX, RAX, imm(8));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(256), 1);
        assert_eq!(func(0x8000000000000000), 0x0080000000000000);
    }

    // ==================== Data Movement Opcodes ====================

    #[test]
    fn mov_gp_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0x123456789ABCDEF0), 0x123456789ABCDEF0);
    }

    #[test]
    fn mov_gp_imm() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, imm(0xDEADBEEFu64 as i64));
            asm.ret();
        });

        let func: extern "C" fn() -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(), 0xDEADBEEF);
    }

    #[test]
    fn load8_no_sign_extend() {
        let code = CompiledCode::new(|asm| {
            asm.load8(RAX, ptr8(RDI, 0), None);
            asm.ret();
        });

        let func: extern "C" fn(&u8) -> u8 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(&0x7F), 0x7F);
        assert_eq!(func(&0xFF), 0xFF);
    }

    #[test]
    fn load16_no_sign_extend() {
        let code = CompiledCode::new(|asm| {
            asm.load16(RAX, ptr16(RDI, 0), None);
            asm.zero_extend(RAX, RAX, ExtensionSize::I16ToI64);
            asm.ret();
        });

        let func: extern "C" fn(&u16) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(&0x7FFF), 0x7FFF);
        assert_eq!(func(&0xFFFF), 0xFFFF);
    }

    #[test]
    fn load32_no_sign_extend() {
        let code = CompiledCode::new(|asm| {
            asm.load32(RAX, ptr32(RDI, 0), None);
            asm.ret();
        });

        let func: extern "C" fn(*const u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        let val: u32 = 0x7FFFFFFF;
        assert_eq!(func(&val), 0x7FFFFFFF);
    }

    #[test]
    fn load64_basic() {
        let code = CompiledCode::new(|asm| {
            asm.load64(RAX, ptr64(RDI, 0));
            asm.ret();
        });

        let func: extern "C" fn(*const u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        let val: u64 = 0x123456789ABCDEF0;
        assert_eq!(func(&val), 0x123456789ABCDEF0);
    }

    #[test]
    fn store8_basic() {
        let code = CompiledCode::new(|asm| {
            asm.store8(ptr8(RDI, 0), RSI);
            asm.mov(RAX, RDI);
            asm.ret();
        });

        let func: extern "C" fn(*mut u8, u64) -> *mut u8 =
            unsafe { std::mem::transmute(code.entry()) };
        let mut val: u8 = 0;
        let ptr = func(&mut val, 0xAB);
        unsafe { assert_eq!(*ptr, 0xAB) };
    }

    #[test]
    fn store16_basic() {
        let code = CompiledCode::new(|asm| {
            asm.store16(ptr16(RDI, 0), RSI);
            asm.mov(RAX, RDI);
            asm.ret();
        });

        let func: extern "C" fn(*mut u16, u64) -> *mut u16 =
            unsafe { std::mem::transmute(code.entry()) };
        let mut val: u16 = 0;
        let ptr = func(&mut val, 0xABCD);
        unsafe { assert_eq!(*ptr, 0xABCD) };
    }

    #[test]
    fn store32_basic() {
        let code = CompiledCode::new(|asm| {
            asm.store32(ptr32(RDI, 0), RSI);
            asm.mov(RAX, RDI);
            asm.ret();
        });

        let func: extern "C" fn(*mut u32, u64) -> *mut u32 =
            unsafe { std::mem::transmute(code.entry()) };
        let mut val: u32 = 0;
        let ptr = func(&mut val, 0x12345678);
        unsafe { assert_eq!(*ptr, 0x12345678) };
    }

    #[test]
    fn store64_basic() {
        let code = CompiledCode::new(|asm| {
            asm.store64(ptr64(RDI, 0), RSI);
            asm.mov(RAX, RDI);
            asm.ret();
        });

        let func: extern "C" fn(*mut u64, u64) -> *mut u64 =
            unsafe { std::mem::transmute(code.entry()) };
        let mut val: u64 = 0;
        let ptr = func(&mut val, 0x123456789ABCDEF0);
        unsafe { assert_eq!(*ptr, 0x123456789ABCDEF0) };
    }

    #[test]
    fn zero_extend_i8_to_i32() {
        let code = CompiledCode::new(|asm| {
            asm.zero_extend(RAX, RDI, ExtensionSize::I8ToI32);
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0xFF), 0xFF);
        assert_eq!(func(0x01), 0x01);
    }

    #[test]
    fn zero_extend_i16_to_i32() {
        let code = CompiledCode::new(|asm| {
            asm.zero_extend(RAX, RDI, ExtensionSize::I16ToI32);
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0xFFFF), 0xFFFF);
    }

    #[test]
    fn sign_extend_i8_to_i64() {
        let code = CompiledCode::new(|asm| {
            asm.sign_extend(RAX, RDI, ExtensionSize::I8ToI64);
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0x7F), 0x7F);
        assert_eq!(func(0x80), 0xFFFFFFFFFFFFFF80);
    }

    #[test]
    fn sign_extend_i32_to_i64() {
        let code = CompiledCode::new(|asm| {
            asm.sign_extend(RAX, RDI, ExtensionSize::I32ToI64);
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0x7FFFFFFF), 0x000000007FFFFFFF);
        assert_eq!(func(0x80000000u32 as u64), 0xFFFFFFFF80000000);
    }

    #[test]
    fn swap32_gp_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.mov(RDX, RSI);
            asm.swap32(RAX, RDX);
            asm.ret();
        });

        let func: extern "C" fn(u64, u64) -> (u64, u64) =
            unsafe { std::mem::transmute(code.entry()) };
        let (a, b) = func(0x12345678, 0x9ABCDEF0);
        assert_eq!(a, 0x9ABCDEF0);
        assert_eq!(b, 0x12345678);
    }

    #[test]
    fn swap64_gp_reg() {
        let code = CompiledCode::new(|asm| {
            asm.mov(RAX, RDI);
            asm.mov(RDX, RSI);
            asm.swap64(RAX, RDX);
            asm.ret();
        });

        let func: extern "C" fn(u64, u64) -> (u64, u64) =
            unsafe { std::mem::transmute(code.entry()) };
        let (a, b) = func(0x123456789ABCDEF0, 0xFEDCBA9876543210);
        assert_eq!(a, 0xFEDCBA9876543210);
        assert_eq!(b, 0x123456789ABCDEF0);
    }

    // ==================== Control Flow Opcodes ====================

    #[test]
    fn push_pop_roundtrip() {
        let code = CompiledCode::new(|asm| {
            asm.push(RDI);
            asm.pop(RAX);
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0xDEADBEEF), 0xDEADBEEF);
    }

    #[test]
    fn jump_basic() {
        let code = CompiledCode::new(|asm| {
            let target = asm.get_label();
            asm.jump(target);
            asm.mov(RAX, imm(0));
            asm.bind_label(target);
            asm.mov(RAX, imm(1));
            asm.ret();
        });

        let func: extern "C" fn() -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(), 1);
    }

    #[test]
    fn call_basic() {
        let code = CompiledCode::new(|asm| {
            let func_label = asm.get_label();
            asm.call(func_label);
            asm.ret();

            asm.bind_label(func_label);
            asm.mov(RAX, imm(42));
            asm.ret();
        });

        let func: extern "C" fn() -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(), 42);
    }

    // ==================== Compare and Test Opcodes ====================

    #[test]
    fn branch32_equal() {
        let code = CompiledCode::new(|asm| {
            let equal = asm.branch32(RDI, imm(42), RelationalCondition::Equal);
            asm.mov(RAX, imm(0));
            asm.ret();
            asm.bind_label(equal);
            asm.mov(RAX, imm(1));
            asm.ret();
        });

        let func: extern "C" fn(u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(42), 1);
        assert_eq!(func(0), 0);
    }

    #[test]
    fn branch64_less_than() {
        let code = CompiledCode::new(|asm| {
            let less = asm.branch64(RDI, imm(100), RelationalCondition::LessThan);
            asm.mov(RAX, imm(0));
            asm.ret();
            asm.bind_label(less);
            asm.mov(RAX, imm(1));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(50), 1);
        assert_eq!(func(100), 0);
        assert_eq!(func(150), 0);
    }

    #[test]
    fn test32_nonzero() {
        let code = CompiledCode::new(|asm| {
            asm.test32(RAX, RDI, imm(0xFF), ResultCondition::NonZero);
            asm.ret();
        });

        let func: extern "C" fn(u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0x000000FF), 1);
        assert_eq!(func(0x00000000), 0);
    }

    #[test]
    fn test64_zero() {
        let code = CompiledCode::new(|asm| {
            asm.test64(RAX, RDI, imm(0xFF), ResultCondition::Zero);
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0x00000000), 1);
        assert_eq!(func(0x00000001), 0);
    }

    #[test]
    fn branch_test32_nonzero() {
        let code = CompiledCode::new(|asm| {
            let nonzero = asm.branch_test32(RDI, RDI, ResultCondition::NonZero);
            asm.mov(RAX, imm(0));
            asm.ret();
            asm.bind_label(nonzero);
            asm.mov(RAX, imm(1));
            asm.ret();
        });

        let func: extern "C" fn(u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(1), 1);
        assert_eq!(func(0), 0);
    }

    #[test]
    fn branch_test64_nonzero() {
        let code = CompiledCode::new(|asm| {
            let nonzero = asm.branch_test64(RDI, RDI, ResultCondition::NonZero);
            asm.mov(RAX, imm(0));
            asm.ret();
            asm.bind_label(nonzero);
            asm.mov(RAX, imm(1));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(1), 1);
        assert_eq!(func(0), 0);
    }

    #[test]
    fn branch_test_bit64() {
        let code = CompiledCode::new(|asm| {
            let bit_set = asm.branch_test_bit64(RDI, imm(3), ResultCondition::NonZero);
            asm.mov(RAX, imm(0));
            asm.ret();
            asm.bind_label(bit_set);
            asm.mov(RAX, imm(1));
            asm.ret();
        });

        let func: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(0b1000), 1);
        assert_eq!(func(0b0111), 0);
    }

    // ==================== Floating Point Conversion Opcodes ====================

    #[test]
    fn convert_int32_to_double() {
        let code = CompiledCode::new(|asm| {
            asm.convert_int32_to_double(XMM0, RDI);
            asm.ret();
        });

        let func: extern "C" fn(i32) -> f64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(42).to_bits(), 42.0f64.to_bits());
        assert_eq!(func(-100).to_bits(), (-100.0f64).to_bits());
    }

    #[test]
    fn convert_int32_to_float() {
        let code = CompiledCode::new(|asm| {
            asm.convert_int32_to_float(XMM0, RDI);
            asm.ret();
        });

        let func: extern "C" fn(i32) -> f32 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(42).to_bits(), 42.0f32.to_bits());
    }

    #[test]
    fn convert_double_to_int32() {
        let code = CompiledCode::new(|asm| {
            asm.convert_double_to_int32(RAX, XMM0);
            asm.ret();
        });

        let func: extern "C" fn(f64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        let val1: u64 = 42.0f64.to_bits();
        let val2: u64 = 3.9f64.to_bits();
        assert_eq!(func(f64::from_bits(val1)), 42);
        assert_eq!(func(f64::from_bits(val2)), 4);
    }

    #[test]
    fn truncate_double_to_int32() {
        let code = CompiledCode::new(|asm| {
            asm.truncate_double_to_int32(RAX, XMM0);
            asm.ret();
        });

        let func: extern "C" fn(f64) -> i32 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(42.7), 42);
        assert_eq!(func(-10.9), -10);
    }

    #[test]
    fn convert_int64_to_double() {
        let code = CompiledCode::new(|asm| {
            asm.convert_int64_to_double(XMM0, RDI);
            asm.ret();
        });

        let func: extern "C" fn(i64) -> f64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(
            func(1_000_000_000_000).to_bits(),
            1_000_000_000_000.0f64.to_bits()
        );
    }

    #[test]
    fn convert_uint32_to_double() {
        let code = CompiledCode::new(|asm| {
            asm.convert_uint32_to_double(XMM0, RDI);
            asm.ret();
        });

        let func: extern "C" fn(u32) -> f64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(42).to_bits(), 42.0f64.to_bits());
        assert_eq!(func(u32::MAX).to_bits(), (u32::MAX as f64).to_bits());
    }

    // ==================== Additional Branch Opcodes ====================

    #[test]
    fn branch_sub32_zero() {
        let code = CompiledCode::new(|asm| {
            let zero = asm.branch_sub32(RAX, RDI, RSI, ResultCondition::Zero);
            asm.mov(RAX, imm(0));
            asm.ret();
            asm.bind_label(zero);
            asm.mov(RAX, imm(1));
            asm.ret();
        });

        let func: extern "C" fn(u32, u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(10, 10), 1);
        assert_eq!(func(10, 5), 0);
    }

    #[test]
    fn branch_mul32_overflow() {
        let code = CompiledCode::new(|asm| {
            let overflow = asm.branch_mul32(RAX, RDI, RSI, ResultCondition::Overflow);
            asm.mov(RAX, imm(0));
            asm.ret();
            asm.bind_label(overflow);
            asm.mov(RAX, imm(1));
            asm.ret();
        });

        let func: extern "C" fn(u32, u32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(2, u32::MAX / 2 + 1), 1);
    }

    // ==================== Compare Float/Double Opcodes ====================

    #[test]
    fn compare_float_equal() {
        let code = CompiledCode::new(|asm| {
            asm.compare_float(XMM0, XMM1, RAX, DoubleCondition::EqualAndOrdered);
            asm.ret();
        });

        let func: extern "C" fn(f32, f32) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(42.0, 42.0), 1);
        assert_eq!(func(42.0, 43.0), 0);
    }

    #[test]
    fn compare_double_equal() {
        let code = CompiledCode::new(|asm| {
            asm.compare_double(XMM0, XMM1, RAX, DoubleCondition::EqualAndOrdered);
            asm.ret();
        });

        let func: extern "C" fn(f64, f64) -> u64 = unsafe { std::mem::transmute(code.entry()) };
        assert_eq!(func(42.0, 42.0), 1);
        assert_eq!(func(42.0, 43.0), 0);
    }
}
