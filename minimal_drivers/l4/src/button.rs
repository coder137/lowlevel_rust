use l3::{GpioIn, GpioValue};

pub struct Button<'a> {
    gpio: &'a mut dyn GpioIn,
    default: GpioValue, // Default value when button is not pressed
}

impl<'a> Button<'a> {
    pub fn new(gpio: &'a mut dyn GpioIn, default: GpioValue) -> Self {
        Self { gpio, default }
    }

    pub fn pressed(&self) -> bool {
        let mut pressed = false;
        if self.gpio.read() != self.default {
            pressed = true;
        }
        pressed
    }
}
