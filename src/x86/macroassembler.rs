//! MacroAssembler for X86/64

use crate::core::{
    buffer::CodeBuffer,
    operand::{Imm, OperandCast},
};

use super::*;

pub struct MacroAssembler<'a> {
    pub asm: Assembler<'a>,
}

impl<'a> MacroAssembler<'a> {
    pub const SCRATCH_REGISTER: Gpq = R11;
    pub const FP_TEMP_REGISTER: Xmm = XMM15;

    pub fn new(buffer: &'a mut CodeBuffer) -> Self {
        Self {
            asm: Assembler::new(buffer),
        }
    }

    pub fn supports_avx(&self) -> bool {
        true
    }

    #[cold]
    #[inline(never)]
    fn unsupported_operands(&self, s: &str) {
        {
            unreachable!("{:?}", s);
        }
    }

    pub fn swap32(&mut self, src1: impl OperandCast, src2: impl OperandCast) {
        let src1 = *src1.as_operand();
        let src2 = *src2.as_operand();

        if src1.is_gp() && src2.is_gp() {
            if src1.id() == src2.id() {
                return;
            }
            self.asm.xchg32rr(src1, src2);
        } else if src1.is_mem() && src2.is_gp() {
            self.asm.xchg32mr(src1, src2);
        } else if src1.is_vec() && src2.is_vec() {
            if src1.id() == src2.id() {
                return;
            }
            self.mov64(Self::FP_TEMP_REGISTER, src1);
            self.mov64(src1, src2);
            self.mov64(src2, Self::FP_TEMP_REGISTER);
        } else {
            self.unsupported_operands("RR or RM pair expected");
        }
    }

    pub fn swap64(&mut self, src1: impl OperandCast, src2: impl OperandCast) {
        let src1 = *src1.as_operand();
        let src2 = *src2.as_operand();

        if src1.is_gp() && src2.is_gp() {
            if src1.id() == src2.id() {
                return;
            }
            self.asm.xchg64rr(src1, src2);
        } else if src1.is_mem() && src2.is_gp() {
            self.asm.xchg64mr(src1, src2);
        } else if src1.is_vec() && src2.is_vec() {
            if src1.id() == src2.id() {
                return;
            }
            self.mov64(Self::FP_TEMP_REGISTER, src1);
            self.mov64(src1, src2);
            self.mov64(src2, Self::FP_TEMP_REGISTER);
        } else {
            self.unsupported_operands("RR or RM pair expected");
        }
    }

    pub fn mov32(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = *src.as_operand();
        if dst.is_gp() && src.is_gp() {
            if dst.id() == src.id() {
                return;
            }
            self.asm.mov32rr(dst, src);
        } else if dst.is_gp() && src.is_imm() {
            self.asm.mov32ri(dst, src);
        } else if dst.is_vec() && src.is_vec() {
            if dst.id() == src.id() {
                return;
            }
            if self.supports_avx() {
                self.asm.vmovaps128rr(dst, src);
            } else {
                self.asm.sse_movapsrr(dst, src);
            }
        } else if dst.is_vec() && src.is_gp() {
            if self.supports_avx() {
                self.asm.vmovd_g2xrr(dst, src);
            } else {
                self.asm.sse_movd_g2xrr(dst, src);
            }
        } else if dst.is_gp() && src.is_vec() {
            if self.supports_avx() {
                self.asm.vmovd_x2grr(dst, src);
            } else {
                self.asm.sse_movd_x2grr(dst, src);
            }
        } else {
            self.unsupported_operands("RR or RI pair expected");
        }
    }

