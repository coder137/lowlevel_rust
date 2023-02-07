#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

use crate::{EnumToNum, GpioIn, GpioOut, GpioValue, Port, Singleton};
use l0::{
    GPIO_TypeDef, GPIOA_BASE, GPIOB_BASE, GPIOC_BASE, GPIOD_BASE, GPIOE_BASE, GPIOF_BASE,
    GPIOG_BASE, GPIOH_BASE,
};

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

struct GPIORegister<const B: u32> {
    pin: u32,
    mode: GpioMode,
    typ: GpioType,
    pull: GpioPull,
    speed: GpioSpeed,
}

impl<const B: u32> Port<GPIO_TypeDef, B> for GPIORegister<B> {}

impl<const B: u32> GPIORegister<B> {
    pub fn configure(&mut self) {
        self.set_moder();
        self.set_otyper();
        self.set_ospeedr();
        self.set_pupdr();
    }

    /// Sets the mode according to the pin value
    fn set_moder(&mut self) {
        let mut moder_data = unsafe { read_volatile(&self.get_port().MODER) };
        moder_data &= !(0x3 << self.pin * 2); // clear mode register
        moder_data |= self.mode.to_num() << self.pin * 2;
        unsafe { write_volatile(&mut self.get_port().MODER, moder_data) };
    }

    fn set_otyper(&mut self) {
        let mut otyper_data = unsafe { read_volatile(&self.get_port().OTYPER) };
        otyper_data &= !(0x1 << self.pin); // clear type register
        otyper_data |= self.typ.to_num() << self.pin;
        unsafe { write_volatile(&mut self.get_port().OTYPER, otyper_data) };
    }

    fn set_ospeedr(&mut self) {
        let mut ospeedr_data = unsafe { read_volatile(&self.get_port().OSPEEDR) };
        ospeedr_data &= !(0x3 << self.pin * 2); // clear speed register
        ospeedr_data |= self.speed.to_num() << self.pin * 2;
        unsafe { write_volatile(&mut self.get_port().OSPEEDR, ospeedr_data) };
    }

    fn set_pupdr(&mut self) {
        let mut pu_pd = unsafe { read_volatile(&self.get_port().PUPDR) };
        pu_pd &= !(0x3 << self.pin * 2);
        pu_pd |= self.pull.to_num() << self.pin * 2;
        unsafe { write_volatile(&mut self.get_port().PUPDR, pu_pd) };
    }

    fn set_bsrr(&mut self) {
        let mut bsrr = unsafe { read_volatile(&self.get_port().BSRR) };
        bsrr |= 1 << self.pin;
        unsafe { write_volatile(&mut self.get_port().BSRR, bsrr) };
    }

    fn set_brr(&mut self) {
        let mut brr = unsafe { read_volatile(&self.get_port().BRR) };
        brr |= 1 << self.pin;
        unsafe { write_volatile(&mut self.get_port().BRR, brr) };
    }
}

impl<const B: u32> GpioOut for GPIORegister<B> {
    fn write(&mut self, value: GpioValue) {
        match value {
            GpioValue::Low => self.set_brr(),
            GpioValue::High => self.set_bsrr(),
        }
    }
}

impl<const B: u32> GpioIn for GPIORegister<B> {
    fn read(&self) -> GpioValue {
        let idr = unsafe { read_volatile(&self.get_port().IDR) };
        let value = (idr >> self.pin) & 0x01;
        let value = match value {
            0x0 => GpioValue::Low,
            0x1 => GpioValue::High,
            _ => unreachable!(),
        };
        value
    }
}

// Put functionality here i.e various valid configurations for your peripheral
pub struct GPIOPeripheral<const B: u32>;

impl<const B: u32> GPIOPeripheral<B> {
    pub fn configure_as_output(&self, pin: u32) -> impl GpioOut {
        let mut gpio = GPIORegister::<B> {
            pin,
            mode: GpioMode::Output,
            typ: GpioType::PushPull,
            pull: GpioPull::NoPullupOrPulldown,
            speed: GpioSpeed::LowSpeed,
        };
        gpio.configure();
        gpio
    }

    pub fn configure_as_input(&self, pin: u32) -> impl GpioIn {
        let mut gpio = GPIORegister::<B> {
            pin,
            mode: GpioMode::Input,
            typ: GpioType::PushPull,
            pull: GpioPull::NoPullupOrPulldown,
            speed: GpioSpeed::LowSpeed,
        };
        gpio.configure();
        gpio
    }
}

// Create established ports here

type GPIOA = GPIOPeripheral<GPIOA_BASE>;
type GPIOB = GPIOPeripheral<GPIOB_BASE>;
type GPIOC = GPIOPeripheral<GPIOC_BASE>;
type GPIOD = GPIOPeripheral<GPIOD_BASE>;
type GPIOE = GPIOPeripheral<GPIOE_BASE>;
type GPIOF = GPIOPeripheral<GPIOF_BASE>;
type GPIOG = GPIOPeripheral<GPIOG_BASE>;
type GPIOH = GPIOPeripheral<GPIOH_BASE>;

pub static mut GPIOA_PORT: Singleton<GPIOA> = Singleton::new(GPIOA {});
pub static mut GPIOB_PORT: Singleton<GPIOB> = Singleton::new(GPIOB {});
pub static mut GPIOC_PORT: Singleton<GPIOC> = Singleton::new(GPIOC {});
pub static mut GPIOD_PORT: Singleton<GPIOD> = Singleton::new(GPIOD {});
pub static mut GPIOE_PORT: Singleton<GPIOE> = Singleton::new(GPIOE {});
pub static mut GPIOF_PORT: Singleton<GPIOF> = Singleton::new(GPIOF {});
pub static mut GPIOG_PORT: Singleton<GPIOG> = Singleton::new(GPIOG {});
pub static mut GPIOH_PORT: Singleton<GPIOH> = Singleton::new(GPIOH {});
