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
    // /// If queue contains a newline character
    // /// Returns size of string and true/false
    // fn contains_newline(&self) -> (usize, bool);
    // /// Fill line buffer with data present in the queue
    // /// Returns true only if all conditions are satisfied
    // /// - line buffer has enough space to hold the entire line
    // /// - there is a newline present in the queue
    // fn read_line(&mut self, line: &mut [char]) -> bool;
}

pub trait UsartInOut: UsartIn + Write {}

pub trait UsartBufferedInOut: UsartBufferedIn + Write {}
