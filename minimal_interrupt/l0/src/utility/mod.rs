use core::marker::PhantomData;

#[macro_export]
macro_rules! get_port {
    ($register_map_struct:ident, $address:ident) => {
        unsafe { &mut *($address as *mut $register_map_struct) }
    };
}

#[macro_export]
macro_rules! read_register {
    ($register:expr) => {
        unsafe { core::ptr::read_volatile(&$register) }
    };
}

// TODO, Overload this macro to support $data:literal
#[macro_export]
macro_rules! write_register {
    ($register:expr, $data:expr) => {
        unsafe { core::ptr::write_volatile(&mut $register, $data) }
    };
}

#[macro_export]
macro_rules! write_assign_register {
    ($register:expr, $operation:tt, $data:expr) => {
        let read_data = read_register!($register);
        write_register!($register, read_data $operation $data);
    };
}

pub struct Port<T: 'static, const B: u32> {
    pub marker: PhantomData<T>,
}

impl<T: 'static, const B: u32> Port<T, B> {
    pub fn port() -> &'static mut T {
        unsafe { &mut *(B as *mut T) }
    }
}