    pub fn mov64(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = *src.as_operand();
        if dst.is_reg() && src.is_reg() {
            if dst.id() == src.id() {
                return;
            }
            self.asm.mov64rr(dst, src);
        } else if dst.is_reg() && src.is_imm() {
            self.asm.mov64ri(dst, src);
        } else if dst.is_vec() && src.is_vec() {
            if dst.id() == src.id() {
                return;
            }
            if self.supports_avx() {
                self.asm.vmovaps128rr(dst, src);
            } else {
                self.asm.sse_movapsrr(dst, src);
            }
        } else if dst.is_vec() && src.is_gp() {
            if self.supports_avx() {
                self.asm.vmovq_g2xrr(dst, src);
            } else {
                self.asm.sse_movq_g2xrr(dst, src);
            }
        } else if dst.is_gp() && src.is_vec() {
            if self.supports_avx() {
                self.asm.vmovq_x2grr(dst, src);
            } else {
                self.asm.sse_movq_x2grr(dst, src);
            }
        } else {
            self.unsupported_operands("RR or RI pair expected");
        }
    }

    pub fn load8(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = *src.as_operand();
        if dst.is_reg() && src.is_mem() {
            self.asm.mov8rm(dst, src);
        } else {
            self.unsupported_operands("RM or MR pair expected");
        }
    }

    pub fn load16(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = *src.as_operand();
        if dst.is_reg() && src.is_mem() {
            self.asm.mov16rm(dst, src);
        } else {
            self.unsupported_operands("RM or MR pair expected");
        }
    }

    pub fn load32(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = *src.as_operand();
        if dst.is_reg() && src.is_mem() {
            self.asm.mov32rm(dst, src);
        } else if dst.is_vec() && src.is_mem() {
            if !self.supports_avx() {
                self.asm.sse_movssmr(dst, src);
            } else {
                self.asm.vmovssmr(dst, src);
            }
        } else {
            self.unsupported_operands("RM or MR pair expected");
        }
    }

    pub fn load64(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = *src.as_operand();
        if dst.is_reg() && src.is_mem() {
            self.asm.mov64rm(dst, src);
        } else if dst.is_vec() && src.is_mem() {
            if self.supports_avx() {
                self.asm.vmovsdrm(dst, src);
            } else {
                self.asm.sse_movsdrm(dst, src);
            }
        } else {
            self.unsupported_operands("RM or MR pair expected");
        }
    }

    pub fn store8(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = *src.as_operand();
        if dst.is_mem() && src.is_reg() {
            self.asm.mov8mr(dst, src);
        } else if dst.is_mem() && src.is_imm() {
            self.asm.mov8mi(dst, src);
        } else {
            self.unsupported_operands("MR or MI pair expected");
        }
    }

    pub fn store16(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = *src.as_operand();
        if dst.is_mem() && src.is_reg() {
            self.asm.mov16mr(dst, src);
        } else if dst.is_mem() && src.is_imm() {
            self.asm.mov16mi(dst, src);
        } else {
            self.unsupported_operands("MR or MI pair expected");
        }
    }

    pub fn store32(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = *src.as_operand();
        if dst.is_mem() && src.is_reg() {
            self.asm.mov32mr(dst, src);
        } else if dst.is_mem() && src.is_imm() {
            self.asm.mov32mi(dst, src);
        } else {
            self.unsupported_operands("MR or MI pair expected");
        }
    }

    pub fn store64(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = *src.as_operand();
        if dst.is_mem() && src.is_reg() {
            self.asm.mov64mr(dst, src);
        } else if dst.is_mem() && src.is_imm() {
            self.asm.mov64mi(dst, src);
        } else {
            self.unsupported_operands("MR or MI pair expected");
        }
    }

    pub fn zero32(&mut self, dst: impl OperandCast) {
        let dst = *dst.as_operand();

        if dst.is_gp() {
            self.asm.xor32rr(dst, dst);
        } else if dst.is_vec() {
            if self.supports_avx() {
                self.asm.vxorps128rrr(dst, dst, dst);
            } else {
                self.asm.sse_xorpsrr(dst, dst);
            }
        }
    }

    pub fn zero64(&mut self, dst: impl OperandCast) {
        let dst = *dst.as_operand();
        if dst.is_gp() {
            self.asm.xor64rr(dst, dst);
        } else if dst.is_vec() {
            if self.supports_avx() {
                self.asm.vxorps128rrr(dst, dst, dst);
            } else {
                self.asm.sse_xorpsrr(dst, dst);
            }
        } else {
            self.unsupported_operands("GP or VEC expected");
        }
    }

