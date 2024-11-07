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
