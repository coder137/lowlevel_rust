use core::{
    future::Future,
    pin::Pin,
    ptr,
    task::{Context, Poll, RawWaker, Waker},
};

use super::VTABLE;

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
}

// Join async tasks so that they can run concurrently
pub fn join_tasks<'a, const N: usize>(tasks: [AsyncTask<'a>; N]) -> impl Future<Output = ()> + 'a {
    Join { tasks }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_on;
    use crate::wait;
    use core::cell::Cell;
    use core::pin::pin;

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
