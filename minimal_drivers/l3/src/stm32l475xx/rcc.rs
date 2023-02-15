#![allow(non_camel_case_types)]

use core::ptr::{read_volatile, write_volatile};

use l0::{RCC_TypeDef, RCC_BASE};
use l2::bitflags;

use crate::{Port, Singleton};

bitflags! {
    pub struct RCC_AHB2ENR : u32 {
        const GPIOAEN = 1 << 0;
        const GPIOBEN = 1 << 1;
        const GPIOCEN = 1 << 2;
        const GPIODEN = 1 << 3;
        const GPIOEEN = 1 << 4;
        const GPIOFEN = 1 << 5;
        const GPIOGEN = 1 << 6;
        const GPIOHEN = 1 << 7;
        const GPIOIEN = 1 << 8;
        // TODO, Add more
    }
}

bitflags! {
    pub struct RCC_APB2ENR : u32 {
        const USART1EN = 1 << 14;
        // TODO, Add more
    }
}

// Put functionality here i.e various valid configurations for your port
pub struct RCCPeripheral<const B: u32>;

impl<const B: u32> RCCPeripheral<B> {
    pub fn set_ahb2enr(&mut self, ahb2: RCC_AHB2ENR) {
        let mut ahb2enr = unsafe { read_volatile(&mut self.get_port().AHB2ENR) };
        ahb2enr |= ahb2.bits();
        unsafe {
            write_volatile(&mut self.get_port().AHB2ENR, ahb2enr);
        }
    }

    pub fn set_apb2enr(&mut self, apb2: RCC_APB2ENR) {
        let mut apb2enr = unsafe { read_volatile(&mut self.get_port().AHB2ENR) };
        apb2enr |= apb2.bits();
        unsafe {
            write_volatile(&mut self.get_port().AHB2ENR, apb2enr);
        }
    }
}

impl<const B: u32> Port<RCC_TypeDef, B> for RCCPeripheral<B> {}

// Create established ports here

type RCC = RCCPeripheral<RCC_BASE>;

pub static RCC_PORT: Singleton<RCC> = Singleton::new(RCC {});
