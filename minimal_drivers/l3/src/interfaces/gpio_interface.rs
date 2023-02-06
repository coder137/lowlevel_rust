use crate::EnumToNum;

#[derive(PartialEq, Eq)]
pub enum GpioValue {
    Low,
    High,
}

impl EnumToNum for GpioValue {
    fn to_num(&self) -> u32 {
        match self {
            GpioValue::Low => 0x0,
            GpioValue::High => 0x1,
        }
    }
}

pub trait GpioOut {
    fn write(&mut self, value: GpioValue);
    fn set(&mut self) {
        self.write(GpioValue::High);
    }
    fn reset(&mut self) {
        self.write(GpioValue::Low);
    }
}

pub trait GpioIn {
    fn read(&self) -> GpioValue;
}
