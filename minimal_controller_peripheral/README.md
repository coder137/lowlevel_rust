- [Minimal Controller Peripheral](#minimal-controller-peripheral)
  - [Links](#links)
  - [Microcontrollers layers](#microcontrollers-layers)
  - [Pre-requisites](#pre-requisites)
  - [Controller Peripherals](#controller-peripherals)
    - [Bindgen](#bindgen)
    - [CMSIS2Rust](#cmsis2rust)

# Minimal Controller Peripheral

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
- L4 Sensor
- L5 Application

## Pre-requisites

- Pre-requisites from `minimal_buildsystem`

## Controller Peripherals

We do not want to map out controller peripherals by hand, going through the datasheet.
There are current a few options to generate controller peripherals in rust

- Bindgen (C header to rust)
- Svd2Rust (CMSIS SVD to rust)

### Bindgen

- [CMSIS 5](https://github.com/ARM-software/CMSIS_5)
- [STM32 Github repository](https://github.com/STMicroelectronics/STM32Cube_MCU_Overall_Offer)
- [STM32L4 microcontroller family](https://github.com/STMicroelectronics/STM32CubeL4)


---

- Header CMSIS and STM32L4 startup files are taken from the repositories mentioned above
- These files have been put in the `l0/device` folder
- Bindgen is used to convert the header files to rust code
  - See `build.rs` for parsing `l0/device/controller/stm32l475xx.c`
  - See `src/controller.rs` which is the generated rust file
- `l0` workspace is added as a library
- `l5` workspace is the **application** which can use the peripherals in l0 to cleanup `l5/src/blink.rs`

### CMSIS2Rust

- Aggregated CMSIS-SVD files for [multiple microcontroller families](https://github.com/posborne/cmsis-svd)

---

- CMSIS to rust currently has a lot of dependencies on additional modules which is why it has not been used in this project.
- The goal is to minimize dependencies on external packages at lower layers

