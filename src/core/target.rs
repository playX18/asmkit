pub use super::arch_traits::Arch;

#[cfg(feature = "aarch64")]
use crate::aarch64::CPU_FEATURE_COUNT as AARCH64_FEATURE_COUNT;
#[cfg(feature = "aarch64")]
pub use crate::aarch64::CpuFeature as AArch64Feature;
#[cfg(feature = "riscv")]
pub use crate::riscv::instdb::CpuFeature as RiscVFeature;
#[cfg(feature = "riscv")]
use crate::riscv::instdb::{
    BASELINE_CPU_FEATURES as BASELINE_RISCV_FEATURES, CPU_FEATURE_WORDS as RISCV_FEATURE_WORDS,
};
#[cfg(feature = "x86")]
pub use crate::x86::instdb::CpuFeature as X86Feature;
#[cfg(feature = "x86")]
use crate::x86::instdb::{CPU_FEATURE_COUNT, DEFAULT_X86_FEATURES};

#[cfg(feature = "x86")]
const X86_FEATURE_WORDS: usize = CPU_FEATURE_COUNT.div_ceil(64);

#[cfg(feature = "x86")]
const fn default_x86_features(arch: Arch) -> [u64; X86_FEATURE_WORDS] {
    let mut features = [0; X86_FEATURE_WORDS];
    if matches!(arch, Arch::X86 | Arch::X64) {
        let mut index = 0;
        while index < DEFAULT_X86_FEATURES.len() {
            let feature = DEFAULT_X86_FEATURES[index] as usize;
            features[feature / 64] |= 1u64 << (feature % 64);
            index += 1;
        }
    }
    features
}

#[cfg(feature = "aarch64")]
const fn default_aarch64_features(arch: Arch) -> u64 {
    if matches!(arch, Arch::AArch64) {
        if AARCH64_FEATURE_COUNT == 64 {
            u64::MAX
        } else {
            (1u64 << AARCH64_FEATURE_COUNT) - 1
        }
    } else {
        0
    }
}

#[cfg(feature = "riscv")]
const fn default_riscv_features(arch: Arch) -> [u64; RISCV_FEATURE_WORDS] {
    if matches!(arch, Arch::RISCV32 | Arch::RISCV64) {
        [u64::MAX; RISCV_FEATURE_WORDS]
    } else {
        [0; RISCV_FEATURE_WORDS]
    }
}

