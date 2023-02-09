use core::{
    cell::{RefCell, RefMut},
    ops::{Deref, DerefMut},
};

pub trait Port<T, const B: u32> {
    fn get_port(&self) -> &'static mut T {
        let mutable_ref = unsafe { &mut *(B as *mut T) };
        mutable_ref
    }
}

pub struct Singleton<T> {
    data: RefCell<T>,
}

pub struct SingletonGuard<'a, T> {
    data: RefMut<'a, T>,
}

impl<'a, T> Deref for SingletonGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<'a, T> DerefMut for SingletonGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.deref_mut()
    }
}

// TODO, Check if more traits are required for SingletonGuard

impl<T> Singleton<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: RefCell::new(data),
        }
    }

    pub fn take(&self) -> SingletonGuard<T> {
        SingletonGuard {
            data: self.data.borrow_mut(),
        }
    }
}

unsafe impl<T> Sync for Singleton<T> {}

// TODO, Check if more traits are required for Singleton

#[cfg(test)]
mod tests {
    use super::*;

    struct Tester;

    impl Tester {
        fn tester(&self) {}
        fn tester_mut(&mut self) {}
    }

    #[test]
    fn singleton_local() {
        let singleton: Singleton<Tester> = Singleton::new(Tester {});

        // Non mutable
        {
            let lock = singleton.take();
            lock.tester();
        }

        // Mutable
        {
            let mut lock = singleton.take();
            lock.tester();
            lock.tester_mut();
        }
    }

    static TESTER: Singleton<Tester> = Singleton::new(Tester {});

    #[test]
    fn singleton_static() {
        // Non mutable
        {
            let lock = TESTER.take();
            lock.tester();
        }

        // Mutable
        {
            let mut lock = TESTER.take();
            lock.tester();
            lock.tester_mut();
        }
    }
}
