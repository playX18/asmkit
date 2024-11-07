use crate::assembler::assembler::BaseAssembler;
use crate::assembler::binemit::{BranchTarget, Label, LabelUse, Reloc};
use crate::assembler::codeholder::{CodeBuffer, ExternalName, RelocTarget};
use crate::encdec::riscv::*;
use crate::AsmError;
/// An underlying assembler behind `Assembler`. This assembler type solely implements
/// instruction encoding without labels and relocations. For full-fledged assembler look into [`Assembler`].
pub struct RawRISCVAssembler<'a> {
    base: BaseAssembler<'a>,
    long: bool,
}

impl<'a> RawRISCVAssembler<'a> {
    pub fn emit(&mut self, inst: u32) {
        self.base.buffer.write_u32(inst);
        self.long = false;
    }

    pub fn emit_compressed(&mut self, inst: u16) {
        self.base.buffer.write_u16(inst);
        self.long = false;
    }
}

impl<'a> core::ops::Deref for RawRISCVAssembler<'a> {
    type Target = BaseAssembler<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a> core::ops::DerefMut for RawRISCVAssembler<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

include!("impl.rs");

/// RV32/RV64 assembler implementation.
///
/// [`riscv::Assembler`](Assembler) is a code emitter that emits machine code directly into the [CodeBuffer]. The assembler is capable
/// of targeting both 32-bit and 64-bit instruction sets.
///
/// ## Supported extensions
///
/// We aim to support all extensions from RISCV specification (except unratified ones). At the moment *every* extension except `C` is supported
/// in full capacity. For compressed encoding we're lacking in immediate and register encoding, you still can try to emit compressed opcodes
/// but they're not guaranteed to be valid at the moment (they're all prefixed with `c_` in API).
pub struct Assembler<'a> {
    pub base: RawRISCVAssembler<'a>,
}

impl<'a> Assembler<'a> {
    pub fn new(buffer: &'a mut CodeBuffer) -> Self {
        Self {
            base: RawRISCVAssembler {
                base: BaseAssembler { buffer },
                long: false,
            },
        }
    }

    /// Emit 'long' instruction or instruction sequence. This is useful if you want to emit 64-bit immediate
    /// that is patched later. Otherwise opcodes like `li` will emit only required amount of instructions for immediate.
    pub fn long(&mut self) -> &mut Self {
        self.long = true;
        self
    }

    /// A pseudo-op to load `label` address to `rd`.
    pub fn la(&mut self, rd: Reg, target: impl Into<BranchTarget>) -> Result<(), AsmError> {
        match target.into() {
            BranchTarget::Label(label) => {
                let off = self.buffer.cur_offset();
                self.buffer
                    .use_label_at_offset(off, label, LabelUse::RVPCRelHi20);
                self.auipc(rd, 0)?;
                let off = self.buffer.cur_offset();
                self.buffer
                    .use_label_at_offset(off, label, LabelUse::RVPCRelLo12I);
                self.addi(rd, rd, 0)?;
            }

            BranchTarget::Ext(ext) => {
                let off = self.buffer.cur_offset();
                self.buffer.add_reloc_at_offset(
                    off,
                    Reloc::RiscvAbs8,
                    RelocTarget::ExternalName(ext),
                    0,
                );
                self.long().li(rd, 0)?;
                self.long = false;
            }
        }
        Ok(())
    }


    pub fn mv(&mut self, rd: Reg, rs: Reg) -> Result<(), AsmError> {
        self.addi(rd, rs, 0)
    }

    pub fn jal(&mut self, rd: Reg, label: Label) -> Result<(), AsmError> {
        let offset = self.offset();
        self.buffer
            .use_label_at_offset(offset, label, LabelUse::RVJal20);
        self.base.jal(rd, 0)
    }

    pub fn j(&mut self, label: Label) -> Result<(), AsmError> {
        let offset = self.offset();
        self.buffer
            .use_label_at_offset(offset, label, LabelUse::RVJal20);
        self.base.j(0)
    }

