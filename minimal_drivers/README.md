- [Minimal Drivers](#minimal-drivers)
  - [Links](#links)
  - [Microcontrollers layers](#microcontrollers-layers)
  - [Pre-requisites](#pre-requisites)
  - [GPIO](#gpio)
- [Changelog](#changelog)
  - [L3 Layer - Interfaces](#l3-layer---interfaces)
  - [L3 Layer - Drivers](#l3-layer---drivers)
  - [L3 Layer - Miscellaneous](#l3-layer---miscellaneous)
  - [L4](#l4)

# Minimal Drivers

This code has been tested on

- B-L475-IOT01A board (STM32L475VGT6 ARM Cortex M4 CPU with FPU)

## Links

- [Cargo binutils](https://github.com/rust-embedded/cargo-binutils)
- [Embedded Rust book](https://doc.rust-lang.org/stable/embedded-book/)
- [Lowlevel Embedded Rust book](https://docs.rust-embedded.org/embedonomicon/)

## Microcontrollers layers

- L0 Lowlevel
  - CMSIS
  - Controller registers
  - Startup
  - Linker script
- L1 RTOS
- L2 Utility
- L3 Driver
  - GPIO
  - UART
- L4 Sensor
- L5 Application

## Pre-requisites

- Pre-requisites from `minimal_controller_peripheral`

## GPIO

- Module
  - Functionality
- Port
  - Registers
- Pin

---

- [GPIO Traits](https://github.com/mbr/gpio-rs)

```mermaid
graph BT;
  subgraph Port
    GPIOA-H
    subgraph Periphal
      GPIO
      subgraph Register
        MODER
        IDR
        ODR
        subgraph Pin
          0-15
        end
      end
    end
  end
```

# Changelog

## L3 Layer - Interfaces

- GpioIn
- GpioOut
- UsartIn
- UsartOut
- UsartInOut
- Port
  - Generic interface that creates a port using base address and peripheral register layout
  - In C it would be the equivalent of `GPIO_TypeDef * gpio = (GPIO_TypeDef *)BASE_ADDRESS`

## L3 Layer - Drivers

- RCC
- GPIO

## L3 Layer - Miscellaneous

- Singleton
  - Safe access to global ports

## L4 

- Led
- Button
