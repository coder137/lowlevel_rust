use core::{
    future::Future,
    pin::pin,
    ptr,
    task::{Context, RawWaker, Waker},
};

use super::VTABLE;

// Run one async task till completion
// Blocks the current thread and no other concurrent operations takes place till this future is resolved
pub fn block_on<T>(future: impl Future<Output = T>) -> T {
    let mut future = pin!(future);
    // let mut task = AsyncTask::new(future);
    let waker = unsafe { Waker::from_raw(RawWaker::new(ptr::null(), &VTABLE)) };
    let mut context = Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(&mut context) {
            core::task::Poll::Ready(data) => break data,
            core::task::Poll::Pending => {}
        }
    }
}
