use std::fmt::{Display, Formatter};

mod changes;
mod frame;
mod part;

pub use {
    changes::{Change, Changes},
    frame::Frame,
    part::{LinearGradient, Part, SolidColor},
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
