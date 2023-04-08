use core::{future::Future, task::Poll};

pub fn wait(ready: impl Fn() -> bool) -> impl Future<Output = ()> {
    Wait {
        ready: move || (Some(()), ready()),
    }
}

pub fn wait_and_return<T>(ready: impl Fn() -> (Option<T>, bool)) -> impl Future<Output = T> {
    Wait { ready }
}

struct Wait<T, F: Fn() -> (Option<T>, bool)> {
    ready: F,
}

impl<T, F: Fn() -> (Option<T>, bool)> Future for Wait<T, F> {
    // TODO, Make this return data of a particular type
    type Output = T;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        _: &mut core::task::Context<'_>,
    ) -> Poll<Self::Output> {
        let (data, ready) = (self.ready)();
        if ready {
            Poll::Ready(data.unwrap())
        } else {
            Poll::Pending
        }
    }
}
