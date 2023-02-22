pub mod nvic {
    use crate::{
        controller::{NVIC_Type, NVIC_BASE},
        get_port, write_register,
    };

    pub fn enable_irq(irq: u8) {
        // NVIC->ISER[(((uint32_t)IRQn) >> 5UL)] = (uint32_t)(1UL << (((uint32_t)IRQn) & 0x1FUL));
        let nvic_port = get_port!(NVIC_Type, NVIC_BASE);
        write_register!(nvic_port.ISER[(irq as usize) >> 5], 1 << (irq & 0x1F));
    }
}
