use core::{
    future::Future,
    ops::{Add, Sub},
    task::Poll,
    time::Duration,
};

use l0::get_current_time;

use crate::wait;

pub fn sleep_via_wait(wait_duration: Duration) -> impl Future<Output = ()> {
    let wakeup_time = get_current_time().add(wait_duration);
    wait(move || get_current_time() >= wakeup_time)
}

pub fn sleep_via_timer(wait_duration: Duration) -> impl Future<Output = ()> {
    AsyncTimer::new(wait_duration)
}

struct AsyncTimer {
    wait_duration: Duration,
    start_time: Duration,
}

impl AsyncTimer {
    fn new(wait_duration: Duration) -> Self {
        Self {
            wait_duration,
            start_time: get_current_time(), // TODO, Create an Instant API
        }
    }
}

impl Future for AsyncTimer {
    type Output = ();

    fn poll(
        self: core::pin::Pin<&mut Self>,
        _: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        if get_current_time().sub(self.start_time) >= self.wait_duration {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
