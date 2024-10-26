use crate::{assembler::riscv::generate_imm, encdec::riscv::{auipc, c_j, jalr, ZERO}};

use super::codeholder::ExternalName;

/// Offset in bytes from the beginning of the function.
///
/// Cranelift can be used as a cross compiler, so we don't want to use a type like `usize` which
/// depends on the *host* platform, not the *target* platform.
pub type CodeOffset = u32;

/// Addend to add to the symbol value.
pub type Addend = i64;

/// Relocation kinds for every ISA
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Reloc {
    /// absolute 4-byte
    Abs4,
    /// absolute 8-byte
    Abs8,
    /// x86 PC-relative 4-byte
    X86PCRel4,
    /// x86 call to PC-relative 4-byte
    X86CallPCRel4,
    /// x86 call to PLT-relative 4-byte
    X86CallPLTRel4,
    /// x86 GOT PC-relative 4-byte
    X86GOTPCRel4,
    /// The 32-bit offset of the target from the beginning of its section.
    /// Equivalent to `IMAGE_REL_AMD64_SECREL`.
    /// See: [PE Format](https://docs.microsoft.com/en-us/windows/win32/debug/pe-format)
    X86SecRel,
    /// Arm32 call target
    Arm32Call,
    /// Arm64 call target. Encoded as bottom 26 bits of instruction. This
    /// value is sign-extended, multiplied by 4, and added to the PC of
    /// the call instruction to form the destination address.
    Arm64Call,

    /// Elf x86_64 32 bit signed PC relative offset to two GOT entries for GD symbol.
    ElfX86_64TlsGd,

    /// Mach-O x86_64 32 bit signed PC relative offset to a `__thread_vars` entry.
    MachOX86_64Tlv,

    /// Mach-O Aarch64 TLS
    /// PC-relative distance to the page of the TLVP slot.
    MachOAarch64TlsAdrPage21,

    /// Mach-O Aarch64 TLS
    /// Offset within page of TLVP slot.
    MachOAarch64TlsAdrPageOff12,

    /// Aarch64 TLSDESC Adr Page21
    /// This is equivalent to `R_AARCH64_TLSDESC_ADR_PAGE21` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescAdrPage21,

    /// Aarch64 TLSDESC Ld64 Lo12
    /// This is equivalent to `R_AARCH64_TLSDESC_LD64_LO12` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescLd64Lo12,

    /// Aarch64 TLSDESC Add Lo12
    /// This is equivalent to `R_AARCH64_TLSGD_ADD_LO12` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescAddLo12,

    /// Aarch64 TLSDESC Call
    /// This is equivalent to `R_AARCH64_TLSDESC_CALL` in the [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#57105thread-local-storage-descriptors)
    Aarch64TlsDescCall,

    /// AArch64 GOT Page
    /// Set the immediate value of an ADRP to bits 32:12 of X; check that –2^32 <= X < 2^32
    /// This is equivalent to `R_AARCH64_ADR_GOT_PAGE` (311) in the  [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#static-aarch64-relocations)
    Aarch64AdrGotPage21,

    /// AArch64 GOT Low bits

    /// Set the LD/ST immediate field to bits 11:3 of X. No overflow check; check that X&7 = 0
    /// This is equivalent to `R_AARCH64_LD64_GOT_LO12_NC` (312) in the  [aaelf64](https://github.com/ARM-software/abi-aa/blob/2bcab1e3b22d55170c563c3c7940134089176746/aaelf64/aaelf64.rst#static-aarch64-relocations)
    Aarch64Ld64GotLo12Nc,

    /// RISC-V Call PLT: 32-bit PC-relative function call, macros call, tail (PIC)
    ///
    /// Despite having PLT in the name, this relocation is also used for normal calls.
    /// The non-PLT version of this relocation has been deprecated.
    ///
    /// This is the `R_RISCV_CALL_PLT` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#procedure-calls>
    RiscvCallPlt,

    /// RISC-V TLS GD: High 20 bits of 32-bit PC-relative TLS GD GOT reference,
    ///
    /// This is the `R_RISCV_TLS_GD_HI20` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#global-dynamic>
    RiscvTlsGdHi20,

    /// Low 12 bits of a 32-bit PC-relative relocation (I-Type instruction)
    ///
    /// This is the `R_RISCV_PCREL_LO12_I` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#pc-relative-symbol-addresses>
    RiscvPCRelLo12I,

    /// High 20 bits of a 32-bit PC-relative GOT offset relocation
    ///
    /// This is the `R_RISCV_GOT_HI20` relocation from the RISC-V ELF psABI document.
    /// <https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#pc-relative-symbol-addresses>
    RiscvGotHi20,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LabelUse {
    X86JmpRel32,
    /// 20-bit branch offset (unconditional branches). PC-rel, offset is
    /// imm << 1. Immediate is 20 signed bits. Use in Jal instructions.
    RVJal20,
    /// The unconditional jump instructions all use PC-relative
    /// addressing to help support position independent code. The JALR
    /// instruction was defined to enable a two-instruction sequence to
    /// jump anywhere in a 32-bit absolute address range. A LUI
    /// instruction can first load rs1 with the upper 20 bits of a
    /// target address, then JALR can add in the lower bits. Similarly,
    /// AUIPC then JALR can jump anywhere in a 32-bit pc-relative
    /// address range.
    RVPCRel32,

    /// All branch instructions use the B-type instruction format. The
    /// 12-bit B-immediate encodes signed offsets in multiples of 2, and
    /// is added to the current pc to give the target address. The
    /// conditional branch range is ±4 KiB.
    RVB12,

    /// Equivalent to the `R_RISCV_PCREL_HI20` relocation, Allows setting
    /// the immediate field of an `auipc` instruction.
    RVPCRelHi20,

    /// Similar to the `R_RISCV_PCREL_LO12_I` relocation but pointing to
    /// the final address, instead of the `PCREL_HI20` label. Allows setting
    /// the immediate field of I Type instructions such as `addi` or `lw`.
    ///
    /// Since we currently don't support offsets in labels, this relocation has
    /// an implicit offset of 4.
    RVPCRelLo12I,

    /// 11-bit PC-relative jump offset. Equivalent to the `RVC_JUMP` relocation
    RVCJump,
}

impl LabelUse {
    pub const fn patch_size(&self) -> usize {
        match self {
            Self::X86JmpRel32 => 4,
            Self::RVCJump => 2,
            Self::RVJal20 | Self::RVB12 | Self::RVPCRelHi20 | Self::RVPCRelLo12I => 4,
            Self::RVPCRel32 => 8,
        }
    }

    pub fn patch(&self, buffer: &mut [u8], use_offset: CodeOffset, label_offset: CodeOffset) {
        let pc_rel = (label_offset as i64) - (use_offset as i64);

        let pc_rel = pc_rel as u32;

        match self {
            Self::X86JmpRel32 => {
                let addend = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let value = pc_rel.wrapping_add(addend).wrapping_sub(4);

                buffer.copy_from_slice(&value.to_le_bytes());
            }

            Self::RVJal20 => {
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let offset = pc_rel as u32;
                let v = ((offset >> 12 & 0b1111_1111) << 12)
                    | ((offset >> 11 & 0b1) << 20)
                    | ((offset >> 1 & 0b11_1111_1111) << 21)
                    | ((offset >> 20 & 0b1) << 31);
                buffer[0..4].clone_from_slice(&u32::to_le_bytes(insn | v));
            }

            Self::RVPCRel32 => {
                let (imm20, imm12) = generate_imm(pc_rel as u64);
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let insn2 = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
                buffer[0..4].copy_from_slice(&(insn | auipc(ZERO, 0) | imm20).to_le_bytes());
                buffer[4..8].copy_from_slice(&(insn2 | jalr(ZERO, ZERO, 0) | imm12).to_le_bytes());
            }

            Self::RVB12 => {
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let offset = pc_rel as u32;
                let v = ((offset >> 11 & 0b1) << 7)
                    | ((offset >> 1 & 0b1111) << 8)
                    | ((offset >> 5 & 0b11_1111) << 25)
                    | ((offset >> 12 & 0b1) << 31);
                buffer[0..4].clone_from_slice(&u32::to_le_bytes(insn | v));
            }

            Self::RVPCRelHi20 => {
                // See https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#pc-relative-symbol-addresses
                //
                // We need to add 0x800 to ensure that we land at the next page as soon as it goes out of range for the
                // Lo12 relocation. That relocation is signed and has a maximum range of -2048..2047. So when we get an
                // offset of 2048, we need to land at the next page and subtract instead.
                let offset = pc_rel;
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let hi20 = offset.wrapping_add(0x800) >> 12;
                let insn = (insn & 0xffff) | (hi20 << 12);
                buffer[0..4].copy_from_slice(&insn.to_le_bytes());
            }

            Self::RVPCRelLo12I => {
                // `offset` is the offset from the current instruction to the target address.
                //
                // However we are trying to compute the offset to the target address from the previous instruction.
                // The previous instruction should be the one that contains the PCRelHi20 relocation and
                // stores/references the program counter (`auipc` usually).
                //
                // Since we are trying to compute the offset from the previous instruction, we can
                // represent it as offset = target_address - (current_instruction_address - 4)
                // which is equivalent to offset = target_address - current_instruction_address + 4.
                //
                let insn = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let lo12 = (pc_rel + 4) as u32 & 0xfff;
                let insn = (insn & 0xffff) | (lo12 << 20);
                buffer[0..4].copy_from_slice(&insn.to_le_bytes());
            }

            Self::RVCJump => {
                debug_assert!(pc_rel & 1 == 1);
                buffer[0..2].clone_from_slice(&c_j(pc_rel as _).to_le_bytes());
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(transparent)]
pub struct Label(pub u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BranchTarget {
    Label(Label),
    Ext(ExternalName),
}

impl From<Label> for BranchTarget {
    fn from(l: Label) -> Self {
        Self::Label(l)
    }
}

impl From<ExternalName> for BranchTarget {
    fn from(e: ExternalName) -> Self {
        Self::Ext(e)
    }
}
