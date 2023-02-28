use core::fmt::Write;

// * Instead of having an `UsartOut` trait, we can use the Rust core library `core::fmt::Write` trait

pub trait UsartIn {
    /// Blocking read
    fn read_character(&mut self) -> char;
}

pub trait UsartBufferedIn {
    /// Number of elements contained within the queue
    fn size(&self) -> usize;
    /// Non blocking read
    fn try_read_character(&mut self) -> Option<char>;
}

pub trait UsartInOut: UsartIn + Write {}

pub trait UsartBufferedInOut: UsartBufferedIn + Write {}
