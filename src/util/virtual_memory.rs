#![allow(unused_imports, dead_code)]
use alloc::format;
use alloc::string::String;

use core::{
    ffi::CStr,
    mem::MaybeUninit,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
    ptr::{addr_of, addr_of_mut},
    sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering},
};

/// Virtual memory information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Info {
    /// The size of a page of virtual memory.
    pub page_size: u32,
    /// The granularity of a page of virtual memory.
    pub page_granularity: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MemoryFlags(pub u32);

impl From<MemoryFlags> for u32 {
    fn from(val: MemoryFlags) -> Self {
        val.0
    }
}

impl From<u32> for MemoryFlags {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl MemoryFlags {
    /// No flags
    pub const NONE: u32 = 0;
    /// Memory is readable.
    pub const ACCESS_READ: u32 = 0x00000001;

    /// Memory is writable.
    pub const ACCESS_WRITE: u32 = 0x00000002;

    /// Memory is executable.
    pub const ACCESS_EXECUTE: u32 = 0x00000004;

    /// Memory is readable and writable.
    pub const ACCESS_RW: u32 = Self::ACCESS_READ | Self::ACCESS_WRITE;

    /// Memory is readable and executable.
    pub const ACCESS_RX: u32 = Self::ACCESS_READ | Self::ACCESS_EXECUTE;

    /// Memory is readable, writable and executable.
    pub const ACCESS_RWX: u32 = Self::ACCESS_READ | Self::ACCESS_WRITE | Self::ACCESS_EXECUTE;

    /// Use a `MAP_JIT` flag available on Apple platforms (introduced by Mojave), which allows JIT code to be
    /// executed in a MAC bundle.
    ///
    /// This flag may be turned on by the allocator if there is no other way of allocating executable memory.
    ///
    /// ## Note
    /// This flag can only be used with [alloc()], `MAP_JIT` only works on OSX and not on iOS.
    /// When a process uses `fork()` the child process has no access to the pages mapped with `MAP_JIT`.
    pub const MMAP_ENABLE_JIT: u32 = 0x00000010;
    /// Pass `PROT_MAX(PROT_READ)` or `PROT_MPROTECT(PROT_READ)` to `mmap()` on platforms that support it.
    ///
    /// This flag allows to set a "maximum access" that the memory page can get during its lifetime. Use
    /// [protect] to change the access flags.
    ///
    /// ## Note
    /// This flag can only be used with [alloc()] and [alloc_dual_mapping].
    /// However [alloc_dual_mapping] may automatically use this if `AccessRead` is used.
    pub const MMAP_MAX_ACCESS_READ: u32 = 0x00000020;

    /// Pass `PROT_MAX(PROT_WRITE)` or `PROT_MPROTECT(PROT_WRITE)` to `mmap()` on platforms that support it.
    ///
    /// This flag allows to set a "maximum access" that the memory page can get during its lifetime. Use
    /// [protect] to change the access flags.
    ///
    /// ## Note
    /// This flag can only be used with [alloc()] and [alloc_dual_mapping].
    /// However [alloc_dual_mapping] may automatically use this if `AccessWrite` is used.
    pub const MMAP_MAX_ACCESS_WRITE: u32 = 0x00000040;

    /// Pass `PROT_MAX(PROT_EXEC)` or `PROT_MPROTECT(PROT_EXEC)` to `mmap()` on platforms that support it.
    ///
    /// This flag allows to set a "maximum access" that the memory page can get during its lifetime. Use
    /// [protect] to change the access flags.
    ///
    /// ## Note
    /// This flag can only be used with [alloc()] and [alloc_dual_mapping].
    /// However [alloc_dual_mapping] may automatically use this if `AccessExecute` is used.
    pub const MMAP_MAX_ACCESS_EXECUTE: u32 = 0x00000080;

    pub const MMAP_MAX_ACCESS_RW: u32 = Self::MMAP_MAX_ACCESS_READ | Self::MMAP_MAX_ACCESS_WRITE;
    pub const MMAP_MAX_ACCESS_RX: u32 = Self::MMAP_MAX_ACCESS_READ | Self::MMAP_MAX_ACCESS_EXECUTE;
    pub const MMAP_MAX_ACCESS_RWX: u32 =
        Self::MMAP_MAX_ACCESS_READ | Self::MMAP_MAX_ACCESS_WRITE | Self::MMAP_MAX_ACCESS_EXECUTE;

    /// Use `MAP_SHARED` when calling mmap().
    ///
    /// ## Note
    /// In some cases `MAP_SHARED` may be set automatically. For example, some dual mapping implementations must
    /// use `MAP_SHARED` instead of `MAP_PRIVATE` to ensure that the OS would not apply copy on write on RW page, which
    /// would cause RX page not having the updated content.
    pub const MAP_SHARED: u32 = 0x00000100;

