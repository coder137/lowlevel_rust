use core::fmt::Write;

// * Instead of having an `UsartOut` trait, we can use the Rust core library `core::fmt::Write` trait

pub trait UsartIn {
    fn read_character(&mut self) -> char;
}

pub trait UsartInOut: UsartIn + Write {}
