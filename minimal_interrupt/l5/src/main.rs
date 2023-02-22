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
    use l0::{IRQn_Type::EXTI15_10_IRQn, *};
    use l3::*;
    use l4::*;

    // GPIOA Pin 5
    fn configure_gpio_output() -> impl GpioOut {
        let gpioa_peripheral = GPIOA_PORT.take();
        // Configure GPIOA port and Pin 5 as output
        let gpio_out_at_pin5 = gpioa_peripheral.configure_for_output(5);
        gpio_out_at_pin5
    }

    // GPIOC Pin 13
    fn configure_gpio_input() -> impl GpioIn {
        let gpioc_peripheral = GPIOC_PORT.take();
        // Configure GPIOC port and Pin 13 as input
        let gpio_in_at_pin13 = gpioc_peripheral.configure_for_input(13);
        gpio_in_at_pin13
    }

    // GPIOC Pin 13, Interrupt activation
    fn configure_gpio_input_interrupt() {
        // Configure SYSCFG port for pin 13
        let syscfg_port = get_port!(SYSCFG_TypeDef, SYSCFG_BASE);
        write_assign_register!(syscfg_port.EXTICR[3], |, (1 << 1) << 4);

        // Configure EXTI register for pin 13
        // Configure IMR and set the RTSR or FTSR register
        let exti_port = get_port!(EXTI_TypeDef, EXTI_BASE);
        write_assign_register!(exti_port.IMR1, |, 1 << 13);

        // Falling edge
        write_assign_register!(exti_port.RTSR1, |, 1 << 13);

        // Enable NVIC IRQ
        nvic::enable_irq(EXTI15_10_IRQn as u8);
    }

    // GPIOB Pin 6, 7
    fn configure_usart_rx_tx() -> impl UsartInOut {
        let gpiob_peripheral = GPIOB_PORT.take();
        // Configure GPIOB port Pin 6 and Pin 7 for USART
        gpiob_peripheral.configure_for_usart(GPIOAlternate::AF7, 6);
        gpiob_peripheral.configure_for_usart(GPIOAlternate::AF7, 7);

        let usart1_rx_tx = USART1_PORT.take().configure_default_rx_tx();
        usart1_rx_tx
    }

    // Start
    let mut rcc_register = RCC_PORT.take().get_register();
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
        let exti_port = get_port!(EXTI_TypeDef, EXTI_BASE);
        if read_register!(exti_port.PR1) >> 13 & 0x01 == 1 {
            write_assign_register!(exti_port.PR1, |, 1 << 13);
            BUTTON_PRESSED.store(true, Ordering::SeqCst);
        }
    });
    configure_gpio_input_interrupt();

    // USART
    let mut usart1_rx_tx = configure_usart_rx_tx();

    const TIME: u32 = 100_000;
    let mut counter = 0;
    loop {
        if BUTTON_PRESSED.load(Ordering::SeqCst) {
            usart1_rx_tx.write_str("Button Pressed\r\n").unwrap();
            BUTTON_PRESSED.store(false, Ordering::SeqCst);
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
