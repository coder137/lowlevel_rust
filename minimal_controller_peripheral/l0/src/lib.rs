// This should only be included for supported target architectures
#![cfg(not(test))] // No tests allowed
#![cfg(all(target_arch = "arm", target_os = "none"))] // ARM for microcontrollers allowed
#![no_std] // No std library, core library only

// Other macros for generated controller.rs
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod entry_point;
mod rust_entry_point;

// Generated controller bindings
mod controller;
pub use controller::*;
