use core::{
    cell::{RefCell, RefMut},
    ops::{Deref, DerefMut},
};

use crate::wait;

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
        wait(|| self.data.try_borrow_mut().is_ok()).await;
        AsyncMutexGuard {
            data: self.data.borrow_mut(),
        }
    }
}

pub struct AsyncMutexGuard<'a, T> {
    data: RefMut<'a, T>,
}

impl<'a, T> Drop for AsyncMutexGuard<'a, T> {
    fn drop(&mut self) {
        // TODO, Add SEV here
    }
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
