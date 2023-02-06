use crate::EnumToNum;

pub enum GpioValue {
    Off,
    On,
}

impl EnumToNum for GpioValue {
    fn to_num(&self) -> u32 {
        match self {
            GpioValue::Off => 0x0,
            GpioValue::On => 0x1,
        }
    }
}

pub trait GpioOut {
    fn write(&mut self, value: GpioValue);
    fn set(&mut self) {
        self.write(GpioValue::On);
    }
    fn reset(&mut self) {
        self.write(GpioValue::Off);
    }
}

pub trait GpioIn {
    fn read(&mut self) -> GpioValue;
}
