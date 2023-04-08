use core::{
    cell::{RefCell, RefMut},
    ops::{Deref, DerefMut},
};

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

    pub async fn lock(&self) -> AsyncMutexGuard<T> {
        let data = wait_and_return(|| {
            let result = self.data.try_borrow_mut();
            match result {
                Ok(data) => (Some(data), true),
                Err(_) => (None, false),
            }
        })
        .await;
        AsyncMutexGuard { data }
    }
}

pub struct AsyncMutexGuard<'a, T> {
    data: RefMut<'a, T>,
}

impl<'a, T> Deref for AsyncMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<'a, T> DerefMut for AsyncMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.deref_mut()
    }
}
