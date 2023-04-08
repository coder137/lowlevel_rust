use core::{
    future::Future,
    ops::{Add, Sub},
    task::Poll,
    time::Duration,
};

use l0::get_current_time;

use crate::{wait, wait_and_return};

pub enum WaitUntilReason {
    DataReady,
    Timeout,
}

pub fn wait_until<F: Fn() -> bool>(
    ready: F,
    timeout: Duration,
) -> impl Future<Output = WaitUntilReason> {
    let timeout_time = get_current_time().add(timeout);
    wait_and_return(move || {
        if ready() {
            (Some(WaitUntilReason::DataReady), true)
        } else if get_current_time() >= timeout_time {
            (Some(WaitUntilReason::Timeout), true)
        } else {
            (None, false)
        }
    })
}

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
            start_time: get_current_time(),
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

#[cfg(test)]
mod tests {
    use core::time::Duration;
    use std::time::Instant;

    use crate::{block_on, sleep_via_timer, sleep_via_wait};

    #[test]
    fn sleep_via_wait_test() {
        let async_timer_cb = async {
            sleep_via_wait(Duration::from_secs(1)).await;
        };
        let instant = Instant::now();
        block_on(async_timer_cb);
        let duration = instant.elapsed();
        assert!(duration.as_secs() >= 1);
    }

    #[test]
    fn sleep_via_timer_test() {
        let async_timer_cb = async {
            sleep_via_timer(Duration::from_secs(1)).await;
        };
        let instant = Instant::now();
        block_on(async_timer_cb);
        let duration = instant.elapsed();
        assert!(duration.as_secs() >= 1);
    }
}
