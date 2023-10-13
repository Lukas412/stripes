use std::fmt::{Display, Formatter};

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
