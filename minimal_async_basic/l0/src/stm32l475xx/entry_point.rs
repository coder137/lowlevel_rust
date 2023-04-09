use core::{ops::Add, time::Duration};

use crate::{chip::controller_init, global::SYSTEM_TIME};

// NOTE, All the externed modules come here
#[no_mangle]
pub unsafe extern "C" fn Reset() {
    // Data and BSS sections
    extern "C" {
        // .data section
        static mut __data_end__: u8;
        static mut __data_start__: u8;
        static mut __etext: u8;

        // .bss section
        static mut __bss_start__: u8;
        static mut __bss_end__: u8;
    }

    // Copy from VMA to LMA
    let vma_data_end = &__data_end__ as *const u8;
    let vma_data_start = &__data_start__ as *const u8;
    let lma_data_start = &__etext as *const u8;
    let count: usize = vma_data_end as usize - vma_data_start as usize;
    // core::ptr::copy_nonoverlapping(lma_data_start, &mut __data_start__ as *mut u8, count);
    core::ptr::copy_nonoverlapping(lma_data_start, vma_data_start as *mut u8, count);

    // Write 0 to .bss section
    let bss_end = &__bss_end__ as *const u8;
    let bss_start = &__bss_start__ as *const u8;
    let count = bss_end as usize - bss_start as usize;
    // core::ptr::write_bytes(&mut __bss_start__ as *mut u8, 0, count);
    core::ptr::write_bytes(bss_start as *mut u8, 0, count);

    // Controller level startup system initialization

    // Jump to L0 controller system init
    // TODO, Jump to L4 board init
    // Jump to L5 main function
    extern "Rust" {
        fn main() -> !;
    }
    controller_init();
    main();
}

extern "C" {
    fn __StackTop(); // Check `gcc_arm.ld`
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
}

#[repr(C)]
pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 16] = [
    Vector {
        handler: __StackTop,
    },
    Vector { handler: Reset },
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 }, // Debug Monitor Handler comes here
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

// SysTick interrupt
#[no_mangle]
unsafe extern "C" fn SysTick() {
    SYSTEM_TIME = SYSTEM_TIME.add(Duration::from_millis(1));
}

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}
