// This should only be included for supported target architectures
#![cfg(not(test))] // No tests allowed
#![cfg(all(target_arch = "arm", target_os = "none"))] // ARM for microcontrollers allowed
#![no_std] // No std library, core library only

// Other macros for generated controller.rs
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// * Private to l0
mod entry_point;
mod global;
mod rust_entry_point;

// TODO, Add features
// #[cfg(feature = "stm32l475xx")]
mod stm32l475xx;
use stm32l475xx as chip;

// * Public APIs usable when l0 is a dependency
mod utility; // Macro export makes macros always public
pub use global::get_system_clock;

// TODO, Add features
// #[cfg(feature = "arm_cm4")]
mod arm_cm4;
pub use arm_cm4::*;

pub use chip::public::*;

// Generated controller bindings from C to Rust
pub mod controller;

// Whats the difference between the chip module and controller module
// controller.rs contains bindings for ARM Architecture + Chip specific peripheral structs (STM32L475xx)
// chip.rs contains only Chip specific functionality (STM32L475xx)
