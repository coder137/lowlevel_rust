#![cfg_attr(not(test), no_std)]

mod interfaces;
pub use interfaces::*;

pub mod peripheral;
pub use peripheral::*;

// TODO, Add features
// #[cfg(feature = "stm32l475xx")]
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub mod stm32l475xx;
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use stm32l475xx::*;
