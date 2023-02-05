#![cfg(not(test))]
#![allow(non_snake_case)]

use core::arch::asm;

pub fn _spin_delay(delay: u32) {
    let mut mdelay = delay;
    while mdelay != 0 {
        unsafe {
            asm!("nop");
        }
        mdelay -= 1;
    }
}

const PERIPH_BASE: u32 = 0x4000_0000;

const AHB1PERIPH_BASE: u32 = PERIPH_BASE + 0x0002_0000;
const RCC_BASE: u32 = AHB1PERIPH_BASE + 0x1000;

const AHB2PERIPH_BASE: u32 = PERIPH_BASE + 0x0800_0000;
const GPIOA_BASE: u32 = AHB2PERIPH_BASE + 0x0000;

pub fn blink_init() {
    //   RCC->AHB2ENR |= RCC_AHB2ENR_GPIOAEN;
    //   GPIOA->BRR |= (1 << 5); // Reset the pin here

    // Set the mode
    //   GPIOA->MODER &= ~(3 << 10);
    //   GPIOA->MODER |= (1 << 10); // 01 00 00 00 00 00

    // Check these registers
    //   GPIOA->OTYPER &= ~(1 << 5); // set to 0
    //   GPIOA->OSPEEDR &= ~(3 << 10);
    //   GPIOA->PUPDR &= ~(3 << 10);
    let safe_rcc = RCC_BASE as *mut l0::RCC_TypeDef;
    let rcc = unsafe { &mut *(safe_rcc) };

    let safe_gpioa = GPIOA_BASE as *mut l0::GPIO_TypeDef;
    let gpioa = unsafe { &mut *(safe_gpioa) };

    // activate GPIOA
    let mut rcc_ahb2enr = unsafe { core::ptr::read_volatile(&mut rcc.AHB2ENR) };
    rcc_ahb2enr |= 1 << 0;
    unsafe { core::ptr::write_volatile(&mut rcc.AHB2ENR, rcc_ahb2enr) };

    // MODER
    let mut gpioa_moder = unsafe { core::ptr::read_volatile(&mut gpioa.MODER) };
    gpioa_moder &= !(3 << 10);
    gpioa_moder |= 1 << 10;
    unsafe { core::ptr::write_volatile(&mut gpioa.MODER, gpioa_moder) };

    // OTYPER
    let mut gpioa_otyper = unsafe { core::ptr::read_volatile(&mut gpioa.OTYPER) };
    gpioa_otyper &= !(1 << 5);
    unsafe { core::ptr::write_volatile(&mut gpioa.OTYPER, gpioa_otyper) };

    // OSPEEDR
    let mut gpioa_ospeedr = unsafe { core::ptr::read_volatile(&mut gpioa.OSPEEDR) };
    gpioa_ospeedr &= !(3 << 10);
    unsafe { core::ptr::write_volatile(&mut gpioa.OSPEEDR, gpioa_ospeedr) };

    // PUPDR
    let mut gpioa_pupdr = unsafe { core::ptr::read_volatile(&mut gpioa.PUPDR) };
    gpioa_pupdr &= !(3 << 10);
    unsafe { core::ptr::write_volatile(&mut gpioa.PUPDR, gpioa_pupdr) };
}

pub fn blink_set() {
    // Set the pin here
    //   GPIOA->BSRR |= (1 << 5);
    let safe_gpioa = GPIOA_BASE as *mut l0::GPIO_TypeDef;
    let gpioa = unsafe { &mut *(safe_gpioa) };

    let mut gpioa_bsrr = unsafe { core::ptr::read_volatile(&mut gpioa.BSRR) };
    gpioa_bsrr |= 1 << 5;
    unsafe { core::ptr::write_volatile(&mut gpioa.BSRR, gpioa_bsrr) };
}

pub fn blink_reset() {
    //     _spin_delay(1000 * 1000);
    //     GPIOA->BRR = (1 << 5); // Reset
    let safe_gpioa = GPIOA_BASE as *mut l0::GPIO_TypeDef;
    let gpioa = unsafe { &mut *(safe_gpioa) };

    let mut gpioa_brr = unsafe { core::ptr::read_volatile(&mut gpioa.BRR) };
    gpioa_brr |= 1 << 5;
    unsafe { core::ptr::write_volatile(&mut gpioa.BRR, gpioa_brr) };
}
