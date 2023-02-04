- [Minimal Buildsystem](#minimal-buildsystem)
  - [Links](#links)
  - [Pre-requisites](#pre-requisites)
  - [Build system for Rust](#build-system-for-rust)
    - [\[build\_debug | build\_release\]](#build_debug--build_release)
    - [test](#test)
    - [flash\_debug](#flash_debug)
    - [\[ci\_debug | ci\_release\]](#ci_debug--ci_release)

# Minimal Buildsystem

This code has been tested on

- B-L475-IOT01A board (STM32L475VGT6 ARM Cortex M4 CPU with FPU)

## Links

- [Cargo binutils](https://github.com/rust-embedded/cargo-binutils)
- [Embedded Rust book](https://doc.rust-lang.org/stable/embedded-book/)
- [Lowlevel Embedded Rust book](https://docs.rust-embedded.org/embedonomicon/)

## Pre-requisites

- Pre-requisites from `minimal_blinky`
- cargo install cargo-make

## Build system for Rust

Cargo make is used to build, run and deploy various aspects of this project.
This is because we need configurations for

- Building microcontroller (on-target) code for different supported architectures and toolchains.
  - Pre-processing (.c to .rs conversion, code generation)
  - Building (convert to .elf)
  - Post-processing (.elf size, .bin and .hex generation, flashing after build, CI run)
- Unit-testing functionality (off-target) using host toolchain
- Documentation generation

Commands can be run using 

```bash
cargo make [command]
```

### [build_debug | build_release]

Makes a debug or release build of the project using the microcontroller target

See `.cargo/config.toml`, **build.target** field

### test

Make a build of the project using the default system host toolchain and target

Run `rustup default` to see your system host toolchain
Run `rustup target list` to see the system host target installed for your toolchain

### flash_debug

Uses openocd to flash your generated `/debug/*.elf` file to the STM32 microcontroller

### [ci_debug | ci_release]

Single command that does the following in order

- Build on-target code [debug | release]
- Size of on-target code
- Executes unit-tests and mocks using an off-target build
- Convert `*.elf` to `*.bin`
- Convert `*.elf` to `*.hex`
- Dump `*.elf` symbols to `*.lst`
