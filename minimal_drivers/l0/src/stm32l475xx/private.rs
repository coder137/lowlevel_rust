use crate::{get_port, global::SYSTEM_CLOCK, read_register};
use core::ptr::read_volatile;
use core::sync::atomic::Ordering;

use crate::controller::{RCC_TypeDef, RCC_BASE};

pub fn controller_init() {
    // Update the System clock
    let rcc_port = get_port!(RCC_TypeDef, RCC_BASE);
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
    SYSTEM_CLOCK.store(system_clock, Ordering::SeqCst)

    // TODO, Do more things
}
