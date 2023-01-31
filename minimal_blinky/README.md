- [Getting started](#getting-started)
  - [Links](#links)
  - [Pre-requisites](#pre-requisites)
  - [Build](#build)
  - [Running the code](#running-the-code)

# Getting started

## Links

- [Cargo binutils](https://github.com/rust-embedded/cargo-binutils)
- [Embedded Rust book](https://doc.rust-lang.org/stable/embedded-book/)
- [Lowlevel Embedded Rust book](https://docs.rust-embedded.org/embedonomicon/)

## Pre-requisites

- CMake (v3.16)
- arm-none-eabi-gcc (v11.2)
- Ninja
- Rust

## Build

- `rustup default stable`
- `rustup target add <your_target>`
  - See **.cargo/config.toml** file
- `cargo install cargo-binutils`

## Running the code

- `cargo build`
- Run the code on the target board using the **.vscode/launch.json** configurations
  - These can also be manually run on the target using OpenOCD
  - Requires the **cortex-debug** vscode extension
