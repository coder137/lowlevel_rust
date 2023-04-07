use core::{
    sync::atomic::AtomicBool,
    task::{RawWaker, RawWakerVTable},
};

mod async_mutex;
pub use async_mutex::*;

mod async_timer;
pub use async_timer::*;

mod async_task;
pub use async_task::*;

mod async_util;
pub use async_util::*;

mod simple_executor;
pub use simple_executor::*;

// RawWakerVTable for efficient wakeups
static VTABLE: RawWakerVTable = {
    unsafe fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    unsafe fn wake(p: *const ()) {
        wake_by_ref(p)
    }
    unsafe fn wake_by_ref(p: *const ()) {
        if p.is_null() {
            return;
        }
        // TODO, Use this when Waker support is implemented
        // Otherwise should only be castable to AtomicBool
        let indicate_ready = &*(p as *const AtomicBool); // unsafe operation (be careful when constructing a RawWaker)
        indicate_ready.store(true, core::sync::atomic::Ordering::SeqCst);
    }
    unsafe fn drop(_: *const ()) {
        // no-op
    }
    RawWakerVTable::new(clone, wake, wake_by_ref, drop)
};
