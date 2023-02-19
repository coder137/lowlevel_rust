#[macro_export]
macro_rules! get_port {
    ($register_map_struct:ident, $address:ident) => {
        unsafe { &mut *($address as *mut $register_map_struct) }
    };
}

#[macro_export]
macro_rules! read_register {
    ($register:expr) => {
        unsafe { read_volatile(&$register) }
    };
}

// TODO, Overload this macro to support $data:literal
#[macro_export]
macro_rules! write_register {
    ($register:expr, $data:ident) => {
        unsafe { write_volatile(&mut $register, $data) }
    };
}
