use crate::assembler::binemit::Label;
use crate::assembler::codeholder::{ExternalName, RelocTarget};
use crate::{assembler::assembler::BaseAssembler, assembler::codeholder::CodeBuffer, AsmError};

/// An underlying assembler behind `Assembler`. This assembler type solely implements
/// instruction encoding without labels and relocations. For full-fledged assembler look into [`Assembler`].
pub struct RawX86Assembler<'a> {
    base: BaseAssembler<'a>,
    flags: u32,
    opmask: Mask,
}

impl<'a> core::ops::Deref for RawX86Assembler<'a> {
    type Target = BaseAssembler<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a> core::ops::DerefMut for RawX86Assembler<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'a> RawX86Assembler<'a> {
    fn emit(&mut self, inst: Inst) -> Result<(), AsmError> {
        for byte in inst.as_bytes() {
            self.buffer.write_u8(*byte);
        }
        self.flags = 0;
        self.opmask = K0;
        Ok(())
    }

    pub fn new(buffer: &'a mut CodeBuffer) -> Self {
        Self {
            base: BaseAssembler { buffer },
            flags: 0,
            opmask: K0,
        }
    }

    pub fn k(&mut self, k: Mask) -> &mut Self {
        self.opmask = k;
        self
    }

    pub fn long(&mut self) -> &mut Self {
        self.flags |= LONG;
        self
    }

    pub fn is_long(&self) -> bool {
        self.flags & LONG != 0
    }

    pub fn addr32(&mut self) -> &mut Self {
        self.flags |= ADDR32;
        self
    }

    pub fn es(&mut self) -> &mut Self {
        self.flags = (self.flags & !SEG_MASK) | (ES.0 as u32 + 1);
        self
    }

    pub fn cs(&mut self) -> &mut Self {
        self.flags = (self.flags & !SEG_MASK) | (CS.0 as u32 + 1);
        self
    }

    pub fn ss(&mut self) -> &mut Self {
        self.flags = (self.flags & !SEG_MASK) | (SS.0 as u32 + 1);
        self
    }

    pub fn ds(&mut self) -> &mut Self {
        self.flags = (self.flags & !SEG_MASK) | (DS.0 as u32 + 1);
        self
    }

    pub fn fs(&mut self) -> &mut Self {
        self.flags = (self.flags & !SEG_MASK) | (FS.0 as u32 + 1);
        self
    }

    pub fn gs(&mut self) -> &mut Self {
        self.flags = (self.flags & !SEG_MASK) | (GS.0 as u32 + 1);
        self
    }

    pub fn rounding_control(&mut self, rc: u32) -> &mut Self {
        self.flags = (self.flags & !RC_MASK) | (rc & RC_MASK);
        self
    }

    pub fn offset(&self) -> u32 {
        self.buffer.cur_offset()
    }

    pub fn is_bound(&self, _label: Label) -> bool {
        false
    }

    /// Return offset for 32-bit reloc. `uasm` does not return offset
    /// of immediate for relocation in encoding API so we instead have this
    /// simple function which just returns `offset() - 4`. In X86/X64 *all* immeidates
    /// are always go after opcode so its a safe assumption to do this subtraction.
    pub fn offset_for_reloc32(&self) -> CodeOffset {
        self.offset() - 4
    }

    /// Return offset for 64-bit reloc. `uasm` does not return offset
    /// of immediate for relocation in encoding API so we instead have this
    /// simple function which just returns `offset() - 8`. In X64 *all* immeidates
    /// are always go after opcode so its a safe assumption to do this subtraction.
    pub fn offset_for_reloc64(&self) -> CodeOffset {
        self.offset() - 8
    }
}

use crate::encdec::x86;
pub use crate::encdec::x86::v2::{
    Cr, Dr, Gp, GpH, GpLH, Inst, Mask, Mm, SReg, St, Tmm, Xmm, ADDR32, AH, AX, BH, BP, BX, CH, CS,
    CX, DH, DI, DS, DX, ES, FS, GS, IP, K0, K1, K2, K3, K4, K5, K6, K7, LONG, MM0, MM1, MM2, MM3,
    MM4, MM5, MM6, MM7, NOREG, R10, R11, R12, R13, R14, R15, R8, R9, RC_MASK, RC_RD, RC_RN, RC_RU,
    RC_RZ, SEG_MASK, SI, SP, SS, ST0, ST1, ST2, ST3, ST4, ST5, ST6, ST7, TMM0, TMM1, TMM2, TMM3,
    TMM4, TMM5, TMM6, TMM7, XMM0, XMM1, XMM10, XMM11, XMM12, XMM13, XMM14, XMM15, XMM16, XMM17,
    XMM18, XMM19, XMM2, XMM20, XMM21, XMM22, XMM23, XMM24, XMM25, XMM26, XMM27, XMM28, XMM29, XMM3,
    XMM30, XMM31, XMM4, XMM5, XMM6, XMM7, XMM8, XMM9,
};

pub use super::operands::*;
use crate::assembler::binemit::*;

include!("impl.rs");

/// X86/X64 assembler implementation.
///
/// [`x86::Assembler`](Assembler) is a code emitter that emits machine code directly into the [CodeBuffer]. The assembler is capable
/// of targeting both 32-bit and 64-bit instruction sets.
pub struct Assembler<'a> {
    pub raw: RawX86Assembler<'a>,
}

