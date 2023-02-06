#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

use crate::{EnumToNum, GpioIn, GpioOut, GpioValue, Peripheral};
use l0::{GPIO_TypeDef, GPIOA_BASE, GPIOB_BASE, GPIOC_BASE};

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
            GpioPin::Num0 => 0,
            GpioPin::Num1 => 1,
            GpioPin::Num2 => 2,
            GpioPin::Num3 => 3,
            GpioPin::Num4 => 4,
            GpioPin::Num5 => 5,
            GpioPin::Num6 => 6,
            GpioPin::Num7 => 7,
            GpioPin::Num8 => 8,
            GpioPin::Num9 => 9,
            GpioPin::Num10 => 10,
            GpioPin::Num11 => 11,
            GpioPin::Num12 => 12,
            GpioPin::Num13 => 13,
            GpioPin::Num14 => 14,
            GpioPin::Num15 => 15,
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

impl EnumToNum for GpioType {
    fn to_num(&self) -> u32 {
        match self {
            GpioType::PushPull => 0x0,
            GpioType::OpenDrain => 0x1,
        }
    }
}

enum GpioPull {
    NoPullupOrPulldown,
    Pullup,
    Pulldown,
}

impl EnumToNum for GpioPull {
    fn to_num(&self) -> u32 {
        match self {
            GpioPull::NoPullupOrPulldown => 0x0,
            GpioPull::Pullup => 0x1,
            GpioPull::Pulldown => 0x2,
        }
    }
}

enum GpioSpeed {
    LowSpeed,
    MediumSpeed,
    HighSpeed,
    VeryHighSpeed,
}

impl EnumToNum for GpioSpeed {
    fn to_num(&self) -> u32 {
        match self {
            GpioSpeed::LowSpeed => 0x0,
            GpioSpeed::MediumSpeed => 0x1,
            GpioSpeed::HighSpeed => 0x2,
            GpioSpeed::VeryHighSpeed => 0x3,
        }
    }
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

    pub fn configure_as_input(port: &'static mut GPIO_TypeDef, pin: GpioPin) -> Self {
        let mut gpio = Self {
            port,
            pin,
            mode: GpioMode::Input,
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
        typ |= self.typ.to_num() << self.pin.to_num();
        unsafe { write_volatile(&mut self.port.OTYPER, typ) };
    }

    fn set_ospeedr(&mut self) {
        let mut speed = unsafe { read_volatile(&mut self.port.OSPEEDR) };
        speed &= !(0x3 << self.pin.to_num() * 2); // clear speed register
        speed |= self.speed.to_num() << self.pin.to_num() * 2;
        unsafe { write_volatile(&mut self.port.OSPEEDR, speed) };
    }

    fn set_pupdr(&mut self) {
        let mut pu_pd = unsafe { read_volatile(&mut self.port.PUPDR) };
        pu_pd &= !(0x3 << self.pin.to_num() * 2);
        pu_pd |= self.pull.to_num() << self.pin.to_num() * 2;
        unsafe { write_volatile(&mut self.port.PUPDR, pu_pd) };
    }

    fn set_odr(&mut self, value: GpioValue) {
        let mut odr = unsafe { read_volatile(&mut self.port.ODR) };
        odr &= !(0x1 << self.pin.to_num());
        odr |= value.to_num() << self.pin.to_num();
        unsafe { write_volatile(&mut self.port.ODR, odr) };
    }

    fn set_bsrr(&mut self) {
        let mut bsrr = unsafe { read_volatile(&mut self.port.BSRR) };
        bsrr |= 1 << self.pin.to_num();
        unsafe { write_volatile(&mut self.port.BSRR, bsrr) };
    }

    fn set_brr(&mut self) {
        let mut brr = unsafe { read_volatile(&mut self.port.BRR) };
        brr |= 1 << self.pin.to_num();
        unsafe { write_volatile(&mut self.port.BRR, brr) };
    }
}

impl GpioOut for GPIO {
    fn write(&mut self, value: GpioValue) {
        match value {
            GpioValue::Low => self.set_brr(),
            GpioValue::High => self.set_bsrr(),
        }
    }
}

impl GpioIn for GPIO {
    fn read(&self) -> GpioValue {
        let idr = unsafe { read_volatile(&self.port.IDR) };
        let value = (idr >> self.pin.to_num()) & 0x01;
        let value = match value {
            0x0 => GpioValue::Low,
            0x1 => GpioValue::High,
            _ => unreachable!(),
        };
        value
    }
}

// Create established ports here
pub struct GPIOA_Port;
impl Peripheral<GPIO_TypeDef, GPIOA_BASE> for GPIOA_Port {}

pub struct GPIOB_Port;
impl Peripheral<GPIO_TypeDef, GPIOB_BASE> for GPIOB_Port {}

pub struct GPIOC_Port;
impl Peripheral<GPIO_TypeDef, GPIOC_BASE> for GPIOC_Port {}

// TODO, Do this for other GPIO modules as well
