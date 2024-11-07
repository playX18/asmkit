pub const fn lsb_mask_usize(n: usize) -> usize {
    if n != 0 {
        usize::MAX >> (size_of::<usize>() * 8 - n)
    } else {
        0
    }
}

pub const fn lsb_mask32(n: u32) -> usize {
    if size_of::<u32>() < size_of::<usize>() {
        (1 << n as usize) - 1
    } else {
        if n != 0 {
            u32::MAX as usize >> (size_of::<u32>() * 8 - n as usize)
        } else {
            0
        }
    }
}

pub const fn lsb_mask64(n: u64) -> usize {
    if size_of::<u64>() < size_of::<usize>() {
        (1 << n as usize) - 1
    } else {
        if n != 0 {
            u64::MAX as usize >> (size_of::<u64>() * 8 - n as usize)
        } else {
            0
        }
    }
}

pub fn bitmask_from_bool(src: bool) -> u32 {
    0u32.wrapping_sub(src as u32)
}

pub fn align_up_usize(addr: usize, align: usize) -> usize {
    let mask = align - 1;
    (addr + mask) & !mask
}

pub fn is_between<T>(x: T, a: T, b: T) -> bool
where
    T: PartialOrd + Ord,
{
    x >= a && x <= b
}

#[cfg(not(windows))]
pub fn lookup_with_dlsym(name: &str) -> *const u8 {
    use alloc::ffi::CString;
    use core::ptr::null;

    let c_str = CString::new(name).unwrap();
    let c_str_ptr = c_str.as_ptr();
    let sym = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c_str_ptr) };
    if sym.is_null() {
        null()
    } else {
        sym as *const u8
    }
}