    pub fn add32(&mut self, dst: impl OperandCast, src1: impl OperandCast, src2: impl OperandCast) {
        let dst = *dst.as_operand();
        let src1 = *src1.as_operand();
        let src2 = *src2.as_operand();

        if dst.is_gp() && src1.is_gp() && src2.is_gp() {
            self.x86_lea32(dst, ptr32_index(src1.as_::<Gpq>(), src2.as_::<Gpq>(), 0, 0));
        } else if dst.is_gp() && src1.is_gp() && src2.is_imm() {
            if src1.id() == dst.id() {
                self.asm.add32ri(dst, src2);
            } else {
                self.x86_lea32(
                    dst,
                    ptr32(src1.as_::<Gpq>(), src2.as_::<Imm>().value() as i32),
                );
            }
        } else if dst.is_mem() && src1.is_gp() && src2.is_reg() {
            self.asm.mov32mr(dst, src1);
            self.asm.add32mr(dst, src2);
        } else if dst.is_mem() && src1.is_gp() && src2.is_imm() {
            self.asm.mov32mr(dst, src1);
            self.asm.add32mi(dst, src2);
        } else if dst.is_vec() && src1.is_vec() && src2.is_vec() {
            if self.supports_avx() {
                self.asm.vaddssrrr(dst, src1, src2);
            } else {
                if dst.id() == src1.id() {
                    self.asm.sse_addssrr(dst, src2);
                } else {
                    self.mov64(dst, src1);
                    self.asm.sse_addssrr(dst, src2);
                }
            }
        } else if dst.is_vec() && src1.is_vec() && src2.is_mem() {
            if self.supports_avx() {
                self.asm.vaddssrrm(dst, src1, src2);
            } else {
                self.mov64(dst, src1);
                self.asm.sse_addsdrm(dst, src2);
            }
        } else {
            self.unsupported_operands("RRR, RRI, or VVV expected");
        }
    }

    pub fn add64(&mut self, dst: impl OperandCast, src1: impl OperandCast, src2: impl OperandCast) {
        let dst = *dst.as_operand();
        let src1 = *src1.as_operand();
        let src2 = *src2.as_operand();

        if dst.is_gp() && src1.is_gp() && src2.is_gp() {
            self.x86_lea64(dst, ptr64_index(src1.as_::<Gpq>(), src2.as_::<Gpq>(), 0, 0));
        } else if dst.is_gp() && src1.is_gp() && src2.is_imm() {
            if src1.id() == dst.id() {
                self.asm.add64ri(dst, src2);
            } else {
                self.x86_lea64(
                    dst,
                    ptr64(src1.as_::<Gpq>(), src2.as_::<Imm>().value() as i32),
                );
            }
        } else if dst.is_gp() && src1.is_gp() && src2.is_mem() {
            if src1.id() == dst.id() {
                self.asm.add64rm(dst, src2);
            } else {
                self.mov64(dst, src1);
                self.asm.add64rm(dst, src2);
            }
        } else if dst.is_mem() && src1.is_gp() && src2.is_reg() {
            self.asm.mov64mr(dst, src1);
            self.asm.add64mr(dst, src2);
        } else if dst.is_mem() && src1.is_gp() && src2.is_imm() {
            self.asm.mov64mr(dst, src1);
            self.asm.add64mi(dst, src2);
        } else if dst.is_vec() && src1.is_vec() && src2.is_vec() {
            if self.supports_avx() {
                self.asm.vaddsdrrr(dst, src1, src2);
            } else {
                if dst.id() == src1.id() {
                    self.asm.sse_addsdrr(dst, src2);
                } else {
                    self.mov64(dst, src1);
                    self.asm.sse_addsdrr(dst, src2);
                }
            }
        } else if dst.is_vec() && src1.is_vec() && src2.is_vec() {
            if self.supports_avx() {
                self.asm.vaddsdrrm(dst, src1, src2);
            } else {
                if dst.id() == src1.id() {
                    self.asm.sse_addsdrm(dst, src2);
                } else {
                    self.mov64(dst, src1);
                    self.asm.sse_addsdrm(dst, src2);
                }
            }
        } else {
            self.unsupported_operands("GP and GP or GP and IMM expected");
        }
    }

