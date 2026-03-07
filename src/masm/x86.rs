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
                .lea32rm(*dst, ptr64_index(&lhs.as_::<Gp>(), &rhs.as_::<Gp>(), 1, 0));
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.lea32rm(
                *dst,
                ptr64(&lhs.as_::<Gp>(), rhs.as_::<Imm>().value() as i32),
            );
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.add32rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32mr(*dst, *lhs);
            self.assembler.add32mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov32mr(*dst, *lhs);
            self.assembler.add32mi(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                rhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            self.assembler.add32mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                rhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            self.assembler.add32mi(*dst, *rhs);
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vaddssrrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_addssrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vaddssrrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_addssrm(*dst, *rhs);
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
                .lea64rm(*dst, ptr64_index(&lhs.as_::<Gp>(), &rhs.as_::<Gp>(), 1, 0));
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.lea64rm(
                    *dst,
                    ptr64(&lhs.as_::<Gp>(), rhs.as_::<Imm>().value() as i32),
                );
            } else {
                if dst.id() != lhs.id() {
                    self.mov(*dst, *lhs);
                }

                self.assembler.add64ri(*dst, *rhs);
            }
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.add64rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64mr(*dst, *lhs);
            self.assembler.add64mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64mr(*dst, *lhs);
            self.assembler.add64mi(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                rhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            self.assembler.add64mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                rhs.as_::<Mem>(),
                "Cannot add two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.add64mi(*dst, *rhs);
            } else {
                self.assembler.mov64ri(Self::scratch_register(), *rhs);
                self.assembler.add64mr(*dst, Self::scratch_register());
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vaddsdrrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_addsdrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vaddsdrrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_addsdrm(*dst, *rhs);
            }
        } else {
            unimplemented!("Unsupported add combination");
        }
    }

    pub fn sub32(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_gp() && lhs.is_gp() && rhs.is_gp() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub32rr(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub32ri(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub32rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32mr(*dst, *lhs);
            self.assembler.sub32mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov32mr(*dst, *lhs);
            self.assembler.sub32mi(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub32mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub32mi(*dst, *rhs);
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vsubssrrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_subssrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vsubssrrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_subssrm(*dst, *rhs);
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

            self.assembler.sub64rr(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub64ri(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.sub64rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64mr(*dst, *lhs);
            self.assembler.sub64mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64mr(*dst, *lhs);
            self.assembler.sub64mi(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            self.assembler.sub64mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot sub two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.sub64mi(*dst, *rhs);
            } else {
                self.assembler.mov64ri(Self::scratch_register(), *rhs);
                self.assembler.sub64mr(*dst, Self::scratch_register());
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vsubsdrrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_subsdrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vsubsdrrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_subsdrm(*dst, *rhs);
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

            self.assembler.and32rr(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and32ri(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and32rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32mr(*dst, *lhs);
            self.assembler.and32mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov32mr(*dst, *lhs);
            self.assembler.and32mi(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            self.assembler.and32mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            self.assembler.and32mi(*dst, *rhs);
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vandps128rrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_andpsrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vandps128rrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_andpsrm(*dst, *rhs);
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

            self.assembler.and64rr(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and64ri(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.and64rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64mr(*dst, *lhs);
            self.assembler.and64mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64mr(*dst, *lhs);
            self.assembler.and64mi(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            self.assembler.and64mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot and two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.and64mi(*dst, *rhs);
            } else {
                self.assembler.mov64ri(Self::scratch_register(), *rhs);
                self.assembler.and64mr(*dst, Self::scratch_register());
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vandpd128rrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_andpdrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vandpd128rrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_andpdrm(*dst, *rhs);
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

            self.assembler.or32rr(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or32ri(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or32rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32mr(*dst, *lhs);
            self.assembler.or32mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov32mr(*dst, *lhs);
            self.assembler.or32mi(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            self.assembler.or32mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            self.assembler.or32mi(*dst, *rhs);
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vorps128rrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_orpsrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vorps128rrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_orpsrm(*dst, *rhs);
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

            self.assembler.or64rr(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or64ri(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.or64rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64mr(*dst, *lhs);
            self.assembler.or64mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64mr(*dst, *lhs);
            self.assembler.or64mi(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            self.assembler.or64mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot or two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.or64mi(*dst, *rhs);
            } else {
                self.assembler.mov64ri(Self::scratch_register(), *rhs);
                self.assembler.or64mr(*dst, Self::scratch_register());
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vorpd128rrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_orpdrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vorpd128rrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_orpdrm(*dst, *rhs);
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

            self.assembler.xor32rr(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor32ri(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor32rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32mr(*dst, *lhs);
            self.assembler.xor32mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov32mr(*dst, *lhs);
            self.assembler.xor32mi(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            self.assembler.xor32mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            self.assembler.xor32mi(*dst, *rhs);
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vxorps128rrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_xorpsrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vxorps128rrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_xorpsrm(*dst, *rhs);
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

            self.assembler.xor64rr(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor64ri(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.mov(*dst, *lhs);
            }

            self.assembler.xor64rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64mr(*dst, *lhs);
            self.assembler.xor64mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            self.assembler.mov64mr(*dst, *lhs);
            self.assembler.xor64mi(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_gp() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            self.assembler.xor64mr(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_mem() && rhs.is_imm() {
            assert_eq!(
                dst.as_::<Mem>(),
                lhs.as_::<Mem>(),
                "Cannot xor two memory operands"
            );
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.xor64mi(*dst, *rhs);
            } else {
                self.assembler.mov64ri(Self::scratch_register(), *rhs);
                self.assembler.xor64mr(*dst, Self::scratch_register());
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vxorpd128rrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_xorpdrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vxorpd128rrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_xorpdrm(*dst, *rhs);
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
                self.assembler.mov32rr(*dst, *lhs);
            }

            self.assembler.imul32rr(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.imul32rri(*dst, *lhs, *rhs);
            } else {
                self.assembler.mov64ri(Self::scratch_register(), *rhs);
                if dst.id() != lhs.id() {
                    self.assembler.mov32rr(*dst, *lhs);
                }
                self.assembler.imul32rr(*dst, Self::scratch_register());
            }
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.assembler.mov32rr(*dst, *lhs);
            }

            self.assembler.imul32rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov32rr(Self::scratch_register(), *lhs);
            self.assembler.imul32rr(Self::scratch_register(), *rhs);
            self.assembler.mov32mr(*dst, Self::scratch_register());
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler
                    .imul32rri(Self::scratch_register(), *lhs, *rhs);
            } else {
                self.assembler.mov64ri(Self::scratch_register(), *rhs);
                self.assembler.imul32rr(Self::scratch_register(), *lhs);
            }
            self.assembler.mov32mr(*dst, Self::scratch_register());
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_mem() {
            self.assembler.mov32rr(Self::scratch_register(), *lhs);
            self.assembler.imul32rm(Self::scratch_register(), *rhs);
            self.assembler.mov32mr(*dst, Self::scratch_register());
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vmulssrrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_mulssrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vmulssrrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_mulssrm(*dst, *rhs);
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
                self.assembler.mov64rr(*dst, *lhs);
            }

            self.assembler.imul64rr(*dst, *rhs);
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler.imul64rri(*dst, *lhs, *rhs);
            } else {
                self.assembler.mov64ri(Self::scratch_register(), *rhs);
                if dst.id() != lhs.id() {
                    self.assembler.mov64rr(*dst, *lhs);
                }
                self.assembler.imul64rr(*dst, Self::scratch_register());
            }
        } else if dst.is_gp() && lhs.is_gp() && rhs.is_mem() {
            if dst.id() != lhs.id() {
                self.assembler.mov64rr(*dst, *lhs);
            }

            self.assembler.imul64rm(*dst, *rhs);
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_gp() {
            self.assembler.mov64rr(Self::scratch_register(), *lhs);
            self.assembler.imul64rr(Self::scratch_register(), *rhs);
            self.assembler.mov64mr(*dst, Self::scratch_register());
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_imm() {
            let imm = rhs.as_::<Imm>().value();
            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                self.assembler
                    .imul64rri(Self::scratch_register(), *lhs, *rhs);
            } else {
                self.assembler.mov64ri(Self::scratch_register(), *rhs);
                self.assembler.imul64rr(Self::scratch_register(), *lhs);
            }
            self.assembler.mov64mr(*dst, Self::scratch_register());
        } else if dst.is_mem() && lhs.is_gp() && rhs.is_mem() {
            self.assembler.mov64rr(Self::scratch_register(), *lhs);
            self.assembler.imul64rm(Self::scratch_register(), *rhs);
            self.assembler.mov64mr(*dst, Self::scratch_register());
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vmulsdrrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_mulsdrr(*dst, *rhs);
            }
        } else if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vmulsdrrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_mulsdrm(*dst, *rhs);
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
                self.assembler.vdivssrrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_divssrr(*dst, *rhs);
            }
            return;
        }

        if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vdivssrrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_divssrm(*dst, *rhs);
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
            self.assembler.mov64ri(Self::scratch_register(), *rhs);
            *Self::scratch_register().as_operand()
        } else if rhs.is_gp() {
            let rhs_gp = rhs.as_::<Gp>();
            if rhs_gp.id() == RAX.id() || rhs_gp.id() == RDX.id() {
                self.assembler.mov64rr(Self::scratch_register(), *rhs);
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
            self.assembler.xchg32rr(EAX, *dst);
        }

        if lhs.is_gp() {
            if !(preserve_rax_via_dst && lhs.id() == dst.id()) {
                self.assembler.mov32rr(RAX, *lhs);
            }
        } else {
            self.assembler.mov32rm(RAX, *lhs);
        }

        self.assembler.cdq();

        if divisor.is_gp() {
            self.assembler.idiv32r(divisor);
        } else {
            self.assembler.idiv32m(divisor);
        }

        if dst.is_gp() {
            if preserve_rax_via_dst {
                self.assembler.xchg32rr(EAX, *dst);
            } else {
                self.assembler.mov32rr(*dst, RAX);
            }
        } else {
            self.assembler.mov32mr(*dst, RAX);
        }
    }

    pub fn div64(&mut self, dst: impl OperandCast, lhs: impl OperandCast, rhs: impl OperandCast) {
        let dst = dst.as_operand();
        let lhs = lhs.as_operand();
        let rhs = rhs.as_operand();

        if dst.is_vec() && lhs.is_vec() && rhs.is_vec() {
            if self.supports_avx() {
                self.assembler.vdivsdrrr(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_divsdrr(*dst, *rhs);
            }
            return;
        }

        if dst.is_vec() && lhs.is_vec() && rhs.is_mem() {
            if self.supports_avx() {
                self.assembler.vdivsdrrm(*dst, *lhs, *rhs);
            } else {
                self.mov(*dst, *lhs);
                self.assembler.sse_divsdrm(*dst, *rhs);
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
            self.assembler.mov64ri(Self::scratch_register(), *rhs);
            *Self::scratch_register().as_operand()
        } else if rhs.is_gp() {
            let rhs_gp = rhs.as_::<Gp>();
            if rhs_gp.id() == RAX.id() || rhs_gp.id() == RDX.id() {
                self.assembler.mov64rr(Self::scratch_register(), *rhs);
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
            self.assembler.xchg64rr(RAX, *dst);
        }

        if lhs.is_gp() {
            if !(preserve_rax_via_dst && lhs.id() == dst.id()) {
                self.assembler.mov64rr(RAX, *lhs);
            }
        } else {
            self.assembler.mov64rm(RAX, *lhs);
        }

        self.assembler.cqo();

        if divisor.is_gp() {
            self.assembler.idiv64r(divisor);
        } else {
            self.assembler.idiv64m(divisor);
        }

        if dst.is_gp() {
            if preserve_rax_via_dst {
                self.assembler.xchg64rr(RAX, *dst);
            } else {
                self.assembler.mov64rr(*dst, RAX);
            }
        } else {
            self.assembler.mov64mr(*dst, RAX);
        }
    }

    pub fn mov(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        if dst.is_gp() && src.is_gp() {
            if dst.id() != src.id() {
                self.assembler.mov64rr(*dst, *src);
            }
        } else if dst.is_gp() && src.is_imm() {
            self.assembler.mov64ri(*dst, *src);
        } else if dst.is_gp() && src.is_mem() {
            self.assembler.mov64rm(*dst, *src);
        } else if dst.is_mem() && src.is_gp() {
            self.assembler.mov64mr(*dst, *src);
        } else if dst.is_mem() && src.is_imm() {
            self.assembler.mov64mi(*dst, *src);
        } else if dst.is_vec() && src.is_vec() {
            if dst.id() != src.id() {
                if self.supports_avx() {
                    self.assembler.vmovaps128rr(*dst, *src);
                } else {
                    self.assembler.sse_movapsrr(*dst, *src);
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
            None => self.assembler.mov8rm(*dst, *src),
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
            None => self.assembler.mov16rm(*dst, *src),
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
                            .mov64ri(Self::scratch_register(), imm(addr as i64));
                        self.load32(*dst, ptr32(Self::scratch_register(), 0), None);
                    } else {
                        if self.supports_avx() {
                            self.assembler.vmovssrm(*dst, *src);
                        } else {
                            self.assembler.sse_movssrm(*dst, *src);
                        }
                    }
                } else {
                    self.assembler.mov32rm(*dst, *src);
                }
            }
            _ => unimplemented!("Unsupported sign extend size for load32"),
        }
    }

    pub fn load64(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_reg());
        assert!(src.is_mem());

        if dst.is_vec() {
            let mem = src.as_::<Mem>();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64ri(Self::scratch_register(), imm(addr as i64));
                self.load64(*dst, ptr64(Self::scratch_register(), 0));
            } else {
                if self.supports_avx() {
                    self.assembler.vmovsdrm(*dst, *src);
                } else {
                    self.assembler.sse_movsdrm(*dst, *src);
                }
            }
        } else {
            self.assembler.mov64rm(*dst, *src);
        }
    }

    pub fn store8(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());

        if src.is_imm() {
            self.assembler.mov8mi(*dst, *src);
        } else if src.is_gp() {
            self.assembler.mov8mr(*dst, *src);
        } else {
            unimplemented!("Unsupported store8 combination");
        }
    }

    pub fn store16(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());

        if src.is_imm() {
            self.assembler.mov16mi(*dst, *src);
        } else if src.is_gp() {
            self.assembler.mov16mr(*dst, *src);
        } else {
            unimplemented!("Unsupported store16 combination");
        }
    }

    pub fn store32(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());

        if src.is_imm() {
            self.assembler.mov32mi(*dst, *src);
        } else if src.is_gp() {
            self.assembler.mov32mr(*dst, *src);
        } else if src.is_vec() {
            if self.supports_avx() {
                self.assembler.vmovssmr(*dst, *src);
            } else {
                self.assembler.sse_movssmr(*dst, *src);
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
            self.assembler.mov64mi(*dst, *src);
        } else if src.is_gp() {
            self.assembler.mov64mr(*dst, *src);
        } else if src.is_vec() {
            if self.supports_avx() {
                self.assembler.vmovsdrm(*dst, *src);
            } else {
                self.assembler.sse_movsdrm(*dst, *src);
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
            self.assembler.vmovaps128mr(*dst, *src);
        } else {
            self.assembler.sse_movapsmr(*dst, *src);
        }
    }

    pub fn store256(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());
        assert!(src.is_vec());

        if self.supports_avx2() {
            self.assembler.vmovaps256mr(*dst, *src);
        } else {
            unimplemented!("store256 requires AVX2");
        }
    }

    pub fn store512(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = dst.as_operand();
        let src = src.as_operand();

        assert!(dst.is_mem());
        assert!(src.is_vec());

        if self.supports_avx512f() {
            self.assembler.vmovaps512mr(*dst, *src);
        } else {
            unimplemented!("store512 requires AVX-512F");
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
            ExtensionSize::I32ToI64 => self.assembler.mov32rr(*dst, *src),
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
            self.assembler.xchg32rr(*a, *b);
        } else if a.is_mem() && b.is_gp() {
            self.assembler.xchg32mr(*a, *b);
        } else {
            unimplemented!("Unsupported swap combination");
        }
    }

    pub fn swap64(&mut self, a: impl OperandCast, b: impl OperandCast) {
        let a = a.as_operand();
        let b = b.as_operand();

        if a.is_gp() && b.is_gp() {
            self.assembler.xchg64rr(*a, *b);
        } else if a.is_mem() && b.is_gp() {
            self.assembler.xchg64mr(*a, *b);
        } else {
            unimplemented!("Unsupported swap combination");
        }
    }

    pub fn call(&mut self, target: impl OperandCast) {
        let target = target.as_operand();

        if target.is_mem() {
            self.assembler.callm(*target);
        } else if target.is_gp() {
            self.assembler.callr(*target);
        } else if target.is_label() || target.is_sym() {
            self.assembler.call(*target);
        } else {
            unimplemented!("Unsupported call target");
        }
    }

    pub fn jump(&mut self, target: impl OperandCast) {
        let target = target.as_operand();

        if target.is_mem() {
            self.assembler.jmpm(*target);
        } else if target.is_gp() {
            self.assembler.jmpr(*target);
        } else if target.is_label() || target.is_sym() {
            self.assembler.jmp(*target);
        } else {
            unimplemented!("Unsupported jump target");
        }
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
            self.assembler.test32rr(*value, *mask);
            self.branch(cond)
        } else if value.is_gp() && mask.is_imm() {
            self.assembler.test32ri(*value, *mask);
            self.branch(cond)
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.has_shift() || mem.has_index() {
                if mask == -1 {
                    self.assembler.cmp32mi(*value, imm(0));
                } else {
                    self.assembler.test32mi(*value, imm(mask));
                }
            } else if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64ri(Self::scratch_register(), imm(addr as i64));
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
            self.assembler.test8rr(*value, *mask);
            self.branch(cond)
        } else if value.is_gp() && mask.is_imm() {
            self.assembler.test8ri(*value, *mask);
            self.branch(cond)
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64ri(Self::scratch_register(), imm(addr as i64));
                return self.branch_test8(ptr8(Self::scratch_register(), 0), imm(mask), cond);
            } else {
                if mask == -1 {
                    self.assembler.cmp8mi(*value, imm(0));
                } else {
                    self.assembler.test8mi(*value, imm(mask));
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
            self.assembler.test16rr(*value, *mask);
            self.branch(cond)
        } else if value.is_gp() && mask.is_imm() {
            self.assembler.test16ri(*value, *mask);
            self.branch(cond)
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64ri(Self::scratch_register(), imm(addr as i64));
                return self.branch_test16(ptr16(Self::scratch_register(), 0), imm(mask), cond);
            } else {
                if mask == -1 {
                    self.assembler.cmp16mi(*value, imm(0));
                } else {
                    self.assembler.test16mi(*value, imm(mask));
                }
                self.branch(cond)
            }
        } else {
            unimplemented!("Unsupported branch_test16 combination");
        }
    }

    fn generate_test32(&mut self, ptr: Mem, mask: i32) {
        if mask == -1 {
            self.assembler.cmp32mi(ptr, imm(mask));
        } else if (mask & !0xff) == 0 {
            self.assembler.test8mi(ptr, imm(mask));
        } else if (mask & !0xff00) == 0 {
            self.assembler.test8mi(ptr + 1, imm(mask >> 8));
        } else if (mask & !0xff0000) == 0 {
            self.assembler.test8mi(ptr + 2, imm(mask >> 16));
        } else if (mask & !(0xff000000u32 as i32)) == 0 {
            self.assembler.test8mi(ptr + 3, imm(mask >> 24));
        } else {
            self.assembler.test32mi(ptr, imm(mask));
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
            self.assembler.test64rr(*value, *mask);
            self.branch(cond)
        } else if value.is_gp() && mask.is_imm() {
            let mask = mask.as_::<Imm>().value();
            if mask == -1 {
                self.assembler.cmp64ri(*value, imm(0));
            } else if mask & !0x7f == 0 {
                self.assembler.test8ri(*value, imm(mask));
            } else {
                self.assembler.test64ri(*value, imm(mask));
            }

            self.branch(cond)
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64ri(Self::scratch_register(), imm(addr as i64));
                return self.branch_test64(ptr64(Self::scratch_register(), 0), imm(mask), cond);
            } else {
                if mask == -1 {
                    self.assembler.cmp64mi(*value, imm(0));
                } else if mask & !0x7f == 0 {
                    self.assembler.test8mi(*value, imm(mask));
                } else {
                    self.assembler.test64mi(*value, imm(mask));
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
            self.assembler.bt64rr(*test_value, *bit);
        } else if test_value.is_mem() && bit.is_imm() {
            self.assembler.bt64mi(*test_value, *bit);
        } else if test_value.is_gp() && bit.is_imm() {
            self.assembler.bt64ri(*test_value, *bit);
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
            self.assembler.bt32rr(*test_value, *bit);
        } else if test_value.is_mem() && bit.is_imm() {
            self.assembler.bt32mi(*test_value, *bit);
        } else if test_value.is_gp() && bit.is_imm() {
            self.assembler.bt32ri(*test_value, *bit);
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
            self.assembler.test32rr(*value, *mask);
        } else if value.is_gp() && mask.is_imm() {
            self.assembler.test32ri(*value, *mask);
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64ri(Self::scratch_register(), imm(addr as i64));
                return self.test32(dest, ptr32(Self::scratch_register(), 0), imm(mask), cond);
            } else {
                self.generate_test32(mem, mask as i32);
            }
        } else {
            unimplemented!("Unsupported test32 combination");
        }

        self.set32(dest, cond);
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
            self.assembler.test64rr(*value, *mask);
        } else if value.is_gp() && mask.is_imm() {
            let mask = mask.as_::<Imm>().value();
            if mask == -1 {
                self.assembler.cmp64ri(*value, imm(0));
            } else if mask & !0x7f == 0 {
                self.assembler.test8ri(*value, imm(mask));
            } else {
                self.assembler.test64ri(*value, imm(mask));
            }
        } else if value.is_mem() && mask.is_imm() {
            let mem = value.as_::<Mem>();
            let mask = mask.as_::<Imm>().value();
            if mem.is_abs() {
                let addr = mem.absolute_address();
                self.assembler
                    .mov64ri(Self::scratch_register(), imm(addr as i64));
                return self.test64(dest, ptr64(Self::scratch_register(), 0), imm(mask), cond);
            } else {
                if mask == -1 {
                    self.assembler.cmp64mi(*value, imm(0));
                } else if mask & !0x7f == 0 {
                    self.assembler.test8mi(*value, imm(mask));
                } else {
                    self.assembler.test64mi(*value, imm(mask));
                }
            }
        } else {
            unimplemented!("Unsupported test64 combination");
        }

        self.set32(dest, cond);
    }

    pub fn set32(&mut self, dest: impl OperandCast, cond: ResultCondition) {
        let dest = dest.as_operand();

        assert!(dest.is_gp());

        let op = SETCC8R | cond.x86_condition() as i64;
        self.assembler.emit_n(op, &[&dest.as_operand()]);
        self.assembler.movzxr32r8(*dest, *dest);
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
