use core::fmt::Write;

pub trait UsartOut {
    fn write_character(&mut self, data: char);
    fn write_string(&mut self, data: &str) {
        data.chars().for_each(|d| {
            self.write_character(d);
        });
    }
}

pub trait UsartIn {
    fn read_character(&mut self) -> char;
}

pub trait UsartInOut: UsartIn + UsartOut {}

impl Write for dyn UsartOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(&s);
        Ok(())
    }
}

impl Write for dyn UsartInOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(&s);
        Ok(())
    }
}
