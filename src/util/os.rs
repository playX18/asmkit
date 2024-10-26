pub fn get_tick_count() -> u32 {
    cfgenius::cond! {
        if cfg(windows) {
            extern "C" {
                fn GetTickCount() -> u32;
            }

            unsafe { GetTickCount() }
        } else {
            use core::mem::MaybeUninit;
            let mut ts: MaybeUninit<libc::timespec> = MaybeUninit::zeroed();

            unsafe {
                if libc::clock_gettime(libc::CLOCK_MONOTONIC, ts.as_mut_ptr()) != 0 {
                    return 0;
                }
                let ts = ts.assume_init();
                let t = ((ts.tv_sec as u64) * 1000) + (ts.tv_nsec as u64 / 1000000);
                (t & 0xFFFFFFFF) as u32
            }
        }
    }
}
