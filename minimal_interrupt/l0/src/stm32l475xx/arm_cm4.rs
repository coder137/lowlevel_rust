pub mod nvic {
    use crate::{write_register, Interrupt, NVIC_PORT, __NVIC_PRIO_BITS};

    pub fn enable_irq(irq: Interrupt) {
        // NVIC->ISER[(((uint32_t)IRQn) >> 5UL)] = (uint32_t)(1UL << (((uint32_t)IRQn) & 0x1FUL));
        let nvic_port = NVIC_PORT::port();
        write_register!(
            nvic_port.ISER[(irq as usize) >> 5],
            1 << ((irq as u8) & 0x1F)
        );
    }

    pub fn set_priority(irq: Interrupt, priority: u8) {
        //     NVIC->IP[((uint32_t)IRQn)]               = (uint8_t)((priority << (8U - __NVIC_PRIO_BITS)) & (uint32_t)0xFFUL);
        let nvic_port = NVIC_PORT::port();
        const SHIFT: u8 = 8 - __NVIC_PRIO_BITS as u8;
        write_register!(nvic_port.IP[irq as usize], priority << SHIFT);
    }
}

pub mod arm {
    use core::arch::asm;

    pub fn ldrexw(addr: *const u32) -> u32 {
        // __ASM volatile("ldrex %0, %1"
        // : "=r"(result)
        // : "Q"(*addr));
        let result: u32;
        unsafe {
            asm!(
                "ldrex {0}, [{1}]", out(reg) result, in(reg) addr);
        }
        result
    }

    pub fn strexw(addr: *const u32, data: u32) -> u32 {
        // __ASM volatile("strex %0, %2, %1"
        //                : "=&r"(result), "=Q"(*addr)
        //                : "r"(value));
        let result: u32;
        unsafe {
            asm!("strex {0}, {2}, [{1}]", out(reg) result, in(reg) addr, in(reg) data);
        }
        result
    }

    /// Example usage:
    /// let mut storage = 2;
    /// let storage_mut = &mut storage;
    /// loop {
    ///     let mut data = arm::ldrexw(storage_mut);
    ///     data += 25; // mutate or check data
    ///     let stored = arm::strexw(storage_mut, data);
    ///     if stored == 0 {
    ///         break;
    ///     }
    /// }
    pub fn simple_mutex<F>(addr: *const u32, mut mutate_data_cb: F)
    where
        F: FnMut(u32) -> (u32, bool),
    {
        loop {
            let data = ldrexw(addr);
            let (data, store) = mutate_data_cb(data);
            if !store {
                break;
            }
            if strexw(addr, data) == 0 {
                break;
            }
        }
    }
}
