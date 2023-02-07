#![allow(non_camel_case_types)]
use core::ptr::{read_volatile, write_volatile};

use l0::{RCC_TypeDef, RCC_BASE};

use crate::{EnumToNum, Port, Singleton};

pub enum RCC_AHB2ENR {
    GPIOAEN,
    GPIOBEN,
    GPIOCEN,
    GPIODEN,
    GPIOEEN,
    GPIOFEN,
    GPIOGEN,
    GPIOHEN,
    GPIOIEN,
}

impl EnumToNum for RCC_AHB2ENR {
    fn to_num(&self) -> u32 {
        match self {
            RCC_AHB2ENR::GPIOAEN => 0x0,
            RCC_AHB2ENR::GPIOBEN => 0x1,
            RCC_AHB2ENR::GPIOCEN => 0x2,
            RCC_AHB2ENR::GPIODEN => 0x3,
            RCC_AHB2ENR::GPIOEEN => 0x4,
            RCC_AHB2ENR::GPIOFEN => 0x5,
            RCC_AHB2ENR::GPIOGEN => 0x6,
            RCC_AHB2ENR::GPIOHEN => 0x7,
            RCC_AHB2ENR::GPIOIEN => 0x8,
        }
    }
}

// TODO, If required make a RCCRegister module

// Put functionality here i.e various valid configurations for your port
pub struct RCCPeripheral<const B: u32>;

impl<const B: u32> RCCPeripheral<B> {
    pub fn set_ahb2enr(&mut self, ahb2: RCC_AHB2ENR) {
        let mut ahb2enr = unsafe { read_volatile(&mut self.get_port().AHB2ENR) };
        ahb2enr |= 1 << ahb2.to_num();
        unsafe {
            write_volatile(&mut self.get_port().AHB2ENR, ahb2enr);
        }
    }

    pub fn reset_ahb2enr(&mut self, ahb2: RCC_AHB2ENR) {
        let mut ahb2enr = unsafe { read_volatile(&mut self.get_port().AHB2ENR) };
        ahb2enr &= !(1 << ahb2.to_num());
        unsafe { write_volatile(&mut self.get_port().AHB2ENR, ahb2enr) };
    }
}

impl<const B: u32> Port<RCC_TypeDef, B> for RCCPeripheral<B> {}

// Create established ports here

type RCC = RCCPeripheral<RCC_BASE>;

pub static mut RCC_PORT: Singleton<RCC> = Singleton::new(RCC {});