#[cfg(feature = "riscv")]
const fn baseline_riscv_features(arch: Arch) -> [u64; RISCV_FEATURE_WORDS] {
    let mut features = [0; RISCV_FEATURE_WORDS];
    if matches!(arch, Arch::RISCV32 | Arch::RISCV64) {
        let mut index = 0;
        while index < BASELINE_RISCV_FEATURES.len() {
            let feature = BASELINE_RISCV_FEATURES[index] as usize;
            features[feature / 64] |= 1u64 << (feature % 64);
            index += 1;
        }
    }
    features
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Environment {
    arch: Arch,
    is_pic: bool,
    #[cfg(feature = "x86")]
    x86_features: [u64; X86_FEATURE_WORDS],
    #[cfg(feature = "aarch64")]
    aarch64_features: u64,
    #[cfg(feature = "riscv")]
    riscv_features: [u64; RISCV_FEATURE_WORDS],
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
            #[cfg(feature = "x86")]
            x86_features: default_x86_features(arch),
            #[cfg(feature = "aarch64")]
            aarch64_features: default_aarch64_features(arch),
            #[cfg(feature = "riscv")]
            riscv_features: default_riscv_features(arch),
        }
    }

    /// Creates a target with no optional ISA extensions enabled.
    pub const fn baseline(arch: Arch) -> Self {
        Self {
            arch,
            is_pic: false,
            #[cfg(feature = "x86")]
            x86_features: [0; X86_FEATURE_WORDS],
            #[cfg(feature = "aarch64")]
            aarch64_features: 0,
            #[cfg(feature = "riscv")]
            riscv_features: baseline_riscv_features(arch),
        }
    }

    pub const fn host() -> Self {
        Self {
            arch: Arch::HOST,
            is_pic: false,
            #[cfg(feature = "x86")]
            x86_features: default_x86_features(Arch::HOST),
            #[cfg(feature = "aarch64")]
            aarch64_features: default_aarch64_features(Arch::HOST),
            #[cfg(feature = "riscv")]
            riscv_features: default_riscv_features(Arch::HOST),
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
    #[cfg(feature = "x86")]
    pub fn set_x86_feature(&mut self, feature: X86Feature, value: bool) {
        let feature = feature as usize;
        let mask = 1u64 << (feature % 64);
        if value {
            self.x86_features[feature / 64] |= mask;
        } else {
            self.x86_features[feature / 64] &= !mask;
        }
    }

    #[cfg(feature = "x86")]
    pub const fn x86_feature(&self, feature: X86Feature) -> bool {
        self.x86_feature_id(feature as u8)
    }

    #[cfg(feature = "x86")]
    pub(crate) const fn x86_feature_id(&self, feature: u8) -> bool {
        let feature = feature as usize;
        feature < CPU_FEATURE_COUNT
            && (self.x86_features[feature / 64] & (1u64 << (feature % 64))) != 0
    }

    /// Enables or disables an AArch64 architectural feature.
    #[cfg(feature = "aarch64")]
    pub fn set_aarch64_feature(&mut self, feature: AArch64Feature, value: bool) {
        let mask = 1u64 << feature as usize;
        if value {
            self.aarch64_features |= mask;
        } else {
            self.aarch64_features &= !mask;
        }
    }

    #[cfg(feature = "aarch64")]
    pub const fn aarch64_feature(&self, feature: AArch64Feature) -> bool {
        self.aarch64_features & (1u64 << feature as usize) != 0
    }

    #[cfg(feature = "aarch64")]
    pub(crate) const fn supports_aarch64_features(&self, required: u64) -> bool {
        self.aarch64_features & required == required
    }

    /// Enables or disables a RISC-V extension from the pinned opcode metadata.
    #[cfg(feature = "riscv")]
    pub fn set_riscv_feature(&mut self, feature: RiscVFeature, value: bool) {
        let feature = feature as usize;
        let mask = 1u64 << (feature % 64);
        if value {
            self.riscv_features[feature / 64] |= mask;
        } else {
            self.riscv_features[feature / 64] &= !mask;
        }
    }

    #[cfg(feature = "riscv")]
    pub const fn riscv_feature(&self, feature: RiscVFeature) -> bool {
        let feature = feature as usize;
        self.riscv_features[feature / 64] & (1u64 << (feature % 64)) != 0
    }

    #[cfg(feature = "riscv")]
    pub(crate) const fn supports_any_riscv_feature(
        &self,
        required: &[u64; RISCV_FEATURE_WORDS],
    ) -> bool {
        let mut index = 0;
        while index < RISCV_FEATURE_WORDS {
            if self.riscv_features[index] & required[index] != 0 {
                return true;
            }
            index += 1;
        }
        false
    }

    #[cfg(feature = "x86")]
    pub fn set_avx(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX, value);
    }

    #[cfg(feature = "x86")]
    pub const fn avx(&self) -> bool {
        self.x86_feature(X86Feature::AVX)
    }

    #[cfg(feature = "x86")]
    pub fn set_avx2(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX2, value);
    }

    #[cfg(feature = "x86")]
    pub const fn avx2(&self) -> bool {
        self.x86_feature(X86Feature::AVX2)
    }

    #[cfg(feature = "x86")]
    pub fn set_avx512f(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX512_F, value);
    }

    #[cfg(feature = "x86")]
    pub const fn avx512f(&self) -> bool {
        self.x86_feature(X86Feature::AVX512_F)
    }

    #[cfg(feature = "x86")]
    pub fn set_avx512bw(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX512_BW, value);
    }

    #[cfg(feature = "x86")]
    pub const fn avx512bw(&self) -> bool {
        self.x86_feature(X86Feature::AVX512_BW)
    }

    #[cfg(feature = "x86")]
    pub fn set_avx512cd(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX512_CD, value);
    }

    #[cfg(feature = "x86")]
    pub const fn avx512cd(&self) -> bool {
        self.x86_feature(X86Feature::AVX512_CD)
    }

    #[cfg(feature = "x86")]
    pub fn set_avx512dq(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX512_DQ, value);
    }

    #[cfg(feature = "x86")]
    pub const fn avx512dq(&self) -> bool {
        self.x86_feature(X86Feature::AVX512_DQ)
    }

    #[cfg(feature = "x86")]
    pub fn set_avx512vl(&mut self, value: bool) {
        self.set_x86_feature(X86Feature::AVX512_VL, value);
    }

    #[cfg(feature = "x86")]
    pub const fn avx512vl(&self) -> bool {
        self.x86_feature(X86Feature::AVX512_VL)
    }

    pub const fn arch(&self) -> Arch {
        self.arch
    }

    /// Tests whether the environment targets a 32-bit architecture.
    pub const fn is_32bit(&self) -> bool {
        self.arch.is_32bit()
    }

    /// Tests whether the environment targets a 64-bit architecture.
    pub const fn is_64bit(&self) -> bool {
        self.arch.is_64bit()
    }
}