use core::ops::{Deref, DerefMut};

impl<'a> Deref for Assembler<'a> {
    type Target = RawX86Assembler<'a>;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl<'a> DerefMut for Assembler<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

impl<'a> Assembler<'a> {
    pub fn new(buffer: &'a mut CodeBuffer) -> Self {
        Self {
            raw: RawX86Assembler::new(buffer),
        }
    }

    pub fn jmp(&mut self, target: impl Into<BranchTarget>) -> Result<(), AsmError> {
        self.raw.long().jmp(0)?;

        match target.into() {
            BranchTarget::Label(label) => {
                let off = self.offset_for_reloc32();
                self.use_label_at_offset(off, label, LabelUse::X86JmpRel32);
            }

            BranchTarget::Ext(ext) => {
                let offset = self.offset_for_reloc32();
                self.buffer.add_reloc_at_offset(
                    offset,
                    Reloc::X86PCRel4,
                    RelocTarget::ExternalName(ext),
                    -4,
                );
            }
        }

        Ok(())
    }

    pub fn call(&mut self, target: impl Into<BranchTarget>) -> Result<(), AsmError> {
        self.raw.long().call(0)?;
        match target.into() {
            BranchTarget::Label(label) => {
                let offset = self.offset_for_reloc32();
                self.use_label_at_offset(offset, label, LabelUse::X86JmpRel32);
            }

            BranchTarget::Ext(ext) => {
                let offset = self.offset_for_reloc32();
                self.buffer.add_reloc_at_offset(
                    offset,
                    Reloc::X86CallPCRel4,
                    RelocTarget::ExternalName(ext),
                    -4,
                );
            }
        }

        Ok(())
    }

    pub fn mov64ri_with_reloc(
        &mut self,
        dest: Gp,
        imm: i64,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov64ri(dest, imm)?;
        let offset = self.offset_for_reloc64();
        self.buffer
            .add_reloc_at_offset(offset, Reloc::Abs8, RelocTarget::ExternalName(target), 0);
        Ok(())
    }

    pub fn mov32ri_with_reloc(
        &mut self,
        dest: Gp,
        imm: i32,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov32ri(dest, imm)?;
        let offset = self.offset_for_reloc32();
        self.buffer
            .add_reloc_at_offset(offset, Reloc::Abs4, RelocTarget::ExternalName(target), 0);
        Ok(())
    }

    pub fn mov64rm_with_reloc(
        &mut self,
        dest: Gp,
        mem: Mem,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov64rm(dest, mem)?;
        let offset = self.offset_for_reloc32();
        self.buffer
            .add_reloc_at_offset(offset, Reloc::Abs4, RelocTarget::ExternalName(target), 0);
        Ok(())
    }

    pub fn mov64rm_with_relative_reloc(
        &mut self,
        dest: Gp,
        mem: Mem,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov64rm(dest, mem)?;
        let offset = self.offset_for_reloc32();
        self.buffer.add_reloc_at_offset(
            offset,
            Reloc::X86PCRel4,
            RelocTarget::ExternalName(target),
            0,
        );
        Ok(())
    }

    pub fn mov8rm_with_relative_reloc(
        &mut self,
        dest: impl Into<GpLH>,
        mem: Mem,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov8rm(dest.into(), mem)?;
        let offset = self.offset_for_reloc32();
        self.buffer.add_reloc_at_offset(
            offset,
            Reloc::X86PCRel4,
            RelocTarget::ExternalName(target),
            -4,
        );
        Ok(())
    }

    pub fn mov8rm_with_reloc(
        &mut self,
        dest: impl Into<GpLH>,
        mem: Mem,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov8rm(dest.into(), mem)?;
        let offset = self.offset_for_reloc32();
        self.buffer
            .add_reloc_at_offset(offset, Reloc::Abs4, RelocTarget::ExternalName(target), 0);
        Ok(())
    }

    pub fn mov16rm_with_relative_reloc(
        &mut self,
        dest: Gp,
        mem: Mem,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov16rm(dest, mem)?;
        let offset = self.offset_for_reloc32();
        self.buffer.add_reloc_at_offset(
            offset,
            Reloc::X86PCRel4,
            RelocTarget::ExternalName(target),
            -4,
        );
        Ok(())
    }

