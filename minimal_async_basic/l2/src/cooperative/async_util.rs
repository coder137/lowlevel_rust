use core::{future::Future, task::Poll};

pub fn wait(ready: impl Fn() -> bool) -> impl Future<Output = ()> {
    Wait { ready }
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
