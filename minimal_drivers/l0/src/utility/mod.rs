#[macro_export]
macro_rules! get_port {
    ($register_map_struct:ident, $address:ident) => {
        unsafe { &mut *($address as *mut $register_map_struct) }
    };
}

#[macro_export]
macro_rules! read_register {
    ($port:expr, $register_name:ident) => {
        unsafe { read_volatile(&$port.$register_name) }
    };
}

// TODO, Overload this macro to support $data:literal
#[macro_export]
macro_rules! write_register {
    ($port:expr, $register_name:ident, $data:ident) => {
        unsafe { write_volatile(&mut $port.$register_name, $data) }
    };
}
