use crate::Part;
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
        let mut parts = self.parts.iter();
        if let Some(first) = parts.next() {
            write!(f, "{}", first)?;
        }
        for part in parts {
            write!(f, ",{}", part)?;
        }
        Ok(())
    }
}
