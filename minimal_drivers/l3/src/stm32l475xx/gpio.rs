#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::{EnumToNum, GpioOut, GpioValue, Peripheral, RCC};
use core::ptr::{read_volatile, write_volatile};
use l0::{GPIO_TypeDef, GPIOA_BASE};

pub enum GpioPin {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num10,
    Num11,
    Num12,
    Num13,
    Num14,
    Num15,
}

impl EnumToNum for GpioPin {
    fn to_num(&self) -> u32 {
        match self {
            GpioPin::Num0 => 0x0,
            GpioPin::Num1 => 0x1,
            GpioPin::Num2 => 0x2,
            GpioPin::Num3 => 0x3,
            GpioPin::Num4 => 0x4,
            GpioPin::Num5 => 0x5,
            GpioPin::Num6 => 0x6,
            GpioPin::Num7 => 0x7,
            GpioPin::Num8 => 0x8,
            GpioPin::Num9 => 0x9,
            GpioPin::Num10 => 0x10,
            GpioPin::Num11 => 0x11,
            GpioPin::Num12 => 0x12,
            GpioPin::Num13 => 0x13,
            GpioPin::Num14 => 0x14,
            GpioPin::Num15 => 0x15,
        }
    }
}

enum GpioMode {
    Input,
    Output,
    AlternateFunction,
    AnalogMode,
}

impl EnumToNum for GpioMode {
    fn to_num(&self) -> u32 {
        match self {
            GpioMode::Input => 0x0,
            GpioMode::Output => 0x1,
            GpioMode::AlternateFunction => 0x2,
            GpioMode::AnalogMode => 0x3,
        }
    }
}

enum GpioType {
    PushPull,
    OpenDrain,
}

enum GpioPull {
    NoPullupOrPulldown,
    Pullup,
    Pulldown,
}

enum GpioSpeed {
    LowSpeed,
    MediumSpeed,
    HighSpeed,
    VeryHighSpeed,
}

pub struct GPIO {
    port: &'static mut GPIO_TypeDef,
    pin: GpioPin,

    mode: GpioMode,
    typ: GpioType,
    pull: GpioPull,
    speed: GpioSpeed,
}

impl GPIO {
    // TODO, Make sure a new API is different from the init API
    pub fn configure_as_output(port: &'static mut GPIO_TypeDef, pin: GpioPin) -> Self {
        let mut gpio = Self {
            port,
            pin,
            mode: GpioMode::Output,
            typ: GpioType::PushPull,
            pull: GpioPull::NoPullupOrPulldown,
            speed: GpioSpeed::LowSpeed,
        };
        gpio.configure();
        gpio
    }

    fn configure(&mut self) {
        self.set_moder();
        self.set_otyper();
        self.set_ospeedr();
        self.set_pupdr();
    }

    /// Sets the mode according to the pin value
    fn set_moder(&mut self) {
        let mut mode = unsafe { read_volatile(&mut self.port.MODER) };
        mode &= !(0x3 << self.pin.to_num() * 2); // clear mode register
        mode |= self.mode.to_num() << self.pin.to_num() * 2;
        unsafe { write_volatile(&mut self.port.MODER, mode) };
    }

    fn set_otyper(&mut self) {
        let mut typ = unsafe { read_volatile(&mut self.port.OTYPER) };
        typ &= !(0x1 << self.pin.to_num()); // clear type register

        // TODO, Set required value here
        unsafe { write_volatile(&mut self.port.OTYPER, typ) };
    }

    fn set_ospeedr(&mut self) {
        let mut speed = unsafe { read_volatile(&mut self.port.OSPEEDR) };
        speed &= !(0x3 << self.pin.to_num() * 2); // clear speed register

        // TODO, Set required value here
        unsafe { write_volatile(&mut self.port.OSPEEDR, speed) };
    }

    fn set_pupdr(&mut self) {
        let mut pu_pd = unsafe { read_volatile(&mut self.port.PUPDR) };
        pu_pd &= !(0x3 << self.pin.to_num() * 2);

        // TODO, Set required value here
        unsafe { write_volatile(&mut self.port.PUPDR, pu_pd) };
    }

    fn set_odr(&mut self, value: GpioValue) {
        let mut odr = unsafe { read_volatile(&mut self.port.ODR) };
        odr &= !(0x1 << self.pin.to_num());
        odr |= value.to_num() << self.pin.to_num();
        unsafe { write_volatile(&mut self.port.ODR, odr) };
    }

    fn set_bsrr(&mut self) {
        let mut bsrr = unsafe { core::ptr::read_volatile(&mut self.port.BSRR) };
        bsrr |= 1 << self.pin.to_num();
        unsafe { core::ptr::write_volatile(&mut self.port.BSRR, bsrr) };
    }

    fn set_brr(&mut self) {
        let mut brr = unsafe { core::ptr::read_volatile(&mut self.port.BRR) };
        brr |= 1 << self.pin.to_num();
        unsafe { core::ptr::write_volatile(&mut self.port.BRR, brr) };
    }
}

impl GpioOut for GPIO {
    fn write(&mut self, value: GpioValue) {
        match value {
            GpioValue::Off => self.set_brr(),
            GpioValue::On => self.set_bsrr(),
        }
    }
}

// Create established ports here
pub struct GPIOA_Port;
impl Peripheral<GPIO_TypeDef, GPIOA_BASE> for GPIOA_Port {}

// TODO, Do this for other GPIO modules as well