    pub fn c_j(&mut self, label: Label) -> Result<(), AsmError> {
        let offset = self.offset();
        self.buffer
            .use_label_at_offset(offset, label, LabelUse::RVCJump);
        self.base.c_j(0)
    }

    pub fn beq(&mut self, rs1: Reg, rs2: Reg, label: Label) -> Result<(), AsmError> {
        let offset = self.offset();
        self.buffer
            .use_label_at_offset(offset, label, LabelUse::RVB12);
        self.base.beq(0, rs1, rs2)
    }

    pub fn ble(&mut self, rs1: Reg, rs2: Reg, label: Label) -> Result<(), AsmError> {
        let offset = self.offset();
        self.buffer
            .use_label_at_offset(offset, label, LabelUse::RVB12);
        self.base.ble(0, rs1, rs2)
    }

    pub fn bge(&mut self, rs1: Reg, rs2: Reg, label: Label) -> Result<(), AsmError> {
        let offset = self.offset();
        self.buffer
            .use_label_at_offset(offset, label, LabelUse::RVB12);
        self.base.bge(0, rs1, rs2)
    }

    pub fn blt(&mut self, rs1: Reg, rs2: Reg, label: Label) -> Result<(), AsmError> {
        let offset = self.offset();
        self.buffer
            .use_label_at_offset(offset, label, LabelUse::RVB12);
        self.base.blt(0, rs1, rs2)
    }

    pub fn bltu(&mut self, rs1: Reg, rs2: Reg, label: Label) -> Result<(), AsmError> {
        let offset = self.offset();
        self.buffer
            .use_label_at_offset(offset, label, LabelUse::RVB12);
        self.base.bltu(0, rs1, rs2)
    }

    pub fn li(&mut self, rd: Reg, imm: i64) -> Result<(), AsmError> {
        if !self.long
            && imm >= i32::MIN as i64
            && imm <= i32::MAX as i64
            && is_immediate_valid(&C_IMM6LOHI, imm as i32)
        {
            return self.c_li(rd, imm as _);
        }
        if self.long {
            ImmediateLoader::placeholder64(imm)
        } else {
            ImmediateLoader::load64(imm)
        }
        .move_into(self, rd)
    }

    /// Patch `li` pseudo-op in buffer.
    ///
    /// # Arguments
    /// - `buffer`: encoded `li` pseudo-op bytes.
    /// - `rd`: destination register to store immediate at or None to fetch from `buffer`.
    /// - `imm`: immediate to patch into code.
    pub fn patch_li(buffer: &mut [u8], rd: Option<Reg>, imm: i64) -> Result<(), AsmError> {
        let rd = rd.unwrap_or_else(|| {
            let range = (7 * 4)..(7 * 4) + 4;
            Reg(InstructionValue::new(u32::from_le_bytes(buffer[range].try_into().unwrap())).rd())
        });

        let imml = ImmediateLoader::load64(imm);
        for i in 0..imml.op_count {
            println!("{:x} -> {:?}", imm, imml.ops[imml.op_count - (i + 1)]);
            let range = (i * 4)..(i * 4) + 4;
            match imml.ops[imml.op_count - (i + 1)] {
                ImmOp::Addi(imm) => {
                    buffer[range].copy_from_slice(&addi(rd, rd, imm).to_le_bytes());
                }

                ImmOp::IImmediate(imm) => {
                    buffer[range].copy_from_slice(&addi(rd, ZERO, imm).to_le_bytes());
                }

                ImmOp::LShift12 => {
                    buffer[range].copy_from_slice(&slli(rd, rd, 12).to_le_bytes());
                }

                ImmOp::Lui(imm) => {
                    buffer[range].copy_from_slice(&lui(rd, imm).to_le_bytes());
                }

                ImmOp::Nop => {
                    buffer[range].copy_from_slice(&addi(ZERO, ZERO, 0).to_le_bytes());
                }
            }
        }

        Ok(())
    }

