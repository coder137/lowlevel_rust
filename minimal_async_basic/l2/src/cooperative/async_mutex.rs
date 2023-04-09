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

#[cfg(test)]
mod tests {
    use crate::{block_on, join_tasks, sleep_via_wait, AsyncMutex, AsyncTask};
    use core::pin::pin;
    use core::time::Duration;

    #[test]
    fn async_mutex_nowait_test() {
        let shared_data = AsyncMutex::new(0_u32);

        let async_addition = async {
            shared_data.lock().await.action(|mut data| {
                *data += 2;
            });
        };

        block_on(async_addition);
        assert_eq!(shared_data.data.take(), 2);
    }

    #[test]
    fn async_mutex_wait_test() {
        let shared_data = AsyncMutex::new(0_u32);

        let async_addition1 = async {
            let sd = shared_data.lock().await;
            sleep_via_wait(Duration::from_secs(1)).await;
            sd.action(|mut data| {
                *data += 2;
            });
        };

        let async_addition2 = async {
            shared_data.lock().await.action(|mut data| {
                *data += 2;
            });
        };

        block_on(async {
            let async_addition1 = pin!(async_addition1);
            let async_addition2 = pin!(async_addition2);
            join_tasks([
                AsyncTask::new(async_addition1),
                AsyncTask::new(async_addition2),
            ])
            .await;
        });

        assert_eq!(shared_data.data.take(), 4);
    }
}
