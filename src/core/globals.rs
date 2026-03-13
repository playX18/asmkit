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
    pub struct InstOptions: u32 {
        const NONE = 0;
        const RESERVED =  0x00000001;
        const UNFOLLOW = 0x00000002;
        const OVERWRITE = 0x00000004;
        const SHORT_FORM = 0x00000010;
        const LONG_FORM = 0x00000020;
        const TAKEN = 0x00000040;
        const NOT_TAKEN = 0x00000080;
    }
}
