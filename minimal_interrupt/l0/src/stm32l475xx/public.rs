use crate::{
    controller::{SCB_Type, SCB_BASE},
    get_port, read_register,
};

pub enum Interrupt {
    WWDG,
    PVD_PVM,
    RTC_TAMP_STAMP,
    RTC_WKUP,
    FLASH,
    RCC,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    ADC1_2,
    CAN1_TX,
    CAN1_RX0,
    CAN1_RX1,
    CAN1_SCE,
    EXTI9_5,
    TIM1_BRK,
    TIM1_UP,
    TIM1_TRG_COM,
    TIM1_CC,
    TIM2,
    TIM3,
    TIM4,
    I2C1_EV,
    I2C1_ER,
    I2C2_EV,
    I2C2_ER,
    SPI1,
    SPI2,
    USART1,
    USART2,
    USART3,
    EXTI15_10,
    RTC_ALARM,
    DFSDM1_FLT3,
    TIM8_BRK,
    TIM8_UP,
    TIM8_TRG_COM,
    TIM8_CC,
    ADC3,
    FMC,
    SDMMC1,
    TIM5,
    SPI3,
    UART4,
    UART5,
    TIM6_DAC,
    TIM7,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DFSDM1_FLT0,
    DFSDM1_FLT1,
    DFSDM1_FLT2,
    COMP,
    LPTIM1,
    LPTIM2,
    OTG_FS,
    DMA2_CH6,
    DMA2_CH7,
    LPUART1,
    QUADSPI,
    I2C3_EV,
    I2C3_ER,
    SAI1,
    SAI2,
    SWPMI1,
    TSC,
    LCD,
    AES,
    RNG,
    FPU,
}

#[no_mangle]
pub extern "C" fn DefaultInterruptHandler() {
    let scb_port = get_port!(SCB_Type, SCB_BASE);
    let irq_num = (read_register!(scb_port.ICSR) & 0xFF) - 16;
    unsafe {
        MY_INTERRUPTS
            .get(irq_num as usize)
            .expect("No interrupt registered")();
    }
}

static mut MY_INTERRUPTS: [&dyn Fn() -> (); 82] = [&|| {}; 82];

pub fn attach_interrupt_handler(interrupt: Interrupt, handler: &'static dyn Fn()) {
    let index: usize = interrupt as usize;
    unsafe {
        MY_INTERRUPTS[index] = handler;
    }
}

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
static INTERRUPTS: [unsafe extern "C" fn(); 82] = [DefaultInterruptHandler; 82];
