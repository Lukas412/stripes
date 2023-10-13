use crate::{Part, SolidColor};
use std::fmt::{Display, Formatter};

pub struct Frame {
    parts: Vec<Part>,
}

impl Frame {
    pub fn new() -> Self {
        Self { parts: vec![] }
    }

    pub fn add(&mut self, part: impl Into<Part>) {
        self.parts.push(part.into())
    }

    pub fn with(mut self, part: impl Into<Part>) -> Self {
        self.add(part);
        self
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
