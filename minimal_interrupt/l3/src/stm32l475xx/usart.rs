#![allow(non_camel_case_types)]

use core::fmt::Write;

use l0::{get_port, get_system_clock, read_register, write_register, USART_TypeDef, USART1_BASE};
use l2::bitflags;

use crate::{PeripheralConfiguration, Singleton, UsartIn, UsartInOut};

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

pub struct USARTRegister {
    port: &'static mut USART_TypeDef,
}

impl USARTRegister {
    fn via_configure(&mut self, config: &USARTConfig) {
        // Disable USART
        self.reset_cr1(USART_CR1::UE);

        // Baud rate
        let system_clock = get_system_clock();
        let usartdiv = system_clock / config.baud_rate;
        write_register!(self.port.BRR, usartdiv);

        // Stop bits
        let mut cr2_data = read_register!(self.port.CR2);
        cr2_data &= (!USART_CR2::STOP).bits();
        cr2_data |= match config.stop_bit {
            USARTStopBit::Bit1_0 => 0 << 12,
            USARTStopBit::Bit0_5 => 1 << 12,
            USARTStopBit::Bit2_0 => 2 << 12,
            USARTStopBit::Bit1_5 => 3 << 12,
        };
        write_register!(self.port.CR2, cr2_data);

        // Set word length, usart mode and enable
        let mut cr1_data = read_register!(self.port.CR1);
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

        write_register!(self.port.CR1, cr1_data);
    }

    fn reset_cr1(&mut self, cr1: USART_CR1) {
        let mut cr1_data = read_register!(self.port.CR1);
        cr1_data &= !(cr1.bits());
        write_register!(self.port.CR1, cr1_data);
    }
}

impl UsartIn for USARTRegister {
    fn read_character(&mut self) -> char {
        const ISR_RXNE: u32 = 5;
        while (read_register!(self.port.ISR) & 1 << ISR_RXNE) == 0 {}
        let data = read_register!(self.port.RDR) as u8;
        data as char
    }
}

impl Write for USARTRegister {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bit_not_set = |bit: u32| {
            let isr_data = read_register!(self.port.ISR);
            isr_data & (1 << bit) == 0
        };

        const ISR_TXE: u32 = 7;
        const ISR_TC: u32 = 6;
        s.chars().for_each(|c| {
            while bit_not_set(ISR_TXE) {}
            write_register!(self.port.TDR, c as u16);
        });
        while bit_not_set(ISR_TC) {}
        Ok(())
    }
}

impl UsartInOut for USARTRegister {}

// Put functionality here i.e various valid configurations for your peripheral
pub struct USARTPeripheral<const B: u32> {}

impl<const B: u32> USARTPeripheral<B> {
    pub fn configure_default_rx(&self) -> impl UsartIn {
        self.configure(&USARTConfig {
            mode: USARTMode::RxOnly,
            word_length: USARTWordLength::Len8,
            stop_bit: USARTStopBit::Bit1_0,
            baud_rate: 115200,
        })
    }

    pub fn configure_default_tx(&self) -> impl Write {
        self.configure(&USARTConfig {
            mode: USARTMode::TxOnly,
            word_length: USARTWordLength::Len8,
            stop_bit: USARTStopBit::Bit1_0,
            baud_rate: 115200,
        })
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
    type Register = USARTRegister;

    fn configure(&self, configuration: &Self::Config) -> Self::Register {
        let mut usart = USARTRegister {
            port: get_port!(USART_TypeDef, B),
        };
        usart.via_configure(&configuration);
        usart
    }
}

// Create established ports here

type USART1 = USARTPeripheral<USART1_BASE>;

pub static USART1_GLOBAL: Singleton<USART1> = Singleton::new(USART1 {});
