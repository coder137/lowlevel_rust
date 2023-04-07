use core::{ops::Add, time::Duration};

pub use super::{arm_cm4::*, controller::*, interrupt::*, registers::*};

// TODO, Put this in an appropriate place

static mut SYSTEM_TIME: Duration = Duration::new(0, 0);

// SysTick interrupt
#[no_mangle]
unsafe extern "C" fn SysTick() {
    SYSTEM_TIME = SYSTEM_TIME.add(Duration::from_millis(1));
}

pub fn get_current_time() -> Duration {
    unsafe { SYSTEM_TIME }
}
