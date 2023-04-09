# lowlevel_rust

Rust on microcontrollers

# Base Projects

- Minimal blinky
  - Barebones blinky example i.e linker script to main
- Minimal buildsystem
  - Initial [cargo-make](https://github.com/sagiegurari/cargo-make) framework to have configurable build options i.e extending `cargo`
- Minimal controller peripheral
  - Use bindgen to transform `.c` architecture and chip files to `.rs`
  - Create `l0` and `l5` workspace layers
  - Add **architecture**, **controller** and **startup** files in `l0`
- Minimal Drivers
  - Write GPIO and USART drivers for high level application usage
  - Added `l2`, `l3` and `l4` workspace layers
  - Added **bitflags** utility library in `l2` via crates.io
  - Added **driver interfaces**, **gpio**, **usart** drivers in `l3`
  - Added **led** and **button** modules in `l4`
- Minimal Interrupt
  - Configure interrupts from high level application code
  - Updated `l0` `l2`, `l3` and `l4` workspace layers
  - Updated `l0` with user interrupt considerations
  - Added **heapless** library to `l2`
  - Added USART buffered traits to `l3`

# Async Rust

- Minimal Async Basic
  - Forked from `Minimal Interrupt`
  - Configure Interrupts and main loop with rust cooperative async/await
  - No executor/waker implementation. Just basic polling functionality

# Roadmap

## Supported Architecture

- [x] ARM Cortex M4

> TODO, Add more eventually

## Supported Chips

- [x] STM32L475xx
- [ ] LPC4078xx

## Supported Development platforms

**See CI/CD**

- [x] Windows
- [x] Linux
- [x] Mac  

## Async Rust

- [x] Basic Async Rust 
  - Polling support
- [ ] Efficient Async Rust
  - Waker support
  - Interrupt support

## RTOS

### C based

- [ ] FreeRTOS
- [ ] Zephyr RTOS

### Rust based

- [ ] RTIC

## Debugging

- [x] OpenOCD
- [ ] Semihosting

## Mocking 

- [ ] Mockall

## Buildsystem

- [x] Cargo
- [x] Cargo Make
- [ ] Cargo features
  - Conditional compiling for additional platforms 

## Code coverage

- [ ] Grcov
  - Rust based code coverage 
- [ ] Lcov
  - Stable code coverage tool
- [ ] Codecov
  - Web based code coverage
- [ ] Coveralls
  - Web based code coverage

## FFI compat with C

- [x] Bindgen
  - Use C in Rust 
- [ ] CBindgen
  - Use Rust in C 

## Crates.io Libraries

- [x] [Bitflags](https://crates.io/crates/bitflags)
  - Rust macros to generate bitflags
- [x] [Heapless](https://crates.io/crates/heapless)
  - Stack allocated data structures

## Rust integrated tooling

- [x] Unit testing
- [ ] Clippy
  - Linting 
- [x] Cargofmt
  - Integrated in VSCode
- [ ] Documentation 

## External tooling

- [x] Continuous Integration
  - [x] Github Actions
