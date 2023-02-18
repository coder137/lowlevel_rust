pub trait Port<T, const B: u32> {
    fn get_port() -> &'static mut T {
        let mutable_ref = unsafe { &mut *(B as *mut T) };
        mutable_ref
    }
}

pub trait PeripheralConfiguration {
    type Config;
    type Register;

    fn configure(&self, configuration: &Self::Config) -> Self::Register;
}
