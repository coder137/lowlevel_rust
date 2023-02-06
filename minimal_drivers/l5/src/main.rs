#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use l3::*;
use l4::*;

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
    const TIME: u32 = 100_000;

    // TODO, This should go instead GPIO
    let mut rcc = RCC::new(RCC_Port::get());
    rcc.set_ahb2enr(RCC_AHB2ENR::GPIOAEN);

    let mut gpio = GPIO::configure_as_output(GPIOA_Port::get(), GpioPin::Num5);
    let mut led = Led::new(&mut gpio);
    loop {
        led.on();
        spin_delay(TIME);
        led.off();
        spin_delay(TIME);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_tests_work() {
        assert_eq!(1, 1);
    }
}
