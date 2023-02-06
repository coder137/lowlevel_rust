use l3::GpioOut;

pub struct Led<'a> {
    pub gpio: &'a mut dyn GpioOut,
}

impl<'a> Led<'a> {
    pub fn new(gpio: &'a mut dyn GpioOut) -> Self {
        Self { gpio }
    }

    pub fn on(&mut self) {
        self.gpio.set();
    }

    pub fn off(&mut self) {
        self.gpio.reset();
    }
}
