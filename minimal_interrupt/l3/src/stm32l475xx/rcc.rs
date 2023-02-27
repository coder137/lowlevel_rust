#![allow(non_camel_case_types)]

use l0::{get_port, read_register, write_register, RCC_TypeDef, RCC_BASE};
use l2::bitflags;

use crate::Singleton;

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
        const SYSCFGEN = 1 << 0;
        const USART1EN = 1 << 14;
        // TODO, Add more
    }
}

pub struct RCCRegister {
    port: &'static mut RCC_TypeDef,
}

impl RCCRegister {
    pub fn set_ahb2enr(&mut self, ahb2: RCC_AHB2ENR) {
        let mut ahb2enr_data = read_register!(self.port.AHB2ENR);
        ahb2enr_data |= ahb2.bits();
        write_register!(self.port.AHB2ENR, ahb2enr_data);
    }

    pub fn set_apb2enr(&mut self, apb2: RCC_APB2ENR) {
        let mut apb2enr_data = read_register!(self.port.APB2ENR);
        apb2enr_data |= apb2.bits();
        write_register!(self.port.APB2ENR, apb2enr_data);
    }
}

// Put functionality here i.e various valid configurations for your port
pub struct RCCPeripheral<const B: u32>;

impl<const B: u32> RCCPeripheral<B> {
    pub fn get_register(&self) -> RCCRegister {
        RCCRegister {
            port: get_port!(RCC_TypeDef, B),
        }
    }
}

// Create established ports here

type RCC = RCCPeripheral<RCC_BASE>;

pub static RCC_GLOBAL: Singleton<RCC> = Singleton::new(RCC {});
