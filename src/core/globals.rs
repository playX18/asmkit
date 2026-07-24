use bitflags::bitflags;

pub const ALLOC_OVERHEAD: u32 = core::mem::size_of::<usize>() as u32 * 4;
pub const ALLOC_ALIGNMENT: u32 = 8;
pub const GROW_THRESHOLD: u32 = 1024 * 1024 * 16;
pub const MAX_TREE_HEIGHT: u32 = if cfg!(target_pointer_width = "32") {
    30
} else {
    61
} + 1;
pub const MAX_OP_COUNT: usize = 6;
pub const MAX_FUNC_ARGS: u32 = 32;
pub const MAX_VALUE_PACK: u32 = 4;
pub const MAX_PHYS_REGS: u32 = 32;
pub const MAX_ALIGNMENT: u32 = 64;
pub const MAX_LABEL_NAME_SIZE: u32 = 2048;
pub const MAX_SECTION_NAME_SIZE: u32 = 35;
pub const MAX_COMMENT_SIZE: u32 = 1024;
pub const INVALID_ID: u32 = 0xFFFFFFFFu32;
pub const NOT_FOUND: u32 = 0xFFFFFFFFu32;
pub const NO_BASE_ADDRESS: u64 = !0u64;
pub const NUM_VIRT_GROUPS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondCode {
    /// (no condition code) (always)
    AL = 0x00,
    /// (not available)     (special)
    NA = 0x01,
    ///        Z==1         (any_sign ==)
    EQ = 0x02,
    ///        Z==0         (any_sign !=)
    NE = 0x03,
    /// C==1                (unsigned >=)
    CS = 0x04,

    /// C==0                (unsigned < )
    LO = 0x05,

    ///               N==1  (is negative)
    MI = 0x06,
    ///               N==0  (is positive or zero)
    PL = 0x07,
    ///               V==1  (is overflow)
    VS = 0x08,
    ///               V==0  (no overflow)
    VC = 0x09,
    /// C==1 & Z==0         (unsigned > )
    HI = 0x0A,
    /// C==0 | Z==1         (unsigned <=)
    LS = 0x0B,
    ///               N==V  (signed   >=)
    GE = 0x0C,
    ///               N!=V  (signed   < )
    LT = 0x0D,
    ///        Z==0 & N==V  (signed   > )
    GT = 0x0E,
    ///        Z==1 | N!=V  (signed   <=)
    LE = 0x0F,
}
impl CondCode {
    pub const MAX_VALUE: Self = Self::LE;
    pub const CC: Self = Self::LO;
    pub const HS: Self = Self::CS;
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct InstOptions: u32 {
        const NONE = 0;
        const RESERVED =  0x00000001;
        const UNFOLLOW = 0x00000002;
        const OVERWRITE = 0x00000004;
        const SHORT_FORM = 0x00000010;
        const LONG_FORM = 0x00000020;
        const TAKEN = 0x00000040;
        const NOT_TAKEN = 0x00000080;

        /// Use ModMR instead of ModRM if applicable.
        const X86_MOD_MR = 0x00000100;
        /// Use ModRM instead of ModMR if applicable.
        const X86_MOD_RM = 0x00000200;
        /// Use 3-byte VEX prefix if possible (AVX).
        const X86_VEX3 = 0x00000400;
        /// Use VEX prefix when both VEX|EVEX prefixes are available.
        const X86_VEX = 0x00000800;
        /// Use 4-byte EVEX prefix if possible (AVX-512).
        const X86_EVEX = 0x00001000;

        /// LOCK prefix (lock-enabled instructions only).
        const X86_LOCK = 0x00002000;
        /// REP prefix (string instructions only).
        const X86_REP = 0x00004000;
        /// REPNE prefix (string instructions only).
        const X86_REPNE = 0x00008000;

        /// XACQUIRE prefix (only allowed instructions).
        const X86_XACQUIRE = 0x00010000;
        /// XRELEASE prefix (only allowed instructions).
        const X86_XRELEASE = 0x00020000;

        /// AVX-512: embedded-rounding {er} and implicit {sae}.
        const X86_ER = 0x00040000;
        /// AVX-512: suppress-all-exceptions {sae}.
        const X86_SAE = 0x00080000;
        /// AVX-512: round-to-nearest (even) {rn-sae} (bits 00).
        const X86_RN_SAE = 0x00000000;
        /// AVX-512: round-down (toward -inf) {rd-sae} (bits 01).
        const X86_RD_SAE = 0x00200000;
        /// AVX-512: round-up (toward +inf) {ru-sae} (bits 10).
        const X86_RU_SAE = 0x00400000;
        /// AVX-512: round-toward-zero (truncate) {rz-sae} (bits 11).
        const X86_RZ_SAE = 0x00600000;
        /// AVX-512: Use zeroing {k}{z} instead of merging {k}.
        const X86_ZMASK = 0x00800000;

        /// AVX-512: mask to get embedded rounding bits (2 bits).
        const X86_ER_MASK = Self::X86_RZ_SAE.bits();
        /// AVX-512: mask of all possible AVX-512 options except the EVEX prefix flag.
        const X86_AVX512_MASK = 0x00FC0000;

        /// Force REX.B and/or VEX.B field (X64 only, used internally).
        const X86_OP_CODE_B = 0x01000000;
        /// Force REX.X and/or VEX.X field (X64 only, used internally).
        const X86_OP_CODE_X = 0x02000000;
        /// Force REX.R and/or VEX.R field (X64 only, used internally).
        const X86_OP_CODE_R = 0x04000000;
        /// Force REX.W and/or VEX.W field (X64 only, used internally).
        const X86_OP_CODE_W = 0x08000000;
        /// Force REX prefix (X64 only).
        const X86_REX = 0x40000000;
        /// Invalid REX prefix (set by X86 or when AH|BH|CH|DH regs are used on X64).
        const X86_INVALID_REX = 0x80000000;
    }
}
