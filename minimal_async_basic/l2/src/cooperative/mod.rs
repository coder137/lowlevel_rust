use core::{
    future::Future,
    pin::{pin, Pin},
    ptr,
    sync::atomic::AtomicBool,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

pub mod simple_executor;

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

pub mod poll {
    use core::cell::{RefCell, RefMut};

    use super::*;

    // Async Task that does not have any Waker support
    // The only way to resolve the future is to keep polling
    pub struct AsyncTask<'a> {
        future: Pin<&'a mut dyn Future<Output = ()>>,
        ready: bool,
    }

    impl<'a> AsyncTask<'a> {
        pub fn new(future: Pin<&'a mut dyn Future<Output = ()>>) -> Self {
            Self {
                future,
                ready: false,
            }
        }

        fn poll(&mut self) {
            // Waker constructed inplace with no AtomicBool to indicate wakeup
            let waker = unsafe { Waker::from_raw(RawWaker::new(ptr::null(), &VTABLE)) };
            let mut context = Context::from_waker(&waker);
            self.ready = self.future.as_mut().poll(&mut context).is_ready();
        }

        // Can keep polling a future without any side effects
        fn safe_poll(&mut self) {
            if self.ready {
                return;
            }
            self.poll();
        }
    }

    // Join async tasks so that they can run concurrently
    pub fn join_tasks<'a, const N: usize>(
        tasks: [AsyncTask<'a>; N],
    ) -> impl Future<Output = ()> + 'a {
        Join { tasks }
    }

    pub fn wait(ready: impl Fn() -> bool) -> impl Future<Output = ()> {
        Wait { ready }
    }

    // TODO, Add wait_until API

    struct Join<'a, const N: usize> {
        tasks: [AsyncTask<'a>; N],
    }

    impl<'a, const N: usize> Future for Join<'a, N> {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
            let pending_tasks = self
                .tasks
                .iter_mut()
                // Filter by tasks that are pending (we do not want to poll ready tasks)
                .filter(|t| !t.ready)
                // Poll the tasks that are pending
                .map(|t| {
                    t.poll();
                    t.ready
                })
                // Filter and count the tasks that are still pending
                .filter(|ready| !ready)
                .count();

            if pending_tasks == 0 {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }

    struct Wait<F: Fn() -> bool> {
        ready: F,
    }

    impl<F: Fn() -> bool> Future for Wait<F> {
        // TODO, Make this return data of a particular type
        type Output = ();

        fn poll(
            self: core::pin::Pin<&mut Self>,
            _: &mut core::task::Context<'_>,
        ) -> Poll<Self::Output> {
            if (self.ready)() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }

    // TODO, Add WaitUntil API

    pub struct AsyncMutex<T> {
        data: RefCell<T>,
    }

    impl<T> AsyncMutex<T> {
        pub const fn new(data: T) -> Self {
            Self {
                data: RefCell::new(data),
            }
        }

        pub async fn lock(&self) -> RefMut<T> {
            wait(|| self.data.try_borrow_mut().is_ok()).await;
            self.data.borrow_mut()
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::simple_executor::block_on;

        use super::*;
        use core::cell::Cell;

        #[test]
        fn join_task_test() {
            let success = Cell::new(false);

            let f1 = pin!(async {
                println!("F1: Started");
                wait(|| success.get()).await;
                println!("F1: Ended");
            });

            let f2 = pin!(async {
                println!("F2: Setting true");
                success.set(true);
                println!("F2: Ended");
            });

            let f3 = pin!(async {
                println!("F3: Started");
                wait(|| success.get()).await;
                println!("F3: Ended");
            });

            block_on(async {
                join_tasks([AsyncTask::new(f1), AsyncTask::new(f2), AsyncTask::new(f3)]).await;
            });

            // assert_eq!(true, false);
        }
    }
}