#[cfg(test)]
mod tests {
    use super::Environment;
    #[cfg(feature = "x86")]
    use super::X86Feature;
    #[cfg(feature = "aarch64")]
    use crate::aarch64::{ALL_CPU_FEATURES as ALL_AARCH64_FEATURES, CPU_FEATURE_REPRESENTATIVE};
    use crate::core::arch_traits::Arch;

    #[cfg(feature = "x86")]
    #[test]
    fn x86_targets_enable_vector_features_by_default() {
        let env = Environment::new(Arch::X64);

        for feature in crate::x86::instdb::DEFAULT_X86_FEATURES {
            assert!(env.x86_feature(*feature), "{feature:?}");
        }
    }

    #[cfg(feature = "x86")]
    #[test]
    fn non_x86_targets_start_without_x86_features() {
        let env = Environment::new(Arch::AArch64);

        assert!(!env.x86_feature(X86Feature::AVX));
        assert!(!env.x86_feature(X86Feature::AVX512_F));
    }

    #[cfg(feature = "x86")]
    #[test]
    fn x86_features_can_be_toggled() {
        let mut env = Environment::new(Arch::X64);

        env.set_avx(false);
        env.set_x86_feature(X86Feature::AVX512_VL, false);

        assert!(!env.avx());
        assert!(!env.avx512vl());
    }

    #[cfg(feature = "x86")]
    #[test]
    fn baseline_has_no_optional_x86_features() {
        let env = Environment::baseline(Arch::X64);

        for feature in crate::x86::instdb::DEFAULT_X86_FEATURES {
            assert!(!env.x86_feature(*feature), "{feature:?}");
        }
    }

    #[cfg(feature = "aarch64")]
    #[test]
    fn every_aarch64_feature_is_enabled_by_default_and_removable() {
        for (feature, instruction) in ALL_AARCH64_FEATURES.iter().zip(CPU_FEATURE_REPRESENTATIVE) {
            let required = crate::aarch64::INST_FEATURE_MASKS[instruction as usize];
            let mut environment = Environment::new(Arch::AArch64);
            assert!(environment.supports_aarch64_features(required));
            environment.set_aarch64_feature(*feature, false);
            assert!(
                !environment.supports_aarch64_features(required),
                "{feature:?}"
            );
        }
    }

    #[cfg(feature = "riscv")]
    #[test]
    fn every_riscv_feature_is_addable_and_removable() {
        use crate::riscv::instdb::{
            ALL_CPU_FEATURES, BASELINE_CPU_FEATURES, CPU_FEATURE_REPRESENTATIVE,
            OPCODE_FEATURE_MASKS,
        };

        let mut environment = Environment::baseline(Arch::RISCV64);
        for feature in BASELINE_CPU_FEATURES {
            environment.set_riscv_feature(*feature, false);
        }
        for (feature, opcode) in ALL_CPU_FEATURES.iter().zip(CPU_FEATURE_REPRESENTATIVE) {
            let required = &OPCODE_FEATURE_MASKS[opcode as usize];
            environment.set_riscv_feature(*feature, true);
            assert!(
                environment.supports_any_riscv_feature(required),
                "{feature:?}"
            );
            environment.set_riscv_feature(*feature, false);
            assert!(
                !environment.supports_any_riscv_feature(required),
                "{feature:?}"
            );
        }
    }

    #[test]
    fn arch_determines_mode() {
        assert!(Environment::new(Arch::X86).is_32bit());
        assert!(!Environment::new(Arch::X86).is_64bit());

        assert!(Environment::new(Arch::X64).is_64bit());
        assert!(Environment::new(Arch::RISCV32).is_32bit());
        assert!(Environment::new(Arch::RISCV64).is_64bit());
        assert!(Environment::new(Arch::AArch64).is_64bit());
        assert!(!Environment::new(Arch::Unknown).is_32bit());
        assert!(!Environment::new(Arch::Unknown).is_64bit());
    }
}
