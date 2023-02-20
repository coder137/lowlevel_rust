# lowlevel_rust

Rust on microcontrollers

# Projects

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

# Roadmap

## RTOS

## Debugging

## Tooling