    /// Not an access flag, only used by `alloc_dual_mapping()` to override the default allocation strategy to always use
    /// a 'tmp' directory instead of "/dev/shm" (on POSIX platforms). Please note that this flag will be ignored if the
    /// operating system allows to allocate an executable memory by a different API than `open()` or `shm_open()`. For
    /// example on Linux `memfd_create()` is preferred and on BSDs `shm_open(SHM_ANON, ...)` is used if SHM_ANON is
    /// defined.
    ///
    /// ## Note
    /// This flag can only be used with [alloc()].
    pub const MAPPING_PREFER_TMP: u32 = 0x80000000;
}

impl MemoryFlags {
    pub fn contains(self, other: u32) -> bool {
        (self.0 & other) != 0
    }
}

impl BitOr<MemoryFlags> for MemoryFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOr<u32> for MemoryFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: u32) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOrAssign<MemoryFlags> for MemoryFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitOrAssign<u32> for MemoryFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: u32) {
        *self = *self | rhs;
    }
}

impl BitAnd<MemoryFlags> for MemoryFlags {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAnd<u32> for MemoryFlags {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: u32) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAndAssign<MemoryFlags> for MemoryFlags {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitAndAssign<u32> for MemoryFlags {
    #[inline]
    fn bitand_assign(&mut self, rhs: u32) {
        *self = *self & rhs;
    }
}

impl PartialEq<u32> for MemoryFlags {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

/// Dual memory mapping used to map an anonymous memory into two memory regions where one region is read-only, but
/// executable, and the second region is read+write, but not executable. See [alloc_dual_mapping] for
/// more details.
pub struct DualMapping {
    /// Pointer to data with 'Read+Execute' access (this memory is not writable).
    pub rx: *const u8,
    /// Pointer to data with 'Read+Write' access (this memory is not executable).
    pub rw: *mut u8,
}

/// Hardened runtime flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u32)]
pub enum HardenedRuntimeFlags {
    /// No flags
    #[default]
    None = 0,
    /// Hardened runtime is enabled - it's not possible to have "Write & Execute" memory protection. The runtime
    /// enforces W^X (either write or execute).
    ///
    /// ## Note
    /// If the runtime is hardened it means that an operating system specific protection is used. For example
    /// on Apple OSX it's possible to allocate memory with MAP_JIT flag and then use `pthread_jit_write_protect_np()`
    /// to temporarily swap access permissions for the current thread. Dual mapping is also a possibility on X86/X64
    /// architecture.
    Enabled = 0x00000001,
    /// Read+Write+Execute can only be allocated with MAP_JIT flag (Apple specific, only available on OSX).
    MapJit = 0x00000002,

