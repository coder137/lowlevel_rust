use crate::{
    global::SYSTEM_CLOCK, read_register, write_assign_register, write_register, FLASH_BASE,
    RCC_PORT, SCB_PORT, SYSTICK_PORT,
};
use core::sync::atomic::Ordering;

pub fn controller_init() {
    // Update the System clock
    let rcc_port = RCC_PORT::port();
    let cr_data = read_register!(rcc_port.CR);
    let msi_range = (cr_data >> 4) & 0xF;
    let system_clock: u32 = match msi_range {
        0 => todo!(),
        1 => todo!(),
        2 => todo!(),
        3 => todo!(),
        4 => 1_000_000,
        5 => 2_000_000,
        6 => 4_000_000,
        7 => todo!(),
        8 => todo!(),
        9 => todo!(),
        10 => todo!(),
        11 => todo!(),
        _ => unreachable!(),
    };
    SYSTEM_CLOCK.store(system_clock, Ordering::SeqCst);

    // Set SCB VTOR
    let scb_port = SCB_PORT::port();
    write_register!(scb_port.VTOR, FLASH_BASE);

    // Configure SysTick
    configure_systick_for_1ms(system_clock);
}

// 1ms <- More realistic
// 1us <- Extremely granular
fn configure_systick_for_1ms(systemclock: u32) {
    let systick_port = SYSTICK_PORT::port();
    // 1second -> systemclock cycles
    // 1ms -> systemclock / 1000 - 1 cycles;
    write_register!(systick_port.LOAD, (systemclock / 1000) - 1);

    const CTRL_ENABLE: u32 = 0;
    const CTRL_TICKINT: u32 = 1;
    const CTRL_CLKSOURCE: u32 = 2;
    write_assign_register!(systick_port.CTRL, |, (1 << CTRL_CLKSOURCE) | (1 << CTRL_TICKINT) | (1 << CTRL_ENABLE));
}
