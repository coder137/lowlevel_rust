// This should only be included for supported target architectures
#![cfg(not(test))] // No tests allowed
#![no_std] // No std only

// Other macros for generated controller.rs
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Generated controller bindings
mod controller;
pub use controller::*;
