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
    use core::fmt::Write;
    use l0::*;
    use l3::*;
    use l4::*;

    let mut rcc_register = RCC_PORT.take().get_register();
    // Activate clock control for GPIOA, GPIOB and GPIOC and USART1EN
    rcc_register.set_ahb2enr(RCC_AHB2ENR::GPIOAEN | RCC_AHB2ENR::GPIOBEN | RCC_AHB2ENR::GPIOCEN);
    rcc_register.set_apb2enr(RCC_APB2ENR::USART1EN);

    let gpioa_peripheral = GPIOA_PORT.take();
    // Configure GPIOA port and Pin 5 as output
    let mut gpio_out_at_pin5 = gpioa_peripheral.configure_for_output(5);

    let gpiob_peripheral = GPIOB_PORT.take();
    // Configure GPIOB port Pin 6 and Pin 7 for USART
    gpiob_peripheral.configure_for_usart(GPIOAlternate::AF7, 6);
    gpiob_peripheral.configure_for_usart(GPIOAlternate::AF7, 7);

    let usart1_rx_tx: &mut dyn UsartInOut = &mut USART1_PORT.take().configure_default_rx_tx();

    let gpioc_peripheral = GPIOC_PORT.take();
    // Configure GPIOC port and Pin 13 as input
    let mut gpio_in_at_pin13 = gpioc_peripheral.configure_for_input(13);

    // Created led module
    let mut led = Led::new(&mut gpio_out_at_pin5);
    let button = Button::new(&mut gpio_in_at_pin13, GpioValue::High);

    let mut time;
    let mut counter = 0;
    loop {
        if button.pressed() {
            time = 20_000;
        } else {
            time = 100_000;
        }

        led.on();
        // Can also use write! and writeln!
        usart1_rx_tx
            .write_fmt(format_args!("LED ON: {}\r\n", counter))
            .unwrap();
        spin_delay(time);
        led.off();
        usart1_rx_tx
            .write_fmt(format_args!("LED OFF: {}\r\n", counter))
            .unwrap();
        spin_delay(time);
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
