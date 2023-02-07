use core::mem::replace;

pub trait Port<T, const B: u32> {
    fn get_port(&self) -> &'static mut T {
        let mutable_ref = unsafe { &mut *(B as *mut T) };
        mutable_ref
    }
}

pub struct Singleton<T> {
    data: Option<T>,
}

impl<T> Singleton<T> {
    pub const fn new(data: T) -> Self {
        Self { data: Some(data) }
    }

    pub fn take(&mut self) -> T {
        let data = replace(&mut self.data, None);
        data.unwrap()
    }
}

#[macro_export]
macro_rules! get_peripheral {
    ($x:expr) => {{
        unsafe { $x.take() }
    }};
}
