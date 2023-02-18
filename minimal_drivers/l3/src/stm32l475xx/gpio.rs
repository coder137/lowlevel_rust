#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

use crate::{EnumToNum, GpioIn, GpioOut, GpioValue, PeripheralConfiguration};
use crate::{Port, Singleton};
use l0::controller::*;

pub enum GPIOMode {
    Input,
    Output,
    AlternateFunction,
    AnalogMode,
}

impl EnumToNum for GPIOMode {
    fn to_num(&self) -> u32 {
        match self {
            GPIOMode::Input => 0x0,
            GPIOMode::Output => 0x1,
            GPIOMode::AlternateFunction => 0x2,
            GPIOMode::AnalogMode => 0x3,
        }
    }
}

pub enum GPIOType {
    PushPull,
    OpenDrain,
}

impl EnumToNum for GPIOType {
    fn to_num(&self) -> u32 {
        match self {
            GPIOType::PushPull => 0x0,
            GPIOType::OpenDrain => 0x1,
        }
    }
}

pub enum GPIOPull {
    NoPullupOrPulldown,
    Pullup,
    Pulldown,
}

impl EnumToNum for GPIOPull {
    fn to_num(&self) -> u32 {
        match self {
            GPIOPull::NoPullupOrPulldown => 0x0,
            GPIOPull::Pullup => 0x1,
            GPIOPull::Pulldown => 0x2,
        }
    }
}

pub enum GPIOSpeed {
    LowSpeed,
    MediumSpeed,
    HighSpeed,
    VeryHighSpeed,
}

impl EnumToNum for GPIOSpeed {
    fn to_num(&self) -> u32 {
        match self {
            GPIOSpeed::LowSpeed => 0x0,
            GPIOSpeed::MediumSpeed => 0x1,
            GPIOSpeed::HighSpeed => 0x2,
            GPIOSpeed::VeryHighSpeed => 0x3,
        }
    }
}

pub enum GPIOAlternate {
    AF0,
    AF1,
    AF2,
    AF3,
    AF4,
    AF5,
    AF6,
    AF7,
    AF8,
    AF9,
    AF10,
    AF11,
    AF12,
    AF13,
    AF14,
    AF15,
}

impl EnumToNum for GPIOAlternate {
    fn to_num(&self) -> u32 {
        match self {
            GPIOAlternate::AF0 => 0,
            GPIOAlternate::AF1 => 1,
            GPIOAlternate::AF2 => 2,
            GPIOAlternate::AF3 => 3,
            GPIOAlternate::AF4 => 4,
            GPIOAlternate::AF5 => 5,
            GPIOAlternate::AF6 => 6,
            GPIOAlternate::AF7 => 7,
            GPIOAlternate::AF8 => 8,
            GPIOAlternate::AF9 => 9,
            GPIOAlternate::AF10 => 10,
            GPIOAlternate::AF11 => 11,
            GPIOAlternate::AF12 => 12,
            GPIOAlternate::AF13 => 13,
            GPIOAlternate::AF14 => 14,
            GPIOAlternate::AF15 => 15,
        }
    }
}

pub struct GPIORegister<const B: u32> {
    pin: u32,
}

impl<const B: u32> Port<GPIO_TypeDef, B> for GPIORegister<B> {}

impl<const B: u32> GPIORegister<B> {
    fn via_config(&mut self, config: &GPIOConfig) {
        self.set_moder(&config.moder);
        self.set_otyper(&config.otyper);
        self.set_pupdr(&config.pupdr);
        self.set_ospeedr(&config.ospeedr);
        self.set_afr(&config.afr);
    }

    fn set_moder(&mut self, moder: &GPIOMode) {
        let mut moder_data = unsafe { read_volatile(&Self::get_port().MODER) };
        moder_data &= !(0x3 << self.pin * 2); // clear mode register
        moder_data |= moder.to_num() << self.pin * 2;
        unsafe { write_volatile(&mut Self::get_port().MODER, moder_data) };
    }

    fn set_otyper(&mut self, otyper: &GPIOType) {
        let mut otyper_data = unsafe { read_volatile(&Self::get_port().OTYPER) };
        otyper_data &= !(0x1 << self.pin); // clear type register
        otyper_data |= otyper.to_num() << self.pin;
        unsafe { write_volatile(&mut Self::get_port().OTYPER, otyper_data) };
    }

