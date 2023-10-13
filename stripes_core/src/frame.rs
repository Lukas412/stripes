use crate::{Part, SolidColor};
use std::fmt::{Display, Formatter};

pub struct Frame {
    parts: Vec<Part>,
}

impl Frame {
    pub fn new() -> Self {
        Self { parts: vec![] }
    }

    pub fn add_solid_color(&mut self, value: SolidColor) {
        self.add(Part::Solid(value))
    }

    fn add(&mut self, part: Part) {
        self.parts.push(part)
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
