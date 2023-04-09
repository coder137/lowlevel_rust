use core::cell::{RefCell, RefMut};

use crate::wait_and_return;

pub struct AsyncMutex<T> {
    data: RefCell<T>,
}

impl<T> AsyncMutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: RefCell::new(data),
        }
    }

    #[must_use]
    pub async fn lock(&self) -> AsyncScopedMutex<T> {
        let data = wait_and_return(|| {
            let result = self.data.try_borrow_mut();
            match result {
                Ok(data) => (Some(data), true),
                Err(_) => (None, false),
            }
        })
        .await;
        AsyncScopedMutex { data }
    }
}

pub struct AsyncScopedMutex<'a, T> {
    data: RefMut<'a, T>,
}

impl<'a, T> AsyncScopedMutex<'a, T> {
    pub fn action(self, cb: impl Fn(RefMut<T>)) {
        cb(self.data)
    }
}
