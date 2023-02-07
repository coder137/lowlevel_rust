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
    use l3::*;
    use l4::*;

    // Activate clock control for GPIOA
    let mut rcc_port = get_peripheral!(RCC_PERIPHERAL);
    rcc_port.set_ahb2enr(RCC_AHB2ENR::GPIOAEN);
    rcc_port.set_ahb2enr(RCC_AHB2ENR::GPIOCEN);

    let gpioa_port = get_peripheral!(GPIOA_PERIPHERAL);
    let gpioc_port = get_peripheral!(GPIOC_PERIPHERAL);

    // Configure GPIOA port and Pin 5 as output
    let mut gpio_out_at_pin5 = gpioa_port.configure_as_output(5);

    // Configure GPIOC port and Pin 13 as input
    let mut gpio_in_at_pin13 = gpioc_port.configure_as_input(13);

    // Created led module
    let mut led = Led::new(&mut gpio_out_at_pin5);
    let button = Button::new(&mut gpio_in_at_pin13, GpioValue::High);

    let mut time;
    loop {
        if button.pressed() {
            time = 20_000;
        } else {
            time = 100_000;
        }

        led.on();
        spin_delay(time);
        led.off();
        spin_delay(time);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_tests_work() {
        assert_eq!(1, 1);
    }
}
