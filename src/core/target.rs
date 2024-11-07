use super::arch_traits::Arch;

/// Object format.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectFormat {
    /// Unknown or uninitialized object format.
    Unknown = 0,
    /// JIT code generation object.
    JIT,
    /// Executable and linkable format (ELF).
    ELF,
    /// Common object file format.
    COFF,
    /// Extended COFF object format.
    XCOFF,
    /// Mach object file format.
    MachO,
    /// Maximum value of `ObjectFormat`.
    MaxValue,
}

/// Platform ABI (application binary interface).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformABI {
    /// Unknown or uninitialized environment.
    Unknown = 0,
    /// Microsoft ABI.
    MSVC,
    /// GNU ABI.
    GNU,
    /// Android Environment / ABI.
    Android,
    /// Cygwin ABI.
    Cygwin,
    /// Darwin ABI.
    Darwin,
    /// Maximum value of `PlatformABI`.
    MaxValue,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Environment {
    arch: Arch,
    is_pic: bool,
}

impl Environment {
    pub const fn new(arch: Arch) -> Self {
        Self {
            arch,
            is_pic: false,
        }
    }

    pub const fn host() -> Self {
        Self {
            arch: Arch::HOST,
            is_pic: false,
        }
    }

    /// Enable or disable PIC (Position independent code).
    ///
    /// If enabled code for calls and jumps to symbols will emit near relocations
    pub fn set_pic(&mut self, value: bool) {
        self.is_pic = value;
    }

    pub fn pic(&self) -> bool {
        self.is_pic
    }
}
