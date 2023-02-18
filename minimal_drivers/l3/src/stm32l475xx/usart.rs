// TODO, Remove this later
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use core::ptr::{read_volatile, write_volatile};

use l0::{
    controller::{USART_TypeDef, USART1_BASE},
    get_system_clock,
};
use l2::bitflags;

use crate::{PeripheralConfiguration, Port, Singleton, UsartIn, UsartInOut, UsartOut};

bitflags! {
    pub struct USART_CR1 : u32 {
        const UE = 1 << 0;
        const RE = 1 << 2;
        const TE = 1 << 3;
        const M0 = 1 << 12;
        const OVER8 = 1 << 15;
        const M1 = 1 << 28;
    }
}

bitflags! {
    pub struct USART_CR2 : u32 {
        const STOP = 3 << 12;
    }
}

pub enum USARTWordLength {
    Len8,
    Len9,
    Len7,
}

pub enum USARTStopBit {
    Bit1_0,
    Bit0_5,
    Bit2_0,
    Bit1_5,
}

pub enum USARTMode {
    Inactive,
    RxOnly,
    TxOnly,
    RxTx,
}

pub struct USARTRegister<const B: u32> {}

impl<const B: u32> USARTRegister<B> {
    fn via_configure(&mut self, config: &USARTConfig) {
        // Disable USART
        self.reset_cr1(USART_CR1::UE);

        // Baud rate
        let system_clock = get_system_clock();
        let usartdiv = system_clock / config.baud_rate;
        unsafe { write_volatile(&mut Self::get_port().BRR, usartdiv) };

        // Stop bits
        let mut cr2_data = unsafe { read_volatile(&Self::get_port().CR2) };
        cr2_data &= (!USART_CR2::STOP).bits();
        unsafe { write_volatile(&mut Self::get_port().CR2, cr2_data) };

        // Set word length, usart mode and enable
        let mut cr1_data = unsafe { read_volatile(&Self::get_port().CR1) };
        cr1_data &= (!USART_CR1::all()).bits();

        // Set word length
        cr1_data |= match config.word_length {
            USARTWordLength::Len8 => 0,
            USARTWordLength::Len9 => USART_CR1::M0.bits(),
            USARTWordLength::Len7 => USART_CR1::M1.bits(),
        };

        // Set Mode
        cr1_data |= match config.mode {
            USARTMode::Inactive => 0,
            USARTMode::RxOnly => USART_CR1::RE.bits(),
            USARTMode::TxOnly => USART_CR1::TE.bits(),
            USARTMode::RxTx => (USART_CR1::RE | USART_CR1::TE).bits(),
        };

        // Enable
        cr1_data |= USART_CR1::UE.bits();

        unsafe { write_volatile(&mut Self::get_port().CR1, cr1_data) };
    }

    fn reset_cr1(&mut self, cr1: USART_CR1) {
        let mut cr1_data = unsafe { read_volatile(&Self::get_port().CR1) };
        cr1_data &= !(cr1.bits());
        unsafe { write_volatile(&mut Self::get_port().CR1, cr1_data) };
    }
}

impl<const B: u32> Port<USART_TypeDef, B> for USARTRegister<B> {}

impl<const B: u32> UsartIn for USARTRegister<B> {
    fn read_character(&mut self) -> char {
        todo!()
    }
}

impl<const B: u32> UsartOut for USARTRegister<B> {
    fn write_character(&mut self, data: char) {
        let is_bit_set = |bit: u32| {
            let isr_data = unsafe { read_volatile(&Self::get_port().ISR) };
            isr_data & (1 << bit) == 0
        };

        while is_bit_set(7) {}
        unsafe { write_volatile(&mut Self::get_port().TDR, data as u16) };
        while is_bit_set(6) {}
    }
}

impl<const B: u32> UsartInOut for USARTRegister<B> {}

pub struct USARTPeripheral<const B: u32> {}

impl<const B: u32> USARTPeripheral<B> {
    pub fn configure_as_rx(&self) -> impl UsartIn {
        let usart = USARTRegister::<B> {};
        usart
    }

    pub fn configure_as_tx(&self) -> impl UsartOut {
        let usart = USARTRegister::<B> {};
        usart
    }

    pub fn configure_default_rx_tx(&self) -> impl UsartInOut {
        self.configure(&USARTConfig {
            mode: USARTMode::RxTx,
            word_length: USARTWordLength::Len8,
            stop_bit: USARTStopBit::Bit1_0,
            baud_rate: 115200,
        })
    }
}

pub struct USARTConfig {
    mode: USARTMode,
    word_length: USARTWordLength,
    stop_bit: USARTStopBit,
    baud_rate: u32,
}

impl<const B: u32> PeripheralConfiguration for USARTPeripheral<B> {
    type Config = USARTConfig;
    type Register = USARTRegister<B>;

    fn configure(&self, configuration: &Self::Config) -> Self::Register {
        let mut usart = USARTRegister::<B> {};
        usart.via_configure(&configuration);
        usart
    }
}

type USART1 = USARTPeripheral<USART1_BASE>;

pub static USART1_PORT: Singleton<USART1> = Singleton::new(USART1 {});
