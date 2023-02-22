#![cfg_attr(not(test), no_std)]

// * Private to l0
mod global; // Testable

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod rust_entry_point;

// TODO, Add features
// #[cfg(feature = "stm32l475xx")]
#[cfg(all(target_arch = "arm", target_os = "none"))]
mod stm32l475xx;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use stm32l475xx as chip;

// * Public APIs usable when l0 is a dependency
mod utility; // Macro export makes macros always public, Testable

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use chip::public::*;

pub use global::get_system_clock;
