use l0::get_port;

pub trait Port<T, const B: u32> {
    fn get_port() -> &'static mut T {
        get_port!(T, B)
    }
}

pub trait PeripheralConfiguration {
    type Config;
    type Register;

    fn configure(&self, configuration: &Self::Config) -> Self::Register;
}
