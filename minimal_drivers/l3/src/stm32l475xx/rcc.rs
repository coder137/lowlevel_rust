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

pub struct RCC;

impl RCC {
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

impl Port<RCC_TypeDef, RCC_BASE> for RCC {}

// Create established ports here

pub static mut RCC_PERIPHERAL: Singleton<RCC> = Singleton::new(RCC {});
