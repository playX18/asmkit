//! Shared code generation, target, relocation, and deferred-emission APIs.

#[allow(dead_code)]
#[doc(hidden)]
pub mod arch_traits;
#[allow(dead_code)]
pub mod buffer;
pub mod builder;
#[allow(dead_code)]
#[doc(hidden)]
pub mod formatter;
#[allow(dead_code)]
pub mod globals;
pub mod inst;
#[cfg(feature = "jit")]
pub mod jit_allocator;
pub mod linker;
#[allow(dead_code, unused_imports, unused_macros)]
pub mod operand;

pub mod patch;
pub mod rwinfo;
pub mod section;
#[allow(dead_code)]
#[doc(hidden)]
pub mod support;
pub mod target;
#[doc(hidden)]
pub mod types;
