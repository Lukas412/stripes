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

    pub fn red() -> Self {
        Self::new(u8::MAX, u8::MIN, u8::MIN)
    }

    pub fn green() -> Self {
        Self::new(u8::MIN, u8::MAX, u8::MIN)
    }

    pub fn blue() -> Self {
        Self::new(u8::MIN, u8::MIN, u8::MAX)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
