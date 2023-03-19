#[derive(Copy, Clone)]
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

extern "C" {
    fn WWDG_Interrupt_Handler();
    fn PVD_PVM_Interrupt_Handler();
    fn RTC_TAMP_STAMP_Interrupt_Handler();
    fn RTC_WKUP_Interrupt_Handler();
    fn FLASH_Interrupt_Handler();
    fn RCC_Interrupt_Handler();
    fn EXTI0_Interrupt_Handler();
    fn EXTI1_Interrupt_Handler();
    fn EXTI2_Interrupt_Handler();
    fn EXTI3_Interrupt_Handler();
    fn EXTI4_Interrupt_Handler();
    fn DMA1_CH1_Interrupt_Handler();
    fn DMA1_CH2_Interrupt_Handler();
    fn DMA1_CH3_Interrupt_Handler();
    fn DMA1_CH4_Interrupt_Handler();
    fn DMA1_CH5_Interrupt_Handler();
    fn DMA1_CH6_Interrupt_Handler();
    fn DMA1_CH7_Interrupt_Handler();
    fn ADC1_2_Interrupt_Handler();
    fn CAN1_TX_Interrupt_Handler();
    fn CAN1_RX0_Interrupt_Handler();
    fn CAN1_RX1_Interrupt_Handler();
    fn CAN1_SCE_Interrupt_Handler();
    fn EXTI9_5_Interrupt_Handler();
    fn TIM1_BRK_Interrupt_Handler();
    fn TIM1_UP_Interrupt_Handler();
    fn TIM1_TRG_COM_Interrupt_Handler();
    fn TIM1_CC_Interrupt_Handler();
    fn TIM2_Interrupt_Handler();
    fn TIM3_Interrupt_Handler();
    fn TIM4_Interrupt_Handler();
    fn I2C1_EV_Interrupt_Handler();
    fn I2C1_ER_Interrupt_Handler();
    fn I2C2_EV_Interrupt_Handler();
    fn I2C2_ER_Interrupt_Handler();
    fn SPI1_Interrupt_Handler();
    fn SPI2_Interrupt_Handler();
    fn USART1_Interrupt_Handler();
    fn USART2_Interrupt_Handler();
    fn USART3_Interrupt_Handler();
    fn EXTI15_10_Interrupt_Handler();
    fn RTC_ALARM_Interrupt_Handler();
    fn DFSDM1_FLT3_Interrupt_Handler();
    fn TIM8_BRK_Interrupt_Handler();
    fn TIM8_UP_Interrupt_Handler();
    fn TIM8_TRG_COM_Interrupt_Handler();
    fn TIM8_CC_Interrupt_Handler();
    fn ADC3_Interrupt_Handler();
    fn FMC_Interrupt_Handler();
    fn SDMMC1_Interrupt_Handler();
    fn TIM5_Interrupt_Handler();
    fn SPI3_Interrupt_Handler();
    fn UART4_Interrupt_Handler();
    fn UART5_Interrupt_Handler();
    fn TIM6_DAC_Interrupt_Handler();
    fn TIM7_Interrupt_Handler();
    fn DMA2_CH1_Interrupt_Handler();
    fn DMA2_CH2_Interrupt_Handler();
    fn DMA2_CH3_Interrupt_Handler();
    fn DMA2_CH4_Interrupt_Handler();
    fn DMA2_CH5_Interrupt_Handler();
    fn DFSDM1_FLT0_Interrupt_Handler();
    fn DFSDM1_FLT1_Interrupt_Handler();
    fn DFSDM1_FLT2_Interrupt_Handler();
    fn COMP_Interrupt_Handler();
    fn LPTIM1_Interrupt_Handler();
    fn LPTIM2_Interrupt_Handler();
    fn OTG_FS_Interrupt_Handler();
    fn DMA2_CH6_Interrupt_Handler();
    fn DMA2_CH7_Interrupt_Handler();
    fn LPUART1_Interrupt_Handler();
    fn QUADSPI_Interrupt_Handler();
    fn I2C3_EV_Interrupt_Handler();
    fn I2C3_ER_Interrupt_Handler();
    fn SAI1_Interrupt_Handler();
    fn SAI2_Interrupt_Handler();
    fn SWPMI1_Interrupt_Handler();
    fn TSC_Interrupt_Handler();
    fn LCD_Interrupt_Handler();
    fn AES_Interrupt_Handler();
    fn RNG_Interrupt_Handler();
    fn FPU_Interrupt_Handler();
}

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
static INTERRUPTS: [unsafe extern "C" fn(); 82] = [
    WWDG_Interrupt_Handler,
    PVD_PVM_Interrupt_Handler,
    RTC_TAMP_STAMP_Interrupt_Handler,
    RTC_WKUP_Interrupt_Handler,
    FLASH_Interrupt_Handler,
    RCC_Interrupt_Handler,
    EXTI0_Interrupt_Handler,
    EXTI1_Interrupt_Handler,
    EXTI2_Interrupt_Handler,
    EXTI3_Interrupt_Handler,
    EXTI4_Interrupt_Handler,
    DMA1_CH1_Interrupt_Handler,
    DMA1_CH2_Interrupt_Handler,
    DMA1_CH3_Interrupt_Handler,
    DMA1_CH4_Interrupt_Handler,
    DMA1_CH5_Interrupt_Handler,
    DMA1_CH6_Interrupt_Handler,
    DMA1_CH7_Interrupt_Handler,
    ADC1_2_Interrupt_Handler,
    CAN1_TX_Interrupt_Handler,
    CAN1_RX0_Interrupt_Handler,
    CAN1_RX1_Interrupt_Handler,
    CAN1_SCE_Interrupt_Handler,
    EXTI9_5_Interrupt_Handler,
    TIM1_BRK_Interrupt_Handler,
    TIM1_UP_Interrupt_Handler,
    TIM1_TRG_COM_Interrupt_Handler,
    TIM1_CC_Interrupt_Handler,
    TIM2_Interrupt_Handler,
    TIM3_Interrupt_Handler,
    TIM4_Interrupt_Handler,
    I2C1_EV_Interrupt_Handler,
    I2C1_ER_Interrupt_Handler,
    I2C2_EV_Interrupt_Handler,
    I2C2_ER_Interrupt_Handler,
    SPI1_Interrupt_Handler,
    SPI2_Interrupt_Handler,
    USART1_Interrupt_Handler,
    USART2_Interrupt_Handler,
    USART3_Interrupt_Handler,
    EXTI15_10_Interrupt_Handler,
    RTC_ALARM_Interrupt_Handler,
    DFSDM1_FLT3_Interrupt_Handler,
    TIM8_BRK_Interrupt_Handler,
    TIM8_UP_Interrupt_Handler,
    TIM8_TRG_COM_Interrupt_Handler,
    TIM8_CC_Interrupt_Handler,
    ADC3_Interrupt_Handler,
    FMC_Interrupt_Handler,
    SDMMC1_Interrupt_Handler,
    TIM5_Interrupt_Handler,
    SPI3_Interrupt_Handler,
    UART4_Interrupt_Handler,
    UART5_Interrupt_Handler,
    TIM6_DAC_Interrupt_Handler,
    TIM7_Interrupt_Handler,
    DMA2_CH1_Interrupt_Handler,
    DMA2_CH2_Interrupt_Handler,
    DMA2_CH3_Interrupt_Handler,
    DMA2_CH4_Interrupt_Handler,
    DMA2_CH5_Interrupt_Handler,
    DFSDM1_FLT0_Interrupt_Handler,
    DFSDM1_FLT1_Interrupt_Handler,
    DFSDM1_FLT2_Interrupt_Handler,
    COMP_Interrupt_Handler,
    LPTIM1_Interrupt_Handler,
    LPTIM2_Interrupt_Handler,
    OTG_FS_Interrupt_Handler,
    DMA2_CH6_Interrupt_Handler,
    DMA2_CH7_Interrupt_Handler,
    LPUART1_Interrupt_Handler,
    QUADSPI_Interrupt_Handler,
    I2C3_EV_Interrupt_Handler,
    I2C3_ER_Interrupt_Handler,
    SAI1_Interrupt_Handler,
    SAI2_Interrupt_Handler,
    SWPMI1_Interrupt_Handler,
    TSC_Interrupt_Handler,
    LCD_Interrupt_Handler,
    AES_Interrupt_Handler,
    RNG_Interrupt_Handler,
    FPU_Interrupt_Handler,
];