    pub fn mov16rm_with_reloc(
        &mut self,
        dest: Gp,
        mem: Mem,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov16rm(dest, mem)?;
        let offset = self.offset_for_reloc32();
        self.buffer
            .add_reloc_at_offset(offset, Reloc::Abs4, RelocTarget::ExternalName(target), 0);
        Ok(())
    }

    pub fn mov16mr_with_reloc(
        &mut self,
        mem: Mem,
        src: Gp,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov16mr(mem, src)?;
        let offset = self.offset_for_reloc32();
        self.buffer
            .add_reloc_at_offset(offset, Reloc::Abs4, RelocTarget::ExternalName(target), 0);
        Ok(())
    }

    pub fn mov16mr_with_relative_reloc(
        &mut self,
        mem: Mem,
        src: Gp,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov16mr(mem, src)?;
        let offset = self.offset_for_reloc32();
        self.buffer.add_reloc_at_offset(
            offset,
            Reloc::X86PCRel4,
            RelocTarget::ExternalName(target),
            -4,
        );
        Ok(())
    }

    pub fn mov32mr_with_reloc(
        &mut self,
        mem: Mem,
        src: Gp,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov32mr(mem, src)?;
        let offset = self.offset_for_reloc32();
        self.buffer
            .add_reloc_at_offset(offset, Reloc::Abs4, RelocTarget::ExternalName(target), 0);
        Ok(())
    }

    pub fn mov32mr_with_relative_reloc(
        &mut self,
        mem: Mem,
        src: Gp,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov32mr(mem, src)?;
        let offset = self.offset_for_reloc32();
        self.buffer.add_reloc_at_offset(
            offset,
            Reloc::X86PCRel4,
            RelocTarget::ExternalName(target),
            -4,
        );
        Ok(())
    }

    pub fn mov64mr_with_reloc(
        &mut self,
        mem: Mem,
        src: Gp,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov64mr(mem, src)?;
        let offset = self.offset_for_reloc32();
        self.buffer
            .add_reloc_at_offset(offset, Reloc::Abs4, RelocTarget::ExternalName(target), 0);
        Ok(())
    }

    pub fn mov64mr_with_relative_reloc(
        &mut self,
        mem: Mem,
        src: Gp,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().mov64mr(mem, src)?;
        let offset = self.offset_for_reloc32();
        self.buffer.add_reloc_at_offset(
            offset,
            Reloc::X86PCRel4,
            RelocTarget::ExternalName(target),
            -4,
        );
        Ok(())
    }

    pub fn add64ri_with_reloc(
        &mut self,
        dest: Gp,
        imm: i32,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().add64ri(dest, imm)?;
        let offset = self.offset_for_reloc32();
        self.buffer
            .add_reloc_at_offset(offset, Reloc::Abs4, RelocTarget::ExternalName(target), 0);
        Ok(())
    }

    pub fn add64ri_with_relative_reloc(
        &mut self,
        dest: Gp,
        imm: i32,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().add64ri(dest, imm)?;
        let offset = self.offset_for_reloc32();
        self.buffer.add_reloc_at_offset(
            offset,
            Reloc::X86PCRel4,
            RelocTarget::ExternalName(target),
            -4,
        );
        Ok(())
    }

    pub fn add64rm_with_reloc(
        &mut self,
        dest: Gp,
        mem: Mem,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().add64rm(dest, mem)?;
        let offset = self.offset_for_reloc32();
        self.buffer
            .add_reloc_at_offset(offset, Reloc::Abs4, RelocTarget::ExternalName(target), 0);
        Ok(())
    }

    pub fn add64rm_with_relative_reloc(
        &mut self,
        dest: Gp,
        mem: Mem,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().add64rm(dest, mem)?;
        let offset = self.offset_for_reloc32();
        self.buffer.add_reloc_at_offset(
            offset,
            Reloc::X86PCRel4,
            RelocTarget::ExternalName(target),
            -4,
        );
        Ok(())
    }

    pub fn sub32rm_with_reloc(
        &mut self,
        dest: Gp,
        mem: Mem,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().sub32rm(dest, mem)?;
        let offset = self.offset_for_reloc32();
        self.buffer
            .add_reloc_at_offset(offset, Reloc::Abs4, RelocTarget::ExternalName(target), 0);
        Ok(())
    }

    pub fn sub32rm_with_relative_reloc(
        &mut self,
        dest: Gp,
        mem: Mem,
        target: ExternalName,
    ) -> Result<(), AsmError> {
        self.raw.long().sub32rm(dest, mem)?;
        let offset = self.offset_for_reloc32();
        self.buffer.add_reloc_at_offset(
            offset,
            Reloc::X86PCRel4,
            RelocTarget::ExternalName(target),
            -4,
        );
        Ok(())
    }
}