    // LLVM Code
    //===- RISCVMatInt.cpp - Immediate materialisation -------------*- C++
    //-*--===//
    //
    //  Part of the LLVM Project, under the Apache License v2.0 with LLVM
    //  Exceptions. See https://llvm.org/LICENSE.txt for license information.
    //  SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
    //
    //===----------------------------------------------------------------------===//

    pub fn recursive_li(&mut self, rd: Reg, val: i64) -> Result<(), AsmError> {
        if val > 0 && self.recursive_li_impl_count(val) > 2 {
            let leading_zeros = (val as u64).leading_zeros();
            let shifted_val = (val as u64) << leading_zeros;
            let count_fill_zero = self.recursive_li_impl_count(shifted_val as _) + 1;
            if count_fill_zero < self.recursive_li_impl_count(val) {
                self.recursive_li_impl(rd, shifted_val as _)?;
                self.slli(rd, rd, leading_zeros as _)?;
                return Ok(());
            }
        }

        self.recursive_li_impl(rd, val)
    }
    fn recursive_li_impl(&mut self, rd: Reg, val: i64) -> Result<(), AsmError> {
        if val as i32 as i64 == val {
            // Depending on the active bits in the immediate Value v, the following
            // instruction sequences are emitted:
            //
            // v == 0                        : ADDI
            // v[0,12) != 0 && v[12,32) == 0 : ADDI
            // v[0,12) == 0 && v[12,32) != 0 : LUI
            // v[0,32) != 0                  : LUI+ADDI(W)
            let hi20 = ((val + 0x800) >> 12) & 0xfffff;
            let lo12 = val << 52 >> 52;

            if hi20 != 0 {
                self.lui(rd, hi20 as i32)?;
            }

            if lo12 != 0 || hi20 == 0 {
                if hi20 != 0 {
                    self.addiw(rd, rd, lo12 as i32)?;
                } else {
                    self.addi(rd, ZERO, lo12 as i32)?;
                }
            }
            return Ok(());
        }

        // In the worst case, for a full 64-bit constant, a sequence of 8
        // instructions (i.e., LUI+ADDIW+SLLI+ADDI+SLLI+ADDI+SLLI+ADDI) has to be
        // emitted. Note that the first two instructions (LUI+ADDIW) can contribute
        // up to 32 bits while the following ADDI instructions contribute up to 12
        // bits each.
        //
        // On the first glance, implementing this seems to be possible by simply
        // emitting the most significant 32 bits (LUI+ADDIW) followed by as many
        // left shift (SLLI) and immediate additions (ADDI) as needed. However, due
        // to the fact that ADDI performs a sign extended addition, doing it like
        // that would only be possible when at most 11 bits of the ADDI instructions
        // are used. Using all 12 bits of the ADDI instructions, like done by GAS,
        // actually requires that the constant is processed starting with the least
        // significant bit.
        //
        // In the following, constants are processed from LSB to MSB but instruction
        // emission is performed from MSB to LSB by recursively calling
        // generateInstSeq. In each recursion, first the lowest 12 bits are removed
        // from the constant and the optimal shift amount, which can be greater than
        // 12 bits if the constant is sparse, is determined. Then, the shifted
        // remaining constant is processed recursively and gets emitted as soon as
        // it fits into 32 bits. The emission of the shifts and additions is
        // subsequently performed when the recursion returns.
        let lo12 = val << 52 >> 52;
        let mut hi52 = ((val as u64 + 0x800) >> 12) as i64;
        let mut shift_amount = 12 + (hi52 as u64).trailing_zeros();
        hi52 = Self::sign_extend(
            (hi52 >> (shift_amount - 12)) as _,
            64 - shift_amount as usize,
        );

        // If the remaining bits don't fit in 12 bits, we might be able to reduce
        // the shift amount in order to use LUI which will zero the lower 12 bits.
        let unsigned = false;
        if shift_amount > 12
            && (hi52 < i32::MIN as i64
                || hi52 > i32::MAX as i64
                || !is_immediate_valid(&IMM12, hi52 as _))
        {
            let imm = ((hi52 as u64) << 12) as i64;

            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                // Reduce the shift amount and add zeros to the LSBs so it will match
                // LUI.
                shift_amount -= 12;
                hi52 = ((hi52 as u64) << 12) as i64;
            }
        }

