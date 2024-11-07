use crate::x86::arch_traits::{X64_ARCH_TRAITS, X86_ARCH_TRAITS};

use super::operand::{OperandSignature, RegGroup, RegType};
use super::types::TypeId;

pub struct ArchTraits {
    pub sp_reg_id: u8,
    pub fp_reg_id: u8,
    pub link_reg_id: u8,
    pub ip_reg_id: u8,
    pub hw_stack_alignment: u8,
    pub min_stack_offset: u32,
    pub max_stack_offset: u32,
    pub regs_signature: [OperandSignature; RegType::MaxValue as usize + 1],
    pub reg_type_to_type_id: [TypeId; RegType::MaxValue as usize + 1],
    pub type_id_to_reg_type: [RegType; 32],
}

pub const ARCH_X86: usize = if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
    size_of::<usize>() * 8
} else {
    0
};

pub const ARCH_ARM: usize = if cfg!(any(target_arch = "aarch64", target_arch = "arm")) {
    size_of::<usize>() * 8
} else {
    0
};
pub const ARCH_RISCV: usize = if cfg!(any(target_arch = "riscv64", target_arch = "riscv32")) {
    size_of::<usize>() * 8
} else {
    0
};

/// Instruction set architecture (ISA).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Arch {
    /// Unknown or uninitialized ISA.
    Unknown = 0,

    /// 32-bit X86 ISA.
    X86 = 1,
    /// 64-bit X86 ISA alsonown as X64, X86_64, and AMD64.
    X64 = 2,

    /// 32-bit RISC-V ISA.
    RISCV32 = 3,
    /// 64-bit RISC-V ISA.
    RISCV64 = 4,

    /// 32-bit ARM ISA (little endian).
    ARM = 5,
    /// 64-bit ARM ISA in (little endian).
    AArch64 = 6,
    /// 32-bit ARM ISA in Thumb mode (little endian).
    Thumb = 7,

    // 8 is not used at the moment, even numbers are 64-bit architectures.
    /// 32-bit MIPS ISA in (little endian).
    MIPS32LE = 9,
    /// 64-bit MIPS ISA in (little endian).
    MIPS64LE = 10,

    /// 32-bit ARM ISA (big endian).
    ARMBE = 11,
    /// 64-bit ARM ISA in (big endian).
    AArch64BE = 12,
    /// 32-bit ARM ISA in Thumb mode (big endian).
    ThumbBE = 13,

    // 14 is not used at the moment, even numbers are 64-bit architectures.
    /// 32-bit MIPS ISA in (big endian).
    MIPS32BE = 15,
    /// 64-bit MIPS ISA in (big endian).
    MIPS64BE = 16,

    Max,
}

impl Arch {
    pub const HOST: Arch = {
        if ARCH_X86 == 32 {
            Arch::X86
        } else if ARCH_X86 == 64 {
            Arch::X64
        } else if ARCH_RISCV == 32 {
            Arch::RISCV32
        } else if ARCH_RISCV == 64 {
            Arch::RISCV64
        } else if ARCH_ARM == 32 && cfg!(target_endian = "little") {
            Arch::ARM
        } else if ARCH_ARM == 32 && cfg!(target_endian = "big") {
            Arch::ARMBE
        } else if ARCH_ARM == 64 && cfg!(target_endian = "little") {
            Arch::AArch64
        } else if ARCH_ARM == 64 && cfg!(target_endian = "big") {
            Arch::AArch64BE
        } else {
            Arch::Unknown
        }
    };
}

impl Default for Arch {
    fn default() -> Self {
        Self::HOST
    }
}

const NO_ARCH_TRAITS: ArchTraits = ArchTraits {
    fp_reg_id: 0xff,
    sp_reg_id: 0xff,
    link_reg_id: 0xff,
    ip_reg_id: 0xff,
    hw_stack_alignment: 0,
    min_stack_offset: 0,
    max_stack_offset: 0,
    regs_signature: [OperandSignature::new(0); 32],
    reg_type_to_type_id: [TypeId::Void; 32],
    type_id_to_reg_type: [RegType::None; 32],
};

#[rustfmt::skip]
const ARCH_TRAITS: [ArchTraits; Arch::Max as usize] =[
    NO_ARCH_TRAITS,
    X86_ARCH_TRAITS,
    X64_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
    NO_ARCH_TRAITS,
];

impl ArchTraits {
    /// Returns stack pointer register id.
    pub const fn sp_reg_id(&self) -> u32 {
        self.sp_reg_id as _
    }

    /// Returns stack frame register id.
    pub const fn fp_reg_id(&self) -> u32 {
        self.fp_reg_id as _
    }

    /// Returns link register id, if the architecture provides it.
    pub const fn link_reg_id(&self) -> u32 {
        self.link_reg_id as _
    }

    /// Returns instruction pointer register id, if the architecture provides it.
    pub const fn ip_reg_id(&self) -> u32 {
        self.ip_reg_id as _
    }
    /// Returns a hardware stack alignment requirement.
    ///
    /// ## Note
    /// This is a hardware constraint. Architectures that don't constrain it would return the lowest alignment
    /// (1), however, some architectures may constrain the alignment, for example AArch64 requires 16-byte alignment.
    pub const fn hw_stack_alignment(&self) -> u32 {
        self.hw_stack_alignment as _
    }

    /// Tests whether the architecture provides link register, which is used across function calls. If the link
    /// register is not provided then a function call pushes the return address on stack (X86/X64).
    pub const fn has_link_reg(&self) -> bool {
        self.link_reg_id != 0xff
    }

    /// Returns minimum addressable offset on stack guaranteed for all instructions.
    pub const fn min_stack_offset(&self) -> u32 {
        self.min_stack_offset
    }

    /// Returns maximum addressable offset on stack depending on specific instruction.
    pub const fn max_stack_offset(&self) -> u32 {
        self.max_stack_offset
    }

    pub const fn has_reg_type(&self, typ: RegType) -> bool {
        (typ as u32) <= RegType::MaxValue as u32 && self.regs_signature[typ as usize].is_valid()
    }

    pub const fn reg_type_to_signature(&self, typ: RegType) -> OperandSignature {
        self.regs_signature[typ as usize]
    }

    pub fn reg_type_to_group(&self, typ: RegType) -> RegGroup {
        self.reg_type_to_signature(typ).reg_group()
    }

    pub fn reg_type_to_size(&self, typ: RegType) -> u32 {
        self.reg_type_to_signature(typ).size()
    }

    pub const fn reg_type_to_type_id(&self, typ: RegType) -> TypeId {
        self.reg_type_to_type_id[typ as usize]
    }

    pub const fn by_arch(arch: Arch) -> &'static Self {
        &ARCH_TRAITS[arch as usize]
    }
}