    fn set_ospeedr(&mut self, ospeedr: &GPIOSpeed) {
        let mut ospeedr_data = unsafe { read_volatile(&Self::get_port().OSPEEDR) };
        ospeedr_data &= !(0x3 << self.pin * 2); // clear ospeedr register
        ospeedr_data |= ospeedr.to_num() << self.pin * 2;
        unsafe { write_volatile(&mut Self::get_port().OSPEEDR, ospeedr_data) };
    }

    fn set_pupdr(&mut self, pupdr: &GPIOPull) {
        let mut pupdr_data = unsafe { read_volatile(&Self::get_port().PUPDR) };
        pupdr_data &= !(0x3 << self.pin * 2);
        pupdr_data |= pupdr.to_num() << self.pin * 2;
        unsafe { write_volatile(&mut Self::get_port().PUPDR, pupdr_data) };
    }

    fn set_afr(&mut self, afr: &GPIOAlternate) {
        let (register, pin) = if self.pin > 7 {
            // Use AFRH
            (&mut Self::get_port().AFR[1], self.pin - 7)
        } else {
            // Use AFRL
            (&mut Self::get_port().AFR[0], self.pin)
        };

        let mut afr_data = unsafe { read_volatile(register) };
        afr_data &= !(0xF << (pin << 2));
        afr_data |= afr.to_num() << (pin << 2);
        unsafe { write_volatile(register, afr_data) };
    }

    fn set_bsrr(&mut self) {
        let mut bsrr_data = unsafe { read_volatile(&Self::get_port().BSRR) };
        bsrr_data |= 1 << self.pin;
        unsafe { write_volatile(&mut Self::get_port().BSRR, bsrr_data) };
    }

    fn set_brr(&mut self) {
        let mut brr_data = unsafe { read_volatile(&Self::get_port().BRR) };
        brr_data |= 1 << self.pin;
        unsafe { write_volatile(&mut Self::get_port().BRR, brr_data) };
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
        let idr = unsafe { read_volatile(&Self::get_port().IDR) };
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
    pub fn configure_for_output(&self, pin: u32) -> impl GpioOut {
        let config = GPIOConfig {
            pin,
            moder: GPIOMode::Output,
            otyper: GPIOType::PushPull,
            pupdr: GPIOPull::NoPullupOrPulldown,
            ospeedr: GPIOSpeed::LowSpeed,
            afr: GPIOAlternate::AF0,
        };
        self.configure(&config)
    }

    pub fn configure_for_input(&self, pin: u32) -> impl GpioIn {
        let config = GPIOConfig {
            pin,
            moder: GPIOMode::Input,
            otyper: GPIOType::PushPull,
            pupdr: GPIOPull::NoPullupOrPulldown,
            ospeedr: GPIOSpeed::LowSpeed,
            afr: GPIOAlternate::AF0,
        };
        self.configure(&config)
    }

    pub fn configure_for_usart(&self, afr: GPIOAlternate, pin: u32) -> GPIORegister<B> {
        let config = GPIOConfig {
            pin,
            moder: GPIOMode::AlternateFunction,
            otyper: GPIOType::PushPull,
            pupdr: GPIOPull::NoPullupOrPulldown,
            ospeedr: GPIOSpeed::VeryHighSpeed,
            afr,
        };
        self.configure(&config)
    }
}

pub struct GPIOConfig {
    pin: u32,
    moder: GPIOMode,
    otyper: GPIOType,
    pupdr: GPIOPull,
    ospeedr: GPIOSpeed,
    afr: GPIOAlternate,
}

impl<const B: u32> PeripheralConfiguration for GPIOPeripheral<B> {
    type Config = GPIOConfig;
    type Register = GPIORegister<B>;

    fn configure(&self, configuration: &Self::Config) -> Self::Register {
        let mut gpio = GPIORegister::<B> {
            pin: configuration.pin,
        };
        gpio.via_config(&configuration);
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

pub static GPIOA_PORT: Singleton<GPIOA> = Singleton::new(GPIOA {});
pub static GPIOB_PORT: Singleton<GPIOB> = Singleton::new(GPIOB {});
pub static GPIOC_PORT: Singleton<GPIOC> = Singleton::new(GPIOC {});
pub static GPIOD_PORT: Singleton<GPIOD> = Singleton::new(GPIOD {});
pub static GPIOE_PORT: Singleton<GPIOE> = Singleton::new(GPIOE {});
pub static GPIOF_PORT: Singleton<GPIOF> = Singleton::new(GPIOF {});
pub static GPIOG_PORT: Singleton<GPIOG> = Singleton::new(GPIOG {});
pub static GPIOH_PORT: Singleton<GPIOH> = Singleton::new(GPIOH {});