        self.recursive_li(rd, hi52)?;

        if unsigned {
        } else {
            self.slli(rd, rd, shift_amount as _)?;
        }

        if lo12 != 0 {
            self.addi(rd, rd, lo12 as i32)?;
        }

        Ok(())
    }

    fn sign_extend(v: u64, n: usize) -> i64 {
        (v << (64 - n)) as i64 >> (64 - n)
    }
    #[allow(dead_code, unused_assignments)]
    fn recursive_li_impl_count(&self, val: i64) -> usize {
        let mut count = 0;
        if val >= i32::MIN as i64 && val <= i32::MAX as i64 {
            // Depending on the active bits in the immediate Value v, the following
            // instruction sequences are emitted:
            //
            // v == 0                        : ADDI
            // v[0,12) != 0 && v[12,32) == 0 : ADDI
            // v[0,12) == 0 && v[12,32) != 0 : LUI
            // v[0,32) != 0                  : LUI+ADDI(W)
            let hi20 = ((val + 0x800) >> 12) & 0xfffff;
            let lo12 = val << 52 >> 52;

            if hi20 != 0 {
                // lui(rd, (int32_t)Hi20);
                count += 1;
            }

            if lo12 != 0 || hi20 == 0 {
                //   unsigned AddiOpc = (IsRV64 && Hi20) ? RISCV::ADDIW : RISCV::ADDI;
                //   Res.push_back(RISCVMatInt::Inst(AddiOpc, Lo12));
                count += 1;
            }

            return count;
        }

        // In the worst case, for a full 64-bit constant, a sequence of 8
        // instructions (i.e., LUI+ADDIW+SLLI+ADDI+SLLI+ADDI+SLLI+ADDI) has to be
        // emitted. Note that the first two instructions (LUI+ADDIW) can contribute
        // up to 32 bits while the following ADDI instructions contribute up to 12
        // bits each.
        //
        // On the first glance, implementing this seems to be possible by simply
        // emitting the most significant 32 bits (LUI+ADDIW) followed by as many
        // left shift (SLLI) and immediate additions (ADDI) as needed. However, due
        // to the fact that ADDI performs a sign extended addition, doing it like
        // that would only be possible when at most 11 bits of the ADDI instructions
        // are used. Using all 12 bits of the ADDI instructions, like done by GAS,
        // actually requires that the constant is processed starting with the least
        // significant bit.
        //
        // In the following, constants are processed from LSB to MSB but instruction
        // emission is performed from MSB to LSB by recursively calling
        // generateInstSeq. In each recursion, first the lowest 12 bits are removed
        // from the constant and the optimal shift amount, which can be greater than
        // 12 bits if the constant is sparse, is determined. Then, the shifted
        // remaining constant is processed recursively and gets emitted as soon as
        // it fits into 32 bits. The emission of the shifts and additions is
        // subsequently performed when the recursion returns.
        let lo12 = val << 52 >> 52;
        let mut hi52 = ((val as u64 + 0x800) >> 12) as i64;
        let mut shift_amount = 12 + (hi52 as u64).trailing_zeros();
        hi52 = Self::sign_extend(
            (hi52 >> (shift_amount - 12)) as u64,
            64 - shift_amount as usize,
        );

        // If the remaining bits don't fit in 12 bits, we might be able to reduce
        // the shift amount in order to use LUI which will zero the lower 12 bits.
        let unsigned = false;
        if shift_amount > 12
            && (hi52 < i32::MIN as i64
                || hi52 > i32::MAX as i64
                || !is_immediate_valid(&IMM12, hi52 as _))
        {
            let imm = ((hi52 as u64) << 12) as i64;

            if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                // Reduce the shift amount and add zeros to the LSBs so it will match
                // LUI.
                shift_amount -= 12;
                hi52 = ((hi52 as u64) << 12) as i64;
            }
        }

        count += 1;

        if unsigned {
        } else {
            // slli(rd, rd, shift_amount)
            count += 1;
        }

        if lo12 != 0 {
            count += 1;
        }

        count
    }
}
#[allow(dead_code, unused_assignments)]
const fn sext<const B: i64>(val: i64) -> i64 {
    (val << (64 - B)) >> (64 - B)
}
#[allow(dead_code, unused_assignments)]
const fn sext_(val: i64, b: i64) -> i64 {
    (val << (64 - b)) >> (64 - b)
}

