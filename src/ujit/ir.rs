use std::sync::atomic::{AtomicBool, AtomicUsize};


#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct BlockId(pub(crate) u32);



#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct FunctionId(pub(crate) u32);


/// A basic-block of ujit.
pub struct Block {
    pub func: FunctionId,
}

pub struct Function {
    pub(crate) is_recompilable: bool,
    pub(crate) is_optimized: bool,
    pub(crate) no_throw: bool,
    pub(crate) no_return: bool,
    pub(crate) has_try: bool,
    pub(crate) opt_level: u8,
    pub(crate) is_compiled: AtomicBool,
    pub(crate) entry_point: AtomicUsize,
    pub(crate) redirector: AtomicUsize,
    pub(crate) indirector: AtomicUsize
}

