- [Minimal Buildsystem](#minimal-buildsystem)
  - [Links](#links)
  - [Pre-requisites](#pre-requisites)
  - [Build](#build)
  - [Running the code](#running-the-code)

# Minimal Buildsystem

This code has been tested on

- B-L475-IOT01A board (STM32L475VGT6 ARM Cortex M4 CPU with FPU)

## Links

- [Cargo binutils](https://github.com/rust-embedded/cargo-binutils)
- [Embedded Rust book](https://doc.rust-lang.org/stable/embedded-book/)
- [Lowlevel Embedded Rust book](https://docs.rust-embedded.org/embedonomicon/)

## Pre-requisites

- arm-none-eabi-gcc (v11.2)
- Rust

## Build

- `rustup default stable`
- `rustup target add <your_target>`
  - See **.cargo/config.toml** file to install the correct target
- `cargo install cargo-binutils`

## Running the code

- `cargo build`
- Run the code on the target board using the **.vscode/launch.json** configurations
  - These can also be manually run on the target using OpenOCD
  - Requires the **cortex-debug** vscode extension
