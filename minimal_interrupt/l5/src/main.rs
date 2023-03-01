#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(unused_imports)]

#[cfg(not(test))]
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub fn spin_delay(delay: u32) {
    use core::arch::asm;

    let mut mdelay = delay;
    while mdelay != 0 {
        unsafe {
            asm!("nop");
        }
        mdelay -= 1;
    }
}

#[cfg(not(test))]
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[no_mangle]
fn main() -> ! {
    use core::{
        fmt::Write,
        ptr,
        sync::atomic::{AtomicBool, AtomicPtr, Ordering},
    };
    use l0::*;
    use l2::heapless::{spsc::Queue, String, Vec};
    use l3::*;
    use l4::*;

    // GPIOA Pin 5
    fn configure_gpio_output() -> impl GpioOut {
        let gpioa_peripheral = GPIOA_GLOBAL.take();
        // Configure GPIOA port and Pin 5 as output
        let gpio_out_at_pin5 = gpioa_peripheral.configure_for_output(5);
        gpio_out_at_pin5
    }

    // GPIOC Pin 13
    fn configure_gpio_input() -> impl GpioIn {
        let gpioc_peripheral = GPIOC_GLOBAL.take();
        // Configure GPIOC port and Pin 13 as input
        let gpio_in_at_pin13 = gpioc_peripheral.configure_for_input(13);
        gpio_in_at_pin13
    }

    // GPIOC Pin 13, Interrupt activation
    fn configure_gpio_input_interrupt() {
        // Configure SYSCFG port for pin 13
        // Select the GPIO pin which triggers this Interrupt
        let syscfg_port = SYSCFG_PORT::port();
        write_assign_register!(syscfg_port.EXTICR[3], |, (1 << 1) << 4);

        // Configure EXTI register for pin 13
        EXTI::get_register().configure_interrupt(13, EXTIType::FallingEdge);

        // Enable NVIC IRQ
        nvic::enable_irq(Interrupt::EXTI15_10);
    }

    // GPIOB Pin 6, 7
    fn configure_usart_rx_tx() -> impl UsartBufferedInOut {
        let gpiob_peripheral = GPIOB_GLOBAL.take();
        // Configure GPIOB port Pin 6 and Pin 7 for USART
        gpiob_peripheral.configure_for_usart(GPIOAlternate::AF7, 6);
        gpiob_peripheral.configure_for_usart(GPIOAlternate::AF7, 7);

        let usart1_rx_tx = USART1_GLOBAL
            .take()
            .configure_buffered_rx_tx(unsafe { &mut RX_BUF }, unsafe { &mut TX_BUF });
        usart1_rx_tx
    }

    fn configure_usart_rx_tx_interrupt() {
        nvic::enable_irq(Interrupt::USART1);
    }

    // Start
    let mut rcc_register = RCC_GLOBAL.take().get_register();
    // Activate clock control for GPIOA, GPIOB and GPIOC and USART1EN
    rcc_register.set_ahb2enr(RCC_AHB2ENR::GPIOAEN | RCC_AHB2ENR::GPIOBEN | RCC_AHB2ENR::GPIOCEN);
    rcc_register.set_apb2enr(RCC_APB2ENR::USART1EN | RCC_APB2ENR::SYSCFGEN);

    // LED module
    let mut gpio_output = configure_gpio_output();
    let mut led = Led::new(&mut gpio_output);

    // Button module
    static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
    configure_gpio_input();
    attach_interrupt_handler(Interrupt::EXTI15_10, || {
        let mut exti_register = EXTI::get_register();
        if exti_register.is_pending_interrupt(13) {
            exti_register.clear_pending_interrupt(13);
            BUTTON_PRESSED.store(true, Ordering::SeqCst);
        }
    });
    configure_gpio_input_interrupt();

    // USART
    let mut usart1_rx_tx = configure_usart_rx_tx();
    static mut RX_BUF: Queue<char, 128> = Queue::new();
    static mut TX_BUF: Queue<char, 128> = Queue::new();
    attach_interrupt_handler(Interrupt::USART1, || {
        let usart1_port = USART1_PORT::port();
        let isr_data = read_register!(usart1_port.ISR);
        const RXNE: u32 = 5;
        const TC: u32 = 6;
        const TXE: u32 = 7;
        const TXEIE: u32 = 7;
        if (isr_data >> RXNE) & 0x01 == 1 {
            // Read data
            let rdr_data = read_register!(usart1_port.RDR) as u8 as char;
            unsafe { RX_BUF.enqueue(rdr_data).unwrap() };
        }

        if (isr_data >> TXE) & 0x01 == 1 {
            unsafe {
                match TX_BUF.dequeue() {
                    Some(data) => {
                        write_register!(usart1_port.TDR, data as u16);
                    }
                    None => {
                        // Reset the CR1 TXEIE register
                        write_assign_register!(usart1_port.CR1, &, !(1 << TXEIE));
                    }
                }
            };
        }
    });
    configure_usart_rx_tx_interrupt();

    const TIME: u32 = 100_000;
    let mut counter = 0;
    let mut line_buffer: String<50> = String::new();
    loop {
        if let Ok(_) =
            BUTTON_PRESSED.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        {
            usart1_rx_tx.write_str("Button Pressed\r\n").unwrap();
        }

        while let Some(data) = usart1_rx_tx.try_read_character() {
            if data == '\r' || data == '\n' {
                usart1_rx_tx
                    .write_fmt(format_args!("W: {} {:#?}\r\n", line_buffer, line_buffer))
                    .unwrap();
                line_buffer.clear();
            } else {
                line_buffer.push(data).unwrap();
            }
        }

        // Can also use write! and writeln!
        led.on();
        usart1_rx_tx
            .write_fmt(format_args!("LED ON: {}\r\n", counter))
            .unwrap();
        spin_delay(TIME);

        led.off();
        usart1_rx_tx
            .write_fmt(format_args!("LED OFF: {}\r\n", counter))
            .unwrap();

        spin_delay(TIME);
        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_tests_work() {
        assert_eq!(1, 1);
    }
}
