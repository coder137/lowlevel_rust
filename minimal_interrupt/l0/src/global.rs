use core::sync::atomic::AtomicU32;

pub static SYSTEM_CLOCK: AtomicU32 = AtomicU32::new(4_000_000);

pub fn get_system_clock() -> u32 {
    use core::sync::atomic::Ordering;
    SYSTEM_CLOCK.load(Ordering::SeqCst)
}
