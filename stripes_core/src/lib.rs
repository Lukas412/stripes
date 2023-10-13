use std::fmt::{Display, Formatter};

mod changes;
mod frame;

pub use {
    changes::{Change, Changes},
    frame::Frame,
};

pub struct Animation {
    start: Frame,
    changes: Vec<Changes>,
}

impl Animation {
    pub fn new(start: Frame) -> Self {
        Self {
            start,
            changes: vec![],
        }
    }

    pub fn add(&mut self, changes: Changes) {
        self.changes.push(changes)
    }

    pub fn with(mut self, changes: Changes) -> Self {
        self.add(changes);
        self
    }
}

impl Display for Animation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.start)?;
        for changes in self.changes.iter() {
            write!(f, "{}", changes)?;
        }
        Ok(())
    }
}

pub enum Part {
    Solid(SolidColor),
    Linear(LinearGradient),
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Solid(color) => {
                write!(f, "{}", color)
            }
            Part::Linear(gradient) => {
                write!(f, "{}", gradient)
            }
        }
    }
}

pub struct SolidColor {
    color: Color,
    length: u8,
}

impl SolidColor {
    pub fn new(color: Color, mut length: u8) -> Self {
        if length == 0 {
            length = 1;
        }
        Self { color, length }
    }

    pub fn single(color: Color) -> Self {
        Self { color, length: 1 }
    }
}

impl Display for SolidColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "c{}l{}", self.color, self.length)
    }
}

pub struct LinearGradient {
    start: Color,
    end: Color,
    length: u8,
}

impl LinearGradient {
    pub fn new(start: Color, end: Color, mut length: u8) -> Self {
        if length <= 1 {
            length = 2;
        }
        Self { start, end, length }
    }
}

impl Display for LinearGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "s{}e{}l{}", self.start, self.end, self.length)
    }
}

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

#[cfg(test)]
mod tests;
