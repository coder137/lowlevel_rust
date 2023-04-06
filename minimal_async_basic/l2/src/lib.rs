#![cfg_attr(not(test), no_std)]

pub use bitflags::bitflags;
pub use heapless;

// Asynchronous support
mod cooperative;
pub use cooperative::*;

// Contains following module
// cooperative::poll
// TODO, cooperative::wakeup

// cooperative::poll module aim to resolve the future through polling
// No waker / wakeup support
//

// cooperative::wakeup module aim to resolve the future via wakers/wakeups
// TODO, Make efficient versions of the above cooperative::poll APIs
// TODO, Need to model how interrupts can wakeup (AtomicWaker)
