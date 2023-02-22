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

#[macro_export]
macro_rules! atomicT {
    ($name:ident, $type:ty) => {
        static $name: core::sync::atomic::AtomicPtr<$type> =
            core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());
    }; // ($name:ident, $type:ty, $setter:ident, $getter:ident) => {
       //     atomicT!($name, $type);

       //     pub fn $setter(data: *mut $type) {
       //         store_atomicT(&$name, data);
       //     }

       //     pub fn $getter() -> Option<&mut $type> {
       //         load_atomicT::<$type>(&$name)
       //     }
       // };
}

pub fn load_atomicT<T>(var: &core::sync::atomic::AtomicPtr<T>) -> Option<&mut T> {
    let data = var.load(core::sync::atomic::Ordering::SeqCst);
    if data.is_null() {
        None
    } else {
        Some(unsafe { &mut *data })
    }
}

pub fn store_atomicT<T>(var: &core::sync::atomic::AtomicPtr<T>, data: *mut T) {
    var.store(data, core::sync::atomic::Ordering::SeqCst);
}
