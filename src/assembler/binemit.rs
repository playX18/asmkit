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
}

impl LabelUse {
    pub const fn patch_size(&self) -> usize {
        match self {
            Self::X86JmpRel32 => 4,
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
