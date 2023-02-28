use crate::{
    EXTI_TypeDef, NVIC_Type, Port, RCC_TypeDef, SCB_Type, SYSCFG_TypeDef, USART_TypeDef, EXTI_BASE,
    NVIC_BASE, RCC_BASE, SCB_BASE, SYSCFG_BASE, USART1_BASE,
};

// ARM specific
pub type SCB_PORT = Port<SCB_Type, SCB_BASE>;
pub type NVIC_PORT = Port<NVIC_Type, NVIC_BASE>;

// STM32 specific
pub type RCC_PORT = Port<RCC_TypeDef, RCC_BASE>;
pub type SYSCFG_PORT = Port<SYSCFG_TypeDef, SYSCFG_BASE>;
pub type EXTI_PORT = Port<EXTI_TypeDef, EXTI_BASE>;
pub type USART1_PORT = Port<USART_TypeDef, USART1_BASE>;