    pub fn x86_lea32(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = src.as_operand().as_::<Mem>();

        if !src.has_offset() && !src.has_shift() {
            if src.base_id() == dst.id() {
                self.asm.add32rr(dst, Gpq::from_id(src.index_id()));
                return;
            }

            if src.index_id() == dst.id() {
                self.asm.add32rr(dst, Gpq::from_id(src.base_id()));
                return;
            }
        }

        self.asm.lea32rm(dst, src);
    }

    pub fn x86_lea64(&mut self, dst: impl OperandCast, src: impl OperandCast) {
        let dst = *dst.as_operand();
        let src = src.as_operand().as_::<Mem>();

        if !src.has_offset() && !src.has_shift() {
            if src.base_id() == dst.id() {
                self.asm.add64rr(dst, Gpq::from_id(src.index_id()));
                return;
            }

            if src.index_id() == dst.id() {
                self.asm.add64rr(dst, Gpq::from_id(src.base_id()));
                return;
            }
        }

        self.asm.lea64rm(dst, src);
    }

    pub fn sub32(&mut self, dst: impl OperandCast, src1: impl OperandCast, src2: impl OperandCast) {
        let dst = *dst.as_operand();
        let src1 = *src1.as_operand();
        let src2 = *src2.as_operand();

        if dst.is_gp() && src1.is_gp() && src2.is_gp() {
            if dst.id() == src1.id() {
                self.neg32(dst);
                self.add32(dst, dst, src2);
            } else {
                self.mov32(dst, src1);
                self.asm.sub32rr(dst, src2);
            }
        } else if dst.is_gp() && src1.is_gp() && src2.is_imm() {
            if dst.id() == src1.id() {
                self.asm.sub32ri(dst, src2);
            } else {
                self.x86_lea32(
                    dst,
                    ptr32(
                        src1.as_::<Gpq>(),
                        (src2.as_::<Imm>().value() as i32).wrapping_neg(),
                    ),
                );
            }
        } else if dst.is_gp() && src1.is_gp() && src2.is_mem() {
            if dst.id() == src1.id() {
                self.asm.sub32rm(dst, src2);
            } else {
                self.asm.mov32rr(dst, src1);
                self.asm.sub32rm(dst, src2);
            }
        } else if dst.is_mem() && src1.is_gp() && src2.is_gp() {
            self.asm.mov32mr(dst, src1);
            self.asm.sub32mr(dst, src2);
        } else if dst.is_mem() && src1.is_gp() && src2.is_imm() {
            self.asm.mov32mr(dst, src1);
            self.asm.sub32mi(dst, src2);
        } else if dst.is_vec() && src1.is_vec() && src2.is_vec() {
            if self.supports_avx() {
                self.asm.vsubssrrr(dst, src1, src2);
            } else {
                // B := A - B is invalid.
                if src1.id() != dst.id() && src2.id() == dst.id() {
                    self.mov32(Self::FP_TEMP_REGISTER, src2);
                    self.mov64(dst, src1);
                    self.asm.sse_subssrr(dst, Self::FP_TEMP_REGISTER);
                } else {
                    self.mov64(dst, src1);
                    self.asm.sse_subssrr(dst, src2);
                }
            }
        } else if dst.is_vec() && src1.is_vec() && src2.is_vec() {
            if self.supports_avx() {
                self.asm.vsubssrrm(dst, src1, src2);
            } else {
                self.mov64(dst, src1);
                self.asm.sse_subssrm(dst, src2);
            }
        } else {
            self.unsupported_operands("RRR or RRI expected");
        }
    }

