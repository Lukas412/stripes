use std::fmt::{Display, Formatter};

mod changes;
mod color;
mod frame;
mod part;

pub use {
    changes::{Change, Changes},
    color::Color,
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

#[cfg(test)]
mod tests;
