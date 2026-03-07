use super::arch_traits::Arch;

/// X86 ISA extensions that can affect code generation.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86Feature {
    /// Advanced vector extensions.
    AVX = 0,
    /// Advanced vector extensions 2.
    AVX2,
    /// AVX-512 foundation.
    AVX512F,
    /// AVX-512 byte and word instructions.
    AVX512BW,
    /// AVX-512 conflict detection instructions.
    AVX512CD,
    /// AVX-512 doubleword and quadword instructions.
    AVX512DQ,
    /// AVX-512 vector length extensions.
    AVX512VL,
}

impl X86Feature {
    pub const fn mask(self) -> u64 {
        1u64 << self as u8
    }
}

const DEFAULT_X86_FEATURES: u64 = X86Feature::AVX.mask()
    | X86Feature::AVX2.mask()
    | X86Feature::AVX512F.mask()
    | X86Feature::AVX512BW.mask()
    | X86Feature::AVX512CD.mask()
    | X86Feature::AVX512DQ.mask()
    | X86Feature::AVX512VL.mask();

const fn default_x86_features(arch: Arch) -> u64 {
    match arch {
        Arch::X86 | Arch::X64 => DEFAULT_X86_FEATURES,
        _ => 0,
    }
}

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Environment {
    arch: Arch,
    is_pic: bool,
    x86_features: u64,
}

impl Default for Environment {
    fn default() -> Self {
        Self::host()
    }
}

impl Environment {
    pub const fn new(arch: Arch) -> Self {
        Self {
            arch,
            is_pic: false,
            x86_features: default_x86_features(arch),
        }
    }

    pub const fn host() -> Self {
        Self {
            arch: Arch::HOST,
            is_pic: false,
            x86_features: default_x86_features(Arch::HOST),
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

    /// Enable or disable an x86 ISA extension.
    pub fn set_x86_feature(&mut self, feature: X86Feature, value: bool) {
        if value {
            self.x86_features |= feature.mask();
        } else {
            self.x86_features &= !feature.mask();
        }
    }

    pub const fn x86_feature(&self, feature: X86Feature) -> bool {
        (self.x86_features & feature.mask()) != 0
    }

    pub fn set_avx(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX, value);
    }

    pub const fn avx(&self) -> bool {
        self.x86_feature(X86Feature::AVX)
    }

    pub fn set_avx2(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX2, value);
    }

    pub const fn avx2(&self) -> bool {
        self.x86_feature(X86Feature::AVX2)
    }

    pub fn set_avx512f(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX512F, value);
    }

    pub const fn avx512f(&self) -> bool {
        self.x86_feature(X86Feature::AVX512F)
    }

    pub fn set_avx512bw(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX512BW, value);
    }

    pub const fn avx512bw(&self) -> bool {
        self.x86_feature(X86Feature::AVX512BW)
    }

    pub fn set_avx512cd(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX512CD, value);
    }

    pub const fn avx512cd(&self) -> bool {
        self.x86_feature(X86Feature::AVX512CD)
    }

    pub fn set_avx512dq(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX512DQ, value);
    }

    pub const fn avx512dq(&self) -> bool {
        self.x86_feature(X86Feature::AVX512DQ)
    }

    pub fn set_avx512vl(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX512VL, value);
    }

    pub const fn avx512vl(&self) -> bool {
        self.x86_feature(X86Feature::AVX512VL)
    }

    pub const fn arch(&self) -> Arch {
        self.arch
    }
}

#[cfg(test)]
mod tests {
    use super::{Environment, X86Feature};
    use crate::core::arch_traits::Arch;

    #[test]
    fn x86_targets_enable_vector_features_by_default() {
        let env = Environment::new(Arch::X64);

        assert!(env.avx());
        assert!(env.avx2());
        assert!(env.avx512f());
        assert!(env.avx512bw());
        assert!(env.avx512cd());
        assert!(env.avx512dq());
        assert!(env.avx512vl());
    }

    #[test]
    fn non_x86_targets_start_without_x86_features() {
        let env = Environment::new(Arch::AArch64);

        assert!(!env.x86_feature(X86Feature::AVX));
        assert!(!env.x86_feature(X86Feature::AVX512F));
    }

    #[test]
    fn x86_features_can_be_toggled() {
        let mut env = Environment::new(Arch::X64);

        env.set_avx(false);
        env.set_x86_feature(X86Feature::AVX512VL, false);

        assert!(!env.avx());
        assert!(!env.avx512vl());
    }
}