    pub fn sub64(&mut self, dst: impl OperandCast, src1: impl OperandCast, src2: impl OperandCast) {
        let dst = *dst.as_operand();
        let src1 = *src1.as_operand();
        let src2 = *src2.as_operand();

        if dst.is_gp() && src1.is_gp() && src2.is_gp() {
            if dst.id() == src1.id() {
                self.neg64(dst);
                self.add64(dst, dst, src2);
            } else {
                self.mov64(dst, src1);
                self.asm.sub64rr(dst, src2);
            }
        } else if dst.is_gp() && src1.is_gp() && src2.is_imm() {
            if dst.id() == src1.id() {
                self.asm.sub64ri(dst, src2);
            } else {
                self.x86_lea64(
                    dst,
                    ptr64(
                        src1.as_::<Gpq>(),
                        (src2.as_::<Imm>().value() as i32).wrapping_neg(),
                    ),
                );
            }
        } else if dst.is_gp() && src1.is_gp() && src2.is_mem() {
            if dst.id() == src1.id() {
                self.asm.sub64rm(dst, src2);
            } else {
                self.asm.mov64rr(dst, src1);
                self.asm.sub64rm(dst, src2);
            }
        } else if dst.is_mem() && src1.is_gp() && src2.is_gp() {
            self.asm.mov64mr(dst, src1);
            self.asm.sub64mr(dst, src2);
        } else if dst.is_mem() && src1.is_gp() && src2.is_imm() {
            self.asm.mov64mr(dst, src1);
            self.asm.sub64mi(dst, src2);
        } else if dst.is_vec() && src1.is_vec() && src2.is_vec() {
            if self.supports_avx() {
                self.asm.vsubsdrrr(dst, src1, src2);
            } else {
                // B := A - B is invalid.
                if src1.id() != dst.id() && src2.id() == dst.id() {
                    self.mov64(Self::FP_TEMP_REGISTER, src2);
                    self.mov64(dst, src1);
                    self.asm.sse_subsdrr(dst, Self::FP_TEMP_REGISTER);
                } else {
                    self.mov64(dst, src1);
                    self.asm.sse_subsdrr(dst, src2);
                }
            }
        } else if dst.is_vec() && src1.is_vec() && src2.is_mem() {
            if self.supports_avx() {
                self.asm.vsubsdrrm(dst, src1, src2);
            } else {
                self.mov64(dst, src1);
                self.asm.sse_subsdrm(dst, src2);
            }
        } else {
            self.unsupported_operands("RRR or RRI expected");
        }
    }

    pub fn neg32(&mut self, srcdst: impl OperandCast) {
        let srcdst = *srcdst.as_operand();
        if srcdst.is_gp() {
            self.asm.neg32r(srcdst);
        } else if srcdst.is_mem() {
            self.asm.neg32m(srcdst);
        } else {
            self.unsupported_operands("GP or MEM expected");
        }
    }

    pub fn neg64(&mut self, srcdst: impl OperandCast) {
        let srcdst = *srcdst.as_operand();
        if srcdst.is_gp() {
            self.asm.neg64r(srcdst);
        } else if srcdst.is_mem() {
            self.asm.neg64m(srcdst);
        } else {
            self.unsupported_operands("GP or MEM expected");
        }
    }

    pub fn mul64(&mut self, dst: impl OperandCast, src1: impl OperandCast, src2: impl OperandCast) {
        let dst = *dst.as_operand();
        let src1 = *src1.as_operand();
        let src2 = *src2.as_operand();

        if dst.is_gp() && src1.is_gp() && src2.is_gp() {
            if dst.id() == src1.id() {
                self.asm.imul64rr(dst, src2);
            } else if dst.id() == src2.id() {
                self.asm.imul64rr(dst, src1);
            } else {
                self.mov64(dst, src1);
                self.asm.imul64rr(dst, src2);
            }
        } else if dst.is_gp() && src1.is_gp() && src2.is_mem() {
            self.mov64(dst, src1);
            self.asm.imul64rm(dst, src2);
        } else if dst.is_gp() && src1.is_mem() && src2.is_gp() {
            self.mov64(dst, src2);
            self.asm.imul64rm(dst, src1);
        } else if dst.is_gp() && src1.is_gp() && src2.is_imm() {
            self.asm.mov64ri(dst, src2);
            self.asm.imul64rr(dst, src1);
        } else if dst.is_vec() && src1.is_vec() && src2.is_vec() {
            if self.supports_avx() {
                self.asm.vmulsdrrr(dst, src1, src2);
            } else {
                self.mov64(dst, src1);
                self.asm.sse_mulsdrr(dst, src2);
            }
        } else if dst.is_vec() && src1.is_vec() && src2.is_mem() {
            if self.supports_avx() {
                self.asm.vmulsdrrm(dst, src1, src2);
            } else {
                self.mov64(dst, src1);
                self.asm.sse_mulsdrm(dst, src2);
            }
        } else {
            self.unsupported_operands("RRR expected");
        }
    }

