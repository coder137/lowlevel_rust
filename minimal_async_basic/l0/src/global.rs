use core::{sync::atomic::AtomicU32, time::Duration};

pub static SYSTEM_CLOCK: AtomicU32 = AtomicU32::new(4_000_000);

pub fn get_system_clock() -> u32 {
    use core::sync::atomic::Ordering;
    SYSTEM_CLOCK.load(Ordering::SeqCst)
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub static mut SYSTEM_TIME: Duration = Duration::new(0, 0);

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub fn get_current_time() -> Duration {
    unsafe { SYSTEM_TIME }
}

#[cfg(any(target_family = "windows", target_family = "unix"))]
pub fn get_current_time() -> Duration {
    use std::time::Instant;
    static mut INITIALIZE_INSTANT: Option<Instant> = None;
    unsafe {
        if INITIALIZE_INSTANT.is_none() {
            INITIALIZE_INSTANT = Some(Instant::now());
            Duration::new(0, 0)
        } else {
            INITIALIZE_INSTANT.unwrap().elapsed()
        }
    }
}