impl<'a> core::ops::Deref for Assembler<'a> {
    type Target = RawRISCVAssembler<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a> core::ops::DerefMut for Assembler<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub use crate::encdec::riscv::regs::*;

#[derive(Copy, Clone, Default, Debug)]
enum ImmOp {
    IImmediate(i32),
    Lui(i32),
    Addi(i32),
    LShift12,
    #[default]
    Nop,
}

#[derive(Default)]
pub struct ImmediateLoader {
    ops: [ImmOp; 8],
    op_count: usize,
}

impl ImmediateLoader {
    pub fn load64(imm: i64) -> Self {
        let mut this = Self::default();

        if imm >= i32::MIN as i64
            && imm <= i32::MAX as i64
            && is_immediate_valid(&IMM12, imm as i32)
        {
            this.ops[0] = ImmOp::IImmediate(imm as _);
            this.op_count += 1;
            return this;
        }

        let mut value = imm;

        loop {
            let addi_imm = (value & ((1 << 12) - 1)) as u32;

            if (addi_imm & (1 << 11)) != 0 {
                value += 1 << 12;
            }

            this.ops[this.op_count] = ImmOp::Addi(addi_imm as _);
            this.op_count += 1;

            // shift out the bits just incorporated into addi
            value >>= 12;
            // If the remainder of the immediate can fit into a 20-bit immediate, we can generate the LUI instruction that will end up
            // loading the initial higher bits of the desired immediate.
            if value >= i32::MIN as i64
                && value <= i32::MAX as i64
                && is_immediate_valid(&IMM20, value as i32)
            {
                this.ops[this.op_count] = ImmOp::Lui((value & ((1 << 20) - 1) << 12) as i32);
                this.op_count += 1;
                return this;
            }

            this.ops[this.op_count] = ImmOp::LShift12;
            this.op_count += 1;
        }
    }

    pub fn load32(imm: i32) -> Self {
        Self::load64(imm as i64)
    }

    pub fn placeholder64(imm: i64) -> Self {
        let mut this = Self::load64(imm);
        // The non-placeholder constructor already generated the necessary operations to load this immediate.
        // This constructor still fills out the remaining potential operations as nops. This enables future patching
        // of these instructions with other immediate-load sequences.

        for i in this.op_count..this.ops.len() {
            this.ops[i] = ImmOp::Nop;
        }
        this.op_count = this.ops.len();

        this
    }

    pub fn placeholder32(imm: i32) -> Self {
        Self::placeholder64(imm as _)
    }

    pub fn move_into(self, assembler: &mut RawRISCVAssembler, dest: Reg) -> Result<(), AsmError> {
        for i in 0..self.op_count {
            match self.ops[self.op_count - (i + 1)] {
                ImmOp::IImmediate(imm) => {
                    assembler.addi(dest, ZERO, imm)?;
                }

                ImmOp::Lui(imm) => {
                    assembler.lui(dest, imm)?;
                }

                ImmOp::Addi(imm) => {
                    assembler.addi(dest, dest, imm)?;
                }

                ImmOp::LShift12 => {
                    assembler.slli(dest, dest, 12)?;
                }

                ImmOp::Nop => {
                    assembler.addi(ZERO, ZERO, 0)?;
                }
            }
        }

        Ok(())
    }
}