    pub fn mul32(&mut self, dst: impl OperandCast, src1: impl OperandCast, src2: impl OperandCast) {
        let dst = *dst.as_operand();
        let src1 = *src1.as_operand();
        let src2 = *src2.as_operand();

        if dst.is_gp() && src1.is_gp() && src2.is_gp() {
            if dst.id() == src1.id() {
                self.asm.imul32rr(dst, src2);
            } else if dst.id() == src2.id() {
                self.asm.imul32rr(dst, src1);
            } else {
                self.mov32(dst, src1);
                self.asm.imul32rr(dst, src2);
            }
        } else if dst.is_gp() && src1.is_gp() && src2.is_mem() {
            self.mov32(dst, src1);
            self.asm.imul32rm(dst, src2);
        } else if dst.is_gp() && src1.is_mem() && src2.is_gp() {
            self.mov32(dst, src2);
            self.asm.imul32rm(dst, src1);
        } else if dst.is_gp() && src1.is_gp() && src2.is_imm() {
            self.asm.mov32ri(dst, src2);
            self.asm.imul32rr(dst, src1);
        } else if dst.is_vec() && src1.is_vec() && src2.is_vec() {
            if self.supports_avx() {
                self.asm.vmulssrrr(dst, src1, src2);
            } else {
                self.mov32(dst, src1);
                self.asm.sse_mulssrr(dst, src2);
            }
        } else if dst.is_vec() && src1.is_vec() && src2.is_mem() {
            if self.supports_avx() {
                self.asm.vmulssrrm(dst, src1, src2);
            } else {
                self.mov32(dst, src1);
                self.asm.sse_mulssrm(dst, src2);
            }
        } else {
            self.unsupported_operands("RRR expected");
        }
    }

    pub fn div64(&mut self, dst: impl OperandCast, src1: impl OperandCast, src2: impl OperandCast) {
        let dst = *dst.as_operand();
        let src1 = *src1.as_operand();
        let src2 = *src2.as_operand();

        if dst.is_gp() && src1.is_gp() && src2.is_gp() {
            self.mov64(RAX, src1);
            self.asm.cqo();
            self.asm.idiv64r(src2);
            self.mov64(dst, RAX);
        } else if dst.is_gp() && src1.is_gp() && src2.is_mem() {
            self.mov64(RAX, src1);
            self.asm.cqo();
            self.asm.idiv64m(src2);
            self.mov64(dst, RAX);
        } else if dst.is_gp() && src1.is_mem() && src2.is_gp() {
            self.asm.mov64rm(RAX, src1);
            self.asm.cqo();
            self.asm.idiv64r(src2);
            self.mov64(dst, RAX);
        } else if dst.is_vec() && src1.is_vec() && src2.is_vec() {
            if self.supports_avx() {
                self.asm.vdivsdrrr(dst, src1, src2);
            } else {
                self.mov64(dst, src1);
                self.asm.sse_divsdrr(dst, src2);
            }
        } else if dst.is_vec() && src1.is_vec() && src2.is_mem() {
            if self.supports_avx() {
                self.asm.vdivsdrrm(dst, src1, src2);
            } else {
                self.mov64(dst, src1);
                self.asm.sse_divsdrm(dst, src2);
            }
        } else {
            self.unsupported_operands("RRR or RRI expected");
        }
    }
}
