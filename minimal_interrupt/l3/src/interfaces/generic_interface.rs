pub trait PeripheralConfiguration {
    type Config;
    type Register;

    fn configure(&self, configuration: &Self::Config) -> Self::Register;
}
