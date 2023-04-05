use l0::{get_port, read_register, write_assign_register, write_register, EXTI_TypeDef, EXTI_BASE};

pub enum EXTIType {
    RisingEdge,
    FallingEdge,
    RisingAndFallingEdge,
}

pub struct EXTIRegister {
    port: &'static mut EXTI_TypeDef,
}

impl EXTIRegister {
    pub fn configure_interrupt(&mut self, pin: u32, interrupt_type: EXTIType) {
        write_assign_register!(self.port.IMR1, |, 1 << pin);

        match interrupt_type {
            EXTIType::RisingEdge => {
                write_assign_register!(self.port.RTSR1, |, 1 << pin);
            }
            EXTIType::FallingEdge => {
                write_assign_register!(self.port.FTSR1, |, 1 << pin);
            }
            EXTIType::RisingAndFallingEdge => {
                write_assign_register!(self.port.RTSR1, |, 1 << pin);
                write_assign_register!(self.port.FTSR1, |, 1 << pin);
            }
        }
    }

    pub fn is_pending_interrupt(&self, pin: u32) -> bool {
        read_register!(self.port.PR1) >> pin & 0x01 == 1
    }

    pub fn clear_pending_interrupt(&mut self, pin: u32) {
        write_register!(self.port.PR1, 1 << pin);
    }
}

pub struct EXTIPeripheral<const B: u32>;

impl<const B: u32> EXTIPeripheral<B> {
    pub fn get_register() -> EXTIRegister {
        EXTIRegister {
            port: get_port!(EXTI_TypeDef, B),
        }
    }
}

pub type EXTI = EXTIPeripheral<EXTI_BASE>;