    EnabledMapJit = Self::Enabled as u32 | Self::MapJit as u32,
}

#[derive(Default)]
pub struct HardenedRuntimeInfo {
    pub flags: HardenedRuntimeFlags,
}

/// Values that can be used with [`protect_jit_memory`] function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ProtectJitAccess {
    /// Protect JIT memory with Read+Write permissions.
    ReadWrite = 0,
    /// Protect JIT memory with Read+Execute permissions.
    ReadExecute = 1,
}

pub const DUAL_MAPPING_FILTER: [u32; 2] = [
    MemoryFlags::ACCESS_WRITE | MemoryFlags::MMAP_MAX_ACCESS_WRITE,
    MemoryFlags::ACCESS_EXECUTE | MemoryFlags::MMAP_MAX_ACCESS_EXECUTE,
];

use errno::errno;

use libc::*;

use crate::AsmError;

cfgenius::define! {
    vm_shm_detect = cfg(
        any(
            target_vendor="apple",
            target_os="android"
        )
    );

    has_shm_open = cfg(not(target_os="android"));
    has_pthread_jit_write_protect_np = cfg(all(
        target_os="macos"
    ));

    has_shm_anon = cfg(target_os="freebsd");


}

fn error_from_errno() -> AsmError {
    match errno().0 {
        EACCES | EAGAIN | ENODEV | EPERM => AsmError::InvalidState,
        EFBIG | ENOMEM | EOVERFLOW => AsmError::OutOfMemory,
        EMFILE | ENFILE => AsmError::TooManyHandles,

        _ => AsmError::InvalidArgument,
    }
}

cfgenius::cond! {
    if cfg(not(windows))
    {


        fn get_vm_info() -> Info {
            extern "C" {
                fn getpagesize() -> c_int;
            }

            let page_size = unsafe { getpagesize() as usize };

            Info {
                page_size: page_size as _,
                page_granularity: 65536.max(page_size) as _,
            }
        }

        #[cfg(target_os="macos")]
        fn get_osx_version() -> i32 {
            static GLOBAL_VERSION: AtomicI32 = AtomicI32::new(0);

            let mut ver = GLOBAL_VERSION.load(Ordering::Relaxed);

            if ver == 0 {
                unsafe {
                    let mut osname: MaybeUninit<utsname> = MaybeUninit::uninit();
                    uname(osname.as_mut_ptr());
                    ver = atoi(CStr::from_ptr((*osname.as_ptr()).release.as_ptr().cast()).to_bytes().as_ptr().cast());
                    GLOBAL_VERSION.store(ver, Ordering::Relaxed);
                }
            }

            ver
        }

        fn mm_prot_from_memory_flags(memory_flags: MemoryFlags) -> i32 {
            let mut prot = 0;


            let x = memory_flags;
            if x.contains(MemoryFlags::ACCESS_READ) { prot |= PROT_READ }
            if x.contains(MemoryFlags::ACCESS_WRITE) { prot |= PROT_WRITE }
            if x.contains(MemoryFlags::ACCESS_EXECUTE) { prot |= PROT_EXEC }



            prot
        }
        // Some operating systems don't allow /dev/shm to be executable. On Linux this happens when /dev/shm is mounted with
        // 'noexec', which is enforced by systemd. Other operating systems like MacOS also restrict executable permissions
        // regarding /dev/shm, so we use a runtime detection before attempting to allocate executable memory. Sometimes we
        // don't need the detection as we know it would always result in `AnonymousMemoryStrategy::TmpDir`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum AnonymousMemoryStrategy {
            Unknown = 0,
            DevShm = 1,
            TmpDir = 2,
        }

        #[cfg(not(target_os="freebsd"))]
        fn get_tmp_dir() -> String {
            unsafe{
                let env = getenv(b"TMPDIR\0".as_ptr() as *const _);

                if !env.is_null() {
                    CStr::from_ptr(env).to_string_lossy().into_owned()
                } else {
                    String::from("/tmp")
                }
            }

        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum FileType {
            None,
            Shm,
            Tmp,
        }

        struct AnonymousMemory {
            fd: i32,
            filetype: FileType,
            tmpname: String,
        }
        #[allow(clippy::needless_late_init)]
        impl AnonymousMemory {
            #[allow(unused_variables)]
            fn open(&mut self, prefer_tmp_over_dev_shm: bool) -> Result<(), AsmError> {
                cfgenius::cond! {
                    if cfg(target_os="linux") {


                        // Linux specific 'memfd_create' - if the syscall returns `ENOSYS` it means
                        // it's not available and we will never call it again (would be pointless).
                        //
                        // NOTE: There is also memfd_create() libc function in FreeBSD, but it internally
                        // uses `shm_open(SHM_ANON, ...)` so it's not needed to add support for it (it's
                        // not a syscall as in Linux).

                        /// If ever changed to '1' that would mean the syscall is not
                        /// available and we must use `shm_open()` and `shm_unlink()` (or regular `open()`).
                        static MEMFD_CREATE_NOT_SUPPORTED: AtomicBool = AtomicBool::new(false);

                        if !MEMFD_CREATE_NOT_SUPPORTED.load(Ordering::Relaxed) {
                            unsafe {
                                self.fd = libc::syscall(libc::SYS_memfd_create, b"vmem\0".as_ptr(), libc::MFD_CLOEXEC) as i32;

                                if self.fd >= 0 {

                                    return Ok(());
                                }

                                if errno().0 == ENOSYS {
                                    MEMFD_CREATE_NOT_SUPPORTED.store(true, Ordering::Relaxed);
                                } else {
                                    return Err(error_from_errno());
                                }
                            }
                        }
                    }
                }

                cfgenius::cond! {
                    if all(macro(has_shm_open), macro(has_shm_anon)) {
                        unsafe {
                            let _ = prefer_tmp_over_dev_shm;
                            self.fd = shm_open(libc::SHM_ANON, libc::O_RDWR | libc::O_CREAT | libc::O_EXCL, libc::S_IRUSR | libc::S_IWUSR);

                            if self.fd >= 0 {
                                return Ok(())
                            } else {
                                return Err(error_from_errno());
                            }
                        }
                    } else {
                        // POSIX API. We have to generate somehow a unique name. This is nothing cryptographic, just using a bit from
                        // the stack address to always have a different base for different threads (as threads have their own stack)
                        // and retries for avoiding collisions. We use `shm_open()` with flags that require creation of the file so we
                        // never open an existing shared memory.
                        static INTERNAL_COUNTER: AtomicU32 = AtomicU32::new(0);



                        let retry_count = 100;
                        let mut bits = self as *const Self as u64 & 0x55555555;

                        for _ in 0..retry_count {
                            bits = bits.wrapping_sub(super::os::get_tick_count() as u64 * 773703683);
                            bits = ((bits >> 14) ^ (bits << 6)) + INTERNAL_COUNTER.fetch_add(1, Ordering::AcqRel) as u64 + 10619863;

                            let use_tmp;
                            cfgenius::cond! {
                                if macro(vm_shm_detect) {
                                    use_tmp = true;
                                } else {
                                    use_tmp = prefer_tmp_over_dev_shm;
                                }
                            };

                            if use_tmp {
                                self.tmpname.push_str(&get_tmp_dir());
                                self.tmpname.push_str(&format!("/shm-id-{:016X}\0", bits));

                                unsafe {
                                    self.fd = libc::open(
                                        self.tmpname.as_ptr() as *const c_char,
                                        libc::O_RDWR | libc::O_CREAT | libc::O_EXCL,
                                        0
                                    );

                                    if self.fd >= 0 {
                                        self.filetype = FileType::Tmp;
                                        return Ok(());
                                    }
                                }
                            } else {
                                self.tmpname = format!("shm-id-{:016X}\0", bits);

                                unsafe {
                                    self.fd = libc::shm_open(
                                        self.tmpname.as_ptr() as *const c_char,
                                        libc::O_RDWR | libc::O_CREAT | libc::O_EXCL,
                                        0
                                    );

                                    if self.fd >= 0 {
                                        self.filetype = FileType::Shm;
                                        return Ok(());
                                    }
                                }
                            }

                            if errno().0 != EEXIST {
                                return Err(error_from_errno());
                            }
                        }
                    }
                }

                Err(AsmError::FailedToOpenAnonymousMemory)
            }

            fn unlink(&mut self) {
                #[allow(unused_variables)]
                let typ = self.filetype;
                self.filetype = FileType::None;

                cfgenius::cond! {
                    if macro(has_shm_open) {
                        if typ== FileType::Shm {
                            unsafe {
                                libc::shm_unlink(self.tmpname.as_ptr() as *const c_char);
                                return;
                            }
                        }

                    }
                }
                #[allow(unreachable_code)]
                if typ == FileType::Tmp {
                    unsafe {
                        libc::unlink(self.tmpname.as_ptr() as *const c_char);
                    }


                }

            }

            fn close(&mut self) {
                if self.fd >= 0 {
                    unsafe {
                        libc::close(self.fd);
                    }

                    self.fd = -1;
                }
            }

            const fn new() -> Self {
                Self {
                    fd: -1,
                    filetype: FileType::None,
                    tmpname: String::new(),
                }
            }

            fn allocate(&self, size: usize) -> Result<(), AsmError> {
                unsafe {
                    if libc::ftruncate(self.fd, size as _) != 0 {
                        return Err(error_from_errno());
                    }

                    Ok(())
                }
            }
        }

        impl Drop for AnonymousMemory {
            fn drop(&mut self) {
                self.unlink();
                self.close();
            }
        }
    }
}

cfgenius::cond! {
    if macro(vm_shm_detect) {
        fn detect_anonymous_memory_strategy() -> Result<AnonymousMemoryStrategy, AsmError> {
            let mut anon_mem = AnonymousMemory::new();
            let vm_info = info();

            anon_mem.open(false)?;
            anon_mem.allocate(vm_info.page_size as usize)?;

            unsafe {
                let ptr = libc::mmap(core::ptr::null_mut(), vm_info.page_size as _, libc::PROT_READ | libc::PROT_EXEC, libc::MAP_SHARED, anon_mem.fd, 0);
                if ptr == libc::MAP_FAILED {
                    if errno().0 == EINVAL {
                        return Ok(AnonymousMemoryStrategy::TmpDir);
                    }

                    return Err(error_from_errno());
                } else {
                    libc::munmap(ptr, vm_info.page_size as _);
                    Ok(AnonymousMemoryStrategy::DevShm)
                }
            }
        }
    }
}

cfgenius::cond! {
    if cfg(not(windows)) {
        #[allow(unreachable_code)]
        pub fn get_anonymous_memory_strategy() -> Result<AnonymousMemoryStrategy, AsmError> {
            cfgenius::cond! {
                if macro(vm_shm_detect) {
                    use core::sync::atomic::AtomicU8;
                    static GLOBAL_STRATEGY: AtomicU8 = AtomicU8::new(0);

                    if GLOBAL_STRATEGY.load(Ordering::Acquire) != 0 {
                        return Ok(unsafe { core::mem::transmute(GLOBAL_STRATEGY.load(Ordering::Acquire)) });
                    }

                    let strategy = detect_anonymous_memory_strategy()?;

                    GLOBAL_STRATEGY.store(strategy as u8, Ordering::Release);

                    return Ok(strategy)
                }
            }

            Ok(AnonymousMemoryStrategy::TmpDir)
        }
/// Detects whether the current process is hardened, which means that pages that have WRITE and EXECUTABLE flags
/// cannot be normally allocated. On OSX + AArch64 such allocation requires MAP_JIT flag, other platforms don't
/// support this combination.
#[cfg(not(windows))]
pub fn has_hardened_runtime() -> bool {
    cfgenius::cond! {
        if cfg(all(target_os="macos")) {
            true
        } else {
            static GLOBAL_HARDENED_FLAG: AtomicU32 = AtomicU32::new(0);

            let mut flag = GLOBAL_HARDENED_FLAG.load(Ordering::Acquire);

            if flag == 0 {
                let page_size = info().page_size;

                unsafe {
                    let ptr = libc::mmap(core::ptr::null_mut(), page_size as _, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC, libc::MAP_PRIVATE | libc::MAP_ANONYMOUS, -1, 0);

                    if ptr == libc::MAP_FAILED {
                        flag = 2;
                    } else {
                        flag = 1;
                        libc::munmap(ptr, page_size as _);
                    }
                }

                GLOBAL_HARDENED_FLAG.store(flag, Ordering::Release);
            }

            flag == 2
        }
    }
}

pub const fn has_map_jit_support() -> bool {
    cfgenius::cond! {
        if cfg(all(target_os="macos")) {
            true
        } else {
            false
        }
    }
}

pub fn map_jit_from_memory_flags(memory_flags: MemoryFlags) -> i32 {
    cfgenius::cond! {
        if cfg(target_vendor="apple") {
            // Always use MAP_JIT flag if user asked for it (could be used for testing on non-hardened processes) and detect
            // whether it must be used when the process is actually hardened (in that case it doesn't make sense to rely on
            // user `memoryFlags`).
            //
            // MAP_JIT is not required when dual-mapping memory and is incompatible with MAP_SHARED, so it will not be
            // added when the latter is enabled.

            let use_map_jit = (memory_flags.contains(MemoryFlags::MMAP_ENABLE_JIT) || has_hardened_runtime())
                && !memory_flags.contains(MemoryFlags::MAP_SHARED);

            if use_map_jit {
                if has_map_jit_support() {
                    return libc::MAP_JIT as i32;
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            let _ = memory_flags;
            0
        }
    }
}

pub fn get_hardened_runtime_flags() -> HardenedRuntimeFlags {
    let mut flags = 0;

    if has_hardened_runtime() {
        flags = HardenedRuntimeFlags::Enabled as u32;
    }

    if has_map_jit_support() {
        flags |= HardenedRuntimeFlags::MapJit as u32;
    }

    match flags {
        0 => HardenedRuntimeFlags::None,
        1 => HardenedRuntimeFlags::Enabled,
        2 => HardenedRuntimeFlags::MapJit,
        3 => HardenedRuntimeFlags::EnabledMapJit,
        _ => unreachable!(),
    }
}

pub fn max_access_flags_to_regular_access_flags(memory_flags: MemoryFlags) -> MemoryFlags {
    const MAX_PROT_SHIFT: u32 = MemoryFlags::MMAP_MAX_ACCESS_READ.trailing_zeros();

    MemoryFlags((memory_flags.0 & MemoryFlags::MMAP_MAX_ACCESS_RWX) >> MAX_PROT_SHIFT)
}

pub fn regular_access_flags_to_max_access_flags(memory_flags: MemoryFlags) -> MemoryFlags {
    const MAX_PROT_SHIFT: u32 = MemoryFlags::MMAP_MAX_ACCESS_READ.trailing_zeros();

    MemoryFlags((memory_flags.0 & MemoryFlags::MMAP_MAX_ACCESS_RWX) << MAX_PROT_SHIFT)
}

pub fn mm_max_prot_from_memory_flags(_memory_flags: MemoryFlags) -> i32 {
    _memory_flags.0 as _
}


fn map_memory(
    size: usize,
    memory_flags: MemoryFlags,
    fd: i32,
    offset: libc::off_t,
) -> Result<*mut u8, AsmError> {
    if size == 0 {
        return Err(AsmError::InvalidArgument);
    }

    let protection = mm_prot_from_memory_flags(memory_flags);

    let mut mm_flags = map_jit_from_memory_flags(memory_flags);

    mm_flags |= if memory_flags.contains(MemoryFlags::MAP_SHARED) {
        libc::MAP_SHARED
    } else {
        libc::MAP_PRIVATE
    };

    if fd == -1 {
        mm_flags |= libc::MAP_ANONYMOUS;
    }
    unsafe {
        let ptr = libc::mmap(
            core::ptr::null_mut(),
            size as _,
            protection,
            mm_flags,
            fd,
            offset,
        );

        if ptr == libc::MAP_FAILED {
            return Err(error_from_errno());
        }
        Ok(ptr.cast())
    }
}

fn unmap_memory(ptr: *mut u8, size: usize) -> Result<(), AsmError> {
    if size == 0 {
        return Err(AsmError::InvalidArgument);
    }

    unsafe {
        if libc::munmap(ptr.cast(), size as _) == 0 {
            Ok(())
        } else {
            Err(error_from_errno())
        }
    }
}

pub fn alloc(size: usize, memory_flags: MemoryFlags) -> Result<*mut u8, AsmError> {
    map_memory(size, memory_flags, -1, 0)
}

pub fn release(ptr: *mut u8, size: usize) -> Result<(), AsmError> {
    unmap_memory(ptr, size)
}

pub fn protect(p: *mut u8, size: usize, memory_flags: MemoryFlags) -> Result<(), AsmError> {
    let protection = mm_prot_from_memory_flags(memory_flags);

    unsafe {
        if libc::mprotect(p.cast(), size as _, protection) == 0 {
            Ok(())
        } else {
            Err(error_from_errno())
        }
    }
}

fn unmap_dual_mapping(dm: &mut DualMapping, size: usize) -> Result<(), AsmError> {
    let err1 = unmap_memory(dm.rx as _, size);
    let mut err2 = Ok(());

    if dm.rx != dm.rw {
        err2 = unmap_memory(dm.rw as _, size);
    }

    err1?;
    err2?;

    dm.rx = core::ptr::null_mut();
    dm.rw = core::ptr::null_mut();

    Ok(())
}

/// Allocates virtual memory and creates two views of it where the first view has no write access. This is an addition
/// to the API that should be used in cases in which the operating system either enforces W^X security policy or the
/// application wants to use this policy by default to improve security and prevent an accidental (or purposed)
/// self-modifying code.
///
/// The memory returned in the `dm` are two independent mappings of the same shared memory region. You must use
/// [release_dual_mapping](release_dual_mapping) to release it when it's no longer needed. Never use [release](release) to
/// release the memory returned by `alloc_dual_mapping()` as that would fail on Windows.
///
/// Both pointers in `dm` would be set to `null` if the function fails.
pub fn alloc_dual_mapping(size: usize, memory_flags: MemoryFlags) -> Result<DualMapping, AsmError> {
    let mut dm = DualMapping {
        rx: core::ptr::null_mut(),
        rw: core::ptr::null_mut(),
    };

    if size as isize <= 0 {
        return Err(AsmError::InvalidArgument);
    }

    let mut prefer_tmp_over_dev_shm = memory_flags.contains(MemoryFlags::MAPPING_PREFER_TMP);

    if !prefer_tmp_over_dev_shm {
        let strategy = get_anonymous_memory_strategy()?;

        prefer_tmp_over_dev_shm = strategy == AnonymousMemoryStrategy::TmpDir;
    }

    let mut anon_mem = AnonymousMemory::new();

    anon_mem.open(prefer_tmp_over_dev_shm)?;
    anon_mem.allocate(size)?;

    let mut ptr = [core::ptr::null_mut(), core::ptr::null_mut()];

    for i in 0..2 {
        let restricted_memory_flags = memory_flags.0 & !DUAL_MAPPING_FILTER[i];

        ptr[i] = match map_memory(
            size,
            (restricted_memory_flags | MemoryFlags::MAP_SHARED).into(),
            anon_mem.fd,
            0,
        ) {
            Ok(p) => p,
            Err(e) => {
                if i == 1 {
                    let _ = unmap_memory(ptr[0], size);
                }

                return Err(e);
            }
        };
    }

    dm.rx = ptr[0];
    dm.rw = ptr[1];

    Ok(dm)
}

/// Releases virtual memory mapping previously allocated by [alloc_dual_mapping()](alloc_dual_mapping).
///
/// Both pointers in `dm` would be set to `nullptr` if the function succeeds.
pub fn release_dual_mapping(dm: &mut DualMapping, size: usize) -> Result<(), AsmError> {
    unmap_dual_mapping(dm, size)
}


    }
}

pub fn info() -> Info {
    static mut INFO: Info = Info {
        page_granularity: 0,
        page_size: 0,
    };
    static INFO_INIT: AtomicBool = AtomicBool::new(false);
    if INFO_INIT.load(Ordering::Relaxed) {
        unsafe { addr_of!(INFO).read() }
    } else {
        unsafe {
            let info = get_vm_info();
            addr_of_mut!(INFO).write(info);
            INFO_INIT.store(true, Ordering::Relaxed);
            info
        }
    }
}

/// Flushes instruction cache in the given region.
///
/// Only useful on non-x86 architectures, however, it's a good practice to call it on any platform to make your
/// code more portable.
pub fn flush_instruction_cache(p: *const u8, size: usize) {
    cfgenius::cond! {
        if cfg(any(target_arch="x86", target_arch="x86_64")) {
            let _ = p;
            let _ = size;
        } else if cfg(target_vendor="apple") {
            extern "C" {
                fn sys_icache_invalidate(p: *const u8, size: usize);
            }

            unsafe {
                sys_icache_invalidate(p, size);
            }
        } else if cfg(windows) {
            extern "C" {
                fn GetCurrentProcess() -> *mut libc::c_void;
                fn FlushInstructionCache(
                    proc: *mut libc::c_void,
                    lp: *const u8,
                    dw_size: usize,
                ) -> i32;
            }

            unsafe {
                FlushInstructionCache(GetCurrentProcess(), p, size);
            }
        } else if cfg(target_arch="aarch64")
            {
                let code = p as usize;
                let end = code + size;


                use core::arch::asm;

                const ICACHE_LINE_SIZE: usize = 4;
                const DCACHE_LINE_SIZE: usize = 4;

                let mut addr = code & !(DCACHE_LINE_SIZE - 1);

                while addr < end {
                    unsafe {
                        asm!("dc civac, {x}", x = in(reg) addr);
                    }
                    addr += ICACHE_LINE_SIZE;
                }

                unsafe {
                    asm!("dsb ish");
                }

                addr = code & !(ICACHE_LINE_SIZE - 1);

                while addr < end {
                    unsafe {
                        asm!("ic ivau, {x}", x = in(reg) addr);
                    }
                    addr += ICACHE_LINE_SIZE;
                }

                unsafe {
                    asm!(
                        "dsb ish"
                    );
                    asm!(
                        "isb"
                    );
                }

            } else if cfg(any(target_arch="riscv64", target_arch = "riscv32")) {
                unsafe {
                    let _ = wasmtime_jit_icache_coherence::clear_cache(p.cast(), size);
                    let _ = wasmtime_jit_icache_coherence::pipeline_flush_mt();
                }
            } else {
                // TODO: Should we error here?
                //compile_error!("icache invalidation not implemented for target platform");
            }

    }
}

#[cfg(not(windows))]
pub fn hardened_runtime_info() -> HardenedRuntimeInfo {
    HardenedRuntimeInfo {
        flags: get_hardened_runtime_flags(),
    }
}
/// Protects access of memory mapped with MAP_JIT flag for the current thread.
///
/// # Note
/// This feature is only available on Apple hardware (AArch64) at the moment and and uses a non-portable
/// `pthread_jit_write_protect_np()` call when available.
///
/// This function must be called before and after a memory mapped with MAP_JIT flag is modified. Example:
///
/// ```mustfail,rust
/// let code_ptr = ...;
/// let code_size = ...;
///
/// protect_jit_memory(ProtectJitAccess::ReadWrite);
/// copy_nonoverlapping(source, code_ptr, code_size);
/// protect_jit_memory(ProtectJitAccess::ReadOnly);
/// flush_instruction_cache(code_ptr, code_size);
///
/// ```
pub fn protect_jit_memory(access: ProtectJitAccess) {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        unsafe {
            let x = match access {
                ProtectJitAccess::ReadWrite => 0,
                _ => 1,
            };

            libc::pthread_jit_write_protect_np(x);
        }
    }
    let _ = access;
}

cfgenius::cond! {

    if cfg(windows) {

        use winapi::um::sysinfoapi::SYSTEM_INFO;
        use winapi::{
            shared::{minwindef::DWORD, ntdef::HANDLE},
            um::{
                handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
                memoryapi::{
                    CreateFileMappingW, MapViewOfFile, UnmapViewOfFile, VirtualAlloc, VirtualFree,
                    VirtualProtect, FILE_MAP_EXECUTE, FILE_MAP_READ, FILE_MAP_WRITE,
                },
                sysinfoapi::GetSystemInfo,
                winnt::{
                    MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
                    PAGE_READONLY, PAGE_READWRITE,
                },
            },
        };


        struct ScopedHandle {
            value: HANDLE
        }

        impl ScopedHandle {
            fn new() -> Self {
                Self { value: core::ptr::null_mut() }
            }
        }

        impl Drop for ScopedHandle {
            fn drop(&mut self) {
                if !self.value.is_null() {
                    unsafe {
                        CloseHandle(self.value);
                    }
                }
            }
        }

        fn get_vm_info() -> Info {
            let mut system_info = MaybeUninit::<SYSTEM_INFO>::uninit();
            unsafe {
                GetSystemInfo(system_info.as_mut_ptr());

                let system_info = system_info.assume_init();

                Info {
                    page_size: system_info.dwPageSize as u32,
                    page_granularity: system_info.dwAllocationGranularity as u32,
                }
            }
        }

        fn protect_flags_from_memory_flags(memory_flags: MemoryFlags) -> DWORD {
            let protect_flags;

            if memory_flags.contains(MemoryFlags::ACCESS_EXECUTE) {
                protect_flags = if memory_flags.contains(MemoryFlags::ACCESS_WRITE) {
                    PAGE_EXECUTE_READWRITE
                } else {
                    PAGE_EXECUTE_READ
                };
            } else if memory_flags.contains(MemoryFlags::ACCESS_RW) {
                protect_flags = if memory_flags.contains(MemoryFlags::ACCESS_WRITE) {
                    PAGE_READWRITE
                } else {
                    PAGE_READONLY
                };
            } else {
                protect_flags = PAGE_READONLY;
            }

            protect_flags
        }

        fn desired_access_from_memory_flags(memory_flags: MemoryFlags) -> DWORD {
            let mut access = if memory_flags.contains(MemoryFlags::ACCESS_WRITE) {
                FILE_MAP_WRITE
            } else {
                FILE_MAP_READ
            };

            if memory_flags.contains(MemoryFlags::ACCESS_EXECUTE) {
                access |= FILE_MAP_EXECUTE;
            }

            access
        }

        pub fn alloc(size: usize, memory_flags: MemoryFlags) -> Result<*mut u8, AsmError> {
            if size == 0 {
                return Err(AsmError::InvalidArgument)
            }

            unsafe {
                let protect = protect_flags_from_memory_flags(memory_flags);
                let result = VirtualAlloc(core::ptr::null_mut(), size, MEM_COMMIT | MEM_RESERVE, protect);

                if result.is_null() {
                    return Err(AsmError::OutOfMemory)
                }

                Ok(result as *mut u8)
            }
        }

        pub fn release(ptr: *mut u8, size: usize) -> Result<(), AsmError> {
            if size == 0 || ptr.is_null() {
                return Err(AsmError::InvalidArgument)
            }

            unsafe {
                if VirtualFree(ptr as *mut _, 0, MEM_RELEASE) == 0 {
                    return Err(AsmError::InvalidArgument)
                }
            }

            Ok(())
        }

        pub fn protect(p: *mut u8, size: usize, memory_flags: MemoryFlags) -> Result<(), AsmError> {
            let protect_flags = protect_flags_from_memory_flags(memory_flags);
            let mut old_flags = 0;

            unsafe {
                if VirtualProtect(p as _, size, protect_flags, &mut old_flags) != 0 {
                    return Ok(())
                }

                Err(AsmError::InvalidArgument)
            }
        }

        pub fn alloc_dual_mapping(size: usize, memory_flags: MemoryFlags) -> Result<DualMapping, AsmError> {
            if size == 0 {
                return Err(AsmError::InvalidArgument)
            }

            let mut handle = ScopedHandle::new();

            unsafe {
                handle.value = CreateFileMappingW(
                    INVALID_HANDLE_VALUE,
                    core::ptr::null_mut(),
                    PAGE_EXECUTE_READWRITE,
                    ((size as u64) >> 32) as _,
                    (size & 0xFFFFFFFF) as _,
                    core::ptr::null_mut()
                );

                if handle.value.is_null() {
                    return Err(AsmError::OutOfMemory);
                }

                let mut ptr = [core::ptr::null_mut(), core::ptr::null_mut()];

                for i in 0..2 {
                    let access_flags = memory_flags.0 & !DUAL_MAPPING_FILTER[i];
                    let desired_access = desired_access_from_memory_flags(access_flags.into());
                    ptr[i] = MapViewOfFile(handle.value, desired_access, 0, 0, size);

                    if ptr[i].is_null() {
                        if i == 0 {
                            UnmapViewOfFile(ptr[0]);
                        }

                        return Err(AsmError::OutOfMemory);
                    }
                }

                Ok(DualMapping {
                    rx: ptr[0] as _,
                    rw: ptr[1] as _,
                })
            }
        }

        pub fn release_dual_mapping(dm: &mut DualMapping, _size: usize) -> Result<(), AsmError> {
            let mut failed = false;

            unsafe {
                if UnmapViewOfFile(dm.rx as _) == 0 {
                    failed = true;
                }

                if dm.rx != dm.rw && UnmapViewOfFile(dm.rw as _) == 0 {
                    failed = true;
                }

                if failed {
                    return Err(AsmError::InvalidArgument);
                }

                dm.rx = core::ptr::null_mut();
                dm.rw = core::ptr::null_mut();

                Ok(())
            }
        }
    }
}
